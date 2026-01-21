#!/usr/bin/env node
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

import { execSync } from 'child_process';
import { existsSync, readdirSync, writeFileSync } from 'fs';
import { basename, join } from 'path';

const UPSTREAM_REMOTE = 'upstream';
const UPSTREAM_BRANCH = 'main';
const REACT_SRC = 'src';
const RUST_SRC = 'rust-components/crates/components/src';

// Map Rust component file names to React directory names
const RUST_TO_REACT_MAP = {
  box_component: 'box',
  app_layout: 'app-layout',
  content_layout: 'content-layout',
  radio_group: 'radio-group',
  status_indicator: 'status-indicator',
  progress_bar: 'progress-bar',
  form_field: 'form-field',
  key_value_pairs: 'key-value-pairs',
  expandable_section: 'expandable-section',
  column_layout: 'column-layout',
  space_between: 'space-between',
  side_navigation: 'side-navigation',
  top_navigation: 'top-navigation',
  date_picker: 'date-picker',
  date_range_picker: 'date-range-picker',
  file_upload: 'file-upload',
  token_group: 'token-group',
  button_dropdown: 'button-dropdown',
  copy_to_clipboard: 'copy-to-clipboard',
};

function runGit(cmd) {
  try {
    return execSync(`git ${cmd}`, { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'pipe'] }).trim();
  } catch {
    return '';
  }
}

function getImplementedRustComponents() {
  const files = readdirSync(RUST_SRC).filter(
    f => f.endsWith('.rs') && !['lib.rs', 'mod.rs'].includes(f) && !f.startsWith('internal')
  );
  return files.map(f => f.replace('.rs', ''));
}

function rustToReact(rustName) {
  return RUST_TO_REACT_MAP[rustName] || rustName;
}

function getUpstreamDiff(reactComponent) {
  const reactDir = join(REACT_SRC, reactComponent);
  if (!existsSync(reactDir)) {
    return null;
  }

  const diff = runGit(`diff HEAD...${UPSTREAM_REMOTE}/${UPSTREAM_BRANCH} -- "${reactDir}"`);
  return diff || null;
}

function getChangedFiles(reactComponent) {
  const reactDir = join(REACT_SRC, reactComponent);
  const output = runGit(`diff --name-status HEAD...${UPSTREAM_REMOTE}/${UPSTREAM_BRANCH} -- "${reactDir}"`);
  if (!output) {
    return [];
  }

  return output
    .split('\n')
    .filter(Boolean)
    .map(line => {
      const [status, ...pathParts] = line.split('\t');
      return { status, path: pathParts.join('\t') };
    });
}

// Parse TypeScript interface from diff to extract prop changes
function extractPropChanges(diff) {
  const changes = {
    added: [],
    removed: [],
    modified: [],
  };

  const lines = diff.split('\n');
  let inInterface = false;
  let interfaceName = '';

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Detect interface start
    const interfaceMatch = line.match(/^[+-]?\s*(?:export\s+)?interface\s+(\w+Props)/);
    if (interfaceMatch) {
      inInterface = true;
      interfaceName = interfaceMatch[1];
      continue;
    }

    // Detect interface end
    if (inInterface && line.match(/^[+-]?\s*\}/)) {
      inInterface = false;
      continue;
    }

    // Extract prop changes within interface
    if (inInterface) {
      // Added prop
      const addedMatch = line.match(/^\+\s+(\w+)(\?)?:\s*(.+);/);
      if (addedMatch) {
        changes.added.push({
          name: addedMatch[1],
          optional: !!addedMatch[2],
          type: addedMatch[3].trim(),
          interface: interfaceName,
        });
      }

      // Removed prop
      const removedMatch = line.match(/^-\s+(\w+)(\?)?:\s*(.+);/);
      if (removedMatch) {
        changes.removed.push({
          name: removedMatch[1],
          optional: !!removedMatch[2],
          type: removedMatch[3].trim(),
          interface: interfaceName,
        });
      }
    }
  }

  return changes;
}

// Extract event handler changes
function extractEventChanges(diff) {
  const events = {
    added: [],
    removed: [],
    modified: [],
  };

  const eventPattern = /^([+-])\s*(on\w+)(\?)?:\s*(.+)/gm;
  let match;

  while ((match = eventPattern.exec(diff)) !== null) {
    const [, sign, name, optional, type] = match;
    const entry = { name, optional: !!optional, type: type.trim() };

    if (sign === '+') {
      events.added.push(entry);
    } else {
      events.removed.push(entry);
    }
  }

  return events;
}

