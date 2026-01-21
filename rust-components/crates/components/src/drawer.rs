// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Drawer component for side panel overlay with slide-in animation.
//!
//! The Drawer component displays content in a panel that slides in from the right edge
//! of the screen. It supports various sizes, custom header and footer content, and handles
//! focus management and keyboard interactions. The drawer blocks interaction with the rest
//! of the page via an overlay backdrop.

use crate::internal::{AriaAttributes, BaseComponentProps, ClassBuilder, CustomEvent};
use crate::modal::DismissReason;
use web_sys::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

/// Drawer size variants
///
/// Determines the width of the drawer panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerSize {
    /// Small drawer (290px)
    Small,
    /// Medium drawer (520px) - default
    #[default]
    Medium,
    /// Large drawer (820px)
    Large,
}

impl DrawerSize {
    /// Returns the CSS class name suffix for this size
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Event detail for drawer dismiss events
#[derive(Clone, PartialEq, Debug)]
pub struct DrawerDismissDetail {
    /// The reason the drawer was dismissed
    pub reason: DismissReason,
}

/// Properties for the Drawer component
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Whether the drawer is visible
    ///
    /// This is a controlled property. The drawer is only shown when this is `true`.
    #[prop_or_default]
    pub visible: bool,

    /// Size variant of the drawer
    #[prop_or_default]
    pub size: DrawerSize,

    /// Simple header text (deprecated in favor of header_text)
    #[prop_or_default]
    pub header: Option<String>,

    /// Header content (can be text or HTML)
    #[prop_or_default]
    pub header_text: Option<Html>,

    /// Footer content
    #[prop_or_default]
    pub footer: Option<Html>,

    /// Whether to show loading spinner in content area
    #[prop_or_default]
    pub loading: bool,

    /// ARIA label for the close button
    ///
    /// Default: "Close drawer"
    #[prop_or_default]
    pub close_aria_label: Option<String>,

    /// Callback fired when the drawer is dismissed
    ///
    /// The detail contains the reason for dismissal (closeButton, overlay, or keyboard)
    #[prop_or_default]
    pub on_dismiss: Option<Callback<CustomEvent<DrawerDismissDetail>>>,

    /// Drawer content (children)
    #[prop_or_default]
    pub children: Children,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,
}

