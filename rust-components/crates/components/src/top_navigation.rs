// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Top Navigation component
//!
//! A top application bar with menu items, identity section, and utility items.
//! This component provides the main navigation header for applications.

use yew::prelude::*;
use web_sys::MouseEvent;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
    AnalyticsMetadata, AriaAttributes,
};

/// Type of utility item in the top navigation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UtilityType {
    /// A button utility
    Button,
    /// A menu dropdown utility
    MenuDropdown,
}

impl UtilityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UtilityType::Button => "button",
            UtilityType::MenuDropdown => "menu-dropdown",
        }
    }
}

/// Button variant for button utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UtilityButtonVariant {
    /// Link-style button (default)
    #[default]
    Link,
    /// Primary button style
    PrimaryButton,
}

impl UtilityButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            UtilityButtonVariant::Link => "link",
            UtilityButtonVariant::PrimaryButton => "primary-button",
        }
    }
}

/// Logo configuration for the identity section
#[derive(Clone, PartialEq, Debug)]
pub struct TopNavigationLogo {
    /// Source URL for the logo image
    pub src: String,
    /// Alt text for the logo
    pub alt: Option<String>,
}

impl TopNavigationLogo {
    /// Creates a new logo with the given source
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: None,
        }
    }

    /// Sets the alt text for the logo
    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }
}

/// Identity section configuration
#[derive(Clone, PartialEq)]
pub struct TopNavigationIdentity {
    /// Link href for the identity/logo
    pub href: String,
    /// Title text
    pub title: Option<String>,
    /// Logo configuration
    pub logo: Option<TopNavigationLogo>,
    /// Callback fired when identity is clicked without modifier keys
    pub on_follow: Option<Callback<CustomEvent<IdentityFollowDetail>>>,
}

impl TopNavigationIdentity {
    /// Creates a new identity with the given href
    pub fn new(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            title: None,
            logo: None,
            on_follow: None,
        }
    }

    /// Sets the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the logo
    pub fn with_logo(mut self, logo: TopNavigationLogo) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Sets the on_follow callback
    pub fn with_on_follow(mut self, callback: Callback<CustomEvent<IdentityFollowDetail>>) -> Self {
        self.on_follow = Some(callback);
        self
    }
}

/// Event detail for identity follow events
#[derive(Debug, Clone, PartialEq)]
pub struct IdentityFollowDetail {
    /// The href being navigated to
    pub href: String,
}

/// A utility item in the top navigation
#[derive(Clone, PartialEq)]
pub struct TopNavigationUtility {
    /// Unique identifier for the utility
    pub id: String,
    /// Type of utility (button or menu-dropdown)
    pub utility_type: UtilityType,
    /// Display text
    pub text: Option<String>,
    /// Title displayed in dropdown (for menu-dropdown type)
    pub title: Option<String>,
    /// Icon name (Cloudscape icon)
    pub icon_name: Option<String>,
    /// Custom icon SVG
    pub icon_svg: Option<Html>,
    /// Custom icon URL
    pub icon_url: Option<String>,
    /// Icon alt text for custom icons
    pub icon_alt: Option<String>,
    /// ARIA label
    pub aria_label: Option<String>,
    /// Whether to show a badge indicator
    pub badge: bool,
    /// Prevent text from collapsing on small screens
    pub disable_text_collapse: bool,
    /// Prevent utility from being moved to overflow menu
    pub disable_utility_collapse: bool,

    // Button-specific properties
    /// Button variant (for button type)
    pub variant: Option<UtilityButtonVariant>,
    /// Whether the button is disabled
    pub disabled: bool,
    /// Link href (for button type)
    pub href: Option<String>,
    /// Link target (for button type)
    pub target: Option<String>,
    /// Link rel attribute (for button type)
    pub rel: Option<String>,
    /// Whether link is external (for button type)
    pub external: bool,
    /// External icon aria label (for button type)
    pub external_icon_aria_label: Option<String>,
    /// Click callback (for button type)
    pub on_click: Option<Callback<CustomEvent<UtilityClickDetail>>>,
    /// Follow callback (for button type with href)
    pub on_follow: Option<Callback<CustomEvent<UtilityFollowDetail>>>,

    // Menu dropdown-specific properties
    /// Description text (for menu-dropdown type)
    pub description: Option<String>,
    /// Menu items (for menu-dropdown type) - simplified for now
    pub items: Vec<String>,
    /// Whether groups are expandable (for menu-dropdown type)
    pub expandable_groups: bool,
}

