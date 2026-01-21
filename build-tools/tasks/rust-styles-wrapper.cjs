// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * CommonJS wrapper for Rust styles tasks
 */

const loadRustStylesModule = async () => {
  try {
    return await import('./rust-styles.js');
  } catch (error) {
    return null;
  }
};

const runOrSkip = async (taskName, done) => {
  const rustStylesModule = await loadRustStylesModule();
  if (!rustStylesModule || typeof rustStylesModule[taskName] !== 'function') {
    console.warn('⚠️  Rust styles task skipped');
    if (typeof done === 'function') {
      done();
    }
    return;
  }

  try {
    await rustStylesModule[taskName]();
    if (typeof done === 'function') {
      done();
    }
  } catch (error) {
    if (typeof done === 'function') {
      done(error);
    } else {
      throw error;
    }
  }
};

module.exports = {
  buildRustStyles: (done) => runOrSkip('buildRustStyles', done),
  watchRustStyles: (done) => runOrSkip('watchRustStyles', done),
  cleanRustStyles: (done) => runOrSkip('cleanRustStyles', done),
};
