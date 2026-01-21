// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Badge component
//!
//! A small visual indicator for labels and metadata.

use crate::internal::styles::BadgeStyle;
use crate::internal::{AnalyticsMetadata, BaseComponentProps, ComponentMetadata, ComponentStyles};
use yew::prelude::*;

/// Badge color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeColor {
    Blue,
    #[default]
    Grey,
    Green,
    Red,
}

impl BadgeColor {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeColor::Blue => "blue",
            BadgeColor::Grey => "grey",
            BadgeColor::Green => "green",
            BadgeColor::Red => "red",
        }
    }
}

/// Properties for the Badge component
#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Badge color variant
    #[prop_or_default]
    pub color: BadgeColor,

    /// Style overrides for customizing badge appearance
    #[prop_or_default]
    pub style: Option<BadgeStyle>,

    /// Child content
    #[prop_or_default]
    pub children: Children,
}

/// Badge component
///
/// Displays a small colored badge with text content.
///
/// # Example
/// ```rust,ignore
/// use cloudscape_components::{Badge, BadgeColor};
/// use cloudscape_components::internal::styles::BadgeStyle;
///
/// html! {
///     <Badge color={BadgeColor::Blue}>{"New"}</Badge>
/// }
///
/// // With style overrides:
/// html! {
///     <Badge
///         color={BadgeColor::Green}
///         style={BadgeStyle {
///             background: Some("#00ff00".to_string()),
///             ..Default::default()
///         }}
///     >
///         {"Custom"}
///     </Badge>
/// }
/// ```
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    // Component metadata for analytics
    let _metadata = ComponentMetadata::new("Badge");

    // Build component styles
    let mut styles = ComponentStyles::new();
    styles.add_class("awsui-badge");
    styles.add_class(&format!("awsui-badge-color-{}", props.color.as_str()));

    // Apply style overrides if provided
    if let Some(ref badge_style) = props.style {
        let override_style = badge_style.to_style_override();
        styles.merge_override(&override_style);
    }

    let class = props.base.merge_classes(&styles.class_attr());
    let style_attr = styles.style_attr();

    // Create analytics metadata
    let analytics = AnalyticsMetadata::badge("badge", props.color.as_str());
    let analytics_attr = analytics.to_data_attribute();

    html! {
        <span
            id={props.base.id.clone()}
            class={class}
            style={style_attr}
            data-analytics-metadata={analytics_attr}
        >
            { props.children.clone() }
        </span>
    }
}
