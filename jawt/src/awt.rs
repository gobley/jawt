// Copyright (c) 2025 Gobley Contributors.

//! Implements the [Awt] struct.

use std::ffi::c_void;
use std::fmt;
use std::ptr::NonNull;

use jni::objects::JObject;
use jni::sys::*;
use jni::JNIEnv;

#[cfg(feature = "java-9")]
use crate::rect::Rect;
use crate::sys::*;
use crate::version::AwtVersion;
use crate::DrawingSurface;

#[cfg(target_os = "windows")]
pub type AwtPlatformInfo = windows::Win32::Foundation::HWND;

#[cfg(target_os = "macos")]
pub type AwtPlatformInfo = NonNull<objc2_app_kit::NSView>;

#[cfg(all(
    target_family = "unix",
    not(target_vendor = "apple"),
    not(target_os = "android")
))]
pub type AwtPlatformInfo = x11_dl::xlib::Window;

type UnsafeAwtGetter = unsafe extern "C" fn(*mut jni::sys::JNIEnv, *mut JAWT) -> jboolean;

/// Structure for containing native AWT functions.
pub struct Awt(pub(crate) JAWT);

impl fmt::Debug for Awt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Awt")
            .field("version", &self.version())
            .field("GetDrawingSurface", &self.0.GetDrawingSurface)
            .field("FreeDrawingSurface", &self.0.FreeDrawingSurface)
            .field("Lock", &self.0.Lock)
            .field("Unlock", &self.0.Unlock)
            .field("GetComponent", &self.0.GetComponent)
            .field("CreateEmbeddedFrame", &self.0.CreateEmbeddedFrame)
            .field("SetBoudns", &self.0.SetBounds)
            .field(
                "SynthesizeWindowActivation",
                &self.0.SynthesizeWindowActivation,
            )
            .finish()
    }
}

impl Clone for Awt {
    fn clone(&self) -> Self {
        Self(JAWT {
            version: self.0.version,
            GetDrawingSurface: self.0.GetDrawingSurface,
            FreeDrawingSurface: self.0.FreeDrawingSurface,
            Lock: self.0.Lock,
            Unlock: self.0.Unlock,
            GetComponent: self.0.GetComponent,
            CreateEmbeddedFrame: self.0.CreateEmbeddedFrame,
            SetBounds: self.0.SetBounds,
            SynthesizeWindowActivation: self.0.SynthesizeWindowActivation,
        })
    }
}

// Safety: the underlying `JAWT` struct is just a collection of function pointers.
unsafe impl Send for Awt {}

// Safety: the underlying `JAWT` struct is just a collection of function pointers.
unsafe impl Sync for Awt {}

impl Awt {
    /// Construct a safe [Awt] instance from a raw [JAWT] instance.
    ///
    /// # Safety
    ///
    /// `inner` must be properly initialized using [JAWT_GetAWT].
    pub const unsafe fn from_inner(inner: JAWT) -> Self {
        Self(inner)
    }

    /// Returns a shared reference to the underlying [JAWT] instance.
    pub const fn as_ref(&self) -> &JAWT {
        &self.0
    }

    /// Returns a mutable reference to the underlying [JAWT] instance.
    ///
    /// # Safety
    ///
    /// The caller should ensure that the underlying instance keep containing valid field values.
    pub const unsafe fn as_mut(&mut self) -> &mut JAWT {
        &mut self.0
    }

    /// Consumes [Awt] and returns the underlying [JAWT] instance.
    #[inline(always)]
    pub fn into_inner(self) -> JAWT {
        self.0
    }

