// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Link component for navigation and external references.
//!
//! Links allow users to navigate to different locations, either within the application
//! or to external resources. They support different visual variants and can display
//! an external icon for links that open in new windows or tabs.

use crate::internal::{
    AnalyticsMetadata, BaseComponentProps, ClassBuilder, ClickDetail, ClickEvent,
    ComponentMetadata, CustomEvent,
};
use web_sys::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

/// Link variant styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkVariant {
    /// Primary variant - bold styling for sufficient contrast (default in some contexts)
    Primary,
    /// Secondary variant - minimal styling, underline on hover (default)
    #[default]
    Secondary,
    /// Info variant - used for info links that open help panels
    Info,
    /// Value large variant - for displaying large values as links
    ValueLarge,
}

impl LinkVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkVariant::Primary => "primary",
            LinkVariant::Secondary => "secondary",
            LinkVariant::Info => "info",
            LinkVariant::ValueLarge => "awsui-value-large",
        }
    }
}

/// Font size options for links
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkFontSize {
    BodyS,
    #[default]
    BodyM,
    HeadingXs,
    HeadingS,
    HeadingM,
    HeadingL,
    HeadingXl,
    DisplayL,
    Inherit,
}

impl LinkFontSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkFontSize::BodyS => "body-s",
            LinkFontSize::BodyM => "body-m",
            LinkFontSize::HeadingXs => "heading-xs",
            LinkFontSize::HeadingS => "heading-s",
            LinkFontSize::HeadingM => "heading-m",
            LinkFontSize::HeadingL => "heading-l",
            LinkFontSize::HeadingXl => "heading-xl",
            LinkFontSize::DisplayL => "display-l",
            LinkFontSize::Inherit => "inherit",
        }
    }
}

/// Color options for links
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkColor {
    /// Normal color (default)
    #[default]
    Normal,
    /// Inverted color (for use in Flashbars and dark backgrounds)
    Inverted,
}

impl LinkColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkColor::Normal => "normal",
            LinkColor::Inverted => "inverted",
        }
    }
}

/// Event detail for follow events
///
/// Contains information about the navigation action when a link is followed.
#[derive(Debug, Clone, PartialEq)]
pub struct FollowDetail {
    /// The URL being navigated to
    pub href: Option<String>,
    /// Whether the link is external
    pub external: bool,
    /// The target attribute value
    pub target: Option<String>,
}

/// Follow event type
pub type FollowEvent = CustomEvent<FollowDetail>;

/// Properties for the Link component
#[derive(Properties, PartialEq, Clone)]
pub struct LinkProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Link variant style
    ///
    /// - `Primary`: Bold styling for sufficient contrast with surrounding text.
    ///   Use for links where context doesn't imply interactivity.
    /// - `Secondary`: Minimal styling with underline on hover. Use when
    ///   interactivity is strongly implied by context (tables, lists).
    /// - `Info`: For info links that open help panels.
    /// - `ValueLarge`: For displaying large values as links.
    #[prop_or_default]
    pub variant: LinkVariant,

    /// Font size and line height
    ///
    /// This property is overridden if the variant is `Info`.
    #[prop_or_default]
    pub font_size: LinkFontSize,

    /// Text color of the link and its icon
    ///
    /// - `Normal`: Use in most cases where a link is required.
    /// - `Inverted`: Use to style links inside Flashbars.
    ///
    /// This property is overridden if the variant is `Info`.
    #[prop_or_default]
    pub color: LinkColor,

    /// The URL that the link points to
    ///
    /// If an `href` is not provided, the component will render using a
    /// "button" role and `target` will not be used.
    #[prop_or_default]
    pub href: Option<String>,

    /// Marks the link as external by adding an icon after the text
    ///
    /// If `href` is provided, opens the link in a new tab when clicked.
    #[prop_or_default]
    pub external: bool,

    /// Specifies where to open the linked URL
    ///
    /// Set this to `_blank` to open the URL in a new tab. If you set this
    /// property to `_blank`, the component automatically adds
    /// `rel="noopener noreferrer"` to avoid performance and security issues.
    #[prop_or_default]
    pub target: Option<String>,

    /// The rel attribute for the link
    ///
    /// If provided, this overrides the default behavior. By default, the
    /// component sets `rel="noopener noreferrer"` when `external` is `true`
    /// or `target` is `"_blank"`.
    #[prop_or_default]
    pub rel: Option<String>,

    /// ARIA label for the link
    ///
    /// Use this when there's no visible label or to provide additional context.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// ARIA label for the external icon
    ///
    /// Defaults to "(opens in a new tab)" if not provided.
    #[prop_or_default]
    pub external_icon_aria_label: Option<String>,

    /// Callback fired when a link is clicked without modifier keys
    ///
    /// If the link has no `href` provided, it will be called on all clicks.
    /// Use this event and prevent default browser navigation (by calling
    /// `prevent_default`) to implement client-side routing.
    #[prop_or_default]
    pub on_follow: Option<Callback<FollowEvent>>,

    /// Callback fired when the user clicks on the link
    ///
    /// Do not use this handler for navigation, use the `on_follow` event instead.
    /// This event is non-cancelable to prevent blocking full page reload.
    #[prop_or_default]
    pub on_click: Option<Callback<ClickEvent>>,

    /// Link content (children)
    #[prop_or_default]
    pub children: Children,
}

