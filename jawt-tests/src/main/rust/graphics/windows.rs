// Copyright (c) 2025 Gobley Contributors.

use std::mem::MaybeUninit;
use std::num::NonZeroIsize;

use wgpu::rwh::*;
use wgpu::*;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

use crate::graphics::RenderTarget;

#[repr(transparent)]
pub struct WindowRenderTarget(HWND);

impl WindowRenderTarget {
    pub fn new(inner: HWND) -> Self {
        Self(inner)
    }
}

unsafe impl Send for WindowRenderTarget {}

unsafe impl Sync for WindowRenderTarget {}

impl RenderTarget for WindowRenderTarget {
    fn size(&self) -> (u32, u32) {
        let rect = unsafe {
            let mut rect = MaybeUninit::uninit();
            if GetWindowRect(self.0, rect.as_mut_ptr()).is_err() {
                return (0, 0);
            }
            rect.assume_init()
        };
        (
            u32::try_from(rect.right - rect.left).unwrap_or_default(),
            u32::try_from(rect.bottom - rect.top).unwrap_or_default(),
        )
    }

    unsafe fn create_surface(&self, instance: &wgpu::Instance) -> wgpu::Surface<'static> {
        instance
            .create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: RawDisplayHandle::Windows(WindowsDisplayHandle::new()),
                raw_window_handle: RawWindowHandle::Win32(Win32WindowHandle::new(
                    NonZeroIsize::new((self.0).0 as _).unwrap(),
                )),
            })
            .expect("could not create WGPU surface")
    }
}
