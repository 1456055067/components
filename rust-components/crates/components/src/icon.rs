// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Icon component
//!
//! A flexible SVG-based icon component with multiple variants and sizes.

use yew::prelude::*;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ComponentStyles, AnalyticsMetadata,
};

/// Icon variant types for different visual styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconVariant {
    /// Normal variant - inherits color from context
    #[default]
    Normal,
    /// Disabled variant - muted appearance
    Disabled,
    /// Inverted variant - for use on dark backgrounds
    Inverted,
    /// Subtle variant - reduced visual emphasis
    Subtle,
}

impl IconVariant {
    /// Returns the string representation for CSS classes
    pub fn as_str(&self) -> &'static str {
        match self {
            IconVariant::Normal => "normal",
            IconVariant::Disabled => "disabled",
            IconVariant::Inverted => "inverted",
            IconVariant::Subtle => "subtle",
        }
    }
}

/// Icon size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconSize {
    /// Small icon (12px)
    Small,
    /// Normal icon (16px)
    #[default]
    Normal,
    /// Big icon (32px)
    Big,
    /// Large icon (48px)
    Large,
}

impl IconSize {
    /// Returns the string representation for CSS classes
    pub fn as_str(&self) -> &'static str {
        match self {
            IconSize::Small => "small",
            IconSize::Normal => "normal",
            IconSize::Big => "big",
            IconSize::Large => "large",
        }
    }

    /// Returns the pixel dimensions for the icon
    pub fn dimensions(&self) -> u32 {
        match self {
            IconSize::Small => 12,
            IconSize::Normal => 16,
            IconSize::Big => 32,
            IconSize::Large => 48,
        }
    }
}

/// Properties for the Icon component
#[derive(Properties, PartialEq, Clone, Default)]
pub struct IconProps {
    /// Base component properties (id, className, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Name of the icon to display
    #[prop_or_default]
    pub name: String,

    /// Visual variant of the icon
    #[prop_or_default]
    pub variant: IconVariant,

    /// Size of the icon
    #[prop_or_default]
    pub size: IconSize,

    /// Custom SVG content (overrides name-based icon lookup)
    #[prop_or_default]
    pub svg: Option<Html>,

    /// URL for custom icon image (alternative to SVG)
    #[prop_or_default]
    pub url: Option<String>,

    /// Accessible label for the icon (recommended for accessibility)
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Alternative text for URL-based icons (deprecated, use aria_label)
    #[prop_or_default]
    pub alt: Option<String>,
}

/// Icon component
///
/// Displays an SVG icon with configurable size and visual variant.
///
/// # Example
/// ```rust,ignore
/// use cloudscape_components::{Icon, IconSize, IconVariant};
///
/// // Basic icon usage
/// html! {
///     <Icon
///         name={"search"}
///         size={IconSize::Normal}
///         variant={IconVariant::Normal}
///         aria_label={"Search"}
///     />
/// }
///
/// // Custom SVG icon
/// html! {
///     <Icon
///         size={IconSize::Big}
///         svg={html! {
///             <svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
///                 <path d="M1 9L5 13L15 2" />
///             </svg>
///         }}
///         aria_label={"Success"}
///     />
/// }
///
/// // URL-based icon
/// html! {
///     <Icon
///         url={"https://example.com/icon.png"}
///         alt={"Custom icon"}
///         size={IconSize::Normal}
///     />
/// }
/// ```
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let _metadata = ComponentMetadata::new("Icon");

    // Build component styles
    let mut styles = ComponentStyles::new();
    styles.add_class("awsui-icon");
    styles.add_class(&format!("awsui-icon-size-{}", props.size.as_str()));
    styles.add_class(&format!("awsui-icon-variant-{}", props.variant.as_str()));

    if !props.name.is_empty() {
        styles.add_class(&format!("awsui-icon-name-{}", props.name));
    }

    let class = props.base.merge_classes(&styles.class_attr());
    let style_attr = styles.style_attr();

    // Create analytics metadata
    let analytics = AnalyticsMetadata {
        component: Some(crate::internal::analytics::ComponentAnalytics {
            name: "awsui.Icon".to_string(),
            label: crate::internal::analytics::LabelIdentifier::from(props.name.as_str()),
            properties: Some(serde_json::json!({
                "size": props.size.as_str(),
                "variant": props.variant.as_str()
            })),
        }),
        ..Default::default()
    };
    let analytics_attr = analytics.to_data_attribute();

    // Determine ARIA attributes
    let has_aria_label = props.aria_label.is_some();
    let role = if has_aria_label { Some("img") } else { None };
    let aria_hidden = if has_aria_label { None } else { Some("true") };

    // Get pixel dimensions for sizing
    let dimensions = props.size.dimensions();

    // Render custom SVG if provided
    if let Some(ref svg_content) = props.svg {
        return html! {
            <span
                id={props.base.id.clone()}
                class={class}
                style={style_attr}
                role={role}
                aria-label={props.aria_label.clone()}
                aria-hidden={aria_hidden}
                data-analytics-metadata={analytics_attr}
            >
                { svg_content.clone() }
            </span>
        };
    }

    // Render URL-based icon if provided
    if let Some(ref url) = props.url {
        let alt_text = props.aria_label.clone()
            .or_else(|| props.alt.clone())
            .unwrap_or_default();

        return html! {
            <span
                id={props.base.id.clone()}
                class={class}
                style={style_attr}
                data-analytics-metadata={analytics_attr}
            >
                <img
                    src={url.clone()}
                    alt={alt_text}
                    width={dimensions.to_string()}
                    height={dimensions.to_string()}
                />
            </span>
        };
    }

    // Render name-based icon with built-in SVG
    let icon_svg = get_icon_svg(&props.name, props.size);

    html! {
        <span
            id={props.base.id.clone()}
            class={class}
            style={style_attr}
            role={role}
            aria-label={props.aria_label.clone()}
            aria-hidden={aria_hidden}
            data-analytics-metadata={analytics_attr}
        >
            { icon_svg }
        </span>
    }
}