/// Link component for navigation and external references.
///
/// # Examples
///
/// ## Basic link
///
/// ```rust
/// use cloudscape_components::{Link, LinkVariant};
///
/// html! {
///     <Link href="https://example.com" variant={LinkVariant::Primary}>
///         { "Learn more" }
///     </Link>
/// }
/// ```
///
/// ## External link
///
/// ```rust
/// use cloudscape_components::{Link, LinkVariant};
///
/// html! {
///     <Link
///         href="https://docs.example.com"
///         external={true}
///         variant={LinkVariant::Primary}
///     >
///         { "Documentation" }
///     </Link>
/// }
/// ```
///
/// ## Button-style link (no href)
///
/// ```rust
/// use cloudscape_components::{Link, FollowEvent};
/// use yew::prelude::*;
///
/// let on_follow = Callback::from(|event: FollowEvent| {
///     // Handle navigation programmatically
///     log::info!("Link clicked");
/// });
///
/// html! {
///     <Link on_follow={on_follow}>
///         { "Click me" }
///     </Link>
/// }
/// ```
///
/// ## Client-side routing
///
/// ```rust
/// use cloudscape_components::{Link, FollowEvent};
/// use yew::prelude::*;
///
/// let on_follow = Callback::from(|mut event: FollowEvent| {
///     event.prevent_default(); // Prevent default navigation
///     // Implement your routing logic here
///     if let Some(href) = &event.detail.href {
///         // Navigate using your router
///     }
/// });
///
/// html! {
///     <Link href="/dashboard" on_follow={on_follow}>
///         { "Dashboard" }
///     </Link>
/// }
/// ```
#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let _metadata = ComponentMetadata::new("Link");

    // Determine if this is a button or anchor
    let is_button = props.href.is_none();

    // Determine the actual target
    let actual_target = if props.external && props.target.is_none() {
        Some("_blank".to_string())
    } else {
        props.target.clone()
    };

    // Determine the rel attribute
    let actual_rel = if let Some(ref rel) = props.rel {
        Some(rel.clone())
    } else if props.external || actual_target.as_deref() == Some("_blank") {
        Some("noopener noreferrer".to_string())
    } else {
        None
    };

    // Build CSS classes
    let classes = ClassBuilder::new()
        .add("awsui-link")
        .add_if(is_button, "awsui-link-button")
        .add(format!("awsui-link-variant-{}", props.variant.as_str()))
        .add(get_font_size_class(props.variant, props.font_size))
        .add(get_color_class(props.variant, props.color));

    let class = props.base.merge_classes(&classes.build());

    // Build analytics metadata
    let analytics = AnalyticsMetadata {
        action: Some("click".to_string()),
        detail: Some(serde_json::json!({
            "label": { "root": "self" },
            "external": props.external.to_string(),
            "href": props.href.as_ref().unwrap_or(&"".to_string()),
        })),
        component: Some(crate::internal::analytics::ComponentAnalytics {
            name: "awsui.Link".to_string(),
            label: crate::internal::analytics::LabelIdentifier::Root,
            properties: Some(serde_json::json!({
                "variant": props.variant.as_str(),
            })),
        }),
    };
    let analytics_attr = analytics.to_data_attribute();

    // Get external icon label
    let external_icon_label = props
        .external_icon_aria_label
        .clone()
        .unwrap_or_else(|| "(opens in a new tab)".to_string());

    // Create the link content
    let content = html! {
        <>
            { props.children.clone() }
            if props.external {
                <span class="awsui-link-icon-wrapper">
                    { " " }
                    <span
                        class="awsui-link-icon awsui-icon awsui-icon-external"
                        aria-label={external_icon_label}
                        role="img"
                    >
                        { "↗" }
                    </span>
                </span>
            }
        </>
    };

    // Create event handlers
    let on_click_handler = create_click_handler(
        props.on_click.clone(),
        props.on_follow.clone(),
        props.href.clone(),
        props.external,
        actual_target.clone(),
        is_button,
    );

    let on_keydown_handler = if is_button {
        Some(create_keydown_handler(
            props.on_click.clone(),
            props.on_follow.clone(),
            props.href.clone(),
            props.external,
            actual_target.clone(),
        ))
    } else {
        None
    };

    // Render as button or anchor
    if is_button {
        html! {
            <a
                id={props.base.id.clone()}
                class={class}
                role="button"
                tabindex="0"
                aria-label={props.aria_label.clone()}
                onclick={on_click_handler}
                onkeydown={on_keydown_handler.unwrap()}
                data-analytics-metadata={analytics_attr}
            >
                { content }
            </a>
        }
    } else {
        html! {
            <a
                id={props.base.id.clone()}
                class={class}
                href={props.href.clone()}
                target={actual_target}
                rel={actual_rel}
                aria-label={props.aria_label.clone()}
                onclick={on_click_handler}
                data-analytics-metadata={analytics_attr}
            >
                { content }
            </a>
        }
    }
}

