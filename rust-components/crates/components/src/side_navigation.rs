// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! SideNavigation component
//!
//! Sidebar navigation with hierarchical items for organizing application navigation.

use crate::internal::{
    AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent,
};
use web_sys::MouseEvent;
use yew::prelude::*;

/// Types of items that can appear in the side navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideNavigationItemType {
    /// A navigation link
    Link,
    /// A divider between navigation sections
    Divider,
    /// A collapsible section containing other items
    Section,
    /// An expandable group of links
    ExpandableLinkGroup,
}

impl SideNavigationItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SideNavigationItemType::Link => "link",
            SideNavigationItemType::Divider => "divider",
            SideNavigationItemType::Section => "section",
            SideNavigationItemType::ExpandableLinkGroup => "expandable-link-group",
        }
    }
}

/// A single item in the side navigation
#[derive(Clone, PartialEq)]
pub struct SideNavigationItem {
    /// The type of navigation item
    pub item_type: SideNavigationItemType,
    /// Display text for the item
    pub text: String,
    /// Optional href for link items
    pub href: Option<String>,
    /// Whether the link opens in an external context
    pub external: bool,
    /// Nested items (for sections and expandable groups)
    pub items: Vec<SideNavigationItem>,
    /// Optional info content displayed next to the item
    pub info: Option<Html>,
    /// Whether the item should be expanded by default
    pub default_expanded: Option<bool>,
}

impl SideNavigationItem {
    /// Creates a new link item
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::SideNavigationItem;
    ///
    /// let link = SideNavigationItem::link("Dashboard", "/dashboard");
    /// ```
    pub fn link(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            item_type: SideNavigationItemType::Link,
            text: text.into(),
            href: Some(href.into()),
            external: false,
            items: Vec::new(),
            info: None,
            default_expanded: None,
        }
    }

    /// Creates a new divider item
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::SideNavigationItem;
    ///
    /// let divider = SideNavigationItem::divider();
    /// ```
    pub fn divider() -> Self {
        Self {
            item_type: SideNavigationItemType::Divider,
            text: String::new(),
            href: None,
            external: false,
            items: Vec::new(),
            info: None,
            default_expanded: None,
        }
    }

    /// Creates a new section item
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::SideNavigationItem;
    ///
    /// let section = SideNavigationItem::section("Settings")
    ///     .with_items(vec![
    ///         SideNavigationItem::link("Profile", "/settings/profile"),
    ///         SideNavigationItem::link("Security", "/settings/security"),
    ///     ]);
    /// ```
    pub fn section(text: impl Into<String>) -> Self {
        Self {
            item_type: SideNavigationItemType::Section,
            text: text.into(),
            href: None,
            external: false,
            items: Vec::new(),
            info: None,
            default_expanded: Some(true),
        }
    }

    /// Creates a new expandable link group
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::SideNavigationItem;
    ///
    /// let group = SideNavigationItem::expandable_link_group("Resources", "/resources")
    ///     .with_items(vec![
    ///         SideNavigationItem::link("Docs", "/resources/docs"),
    ///         SideNavigationItem::link("API", "/resources/api"),
    ///     ]);
    /// ```
    pub fn expandable_link_group(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            item_type: SideNavigationItemType::ExpandableLinkGroup,
            text: text.into(),
            href: Some(href.into()),
            external: false,
            items: Vec::new(),
            info: None,
            default_expanded: None,
        }
    }

    /// Sets whether the link opens externally
    pub fn with_external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    /// Sets the nested items
    pub fn with_items(mut self, items: Vec<SideNavigationItem>) -> Self {
        self.items = items;
        self
    }

    /// Sets the info content
    pub fn with_info(mut self, info: Html) -> Self {
        self.info = Some(info);
        self
    }

    /// Sets whether the item should be expanded by default
    pub fn with_default_expanded(mut self, expanded: bool) -> Self {
        self.default_expanded = Some(expanded);
        self
    }
}

/// Header configuration for the side navigation
#[derive(Clone, PartialEq)]
pub struct SideNavigationHeader {
    /// Header text
    pub text: String,
    /// Header href
    pub href: String,
}

