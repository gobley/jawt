// Copyright (c) 2025 Gobley Contributors.

use std::error::Error;
use std::fmt;
use std::mem::MaybeUninit;

use jawt_sys::*;
use x11_dl::error::OpenError;
use x11_dl::xlib::*;

use crate::dsi::DrawingSurfaceInfo;

#[repr(transparent)]
pub struct X11DrawingSurfaceInfo(pub(crate) JAWT_X11DrawingSurfaceInfo);

impl fmt::Debug for X11DrawingSurfaceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("X11DrawingSurfaceInfo")
            .field("drawable", &self.0.drawable)
            .field("display", &self.0.display)
            .field("visualID", &self.0.visualID)
            .field("colormapID", &self.0.colormapID)
            .field("depth", &self.0.depth)
            .field("GetAWTColor", &self.0.GetAWTColor)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum WindowRetrievalError {
    XlibOpenFailed(OpenError),
    DrawableIsNotWindow(Drawable),
}

impl fmt::Display for WindowRetrievalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowRetrievalError::XlibOpenFailed(open_error) => {
                write!(f, "failed to open X11: {open_error}")
            }
            WindowRetrievalError::DrawableIsNotWindow(drawable) => {
                write!(f, "drawable {drawable} is not a window")
            }
        }
    }
}

impl Error for WindowRetrievalError {}

impl X11DrawingSurfaceInfo {
    pub fn drawable(&self) -> Drawable {
        self.0.drawable
    }

    pub fn window(&self) -> Result<Window, WindowRetrievalError> {
        let xlib = Xlib::open().map_err(WindowRetrievalError::XlibOpenFailed)?;
        unsafe {
            let mut attributes = MaybeUninit::uninit();
            if (xlib.XGetWindowAttributes)(self.0.display, self.0.drawable, attributes.as_mut_ptr())
                == 0
            {
                return Err(WindowRetrievalError::DrawableIsNotWindow(self.0.drawable));
            }
        };
        Ok(self.0.drawable)
    }

    pub fn display(&self) -> *mut Display {
        self.0.display
    }

    pub fn visual_id(&self) -> VisualID {
        self.0.visualID
    }

    pub fn colormap_id(&self) -> Colormap {
        self.0.colormapID
    }

    pub fn depth(&self) -> i32 {
        self.0.depth as _
    }

    #[cfg(feature = "java-1-4")]
    pub fn get_awt_color(&self, dsi: &DrawingSurfaceInfo, r: i32, g: i32, b: i32) -> i32 {
        unsafe {
            (self
                .0
                .GetAWTColor
                .expect("JAWT_X11DrawingSurfaceInfo.GetAWTColor is not available"))(
                dsi.as_ref().ds,
                r as _,
                g as _,
                b as _,
            ) as _
        }
    }
}

impl AsRef<JAWT_X11DrawingSurfaceInfo> for X11DrawingSurfaceInfo {
    fn as_ref(&self) -> &JAWT_X11DrawingSurfaceInfo {
        &self.0
    }
}
