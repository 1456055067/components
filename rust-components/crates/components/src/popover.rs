// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Popover component for displaying additional content in an overlay.
//!
//! The Popover component displays content in a positioned overlay relative to a trigger element.
//! It supports multiple positions, sizes, and dismissal methods including close button,
//! clicking outside, and keyboard interactions.

use crate::internal::{AriaAttributes, BaseComponentProps, ClassBuilder, CustomEvent};
use crate::modal::DismissReason;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, KeyboardEvent, MouseEvent};
use yew::prelude::*;

/// Popover size variants
///
/// Determines the width of the popover content container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverSize {
    /// Small popover (200px)
    Small,
    /// Medium popover (280px) - default
    #[default]
    Medium,
    /// Large popover (400px)
    Large,
}

impl PopoverSize {
    /// Returns the CSS class name suffix for this size
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Popover position variants
///
/// Determines the preferred position of the popover relative to its trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverPosition {
    /// Position above the trigger
    #[default]
    Top,
    /// Position below the trigger
    Bottom,
    /// Position to the left of the trigger
    Left,
    /// Position to the right of the trigger
    Right,
}

impl PopoverPosition {
    /// Returns the CSS class name suffix for this position
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// Event detail for popover dismiss events
///
/// Contains information about why the popover was dismissed.
#[derive(Clone, PartialEq, Debug)]
pub struct PopoverDismissDetail {
    /// The reason the popover was dismissed
    pub reason: DismissReason,
}

/// Properties for the Popover component
#[derive(Properties, PartialEq, Clone)]
pub struct PopoverProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Size variant of the popover
    #[prop_or_default]
    pub size: PopoverSize,

    /// Position preference for the popover
    #[prop_or_default]
    pub position: PopoverPosition,

    /// Whether the popover can be dismissed
    ///
    /// When `true`, shows a close button and allows dismissal via overlay click or Escape key.
    #[prop_or_default]
    pub dismissible: bool,

    /// Optional header text for the popover
    #[prop_or_default]
    pub header: Option<String>,

    /// Content to display in the popover body
    ///
    /// This is the main content area of the popover.
    #[prop_or_default]
    pub content: Option<Html>,

    /// Type of trigger element
    ///
    /// Can be "text" for inline text triggers or "custom" for custom trigger elements.
    #[prop_or_default]
    pub trigger_type: Option<String>,

    /// Callback fired when the popover is dismissed
    ///
    /// The detail contains the reason for dismissal (closeButton, overlay, or keyboard).
    #[prop_or_default]
    pub on_dismiss: Option<Callback<CustomEvent<PopoverDismissDetail>>>,

    /// Trigger element (children)
    ///
    /// The element that triggers the popover when clicked.
    #[prop_or_default]
    pub children: Children,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA label for the close button
    ///
    /// Default: "Close popover"
    #[prop_or_default]
    pub close_aria_label: Option<String>,

    /// Whether to use fixed positioning
    ///
    /// When `true`, the popover uses fixed positioning instead of absolute.
    #[prop_or_default]
    pub fixed: bool,
}

