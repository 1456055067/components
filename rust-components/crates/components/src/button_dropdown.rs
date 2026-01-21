// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! ButtonDropdown component
//!
//! A button component with a dropdown menu that appears when clicked.
//! Provides a list of actions or options that the user can select from.

use crate::button::ButtonVariant;
use crate::internal::{
    AnalyticsMetadata, AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata,
    CustomEvent,
};
use crate::spinner::{Spinner, SpinnerSize, SpinnerVariant};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use yew::prelude::*;

/// A single item in the button dropdown menu
#[derive(Clone, PartialEq, Debug)]
pub struct ButtonDropdownItem {
    /// Unique identifier for this item
    pub id: String,
    /// Display text for the item
    pub text: String,
    /// Whether this item is disabled and cannot be selected
    pub disabled: bool,
    /// Whether this item represents an external link
    pub external: bool,
    /// Optional URL for link-style items
    pub href: Option<String>,
    /// Optional icon to display before the text
    pub icon: Option<Html>,
}

impl ButtonDropdownItem {
    /// Creates a new button dropdown item
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::ButtonDropdownItem;
    ///
    /// let item = ButtonDropdownItem::new("action1", "Perform Action 1");
    /// ```
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            disabled: false,
            external: false,
            href: None,
            icon: None,
        }
    }

    /// Sets whether this item is disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether this item represents an external link
    pub fn with_external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    /// Sets the href for this item
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Sets an icon for this item
    pub fn with_icon(mut self, icon: Html) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Group of related items in the dropdown
#[derive(Clone, PartialEq, Debug)]
pub struct ButtonDropdownItemGroup {
    /// Optional header text for the group
    pub text: Option<String>,
    /// Items in this group
    pub items: Vec<ButtonDropdownItem>,
}

impl ButtonDropdownItemGroup {
    /// Creates a new item group
    pub fn new() -> Self {
        Self {
            text: None,
            items: Vec::new(),
        }
    }

    /// Sets the header text for this group
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the items in this group
    pub fn with_items(mut self, items: Vec<ButtonDropdownItem>) -> Self {
        self.items = items;
        self
    }

    /// Adds an item to this group
    pub fn add_item(mut self, item: ButtonDropdownItem) -> Self {
        self.items.push(item);
        self
    }
}

impl Default for ButtonDropdownItemGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// Event detail for item click events
#[derive(Clone, PartialEq, Debug)]
pub struct ButtonDropdownItemClickDetail {
    /// ID of the clicked item
    pub id: String,
    /// Whether this is an external link
    pub external: bool,
    /// Optional href if this is a link
    pub href: Option<String>,
}

/// Properties for the ButtonDropdown component
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonDropdownProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Items to display in the dropdown
    /// Use this for a flat list of items
    #[prop_or_default]
    pub items: Vec<ButtonDropdownItem>,

    /// Item groups to display in the dropdown
    /// Use this for organized groups with headers
    #[prop_or_default]
    pub item_groups: Vec<ButtonDropdownItemGroup>,

    /// Button variant
    #[prop_or_default]
    pub variant: ButtonVariant,

    /// Whether the button is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the button is in a loading state
    #[prop_or_default]
    pub loading: bool,

    /// Whether the button should expand to full width
    #[prop_or_default]
    pub expanded: bool,

    /// Icon to display in the button
    #[prop_or_default]
    pub icon: Option<Html>,

    /// Main action handler (fires when main button is clicked if split)
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    /// Item click event handler
    #[prop_or_default]
    pub on_item_click: Option<Callback<CustomEvent<ButtonDropdownItemClickDetail>>>,

    /// ARIA label for the button
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Text or content for the button
    #[prop_or_default]
    pub children: Children,
}

