// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Container component for grouping and organizing content.
//!
//! The Container component provides a flexible layout container with support for
//! headers, footers, media elements, and different visual variants. It's designed
//! to organize content sections within your application.

use crate::internal::{BaseComponentProps, ClassBuilder};
use yew::prelude::*;

/// Container visual variants
///
/// Determines the visual style and spacing of the container based on its context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContainerVariant {
    /// Default variant for standalone use
    #[default]
    Default,
    /// Stacked variant for use adjacent to other stacked containers
    Stacked,
}

impl ContainerVariant {
    /// Returns the CSS class name suffix for this variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Stacked => "stacked",
        }
    }
}

/// Media element position within the container
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaPosition {
    /// Media positioned at the top of the container
    #[default]
    Top,
    /// Media positioned on the side of the container
    Side,
}

impl MediaPosition {
    /// Returns the CSS class name suffix for this position
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Side => "side",
        }
    }
}

/// Media element configuration
///
/// Defines how media content (images, videos) should be displayed within the container.
#[derive(Clone, PartialEq)]
pub struct Media {
    /// The media content to render (img, video, picture, iframe elements)
    pub content: Html,

    /// Position of the media element within the container
    #[allow(dead_code)]
    pub position: MediaPosition,

    /// Width of the media slot when positioned on the side (CSS width value)
    ///
    /// Only applies when position is `Side`. If not specified, defaults to
    /// a maximum of 66% of the container's width.
    pub width: Option<String>,

    /// Height of the media slot when positioned on top (CSS height value)
    ///
    /// Only applies when position is `Top`. If not specified, the media
    /// will be displayed at its full height.
    pub height: Option<String>,
}

impl Media {
    /// Creates a new Media configuration with content at the top position
    pub fn new(content: Html) -> Self {
        Self {
            content,
            position: MediaPosition::default(),
            width: None,
            height: None,
        }
    }

    /// Sets the position of the media element
    pub fn with_position(mut self, position: MediaPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets the width of the media element (for side position)
    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the media element (for top position)
    pub fn with_height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }
}

/// Properties for the Container component
#[derive(Properties, PartialEq, Clone)]
pub struct ContainerProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Heading element of the container
    ///
    /// Typically used with a Header component to provide a title and
    /// optional actions for the container.
    #[prop_or_default]
    pub header: Option<Html>,

    /// Footer element of the container
    ///
    /// Used to display supplementary information or actions at the
    /// bottom of the container.
    #[prop_or_default]
    pub footer: Option<Html>,

    /// Container visual variant
    ///
    /// * `Default` - Use for standalone containers
    /// * `Stacked` - Use when placing containers adjacent to each other
    #[prop_or_default]
    pub variant: ContainerVariant,

    /// Media element configuration
    ///
    /// Allows you to add images, videos, or other media content to the
    /// container with configurable positioning and sizing.
    #[prop_or_default]
    pub media: Option<Media>,

    /// Whether to remove default padding from the header
    #[prop_or_default]
    pub disable_header_paddings: bool,

    /// Whether to remove default padding from the content area
    #[prop_or_default]
    pub disable_content_paddings: bool,

    /// Whether to remove default padding from the footer
    #[prop_or_default]
    pub disable_footer_paddings: bool,

    /// Whether the container should fit available height
    ///
    /// When enabled, the container will stretch to fill available vertical
    /// space. If content is too short, the container will stretch; if too
    /// long, it will shrink and show a vertical scrollbar.
    ///
    /// This is useful for aligning heights of multiple containers in a row.
    #[prop_or_default]
    pub fit_height: bool,

    /// Main content of the container
    #[prop_or_default]
    pub children: Children,
}

