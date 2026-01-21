// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Component styling system
//!
//! Provides utilities for managing component styles, CSS custom properties,
//! and integrating with the shared design token system.

use super::props::StyleOverride;
use std::collections::HashMap;

/// Component-specific style configuration
///
/// This allows components to define their own CSS custom property mappings
/// and integrate with the design token system.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentStyles {
    /// CSS class names for the component
    pub classes: Vec<String>,

    /// CSS custom property overrides
    pub custom_properties: HashMap<String, String>,

    /// Inline style attribute value
    pub inline_style: Option<String>,
}

impl ComponentStyles {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
            custom_properties: HashMap::new(),
            inline_style: None,
        }
    }

    /// Adds a CSS class
    pub fn add_class(&mut self, class: impl Into<String>) -> &mut Self {
        self.classes.push(class.into());
        self
    }

    /// Sets a CSS custom property
    pub fn set_property(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.custom_properties.insert(name.into(), value.into());
        self
    }

    /// Merges a style override into this component's styles
    pub fn merge_override(&mut self, override_style: &StyleOverride) -> &mut Self {
        let inline = override_style.to_inline_style();
        if !inline.is_empty() {
            self.inline_style = Some(inline);
        }
        self
    }

    /// Gets the class attribute value
    pub fn class_attr(&self) -> String {
        self.classes.join(" ")
    }

    /// Gets the style attribute value (combining custom properties and inline styles)
    pub fn style_attr(&self) -> Option<String> {
        let mut styles = Vec::new();

        // Add custom properties
        for (name, value) in &self.custom_properties {
            styles.push(format!("{}:{}", name, value));
        }

        // Add inline style if present
        if let Some(ref inline) = self.inline_style {
            styles.push(inline.clone());
        }

        if styles.is_empty() {
            None
        } else {
            Some(styles.join(";"))
        }
    }
}

impl Default for ComponentStyles {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS custom property name constants
///
/// These will be populated by the design token generator
/// to match the React implementation's custom CSS properties.
pub struct CssProperties;

impl CssProperties {
    // Badge properties
    pub const BADGE_BACKGROUND: &'static str = "--awsui-badge-background";
    pub const BADGE_COLOR: &'static str = "--awsui-badge-color";
    pub const BADGE_BORDER_COLOR: &'static str = "--awsui-badge-border-color";
    pub const BADGE_BORDER_RADIUS: &'static str = "--awsui-badge-border-radius";
    pub const BADGE_BORDER_WIDTH: &'static str = "--awsui-badge-border-width";
    pub const BADGE_PADDING_BLOCK: &'static str = "--awsui-badge-padding-block";
    pub const BADGE_PADDING_INLINE: &'static str = "--awsui-badge-padding-inline";

    // Button properties
    pub const BUTTON_BACKGROUND_DEFAULT: &'static str = "--awsui-button-background-default";
    pub const BUTTON_BACKGROUND_HOVER: &'static str = "--awsui-button-background-hover";
    pub const BUTTON_BACKGROUND_ACTIVE: &'static str = "--awsui-button-background-active";
    pub const BUTTON_BACKGROUND_DISABLED: &'static str = "--awsui-button-background-disabled";
    pub const BUTTON_COLOR_DEFAULT: &'static str = "--awsui-button-color-default";
    pub const BUTTON_COLOR_HOVER: &'static str = "--awsui-button-color-hover";
    pub const BUTTON_COLOR_ACTIVE: &'static str = "--awsui-button-color-active";
    pub const BUTTON_COLOR_DISABLED: &'static str = "--awsui-button-color-disabled";
    pub const BUTTON_BORDER_COLOR_DEFAULT: &'static str = "--awsui-button-border-color-default";
    pub const BUTTON_BORDER_RADIUS: &'static str = "--awsui-button-border-radius";
    pub const BUTTON_BORDER_WIDTH: &'static str = "--awsui-button-border-width";
    pub const BUTTON_PADDING_BLOCK: &'static str = "--awsui-button-padding-block";
    pub const BUTTON_PADDING_INLINE: &'static str = "--awsui-button-padding-inline";

