// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Flashbar component for displaying multiple flash messages.
//!
//! The Flashbar is a notification container that displays multiple flash messages,
//! typically used for showing success, error, warning, or informational messages to users.

use crate::internal::{BaseComponentProps, ClassBuilder, CustomEvent};
use crate::spinner::{Spinner, SpinnerSize};
use yew::prelude::*;

/// Flash message type variants
///
/// Determines the visual styling and icon for a flash item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlashbarType {
    /// Informational message (default)
    #[default]
    Info,
    /// Success confirmation message
    Success,
    /// Warning or caution message
    Warning,
    /// Error or critical message
    Error,
}

impl FlashbarType {
    /// Returns the CSS class name for this flash type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Returns the icon name for this flash type
    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Info => "status-info",
            Self::Success => "status-positive",
            Self::Warning => "status-warning",
            Self::Error => "status-negative",
        }
    }

    /// Returns the default ARIA label for this flash type
    pub fn default_aria_label(&self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }
}

/// Individual flash message item
///
/// Represents a single notification message in the flashbar.
#[derive(Clone, PartialEq)]
pub struct FlashbarItem {
    /// Unique identifier for the item
    pub id: Option<String>,
    /// Type of flash message
    pub flash_type: FlashbarType,
    /// Optional header text
    pub header: Option<String>,
    /// Message content
    pub content: Html,
    /// Whether the item can be dismissed
    pub dismissible: bool,
    /// Whether to show a loading spinner
    pub loading: bool,
    /// Optional action button content
    pub action: Option<Html>,
    /// Action button text
    pub button_text: Option<String>,
    /// Callback for action button clicks
    pub on_button_click: Option<Callback<MouseEvent>>,
}

impl FlashbarItem {
    /// Creates a new flash item with the specified type and content
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::{FlashbarItem, FlashbarType};
    /// use yew::prelude::*;
    ///
    /// let item = FlashbarItem::new(
    ///     FlashbarType::Success,
    ///     html! { "Operation completed successfully" }
    /// );
    /// ```
    pub fn new(flash_type: FlashbarType, content: Html) -> Self {
        Self {
            id: None,
            flash_type,
            header: None,
            content,
            dismissible: false,
            loading: false,
            action: None,
            button_text: None,
            on_button_click: None,
        }
    }

    /// Sets the unique identifier for this item
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the header text for this item
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Sets whether this item is dismissible
    pub fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Sets whether this item shows a loading spinner
    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets the action button content
    pub fn with_action(mut self, action: Html) -> Self {
        self.action = Some(action);
        self
    }

    /// Sets the action button text
    pub fn with_button_text(mut self, text: impl Into<String>) -> Self {
        self.button_text = Some(text.into());
        self
    }

    /// Sets the action button click handler
    pub fn with_on_button_click(mut self, callback: Callback<MouseEvent>) -> Self {
        self.on_button_click = Some(callback);
        self
    }
}

/// Event detail for item dismiss events
#[derive(Clone, PartialEq, Debug)]
pub struct FlashbarDismissDetail {
    /// The ID of the dismissed item (if provided)
    pub item_id: Option<String>,
}

/// Properties for the Flashbar component
#[derive(Properties, PartialEq, Clone)]
pub struct FlashbarProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Array of flash items to display
    #[prop_or_default]
    pub items: Vec<FlashbarItem>,

    /// Whether to stack items in a compact vertical list
    #[prop_or_default]
    pub stack_items: bool,

    /// Callback fired when an item is dismissed
    #[prop_or_default]
    pub on_item_dismiss: Option<Callback<CustomEvent<FlashbarDismissDetail>>>,
}

