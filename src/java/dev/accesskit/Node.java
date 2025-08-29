// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

package dev.accesskit;

public final class Node {
    public Node(Role role) {
        ptr = nativeNew(role.ordinal());
    }

    /**
     * Releases resources associated with this object. In normal usage,
     * you don't need to call this method, since the node is consumed
     * when you add it to a tree update.
     */
    public void drop() {
        if (ptr != 0) {
            nativeDrop(ptr);
            ptr = 0;
        }
    }

    public void addAction(Action action) {
        checkActive();
        nativeAddAction(ptr, action.ordinal());
    }

    public void setLabel(String value) {
        checkActive();
        nativeSetLabel(ptr, Util.bytesFromString(value));
    }

    public void setDescription(String value) {
        checkActive();
        nativeSetDescription(ptr, Util.bytesFromString(value));
    }

    public void setValue(String value) {
        checkActive();
        nativeSetValue(ptr, Util.bytesFromString(value));
    }

    public void setAccessKey(String value) {
        checkActive();
        nativeSetAccessKey(ptr, Util.bytesFromString(value));
    }

    public void setKeyboardShortcut(String value) {
        checkActive();
        nativeSetKeyboardShortcut(ptr, Util.bytesFromString(value));
    }

    public void setBounds(Rect value) {
        checkActive();
        nativeSetBounds(ptr, value.x0, value.y0, value.x1, value.y1);
    }

    public void addChild(NodeId id) {
        checkActive();
        nativeAddChild(ptr, id.value);
    }

    public void clearChildren() {
        checkActive();
        nativeClearChildren(ptr);
    }

    public void setChildren(Iterable<NodeId> ids) {
        clearChildren();
        for (NodeId id : ids) {
            addChild(id);
        }
    }

    public void setToggled(Toggled value) {
        checkActive();
        nativeSetToggled(ptr, value.ordinal());
    }

    public void setLive(Live value) {
        checkActive();
        nativeSetLive(ptr, value.ordinal());
    }

    public void setTextDirection(TextDirection value) {
        checkActive();
        nativeSetTextDirection(ptr, value.ordinal());
    }

    public void setNumericValue(double value) {
        checkActive();
        nativeSetNumericValue(ptr, value);
    }

    public void setMinNumericValue(double value) {
        checkActive();
        nativeSetMinNumericValue(ptr, value);
    }

    public void setMaxNumericValue(double value) {
        checkActive();
        nativeSetMaxNumericValue(ptr, value);
    }

    public void setNumericValueStep(double value) {
        checkActive();
        nativeSetNumericValueStep(ptr, value);
    }

    public void setNumericValueJump(double value) {
        checkActive();
        nativeSetNumericValueJump(ptr, value);
    }

    public void setTextSelection(TextSelection value) {
        checkActive();
        nativeSetTextSelection(ptr, value.anchor.node.value, value.anchor.characterIndex, value.focus.node.value, value.focus.characterIndex);
    }

    public void setCharacterLengths(byte[] value) {
        checkActive();
        nativeSetCharacterLengths(ptr, value);
    }

    public void setWordLengths(byte[] value) {
        checkActive();
        nativeSetWordLengths(ptr, value);
    }

    public void setCharacterPositions(float[] value) {
        checkActive();
        nativeSetCharacterPositions(ptr, value);
    }

    public void setCharacterWidths(float[] value) {
        checkActive();
        nativeSetCharacterWidths(ptr, value);
    }

    public void setPositionInSet(int value) {
        checkActive();
        nativeSetPositionInSet(ptr, value);
    }

    public void setSizeOfSet(int value) {
        checkActive();
        nativeSetSizeOfSet(ptr, value);
    }

    long ptr;
    private static native long nativeNew(int role);
    private static native void nativeDrop(long ptr);
    private static native void nativeAddAction(long ptr, int action);
    private static native void nativeSetLabel(long ptr, byte[] value);
    private static native void nativeSetDescription(long ptr, byte[] value);
    private static native void nativeSetValue(long ptr, byte[] value);
    private static native void nativeSetAccessKey(long ptr, byte[] value);
    private static native void nativeSetKeyboardShortcut(long ptr, byte[] value);
    private static native void nativeSetBounds(long ptr, double x0, double y0, double x1, double y1);
    private static native void nativeAddChild(long ptr, long id);
    private static native void nativeClearChildren(long ptr);
    private static native void nativeSetToggled(long ptr, int value);
    private static native void nativeSetLive(long ptr, int value);
    private static native void nativeSetTextDirection(long ptr, int value);
    private static native void nativeSetNumericValue(long ptr, double value);
    private static native void nativeSetMinNumericValue(long ptr, double value);
    private static native void nativeSetMaxNumericValue(long ptr, double value);
    private static native void nativeSetNumericValueStep(long ptr, double value);
    private static native void nativeSetNumericValueJump(long ptr, double value);
    private static native void nativeSetTextSelection(long ptr, long anchorNodeId, int anchorCharacterIndex, long focusNodeId, int focusCharacterIndex);
    private static native void nativeSetCharacterLengths(long ptr, byte[] value);
    private static native void nativeSetWordLengths(long ptr, byte[] value);
    private static native void nativeSetCharacterPositions(long ptr, float[] value);
    private static native void nativeSetCharacterWidths(long ptr, float[] value);
    private static native void nativeSetPositionInSet(long ptr, int value);
    private static native void nativeSetSizeOfSet(long ptr, int value);

    void checkActive() {
        Util.checkActive(ptr);
    }
}
