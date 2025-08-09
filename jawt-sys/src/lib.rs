// Copyright (c) 2025 Gobley Contributors.

//! Raw bindings generated from Java AWT headers. For more details about how to
//! use these bindings, please refer to the [Oracle Documentation].
//!
//! [Oracle Documentation]: https://docs.oracle.com/en/java/javase/17/docs/specs/AWT_Native_Interface.html

#![no_std]

#[cfg(target_os = "windows")]
mod bindings_windows;
#[cfg(target_os = "windows")]
pub use bindings_windows::*;

#[cfg(target_os = "macos")]
mod bindings_macos;
#[cfg(target_os = "macos")]
pub use bindings_macos::*;

#[cfg(all(
    target_family = "unix",
    not(target_vendor = "apple"),
    not(target_os = "android")
))]
mod bindings_unix;
#[cfg(all(
    target_family = "unix",
    not(target_vendor = "apple"),
    not(target_os = "android")
))]
pub use bindings_unix::*;
