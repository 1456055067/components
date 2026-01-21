// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Box component
//!
//! A layout utility component for controlling spacing, typography, and display properties.

use crate::internal::{BaseComponentProps, ClassBuilder};
use yew::prelude::*;

/// Box HTML tag variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoxVariant {
    #[default]
    Div,
    Span,
    H1,
    H2,
    H3,
    H4,
    H5,
    P,
    Strong,
    Code,
    Pre,
    Samp,
    AwsuiKeyLabel,
    AwsuiValueLarge,
}

impl BoxVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BoxVariant::Div => "div",
            BoxVariant::Span => "span",
            BoxVariant::H1 => "h1",
            BoxVariant::H2 => "h2",
            BoxVariant::H3 => "h3",
            BoxVariant::H4 => "h4",
            BoxVariant::H5 => "h5",
            BoxVariant::P => "p",
            BoxVariant::Strong => "strong",
            BoxVariant::Code => "code",
            BoxVariant::Pre => "pre",
            BoxVariant::Samp => "samp",
            BoxVariant::AwsuiKeyLabel => "awsui-key-label",
            BoxVariant::AwsuiValueLarge => "awsui-value-large",
        }
    }
}

/// Spacing configuration (can be uniform or per-side)
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SpacingSize {
    #[default]
    None,
    Xxxs,
    Xxs,
    Xs,
    S,
    M,
    L,
    Xl,
    Xxl,
    Xxxl,
}

impl SpacingSize {
    fn as_str(&self) -> &'static str {
        match self {
            SpacingSize::None => "n",
            SpacingSize::Xxxs => "xxxs",
            SpacingSize::Xxs => "xxs",
            SpacingSize::Xs => "xs",
            SpacingSize::S => "s",
            SpacingSize::M => "m",
            SpacingSize::L => "l",
            SpacingSize::Xl => "xl",
            SpacingSize::Xxl => "xxl",
            SpacingSize::Xxxl => "xxxl",
        }
    }
}

/// Properties for the Box component
#[derive(Properties, PartialEq, Clone)]
pub struct BoxProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// HTML tag variant to render
    #[prop_or_default]
    pub variant: BoxVariant,

    /// Margin size (uniform)
    #[prop_or_default]
    pub margin: SpacingSize,

    /// Padding size (uniform)
    #[prop_or_default]
    pub padding: SpacingSize,

    /// Child content
    #[prop_or_default]
    pub children: Children,
}

/// Box component
///
/// A flexible layout component for controlling spacing and typography.
///
/// # Example
/// ```rust,ignore
/// use cloudscape_components::{Box, BoxVariant, SpacingSize};
///
/// html! {
///     <Box variant={BoxVariant::Div} padding={SpacingSize::M}>
///         {"Content"}
///     </Box>
/// }
/// ```
#[function_component(Box)]
pub fn box_component(props: &BoxProps) -> Html {
    let class = ClassBuilder::new()
        .add("awsui-box")
        .add(format!("awsui-box-variant-{}", props.variant.as_str()))
        .add_if(
            props.margin != SpacingSize::None,
            format!("awsui-box-m-{}", props.margin.as_str()),
        )
        .add_if(
            props.padding != SpacingSize::None,
            format!("awsui-box-p-{}", props.padding.as_str()),
        )
        .build();

    let class = props.base.merge_classes(&class);

    // For now, always render as div. In a full implementation,
    // we'd use @html! macro or VTag to dynamically select the tag
    html! {
        <div
            id={props.base.id.clone()}
            class={class}
        >
            { props.children.clone() }
        </div>
    }
}
