// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Button component
//!
//! An interactive button element with multiple variants and states.

use crate::internal::events::FollowDetail;
use crate::internal::styles::ButtonStyle;
use crate::internal::{
    AnalyticsMetadata, AriaAttributes, BaseComponentProps, ClickEvent, ComponentMetadata,
    ComponentStyles, FollowEvent, I18nStrings, NativeAttributes,
};
use crate::spinner::{Spinner, SpinnerSize, SpinnerVariant};
use web_sys::MouseEvent;
use yew::prelude::*;

/// Button variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Normal,
    Primary,
    Link,
    Icon,
    InlineIcon,
    InlineLink,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Normal => "normal",
            ButtonVariant::Primary => "primary",
            ButtonVariant::Link => "link",
            ButtonVariant::Icon => "icon",
            ButtonVariant::InlineIcon => "inline-icon",
            ButtonVariant::InlineLink => "inline-link",
        }
    }

    pub fn is_link_variant(&self) -> bool {
        matches!(self, ButtonVariant::Link | ButtonVariant::InlineLink)
    }
}

/// Button form action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormAction {
    #[default]
    Submit,
    None,
}

impl FormAction {
    fn as_str(&self) -> &'static str {
        match self {
            FormAction::Submit => "submit",
            FormAction::None => "button",
        }
    }
}

/// Icon alignment in button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconAlign {
    #[default]
    Left,
    Right,
}

/// Properties for the Button component
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Button variant
    #[prop_or_default]
    pub variant: ButtonVariant,

    /// Whether the button is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the button is in a loading state
    #[prop_or_default]
    pub loading: bool,

    /// Icon to display in the button
    #[prop_or_default]
    pub icon: Option<Html>,

    /// Icon alignment (left or right)
    #[prop_or_default]
    pub icon_align: IconAlign,

    /// Whether the button should expand to full width
    #[prop_or_default]
    pub full_width: bool,

    /// Whether text should wrap
    #[prop_or_default]
    pub wrap_text: bool,

    /// Reason why the button is disabled (shows as tooltip)
    #[prop_or_default]
    pub disabled_reason: Option<String>,

    /// Click event handler
    #[prop_or_default]
    pub on_click: Option<Callback<ClickEvent>>,

    /// Follow event handler (for links)
    #[prop_or_default]
    pub on_follow: Option<Callback<FollowEvent>>,

    /// Optional href for link-style buttons
    #[prop_or_default]
    pub href: Option<String>,

    /// Target for link buttons
    #[prop_or_default]
    pub target: Option<String>,

    /// Whether link opens in external context
    #[prop_or_default]
    pub external: bool,

    /// Download attribute for links
    #[prop_or_default]
    pub download: Option<String>,

    /// Form action type
    #[prop_or_default]
    pub form_action: FormAction,

    /// ARIA label
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// ARIA expanded state
    #[prop_or_default]
    pub aria_expanded: Option<bool>,

    /// Style overrides
    #[prop_or_default]
    pub style: Option<ButtonStyle>,

    /// Native HTML attributes
    #[prop_or_default]
    pub native_attributes: Option<NativeAttributes>,

    /// I18n strings
    #[prop_or_default]
    pub i18n_strings: Option<I18nStrings>,

    /// Child content
    #[prop_or_default]
    pub children: Children,
}

