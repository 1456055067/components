# Upstream Sync Workflow

This document describes the process for syncing changes from the upstream `cloudscape-design/components` repository and evaluating which React changes require corresponding Rust/Yew updates.

## Overview

The fork maintains Rust/Yew implementations of Cloudscape React components. When the upstream repository updates React components, we need to:

1. Fetch upstream changes
2. Analyze which implemented Rust components are affected
3. Review the specific changes (props, events, styles, accessibility)
4. Update the Rust/Yew implementations accordingly

## Quick Start

```bash
# Analyze which Rust components need updates (recommended first step)
./scripts/sync-upstream.sh analyze

# Preview what would be merged
./scripts/sync-upstream.sh preview

# Merge upstream and analyze
./scripts/sync-upstream.sh merge
```

## Available Scripts

### 1. sync-upstream.sh (Bash)

Main orchestration script for syncing with upstream.

**Commands:**

| Command | Description |
|---------|-------------|
| `fetch` | Fetch upstream changes only |
| `analyze` | Fetch and analyze which Rust components need updates (default) |
| `preview` | Show commits that would be merged |
| `merge` | Merge upstream changes with confirmation |
| `full` | Merge then analyze |
| `help` | Show usage information |

**Example Output:**

```
=== Cloudscape Upstream Sync & Analysis ===

Upstream remote exists: https://github.com/cloudscape-design/components.git

Fetching upstream changes...
Fetch complete

Analyzing changes for implemented Rust/Yew components...

✓ badge is up to date
⚠ button has upstream changes
✓ spinner is up to date
⚠ input has upstream changes
...

Analysis complete!
Report saved to: sync-report-20260121-143022.md
```

### 2. analyze-react-changes.js (Node.js)

Deep analysis of React component changes with Rust code suggestions.

**Usage:**

```bash
# Analyze all implemented components
node scripts/analyze-react-changes.js --all

# Analyze specific component(s)
node scripts/analyze-react-changes.js button input select
```

**Features:**

- Parses TypeScript interface changes to identify new/removed props
- Detects event handler changes
- Identifies CSS class changes
- Tracks ARIA/accessibility updates
- Generates suggested Rust code for new props
- Produces detailed markdown report

**Example Report:**

```markdown
## button

**React component:** `src/button/`
**Rust component:** `rust-components/crates/components/src/button.rs`

**Recommendations:**

🔴 **HIGH** - Add 2 new prop(s) to buttonProps struct
  - iconAlign: 'left' | 'right'
  - fullWidth: boolean

**Suggested Rust code:**

​```rust
// props
    #[prop_or_default]
    pub icon_align: Option</* enum: 'left' | 'right' */>,
    #[prop_or_default]
    pub full_width: Option<bool>,
​```
```

## Workflow Steps

### Step 1: Check for Upstream Changes

```bash
./scripts/sync-upstream.sh preview
```

This shows how many commits are ahead in upstream and lists recent changes.

### Step 2: Analyze Impact

```bash
./scripts/sync-upstream.sh analyze
```

This generates a report (`sync-report-YYYYMMDD-HHMMSS.md`) showing:
- Which Rust components have corresponding React changes
- Categories of changes (props, events, styles, a11y)
- Recommended actions

### Step 3: Deep Analysis (Optional)

For detailed code-level analysis:

```bash
node scripts/analyze-react-changes.js button
```

This provides:
- Specific prop additions/removals
- Suggested Rust code snippets
- Priority ratings for changes

### Step 4: Merge Upstream

```bash
./scripts/sync-upstream.sh merge
```

This will:
1. Create a backup branch
2. Attempt to merge upstream/main
3. If conflicts occur, pause for manual resolution
4. Run analysis after successful merge

### Step 5: Update Rust Components

Use the generated reports to update affected Rust components:

1. Open the sync report
2. For each component with changes:
   - Review the React changes in `src/<component>/`
   - Update the Rust props struct in `rust-components/crates/components/src/<component>.rs`
   - Update event handlers if needed
   - Verify CSS class names match
   - Update ARIA attributes

### Step 6: Test

```bash
# Run Rust tests
npx gulp rust:test

# Build to verify
npx gulp rust:build
```

## Understanding the Reports

### Priority Levels

| Priority | Meaning |
|----------|---------|
| 🔴 HIGH | Breaking changes - new required props, API changes |
| 🟡 MEDIUM | Non-breaking but important - new optional props, deprecations |
| 🟢 LOW | Minor changes - documentation, internal refactoring |

### Change Categories

| Category | Description | Rust Impact |
|----------|-------------|-------------|
| `props` | Interface/type changes | Update Props struct |
| `events` | Event handler changes | Update Callback types |
| `styles` | CSS class changes | Update ClassBuilder usage |
| `a11y` | ARIA/accessibility | Update ARIA attributes |
| `exports` | Module export changes | Update lib.rs exports |
| `i18n` | Internationalization | Update I18nStrings |

## Component Mapping

The scripts automatically map between Rust and React naming conventions:

| Rust File | React Directory |
|-----------|-----------------|
| `app_layout.rs` | `src/app-layout/` |
| `box_component.rs` | `src/box/` |
| `radio_group.rs` | `src/radio-group/` |
| `form_field.rs` | `src/form-field/` |
| `status_indicator.rs` | `src/status-indicator/` |

## Troubleshooting

### "Upstream remote not found"

The script will automatically add it:
```bash
git remote add upstream https://github.com/cloudscape-design/components.git
```

### Merge Conflicts

If conflicts occur during merge:
```bash
# Resolve conflicts in your editor
git add .
git commit

# Or abort and try again later
git merge --abort
```

### Component Not Analyzed

If a Rust component isn't being analyzed:
1. Check the component exists in `rust-components/crates/components/src/`
2. Verify the React equivalent exists in `src/`
3. Add mapping to `RUST_TO_REACT_MAP` in `analyze-react-changes.js` if needed

## Best Practices

1. **Sync regularly** - Don't let too many upstream commits accumulate
2. **Review before merging** - Always run `preview` first
3. **Test after updates** - Run `rust:test` after updating components
4. **Keep reports** - Reports are timestamped; keep them for reference
5. **Update one component at a time** - Easier to test and debug