/// Popover component for displaying additional content in an overlay.
///
/// A component that displays content in a positioned overlay relative to a trigger element.
/// The popover can be dismissed via close button, clicking outside, or pressing the Escape key.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Popover, PopoverSize, PopoverPosition, PopoverDismissDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let visible = use_state(|| false);
///
///     let on_dismiss = {
///         let visible = visible.clone();
///         Callback::from(move |_event: CustomEvent<PopoverDismissDetail>| {
///             visible.set(false);
///         })
///     };
///
///     html! {
///         <Popover
///             size={PopoverSize::Medium}
///             position={PopoverPosition::Top}
///             dismissible={true}
///             header="More information"
///             content={html! { <div>{"Detailed content here"}</div> }}
///             on_dismiss={on_dismiss}
///         >
///             <button>{"Show info"}</button>
///         </Popover>
///     }
/// }
/// ```
#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    // Track whether popover is visible
    let visible = use_state(|| false);

    // Reference to the popover container
    let popover_ref = use_node_ref();
    let trigger_ref = use_node_ref();

    // Generate unique IDs for ARIA
    let control_id = use_state(|| {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("awsui-popover-{}", id)
    });

    let header_id = format!("{}-header", *control_id);
    let content_id = format!("{}-content", *control_id);

    // Toggle popover visibility
    let on_trigger_click = {
        let visible = visible.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            visible.set(!*visible);
        })
    };

    // Dismiss handler
    let dismiss = {
        let visible = visible.clone();
        let on_dismiss = props.on_dismiss.clone();
        Callback::from(move |reason: DismissReason| {
            visible.set(false);
            if let Some(ref callback) = on_dismiss {
                callback.emit(CustomEvent::new_non_cancelable(PopoverDismissDetail {
                    reason,
                }));
            }
        })
    };

    // Close button handler
    let on_close_button_click = {
        let dismiss = dismiss.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            dismiss.emit(DismissReason::CloseButton);
        })
    };

    // Escape key handler
    let on_keydown = {
        let dismiss = dismiss.clone();
        let dismissible = props.dismissible;
        Callback::from(move |e: KeyboardEvent| {
            if dismissible && e.key() == "Escape" {
                e.prevent_default();
                dismiss.emit(DismissReason::Keyboard);
            }
        })
    };

    // Click outside handler using gloo events
    {
        let visible = visible.clone();
        let dismiss = dismiss.clone();
        let dismissible = props.dismissible;
        let popover_ref = popover_ref.clone();

        use_effect_with(
            (*visible, dismissible),
            move |(is_visible, is_dismissible)| {
                let listener = if *is_visible && *is_dismissible {
                    let document = web_sys::window()
                        .and_then(|w| w.document())
                        .expect("Should have document");

                    let popover_ref = popover_ref.clone();
                    let dismiss = dismiss.clone();

                    Some(EventListener::new(&document, "mousedown", move |event| {
                        let mouse_event =
                            event.dyn_ref::<MouseEvent>().expect("Should be MouseEvent");

                        if let Some(popover_element) = popover_ref.cast::<Element>()
                            && let Some(target) = mouse_event.target()
                            && let Ok(target_element) = target.dyn_into::<Element>()
                        {
                            // Check if click is outside popover
                            if !popover_element.contains(Some(&target_element)) {
                                dismiss.emit(DismissReason::Overlay);
                            }
                        }
                    }))
                } else {
                    None
                };

                // Return cleanup function
                move || {
                    drop(listener);
                }
            },
        );
    }

    // Build CSS classes
    let root_classes = ClassBuilder::new()
        .add("awsui-popover")
        .add_if(*visible, "awsui-popover-visible");

    let trigger_classes = ClassBuilder::new()
        .add("awsui-popover-trigger")
        .add_option(
            props
                .trigger_type
                .as_ref()
                .map(|t| format!("awsui-popover-trigger-{}", t)),
        )
        .build();

    let container_classes = ClassBuilder::new()
        .add("awsui-popover-container")
        .add(format!("awsui-popover-size-{}", props.size.as_str()))
        .add(format!(
            "awsui-popover-position-{}",
            props.position.as_str()
        ))
        .add_if(props.fixed, "awsui-popover-fixed")
        .build();

    let header_classes = ClassBuilder::new().add("awsui-popover-header").build();

    let content_classes = ClassBuilder::new().add("awsui-popover-content").build();

    let dismiss_button_classes = ClassBuilder::new()
        .add("awsui-popover-dismiss-button")
        .build();

    // Close button ARIA label
    let close_label = props
        .close_aria_label
        .clone()
        .unwrap_or_else(|| "Close popover".to_string());

    let root_class = props.base.merge_classes(&root_classes.build());

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
            ref={popover_ref.clone()}
            onkeydown={on_keydown}
        >
            // Trigger element
            <div
                class={trigger_classes}
                onclick={on_trigger_click}
                ref={trigger_ref}
                aria-expanded={(*visible).to_string()}
                aria-controls={(*control_id).clone()}
            >
                { props.children.clone() }
            </div>

            // Popover content (only render when visible)
            if *visible {
                <div
                    id={(*control_id).clone()}
                    class={container_classes}
                    role="dialog"
                    aria-modal="false"
                    aria-labelledby={if props.header.is_some() { Some(header_id.clone()) } else { None }}
                    aria-describedby={content_id.clone()}
                    onclick={Callback::from(|e: MouseEvent| {
                        // Prevent clicks inside the popover from bubbling
                        e.stop_propagation();
                    })}
                >
                    // Arrow/pointer
                    <div class="awsui-popover-arrow" />

                    // Header (if provided)
                    if let Some(ref header) = props.header {
                        <div class={header_classes}>
                            <div class="awsui-popover-header-content">
                                <h3 id={header_id} class="awsui-popover-header-text">
                                    { header.clone() }
                                </h3>
                            </div>
                            if props.dismissible {
                                <button
                                    type="button"
                                    class={dismiss_button_classes.clone()}
                                    aria-label={close_label.clone()}
                                    onclick={on_close_button_click.clone()}
                                >
                                    <span class="awsui-icon awsui-icon-close" aria-hidden="true">
                                        { "×" }
                                    </span>
                                </button>
                            }
                        </div>
                    }

                    // Content
                    <div id={content_id} class={content_classes}>
                        if let Some(ref content) = props.content {
                            { content.clone() }
                        }
                    </div>

                    // Close button at bottom (if dismissible and no header)
                    if props.dismissible && props.header.is_none() {
                        <div class="awsui-popover-footer">
                            <button
                                type="button"
                                class={dismiss_button_classes}
                                aria-label={close_label}
                                onclick={on_close_button_click}
                            >
                                <span class="awsui-icon awsui-icon-close" aria-hidden="true">
                                    { "×" }
                                </span>
                            </button>
                        </div>
                    }
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_size_default() {
        assert_eq!(PopoverSize::default(), PopoverSize::Medium);
    }

    #[test]
    fn test_popover_size_as_str() {
        assert_eq!(PopoverSize::Small.as_str(), "small");
        assert_eq!(PopoverSize::Medium.as_str(), "medium");
        assert_eq!(PopoverSize::Large.as_str(), "large");
    }

    #[test]
    fn test_popover_position_default() {
        assert_eq!(PopoverPosition::default(), PopoverPosition::Top);
    }

    #[test]
    fn test_popover_position_as_str() {
        assert_eq!(PopoverPosition::Top.as_str(), "top");
        assert_eq!(PopoverPosition::Bottom.as_str(), "bottom");
        assert_eq!(PopoverPosition::Left.as_str(), "left");
        assert_eq!(PopoverPosition::Right.as_str(), "right");
    }

    #[test]
    fn test_popover_size_equality() {
        assert_eq!(PopoverSize::Small, PopoverSize::Small);
        assert_ne!(PopoverSize::Small, PopoverSize::Medium);
    }

    #[test]
    fn test_popover_position_equality() {
        assert_eq!(PopoverPosition::Top, PopoverPosition::Top);
        assert_ne!(PopoverPosition::Top, PopoverPosition::Bottom);
    }

    #[test]
    fn test_popover_dismiss_detail() {
        let detail = PopoverDismissDetail {
            reason: DismissReason::CloseButton,
        };
        assert_eq!(detail.reason, DismissReason::CloseButton);
    }

    #[test]
    fn test_popover_dismiss_detail_equality() {
        let detail1 = PopoverDismissDetail {
            reason: DismissReason::Overlay,
        };
        let detail2 = PopoverDismissDetail {
            reason: DismissReason::Overlay,
        };
        let detail3 = PopoverDismissDetail {
            reason: DismissReason::Keyboard,
        };

        assert_eq!(detail1, detail2);
        assert_ne!(detail1, detail3);
    }

    #[test]
    fn test_popover_dismiss_detail_clone() {
        let detail = PopoverDismissDetail {
            reason: DismissReason::CloseButton,
        };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_popover_size_debug() {
        let size = PopoverSize::Large;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Large"));
    }

    #[test]
    fn test_popover_position_debug() {
        let position = PopoverPosition::Right;
        let debug_str = format!("{:?}", position);
        assert!(debug_str.contains("Right"));
    }

    #[test]
    fn test_all_popover_sizes() {
        let sizes = vec![PopoverSize::Small, PopoverSize::Medium, PopoverSize::Large];

        for size in sizes {
            assert!(!size.as_str().is_empty());
        }
    }

    #[test]
    fn test_all_popover_positions() {
        let positions = vec![
            PopoverPosition::Top,
            PopoverPosition::Bottom,
            PopoverPosition::Left,
            PopoverPosition::Right,
        ];

        for position in positions {
            assert!(!position.as_str().is_empty());
        }
    }
}
