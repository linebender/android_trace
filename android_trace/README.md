<div align="center" class="rustdoc-hidden">

# Android Trace

</div>

<div align="center">

**Support for Android NDK [Tracing](https://developer.android.com/ndk/reference/group/tracing)**

[![Latest published version.](https://img.shields.io/crates/v/android_trace.svg)](https://crates.io/crates/android_trace)
[![Documentation build status.](https://img.shields.io/docsrs/android_trace.svg)](https://docs.rs/android_trace)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
\
[![Linebender Zulip chat.](https://img.shields.io/badge/Linebender-%23general%20%3E%20Android%20Tracing-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/147921-general/topic/Android.20Tracing)
[![GitHub Actions CI status.](https://img.shields.io/github/actions/workflow/status/linebender/android_trace/ci.yml?logo=github&label=CI)](https://github.com/linebender/android_trace/actions)
[![Dependency staleness status.](https://deps.rs/crate/android_trace/latest/status.svg)](https://deps.rs/crate/android_trace)

</div>

⚠️ Android Trace only supports Android

Android Trace provides access to the Android NDK methods, such as `ATrace_beginSection` and `ATrace_endSection`.
This enables using [Android GPU Inspector](https://gpuinspector.dev/) for Rust code.

<!-- We use the link to the crate page because of crates.io and docs.rs.
If you're reading this comment, you probably want the tracing_android_trace which is
a sibling to this file's parent folder-->
See [tracing_android_trace](https://crates.io/crates/tracing_android_trace)
for an integration of this with [`tracing`](https://docs.rs/tracing/latest/tracing/).

Significant changes are documented in [the changelog][].

## Quickstart

Add a dependency on Android Trace:

```toml
[target.'cfg(target_os = "android")'.dependencies]
android_trace = "0.1.0"
```

The main entry point to the library is [AndroidTrace][], which stores function pointers to each available NDK function:

```rust,no_run
use android_trace::AndroidTrace;
let trace = AndroidTrace::new();

// If the `is_enabled` method isn't available, we also shouldn't trace
let should_trace = trace.is_enabled().unwrap_or(false);
if should_trace {
  trace.begin_section(c"My expensive calculation");
}

// ...performing an expensive calculation

if should_trace {
  trace.end_section();
}
```

## Android API levels

The first level of the [tracing API](https://developer.android.com/ndk/reference/group/tracing) has been available since Android API level 23, and a more flexible API was added in Android API level 29.
To support devices with any Android API versions, we resolve these functions at runtime using [dlsym][].
This runtime access is used unless we know (through [features](#crate-feature-flags)) that a certain API level is available.

## Crate feature flags

The following feature flags are available:

* `api_level_23` (enabled by default): Require Android API level 23, to avoid some runtime symbol resolution
* `api_level_29`: Require Android API level 29, to improve efficiency, to avoid runtime symbol resolution entirely

To support Android API versions less than 23, you should disable default features:

```toml
[target.'cfg(target_os = "android")'.dependencies]
android_trace = { version = "0.1.0", default-features = false }
```

## Minimum supported Rust Version (MSRV)

This version of Android Trace has been verified to compile with **Rust 1.77** and later.

Future versions of Android Trace might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of Android Trace's dependencies could have released versions with a higher Rust requirement.
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

[the changelog]: https://github.com/linebender/android_trace/blob/main/CHANGELOG.md
[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT

<!-- Replacement intra-doc links for GitHub and crates.io. See https://linebender.org/blog/doc-include -->
[AndroidTrace]: https://docs.rs/android_trace/0.1.0/android_trace/struct.AndroidTrace.html
[dlsym]: https://man7.org/linux/man-pages/man3/dlsym.3.html