    #[inline(always)]
    fn find_get_awt(env: &JNIEnv) -> Option<UnsafeAwtGetter> {
        use std::sync::atomic::{AtomicPtr, Ordering};

        fn invalid_get_awt(_: *mut jni::sys::JNIEnv, _: *mut JAWT) -> jboolean {
            0
        }

        const INVALID_GET_AWT: *mut () = invalid_get_awt as _;
        static GET_AWT: AtomicPtr<()> = AtomicPtr::new(INVALID_GET_AWT);

        let cached_get_awt = GET_AWT.load(Ordering::SeqCst);
        if cached_get_awt != INVALID_GET_AWT {
            return Some(unsafe {
                std::mem::transmute::<*mut (), UnsafeAwtGetter>(cached_get_awt)
            });
        }

        let finders: &[unsafe fn(&JNIEnv) -> Option<UnsafeAwtGetter>] = &[
            #[cfg(feature = "dynamic-get-awt")]
            Self::find_dynamic_get_awt,
            #[cfg(feature = "static-get-awt")]
            Self::find_static_get_awt,
        ];
        for finder in finders {
            if let Some(get_awt) = unsafe { finder(env) } {
                let _ = GET_AWT.compare_exchange(
                    INVALID_GET_AWT,
                    get_awt as _,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                return Some(get_awt);
            }
        }

        None
    }

    #[inline(always)]
    #[cfg(feature = "dynamic-get-awt")]
    unsafe fn find_dynamic_get_awt(env: &JNIEnv) -> Option<UnsafeAwtGetter> {
        // Unsafe operations below: the safe JNI wrapper allocates a new `CString` every time a
        // `&str` is passed. We choose to use the unsafe counterpart to prevent it.

        let env = env.get_raw();

        let find_class = unsafe { (**env).FindClass }?;
        let get_static_method_id = unsafe { (**env).GetStaticMethodID }?;
        let call_static_object_method = unsafe { (**env).CallStaticObjectMethod }?;
        let new_string_utf = unsafe { (**env).NewStringUTF }?;

        // C-string literals become stable starting with Rust 1.77
        let system_class = unsafe { find_class(env, b"java/lang/System\0".as_ptr() as _) };
        if system_class.is_null() {
            return None;
        }

        let get_property_method = unsafe {
            get_static_method_id(
                env,
                system_class,
                b"getProperty\0".as_ptr() as _,
                b"(Ljava/lang/String;)Ljava/lang/String;\0".as_ptr() as _,
            )
        };
        if get_property_method.is_null() {
            return None;
        }

        let java_home_string = unsafe { new_string_utf(env, b"java.home\0".as_ptr() as _) };
        if java_home_string.is_null() {
            return None;
        }

        let property_string = unsafe {
            call_static_object_method(env, system_class, get_property_method, java_home_string)
        };
        if property_string.is_null() {
            return None;
        }

        Self::find_awt_from_java_home(env, property_string)
    }

    #[inline(always)]
    #[cfg(all(feature = "dynamic-get-awt", target_family = "windows"))]
    unsafe fn find_awt_from_java_home(
        env: *mut jni::sys::JNIEnv,
        java_home: jstring,
    ) -> Option<UnsafeAwtGetter> {
        use std::ptr;

        use libc::*;
        use windows::core::{PCSTR, PCWSTR};
        use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};

        let get_string_chars = (**env).GetStringChars?;
        let release_string_chars = (**env).ReleaseStringChars?;
        let java_home_chars = get_string_chars(env, java_home, ptr::null_mut());
        let java_home_chars_len = wcslen(java_home_chars);

        let path_suffix = utf16_literal::utf16!("\\bin\\jawt.dll\0");

        let mut jawt_path = Vec::with_capacity(java_home_chars_len + path_suffix.len());

        ptr::copy(java_home_chars, jawt_path.as_mut_ptr(), java_home_chars_len);
        ptr::copy(
            path_suffix.as_ptr() as _,
            jawt_path.as_mut_ptr().add(java_home_chars_len),
            path_suffix.len(),
        );

        jawt_path.set_len(java_home_chars_len + path_suffix.len());

        release_string_chars(env, java_home, java_home_chars);

        let library = LoadLibraryW(PCWSTR(jawt_path.as_mut_ptr() as _)).ok()?;
        let symbol = GetProcAddress(library, PCSTR(b"JAWT_GetAWT\0".as_ptr() as _))?;

        Some(std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            UnsafeAwtGetter,
        >(symbol))
    }

