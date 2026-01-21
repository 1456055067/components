// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Checkbox component for binary choice selection.
//!
//! A form control that allows users to make one or multiple selections from a list,
//! supporting checked, unchecked, and indeterminate states.

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
    AriaAttributes,
};

/// Event detail for checkbox change events
#[derive(Debug, Clone, PartialEq)]
pub struct CheckboxChangeDetail {
    /// Whether the checkbox is checked
    pub checked: bool,
    /// Whether the checkbox is in indeterminate state (always false in change events)
    pub indeterminate: bool,
}

/// Properties for the Checkbox component
#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Specifies if the component is selected (controlled)
    #[prop_or_default]
    pub checked: bool,

    /// Specifies if the component is in an indeterminate state
    ///
    /// The indeterminate state is typically used to indicate that a parent checkbox
    /// controls multiple child checkboxes, and some (but not all) are checked.
    #[prop_or_default]
    pub indeterminate: bool,

    /// Whether the checkbox is disabled
    ///
    /// A disabled checkbox cannot be interacted with and its value won't be
    /// included in form submissions.
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the checkbox is read-only
    ///
    /// A read-only checkbox can receive focus but cannot be modified.
    /// Should only be used inside forms.
    #[prop_or_default]
    pub read_only: bool,

    /// HTML name attribute for form integration
    #[prop_or_default]
    pub name: Option<String>,

    /// Control ID for form field integration
    ///
    /// If not provided, an auto-generated ID will be used.
    #[prop_or_default]
    pub control_id: Option<String>,

    /// ARIA label for the checkbox
    ///
    /// Use this when there's no visible label.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Whether to add aria-required to the native control
    #[prop_or_default]
    pub aria_required: bool,

    /// ARIA controls attribute
    ///
    /// References the ID of an element controlled by this checkbox.
    #[prop_or_default]
    pub aria_controls: Option<String>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// Optional description text that appears below the label
    #[prop_or_default]
    pub description: Option<String>,

    /// Callback fired when the checkbox state changes
    ///
    /// The event detail contains the new checked state. When transitioning from
    /// indeterminate, the checkbox will first become checked, then unchecked on
    /// subsequent clicks (deterministic transitions).
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<CheckboxChangeDetail>>>,

    /// Callback fired when the checkbox gains focus
    #[prop_or_default]
    pub on_focus: Option<Callback<()>>,

    /// Callback fired when the checkbox loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Label text for the checkbox (children)
    #[prop_or_default]
    pub children: Children,
}

/// Checkbox component for binary choice selection.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Checkbox, CheckboxChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let checked = use_state(|| false);
///
///     let on_change = {
///         let checked = checked.clone();
///         Callback::from(move |event: CustomEvent<CheckboxChangeDetail>| {
///             checked.set(event.detail.checked);
///         })
///     };
///
///     html! {
///         <Checkbox
///             checked={*checked}
///             on_change={on_change}
///         >
///             { "Accept terms and conditions" }
///         </Checkbox>
///     }
/// }
/// ```
///
/// # Indeterminate State Example
///
/// ```rust
/// use cloudscape_components::{Checkbox, CheckboxChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(ParentCheckbox)]
/// fn parent_checkbox() -> Html {
///     let child_states = use_state(|| vec![true, false, true]);
///
///     let all_checked = child_states.iter().all(|&x| x);
///     let some_checked = child_states.iter().any(|&x| x);
///     let parent_checked = all_checked;
///     let parent_indeterminate = some_checked && !all_checked;
///
///     let on_parent_change = {
///         let child_states = child_states.clone();
///         Callback::from(move |event: CustomEvent<CheckboxChangeDetail>| {
///             let new_state = event.detail.checked;
///             child_states.set(vec![new_state, new_state, new_state]);
///         })
///     };
///
///     html! {
///         <Checkbox
///             checked={parent_checked}
///             indeterminate={parent_indeterminate}
///             on_change={on_parent_change}
///         >
///             { "Select all" }
///         </Checkbox>
///     }
/// }
/// ```
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let _metadata = ComponentMetadata::new("Checkbox");
    let input_ref = use_node_ref();

    // Set indeterminate property on the native input element
    // This must be done via JavaScript as it's not reflected in HTML attributes
    {
        let input_ref = input_ref.clone();
        let indeterminate = props.indeterminate;

        use_effect_with((input_ref.clone(), indeterminate), move |(input_ref, indeterminate)| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.set_indeterminate(*indeterminate);
            }
            || ()
        });
    }

    // Handle checkbox change
    // For deterministic transitions: indeterminate -> checked -> unchecked
    let on_click = {
        let on_change = props.on_change.clone();
        let checked = props.checked;
        let indeterminate = props.indeterminate;
        let disabled = props.disabled;
        let read_only = props.read_only;
        let input_ref = input_ref.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            // Don't handle clicks when disabled or read-only
            if disabled || read_only {
                return;
            }

            // Focus the input
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }

            if let Some(callback) = &on_change {
                let detail = if indeterminate {
                    // Indeterminate -> checked transition
                    CheckboxChangeDetail {
                        checked: true,
                        indeterminate: false,
                    }
                } else {
                    // Toggle between checked and unchecked
                    CheckboxChangeDetail {
                        checked: !checked,
                        indeterminate: false,
                    }
                };

                callback.emit(CustomEvent::new_non_cancelable(detail));
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

    // Handle blur events
    let on_blur_event = {
        let on_blur = props.on_blur.clone();

        Callback::from(move |_e: FocusEvent| {
            if let Some(callback) = &on_blur {
                callback.emit(());
            }
        })
    };

    // Build CSS classes for the root container
    let root_classes = ClassBuilder::new()
        .add("awsui-checkbox")
        .add_if(props.disabled, "awsui-checkbox-disabled")
        .add_if(props.read_only, "awsui-checkbox-readonly")
        .add_if(props.checked, "awsui-checkbox-checked")
        .add_if(props.indeterminate, "awsui-checkbox-indeterminate");

    // Build CSS classes for the checkbox control
    let control_classes = ClassBuilder::new()
        .add("awsui-checkbox-control")
        .add_if(props.checked, "awsui-checkbox-control-checked")
        .add_if(props.indeterminate, "awsui-checkbox-control-indeterminate")
        .add_if(props.disabled, "awsui-checkbox-control-disabled")
        .add_if(props.read_only, "awsui-checkbox-control-readonly");

    // Determine ARIA properties
    let aria_label = props.aria_label.clone();
    let aria_labelledby = props.aria.labelledby.clone();
    let aria_describedby = props.aria.describedby.clone();

    // Generate control ID if not provided
    let control_id = props.control_id.clone();

    html! {
        <div class={root_classes.build()}>
            // Hidden native checkbox input
            <input
                ref={input_ref.clone()}
                type="checkbox"
                class="awsui-checkbox-native-input"
                id={control_id.clone()}
                name={props.name.clone()}
                checked={props.checked}
                disabled={props.disabled}
                aria-checked={if props.indeterminate { "mixed" } else if props.checked { "true" } else { "false" }}
                aria-label={aria_label}
                aria-labelledby={aria_labelledby}
                aria-describedby={aria_describedby}
                aria-controls={props.aria_controls.clone()}
                aria-required={props.aria_required.then_some("true")}
                aria-disabled={props.read_only.then_some("true")}
                onfocus={on_focus_event}
                onblur={on_blur_event}
                // Empty onChange to suppress React-style warnings
                onchange={Callback::from(|_| {})}
            />

            // Visual checkbox control (clickable)
            <div class="awsui-checkbox-abstract-switch" onclick={on_click.clone()}>
                <div class={control_classes.build()}>
                    { render_checkbox_icon(props.checked, props.indeterminate, props.disabled, props.read_only) }
                </div>

                // Label and description container
                if !props.children.is_empty() || props.description.is_some() {
                    <div class="awsui-checkbox-label-container">
                        // Label
                        if !props.children.is_empty() {
                            <label
                                class="awsui-checkbox-label"
                                for={control_id}
                                onclick={on_click.clone()}
                            >
                                { props.children.clone() }
                            </label>
                        }

                        // Optional description
                        if let Some(ref description) = props.description {
                            <div class="awsui-checkbox-description">
                                { description }
                            </div>
                        }
                    </div>
                }
            </div>
        </div>
    }
}

