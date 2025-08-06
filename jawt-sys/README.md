# jawt-sys

Raw Rust bindings to Java AWT Native Interface.

## Interoperability

This package can be used with popular FFI packages such as [jni-sys](https://crates.io/crates/jni-sys), [windows-sys](https://crates.io/crates/windows-sys), and [x11-dl](https://crates.io/crates/x11-dl).

## How to re-generate bindings

Run [./generate.ps1](./generate.ps1) on PowerShell. This will install bindgen 0.69 in `.bindgen` not to update the bindgen already installed in `CARGO_HOME`.

## Versions

| jawt-sys | OpenJDK | jni-sys | windows-sys | x11-dl | MSRV |
| -------- | ------- | ------- | ----------- | ------ | ---- |
| 0.1.0    | 17      | 0.3     | 0.60        | 2      | 1.73 |

## Licensing

Dual-licensed under MIT and Apache License version 2.0.
