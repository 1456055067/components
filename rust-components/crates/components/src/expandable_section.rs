// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! ExpandableSection component for collapsible content sections.
//!
//! The ExpandableSection component provides a collapsible content area with a header
//! that users can click to expand or collapse the section. It supports both controlled
//! and uncontrolled modes, multiple visual variants, and optional header actions.

use yew::prelude::*;
use web_sys::{KeyboardEvent, MouseEvent};
use crate::internal::{
    BaseComponentProps, ClassBuilder, CustomEvent, AriaAttributes,
};

/// Visual variants for the expandable section
///
/// Determines the visual style and spacing of the expandable section based on context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpandableSectionVariant {
    /// Default variant for general use
    Default,
    /// Container variant for use alongside other containers
    Container,
    /// Footer variant for use in container footers
    Footer,
    /// Navigation variant for use in navigation panels
    Navigation,
}

impl Default for ExpandableSectionVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl ExpandableSectionVariant {
    /// Returns the CSS class name suffix for this variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Container => "container",
            Self::Footer => "footer",
            Self::Navigation => "navigation",
        }
    }
}

/// Event detail for expand/collapse state changes
#[derive(Clone, PartialEq, Debug)]
pub struct ExpandableSectionChangeDetail {
    /// Whether the section is now expanded
    pub expanded: bool,
}

/// Properties for the ExpandableSection component
#[derive(Properties, PartialEq, Clone)]
pub struct ExpandableSectionProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Simple header text (deprecated in favor of header_text)
    #[prop_or_default]
    pub header: Option<String>,

    /// Header content (can be text or HTML)
    #[prop_or_default]
    pub header_text: Option<Html>,

    /// Description text displayed below the header
    #[prop_or_default]
    pub header_description: Option<Html>,

    /// Actions slot displayed in the header
    #[prop_or_default]
    pub header_actions: Option<Html>,

    /// ARIA label for the header button
    #[prop_or_default]
    pub header_aria_label: Option<String>,

    /// Visual variant of the expandable section
    #[prop_or_default]
    pub variant: ExpandableSectionVariant,

    /// Controlled expanded state
    ///
    /// If provided, the component operates in controlled mode. Use with `on_change`
    /// to manage the state externally.
    #[prop_or_default]
    pub expanded: Option<bool>,

    /// Initial expanded state for uncontrolled mode
    ///
    /// Only used when `expanded` is not provided.
    #[prop_or_default]
    pub default_expanded: bool,

    /// Whether to remove default padding from the content area
    #[prop_or_default]
    pub disable_content_paddings: bool,

    /// Callback fired when the expanded state changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<ExpandableSectionChangeDetail>>>,

    /// Content displayed when the section is expanded
    #[prop_or_default]
    pub children: Children,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,
}

