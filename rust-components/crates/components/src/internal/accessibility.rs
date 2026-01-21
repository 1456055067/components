// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Accessibility utilities
//!
//! Provides ARIA attribute handling, focus management, and other
//! accessibility-related functionality.

/// ARIA attributes for components
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AriaAttributes {
    pub label: Option<String>,
    pub labelledby: Option<String>,
    pub describedby: Option<String>,
    pub controls: Option<String>,
    pub expanded: Option<bool>,
    pub haspopup: Option<AriaHasPopup>,
    pub hidden: Option<bool>,
    pub live: Option<AriaLive>,
    pub atomic: Option<bool>,
    pub relevant: Option<String>,
    pub busy: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub invalid: Option<bool>,
    pub pressed: Option<bool>,
    pub checked: Option<AriaChecked>,
    pub selected: Option<bool>,
    pub current: Option<AriaCurrent>,
}

/// ARIA haspopup values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaHasPopup {
    True,
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
}

impl AriaHasPopup {
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaHasPopup::True => "true",
            AriaHasPopup::Menu => "menu",
            AriaHasPopup::Listbox => "listbox",
            AriaHasPopup::Tree => "tree",
            AriaHasPopup::Grid => "grid",
            AriaHasPopup::Dialog => "dialog",
        }
    }
}

/// ARIA live region politeness levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaLive {
    Off,
    Polite,
    Assertive,
}

impl AriaLive {
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaLive::Off => "off",
            AriaLive::Polite => "polite",
            AriaLive::Assertive => "assertive",
        }
    }
}

/// ARIA checked state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaChecked {
    True,
    False,
    Mixed,
}

impl AriaChecked {
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaChecked::True => "true",
            AriaChecked::False => "false",
            AriaChecked::Mixed => "mixed",
        }
    }
}

/// ARIA current values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaCurrent {
    Page,
    Step,
    Location,
    Date,
    Time,
    True,
}

impl AriaCurrent {
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaCurrent::Page => "page",
            AriaCurrent::Step => "step",
            AriaCurrent::Location => "location",
            AriaCurrent::Date => "date",
            AriaCurrent::Time => "time",
            AriaCurrent::True => "true",
        }
    }
}

/// Focus options for programmatic focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FocusOptions {
    pub prevent_scroll: bool,
}

impl FocusOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_prevent_scroll(mut self, prevent: bool) -> Self {
        self.prevent_scroll = prevent;
        self
    }
}

/// Keyboard navigation helper
pub struct KeyboardNavigation;

impl KeyboardNavigation {
    /// Arrow keys
    pub const ARROW_UP: &'static str = "ArrowUp";
    pub const ARROW_DOWN: &'static str = "ArrowDown";
    pub const ARROW_LEFT: &'static str = "ArrowLeft";
    pub const ARROW_RIGHT: &'static str = "ArrowRight";

    /// Action keys
    pub const ENTER: &'static str = "Enter";
    pub const SPACE: &'static str = " ";
    pub const ESCAPE: &'static str = "Escape";
    pub const TAB: &'static str = "Tab";

    /// Home/End
    pub const HOME: &'static str = "Home";
    pub const END: &'static str = "End";
    pub const PAGE_UP: &'static str = "PageUp";
    pub const PAGE_DOWN: &'static str = "PageDown";

    /// Checks if a key is an arrow key
    pub fn is_arrow_key(key: &str) -> bool {
        matches!(
            key,
            Self::ARROW_UP | Self::ARROW_DOWN | Self::ARROW_LEFT | Self::ARROW_RIGHT
        )
    }

    /// Checks if a key is an action key (Enter or Space)
    pub fn is_action_key(key: &str) -> bool {
        matches!(key, Self::ENTER | Self::SPACE)
    }

    /// Checks if a key is a navigation key
    pub fn is_navigation_key(key: &str) -> bool {
        Self::is_arrow_key(key)
            || matches!(
                key,
                Self::HOME | Self::END | Self::PAGE_UP | Self::PAGE_DOWN | Self::TAB
            )
    }
}

/// Screen reader text helper
///
/// Creates visually hidden text that's accessible to screen readers
pub struct ScreenReaderOnly;

impl ScreenReaderOnly {
    /// CSS class name for screen reader only content
    pub const CLASS: &'static str = "awsui-screen-reader-only";

    /// Inline styles for screen reader only content
    pub const STYLES: &'static str = "\
        position:absolute;\
        left:-10000px;\
        width:1px;\
        height:1px;\
        overflow:hidden;\
    ";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aria_haspopup() {
        assert_eq!(AriaHasPopup::Menu.as_str(), "menu");
        assert_eq!(AriaHasPopup::Dialog.as_str(), "dialog");
    }

    #[test]
    fn test_aria_live() {
        assert_eq!(AriaLive::Polite.as_str(), "polite");
        assert_eq!(AriaLive::Assertive.as_str(), "assertive");
    }

    #[test]
    fn test_aria_checked() {
        assert_eq!(AriaChecked::True.as_str(), "true");
        assert_eq!(AriaChecked::Mixed.as_str(), "mixed");
    }

    #[test]
    fn test_focus_options() {
        let opts = FocusOptions::new().with_prevent_scroll(true);
        assert!(opts.prevent_scroll);
    }

    #[test]
    fn test_keyboard_navigation() {
        assert!(KeyboardNavigation::is_arrow_key("ArrowUp"));
        assert!(KeyboardNavigation::is_arrow_key("ArrowDown"));
        assert!(!KeyboardNavigation::is_arrow_key("Enter"));

        assert!(KeyboardNavigation::is_action_key("Enter"));
        assert!(KeyboardNavigation::is_action_key(" "));
        assert!(!KeyboardNavigation::is_action_key("Escape"));

        assert!(KeyboardNavigation::is_navigation_key("ArrowLeft"));
        assert!(KeyboardNavigation::is_navigation_key("Home"));
        assert!(KeyboardNavigation::is_navigation_key("Tab"));
    }

    #[test]
    fn test_aria_attributes_default() {
        let attrs = AriaAttributes::default();
        assert!(attrs.label.is_none());
        assert!(attrs.expanded.is_none());
    }
}
