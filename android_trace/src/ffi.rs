// Copyright 2024 the Android Trace Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(not(all(feature = "api_level_23", feature = "api_level_29")))]
use core::{
    ffi::{c_void, CStr},
    mem,
};

use core::ffi::c_char;

/// # Safety
///
/// `func` must have been produced from a call to `dlsym` which is
/// reasonably expected to have the right type.
///
/// All the preconditions from [`transmute_copy`](core::mem::transmute_copy) apply.
#[cfg(not(all(feature = "api_level_23", feature = "api_level_29")))]
#[allow(
    unused_qualifications,
    // reason = "These qualifications are used, because our MSRV is lower than 1.81"
)]
unsafe fn transmute_if_not_null<F>(func: *mut c_void) -> Option<F> {
    assert_eq!(
        mem::size_of::<F>(),
        mem::size_of::<*mut c_void>(),
        "transmute_copy is used because this function is generic"
    );
    if func.is_null() {
        return None;
    }
    // Safety: The preconditions are guaranteed by the caller.
    Some(unsafe { mem::transmute_copy::<*mut c_void, F>(&func) })
}

#[link(name = "android", kind = "dylib")]
#[cfg(target_os = "android")]
#[cfg(feature = "api_level_23")]
extern "C" {
    #[link_name = "ATrace_beginSection"]
    /// <https://developer.android.com/ndk/reference/group/tracing#atrace_beginsection>
    pub(crate) fn atrace_begin_section_raw(section_name: *const c_char);

    #[link_name = "ATrace_endSection"]
    /// <https://developer.android.com/ndk/reference/group/tracing#atrace_endsection>
    pub(crate) fn atrace_end_section_raw();

    #[link_name = "ATrace_isEnabled"]
    /// <https://developer.android.com/ndk/reference/group/tracing#atrace_isenabled>
    pub(crate) fn atrace_is_enabled_raw() -> bool;
}

#[cfg(not(feature = "api_level_23"))]
pub(crate) struct ATraceAPILevel23Methods {
    pub(crate) begin_section: unsafe extern "C" fn(*const c_char),
    pub(crate) end_section: unsafe extern "C" fn(),
    pub(crate) is_enabled: unsafe extern "C" fn() -> bool,
}

// Link to Android in case the api_level_23 is disabled (i.e. we don't have the extern block above)
// SAFETY: This is required for the calls to dlsym to be safe, ensuring that the accessed methods
// don't get unlinked
#[link(name = "android", kind = "dylib")]
extern "C" {}

#[cfg(not(feature = "api_level_23"))]
impl ATraceAPILevel23Methods {
    pub(crate) fn get() -> Option<&'static Self> {
        use libc::RTLD_DEFAULT;
        use std::sync::OnceLock;

        static API_LEVEL_23_METHODS: OnceLock<Option<ATraceAPILevel23Methods>> = OnceLock::new();
        API_LEVEL_23_METHODS
            .get_or_init(|| {
                let is_enabled = unsafe {
                    const IS_ENABLED_NAME: &CStr = c"ATrace_isEnabled";
                    // Safety: We're on Android, and have definitely linked to libandroid, so this function
                    // should have the expected signature if present
                    transmute_if_not_null(libc::dlsym(RTLD_DEFAULT, IS_ENABLED_NAME.as_ptr()))?
                };
                let begin_section = unsafe {
                    const BEGIN_SECTION_NAME: &CStr = c"ATrace_beginSection";
                    // Safety: As above
                    transmute_if_not_null(libc::dlsym(RTLD_DEFAULT, BEGIN_SECTION_NAME.as_ptr()))?
                };
                let end_section = unsafe {
                    const END_SECTION_NAME: &CStr = c"ATrace_endSection";
                    // Safety: As above
                    transmute_if_not_null(libc::dlsym(RTLD_DEFAULT, END_SECTION_NAME.as_ptr()))?
                };
                Some(Self {
                    is_enabled,
                    begin_section,
                    end_section,
                })
            })
            .as_ref()
    }
}

#[link(name = "android", kind = "dylib")]
#[cfg(target_os = "android")]
#[cfg(feature = "api_level_29")]
extern "C" {
    #[link_name = "ATrace_beginAsyncSection"]
    /// <https://developer.android.com/ndk/reference/group/tracing#atrace_beginasyncsection>
    pub(crate) fn atrace_begin_async_section_raw(section_name: *const c_char, cookie: i32);

    #[link_name = "ATrace_endAsyncSection"]
    /// <https://developer.android.com/ndk/reference/group/tracing#atrace_endasyncsection>
    pub(crate) fn atrace_end_async_section_raw(section_name: *const c_char, cookie: i32);

    #[link_name = "ATrace_setCounter"]
    //<https://developer.android.com/ndk/reference/group/tracing#atrace_setcounter>
    pub(crate) fn atrace_set_counter_raw(counter_name: *const c_char, counter_value: i64);
}

#[cfg(not(feature = "api_level_29"))]
pub(crate) struct ATraceAPILevel29Methods {
    pub(crate) begin_async_section: unsafe extern "C" fn(*const c_char, i32),
    pub(crate) end_async_section: unsafe extern "C" fn(*const c_char, i32),
    pub(crate) set_counter: unsafe extern "C" fn(*const c_char, i64),
}

#[cfg(not(feature = "api_level_29"))]
impl ATraceAPILevel29Methods {
    pub(crate) fn get() -> Option<&'static Self> {
        use libc::RTLD_DEFAULT;
        use std::sync::OnceLock;

        static API_LEVEL_29_METHODS: OnceLock<Option<ATraceAPILevel29Methods>> = OnceLock::new();
        API_LEVEL_29_METHODS
            .get_or_init(|| {
                let set_counter = unsafe {
                    const SET_COUNTER_NAME: &CStr = c"ATrace_setCounter";
                    // Safety: We're on Android, and have definitely linked to libandroid, so this function
                    // should have the expected signature if present
                    transmute_if_not_null(libc::dlsym(RTLD_DEFAULT, SET_COUNTER_NAME.as_ptr()))?
                };
                let begin_async_section = unsafe {
                    const BEGIN_SECTION_NAME: &CStr = c"ATrace_beginAsyncSection";
                    // Safety: As above
                    transmute_if_not_null(libc::dlsym(RTLD_DEFAULT, BEGIN_SECTION_NAME.as_ptr()))?
                };
                let end_async_section = unsafe {
                    const END_SECTION_NAME: &CStr = c"ATrace_endAsyncSection";
                    // Safety: As above
                    transmute_if_not_null(libc::dlsym(RTLD_DEFAULT, END_SECTION_NAME.as_ptr()))?
                };
                Some(Self {
                    set_counter,
                    begin_async_section,
                    end_async_section,
                })
            })
            .as_ref()
    }
}
