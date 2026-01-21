// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! ProgressBar component
//!
//! Progress indicator with percentage display and status variants.

use crate::internal::{AnalyticsMetadata, BaseComponentProps, ClassBuilder, ComponentMetadata};
use yew::prelude::*;

/// Status variants for the progress bar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarStatus {
    /// Progress is actively in progress
    #[default]
    InProgress,
    /// Progress completed successfully
    Success,
    /// Progress completed with an error
    Error,
}

impl ProgressBarStatus {
    /// Returns the CSS class name for this status
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InProgress => "in-progress",
            Self::Success => "success",
            Self::Error => "error",
        }
    }
}

/// Variant for the progress bar size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarVariant {
    /// Default full-sized variant
    #[default]
    Default,
    /// Compact flash variant
    Flash,
}

impl ProgressBarVariant {
    /// Returns the CSS class name for this variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Flash => "flash",
        }
    }
}

/// Properties for the ProgressBar component
#[derive(Properties, PartialEq, Clone)]
pub struct ProgressBarProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Progress value (0.0 to 100.0)
    #[prop_or(0.0)]
    pub value: f32,

    /// Status variant
    #[prop_or_default]
    pub status: ProgressBarStatus,

    /// Size variant
    #[prop_or_default]
    pub variant: ProgressBarVariant,

    /// Label text for the progress bar
    #[prop_or_default]
    pub label: Option<String>,

    /// Description content
    #[prop_or_default]
    pub description: Option<Html>,

    /// Additional info content (e.g., "3 of 4 files")
    #[prop_or_default]
    pub additional_info: Option<Html>,

    /// Result text shown when complete
    #[prop_or_default]
    pub result_text: Option<String>,

    /// Action button shown when complete
    #[prop_or_default]
    pub result_button: Option<Html>,
}

