// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Select component for choosing one option from a dropdown list.
//!
//! A dropdown selection component that allows users to choose a single option
//! from a list of choices. Supports disabled states, validation, descriptions,
//! and label tags for options.

use crate::internal::{
    AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent,
};
use web_sys::MouseEvent;
use yew::prelude::*;

/// A single option in the select dropdown
#[derive(Clone, PartialEq, Debug)]
pub struct SelectOption {
    /// The value that will be returned when this option is selected
    pub value: String,
    /// Display label for the option
    pub label: Option<String>,
    /// Whether this option is disabled and cannot be selected
    pub disabled: bool,
    /// Optional description text shown below the label
    pub description: Option<String>,
    /// Optional tag displayed next to the label for additional context
    pub label_tag: Option<String>,
}

impl SelectOption {
    /// Creates a new select option with the given value
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::SelectOption;
    ///
    /// let option = SelectOption::new("us-west-2");
    /// ```
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: None,
            disabled: false,
            description: None,
            label_tag: None,
        }
    }

    /// Sets the display label for this option
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets whether this option is disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the description for this option
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the label tag for this option
    pub fn with_label_tag(mut self, label_tag: impl Into<String>) -> Self {
        self.label_tag = Some(label_tag.into());
        self
    }

    /// Gets the display text for this option (label or value)
    pub fn display_text(&self) -> &str {
        self.label.as_ref().unwrap_or(&self.value)
    }
}

/// Event detail for select change events
#[derive(Clone, PartialEq)]
pub struct SelectChangeDetail {
    /// The selected option
    pub selected_option: SelectOption,
}

/// Properties for the Select component
#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// The currently selected option (controlled component)
    ///
    /// Use `None` to clear the selection.
    #[prop_or_default]
    pub selected_option: Option<SelectOption>,

    /// The list of available options to display in the dropdown
    #[prop_or_default]
    pub options: Vec<SelectOption>,

    /// Placeholder text shown when no option is selected
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Whether the select is disabled
    ///
    /// A disabled select cannot be opened or interacted with.
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the select has invalid content
    ///
    /// Used to indicate validation errors.
    #[prop_or_default]
    pub invalid: bool,

    /// Whether the select is read-only
    ///
    /// A read-only select can be focused but cannot be modified.
    #[prop_or_default]
    pub read_only: bool,

    /// Callback fired when the selected option changes
    ///
    /// The event detail contains the newly selected option.
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<SelectChangeDetail>>>,

    /// Callback fired when the select loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Callback fired when the select gains focus
    #[prop_or_default]
    pub on_focus: Option<Callback<()>>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA required attribute
    #[prop_or_default]
    pub aria_required: bool,

    /// ARIA label for the select
    ///
    /// Use this when there's no visible label.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Control ID for form field integration
    ///
    /// If not provided, an auto-generated ID will be used.
    #[prop_or_default]
    pub control_id: Option<String>,

    /// HTML name attribute for form integration
    #[prop_or_default]
    pub name: Option<String>,

    /// Automatically focus the select when component is mounted
    #[prop_or_default]
    pub auto_focus: bool,
}

