#![doc = concat!(
    // TODO: Is this a new pattern?
    "[AndroidTrace]: crate::AndroidTrace
    [dlsym]: libc::dlsym
    [`begin_section_try_async`]: AndroidTrace::begin_section_try_async
    [`end_section_try_async`]: AndroidTrace::end_section_try_async
    
    <!-- Hide the header section of the README when using rustdoc -->
    <div style=\"display:none\">
    ",
        include_str!("../README.md"),
    )]
#![warn(
    unreachable_pub,
    clippy::doc_markdown,
    clippy::semicolon_if_nothing_returned
)]
#![forbid(unsafe_code)]

#[cfg(not(target_os = "android"))]
compile_error!(
    r#"tracing_android_trace only supports Android. If you are depending on it, ensure that it is within
[target.'cfg(target_os = "android")'.dependencies]
in your Cargo.toml"#
);

use std::{
    collections::HashMap,
    ffi::CString,
    sync::{
        atomic::{self, AtomicI32},
        RwLock,
    },
};

pub use android_trace;
use android_trace::AndroidTrace;
use tracing::{span, subscriber::Interest};
use tracing_core::callsite::Identifier;
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
    cookies: RwLock<HashMap<Identifier, AtomicI32>>,
}

impl ATraceLayer {
    pub fn new() -> Self {
        let trace = AndroidTrace::new();
        Self::with_trace(trace)
    }
    pub fn with_trace(trace: AndroidTrace) -> Self {
        ATraceLayer {
            trace,
            fmt_fields: DefaultFields::new(),
            cookies: RwLock::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ATraceExtension {
    name: CString,
    cookie: i32,
}

impl<S> tracing_subscriber::Layer<S> for ATraceLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn register_callsite(
        &self,
        metadata: &'static tracing::Metadata<'static>,
    ) -> tracing::subscriber::Interest {
        let id = metadata.callsite();
        {
            let mut guard = self
                .cookies
                .write()
                .expect("Putting Identifier into HashMap paniced. This shouldn't be possible");
            match guard.insert(id, AtomicI32::new(0)) {
                Some(mut v) => eprintln!(
                    "Got unexpected existing cookie, which had value {}",
                    v.get_mut()
                ),
                None => (),
            };
        }

        Interest::always()
    }
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
                    Ok(name) => {
                        let id = span.metadata().callsite();
                        let cookie = {
                            let guard = self.cookies.read().expect("Cookies cannot be poisoned");
                            let jar = guard.get(&id).unwrap();
                            jar.fetch_add(1, atomic::Ordering::Relaxed)
                        };
                        extensions.insert::<ATraceExtension>(ATraceExtension { name, cookie })
                    }
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
        } else {
            eprintln!("Tracing reported as disabled2");
        }
    }

    fn on_record(
        &self,
        _span: &span::Id,
        _values: &span::Record<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Not meaningfully implementable, as Android Tracing doesn't have a sense of changing the name partway through
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
        // TODO: Does it maextke sense to do anything here?

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
        if let Some(ext) = extensions.get::<ATraceExtension>() {
            // TODO: Debug:
            // self.trace.begin_section_try_async(&ext.name, ext.cookie);
            self.trace.begin_section(&ext.name);
        }
    }

    fn on_exit(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        if let Some(_ext) = extensions.get::<ATraceExtension>() {
            // Matches the call in `on_enter`
            // self.trace.end_section_try_async(&ext.name, ext.cookie);
            self.trace.end_section();
        }
    }
}

// TODO: pub struct ATraceCounterLayer {}