/// ProgressBar component
///
/// Displays a progress indicator with a percentage value and optional status indicators.
/// Supports various status states and can show additional information and result actions.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{ProgressBar, ProgressBarStatus};
///
/// html! {
///     <ProgressBar
///         value={75.0}
///         status={ProgressBarStatus::InProgress}
///         label="Uploading files"
///     />
/// }
/// ```
///
/// # With additional info
///
/// ```rust
/// use cloudscape_components::{ProgressBar, ProgressBarStatus};
///
/// html! {
///     <ProgressBar
///         value={50.0}
///         status={ProgressBarStatus::InProgress}
///         label="Processing"
///         additional_info={html! { <span>{ "2 of 4 items" }</span> }}
///         description={html! { <span>{ "This may take a few moments" }</span> }}
///     />
/// }
/// ```
///
/// # With result text and button
///
/// ```rust
/// use cloudscape_components::{ProgressBar, ProgressBarStatus, Button};
///
/// html! {
///     <ProgressBar
///         value={100.0}
///         status={ProgressBarStatus::Success}
///         label="Upload complete"
///         result_text="All files uploaded successfully"
///         result_button={html! { <Button>{ "View files" }</Button> }}
///     />
/// }
/// ```
///
/// # Compact flash variant
///
/// ```rust
/// use cloudscape_components::{ProgressBar, ProgressBarVariant, ProgressBarStatus};
///
/// html! {
///     <ProgressBar
///         value={80.0}
///         variant={ProgressBarVariant::Flash}
///         status={ProgressBarStatus::InProgress}
///     />
/// }
/// ```
#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let _metadata = ComponentMetadata::new("ProgressBar");

    // Clamp value between 0 and 100
    let clamped_value = props.value.clamp(0.0, 100.0);
    let percentage_text = format!("{:.0}%", clamped_value);

    // Build CSS classes
    let classes = ClassBuilder::new()
        .add("awsui-progress-bar")
        .add(format!(
            "awsui-progress-bar-status-{}",
            props.status.as_str()
        ))
        .add(format!(
            "awsui-progress-bar-variant-{}",
            props.variant.as_str()
        ))
        .build();

    let class = props.base.merge_classes(&classes);

    // Build analytics metadata
    let analytics = AnalyticsMetadata {
        action: Some("progress-bar".to_string()),
        detail: Some(serde_json::json!({
            "status": props.status.as_str(),
            "value": clamped_value
        })),
        component: None,
    };
    let analytics_attr = serde_json::to_string(&analytics).ok();

    // Render label section if label exists
    let label_section = if let Some(ref label) = props.label {
        html! {
            <div class="awsui-progress-bar-header">
                <div class="awsui-progress-bar-label">
                    { label }
                </div>
                {
                    if props.additional_info.is_some() {
                        html! {
                            <div class="awsui-progress-bar-additional-info">
                                { props.additional_info.clone() }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    } else {
        html! {}
    };

    // Render description if provided
    let description_section = if let Some(ref description) = props.description {
        html! {
            <div class="awsui-progress-bar-description">
                { description.clone() }
            </div>
        }
    } else {
        html! {}
    };

    // Render progress track and fill
    let progress_section = html! {
        <div class="awsui-progress-bar-content">
            <div class="awsui-progress-bar-track">
                <div
                    class="awsui-progress-bar-fill"
                    style={format!("width: {}%", clamped_value)}
                    role="progressbar"
                    aria-valuenow={clamped_value.to_string()}
                    aria-valuemin="0"
                    aria-valuemax="100"
                />
            </div>
            <div class="awsui-progress-bar-percentage">
                { percentage_text }
            </div>
        </div>
    };

    // Render result section if result text or button is provided
    let result_section = if props.result_text.is_some() || props.result_button.is_some() {
        html! {
            <div class="awsui-progress-bar-result">
                {
                    if let Some(ref result_text) = props.result_text {
                        html! {
                            <div class="awsui-progress-bar-result-text">
                                { result_text }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(ref result_button) = props.result_button {
                        html! {
                            <div class="awsui-progress-bar-result-button">
                                { result_button.clone() }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div
            id={props.base.id.clone()}
            class={class}
            data-analytics-metadata={analytics_attr}
        >
            { label_section }
            { description_section }
            { progress_section }
            { result_section }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_strings() {
        assert_eq!(ProgressBarStatus::InProgress.as_str(), "in-progress");
        assert_eq!(ProgressBarStatus::Success.as_str(), "success");
        assert_eq!(ProgressBarStatus::Error.as_str(), "error");
    }

    #[test]
    fn variant_strings() {
        assert_eq!(ProgressBarVariant::Default.as_str(), "default");
        assert_eq!(ProgressBarVariant::Flash.as_str(), "flash");
    }

    #[test]
    fn status_default() {
        assert_eq!(ProgressBarStatus::default(), ProgressBarStatus::InProgress);
    }

    #[test]
    fn variant_default() {
        assert_eq!(ProgressBarVariant::default(), ProgressBarVariant::Default);
    }

    #[test]
    fn status_equality() {
        assert_eq!(ProgressBarStatus::InProgress, ProgressBarStatus::InProgress);
        assert_ne!(ProgressBarStatus::InProgress, ProgressBarStatus::Success);
        assert_ne!(ProgressBarStatus::Success, ProgressBarStatus::Error);
    }

    #[test]
    fn variant_equality() {
        assert_eq!(ProgressBarVariant::Default, ProgressBarVariant::Default);
        assert_ne!(ProgressBarVariant::Default, ProgressBarVariant::Flash);
    }

    #[test]
    fn all_status_types_have_unique_strings() {
        let statuses = vec![
            ProgressBarStatus::InProgress,
            ProgressBarStatus::Success,
            ProgressBarStatus::Error,
        ];

        let strings: Vec<_> = statuses.iter().map(|s| s.as_str()).collect();
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
    fn all_variants_have_unique_strings() {
        let variants = vec![ProgressBarVariant::Default, ProgressBarVariant::Flash];

        let strings: Vec<_> = variants.iter().map(|v| v.as_str()).collect();
        let mut sorted = strings.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(
            strings.len(),
            sorted.len(),
            "All variants should have unique string representations"
        );
    }

    #[test]
    fn value_clamping() {
        // Test that values are clamped within render logic
        // We can't directly test the rendering, but we can verify the logic
        let test_cases = vec![
            (-10.0_f32, 0.0_f32),
            (0.0_f32, 0.0_f32),
            (50.0_f32, 50.0_f32),
            (100.0_f32, 100.0_f32),
            (150.0_f32, 100.0_f32),
        ];

        for (input, expected) in test_cases {
            let clamped = input.clamp(0.0, 100.0);
            assert_eq!(
                clamped, expected,
                "Value {} should clamp to {}",
                input, expected
            );
        }
    }

    #[test]
    fn status_debug_format() {
        let status = ProgressBarStatus::Success;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Success"));
    }

    #[test]
    fn variant_debug_format() {
        let variant = ProgressBarVariant::Flash;
        let debug_str = format!("{:?}", variant);
        assert!(debug_str.contains("Flash"));
    }
}
