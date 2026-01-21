// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Input component for text entry.
//!
//! Provides a controlled text input with validation states, types, and accessibility.

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
    AriaAttributes,
};

/// Input type variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    /// Standard text input
    Text,
    /// Password input (masked)
    Password,
    /// Search input (with clear button)
    Search,
    /// Number input
    Number,
    /// Email input
    Email,
    /// URL input
    Url,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

impl InputType {
    /// Returns the HTML input type attribute value
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Search => "search",
            Self::Number => "number",
            Self::Email => "email",
            Self::Url => "url",
        }
    }
}

/// Event detail for change events
#[derive(Clone, PartialEq)]
pub struct InputChangeDetail {
    /// The new input value
    pub value: String,
}

/// Properties for the Input component
#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Input type
    #[prop_or_default]
    pub input_type: InputType,

    /// Current value (controlled component)
    #[prop_or_default]
    pub value: String,

    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Whether the input is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the input is read-only
    #[prop_or_default]
    pub read_only: bool,

    /// Whether the input has invalid content
    #[prop_or_default]
    pub invalid: bool,

    /// Whether to show a warning
    #[prop_or_default]
    pub warning: bool,

    /// HTML name attribute
    #[prop_or_default]
    pub name: Option<String>,

    /// Auto-focus on mount
    #[prop_or_default]
    pub auto_focus: bool,

    /// Autocomplete attribute (true = "on", false = "off")
    #[prop_or_default]
    pub autocomplete: Option<bool>,

    /// Spellcheck attribute
    #[prop_or_default]
    pub spellcheck: Option<bool>,

    /// Callback fired when value changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<InputChangeDetail>>>,

    /// Callback fired when input loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Callback fired when input gains focus
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

    /// Clear button ARIA label (for search type)
    #[prop_or_default]
    pub clear_aria_label: Option<String>,
}

/// Input component for text entry.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Input, InputType};
///
/// let on_change = Callback::from(|event: CustomEvent<InputChangeDetail>| {
///     console::log!("New value: {}", event.detail.value);
/// });
///
/// html! {
///     <Input
///         input_type={InputType::Text}
///         value={current_value}
///         placeholder="Enter text..."
///         on_change={on_change}
///     />
/// }
/// ```
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let _metadata = ComponentMetadata::new("Input");
    let input_ref = use_node_ref();

    // Handle input change
    let on_input = {
        let on_change = props.on_change.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                let value = target.value();

                if let Some(callback) = &on_change {
                    callback.emit(CustomEvent::new_non_cancelable(
                        InputChangeDetail { value }
                    ));
                }
            }
        })
    };

    // Handle blur
    let on_blur_event = {
        let on_blur = props.on_blur.clone();

        Callback::from(move |_e: FocusEvent| {
            if let Some(callback) = &on_blur {
                callback.emit(());
            }
        })
    };

    // Handle focus
    let on_focus_event = {
        let on_focus = props.on_focus.clone();

        Callback::from(move |_e: FocusEvent| {
            if let Some(callback) = &on_focus {
                callback.emit(());
            }
        })
    };

    // Handle clear button (for search type)
    let on_clear = {
        let on_change = props.on_change.clone();
        let input_ref = input_ref.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if let Some(callback) = &on_change {
                callback.emit(CustomEvent::new_non_cancelable(
                    InputChangeDetail { value: String::new() }
                ));
            }

            // Refocus the input after clearing
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        })
    };

    // Build CSS classes
    let input_classes = ClassBuilder::new()
        .add("awsui-input")
        .add(format!("awsui-input-type-{}", props.input_type.as_str()))
        .add_if(props.disabled, "awsui-input-disabled")
        .add_if(props.read_only, "awsui-input-readonly")
        .add_if(props.invalid, "awsui-input-invalid")
        .add_if(!props.invalid && props.warning, "awsui-input-warning");

    // Determine autocomplete attribute
    let autocomplete_attr = props.autocomplete.map(|ac| {
        if ac { "on" } else { "off" }
    });

    // Determine control ID (use provided or None to let browser auto-generate)
    let input_id = props.control_id.clone();

    // ARIA label
    let aria_label = props.aria.label.clone();

    // Build wrapper classes
    let wrapper_classes = ClassBuilder::new()
        .add("awsui-input-wrapper")
        .add_if(props.input_type == InputType::Search, "awsui-input-has-icon-left")
        .add_if(props.input_type == InputType::Search && !props.value.is_empty(), "awsui-input-has-clear-button");

    html! {
        <div class={wrapper_classes.build()}>
            // Left icon for search type
            if props.input_type == InputType::Search {
                <span class="awsui-input-icon-left">
                    <span class="awsui-icon awsui-icon-search" aria-hidden="true" />
                </span>
            }

            // Input element
            <input
                ref={input_ref}
                type={props.input_type.as_str()}
                class={input_classes.build()}
                id={input_id}
                name={props.name.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                readonly={props.read_only}
                autofocus={props.auto_focus}
                autocomplete={autocomplete_attr}
                spellcheck={props.spellcheck.map(|s| s.to_string())}
                aria-label={aria_label}
                aria-required={props.aria_required.to_string()}
                aria-invalid={props.invalid.to_string()}
                aria-labelledby={props.aria.labelledby.clone()}
                aria-describedby={props.aria.describedby.clone()}
                oninput={on_input}
                onblur={on_blur_event}
                onfocus={on_focus_event}
            />

            // Clear button for search type (when value exists)
            if props.input_type == InputType::Search && !props.value.is_empty() && !props.disabled && !props.read_only {
                <button
                    type="button"
                    class="awsui-input-clear-button"
                    aria-label={
                        props.clear_aria_label.clone()
                            .unwrap_or_else(|| "Clear".to_string())
                    }
                    onclick={on_clear}
                >
                    <span class="awsui-icon awsui-icon-close" aria-hidden="true" />
                </button>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_type_strings() {
        assert_eq!(InputType::Text.as_str(), "text");
        assert_eq!(InputType::Password.as_str(), "password");
        assert_eq!(InputType::Search.as_str(), "search");
        assert_eq!(InputType::Number.as_str(), "number");
        assert_eq!(InputType::Email.as_str(), "email");
        assert_eq!(InputType::Url.as_str(), "url");
    }

    #[test]
    fn input_type_default() {
        assert_eq!(InputType::default(), InputType::Text);
    }
}
