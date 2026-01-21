// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Breadcrumb navigation component for displaying navigation hierarchy.
//!
//! A breadcrumb trail shows the user's current location within the application's
//! navigation hierarchy. The last item represents the current page and is not clickable.
//! When there are many items, the component can collapse them into an expandable dropdown.

use yew::prelude::*;
use web_sys::MouseEvent;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
    AnalyticsMetadata,
};

/// A single breadcrumb item in the navigation trail
#[derive(Clone, PartialEq, Debug)]
pub struct BreadcrumbItem {
    /// Display text for the breadcrumb
    pub text: String,
    /// URL that the breadcrumb links to
    pub href: String,
}

impl BreadcrumbItem {
    /// Creates a new breadcrumb item with the given text and href
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::BreadcrumbItem;
    ///
    /// let item = BreadcrumbItem::new("Home", "/");
    /// ```
    pub fn new(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            href: href.into(),
        }
    }

    /// Sets the display text for this breadcrumb
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Sets the href for this breadcrumb
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = href.into();
        self
    }
}

/// Event detail for breadcrumb follow/click events
#[derive(Clone, PartialEq, Debug)]
pub struct FollowDetail {
    /// The text of the breadcrumb that was clicked
    pub text: String,
    /// The href of the breadcrumb that was clicked
    pub href: String,
    /// The index of the item in the breadcrumbs array
    pub item_index: usize,
}

/// Follow event type for breadcrumbs
pub type BreadcrumbFollowEvent = CustomEvent<FollowDetail>;

/// Properties for the Breadcrumbs component
#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbsProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// An array of breadcrumb items describing the navigation hierarchy
    ///
    /// Each item has text and href properties. The last item is automatically
    /// considered the current page and will not be rendered as a link.
    #[prop_or_default]
    pub items: Vec<BreadcrumbItem>,

    /// Callback fired when a breadcrumb item is clicked with the left mouse button
    /// without pressing modifier keys (CTRL, ALT, SHIFT, META).
    ///
    /// Use this event and prevent default browser navigation (by calling
    /// `prevent_default()`) to implement client-side routing.
    #[prop_or_default]
    pub on_follow: Option<Callback<BreadcrumbFollowEvent>>,

    /// Callback fired when the user clicks on a breadcrumb item
    ///
    /// This event fires for all clicks, including those with modifier keys.
    #[prop_or_default]
    pub on_click: Option<Callback<BreadcrumbFollowEvent>>,

    /// ARIA label for the breadcrumb navigation
    ///
    /// Provides an accessible label that screen readers can announce.
    /// Defaults to "Breadcrumbs" if not provided.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// ARIA label for the expand/ellipsis button
    ///
    /// Used when breadcrumbs are collapsed and shown in a dropdown.
    /// Defaults to "Show path" if not provided.
    #[prop_or_default]
    pub expand_aria_label: Option<String>,
}

