// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::*;
use accesskit_windows::*;
use jni::{
    objects::{JClass, JObject},
    sys::jlong,
    JNIEnv,
};

use crate::{box_from_jptr, into_jptr, mut_from_jptr, JavaActivationHandler};

// TODO: eliminate the need for this
struct NullActionHandler;

impl ActionHandler for NullActionHandler {
    fn do_action(&mut self, _request: ActionRequest) {}
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_WindowsSubclassingAdapter_nativeNew(
    env: JNIEnv,
    _class: JClass,
    hwnd: jlong,
    initial_tree_supplier: JObject,
) -> jlong {
    let hwnd = HWND(hwnd as _);
    let activation_handler = JavaActivationHandler::new(&env, initial_tree_supplier);
    // TODO: real action handler
    let adapter = SubclassingAdapter::new(hwnd, activation_handler, NullActionHandler {});
    into_jptr(adapter)
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_WindowsSubclassingAdapter_nativeDrop(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    drop(box_from_jptr::<SubclassingAdapter>(ptr));
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_WindowsSubclassingAdapter_nativeUpdateIfActive(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    update_supplier: JObject,
) {
    let adapter = mut_from_jptr::<SubclassingAdapter>(ptr);
    let update_source = move || {
        let ptr = env
            .call_method(&update_supplier, "get", "()J", &[])
            .unwrap()
            .j()
            .unwrap();
        *box_from_jptr::<TreeUpdate>(ptr)
    };
    if let Some(events) = adapter.update_if_active(update_source) {
        events.raise();
    }
}
