// Copyright (c) 2025 Gobley Contributors.

//! Implements the [DrawingSurfaceInfo] struct.

use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

use jawt_sys::*;

use crate::ds::DrawingSurfaceGuard;
use crate::Rect;

type DrawingSurfaceInfoFree = unsafe extern "C" fn(dsi: *mut JAWT_DrawingSurfaceInfo);

#[cfg(target_os = "windows")]
pub type DrawingSurfacePlatformInfo = crate::md::windows::Win32DrawingSurfaceInfo;

#[cfg(target_os = "macos")]
pub type DrawingSurfacePlatformInfo =
    objc2::runtime::ProtocolObject<dyn crate::md::macos::SurfaceLayers>;

#[cfg(all(
    target_family = "unix",
    not(target_vendor = "apple"),
    not(target_os = "android")
))]
pub type DrawingSurfacePlatformInfo = crate::md::unix::X11DrawingSurfaceInfo;

/// Structure for containing the underlying drawing information of a component.
pub struct DrawingSurfaceInfo<'a> {
    pub(crate) inner: NonNull<JAWT_DrawingSurfaceInfo>,
    pub(crate) free: DrawingSurfaceInfoFree,
    pub(crate) _drawing_surface: PhantomData<DrawingSurfaceGuard<'a>>,
}

impl fmt::Debug for DrawingSurfaceInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DrawingSurfaceInfo")
            .field("platformInfo", &self.inner)
            .field("JAWT_DrawingSurface.FreeDrawingSurfaceInfo", &self.free)
            .finish()
    }
}

impl DrawingSurfaceInfo<'_> {
    /// Constructs a safe [DrawingSurfaceInfo] instance from a raw [JAWT_DrawingSurfaceInfo] with
    /// its destroying function retrieved from a [JAWT_DrawingSurface].
    ///
    /// # Safety
    ///
    /// `inner` must be properly created using [JAWT_DrawingSurface::GetDrawingSurfaceInfo]. `free`
    /// should be a valid value of [JAWT_DrawingSurface::FreeDrawingSurfaceInfo].
    pub const unsafe fn from_raw_parts(
        inner: NonNull<JAWT_DrawingSurfaceInfo>,
        free: DrawingSurfaceInfoFree,
    ) -> Self {
        Self {
            inner,
            free,
            _drawing_surface: PhantomData,
        }
    }

    /// Returns a shared reference to the underlying [JAWT_DrawingSurfaceInfo] instance.
    pub const fn as_ref(&self) -> &JAWT_DrawingSurfaceInfo {
        // Safety: `inner` is owned by `DrawingSurface`.
        unsafe { self.inner.as_ref() }
    }

    /// Returns a mutable reference to the underlying [JAWT_DrawingSurfaceInfo] instance.
    ///
    /// # Safety
    ///
    /// The caller should ensure that the underlying instance keep containing valid field values.
    pub unsafe fn as_mut(&mut self) -> &mut JAWT_DrawingSurfaceInfo {
        self.inner.as_mut()
    }

    /// Destructs [DrawingSurfaceInfo] into a raw [JAWT_DrawingSurfaceInfo] and a pointer to its destroying
    /// function.
    pub fn into_raw_parts(self) -> (NonNull<JAWT_DrawingSurfaceInfo>, DrawingSurfaceInfoFree) {
        (self.inner, self.free)
    }

    /// Pointer to the platform-specific information.
    pub const fn platform_info(&self) -> &DrawingSurfacePlatformInfo {
        unsafe { &*(self.as_ref().platformInfo as *const DrawingSurfacePlatformInfo) }
    }

    /// Bounding rectangle of the drawing surface.
    pub const fn bounds(&self) -> Rect {
        let bounds = &self.as_ref().bounds;
        Rect {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
        }
    }

    /// Clip rectangle array.
    pub const fn clip(&self) -> &[Rect] {
        unsafe {
            slice::from_raw_parts(
                self.as_ref().clip as *const Rect,
                self.as_ref().clipSize as usize,
            )
        }
    }
}
