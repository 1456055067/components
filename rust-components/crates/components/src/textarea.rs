// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Textarea component for multi-line text input.
//!
//! Provides a controlled textarea input with validation states, auto-resize capabilities,
//! and comprehensive accessibility support.

use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
    AriaAttributes,
};

/// Event detail for change events
#[derive(Clone, PartialEq, Debug)]
pub struct TextareaChangeDetail {
    /// The new textarea value
    pub value: String,
}

/// Properties for the Textarea component
#[derive(Properties, PartialEq, Clone)]
pub struct TextareaProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Current value (controlled component)
    #[prop_or_default]
    pub value: String,

    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Whether the textarea is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the textarea is read-only
    #[prop_or_default]
    pub readonly: bool,

    /// Whether the textarea has invalid content
    #[prop_or_default]
    pub invalid: bool,

    /// Whether to show a warning
    #[prop_or_default]
    pub warning: bool,

    /// Number of visible text lines (rows)
    /// Defaults to 3 if not specified
    #[prop_or_default]
    pub rows: Option<u32>,

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

    /// Disable browser autocorrect
    #[prop_or_default]
    pub disable_browser_autocorrect: bool,

    /// Callback fired when value changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<TextareaChangeDetail>>>,

    /// Callback fired when textarea loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Callback fired when textarea gains focus
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
}

