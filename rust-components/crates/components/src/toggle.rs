// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Toggle (switch) component
//!
//! A binary switch control for toggling between on/off states.
//! Commonly used for boolean settings and preferences.

use yew::prelude::*;
use web_sys::{HtmlInputElement, FocusEvent};
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
};

/// Event detail for toggle change events
#[derive(Clone, PartialEq, Debug)]
pub struct ToggleChangeDetail {
    /// The new checked state
    pub checked: bool,
}

/// Properties for the Toggle component
#[derive(Properties, PartialEq, Clone)]
pub struct ToggleProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Whether the toggle is checked (controlled component)
    #[prop_or_default]
    pub checked: bool,

    /// Whether the toggle is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the toggle is read-only
    #[prop_or_default]
    pub read_only: bool,

    /// HTML name attribute for form integration
    #[prop_or_default]
    pub name: Option<String>,

    /// Control ID for form field integration
    #[prop_or_default]
    pub control_id: Option<String>,

    /// Label text (displayed next to the toggle)
    #[prop_or_default]
    pub children: Children,

    /// Description text (displayed below the label)
    #[prop_or_default]
    pub description: Option<Html>,

    /// ARIA label (for accessibility when no visible label)
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// ARIA controls attribute
    #[prop_or_default]
    pub aria_controls: Option<String>,

    /// ARIA labelledby attribute
    #[prop_or_default]
    pub aria_labelledby: Option<String>,

    /// ARIA describedby attribute
    #[prop_or_default]
    pub aria_describedby: Option<String>,

    /// Callback fired when the toggle state changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<ToggleChangeDetail>>>,

    /// Callback fired when the toggle gains focus
    #[prop_or_default]
    pub on_focus: Option<Callback<()>>,

    /// Callback fired when the toggle loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Auto-focus on mount
    #[prop_or_default]
    pub auto_focus: bool,
}

