// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! CopyToClipboard component
//!
//! A button that copies text to the system clipboard with feedback messages.

use crate::internal::{BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{MouseEvent, Window};
use yew::prelude::*;

/// Copy operation status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyStatus {
    /// Copy operation succeeded
    Success,
    /// Copy operation failed
    Error,
}

/// Event detail for copy events
#[derive(Debug, Clone, PartialEq)]
pub struct CopyDetail {
    /// The status of the copy operation
    pub status: CopyStatus,
}

/// Visual variants for the copy button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CopyToClipboardVariant {
    /// Standard button variant (default)
    #[default]
    Button,
    /// Icon-only variant
    Icon,
    /// Inline text variant
    Inline,
}

impl CopyToClipboardVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            CopyToClipboardVariant::Button => "button",
            CopyToClipboardVariant::Icon => "icon",
            CopyToClipboardVariant::Inline => "inline",
        }
    }
}

/// Properties for the CopyToClipboard component
#[derive(Properties, PartialEq, Clone)]
pub struct CopyToClipboardProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// The text to copy to clipboard
    pub copy_text: String,

    /// Button text (defaults to "Copy")
    #[prop_or_default]
    pub copy_button_text: Option<String>,

    /// Success feedback message (defaults to "Copied")
    #[prop_or_default]
    pub copy_success_text: Option<String>,

    /// Error feedback message (defaults to "Failed to copy")
    #[prop_or_default]
    pub copy_error_text: Option<String>,

    /// Visual variant
    #[prop_or_default]
    pub variant: CopyToClipboardVariant,

    /// Callback fired when copy operation completes
    #[prop_or_default]
    pub on_copy: Option<Callback<CustomEvent<CopyDetail>>>,

    /// Whether the button is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// ARIA label for accessibility
    #[prop_or_default]
    pub aria_label: Option<String>,
}

/// Internal state for tracking copy status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CopyState {
    Idle,
    Success,
    Error,
}

/// CopyToClipboard component for copying text to the system clipboard.
///
/// This component provides a button that copies text to the clipboard and shows
/// feedback to the user about the operation status. It uses the modern Clipboard API
/// and falls back gracefully when the API is not available.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{CopyToClipboard, CopyToClipboardVariant};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     html! {
///         <CopyToClipboard
///             copy_text="Hello, world!"
///             variant={CopyToClipboardVariant::Button}
///         />
///     }
/// }
/// ```
///
/// # With callback
///
/// ```rust
/// use cloudscape_components::{CopyToClipboard, CopyDetail, CopyStatus, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let on_copy = Callback::from(|event: CustomEvent<CopyDetail>| {
///         match event.detail.status {
///             CopyStatus::Success => {
///                 web_sys::console::log_1(&"Copied successfully!".into());
///             }
///             CopyStatus::Error => {
///                 web_sys::console::log_1(&"Failed to copy!".into());
///             }
///         }
///     });
///
///     html! {
///         <CopyToClipboard
///             copy_text="Hello, world!"
///             on_copy={on_copy}
///         />
///     }
/// }
/// ```
///
/// # Accessibility
///
/// The component follows accessibility best practices:
/// - Uses semantic button element
/// - Provides clear ARIA labels
/// - Shows visual and screen-reader feedback for operation status
/// - Supports keyboard navigation
/// - Respects disabled state
///
/// # Browser Compatibility
///
/// The component uses the modern Clipboard API which is widely supported in
/// modern browsers. The copy operation requires a secure context (HTTPS or localhost).
#[function_component(CopyToClipboard)]
pub fn copy_to_clipboard(props: &CopyToClipboardProps) -> Html {
    let _metadata = ComponentMetadata::new("CopyToClipboard");

    // Track copy state
    let copy_state = use_state(|| CopyState::Idle);

    // Get window for clipboard API access
    let window = web_sys::window();

    // Default text values
    let button_text = props
        .copy_button_text
        .clone()
        .unwrap_or_else(|| "Copy".to_string());
    let success_text = props
        .copy_success_text
        .clone()
        .unwrap_or_else(|| "Copied".to_string());
    let error_text = props
        .copy_error_text
        .clone()
        .unwrap_or_else(|| "Failed to copy".to_string());

    // Handle click to copy
    let on_click = {
        let copy_text = props.copy_text.clone();
        let copy_state = copy_state.clone();
        let on_copy = props.on_copy.clone();
        let disabled = props.disabled;
        let window = window.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if disabled {
                return;
            }

            let copy_text = copy_text.clone();
            let copy_state = copy_state.clone();
            let on_copy = on_copy.clone();
            let window = window.clone();

            // Spawn async task to copy to clipboard
            wasm_bindgen_futures::spawn_local(async move {
                let result = copy_to_clipboard_async(&copy_text, window).await;

                match result {
                    Ok(_) => {
                        copy_state.set(CopyState::Success);
                        if let Some(callback) = &on_copy {
                            callback.emit(CustomEvent::new_non_cancelable(CopyDetail {
                                status: CopyStatus::Success,
                            }));
                        }

                        // Reset state after 2 seconds
                        let copy_state = copy_state.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_state.set(CopyState::Idle);
                        })
                        .forget();
                    }
                    Err(_) => {
                        copy_state.set(CopyState::Error);
                        if let Some(callback) = &on_copy {
                            callback.emit(CustomEvent::new_non_cancelable(CopyDetail {
                                status: CopyStatus::Error,
                            }));
                        }

                        // Reset state after 2 seconds
                        let copy_state = copy_state.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_state.set(CopyState::Idle);
                        })
                        .forget();
                    }
                }
            });
        })
    };

    // Build CSS classes
    let wrapper_classes = ClassBuilder::new()
        .add("awsui-copy-to-clipboard")
        .add(format!(
            "awsui-copy-to-clipboard-variant-{}",
            props.variant.as_str()
        ))
        .add_if(props.disabled, "awsui-copy-to-clipboard-disabled")
        .add_if(*copy_state == CopyState::Success, "awsui-copy-to-clipboard-success")
        .add_if(*copy_state == CopyState::Error, "awsui-copy-to-clipboard-error");

    let class = props.base.merge_classes(&wrapper_classes.build());

    // Determine button text based on state
    let display_text = match *copy_state {
        CopyState::Idle => &button_text,
        CopyState::Success => &success_text,
        CopyState::Error => &error_text,
    };

    // Build ARIA label
    let aria_label = props.aria_label.clone().or_else(|| {
        Some(format!(
            "{} {}",
            match *copy_state {
                CopyState::Idle => "Copy",
                CopyState::Success => "Copied",
                CopyState::Error => "Failed to copy",
            },
            if props.variant == CopyToClipboardVariant::Icon {
                "to clipboard"
            } else {
                ""
            }
        ))
    });

    // Render icon for icon variant
    let icon = if props.variant == CopyToClipboardVariant::Icon {
        Some(html! {
            <span class="awsui-copy-to-clipboard-icon">
                { match *copy_state {
                    CopyState::Success => "✓",
                    CopyState::Error => "✗",
                    CopyState::Idle => "📋",
                }}
            </span>
        })
    } else {
        None
    };

    html! {
        <span class={class}>
            <button
                class="awsui-copy-to-clipboard-button"
                type="button"
                disabled={props.disabled}
                onclick={on_click}
                aria-label={aria_label}
            >
                { icon }
                if props.variant != CopyToClipboardVariant::Icon {
                    <span class="awsui-copy-to-clipboard-text">
                        { display_text }
                    </span>
                }
            </button>
            if *copy_state != CopyState::Idle {
                <span class="awsui-copy-to-clipboard-status" role="status" aria-live="polite">
                    { display_text }
                </span>
            }
        </span>
    }
}