// Extract CSS class changes
function extractStyleChanges(diff) {
  const styles = {
    newClasses: [],
    removedClasses: [],
    modifiedSelectors: [],
  };

  // Look for clsx or className changes
  const classPattern = /^([+-]).*(?:className|clsx|styles\.).*['"]([^'"]+)['"]/gm;
  let match;

  while ((match = classPattern.exec(diff)) !== null) {
    const [, sign, className] = match;
    if (sign === '+') {
      styles.newClasses.push(className);
    } else {
      styles.removedClasses.push(className);
    }
  }

  return styles;
}

// Extract ARIA/accessibility changes
function extractA11yChanges(diff) {
  const a11y = {
    added: [],
    removed: [],
  };

  const ariaPattern = /^([+-]).*(?:aria-\w+|role)=["']([^"']+)["']/gm;
  let match;

  while ((match = ariaPattern.exec(diff)) !== null) {
    const [fullMatch, sign] = match;
    const attrMatch = fullMatch.match(/(aria-\w+|role)=["']([^"']+)["']/);
    if (attrMatch) {
      const entry = { attr: attrMatch[1], value: attrMatch[2] };
      if (sign === '+') {
        a11y.added.push(entry);
      } else {
        a11y.removed.push(entry);
      }
    }
  }

  return a11y;
}

// Generate Rust code suggestions for prop changes
function generateRustPropSuggestion(prop) {
  const typeMap = {
    string: 'String',
    number: 'i32',
    boolean: 'bool',
    'React.ReactNode': 'Children',
    ReactNode: 'Children',
  };

  let rustType = typeMap[prop.type] || `/* TODO: map ${prop.type} */`;

  // Handle common patterns
  if (prop.type.includes('|')) {
    rustType = `/* enum: ${prop.type} */`;
  }
  if (prop.type.includes('=> void') || prop.type.includes('Handler')) {
    rustType = `Callback</* event type */>`;
  }

  const optionalWrapper = prop.optional ? 'Option<' : '';
  const optionalClose = prop.optional ? '>' : '';

  return `    #[prop_or_default]\n    pub ${toSnakeCase(prop.name)}: ${optionalWrapper}${rustType}${optionalClose},`;
}

function toSnakeCase(str) {
  return str
    .replace(/([A-Z])/g, '_$1')
    .toLowerCase()
    .replace(/^_/, '');
}

// Analyze a single component
function analyzeComponent(rustComponent) {
  const reactComponent = rustToReact(rustComponent);
  const diff = getUpstreamDiff(reactComponent);

  if (!diff) {
    return {
      component: rustComponent,
      reactComponent,
      hasChanges: false,
      message: 'No upstream changes or component not found',
    };
  }

  const changedFiles = getChangedFiles(reactComponent);
  const propChanges = extractPropChanges(diff);
  const eventChanges = extractEventChanges(diff);
  const styleChanges = extractStyleChanges(diff);
  const a11yChanges = extractA11yChanges(diff);

  const analysis = {
    component: rustComponent,
    reactComponent,
    hasChanges: true,
    changedFiles,
    propChanges,
    eventChanges,
    styleChanges,
    a11yChanges,
    recommendations: [],
    rustSuggestions: [],
  };

  // Generate recommendations
  if (propChanges.added.length > 0) {
    analysis.recommendations.push({
      priority: 'HIGH',
      type: 'props',
      message: `Add ${propChanges.added.length} new prop(s) to ${rustComponent}Props struct`,
      details: propChanges.added.map(p => `- ${p.name}: ${p.type}`),
    });

    analysis.rustSuggestions.push({
      type: 'props',
      code: propChanges.added.map(generateRustPropSuggestion).join('\n'),
    });
  }

  if (propChanges.removed.length > 0) {
    analysis.recommendations.push({
      priority: 'MEDIUM',
      type: 'props',
      message: `Consider removing ${propChanges.removed.length} deprecated prop(s)`,
      details: propChanges.removed.map(p => `- ${p.name}`),
    });
  }

  if (eventChanges.added.length > 0) {
    analysis.recommendations.push({
      priority: 'HIGH',
      type: 'events',
      message: `Add ${eventChanges.added.length} new event handler(s)`,
      details: eventChanges.added.map(e => `- ${e.name}: ${e.type}`),
    });
  }

  if (styleChanges.newClasses.length > 0) {
    analysis.recommendations.push({
      priority: 'MEDIUM',
      type: 'styles',
      message: `Update ClassBuilder with ${styleChanges.newClasses.length} new CSS class(es)`,
      details: styleChanges.newClasses.map(c => `- "${c}"`),
    });
  }

  if (a11yChanges.added.length > 0) {
    analysis.recommendations.push({
      priority: 'HIGH',
      type: 'a11y',
      message: `Add ${a11yChanges.added.length} new ARIA attribute(s)`,
      details: a11yChanges.added.map(a => `- ${a.attr}="${a.value}"`),
    });
  }

  return analysis;
}

