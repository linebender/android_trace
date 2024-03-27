use thread_local::ThreadLocal;

use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    fmt::Debug,
};

use android_trace::AndroidTrace;
use tracing::span::{self, Id};
use tracing_subscriber::{
    fmt::{
        format::{DefaultFields, Writer},
        FormatFields,
    },
    registry::LookupSpan,
};

/// A [`tracing_subscriber::Layer`] which uses [`ATrace_beginSection`](AndroidTrace::begin_section)
/// and [`ATrace_endSection`](AndroidTrace::end_section)
///
/// This requires the host device to support Android API level 23, although this can be
/// gracefully handled by disabling the `api_level_23` feature.
/// See the [crate level documentation](crate#android-api-levels) for more.
///
/// <figure>
/// <img src="https://github.com/DJMcNab/android_trace/assets/36049421/a7f03b74-d690-42be-91b5-326fbb698a03" alt="Screenshot showing a thread timeline including spans of a single thread">
/// <figcaption>
///
/// Tracing spans for [Vello](https://github.com/linebender/vello) shown in Android GPU Inspector, created using this layer
/// </figcaption>
/// </figure>
///
/// ## Usage
///
/// This should be used as a layer on top of the [`tracing_subscriber::Registry`].
/// ```no_run
/// # use tracing_subscriber::prelude::*;
///
/// fn main(){
///   tracing_subscriber::registry()
///     .with(tracing_android_trace::AndroidTraceLayer::new())
///     .try_init()
///     .unwrap();
/// }
/// ```
///
/// ## Caveats
///
/// [tracing] does not guarantee that spans are exited in a true stack.
/// This is mismatched with the assumptions made by `ATrace_beginSection` and `ATrace_endSection`.
/// To work around this, this layer tears down all spans "above" an exiting span, then re-opens them.
/// In this situation, we currently include a span with the name `_` to ensure that the child
/// spans appear continuous, but this strategy might change - feedback welcome.
///
/// This may lead to spurious gaps in a trace in the prescense of interleaved spans.
#[derive(Debug)]
pub struct AndroidTraceLayer {
    trace: AndroidTrace,
    fmt_fields: DefaultFields,
    current_actual_stack: ThreadLocal<RefCell<ThreadLocalData>>,
}

#[derive(Debug, Default)]
struct ThreadLocalData {
    stack: Vec<Option<Id>>,
    extra_unclosed_values: u32,
}

impl AndroidTraceLayer {
    /// Create a `AndroidTraceLayer`
    pub fn new() -> Self {
        let trace = AndroidTrace::new_downlevel();
        Self::with_trace(trace)
    }

    /// Create a `AndroidTraceLayer` from a pre-existing [`AndroidTrace`].
    /// This can avoid some minor synchronization costs if the `api_level_23` feature is disabled.
    ///
    /// Note that this takes ownership because `AndroidTrace` has a trivial `Clone`
    pub fn with_trace(trace: AndroidTrace) -> Self {
        AndroidTraceLayer {
            trace,
            fmt_fields: DefaultFields::new(),
            current_actual_stack: ThreadLocal::new(),
        }
    }
}

impl Default for AndroidTraceLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct ATraceExtension {
    name: CString,
}

impl<S> tracing_subscriber::Layer<S> for AndroidTraceLayer
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
    }

    fn on_enter(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        // The extension is optional in case tracing is disabled
        if let Some(ext) = extensions.get::<ATraceExtension>() {
            self.trace.begin_section(&ext.name);
            let stack = self.current_actual_stack.get_or_default();
            stack.borrow_mut().stack.push(Some(id.clone()));
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
        let mut data = stack.borrow_mut();
        let stack = &mut data.stack;
        if stack.is_empty() {
            // This span should have been already closed in the case where a previous span was not
            // found in the stack. This means that tracing was enabled then disabled
            let extensions = this_span.extensions();
            if extensions.get::<ATraceExtension>().is_some() {
                if data.extra_unclosed_values == 0 {
                    panic!("Internal error: The same span was exited twice?");
                } else {
                    data.extra_unclosed_values -= 1;
                }
            }
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
                // There are two cases where this could occur:
                // 1) The span was created *before* tracing was enabled, then tracing was on and then off, then the span exited
                // 2) The span was created then exited *after* tracing was disabled, but before all parent spans were exited
                //
                // In either case tracing is disabled
                let extra_values = stack
                    .len()
                    .try_into()
                    .expect("Shouldn't have more than u32::MAX depth of stack");
                stack.clear();
                data.extra_unclosed_values = extra_values;
                let extensions = this_span.extensions();
                debug_assert!(extensions.get::<ATraceExtension>().is_none());
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
