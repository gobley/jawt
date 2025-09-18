# jawt

Cross-platform, safe Rust bindings to Java AWT Native Interface.

## Versions

| jawt  | jawt-sys | jni  | jni-sys | euclid | windows | objc2 | objc2-{app-kit, quartz-core} | x11-dl | MSRV |
| ----- | -------- | ---- | ------- | ------ | ------- | ----- | ---------------------------- | ------ | ---- |
| 0.1.0 | 0.1      | 0.22 | 0.3     | 0.22   | 0.60    | 0.6   | 0.3                          | 0.2    | 1.74 |
| 0.2.0 | 0.1      | 0.22 | 0.3     | 0.22   | 0.60    | 0.6   | 0.3                          | 0.2    | 1.74 |

## Features

| Feature name      | Default | Description                                                                                                                        |
| ----------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `euclid`          |         | Enables conversions between `jawt::Rect` and `euclid::Rect`.                                                                       |
| `java-1-4`        | ✅      | Enables APIs introduced in Java 1.4.                                                                                               |
| `java-9`          | ✅      | Enables APIs introduced in Java 9.                                                                                                 |
| `dynamic-get-awt` | ✅      | Configures `jawt::Awt` to locate `JAWT_GetAWT` in `jawt.dll` or`libjawt.{dylib, so}` at runtime.                                   |
| `static-get-awt`  |         | Configures `jawt::Awt` to use `jawt_sys::JAWT_GetAWT`. Users must manually link `jawt.dll` or `libjawt.{dylib, so}` at build time. |

## How to use

Please refer to [the WGPU example](https://github.com/gobley/jawt/tree/main/jawt-tests) in the [GitHub repository](https://github.com/gobley/jawt).

| Windows              | macOS              | Linux              |
| -------------------- | ------------------ | ------------------ |
| ![Windows WGPU Demo] | ![macOS WGPU Demo] | ![Linux WGPU Demo] |

[Windows WGPU Demo]: https://raw.githubusercontent.com/gobley/jawt/refs/tags/jawt-v0.2.0/images/windows.png
[macOS WGPU Demo]: https://raw.githubusercontent.com/gobley/jawt/refs/tags/jawt-v0.2.0/images/macos.png
[Linux WGPU Demo]: https://raw.githubusercontent.com/gobley/jawt/refs/tags/jawt-v0.2.0/images/linux.png

## Licensing

Dual-licensed under MIT and Apache License version 2.0.
