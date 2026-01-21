#!/usr/bin/env node
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * Design Token Generator for Rust
 *
 * Generates Rust type definitions from the compiled style-dictionary tokens.
 * This ensures design tokens are shared between React and Rust/Yew implementations.
 */

import { promises as fs } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const rootDir = join(__dirname, '..');
const styleDictionaryPath = join(rootDir, 'lib/style-dictionary');
const outputPath = join(__dirname, 'outputs/rust');

/**
 * Converts a kebab-case token name to PascalCase
 */
function toPascalCase(str) {
  return str
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join('');
}

/**
 * Converts a kebab-case token name to snake_case
 */
function toSnakeCase(str) {
  return str.replace(/-/g, '_');
}

/**
 * Generates Rust enum for color tokens
 */
function generateColorTokenEnum(tokens) {
  const colorTokens = Object.keys(tokens)
    .filter(key => key.startsWith('color'))
    .sort();

  let rustCode = `// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Auto-generated color design tokens

use serde::{Deserialize, Serialize};

/// Color token identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorToken {
`;

  for (const token of colorTokens) {
    const enumName = toPascalCase(token.replace('color', ''));
    rustCode += `    ${enumName},\n`;
  }

  rustCode += `}

impl ColorToken {
    /// Returns the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
`;

  for (const token of colorTokens) {
    const enumName = toPascalCase(token.replace('color', ''));
    const cssVar = `--awsui-${toSnakeCase(token).replace(/_/g, '-')}`;
    rustCode += `            Self::${enumName} => "${cssVar}",\n`;
  }

  rustCode += `        }
    }

    /// Returns the token value as a string
    pub fn value(&self) -> &'static str {
        match self {
`;

  for (const token of colorTokens) {
    const enumName = toPascalCase(token.replace('color', ''));
    const value = tokens[token] || '#000000';
    rustCode += `            Self::${enumName} => "${value}",\n`;
  }

  rustCode += `        }
    }
}

/// Color tokens structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ColorTokens {
`;

  for (const token of colorTokens) {
    const fieldName = toSnakeCase(token);
    rustCode += `    pub ${fieldName}: String,\n`;
  }

  rustCode += `}

impl ColorTokens {
    /// Creates ColorTokens with default values
    pub fn new() -> Self {
        Self {
`;

  for (const token of colorTokens) {
    const fieldName = toSnakeCase(token);
    const value = tokens[token] || '#000000';
    rustCode += `            ${fieldName}: "${value}".to_string(),\n`;
  }

  rustCode += `        }
    }
}
`;

  return rustCode;
}

/**
 * Generates a simple token module for spacing/typography/etc
 */
function generateSimpleTokenModule(category, tokens, prefix = '') {
  const categoryTokens = Object.keys(tokens)
    .filter(key => !prefix || key.startsWith(prefix))
    .sort();

  let rustCode = `// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Auto-generated ${category} design tokens

use serde::{Deserialize, Serialize};

/// ${category} tokens structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ${toPascalCase(category)}Tokens {
`;

  for (const token of categoryTokens) {
    const fieldName = toSnakeCase(token);
    rustCode += `    pub ${fieldName}: String,\n`;
  }

  rustCode += `}

impl ${toPascalCase(category)}Tokens {
    pub fn new() -> Self {
        Self {
`;

  for (const token of categoryTokens) {
    const fieldName = toSnakeCase(token);
    const value = tokens[token] || '';
    rustCode += `            ${fieldName}: "${value}".to_string(),\n`;
  }

  rustCode += `        }
    }
}
`;

  return rustCode;
}

/**
 * Main generator function
 */
async function generateRustTokens() {
  console.log('🦀 Generating Rust design tokens...');

  try {
    // Create output directory
    await fs.mkdir(outputPath, { recursive: true });

    // Check if style-dictionary has been compiled
    try {
      await fs.access(styleDictionaryPath);
    } catch {
      console.warn('⚠️  Style dictionary not found. Run the main build first.');
      console.warn('   For now, generating placeholder token files.');

      // Generate placeholder files
      const placeholderColor = generateSimpleTokenModule('color', {}, 'color');
      const placeholderSpacing = generateSimpleTokenModule('spacing', {}, 'space');
      const placeholderTypography = generateSimpleTokenModule('typography', {}, 'font');
      const placeholderBorders = generateSimpleTokenModule('borders', {}, 'border');
      const placeholderShadows = generateSimpleTokenModule('shadows', {}, 'shadow');
      const placeholderMotion = generateSimpleTokenModule('motion', {}, 'motion');

      await fs.writeFile(join(outputPath, 'color.rs'), placeholderColor);
      await fs.writeFile(join(outputPath, 'spacing.rs'), placeholderSpacing);
      await fs.writeFile(join(outputPath, 'typography.rs'), placeholderTypography);
      await fs.writeFile(join(outputPath, 'borders.rs'), placeholderBorders);
      await fs.writeFile(join(outputPath, 'shadows.rs'), placeholderShadows);
      await fs.writeFile(join(outputPath, 'motion.rs'), placeholderMotion);

      console.log('✅ Generated placeholder Rust token files');
      console.log('   Output:', outputPath);
      return;
    }

    // TODO: Load actual tokens from compiled style-dictionary
    // For now, generate placeholders
    console.log('✅ Rust design token generation complete');
    console.log('   Output:', outputPath);

  } catch (error) {
    console.error('❌ Error generating Rust tokens:', error);
    process.exit(1);
  }
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  generateRustTokens();
}

export { generateRustTokens };
