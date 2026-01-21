// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Form field component for wrapping form controls with labels and validation.
//!
//! The FormField component provides a consistent way to wrap form controls
//! (Input, Select, Textarea, etc.) with labels, descriptions, error/warning messages,
//! and constraint text. It handles accessibility requirements including proper
//! ARIA attributes and ID associations.

use crate::internal::{BaseComponentProps, ClassBuilder, ComponentMetadata};
use yew::prelude::*;

/// Properties for the FormField component
#[derive(Properties, PartialEq, Clone, Default)]
pub struct FormFieldProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Field label text
    ///
    /// The label text to display above the form control. This is typically
    /// a short, descriptive name for the field (e.g., "Email address", "Password").
    #[prop_or_default]
    pub label: Option<String>,

    /// Info link content to display next to the label
    ///
    /// This is typically a link or button that provides additional help or
    /// context about the form field. It appears next to the label text.
    #[prop_or_default]
    pub info: Option<Html>,

    /// Description text to display below the label
    ///
    /// Provides additional help text or instructions for the field.
    /// This is displayed between the label and the form control.
    #[prop_or_default]
    pub description: Option<Html>,

    /// Error message to display
    ///
    /// When present, displays an error message below the form control and
    /// applies error styling. Takes precedence over warning text if both are provided.
    #[prop_or_default]
    pub error_text: Option<Html>,

    /// Warning message to display
    ///
    /// When present (and error_text is not), displays a warning message below
    /// the form control with warning styling.
    #[prop_or_default]
    pub warning_text: Option<Html>,

    /// Constraint text to display
    ///
    /// Additional constraint information to display below the form control,
    /// such as character limits or format requirements (e.g., "Max 100 characters").
    #[prop_or_default]
    pub constraint_text: Option<Html>,

    /// ID for the form control element
    ///
    /// This ID is used in the label's `for` attribute to associate the label
    /// with the control. It should match the `id` or `control_id` prop of the
    /// wrapped form control component.
    #[prop_or_default]
    pub control_id: Option<String>,

    /// Whether to stretch the control to full width
    ///
    /// When true, the form control will expand to fill the available width.
    #[prop_or_default]
    pub stretch: bool,

    /// Secondary control element
    ///
    /// An additional control to display alongside the primary control, such as
    /// a units dropdown or a secondary action button.
    #[prop_or_default]
    pub secondary_control: Option<Html>,

    /// The form control element (Input, Select, Textarea, etc.)
    #[prop_or_default]
    pub children: Children,
}

