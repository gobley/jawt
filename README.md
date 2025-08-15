# `jawt`

[![License](https://img.shields.io/github/license/gobley/jawt)](https://github.com/gobley/jawt/blob/main/LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/jawt)](https://crates.io/crates/jawt)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/gobley/jawt/pr-build-jawt.yml?branch=main&label=tests)](https://github.com/gobley/jawt/actions/workflows/pr-build-jawt.yml?query=branch%3Amain)

Rust bindings to Java AWT Native Interface.

## Packages

- [jawt](./jawt/README.md): Safe bindings to Java AWT.
- [jawt-sys](./jawt-sys/README.md): Raw bindings to Java AWT.
- [jawt-sys-generator](./jawt-sys-generator): The bindgen for `jawt-sys`.
- [jawt-tests](./jawt-tests): A simple Kotlin project that integrates WGPU and AWT using `jawt`.

  | Windows                                    | macOS                                  | Linux                                  |
  | ------------------------------------------ | -------------------------------------- | -------------------------------------- |
  | ![Windows WGPU Demo](./images/windows.png) | ![macOS WGPU Demo](./images/macos.png) | ![Linux WGPU Demo](./images/linux.png) |

## Licensing

Dual-licensed under MIT and Apache License version 2.0.