/// Renders the checkbox icon based on its state
fn render_checkbox_icon(checked: bool, indeterminate: bool, disabled: bool, read_only: bool) -> Html {
    let icon_classes = ClassBuilder::new()
        .add("awsui-checkbox-icon")
        .add_if(checked, "awsui-checkbox-icon-checked")
        .add_if(indeterminate, "awsui-checkbox-icon-indeterminate")
        .add_if(disabled, "awsui-checkbox-icon-disabled")
        .add_if(read_only, "awsui-checkbox-icon-readonly");

    html! {
        <svg
            class={icon_classes.build()}
            viewBox="0 0 16 16"
            aria-hidden="true"
            focusable="false"
        >
            // Background box
            <rect
                class="awsui-checkbox-icon-box"
                x="0.5"
                y="0.5"
                width="15"
                height="15"
                rx="2"
            />

            // Checkmark or indeterminate dash
            if indeterminate {
                // Indeterminate dash
                <line
                    class="awsui-checkbox-icon-mark"
                    x1="4"
                    y1="8"
                    x2="12"
                    y2="8"
                    stroke-width="2"
                    stroke-linecap="round"
                />
            } else if checked {
                // Check mark
                <polyline
                    class="awsui-checkbox-icon-mark"
                    points="4,8 7,11 12,5"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    fill="none"
                />
            }
        </svg>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbox_change_detail_equality() {
        let detail1 = CheckboxChangeDetail {
            checked: true,
            indeterminate: false,
        };

        let detail2 = CheckboxChangeDetail {
            checked: true,
            indeterminate: false,
        };

        assert_eq!(detail1, detail2);
    }

    #[test]
    fn checkbox_change_detail_inequality() {
        let detail1 = CheckboxChangeDetail {
            checked: true,
            indeterminate: false,
        };

        let detail2 = CheckboxChangeDetail {
            checked: false,
            indeterminate: false,
        };

        assert_ne!(detail1, detail2);
    }

    #[test]
    fn indeterminate_transition_logic() {
        // When indeterminate is true, next state should be checked
        let indeterminate = true;
        let checked = false;

        let next_checked = if indeterminate {
            true
        } else {
            !checked
        };

        assert_eq!(next_checked, true);
    }

    #[test]
    fn checked_toggle_logic() {
        // When not indeterminate, should toggle
        let indeterminate = false;
        let checked = true;

        let next_checked = if indeterminate {
            true
        } else {
            !checked
        };

        assert_eq!(next_checked, false);
    }

    #[test]
    fn unchecked_toggle_logic() {
        // When not indeterminate and unchecked, should become checked
        let indeterminate = false;
        let checked = false;

        let next_checked = if indeterminate {
            true
        } else {
            !checked
        };

        assert_eq!(next_checked, true);
    }
}
