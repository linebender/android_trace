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

/// A [`tracing_subscriber::Layer`] which uses [`ATrace_beginAsyncSection`](AndroidTrace::begin_async_section)
/// and [`ATrace_endAsyncSection`](AndroidTrace::end_async_section)
///
/// This requires the host device to support Android API level 29, although if
/// this level is not available, this layer has no effect.
/// This does have some (small) runtime overhead, which can be avoided if the `api_level_29`
/// feature is used (removing the graceful handling of lower API levels)
/// See the [crate level documentation](crate#android-api-levels) for more.
///
/// It is recommended to use this layer with a suitable [`tracing_subscriber::filter`] to only
/// target your desired async tasks, as each async task name will have a different row in the
/// currently existing UIs for Android Tracing, which can be unwiedly
#[derive(Debug)]
pub struct AndroidTraceAsyncLayer {
    trace: AndroidTrace,
    fmt_fields: DefaultFields,
    could_use_api_level_29: bool,
}

impl AndroidTraceAsyncLayer {
    /// Create a `AndroidTraceAsyncLayer`
    pub fn new() -> Self {
        let trace = AndroidTrace::new();
        Self::with_trace(trace)
    }

    /// Create a `AndroidTraceAsyncLayer` from a pre-existing [`AndroidTrace`].
    /// This can avoid some minor synchronization costs if the `api_level_23` feature is disabled.
    ///
    /// Note that this takes ownership because `AndroidTrace` has a trivial `Clone`
    pub fn with_trace(trace: AndroidTrace) -> Self {
        let could_use_api_level_29 = trace.could_use_api_level_29();
        AndroidTraceAsyncLayer {
            trace,
            fmt_fields: DefaultFields::new(),
            could_use_api_level_29,
        }
    }
}

impl Default for AndroidTraceAsyncLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub(crate) struct ATraceExtensionAsync {
    name: CString,
    cookie: i32,
}

impl<S> tracing_subscriber::Layer<S> for AndroidTraceAsyncLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if self.could_use_api_level_29 && self.trace.is_enabled().unwrap_or(false) {
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
        // TODO: Does it make sense to do anything here?
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