/// Drawer component for side panel overlay with slide-in animation.
///
/// A component that displays content in a panel that slides in from the right edge.
/// The drawer blocks interaction with the rest of the page and can be dismissed via
/// close button, clicking the overlay, or pressing the Escape key.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Drawer, DrawerSize, DrawerDismissDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let visible = use_state(|| false);
///
///     let on_dismiss = {
///         let visible = visible.clone();
///         Callback::from(move |_event: CustomEvent<DrawerDismissDetail>| {
///             visible.set(false);
///         })
///     };
///
///     html! {
///         <>
///             <button onclick={
///                 let visible = visible.clone();
///                 Callback::from(move |_| visible.set(true))
///             }>
///                 {"Open Drawer"}
///             </button>
///             <Drawer
///                 visible={*visible}
///                 size={DrawerSize::Medium}
///                 header="Settings"
///                 footer={html! {
///                     <div style="text-align: right;">
///                         <button>{"Cancel"}</button>
///                         <button>{"Apply"}</button>
///                     </div>
///                 }}
///                 on_dismiss={on_dismiss}
///             >
///                 <div>{"Drawer content"}</div>
///             </Drawer>
///         </>
///     }
/// }
/// ```
#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    // Don't render anything if not visible
    if !props.visible {
        return html! {};
    }

    // Generate unique IDs for ARIA
    let control_id = use_state(|| {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("awsui-drawer-{}", id)
    });

    let header_id = format!("{}-header", *control_id);
    let content_id = format!("{}-content", *control_id);

    // Track whether mousedown happened on overlay
    let mousedown_on_overlay = use_state(|| false);

    // Lock body scroll when drawer is visible
    use_effect_with(props.visible, |visible| {
        if *visible
            && let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(body) = document.body()
        {
            // HtmlElement inherits from Element which has set_attribute
            let _ = body.set_attribute("style", "overflow: hidden");
        }

        // Cleanup: restore body scroll
        move || {
            if let Some(window) = web_sys::window()
                && let Some(document) = window.document()
                && let Some(body) = document.body()
            {
                let _ = body.remove_attribute("style");
            }
        }
    });

    // Dismiss handler
    let dismiss = {
        let on_dismiss = props.on_dismiss.clone();
        Callback::from(move |reason: DismissReason| {
            if let Some(ref callback) = on_dismiss {
                callback.emit(CustomEvent::new_non_cancelable(DrawerDismissDetail {
                    reason,
                }));
            }
        })
    };

    // Overlay mousedown handler
    let on_overlay_mousedown = {
        let mousedown_on_overlay = mousedown_on_overlay.clone();
        Callback::from(move |_e: MouseEvent| {
            mousedown_on_overlay.set(true);
        })
    };

    // Overlay click handler
    let on_overlay_click = {
        let dismiss = dismiss.clone();
        let mousedown_on_overlay = mousedown_on_overlay.clone();
        Callback::from(move |_e: MouseEvent| {
            // Only dismiss if both mousedown and click happened on overlay
            if *mousedown_on_overlay {
                dismiss.emit(DismissReason::Overlay);
            }
            mousedown_on_overlay.set(false);
        })
    };

    // Close button handler
    let on_close_button_click = {
        let dismiss = dismiss.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dismiss.emit(DismissReason::CloseButton);
        })
    };

    // Escape key handler
    let on_keydown = {
        let dismiss = dismiss.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Escape" {
                e.prevent_default();
                dismiss.emit(DismissReason::Keyboard);
            }
        })
    };

    // Build CSS classes
    let root_classes = ClassBuilder::new()
        .add("awsui-drawer")
        .add_if(props.visible, "awsui-drawer-visible");

    let overlay_classes = ClassBuilder::new().add("awsui-drawer-overlay").build();

    let container_classes = ClassBuilder::new()
        .add("awsui-drawer-container")
        .add(format!("awsui-drawer-size-{}", props.size.as_str()))
        .build();

    let header_classes = ClassBuilder::new().add("awsui-drawer-header").build();

    let content_classes = ClassBuilder::new()
        .add("awsui-drawer-content")
        .add_if(props.loading, "awsui-drawer-loading")
        .build();

    let footer_classes = ClassBuilder::new().add("awsui-drawer-footer").build();

    // Determine header content
    let header_content = if let Some(ref text) = props.header_text {
        text.clone()
    } else if let Some(ref header) = props.header {
        html! { { header.clone() } }
    } else {
        html! {}
    };

    // Close button ARIA label
    let close_label = props
        .close_aria_label
        .clone()
        .unwrap_or_else(|| "Close drawer".to_string());

    let root_class = props.base.merge_classes(&root_classes.build());

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
            role="dialog"
            aria-modal="true"
            aria-labelledby={header_id.clone()}
            aria-describedby={content_id.clone()}
            onkeydown={on_keydown}
        >
            <div
                class={overlay_classes}
                onmousedown={on_overlay_mousedown}
                onclick={on_overlay_click}
            >
                <div
                    class={container_classes}
                    onmousedown={{
                        let mousedown_on_overlay = mousedown_on_overlay.clone();
                        Callback::from(move |e: MouseEvent| {
                            // Mark that mousedown was NOT on overlay
                            mousedown_on_overlay.set(false);
                            e.stop_propagation();
                        })
                    }}
                    onclick={Callback::from(|e: MouseEvent| {
                        // Prevent clicks inside the drawer from bubbling to overlay
                        e.stop_propagation();
                    })}
                >
                    // Header
                    <div class={header_classes}>
                        <div class="awsui-drawer-header-content">
                            <h2 id={header_id} class="awsui-drawer-header-text">
                                { header_content }
                            </h2>
                        </div>
                        <button
                            type="button"
                            class="awsui-drawer-close-button"
                            aria-label={close_label}
                            onclick={on_close_button_click}
                        >
                            <span class="awsui-icon awsui-icon-close" aria-hidden="true">
                                // Simple × character as close icon
                                { "×" }
                            </span>
                        </button>
                    </div>

                    // Content
                    <div id={content_id} class={content_classes}>
                        if props.loading {
                            <div class="awsui-drawer-loading-spinner">
                                // Simple loading indicator
                                <div class="awsui-spinner" role="status" aria-live="polite">
                                    <span class="awsui-spinner-text">{"Loading..."}</span>
                                </div>
                            </div>
                        } else {
                            { props.children.clone() }
                        }
                    </div>

                    // Footer (if provided)
                    if let Some(ref footer) = props.footer {
                        <div class={footer_classes}>
                            { footer.clone() }
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawer_size_default() {
        assert_eq!(DrawerSize::default(), DrawerSize::Medium);
    }

    #[test]
    fn test_drawer_size_as_str() {
        assert_eq!(DrawerSize::Small.as_str(), "small");
        assert_eq!(DrawerSize::Medium.as_str(), "medium");
        assert_eq!(DrawerSize::Large.as_str(), "large");
    }

    #[test]
    fn test_drawer_size_equality() {
        assert_eq!(DrawerSize::Small, DrawerSize::Small);
        assert_ne!(DrawerSize::Small, DrawerSize::Medium);
    }

    #[test]
    fn test_drawer_dismiss_detail() {
        let detail = DrawerDismissDetail {
            reason: DismissReason::CloseButton,
        };
        assert_eq!(detail.reason, DismissReason::CloseButton);
    }

    #[test]
    fn test_drawer_dismiss_detail_equality() {
        let detail1 = DrawerDismissDetail {
            reason: DismissReason::Overlay,
        };
        let detail2 = DrawerDismissDetail {
            reason: DismissReason::Overlay,
        };
        let detail3 = DrawerDismissDetail {
            reason: DismissReason::Keyboard,
        };

        assert_eq!(detail1, detail2);
        assert_ne!(detail1, detail3);
    }

    #[test]
    fn test_drawer_dismiss_detail_clone() {
        let detail = DrawerDismissDetail {
            reason: DismissReason::CloseButton,
        };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_drawer_size_debug() {
        let size = DrawerSize::Large;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Large"));
    }

    #[test]
    fn test_all_drawer_sizes() {
        let sizes = vec![DrawerSize::Small, DrawerSize::Medium, DrawerSize::Large];

        for size in sizes {
            assert!(!size.as_str().is_empty());
        }
    }

    #[test]
    fn test_drawer_dismiss_detail_debug() {
        let detail = DrawerDismissDetail {
            reason: DismissReason::Keyboard,
        };
        let debug_str = format!("{:?}", detail);
        assert!(debug_str.contains("Keyboard"));
    }

    #[test]
    fn test_drawer_size_copy() {
        let size1 = DrawerSize::Medium;
        let size2 = size1;
        assert_eq!(size1, size2);
    }
}
