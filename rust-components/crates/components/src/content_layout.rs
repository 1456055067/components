// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! ContentLayout component for structured page content.
//!
//! The ContentLayout component provides consistent spacing and structure for page
//! content within AppLayout. It manages header, notifications, default actions,
//! and main content areas with proper spacing and optional header overlap effects.

use crate::internal::{BaseComponentProps, ClassBuilder};
use yew::prelude::*;

/// Properties for the ContentLayout component
#[derive(Properties, PartialEq, Clone)]
pub struct ContentLayoutProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Header content for the layout
    ///
    /// Typically contains a Header component with the page title and description.
    /// This content appears at the top of the layout and can overlap with the
    /// background unless `disable_overlap` is set to true.
    #[prop_or_default]
    pub header: Option<Html>,

    /// Default action content
    ///
    /// Positioned in the header area, typically contains a primary Button for
    /// the main page action (e.g., "Create resource"). Appears on the right
    /// side of the header on larger screens.
    #[prop_or_default]
    pub default_action: Option<Html>,

    /// Notifications content
    ///
    /// Typically contains a Flashbar component for displaying notifications,
    /// alerts, and messages. Appears between the header and main content.
    #[prop_or_default]
    pub notifications: Option<Html>,

    /// Disables the header overlap effect
    ///
    /// When false (default), the header has a visual overlap effect with the
    /// background. When true, the header sits flush without overlap styling.
    #[prop_or_default]
    pub disable_overlap: bool,

    /// Main content of the layout
    #[prop_or_default]
    pub children: Children,
}