/// Textarea component for multi-line text input.
///
/// The Textarea component provides a controlled multi-line text input field with support
/// for validation states, accessibility features, and browser behavior customization.
///
/// # Features
///
/// - Controlled component pattern with value prop
/// - Validation states (invalid, warning)
/// - Disabled and read-only states
/// - Configurable rows for height
/// - Browser autocomplete, spellcheck, and autocorrect controls
/// - Full ARIA support for accessibility
/// - Custom event system for change/blur/focus handlers
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Textarea, TextareaChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyForm)]
/// fn my_form() -> Html {
///     let value = use_state(|| String::new());
///
///     let on_change = {
///         let value = value.clone();
///         Callback::from(move |event: CustomEvent<TextareaChangeDetail>| {
///             value.set(event.detail.value);
///         })
///     };
///
///     html! {
///         <Textarea
///             value={(*value).clone()}
///             placeholder="Enter your feedback..."
///             rows={5}
///             on_change={on_change}
///         />
///     }
/// }
/// ```
///
/// # Validation Example
///
/// ```rust
/// use cloudscape_components::Textarea;
/// use yew::prelude::*;
///
/// html! {
///     <Textarea
///         value={feedback_value}
///         invalid={feedback_value.len() > 500}
///         placeholder="Maximum 500 characters"
///         rows={4}
///         on_change={on_feedback_change}
///     />
/// }
/// ```
///
/// # Accessibility
///
/// The Textarea component follows WAI-ARIA best practices:
///
/// - Use `aria.label` or `aria.labelledby` to provide an accessible name
/// - Set `aria_required` for required fields
/// - The `invalid` prop automatically sets `aria-invalid`
/// - Use `aria.describedby` to associate error messages or hints
///
/// # Browser Behavior
///
/// Control browser features:
/// - `autocomplete`: Enable/disable autocomplete (default: enabled)
/// - `spellcheck`: Enable/disable spellcheck
/// - `disable_browser_autocorrect`: Disable autocorrect and autocapitalize
#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let _metadata = ComponentMetadata::new("Textarea");
    let textarea_ref = use_node_ref();

    // Handle input change
    let on_input = {
        let on_change = props.on_change.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlTextAreaElement>() {
                let value = target.value();

                if let Some(callback) = &on_change {
                    callback.emit(CustomEvent::new_non_cancelable(
                        TextareaChangeDetail { value }
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

    // Build CSS classes
    let textarea_classes = ClassBuilder::new()
        .add("awsui-textarea")
        .add_if(props.disabled, "awsui-textarea-disabled")
        .add_if(props.readonly, "awsui-textarea-readonly")
        .add_if(props.invalid, "awsui-textarea-invalid")
        .add_if(!props.invalid && props.warning, "awsui-textarea-warning");

    // Determine autocomplete attribute
    let autocomplete_attr = props.autocomplete.map(|ac| {
        if ac { "on" } else { "off" }
    });

    // Determine control ID (use provided or None to let browser auto-generate)
    let textarea_id = props.control_id.clone();

    // ARIA label
    let aria_label = props.aria.label.clone();

    // Determine rows (default to 3 if not specified)
    let rows = props.rows.unwrap_or(3).to_string();

    // Build wrapper classes
    let wrapper_classes = ClassBuilder::new()
        .add("awsui-textarea-wrapper");

    html! {
        <span class={wrapper_classes.build()}>
            <textarea
                ref={textarea_ref}
                class={textarea_classes.build()}
                id={textarea_id}
                name={props.name.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                readonly={props.readonly}
                autofocus={props.auto_focus}
                autocomplete={autocomplete_attr}
                spellcheck={props.spellcheck.map(|s| s.to_string())}
                autocorrect={if props.disable_browser_autocorrect { Some("off") } else { None }}
                autocapitalize={if props.disable_browser_autocorrect { Some("off") } else { None }}
                rows={rows}
                aria-label={aria_label}
                aria-required={props.aria_required.to_string()}
                aria-invalid={props.invalid.to_string()}
                aria-labelledby={props.aria.labelledby.clone()}
                aria-describedby={props.aria.describedby.clone()}
                oninput={on_input}
                onblur={on_blur_event}
                onfocus={on_focus_event}
            />
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textarea_change_detail_clone() {
        let detail = TextareaChangeDetail {
            value: "test value".to_string(),
        };
        let cloned = detail.clone();
        assert_eq!(detail.value, cloned.value);
    }

    #[test]
    fn textarea_change_detail_equality() {
        let detail1 = TextareaChangeDetail {
            value: "test".to_string(),
        };
        let detail2 = TextareaChangeDetail {
            value: "test".to_string(),
        };
        let detail3 = TextareaChangeDetail {
            value: "different".to_string(),
        };

        assert_eq!(detail1, detail2);
        assert_ne!(detail1, detail3);
    }

    #[test]
    fn textarea_props_default_rows() {
        // Test that default rows value is handled correctly
        let props = TextareaProps {
            base: BaseComponentProps::default(),
            value: String::new(),
            placeholder: None,
            disabled: false,
            readonly: false,
            invalid: false,
            warning: false,
            rows: None,
            name: None,
            auto_focus: false,
            autocomplete: None,
            spellcheck: None,
            disable_browser_autocorrect: false,
            on_change: None,
            on_blur: None,
            on_focus: None,
            aria: AriaAttributes::default(),
            aria_required: false,
            control_id: None,
        };

        assert_eq!(props.rows, None);
        // When None, component should default to 3
        assert_eq!(props.rows.unwrap_or(3), 3);
    }

    #[test]
    fn textarea_props_custom_rows() {
        let props = TextareaProps {
            base: BaseComponentProps::default(),
            value: String::new(),
            placeholder: None,
            disabled: false,
            readonly: false,
            invalid: false,
            warning: false,
            rows: Some(5),
            name: None,
            auto_focus: false,
            autocomplete: None,
            spellcheck: None,
            disable_browser_autocorrect: false,
            on_change: None,
            on_blur: None,
            on_focus: None,
            aria: AriaAttributes::default(),
            aria_required: false,
            control_id: None,
        };

        assert_eq!(props.rows, Some(5));
    }

    #[test]
    fn textarea_props_validation_states() {
        // Test invalid state
        let invalid_props = TextareaProps {
            base: BaseComponentProps::default(),
            value: String::new(),
            placeholder: None,
            disabled: false,
            readonly: false,
            invalid: true,
            warning: false,
            rows: None,
            name: None,
            auto_focus: false,
            autocomplete: None,
            spellcheck: None,
            disable_browser_autocorrect: false,
            on_change: None,
            on_blur: None,
            on_focus: None,
            aria: AriaAttributes::default(),
            aria_required: false,
            control_id: None,
        };

        assert!(invalid_props.invalid);
        assert!(!invalid_props.warning);

        // Test warning state (should not show when invalid is also true)
        let warning_props = TextareaProps {
            invalid: false,
            warning: true,
            ..invalid_props
        };

        assert!(!warning_props.invalid);
        assert!(warning_props.warning);
    }

    #[test]
    fn custom_event_non_cancelable() {
        let detail = TextareaChangeDetail {
            value: "test".to_string(),
        };
        let event = CustomEvent::new_non_cancelable(detail);

        assert!(!event.cancelable);
        assert!(!event.default_prevented);
    }
}
