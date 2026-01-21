// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * Watch Mode for Rust Design Token Generation
 *
 * Automatically regenerates design tokens when style-dictionary files change.
 * This enables a fast development workflow for the Rust/WASM components.
 */

import chokidar from 'chokidar';
import path from 'path';
import { dirname } from 'path';
import { fileURLToPath } from 'url';

import generateTokens from './generate-rust-tokens.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = path.resolve(__dirname, '../..');

// Directories to watch for changes
const WATCH_PATHS = [
  path.join(rootDir, 'lib/style-dictionary/**/*.js'),
  path.join(rootDir, 'style-dictionary/**/*.ts'),
  path.join(rootDir, 'style-dictionary/**/*.json'),
];

// Debounce delay to avoid multiple rapid rebuilds
const DEBOUNCE_MS = 500;

let regenerateTimeout = null;
let isGenerating = false;

/**
 * Regenerate design tokens with debouncing
 */
function regenerateTokens(changedFile) {
  // Clear existing timeout
  if (regenerateTimeout) {
    clearTimeout(regenerateTimeout);
  }

  // Debounce the regeneration
  regenerateTimeout = setTimeout(async () => {
    if (isGenerating) {
      console.log('⏳ Token generation already in progress, skipping...');
      return;
    }

    try {
      isGenerating = true;
      const relativePath = path.relative(rootDir, changedFile);

      console.log('');
      console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
      console.log(`📝 Change detected: ${relativePath}`);
      console.log('🔄 Regenerating design tokens...');

      const startTime = Date.now();
      await generateTokens();
      const duration = Date.now() - startTime;

      console.log(`✅ Tokens regenerated in ${duration}ms`);
      console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
      console.log('');
    } catch (error) {
      console.error('❌ Token generation failed:', error.message);
      console.error(error.stack);
    } finally {
      isGenerating = false;
    }
  }, DEBOUNCE_MS);
}

/**
 * Start watching for file changes
 */
export function watchRustTokens() {
  console.log('');
  console.log('👀 Watching for design token changes...');
  console.log('');
  console.log('Watching:');
  WATCH_PATHS.forEach(p => {
    console.log(`   • ${path.relative(rootDir, p.replace('**/*', ''))}`);
  });
  console.log('');
  console.log('Press Ctrl+C to stop watching');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('');

  const watcher = chokidar.watch(WATCH_PATHS, {
    ignoreInitial: true,
    persistent: true,
    awaitWriteFinish: {
      stabilityThreshold: 200,
      pollInterval: 100,
    },
    ignored: ['**/node_modules/**', '**/target/**', '**/dist/**', '**/.git/**'],
  });

  watcher.on('change', regenerateTokens);
  watcher.on('add', regenerateTokens);
  watcher.on('unlink', filePath => {
    const relativePath = path.relative(rootDir, filePath);
    console.log(`🗑️  File deleted: ${relativePath}`);
    console.log('   Consider regenerating tokens manually if this affects token generation');
  });

  watcher.on('error', error => {
    console.error('❌ Watcher error:', error);
  });

  // Handle graceful shutdown
  process.on('SIGINT', () => {
    console.log('');
    console.log('👋 Stopping token watcher...');
    watcher.close();
    process.exit(0);
  });

  process.on('SIGTERM', () => {
    watcher.close();
    process.exit(0);
  });

  return watcher;
}

/**
 * CLI entry point
 */
if (import.meta.url === `file://${process.argv[1]}`) {
  watchRustTokens();
}

export default watchRustTokens;
