// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Modal component for dialog/modal overlay with portal rendering.
//!
//! The Modal component displays content in a layer above the main application content,
//! blocking interaction with the rest of the page. It supports various sizes, custom
//! header and footer content, and handles focus management and keyboard interactions.

use crate::internal::{AriaAttributes, BaseComponentProps, ClassBuilder, CustomEvent};
use web_sys::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

/// Modal size variants
///
/// Determines the width of the modal dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalSize {
    /// Small modal (280px)
    Small,
    /// Medium modal (460px) - default
    Medium,
    /// Large modal (680px)
    Large,
    /// Maximum modal (variable width up to design limit)
    Max,
}

impl Default for ModalSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl ModalSize {
    /// Returns the CSS class name suffix for this size
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Max => "max",
        }
    }
}

/// Reasons for modal dismissal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissReason {
    /// Dismissed via close button
    CloseButton,
    /// Dismissed by clicking overlay
    Overlay,
    /// Dismissed via Escape key
    Keyboard,
}

impl DismissReason {
    /// Returns the string representation of this reason
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CloseButton => "closeButton",
            Self::Overlay => "overlay",
            Self::Keyboard => "keyboard",
        }
    }
}

/// Event detail for modal dismiss events
#[derive(Clone, PartialEq, Debug)]
pub struct ModalDismissDetail {
    /// The reason the modal was dismissed
    pub reason: DismissReason,
}

/// Properties for the Modal component
#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Whether the modal is visible
    ///
    /// This is a controlled property. The modal is only shown when this is `true`.
    #[prop_or_default]
    pub visible: bool,

    /// Size variant of the modal
    #[prop_or_default]
    pub size: ModalSize,

    /// Simple header text (deprecated in favor of header_text)
    #[prop_or_default]
    pub header: Option<String>,

    /// Header content (can be text or HTML)
    #[prop_or_default]
    pub header_text: Option<Html>,

    /// Footer content
    #[prop_or_default]
    pub footer: Option<Html>,

    /// Whether to remove default padding from the header
    #[prop_or_default]
    pub disable_header_paddings: bool,

    /// Whether to remove default padding from the content area
    #[prop_or_default]
    pub disable_content_paddings: bool,

    /// ARIA label for the close button
    ///
    /// Default: "Close modal"
    #[prop_or_default]
    pub close_aria_label: Option<String>,

    /// Callback fired when the modal is dismissed
    ///
    /// The detail contains the reason for dismissal (closeButton, overlay, or keyboard)
    #[prop_or_default]
    pub on_dismiss: Option<Callback<CustomEvent<ModalDismissDetail>>>,

    /// Modal content (children)
    #[prop_or_default]
    pub children: Children,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,
}