/// ExpandableSection component for collapsible content sections.
///
/// A component that displays a header with an expand/collapse icon and toggles
/// visibility of child content. Supports both controlled and uncontrolled modes.
///
/// # Controlled Example
///
/// ```rust
/// use cloudscape_components::{ExpandableSection, ExpandableSectionVariant, ExpandableSectionChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let expanded = use_state(|| false);
///
///     let on_change = {
///         let expanded = expanded.clone();
///         Callback::from(move |event: CustomEvent<ExpandableSectionChangeDetail>| {
///             expanded.set(event.detail.expanded);
///         })
///     };
///
///     html! {
///         <ExpandableSection
///             header_text={html! { "Advanced Settings" }}
///             expanded={Some(*expanded)}
///             on_change={on_change}
///             variant={ExpandableSectionVariant::Container}
///         >
///             <div>{"Expandable content here"}</div>
///         </ExpandableSection>
///     }
/// }
/// ```
///
/// # Uncontrolled Example
///
/// ```rust
/// use cloudscape_components::{ExpandableSection};
/// use yew::prelude::*;
///
/// #[function_component(SimpleExample)]
/// fn simple_example() -> Html {
///     html! {
///         <ExpandableSection
///             header="Settings"
///             default_expanded={true}
///         >
///             <div>{"Content that starts expanded"}</div>
///         </ExpandableSection>
///     }
/// }
/// ```
#[function_component(ExpandableSection)]
pub fn expandable_section(props: &ExpandableSectionProps) -> Html {
    // Internal state for uncontrolled mode
    let internal_expanded = use_state(|| props.default_expanded);

    // Determine if we're in controlled mode
    let is_controlled = props.expanded.is_some();

    // Get the current expanded state
    let expanded = if is_controlled {
        props.expanded.unwrap_or(false)
    } else {
        *internal_expanded
    };

    // Sync controlled prop changes to internal state
    {
        let internal_expanded = internal_expanded.clone();
        let controlled_expanded = props.expanded;
        use_effect_with(controlled_expanded, move |controlled| {
            if let Some(value) = controlled {
                internal_expanded.set(*value);
            }
            || ()
        });
    }

    // Generate unique IDs for ARIA
    let control_id = use_state(|| {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("awsui-expandable-section-{}", id)
    });

    let trigger_id = format!("{}-trigger", *control_id);
    let description_id = format!("{}-description", *control_id);

    // Handle expand/collapse toggle
    let on_toggle = {
        let internal_expanded = internal_expanded.clone();
        let on_change = props.on_change.clone();
        let is_controlled = is_controlled;

        Callback::from(move |new_expanded: bool| {
            // Update internal state if uncontrolled
            if !is_controlled {
                internal_expanded.set(new_expanded);
            }

            // Emit change event
            if let Some(ref callback) = on_change {
                callback.emit(CustomEvent::new_non_cancelable(
                    ExpandableSectionChangeDetail {
                        expanded: new_expanded,
                    }
                ));
            }
        })
    };

    // Click handler
    let onclick = {
        let on_toggle = on_toggle.clone();
        let expanded = expanded;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_toggle.emit(!expanded);
        })
    };

    // Keyboard handler
    let onkeydown = {
        let on_toggle = on_toggle.clone();
        let expanded = expanded;
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            if key == "Enter" || key == " " {
                e.prevent_default();
                on_toggle.emit(!expanded);
            }
        })
    };

    // Build CSS classes
    let root_classes = ClassBuilder::new()
        .add("awsui-expandable-section")
        .add(format!("awsui-expandable-section-variant-{}", props.variant.as_str()))
        .add_if(expanded, "awsui-expandable-section-expanded");

    let root_class = props.base.merge_classes(&root_classes.build());

    let header_classes = ClassBuilder::new()
        .add("awsui-expandable-section-header")
        .add(format!("awsui-expandable-section-header-{}", props.variant.as_str()))
        .build();

    let trigger_classes = ClassBuilder::new()
        .add("awsui-expandable-section-trigger")
        .add_if(expanded, "awsui-expandable-section-trigger-expanded")
        .build();

    let icon_classes = ClassBuilder::new()
        .add("awsui-expandable-section-header-icon")
        .add_if(expanded, "awsui-expandable-section-header-icon-expanded")
        .build();

    let header_text_classes = ClassBuilder::new()
        .add("awsui-expandable-section-header-text")
        .build();

    let content_classes = ClassBuilder::new()
        .add("awsui-expandable-section-content")
        .add_if(!props.disable_content_paddings, "awsui-expandable-section-content-paddings")
        .add_if(expanded, "awsui-expandable-section-content-expanded")
        .build();

    // Determine header content
    let header_content = if let Some(ref text) = props.header_text {
        text.clone()
    } else if let Some(ref header) = props.header {
        html! { { header.clone() } }
    } else {
        html! {}
    };

    // Determine ARIA label attributes
    let aria_label = props.header_aria_label.clone();
    let aria_labelledby = if aria_label.is_none() {
        Some(trigger_id.clone())
    } else {
        None
    };

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
        >
            <div class={header_classes}>
                <div
                    id={trigger_id.clone()}
                    class={trigger_classes}
                    role="button"
                    tabindex="0"
                    aria-expanded={expanded.to_string()}
                    aria-controls={(*control_id).clone()}
                    aria-label={aria_label}
                    onclick={onclick}
                    onkeydown={onkeydown}
                >
                    // Icon (chevron/caret)
                    <span class={icon_classes}>
                        // Simple caret-down icon using CSS
                        <svg
                            class="awsui-expandable-section-icon-svg"
                            focusable="false"
                            aria-hidden="true"
                            viewBox="0 0 16 16"
                        >
                            <path d="M8 10.5L3.5 6h9L8 10.5z" />
                        </svg>
                    </span>

                    // Header text
                    <span class={header_text_classes}>
                        { header_content }
                    </span>
                </div>

                // Header actions (if provided)
                if let Some(ref actions) = props.header_actions {
                    <div class="awsui-expandable-section-header-actions">
                        { actions.clone() }
                    </div>
                }
            </div>

            // Header description (if provided)
            if let Some(ref description) = props.header_description {
                <div
                    id={description_id.clone()}
                    class="awsui-expandable-section-header-description"
                >
                    { description.clone() }
                </div>
            }

            // Content
            <div
                id={(*control_id).clone()}
                class={content_classes}
                role="group"
                aria-label={props.header_aria_label.clone()}
                aria-labelledby={aria_labelledby}
                aria-describedby={props.header_description.as_ref().map(|_| description_id)}
                hidden={!expanded}
            >
                { props.children.clone() }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_default() {
        assert_eq!(ExpandableSectionVariant::default(), ExpandableSectionVariant::Default);
    }

    #[test]
    fn test_variant_as_str() {
        assert_eq!(ExpandableSectionVariant::Default.as_str(), "default");
        assert_eq!(ExpandableSectionVariant::Container.as_str(), "container");
        assert_eq!(ExpandableSectionVariant::Footer.as_str(), "footer");
        assert_eq!(ExpandableSectionVariant::Navigation.as_str(), "navigation");
    }

    #[test]
    fn test_change_detail() {
        let detail = ExpandableSectionChangeDetail { expanded: true };
        assert_eq!(detail.expanded, true);

        let detail2 = ExpandableSectionChangeDetail { expanded: false };
        assert_eq!(detail2.expanded, false);
    }

    #[test]
    fn test_change_detail_equality() {
        let detail1 = ExpandableSectionChangeDetail { expanded: true };
        let detail2 = ExpandableSectionChangeDetail { expanded: true };
        let detail3 = ExpandableSectionChangeDetail { expanded: false };

        assert_eq!(detail1, detail2);
        assert_ne!(detail1, detail3);
    }

    #[test]
    fn test_variant_equality() {
        assert_eq!(ExpandableSectionVariant::Default, ExpandableSectionVariant::Default);
        assert_ne!(ExpandableSectionVariant::Default, ExpandableSectionVariant::Container);
    }

    #[test]
    fn test_variant_debug() {
        let variant = ExpandableSectionVariant::Container;
        let debug_str = format!("{:?}", variant);
        assert!(debug_str.contains("Container"));
    }

    #[test]
    fn test_change_detail_clone() {
        let detail = ExpandableSectionChangeDetail { expanded: true };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_variant_copy() {
        let variant1 = ExpandableSectionVariant::Footer;
        let variant2 = variant1;
        assert_eq!(variant1, variant2);
    }

    #[test]
    fn test_all_variants() {
        let variants = vec![
            ExpandableSectionVariant::Default,
            ExpandableSectionVariant::Container,
            ExpandableSectionVariant::Footer,
            ExpandableSectionVariant::Navigation,
        ];

        for variant in variants {
            assert!(!variant.as_str().is_empty());
        }
    }

    #[test]
    fn test_change_detail_debug() {
        let detail = ExpandableSectionChangeDetail { expanded: false };
        let debug_str = format!("{:?}", detail);
        assert!(debug_str.contains("expanded"));
        assert!(debug_str.contains("false"));
    }
}
