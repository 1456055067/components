// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Header component for page, container, and section headings.
//!
//! The Header component provides consistent heading styles across different hierarchy levels
//! with support for descriptions, actions, counters, and info links.

use yew::prelude::*;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder,
    AnalyticsMetadata, AriaAttributes,
};

/// Header variant types corresponding to HTML heading levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderVariant {
    /// Page-level header (h1 tag)
    H1,
    /// Container-level header (h2 tag) - default
    H2,
    /// Section-level header (h3 tag)
    H3,
}

impl Default for HeaderVariant {
    fn default() -> Self {
        Self::H2
    }
}

impl HeaderVariant {
    /// Returns the CSS class suffix for this variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
        }
    }

    /// Returns the HTML tag name for this variant
    pub fn tag_name(&self) -> &'static str {
        self.as_str()
    }
}

/// Properties for the Header component
#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Header variant (h1, h2, or h3)
    #[prop_or_default]
    pub variant: HeaderVariant,

    /// Supplementary text below the heading
    #[prop_or_default]
    pub description: Option<String>,

    /// Actions content displayed on the right side
    #[prop_or_default]
    pub actions: Option<Html>,

    /// Counter content displayed next to the heading
    #[prop_or_default]
    pub counter: Option<Html>,

    /// Info link displayed next to the heading
    #[prop_or_default]
    pub info: Option<Html>,

    /// ARIA attributes for accessibility
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// Header title content (children)
    #[prop_or_default]
    pub children: Children,
}

/// Header component for displaying page, container, and section headings.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Header, HeaderVariant};
///
/// // Basic page header
/// html! {
///     <Header variant={HeaderVariant::H1}>
///         { "Dashboard" }
///     </Header>
/// }
///
/// // Container header with description and actions
/// html! {
///     <Header
///         variant={HeaderVariant::H2}
///         description="View and manage your resources"
///         actions={html! { <button>{"Create"}</button> }}
///     >
///         { "Resources" }
///     </Header>
/// }
///
/// // Section header with counter and info link
/// html! {
///     <Header
///         variant={HeaderVariant::H3}
///         counter={html! { <span>{"(5)"}</span> }}
///         info={html! { <a href="#">{"Info"}</a> }}
///     >
///         { "Items" }
///     </Header>
/// }
/// ```
#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let _metadata = ComponentMetadata::new("Header");

    // Build CSS classes for root element
    let root_classes = ClassBuilder::new()
        .add("awsui-header")
        .add(format!("awsui-header-variant-{}", props.variant.as_str()))
        .add_if(props.actions.is_none(), "awsui-header-no-actions")
        .add_if(props.description.is_some(), "awsui-header-has-description");

    // Build CSS classes for main content area
    let main_classes = ClassBuilder::new()
        .add("awsui-header-main")
        .add(format!("awsui-header-main-variant-{}", props.variant.as_str()));

    // Build CSS classes for title area
    let title_classes = ClassBuilder::new()
        .add("awsui-header-title")
        .add(format!("awsui-header-title-variant-{}", props.variant.as_str()));

    // Build CSS classes for heading element
    let heading_classes = ClassBuilder::new()
        .add("awsui-header-heading")
        .add(format!("awsui-header-heading-variant-{}", props.variant.as_str()));

    // Build CSS classes for heading text span
    let heading_text_classes = ClassBuilder::new()
        .add("awsui-header-heading-text")
        .add(format!("awsui-header-heading-text-variant-{}", props.variant.as_str()));

    // Build CSS classes for actions area
    let actions_classes = ClassBuilder::new()
        .add("awsui-header-actions")
        .add(format!("awsui-header-actions-variant-{}", props.variant.as_str()))
        .add("awsui-header-actions-centered");

    // Build CSS classes for description
    let description_classes = ClassBuilder::new()
        .add("awsui-header-description")
        .add(format!("awsui-header-description-variant-{}", props.variant.as_str()));

    // Build analytics metadata
    let analytics = AnalyticsMetadata {
        action: Some("header".to_string()),
        detail: Some(serde_json::Value::String(props.variant.as_str().to_string())),
        component: None,
    };
    let analytics_attr = serde_json::to_string(&analytics).ok();

    // Determine if we need tabindex for h1 variant (for flashbar focus behavior)
    let heading_tab_index = if matches!(props.variant, HeaderVariant::H1) {
        Some("-1")
    } else {
        None
    };

    // Render heading based on variant
    let heading_content = match props.variant {
        HeaderVariant::H1 => html! {
            <h1
                class={heading_classes.build()}
                tabindex={heading_tab_index}
            >
                <span class={heading_text_classes.build()}>
                    { props.children.clone() }
                </span>
                if let Some(ref counter) = props.counter {
                    <span class="awsui-header-counter">
                        { " " }
                        { counter.clone() }
                    </span>
                }
            </h1>
        },
        HeaderVariant::H2 => html! {
            <h2 class={heading_classes.build()}>
                <span class={heading_text_classes.build()}>
                    { props.children.clone() }
                </span>
                if let Some(ref counter) = props.counter {
                    <span class="awsui-header-counter">
                        { " " }
                        { counter.clone() }
                    </span>
                }
            </h2>
        },
        HeaderVariant::H3 => html! {
            <h3 class={heading_classes.build()}>
                <span class={heading_text_classes.build()}>
                    { props.children.clone() }
                </span>
                if let Some(ref counter) = props.counter {
                    <span class="awsui-header-counter">
                        { " " }
                        { counter.clone() }
                    </span>
                }
            </h3>
        },
    };

    html! {
        <div
            id={props.base.id.clone()}
            class={root_classes.build()}
            data-analytics-metadata={analytics_attr}
        >
            <div class={main_classes.build()}>
                <div class={title_classes.build()}>
                    { heading_content }
                    if let Some(ref info) = props.info {
                        <>
                            // Virtual space to prevent double-click selection from including info
                            <span class="awsui-header-virtual-space">
                                { " \u{00A0}" }
                            </span>
                            <span class="awsui-header-info">
                                { info.clone() }
                            </span>
                        </>
                    }
                </div>
                if let Some(ref actions) = props.actions {
                    <div class={actions_classes.build()}>
                        { actions.clone() }
                    </div>
                }
            </div>
            if let Some(ref description) = props.description {
                <p class={description_classes.build()}>
                    { description }
                </p>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_variant_strings() {
        assert_eq!(HeaderVariant::H1.as_str(), "h1");
        assert_eq!(HeaderVariant::H2.as_str(), "h2");
        assert_eq!(HeaderVariant::H3.as_str(), "h3");
    }

    #[test]
    fn header_variant_tag_names() {
        assert_eq!(HeaderVariant::H1.tag_name(), "h1");
        assert_eq!(HeaderVariant::H2.tag_name(), "h2");
        assert_eq!(HeaderVariant::H3.tag_name(), "h3");
    }

    #[test]
    fn header_variant_default() {
        assert_eq!(HeaderVariant::default(), HeaderVariant::H2);
    }

    #[test]
    fn header_variant_equality() {
        assert_eq!(HeaderVariant::H1, HeaderVariant::H1);
        assert_eq!(HeaderVariant::H2, HeaderVariant::H2);
        assert_eq!(HeaderVariant::H3, HeaderVariant::H3);
        assert_ne!(HeaderVariant::H1, HeaderVariant::H2);
        assert_ne!(HeaderVariant::H2, HeaderVariant::H3);
    }

    #[test]
    fn header_variant_clone() {
        let variant = HeaderVariant::H1;
        let cloned = variant;
        assert_eq!(variant, cloned);
    }
}
