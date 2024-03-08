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
    clippy::semicolon_if_nothing_returned,
    unsafe_op_in_unsafe_fn
)]

#[cfg(not(feature = "api_level_23"))]
use ffi::ATraceAPILevel23Methods;
#[cfg(not(feature = "api_level_29"))]
use ffi::ATraceAPILevel29Methods;

use core::ffi::CStr;
use std::fmt::Debug;

#[cfg(not(target_os = "android"))]
compile_error!(
    r#"android_trace only supports Android. If you are depending on it, ensure that it is within
    [target.'cfg(target_os = "android")'.dependencies]
    in your Cargo.toml"#
);

mod ffi;

/// A handle to the available NDK tracing functions
///
/// All access is thread safe
#[derive(Clone)]
pub struct AndroidTrace {
    #[cfg(not(feature = "api_level_23"))]
    api_level_23: Option<&'static ATraceAPILevel23Methods>,
    #[cfg(not(feature = "api_level_29"))]
    api_level_29: Option<&'static ATraceAPILevel29Methods>,
}

impl AndroidTrace {
    /// Get a handle to all of the NDK tracing functions available on this device.
    ///
    /// This should be expected to have a low runtime cost.
    ///
    /// Can subsequently be used across multiple threads
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "api_level_23"))]
            api_level_23: ATraceAPILevel23Methods::get(),
            #[cfg(not(feature = "api_level_29"))]
            api_level_29: ATraceAPILevel29Methods::get(),
        }
    }

    /// Get a handle to the first level of the NDK tracing functions, where available
    /// on this device.
    ///
    /// This should be expected to have a low runtime cost.
    pub fn new_downlevel() -> Self {
        Self {
            #[cfg(not(feature = "api_level_23"))]
            api_level_23: ATraceAPILevel23Methods::get(),
            #[cfg(not(feature = "api_level_29"))]
            api_level_29: None,
        }
    }

    /// Writes a tracing message to indicate that the given section of code has begun.
    /// If the same cookie is used, this can re-open a previously closed section.
    /// This is especially useful for tracking the same async task.
    ///
    /// This should be followed by a call to [`Self::end_section_try_async`] on the same thread, to close the
    /// opened section.
    ///
    /// This calls [`Self::begin_async_section`], but if that fails to call into the NDK, it instead
    /// calls [`Self::begin_section`].
    pub fn begin_section_try_async(&self, section_name: &CStr, cookie: i32) {
        if self.begin_async_section(section_name, cookie).is_none() {
            dbg!("Trying not async");
            self.begin_section(section_name);
        }
    }

    /// Writes a tracing message to indicate that the given section of code has begun.
    ///
    /// This should follow a call to [`Self::begin_section_try_async`] on the same thread.
    pub fn end_section_try_async(&self, section_name: &CStr, cookie: i32) {
        if self.end_async_section(section_name, cookie).is_none() {
            self.end_section();
        }
    }

    /// Returns Some(true) if tracing through Android Trace is enabled (and Some(false) if it is disabled).
    ///
    /// Calls [`ATrace_isEnabled`](https://developer.android.com/ndk/reference/group/tracing#atrace_isenabled)
    /// if available. This is only available since Android API level 23. If the `api_level_23` feature is not
    /// enabled, this will attempt to access a dynamically linked version of the underlying function.
    /// Please note that `api_level_23` is a default feature.
    ///
    /// If `ATrace_isEnabled` is not available, returns None.
    #[doc(alias = "ATrace_isEnabled")]
    #[must_use = "Detecting if tracing is enabled has no side effects"]
    pub fn is_enabled(&self) -> Option<bool> {
        // SAFETY: No preconditions
        #[cfg(feature = "api_level_23")]
        unsafe {
            Some(ffi::atrace_is_enabled_raw())
        }
        #[cfg(not(feature = "api_level_23"))]
        if let Some(methods) = self.api_level_23 {
            // Safety: No preconditions
            let result = unsafe { (methods.is_enabled)() };
            Some(result)
        } else {
            None
        }
    }

    /// Writes a tracing message to indicate that the given section of code has begun.
    ///
    /// This should be followed by a call to [`Self::end_section`] on the same thread, to close the
    /// opened section.
    ///
    /// Calls [`ATrace_beginSection`](https://developer.android.com/ndk/reference/group/tracing#atrace_beginsection)
    /// if available. This is only available since Android API level 23. If the `api_level_23` feature is not
    /// enabled, this will attempt to access a dynamically linked version of the underlying function.
    /// Please note that `api_level_23` is a default feature.
    ///
    /// If `ATrace_beginSection` is not available, this has no effect.
    #[doc(alias = "ATrace_beginSection")]
    pub fn begin_section(&self, section_name: &CStr) {
        #[cfg(feature = "api_level_23")]
        unsafe {
            // SAFETY: section_name is a valid C string
            ffi::atrace_begin_section_raw(section_name.as_ptr());
        }
        #[cfg(not(feature = "api_level_23"))]
        if let Some(methods) = self.api_level_23 {
            // SAFETY: section_name is a valid C string
            unsafe { (methods.begin_section)(section_name.as_ptr()) }
        }
    }

    /// Writes a tracing message to indicate that a given section of code has ended.
    ///
    /// This should follow a call to [`Self::begin_section`] on the same thread, as
    /// this will be closing an opened section.
    ///
    /// Calls [`ATrace_endSection`](https://developer.android.com/ndk/reference/group/tracing#atrace_endsection)
    /// if available. This is only available since Android API level 23. If the `api_level_23` feature is not
    /// enabled, this will attempt to access a dynamically linked version of the underlying function.
    /// Please note that `api_level_23` is a default feature
    ///
    /// If `ATrace_endSection` is not available, this has no effect.
    #[doc(alias = "ATrace_endSection")]
    pub fn end_section(&self) {
        // SAFETY: No preconditions.
        #[cfg(feature = "api_level_23")]
        unsafe {
            ffi::atrace_end_section_raw();
        }
        #[cfg(not(feature = "api_level_23"))]
        if let Some(methods) = self.api_level_23 {
            // Safety: No preconditions
            unsafe { (methods.end_section)() }
        }
    }

    /// Writes a tracing message to indicate that a given section of code has begun.
    ///
    /// This should be followed by a call to [`Self::end_async_section`] with the same `section_name` and `cookie`,
    /// although this subsequent call can occur on any thread.
    ///
    /// Calls [`ATrace_beginAsyncSection`](https://developer.android.com/ndk/reference/group/tracing#atrace_beginasyncsection)
    /// if available. This is only available since Android API level 29. If the `api_level_29` feature is not
    /// enabled, this will attempt to access a dynamically linked version of the underlying function.
    ///
    /// If `ATrace_beginAsyncSection` is not available, this has no effect.
    #[doc(alias = "ATrace_beginAsyncSection")]
    pub fn begin_async_section(&self, section_name: &CStr, cookie: i32) -> Option<()> {
        // SAFETY: No preconditions.
        #[cfg(feature = "api_level_29")]
        unsafe {
            ffi::atrace_begin_async_section_raw(section_name.as_ptr(), cookie);
            Some(())
        }
        #[cfg(not(feature = "api_level_29"))]
        if let Some(methods) = self.api_level_29 {
            // Safety: No preconditions
            unsafe { (methods.begin_async_section)(section_name.as_ptr(), cookie) }
            Some(())
        } else {
            None
        }
    }

    /// Writes a tracing message to indicate that a given section of code has ended
    ///
    /// This should follow a call to [`Self::begin_async_section`] with the same `section_name` and `cookie`,
    /// although this call can occur on any thread
    ///
    /// Calls [`ATrace_endAsyncSection`](https://developer.android.com/ndk/reference/group/tracing#atrace_endasyncsection)
    /// if available. This is only available since Android API level 29. If the `api_level_29` feature is not
    /// enabled, this will attempt to access a dynamically linked version of the underlying function.
    ///
    /// If `ATrace_endAsyncSection` is not available, this has no effect
    #[doc(alias = "ATrace_endAsyncSection")]
    pub fn end_async_section(&self, section_name: &CStr, cookie: i32) -> Option<()> {
        // SAFETY: No preconditions.
        #[cfg(feature = "api_level_29")]
        unsafe {
            ffi::atrace_end_async_section_raw(section_name.as_ptr(), cookie);
            Some(())
        }
        #[cfg(not(feature = "api_level_29"))]
        if let Some(methods) = self.api_level_29 {
            // Safety: No preconditions
            unsafe { (methods.end_async_section)(section_name.as_ptr(), cookie) }
            Some(())
        } else {
            None
        }
    }

    pub fn could_set_counter(&self) -> bool {
        #[cfg(not(feature = "api_level_29"))]
        return self.api_level_29.is_some();
        #[cfg(feature = "api_level_29")]
        true
    }

    /// Writes a trace message to indicate that the counter with the given name has the given value.
    ///
    /// Calls [`ATrace_setCounter`](https://developer.android.com/ndk/reference/group/tracing#atrace_setcounter)
    /// if available. This is only available since Android API level 29. If the `api_level_29` feature is not
    /// enabled, this will attempt to access a dynamically linked version of the underlying function.
    ///
    /// If `ATrace_endAsyncSection` is not available, this has no effect
    #[doc(alias = "ATrace_setCounter")]
    pub fn set_counter(&self, counter_name: &CStr, value: i64) -> Option<()> {
        // SAFETY: No preconditions.
        #[cfg(feature = "api_level_29")]
        unsafe {
            ffi::atrace_set_counter_raw(counter_name.as_ptr(), value);
            Some(())
        }
        #[cfg(not(feature = "api_level_29"))]
        if let Some(methods) = self.api_level_29 {
            // Safety: No preconditions
            unsafe { (methods.set_counter)(counter_name.as_ptr(), value) }
            Some(())
        } else {
            None
        }
    }
}

impl Default for AndroidTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for AndroidTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "api_level_23"))]
        let has_level_23 = self.api_level_23.is_some();
        #[cfg(feature = "api_level_23")]
        let has_level_23 = true;
        #[cfg(not(feature = "api_level_29"))]
        let has_level_29 = self.api_level_29.is_some();
        #[cfg(feature = "api_level_29")]
        let has_level_29 = true;
        let api_level = match (has_level_29, has_level_23) {
            (true, true) => "29",
            (false, true) => "23",
            (false, false) => "None",
            (true, false) => "Unexpected: 29 but not 23",
        };
        f.debug_struct("AndroidTrace")
            .field("api_level", &api_level)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use static_assertions as sa;
    sa::assert_impl_all!(AndroidTrace: Send, Sync);
}