impl TopNavigationUtility {
    /// Creates a new button utility
    pub fn button(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            utility_type: UtilityType::Button,
            text: None,
            title: None,
            icon_name: None,
            icon_svg: None,
            icon_url: None,
            icon_alt: None,
            aria_label: None,
            badge: false,
            disable_text_collapse: false,
            disable_utility_collapse: false,
            variant: None,
            disabled: false,
            href: None,
            target: None,
            rel: None,
            external: false,
            external_icon_aria_label: None,
            on_click: None,
            on_follow: None,
            description: None,
            items: Vec::new(),
            expandable_groups: false,
        }
    }

    /// Creates a new menu dropdown utility
    pub fn menu_dropdown(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            utility_type: UtilityType::MenuDropdown,
            text: None,
            title: None,
            icon_name: None,
            icon_svg: None,
            icon_url: None,
            icon_alt: None,
            aria_label: None,
            badge: false,
            disable_text_collapse: false,
            disable_utility_collapse: false,
            variant: None,
            disabled: false,
            href: None,
            target: None,
            rel: None,
            external: false,
            external_icon_aria_label: None,
            on_click: None,
            on_follow: None,
            description: None,
            items: Vec::new(),
            expandable_groups: false,
        }
    }

    /// Sets the display text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the icon name
    pub fn with_icon_name(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }

    /// Sets the custom icon SVG
    pub fn with_icon_svg(mut self, icon_svg: Html) -> Self {
        self.icon_svg = Some(icon_svg);
        self
    }

    /// Sets the custom icon URL
    pub fn with_icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    /// Sets the icon alt text
    pub fn with_icon_alt(mut self, icon_alt: impl Into<String>) -> Self {
        self.icon_alt = Some(icon_alt.into());
        self
    }

    /// Sets the ARIA label
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// Sets whether to show a badge
    pub fn with_badge(mut self, badge: bool) -> Self {
        self.badge = badge;
        self
    }

    /// Sets whether to disable text collapse
    pub fn with_disable_text_collapse(mut self, disable: bool) -> Self {
        self.disable_text_collapse = disable;
        self
    }

    /// Sets whether to disable utility collapse
    pub fn with_disable_utility_collapse(mut self, disable: bool) -> Self {
        self.disable_utility_collapse = disable;
        self
    }

    /// Sets the button variant (for button type)
    pub fn with_variant(mut self, variant: UtilityButtonVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Sets whether the button is disabled (for button type)
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the href (for button type)
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Sets the target (for button type)
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Sets the rel attribute (for button type)
    pub fn with_rel(mut self, rel: impl Into<String>) -> Self {
        self.rel = Some(rel.into());
        self
    }

    /// Sets whether the link is external (for button type)
    pub fn with_external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    /// Sets the external icon aria label (for button type)
    pub fn with_external_icon_aria_label(mut self, label: impl Into<String>) -> Self {
        self.external_icon_aria_label = Some(label.into());
        self
    }

    /// Sets the click callback (for button type)
    pub fn with_on_click(mut self, callback: Callback<CustomEvent<UtilityClickDetail>>) -> Self {
        self.on_click = Some(callback);
        self
    }

    /// Sets the follow callback (for button type)
    pub fn with_on_follow(mut self, callback: Callback<CustomEvent<UtilityFollowDetail>>) -> Self {
        self.on_follow = Some(callback);
        self
    }

    /// Sets the description (for menu-dropdown type)
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the menu items (for menu-dropdown type)
    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }

    /// Sets whether groups are expandable (for menu-dropdown type)
    pub fn with_expandable_groups(mut self, expandable: bool) -> Self {
        self.expandable_groups = expandable;
        self
    }
}

/// Event detail for utility click events
#[derive(Debug, Clone, PartialEq)]
pub struct UtilityClickDetail {
    /// The ID of the clicked utility
    pub id: String,
    /// Whether the link is external
    pub external: bool,
    /// The href if present
    pub href: Option<String>,
}

/// Event detail for utility follow events
#[derive(Debug, Clone, PartialEq)]
pub struct UtilityFollowDetail {
    /// The ID of the utility
    pub id: String,
    /// The href being navigated to
    pub href: String,
    /// Whether the link is external
    pub external: bool,
    /// The target attribute
    pub target: Option<String>,
}

