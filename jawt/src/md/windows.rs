// Copyright (c) 2025 Gobley Contributors.

use std::error::Error;
use std::ffi::c_void;
use std::fmt;

use jawt_sys::*;
use windows::Win32::Foundation::{HWND, TRUE};
use windows::Win32::Graphics::Gdi::{GetObjectType, HBITMAP, HDC, HGDIOBJ, HPALETTE, OBJ_BITMAP};
use windows::Win32::UI::WindowsAndMessaging::IsWindow;

/// Microsoft Windows specific declarations for AWT native interface.
#[repr(transparent)]
pub struct Win32DrawingSurfaceInfo(pub(crate) JAWT_Win32DrawingSurfaceInfo);

impl fmt::Debug for Win32DrawingSurfaceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Win32DrawingSurfaceInfo")
            .field("surface_kind", &self.surface_kind())
            .field("hdc", &self.0.hdc)
            .field("hpalette", &self.0.hpalette)
            .finish()
    }
}

/// Represents a native window, DDB, or DIB handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    Window(HWND),
    Bitmap(HBITMAP),
    DIBits(*mut c_void),
}

impl SurfaceKind {
    pub fn window(self) -> Option<HWND> {
        self.try_into().ok()
    }

    pub fn unwrap_window(self) -> HWND {
        self.try_into().unwrap()
    }

    pub fn bitmap(self) -> Option<HBITMAP> {
        self.try_into().ok()
    }

    pub fn unwrap_bitmap(self) -> HBITMAP {
        self.try_into().unwrap()
    }

    pub fn di_bits(self) -> Option<*mut c_void> {
        self.try_into().ok()
    }

    pub fn unwrap_di_bits(self) -> *mut c_void {
        self.try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct DifferentSurfaceKind;

impl fmt::Display for DifferentSurfaceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "different kind of surface")
    }
}

impl Error for DifferentSurfaceKind {}

impl TryFrom<SurfaceKind> for HWND {
    type Error = DifferentSurfaceKind;

    fn try_from(value: SurfaceKind) -> Result<Self, Self::Error> {
        let SurfaceKind::Window(window) = value else {
            return Err(DifferentSurfaceKind);
        };
        Ok(window)
    }
}

impl TryFrom<SurfaceKind> for HBITMAP {
    type Error = DifferentSurfaceKind;

    fn try_from(value: SurfaceKind) -> Result<Self, Self::Error> {
        let SurfaceKind::Bitmap(bitmap) = value else {
            return Err(DifferentSurfaceKind);
        };
        Ok(bitmap)
    }
}

impl TryFrom<SurfaceKind> for *mut c_void {
    type Error = DifferentSurfaceKind;

    fn try_from(value: SurfaceKind) -> Result<Self, Self::Error> {
        let SurfaceKind::DIBits(bits) = value else {
            return Err(DifferentSurfaceKind);
        };
        Ok(bits)
    }
}

impl Win32DrawingSurfaceInfo {
    /// Native window, DDB, or DIB handle
    pub fn surface_kind(&self) -> Option<SurfaceKind> {
        let hwnd = HWND(unsafe { self.0.__bindgen_anon_1.hwnd });
        if unsafe { IsWindow(Some(hwnd)) } == TRUE {
            return Some(SurfaceKind::Window(hwnd));
        }

        let hbitmap = HBITMAP(unsafe { self.0.__bindgen_anon_1.hbitmap });
        if unsafe { GetObjectType(HGDIOBJ(hbitmap.0)) } == OBJ_BITMAP.0 as u32 {
            return Some(SurfaceKind::Bitmap(hbitmap));
        }

        let bits = unsafe { self.0.__bindgen_anon_1.pbits };
        if !bits.is_null() {
            return Some(SurfaceKind::DIBits(bits));
        }

        None
    }

    pub fn hdc(&self) -> HDC {
        HDC(self.0.hdc)
    }

    pub fn hpalette(&self) -> HPALETTE {
        HPALETTE(self.0.hpalette)
    }
}

impl AsRef<JAWT_Win32DrawingSurfaceInfo> for Win32DrawingSurfaceInfo {
    fn as_ref(&self) -> &JAWT_Win32DrawingSurfaceInfo {
        &self.0
    }
}
