// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Custom event types for Cloudscape components
//!
//! Provides event structures that mirror the React implementation's
//! CancelableEventHandler and event detail types.

use web_sys::{KeyboardEvent, MouseEvent};

/// Custom event wrapper that can be prevented
#[derive(Debug, Clone)]
pub struct CustomEvent<T> {
    pub detail: T,
    pub cancelable: bool,
    pub default_prevented: bool,
}

impl<T> CustomEvent<T> {
    /// Creates a new cancelable event
    pub fn new(detail: T) -> Self {
        Self {
            detail,
            cancelable: true,
            default_prevented: false,
        }
    }

    /// Creates a new non-cancelable event
    pub fn new_non_cancelable(detail: T) -> Self {
        Self {
            detail,
            cancelable: false,
            default_prevented: false,
        }
    }

    /// Prevents the default action
    pub fn prevent_default(&mut self) {
        if self.cancelable {
            self.default_prevented = true;
        }
    }
}

/// Click event detail matching React implementation
#[derive(Debug, Clone, Default)]
pub struct ClickDetail {
    pub button: u16,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

impl From<&MouseEvent> for ClickDetail {
    fn from(event: &MouseEvent) -> Self {
        Self {
            button: event.button() as u16,
            ctrl_key: event.ctrl_key(),
            shift_key: event.shift_key(),
            alt_key: event.alt_key(),
            meta_key: event.meta_key(),
        }
    }
}

/// Click event type
pub type ClickEvent = CustomEvent<ClickDetail>;

impl ClickEvent {
    /// Creates a click event from a MouseEvent
    pub fn from_mouse_event(event: &MouseEvent) -> Self {
        Self::new(ClickDetail::from(event))
    }

    /// Checks if this was a plain left click (no modifiers)
    pub fn is_plain_left_click(&self) -> bool {
        self.detail.button == 0
            && !self.detail.ctrl_key
            && !self.detail.shift_key
            && !self.detail.alt_key
            && !self.detail.meta_key
    }

    /// Checks if any modifier keys are pressed
    pub fn has_modifier_keys(&self) -> bool {
        self.detail.ctrl_key
            || self.detail.shift_key
            || self.detail.alt_key
            || self.detail.meta_key
    }
}

/// Follow event detail for anchor buttons
#[derive(Debug, Clone)]
pub struct FollowDetail {
    pub href: Option<String>,
    pub external: bool,
    pub target: Option<String>,
}

/// Follow event type
pub type FollowEvent = CustomEvent<FollowDetail>;

/// Keyboard event detail
#[derive(Debug, Clone)]
pub struct KeyDetail {
    pub key: String,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
}

impl From<&KeyboardEvent> for KeyDetail {
    fn from(event: &KeyboardEvent) -> Self {
        Self {
            key: event.key(),
            ctrl_key: event.ctrl_key(),
            shift_key: event.shift_key(),
            alt_key: event.alt_key(),
            meta_key: event.meta_key(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_event_prevent_default() {
        let mut event = CustomEvent::new(ClickDetail::default());
        assert!(!event.default_prevented);

        event.prevent_default();
        assert!(event.default_prevented);
    }

    #[test]
    fn test_non_cancelable_event() {
        let mut event = CustomEvent::new_non_cancelable(ClickDetail::default());
        event.prevent_default();
        assert!(!event.default_prevented); // Should not be prevented
    }

    #[test]
    fn test_click_detail_default() {
        let detail = ClickDetail::default();
        let event = ClickEvent::new(detail);

        assert!(event.is_plain_left_click());
        assert!(!event.has_modifier_keys());
    }

    #[test]
    fn test_click_with_modifiers() {
        let detail = ClickDetail {
            button: 0,
            ctrl_key: true,
            ..Default::default()
        };
        let event = ClickEvent::new(detail);

        assert!(!event.is_plain_left_click());
        assert!(event.has_modifier_keys());
    }
}
