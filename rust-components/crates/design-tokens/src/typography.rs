// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Typography design tokens

use serde::{Deserialize, Serialize};

/// Typography tokens for font sizes, weights, and line heights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyTokens {
    // Placeholder - will be generated
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {}
    }
}