/// ButtonDropdown component
///
/// A button with a dropdown menu that provides a list of actions or options.
/// The dropdown can contain flat items or organized groups with headers.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{ButtonDropdown, ButtonDropdownItem, ButtonVariant, CustomEvent, ButtonDropdownItemClickDetail};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let items = vec![
///         ButtonDropdownItem::new("action1", "Action 1"),
///         ButtonDropdownItem::new("action2", "Action 2").with_disabled(true),
///         ButtonDropdownItem::new("action3", "Action 3")
///             .with_href("https://example.com")
///             .with_external(true),
///     ];
///
///     let on_item_click = Callback::from(|event: CustomEvent<ButtonDropdownItemClickDetail>| {
///         web_sys::console::log_1(&format!("Item clicked: {}", event.detail.id).into());
///     });
///
///     html! {
///         <ButtonDropdown
///             items={items}
///             variant={ButtonVariant::Primary}
///             on_item_click={on_item_click}
///         >
///             {"Actions"}
///         </ButtonDropdown>
///     }
/// }
/// ```
///
/// # Example with groups
///
/// ```rust
/// use cloudscape_components::{ButtonDropdown, ButtonDropdownItem, ButtonDropdownItemGroup, ButtonVariant};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let item_groups = vec![
///         ButtonDropdownItemGroup::new()
///             .with_text("General Actions")
///             .with_items(vec![
///                 ButtonDropdownItem::new("edit", "Edit"),
///                 ButtonDropdownItem::new("copy", "Copy"),
///             ]),
///         ButtonDropdownItemGroup::new()
///             .with_text("Danger Zone")
///             .with_items(vec![
///                 ButtonDropdownItem::new("delete", "Delete"),
///             ]),
///     ];
///
///     html! {
///         <ButtonDropdown
///             item_groups={item_groups}
///             variant={ButtonVariant::Normal}
///         >
///             {"More Actions"}
///         </ButtonDropdown>
///     }
/// }
/// ```
#[function_component(ButtonDropdown)]
pub fn button_dropdown(props: &ButtonDropdownProps) -> Html {
    let _metadata = ComponentMetadata::new("ButtonDropdown");
    let is_open = use_state(|| false);
    let dropdown_ref = use_node_ref();

    // Determine if button is interactive
    let is_disabled = props.disabled || props.loading;

    // Toggle dropdown open/closed
    let on_toggle = {
        let is_open = is_open.clone();
        let disabled = is_disabled;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            if !disabled {
                is_open.set(!*is_open);
            }
        })
    };

    // Handle item click
    let on_item_click_handler = {
        let on_item_click = props.on_item_click.clone();
        let is_open = is_open.clone();

        Callback::from(move |item: ButtonDropdownItem| {
            is_open.set(false);

            if !item.disabled {
                if let Some(callback) = &on_item_click {
                    let detail = ButtonDropdownItemClickDetail {
                        id: item.id.clone(),
                        external: item.external,
                        href: item.href.clone(),
                    };
                    callback.emit(CustomEvent::new_non_cancelable(detail));
                }

                // If item has href, navigate to it
                if let Some(href) = &item.href
                    && let Some(window) = web_sys::window()
                {
                    if item.external {
                        let _ = window.open_with_url_and_target(href, "_blank");
                    } else {
                        let _ = window.location().set_href(href);
                    }
                }
            }
        })
    };

    // Close dropdown when clicking outside
    {
        let is_open = is_open.clone();
        let dropdown_ref = dropdown_ref.clone();

        use_effect_with(*is_open, move |open| {
            if *open {
                let is_open = is_open.clone();
                let dropdown_ref = dropdown_ref.clone();

                let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                    if let Some(target) = e.target()
                        && let Some(element) = dropdown_ref.cast::<web_sys::HtmlElement>()
                        && let Some(target_element) = target.dyn_ref::<web_sys::Node>()
                        && !element.contains(Some(target_element))
                    {
                        is_open.set(false);
                    }
                }) as Box<dyn FnMut(_)>);

                if let Some(window) = web_sys::window()
                    && let Some(document) = window.document()
                {
                    let _ = document.add_event_listener_with_callback(
                        "click",
                        closure.as_ref().unchecked_ref(),
                    );
                }

                // Keep closure alive
                closure.forget();
            }
            || ()
        });
    }

    // Build button classes
    let button_classes = ClassBuilder::new()
        .add("awsui-button-dropdown-trigger")
        .add(format!(
            "awsui-button-dropdown-trigger-variant-{}",
            props.variant.as_str()
        ))
        .add_if(is_disabled, "awsui-button-dropdown-trigger-disabled")
        .add_if(props.loading, "awsui-button-dropdown-trigger-loading")
        .add_if(props.expanded, "awsui-button-dropdown-trigger-expanded")
        .add_if(*is_open, "awsui-button-dropdown-trigger-open");

    // Build dropdown classes
    let dropdown_classes = ClassBuilder::new()
        .add("awsui-button-dropdown-content")
        .add_if(*is_open, "awsui-button-dropdown-content-open");

    // Create analytics metadata
    let analytics =
        AnalyticsMetadata::button("button-dropdown", props.variant.as_str(), is_disabled);
    let analytics_attr = analytics.to_data_attribute();

    // Build ARIA attributes
    let mut aria = AriaAttributes::default();
    if let Some(ref label) = props.aria_label {
        aria.label = Some(label.clone());
    }
    aria.expanded = Some(*is_open);
    if is_disabled {
        aria.disabled = Some(true);
    }

    // Render all items (either from items or item_groups)
    let render_items = {
        let on_item_click = on_item_click_handler.clone();

        move |items: &[ButtonDropdownItem]| {
            items.iter().map(|item| {
                let item_clone = item.clone();
                let on_click = {
                    let item = item.clone();
                    let on_item_click = on_item_click.clone();
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        e.stop_propagation();
                        on_item_click.emit(item.clone());
                    })
                };

                let item_classes = ClassBuilder::new()
                    .add("awsui-button-dropdown-item")
                    .add_if(item.disabled, "awsui-button-dropdown-item-disabled")
                    .add_if(item.external, "awsui-button-dropdown-item-external");

                html! {
                    <li
                        key={item_clone.id.clone()}
                        class={item_classes.build()}
                        role="menuitem"
                        aria-disabled={item.disabled.to_string()}
                    >
                        <button
                            type="button"
                            class="awsui-button-dropdown-item-button"
                            disabled={item.disabled}
                            onclick={on_click}
                        >
                            if let Some(ref icon) = item.icon {
                                <span class="awsui-button-dropdown-item-icon">
                                    { icon.clone() }
                                </span>
                            }
                            <span class="awsui-button-dropdown-item-text">
                                { &item.text }
                            </span>
                            if item.external && item.href.is_some() {
                                <span class="awsui-button-dropdown-item-external-icon" aria-label="(opens in a new tab)">
                                    {"↗"}
                                </span>
                            }
                        </button>
                    </li>
                }
            }).collect::<Html>()
        }
    };

    // Build dropdown content
    let dropdown_content = if !props.item_groups.is_empty() {
        // Render grouped items
        html! {
            <>
                {
                    props.item_groups.iter().enumerate().map(|(index, group)| {
                        html! {
                            <div key={index} class="awsui-button-dropdown-group">
                                if let Some(ref text) = group.text {
                                    <div class="awsui-button-dropdown-group-header">
                                        { text }
                                    </div>
                                }
                                <ul class="awsui-button-dropdown-items" role="menu">
                                    { render_items(&group.items) }
                                </ul>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </>
        }
    } else {
        // Render flat items
        html! {
            <ul class="awsui-button-dropdown-items" role="menu">
                { render_items(&props.items) }
            </ul>
        }
    };

    html! {
        <div
            ref={dropdown_ref}
            class={props.base.merge_classes("awsui-button-dropdown")}
            id={props.base.id.clone()}
            data-analytics-metadata={analytics_attr}
        >
            // Trigger button
            <button
                type="button"
                class={button_classes.build()}
                disabled={is_disabled}
                onclick={on_toggle}
                aria-expanded={is_open.to_string()}
                aria-haspopup="menu"
                aria-label={aria.label.clone()}
            >
                if props.loading {
                    <span class="awsui-button-dropdown-spinner">
                        <Spinner
                            size={SpinnerSize::Normal}
                            variant={SpinnerVariant::Normal}
                        />
                    </span>
                }
                if let Some(ref icon) = props.icon {
                    if !props.loading {
                        <span class="awsui-button-dropdown-icon">
                            { icon.clone() }
                        </span>
                    }
                }
                <span class="awsui-button-dropdown-text">
                    { props.children.clone() }
                </span>
                <span class="awsui-button-dropdown-chevron" aria-hidden="true">
                    { if *is_open { "▲" } else { "▼" } }
                </span>
            </button>

            // Dropdown menu
            if *is_open {
                <div class={dropdown_classes.build()}>
                    { dropdown_content }
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_dropdown_item_new() {
        let item = ButtonDropdownItem::new("action1", "Action 1");
        assert_eq!(item.id, "action1");
        assert_eq!(item.text, "Action 1");
        assert!(!item.disabled);
        assert!(!item.external);
        assert_eq!(item.href, None);
    }

    #[test]
    fn test_button_dropdown_item_builder() {
        let item = ButtonDropdownItem::new("download", "Download")
            .with_disabled(true)
            .with_external(true)
            .with_href("https://example.com/file.pdf");

        assert_eq!(item.id, "download");
        assert_eq!(item.text, "Download");
        assert!(item.disabled);
        assert!(item.external);
        assert_eq!(item.href, Some("https://example.com/file.pdf".to_string()));
    }

    #[test]
    fn test_button_dropdown_item_with_icon() {
        let icon = html! { <span>{"🔒"}</span> };
        let item = ButtonDropdownItem::new("lock", "Lock").with_icon(icon);

        assert_eq!(item.id, "lock");
        assert!(item.icon.is_some());
    }

    #[test]
    fn test_button_dropdown_item_group_new() {
        let group = ButtonDropdownItemGroup::new();
        assert_eq!(group.text, None);
        assert!(group.items.is_empty());
    }

    #[test]
    fn test_button_dropdown_item_group_builder() {
        let items = vec![
            ButtonDropdownItem::new("action1", "Action 1"),
            ButtonDropdownItem::new("action2", "Action 2"),
        ];

        let group = ButtonDropdownItemGroup::new()
            .with_text("Group Header")
            .with_items(items.clone());

        assert_eq!(group.text, Some("Group Header".to_string()));
        assert_eq!(group.items.len(), 2);
        assert_eq!(group.items[0].id, "action1");
    }

    #[test]
    fn test_button_dropdown_item_group_add_item() {
        let group = ButtonDropdownItemGroup::new()
            .add_item(ButtonDropdownItem::new("a1", "Action 1"))
            .add_item(ButtonDropdownItem::new("a2", "Action 2"));

        assert_eq!(group.items.len(), 2);
        assert_eq!(group.items[0].id, "a1");
        assert_eq!(group.items[1].id, "a2");
    }

    #[test]
    fn test_button_dropdown_item_click_detail() {
        let detail = ButtonDropdownItemClickDetail {
            id: "test-action".to_string(),
            external: true,
            href: Some("https://example.com".to_string()),
        };

        assert_eq!(detail.id, "test-action");
        assert!(detail.external);
        assert_eq!(detail.href, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_button_dropdown_item_equality() {
        let item1 = ButtonDropdownItem::new("action", "Action").with_disabled(true);
        let item2 = ButtonDropdownItem::new("action", "Action").with_disabled(true);
        let item3 = ButtonDropdownItem::new("different", "Action").with_disabled(true);

        assert_eq!(item1, item2);
        assert_ne!(item1, item3);
    }

    #[test]
    fn test_button_dropdown_item_click_detail_equality() {
        let detail1 = ButtonDropdownItemClickDetail {
            id: "id1".to_string(),
            external: false,
            href: None,
        };
        let detail2 = ButtonDropdownItemClickDetail {
            id: "id1".to_string(),
            external: false,
            href: None,
        };
        let detail3 = ButtonDropdownItemClickDetail {
            id: "id2".to_string(),
            external: false,
            href: None,
        };

        assert_eq!(detail1, detail2);
        assert_ne!(detail1, detail3);
    }

    #[test]
    fn test_button_variant_integration() {
        // Test that ButtonVariant can be used with ButtonDropdown
        let variants = vec![
            ButtonVariant::Normal,
            ButtonVariant::Primary,
            ButtonVariant::Icon,
        ];

        for variant in variants {
            assert!(!variant.as_str().is_empty());
        }
    }
}