/// Breadcrumb navigation component
///
/// Displays a breadcrumb trail showing the navigation hierarchy. The last item
/// represents the current page and is not clickable. When there are many items,
/// the component can collapse middle items into an expandable dropdown.
///
/// # Examples
///
/// ## Basic breadcrumbs
///
/// ```rust
/// use cloudscape_components::{Breadcrumbs, BreadcrumbItem};
///
/// let items = vec![
///     BreadcrumbItem::new("Home", "/"),
///     BreadcrumbItem::new("Products", "/products"),
///     BreadcrumbItem::new("Details", "/products/123"),
/// ];
///
/// html! {
///     <Breadcrumbs items={items} />
/// }
/// ```
///
/// ## With navigation handling
///
/// ```rust
/// use cloudscape_components::{Breadcrumbs, BreadcrumbItem, BreadcrumbFollowEvent};
/// use yew::prelude::*;
///
/// let on_follow = Callback::from(|mut event: BreadcrumbFollowEvent| {
///     event.prevent_default(); // Prevent default navigation
///     // Implement your routing logic here
///     log::info!("Navigate to: {}", event.detail.href);
/// });
///
/// let items = vec![
///     BreadcrumbItem::new("Home", "/"),
///     BreadcrumbItem::new("Products", "/products"),
/// ];
///
/// html! {
///     <Breadcrumbs
///         items={items}
///         on_follow={on_follow}
///         aria_label="Navigation breadcrumbs"
///     />
/// }
/// ```
///
/// ## Builder pattern for items
///
/// ```rust
/// use cloudscape_components::{Breadcrumbs, BreadcrumbItem};
///
/// let items = vec![
///     BreadcrumbItem::new("Home", "/")
///         .with_text("Dashboard")
///         .with_href("/dashboard"),
///     BreadcrumbItem::new("Settings", "/settings"),
/// ];
///
/// html! {
///     <Breadcrumbs items={items} />
/// }
/// ```
#[function_component(Breadcrumbs)]
pub fn breadcrumbs(props: &BreadcrumbsProps) -> Html {
    let _metadata = ComponentMetadata::new("Breadcrumbs");

    // Default ARIA labels
    let aria_label = props.aria_label.clone()
        .unwrap_or_else(|| "Breadcrumbs".to_string());
    let _expand_aria_label = props.expand_aria_label.clone()
        .unwrap_or_else(|| "Show path".to_string());

    // Build component styles
    let classes = ClassBuilder::new()
        .add("awsui-breadcrumbs");

    let class = props.base.merge_classes(&classes.build());

    // Create analytics metadata
    let analytics = AnalyticsMetadata {
        action: None,
        detail: None,
        component: Some(crate::internal::analytics::ComponentAnalytics {
            name: "awsui.BreadcrumbGroup".to_string(),
            label: crate::internal::analytics::LabelIdentifier::Root,
            properties: None,
        }),
    };
    let analytics_attr = analytics.to_data_attribute();

    // Render breadcrumb items
    let items_html = props.items.iter().enumerate().map(|(index, item)| {
        let is_last = index == props.items.len() - 1;

        render_breadcrumb_item(
            item,
            index,
            is_last,
            props.on_click.clone(),
            props.on_follow.clone(),
        )
    }).collect::<Html>();

    html! {
        <nav
            id={props.base.id.clone()}
            class={class}
            aria-label={aria_label}
            data-analytics-metadata={analytics_attr}
        >
            <ol class="awsui-breadcrumbs-list">
                { items_html }
            </ol>
        </nav>
    }
}

/// Renders a single breadcrumb item
fn render_breadcrumb_item(
    item: &BreadcrumbItem,
    index: usize,
    is_last: bool,
    on_click: Option<Callback<BreadcrumbFollowEvent>>,
    on_follow: Option<Callback<BreadcrumbFollowEvent>>,
) -> Html {
    let item_classes = ClassBuilder::new()
        .add("awsui-breadcrumbs-item")
        .add_if(is_last, "awsui-breadcrumbs-item-last");

    // Create analytics metadata for the item
    let item_analytics = if !is_last {
        let analytics = AnalyticsMetadata {
            action: Some("click".to_string()),
            detail: Some(serde_json::json!({
                "position": (index + 1).to_string(),
                "label": ".awsui-breadcrumb-item",
                "href": &item.href,
            })),
            component: None,
        };
        Some(analytics.to_data_attribute())
    } else {
        None
    };

    // Create click handler for non-last items
    let click_handler = if !is_last {
        let item_clone = item.clone();
        let on_click_cb = on_click.clone();
        let on_follow_cb = on_follow.clone();

        Some(Callback::from(move |e: MouseEvent| {
            let detail = FollowDetail {
                text: item_clone.text.clone(),
                href: item_clone.href.clone(),
                item_index: index,
            };

            // Fire click event (always fires)
            if let Some(ref cb) = on_click_cb {
                let event = BreadcrumbFollowEvent::new_non_cancelable(detail.clone());
                cb.emit(event);
            }

            // Fire follow event for plain left clicks without modifiers
            if is_plain_left_click(&e) {
                if let Some(ref cb) = on_follow_cb {
                    let event = BreadcrumbFollowEvent::new(detail);
                    cb.emit(event.clone());

                    // Prevent default navigation if prevented by callback
                    if event.default_prevented {
                        e.prevent_default();
                    }
                }
            }
        }))
    } else {
        None
    };

    html! {
        <li class={item_classes.build()}>
            if is_last {
                // Last item is not a link - it's the current page
                <span
                    class="awsui-breadcrumbs-text awsui-breadcrumbs-current"
                    aria-current="page"
                >
                    { &item.text }
                </span>
            } else {
                // Regular breadcrumb link
                <a
                    class="awsui-breadcrumbs-link awsui-breadcrumb-item"
                    href={item.href.clone()}
                    onclick={click_handler}
                    data-analytics-metadata={item_analytics}
                >
                    { &item.text }
                </a>
                // Separator icon
                <span class="awsui-breadcrumbs-separator" aria-hidden="true">
                    { " / " }
                </span>
            }
        </li>
    }
}