/// Flashbar component for displaying multiple flash messages.
///
/// A notification container that displays multiple flash messages with different
/// severity levels. Each message can be dismissed individually and supports
/// loading states, action buttons, and custom content.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Flashbar, FlashbarItem, FlashbarType};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let items = vec![
///         FlashbarItem::new(FlashbarType::Success, html! { "Operation completed" })
///             .with_header("Success")
///             .with_dismissible(true),
///         FlashbarItem::new(FlashbarType::Error, html! { "An error occurred" })
///             .with_header("Error")
///             .with_dismissible(true),
///     ];
///
///     html! {
///         <Flashbar items={items} />
///     }
/// }
/// ```
#[function_component(Flashbar)]
pub fn flashbar(props: &FlashbarProps) -> Html {
    // Build root CSS classes
    let classes = ClassBuilder::new()
        .add("awsui-flashbar")
        .add_if(props.stack_items, "awsui-flashbar-stacked");

    let root_class = props.base.merge_classes(&classes.build());

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
            role="region"
            aria-live="polite"
            aria-label="Notifications"
        >
            {
                props.items.iter().enumerate().map(|(index, item)| {
                    render_flash_item(item, index, &props.on_item_dismiss)
                }).collect::<Html>()
            }
        </div>
    }
}

/// Renders a single flash item
fn render_flash_item(
    item: &FlashbarItem,
    index: usize,
    on_dismiss_callback: &Option<Callback<CustomEvent<FlashbarDismissDetail>>>,
) -> Html {
    // Build item CSS classes
    let item_classes = ClassBuilder::new()
        .add("awsui-flashbar-item")
        .add(format!(
            "awsui-flashbar-item-type-{}",
            item.flash_type.as_str()
        ))
        .add_if(item.dismissible, "awsui-flashbar-item-dismissible")
        .add_if(item.loading, "awsui-flashbar-item-loading")
        .build();

    // Handle dismiss button click
    let on_dismiss_click = {
        let item_id = item.id.clone();
        let on_dismiss = on_dismiss_callback.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if let Some(callback) = &on_dismiss {
                callback.emit(CustomEvent::new_non_cancelable(FlashbarDismissDetail {
                    item_id: item_id.clone(),
                }));
            }
        })
    };

    // ARIA label for the item
    let aria_label = item.flash_type.default_aria_label();

    html! {
        <div
            key={item.id.clone().unwrap_or_else(|| index.to_string())}
            class={item_classes}
            role="alert"
            aria-label={aria_label}
        >
            // Icon or loading spinner
            <div class="awsui-flashbar-item-icon">
                if item.loading {
                    <Spinner size={SpinnerSize::Normal} />
                } else {
                    <span
                        class={format!("awsui-icon awsui-icon-{}", item.flash_type.icon_name())}
                        aria-hidden="true"
                    />
                }
            </div>

            // Content wrapper
            <div class="awsui-flashbar-item-content-wrapper">
                // Optional header
                if let Some(ref header) = item.header {
                    <div class="awsui-flashbar-item-header">
                        { header }
                    </div>
                }

                // Message content
                <div class="awsui-flashbar-item-content">
                    { item.content.clone() }
                </div>

                // Optional action button
                if let Some(ref action) = item.action {
                    <div class="awsui-flashbar-item-action">
                        { action.clone() }
                    </div>
                } else if let Some(ref button_text) = item.button_text {
                    <div class="awsui-flashbar-item-action">
                        <button
                            type="button"
                            class="awsui-button awsui-button-variant-link"
                            onclick={item.on_button_click.clone()}
                        >
                            { button_text }
                        </button>
                    </div>
                }
            </div>

            // Optional dismiss button
            if item.dismissible {
                <div class="awsui-flashbar-item-dismiss">
                    <button
                        type="button"
                        class="awsui-flashbar-item-dismiss-button awsui-button awsui-button-variant-icon"
                        aria-label="Dismiss notification"
                        onclick={on_dismiss_click}
                    >
                        <span class="awsui-icon awsui-icon-close" aria-hidden="true" />
                    </button>
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flashbar_type_strings() {
        assert_eq!(FlashbarType::Info.as_str(), "info");
        assert_eq!(FlashbarType::Success.as_str(), "success");
        assert_eq!(FlashbarType::Warning.as_str(), "warning");
        assert_eq!(FlashbarType::Error.as_str(), "error");
    }

    #[test]
    fn test_flashbar_type_icons() {
        assert_eq!(FlashbarType::Info.icon_name(), "status-info");
        assert_eq!(FlashbarType::Success.icon_name(), "status-positive");
        assert_eq!(FlashbarType::Warning.icon_name(), "status-warning");
        assert_eq!(FlashbarType::Error.icon_name(), "status-negative");
    }

    #[test]
    fn test_flashbar_type_aria_labels() {
        assert_eq!(FlashbarType::Info.default_aria_label(), "Info");
        assert_eq!(FlashbarType::Success.default_aria_label(), "Success");
        assert_eq!(FlashbarType::Warning.default_aria_label(), "Warning");
        assert_eq!(FlashbarType::Error.default_aria_label(), "Error");
    }

    #[test]
    fn test_flashbar_type_default() {
        assert_eq!(FlashbarType::default(), FlashbarType::Info);
    }

    #[test]
    fn test_flashbar_item_builder_basic() {
        let item = FlashbarItem::new(FlashbarType::Success, html! { "Test" });
        assert_eq!(item.flash_type, FlashbarType::Success);
        assert_eq!(item.dismissible, false);
        assert_eq!(item.loading, false);
        assert!(item.id.is_none());
        assert!(item.header.is_none());
    }

    #[test]
    fn test_flashbar_item_builder_with_id() {
        let item = FlashbarItem::new(FlashbarType::Info, html! { "Test" }).with_id("test-id");
        assert_eq!(item.id, Some("test-id".to_string()));
    }

    #[test]
    fn test_flashbar_item_builder_with_header() {
        let item = FlashbarItem::new(FlashbarType::Warning, html! { "Test" })
            .with_header("Warning Header");
        assert_eq!(item.header, Some("Warning Header".to_string()));
    }

    #[test]
    fn test_flashbar_item_builder_with_dismissible() {
        let item = FlashbarItem::new(FlashbarType::Error, html! { "Test" }).with_dismissible(true);
        assert_eq!(item.dismissible, true);
    }

    #[test]
    fn test_flashbar_item_builder_with_loading() {
        let item = FlashbarItem::new(FlashbarType::Info, html! { "Test" }).with_loading(true);
        assert_eq!(item.loading, true);
    }

    #[test]
    fn test_flashbar_item_builder_chained() {
        let item = FlashbarItem::new(FlashbarType::Success, html! { "Test" })
            .with_id("success-1")
            .with_header("Success!")
            .with_dismissible(true)
            .with_loading(false);

        assert_eq!(item.id, Some("success-1".to_string()));
        assert_eq!(item.header, Some("Success!".to_string()));
        assert_eq!(item.dismissible, true);
        assert_eq!(item.loading, false);
        assert_eq!(item.flash_type, FlashbarType::Success);
    }

    #[test]
    fn test_flashbar_dismiss_detail() {
        let detail = FlashbarDismissDetail {
            item_id: Some("test-id".to_string()),
        };
        assert_eq!(detail.item_id, Some("test-id".to_string()));
    }

    #[test]
    fn test_flashbar_dismiss_detail_without_id() {
        let detail = FlashbarDismissDetail { item_id: None };
        assert!(detail.item_id.is_none());
    }

    #[test]
    fn test_flashbar_type_equality() {
        assert_eq!(FlashbarType::Success, FlashbarType::Success);
        assert_ne!(FlashbarType::Success, FlashbarType::Error);
    }

    #[test]
    fn test_flashbar_type_copy() {
        let type1 = FlashbarType::Warning;
        let type2 = type1;
        assert_eq!(type1, type2);
    }
}
