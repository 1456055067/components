// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Multiselect component for choosing multiple options from a dropdown list.
//!
//! A dropdown selection component that allows users to choose multiple options
//! from a list of choices. Selected items are displayed as dismissible tokens,
//! and the dropdown remains open after selections. Supports disabled states,
//! validation, descriptions, label tags, and filtering.

use crate::internal::{
    AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent,
};
use web_sys::MouseEvent;
use yew::prelude::*;

/// A single option in the multiselect dropdown
#[derive(Clone, PartialEq, Debug)]
pub struct MultiselectOption {
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

impl MultiselectOption {
    /// Creates a new multiselect option with the given value
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::MultiselectOption;
    ///
    /// let option = MultiselectOption::new("us-west-2");
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

/// Filtering behavior for the multiselect
#[derive(Clone, PartialEq, Debug, Default)]
pub enum FilteringType {
    /// No filtering
    #[default]
    None,
    /// Automatic client-side filtering
    Auto,
    /// Manual filtering (user provides filtered options)
    Manual,
}

/// Event detail for multiselect change events
#[derive(Clone, PartialEq)]
pub struct MultiselectChangeDetail {
    /// The currently selected options
    pub selected_options: Vec<MultiselectOption>,
}

/// Properties for the Multiselect component
#[derive(Properties, PartialEq, Clone)]
pub struct MultiselectProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// The currently selected options (controlled component)
    #[prop_or_default]
    pub selected_options: Vec<MultiselectOption>,

    /// The list of available options to display in the dropdown
    #[prop_or_default]
    pub options: Vec<MultiselectOption>,

    /// Placeholder text shown when no options are selected
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Whether the multiselect is disabled
    ///
    /// A disabled multiselect cannot be opened or interacted with.
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the multiselect has invalid content
    ///
    /// Used to indicate validation errors.
    #[prop_or_default]
    pub invalid: bool,

    /// Whether the multiselect has a warning state
    #[prop_or_default]
    pub warning: bool,

    /// Maximum number of tokens to display before showing "+N more"
    ///
    /// Defaults to 3 if not specified.
    #[prop_or_default]
    pub token_limit: Option<usize>,

    /// Type of filtering behavior
    #[prop_or_default]
    pub filtering_type: FilteringType,

    /// Placeholder text for the filter input
    #[prop_or_default]
    pub filtering_placeholder: Option<String>,

    /// Whether to keep the dropdown open after selection
    ///
    /// Defaults to true. Set to false to close after each selection.
    #[prop_or_default]
    pub keep_open: Option<bool>,

    /// Callback fired when the selected options change
    ///
    /// The event detail contains the newly selected options.
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<MultiselectChangeDetail>>>,

    /// Callback fired when the multiselect loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Callback fired when the multiselect gains focus
    #[prop_or_default]
    pub on_focus: Option<Callback<()>>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA required attribute
    #[prop_or_default]
    pub aria_required: bool,

    /// ARIA label for the multiselect
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

