// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * CSS Integration for Rust Components
 *
 * Extracts and bundles CSS from React component builds for use with Rust/WASM components.
 * The Rust components use the same CSS classes as React, so we can reuse the compiled styles.
 */

import { promises as fs } from 'fs';
import path from 'path';
import { dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = path.resolve(__dirname, '../..');

const REACT_BUILD_DIR = path.join(rootDir, 'lib/components');
const RUST_STYLES_OUTPUT = path.join(rootDir, 'rust-components/dist/styles');
const RUST_DIST_OUTPUT = path.join(rootDir, 'lib/rust-components');

/**
 * Component list to extract CSS for
 * These match the Rust component implementations
 */
const COMPONENTS = [
  // Basic components
  'alert',
  'badge',
  'box',
  'button',
  'icon',
  'spinner',

  // Form components
  'autosuggest',
  'checkbox',
  'date-picker',
  'date-range-picker',
  'file-upload',
  'form-field',
  'input',
  'multiselect',
  'radio-group',
  'select',
  'textarea',
  'tiles',
  'toggle',

  // Layout components
  'app-layout',
  'column-layout',
  'container',
  'content-layout',
  'header',
  'space-between',

  // Navigation components
  'breadcrumb-group',
  'button-dropdown',
  'link',
  'pagination',
  'side-navigation',
  'tabs',
  'top-navigation',

  // Data display components
  'cards',
  'expandable-section',
  'key-value-pairs',
  'status-indicator',
  'table',
  'text-content',
  'token-group',

  // Overlay components
  'drawer',
  'modal',
  'popover',

  // Notification components
  'flashbar',

  // Additional components
  'copy-to-clipboard',
  'progress-bar',
];

/**
 * Extracts CSS content from React component CSS.js files
 */
async function extractCssFromJsFile(filePath) {
  try {
    const content = await fs.readFile(filePath, 'utf-8');

    // CSS is exported as a string in the format:
    // export default "...css content...";
    // or: module.exports = "...css content...";

    const cssMatch = content.match(/(?:export default|module\.exports\s*=)\s*["'](.+?)["']/s);

    if (cssMatch && cssMatch[1]) {
      // Unescape the CSS string
      let css = cssMatch[1];
      css = css.replace(/\\n/g, '\n');
      css = css.replace(/\\"/g, '"');
      css = css.replace(/\\'/g, "'");
      css = css.replace(/\\\\/g, '\\');
      return css;
    }

    return null;
  } catch (error) {
    console.warn(`Could not extract CSS from ${filePath}:`, error.message);
    return null;
  }
}

/**
 * Extracts internal/shared CSS used across components
 */
async function extractInternalStyles() {
  const internalStyles = [];

  // List of internal style files to extract
  const internalFiles = [
    'internal/styles/global-styles.css.js',
    'internal/base-component/styles.css.js',
    'internal/components/button-trigger/styles.css.js',
    'internal/components/dropdown/styles.css.js',
    'internal/components/option/styles.css.js',
    'internal/components/portal/styles.css.js',
    'internal/components/transition/styles.css.js',
  ];

  for (const file of internalFiles) {
    const filePath = path.join(REACT_BUILD_DIR, file);
    try {
      const css = await extractCssFromJsFile(filePath);
      if (css) {
        internalStyles.push(`/* ${file} */\n${css}`);
      }
    } catch {
      // Skip files that don't exist
    }
  }

  return internalStyles.join('\n\n');
}

/**
 * Extracts global styles
 */
async function extractGlobalStyles() {
  const globalStylesPath = path.join(REACT_BUILD_DIR, 'internal/styles/global-styles.css.js');

  try {
    const css = await extractCssFromJsFile(globalStylesPath);
    if (css) {
      return css;
    }
  } catch {
    console.warn('Global styles not found, will generate minimal defaults');
  }

  // Fallback: minimal global styles
  return `
/* Cloudscape Global Styles */
:root {
  --awsui-color-text-body-default: #16191f;
  --awsui-color-background-container-content: #ffffff;
  --awsui-space-scaled-xs: 4px;
  --awsui-space-scaled-s: 8px;
  --awsui-space-scaled-m: 12px;
  --awsui-space-scaled-l: 16px;
  --awsui-space-scaled-xl: 20px;
  --awsui-font-family-base: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
}

body {
  margin: 0;
  padding: 0;
  font-family: var(--awsui-font-family-base);
  color: var(--awsui-color-text-body-default);
  background: var(--awsui-color-background-container-content);
}

*, *::before, *::after {
  box-sizing: border-box;
}
`;
}

/**
 * Builds component-specific CSS bundle
 */
export async function buildRustStyles() {
  console.log('🎨 Building CSS for Rust components...');

  try {
    // First, generate design tokens from style-dictionary
    console.log('   🔧 Generating design tokens...');
    const generateTokens = await import('./generate-rust-tokens.js');
    await generateTokens.default();

    // Ensure output directories exist
    await fs.mkdir(RUST_STYLES_OUTPUT, { recursive: true });
    await fs.mkdir(RUST_DIST_OUTPUT, { recursive: true });

    let combinedCss = '';

    // Add generated design tokens first
    const designTokensPath = path.join(RUST_STYLES_OUTPUT, 'design-tokens.css');
    try {
      const designTokensCss = await fs.readFile(designTokensPath, 'utf-8');
      combinedCss += designTokensCss + '\n\n';
      console.log('   ✓ Included design tokens');
    } catch {
      console.warn('   ⚠️  Design tokens not found, will use fallback');
    }

    // Add global styles
    const globalStyles = await extractGlobalStyles();
    combinedCss += '/* Global Styles */\n' + globalStyles + '\n\n';

    // Add internal/shared styles
    console.log('   🔧 Extracting internal styles...');
    const internalStyles = await extractInternalStyles();
    if (internalStyles) {
      combinedCss += '/* Internal/Shared Styles */\n' + internalStyles + '\n\n';
      console.log('   ✓ Included internal styles');
    }

    // Extract and combine component styles
    let componentCount = 0;
    for (const component of COMPONENTS) {
      // Try .scoped.css file (contains actual CSS)
      const scopedCssPath = path.join(REACT_BUILD_DIR, component, 'styles.scoped.css');

      try {
        const componentCss = await fs.readFile(scopedCssPath, 'utf-8');

        if (componentCss) {
          combinedCss += `/* Component: ${component} */\n`;
          combinedCss += componentCss + '\n\n';
          componentCount++;
          console.log(`   ✓ Extracted CSS for ${component}`);
        }
      } catch {
        console.log(`   ⊘ Skipped ${component} (not built yet)`);
      }
    }

    if (componentCount === 0) {
      console.warn('⚠️  No React component CSS found. Build React components first with: gulp quick-build');

      // Create placeholder CSS
      combinedCss += generatePlaceholderStyles();
    }

    // Deduplicate CSS before writing
    combinedCss = deduplicateCss(combinedCss);

    // Write combined CSS bundle
    const outputPath = path.join(RUST_STYLES_OUTPUT, 'cloudscape-components.css');
    await fs.writeFile(outputPath, combinedCss, 'utf-8');
    console.log(`   Wrote ${outputPath}`);

    // Copy to public dist
    const publicPath = path.join(RUST_DIST_OUTPUT, 'cloudscape-components.css');
    await fs.copyFile(outputPath, publicPath);
    console.log(`   Copied to ${publicPath}`);

    console.log(`✅ CSS bundle created (${componentCount} components)`);

    // Generate minified version
    await createMinifiedCss(combinedCss);
  } catch (error) {
    console.error('❌ CSS build failed:', error.message);
    throw error;
  }
}

/**
 * Removes duplicate comments and fixes stylelint issues
 */
function deduplicateCss(css) {
  // Simpler approach: just remove all stylelint-disable comments
  // They're not needed in the final bundled CSS file
  css = css.replace(/\/\*\s*stylelint-disable[^*]*\*\/\s*/g, '');

  // Remove duplicate consecutive copyright headers
  const lines = css.split('\n');
  const dedupedLines = [];
  let lastWasCopyright = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const isCopyright = line.includes('Copyright Amazon.com');

    // Skip duplicate copyright lines
    if (isCopyright && lastWasCopyright) {
      // Skip lines until we find the end of this copyright block
      while (i < lines.length && !lines[i].includes('*/')) {
        i++;
      }
      continue;
    }

    dedupedLines.push(line);
    lastWasCopyright = isCopyright;
  }

  return dedupedLines.join('\n');
}

/**
 * Creates minified CSS version
 */
async function createMinifiedCss(css) {
  // Simple minification (remove comments, extra whitespace)
  let minified = css;

  // Remove comments
  minified = minified.replace(/\/\*[\s\S]*?\*\//g, '');

  // Remove extra whitespace
  minified = minified.replace(/\s+/g, ' ');
  minified = minified.replace(/\s*([{}:;,])\s*/g, '$1');
  minified = minified.trim();

  // Write minified version
  const minPath = path.join(RUST_STYLES_OUTPUT, 'cloudscape-components.min.css');
  await fs.writeFile(minPath, minified, 'utf-8');

  const publicMinPath = path.join(RUST_DIST_OUTPUT, 'cloudscape-components.min.css');
  await fs.copyFile(minPath, publicMinPath);

  const originalSize = (css.length / 1024).toFixed(2);
  const minifiedSize = (minified.length / 1024).toFixed(2);
  const savings = ((1 - minified.length / css.length) * 100).toFixed(1);

  console.log(`   Minified: ${originalSize}KB → ${minifiedSize}KB (${savings}% smaller)`);
}

/**
 * Generates placeholder styles when React CSS is not available
 */
function generatePlaceholderStyles() {
  return `
/* Placeholder Component Styles */
/* These are basic styles to make components visible during development */
/* Build React components first to get the real Cloudscape styles */

.awsui-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  line-height: 16px;
}

.awsui-badge-color-blue { background: #0972d3; color: white; }
.awsui-badge-color-grey { background: #687078; color: white; }
.awsui-badge-color-green { background: #037f0c; color: white; }
.awsui-badge-color-red { background: #d91515; color: white; }

.awsui-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 6px 16px;
  border: 1px solid #687078;
  border-radius: 4px;
  background: white;
  color: #16191f;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
}

.awsui-button:hover { background: #f5f5f5; }
.awsui-button:active { background: #e9ebed; }
.awsui-button-variant-primary { background: #0972d3; color: white; border-color: #0972d3; }
.awsui-button-variant-primary:hover { background: #0a5bb3; }
.awsui-button-disabled { opacity: 0.5; cursor: not-allowed; }

.awsui-spinner {
  display: inline-block;
  width: 24px;
  height: 24px;
  position: relative;
}

.awsui-spinner-rotator {
  animation: awsui-spin 0.8s linear infinite;
}

.awsui-spinner-circle {
  display: block;
  width: 100%;
  height: 100%;
  border: 3px solid #e9ebed;
  border-top-color: #0972d3;
  border-radius: 50%;
}

@keyframes awsui-spin {
  to { transform: rotate(360deg); }
}

.awsui-box {
  display: block;
}
`;
}

/**
 * Watches for CSS changes and rebuilds
 */
export function watchRustStyles() {
  console.log('👀 Watching for CSS changes...');

  const { watch } = require('gulp');

  return watch(
    ['lib/components/**/styles.css.js', 'lib/components/internal/styles/**/*.css.js'],
    { cwd: rootDir },
    buildRustStyles
  );
}

/**
 * Cleans generated CSS files
 */
export async function cleanRustStyles() {
  try {
    await fs.rm(RUST_STYLES_OUTPUT, { recursive: true, force: true });

    const cssFiles = [
      path.join(RUST_DIST_OUTPUT, 'cloudscape-components.css'),
      path.join(RUST_DIST_OUTPUT, 'cloudscape-components.min.css'),
    ];

    for (const file of cssFiles) {
      await fs.rm(file, { force: true });
    }

    console.log('✅ CSS files cleaned');
  } catch {
    // Ignore if files don't exist
  }
}

export default {
  build: buildRustStyles,
  watch: watchRustStyles,
  clean: cleanRustStyles,
};