/// Async helper function to copy text to clipboard
async fn copy_to_clipboard_async(text: &str, window: Option<Window>) -> Result<(), JsValue> {
    let window = window.ok_or_else(|| JsValue::from_str("No window available"))?;

    let navigator = window.navigator();
    let clipboard = navigator.clipboard();

    // Use modern Clipboard API
    let promise = clipboard.write_text(text);
    JsFuture::from(promise).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_status_equality() {
        assert_eq!(CopyStatus::Success, CopyStatus::Success);
        assert_eq!(CopyStatus::Error, CopyStatus::Error);
        assert_ne!(CopyStatus::Success, CopyStatus::Error);
    }

    #[test]
    fn test_copy_detail_creation() {
        let detail = CopyDetail {
            status: CopyStatus::Success,
        };
        assert_eq!(detail.status, CopyStatus::Success);

        let detail = CopyDetail {
            status: CopyStatus::Error,
        };
        assert_eq!(detail.status, CopyStatus::Error);
    }

    #[test]
    fn test_copy_detail_clone() {
        let detail = CopyDetail {
            status: CopyStatus::Success,
        };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_variant_as_str() {
        assert_eq!(CopyToClipboardVariant::Button.as_str(), "button");
        assert_eq!(CopyToClipboardVariant::Icon.as_str(), "icon");
        assert_eq!(CopyToClipboardVariant::Inline.as_str(), "inline");
    }

    #[test]
    fn test_variant_default() {
        let variant = CopyToClipboardVariant::default();
        assert_eq!(variant, CopyToClipboardVariant::Button);
    }

    #[test]
    fn test_variant_equality() {
        assert_eq!(
            CopyToClipboardVariant::Button,
            CopyToClipboardVariant::Button
        );
        assert_ne!(
            CopyToClipboardVariant::Button,
            CopyToClipboardVariant::Icon
        );
    }

    #[test]
    fn test_copy_state_equality() {
        assert_eq!(CopyState::Idle, CopyState::Idle);
        assert_eq!(CopyState::Success, CopyState::Success);
        assert_eq!(CopyState::Error, CopyState::Error);
        assert_ne!(CopyState::Idle, CopyState::Success);
    }

    #[test]
    fn test_custom_event_copy_detail() {
        let detail = CopyDetail {
            status: CopyStatus::Success,
        };
        let event = CustomEvent::new_non_cancelable(detail);

        assert!(!event.cancelable);
        assert!(!event.default_prevented);
        assert_eq!(event.detail.status, CopyStatus::Success);
    }
}
