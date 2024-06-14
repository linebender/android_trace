// Copyright 2024 the Android Trace Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! [`tracing`]: tracing
//! [tracing_subscriber::filter]: tracing_subscriber::filter
//! [`tracing_subscriber`]: tracing_subscriber
//! [`tracing_subscriber::Layer`]: tracing_subscriber::Layer
//! [`AndroidTraceLayer`]: AndroidTraceLayer
//! [`AndroidTraceAsyncLayer`]: AndroidTraceAsyncLayer
//! [`android_trace`]: android_trace
//!
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
//!
// Hide the header section of the README when using rustdoc
//! <div style=\"display:none\">
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[cfg(not(target_os = "android"))]
compile_error!(
    r#"tracing_android_trace only supports Android. If you are depending on it, ensure that it is within
[target.'cfg(target_os = "android")'.dependencies]
in your Cargo.toml"#
);

#[cfg(target_os = "android")]
pub use android_trace;

#[cfg(target_os = "android")]
mod async_layer;
#[cfg(target_os = "android")]
pub use async_layer::AndroidTraceAsyncLayer;

#[cfg(target_os = "android")]
mod sync_layer;
#[cfg(target_os = "android")]
pub use sync_layer::AndroidTraceLayer;

// TODO: pub use some_mod::ATraceCounterLayer;