impl SideNavigationHeader {
    /// Creates a new header
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::SideNavigationHeader;
    ///
    /// let header = SideNavigationHeader::new("My App", "/");
    /// ```
    pub fn new(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            href: href.into(),
        }
    }
}

/// Event detail for follow events
#[derive(Clone, PartialEq)]
pub struct FollowDetail {
    /// The href that was followed
    pub href: String,
    /// Whether the link is external
    pub external: bool,
    /// The text of the item
    pub text: String,
    /// The type of item that was followed
    pub item_type: SideNavigationItemType,
}

/// Event detail for change events
#[derive(Clone, PartialEq)]
pub struct ChangeDetail {
    /// The item that was expanded or collapsed
    pub item: SideNavigationItem,
    /// Whether the item is now expanded
    pub expanded: bool,
}

/// Properties for the SideNavigation component
#[derive(Properties, PartialEq, Clone)]
pub struct SideNavigationProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Optional header for the navigation
    #[prop_or_default]
    pub header: Option<SideNavigationHeader>,

    /// The href of the currently active link
    ///
    /// All items with a matching href are highlighted.
    #[prop_or_default]
    pub active_href: Option<String>,

    /// The items to display in the navigation
    #[prop_or_default]
    pub items: Vec<SideNavigationItem>,

    /// Callback fired when a link is clicked
    ///
    /// The event detail contains information about the clicked item.
    #[prop_or_default]
    pub on_follow: Option<Callback<CustomEvent<FollowDetail>>>,

    /// Callback fired when a section or expandable group is expanded/collapsed
    ///
    /// The event detail contains the item and its new expanded state.
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<ChangeDetail>>>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA label for the navigation
    #[prop_or_default]
    pub aria_label: Option<String>,
}

/// SideNavigation component
///
/// A sidebar navigation component with hierarchical items for organizing application navigation.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{SideNavigation, SideNavigationItem, SideNavigationHeader, FollowDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let active_href = use_state(|| "/dashboard".to_string());
///
///     let items = vec![
///         SideNavigationItem::link("Dashboard", "/dashboard"),
///         SideNavigationItem::link("Reports", "/reports"),
///         SideNavigationItem::divider(),
///         SideNavigationItem::section("Settings")
///             .with_items(vec![
///                 SideNavigationItem::link("Profile", "/settings/profile"),
///                 SideNavigationItem::link("Security", "/settings/security"),
///             ]),
///         SideNavigationItem::expandable_link_group("Resources", "/resources")
///             .with_items(vec![
///                 SideNavigationItem::link("Documentation", "/resources/docs"),
///                 SideNavigationItem::link("API Reference", "/resources/api")
///                     .with_external(true),
///             ]),
///     ];
///
///     let on_follow = {
///         let active_href = active_href.clone();
///         Callback::from(move |event: CustomEvent<FollowDetail>| {
///             if !event.detail.external {
///                 active_href.set(event.detail.href.clone());
///             }
///         })
///     };
///
///     html! {
///         <SideNavigation
///             header={SideNavigationHeader::new("My Application", "/")}
///             items={items}
///             active_href={(*active_href).clone()}
///             on_follow={on_follow}
///         />
///     }
/// }
/// ```
#[function_component(SideNavigation)]
pub fn side_navigation(props: &SideNavigationProps) -> Html {
    let _metadata = ComponentMetadata::new("SideNavigation");

    // Track expanded state for sections and expandable groups
    let expanded_items = use_state(|| {
        let mut map = std::collections::HashMap::new();
        init_expanded_state(&props.items, &props.active_href, &mut map);
        map
    });

    // Re-initialize expanded state when items change
    use_effect_with((props.items.clone(), props.active_href.clone()), {
        let expanded_items = expanded_items.clone();
        move |(items, active_href)| {
            let mut map = std::collections::HashMap::new();
            init_expanded_state(items, active_href, &mut map);
            expanded_items.set(map);
            || ()
        }
    });

    // Build root classes
    let root_classes = ClassBuilder::new().add("awsui-side-navigation");

    html! {
        <nav
            id={props.base.id.clone()}
            class={root_classes.build()}
            aria-label={props.aria_label.clone().or_else(|| Some("Side navigation".to_string()))}
            aria-labelledby={props.aria.labelledby.clone()}
        >
            // Header
            if let Some(ref header) = props.header {
                { render_header(header, &props.active_href, &props.on_follow) }
            }

            // Items list
            if !props.items.is_empty() {
                <div class="awsui-side-navigation-list-container">
                    { render_items_list(
                        &props.items,
                        &props.active_href,
                        &props.on_follow,
                        &props.on_change,
                        expanded_items.clone(),
                        0
                    )}
                </div>
            }
        </nav>
    }
}