/// I18n strings for the TopNavigation component
#[derive(Clone, PartialEq, Default)]
pub struct TopNavigationI18nStrings {
    /// ARIA label for search icon
    pub search_icon_aria_label: Option<String>,
    /// ARIA label for search dismiss icon
    pub search_dismiss_icon_aria_label: Option<String>,
    /// ARIA label for overflow menu dismiss icon
    pub overflow_menu_dismiss_icon_aria_label: Option<String>,
    /// ARIA label for overflow menu back icon
    pub overflow_menu_back_icon_aria_label: Option<String>,
    /// Text for overflow menu trigger
    pub overflow_menu_trigger_text: Option<String>,
    /// Title text for overflow menu
    pub overflow_menu_title_text: Option<String>,
}

impl TopNavigationI18nStrings {
    /// Creates a new i18n strings configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the search icon aria label
    pub fn with_search_icon_aria_label(mut self, label: impl Into<String>) -> Self {
        self.search_icon_aria_label = Some(label.into());
        self
    }

    /// Sets the search dismiss icon aria label
    pub fn with_search_dismiss_icon_aria_label(mut self, label: impl Into<String>) -> Self {
        self.search_dismiss_icon_aria_label = Some(label.into());
        self
    }

    /// Sets the overflow menu dismiss icon aria label
    pub fn with_overflow_menu_dismiss_icon_aria_label(mut self, label: impl Into<String>) -> Self {
        self.overflow_menu_dismiss_icon_aria_label = Some(label.into());
        self
    }

    /// Sets the overflow menu back icon aria label
    pub fn with_overflow_menu_back_icon_aria_label(mut self, label: impl Into<String>) -> Self {
        self.overflow_menu_back_icon_aria_label = Some(label.into());
        self
    }

    /// Sets the overflow menu trigger text
    pub fn with_overflow_menu_trigger_text(mut self, text: impl Into<String>) -> Self {
        self.overflow_menu_trigger_text = Some(text.into());
        self
    }

    /// Sets the overflow menu title text
    pub fn with_overflow_menu_title_text(mut self, text: impl Into<String>) -> Self {
        self.overflow_menu_title_text = Some(text.into());
        self
    }
}

/// Properties for the TopNavigation component
#[derive(Properties, PartialEq, Clone)]
pub struct TopNavigationProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Identity section (logo, title, href)
    pub identity: TopNavigationIdentity,

    /// List of utility items
    #[prop_or_default]
    pub utilities: Vec<TopNavigationUtility>,

    /// Search input slot
    #[prop_or_default]
    pub search: Option<Html>,

    /// I18n strings for localization
    #[prop_or_default]
    pub i18n_strings: TopNavigationI18nStrings,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,
}

/// TopNavigation component
///
/// A top application bar that provides branding, search, and utility navigation.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{
///     TopNavigation, TopNavigationIdentity, TopNavigationUtility,
///     TopNavigationLogo, UtilityClickDetail, CustomEvent
/// };
/// use yew::prelude::*;
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     let identity = TopNavigationIdentity::new("/")
///         .with_title("My Application")
///         .with_logo(TopNavigationLogo::new("/logo.svg").with_alt("Logo"));
///
///     let utilities = vec![
///         TopNavigationUtility::button("notifications")
///             .with_icon_name("notification")
///             .with_aria_label("Notifications")
///             .with_badge(true),
///         TopNavigationUtility::button("settings")
///             .with_text("Settings")
///             .with_icon_name("settings")
///             .with_href("/settings"),
///         TopNavigationUtility::menu_dropdown("user")
///             .with_text("John Doe")
///             .with_icon_name("user-profile")
///             .with_items(vec!["Profile".to_string(), "Sign out".to_string()]),
///     ];
///
///     html! {
///         <TopNavigation
///             identity={identity}
///             utilities={utilities}
///         />
///     }
/// }
/// ```
///
/// # Example with Search
///
/// ```rust
/// use cloudscape_components::{TopNavigation, TopNavigationIdentity, Input};
/// use yew::prelude::*;
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     let identity = TopNavigationIdentity::new("/")
///         .with_title("My Application");
///
///     let search = html! {
///         <Input placeholder="Search..." />
///     };
///
///     html! {
///         <TopNavigation
///             identity={identity}
///             search={search}
///         />
///     }
/// }
/// ```
#[function_component(TopNavigation)]
pub fn top_navigation(props: &TopNavigationProps) -> Html {
    let _metadata = ComponentMetadata::new("TopNavigation");

    // Build component styles
    let classes = ClassBuilder::new()
        .add("awsui-top-navigation")
        .add_if(props.search.is_some(), "awsui-top-navigation-has-search");

    let class = props.base.merge_classes(&classes.build());

    // Build analytics metadata
    let analytics = AnalyticsMetadata {
        action: Some("render".to_string()),
        detail: Some(serde_json::json!({
            "label": { "root": "self" },
        })),
        component: Some(crate::internal::analytics::ComponentAnalytics {
            name: "awsui.TopNavigation".to_string(),
            label: crate::internal::analytics::LabelIdentifier::Root,
            properties: Some(serde_json::json!({
                "utilitiesCount": props.utilities.len(),
            })),
        }),
    };
    let analytics_attr = analytics.to_data_attribute();

    // Render identity
    let identity_html = render_identity(&props.identity);

    // Render search
    let search_html = if let Some(ref search) = props.search {
        html! {
            <div class="awsui-top-navigation-search" role="search">
                { search.clone() }
            </div>
        }
    } else {
        html! {}
    };

    // Render utilities
    let utilities_html = render_utilities(&props.utilities);

    html! {
        <header
            id={props.base.id.clone()}
            class={class}
            role="banner"
            aria-label={props.aria.label.clone()}
            data-analytics-metadata={analytics_attr}
        >
            <div class="awsui-top-navigation-inner">
                { identity_html }
                { search_html }
                { utilities_html }
            </div>
        </header>
    }
}

