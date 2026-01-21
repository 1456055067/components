// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! StatusIndicator component
//!
//! Status badges with color variants and optional icons to communicate status information.

use crate::internal::{
    AnalyticsMetadata, AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata,
};
use crate::spinner::{Spinner, SpinnerSize};
use yew::prelude::*;

/// Status indicator type variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusIndicatorType {
    /// Success status
    Success,
    /// Error status
    Error,
    /// Warning status
    Warning,
    /// Informational status (default)
    Info,
    /// Stopped status
    Stopped,
    /// Pending status
    Pending,
    /// In-progress status
    InProgress,
    /// Loading status with spinner
    Loading,
}

impl Default for StatusIndicatorType {
    fn default() -> Self {
        Self::Info
    }
}

impl StatusIndicatorType {
    /// Returns the CSS class name for this status type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Stopped => "stopped",
            Self::Pending => "pending",
            Self::InProgress => "in-progress",
            Self::Loading => "loading",
        }
    }

    /// Returns the default icon name for this status type
    pub fn default_icon(&self) -> &'static str {
        match self {
            Self::Success => "status-positive",
            Self::Error => "status-negative",
            Self::Warning => "status-warning",
            Self::Info => "status-info",
            Self::Stopped => "status-stopped",
            Self::Pending => "status-pending",
            Self::InProgress => "status-in-progress",
            Self::Loading => "spinner",
        }
    }
}

/// Color override options for status indicators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusIndicatorColor {
    Blue,
    Grey,
    Green,
    Red,
}

impl StatusIndicatorColor {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Grey => "grey",
            Self::Green => "green",
            Self::Red => "red",
        }
    }
}

/// Properties for the StatusIndicator component
#[derive(Properties, PartialEq, Clone)]
pub struct StatusIndicatorProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Status type/variant
    #[prop_or_default]
    pub status_type: StatusIndicatorType,

    /// Optional color override
    #[prop_or_default]
    pub color_override: Option<StatusIndicatorColor>,

    /// ARIA label for the icon
    #[prop_or_default]
    pub icon_aria_label: Option<String>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// Status text content (children)
    #[prop_or_default]
    pub children: Children,
}

