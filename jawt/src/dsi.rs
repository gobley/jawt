// Copyright (c) 2025 Gobley Contributors.

//! Implements the [DrawingSurfaceInfo] struct.

use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{ffi::c_void, slice};

use jawt_sys::*;

use crate::{DrawingSurfaceGuard, Rect};

type DrawingSurfaceInfoFree = unsafe extern "C" fn(dsi: *mut JAWT_DrawingSurfaceInfo);

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

    pub const unsafe fn platform_info(&mut self) -> *mut c_void {
        self.as_ref().platformInfo
    }

    /// Cached pointer to the underlying drawing surface.
    pub const unsafe fn raw_drawing_surface(&mut self) -> *mut JAWT_DrawingSurface {
        self.as_ref().ds
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
