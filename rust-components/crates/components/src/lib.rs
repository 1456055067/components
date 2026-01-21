// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Cloudscape Design System Components for Yew
//!
//! This library provides Rust/WASM implementations of Cloudscape components
//! using the Yew framework, optimized for performance and security.

use wasm_bindgen::prelude::*;

// Re-export design tokens
pub use cloudscape_design_tokens as tokens;

// Component modules
pub mod badge;
pub mod spinner;
pub mod box_component;
pub mod button;
pub mod alert;
pub mod input;
pub mod checkbox;
pub mod radio_group;
pub mod toggle;
pub mod header;
pub mod textarea;
pub mod container;
pub mod select;
pub mod link;
pub mod icon;

// Internal utilities
pub mod internal;

// Re-export components for convenient access
pub use badge::{Badge, BadgeColor, BadgeProps};
pub use spinner::{Spinner, SpinnerProps, SpinnerSize, SpinnerVariant};
pub use box_component::{Box, BoxProps, BoxVariant, SpacingSize};
pub use button::{Button, ButtonProps, ButtonVariant, FormAction, IconAlign};
pub use alert::{Alert, AlertProps, AlertType, AlertI18nStrings, DismissDetail};
pub use input::{Input, InputProps, InputType, InputChangeDetail};
pub use checkbox::{Checkbox, CheckboxProps, CheckboxChangeDetail};
pub use radio_group::{RadioGroup, RadioGroupProps, RadioGroupItem, RadioGroupChangeDetail, RadioGroupDirection};
pub use toggle::{Toggle, ToggleProps, ToggleChangeDetail};
pub use header::{Header, HeaderProps, HeaderVariant};
pub use textarea::{Textarea, TextareaProps, TextareaChangeDetail};
pub use container::{Container, ContainerProps, ContainerVariant, Media, MediaPosition};
pub use select::{Select, SelectProps, SelectOption, SelectChangeDetail};
pub use icon::{Icon, IconProps, IconSize, IconVariant};
pub use link::{Link, LinkProps, LinkVariant, LinkFontSize, LinkColor, FollowDetail, FollowEvent};

// Re-export commonly used internal types
pub use internal::CustomEvent;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the Cloudscape components library
///
/// This should be called once when your application starts.
/// It sets up any necessary global state or configuration.
#[wasm_bindgen(start)]
pub fn init() {
    // Initialization code can be added here as needed
    // For example: panic hooks, logging setup, etc.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }
}
