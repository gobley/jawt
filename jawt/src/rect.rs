// Copyright (c) 2025 Gobley Contributors.

//! Defines the [Rect] struct.

use jawt_sys::jawt_Rectangle;

/// Structure for a native rectangle.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    #[inline(always)]
    pub const fn into_sys(self) -> jawt_Rectangle {
        jawt_Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    #[inline(always)]
    pub const fn from_sys(rect: jawt_Rectangle) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

impl From<jawt_Rectangle> for Rect {
    fn from(value: jawt_Rectangle) -> Self {
        Self::from_sys(value)
    }
}

impl From<Rect> for jawt_Rectangle {
    fn from(value: Rect) -> Self {
        value.into_sys()
    }
}

#[cfg(feature = "euclid")]
impl Rect {
    #[inline(always)]
    pub const fn into_euclid<U>(self) -> euclid::Rect<i32, U> {
        euclid::Rect::new(
            euclid::Point2D::new(self.x, self.y),
            euclid::Size2D::new(self.width, self.height),
        )
    }

    #[inline(always)]
    pub const fn from_euclid<U>(rect: euclid::Rect<i32, U>) -> Self {
        Self {
            x: rect.origin.x,
            y: rect.origin.y,
            width: rect.size.width,
            height: rect.size.height,
        }
    }
}

#[cfg(feature = "euclid")]
impl<U> From<euclid::Rect<i32, U>> for Rect {
    fn from(value: euclid::Rect<i32, U>) -> Self {
        Self::from_euclid(value)
    }
}

#[cfg(feature = "euclid")]
impl<U> From<Rect> for euclid::Rect<i32, U> {
    fn from(value: Rect) -> Self {
        value.into_euclid()
    }
}
