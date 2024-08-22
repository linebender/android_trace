<div align="center" class="rustdoc-hidden">

# Tracing Android Trace

</div>

<div align="center">

**Write [`tracing`][] spans to Android [NDK Tracing][]**

[![Latest published version.](https://img.shields.io/crates/v/tracing_android_trace.svg)](https://crates.io/crates/tracing_android_trace)
[![Documentation build status.](https://img.shields.io/docsrs/tracing_android_trace.svg)](https://docs.rs/tracing_android_trace)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
[![tracing-subscriber version](https://img.shields.io/badge/tracing--subscriber-v0.3.18-a674e5.svg)](https://crates.io/crates/tracing-subscriber)
\
[![Linebender Zulip chat.](https://img.shields.io/badge/Linebender-%23general%20%3E%20Android%20Tracing-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing)
[![GitHub Actions CI status.](https://img.shields.io/github/actions/workflow/status/linebender/android_trace/ci.yml?logo=github&label=CI)](https://github.com/linebender/android_trace/actions)
[![Dependency staleness status.](https://deps.rs/crate/tracing_android_trace/latest/status.svg)](https://deps.rs/crate/tracing_android_trace)

</div>

⚠️ Tracing Android Trace only supports Android

Tracing Android Trace provides several [`tracing_subscriber::Layer`][]s for Android NDK Tracing, using `ATrace_beginSection` and `ATrace_endSection`.
This allows viewing spans created using the [`tracing`][] macros in [Android GPU Inspector](https://gpuinspector.dev/).

Note that this does not currently support `tracing` *events*, only spans.
This limitation is due to the underlying Android platform APIs.

<figure>
<img src="https://github.com/linebender/android_trace/assets/36049421/a7f03b74-d690-42be-91b5-326fbb698a03" alt="Screenshot showing a thread timeline including spans of a single thread.">
<figcaption>

Tracing spans for [Vello](https://github.com/linebender/vello) shown in Android GPU Inspector
</figcaption>
</figure>

Significant changes are documented in [the changelog][].

## Quickstart

Add a dependency on Android Trace (and on [`tracing_subscriber`][]).

```toml
[target.'cfg(target_os = "android")'.dependencies]
tracing_android_trace = "0.1.0"
```

You can then add an Android Tracing layer to the registry subscriber:

```rust,no_run
use tracing_subscriber::prelude::*;

fn main() {
  tracing_subscriber::registry()
    .with(tracing_android_trace::AndroidTraceLayer::new())
    .try_init()
    .unwrap();
}
```

## Available Layers

[NDK Tracing][] supports three kinds of tracing, with different API level requirements.

### Thread-Matched sections

The first API added, which is useful for tracking time spent in a thread, was `ATrace_beginSection` and `ATrace_endSection`.
This has been available since Android API level 23.

[`AndroidTraceLayer`][] uses this API, and is the preferred layer from this crate - it was used to produce the screenshot above.

Note that if entering and exiting of spans are interleaved, this layer will produce discontinuous traces.
This is required to work around the limitations of the NDK API.
See the documentation on the layer for more details.

### Async

This crate also includes an async layer [`AndroidTraceAsyncLayer`][], which uses `ATrace_beginAsyncSection` and `ATrace_endAsyncSection`.
It is recommended to [filter][tracing_subscriber::filter] the use of this API to only your async tasks.
See the documentation on the layer for an example of how to do so.

This is necessary because Android Tracing does not allow async tasks to be associated with each other.
This means that each task will be shown in their own line in the trace, which is rarely a useful UI.
It is also recommended to not associate any fields with these spans, as lines in the trace will not be re-used.

### Counters

The underlying API also supports setting counter values, however this is not yet implemented.
If you require this, please open an issue.

## Android API levels

This crate uses [`android_trace`][] to call the NDK functions.
Therefore, this crate can support any Android API level on the target device, although by default it requires an API level of 23 (corresponding to Android 6, codename Marshmallow, released in 2015).

To support Android API versions less than 23, you should disable default features:

```toml
[target.'cfg(target_os = "android")'.dependencies]
tracing_android_trace = { version = "0.1.0", default-features = false }
```

## Crate feature flags

The following feature flags are available:

* `api_level_23` (enabled by default): Require Android API level 23, to avoid some runtime symbol resolution
* `api_level_29`: Require Android API level 29, disabling runtime symbol resolution entirely

## Minimum supported Rust Version (MSRV)

This version of Tracing Android Trace has been verified to compile with **Rust 1.77** and later.

Future versions of Tracing Android Trace might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of Tracing Android Trace's dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```

</details>

<!-- We hide these elements when viewing in Rustdoc, because they're not expected to be present in crate level docs -->
<div class="rustdoc-hidden">

## Community

Discussion of Android Trace development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically in
[#general > Android Tracing](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing).
All public content can be read without logging in.

Contributions are welcome by pull request. The [Rust code of conduct][] applies.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE][] or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT][] or <http://opensource.org/licenses/MIT>)

at your option.

</div>

[NDK Tracing]: https://developer.android.com/ndk/reference/group/tracing
[`android_trace`]: https://crates.io/crates/android_trace
[the changelog]: https://github.com/linebender/android_trace/blob/main/CHANGELOG.md
[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT

<!-- Replacement intra-doc links for GitHub and crates.io. See https://linebender.org/blog/doc-include -->
[`tracing`]: https://docs.rs/tracing/latest/tracing/
[tracing_subscriber::filter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/index.html
[`tracing_subscriber`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber
[`tracing_subscriber::Layer`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/trait.Layer.html
[`AndroidTraceLayer`]: https://docs.rs/tracing_android_trace/latest/tracing_android_trace/sync_layer/struct.AndroidTraceLayer.html
[`AndroidTraceAsyncLayer`]: https://docs.rs/tracing_android_trace/latest/tracing_android_trace/async_layer/struct.AndroidTraceAsyncLayer.html
