// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tiles component for radio-button-style selections.
//!
//! Provides a selectable tile grid for single selection with support for images,
//! descriptions, and column layout configuration.

use crate::internal::{
    AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// A single tile item in the grid
#[derive(Clone, PartialEq)]
pub struct TileItem {
    /// The value for this tile
    pub value: String,
    /// The label text
    pub label: String,
    /// Optional description content shown below the label
    pub description: Option<Html>,
    /// Optional image content shown at the top of the tile
    pub image: Option<Html>,
    /// Whether this tile is disabled
    pub disabled: bool,
    /// Optional control ID (usually auto-generated)
    pub control_id: Option<String>,
}

impl TileItem {
    /// Creates a new tile item with the given value and label
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::TileItem;
    ///
    /// let tile = TileItem::new("option1", "Option 1");
    /// ```
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            image: None,
            disabled: false,
            control_id: None,
        }
    }

    /// Sets the description for this tile
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::TileItem;
    /// use yew::prelude::*;
    ///
    /// let tile = TileItem::new("option1", "Option 1")
    ///     .with_description(html! { "This is a description" });
    /// ```
    pub fn with_description(mut self, description: Html) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the image for this tile
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::TileItem;
    /// use yew::prelude::*;
    ///
    /// let tile = TileItem::new("option1", "Option 1")
    ///     .with_image(html! { <img src="image.png" alt="Icon" /> });
    /// ```
    pub fn with_image(mut self, image: Html) -> Self {
        self.image = Some(image);
        self
    }

    /// Sets whether this tile is disabled
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::TileItem;
    ///
    /// let tile = TileItem::new("option1", "Option 1")
    ///     .with_disabled(true);
    /// ```
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the control ID for this tile
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::TileItem;
    ///
    /// let tile = TileItem::new("option1", "Option 1")
    ///     .with_control_id("custom-id");
    /// ```
    pub fn with_control_id(mut self, control_id: impl Into<String>) -> Self {
        self.control_id = Some(control_id.into());
        self
    }
}

/// Event detail for tiles change events
#[derive(Clone, PartialEq)]
pub struct TilesChangeDetail {
    /// The newly selected value
    pub value: String,
}

/// Properties for the Tiles component
#[derive(Properties, PartialEq, Clone)]
pub struct TilesProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// The currently selected value (controlled component)
    #[prop_or_default]
    pub value: Option<String>,

    /// The list of tile items to display
    #[prop_or_default]
    pub items: Vec<TileItem>,

    /// Callback fired when selection changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<TilesChangeDetail>>>,

    /// Number of columns (1-4, defaults to 1)
    #[prop_or(1)]
    pub columns: u32,

    /// Name attribute for the tiles group (auto-generated if not provided)
    #[prop_or_default]
    pub name: Option<String>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA required attribute
    #[prop_or_default]
    pub aria_required: bool,
}

