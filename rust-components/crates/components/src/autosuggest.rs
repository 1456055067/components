// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Autosuggest component for text input with dropdown suggestions.
//!
//! An input component that provides suggestions as the user types, with support
//! for filtering, keyboard navigation, and custom "Use entered text" option.

use crate::internal::{
    AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent,
};
use gloo_timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Filtering type for autosuggest options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilteringType {
    /// No filtering, show all options
    None,
    /// Automatically filter options based on input value
    #[default]
    Auto,
}

/// A single option in the autosuggest dropdown
#[derive(Clone, PartialEq, Debug)]
pub struct AutosuggestOption {
    /// The value that will be returned when this option is selected
    pub value: String,
    /// Display label for the option
    pub label: Option<String>,
    /// Optional description text shown below the label
    pub description: Option<String>,
    /// Optional tag displayed next to the label for additional context
    pub label_tag: Option<String>,
    /// Whether this option is disabled and cannot be selected
    pub disabled: bool,
}

impl AutosuggestOption {
    /// Creates a new autosuggest option with the given value
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::AutosuggestOption;
    ///
    /// let option = AutosuggestOption::new("suggestion");
    /// ```
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: None,
            description: None,
            label_tag: None,
            disabled: false,
        }
    }

    /// Sets the display label for this option
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
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

    /// Sets whether this option is disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Gets the display text for this option (label or value)
    pub fn display_text(&self) -> &str {
        self.label.as_ref().unwrap_or(&self.value)
    }

    /// Checks if this option matches the filter text
    pub fn matches_filter(&self, filter: &str) -> bool {
        if filter.is_empty() {
            return true;
        }

        let filter_lower = filter.to_lowercase();
        let value_lower = self.value.to_lowercase();
        let label_lower = self.display_text().to_lowercase();

        value_lower.contains(&filter_lower) || label_lower.contains(&filter_lower)
    }
}

/// Event detail for autosuggest change events
#[derive(Clone, PartialEq)]
pub struct AutosuggestChangeDetail {
    /// The new input value
    pub value: String,
}

/// Event detail for autosuggest select events
#[derive(Clone, PartialEq)]
pub struct AutosuggestSelectDetail {
    /// The selected value
    pub value: String,
    /// The selected option (None if user selected entered text)
    pub selected_option: Option<AutosuggestOption>,
}

/// Properties for the Autosuggest component
#[derive(Properties, PartialEq, Clone)]
pub struct AutosuggestProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Current input value (controlled component)
    #[prop_or_default]
    pub value: String,

    /// The list of available suggestions
    #[prop_or_default]
    pub options: Vec<AutosuggestOption>,

    /// Placeholder text shown when input is empty
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Whether the autosuggest is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the autosuggest has invalid content
    #[prop_or_default]
    pub invalid: bool,

    /// Whether to show a warning
    #[prop_or_default]
    pub warning: bool,

    /// Label for the "Use entered text" option (default: "Use:")
    #[prop_or_default]
    pub entered_text_label: Option<String>,

    /// Filtering type for options
    #[prop_or_default]
    pub filtering_type: FilteringType,

    /// Callback fired when the input value changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<AutosuggestChangeDetail>>>,

    /// Callback fired when an option is selected
    #[prop_or_default]
    pub on_select: Option<Callback<CustomEvent<AutosuggestSelectDetail>>>,

    /// Callback fired when the input loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Callback fired when the input gains focus
    #[prop_or_default]
    pub on_focus: Option<Callback<()>>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA required attribute
    #[prop_or_default]
    pub aria_required: bool,

    /// Control ID for form field integration
    #[prop_or_default]
    pub control_id: Option<String>,

    /// HTML name attribute
    #[prop_or_default]
    pub name: Option<String>,

    /// Auto-focus on mount
    #[prop_or_default]
    pub auto_focus: bool,
}

