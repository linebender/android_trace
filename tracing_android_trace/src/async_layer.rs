use std::ffi::CString;

use android_trace::AndroidTrace;
use tracing::span;
use tracing_subscriber::{
    fmt::{
        format::{DefaultFields, Writer},
        FormatFields,
    },
    registry::LookupSpan,
};

pub struct ATraceLayerAsync {
    trace: AndroidTrace,
    fmt_fields: DefaultFields,
}

impl ATraceLayerAsync {
    pub fn new() -> Self {
        let trace = AndroidTrace::new();
        Self::with_trace(trace)
    }

    pub fn with_trace(trace: AndroidTrace) -> Self {
        ATraceLayerAsync {
            trace,
            fmt_fields: DefaultFields::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ATraceExtensionAsync {
    name: CString,
    cookie: i32,
}

impl<S> tracing_subscriber::Layer<S> for ATraceLayerAsync
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
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
                    Ok(name) => {
                        // We truncate on purpose here. The scenario where this breaks are rare
                        let cookie = id.into_u64() as u32 as i32;
                        extensions
                            .insert::<ATraceExtensionAsync>(ATraceExtensionAsync { name, cookie });
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
        if let Some(ext) = extensions.get::<ATraceExtensionAsync>() {
            self.trace.begin_async_section(&ext.name, ext.cookie);
        }
    }

    fn on_exit(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        if let Some(ext) = extensions.get::<ATraceExtensionAsync>() {
            // Matches the call in `on_enter`
            self.trace.end_async_section(&ext.name, ext.cookie);
        }
    }
}

// TODO: pub struct ATraceCounterLayer {}
