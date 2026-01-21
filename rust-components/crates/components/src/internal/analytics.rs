// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Analytics metadata system
//!
//! Provides analytics tracking functionality matching the React implementation's
//! analytics metadata and funnel tracking systems.

use serde::{Deserialize, Serialize};
use serde_json::json;

/// Label identifier for analytics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Default)]
pub enum LabelIdentifier {
    /// String label
    String(String),
    /// Root component identifier
    #[default]
    Root,
}

impl From<String> for LabelIdentifier {
    fn from(s: String) -> Self {
        LabelIdentifier::String(s)
    }
}

impl From<&str> for LabelIdentifier {
    fn from(s: &str) -> Self {
        LabelIdentifier::String(s.to_string())
    }
}

/// Component analytics metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentAnalytics {
    pub name: String,
    pub label: LabelIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Analytics metadata structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalyticsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ComponentAnalytics>,
}

impl AnalyticsMetadata {
    /// Creates metadata for a button component
    pub fn button(label: impl Into<LabelIdentifier>, variant: &str, disabled: bool) -> Self {
        let label_id = label.into();
        Self {
            action: Some("click".to_string()),
            detail: Some(json!({
                "label": label_id.clone()
            })),
            component: Some(ComponentAnalytics {
                name: "awsui.Button".to_string(),
                label: label_id,
                properties: Some(json!({
                    "variant": variant,
                    "disabled": disabled.to_string()
                })),
            }),
        }
    }

    /// Creates metadata for a badge component
    pub fn badge(label: impl Into<LabelIdentifier>, color: &str) -> Self {
        Self {
            component: Some(ComponentAnalytics {
                name: "awsui.Badge".to_string(),
                label: label.into(),
                properties: Some(json!({
                    "color": color
                })),
            }),
            ..Default::default()
        }
    }

    /// Converts to JSON string for data attribute
    pub fn to_data_attribute(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Funnel tracking context
///
/// Tracks user flows through multi-step processes (wizards, forms, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct FunnelContext {
    pub funnel_id: String,
    pub step_number: Option<usize>,
    pub total_steps: Option<usize>,
    pub step_name: Option<String>,
}

impl FunnelContext {
    pub fn new(funnel_id: impl Into<String>) -> Self {
        Self {
            funnel_id: funnel_id.into(),
            step_number: None,
            total_steps: None,
            step_name: None,
        }
    }

    pub fn with_step(mut self, step_number: usize, total_steps: usize) -> Self {
        self.step_number = Some(step_number);
        self.total_steps = Some(total_steps);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.step_name = Some(name.into());
        self
    }
}

/// Performance marks for component lifecycle tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceMark {
    ComponentMount,
    ComponentUpdate,
    ComponentUnmount,
    InteractionStart,
    InteractionEnd,
}

impl PerformanceMark {
    pub fn name(&self, component_name: &str) -> String {
        match self {
            PerformanceMark::ComponentMount => format!("{}:mount", component_name),
            PerformanceMark::ComponentUpdate => format!("{}:update", component_name),
            PerformanceMark::ComponentUnmount => format!("{}:unmount", component_name),
            PerformanceMark::InteractionStart => format!("{}:interaction:start", component_name),
            PerformanceMark::InteractionEnd => format!("{}:interaction:end", component_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_identifier() {
        let string_label = LabelIdentifier::from("test-label");
        assert!(matches!(string_label, LabelIdentifier::String(_)));

        let root_label = LabelIdentifier::Root;
        assert_eq!(root_label, LabelIdentifier::Root);
    }

    #[test]
    fn test_button_analytics() {
        let metadata = AnalyticsMetadata::button("submit-button", "primary", false);

        assert_eq!(metadata.action, Some("click".to_string()));
        assert!(metadata.component.is_some());

        let component = metadata.component.unwrap();
        assert_eq!(component.name, "awsui.Button");
    }

    #[test]
    fn test_badge_analytics() {
        let metadata = AnalyticsMetadata::badge("status-badge", "blue");

        assert!(metadata.component.is_some());
        let component = metadata.component.unwrap();
        assert_eq!(component.name, "awsui.Badge");
    }

    #[test]
    fn test_analytics_serialization() {
        let metadata = AnalyticsMetadata::button("test", "normal", false);
        let json_str = metadata.to_data_attribute();

        assert!(json_str.contains("awsui.Button"));
        assert!(!json_str.is_empty());
    }

    #[test]
    fn test_funnel_context() {
        let funnel = FunnelContext::new("checkout-flow")
            .with_step(2, 4)
            .with_name("payment");

        assert_eq!(funnel.funnel_id, "checkout-flow");
        assert_eq!(funnel.step_number, Some(2));
        assert_eq!(funnel.total_steps, Some(4));
        assert_eq!(funnel.step_name, Some("payment".to_string()));
    }

    #[test]
    fn test_performance_marks() {
        let mount_mark = PerformanceMark::ComponentMount.name("Button");
        assert_eq!(mount_mark, "Button:mount");

        let interaction_mark = PerformanceMark::InteractionStart.name("Table");
        assert_eq!(interaction_mark, "Table:interaction:start");
    }
}