/// Button component
///
/// An interactive button with various visual styles and states.
///
/// # Example
/// ```rust,ignore
/// use cloudscape_components::{Button, ButtonVariant};
///
/// html! {
///     <Button
///         variant={ButtonVariant::Primary}
///         on_click={callback}
///         loading={loading_state}
///     >
///         {"Submit"}
///     </Button>
/// }
///
/// // With icon and disabled reason:
/// html! {
///     <Button
///         variant={ButtonVariant::Normal}
///         disabled={true}
///         disabled_reason={"Please fill all required fields"}
///         icon={html! { <span>{"🔒"}</span> }}
///     >
///         {"Save"}
///     </Button>
/// }
///
/// // As a link:
/// html! {
///     <Button
///         variant={ButtonVariant::Link}
///         href={"https://example.com"}
///         external={true}
///         on_follow={follow_callback}
///     >
///         {"Learn More"}
///     </Button>
/// }
/// ```
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let _metadata = ComponentMetadata::new("Button");

    // Determine if button is interactive
    let is_disabled = props.disabled || props.loading;
    let is_link = props.href.is_some();

    // Build component styles
    let mut styles = ComponentStyles::new();
    styles.add_class("awsui-button");
    styles.add_class(format!("awsui-button-variant-{}", props.variant.as_str()));

    if is_disabled {
        styles.add_class("awsui-button-disabled");
    }
    if props.loading {
        styles.add_class("awsui-button-loading");
    }
    if props.full_width {
        styles.add_class("awsui-button-full-width");
    }
    if !props.wrap_text {
        styles.add_class("awsui-button-no-wrap");
    }
    if props.icon.is_some() {
        styles.add_class("awsui-button-has-icon");
        match props.icon_align {
            IconAlign::Left => styles.add_class("awsui-button-icon-left"),
            IconAlign::Right => styles.add_class("awsui-button-icon-right"),
        };
    }

    // Apply style overrides if provided
    if let Some(ref button_style) = props.style {
        // TODO: Convert ButtonStyle to StyleOverride
        // For now, this is a placeholder for when we implement full style override support
        let _ = button_style;
    }

    let class = props.base.merge_classes(&styles.class_attr());
    let style_attr = styles.style_attr();

    // Create analytics metadata
    let analytics = AnalyticsMetadata::button("button", props.variant.as_str(), is_disabled);
    let analytics_attr = analytics.to_data_attribute();

    // Build ARIA attributes
    let mut aria = AriaAttributes::default();
    if let Some(ref label) = props.aria_label {
        aria.label = Some(label.clone());
    }
    if let Some(expanded) = props.aria_expanded {
        aria.expanded = Some(expanded);
    }
    if is_disabled {
        aria.disabled = Some(true);
    }

    // Get external icon label from i18n
    let external_icon_label = props
        .i18n_strings
        .as_ref()
        .and_then(|i18n| i18n.get("externalIconAriaLabel")).cloned()
        .unwrap_or_else(|| "(opens in a new tab)".to_string());

    // Build button content
    let button_content = html! {
        <>
            if props.loading {
                <span class="awsui-button-spinner">
                    <Spinner
                        size={SpinnerSize::Normal}
                        variant={SpinnerVariant::Normal}
                    />
                </span>
            }
            if let Some(ref icon) = props.icon {
                if matches!(props.icon_align, IconAlign::Left) && !props.loading {
                    <span class="awsui-button-icon awsui-button-icon-left">
                        { icon.clone() }
                    </span>
                }
            }
            <span class="awsui-button-content">
                { props.children.clone() }
            </span>
            if let Some(ref icon) = props.icon {
                if matches!(props.icon_align, IconAlign::Right) && !props.loading {
                    <span class="awsui-button-icon awsui-button-icon-right">
                        { icon.clone() }
                    </span>
                }
            }
            if props.external && is_link {
                <span class="awsui-button-external-icon" aria-label={external_icon_label}>
                    {"↗"}
                </span>
            }
        </>
    };

    // Create click handler
    let on_click = {
        let click_callback = props.on_click.clone();
        let follow_callback = props.on_follow.clone();
        let href = props.href.clone();
        let external = props.external;
        let target = props.target.clone();

        Callback::from(move |e: MouseEvent| {
            // Fire click event
            if let Some(ref cb) = click_callback {
                let event = ClickEvent::from_mouse_event(&e);
                cb.emit(event);
            }

            // Fire follow event for links
            if let Some(ref cb) = follow_callback {
                let follow_detail = FollowDetail {
                    href: href.clone(),
                    external,
                    target: target.clone(),
                };
                let event = FollowEvent::new(follow_detail);
                cb.emit(event);
            }
        })
    };

    // Render the button element
    let button_element = if let Some(href) = &props.href {
        // Render as anchor
        html! {
            <a
                id={props.base.id.clone()}
                class={class}
                style={style_attr}
                href={href.clone()}
                target={props.target.clone()}
                download={props.download.clone()}
                rel={if props.external { Some("noopener noreferrer") } else { None }}
                onclick={on_click}
                aria-label={aria.label.clone()}
                aria-expanded={aria.expanded.map(|e| e.to_string())}
                aria-disabled={if is_disabled { Some("true") } else { None }}
                data-analytics-metadata={analytics_attr}
            >
                { button_content }
            </a>
        }
    } else {
        // Render as button
        html! {
            <button
                id={props.base.id.clone()}
                class={class}
                style={style_attr}
                type={props.form_action.as_str()}
                disabled={is_disabled}
                onclick={on_click}
                aria-label={aria.label.clone()}
                aria-expanded={aria.expanded.map(|e| e.to_string())}
                data-analytics-metadata={analytics_attr}
            >
                { button_content }
            </button>
        }
    };

    // Wrap with tooltip if disabled reason is provided
    if let Some(ref reason) = props.disabled_reason {
        html! {
            <span class="awsui-button-tooltip-wrapper" title={reason.clone()}>
                { button_element }
            </span>
        }
    } else {
        button_element
    }
}