/// Initializes the expanded state for all items
fn init_expanded_state(
    items: &[SideNavigationItem],
    active_href: &Option<String>,
    map: &mut std::collections::HashMap<String, bool>,
) {
    for (index, item) in items.iter().enumerate() {
        let item_key = format!("{}-{}", item.item_type.as_str(), index);

        match item.item_type {
            SideNavigationItemType::Section | SideNavigationItemType::ExpandableLinkGroup => {
                // Check if any nested item is active
                let has_active = if let Some(active) = active_href {
                    contains_active_href(&item.items, active)
                } else {
                    false
                };

                // Use default_expanded if set, otherwise expand if contains active item
                let should_expand = item.default_expanded.unwrap_or(has_active);
                map.insert(item_key.clone(), should_expand);

                // Recursively initialize nested items
                init_expanded_state(&item.items, active_href, map);
            }
            _ => {}
        }
    }
}

/// Checks if any item in the list has the given href
fn contains_active_href(items: &[SideNavigationItem], href: &str) -> bool {
    items.iter().any(|item| {
        if let Some(ref item_href) = item.href
            && item_href == href
        {
            return true;
        }
        if !item.items.is_empty() {
            return contains_active_href(&item.items, href);
        }
        false
    })
}

/// Renders the header section
fn render_header(
    header: &SideNavigationHeader,
    active_href: &Option<String>,
    on_follow: &Option<Callback<CustomEvent<FollowDetail>>>,
) -> Html {
    let is_active = active_href
        .as_ref()
        .map(|href| href == &header.href)
        .unwrap_or(false);

    let header_classes = ClassBuilder::new()
        .add("awsui-side-navigation-header")
        .add_if(is_active, "awsui-side-navigation-header-active");

    let onclick = {
        let on_follow = on_follow.clone();
        let href = header.href.clone();
        let text = header.text.clone();

        Callback::from(move |e: MouseEvent| {
            // Only fire for plain left clicks
            if e.button() == 0 && !e.ctrl_key() && !e.shift_key() && !e.alt_key() && !e.meta_key() {
                e.prevent_default();
                if let Some(ref callback) = on_follow {
                    callback.emit(CustomEvent::new(FollowDetail {
                        href: href.clone(),
                        external: false,
                        text: text.clone(),
                        item_type: SideNavigationItemType::Link,
                    }));
                }
            }
        })
    };

    html! {
        <div class={header_classes.build()}>
            <a
                href={header.href.clone()}
                class="awsui-side-navigation-header-link"
                onclick={onclick}
            >
                <span class="awsui-side-navigation-header-link-text">
                    { &header.text }
                </span>
            </a>
        </div>
    }
}

/// Renders a list of navigation items
fn render_items_list(
    items: &[SideNavigationItem],
    active_href: &Option<String>,
    on_follow: &Option<Callback<CustomEvent<FollowDetail>>>,
    on_change: &Option<Callback<CustomEvent<ChangeDetail>>>,
    expanded_items: UseStateHandle<std::collections::HashMap<String, bool>>,
    depth: usize,
) -> Html {
    let list_classes = ClassBuilder::new()
        .add("awsui-side-navigation-list")
        .add(format!("awsui-side-navigation-list-depth-{}", depth));

    html! {
        <ul class={list_classes.build()} role="list">
            {
                items.iter().enumerate().map(|(index, item)| {
                    render_item(
                        item,
                        index,
                        active_href,
                        on_follow,
                        on_change,
                        expanded_items.clone(),
                        depth
                    )
                }).collect::<Html>()
            }
        </ul>
    }
}

