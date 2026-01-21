// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * Token Generator for Rust/Yew Components
 *
 * This script reads compiled design tokens from style-dictionary and generates:
 * 1. CSS custom properties for runtime theming
 * 2. Rust type definitions and constants
 *
 * The generated files maintain parity with React components while being
 * optimized for Rust/WASM performance.
 */

const { promises: fs } = require('fs');
const path = require('path');
const { task } = require('../utils/gulp-utils');

const RUST_OUTPUT_DIR = path.join(__dirname, '../../rust-components/crates/design-tokens/src/generated');
const CSS_OUTPUT_DIR = path.join(__dirname, '../../rust-components/dist/styles');
const STYLE_DICTIONARY_ROOT = path.join(__dirname, '../../lib/style-dictionary');

/**
 * Convert camelCase token name to kebab-case CSS variable name
 */
function toKebabCase(str) {
  return str.replace(/[A-Z]/g, m => '-' + m.toLowerCase());
}

/**
 * Convert camelCase to PascalCase for Rust enum variants
 */
function toPascalCase(str) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

/**
 * Sanitize token name for Rust enum variant
 * Rust keywords and special cases need to be handled
 */
function toRustVariant(tokenName) {
  // Handle numbers at start (not valid in Rust)
  if (/^\d/.test(tokenName)) {
    tokenName = 'N' + tokenName;
  }

  // Convert to PascalCase
  let variant = tokenName
    .split(/(?=[A-Z])/)
    .map(part => part.charAt(0).toUpperCase() + part.slice(1))
    .join('');

  // Handle Rust keywords
  const keywords = ['type', 'ref', 'box', 'self', 'super', 'impl', 'trait'];
  if (keywords.includes(variant.toLowerCase())) {
    variant = variant + '_';
  }

  return variant;
}

/**
 * Resolve token references like {colorPrimary600}
 */
function resolveTokenValue(value, allTokens, mode = 'light') {
  if (typeof value === 'string' && value.startsWith('{') && value.endsWith('}')) {
    const refName = value.slice(1, -1);
    const refToken = allTokens[refName];

    if (!refToken) {
      console.warn(`Warning: Token reference not found: ${value}`);
      return value;
    }

    return resolveTokenValue(refToken, allTokens, mode);
  }

  if (typeof value === 'object' && value !== null) {
    // Handle mode-specific values
    if (value[mode]) {
      return resolveTokenValue(value[mode], allTokens, mode);
    }
    // Fallback to light mode
    if (value.light) {
      return resolveTokenValue(value.light, allTokens, mode);
    }
  }

  return value;
}

/**
 * Load and parse design tokens from compiled style-dictionary
 */
async function loadDesignTokens() {
  console.log('Loading design tokens from:', STYLE_DICTIONARY_ROOT);

  // Import the visual-refresh theme (primary theme)
  const visualRefreshPath = path.join(STYLE_DICTIONARY_ROOT, 'visual-refresh/index.js');
  const metadataPath = path.join(STYLE_DICTIONARY_ROOT, 'visual-refresh/metadata.js');

  try {
    // Dynamic import of ES modules
    // eslint-disable-next-line no-unsanitized/method
    const visualRefresh = await import(`file://${visualRefreshPath}`);
    // eslint-disable-next-line no-unsanitized/method
    const metadataModule = await import(`file://${metadataPath}`);

    const theme = visualRefresh.default;
    const metadata = metadataModule.default;

    console.log(`Loaded theme with ${Object.keys(theme.tokens || {}).length} tokens`);

    return { theme, metadata };
  } catch (error) {
    console.error('Error loading design tokens:', error);
    throw error;
  }
}

/**
 * Generate CSS custom properties from tokens
 */
// eslint-disable-next-line @typescript-eslint/no-unused-vars
function generateCSS(theme, metadata) {
  const tokens = theme.tokens || {};
  const modeTokens = theme.modeTokens || {};

  let css = `/*
 * Cloudscape Design Tokens - CSS Custom Properties
 * Generated from style-dictionary/visual-refresh
 * DO NOT EDIT MANUALLY
 */

:root {
`;

  // Generate light mode (default) tokens
  const lightTokens = {};
  Object.entries(tokens).forEach(([name, value]) => {
    const cssVar = `--awsui-${toKebabCase(name)}`;
    const resolvedValue = resolveTokenValue(value, tokens, 'light');
    lightTokens[name] = resolvedValue;

    css += `  ${cssVar}: ${resolvedValue};\n`;
  });

  css += `}\n\n`;

  // Generate dark mode overrides
  css += `/* Dark Mode */\n`;
  css += `:root[data-awsui-theme="dark"],\n`;
  css += `.awsui-dark-mode {\n`;

  Object.entries(tokens).forEach(([name, value]) => {
    if (typeof value === 'object' && value.dark) {
      const cssVar = `--awsui-${toKebabCase(name)}`;
      const resolvedValue = resolveTokenValue(value, tokens, 'dark');
      css += `  ${cssVar}: ${resolvedValue};\n`;
    }
  });

  css += `}\n\n`;

  // Auto dark mode with prefers-color-scheme
  css += `/* Auto Dark Mode */\n`;
  css += `@media (prefers-color-scheme: dark) {\n`;
  css += `  :root:not([data-awsui-theme="light"]) {\n`;

  Object.entries(tokens).forEach(([name, value]) => {
    if (typeof value === 'object' && value.dark) {
      const cssVar = `--awsui-${toKebabCase(name)}`;
      const resolvedValue = resolveTokenValue(value, tokens, 'dark');
      css += `    ${cssVar}: ${resolvedValue};\n`;
    }
  });

  css += `  }\n`;
  css += `}\n\n`;

  // Generate compact mode overrides
  if (Object.keys(modeTokens).some(key => key.includes('compact'))) {
    css += `/* Compact Mode */\n`;
    css += `.awsui-compact-mode {\n`;

    Object.entries(modeTokens).forEach(([name, value]) => {
      if (name.includes('compact')) {
        const cssVar = `--awsui-${toKebabCase(name)}`;
        css += `  ${cssVar}: ${value};\n`;
      }
    });

    css += `}\n\n`;
  }

  // Add component-specific classes for common patterns
  css += `/* Component Utilities */\n`;
  css += `.awsui-motion-disabled * {\n`;
  css += `  animation-duration: 0s !important;\n`;
  css += `  transition-duration: 0s !important;\n`;
  css += `}\n`;

  return css;
}

