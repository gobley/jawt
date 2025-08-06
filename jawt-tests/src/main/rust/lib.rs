// Copyright (c) 2025 Gobley Contributors.

use std::ffi::c_void;

use jni::objects::JClass;
use jni::sys::{jint, JNI_VERSION_1_8};
use jni::{JNIEnv, JavaVM};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: JavaVM, _reserved: *const c_void) -> jint {
    env_logger::init();
    log::error!("JNI_OnLoad() invoked successfully");
    JNI_VERSION_1_8
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_dev_gobley_jawt_tests_Rust_add(
    _env: JNIEnv,
    _class: JClass,
    lhs: jint,
    rhs: jint,
) -> jint {
    log::error!(
        "dev.gobley.jawt.tests.Rust.add({lhs}, {rhs}) = {}",
        lhs + rhs
    );
    lhs + rhs
}
