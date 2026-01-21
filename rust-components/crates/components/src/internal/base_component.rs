// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Base component properties and utilities
//!
//! Provides common functionality shared across all Cloudscape components,
//! similar to the React implementation's useBaseComponent hook.

use yew::prelude::*;

/// Base properties available to all Cloudscape components
#[derive(Properties, PartialEq, Clone, Default)]
pub struct BaseComponentProps {
    /// HTML id attribute
    #[prop_or_default]
    pub id: Option<String>,

    /// CSS class name(s) to apply
    #[prop_or_default]
    pub class: Option<String>,

    /// Data attributes for testing and analytics
    #[prop_or_default]
    pub data_attributes: Option<Vec<(String, String)>>,
}

impl BaseComponentProps {
    /// Merges base props with component-specific classes
    pub fn merge_classes(&self, component_classes: &str) -> String {
        match &self.class {
            Some(user_class) => format!("{} {}", component_classes, user_class),
            None => component_classes.to_string(),
        }
    }

    /// Gets all data attributes as HTML attribute tuples
    pub fn data_attrs(&self) -> Vec<(&str, &str)> {
        self.data_attributes
            .as_ref()
            .map(|attrs| {
                attrs
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Component metadata for analytics and debugging
#[derive(Debug, Clone)]
pub struct ComponentMetadata {
    pub name: &'static str,
    pub version: &'static str,
}

impl ComponentMetadata {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            version: crate::VERSION,
        }
    }

    /// Gets analytics metadata attribute value
    pub fn analytics_metadata(&self) -> String {
        format!(r#"{{"component":{{"name":"awsui.{}","label":"{}"}}}}"#, self.name, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_classes() {
        let base = BaseComponentProps {
            class: Some("user-class".to_string()),
            ..Default::default()
        };

        assert_eq!(
            base.merge_classes("component-class"),
            "component-class user-class"
        );
    }

    #[test]
    fn test_merge_classes_no_user_class() {
        let base = BaseComponentProps::default();
        assert_eq!(base.merge_classes("component-class"), "component-class");
    }

    #[test]
    fn test_component_metadata() {
        let metadata = ComponentMetadata::new("Button");
        assert_eq!(metadata.name, "Button");
        assert!(!metadata.version.is_empty());
    }
}
