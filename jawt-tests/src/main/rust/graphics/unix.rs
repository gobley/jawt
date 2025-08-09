// Copyright (c) 2025 Gobley Contributors.

use std::mem::MaybeUninit;
use std::ptr::NonNull;

use wgpu::rwh::*;
use wgpu::*;
use x11_dl::xlib::{Display, Window, Xlib};

use crate::graphics::RenderTarget;

pub struct X11RenderTarget {
    display: *mut Display,
    window: Window,
}

unsafe impl Send for X11RenderTarget {}

unsafe impl Sync for X11RenderTarget {}

impl X11RenderTarget {
    pub fn new(display: *mut Display, window: Window) -> Self {
        Self { display, window }
    }
}

impl RenderTarget for X11RenderTarget {
    fn size(&self) -> (u32, u32) {
        let Ok(xlib) = Xlib::open() else {
            return (0, 0);
        };
        let attributes = unsafe {
            let mut attributes = MaybeUninit::uninit();
            if (xlib.XGetWindowAttributes)(self.display, self.window, attributes.as_mut_ptr()) == 0
            {
                return (0, 0);
            }
            attributes.assume_init()
        };
        (
            u32::try_from(attributes.width).unwrap_or_default(),
            u32::try_from(attributes.height).unwrap_or_default(),
        )
    }

    unsafe fn create_surface(&self, instance: &wgpu::Instance) -> wgpu::Surface<'static> {
        instance
            .create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: RawDisplayHandle::Xlib(XlibDisplayHandle::new(
                    NonNull::new(self.display.cast()),
                    Xlib::open()
                        .ok()
                        .and_then(|xlib| {
                            let mut attributes = MaybeUninit::uninit();
                            if (xlib.XGetWindowAttributes)(
                                self.display,
                                self.window,
                                attributes.as_mut_ptr(),
                            ) == 0
                            {
                                return None;
                            }
                            let attributes = attributes.assume_init();
                            Some((xlib.XScreenNumberOfScreen)(attributes.screen))
                        })
                        .unwrap_or_default(),
                )),
                raw_window_handle: RawWindowHandle::Xlib(XlibWindowHandle::new(self.window)),
            })
            .expect("could not create WGPU surface")
    }
}