/// ContentLayout component for structured page content.
///
/// ContentLayout provides consistent spacing and structure for page content within
/// AppLayout. It manages the layout of headers, notifications, actions, and main
/// content with proper spacing and responsive behavior.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{ContentLayout, Header, HeaderVariant, Container, Alert, AlertType};
/// use yew::prelude::*;
///
/// #[function_component(MyPage)]
/// fn my_page() -> Html {
///     html! {
///         <ContentLayout
///             header={html! {
///                 <Header
///                     variant={HeaderVariant::H1}
///                     description="Manage and view your resources"
///                 >
///                     {"Resources"}
///                 </Header>
///             }}
///             notifications={html! {
///                 <Alert alert_type={AlertType::Info}>
///                     {"Your changes have been saved successfully."}
///                 </Alert>
///             }}
///         >
///             <Container>
///                 {"Main page content goes here"}
///             </Container>
///         </ContentLayout>
///     }
/// }
/// ```
///
/// # With Default Action
///
/// ```rust
/// use cloudscape_components::{ContentLayout, Header, HeaderVariant, Button, ButtonVariant};
/// use yew::prelude::*;
///
/// #[function_component(PageWithAction)]
/// fn page_with_action() -> Html {
///     html! {
///         <ContentLayout
///             header={html! {
///                 <Header variant={HeaderVariant::H1}>
///                     {"Dashboard"}
///                 </Header>
///             }}
///             default_action={html! {
///                 <Button variant={ButtonVariant::Primary}>
///                     {"Create resource"}
///                 </Button>
///             }}
///         >
///             <div>{"Dashboard content"}</div>
///         </ContentLayout>
///     }
/// }
/// ```
///
/// # Without Header Overlap
///
/// ```rust
/// use cloudscape_components::{ContentLayout, Header, HeaderVariant};
/// use yew::prelude::*;
///
/// #[function_component(NoOverlapExample)]
/// fn no_overlap_example() -> Html {
///     html! {
///         <ContentLayout
///             header={html! {
///                 <Header variant={HeaderVariant::H1}>
///                     {"Page Title"}
///                 </Header>
///             }}
///             disable_overlap={true}
///         >
///             <div>{"Content without header overlap effect"}</div>
///         </ContentLayout>
///     }
/// }
/// ```
#[function_component(ContentLayout)]
pub fn content_layout(props: &ContentLayoutProps) -> Html {
    // Build CSS classes for the root element
    let root_classes = ClassBuilder::new()
        .add("awsui-content-layout")
        .add_if(props.disable_overlap, "awsui-content-layout-no-overlap")
        .add_if(props.header.is_some(), "awsui-content-layout-has-header")
        .add_if(
            props.default_action.is_some(),
            "awsui-content-layout-has-default-action",
        )
        .add_if(
            props.notifications.is_some(),
            "awsui-content-layout-has-notifications",
        );

    let root_class = props.base.merge_classes(&root_classes.build());

    // Build header section classes
    let header_section_classes = ClassBuilder::new()
        .add("awsui-content-layout-header-section")
        .add_if(
            props.default_action.is_some(),
            "awsui-content-layout-header-section-with-action",
        )
        .build();

    // Build header wrapper classes
    let header_wrapper_classes = ClassBuilder::new()
        .add("awsui-content-layout-header-wrapper")
        .add_if(
            !props.disable_overlap,
            "awsui-content-layout-header-wrapper-overlap",
        )
        .build();

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
        >
            // Header section (header + default action)
            if props.header.is_some() || props.default_action.is_some() {
                <div class={header_section_classes}>
                    <div class={header_wrapper_classes}>
                        // Header content
                        if let Some(ref header) = props.header {
                            <div class="awsui-content-layout-header">
                                { header.clone() }
                            </div>
                        }

                        // Default action
                        if let Some(ref default_action) = props.default_action {
                            <div class="awsui-content-layout-default-action">
                                { default_action.clone() }
                            </div>
                        }
                    </div>
                </div>
            }

            // Notifications section
            if let Some(ref notifications) = props.notifications {
                <div class="awsui-content-layout-notifications">
                    { notifications.clone() }
                </div>
            }

            // Main content area
            <div class="awsui-content-layout-content">
                { props.children.clone() }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_props_default_values() {
        let props = yew::props!(ContentLayoutProps {});
        assert_eq!(props.header, None);
        assert_eq!(props.default_action, None);
        assert_eq!(props.notifications, None);
        assert!(!props.disable_overlap);
        assert!(props.children.is_empty());
    }

    #[test]
    fn test_props_with_disable_overlap() {
        let props = yew::props!(ContentLayoutProps {
            disable_overlap: true,
        });
        assert!(props.disable_overlap);
    }

    #[test]
    fn test_props_with_header() {
        let header = html! { <div>{"Header"}</div> };
        let props = yew::props!(ContentLayoutProps {
            header: Some(header),
        });
        assert!(props.header.is_some());
    }

    #[test]
    fn test_props_with_default_action() {
        let action = html! { <button>{"Action"}</button> };
        let props = yew::props!(ContentLayoutProps {
            default_action: Some(action),
        });
        assert!(props.default_action.is_some());
    }

    #[test]
    fn test_props_with_notifications() {
        let notifications = html! { <div>{"Notification"}</div> };
        let props = yew::props!(ContentLayoutProps {
            notifications: Some(notifications),
        });
        assert!(props.notifications.is_some());
    }

    #[test]
    fn test_base_props_integration() {
        let base = BaseComponentProps {
            id: Some("test-id".to_string()),
            class: Some("custom-class".to_string()),
            data_attributes: None,
        };

        let props = yew::props!(ContentLayoutProps { base: base.clone() });

        assert_eq!(props.base.id, Some("test-id".to_string()));
        assert_eq!(props.base.class, Some("custom-class".to_string()));
    }

    #[test]
    fn test_class_builder_no_overlap() {
        let classes = ClassBuilder::new()
            .add("awsui-content-layout")
            .add_if(true, "awsui-content-layout-no-overlap")
            .build();

        assert!(classes.contains("awsui-content-layout"));
        assert!(classes.contains("awsui-content-layout-no-overlap"));
    }

    #[test]
    fn test_class_builder_with_header() {
        let classes = ClassBuilder::new()
            .add("awsui-content-layout")
            .add_if(true, "awsui-content-layout-has-header")
            .build();

        assert!(classes.contains("awsui-content-layout"));
        assert!(classes.contains("awsui-content-layout-has-header"));
    }

    #[test]
    fn test_class_builder_all_features() {
        let classes = ClassBuilder::new()
            .add("awsui-content-layout")
            .add_if(true, "awsui-content-layout-has-header")
            .add_if(true, "awsui-content-layout-has-default-action")
            .add_if(true, "awsui-content-layout-has-notifications")
            .add_if(false, "awsui-content-layout-no-overlap")
            .build();

        assert!(classes.contains("awsui-content-layout"));
        assert!(classes.contains("awsui-content-layout-has-header"));
        assert!(classes.contains("awsui-content-layout-has-default-action"));
        assert!(classes.contains("awsui-content-layout-has-notifications"));
        assert!(!classes.contains("awsui-content-layout-no-overlap"));
    }

    #[test]
    fn test_header_section_classes() {
        let classes = ClassBuilder::new()
            .add("awsui-content-layout-header-section")
            .add_if(true, "awsui-content-layout-header-section-with-action")
            .build();

        assert!(classes.contains("awsui-content-layout-header-section"));
        assert!(classes.contains("awsui-content-layout-header-section-with-action"));
    }

    #[test]
    fn test_header_wrapper_overlap_classes() {
        let classes_with_overlap = ClassBuilder::new()
            .add("awsui-content-layout-header-wrapper")
            .add_if(true, "awsui-content-layout-header-wrapper-overlap")
            .build();

        assert!(classes_with_overlap.contains("awsui-content-layout-header-wrapper"));
        assert!(classes_with_overlap.contains("awsui-content-layout-header-wrapper-overlap"));

        let classes_without_overlap = ClassBuilder::new()
            .add("awsui-content-layout-header-wrapper")
            .add_if(false, "awsui-content-layout-header-wrapper-overlap")
            .build();

        assert!(classes_without_overlap.contains("awsui-content-layout-header-wrapper"));
        assert!(!classes_without_overlap.contains("awsui-content-layout-header-wrapper-overlap"));
    }
}
