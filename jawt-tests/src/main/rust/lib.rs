// Copyright (c) 2025 Gobley Contributors.

mod graphics;

use std::ffi::c_void;
use std::ptr::{self, NonNull};
use std::sync::OnceLock;

use jawt::{Awt, AwtVersion};
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, JNI_VERSION_1_8};
use jni::{JNIEnv, JavaVM};

use crate::graphics::RenderContext;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: JavaVM, _reserved: *const c_void) -> jint {
    env_logger::init();
    log::debug!("JNI_OnLoad() invoked successfully");
    JNI_VERSION_1_8
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_dev_gobley_jawt_tests_RustCanvas_create(
    env: JNIEnv,
    _class: JClass,
    target: JObject, /* java.awt.Component */
) -> jlong {
    fn awt(env: &JNIEnv) -> &'static Awt {
        static AWT: OnceLock<Awt> = OnceLock::new();
        AWT.get_or_init(|| {
            Awt::from_version(env, AwtVersion::VERSION_9).expect("could not retrieve Awt")
        })
    }

    let awt = awt(&env);

    let mut drawing_surface = awt
        .drawing_surface(&env, target)
        .expect("could not retrieve DrawingSurface");

    let mut lock = drawing_surface
        .lock()
        .expect("could not lock the DrawingSurface")
        .1;

    let dsi = lock
        .drawing_surface_info()
        .expect("could not retrieve DrawingSurfaceInfo");

    #[cfg(target_os = "windows")]
    let render_target = crate::graphics::windows::WindowRenderTarget::new(
        dsi.platform_info()
            .surface_kind()
            .and_then(|s| s.window())
            .expect("surface doesn't have a HWND handle"),
    );

    #[cfg(target_os = "macos")]
    let render_target = unsafe {
        use jawt::macos::SurfaceLayers;
        use objc2_quartz_core::{CAMetalLayer, CATransaction};

        use crate::graphics::macos::CAMetalLayerRenderTarget;

        let platform_info = dsi.platform_info();
        let layer = CAMetalLayer::layer();
        let window_layer = platform_info.window_layer();

        layer.setContentsScale(window_layer.contentsScale());
        layer.setFrame(platform_info.window_layer().bounds());

        CATransaction::begin();
        platform_info.set_layer(Some(&layer));
        window_layer.addSublayer(&layer);
        CATransaction::commit();
        CATransaction::flush();

        CAMetalLayerRenderTarget::new(&layer)
    };

    #[cfg(all(
        target_family = "unix",
        not(target_vendor = "apple"),
        not(target_os = "android")
    ))]
    let render_target = crate::graphics::unix::X11RenderTarget::new(
        dsi.platform_info().display(),
        dsi.platform_info()
            .window()
            .expect("surface doesn't have a window drawable"),
    );

    let render_context = RenderContext::new(render_target);
    let render_context = Box::new(render_context);

    Box::into_raw(render_context).expose_provenance() as jlong
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_dev_gobley_jawt_tests_RustCanvas_render(
    _env: JNIEnv,
    _class: JClass,
    render_context: jlong,
) {
    let render_context = ptr::with_exposed_provenance_mut::<RenderContext>(render_context as usize);
    let Some(render_context) = NonNull::new(render_context) else {
        return;
    };
    let render_context = unsafe { render_context.as_ref() };
    render_context.render();
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_dev_gobley_jawt_tests_RustCanvas_resize(
    _env: JNIEnv,
    _class: JClass,
    render_context: jlong,
    width: jint,
    height: jint,
) {
    let render_context = ptr::with_exposed_provenance_mut::<RenderContext>(render_context as usize);
    let Some(render_context) = NonNull::new(render_context) else {
        return;
    };
    let render_context = unsafe { render_context.as_ref() };
    render_context.change_size(
        u32::try_from(width).unwrap_or_default(),
        u32::try_from(height).unwrap_or_default(),
    );
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_dev_gobley_jawt_tests_RustCanvas_destroy(
    _env: JNIEnv,
    _class: JClass,
    render_context: jlong,
) {
    let render_context = ptr::with_exposed_provenance_mut::<RenderContext>(render_context as usize);
    let Some(render_context) = NonNull::new(render_context) else {
        return;
    };
    // destroy
    let _ = unsafe { Box::from_raw(render_context.as_ptr()) };
}
