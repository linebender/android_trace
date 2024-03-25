<div align="center">

# Tracing Android Trace
</div>
<!-- Close the <div> opened in lib.rs for rustdoc, which hides the above title -->
</div>

<div align="center">

**Write [`tracing`] spans to Android [NDK Tracing]**

[![Linebender Zulip](https://img.shields.io/badge/Linebender-%23general%20%3E%20Android%20Tracing-orange?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![dependency status](https://deps.rs/crate/tracing_android_trace/latest/status.svg)](https://deps.rs/crate/tracing_android_trace)
[![tracing-subscriber version](https://img.shields.io/badge/tracing--subscriber-v0.3.18-a674e5.svg)](https://crates.io/crates/tracing-subscriber)

[![Crates.io](https://img.shields.io/crates/v/tracing_android_trace.svg)](https://crates.io/crates/tracing_android_trace)
[![Docs](https://docs.rs/tracing_android_trace/badge.svg)](https://docs.rs/tracing_android_trace)

</div>

⚠️ Tracing Android Trace only supports Android

Tracing Android Trace provides several [`tracing_subscriber::Layer`]s for Android NDK Tracing, using `ATrace_beginSection` and `ATrace_endSection`.
This allows viewing spans created using the [`tracing`] macros in [Android GPU Inspector](https://gpuinspector.dev/).

Note that this does not currently support `tracing` *events*, only spans.
This limitation is due to the underlying Android platform APIs.

<figure>
<img src="https://github.com/DJMcNab/android_trace/assets/36049421/a7f03b74-d690-42be-91b5-326fbb698a03" alt="Screenshot showing a thread timeline including spans of a single thread">
<figcaption>

Tracing spans for [Vello](https://github.com/linebender/vello) shown in Android GPU Inspector
</figcaption>
</figure>

## Quickstart

Add a dependency on Android Trace (and on [`tracing_subscriber`][tracing_subscriber]).

```toml
[target.'cfg(target_os = "android")'.dependencies]
android_trace = "0.1.0"
```

You can then add an Android Tracing layer to the registry subscriber:
```rust,no_run
use tracing_subscriber::prelude::*;

fn main(){ 
  tracing_subscriber::registry()
    .with(tracing_android_trace::AndroidTraceLayer::new())
    .try_init()
    .unwrap();
}
```

## Available Layers

[NDK Tracing] supports three kinds of tracing, with different API level requirements.

### Thread-Matched sections

The first API added, which is useful for tracking time spent in a thread, was `ATrace_beginSection` and `ATrace_endSection`.
This has been available since Android API level 23.

[`AndroidTraceLayer`] uses this API, and is the preferred layer from this crate - it was used to produce the screenshot above.

Note that if entering and exiting of spans are interleaved, this layer will produce discontinuous traces.
This is required to work around the limitations of the NDK API.
See the documentation on the layer for more details.

### Async

This crate also includes an async layer [`TODO: Name`], which uses `ATrace_beginAsyncSection` and `ATrace_endAsyncSection`.
It is recommended to [filter][tracing_subscriber::filter] the use of this API to only your async tasks.
See the documentation on the layer for an example of how to do so.

This is necessary because Android Tracing does not allow async tasks to be associated with each other.
This means that each task will be shown in their own line in the trace, which is rarely a useful UI.
It is also recommended to not associate any fields with these spans, as lines in the trace will not be re-used.

### Counters

The underlying API also supports setting counter values, however this is not yet implemented.
If you require this, please open an issue.

## Android API levels

This crate uses [`android_trace`] to call the NDK functions.
Therefore, this crate can support any Android API level on the target device, although by default it requires an API level of 23 (corresponding to Android 6, codename Marshmallow, released in 2015).

To support Android API versions less than 23, you should disable default features:
```toml
[target.'cfg(target_os = "android")'.dependencies]
tracing_android_trace = { version = "0.1.0", default-features = false }
```

## Crate feature flags

The following feature flags are available:
- `api_level_23` (enabled by default): Require Android API level 23, to avoid some runtime symbol resolution
- `api_level_29`: Require Android API level 29, disabling runtime symbol resolution entirely

<!-- We hide these elements when viewing in Rustdoc, because they're not expected to be present in crate level docs -->
<div class="rustdoc-hidden">

## Community

Discussion of Android Trace development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically in
[#general > Android Tracing](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing).
All public content can be read without logging in.

Contributions are welcome by pull request. The [Rust code of conduct] applies.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

</div>

[`tracing`]: https://docs.rs/tracing/latest/tracing/
[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
[NDK Tracing]: https://developer.android.com/ndk/reference/group/tracing
[tracing_subscriber::filter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/index.html