/// Renders a single navigation item
fn render_item(
    item: &SideNavigationItem,
    index: usize,
    active_href: &Option<String>,
    on_follow: &Option<Callback<CustomEvent<FollowDetail>>>,
    on_change: &Option<Callback<CustomEvent<ChangeDetail>>>,
    expanded_items: UseStateHandle<std::collections::HashMap<String, bool>>,
    depth: usize,
) -> Html {
    let item_key = format!("{}-{}", item.item_type.as_str(), index);

    match item.item_type {
        SideNavigationItemType::Divider => {
            html! {
                <li key={item_key.clone()} class="awsui-side-navigation-item awsui-side-navigation-divider" role="separator" />
            }
        }
        SideNavigationItemType::Link => render_link_item(item, &item_key, active_href, on_follow),
        SideNavigationItemType::Section => render_section_item(
            item,
            &item_key,
            active_href,
            on_follow,
            on_change,
            expanded_items,
            depth,
        ),
        SideNavigationItemType::ExpandableLinkGroup => render_expandable_group_item(
            item,
            &item_key,
            active_href,
            on_follow,
            on_change,
            expanded_items,
            depth,
        ),
    }
}

/// Renders a link item
fn render_link_item(
    item: &SideNavigationItem,
    item_key: &str,
    active_href: &Option<String>,
    on_follow: &Option<Callback<CustomEvent<FollowDetail>>>,
) -> Html {
    let is_active = if let (Some(active), Some(href)) = (active_href, &item.href) {
        active == href
    } else {
        false
    };

    let item_classes = ClassBuilder::new()
        .add("awsui-side-navigation-item")
        .add("awsui-side-navigation-link")
        .add_if(is_active, "awsui-side-navigation-link-active");

    let onclick = {
        let on_follow = on_follow.clone();
        let item = item.clone();

        Callback::from(move |e: MouseEvent| {
            // Only fire for plain left clicks
            if e.button() == 0 && !e.ctrl_key() && !e.shift_key() && !e.alt_key() && !e.meta_key() {
                if !item.external {
                    e.prevent_default();
                }
                if let Some(ref callback) = on_follow
                    && let Some(ref href) = item.href
                {
                    callback.emit(CustomEvent::new(FollowDetail {
                        href: href.clone(),
                        external: item.external,
                        text: item.text.clone(),
                        item_type: SideNavigationItemType::Link,
                    }));
                }
            }
        })
    };

    html! {
        <li key={item_key} class={item_classes.build()} role="listitem">
            if item.external {
                <a
                    href={item.href.clone()}
                    class="awsui-side-navigation-link-anchor"
                    target="_blank"
                    rel="noopener noreferrer"
                    onclick={onclick.clone()}
                    aria-current={if is_active { Some("page") } else { None }}
                >
                    <span class="awsui-side-navigation-link-text">
                        { &item.text }
                    </span>
                    <span class="awsui-side-navigation-external-icon" aria-label="(opens in a new tab)">
                        { "↗" }
                    </span>
                    if let Some(ref info) = item.info {
                        <span class="awsui-side-navigation-link-info">
                            { info.clone() }
                        </span>
                    }
                </a>
            } else {
                <a
                    href={item.href.clone()}
                    class="awsui-side-navigation-link-anchor"
                    onclick={onclick}
                    aria-current={if is_active { Some("page") } else { None }}
                >
                    <span class="awsui-side-navigation-link-text">
                        { &item.text }
                    </span>
                    if let Some(ref info) = item.info {
                        <span class="awsui-side-navigation-link-info">
                            { info.clone() }
                        </span>
                    }
                </a>
            }
        </li>
    }
}

