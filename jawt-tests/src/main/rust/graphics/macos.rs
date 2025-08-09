// Copyright (c) 2025 Gobley Contributors.

use objc2::rc::Retained;
use objc2::Message;
use objc2_quartz_core::CAMetalLayer;
use wgpu::*;

use super::RenderTarget;

#[repr(transparent)]
pub struct CAMetalLayerRenderTarget(Retained<CAMetalLayer>);

unsafe impl Send for CAMetalLayerRenderTarget {}

unsafe impl Sync for CAMetalLayerRenderTarget {}

impl CAMetalLayerRenderTarget {
    pub fn new(inner: &CAMetalLayer) -> Self {
        Self(inner.retain())
    }
}

impl RenderTarget for CAMetalLayerRenderTarget {
    fn size(&self) -> (u32, u32) {
        let size = self.0.bounds().size;
        (size.width as u32, size.height as u32)
    }

    unsafe fn create_surface(&self, instance: &wgpu::Instance) -> wgpu::Surface<'static> {
        instance
            .create_surface_unsafe(SurfaceTargetUnsafe::CoreAnimationLayer(self.0.as_ref()
                as *const CAMetalLayer
                as _))
            .expect("could not create WGPU surface")
    }
}

impl Drop for CAMetalLayerRenderTarget {
    fn drop(&mut self) {
        self.0.removeFromSuperlayer();
    }
}
