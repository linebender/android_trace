// Copyright 2024 the Android Trace Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// https://linebender.org/blog/doc-include
//! [`tracing`]: tracing
//! [tracing_subscriber::filter]: tracing_subscriber::filter
//! [`tracing_subscriber`]: tracing_subscriber
//! [`tracing_subscriber::Layer`]: tracing_subscriber::Layer
//! [`AndroidTraceLayer`]: AndroidTraceLayer
//! [`AndroidTraceAsyncLayer`]: AndroidTraceAsyncLayer
//! [`android_trace`]: android_trace
// File links are not supported by rustdoc
//! [LICENSE-APACHE]: https://github.com/linebender/android_trace/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/linebender/android_trace/blob/main/LICENSE-MIT
//!
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
// LINEBENDER LINT SET - v1
// See https://linebender.org/wiki/canonical-lints/
// These lints aren't included in Cargo.toml because they
// shouldn't apply to examples and tests
#![warn(unused_crate_dependencies)]
#![warn(clippy::print_stdout, clippy::print_stderr)]

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
