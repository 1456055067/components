// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

const { series, parallel, watch } = require('gulp');
const {
  clean,
  docs,
  generateEnvironment,
  generateIcons,
  generateIndexFile,
  generateCustomCssPropertiesMap,
  packageJSON,
  unit,
  styles,
  typescript,
  buildPages,
  testUtils,
  a11y,
  generateI18nMessages,
  integ,
  motion,
  copyFiles,
  themeableSource,
  bundleVendorFiles,
  sizeLimit,
} = require('./build-tools/tasks');

// Import Rust build tasks (optional)
const {
  buildRust,
  buildRustDev,
  testRust,
  cleanRust,
  checkRust,
  watchRust,
} = require('./build-tools/tasks/rust-build-wrapper.cjs');

// Import Rust styles tasks (optional)
const {
  buildRustStyles,
  watchRustStyles,
  cleanRustStyles,
} = require('./build-tools/tasks/rust-styles-wrapper.cjs');

const quickBuild = series(
  clean,
  parallel(packageJSON, generateI18nMessages, generateEnvironment, generateIcons, generateIndexFile, copyFiles),
  parallel(generateCustomCssPropertiesMap, styles, typescript, testUtils),
  bundleVendorFiles
);

exports.clean = clean;
exports['quick-build'] = quickBuild;
exports.i18n = generateI18nMessages;
exports.build = series(quickBuild, parallel(buildPages, themeableSource, docs, sizeLimit));
exports['build:with-rust'] = series(quickBuild, parallel(buildPages, themeableSource, docs, sizeLimit, series(buildRustStyles, buildRust)));
exports.test = series(unit, integ, a11y);
exports['test:unit'] = unit;
exports['test:integ'] = integ;
exports['test:a11y'] = a11y;
exports['test:motion'] = motion;

// Rust-specific tasks
exports['rust:build'] = series(buildRustStyles, buildRust);
exports['rust:build:dev'] = series(buildRustStyles, buildRustDev);
exports['rust:test'] = testRust;
exports['rust:check'] = checkRust;
exports['rust:clean'] = series(cleanRustStyles, cleanRust);
exports['rust:watch'] = parallel(watchRustStyles, watchRust);
exports['rust:styles'] = buildRustStyles;
exports['rust:styles:watch'] = watchRustStyles;

exports.watch = () => {
  watch(
    [
      'src/**/*.{ts,tsx}',
      '!src/test-utils/**/*.ts',
      '!**/__tests__/**',
      '!**/__integ__/**',
      '!**/__a11y__/**',
      '!**/__motion__/**',
      '!src/internal/vendor/**/*.ts',
    ],
    typescript
  );
  watch(['src/i18n/messages/*.json'], generateI18nMessages);
  watch(['src/test-utils/dom/**/*.ts', '!src/test-utils/dom/index.ts'], testUtils);
  watch(['style-dictionary/**/*.ts', 'src/**/*.scss'], styles);
};
