// Copyright (c) 2025 Gobley Contributors.

//! Cross-platform Rust bindings to Java AWT. For more details about how to use
//! these bindings, please refer to the [Oracle Documentation].
//!
//! [Oracle Documentation]: https://docs.oracle.com/en/java/javase/17/docs/specs/AWT_Native_Interface.html

mod awt;
pub use awt::*;

mod ds;
pub use ds::*;

mod dsi;
pub use dsi::*;

mod md;
pub use md::*;

mod rect;
pub use rect::*;

mod version;
pub use version::*;

pub use jawt_sys as sys;
