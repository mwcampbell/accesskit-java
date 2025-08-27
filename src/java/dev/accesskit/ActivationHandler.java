// Copyright 2025 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

package dev.accesskit;

/**
 * Handles activation of the application's accessibility implementation.
 */
public interface ActivationHandler {
    /**
     * Requests a TreeUpdate with a full tree. If the application
     * can generate the tree synchronously within this method call,
     * it should do so and return the TreeUpdate. Otherwise, it should return
     * null and must send the update to the platform adapter asynchronously,
     * no later than the next display refresh, even if a frame would not
     * normally be rendered due to user input or other activity.
     * The application should not return or send a placeholder TreeUpdate;
     * the platform adapter will provide one if necessary until the real
     * tree is sent.
     * <p>
     * The primary purpose of this method is to allow the application
     * to lazily initialize its accessibility implementation. However,
     * this method may be called consecutively without any call to
     * DeactivationHandler.deactivateAccessibility; this typically happens
     * if the platform adapter merely forwards tree updates to assistive
     * technologies without maintaining any state. A call to this method
     * must always generate a TreeUpdate with a full tree, even if
     * the application normally sends incremental updates.
     * <p>
     * The thread on which this method is called is platform-dependent.
     * Refer to the platform adapter documentation for more details.
     */
    TreeUpdate requestInitialTree();
}