/// Checks if a click is a plain left click without modifiers
fn is_plain_left_click(event: &MouseEvent) -> bool {
    event.button() == 0
        && !event.ctrl_key()
        && !event.shift_key()
        && !event.alt_key()
        && !event.meta_key()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_item_new() {
        let item = BreadcrumbItem::new("Home", "/");
        assert_eq!(item.text, "Home");
        assert_eq!(item.href, "/");
    }

    #[test]
    fn test_breadcrumb_item_builder() {
        let item = BreadcrumbItem::new("Old", "/old")
            .with_text("New")
            .with_href("/new");

        assert_eq!(item.text, "New");
        assert_eq!(item.href, "/new");
    }

    #[test]
    fn test_breadcrumb_item_builder_chain() {
        let item = BreadcrumbItem::new("", "")
            .with_text("Products")
            .with_href("/products");

        assert_eq!(item.text, "Products");
        assert_eq!(item.href, "/products");
    }

    #[test]
    fn test_follow_detail_creation() {
        let detail = FollowDetail {
            text: "Home".to_string(),
            href: "/".to_string(),
            item_index: 0,
        };

        assert_eq!(detail.text, "Home");
        assert_eq!(detail.href, "/");
        assert_eq!(detail.item_index, 0);
    }

    #[test]
    fn test_follow_detail_equality() {
        let detail1 = FollowDetail {
            text: "Home".to_string(),
            href: "/".to_string(),
            item_index: 0,
        };

        let detail2 = FollowDetail {
            text: "Home".to_string(),
            href: "/".to_string(),
            item_index: 0,
        };

        assert_eq!(detail1, detail2);
    }

    #[test]
    fn test_follow_detail_inequality() {
        let detail1 = FollowDetail {
            text: "Home".to_string(),
            href: "/".to_string(),
            item_index: 0,
        };

        let detail2 = FollowDetail {
            text: "Products".to_string(),
            href: "/products".to_string(),
            item_index: 1,
        };

        assert_ne!(detail1, detail2);
    }

    #[test]
    fn test_breadcrumb_item_equality() {
        let item1 = BreadcrumbItem::new("Home", "/");
        let item2 = BreadcrumbItem::new("Home", "/");

        assert_eq!(item1, item2);
    }

    #[test]
    fn test_breadcrumb_item_inequality() {
        let item1 = BreadcrumbItem::new("Home", "/");
        let item2 = BreadcrumbItem::new("Products", "/products");

        assert_ne!(item1, item2);
    }

    #[test]
    fn test_breadcrumb_item_clone() {
        let item1 = BreadcrumbItem::new("Home", "/");
        let item2 = item1.clone();

        assert_eq!(item1, item2);
        assert_eq!(item1.text, item2.text);
        assert_eq!(item1.href, item2.href);
    }

    #[test]
    fn test_custom_event_creation() {
        let detail = FollowDetail {
            text: "Home".to_string(),
            href: "/".to_string(),
            item_index: 0,
        };

        let event = BreadcrumbFollowEvent::new(detail.clone());
        assert_eq!(event.detail, detail);
        assert!(event.cancelable);
        assert!(!event.default_prevented);
    }

    #[test]
    fn test_custom_event_prevent_default() {
        let detail = FollowDetail {
            text: "Home".to_string(),
            href: "/".to_string(),
            item_index: 0,
        };

        let mut event = BreadcrumbFollowEvent::new(detail);
        assert!(!event.default_prevented);

        event.prevent_default();
        assert!(event.default_prevented);
    }

    #[test]
    fn test_custom_event_non_cancelable() {
        let detail = FollowDetail {
            text: "Home".to_string(),
            href: "/".to_string(),
            item_index: 0,
        };

        let mut event = BreadcrumbFollowEvent::new_non_cancelable(detail);
        assert!(!event.cancelable);

        event.prevent_default();
        // Should not be prevented because it's not cancelable
        assert!(!event.default_prevented);
    }
}
