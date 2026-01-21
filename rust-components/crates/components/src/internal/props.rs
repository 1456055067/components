// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Advanced property handling utilities
//!
//! Provides patterns for handling optional props, style overrides,
//! and other advanced property scenarios.

use std::collections::HashMap;

/// Style override for component customization
///
/// Mirrors the React implementation's style prop pattern,
/// allowing users to override CSS custom properties.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleOverride {
    properties: HashMap<String, String>,
}

impl StyleOverride {
    /// Creates a new empty style override
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a CSS custom property value
    pub fn set(&mut self, property: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.properties.insert(property.into(), value.into());
        self
    }

    /// Gets a CSS custom property value
    pub fn get(&self, property: &str) -> Option<&String> {
        self.properties.get(property)
    }

    /// Converts to CSS inline style string
    pub fn to_inline_style(&self) -> String {
        self.properties
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join(";")
    }

    /// Checks if any overrides are present
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

/// Builder pattern for creating style overrides
pub struct StyleOverrideBuilder {
    override_style: StyleOverride,
}

impl StyleOverrideBuilder {
    pub fn new() -> Self {
        Self {
            override_style: StyleOverride::new(),
        }
    }

    pub fn property(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.override_style.set(name, value);
        self
    }

    pub fn build(self) -> StyleOverride {
        self.override_style
    }
}

impl Default for StyleOverrideBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Native HTML attributes wrapper
///
/// Allows passing through arbitrary HTML attributes to components,
/// similar to the React implementation's nativeAttributes pattern.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NativeAttributes {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_controls: Option<String>,
    pub aria_expanded: Option<bool>,
    pub aria_hidden: Option<bool>,
    pub role: Option<String>,
    pub tabindex: Option<i32>,
    pub title: Option<String>,
}

impl NativeAttributes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates attributes with an ARIA label
    pub fn with_aria_label(label: impl Into<String>) -> Self {
        Self {
            aria_label: Some(label.into()),
            ..Default::default()
        }
    }
}

/// I18n strings pattern
///
/// Provides internationalization support for component strings,
/// matching the React implementation's i18nStrings prop pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct I18nStrings {
    strings: HashMap<String, String>,
}

impl I18nStrings {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.strings.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.strings.get(key)
    }

    pub fn get_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.strings.get(key).map(|s| s.as_str()).unwrap_or(default)
    }
}

impl Default for I18nStrings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_override() {
        let mut style = StyleOverride::new();
        style.set("--custom-color", "#ff0000");
        style.set("--custom-padding", "10px");

        assert_eq!(style.get("--custom-color"), Some(&"#ff0000".to_string()));
        assert!(!style.is_empty());

        let inline = style.to_inline_style();
        assert!(inline.contains("--custom-color:#ff0000"));
        assert!(inline.contains("--custom-padding:10px"));
    }

    #[test]
    fn test_style_override_builder() {
        let style = StyleOverrideBuilder::new()
            .property("--color", "blue")
            .property("--size", "large")
            .build();

        assert_eq!(style.get("--color"), Some(&"blue".to_string()));
        assert_eq!(style.get("--size"), Some(&"large".to_string()));
    }

    #[test]
    fn test_native_attributes() {
        let attrs = NativeAttributes::with_aria_label("Test Label");
        assert_eq!(attrs.aria_label, Some("Test Label".to_string()));
    }

    #[test]
    fn test_i18n_strings() {
        let mut i18n = I18nStrings::new();
        i18n.set("button.submit", "Submit");
        i18n.set("button.cancel", "Cancel");

        assert_eq!(i18n.get("button.submit"), Some(&"Submit".to_string()));
        assert_eq!(i18n.get_or("button.missing", "Default"), "Default");
    }
}
