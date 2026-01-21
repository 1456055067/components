// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Alert component for displaying contextual feedback messages.
//!
//! Alerts communicate important information to users with different severity levels.

use crate::internal::{
    AnalyticsMetadata, AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata,
    CustomEvent,
};
use yew::prelude::*;

/// Alert type variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertType {
    /// Informational message (default)
    #[default]
    Info,
    /// Success confirmation message
    Success,
    /// Warning or caution message
    Warning,
    /// Error or critical message
    Error,
}

impl AlertType {
    /// Returns the CSS class name for this alert type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Returns the icon name for this alert type
    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Info => "status-info",
            Self::Success => "status-positive",
            Self::Warning => "status-warning",
            Self::Error => "status-negative",
        }
    }

    /// Returns the default ARIA label for this alert type
    pub fn default_aria_label(&self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }
}

/// Internationalization strings for Alert
#[derive(Clone, PartialEq, Default)]
pub struct AlertI18nStrings {
    /// ARIA label for the dismiss button
    pub dismiss_aria_label: Option<String>,
}

/// Event detail for dismiss events
#[derive(Clone, PartialEq)]
pub struct DismissDetail {
    /// The alert type that was dismissed
    pub alert_type: AlertType,
}

/// Properties for the Alert component
#[derive(Properties, PartialEq, Clone)]
pub struct AlertProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Alert type/variant
    #[prop_or_default]
    pub alert_type: AlertType,

    /// Optional header text
    #[prop_or_default]
    pub header: Option<String>,

    /// Whether the alert can be dismissed
    #[prop_or_default]
    pub dismissible: bool,

    /// Callback fired when the dismiss button is clicked
    #[prop_or_default]
    pub on_dismiss: Option<Callback<CustomEvent<DismissDetail>>>,

    /// Optional action button content
    #[prop_or_default]
    pub action: Option<Html>,

    /// Internationalization strings
    #[prop_or_default]
    pub i18n_strings: AlertI18nStrings,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// Alert content (children)
    #[prop_or_default]
    pub children: Children,
}

/// Alert component for displaying contextual feedback messages.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Alert, AlertType};
///
/// html! {
///     <Alert alert_type={AlertType::Success} header="Success!">
///         { "Your changes have been saved." }
///     </Alert>
/// }
/// ```
#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let _metadata = ComponentMetadata::new("Alert");

    // Build CSS classes
    let classes = ClassBuilder::new()
        .add("awsui-alert")
        .add(format!("awsui-alert-type-{}", props.alert_type.as_str()))
        .add_if(props.dismissible, "awsui-alert-dismissible");

    // Handle dismiss button click
    let on_dismiss_click = {
        let alert_type = props.alert_type;
        let on_dismiss = props.on_dismiss.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if let Some(callback) = &on_dismiss {
                callback.emit(CustomEvent::new_non_cancelable(DismissDetail {
                    alert_type,
                }));
            }
        })
    };

    // Build analytics metadata
    let analytics = AnalyticsMetadata {
        action: Some("alert".to_string()),
        detail: None,
        component: None,
    };
    let analytics_attr = serde_json::to_string(&analytics).ok();

    // ARIA role and label
    let aria_label = props
        .aria
        .label
        .clone()
        .unwrap_or_else(|| props.alert_type.default_aria_label().to_string());

    html! {
        <div
            class={classes.build()}
            role="alert"
            aria-label={aria_label}
            data-analytics-metadata={analytics_attr}
        >
            // Icon
            <div class="awsui-alert-icon">
                <span
                    class={format!("awsui-icon awsui-icon-{}", props.alert_type.icon_name())}
                    aria-hidden="true"
                />
            </div>

            // Content wrapper
            <div class="awsui-alert-content">
                // Optional header
                if let Some(ref header) = props.header {
                    <div class="awsui-alert-header">
                        { header }
                    </div>
                }

                // Message content
                <div class="awsui-alert-message">
                    { props.children.clone() }
                </div>

                // Optional action button
                if let Some(ref action) = props.action {
                    <div class="awsui-alert-action">
                        { action.clone() }
                    </div>
                }
            </div>

            // Optional dismiss button
            if props.dismissible {
                <div class="awsui-alert-dismiss">
                    <button
                        type="button"
                        class="awsui-button awsui-button-variant-icon"
                        aria-label={
                            props.i18n_strings.dismiss_aria_label.clone()
                                .unwrap_or_else(|| "Dismiss".to_string())
                        }
                        onclick={on_dismiss_click}
                    >
                        <span class="awsui-icon awsui-icon-close" aria-hidden="true" />
                    </button>
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_type_strings() {
        assert_eq!(AlertType::Info.as_str(), "info");
        assert_eq!(AlertType::Success.as_str(), "success");
        assert_eq!(AlertType::Warning.as_str(), "warning");
        assert_eq!(AlertType::Error.as_str(), "error");
    }

    #[test]
    fn alert_type_icons() {
        assert_eq!(AlertType::Info.icon_name(), "status-info");
        assert_eq!(AlertType::Success.icon_name(), "status-positive");
        assert_eq!(AlertType::Warning.icon_name(), "status-warning");
        assert_eq!(AlertType::Error.icon_name(), "status-negative");
    }

    #[test]
    fn alert_type_aria_labels() {
        assert_eq!(AlertType::Info.default_aria_label(), "Info");
        assert_eq!(AlertType::Success.default_aria_label(), "Success");
        assert_eq!(AlertType::Warning.default_aria_label(), "Warning");
        assert_eq!(AlertType::Error.default_aria_label(), "Error");
    }

    #[test]
    fn alert_type_default() {
        assert_eq!(AlertType::default(), AlertType::Info);
    }
}