    /// Automatically focus the multiselect when component is mounted
    #[prop_or_default]
    pub auto_focus: bool,
}

/// Multiselect component for choosing multiple options from a dropdown list.
///
/// A controlled dropdown component that allows users to select multiple options
/// from a list of choices. Selected items are displayed as dismissible tokens above
/// the dropdown. The component handles keyboard navigation, accessibility,
/// and supports disabled options, descriptions, label tags, and filtering.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Multiselect, MultiselectOption, MultiselectChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let selected = use_state(|| Vec::new());
///
///     let options = vec![
///         MultiselectOption::new("option1")
///             .with_label("First Option")
///             .with_description("Description for first option"),
///         MultiselectOption::new("option2")
///             .with_label("Second Option")
///             .with_label_tag("Recommended"),
///         MultiselectOption::new("option3")
///             .with_label("Third Option")
///             .with_disabled(true),
///     ];
///
///     let on_change = {
///         let selected = selected.clone();
///         Callback::from(move |event: CustomEvent<MultiselectChangeDetail>| {
///             selected.set(event.detail.selected_options.clone());
///         })
///     };
///
///     html! {
///         <Multiselect
///             selected_options={(*selected).clone()}
///             options={options}
///             placeholder="Choose options"
///             on_change={on_change}
///         />
///     }
/// }
/// ```
#[function_component(Multiselect)]
pub fn multiselect(props: &MultiselectProps) -> Html {
    let _metadata = ComponentMetadata::new("Multiselect");
    let multiselect_ref = use_node_ref();
    let is_open = use_state(|| false);
    let highlighted_index = use_state(|| 0usize);
    let filter_text = use_state(String::new);

    let keep_open = props.keep_open.unwrap_or(true);
    let token_limit = props.token_limit.unwrap_or(3);

    // Filter options based on filtering type and filter text
    let filtered_options = use_memo(
        (
            props.options.clone(),
            (*filter_text).clone(),
            props.filtering_type.clone(),
        ),
        |(options, filter, filtering_type)| {
            if *filtering_type == FilteringType::Auto && !filter.is_empty() {
                let filter_lower = filter.to_lowercase();
                options
                    .iter()
                    .filter(|opt| {
                        opt.display_text().to_lowercase().contains(&filter_lower)
                            || opt
                                .description
                                .as_ref()
                                .is_some_and(|d| d.to_lowercase().contains(&filter_lower))
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                options.clone()
            }
        },
    );

    // Handle dropdown toggle
    let on_trigger_click = {
        let is_open = is_open.clone();
        let disabled = props.disabled;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if !disabled {
                is_open.set(!*is_open);
            }
        })
    };

    // Handle option selection/deselection
    let on_option_click = {
        let is_open = is_open.clone();
        let on_change = props.on_change.clone();
        let selected_options = props.selected_options.clone();

        Callback::from(move |option: MultiselectOption| {
            if !option.disabled {
                // Toggle selection
                let mut new_selection = selected_options.clone();

                if let Some(pos) = new_selection
                    .iter()
                    .position(|opt| opt.value == option.value)
                {
                    // Remove if already selected
                    new_selection.remove(pos);
                } else {
                    // Add if not selected
                    new_selection.push(option);
                }

                if let Some(callback) = &on_change {
                    callback.emit(CustomEvent::new_non_cancelable(MultiselectChangeDetail {
                        selected_options: new_selection,
                    }));
                }

                // Close dropdown if keep_open is false
                if !keep_open {
                    is_open.set(false);
                }
            }
        })
    };

    // Handle token dismiss
    let on_token_dismiss = {
        let on_change = props.on_change.clone();
        let selected_options = props.selected_options.clone();

        Callback::from(move |option_value: String| {
            let mut new_selection = selected_options.clone();

            if let Some(pos) = new_selection
                .iter()
                .position(|opt| opt.value == option_value)
            {
                new_selection.remove(pos);

                if let Some(callback) = &on_change {
                    callback.emit(CustomEvent::new_non_cancelable(MultiselectChangeDetail {
                        selected_options: new_selection,
                    }));
                }
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

    // Handle filter input change
    let on_filter_input = {
        let filter_text = filter_text.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                filter_text.set(input.value());
            }
        })
    };

    // Handle keyboard navigation
    let on_key_down = {
        let is_open = is_open.clone();
        let highlighted_index = highlighted_index.clone();
        let filtered_options = (*filtered_options).clone();
        let on_change = props.on_change.clone();
        let selected_options = props.selected_options.clone();
        let disabled = props.disabled;

        Callback::from(move |e: KeyboardEvent| {
            if disabled {
                return;
            }

            let key = e.key();
            match key.as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if !*is_open {
                        is_open.set(true);
                    } else if !filtered_options.is_empty() {
                        // Move to next non-disabled option
                        let mut new_index = *highlighted_index;
                        loop {
                            new_index = (new_index + 1) % filtered_options.len();
                            if new_index == *highlighted_index
                                || !filtered_options[new_index].disabled
                            {
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
                    } else if !filtered_options.is_empty() {
                        // Move to previous non-disabled option
                        let mut new_index = *highlighted_index;
                        loop {
                            new_index = if new_index == 0 {
                                filtered_options.len() - 1
                            } else {
                                new_index - 1
                            };
                            if new_index == *highlighted_index
                                || !filtered_options[new_index].disabled
                            {
                                break;
                            }
                        }
                        highlighted_index.set(new_index);
                    }
                }
                " " | "Enter" => {
                    e.prevent_default();
                    if *is_open && !filtered_options.is_empty() {
                        // Toggle highlighted option
                        if let Some(option) = filtered_options.get(*highlighted_index)
                            && !option.disabled
                        {
                            let mut new_selection = selected_options.clone();

                            if let Some(pos) = new_selection
                                .iter()
                                .position(|opt| opt.value == option.value)
                            {
                                new_selection.remove(pos);
                            } else {
                                new_selection.push(option.clone());
                            }

                            if let Some(callback) = &on_change {
                                callback.emit(CustomEvent::new_non_cancelable(
                                    MultiselectChangeDetail {
                                        selected_options: new_selection,
                                    },
                                ));
                            }

                            if !keep_open {
                                is_open.set(false);
                            }
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
        .add("awsui-multiselect-trigger")
        .add_if(props.disabled, "awsui-multiselect-trigger-disabled")
        .add_if(props.invalid, "awsui-multiselect-trigger-invalid")
        .add_if(props.warning, "awsui-multiselect-trigger-warning")
        .add_if(*is_open, "awsui-multiselect-trigger-open")
        .add_if(
            props.selected_options.is_empty(),
            "awsui-multiselect-trigger-placeholder",
        );

    // Build dropdown classes
    let dropdown_classes = ClassBuilder::new()
        .add("awsui-multiselect-dropdown")
        .add_if(*is_open, "awsui-multiselect-dropdown-open");

    // Reset highlighted index when filtered options change
    use_effect_with(filtered_options.clone(), {
        let highlighted_index = highlighted_index.clone();
        move |_| {
            highlighted_index.set(0);
            || ()
        }
    });

    // Render tokens
    let render_tokens = {
        let selected_options = &props.selected_options;
        let visible_tokens = if selected_options.len() > token_limit {
            &selected_options[..token_limit]
        } else {
            selected_options
        };

        let remaining_count = if selected_options.len() > token_limit {
            selected_options.len() - token_limit
        } else {
            0
        };

        html! {
            <div class="awsui-multiselect-tokens">
                {
                    visible_tokens.iter().map(|option| {
                        let option_value = option.value.clone();
                        let on_dismiss = {
                            let on_token_dismiss = on_token_dismiss.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                e.stop_propagation();
                                on_token_dismiss.emit(option_value.clone());
                            })
                        };

                        html! {
                            <div key={option.value.clone()} class="awsui-multiselect-token">
                                <span class="awsui-multiselect-token-label">
                                    { option.display_text() }
                                </span>
                                <button
                                    type="button"
                                    class="awsui-multiselect-token-dismiss"
                                    aria-label={format!("Remove {}", option.display_text())}
                                    onclick={on_dismiss}
                                    disabled={props.disabled}
                                >
                                    { "×" }
                                </button>
                            </div>
                        }
                    }).collect::<Html>()
                }
                if remaining_count > 0 {
                    <div class="awsui-multiselect-token awsui-multiselect-token-overflow">
                        <span class="awsui-multiselect-token-label">
                            { format!("+{} more", remaining_count) }
                        </span>
                    </div>
                }
            </div>
        }
    };

    // Determine what to show in the trigger
    let trigger_content = if props.selected_options.is_empty() {
        props
            .placeholder
            .clone()
            .unwrap_or_else(|| "Choose options".to_string())
    } else {
        format!("{} selected", props.selected_options.len())
    };

    html! {
        <div
            ref={multiselect_ref}
            class={ClassBuilder::new()
                .add("awsui-multiselect")
                .add_if(props.disabled, "awsui-multiselect-disabled")
                .add_if(props.invalid, "awsui-multiselect-invalid")
                .add_if(props.warning, "awsui-multiselect-warning")
                .build()}
            onblur={on_blur_event}
            onfocus={on_focus_event}
            onkeydown={on_key_down}
        >
            // Token display (shown when there are selections)
            if !props.selected_options.is_empty() {
                { render_tokens }
            }

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
                <span class="awsui-multiselect-trigger-content">
                    { trigger_content }
                </span>
                <span class="awsui-multiselect-trigger-icon" aria-hidden="true">
                    { if *is_open { "▲" } else { "▼" } }
                </span>
            </button>

            // Dropdown menu
            if *is_open {
                <div
                    class={dropdown_classes.build()}
                    role="listbox"
                    aria-label={props.aria_label.clone()}
                    aria-multiselectable="true"
                >
                    // Filter input (for Auto or Manual filtering)
                    if props.filtering_type != FilteringType::None {
                        <div class="awsui-multiselect-filter">
                            <input
                                type="text"
                                class="awsui-multiselect-filter-input"
                                placeholder={props.filtering_placeholder.clone().unwrap_or_else(|| "Filter options".to_string())}
                                value={(*filter_text).clone()}
                                oninput={on_filter_input}
                                aria-label="Filter options"
                            />
                        </div>
                    }

                    <ul class="awsui-multiselect-options-list">
                        {
                            if filtered_options.is_empty() {
                                html! {
                                    <li class="awsui-multiselect-empty">
                                        { "No options found" }
                                    </li>
                                }
                            } else {
                                filtered_options.iter().enumerate().map(|(index, option)| {
                                    let is_selected = props.selected_options.iter()
                                        .any(|s| s.value == option.value);
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
                                        .add("awsui-multiselect-option")
                                        .add_if(option.disabled, "awsui-multiselect-option-disabled")
                                        .add_if(is_selected, "awsui-multiselect-option-selected")
                                        .add_if(is_highlighted, "awsui-multiselect-option-highlighted");

                                    html! {
                                        <li
                                            key={option.value.clone()}
                                            class={option_classes.build()}
                                            role="option"
                                            aria-selected={is_selected.to_string()}
                                            aria-disabled={option.disabled.to_string()}
                                            onclick={on_click}
                                        >
                                            <div class="awsui-multiselect-option-checkbox">
                                                <input
                                                    type="checkbox"
                                                    checked={is_selected}
                                                    disabled={option.disabled}
                                                    tabindex="-1"
                                                    aria-hidden="true"
                                                />
                                            </div>
                                            <div class="awsui-multiselect-option-content">
                                                <span class="awsui-multiselect-option-label">
                                                    { option.display_text() }
                                                    if let Some(ref tag) = option.label_tag {
                                                        <span class="awsui-multiselect-option-label-tag">
                                                            { tag }
                                                        </span>
                                                    }
                                                </span>
                                                if let Some(ref desc) = option.description {
                                                    <span class="awsui-multiselect-option-description">
                                                        { desc }
                                                    </span>
                                                }
                                            </div>
                                        </li>
                                    }
                                }).collect::<Html>()
                            }
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
    fn test_multiselect_option_new() {
        let option = MultiselectOption::new("test-value");
        assert_eq!(option.value, "test-value");
        assert_eq!(option.label, None);
        assert!(!option.disabled);
        assert_eq!(option.description, None);
        assert_eq!(option.label_tag, None);
    }

    #[test]
    fn test_multiselect_option_builder() {
        let option = MultiselectOption::new("us-west-2")
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
    fn test_multiselect_option_display_text() {
        let option_with_label = MultiselectOption::new("value").with_label("Display Label");
        assert_eq!(option_with_label.display_text(), "Display Label");

        let option_without_label = MultiselectOption::new("value");
        assert_eq!(option_without_label.display_text(), "value");
    }

    #[test]
    fn test_multiselect_change_detail() {
        let options = vec![
            MultiselectOption::new("test1"),
            MultiselectOption::new("test2"),
        ];
        let detail = MultiselectChangeDetail {
            selected_options: options.clone(),
        };

        assert_eq!(detail.selected_options.len(), 2);
        assert_eq!(detail.selected_options[0].value, "test1");
        assert_eq!(detail.selected_options[1].value, "test2");
    }

    #[test]
    fn test_multiselect_option_equality() {
        let option1 = MultiselectOption::new("value").with_label("Label");
        let option2 = MultiselectOption::new("value").with_label("Label");
        let option3 = MultiselectOption::new("different").with_label("Label");

        assert_eq!(option1, option2);
        assert_ne!(option1, option3);
    }

    #[test]
    fn test_filtering_type_default() {
        let filtering_type = FilteringType::default();
        assert_eq!(filtering_type, FilteringType::None);
    }

    #[test]
    fn test_filtering_type_equality() {
        assert_eq!(FilteringType::None, FilteringType::None);
        assert_eq!(FilteringType::Auto, FilteringType::Auto);
        assert_eq!(FilteringType::Manual, FilteringType::Manual);
        assert_ne!(FilteringType::None, FilteringType::Auto);
        assert_ne!(FilteringType::Auto, FilteringType::Manual);
    }

    #[test]
    fn test_multiselect_change_detail_empty() {
        let detail = MultiselectChangeDetail {
            selected_options: Vec::new(),
        };

        assert!(detail.selected_options.is_empty());
    }

    #[test]
    fn test_multiselect_option_clone() {
        let option1 = MultiselectOption::new("value")
            .with_label("Label")
            .with_description("Description");
        let option2 = option1.clone();

        assert_eq!(option1, option2);
        assert_eq!(option1.value, option2.value);
        assert_eq!(option1.label, option2.label);
        assert_eq!(option1.description, option2.description);
    }
}
