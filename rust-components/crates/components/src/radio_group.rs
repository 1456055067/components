// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! RadioGroup component for selecting one option from a list.
//!
//! Provides an accessible radio button group with support for disabled items,
//! descriptions, and form integration.

use crate::internal::{
    AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Direction for radio button layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioGroupDirection {
    /// Stack radio buttons vertically (default)
    Vertical,
    /// Arrange radio buttons horizontally
    Horizontal,
}

impl Default for RadioGroupDirection {
    fn default() -> Self {
        Self::Vertical
    }
}

/// A single radio button item in the group
#[derive(Clone, PartialEq)]
pub struct RadioGroupItem {
    /// The value for this radio button
    pub value: String,
    /// The label text or content
    pub label: Html,
    /// Optional description text shown below the label
    pub description: Option<Html>,
    /// Whether this radio button is disabled
    pub disabled: bool,
    /// Optional control ID (usually auto-generated)
    pub control_id: Option<String>,
}

impl RadioGroupItem {
    /// Creates a new radio button item with the given value and label
    pub fn new(value: impl Into<String>, label: impl Into<Html>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
            control_id: None,
        }
    }

    /// Sets the description for this item
    pub fn with_description(mut self, description: impl Into<Html>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether this item is disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the control ID for this item
    pub fn with_control_id(mut self, control_id: impl Into<String>) -> Self {
        self.control_id = Some(control_id.into());
        self
    }
}

/// Event detail for radio group change events
#[derive(Clone, PartialEq)]
pub struct RadioGroupChangeDetail {
    /// The newly selected value
    pub value: String,
}

/// Properties for the RadioGroup component
#[derive(Properties, PartialEq, Clone)]
pub struct RadioGroupProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// The currently selected value (controlled component)
    pub value: Option<String>,

    /// The list of radio button items to display
    #[prop_or_default]
    pub items: Vec<RadioGroupItem>,

    /// Callback fired when selection changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<RadioGroupChangeDetail>>>,

    /// Name attribute for the radio group (auto-generated if not provided)
    #[prop_or_default]
    pub name: Option<String>,

    /// Whether the entire group is read-only
    #[prop_or_default]
    pub read_only: bool,

    /// Layout direction for radio buttons
    #[prop_or_default]
    pub direction: RadioGroupDirection,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA required attribute
    #[prop_or_default]
    pub aria_required: bool,

    /// ARIA controls attribute
    #[prop_or_default]
    pub aria_controls: Option<String>,
}