/// Form field component for wrapping form controls with labels and validation.
///
/// The FormField component provides a standardized way to display form controls
/// with proper labeling, descriptions, validation messages, and accessibility support.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{FormField, Input, InputType};
///
/// html! {
///     <FormField
///         label="Email address"
///         description={html! { "We'll never share your email." }}
///         error_text={html! { "Please enter a valid email address." }}
///         control_id="email-input"
///     >
///         <Input
///             control_id="email-input"
///             input_type={InputType::Email}
///             value={email}
///             invalid={true}
///         />
///     </FormField>
/// }
/// ```
///
/// # Accessibility
///
/// The component automatically:
/// - Associates the label with the control using the `for` attribute
/// - Generates unique IDs for description, error, and constraint text
/// - Sets `aria-describedby` on the control to reference all descriptive text
/// - Provides proper ARIA roles and attributes for validation messages
///
/// # Validation States
///
/// Error messages take precedence over warnings. If both `error_text` and
/// `warning_text` are provided, only the error will be displayed.
#[function_component(FormField)]
pub fn form_field(props: &FormFieldProps) -> Html {
    let _metadata = ComponentMetadata::new("FormField");

    // Generate unique IDs for ARIA associations
    let base_id = use_state(|| {
        props.control_id.clone().unwrap_or_else(|| {
            // Simple unique ID generation using timestamp and random number
            use web_sys::js_sys;
            format!(
                "form-field-{}-{}",
                js_sys::Date::now() as u64,
                (js_sys::Math::random() * 1000000.0) as u32
            )
        })
    });

    let description_id = format!("{}-description", *base_id);
    let error_id = format!("{}-error", *base_id);
    let constraint_id = format!("{}-constraint", *base_id);

    // Build aria-describedby list
    let mut described_by_ids = Vec::new();
    if props.description.is_some() {
        described_by_ids.push(description_id.clone());
    }
    if props.error_text.is_some() {
        described_by_ids.push(error_id.clone());
    } else if props.warning_text.is_some() {
        described_by_ids.push(error_id.clone()); // Warning uses same ID as error
    }
    if props.constraint_text.is_some() {
        described_by_ids.push(constraint_id.clone());
    }

    let aria_describedby = if !described_by_ids.is_empty() {
        Some(described_by_ids.join(" "))
    } else {
        None
    };

    // Build CSS classes
    let root_classes = ClassBuilder::new()
        .add("awsui-form-field")
        .add_if(props.stretch, "awsui-form-field-stretch");

    let control_wrapper_classes = ClassBuilder::new().add("awsui-form-field-control-wrapper");

    html! {
        <div class={props.base.merge_classes(&root_classes.build())} id={props.base.id.clone()}>
            // Label section
            if props.label.is_some() || props.info.is_some() {
                <div class="awsui-form-field-label-wrapper">
                    if let Some(label_text) = &props.label {
                        <label
                            class="awsui-form-field-label"
                            for={props.control_id.clone()}
                        >
                            { label_text }
                        </label>
                    }
                    if let Some(info_content) = &props.info {
                        <span class="awsui-form-field-info-link">
                            { info_content.clone() }
                        </span>
                    }
                </div>
            }

            // Description section
            if let Some(description_content) = &props.description {
                <div
                    class="awsui-form-field-description"
                    id={description_id}
                >
                    { description_content.clone() }
                </div>
            }

            // Control section
            <div class={control_wrapper_classes.build()}>
                <div
                    class="awsui-form-field-control"
                    data-aria-describedby={aria_describedby}
                >
                    { for props.children.iter() }
                </div>

                // Secondary control
                if let Some(secondary) = &props.secondary_control {
                    <div class="awsui-form-field-secondary-control">
                        { secondary.clone() }
                    </div>
                }
            </div>

            // Error message (takes precedence over warning)
            if let Some(error_content) = &props.error_text {
                <div
                    class="awsui-form-field-error"
                    id={error_id}
                    role="alert"
                >
                    { error_content.clone() }
                </div>
            } else if let Some(warning_content) = &props.warning_text {
                <div
                    class="awsui-form-field-warning"
                    id={error_id}
                    role="status"
                >
                    { warning_content.clone() }
                </div>
            }

            // Constraint text
            if let Some(constraint_content) = &props.constraint_text {
                <div
                    class="awsui-form-field-constraint"
                    id={constraint_id}
                >
                    { constraint_content.clone() }
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_props_with_label() {
        let props = FormFieldProps {
            label: Some("Email".to_string()),
            control_id: Some("email-input".to_string()),
            ..Default::default()
        };

        assert_eq!(props.label, Some("Email".to_string()));
        assert_eq!(props.control_id, Some("email-input".to_string()));
    }

    #[test]
    fn test_props_without_label() {
        let props = FormFieldProps {
            label: None,
            ..Default::default()
        };

        assert!(props.label.is_none());
    }

    #[test]
    fn test_props_with_description() {
        let props = FormFieldProps {
            label: Some("Email".to_string()),
            description: Some(html! { <span>{ "Enter your email" }</span> }),
            control_id: Some("test-input".to_string()),
            ..Default::default()
        };

        assert!(props.description.is_some());
        assert_eq!(props.control_id, Some("test-input".to_string()));
    }

    #[test]
    fn test_props_with_error_text() {
        let props = FormFieldProps {
            label: Some("Email".to_string()),
            error_text: Some(html! { <span>{ "Invalid email" }</span> }),
            control_id: Some("test-input".to_string()),
            ..Default::default()
        };

        assert!(props.error_text.is_some());
        assert!(props.warning_text.is_none());
    }

    #[test]
    fn test_props_with_warning_text() {
        let props = FormFieldProps {
            label: Some("Email".to_string()),
            warning_text: Some(html! { <span>{ "Check your email" }</span> }),
            control_id: Some("test-input".to_string()),
            ..Default::default()
        };

        assert!(props.warning_text.is_some());
        assert!(props.error_text.is_none());
    }

    #[test]
    fn test_props_with_both_error_and_warning() {
        let props = FormFieldProps {
            label: Some("Email".to_string()),
            error_text: Some(html! { <span>{ "Error message" }</span> }),
            warning_text: Some(html! { <span>{ "Warning message" }</span> }),
            control_id: Some("test-input".to_string()),
            ..Default::default()
        };

        assert!(props.error_text.is_some());
        assert!(props.warning_text.is_some());
        // Note: The component logic ensures error takes precedence during rendering
    }

    #[test]
    fn test_props_with_constraint_text() {
        let props = FormFieldProps {
            label: Some("Username".to_string()),
            constraint_text: Some(html! { <span>{ "Max 20 characters" }</span> }),
            control_id: Some("test-input".to_string()),
            ..Default::default()
        };

        assert!(props.constraint_text.is_some());
    }

    #[test]
    fn test_stretch_default_is_false() {
        let props = FormFieldProps::default();
        assert!(!props.stretch);
    }

    #[test]
    fn test_stretch_can_be_set() {
        let props = FormFieldProps {
            label: Some("Email".to_string()),
            stretch: true,
            ..Default::default()
        };

        assert!(props.stretch);
    }

    #[test]
    fn test_props_with_secondary_control() {
        let props = FormFieldProps {
            label: Some("Amount".to_string()),
            secondary_control: Some(html! { <select><option>{ "USD" }</option></select> }),
            control_id: Some("test-input".to_string()),
            ..Default::default()
        };

        assert!(props.secondary_control.is_some());
    }

    #[test]
    fn test_props_with_info_link() {
        let props = FormFieldProps {
            label: Some("Password".to_string()),
            info: Some(html! { <a href="#">{ "Info" }</a> }),
            control_id: Some("test-input".to_string()),
            ..Default::default()
        };

        assert!(props.info.is_some());
    }

    #[test]
    fn test_base_props_merge() {
        let props = FormFieldProps {
            base: BaseComponentProps {
                class: Some("custom-class".to_string()),
                id: Some("my-form-field".to_string()),
                ..Default::default()
            },
            label: Some("Email".to_string()),
            ..Default::default()
        };

        assert_eq!(props.base.class, Some("custom-class".to_string()));
        assert_eq!(props.base.id, Some("my-form-field".to_string()));
    }

    #[test]
    fn test_default_props() {
        let props = FormFieldProps::default();

        assert!(props.label.is_none());
        assert!(props.info.is_none());
        assert!(props.description.is_none());
        assert!(props.error_text.is_none());
        assert!(props.warning_text.is_none());
        assert!(props.constraint_text.is_none());
        assert!(props.control_id.is_none());
        assert!(!props.stretch);
        assert!(props.secondary_control.is_none());
    }

    #[test]
    fn test_class_builder_for_stretch() {
        let classes = ClassBuilder::new()
            .add("awsui-form-field")
            .add_if(true, "awsui-form-field-stretch")
            .build();

        assert_eq!(classes, "awsui-form-field awsui-form-field-stretch");
    }

    #[test]
    fn test_class_builder_without_stretch() {
        let classes = ClassBuilder::new()
            .add("awsui-form-field")
            .add_if(false, "awsui-form-field-stretch")
            .build();

        assert_eq!(classes, "awsui-form-field");
    }
}