/// Renders the identity section
fn render_identity(identity: &TopNavigationIdentity) -> Html {
    let identity_clone = identity.clone();

    let on_click = Callback::from(move |e: MouseEvent| {
        // Only fire on_follow for plain left clicks
        if e.button() == 0 && !e.ctrl_key() && !e.shift_key() && !e.alt_key() && !e.meta_key() {
            if let Some(ref callback) = identity_clone.on_follow {
                let detail = IdentityFollowDetail {
                    href: identity_clone.href.clone(),
                };
                let event = CustomEvent::new(detail);
                callback.emit(event.clone());

                if event.default_prevented {
                    e.prevent_default();
                }
            }
        }
    });

    html! {
        <div class="awsui-top-navigation-identity">
            <a
                href={identity.href.clone()}
                class="awsui-top-navigation-identity-link"
                onclick={on_click}
            >
                if let Some(ref logo) = identity.logo {
                    <div class="awsui-top-navigation-logo">
                        <img
                            src={logo.src.clone()}
                            alt={logo.alt.clone().unwrap_or_default()}
                            class="awsui-top-navigation-logo-image"
                        />
                    </div>
                }
                if let Some(ref title) = identity.title {
                    <span class="awsui-top-navigation-title">
                        { title }
                    </span>
                }
            </a>
        </div>
    }
}

/// Renders the utilities section
fn render_utilities(utilities: &[TopNavigationUtility]) -> Html {
    if utilities.is_empty() {
        return html! {};
    }

    let utility_items = utilities.iter().map(|utility| {
        render_utility(utility)
    }).collect::<Html>();

    html! {
        <div class="awsui-top-navigation-utilities">
            <ul class="awsui-top-navigation-utilities-list" role="list">
                { utility_items }
            </ul>
        </div>
    }
}

/// Renders a single utility item
fn render_utility(utility: &TopNavigationUtility) -> Html {
    let utility_classes = ClassBuilder::new()
        .add("awsui-top-navigation-utility")
        .add(format!("awsui-top-navigation-utility-type-{}", utility.utility_type.as_str()))
        .add_if(utility.badge, "awsui-top-navigation-utility-badge")
        .add_if(utility.disable_text_collapse, "awsui-top-navigation-utility-no-text-collapse")
        .add_if(utility.disable_utility_collapse, "awsui-top-navigation-utility-no-collapse")
        .build();

    match utility.utility_type {
        UtilityType::Button => render_button_utility(utility, &utility_classes),
        UtilityType::MenuDropdown => render_menu_dropdown_utility(utility, &utility_classes),
    }
}