/// RadioGroup component for selecting one option from a list.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{RadioGroup, RadioGroupItem, RadioGroupChangeDetail};
/// use yew::prelude::*;
///
/// let items = vec![
///     RadioGroupItem::new("option1", html! { "Option 1" }),
///     RadioGroupItem::new("option2", html! { "Option 2" })
///         .with_description(html! { "Additional info" }),
///     RadioGroupItem::new("option3", html! { "Option 3" })
///         .with_disabled(true),
/// ];
///
/// let on_change = Callback::from(|event: CustomEvent<RadioGroupChangeDetail>| {
///     console::log!("Selected: {}", event.detail.value);
/// });
///
/// html! {
///     <RadioGroup
///         value={Some("option1".to_string())}
///         items={items}
///         on_change={on_change}
///     />
/// }
/// ```
#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let _metadata = ComponentMetadata::new("RadioGroup");

    // Generate a unique name if not provided
    let group_name = props.name.clone().unwrap_or_else(|| {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("awsui-radio-group-{}", id)
    });

    // Build root CSS classes
    let root_classes = ClassBuilder::new()
        .add("awsui-radio-group")
        .add("awsui-radio-group-root")
        .add_if(
            props.direction == RadioGroupDirection::Horizontal,
            "awsui-radio-group-horizontal",
        );

    // Determine ARIA labelledby (form field context would set this in React)
    let aria_labelledby = props.aria.labelledby.clone();
    let aria_label = props.aria.label.clone();
    let aria_describedby = props.aria.describedby.clone();

    html! {
        <div
            id={props.base.id.clone()}
            class={root_classes.build()}
            role="radiogroup"
            aria-labelledby={aria_labelledby}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-required={props.aria_required.then_some("true")}
            aria-readonly={props.read_only.then_some("true")}
            aria-controls={props.aria_controls.clone()}
        >
            {
                props.items.iter().enumerate().map(|(index, item)| {
                    let is_checked = props.value.as_ref() == Some(&item.value);
                    let item_value = item.value.clone();
                    let on_change = props.on_change.clone();
                    let read_only = props.read_only;
                    let item_disabled = item.disabled;

                    // Build item CSS classes
                    let item_classes = ClassBuilder::new()
                        .add("awsui-radio-group-radio")
                        .add_if(item.description.is_some(), "awsui-radio-group-has-description")
                        .add_if(
                            props.direction == RadioGroupDirection::Horizontal,
                            "awsui-radio-group-horizontal-item"
                        )
                        .add_if(is_checked, "awsui-radio-group-selected");

                    // Handle selection
                    let on_select = Callback::from(move |_e: Event| {
                        if !read_only && !item_disabled {
                            if let Some(callback) = &on_change {
                                callback.emit(CustomEvent::new_non_cancelable(
                                    RadioGroupChangeDetail {
                                        value: item_value.clone(),
                                    }
                                ));
                            }
                        }
                    });

                    // Generate control ID
                    let control_id = item.control_id.clone().unwrap_or_else(|| {
                        format!("{}-{}", group_name, index)
                    });

                    html! {
                        <RadioButton
                            key={item.value.clone()}
                            class={item_classes.build()}
                            name={group_name.clone()}
                            value={item.value.clone()}
                            checked={is_checked}
                            disabled={item.disabled}
                            read_only={props.read_only}
                            control_id={control_id}
                            label={item.label.clone()}
                            description={item.description.clone()}
                            on_select={on_select}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

/// Internal RadioButton component (mimics React's RadioButton)
#[derive(Properties, PartialEq, Clone)]
struct RadioButtonProps {
    pub name: String,
    pub value: String,
    pub checked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub control_id: String,
    pub label: Html,
    pub description: Option<Html>,
    pub on_select: Callback<Event>,
    #[prop_or_default]
    pub class: String,
}

#[function_component(RadioButton)]
fn radio_button(props: &RadioButtonProps) -> Html {
    let input_ref = use_node_ref();

    // Handle click on the entire radio button (label + control)
    let on_click = {
        let on_select = props.on_select.clone();
        let input_ref = input_ref.clone();
        let checked = props.checked;
        let disabled = props.disabled;
        let read_only = props.read_only;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if !disabled && !read_only && !checked {
                // Focus the input
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }

                // Trigger selection
                on_select.emit(e.into());
            }
        })
    };

    // Handle input change (for keyboard interaction)
    let on_change = {
        let on_select = props.on_select.clone();
        let checked = props.checked;

        Callback::from(move |e: Event| {
            if !checked {
                on_select.emit(e);
            }
        })
    };

    // Build control classes
    let control_classes = ClassBuilder::new()
        .add("awsui-radio-control")
        .add_if(props.disabled, "awsui-radio-control-disabled")
        .add_if(props.read_only, "awsui-radio-control-readonly")
        .add_if(props.checked, "awsui-radio-control-checked");

    let label_classes = ClassBuilder::new()
        .add("awsui-radio-label")
        .add_if(props.disabled, "awsui-radio-label-disabled")
        .add_if(props.read_only, "awsui-radio-label-readonly");

    let description_classes = ClassBuilder::new()
        .add("awsui-radio-description")
        .add_if(props.disabled, "awsui-radio-description-disabled");

    html! {
        <div class={props.class.clone()} onclick={on_click}>
            <label class="awsui-radio-wrapper" for={props.control_id.clone()}>
                <span class="awsui-radio-control-wrapper">
                    <input
                        ref={input_ref}
                        type="radio"
                        id={props.control_id.clone()}
                        name={props.name.clone()}
                        value={props.value.clone()}
                        checked={props.checked}
                        disabled={props.disabled}
                        aria-disabled={props.read_only.then_some("true")}
                        onchange={on_change}
                        class="awsui-radio-native-input"
                    />
                    <span class={control_classes.build()}>
                        // SVG styled control
                        <svg viewBox="0 0 100 100" focusable="false" aria-hidden="true" class="awsui-radio-svg">
                            // Outer circle (border)
                            <circle
                                class="awsui-radio-circle-border"
                                stroke-width="6.25"
                                cx="50"
                                cy="50"
                                r="46"
                            />
                            // Inner circle (fill when checked)
                            <circle
                                class={
                                    ClassBuilder::new()
                                        .add("awsui-radio-circle-fill")
                                        .add_if(props.checked, "awsui-radio-circle-fill-checked")
                                        .build()
                                }
                                stroke-width="30"
                                cx="50"
                                cy="50"
                                r="35"
                            />
                        </svg>
                    </span>
                </span>
                <span class="awsui-radio-label-wrapper">
                    <span class={label_classes.build()}>
                        { props.label.clone() }
                    </span>
                    if let Some(description) = &props.description {
                        <span class={description_classes.build()}>
                            { description.clone() }
                        </span>
                    }
                </span>
            </label>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_group_direction_default() {
        assert_eq!(
            RadioGroupDirection::default(),
            RadioGroupDirection::Vertical
        );
    }

    #[test]
    fn test_radio_group_item_new() {
        let item = RadioGroupItem::new("value1", html! { "Label" });
        assert_eq!(item.value, "value1");
        assert!(!item.disabled);
        assert!(item.description.is_none());
        assert!(item.control_id.is_none());
    }

    #[test]
    fn test_radio_group_item_with_description() {
        let item = RadioGroupItem::new("value1", html! { "Label" })
            .with_description(html! { "Description" });
        assert!(item.description.is_some());
    }

    #[test]
    fn test_radio_group_item_with_disabled() {
        let item = RadioGroupItem::new("value1", html! { "Label" }).with_disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_radio_group_item_with_control_id() {
        let item = RadioGroupItem::new("value1", html! { "Label" }).with_control_id("custom-id");
        assert_eq!(item.control_id, Some("custom-id".to_string()));
    }

    #[test]
    fn test_radio_group_change_detail() {
        let detail = RadioGroupChangeDetail {
            value: "test-value".to_string(),
        };
        assert_eq!(detail.value, "test-value");
    }

    #[test]
    fn test_radio_group_item_builder_pattern() {
        let item = RadioGroupItem::new("val", html! { "Label" })
            .with_description(html! { "Desc" })
            .with_disabled(true)
            .with_control_id("id");

        assert_eq!(item.value, "val");
        assert!(item.disabled);
        assert!(item.description.is_some());
        assert_eq!(item.control_id, Some("id".to_string()));
    }
}
