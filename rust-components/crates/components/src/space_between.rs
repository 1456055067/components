// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! SpaceBetween component
//!
//! A layout utility component for spacing elements with consistent gaps.
//! This component wraps child elements and applies spacing between them
//! according to Cloudscape Design System spacing tokens.

use crate::internal::{BaseComponentProps, ClassBuilder};
use yew::prelude::*;

/// Direction of spacing between elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpaceBetweenDirection {
    /// Vertical spacing (stacked layout)
    #[default]
    Vertical,
    /// Horizontal spacing (inline layout)
    Horizontal,
}

impl SpaceBetweenDirection {
    fn as_str(&self) -> &'static str {
        match self {
            SpaceBetweenDirection::Vertical => "vertical",
            SpaceBetweenDirection::Horizontal => "horizontal",
        }
    }
}

/// Size of the spacing gap between elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpaceBetweenSize {
    /// Extra extra extra small spacing (4px)
    Xxxs,
    /// Extra extra small spacing (8px)
    Xxs,
    /// Extra small spacing (12px)
    Xs,
    /// Small spacing (16px)
    S,
    /// Medium spacing (20px) - default
    #[default]
    M,
    /// Large spacing (24px)
    L,
    /// Extra large spacing (32px)
    Xl,
    /// Extra extra large spacing (40px)
    Xxl,
}

impl SpaceBetweenSize {
    fn as_str(&self) -> &'static str {
        match self {
            SpaceBetweenSize::Xxxs => "xxxs",
            SpaceBetweenSize::Xxs => "xxs",
            SpaceBetweenSize::Xs => "xs",
            SpaceBetweenSize::S => "s",
            SpaceBetweenSize::M => "m",
            SpaceBetweenSize::L => "l",
            SpaceBetweenSize::Xl => "xl",
            SpaceBetweenSize::Xxl => "xxl",
        }
    }
}

/// Alignment of child elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceBetweenAlignment {
    /// Align to start (left for horizontal, top for vertical)
    Start,
    /// Center alignment
    Center,
    /// Align to end (right for horizontal, bottom for vertical)
    End,
}

impl SpaceBetweenAlignment {
    fn as_str(&self) -> &'static str {
        match self {
            SpaceBetweenAlignment::Start => "start",
            SpaceBetweenAlignment::Center => "center",
            SpaceBetweenAlignment::End => "end",
        }
    }
}

/// Properties for the SpaceBetween component
#[derive(Properties, PartialEq, Clone)]
pub struct SpaceBetweenProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Direction of spacing between elements
    #[prop_or_default]
    pub direction: SpaceBetweenDirection,

    /// Size of the spacing gap
    #[prop_or_default]
    pub size: SpaceBetweenSize,

    /// Horizontal alignment of child elements
    #[prop_or_default]
    pub alignment_horizontal: Option<SpaceBetweenAlignment>,

    /// Vertical alignment of child elements
    #[prop_or_default]
    pub alignment_vertical: Option<SpaceBetweenAlignment>,

    /// Child elements to space
    #[prop_or_default]
    pub children: Children,
}