/// Autosuggest component for text input with dropdown suggestions.
///
/// An input component that shows suggestions in a dropdown as the user types.
/// Supports filtering, keyboard navigation, and custom "Use entered text" option.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Autosuggest, AutosuggestOption, AutosuggestChangeDetail, AutosuggestSelectDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let value = use_state(|| String::new());
///
///     let options = vec![
///         AutosuggestOption::new("apple")
///             .with_label("Apple")
///             .with_description("A red fruit"),
///         AutosuggestOption::new("banana")
///             .with_label("Banana")
///             .with_description("A yellow fruit"),
///         AutosuggestOption::new("cherry")
///             .with_label("Cherry")
///             .with_description("A small red fruit"),
///     ];
///
///     let on_change = {
///         let value = value.clone();
///         Callback::from(move |event: CustomEvent<AutosuggestChangeDetail>| {
///             value.set(event.detail.value.clone());
///         })
///     };
///
///     let on_select = {
///         let value = value.clone();
///         Callback::from(move |event: CustomEvent<AutosuggestSelectDetail>| {
///             value.set(event.detail.value.clone());
///         })
///     };
///
///     html! {
///         <Autosuggest
///             value={(*value).clone()}
///             options={options}
///             placeholder="Type to search..."
///             on_change={on_change}
///             on_select={on_select}
///         />
///     }
/// }
/// ```
#[function_component(Autosuggest)]
pub fn autosuggest(props: &AutosuggestProps) -> Html {
    let _metadata = ComponentMetadata::new("Autosuggest");
    let input_ref = use_node_ref();
    let is_open = use_state(|| false);
    let highlighted_index = use_state(|| 0usize);

    // Filter options based on current value
    let filtered_options = use_memo(
        (
            props.options.clone(),
            props.value.clone(),
            props.filtering_type,
        ),
        |(options, value, filtering_type)| {
            if *filtering_type == FilteringType::Auto && !value.is_empty() {
                options
                    .iter()
                    .filter(|opt| opt.matches_filter(value))
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                options.clone()
            }
        },
    );

    // Determine if we should show the "Use entered text" option
    let show_entered_text =
        !props.value.is_empty() && !filtered_options.iter().any(|opt| opt.value == props.value);

    // Calculate total dropdown items (including "Use entered text" if shown)
    let total_items = filtered_options.len() + if show_entered_text { 1 } else { 0 };

    // Handle input change
    let on_input = {
        let on_change = props.on_change.clone();
        let is_open = is_open.clone();
        let highlighted_index = highlighted_index.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                let value = target.value();

                // Open dropdown when user types
                is_open.set(true);
                highlighted_index.set(0);

                if let Some(callback) = &on_change {
                    callback.emit(CustomEvent::new_non_cancelable(AutosuggestChangeDetail {
                        value,
                    }));
                }
            }
        })
    };

    // Handle focus
    let on_focus_event = {
        let on_focus = props.on_focus.clone();
        let is_open = is_open.clone();

        Callback::from(move |_e: FocusEvent| {
            is_open.set(true);

            if let Some(callback) = &on_focus {
                callback.emit(());
            }
        })
    };

    // Handle blur
    let on_blur_event = {
        let on_blur = props.on_blur.clone();
        let is_open = is_open.clone();

        Callback::from(move |_e: FocusEvent| {
            // Delay closing to allow click events on options to fire
            let is_open = is_open.clone();
            Timeout::new(200, move || {
                is_open.set(false);
            })
            .forget();

            if let Some(callback) = &on_blur {
                callback.emit(());
            }
        })
    };

    // Handle option selection
    let on_option_select = {
        let on_select = props.on_select.clone();
        let is_open = is_open.clone();

        Callback::from(
            move |(value, option): (String, Option<AutosuggestOption>)| {
                is_open.set(false);

                if let Some(callback) = &on_select {
                    callback.emit(CustomEvent::new_non_cancelable(AutosuggestSelectDetail {
                        value,
                        selected_option: option,
                    }));
                }
            },
        )
    };

    // Handle keyboard navigation
    let on_key_down = {
        let is_open = is_open.clone();
        let highlighted_index = highlighted_index.clone();
        let on_select = props.on_select.clone();
        let filtered_options = filtered_options.clone();
        let value = props.value.clone();

        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            match key.as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if !*is_open {
                        is_open.set(true);
                    } else if total_items > 0 {
                        let new_index = (*highlighted_index + 1) % total_items;
                        highlighted_index.set(new_index);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if !*is_open {
                        is_open.set(true);
                    } else if total_items > 0 {
                        let new_index = if *highlighted_index == 0 {
                            total_items - 1
                        } else {
                            *highlighted_index - 1
                        };
                        highlighted_index.set(new_index);
                    }
                }
                "Enter" => {
                    if *is_open && total_items > 0 {
                        e.prevent_default();

                        // Determine what was selected
                        let idx = *highlighted_index;
                        if show_entered_text && idx == 0 {
                            // Selected "Use entered text"
                            if let Some(callback) = &on_select {
                                callback.emit(CustomEvent::new_non_cancelable(
                                    AutosuggestSelectDetail {
                                        value: value.clone(),
                                        selected_option: None,
                                    },
                                ));
                            }
                        } else {
                            // Selected an option
                            let option_idx = if show_entered_text { idx - 1 } else { idx };
                            if let Some(option) = filtered_options.get(option_idx)
                                && !option.disabled
                                && let Some(callback) = &on_select
                            {
                                callback.emit(CustomEvent::new_non_cancelable(
                                    AutosuggestSelectDetail {
                                        value: option.value.clone(),
                                        selected_option: Some(option.clone()),
                                    },
                                ));
                            }
                        }

                        is_open.set(false);
                    }
                }
                "Escape" => {
                    e.prevent_default();
                    is_open.set(false);
                }
                "Tab" => {
                    // Let Tab work normally, but close dropdown
                    is_open.set(false);
                }
                _ => {}
            }
        })
    };

    // Reset highlighted index when options change
    use_effect_with(filtered_options.clone(), {
        let highlighted_index = highlighted_index.clone();
        move |_| {
            highlighted_index.set(0);
            || ()
        }
    });

    // Build input classes
    let input_classes = ClassBuilder::new()
        .add("awsui-autosuggest-input")
        .add_if(props.disabled, "awsui-autosuggest-input-disabled")
        .add_if(props.invalid, "awsui-autosuggest-input-invalid")
        .add_if(
            !props.invalid && props.warning,
            "awsui-autosuggest-input-warning",
        );

    // Build dropdown classes
    let dropdown_classes = ClassBuilder::new()
        .add("awsui-autosuggest-dropdown")
        .add_if(
            *is_open && total_items > 0,
            "awsui-autosuggest-dropdown-open",
        );

    // Determine entered text label
    let entered_text_label = props
        .entered_text_label
        .clone()
        .unwrap_or_else(|| "Use:".to_string());

    html! {
        <div
            class={ClassBuilder::new()
                .add("awsui-autosuggest")
                .add_if(props.disabled, "awsui-autosuggest-disabled")
                .add_if(props.invalid, "awsui-autosuggest-invalid")
                .build()}
        >
            // Input element
            <input
                ref={input_ref}
                type="text"
                class={input_classes.build()}
                id={props.control_id.clone()}
                name={props.name.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                autofocus={props.auto_focus}
                role="combobox"
                aria-expanded={(*is_open && total_items > 0).to_string()}
                aria-autocomplete="list"
                aria-controls="autosuggest-dropdown"
                aria-label={props.aria.label.clone()}
                aria-labelledby={props.aria.labelledby.clone()}
                aria-describedby={props.aria.describedby.clone()}
                aria-required={props.aria_required.to_string()}
                aria-invalid={props.invalid.to_string()}
                oninput={on_input}
                onblur={on_blur_event}
                onfocus={on_focus_event}
                onkeydown={on_key_down}
            />

            // Dropdown menu
            if *is_open && total_items > 0 {
                <div
                    id="autosuggest-dropdown"
                    class={dropdown_classes.build()}
                    role="listbox"
                >
                    <ul class="awsui-autosuggest-options-list">
                        // "Use entered text" option
                        if show_entered_text {
                            {{
                                let is_highlighted = *highlighted_index == 0;
                                let value = props.value.clone();
                                let on_select = on_option_select.clone();

                                let on_click = Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    on_select.emit((value.clone(), None));
                                });

                                let option_classes = ClassBuilder::new()
                                    .add("awsui-autosuggest-option")
                                    .add("awsui-autosuggest-entered-text")
                                    .add_if(is_highlighted, "awsui-autosuggest-option-highlighted")
                                    .build();

                                html! {
                                    <li
                                        class={option_classes}
                                        role="option"
                                        aria-selected={is_highlighted.to_string()}
                                        onclick={on_click}
                                    >
                                        <span class="awsui-autosuggest-option-label">
                                            { format!("{} \"{}\"", entered_text_label, props.value) }
                                        </span>
                                    </li>
                                }
                            }}
                        }

                        // Regular options
                        {
                            filtered_options.iter().enumerate().map(|(idx, option)| {
                                let actual_idx = if show_entered_text { idx + 1 } else { idx };
                                let is_highlighted = *highlighted_index == actual_idx;

                                let option_clone = option.clone();
                                let value = option.value.clone();
                                let on_select = on_option_select.clone();

                                let on_click = Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    if !option_clone.disabled {
                                        on_select.emit((value.clone(), Some(option_clone.clone())));
                                    }
                                });

                                let option_classes = ClassBuilder::new()
                                    .add("awsui-autosuggest-option")
                                    .add_if(option.disabled, "awsui-autosuggest-option-disabled")
                                    .add_if(is_highlighted, "awsui-autosuggest-option-highlighted");

                                html! {
                                    <li
                                        key={option.value.clone()}
                                        class={option_classes.build()}
                                        role="option"
                                        aria-selected={is_highlighted.to_string()}
                                        aria-disabled={option.disabled.to_string()}
                                        onclick={on_click}
                                    >
                                        <div class="awsui-autosuggest-option-content">
                                            <span class="awsui-autosuggest-option-label">
                                                { option.display_text() }
                                                if let Some(ref tag) = option.label_tag {
                                                    <span class="awsui-autosuggest-option-label-tag">
                                                        { tag }
                                                    </span>
                                                }
                                            </span>
                                            if let Some(ref desc) = option.description {
                                                <span class="awsui-autosuggest-option-description">
                                                    { desc }
                                                </span>
                                            }
                                        </div>
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
    fn test_autosuggest_option_new() {
        let option = AutosuggestOption::new("test-value");
        assert_eq!(option.value, "test-value");
        assert_eq!(option.label, None);
        assert_eq!(option.description, None);
        assert_eq!(option.label_tag, None);
        assert!(!option.disabled);
    }

    #[test]
    fn test_autosuggest_option_builder() {
        let option = AutosuggestOption::new("apple")
            .with_label("Apple Fruit")
            .with_description("A delicious red fruit")
            .with_label_tag("Popular")
            .with_disabled(true);

        assert_eq!(option.value, "apple");
        assert_eq!(option.label, Some("Apple Fruit".to_string()));
        assert_eq!(
            option.description,
            Some("A delicious red fruit".to_string())
        );
        assert_eq!(option.label_tag, Some("Popular".to_string()));
        assert!(option.disabled);
    }

    #[test]
    fn test_autosuggest_option_display_text() {
        let option_with_label = AutosuggestOption::new("value").with_label("Display Label");
        assert_eq!(option_with_label.display_text(), "Display Label");

        let option_without_label = AutosuggestOption::new("value");
        assert_eq!(option_without_label.display_text(), "value");
    }

    #[test]
    fn test_autosuggest_option_matches_filter() {
        let option = AutosuggestOption::new("apple").with_label("Red Apple");

        // Empty filter matches everything
        assert!(option.matches_filter(""));

        // Case-insensitive matching on value
        assert!(option.matches_filter("app"));
        assert!(option.matches_filter("APP"));
        assert!(option.matches_filter("Apple"));

        // Case-insensitive matching on label
        assert!(option.matches_filter("red"));
        assert!(option.matches_filter("RED"));
        assert!(option.matches_filter("Red Apple"));

        // No match
        assert!(!option.matches_filter("banana"));
        assert!(!option.matches_filter("xyz"));
    }

    #[test]
    fn test_autosuggest_option_equality() {
        let option1 = AutosuggestOption::new("value").with_label("Label");
        let option2 = AutosuggestOption::new("value").with_label("Label");
        let option3 = AutosuggestOption::new("different").with_label("Label");

        assert_eq!(option1, option2);
        assert_ne!(option1, option3);
    }

    #[test]
    fn test_filtering_type_default() {
        assert_eq!(FilteringType::default(), FilteringType::Auto);
    }

    #[test]
    fn test_autosuggest_change_detail() {
        let detail = AutosuggestChangeDetail {
            value: "test".to_string(),
        };
        assert_eq!(detail.value, "test");
    }

    #[test]
    fn test_autosuggest_select_detail() {
        let option = AutosuggestOption::new("test");
        let detail = AutosuggestSelectDetail {
            value: "test".to_string(),
            selected_option: Some(option.clone()),
        };

        assert_eq!(detail.value, "test");
        assert_eq!(detail.selected_option.unwrap().value, "test");
    }

    #[test]
    fn test_autosuggest_select_detail_no_option() {
        let detail = AutosuggestSelectDetail {
            value: "custom text".to_string(),
            selected_option: None,
        };

        assert_eq!(detail.value, "custom text");
        assert!(detail.selected_option.is_none());
    }

    #[test]
    fn test_filtering_options() {
        let options = vec![
            AutosuggestOption::new("apple").with_label("Apple"),
            AutosuggestOption::new("apricot").with_label("Apricot"),
            AutosuggestOption::new("banana").with_label("Banana"),
            AutosuggestOption::new("cherry").with_label("Cherry"),
        ];

        // Filter for "ap"
        let filtered: Vec<_> = options
            .iter()
            .filter(|opt| opt.matches_filter("ap"))
            .collect();
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].value, "apple");
        assert_eq!(filtered[1].value, "apricot");

        // Filter for "ban"
        let filtered: Vec<_> = options
            .iter()
            .filter(|opt| opt.matches_filter("ban"))
            .collect();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].value, "banana");

        // Empty filter matches all
        let filtered: Vec<_> = options
            .iter()
            .filter(|opt| opt.matches_filter(""))
            .collect();
        assert_eq!(filtered.len(), 4);
    }
}