    #[inline(always)]
    #[cfg(all(feature = "dynamic-get-awt", target_family = "unix"))]
    unsafe fn find_awt_from_java_home(
        env: *mut jni::sys::JNIEnv,
        java_home: jstring,
    ) -> Option<UnsafeAwtGetter> {
        use std::ptr;

        use libc::*;

        let get_string_utf_chars = (**env).GetStringUTFChars?;
        let release_string_utf_chars = (**env).ReleaseStringUTFChars?;
        let java_home_chars = get_string_utf_chars(env, java_home, ptr::null_mut());
        let java_home_chars_len = strlen(java_home_chars);

        #[cfg(target_os = "macos")]
        let path_suffix = b"/lib/libjawt.dylib\0";
        #[cfg(not(target_os = "macos"))]
        let path_suffix = b"/lib/libjawt.so\0";

        let mut jawt_path = Vec::with_capacity(java_home_chars_len + path_suffix.len());

        ptr::copy(java_home_chars, jawt_path.as_mut_ptr(), java_home_chars_len);
        ptr::copy(
            path_suffix.as_ptr() as _,
            jawt_path.as_mut_ptr().add(java_home_chars_len),
            path_suffix.len(),
        );

        jawt_path.set_len(java_home_chars_len + path_suffix.len());

        release_string_utf_chars(env, java_home, java_home_chars);

        let handle = dlopen(jawt_path.as_ptr() as _, RTLD_LAZY | RTLD_LOCAL);
        if handle.is_null() {
            dlerror();
            return None;
        }

        let symbol = dlsym(handle, b"JAWT_GetAWT\0".as_ptr() as _);
        if symbol.is_null() {
            dlerror();
            return None;
        }

        Some(std::mem::transmute::<*mut c_void, UnsafeAwtGetter>(symbol))
    }

    #[inline(always)]
    #[cfg(feature = "static-get-awt")]
    fn find_static_get_awt(env: &JNIEnv) -> Option<UnsafeAwtGetter> {
        let _ = env;
        Some(JAWT_GetAWT)
    }

    fn from_version_raw(env: &JNIEnv, version: jint) -> Option<Self> {
        let get_awt = Self::find_get_awt(env)?;
        let mut inner = JAWT {
            version,
            GetDrawingSurface: None,
            FreeDrawingSurface: None,
            Lock: None,
            Unlock: None,
            GetComponent: None,
            CreateEmbeddedFrame: None,
            SetBounds: None,
            SynthesizeWindowActivation: None,
        };
        if unsafe { get_awt(env.get_raw(), &mut inner) } == JNI_FALSE {
            return None;
        }
        Some(Self(inner))
    }

    /// Get the AWT native structure. This function returns [None] if an error
    /// occurs.
    pub fn from_version(env: &JNIEnv, version: AwtVersion) -> Option<Self> {
        Self::from_version_raw(env, version.0)
    }

    #[cfg(target_os = "macos")]
    /// Get the AWT native structure with the [JAWT_MACOSX_USE_CALAYER] flag being set. When you
    /// create an [Awt] instance with a JAWT version less than 1.7, you must call this function or
    /// you will get a [None]. This is to maintain compatibility with applications that used the
    /// interface with Java 6 which had multiple rendering models. This function is not necessary
    /// when JAWT version 1.7 or greater is used as the one using [CALayer] is the only supported
    /// rendering mode.
    ///
    /// [CALayer]: objc2_quartz_core::CALayer
    pub fn from_version_with_ca_layer(env: &JNIEnv, version: AwtVersion) -> Option<Self> {
        Self::from_version_raw(env, version.0 | JAWT_MACOSX_USE_CALAYER as jint)
    }

    /// Version of this structure.
    pub fn version(&self) -> AwtVersion {
        AwtVersion(self.0.version as _)
    }

    /// Return a [DrawingSurface] from a target Java object. This value may be cached. Returns
    /// [None] if an error has occurred. Target must be a [java.awt.Component] (should be a Canvas
    /// or Window for native rendering).
    ///
    /// [java.awt.Component]: https://docs.oracle.com/javase/8/docs/api/java/awt/Component.html
    pub fn drawing_surface(&self, env: &JNIEnv, target: JObject) -> Option<DrawingSurface> {
        let get_drawing_surface = self
            .0
            .GetDrawingSurface
            .expect("JAWT.GetDrawingSurface is not available");
        let free_drawing_surface = self
            .0
            .FreeDrawingSurface
            .expect("JAWT.FreeDrawingSurface is not available");
        let drawing_surface =
            NonNull::new(unsafe { get_drawing_surface(env.get_raw(), target.into_raw()) })?;
        Some(DrawingSurface {
            inner: drawing_surface,
            free: free_drawing_surface,
        })
    }

