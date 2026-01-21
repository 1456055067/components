// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Shadow design tokens

use serde::{Deserialize, Serialize};

/// Shadow tokens for box shadows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowTokens {
    // Placeholder - will be generated
}

impl Default for ShadowTokens {
    fn default() -> Self {
        Self {}
    }
}