/// Container component for grouping and organizing content.
///
/// The Container component provides a structured layout with optional header,
/// footer, and media elements. It supports different visual variants and
/// extensive customization options.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Container, ContainerVariant};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     html! {
///         <Container
///             variant={ContainerVariant::Default}
///             header={html! { <h2>{"Container Title"}</h2> }}
///             footer={html! { <div>{"Footer content"}</div> }}
///         >
///             <p>{"Main content goes here"}</p>
///         </Container>
///     }
/// }
/// ```
///
/// # With Media
///
/// ```rust
/// use cloudscape_components::{Container, Media, MediaPosition};
/// use yew::prelude::*;
///
/// #[function_component(MediaExample)]
/// fn media_example() -> Html {
///     let media = Media::new(html! {
///         <img src="example.jpg" alt="Example" />
///     })
///     .with_position(MediaPosition::Side)
///     .with_width("300px");
///
///     html! {
///         <Container media={Some(media)}>
///             <p>{"Content with side media"}</p>
///         </Container>
///     }
/// }
/// ```
#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    // Build CSS classes for the root element
    let root_classes = ClassBuilder::new()
        .add("awsui-container")
        .add(format!(
            "awsui-container-variant-{}",
            props.variant.as_str()
        ))
        .add_if(props.fit_height, "awsui-container-fit-height")
        .add_if(
            props.media.is_some(),
            format!(
                "awsui-container-with-media-{}",
                props
                    .media
                    .as_ref()
                    .map(|m| m.position.as_str())
                    .unwrap_or("top")
            ),
        );

    let root_class = props.base.merge_classes(&root_classes.build());

    // Build header classes
    let header_classes = ClassBuilder::new()
        .add("awsui-container-header")
        .add_if(
            !props.disable_header_paddings,
            "awsui-container-header-with-paddings",
        )
        .add_if(
            props.children.is_empty(),
            "awsui-container-header-no-content",
        )
        .build();

    // Build content classes
    let content_classes = ClassBuilder::new()
        .add("awsui-container-content")
        .add_if(
            !props.disable_content_paddings,
            "awsui-container-content-with-paddings",
        )
        .add_if(
            props.header.is_some(),
            "awsui-container-content-with-header",
        )
        .add_if(props.fit_height, "awsui-container-content-fit-height")
        .build();

    // Build footer classes
    let footer_classes = ClassBuilder::new()
        .add("awsui-container-footer")
        .add_if(
            !props.disable_footer_paddings,
            "awsui-container-footer-with-paddings",
        )
        .build();

    // Build media element with inline styles
    let media_element = props.media.as_ref().map(|media| {
        let media_classes = ClassBuilder::new()
            .add("awsui-container-media")
            .add(format!("awsui-container-media-{}", media.position.as_str()))
            .build();

        // Build inline style for width/height
        let mut style_parts = Vec::new();

        if media.position == MediaPosition::Side {
            if let Some(ref width) = media.width {
                style_parts.push(format!("width: {}", width));
            }
        } else if let Some(ref height) = media.height {
            style_parts.push(format!("height: {}", height));
        }

        let style = if style_parts.is_empty() {
            None
        } else {
            Some(style_parts.join("; "))
        };

        html! {
            <div class={media_classes} style={style}>
                { media.content.clone() }
            </div>
        }
    });

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
        >
            // Media element (if positioned at top or side)
            { media_element }

            // Content wrapper
            <div class="awsui-container-content-wrapper">
                // Header (if provided)
                if let Some(ref header) = props.header {
                    <div class={header_classes}>
                        { header.clone() }
                    </div>
                }

                // Main content
                <div class={content_classes}>
                    { props.children.clone() }
                </div>

                // Footer (if provided)
                if let Some(ref footer) = props.footer {
                    <div class={footer_classes}>
                        { footer.clone() }
                    </div>
                }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_variant_default() {
        assert_eq!(ContainerVariant::default(), ContainerVariant::Default);
    }

    #[test]
    fn test_container_variant_as_str() {
        assert_eq!(ContainerVariant::Default.as_str(), "default");
        assert_eq!(ContainerVariant::Stacked.as_str(), "stacked");
    }

    #[test]
    fn test_media_position_default() {
        assert_eq!(MediaPosition::default(), MediaPosition::Top);
    }

    #[test]
    fn test_media_position_as_str() {
        assert_eq!(MediaPosition::Top.as_str(), "top");
        assert_eq!(MediaPosition::Side.as_str(), "side");
    }

    #[test]
    fn test_media_builder() {
        let content = html! { <img src="test.jpg" /> };
        let media = Media::new(content.clone())
            .with_position(MediaPosition::Side)
            .with_width("300px")
            .with_height("200px");

        assert_eq!(media.position, MediaPosition::Side);
        assert_eq!(media.width, Some("300px".to_string()));
        assert_eq!(media.height, Some("200px".to_string()));
    }

    #[test]
    fn test_media_default_values() {
        let content = html! { <img src="test.jpg" /> };
        let media = Media::new(content);

        assert_eq!(media.position, MediaPosition::Top);
        assert_eq!(media.width, None);
        assert_eq!(media.height, None);
    }

    #[test]
    fn test_media_with_width() {
        let content = html! { <img src="test.jpg" /> };
        let media = Media::new(content).with_width("400px");

        assert_eq!(media.width, Some("400px".to_string()));
    }
}