/// Toggle (switch) component for binary on/off states.
///
/// The Toggle component provides a visual switch control that users can
/// click to toggle between checked and unchecked states. It's commonly
/// used for boolean settings and preferences.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Toggle, ToggleChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let checked = use_state(|| false);
///
///     let on_change = {
///         let checked = checked.clone();
///         Callback::from(move |event: CustomEvent<ToggleChangeDetail>| {
///             checked.set(event.detail.checked);
///         })
///     };
///
///     html! {
///         <Toggle
///             checked={*checked}
///             on_change={on_change}
///         >
///             {"Enable notifications"}
///         </Toggle>
///     }
/// }
/// ```
///
/// # Accessibility
///
/// The Toggle component follows WAI-ARIA best practices:
/// - Uses `role="switch"` for proper semantics
/// - Sets `aria-checked` to reflect the current state
/// - Supports keyboard navigation (Space, Enter)
/// - Provides proper focus indicators
/// - Supports `aria-label` for screen readers when no visible label exists
///
/// # Form Integration
///
/// The Toggle component can be integrated into HTML forms:
/// - Set the `name` prop to include the value in form submissions
/// - Set the `control_id` prop to link with form field labels
/// - Use `disabled` to prevent interaction and form submission
/// - Use `read_only` to prevent changes while maintaining focus
#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let _metadata = ComponentMetadata::new("Toggle");
    let input_ref = use_node_ref();

    // Use provided control_id or generate a simple unique ID
    let control_id = use_state(|| {
        props.control_id.clone().unwrap_or_else(|| {
            // Simple unique ID generation using timestamp and random number
            use web_sys::js_sys;
            format!("toggle-{}-{}",
                js_sys::Date::now() as u64,
                (js_sys::Math::random() * 1000000.0) as u32
            )
        })
    });

    let label_id = format!("{}-label", *control_id);
    let description_id = format!("{}-description", *control_id);

    // Handle toggle change
    let on_click = {
        let on_change = props.on_change.clone();
        let checked = props.checked;
        let disabled = props.disabled;
        let read_only = props.read_only;
        let input_ref = input_ref.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if disabled || read_only {
                return;
            }

            // Focus the input
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }

            // Fire change event
            if let Some(callback) = &on_change {
                callback.emit(CustomEvent::new_non_cancelable(
                    ToggleChangeDetail { checked: !checked }
                ));
            }
        })
    };

    // Handle focus events
    let on_focus_event = {
        let on_focus = props.on_focus.clone();

        Callback::from(move |_e: FocusEvent| {
            if let Some(callback) = &on_focus {
                callback.emit(());
            }
        })
    };

    let on_blur_event = {
        let on_blur = props.on_blur.clone();

        Callback::from(move |_e: FocusEvent| {
            if let Some(callback) = &on_blur {
                callback.emit(());
            }
        })
    };

    // Build ARIA labelledby
    let has_label = !props.children.is_empty();
    let aria_labelledby = {
        let mut ids = Vec::new();

        if has_label && props.aria_label.is_none() {
            ids.push(label_id.clone());
        }

        if let Some(ref labelledby) = props.aria_labelledby {
            ids.push(labelledby.clone());
        }

        if ids.is_empty() {
            None
        } else {
            Some(ids.join(" "))
        }
    };

    // Build ARIA describedby
    let has_description = props.description.is_some();
    let aria_describedby = {
        let mut ids = Vec::new();

        if let Some(ref describedby) = props.aria_describedby {
            ids.push(describedby.clone());
        }

        if has_description {
            ids.push(description_id.clone());
        }

        if ids.is_empty() {
            None
        } else {
            Some(ids.join(" "))
        }
    };

    // Build CSS classes for the wrapper
    let wrapper_classes = ClassBuilder::new()
        .add("awsui-toggle-wrapper");

    // Build CSS classes for the label wrapper
    let label_wrapper_classes = ClassBuilder::new()
        .add("awsui-toggle-label-wrapper");

    // Build CSS classes for the control
    let control_classes = ClassBuilder::new()
        .add("awsui-toggle-control")
        .add_if(props.checked, "awsui-toggle-control-checked")
        .add_if(props.disabled, "awsui-toggle-control-disabled")
        .add_if(props.read_only, "awsui-toggle-control-readonly");

    // Build CSS classes for the handle
    let handle_classes = ClassBuilder::new()
        .add("awsui-toggle-handle")
        .add_if(props.checked, "awsui-toggle-handle-checked")
        .add_if(props.disabled, "awsui-toggle-handle-disabled")
        .add_if(props.read_only, "awsui-toggle-handle-readonly");

    // Build CSS classes for the label
    let label_classes = ClassBuilder::new()
        .add("awsui-toggle-label")
        .add_if(props.disabled, "awsui-toggle-label-disabled");

    // Build CSS classes for the description
    let description_classes = ClassBuilder::new()
        .add("awsui-toggle-description")
        .add_if(props.disabled, "awsui-toggle-description-disabled");

    html! {
        <span class={wrapper_classes.build()}>
            <span
                class={label_wrapper_classes.build()}
                aria-disabled={if props.disabled { Some("true") } else { None }}
                onclick={on_click}
            >
                <span class={control_classes.build()}>
                    // Styled toggle handle
                    <span class={handle_classes.build()} />

                    // Native checkbox input (hidden, for form integration and accessibility)
                    <input
                        ref={input_ref}
                        type="checkbox"
                        role="switch"
                        id={(*control_id).clone()}
                        name={props.name.clone()}
                        class="awsui-toggle-native-input"
                        checked={props.checked}
                        disabled={props.disabled}
                        aria-checked={if props.checked { "true" } else { "false" }}
                        aria-label={props.aria_label.clone()}
                        aria-labelledby={aria_labelledby}
                        aria-describedby={aria_describedby}
                        aria-controls={props.aria_controls.clone()}
                        aria-disabled={if props.read_only && !props.disabled { Some("true") } else { None }}
                        autofocus={props.auto_focus}
                        onfocus={on_focus_event}
                        onblur={on_blur_event}
                        // Empty handler to suppress React controllability warning
                        onchange={Callback::from(|_| {})}
                    />

                    // Focus outline
                    <span class="awsui-toggle-outline" />
                </span>

                // Label and description content
                <span class={ClassBuilder::new()
                    .add("awsui-toggle-content")
                    .add_if(!has_label && !has_description, "awsui-toggle-content-empty")
                    .build()
                }>
                    if has_label {
                        <span id={label_id} class={label_classes.build()}>
                            { props.children.clone() }
                        </span>
                    }

                    if let Some(ref description) = props.description {
                        <span id={description_id} class={description_classes.build()}>
                            { description.clone() }
                        </span>
                    }
                </span>
            </span>
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_change_detail() {
        let detail = ToggleChangeDetail { checked: true };
        assert!(detail.checked);

        let detail = ToggleChangeDetail { checked: false };
        assert!(!detail.checked);
    }

    #[test]
    fn test_toggle_change_detail_clone() {
        let detail = ToggleChangeDetail { checked: true };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_toggle_change_detail_debug() {
        let detail = ToggleChangeDetail { checked: true };
        let debug_str = format!("{:?}", detail);
        assert!(debug_str.contains("checked"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_custom_event_non_cancelable() {
        let detail = ToggleChangeDetail { checked: true };
        let event = CustomEvent::new_non_cancelable(detail);

        assert!(!event.cancelable);
        assert!(!event.default_prevented);
        assert!(event.detail.checked);
    }
}
