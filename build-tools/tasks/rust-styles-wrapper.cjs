// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * CommonJS wrapper for Rust styles tasks
 */

const noopTask = (done) => {
  console.warn('⚠️  Rust styles task skipped');
  done();
};

module.exports = {
  buildRustStyles: noopTask,
  watchRustStyles: () => Promise.resolve(),
  cleanRustStyles: noopTask,
};

// Dynamically load ESM module
(async () => {
  try {
    const rustStylesModule = await import('./rust-styles.js');

    module.exports.buildRustStyles = rustStylesModule.buildRustStyles;
    module.exports.watchRustStyles = rustStylesModule.watchRustStyles;
    module.exports.cleanRustStyles = rustStylesModule.cleanRustStyles;
  } catch (error) {
    // Silent fail - styles are optional
  }
})();