/// Renders a section item
fn render_section_item(
    item: &SideNavigationItem,
    item_key: &str,
    active_href: &Option<String>,
    on_follow: &Option<Callback<CustomEvent<FollowDetail>>>,
    on_change: &Option<Callback<CustomEvent<ChangeDetail>>>,
    expanded_items: UseStateHandle<std::collections::HashMap<String, bool>>,
    depth: usize,
) -> Html {
    let is_expanded = expanded_items.get(item_key).copied().unwrap_or(true);

    let section_classes = ClassBuilder::new()
        .add("awsui-side-navigation-item")
        .add("awsui-side-navigation-section")
        .add_if(is_expanded, "awsui-side-navigation-section-expanded");

    let toggle_onclick = {
        let on_change = on_change.clone();
        let item = item.clone();
        let expanded_items = expanded_items.clone();
        let item_key = item_key.to_string();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let new_expanded = !is_expanded;

            // Update expanded state
            let mut map = (*expanded_items).clone();
            map.insert(item_key.clone(), new_expanded);
            expanded_items.set(map);

            // Fire change event
            if let Some(ref callback) = on_change {
                callback.emit(CustomEvent::new_non_cancelable(ChangeDetail {
                    item: item.clone(),
                    expanded: new_expanded,
                }));
            }
        })
    };

    html! {
        <li key={item_key} class={section_classes.build()} role="listitem">
            <button
                type="button"
                class="awsui-side-navigation-section-toggle"
                aria-expanded={is_expanded.to_string()}
                onclick={toggle_onclick}
            >
                <span class="awsui-side-navigation-section-toggle-icon" aria-hidden="true">
                    { if is_expanded { "▼" } else { "▶" } }
                </span>
                <span class="awsui-side-navigation-section-text">
                    { &item.text }
                </span>
            </button>
            if is_expanded && !item.items.is_empty() {
                { render_items_list(
                    &item.items,
                    active_href,
                    on_follow,
                    on_change,
                    expanded_items,
                    depth + 1
                )}
            }
        </li>
    }
}