/// Renders a button utility
fn render_button_utility(utility: &TopNavigationUtility, utility_classes: &str) -> Html {
    let utility_clone = utility.clone();
    let utility_classes = utility_classes.to_string();
    let is_disabled = utility.disabled;

    let on_click = Callback::from(move |e: MouseEvent| {
        if is_disabled {
            e.prevent_default();
            return;
        }

        let detail = UtilityClickDetail {
            id: utility_clone.id.clone(),
            external: utility_clone.external,
            href: utility_clone.href.clone(),
        };

        // Fire click callback
        if let Some(ref callback) = utility_clone.on_click {
            callback.emit(CustomEvent::new(detail.clone()));
        }

        // Fire follow callback for plain left clicks with href
        if e.button() == 0 && !e.ctrl_key() && !e.shift_key() && !e.alt_key() && !e.meta_key() {
            if let Some(ref href) = utility_clone.href {
                if let Some(ref callback) = utility_clone.on_follow {
                    let follow_detail = UtilityFollowDetail {
                        id: utility_clone.id.clone(),
                        href: href.clone(),
                        external: utility_clone.external,
                        target: utility_clone.target.clone(),
                    };
                    let event = CustomEvent::new(follow_detail);
                    callback.emit(event.clone());

                    if event.default_prevented {
                        e.prevent_default();
                    }
                }
            }
        }
    });

    let variant = utility.variant.unwrap_or_default();
    let button_classes = ClassBuilder::new()
        .add("awsui-top-navigation-utility-button")
        .add(format!("awsui-top-navigation-utility-button-variant-{}", variant.as_str()))
        .add_if(is_disabled, "awsui-top-navigation-utility-button-disabled")
        .add_if(utility.external, "awsui-top-navigation-utility-button-external")
        .build();

    let content = html! {
        <>
            { render_utility_icon(utility) }
            if let Some(ref text) = utility.text {
                <span class="awsui-top-navigation-utility-text">
                    { text }
                </span>
            }
            if utility.external && utility.href.is_some() {
                <span class="awsui-top-navigation-utility-external-icon" aria-label={
                    utility.external_icon_aria_label.clone()
                        .unwrap_or_else(|| "(opens in a new tab)".to_string())
                }>
                    {"↗"}
                </span>
            }
            if utility.badge {
                <span class="awsui-top-navigation-utility-badge-indicator" aria-label="Badge"></span>
            }
        </>
    };

    let element = if let Some(ref href) = utility.href {
        let actual_target = if utility.external && utility.target.is_none() {
            Some("_blank".to_string())
        } else {
            utility.target.clone()
        };

        let actual_rel = if let Some(ref rel) = utility.rel {
            Some(rel.clone())
        } else if utility.external || actual_target.as_deref() == Some("_blank") {
            Some("noopener noreferrer".to_string())
        } else {
            None
        };

        html! {
            <a
                href={href.clone()}
                target={actual_target}
                rel={actual_rel}
                class={button_classes}
                aria-label={utility.aria_label.clone()}
                aria-disabled={if is_disabled { Some("true") } else { None }}
                onclick={on_click}
            >
                { content }
            </a>
        }
    } else {
        html! {
            <button
                type="button"
                class={button_classes}
                aria-label={utility.aria_label.clone()}
                disabled={is_disabled}
                onclick={on_click}
            >
                { content }
            </button>
        }
    };

    html! {
        <li class={utility_classes} role="listitem">
            { element }
        </li>
    }
}

/// Renders a menu dropdown utility
fn render_menu_dropdown_utility(utility: &TopNavigationUtility, utility_classes: &str) -> Html {
    let utility_classes = utility_classes.to_string();
    let dropdown_classes = ClassBuilder::new()
        .add("awsui-top-navigation-utility-dropdown")
        .build();

    html! {
        <li class={utility_classes} role="listitem">
            <div class={dropdown_classes}>
                <button
                    type="button"
                    class="awsui-top-navigation-utility-dropdown-trigger"
                    aria-label={utility.aria_label.clone()}
                    aria-haspopup="true"
                    aria-expanded="false"
                >
                    { render_utility_icon(utility) }
                    if let Some(ref text) = utility.text {
                        <span class="awsui-top-navigation-utility-text">
                            { text }
                        </span>
                    }
                    if utility.badge {
                        <span class="awsui-top-navigation-utility-badge-indicator" aria-label="Badge"></span>
                    }
                    <span class="awsui-top-navigation-utility-dropdown-arrow">{"▼"}</span>
                </button>
                // Dropdown menu would be rendered here when open
                // This is a simplified implementation
            </div>
        </li>
    }
}

