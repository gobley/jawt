// Copyright (c) 2025 Gobley Contributors.

//! Implements platform-specific types.

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(all(
    target_family = "unix",
    not(target_vendor = "apple"),
    not(target_os = "android")
))]
pub mod unix;
