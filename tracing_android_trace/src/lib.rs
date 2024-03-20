#![doc = concat!(
// TODO: Is this a new pattern?
"[AndroidTrace]: crate::AndroidTrace
[`tracing`]: tracing
[`tracing_subscriber::Layer`]: tracing_subscriber::Layer
[tracing_subscriber::filter]: tracing_subscriber::filter

<!-- Hide the header section of the README when using rustdoc -->
<div style=\"display:none\">
",
    include_str!("../README.md"),
)]
#![forbid(unsafe_code)]

#[cfg(not(target_os = "android"))]
compile_error!(
    r#"tracing_android_trace only supports Android. If you are depending on it, ensure that it is within
[target.'cfg(target_os = "android")'.dependencies]
in your Cargo.toml"#
);

mod async_layer;
pub use async_layer::ATraceLayerAsync;
use thread_local::ThreadLocal;

use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    fmt::Debug,
};

pub use android_trace;
use android_trace::AndroidTrace;
use tracing::span::{self, Id};
use tracing_subscriber::{
    fmt::{
        format::{DefaultFields, Writer},
        FormatFields,
    },
    registry::LookupSpan,
};

pub struct ATraceLayer {
    trace: AndroidTrace,
    fmt_fields: DefaultFields,
    current_actual_stack: ThreadLocal<RefCell<Vec<Option<Id>>>>,
}

impl ATraceLayer {
    pub fn new() -> Self {
        let trace = AndroidTrace::new_downlevel();
        Self::with_trace(trace)
    }

    pub fn with_trace(trace: AndroidTrace) -> Self {
        ATraceLayer {
            trace,
            fmt_fields: DefaultFields::new(),
            current_actual_stack: ThreadLocal::new(),
        }
    }
}

impl Default for ATraceLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub(crate) struct ATraceExtension {
    name: CString,
}

impl<S> tracing_subscriber::Layer<S> for ATraceLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a> + Debug,
    for<'a> <S as LookupSpan<'a>>::Data: Debug,
{
    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if self.trace.is_enabled().unwrap_or(false) {
            let span = ctx.span(id).expect("Span not found, this is a bug");
            let mut extensions = span.extensions_mut();
            let mut name = String::from(attrs.metadata().name());
            name.push_str(": ");
            if self
                .fmt_fields
                .format_fields(Writer::new(&mut name), attrs)
                .is_ok()
            {
                let name = CString::new(name);
                match name {
                    Ok(name) => extensions.insert::<ATraceExtension>(ATraceExtension { name }),
                    Err(e) => eprintln!(
                        concat!(
                            "[tracing_android_trace] Unable to format the following ",
                            "span due to a null byte ({:?}), ignoring: {:?}",
                        ),
                        e, attrs
                    ),
                }
            } else {
                eprintln!(
                    "[tracing_android_trace] Unable to format the following event, ignoring: {:?}",
                    attrs
                );
            }
        }
    }

    fn on_record(
        &self,
        _span: &span::Id,
        _values: &span::Record<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Not meaningfully implementable, as Android Tracing doesn't have a sense of changing the name partway through
        // TODO: We could use the same technique as on_exit, i.e. tear down the stack, and push it back up again
    }

    fn on_follows_from(
        &self,
        _span: &span::Id,
        _follows: &span::Id,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Not meaningfully implementable
    }

    fn on_event(&self, _event: &tracing::Event<'_>, _: tracing_subscriber::layer::Context<'_, S>) {
        // TODO: Does it make sense to do anything here?

        // if self.is_enabled {
        //     let mut name = String::new();
        //     event.record(&mut self.fmt_fields.make_visitor(Writer::new(&mut name)));

        //     let name = CString::new(name);
        //     match name {
        //         Ok(name) => {
        //             self.trace.begin_section(&name);
        //             self.trace.end_section();
        //         }
        //         Err(e) => eprintln!(
        //             concat!(
        //                 "[tracing_android_trace] Unable to format the following ",
        //                 "event due to a null byte ({:?}), ignoring: {:?}",
        //             ),
        //             e, event
        //         ),
        //     }
        // }
    }

    fn on_enter(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        // The extension is optional in case tracing is disabled
        if let Some(ext) = extensions.get::<ATraceExtension>() {
            self.trace.begin_section(&ext.name);
            let stack = self.current_actual_stack.get_or_default();
            stack.borrow_mut().push(Some(id.clone()));
        }
    }

    fn on_exit(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // For some reason, the `S`'s `exit` method is called *before* on_exit, so the span which is exiting is
        // no longer in the current stack
        // Because of this, to find the place it *used* to be, we find the item which was the parent of the current item
        // in the stack
        let this_span = ctx.span(id).expect("Span not found, this is a bug");
        let Some(stack) = self.current_actual_stack.get() else {
            // No spans had the extension, so nothing to do
            return;
        };
        let mut stack = stack.borrow_mut();
        if stack.is_empty() {
            let extensions = this_span.extensions();
            debug_assert!(extensions.get::<ATraceExtension>().is_none());
            return;
        }
        let last = stack.last().unwrap().as_ref();
        debug_assert!(last.is_some());
        if last == Some(id) {
            stack.pop();
            // Fast path, if we were at the top of the stack (i.e. the current top is our parent)
            // Matches the call in `on_enter`
            self.trace.end_section();
            #[cfg(debug_assertions)]
            {
                let extensions = this_span.extensions();
                debug_assert!(extensions.get::<ATraceExtension>().is_some());
            }
            // Clear all the dangling items on the stack
            while let Some(None) = stack.last() {
                stack.pop();
                self.trace.end_section();
            }
        } else {
            // We need to handle the case where span opening and closing is interleaved
            // E.g. open A, open B, close A, close B
            //
            // We model this by effectively keeping A open until B is closed, but with a new name
            // of EXTRA_STR - currently `_`
            const EXTRA_STR: &CStr = c"_";

            let mut index_of_this = None;
            for (idx, item) in stack.iter_mut().enumerate().rev() {
                self.trace.end_section();
                if item.as_ref() == Some(id) {
                    index_of_this = Some(idx);
                    *item = None;
                    break;
                }
            }

            let Some(index_of_this) = index_of_this else {
                eprintln!(
                    "Didn't find span in the current stack. Maybe a span was sent between threads?",
                );
                return;
            };
            for id in stack[index_of_this..].iter() {
                if let Some(id) = id {
                    let span = ctx.span(id).expect("Span not found, this is a bug");
                    let extensions = span.extensions();
                    if let Some(ext) = extensions.get::<ATraceExtension>() {
                        self.trace.begin_section(&ext.name);
                    } else {
                        eprintln!("Unexpectedly had item in stack without ATraceExtension");
                    }
                } else {
                    self.trace.begin_section(EXTRA_STR);
                }
            }
        }
    }
}

// TODO: pub struct ATraceCounterLayer {}
