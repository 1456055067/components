# Rust/Yew Components - Development Guide

This guide covers the development workflow for working on the Cloudscape Rust/WASM components.

## Quick Start

### Prerequisites

1. **Rust toolchain** (1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Trunk** (WASM build tool)
   ```bash
   cargo install trunk
   ```

3. **wasm32 target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

4. **Node.js** (for design token generation)
   Already installed for React build

## Development Workflows

### 1. Token Watch Mode (Recommended)

The fastest way to develop with automatic token regeneration:

```bash
# Terminal 1: Watch for token changes
npm run rust:watch

# Terminal 2: Serve the demo with hot reload
npm run rust:serve
```

This setup:
- Watches `lib/style-dictionary/` and `style-dictionary/` for changes
- Automatically regenerates Rust tokens when files change
- Debounces rapid changes (500ms)
- Shows build progress and timing

**What triggers regeneration:**
- Changes to `style-dictionary/**/*.ts` (token definitions)
- Changes to `style-dictionary/**/*.json` (metadata)
- Changes to `lib/style-dictionary/**/*.js` (compiled tokens)

### 2. Manual Token Generation

Generate tokens once without watching:

```bash
npm run rust:tokens
```

This runs the token generator and outputs:
- `rust-components/dist/styles/design-tokens.css` (CSS custom properties)
- `rust-components/crates/design-tokens/src/generated/tokens.rs` (Rust enums)

### 3. Complete Build

Build everything from scratch:

```bash
# Build CSS (includes token generation)
npm run rust:styles

# Build the demo application
cd rust-components/demo
trunk build --release
```

Or use the combined command:

```bash
npm run rust:build
```

## Project Structure

```
rust-components/
├── Cargo.toml                    # Workspace configuration
├── crates/
│   ├── components/               # Main component library
│   │   ├── src/
│   │   │   ├── lib.rs           # Re-exports all components
│   │   │   ├── badge.rs         # Individual components
│   │   │   ├── button.rs
│   │   │   └── ...
│   │   └── Cargo.toml
│   └── design-tokens/            # Design token types
│       ├── src/
│       │   ├── lib.rs           # Token management
│       │   └── generated/       # Auto-generated from style-dictionary
│       │       ├── mod.rs
│       │       └── tokens.rs    # Generated token enums
│       └── Cargo.toml
├── demo/                         # Demo application
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── app.rs               # Main app with routing
│   │   ├── components/          # Demo-specific components
│   │   └── pages/               # Demo pages
│   ├── index.html               # HTML template
│   ├── Trunk.toml               # Trunk configuration
│   └── Cargo.toml
├── dist/                         # Build outputs
│   └── styles/
│       ├── design-tokens.css    # Generated CSS tokens
│       └── cloudscape-components.css  # Complete CSS bundle
└── scripts/
    └── optimize-wasm.sh         # Optional WASM optimization
```

## Adding a New Component

### 1. Create Component File

```bash
touch rust-components/crates/components/src/my_component.rs
```

### 2. Implement Component

```rust
// my_component.rs
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum MyComponentVariant {
    #[default]
    Default,
    Primary,
}

#[derive(Properties, PartialEq)]
pub struct MyComponentProps {
    #[prop_or_default]
    pub variant: MyComponentVariant,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MyComponent)]
pub fn my_component(props: &MyComponentProps) -> Html {
    let class = format!("awsui-my-component awsui-my-component-{}",
                        props.variant.as_str());

    html! {
        <div class={class}>
            { props.children.clone() }
        </div>
    }
}
```

### 3. Export from Library

Add to `src/lib.rs`:

```rust
mod my_component;
pub use my_component::*;
```

### 4. Add CSS to Build

Add component name to `build-tools/tasks/rust-styles.js`:

```javascript
const COMPONENTS = [
  // ...existing components
  'my-component',
];
```

### 5. Use in Demo

Add example to demo pages:

```rust
use cloudscape_components::MyComponent;

html! {
    <MyComponent variant={MyComponentVariant::Primary}>
        {"Hello World"}
    </MyComponent>
}
```

## Testing

### Unit Tests

Run component tests:

```bash
cd rust-components
cargo test --lib
```

Run with design tokens:

```bash
cargo test --lib --features generated
```

### Integration Tests

Run WASM tests in browser:

```bash
cd rust-components/crates/components
wasm-pack test --headless --chrome
```

### Adding Tests

Add tests to your component file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_component_variant() {
        assert_eq!(MyComponentVariant::default(), MyComponentVariant::Default);
    }
}
```

## Design Token Workflow

### Understanding Token Generation

The token generation pipeline:

```
style-dictionary/*.ts
         ↓
    (TypeScript compile)
         ↓
lib/style-dictionary/*.js
         ↓
  (generate-rust-tokens.js)
         ↓
    ┌─────────────────┬──────────────────┐
    ↓                 ↓                  ↓
tokens.rs    design-tokens.css    Component CSS
```

### Token Categories

Generated tokens are organized into enums:

- **ColorToken**: 380+ color tokens (backgrounds, text, borders)
- **FontToken**: 110+ typography tokens (families, sizes, weights)
- **OtherToken**: 231+ misc tokens (including spacing)
- **BorderToken**: 86+ border tokens (radii, widths)
- **ShadowToken**: 16+ shadow tokens
- **MotionToken**: 44+ animation tokens (durations, easings)
- **SizeToken**: 9+ size tokens

### Using Tokens in Components

```rust
use cloudscape_design_tokens::generated::*;

// In your component
let bg_color = ColorToken::ColorBackgroundButtonPrimaryDefault.css_var();
// Returns: "var(--awsui-color-background-button-primary-default)"

html! {
    <div style={format!("background: {}", bg_color)}>
        {"Styled with design token"}
    </div>
}
```

## Debugging

### Enable Logging

```rust
use gloo::console;

console::log!("Debug:", &some_value);
console::error!("Error:", &error);
```

### Browser DevTools

1. Open demo in browser
2. Open DevTools (F12)
3. Check Console for logs
4. Use Sources tab to debug WASM

### Rust Backtrace

Add to demo `Cargo.toml`:

```toml
[dependencies]
console_error_panic_hook = "0.1.7"
```

Initialize in `main.rs`:

```rust
console_error_panic_hook::set_once();
```

## Performance Profiling

### Bundle Size Analysis

Check current bundle size:

```bash
ls -lh demo/dist/*.wasm
gzip -c demo/dist/*.wasm | wc -c | awk '{print $1/1024/1024 " MB"}'
```

### Optimize Further

Run wasm-opt for additional size reduction:

```bash
./scripts/optimize-wasm.sh
```

Expected savings: 5-10%

### Profile Runtime Performance

Use browser DevTools Performance tab:

1. Build with release mode
2. Open Performance tab
3. Record interaction
4. Analyze flame graph

## Common Tasks

### Update All Dependencies

```bash
cd rust-components
cargo update
```

### Check for Outdated Crates

```bash
cargo install cargo-outdated
cargo outdated
```

### Fix Clippy Warnings

```bash
cargo clippy --fix
```

### Format Code

```bash
cargo fmt
```

### Clean Build Artifacts

```bash
cd demo
trunk clean
cargo clean
```

## Troubleshooting

### Token Generation Fails

**Problem**: `generate-rust-tokens.js` errors

**Solution**:
1. Ensure React build completed: `npm run quick-build`
2. Check `lib/style-dictionary/` exists
3. Verify Node.js version: `node --version` (14+)

### WASM Build Fails

**Problem**: `trunk build` errors

**Solution**:
1. Update Rust: `rustup update`
2. Reinstall trunk: `cargo install trunk --force`
3. Clear cache: `rm -rf target/`

### CSS Not Loading

**Problem**: Components unstyled in demo

**Solution**:
1. Rebuild CSS: `npm run rust:styles`
2. Check `dist/styles/cloudscape-components.css` exists
3. Hard refresh browser (Cmd+Shift+R / Ctrl+Shift+R)

### Watch Mode Not Working

**Problem**: Changes not triggering rebuild

**Solution**:
1. Check file paths in `watch-rust-tokens.js`
2. Restart watch mode
3. Check for file permission issues

### Type Errors After Token Update

**Problem**: Rust compilation fails after regenerating tokens

**Solution**:
1. Run `cargo clean` in design-tokens crate
2. Rebuild: `cargo build`
3. Check generated `tokens.rs` is valid

## Git Workflow

### Committing Changes

**Don't commit**:
- `target/` (build artifacts)
- `demo/dist/` (build outputs)
- `node_modules/`

**Do commit**:
- Source code changes
- Generated `tokens.rs` (track changes to tokens)
- CSS files in `dist/styles/`
- Configuration files

### Branch Strategy

Follow main repo conventions:
- `main` - Production-ready code
- `feature/*` - New features
- `fix/*` - Bug fixes

## CI/CD Integration

### Running Tests in CI

```yaml
# .github/workflows/rust-tests.yml
- name: Test Rust components
  run: |
    cd rust-components
    cargo test --all --features generated
```

### Building for Release

```yaml
- name: Build Rust demo
  run: |
    npm run rust:build

- name: Upload artifacts
  uses: actions/upload-artifact@v2
  with:
    name: rust-demo
    path: rust-components/demo/dist/
```

## Resources

- **Yew Book**: https://yew.rs/docs/getting-started/introduction
- **Rust WASM Book**: https://rustwasm.github.io/docs/book/
- **Trunk Guide**: https://trunkrs.dev/
- **Cloudscape Design**: https://cloudscape.design/

## Getting Help

- Check existing GitHub issues
- Review this development guide
- Ask in team discussions
- File bugs with reproducible examples

---

Happy coding! 🦀