/**
 * Generate Rust token type definitions
 */
function generateRustTypes(theme, metadata) {
  const tokens = theme.tokens || {};
  const tokensByCategory = {};

  // Categorize tokens by prefix
  Object.keys(tokens).forEach(name => {
    const match = name.match(/^(color|spacing|size|font|border|shadow|motion)/i);
    const category = match ? match[1].toLowerCase() : 'other';

    if (!tokensByCategory[category]) {
      tokensByCategory[category] = [];
    }
    tokensByCategory[category].push(name);
  });

  let rust = `// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Auto-generated design tokens from style-dictionary
//! DO NOT EDIT MANUALLY

use std::fmt;

`;

  // Generate enum for each category
  Object.entries(tokensByCategory).forEach(([category, tokenNames]) => {
    const enumName = `${toPascalCase(category)}Token`;

    rust += `/// ${toPascalCase(category)} design tokens\n`;
    rust += `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n`;
    rust += `pub enum ${enumName} {\n`;

    tokenNames.forEach(name => {
      const meta = metadata[name];
      if (meta?.description) {
        rust += `    /// ${meta.description}\n`;
      }
      rust += `    ${toRustVariant(name)},\n`;
    });

    rust += `}\n\n`;

    // Implement CSS var name method
    rust += `impl ${enumName} {\n`;
    rust += `    /// Get the CSS custom property name for this token\n`;
    rust += `    pub fn css_var_name(&self) -> &'static str {\n`;
    rust += `        match self {\n`;

    tokenNames.forEach(name => {
      const cssVar = `--awsui-${toKebabCase(name)}`;
      rust += `            Self::${toRustVariant(name)} => "${cssVar}",\n`;
    });

    rust += `        }\n`;
    rust += `    }\n\n`;

    // Implement Display trait
    rust += `    /// Get the CSS var() function for use in styles\n`;
    rust += `    pub fn css_var(&self) -> String {\n`;
    rust += `        format!("var({})", self.css_var_name())\n`;
    rust += `    }\n`;
    rust += `}\n\n`;

    rust += `impl fmt::Display for ${enumName} {\n`;
    rust += `    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n`;
    rust += `        write!(f, "{}", self.css_var())\n`;
    rust += `    }\n`;
    rust += `}\n\n`;
  });

  return rust;
}

/**
 * Generate Rust module file that exports all token types
 */
function generateRustMod() {
  return `// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Auto-generated design tokens module
//! DO NOT EDIT MANUALLY

pub mod tokens;
pub use tokens::*;

/// Design token version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
`;
}

/**
 * Main generation task
 */
async function generateRustTokens() {
  console.log('Starting Rust token generation...');

  // Ensure output directories exist
  await fs.mkdir(RUST_OUTPUT_DIR, { recursive: true });
  await fs.mkdir(CSS_OUTPUT_DIR, { recursive: true });

  // Load design tokens
  const { theme, metadata } = await loadDesignTokens();

  // Generate CSS
  console.log('Generating CSS custom properties...');
  const css = generateCSS(theme, metadata);
  await fs.writeFile(path.join(CSS_OUTPUT_DIR, 'design-tokens.css'), css, 'utf-8');
  console.log(`✓ CSS written to ${path.join(CSS_OUTPUT_DIR, 'design-tokens.css')}`);

  // Generate Rust types
  console.log('Generating Rust token types...');
  const rustTypes = generateRustTypes(theme, metadata);
  await fs.writeFile(path.join(RUST_OUTPUT_DIR, 'tokens.rs'), rustTypes, 'utf-8');
  console.log(`✓ Rust types written to ${path.join(RUST_OUTPUT_DIR, 'tokens.rs')}`);

  // Generate mod.rs
  const rustMod = generateRustMod();
  await fs.writeFile(path.join(RUST_OUTPUT_DIR, 'mod.rs'), rustMod, 'utf-8');
  console.log(`✓ Rust module written to ${path.join(RUST_OUTPUT_DIR, 'mod.rs')}`);

  console.log('Token generation complete!');
}

module.exports = task('generate-rust-tokens', generateRustTokens);