    // Focus ring properties
    pub const FOCUS_RING_BORDER_COLOR: &'static str = "--awsui-focus-ring-border-color";
    pub const FOCUS_RING_BORDER_WIDTH: &'static str = "--awsui-focus-ring-border-width";
    pub const FOCUS_RING_BORDER_RADIUS: &'static str = "--awsui-focus-ring-border-radius";
    pub const FOCUS_RING_BOX_SHADOW: &'static str = "--awsui-focus-ring-box-shadow";
}

/// Badge-specific style structure matching React's BadgeProps.Style
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BadgeStyle {
    pub background: Option<String>,
    pub border_color: Option<String>,
    pub border_radius: Option<String>,
    pub border_width: Option<String>,
    pub color: Option<String>,
    pub padding_block: Option<String>,
    pub padding_inline: Option<String>,
}

impl BadgeStyle {
    pub fn to_style_override(&self) -> StyleOverride {
        let mut override_style = StyleOverride::new();

        if let Some(ref bg) = self.background {
            override_style.set(CssProperties::BADGE_BACKGROUND, bg);
        }
        if let Some(ref color) = self.color {
            override_style.set(CssProperties::BADGE_COLOR, color);
        }
        if let Some(ref border_color) = self.border_color {
            override_style.set(CssProperties::BADGE_BORDER_COLOR, border_color);
        }
        if let Some(ref border_radius) = self.border_radius {
            override_style.set(CssProperties::BADGE_BORDER_RADIUS, border_radius);
        }
        if let Some(ref border_width) = self.border_width {
            override_style.set(CssProperties::BADGE_BORDER_WIDTH, border_width);
        }
        if let Some(ref padding_block) = self.padding_block {
            override_style.set(CssProperties::BADGE_PADDING_BLOCK, padding_block);
        }
        if let Some(ref padding_inline) = self.padding_inline {
            override_style.set(CssProperties::BADGE_PADDING_INLINE, padding_inline);
        }

        override_style
    }
}

/// Button-specific style structure matching React's ButtonProps.Style
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonStyle {
    pub background: Option<ButtonStateColors>,
    pub color: Option<ButtonStateColors>,
    pub border_color: Option<ButtonStateColors>,
    pub border_radius: Option<String>,
    pub border_width: Option<String>,
    pub padding_block: Option<String>,
    pub padding_inline: Option<String>,
}

/// State-based color configuration (default, hover, active, disabled)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonStateColors {
    pub default: Option<String>,
    pub hover: Option<String>,
    pub active: Option<String>,
    pub disabled: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_styles() {
        let mut styles = ComponentStyles::new();
        styles.add_class("base-class");
        styles.add_class("variant-class");
        styles.set_property("--custom-color", "#ff0000");

        assert_eq!(styles.class_attr(), "base-class variant-class");

        let style_attr = styles.style_attr();
        assert!(style_attr.is_some());
        assert!(style_attr.unwrap().contains("--custom-color:#ff0000"));
    }

    #[test]
    fn test_badge_style_conversion() {
        let badge_style = BadgeStyle {
            background: Some("#ff0000".to_string()),
            color: Some("#ffffff".to_string()),
            ..Default::default()
        };

        let override_style = badge_style.to_style_override();
        assert_eq!(
            override_style.get(CssProperties::BADGE_BACKGROUND),
            Some(&"#ff0000".to_string())
        );
        assert_eq!(
            override_style.get(CssProperties::BADGE_COLOR),
            Some(&"#ffffff".to_string())
        );
    }

    #[test]
    fn test_merge_override() {
        let mut styles = ComponentStyles::new();
        let mut override_style = StyleOverride::new();
        override_style.set("--test-prop", "test-value");

        styles.merge_override(&override_style);

        let style_attr = styles.style_attr();
        assert!(style_attr.is_some());
        assert!(style_attr.unwrap().contains("--test-prop:test-value"));
    }
}