    #[cfg(feature = "java-1-4")]
    /// Since [1.4](AwtVersion::VERSION_1_4)
    ///
    /// Locks the entire AWT for synchronization purposes.
    ///
    /// # Safety
    ///
    /// After invoking this function, [Awt::unlock] should be called.
    pub unsafe fn lock(&self, env: &JNIEnv) {
        unsafe { (self.0.Lock.expect("JAWT.Lock is not available"))(env.get_raw()) };
    }

    #[cfg(feature = "java-1-4")]
    /// Since [1.4](AwtVersion::VERSION_1_4)
    ///
    /// Unlocks the entire AWT for synchronization purposes.
    ///
    /// # Safety
    ///
    /// [Awt::lock] should be called before invoking this function.
    pub unsafe fn unlock(&self, env: &JNIEnv) {
        unsafe { (self.0.Unlock.expect("JAWT.Unlock is not available"))(env.get_raw()) };
    }
}

impl Awt {
    #[cfg(any(feature = "java-1-4", feature = "java-9"))]
    #[cfg(target_os = "windows")]
    fn lower_platform_info(platform_info: AwtPlatformInfo) -> *mut c_void {
        platform_info.0
    }

    #[cfg(any(feature = "java-1-4", feature = "java-9"))]
    #[cfg(target_os = "macos")]
    fn lower_platform_info(platform_info: AwtPlatformInfo) -> *mut c_void {
        platform_info.as_ptr() as *mut c_void
    }

    #[cfg(any(feature = "java-1-4", feature = "java-9"))]
    #[cfg(all(
        target_family = "unix",
        not(target_vendor = "apple"),
        not(target_os = "android")
    ))]
    fn lower_platform_info(platform_info: AwtPlatformInfo) -> *mut c_void {
        platform_info as *mut c_void
    }
}

impl Awt {
    #[cfg(feature = "java-1-4")]
    /// Since [1.4](AwtVersion::VERSION_1_4)
    ///
    /// Returns a reference to [java.awt.Component] from a native platform handle. On Windows, this
    /// corresponds to an HWND, on Linux, this is a Drawable. For other platforms, see the
    /// [appropriate machine-dependent header file] for a description. The reference returned by
    /// this function is a local reference that is only valid in this environment. This function
    /// returns a [JObject::null()] reference if no component could be found with matching platform
    /// information.
    ///
    /// [java.awt.Component]: https://docs.oracle.com/javase/8/docs/api/java/awt/Component.html
    /// [appropriate machine-dependent header file]: https://github.com/openjdk/jdk/blob/jdk-17%2B35/src/java.desktop/windows/native/include/jawt_md.h
    ///
    /// # Safety
    ///
    /// The caller should ensure that `platform_info` is a valid platform object.
    pub unsafe fn component_of<'env>(
        &self,
        env: &JNIEnv<'env>,
        platform_info: AwtPlatformInfo,
    ) -> JObject<'env> {
        JObject::from_raw((self
            .0
            .GetComponent
            .expect("JAWT.GetComponent is not available"))(
            env.get_raw(),
            Self::lower_platform_info(platform_info),
        ))
    }
}

#[cfg(feature = "java-9")]
pub struct AwtEmbeddedFrame<'a>(pub(crate) JObject<'a>);

#[cfg(feature = "java-9")]
impl<'a> AwtEmbeddedFrame<'a> {
    /// Construct a safe [AwtEmbeddedFrame] from a raw [JObject] instance.
    ///
    /// # Safety
    ///
    /// `inner` must be created using [JAWT::CreateEmbeddedFrame].
    pub unsafe fn from_inner(inner: JObject<'a>) -> Option<Self> {
        if inner.is_null() {
            return None;
        }
        Some(Self(inner))
    }

    /// Consumes [AwtEmbeddedFrame] and returns the underlying [JObject] instance.
    pub const fn into_inner(self) -> JObject<'a> {
        self.0
    }
}