// Format analysis as markdown
function formatAsMarkdown(analyses) {
  let md = `# React to Rust/Yew Change Analysis

**Generated:** ${new Date().toISOString()}
**Upstream:** ${UPSTREAM_REMOTE}/${UPSTREAM_BRANCH}

## Summary

| Component | Status | Priority Changes |
|-----------|--------|------------------|
`;

  for (const a of analyses) {
    if (a.hasChanges) {
      const highPriority = a.recommendations.filter(r => r.priority === 'HIGH').length;
      md += `| ${a.component} | ⚠️ Changes | ${highPriority} high priority |\n`;
    } else {
      md += `| ${a.component} | ✅ Up to date | - |\n`;
    }
  }

  md += '\n---\n\n## Detailed Analysis\n\n';

  for (const a of analyses.filter(x => x.hasChanges)) {
    md += `### ${a.component}\n\n`;
    md += `**React component:** \`src/${a.reactComponent}/\`\n`;
    md += `**Rust component:** \`rust-components/crates/components/src/${a.component}.rs\`\n\n`;

    if (a.changedFiles.length > 0) {
      md += `**Changed files:**\n`;
      for (const f of a.changedFiles) {
        const statusEmoji = f.status === 'M' ? '📝' : f.status === 'A' ? '➕' : f.status === 'D' ? '➖' : '❓';
        md += `- ${statusEmoji} \`${basename(f.path)}\`\n`;
      }
      md += '\n';
    }

    if (a.recommendations.length > 0) {
      md += `**Recommendations:**\n\n`;
      for (const r of a.recommendations) {
        const emoji = r.priority === 'HIGH' ? '🔴' : r.priority === 'MEDIUM' ? '🟡' : '🟢';
        md += `${emoji} **${r.priority}** - ${r.message}\n`;
        if (r.details && r.details.length > 0) {
          for (const d of r.details) {
            md += `  ${d}\n`;
          }
        }
        md += '\n';
      }
    }

    if (a.rustSuggestions.length > 0) {
      md += `**Suggested Rust code:**\n\n`;
      for (const s of a.rustSuggestions) {
        md += `\`\`\`rust\n// ${s.type}\n${s.code}\n\`\`\`\n\n`;
      }
    }

    md += '---\n\n';
  }

  return md;
}

// Main execution
function main() {
  const args = process.argv.slice(2);

  console.log('🔍 Fetching upstream changes...');
  runGit(`fetch ${UPSTREAM_REMOTE} ${UPSTREAM_BRANCH}`);

  let components;
  if (args.includes('--all') || args.length === 0) {
    components = getImplementedRustComponents();
    console.log(`📦 Analyzing ${components.length} Rust components...\n`);
  } else {
    components = args.filter(a => !a.startsWith('-'));
    console.log(`📦 Analyzing specified components: ${components.join(', ')}\n`);
  }

  const analyses = [];

  for (const comp of components) {
    process.stdout.write(`  Analyzing ${comp}... `);
    const analysis = analyzeComponent(comp);
    analyses.push(analysis);

    if (analysis.hasChanges) {
      const highCount = analysis.recommendations.filter(r => r.priority === 'HIGH').length;
      console.log(`⚠️  ${highCount} high priority changes`);
    } else {
      console.log('✅ up to date');
    }
  }

  const report = formatAsMarkdown(analyses);
  const reportFile = `react-yew-analysis-${Date.now()}.md`;
  writeFileSync(reportFile, report);

  console.log(`\n📄 Report saved to: ${reportFile}`);

  // Print summary
  const withChanges = analyses.filter(a => a.hasChanges);
  const totalHigh = withChanges.reduce(
    (sum, a) => sum + a.recommendations.filter(r => r.priority === 'HIGH').length,
    0
  );

  console.log(`\n📊 Summary:`);
  console.log(`   Components with changes: ${withChanges.length}/${analyses.length}`);
  console.log(`   High priority items: ${totalHigh}`);
}

main().catch(console.error);
