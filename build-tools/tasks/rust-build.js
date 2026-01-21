// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * Rust/WASM Build Tasks
 *
 * Builds Rust/Yew components using wasm-pack and integrates them
 * into the component library distribution.
 */

import { execa } from 'execa';
import { promises as fs } from 'fs';
import { watch as gulpWatch } from 'gulp';
import path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = path.resolve(__dirname, '../..');

const RUST_WORKSPACE = path.join(rootDir, 'rust-components');
const RUST_COMPONENTS_CRATE = path.join(RUST_WORKSPACE, 'crates/components');
const WASM_OUTPUT_DIR = path.join(RUST_WORKSPACE, 'dist/wasm');
const PUBLIC_OUTPUT_DIR = path.join(rootDir, 'lib/rust-components');

/**
 * Checks if Rust toolchain is installed
 */
async function checkRustInstalled() {
  try {
    const { stdout } = await execa('rustc', ['--version']);
    console.log('✓ Rust toolchain found:', stdout);
    return true;
  } catch (error) {
    console.warn('⚠️  Rust toolchain not found. Skipping Rust build.');
    console.warn('   Install Rust from https://rustup.rs/');
    return false;
  }
}

/**
 * Checks if wasm-pack is installed
 */
async function checkWasmPackInstalled() {
  try {
    const { stdout } = await execa('wasm-pack', ['--version']);
    console.log('✓ wasm-pack found:', stdout);
    return true;
  } catch (error) {
    console.warn('⚠️  wasm-pack not found. Skipping Rust build.');
    console.warn('   Install with: cargo install wasm-pack');
    return false;
  }
}

/**
 * Builds Rust components to WASM
 */
export async function buildRustComponents() {
  console.log('🦀 Building Rust/WASM components...');

  // Check prerequisites
  const hasRust = await checkRustInstalled();
  if (!hasRust) return;

  const hasWasmPack = await checkWasmPackInstalled();
  if (!hasWasmPack) return;

  try {
    // Build with wasm-pack
    console.log('   Compiling Rust to WASM (release mode)...');
    await execa(
      'wasm-pack',
      [
        'build',
        RUST_COMPONENTS_CRATE,
        '--target', 'web',
        '--out-dir', WASM_OUTPUT_DIR,
        '--release',
        '--',
        '--features', 'wasm',
      ],
      {
        stdio: 'inherit',
        preferLocal: true,
      }
    );

    console.log('✅ Rust components built successfully');
    console.log('   Output:', WASM_OUTPUT_DIR);

    // Copy to public distribution
    await copyToPublicDist();

  } catch (error) {
    console.error('❌ Rust build failed:', error.message);
    throw error;
  }
}

/**
 * Builds Rust components in development mode (faster, larger)
 */
export async function buildRustComponentsDev() {
  console.log('🦀 Building Rust/WASM components (dev mode)...');

  const hasRust = await checkRustInstalled();
  if (!hasRust) return;

  const hasWasmPack = await checkWasmPackInstalled();
  if (!hasWasmPack) return;

  try {
    console.log('   Compiling Rust to WASM (dev mode)...');
    await execa(
      'wasm-pack',
      [
        'build',
        RUST_COMPONENTS_CRATE,
        '--target', 'web',
        '--out-dir', WASM_OUTPUT_DIR,
        '--dev',
      ],
      {
        stdio: 'inherit',
        preferLocal: true,
      }
    );

    console.log('✅ Rust components built (dev mode)');
    await copyToPublicDist();

  } catch (error) {
    console.error('❌ Rust dev build failed:', error.message);
    throw error;
  }
}

/**
 * Copies WASM build output to public distribution directory
 */