impl Awt {
    #[cfg(feature = "java-9")]
    /// Since [9](AwtVersion::VERSION_9)
    ///
    /// Creates a [java.awt.Frame] placed in a native container. Container is referenced by the
    /// native platform handle. For example on Windows this corresponds to an `HWND`. For other
    /// platforms, see the [appropriate machine-dependent header file] for a description. The
    /// reference returned by this function is a local reference that is only valid in this
    /// environment. This function returns a [JObject::null()] reference if no frame could be
    /// created with matching platform information.
    ///
    /// [java.awt.Frame]: https://docs.oracle.com/en/java/javase/11/docs/api/java.desktop/java/awt/Frame.html
    /// [appropriate machine-dependent header file]: https://github.com/openjdk/jdk/blob/jdk-17%2B35/src/java.desktop/windows/native/include/jawt_md.h
    ///
    /// # Safety
    ///
    /// The caller should ensure that `platform_info` is a valid platform object and alive until the
    /// returned [AwtEmbeddedFrame] drops.
    pub unsafe fn new_embedded_frame<'env>(
        &self,
        env: &JNIEnv<'env>,
        platform_info: AwtPlatformInfo,
    ) -> Option<AwtEmbeddedFrame<'env>> {
        AwtEmbeddedFrame::from_inner(JObject::from_raw((self
            .0
            .CreateEmbeddedFrame
            .expect("JAWT.CreateEmbeddedFrame is not available"))(
            env.get_raw(),
            Self::lower_platform_info(platform_info),
        )))
    }

    #[cfg(feature = "java-9")]
    /// Since [9](AwtVersion::VERSION_9)
    ///
    /// Moves and resizes the embedded frame. The new location of the top-left corner is specified
    /// by x and y parameters relative to the native parent component. The new size is specified by
    /// width and height.
    ///
    /// [java.awt.Component.setLocation()] and [java.awt.Component.setBounds()] for EmbeddedFrame
    /// really don't move it within the native parent. These methods always locate the embedded
    /// frame at (0, 0) for backward compatibility. To allow moving embedded frames this method was
    /// introduced, and it works just the same way as `setLocation()` and `setBounds()` for usual,
    /// non-embedded components.
    ///
    /// Using usual `get/setLocation()` and `get/setBounds()` together with this new
    /// method is not recommended.
    ///
    /// [java.awt.Frame]: https://docs.oracle.com/en/java/javase/11/docs/api/java.desktop/java/awt/Frame.html
    /// [java.awt.Component.setLocation()]: https://docs.oracle.com/en/java/javase/11/docs/api/java.desktop/java/awt/Component.html#setLocation(int,int)
    /// [java.awt.Component.setBounds()]: https://docs.oracle.com/en/java/javase/11/docs/api/java.desktop/java/awt/Component.html#setBounds(int,int,int,int)
    pub fn set_bounds(&self, env: &JNIEnv, embedded_frame: AwtEmbeddedFrame, new_location: Rect) {
        unsafe {
            self.0.SetBounds.expect("JAWT.SetBounds is not available")(
                env.get_raw(),
                embedded_frame.into_inner().into_raw(),
                new_location.x,
                new_location.y,
                new_location.width,
                new_location.height,
            );
        }
    }

    #[cfg(feature = "java-9")]
    /// Since [9](AwtVersion::VERSION_9)
    ///
    /// Synthesize a native message to activate or deactivate an EmbeddedFrame window depending on
    /// the value of parameter `do_activate`, if `true` activates the window; otherwise, deactivates
    /// the window.
    pub fn synthesize_window_activation(
        &self,
        env: &JNIEnv,
        embedded_frame: AwtEmbeddedFrame,
        activate: bool,
    ) {
        unsafe {
            self.0
                .SynthesizeWindowActivation
                .expect("JAWT.SynthesizeWindowActivation is not available")(
                env.get_raw(),
                embedded_frame.into_inner().into_raw(),
                activate as jboolean,
            );
        }
    }
}

impl AsRef<JAWT> for Awt {
    fn as_ref(&self) -> &JAWT {
        &self.0
    }
}
