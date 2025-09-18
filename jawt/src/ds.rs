// Copyright (c) 2025 Gobley Contributors.

//! Implements the [DrawingSurface] struct.

use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

use jawt_sys::*;
use jni::sys::jint;

use crate::dsi::DrawingSurfaceInfo;

type DrawingSurfaceFree = unsafe extern "C" fn(ds: *mut JAWT_DrawingSurface);

/// Structure for containing the underlying drawing information of a component. All operations on a
/// [DrawingSurface] MUST be performed from the same thread as the call to [Awt::drawing_surface()].
///
/// [Awt::drawing_surface()]: crate::awt::Awt::drawing_surface()
pub struct DrawingSurface {
    pub(crate) inner: NonNull<JAWT_DrawingSurface>,
    pub(crate) free: DrawingSurfaceFree,
}

impl fmt::Debug for DrawingSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DrawingSurface")
            .field("env", &self.as_ref().env)
            .field("target", &self.as_ref().target)
            .field("Lock", &self.as_ref().Lock)
            .field(
                "GetDrawingSurfaceInfo",
                &self.as_ref().GetDrawingSurfaceInfo,
            )
            .field(
                "FreeDrawingSurfaceInfo",
                &self.as_ref().FreeDrawingSurfaceInfo,
            )
            .field("Unlock", &self.as_ref().Unlock)
            .field("JAWT.FreeDrawingSurface", &self.free)
            .finish()
    }
}

impl DrawingSurface {
    /// Constructs a safe [DrawingSurface] instance from a raw [JAWT_DrawingSurface] with its
    /// destroying function retrieved from a [JAWT].
    ///
    /// # Safety
    ///
    /// `inner` must be properly created using [JAWT::GetDrawingSurface]. `free` should be a
    /// valid value of [JAWT::FreeDrawingSurface].
    pub const unsafe fn from_raw_parts(
        inner: NonNull<JAWT_DrawingSurface>,
        free: DrawingSurfaceFree,
    ) -> Self {
        Self { inner, free }
    }

    /// Returns a shared reference to the underlying [JAWT_DrawingSurface] instance.
    pub const fn as_ref(&self) -> &JAWT_DrawingSurface {
        // Safety: `inner` is owned by `DrawingSurface`.
        unsafe { self.inner.as_ref() }
    }

    /// Returns a mutable reference to the underlying [JAWT_DrawingSurface] instance.
    ///
    /// # Safety
    ///
    /// The caller should ensure that the underlying instance keep containing valid field values.
    pub unsafe fn as_mut(&mut self) -> &mut JAWT_DrawingSurface {
        self.inner.as_mut()
    }

    /// Destructs [DrawingSurface] into a raw [JAWT_DrawingSurface] and a pointer to its destroying
    /// function.
    pub fn into_raw_parts(self) -> (NonNull<JAWT_DrawingSurface>, DrawingSurfaceFree) {
        (self.inner, self.free)
    }

    /// Lock the surface of the target component for native rendering.
    pub fn lock(&mut self) -> Option<(DrawingSurfaceLockResult, DrawingSurfaceGuard<'_>)> {
        let lock_result = unsafe {
            (self
                .as_ref()
                .Lock
                .expect("JAWT_DrawingSurface.Lock is not available"))(
                self.inner.as_ptr()
            )
        };
        if lock_result & JAWT_LOCK_ERROR != 0 {
            return None;
        }
        Some((
            DrawingSurfaceLockResult::from_bits_truncate(lock_result),
            DrawingSurfaceGuard {
                drawing_surface: self,
            },
        ))
    }
}

impl AsRef<JAWT_DrawingSurface> for DrawingSurface {
    fn as_ref(&self) -> &JAWT_DrawingSurface {
        self.as_ref()
    }
}

impl Drop for DrawingSurface {
    fn drop(&mut self) {
        unsafe { (self.free)(self.inner.as_mut()) };
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DrawingSurfaceLockResult: jint {
        /// When the clip region has changed.
        const CLIP_CHANGED = JAWT_LOCK_CLIP_CHANGED;
        /// When the bounds of the surface have changed.
        const BOUNDS_CHANGED = JAWT_LOCK_BOUNDS_CHANGED;
        /// When the surface itself has changed.
        const SURFACE_CHANGED = JAWT_LOCK_SURFACE_CHANGED;
    }
}

/// An RAII implementation of a scoped lock of a [DrawingSurfaceGuard]. The corresponding
/// [DrawingSurface] is unlocked when this structure is dropped.
pub struct DrawingSurfaceGuard<'a> {
    drawing_surface: &'a mut DrawingSurface,
}

impl DrawingSurfaceGuard<'_> {
    pub fn drawing_surface_info(&mut self) -> Option<DrawingSurfaceInfo<'_>> {
        let get_drawing_surface_info = self
            .drawing_surface
            .as_ref()
            .GetDrawingSurfaceInfo
            .expect("JAWT_DrawingSurface.GetDrawingSurfaceInfo is not available");
        let free_drawing_surface_info = self
            .drawing_surface
            .as_ref()
            .FreeDrawingSurfaceInfo
            .expect("JAWT_DrawingSurface.FreeDrawingSurfaceInfo is not available");
        let drawing_surface_info =
            NonNull::new(unsafe { get_drawing_surface_info(self.drawing_surface.inner.as_ptr()) })?;
        Some(DrawingSurfaceInfo {
            inner: drawing_surface_info,
            free: free_drawing_surface_info,
            _drawing_surface: PhantomData,
        })
    }
}

impl Drop for DrawingSurfaceGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            (self
                .drawing_surface
                .as_ref()
                .Unlock
                .expect("JAWT_DrawingSurface.Unlock is not available"))(
                self.drawing_surface.inner.as_ptr(),
            )
        };
    }
}
