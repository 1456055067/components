// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * CommonJS wrapper for Rust build tasks
 *
 * This file provides a CommonJS-compatible interface to the ESM Rust build module.
 */

// Placeholder tasks that do nothing if Rust toolchain is not available
const noopTask = (done) => {
  console.warn('⚠️  Rust build skipped (optional dependency)');
  done();
};

// Export placeholder tasks
module.exports = {
  buildRust: noopTask,
  buildRustDev: noopTask,
  testRust: noopTask,
  cleanRust: noopTask,
  checkRust: noopTask,
  watchRust: () => Promise.resolve(),
};

// Try to load actual Rust build tasks
(async () => {
  try {
    const rustBuildModule = await import('./rust-build.js');

    // Replace placeholders with actual tasks
    module.exports.buildRust = rustBuildModule.buildRustComponents;
    module.exports.buildRustDev = rustBuildModule.buildRustComponentsDev;
    module.exports.testRust = rustBuildModule.testRust;
    module.exports.cleanRust = rustBuildModule.cleanRust;
    module.exports.checkRust = rustBuildModule.checkRust;
    module.exports.watchRust = rustBuildModule.watchRust;

  } catch (error) {
    // Rust build tasks are optional - silently fail
  }
})();
