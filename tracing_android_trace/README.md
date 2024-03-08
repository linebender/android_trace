<div align="center">

# Tracing Android Trace
</div>
<!-- Close the <div> opened in lib.rs for rustdoc, which hides the above title -->
</div>

<div align="center">

**Write [`tracing`] spans to Android NDK [Tracing](https://developer.android.com/ndk/reference/group/tracing)**

[![Linebender Zulip](https://img.shields.io/badge/Linebender-%23general-orange?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![tracing-subscriber version](https://img.shields.io/badge/tracing--subscriber-v0.3.18-a674e5.svg)](https://crates.io/crates/tracing-subscriber)

<!-- [![Crates.io](https://img.shields.io/crates/v/vello.svg)](https://crates.io/crates/vello) -->
<!-- [![Docs](https://docs.rs/vello/badge.svg)](https://docs.rs/vello) -->
<!-- [![Build status](https://github.com/linebender/vello/workflows/CI/badge.svg)](https://github.com/linebender/vello/actions) -->
<!-- [![dependency status](https://deps.rs/repo/github//status.svg)](https://deps.rs/repo/github/) -->
</div>

⚠️ Tracing Android Trace only supports Android

## Quickstart


## Android API levels

This crate uses [`android_trace`] to provide access to the NDK functions, which supports all Android API feature levels.

## Crate feature flags

The following feature flags are available:
- `api_level_23` (enabled by default): Require Android API level 23, to avoid some runtime symbol resolution
- `api_level_29`: Require Android API level 29, to improve efficiency, to avoid runtime symbol resolution entirely

To support Android API versions less than 23, you should disable default features:
```toml
[target.'cfg(target_os = "android")'.dependencies]
tracing_android_trace = { version = "0.1.0", default-features = false }
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

[`tracing`]: https://docs.rs/tracing/latest/tracing/
[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