/// SpaceBetween component
///
/// Provides consistent spacing between child elements according to
/// Cloudscape Design System spacing tokens.
///
/// # Examples
///
/// ## Vertical spacing (default)
/// ```rust,ignore
/// use cloudscape_components::{SpaceBetween, SpaceBetweenSize, Button};
///
/// html! {
///     <SpaceBetween size={SpaceBetweenSize::M}>
///         <Button>{"First Button"}</Button>
///         <Button>{"Second Button"}</Button>
///         <Button>{"Third Button"}</Button>
///     </SpaceBetween>
/// }
/// ```
///
/// ## Horizontal spacing with alignment
/// ```rust,ignore
/// use cloudscape_components::{
///     SpaceBetween, SpaceBetweenSize, SpaceBetweenDirection,
///     SpaceBetweenAlignment, Button
/// };
///
/// html! {
///     <SpaceBetween
///         size={SpaceBetweenSize::S}
///         direction={SpaceBetweenDirection::Horizontal}
///         alignment_vertical={Some(SpaceBetweenAlignment::Center)}
///     >
///         <Button>{"Cancel"}</Button>
///         <Button>{"Submit"}</Button>
///     </SpaceBetween>
/// }
/// ```
///
/// ## Custom spacing sizes
/// ```rust,ignore
/// use cloudscape_components::{SpaceBetween, SpaceBetweenSize};
///
/// // Extra small spacing for compact layouts
/// html! {
///     <SpaceBetween size={SpaceBetweenSize::Xs}>
///         <div>{"Item 1"}</div>
///         <div>{"Item 2"}</div>
///     </SpaceBetween>
/// }
///
/// // Extra large spacing for emphasis
/// html! {
///     <SpaceBetween size={SpaceBetweenSize::Xxl}>
///         <section>{"Section 1"}</section>
///         <section>{"Section 2"}</section>
///     </SpaceBetween>
/// }
/// ```
#[function_component(SpaceBetween)]
pub fn space_between(props: &SpaceBetweenProps) -> Html {
    let class = ClassBuilder::new()
        .add("awsui-space-between")
        .add(format!(
            "awsui-space-between-direction-{}",
            props.direction.as_str()
        ))
        .add(format!(
            "awsui-space-between-size-{}",
            props.size.as_str()
        ))
        .add_option(
            props
                .alignment_horizontal
                .map(|align| format!("awsui-space-between-align-horizontal-{}", align.as_str())),
        )
        .add_option(
            props
                .alignment_vertical
                .map(|align| format!("awsui-space-between-align-vertical-{}", align.as_str())),
        )
        .build();

    let class = props.base.merge_classes(&class);

    html! {
        <div
            id={props.base.id.clone()}
            class={class}
        >
            { props.children.clone() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_props() {
        let props = SpaceBetweenProps {
            base: BaseComponentProps::default(),
            direction: SpaceBetweenDirection::default(),
            size: SpaceBetweenSize::default(),
            alignment_horizontal: None,
            alignment_vertical: None,
            children: Children::default(),
        };

        assert_eq!(props.direction, SpaceBetweenDirection::Vertical);
        assert_eq!(props.size, SpaceBetweenSize::M);
    }

    #[test]
    fn test_direction_as_str() {
        assert_eq!(SpaceBetweenDirection::Vertical.as_str(), "vertical");
        assert_eq!(SpaceBetweenDirection::Horizontal.as_str(), "horizontal");
    }

    #[test]
    fn test_size_as_str() {
        assert_eq!(SpaceBetweenSize::Xxxs.as_str(), "xxxs");
        assert_eq!(SpaceBetweenSize::Xxs.as_str(), "xxs");
        assert_eq!(SpaceBetweenSize::Xs.as_str(), "xs");
        assert_eq!(SpaceBetweenSize::S.as_str(), "s");
        assert_eq!(SpaceBetweenSize::M.as_str(), "m");
        assert_eq!(SpaceBetweenSize::L.as_str(), "l");
        assert_eq!(SpaceBetweenSize::Xl.as_str(), "xl");
        assert_eq!(SpaceBetweenSize::Xxl.as_str(), "xxl");
    }

    #[test]
    fn test_alignment_as_str() {
        assert_eq!(SpaceBetweenAlignment::Start.as_str(), "start");
        assert_eq!(SpaceBetweenAlignment::Center.as_str(), "center");
        assert_eq!(SpaceBetweenAlignment::End.as_str(), "end");
    }

    #[test]
    fn test_class_generation_with_defaults() {
        let props = SpaceBetweenProps {
            base: BaseComponentProps::default(),
            direction: SpaceBetweenDirection::Vertical,
            size: SpaceBetweenSize::M,
            alignment_horizontal: None,
            alignment_vertical: None,
            children: Children::default(),
        };

        let class = ClassBuilder::new()
            .add("awsui-space-between")
            .add(format!(
                "awsui-space-between-direction-{}",
                props.direction.as_str()
            ))
            .add(format!(
                "awsui-space-between-size-{}",
                props.size.as_str()
            ))
            .build();

        assert!(class.contains("awsui-space-between"));
        assert!(class.contains("awsui-space-between-direction-vertical"));
        assert!(class.contains("awsui-space-between-size-m"));
    }

    #[test]
    fn test_class_generation_with_horizontal_direction() {
        let props = SpaceBetweenProps {
            base: BaseComponentProps::default(),
            direction: SpaceBetweenDirection::Horizontal,
            size: SpaceBetweenSize::S,
            alignment_horizontal: None,
            alignment_vertical: None,
            children: Children::default(),
        };

        let class = ClassBuilder::new()
            .add("awsui-space-between")
            .add(format!(
                "awsui-space-between-direction-{}",
                props.direction.as_str()
            ))
            .add(format!(
                "awsui-space-between-size-{}",
                props.size.as_str()
            ))
            .build();

        assert!(class.contains("awsui-space-between-direction-horizontal"));
        assert!(class.contains("awsui-space-between-size-s"));
    }

    #[test]
    fn test_class_generation_with_alignments() {
        let props = SpaceBetweenProps {
            base: BaseComponentProps::default(),
            direction: SpaceBetweenDirection::Horizontal,
            size: SpaceBetweenSize::L,
            alignment_horizontal: Some(SpaceBetweenAlignment::Center),
            alignment_vertical: Some(SpaceBetweenAlignment::End),
            children: Children::default(),
        };

        let class = ClassBuilder::new()
            .add("awsui-space-between")
            .add(format!(
                "awsui-space-between-direction-{}",
                props.direction.as_str()
            ))
            .add(format!(
                "awsui-space-between-size-{}",
                props.size.as_str()
            ))
            .add_option(
                props
                    .alignment_horizontal
                    .map(|align| format!("awsui-space-between-align-horizontal-{}", align.as_str())),
            )
            .add_option(
                props
                    .alignment_vertical
                    .map(|align| format!("awsui-space-between-align-vertical-{}", align.as_str())),
            )
            .build();

        assert!(class.contains("awsui-space-between-align-horizontal-center"));
        assert!(class.contains("awsui-space-between-align-vertical-end"));
    }

    #[test]
    fn test_all_size_variants() {
        let sizes = [
            SpaceBetweenSize::Xxxs,
            SpaceBetweenSize::Xxs,
            SpaceBetweenSize::Xs,
            SpaceBetweenSize::S,
            SpaceBetweenSize::M,
            SpaceBetweenSize::L,
            SpaceBetweenSize::Xl,
            SpaceBetweenSize::Xxl,
        ];

        for size in sizes.iter() {
            let class = ClassBuilder::new()
                .add("awsui-space-between")
                .add(format!("awsui-space-between-size-{}", size.as_str()))
                .build();

            assert!(class.contains(&format!("awsui-space-between-size-{}", size.as_str())));
        }
    }

    #[test]
    fn test_base_props_integration() {
        let base = BaseComponentProps {
            id: Some("test-space-between".to_string()),
            class: Some("custom-class".to_string()),
            data_attributes: None,
        };

        let class = base.merge_classes("awsui-space-between awsui-space-between-direction-vertical");

        assert!(class.contains("awsui-space-between"));
        assert!(class.contains("custom-class"));
    }
}