/// Renders the icon for a utility
fn render_utility_icon(utility: &TopNavigationUtility) -> Html {
    if let Some(ref icon_svg) = utility.icon_svg {
        html! {
            <span class="awsui-top-navigation-utility-icon awsui-top-navigation-utility-icon-svg">
                { icon_svg.clone() }
            </span>
        }
    } else if let Some(ref icon_url) = utility.icon_url {
        html! {
            <span class="awsui-top-navigation-utility-icon awsui-top-navigation-utility-icon-image">
                <img
                    src={icon_url.clone()}
                    alt={utility.icon_alt.clone().unwrap_or_default()}
                />
            </span>
        }
    } else if let Some(ref icon_name) = utility.icon_name {
        html! {
            <span class={format!("awsui-top-navigation-utility-icon awsui-icon awsui-icon-{}", icon_name)}>
                {"○"}
            </span>
        }
    } else {
        html! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utility_type_as_str() {
        assert_eq!(UtilityType::Button.as_str(), "button");
        assert_eq!(UtilityType::MenuDropdown.as_str(), "menu-dropdown");
    }

    #[test]
    fn test_utility_button_variant_as_str() {
        assert_eq!(UtilityButtonVariant::Link.as_str(), "link");
        assert_eq!(UtilityButtonVariant::PrimaryButton.as_str(), "primary-button");
    }

    #[test]
    fn test_utility_button_variant_default() {
        assert_eq!(UtilityButtonVariant::default(), UtilityButtonVariant::Link);
    }

    #[test]
    fn test_logo_builder() {
        let logo = TopNavigationLogo::new("logo.svg")
            .with_alt("My Logo");

        assert_eq!(logo.src, "logo.svg");
        assert_eq!(logo.alt, Some("My Logo".to_string()));
    }

    #[test]
    fn test_identity_builder() {
        let identity = TopNavigationIdentity::new("/")
            .with_title("My App");

        assert_eq!(identity.href, "/");
        assert_eq!(identity.title, Some("My App".to_string()));
    }

    #[test]
    fn test_utility_button_builder() {
        let utility = TopNavigationUtility::button("test")
            .with_text("Test Button")
            .with_icon_name("settings")
            .with_disabled(true)
            .with_badge(true);

        assert_eq!(utility.id, "test");
        assert_eq!(utility.utility_type, UtilityType::Button);
        assert_eq!(utility.text, Some("Test Button".to_string()));
        assert_eq!(utility.icon_name, Some("settings".to_string()));
        assert!(utility.disabled);
        assert!(utility.badge);
    }

    #[test]
    fn test_utility_menu_dropdown_builder() {
        let utility = TopNavigationUtility::menu_dropdown("user")
            .with_text("John Doe")
            .with_description("User menu")
            .with_items(vec!["Profile".to_string(), "Logout".to_string()])
            .with_expandable_groups(true);

        assert_eq!(utility.id, "user");
        assert_eq!(utility.utility_type, UtilityType::MenuDropdown);
        assert_eq!(utility.text, Some("John Doe".to_string()));
        assert_eq!(utility.description, Some("User menu".to_string()));
        assert_eq!(utility.items.len(), 2);
        assert!(utility.expandable_groups);
    }

    #[test]
    fn test_utility_with_href() {
        let utility = TopNavigationUtility::button("link")
            .with_href("/settings")
            .with_external(true)
            .with_target("_blank");

        assert_eq!(utility.href, Some("/settings".to_string()));
        assert!(utility.external);
        assert_eq!(utility.target, Some("_blank".to_string()));
    }

    #[test]
    fn test_utility_click_detail() {
        let detail = UtilityClickDetail {
            id: "test".to_string(),
            external: true,
            href: Some("/test".to_string()),
        };

        assert_eq!(detail.id, "test");
        assert!(detail.external);
        assert_eq!(detail.href, Some("/test".to_string()));
    }

    #[test]
    fn test_utility_follow_detail() {
        let detail = UtilityFollowDetail {
            id: "test".to_string(),
            href: "/test".to_string(),
            external: false,
            target: None,
        };

        assert_eq!(detail.id, "test");
        assert_eq!(detail.href, "/test");
        assert!(!detail.external);
        assert_eq!(detail.target, None);
    }

    #[test]
    fn test_i18n_strings_builder() {
        let i18n = TopNavigationI18nStrings::new()
            .with_search_icon_aria_label("Search")
            .with_overflow_menu_trigger_text("More options");

        assert_eq!(i18n.search_icon_aria_label, Some("Search".to_string()));
        assert_eq!(i18n.overflow_menu_trigger_text, Some("More options".to_string()));
    }

    #[test]
    fn test_identity_follow_detail() {
        let detail = IdentityFollowDetail {
            href: "/home".to_string(),
        };

        assert_eq!(detail.href, "/home");
    }
}
