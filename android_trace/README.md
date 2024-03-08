<div align="center">

# Android Trace
</div>
<!-- Close the <div> opened in lib.rs for rustdoc, which hides the above title -->
</div>

<div align="center">

**Support for Android NDK [Tracing](https://developer.android.com/ndk/reference/group/tracing)**

[![Linebender Zulip](https://img.shields.io/badge/Linebender-%23general-orange?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)

<!-- [![Crates.io](https://img.shields.io/crates/v/vello.svg)](https://crates.io/crates/vello) -->
<!-- [![Docs](https://docs.rs/vello/badge.svg)](https://docs.rs/vello) -->
<!-- [![Build status](https://github.com/linebender/vello/workflows/CI/badge.svg)](https://github.com/linebender/vello/actions) -->
<!-- [![dependency status](https://deps.rs/repo/github//status.svg)](https://deps.rs/repo/github/) -->
</div>

⚠️ Android Trace only supports Android

## Quickstart
Add a dependency on Android Trace:

```toml
[target.'cfg(target_os = "android")'.dependencies]
android_trace = "0.1.0"
```

The main entry point to the library is [AndroidTrace], which stores function pointers to each available NDK function:
```rust
use android_trace::AndroidTrace;
let trace = AndroidTrace::new();

// If the `is_enabled` method isn't available, we also shouldn't trace
let should_trace = trace.is_enabled().unwrap_or(false);
if should_trace {
  trace.begin_section(CStr::from_bytes_with_nul("My expensive calculation\0").unwrap());
}

// ...performing an expensive calculation

if should_trace {
  trace.end_section();
}
```

## Android API levels

The first level of the [tracing API](https://developer.android.com/ndk/reference/group/tracing) has been available since Android API level 23, and a more flexible API was added in Android API level 29.
To support devices with any Android API versions, we resolve these functions at runtime using [dlsym].
This runtime access is used unless we know (through [features](#crate-feature-flags)) that a certain API level is available.

The helper methods [`begin_section_try_async`] and [`end_section_try_async`] are available, which fallback to the original API if the more flexible API is not available.
This have the same limitations as the original API, namely that sections must begin and end  on the same thread.
However, they allow re-opening the same logical task (including across different threads), when the more flexible API is available.

## Crate feature flags

The following feature flags are available:
- `api_level_23` (enabled by default): Require Android API level 23, to avoid some runtime symbol resolution
- `api_level_29`: Require Android API level 29, to improve efficiency, to avoid runtime symbol resolution entirely

To support Android API versions less than 23, you should disable default features:
```toml
[target.'cfg(target_os = "android")'.dependencies]
android_trace = { version = "0.1.0", default-features = false }
```

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

[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
[AndroidTrace]: https://docs.rs/android_trace/todo
[dlsym]: https://man7.org/linux/man-pages/man3/dlsym.3.html
[`begin_section_try_async`]: https://docs.rs/android_trace/todo
[`end_section_try_async`]: https://docs.rs/android_trace/todo
