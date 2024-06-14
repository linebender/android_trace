<div align="center">

# Android Trace

**Support for Android [NDK Tracing][] in Rust**

[![Linebender Zulip](https://img.shields.io/badge/Linebender-%23general%20%3E%20Android%20Tracing-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/197075-gpu)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![Build status](https://github.com/linebender/android_trace/workflows/CI/badge.svg)](https://github.com/linebender/android_trace/actions)
[![Dependency staleness status.](https://deps.rs/repo/github/linebender/android_trace/status.svg)](https://deps.rs/repo/github/linebender/android_trace)

</div>

⚠️ Android Trace only support Android

This repository contains two crates for working with Android NDK Tracing.
The most useful of these is likely to be the higher level Tracing Android Trace,
which depends on the lower level Android Trace.

<figure>
<img src="https://github.com/linebender/android_trace/assets/36049421/a7f03b74-d690-42be-91b5-326fbb698a03" alt="Screenshot showing a thread timeline including spans of a single thread.">
<figcaption>

Tracing spans for [Vello](https://github.com/linebender/vello) shown in Android GPU Inspector, created using Tracing Android Trace.
</figcaption>
</figure>

Significant changes are documented in [the changelog].

## Tracing Android Trace [tracing_android_trace](./tracing_android_trace)

Tracing Android Trace provides several [`tracing_subscriber::Layer`][]s for Android NDK Tracing, using `ATrace_beginSection` and `ATrace_endSection`.
This allows viewing spans created using the [`tracing`][] macros in [Android GPU Inspector](https://gpuinspector.dev/).

[![Latest published version.](https://img.shields.io/crates/v/tracing_android_trace.svg)](https://crates.io/crates/tracing_android_trace)
[![Documentation build status.](https://img.shields.io/docsrs/tracing_android_trace.svg)](https://docs.rs/tracing_android_trace)
[![Dependency staleness status.](https://deps.rs/crate/tracing_android_trace/latest/status.svg)](https://deps.rs/crate/tracing_android_trace)
[![tracing-subscriber version](https://img.shields.io/badge/tracing--subscriber-v0.3.18-a674e5.svg)](https://crates.io/crates/tracing-subscriber)

## Android Trace [android_trace](./android_trace)

[![Latest published version.](https://img.shields.io/crates/v/android_trace.svg)](https://crates.io/crates/android_trace)
[![Documentation build status.](https://img.shields.io/docsrs/android_trace.svg)](https://docs.rs/android_trace)
[![Dependency staleness status.](https://deps.rs/crate/android_trace/latest/status.svg)](https://deps.rs/crate/android_trace)

Android Trace provides access to the Android NDK Tracing methods, such as `ATrace_beginSection` and `ATrace_endSection`.

## Community

Discussion of Android Trace development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically in
[#general > Android Tracing](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing).
All public content can be read without logging in.

Contributions are welcome by pull request. The [Rust code of conduct][] applies.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`tracing`]: https://docs.rs/tracing/latest/tracing/
[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
[NDK Tracing]: https://developer.android.com/ndk/reference/group/tracing
[`tracing_subscriber::Layer`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/trait.Layer.html
[the changelog]: CHANGELOG.md
