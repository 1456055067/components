# Quick Start Guide - Rust/Yew Components

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
cargo install wasm-pack

# Verify installation
rustc --version
wasm-pack --version
```

## Build Commands

### Production Build

```bash
# Full build (CSS extraction + WASM compilation)
npx gulp rust:build

# Or using npm scripts (if configured)
npm run rust:build
```

This will:
1. Extract CSS from React component builds
2. Create combined CSS bundle
3. Compile Rust to WASM
4. Output to `rust-components/dist/` and `lib/rust-components/`

### Development Build

```bash
# Faster build, larger bundle, no optimizations
npx gulp rust:build:dev
```

### Watch Mode

```bash
# Watch for changes and rebuild automatically
npx gulp rust:watch
```

Watches:
- `rust-components/crates/**/*.rs`
- `rust-components/crates/**/Cargo.toml`

### CSS Only

```bash
# Build just the CSS bundle
npx gulp rust:styles

# Watch CSS changes
npx gulp rust:styles:watch
```

### Testing

```bash
# Run Rust unit tests
npx gulp rust:test

# Or directly with cargo
cd rust-components
cargo test --workspace
```

### Code Quality

```bash
# Check code without building
npx gulp rust:check

# Format code
cd rust-components
cargo fmt

# Lint with clippy
cargo clippy
```

### Clean

```bash
# Remove build artifacts
npx gulp rust:clean
```

This removes:
- `rust-components/target/`
- `rust-components/dist/`
- `lib/rust-components/`

## Viewing the Demo

```bash
# 1. Build components
npx gulp rust:build

# 2. Start a local server from project root
python3 -m http.server 8000

# 3. Open browser
# Navigate to: http://localhost:8000/rust-components/examples/demo.html
```

## Project Structure

```
rust-components/
├── crates/
│   ├── components/          # Main component library
│   │   ├── src/
│   │   │   ├── lib.rs       # Library entry
│   │   │   ├── badge.rs     # Badge component
│   │   │   ├── spinner.rs   # Spinner component
│   │   │   ├── box_component.rs  # Box component
│   │   │   ├── button.rs    # Button component
│   │   │   └── internal/    # Utilities
│   │   └── Cargo.toml
│   └── design-tokens/       # Token types
│       └── src/lib.rs
├── examples/
│   └── demo.html            # Demo page
├── dist/
│   ├── wasm/                # WASM output
│   │   ├── cloudscape_components.js
│   │   ├── cloudscape_components_bg.wasm
│   │   └── package.json
│   └── styles/              # CSS bundles
│       ├── cloudscape-components.css
│       └── cloudscape-components.min.css
└── .cargo/
    └── config.toml          # WASM optimizations
```

## Available Components

### Badge

```rust
use cloudscape_components::{Badge, BadgeColor};

html! {
    <Badge color={BadgeColor::Blue}>
        { "Active" }
    </Badge>
}
```

### Spinner

```rust
use cloudscape_components::{Spinner, SpinnerSize};

html! {
    <Spinner size={SpinnerSize::Normal} />
}
```

### Box

```rust
use cloudscape_components::{Box, BoxVariant, SpacingSize};

html! {
    <Box variant={BoxVariant::Div} padding={SpacingSize::M}>
        { "Content" }
    </Box>
}
```

### Button

```rust
use cloudscape_components::{Button, ButtonVariant};

let onclick = Callback::from(|_| {
    web_sys::console::log_1(&"Clicked!".into());
});

html! {
    <Button variant={ButtonVariant::Primary} on_click={onclick}>
        { "Click me" }
    </Button>
}
```

## Common Issues

### Rust toolchain not found

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart shell or run
source $HOME/.cargo/env
```

### wasm-pack not found

```bash
cargo install wasm-pack
```

### React CSS not found

The CSS extraction requires React components to be built first:

```bash
# Build React components
npx gulp quick-build

# Then build Rust components
npx gulp rust:build
```

If React isn't built, placeholder CSS will be generated automatically.

### Node modules not installed

```bash
npm install
```

## Build Output

After building, you'll find:

**WASM files**:
- `rust-components/dist/wasm/cloudscape_components.js` - JS bindings
- `rust-components/dist/wasm/cloudscape_components_bg.wasm` - WASM binary
- `lib/rust-components/cloudscape_components.js` - Copy for distribution

**CSS files**:
- `rust-components/dist/styles/cloudscape-components.css` - Full CSS
- `rust-components/dist/styles/cloudscape-components.min.css` - Minified
- `lib/rust-components/cloudscape-components.css` - Distribution copy

## Integration with Main Build

```bash
# Build both React and Rust components
npx gulp build:with-rust
```

This runs the standard build process plus Rust compilation.

## Next Steps

1. **Add components**: Follow patterns in existing components
2. **Run tests**: Ensure all tests pass before committing
3. **Check formatting**: Use `cargo fmt` before committing
4. **Review lints**: Run `cargo clippy` for suggestions

## Getting Help

- [Rust Book](https://doc.rust-lang.org/book/)
- [Yew Documentation](https://yew.rs/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Cloudscape Design System](https://cloudscape.design/)

See [README.md](README.md) for detailed documentation.