/// Select component for choosing one option from a dropdown list.
///
/// A controlled dropdown component that allows users to select a single option
/// from a list of choices. The component handles keyboard navigation, accessibility,
/// and supports disabled options, descriptions, and label tags.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Select, SelectOption, SelectChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let selected = use_state(|| None);
///
///     let options = vec![
///         SelectOption::new("option1")
///             .with_label("First Option")
///             .with_description("Description for first option"),
///         SelectOption::new("option2")
///             .with_label("Second Option")
///             .with_label_tag("Recommended"),
///         SelectOption::new("option3")
///             .with_label("Third Option")
///             .with_disabled(true),
///     ];
///
///     let on_change = {
///         let selected = selected.clone();
///         Callback::from(move |event: CustomEvent<SelectChangeDetail>| {
///             selected.set(Some(event.detail.selected_option.clone()));
///         })
///     };
///
///     html! {
///         <Select
///             selected_option={(*selected).clone()}
///             options={options}
///             placeholder="Choose an option"
///             on_change={on_change}
///         />
///     }
/// }
/// ```
#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let _metadata = ComponentMetadata::new("Select");
    let select_ref = use_node_ref();
    let is_open = use_state(|| false);
    let highlighted_index = use_state(|| 0usize);

    // Handle dropdown toggle
    let on_trigger_click = {
        let is_open = is_open.clone();
        let disabled = props.disabled;
        let read_only = props.read_only;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if !disabled && !read_only {
                is_open.set(!*is_open);
            }
        })
    };

    // Handle option selection
    let on_option_click = {
        let is_open = is_open.clone();
        let on_change = props.on_change.clone();

        Callback::from(move |option: SelectOption| {
            is_open.set(false);

            if !option.disabled
                && let Some(callback) = &on_change
            {
                callback.emit(CustomEvent::new_non_cancelable(SelectChangeDetail {
                    selected_option: option,
                }));
            }
        })
    };

    // Handle blur event
    let on_blur_event = {
        let on_blur = props.on_blur.clone();
        let is_open = is_open.clone();

        Callback::from(move |_e: FocusEvent| {
            // Close dropdown on blur
            is_open.set(false);

            if let Some(callback) = &on_blur {
                callback.emit(());
            }
        })
    };

    // Handle focus event
    let on_focus_event = {
        let on_focus = props.on_focus.clone();

        Callback::from(move |_e: FocusEvent| {
            if let Some(callback) = &on_focus {
                callback.emit(());
            }
        })
    };

    // Handle keyboard navigation
    let on_key_down = {
        let is_open = is_open.clone();
        let highlighted_index = highlighted_index.clone();
        let options = props.options.clone();
        let on_change = props.on_change.clone();
        let disabled = props.disabled;
        let read_only = props.read_only;

        Callback::from(move |e: KeyboardEvent| {
            if disabled || read_only {
                return;
            }

            let key = e.key();
            match key.as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if !*is_open {
                        is_open.set(true);
                    } else {
                        // Move to next non-disabled option
                        let mut new_index = *highlighted_index;
                        loop {
                            new_index = (new_index + 1) % options.len();
                            if new_index == *highlighted_index || !options[new_index].disabled {
                                break;
                            }
                        }
                        highlighted_index.set(new_index);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if !*is_open {
                        is_open.set(true);
                    } else {
                        // Move to previous non-disabled option
                        let mut new_index = *highlighted_index;
                        loop {
                            new_index = if new_index == 0 {
                                options.len() - 1
                            } else {
                                new_index - 1
                            };
                            if new_index == *highlighted_index || !options[new_index].disabled {
                                break;
                            }
                        }
                        highlighted_index.set(new_index);
                    }
                }
                "Enter" | " " => {
                    e.prevent_default();
                    if *is_open {
                        // Select highlighted option
                        if let Some(option) = options.get(*highlighted_index)
                            && !option.disabled
                        {
                            if let Some(callback) = &on_change {
                                callback.emit(CustomEvent::new_non_cancelable(
                                    SelectChangeDetail {
                                        selected_option: option.clone(),
                                    },
                                ));
                            }
                            is_open.set(false);
                        }
                    } else {
                        is_open.set(true);
                    }
                }
                "Escape" => {
                    e.prevent_default();
                    is_open.set(false);
                }
                _ => {}
            }
        })
    };

    // Build trigger classes
    let trigger_classes = ClassBuilder::new()
        .add("awsui-select-trigger")
        .add_if(props.disabled, "awsui-select-trigger-disabled")
        .add_if(props.read_only, "awsui-select-trigger-readonly")
        .add_if(props.invalid, "awsui-select-trigger-invalid")
        .add_if(*is_open, "awsui-select-trigger-open")
        .add_if(
            props.selected_option.is_none(),
            "awsui-select-trigger-placeholder",
        );

    // Build dropdown classes
    let dropdown_classes = ClassBuilder::new()
        .add("awsui-select-dropdown")
        .add_if(*is_open, "awsui-select-dropdown-open");

    // Determine what to show in the trigger
    let trigger_content = if let Some(ref option) = props.selected_option {
        option.display_text().to_string()
    } else {
        props
            .placeholder
            .clone()
            .unwrap_or_else(|| "Select an option".to_string())
    };

    // Find index of selected option for initial highlight
    use_effect_with((props.selected_option.clone(), props.options.clone()), {
        let highlighted_index = highlighted_index.clone();
        move |(selected, options)| {
            if let Some(selected_opt) = selected
                && let Some(index) = options
                    .iter()
                    .position(|opt| opt.value == selected_opt.value)
            {
                highlighted_index.set(index);
            }
            || ()
        }
    });

    // Close dropdown when clicking outside
    use_effect_with(*is_open, {
        let _is_open = is_open.clone();
        move |open| {
            if *open {
                // Setup click outside handler
                let window = web_sys::window().expect("window should exist");
                let _document = window.document().expect("document should exist");

                // Note: In production, you'd want to add a proper event listener
                // This is a simplified version for demonstration
            }
            || ()
        }
    });

    html! {
        <div
            ref={select_ref}
            class={ClassBuilder::new()
                .add("awsui-select")
                .add_if(props.disabled, "awsui-select-disabled")
                .add_if(props.invalid, "awsui-select-invalid")
                .build()}
            onblur={on_blur_event}
            onfocus={on_focus_event}
            onkeydown={on_key_down}
        >
            // Trigger button
            <button
                type="button"
                class={trigger_classes.build()}
                id={props.control_id.clone()}
                disabled={props.disabled}
                aria-expanded={is_open.to_string()}
                aria-haspopup="listbox"
                aria-label={props.aria_label.clone()}
                aria-labelledby={props.aria.labelledby.clone()}
                aria-describedby={props.aria.describedby.clone()}
                aria-required={props.aria_required.to_string()}
                aria-invalid={props.invalid.to_string()}
                onclick={on_trigger_click}
                autofocus={props.auto_focus}
            >
                <span class="awsui-select-trigger-content">
                    { trigger_content }
                </span>
                <span class="awsui-select-trigger-icon" aria-hidden="true">
                    { if *is_open { "▲" } else { "▼" } }
                </span>
            </button>

            // Dropdown menu
            if *is_open {
                <div
                    class={dropdown_classes.build()}
                    role="listbox"
                    aria-label={props.aria_label.clone()}
                >
                    <ul class="awsui-select-options-list">
                        {
                            props.options.iter().enumerate().map(|(index, option)| {
                                let is_selected = props.selected_option.as_ref()
                                    .map(|s| s.value == option.value)
                                    .unwrap_or(false);
                                let is_highlighted = index == *highlighted_index;

                                let option_clone = option.clone();
                                let on_click = {
                                    let on_option_click = on_option_click.clone();
                                    Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();
                                        on_option_click.emit(option_clone.clone());
                                    })
                                };

                                let option_classes = ClassBuilder::new()
                                    .add("awsui-select-option")
                                    .add_if(option.disabled, "awsui-select-option-disabled")
                                    .add_if(is_selected, "awsui-select-option-selected")
                                    .add_if(is_highlighted, "awsui-select-option-highlighted");

                                html! {
                                    <li
                                        key={option.value.clone()}
                                        class={option_classes.build()}
                                        role="option"
                                        aria-selected={is_selected.to_string()}
                                        aria-disabled={option.disabled.to_string()}
                                        onclick={on_click}
                                    >
                                        <div class="awsui-select-option-content">
                                            <span class="awsui-select-option-label">
                                                { option.display_text() }
                                                if let Some(ref tag) = option.label_tag {
                                                    <span class="awsui-select-option-label-tag">
                                                        { tag }
                                                    </span>
                                                }
                                            </span>
                                            if let Some(ref desc) = option.description {
                                                <span class="awsui-select-option-description">
                                                    { desc }
                                                </span>
                                            }
                                        </div>
                                        if is_selected {
                                            <span class="awsui-select-option-checkmark" aria-hidden="true">
                                                { "✓" }
                                            </span>
                                        }
                                    </li>
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_new() {
        let option = SelectOption::new("test-value");
        assert_eq!(option.value, "test-value");
        assert_eq!(option.label, None);
        assert!(!option.disabled);
        assert_eq!(option.description, None);
        assert_eq!(option.label_tag, None);
    }

    #[test]
    fn test_select_option_builder() {
        let option = SelectOption::new("us-west-2")
            .with_label("US West (Oregon)")
            .with_description("Northern California region")
            .with_label_tag("Recommended")
            .with_disabled(true);

        assert_eq!(option.value, "us-west-2");
        assert_eq!(option.label, Some("US West (Oregon)".to_string()));
        assert_eq!(
            option.description,
            Some("Northern California region".to_string())
        );
        assert_eq!(option.label_tag, Some("Recommended".to_string()));
        assert!(option.disabled);
    }

    #[test]
    fn test_select_option_display_text() {
        let option_with_label = SelectOption::new("value").with_label("Display Label");
        assert_eq!(option_with_label.display_text(), "Display Label");

        let option_without_label = SelectOption::new("value");
        assert_eq!(option_without_label.display_text(), "value");
    }

    #[test]
    fn test_select_change_detail() {
        let option = SelectOption::new("test");
        let detail = SelectChangeDetail {
            selected_option: option.clone(),
        };

        assert_eq!(detail.selected_option.value, "test");
    }

    #[test]
    fn test_select_option_equality() {
        let option1 = SelectOption::new("value").with_label("Label");
        let option2 = SelectOption::new("value").with_label("Label");
        let option3 = SelectOption::new("different").with_label("Label");

        assert_eq!(option1, option2);
        assert_ne!(option1, option3);
    }
}