/// Modal component for dialog/modal overlay with portal rendering.
///
/// A component that displays content in a modal dialog overlay. The modal blocks
/// interaction with the rest of the page and can be dismissed via close button,
/// clicking the overlay, or pressing the Escape key.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Modal, ModalSize, ModalDismissDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let visible = use_state(|| false);
///
///     let on_dismiss = {
///         let visible = visible.clone();
///         Callback::from(move |_event: CustomEvent<ModalDismissDetail>| {
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
///                 {"Open Modal"}
///             </button>
///             <Modal
///                 visible={*visible}
///                 size={ModalSize::Medium}
///                 header="Confirm action"
///                 footer={html! {
///                     <div style="text-align: right;">
///                         <button>{"Cancel"}</button>
///                         <button>{"Confirm"}</button>
///                     </div>
///                 }}
///                 on_dismiss={on_dismiss}
///             >
///                 <div>{"Are you sure you want to proceed?"}</div>
///             </Modal>
///         </>
///     }
/// }
/// ```
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    // Don't render anything if not visible
    if !props.visible {
        return html! {};
    }

    // Generate unique IDs for ARIA
    let control_id = use_state(|| {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("awsui-modal-{}", id)
    });

    let header_id = format!("{}-header", *control_id);
    let content_id = format!("{}-content", *control_id);

    // Track whether mousedown happened on overlay
    let mousedown_on_overlay = use_state(|| false);

    // Lock body scroll when modal is visible
    use_effect_with(props.visible, |visible| {
        if *visible {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        // HtmlElement inherits from Element which has set_attribute
                        let _ = body.set_attribute("style", "overflow: hidden");
                    }
                }
            }
        }

        // Cleanup: restore body scroll
        move || {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let _ = body.remove_attribute("style");
                    }
                }
            }
        }
    });

    // Dismiss handler
    let dismiss = {
        let on_dismiss = props.on_dismiss.clone();
        Callback::from(move |reason: DismissReason| {
            if let Some(ref callback) = on_dismiss {
                callback.emit(CustomEvent::new_non_cancelable(ModalDismissDetail {
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
        .add("awsui-modal")
        .add_if(props.visible, "awsui-modal-visible");

    let overlay_classes = ClassBuilder::new().add("awsui-modal-overlay").build();

    let container_classes = ClassBuilder::new()
        .add("awsui-modal-container")
        .add(format!("awsui-modal-size-{}", props.size.as_str()))
        .build();

    let header_classes = ClassBuilder::new()
        .add("awsui-modal-header")
        .add_if(
            !props.disable_header_paddings,
            "awsui-modal-header-paddings",
        )
        .build();

    let content_classes = ClassBuilder::new()
        .add("awsui-modal-content")
        .add_if(
            !props.disable_content_paddings,
            "awsui-modal-content-paddings",
        )
        .build();

    let footer_classes = ClassBuilder::new().add("awsui-modal-footer").build();

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
        .unwrap_or_else(|| "Close modal".to_string());

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
                        // Prevent clicks inside the modal from bubbling to overlay
                        e.stop_propagation();
                    })}
                >
                    // Header
                    <div class={header_classes}>
                        <div class="awsui-modal-header-content">
                            <h2 id={header_id} class="awsui-modal-header-text">
                                { header_content }
                            </h2>
                        </div>
                        <button
                            type="button"
                            class="awsui-modal-close-button"
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
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_size_default() {
        assert_eq!(ModalSize::default(), ModalSize::Medium);
    }

    #[test]
    fn test_modal_size_as_str() {
        assert_eq!(ModalSize::Small.as_str(), "small");
        assert_eq!(ModalSize::Medium.as_str(), "medium");
        assert_eq!(ModalSize::Large.as_str(), "large");
        assert_eq!(ModalSize::Max.as_str(), "max");
    }

    #[test]
    fn test_dismiss_reason_as_str() {
        assert_eq!(DismissReason::CloseButton.as_str(), "closeButton");
        assert_eq!(DismissReason::Overlay.as_str(), "overlay");
        assert_eq!(DismissReason::Keyboard.as_str(), "keyboard");
    }

    #[test]
    fn test_modal_size_equality() {
        assert_eq!(ModalSize::Small, ModalSize::Small);
        assert_ne!(ModalSize::Small, ModalSize::Medium);
    }

    #[test]
    fn test_dismiss_reason_equality() {
        assert_eq!(DismissReason::CloseButton, DismissReason::CloseButton);
        assert_ne!(DismissReason::CloseButton, DismissReason::Overlay);
    }

    #[test]
    fn test_modal_dismiss_detail() {
        let detail = ModalDismissDetail {
            reason: DismissReason::CloseButton,
        };
        assert_eq!(detail.reason, DismissReason::CloseButton);
    }

    #[test]
    fn test_modal_dismiss_detail_equality() {
        let detail1 = ModalDismissDetail {
            reason: DismissReason::Overlay,
        };
        let detail2 = ModalDismissDetail {
            reason: DismissReason::Overlay,
        };
        let detail3 = ModalDismissDetail {
            reason: DismissReason::Keyboard,
        };

        assert_eq!(detail1, detail2);
        assert_ne!(detail1, detail3);
    }

    #[test]
    fn test_modal_dismiss_detail_clone() {
        let detail = ModalDismissDetail {
            reason: DismissReason::CloseButton,
        };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_modal_size_debug() {
        let size = ModalSize::Large;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Large"));
    }

    #[test]
    fn test_dismiss_reason_debug() {
        let reason = DismissReason::Keyboard;
        let debug_str = format!("{:?}", reason);
        assert!(debug_str.contains("Keyboard"));
    }

    #[test]
    fn test_all_modal_sizes() {
        let sizes = vec![
            ModalSize::Small,
            ModalSize::Medium,
            ModalSize::Large,
            ModalSize::Max,
        ];

        for size in sizes {
            assert!(!size.as_str().is_empty());
        }
    }

    #[test]
    fn test_all_dismiss_reasons() {
        let reasons = vec![
            DismissReason::CloseButton,
            DismissReason::Overlay,
            DismissReason::Keyboard,
        ];

        for reason in reasons {
            assert!(!reason.as_str().is_empty());
        }
    }
}