/// Tiles component for radio-button-style selections.
///
/// Provides a selectable tile grid for single selection with support for images,
/// descriptions, and column layout configuration.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Tiles, TileItem, TilesChangeDetail};
/// use yew::prelude::*;
///
/// let items = vec![
///     TileItem::new("option1", "Option 1")
///         .with_description(html! { "First option" }),
///     TileItem::new("option2", "Option 2")
///         .with_image(html! { <img src="icon.png" alt="Icon" /> }),
///     TileItem::new("option3", "Option 3")
///         .with_disabled(true),
/// ];
///
/// let on_change = Callback::from(|event: CustomEvent<TilesChangeDetail>| {
///     console::log!("Selected: {}", event.detail.value);
/// });
///
/// html! {
///     <Tiles
///         items={items}
///         value={Some("option1".to_string())}
///         columns={2}
///         on_change={on_change}
///     />
/// }
/// ```
#[function_component(Tiles)]
pub fn tiles(props: &TilesProps) -> Html {
    let _metadata = ComponentMetadata::new("Tiles");

    // Generate a unique name if not provided
    let group_name = props.name.clone().unwrap_or_else(|| {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("awsui-tiles-{}", id)
    });

    // Clamp columns to 1-4 range
    let columns = props.columns.clamp(1, 4);

    // Build root CSS classes
    let root_classes = ClassBuilder::new()
        .add("awsui-tiles")
        .add(format!("awsui-tiles-columns-{}", columns));

    // Determine ARIA attributes
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
        >
            {
                props.items.iter().enumerate().map(|(index, item)| {
                    let is_checked = props.value.as_ref() == Some(&item.value);
                    let item_value = item.value.clone();
                    let on_change = props.on_change.clone();
                    let item_disabled = item.disabled;

                    // Build item CSS classes
                    let item_classes = ClassBuilder::new()
                        .add("awsui-tiles-item")
                        .add_if(is_checked, "awsui-tiles-item-selected")
                        .add_if(item.disabled, "awsui-tiles-item-disabled");

                    // Handle selection
                    let on_select = Callback::from(move |_e: Event| {
                        if !item_disabled
                            && let Some(callback) = &on_change {
                                callback.emit(CustomEvent::new_non_cancelable(
                                    TilesChangeDetail {
                                        value: item_value.clone(),
                                    }
                                ));
                            }
                    });

                    // Generate control ID
                    let control_id = item.control_id.clone().unwrap_or_else(|| {
                        format!("{}-{}", group_name, index)
                    });

                    html! {
                        <Tile
                            key={item.value.clone()}
                            class={item_classes.build()}
                            name={group_name.clone()}
                            value={item.value.clone()}
                            label={item.label.clone()}
                            checked={is_checked}
                            disabled={item.disabled}
                            control_id={control_id}
                            description={item.description.clone()}
                            image={item.image.clone()}
                            on_select={on_select}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

/// Internal Tile component
#[derive(Properties, PartialEq, Clone)]
struct TileProps {
    pub name: String,
    pub value: String,
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
    pub control_id: String,
    pub description: Option<Html>,
    pub image: Option<Html>,
    pub on_select: Callback<Event>,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Tile)]
fn tile(props: &TileProps) -> Html {
    let input_ref = use_node_ref();

    // Handle click on the entire tile
    let on_click = {
        let on_select = props.on_select.clone();
        let input_ref = input_ref.clone();
        let checked = props.checked;
        let disabled = props.disabled;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if !disabled && !checked {
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
        .add("awsui-tiles-item-control")
        .add_if(props.disabled, "awsui-tiles-item-control-disabled")
        .add_if(props.checked, "awsui-tiles-item-control-checked");

    let label_classes = ClassBuilder::new()
        .add("awsui-tiles-item-label")
        .add_if(props.disabled, "awsui-tiles-item-label-disabled");

    let description_classes = ClassBuilder::new()
        .add("awsui-tiles-item-description")
        .add_if(props.disabled, "awsui-tiles-item-description-disabled");

    html! {
        <div class={props.class.clone()} onclick={on_click}>
            <label class="awsui-tiles-item-wrapper" for={props.control_id.clone()}>
                <input
                    ref={input_ref}
                    type="radio"
                    id={props.control_id.clone()}
                    name={props.name.clone()}
                    value={props.value.clone()}
                    checked={props.checked}
                    disabled={props.disabled}
                    onchange={on_change}
                    class="awsui-tiles-item-native-input"
                />
                <div class="awsui-tiles-item-content">
                    if let Some(image) = &props.image {
                        <div class="awsui-tiles-item-image">
                            { image.clone() }
                        </div>
                    }
                    <div class="awsui-tiles-item-text">
                        <span class={control_classes.build()}>
                            // Radio control indicator
                            <svg viewBox="0 0 100 100" focusable="false" aria-hidden="true" class="awsui-tiles-item-control-svg">
                                // Outer circle (border)
                                <circle
                                    class="awsui-tiles-item-control-border"
                                    stroke-width="6.25"
                                    cx="50"
                                    cy="50"
                                    r="46"
                                />
                                // Inner circle (fill when checked)
                                <circle
                                    class={
                                        ClassBuilder::new()
                                            .add("awsui-tiles-item-control-fill")
                                            .add_if(props.checked, "awsui-tiles-item-control-fill-checked")
                                            .build()
                                    }
                                    stroke-width="30"
                                    cx="50"
                                    cy="50"
                                    r="35"
                                />
                            </svg>
                        </span>
                        <span class={label_classes.build()}>
                            { props.label.clone() }
                        </span>
                    </div>
                    if let Some(description) = &props.description {
                        <div class={description_classes.build()}>
                            { description.clone() }
                        </div>
                    }
                </div>
            </label>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_item_new() {
        let item = TileItem::new("value1", "Label 1");
        assert_eq!(item.value, "value1");
        assert_eq!(item.label, "Label 1");
        assert!(!item.disabled);
        assert!(item.description.is_none());
        assert!(item.image.is_none());
        assert!(item.control_id.is_none());
    }

    #[test]
    fn test_tile_item_with_description() {
        let item = TileItem::new("value1", "Label 1").with_description(html! { "Description" });
        assert!(item.description.is_some());
    }

    #[test]
    fn test_tile_item_with_image() {
        let item = TileItem::new("value1", "Label 1").with_image(html! { <img src="test.png" /> });
        assert!(item.image.is_some());
    }

    #[test]
    fn test_tile_item_with_disabled() {
        let item = TileItem::new("value1", "Label 1").with_disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_tile_item_with_control_id() {
        let item = TileItem::new("value1", "Label 1").with_control_id("custom-id");
        assert_eq!(item.control_id, Some("custom-id".to_string()));
    }

    #[test]
    fn test_tile_item_builder_pattern() {
        let item = TileItem::new("val", "Label")
            .with_description(html! { "Desc" })
            .with_image(html! { <img /> })
            .with_disabled(true)
            .with_control_id("id");

        assert_eq!(item.value, "val");
        assert_eq!(item.label, "Label");
        assert!(item.disabled);
        assert!(item.description.is_some());
        assert!(item.image.is_some());
        assert_eq!(item.control_id, Some("id".to_string()));
    }

    #[test]
    fn test_tiles_change_detail() {
        let detail = TilesChangeDetail {
            value: "test-value".to_string(),
        };
        assert_eq!(detail.value, "test-value");
    }

    #[test]
    fn test_tiles_columns_clamp() {
        // Test that columns are properly clamped to 1-4 range
        let props = TilesProps {
            base: Default::default(),
            value: None,
            items: vec![],
            on_change: None,
            columns: 5, // Should be clamped to 4
            name: None,
            aria: Default::default(),
            aria_required: false,
        };
        assert_eq!(props.columns.clamp(1, 4), 4);

        let props = TilesProps {
            base: Default::default(),
            value: None,
            items: vec![],
            on_change: None,
            columns: 0, // Should be clamped to 1
            name: None,
            aria: Default::default(),
            aria_required: false,
        };
        assert_eq!(props.columns.clamp(1, 4), 1);
    }

    #[test]
    fn test_tile_item_string_conversion() {
        let item = TileItem::new("value", "label");
        assert_eq!(item.value, "value");
        assert_eq!(item.label, "label");
    }

    #[test]
    fn test_tile_item_all_optional_fields() {
        let item = TileItem::new("val", "label")
            .with_description(html! { "desc" })
            .with_image(html! { <div>{"image"}</div> })
            .with_disabled(false)
            .with_control_id("custom");

        assert!(item.description.is_some());
        assert!(item.image.is_some());
        assert!(!item.disabled);
        assert_eq!(item.control_id, Some("custom".to_string()));
    }
}