/// Gets the font size CSS class based on variant and font size
fn get_font_size_class(variant: LinkVariant, font_size: LinkFontSize) -> String {
    match variant {
        LinkVariant::Info => "awsui-link-font-size-body-s".to_string(),
        LinkVariant::ValueLarge => "awsui-link-font-size-display-l".to_string(),
        _ => format!("awsui-link-font-size-{}", font_size.as_str()),
    }
}

/// Gets the color CSS class based on variant and color
fn get_color_class(variant: LinkVariant, color: LinkColor) -> String {
    let actual_color = if variant == LinkVariant::Info {
        "normal"
    } else {
        color.as_str()
    };
    format!("awsui-link-color-{}", actual_color)
}

/// Checks if a click is a plain left click without modifiers
fn is_plain_left_click(event: &MouseEvent) -> bool {
    event.button() == 0
        && !event.ctrl_key()
        && !event.shift_key()
        && !event.alt_key()
        && !event.meta_key()
}

/// Creates the click handler for the link
fn create_click_handler(
    on_click: Option<Callback<ClickEvent>>,
    on_follow: Option<Callback<FollowEvent>>,
    href: Option<String>,
    external: bool,
    target: Option<String>,
    is_button: bool,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        // Fire click event (non-cancelable)
        if let Some(ref cb) = on_click {
            let detail = ClickDetail {
                button: e.button() as u16,
                ctrl_key: e.ctrl_key(),
                shift_key: e.shift_key(),
                alt_key: e.alt_key(),
                meta_key: e.meta_key(),
            };
            cb.emit(CustomEvent::new_non_cancelable(detail));
        }

        // Fire follow event for plain left clicks (or all clicks if button)
        if (is_button || is_plain_left_click(&e))
            && let Some(ref cb) = on_follow
        {
            let follow_detail = FollowDetail {
                href: href.clone(),
                external,
                target: target.clone(),
            };
            let event = FollowEvent::new(follow_detail);
            cb.emit(event.clone());

            // Prevent default navigation if prevented by callback
            if event.default_prevented {
                e.prevent_default();
            }
        }
    })
}

