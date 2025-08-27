// Copyright 2023 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

package dev.accesskit;

public enum Action {
    /**
     * Do the equivalent of a single click or tap.
     */
    CLICK,
    FOCUS,
    BLUR,
    COLLAPSE,
    EXPAND,
    /**
     * Requires [`ActionRequest::data`] to be set to [`ActionData::CustomAction`].
     */
    CUSTOM_ACTION,
    /**
     * Decrement a numeric value by one step.
     */
    DECREMENT,
    /**
     * Increment a numeric value by one step.
     */
    INCREMENT,
    HIDE_TOOLTIP,
    SHOW_TOOLTIP,
    /**
     * Delete any selected text in the control's text value and
     * insert the specified value in its place, like when typing or pasting.
     * Requires [`ActionRequest::data`] to be set to [`ActionData::Value`].
     */
    REPLACE_SELECTED_TEXT,
    /**
     * Scroll down by the specified unit.
     */
    SCROLL_DOWN,
    /**
     * Scroll left by the specified unit.
     */
    SCROLL_LEFT,
    /**
     * Scroll right by the specified unit.
     */
    SCROLL_RIGHT,
    /**
     * Scroll up by the specified unit.
     */
    SCROLL_UP,
    /**
     * Scroll any scrollable containers to make the target node visible.
     * Optionally set [`ActionRequest::data`] to [`ActionData::ScrollHint`].
     */
    SCROLL_INTO_VIEW,
    /**
     * Scroll the given object to a specified point in the tree's container
     * (e.g. window). Requires [`ActionRequest::data`] to be set to
     * [`ActionData::ScrollToPoint`].
     */
    SCROLL_TO_POINT,
    /**
     * Requires [`ActionRequest::data`] to be set to
     * [`ActionData::SetScrollOffset`].
     */
    SET_SCROLL_OFFSET,
    /**
     * Requires [`ActionRequest::data`] to be set to
     * [`ActionData::SetTextSelection`].
     */
    SET_TEXT_SELECTION,
    /**
     * Don't focus this node, but set it as the sequential focus navigation
     * starting point, so that pressing Tab moves to the next element
     * following this one, for example.
     */
    SET_SEQUENTIAL_FOCUS_NAVIGATION_STARTING_POINT,
    /**
     * Replace the value of the control with the specified value and
     * reset the selection, if applicable. Requires [`ActionRequest::data`]
     * to be set to [`ActionData::Value`] or [`ActionData::NumericValue`].
     */
    SET_VALUE,
    SHOW_CONTEXT_MENU,
}
