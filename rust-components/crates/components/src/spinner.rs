// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Spinner component
//!
//! A loading indicator that displays an animated spinner.

use crate::internal::{BaseComponentProps, ClassBuilder};
use yew::prelude::*;

/// Spinner size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    #[default]
    Normal,
    Big,
    Large,
}

impl SpinnerSize {
    fn as_str(&self) -> &'static str {
        match self {
            SpinnerSize::Normal => "normal",
            SpinnerSize::Big => "big",
            SpinnerSize::Large => "large",
        }
    }
}

/// Spinner visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerVariant {
    #[default]
    Normal,
    Disabled,
    Inverted,
}

impl SpinnerVariant {
    fn as_str(&self) -> &'static str {
        match self {
            SpinnerVariant::Normal => "normal",
            SpinnerVariant::Disabled => "disabled",
            SpinnerVariant::Inverted => "inverted",
        }
    }
}

/// Properties for the Spinner component
#[derive(Properties, PartialEq, Clone)]
pub struct SpinnerProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Spinner size
    #[prop_or_default]
    pub size: SpinnerSize,

    /// Spinner variant
    #[prop_or_default]
    pub variant: SpinnerVariant,
}

/// Spinner component
///
/// Displays an animated loading spinner.
///
/// # Example
/// ```rust,ignore
/// use cloudscape_components::{Spinner, SpinnerSize};
///
/// html! {
///     <Spinner size={SpinnerSize::Large} />
/// }
/// ```
#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let class = ClassBuilder::new()
        .add("awsui-spinner")
        .add(format!("awsui-spinner-size-{}", props.size.as_str()))
        .add(format!("awsui-spinner-variant-{}", props.variant.as_str()))
        .build();

    let class = props.base.merge_classes(&class);

    html! {
        <span
            id={props.base.id.clone()}
            class={class}
            role="status"
            aria-live="polite"
        >
            <span class="awsui-spinner-rotator">
                <span class="awsui-spinner-circle awsui-spinner-circle-left" />
                <span class="awsui-spinner-circle awsui-spinner-circle-right" />
            </span>
        </span>
    }
}
