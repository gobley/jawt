// Copyright (c) 2025 Gobley Contributors.

use objc2::extern_protocol;
use objc2::rc::Retained;
use objc2::runtime::NSObjectProtocol;
use objc2_quartz_core::CALayer;

extern_protocol! {
    #[allow(clippy::missing_safety_doc)]
    #[name = "JAWT_SurfaceLayers"]
    pub unsafe trait SurfaceLayers: NSObjectProtocol {
        #[unsafe(method(layer))]
        fn layer(&self) -> Option<Retained<CALayer>>;

        #[unsafe(method(setLayer:))]
        fn set_layer(&self, layer: Option<&CALayer>);

        #[unsafe(method(windowLayer))]
        fn window_layer(&self) -> Retained<CALayer>;
    }
}
