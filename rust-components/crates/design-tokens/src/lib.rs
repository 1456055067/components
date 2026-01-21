// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Cloudscape Design Tokens
//!
//! This crate provides type-safe access to Cloudscape design tokens,
//! including colors, spacing, typography, borders, shadows, and motion tokens.
//!
//! Design tokens are shared between React and Rust/Yew implementations
//! and are generated from the style-dictionary source.

use serde::{Deserialize, Serialize};

pub mod color;
pub mod spacing;
pub mod typography;
pub mod borders;
pub mod shadows;
pub mod motion;

pub use color::ColorTokens;
pub use spacing::SpacingTokens;
pub use typography::TypographyTokens;
pub use borders::BorderTokens;
pub use shadows::ShadowTokens;
pub use motion::MotionTokens;

/// Main design tokens structure containing all token categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTokens {
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub typography: TypographyTokens,
    pub borders: BorderTokens,
    pub shadows: ShadowTokens,
    pub motion: MotionTokens,
}

impl DesignTokens {
    /// Creates default design tokens from the Visual Refresh theme
    pub fn default() -> Self {
        Self {
            colors: ColorTokens::default(),
            spacing: SpacingTokens::default(),
            typography: TypographyTokens::default(),
            borders: BorderTokens::default(),
            shadows: ShadowTokens::default(),
            motion: MotionTokens::default(),
        }
    }

    /// Loads design tokens from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serializes design tokens to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Mode identifier for multi-mode tokens (color, density, motion)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    /// Light color mode (default)
    Light,
    /// Dark color mode
    Dark,
    /// Comfortable density mode (default)
    Comfortable,
    /// Compact density mode
    Compact,
    /// Default motion mode (animations enabled)
    MotionDefault,
    /// Motion disabled mode (for users who prefer reduced motion)
    MotionDisabled,
}

impl Mode {
    /// Returns the CSS class name for this mode
    pub fn css_class(&self) -> &'static str {
        match self {
            Mode::Light => "",
            Mode::Dark => "awsui-dark-mode",
            Mode::Comfortable => "",
            Mode::Compact => "awsui-compact-mode",
            Mode::MotionDefault => "",
            Mode::MotionDisabled => "awsui-motion-disabled",
        }
    }

    /// Returns true if this is the default mode
    pub fn is_default(&self) -> bool {
        matches!(self, Mode::Light | Mode::Comfortable | Mode::MotionDefault)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_css_class() {
        assert_eq!(Mode::Light.css_class(), "");
        assert_eq!(Mode::Dark.css_class(), "awsui-dark-mode");
        assert_eq!(Mode::Compact.css_class(), "awsui-compact-mode");
    }

    #[test]
    fn test_mode_is_default() {
        assert!(Mode::Light.is_default());
        assert!(Mode::Comfortable.is_default());
        assert!(Mode::MotionDefault.is_default());
        assert!(!Mode::Dark.is_default());
        assert!(!Mode::Compact.is_default());
        assert!(!Mode::MotionDisabled.is_default());
    }

    #[test]
    fn test_design_tokens_serialization() {
        let tokens = DesignTokens::default();
        let json = tokens.to_json().expect("Failed to serialize");
        let deserialized = DesignTokens::from_json(&json).expect("Failed to deserialize");

        // Basic sanity check that round-trip works
        assert!(deserialized.to_json().is_ok());
    }
}