/// StatusIndicator component
///
/// Displays a status badge with an icon and text to communicate status information.
/// Each status type has a default icon and color, which can be overridden.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{StatusIndicator, StatusIndicatorType};
///
/// html! {
///     <StatusIndicator status_type={StatusIndicatorType::Success}>
///         { "Operational" }
///     </StatusIndicator>
/// }
/// ```
///
/// # With color override
///
/// ```rust
/// use cloudscape_components::{StatusIndicator, StatusIndicatorType, StatusIndicatorColor};
///
/// html! {
///     <StatusIndicator
///         status_type={StatusIndicatorType::Info}
///         color_override={Some(StatusIndicatorColor::Blue)}
///     >
///         { "Custom status" }
///     </StatusIndicator>
/// }
/// ```
///
/// # With custom icon label
///
/// ```rust
/// use cloudscape_components::{StatusIndicator, StatusIndicatorType};
///
/// html! {
///     <StatusIndicator
///         status_type={StatusIndicatorType::Error}
///         icon_aria_label={Some("Critical error".to_string())}
///     >
///         { "Service unavailable" }
///     </StatusIndicator>
/// }
/// ```
#[function_component(StatusIndicator)]
pub fn status_indicator(props: &StatusIndicatorProps) -> Html {
    let _metadata = ComponentMetadata::new("StatusIndicator");

    // Build CSS classes
    let mut classes = ClassBuilder::new()
        .add("awsui-status-indicator")
        .add(format!(
            "awsui-status-indicator-type-{}",
            props.status_type.as_str()
        ));

    // Add color override class if specified
    if let Some(color) = props.color_override {
        classes = classes.add(format!("awsui-status-indicator-color-{}", color.as_str()));
    }

    let class = props.base.merge_classes(&classes.build());

    // Build analytics metadata
    let analytics = AnalyticsMetadata {
        action: Some("status-indicator".to_string()),
        detail: Some(serde_json::json!({
            "type": props.status_type.as_str()
        })),
        component: None,
    };
    let analytics_attr = serde_json::to_string(&analytics).ok();

    // Render the appropriate icon based on status type
    let icon_content = if props.status_type == StatusIndicatorType::Loading {
        html! {
            <Spinner size={SpinnerSize::Normal} />
        }
    } else {
        html! {
            <span
                class={format!("awsui-icon awsui-icon-{}", props.status_type.default_icon())}
                aria-hidden="true"
            />
        }
    };

    html! {
        <span
            id={props.base.id.clone()}
            class={class}
            data-analytics-metadata={analytics_attr}
        >
            <span class="awsui-status-indicator-container">
                <span
                    class="awsui-status-indicator-icon"
                    aria-label={props.icon_aria_label.clone()}
                    role={props.icon_aria_label.as_ref().map(|_| "img")}
                >
                    { icon_content }
                </span>
                { props.children.clone() }
            </span>
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_type_strings() {
        assert_eq!(StatusIndicatorType::Success.as_str(), "success");
        assert_eq!(StatusIndicatorType::Error.as_str(), "error");
        assert_eq!(StatusIndicatorType::Warning.as_str(), "warning");
        assert_eq!(StatusIndicatorType::Info.as_str(), "info");
        assert_eq!(StatusIndicatorType::Stopped.as_str(), "stopped");
        assert_eq!(StatusIndicatorType::Pending.as_str(), "pending");
        assert_eq!(StatusIndicatorType::InProgress.as_str(), "in-progress");
        assert_eq!(StatusIndicatorType::Loading.as_str(), "loading");
    }

    #[test]
    fn status_type_icons() {
        assert_eq!(
            StatusIndicatorType::Success.default_icon(),
            "status-positive"
        );
        assert_eq!(StatusIndicatorType::Error.default_icon(), "status-negative");
        assert_eq!(
            StatusIndicatorType::Warning.default_icon(),
            "status-warning"
        );
        assert_eq!(StatusIndicatorType::Info.default_icon(), "status-info");
        assert_eq!(
            StatusIndicatorType::Stopped.default_icon(),
            "status-stopped"
        );
        assert_eq!(
            StatusIndicatorType::Pending.default_icon(),
            "status-pending"
        );
        assert_eq!(
            StatusIndicatorType::InProgress.default_icon(),
            "status-in-progress"
        );
        assert_eq!(StatusIndicatorType::Loading.default_icon(), "spinner");
    }

    #[test]
    fn status_type_default() {
        assert_eq!(StatusIndicatorType::default(), StatusIndicatorType::Info);
    }

    #[test]
    fn color_override_strings() {
        assert_eq!(StatusIndicatorColor::Blue.as_str(), "blue");
        assert_eq!(StatusIndicatorColor::Grey.as_str(), "grey");
        assert_eq!(StatusIndicatorColor::Green.as_str(), "green");
        assert_eq!(StatusIndicatorColor::Red.as_str(), "red");
    }

    #[test]
    fn status_type_equality() {
        assert_eq!(StatusIndicatorType::Success, StatusIndicatorType::Success);
        assert_ne!(StatusIndicatorType::Success, StatusIndicatorType::Error);
    }

    #[test]
    fn color_override_equality() {
        assert_eq!(StatusIndicatorColor::Blue, StatusIndicatorColor::Blue);
        assert_ne!(StatusIndicatorColor::Blue, StatusIndicatorColor::Red);
    }

    #[test]
    fn all_status_types_have_unique_strings() {
        let types = vec![
            StatusIndicatorType::Success,
            StatusIndicatorType::Error,
            StatusIndicatorType::Warning,
            StatusIndicatorType::Info,
            StatusIndicatorType::Stopped,
            StatusIndicatorType::Pending,
            StatusIndicatorType::InProgress,
            StatusIndicatorType::Loading,
        ];

        let strings: Vec<_> = types.iter().map(|t| t.as_str()).collect();
        let mut sorted = strings.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(
            strings.len(),
            sorted.len(),
            "All status types should have unique string representations"
        );
    }

    #[test]
    fn all_status_types_have_icons() {
        let types = vec![
            StatusIndicatorType::Success,
            StatusIndicatorType::Error,
            StatusIndicatorType::Warning,
            StatusIndicatorType::Info,
            StatusIndicatorType::Stopped,
            StatusIndicatorType::Pending,
            StatusIndicatorType::InProgress,
            StatusIndicatorType::Loading,
        ];

        for status_type in types {
            let icon = status_type.default_icon();
            assert!(
                !icon.is_empty(),
                "{:?} should have a non-empty icon",
                status_type
            );
        }
    }
}
