// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::*;
use jni::{
    objects::{JByteArray, JClass, JFloatArray},
    sys::{jdouble, jint, jlong},
    JNIEnv,
};

use crate::{
    box_from_jptr, box_str_from_utf8_jbytes, convert_float_array, into_jptr, mut_from_jptr,
};

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeNew(
    _env: JNIEnv,
    _class: JClass,
    role: jint,
) -> jlong {
    let role = Role::n(role as u8).unwrap();
    let node = Node::new(role);
    into_jptr(node)
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeDrop(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    drop(box_from_jptr::<Node>(ptr));
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeAddAction(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    action: jint,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let action = Action::n(action as u8).unwrap();
    node.add_action(action);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetLabel(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: JByteArray,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = box_str_from_utf8_jbytes(&env, value);
    node.set_label(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetValue(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: JByteArray,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = box_str_from_utf8_jbytes(&env, value);
    node.set_value(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetBounds(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    x0: jdouble,
    y0: jdouble,
    x1: jdouble,
    y1: jdouble,
) {
    let node = mut_from_jptr::<Node>(ptr);
    node.set_bounds(Rect { x0, y0, x1, y1 })
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeAddChild(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    id: jlong,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let id = NodeId(id as _);
    node.push_child(id);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeClearChildren(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    let node = mut_from_jptr::<Node>(ptr);
    node.clear_children();
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetToggled(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jint,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = Toggled::n(value as u8).unwrap();
    node.set_toggled(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetLive(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jint,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = Live::n(value as u8).unwrap();
    node.set_live(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetTextDirection(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jint,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = TextDirection::n(value as u8).unwrap();
    node.set_text_direction(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetNumericValue(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jdouble,
) {
    let node = mut_from_jptr::<Node>(ptr);
    node.set_numeric_value(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetMinNumericValue(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jdouble,
) {
    let node = mut_from_jptr::<Node>(ptr);
    node.set_min_numeric_value(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetMaxNumericValue(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jdouble,
) {
    let node = mut_from_jptr::<Node>(ptr);
    node.set_max_numeric_value(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetNumericValueStep(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jdouble,
) {
    let node = mut_from_jptr::<Node>(ptr);
    node.set_numeric_value_step(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetNumericValueJump(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jdouble,
) {
    let node = mut_from_jptr::<Node>(ptr);
    node.set_numeric_value_jump(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetTextSelection(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    anchor_id: jlong,
    anchor_character_index: jint,
    focus_id: jlong,
    focus_character_index: jint,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let anchor_id = NodeId(anchor_id as _);
    let focus_id = NodeId(focus_id as _);
    node.set_text_selection(TextSelection {
        anchor: TextPosition {
            node: anchor_id,
            character_index: anchor_character_index as usize,
        },
        focus: TextPosition {
            node: focus_id,
            character_index: focus_character_index as usize,
        },
    });
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetCharacterLengths(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: JByteArray,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = env.convert_byte_array(value).unwrap();
    node.set_character_lengths(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetWordLengths(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: JByteArray,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = env.convert_byte_array(value).unwrap();
    node.set_word_lengths(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetCharacterPositions(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: JFloatArray,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = convert_float_array(&env, value).unwrap();
    node.set_character_positions(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_Node_nativeSetCharacterWidths(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: JFloatArray,
) {
    let node = mut_from_jptr::<Node>(ptr);
    let value = convert_float_array(&env, value).unwrap();
    node.set_character_widths(value);
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_TreeUpdate_nativeWithFocus(
    _env: JNIEnv,
    _class: JClass,
    focus: jlong,
) -> jlong {
    let focus = NodeId(focus as _);
    let update = TreeUpdate {
        nodes: vec![],
        tree: None,
        focus,
    };
    into_jptr(update)
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_TreeUpdate_nativeDrop(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    drop(box_from_jptr::<TreeUpdate>(ptr));
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_TreeUpdate_nativeAddNode(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    id: jlong,
    node_ptr: jlong,
) {
    let update = mut_from_jptr::<TreeUpdate>(ptr);
    let id = NodeId(id as _);
    let node = box_from_jptr::<Node>(node_ptr);
    update.nodes.push((id, *node));
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_TreeUpdate_nativeSetTree(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    root: jlong,
) {
    let update = mut_from_jptr::<TreeUpdate>(ptr);
    let root = NodeId(root as _);
    update.tree = Some(Tree::new(root));
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_TreeUpdate_nativeClearTree(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    let update = mut_from_jptr::<TreeUpdate>(ptr);
    update.tree = None;
}

#[no_mangle]
pub extern "system" fn Java_dev_accesskit_TreeUpdate_nativeSetFocus(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    id: jlong,
) {
    let update = mut_from_jptr::<TreeUpdate>(ptr);
    let id = NodeId(id as _);
    update.focus = id;
}
