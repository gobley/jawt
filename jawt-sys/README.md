# jawt-sys

Raw Rust bindings to Java AWT Native Interface.

## Interoperability

This package can be used with popular FFI packages such as [jni-sys](https://crates.io/crates/jni-sys), [windows-sys](https://crates.io/crates/windows-sys), and [x11-dl](https://crates.io/crates/x11-dl).

## How to re-generate bindings

Run [`jawt-sys-generator`](https://github.com/gobley/jawt/tree/main/jawt-sys-generator). This will download JAWT headers from OpenJDK and generate bindings from them. You can also use [this GitHub Actions workflow](https://github.com/gobley/jawt/actions/workflows/generate.yml) to run `jawt-sys-generator` on Windows, macOS, and Linux simultaneously.

## Versions

| jawt-sys | OpenJDK | jni-sys | windows-sys | x11-dl | MSRV |
| -------- | ------- | ------- | ----------- | ------ | ---- |
| 0.1.0    | 17      | 0.3     | 0.60        | 2      | 1.74 |
| 0.2.0    | 17      | 0.3     | 0.60        | 2      | 1.74 |

## Features

| Feature name     | Description                                                                                                                          |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `static-get-awt` | Enables `jawt_sys::JAWT_GetAWT()`. To call the function, users must manually link `jawt.dll` or `libjawt.{dylib, so}` at build time. |

## How to use

Please refer to [the WGPU example](https://github.com/gobley/jawt/tree/main/jawt-tests) in the [GitHub repository](https://github.com/gobley/jawt).

| Windows              | macOS              | Linux              |
| -------------------- | ------------------ | ------------------ |
| ![Windows WGPU Demo] | ![macOS WGPU Demo] | ![Linux WGPU Demo] |

[Windows WGPU Demo]: https://raw.githubusercontent.com/gobley/jawt/refs/tags/jawt-sys-v0.2.0/images/windows.png
[macOS WGPU Demo]: https://raw.githubusercontent.com/gobley/jawt/refs/tags/jawt-sys-v0.2.0/images/macos.png
[Linux WGPU Demo]: https://raw.githubusercontent.com/gobley/jawt/refs/tags/jawt-sys-v0.2.0/images/linux.png

## Licensing

Dual-licensed under MIT and Apache License version 2.0.