/// Renders an expandable link group item
fn render_expandable_group_item(
    item: &SideNavigationItem,
    item_key: &str,
    active_href: &Option<String>,
    on_follow: &Option<Callback<CustomEvent<FollowDetail>>>,
    on_change: &Option<Callback<CustomEvent<ChangeDetail>>>,
    expanded_items: UseStateHandle<std::collections::HashMap<String, bool>>,
    depth: usize,
) -> Html {
    let is_expanded = expanded_items.get(item_key).copied().unwrap_or(false);
    let is_active = if let (Some(active), Some(href)) = (active_href, &item.href) {
        active == href
    } else {
        false
    };

    let group_classes = ClassBuilder::new()
        .add("awsui-side-navigation-item")
        .add("awsui-side-navigation-expandable-group")
        .add_if(
            is_expanded,
            "awsui-side-navigation-expandable-group-expanded",
        )
        .add_if(is_active, "awsui-side-navigation-expandable-group-active");

    let toggle_onclick = {
        let on_change = on_change.clone();
        let item = item.clone();
        let expanded_items = expanded_items.clone();
        let item_key = item_key.to_string();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let new_expanded = !is_expanded;

            // Update expanded state
            let mut map = (*expanded_items).clone();
            map.insert(item_key.clone(), new_expanded);
            expanded_items.set(map);

            // Fire change event
            if let Some(ref callback) = on_change {
                callback.emit(CustomEvent::new_non_cancelable(ChangeDetail {
                    item: item.clone(),
                    expanded: new_expanded,
                }));
            }
        })
    };

    let link_onclick = {
        let on_follow = on_follow.clone();
        let item = item.clone();

        Callback::from(move |e: MouseEvent| {
            // Only fire for plain left clicks
            if e.button() == 0 && !e.ctrl_key() && !e.shift_key() && !e.alt_key() && !e.meta_key() {
                e.prevent_default();
                if let Some(ref callback) = on_follow
                    && let Some(ref href) = item.href
                {
                    callback.emit(CustomEvent::new(FollowDetail {
                        href: href.clone(),
                        external: false,
                        text: item.text.clone(),
                        item_type: SideNavigationItemType::ExpandableLinkGroup,
                    }));
                }
            }
        })
    };

    html! {
        <li key={item_key} class={group_classes.build()} role="listitem">
            <div class="awsui-side-navigation-expandable-group-header">
                <button
                    type="button"
                    class="awsui-side-navigation-expandable-group-toggle"
                    aria-expanded={is_expanded.to_string()}
                    onclick={toggle_onclick}
                >
                    <span class="awsui-side-navigation-expandable-group-toggle-icon" aria-hidden="true">
                        { if is_expanded { "▼" } else { "▶" } }
                    </span>
                </button>
                <a
                    href={item.href.clone()}
                    class="awsui-side-navigation-expandable-group-link"
                    onclick={link_onclick}
                    aria-current={if is_active { Some("page") } else { None }}
                >
                    <span class="awsui-side-navigation-expandable-group-text">
                        { &item.text }
                    </span>
                    if let Some(ref info) = item.info {
                        <span class="awsui-side-navigation-expandable-group-info">
                            { info.clone() }
                        </span>
                    }
                </a>
            </div>
            if is_expanded && !item.items.is_empty() {
                { render_items_list(
                    &item.items,
                    active_href,
                    on_follow,
                    on_change,
                    expanded_items,
                    depth + 1
                )}
            }
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_navigation_item_link() {
        let item = SideNavigationItem::link("Dashboard", "/dashboard");
        assert_eq!(item.item_type, SideNavigationItemType::Link);
        assert_eq!(item.text, "Dashboard");
        assert_eq!(item.href, Some("/dashboard".to_string()));
        assert!(!item.external);
        assert!(item.items.is_empty());
    }

    #[test]
    fn test_side_navigation_item_link_with_external() {
        let item =
            SideNavigationItem::link("External Link", "https://example.com").with_external(true);
        assert_eq!(item.item_type, SideNavigationItemType::Link);
        assert!(item.external);
    }

    #[test]
    fn test_side_navigation_item_divider() {
        let item = SideNavigationItem::divider();
        assert_eq!(item.item_type, SideNavigationItemType::Divider);
        assert_eq!(item.text, "");
        assert_eq!(item.href, None);
    }

    #[test]
    fn test_side_navigation_item_section() {
        let item = SideNavigationItem::section("Settings").with_items(vec![
            SideNavigationItem::link("Profile", "/settings/profile"),
            SideNavigationItem::link("Security", "/settings/security"),
        ]);

        assert_eq!(item.item_type, SideNavigationItemType::Section);
        assert_eq!(item.text, "Settings");
        assert_eq!(item.items.len(), 2);
        assert_eq!(item.default_expanded, Some(true));
    }

    #[test]
    fn test_side_navigation_item_expandable_group() {
        let item = SideNavigationItem::expandable_link_group("Resources", "/resources")
            .with_items(vec![SideNavigationItem::link("Docs", "/resources/docs")])
            .with_default_expanded(false);

        assert_eq!(item.item_type, SideNavigationItemType::ExpandableLinkGroup);
        assert_eq!(item.text, "Resources");
        assert_eq!(item.href, Some("/resources".to_string()));
        assert_eq!(item.items.len(), 1);
        assert_eq!(item.default_expanded, Some(false));
    }

    #[test]
    fn test_side_navigation_header() {
        let header = SideNavigationHeader::new("My App", "/");
        assert_eq!(header.text, "My App");
        assert_eq!(header.href, "/");
    }

    #[test]
    fn test_contains_active_href() {
        let items = vec![
            SideNavigationItem::link("Home", "/"),
            SideNavigationItem::section("Settings").with_items(vec![SideNavigationItem::link(
                "Profile",
                "/settings/profile",
            )]),
        ];

        assert!(contains_active_href(&items, "/settings/profile"));
        assert!(contains_active_href(&items, "/"));
        assert!(!contains_active_href(&items, "/not-found"));
    }

    #[test]
    fn test_item_type_as_str() {
        assert_eq!(SideNavigationItemType::Link.as_str(), "link");
        assert_eq!(SideNavigationItemType::Divider.as_str(), "divider");
        assert_eq!(SideNavigationItemType::Section.as_str(), "section");
        assert_eq!(
            SideNavigationItemType::ExpandableLinkGroup.as_str(),
            "expandable-link-group"
        );
    }

    #[test]
    fn test_follow_detail() {
        let detail = FollowDetail {
            href: "/test".to_string(),
            external: false,
            text: "Test".to_string(),
            item_type: SideNavigationItemType::Link,
        };

        assert_eq!(detail.href, "/test");
        assert!(!detail.external);
        assert_eq!(detail.text, "Test");
    }

    #[test]
    fn test_change_detail() {
        let item = SideNavigationItem::section("Test");
        let detail = ChangeDetail {
            item: item.clone(),
            expanded: true,
        };

        assert_eq!(detail.item.text, "Test");
        assert!(detail.expanded);
    }
}