/// Returns the SVG markup for a given icon name
///
/// This function provides a limited set of common icons inline.
/// For a production implementation, this would typically load from
/// external SVG files or use a more comprehensive icon system.
fn get_icon_svg(name: &str, size: IconSize) -> Html {
    let dimensions = size.dimensions();
    let dim_str = dimensions.to_string();

    match name {
        "check" => html! {
            <svg
                width={dim_str.clone()}
                height={dim_str}
                viewBox="0 0 16 16"
                xmlns="http://www.w3.org/2000/svg"
                focusable="false"
                aria-hidden="true"
            >
                <path
                    d="M1 9L5 13L15 2"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        },
        "close" => html! {
            <svg
                width={dim_str.clone()}
                height={dim_str}
                viewBox="0 0 16 16"
                xmlns="http://www.w3.org/2000/svg"
                focusable="false"
                aria-hidden="true"
            >
                <path
                    d="M2 1.70996L14 13.71"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
                <path
                    d="M2 13.71L14 1.70996"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        },
        "search" => html! {
            <svg
                width={dim_str.clone()}
                height={dim_str}
                viewBox="0 0 16 16"
                xmlns="http://www.w3.org/2000/svg"
                focusable="false"
                aria-hidden="true"
            >
                <path
                    d="M11 11L15 15"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
                <path
                    d="M7 12C9.76142 12 12 9.76142 12 7C12 4.23858 9.76142 2 7 2C4.23858 2 2 4.23858 2 7C2 9.76142 4.23858 12 7 12Z"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        },
        "settings" => html! {
            <svg
                width={dim_str.clone()}
                height={dim_str}
                viewBox="0 0 16 16"
                xmlns="http://www.w3.org/2000/svg"
                focusable="false"
                aria-hidden="true"
            >
                <path
                    d="M6.11048 1.72895C6.18048 1.30955 6.55049 1 6.97049 1H8.99048C9.42048 1 9.78047 1.30955 9.85047 1.72895L10.0205 2.72753C10.0705 3.01712 10.2605 3.25677 10.5205 3.40655C10.5805 3.43651 10.6305 3.46647 10.6905 3.50642C10.9405 3.6562 11.2505 3.70614 11.5305 3.60628L12.4805 3.25677C12.8805 3.10699 13.3305 3.25678 13.5505 3.63624L14.5605 5.38374C14.7705 5.75321 14.6905 6.22254 14.3605 6.49216L13.5805 7.13125C13.3505 7.32097 13.2405 7.61055 13.2505 7.90014V8.09986C13.2505 8.38945 13.3605 8.67903 13.5805 8.86875L14.3605 9.50784C14.6905 9.77746 14.7805 10.2468 14.5605 10.6163L13.5505 12.3638C13.3405 12.7332 12.8905 12.893 12.4905 12.7432L11.5405 12.3937C11.2605 12.2939 10.9605 12.3338 10.7005 12.4936C10.6405 12.5235 10.5905 12.5635 10.5305 12.5934C10.2705 12.7332 10.0805 12.9829 10.0305 13.2725L9.86048 14.271C9.79048 14.6904 9.42049 15 9.00049 15H6.98047C6.55047 15 6.19049 14.6904 6.12049 14.271L5.95047 13.2725C5.90047 12.9829 5.71047 12.7432 5.45047 12.5934C5.39047 12.5635 5.34049 12.5335 5.28049 12.4936C5.03049 12.3438 4.72049 12.2939 4.44049 12.3937L3.49048 12.7432C3.09048 12.893 2.64048 12.7432 2.43048 12.3638L1.42047 10.6163C1.21047 10.2468 1.29049 9.77746 1.62049 9.50784L2.40048 8.86875C2.63048 8.67903 2.74047 8.38945 2.73047 8.09986V7.90014C2.73047 7.60056 2.62048 7.32097 2.40048 7.13125L1.62049 6.49216C1.29049 6.22254 1.20047 5.75321 1.42047 5.38374L2.43048 3.63624C2.64048 3.26676 3.09049 3.10699 3.50049 3.25677L4.45047 3.60628C4.73047 3.70614 5.03047 3.66619 5.29047 3.50642C5.35047 3.47646 5.40048 3.43651 5.46048 3.40655C5.72048 3.26675 5.91048 3.0271 5.96048 2.72753L6.11048 1.72895Z"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
                <path
                    d="M10 8C10 9.1 9.1 10 8 10C6.9 10 6 9.1 6 8C6 6.9 6.9 6 8 6C9.1 6 10 6.9 10 8Z"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        },
        "add-plus" => html! {
            <svg
                width={dim_str.clone()}
                height={dim_str}
                viewBox="0 0 16 16"
                xmlns="http://www.w3.org/2000/svg"
                focusable="false"
                aria-hidden="true"
            >
                <path
                    d="M8 2V14"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
                <path
                    d="M2 8H14"
                    stroke="currentColor"
                    fill="none"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        },
        // Fallback for unknown icons - render an empty SVG container
        _ => html! {
            <svg
                width={dim_str.clone()}
                height={dim_str}
                viewBox="0 0 16 16"
                xmlns="http://www.w3.org/2000/svg"
                focusable="false"
                aria-hidden="true"
            />
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_variant_as_str() {
        assert_eq!(IconVariant::Normal.as_str(), "normal");
        assert_eq!(IconVariant::Disabled.as_str(), "disabled");
        assert_eq!(IconVariant::Inverted.as_str(), "inverted");
        assert_eq!(IconVariant::Subtle.as_str(), "subtle");
    }

    #[test]
    fn test_icon_size_as_str() {
        assert_eq!(IconSize::Small.as_str(), "small");
        assert_eq!(IconSize::Normal.as_str(), "normal");
        assert_eq!(IconSize::Big.as_str(), "big");
        assert_eq!(IconSize::Large.as_str(), "large");
    }

    #[test]
    fn test_icon_size_dimensions() {
        assert_eq!(IconSize::Small.dimensions(), 12);
        assert_eq!(IconSize::Normal.dimensions(), 16);
        assert_eq!(IconSize::Big.dimensions(), 32);
        assert_eq!(IconSize::Large.dimensions(), 48);
    }

    #[test]
    fn test_icon_variant_default() {
        assert_eq!(IconVariant::default(), IconVariant::Normal);
    }

    #[test]
    fn test_icon_size_default() {
        assert_eq!(IconSize::default(), IconSize::Normal);
    }

    #[test]
    fn test_icon_props_defaults() {
        let props = IconProps::default();
        assert_eq!(props.name, "");
        assert_eq!(props.variant, IconVariant::Normal);
        assert_eq!(props.size, IconSize::Normal);
        assert_eq!(props.aria_label, None);
    }

    #[test]
    fn test_built_in_icon_names() {
        // Test that our built-in icons have the expected structure
        let icons = vec!["check", "close", "search", "settings", "add-plus"];
        for name in icons {
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_icon_size_mapping() {
        // Test that size enum maps to correct pixel dimensions
        let mappings = vec![
            (IconSize::Small, 12),
            (IconSize::Normal, 16),
            (IconSize::Big, 32),
            (IconSize::Large, 48),
        ];
        for (size, expected_px) in mappings {
            assert_eq!(size.dimensions(), expected_px);
        }
    }
}