async function copyToPublicDist() {
  try {
    // Ensure output directory exists
    await fs.mkdir(PUBLIC_OUTPUT_DIR, { recursive: true });

    // Copy WASM files
    const files = await fs.readdir(WASM_OUTPUT_DIR);

    for (const file of files) {
      // Skip unnecessary files
      if (file === '.gitignore' || file === 'package.json') {
        continue;
      }

      const src = path.join(WASM_OUTPUT_DIR, file);
      const dest = path.join(PUBLIC_OUTPUT_DIR, file);

      await fs.copyFile(src, dest);
    }

    console.log('   Copied WASM files to', PUBLIC_OUTPUT_DIR);

  } catch (error) {
    console.error('Failed to copy WASM files:', error.message);
    throw error;
  }
}

/**
 * Runs Rust tests
 */
export async function testRust() {
  console.log('🧪 Running Rust tests...');

  const hasRust = await checkRustInstalled();
  if (!hasRust) return;

  try {
    await execa(
      'cargo',
      ['test', '--workspace'],
      {
        cwd: RUST_WORKSPACE,
        stdio: 'inherit',
        preferLocal: true,
      }
    );

    console.log('✅ Rust tests passed');

  } catch (error) {
    console.error('❌ Rust tests failed');
    throw error;
  }
}

/**
 * Runs Rust tests in WASM environment
 */
export async function testRustWasm() {
  console.log('🧪 Running Rust WASM tests...');

  const hasRust = await checkRustInstalled();
  if (!hasRust) return;

  const hasWasmPack = await checkWasmPackInstalled();
  if (!hasWasmPack) return;

  try {
    await execa(
      'wasm-pack',
      ['test', '--headless', '--firefox', RUST_COMPONENTS_CRATE],
      {
        stdio: 'inherit',
        preferLocal: true,
      }
    );

    console.log('✅ Rust WASM tests passed');

  } catch (error) {
    console.error('❌ Rust WASM tests failed');
    throw error;
  }
}

/**
 * Checks Rust code without building
 */
export async function checkRust() {
  console.log('🔍 Checking Rust code...');

  const hasRust = await checkRustInstalled();
  if (!hasRust) return;

  try {
    await execa(
      'cargo',
      ['check', '--workspace'],
      {
        cwd: RUST_WORKSPACE,
        stdio: 'inherit',
        preferLocal: true,
      }
    );

    console.log('✅ Rust code check passed');

  } catch (error) {
    console.error('❌ Rust code check failed');
    throw error;
  }
}

/**
 * Cleans Rust build artifacts
 */
export async function cleanRust() {
  console.log('🧹 Cleaning Rust build artifacts...');

  try {
    // Remove target directory
    const targetDir = path.join(RUST_WORKSPACE, 'target');
    await fs.rm(targetDir, { recursive: true, force: true });

    // Remove WASM output
    await fs.rm(WASM_OUTPUT_DIR, { recursive: true, force: true });

    // Remove public dist
    await fs.rm(PUBLIC_OUTPUT_DIR, { recursive: true, force: true });

    console.log('✅ Rust artifacts cleaned');

  } catch (error) {
    // Ignore errors if directories don't exist
    if (error.code !== 'ENOENT') {
      console.error('Failed to clean Rust artifacts:', error.message);
    }
  }
}

/**
 * Watches Rust files and rebuilds on changes
 */
export function watchRust() {
  const hasRust = checkRustInstalled();
  if (!hasRust) {
    console.warn('⚠️  Rust watch disabled (toolchain not found)');
    return Promise.resolve();
  }

  console.log('👀 Watching Rust files for changes...');

  const watcher = gulpWatch(
    [
      'rust-components/crates/**/*.rs',
      'rust-components/crates/**/Cargo.toml',
    ],
    { cwd: rootDir },
    buildRustComponentsDev
  );

  watcher.on('change', (filepath) => {
    console.log('   Changed:', filepath);
  });

  return watcher;
}

// Export all tasks
export default {
  build: buildRustComponents,
  buildDev: buildRustComponentsDev,
  test: testRust,
  testWasm: testRustWasm,
  check: checkRust,
  clean: cleanRust,
  watch: watchRust,
};
