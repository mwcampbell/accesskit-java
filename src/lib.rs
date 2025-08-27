// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from rustls-ffi.
// Copyright (c) 2021, Jacob Hoffman-Andrews <jsha@letsencrypt.org>
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file), the ISC license (found in
// the LICENSE-ISC file), or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{ActivationHandler, TreeUpdate};
use jni::{
    objects::{GlobalRef, JByteArray, JFloatArray, JObject},
    sys::{jfloat, jlong},
    JNIEnv, JavaVM,
};

mod common;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub use common::*;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
pub use windows::*;

pub(crate) fn into_jptr<T>(source: T) -> jlong {
    Box::into_raw(Box::new(source)) as jlong
}

pub(crate) fn mut_from_jptr<'a, T>(ptr: jlong) -> &'a mut T {
    unsafe { &mut *(ptr as *mut T) }
}

pub(crate) fn box_from_jptr<T>(ptr: jlong) -> Box<T> {
    unsafe { Box::from_raw(ptr as *mut T) }
}

pub(crate) fn box_str_from_utf8_jbytes(env: &JNIEnv, bytes: JByteArray) -> Box<str> {
    let bytes = env.convert_byte_array(bytes).unwrap();
    unsafe { String::from_utf8_unchecked(bytes) }.into()
}

pub(crate) fn convert_float_array(
    env: &JNIEnv,
    value: JFloatArray,
) -> jni::errors::Result<Vec<jfloat>> {
    let len = env.get_array_length(&value)? as usize;
    let mut buf = vec![0.0; len];
    env.get_float_array_region(value, 0, &mut buf)?;
    Ok(buf)
}

pub(crate) struct JavaActivationHandler {
    jvm: JavaVM,
    initial_tree_supplier: GlobalRef,
}

impl JavaActivationHandler {
    pub(crate) fn new(env: &JNIEnv, initial_tree_supplier: JObject) -> Self {
        let jvm = env.get_java_vm().unwrap();
        let initial_tree_supplier = env.new_global_ref(initial_tree_supplier).unwrap();
        Self {
            jvm,
            initial_tree_supplier,
        }
    }
}

impl ActivationHandler for JavaActivationHandler {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        let mut env = self.jvm.attach_current_thread().unwrap();
        let ptr = env
            .call_method(&self.initial_tree_supplier, "get", "()J", &[])
            .unwrap()
            .j()
            .unwrap();
        (ptr != 0).then(|| *box_from_jptr::<TreeUpdate>(ptr))
    }
}