/// Creates the keydown handler for button-style links
fn create_keydown_handler(
    on_click: Option<Callback<ClickEvent>>,
    on_follow: Option<Callback<FollowEvent>>,
    href: Option<String>,
    external: bool,
    target: Option<String>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        let key = e.key();

        // Handle Space and Enter keys
        if key == " " || key == "Enter" {
            e.prevent_default();

            // Fire click event (non-cancelable)
            if let Some(ref cb) = on_click {
                let detail = ClickDetail {
                    button: 0,
                    ctrl_key: e.ctrl_key(),
                    shift_key: e.shift_key(),
                    alt_key: e.alt_key(),
                    meta_key: e.meta_key(),
                };
                cb.emit(CustomEvent::new_non_cancelable(detail));
            }

            // Fire follow event
            if let Some(ref cb) = on_follow {
                let follow_detail = FollowDetail {
                    href: href.clone(),
                    external,
                    target: target.clone(),
                };
                cb.emit(FollowEvent::new(follow_detail));
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_variant_as_str() {
        assert_eq!(LinkVariant::Primary.as_str(), "primary");
        assert_eq!(LinkVariant::Secondary.as_str(), "secondary");
        assert_eq!(LinkVariant::Info.as_str(), "info");
        assert_eq!(LinkVariant::ValueLarge.as_str(), "awsui-value-large");
    }

    #[test]
    fn test_link_variant_default() {
        assert_eq!(LinkVariant::default(), LinkVariant::Secondary);
    }

    #[test]
    fn test_link_font_size_as_str() {
        assert_eq!(LinkFontSize::BodyS.as_str(), "body-s");
        assert_eq!(LinkFontSize::BodyM.as_str(), "body-m");
        assert_eq!(LinkFontSize::HeadingXs.as_str(), "heading-xs");
        assert_eq!(LinkFontSize::DisplayL.as_str(), "display-l");
        assert_eq!(LinkFontSize::Inherit.as_str(), "inherit");
    }

    #[test]
    fn test_link_color_as_str() {
        assert_eq!(LinkColor::Normal.as_str(), "normal");
        assert_eq!(LinkColor::Inverted.as_str(), "inverted");
    }

    #[test]
    fn test_get_font_size_class_info_variant() {
        let result = get_font_size_class(LinkVariant::Info, LinkFontSize::BodyM);
        assert_eq!(result, "awsui-link-font-size-body-s");
    }

    #[test]
    fn test_get_font_size_class_value_large_variant() {
        let result = get_font_size_class(LinkVariant::ValueLarge, LinkFontSize::BodyM);
        assert_eq!(result, "awsui-link-font-size-display-l");
    }

    #[test]
    fn test_get_font_size_class_normal_variant() {
        let result = get_font_size_class(LinkVariant::Primary, LinkFontSize::HeadingL);
        assert_eq!(result, "awsui-link-font-size-heading-l");
    }

    #[test]
    fn test_get_color_class_info_variant() {
        let result = get_color_class(LinkVariant::Info, LinkColor::Inverted);
        // Info variant always uses normal color
        assert_eq!(result, "awsui-link-color-normal");
    }

    #[test]
    fn test_get_color_class_normal_variant() {
        let result = get_color_class(LinkVariant::Primary, LinkColor::Inverted);
        assert_eq!(result, "awsui-link-color-inverted");
    }

    #[test]
    fn test_follow_detail_equality() {
        let detail1 = FollowDetail {
            href: Some("https://example.com".to_string()),
            external: true,
            target: Some("_blank".to_string()),
        };

        let detail2 = FollowDetail {
            href: Some("https://example.com".to_string()),
            external: true,
            target: Some("_blank".to_string()),
        };

        assert_eq!(detail1, detail2);
    }

    #[test]
    fn test_follow_detail_inequality() {
        let detail1 = FollowDetail {
            href: Some("https://example.com".to_string()),
            external: true,
            target: None,
        };

        let detail2 = FollowDetail {
            href: Some("https://example.com".to_string()),
            external: false,
            target: None,
        };

        assert_ne!(detail1, detail2);
    }
}
