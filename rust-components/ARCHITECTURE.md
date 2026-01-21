# Architecture Overview - Rust/Yew Components

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Cloudscape Components                     │
│                    =====================                     │
│                                                              │
│  ┌────────────────────┐         ┌────────────────────┐     │
│  │  React Components  │         │  Rust/Yew          │     │
│  │  (Existing)        │         │  Components        │     │
│  │                    │         │  (New)             │     │
│  │  src/              │         │  rust-components/  │     │
│  │  ├── badge/        │         │  └── crates/       │     │
│  │  ├── button/       │         │     ├── components/│     │
│  │  ├── spinner/      │         │     └── tokens/    │     │
│  │  └── ...           │         │                    │     │
│  └──────┬─────────────┘         └─────────┬──────────┘     │
│         │                                  │                 │
│         │ Shared Design Tokens            │                 │
│         └──────────┬──────────────────────┘                 │
│                    │                                         │
│         ┌──────────▼──────────┐                            │
│         │  style-dictionary/  │                            │
│         │  (Source of Truth)  │                            │
│         └─────────────────────┘                            │
└─────────────────────────────────────────────────────────────┘
```

## Build Pipeline

```
┌─────────────┐
│   React     │
│   Build     │
│             │
│ gulp        │
│ quick-build │
└──────┬──────┘
       │
       │ Produces
       ▼
┌──────────────────┐
│ lib/components/  │
│ ├── badge/       │
│ │   └── styles.  │      ┌──────────────┐
│ │       css.js   │──────▶ CSS         │
│ ├── button/      │      │ Extraction  │
│ │   └── styles.  │      │             │
│ │       css.js   │      │ rust-       │
│ └── ...          │      │ styles.js   │
└──────────────────┘      └──────┬───────┘
                                 │
                                 │ Generates
                                 ▼
                          ┌──────────────────────┐
                          │ cloudscape-          │
                          │ components.css       │
                          │                      │
                          │ (Combined bundle)    │
                          └──────┬───────────────┘
                                 │
       ┌─────────────────────────┴───────────────┐
       │                                         │
       ▼                                         ▼
┌─────────────────┐                    ┌─────────────────┐
│ Rust/WASM Build │                    │ Demo Page       │
│                 │                    │                 │
│ wasm-pack       │                    │ demo.html       │
│ + cargo         │                    │ <link css>      │
└────────┬────────┘                    └─────────────────┘
         │
         │ Produces
         ▼
┌─────────────────────────┐
│ dist/wasm/              │
│ ├── cloudscape_         │
│ │   components.js       │
│ ├── cloudscape_         │
│ │   components_bg.wasm  │
│ └── package.json        │
└─────────────────────────┘
```

## Component Architecture

### React Component (Existing)

```typescript
// src/button/index.tsx
interface ButtonProps {
  variant?: 'primary' | 'normal';
  disabled?: boolean;
  onClick?: (event) => void;
}

export default function Button(props: ButtonProps) {
  return (
    <button 
      className={styles.button}
      disabled={props.disabled}
      onClick={props.onClick}
    >
      {props.children}
    </button>
  );
}
```

### Rust/Yew Component (New)

```rust
// rust-components/crates/components/src/button.rs
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub on_click: Option<Callback<ClickEvent>>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button 
            class={classes}
            disabled={props.disabled}
            onclick={onclick_handler}
        >
            { props.children.clone() }
        </button>
    }
}
```

## Data Flow

### Props Flow

```
User Code
   │
   │ Sets props
   ▼
Component
   │
   │ Validates & transforms
   ▼
Internal State
   │
   │ Renders
   ▼
DOM Element
```

### Event Flow

```
DOM Event
   │
   │ Browser triggers
   ▼
Yew Event Handler
   │
   │ Transforms to CustomEvent
   ▼
User Callback
   │
   │ Application logic
   ▼
State Update
   │
   │ Triggers re-render
   ▼
Updated DOM
```

### Styling Flow

```
Design Tokens (style-dictionary/)
   │
   ├─────────────┬─────────────┐
   │             │             │
   ▼             ▼             ▼
TypeScript    CSS Vars     Rust Enums
(React)       (Shared)     (Future)
   │             │             │
   │             └─────┬───────┘
   │                   │
   ▼                   ▼
React SCSS         Rust Styles
   │                   │
   │ Compiled          │ Used
   ▼                   │
styles.css.js          │
   │                   │
   │ Extracted         │
   ▼                   │
Combined CSS ◀─────────┘
   │
   │ Loaded in browser
   ▼
Styled Components
```

## Module Structure

### Design Tokens Crate

```rust
cloudscape-design-tokens/
├── src/
│   └── lib.rs
│       ├── pub struct DesignTokens
│       ├── pub struct ColorTokens
│       ├── pub struct SpacingTokens
│       ├── pub enum Mode (Light/Dark)
│       └── Token access methods
```

**Purpose**: Type-safe design token definitions

**Dependencies**: None (standalone)

### Components Crate

```rust
cloudscape-components/
├── src/
│   ├── lib.rs              # Public API
│   ├── badge.rs            # Badge component
│   ├── spinner.rs          # Spinner component
│   ├── box_component.rs    # Box component
│   ├── button.rs           # Button component
│   └── internal/           # Internal utilities
│       ├── mod.rs
│       ├── base_component.rs
│       ├── classes.rs
│       ├── events.rs
│       ├── props.rs
│       ├── styles.rs
│       ├── analytics.rs
│       └── accessibility.rs
```

**Purpose**: Main component library compiled to WASM

**Dependencies**: 
- yew (framework)
- wasm-bindgen (JS interop)
- web-sys (DOM APIs)
- gloo (utilities)
- serde (JSON serialization)
- design-tokens (sibling crate)

## Internal Utilities

### Base Component

Provides:
- Component metadata (version, theme)
- Base props shared across components
- Tracking and analytics hooks

### Classes Builder

```rust
ClassBuilder::new()
    .add("awsui-button")
    .add_conditional("awsui-button-disabled", disabled)
    .add_variant("variant", variant.as_str())
    .build()
```

**Purpose**: Dynamic CSS class name generation

### Events System

```rust
pub struct CustomEvent<T> {
    pub detail: T,
    pub prevent_default: bool,
}

pub struct ClickEvent {
    pub button: u16,
    pub ctrl_key: bool,
    pub shift_key: bool,
    // ...
}
```

**Purpose**: Type-safe event handling with detail payloads

### Styles System

```rust
ComponentStyles {
    classes: Vec<String>,
    custom_properties: HashMap<String, String>,
    inline_style: Option<String>,
}

BadgeStyle {
    background: Option<String>,
    border_color: Option<String>,
    // ...
}
```

**Purpose**: Component-specific styling with CSS variable support

### Analytics

```rust
AnalyticsMetadata::button("submit-btn", "primary", false)
    .with_funnel("checkout", "step2")
    .with_detail(json!({"campaign": "promo"}))
```

**Purpose**: Consistent analytics tracking across components

### Accessibility

```rust
AriaAttributes {
    label: Some("Close dialog"),
    expanded: Some(false),
    controls: Some("dialog-1"),
    has_popup: Some(AriaHasPopup::Dialog),
    // ...
}
```

**Purpose**: Complete ARIA attribute support

## Compilation Targets

### WASM Target Configuration

```toml
[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "opt-level=z",      # Size optimization
  "-C", "lto=fat",          # Link-time optimization
  "-C", "codegen-units=1",  # Single codegen unit
]
```

**Output**: Highly optimized WASM binary (~50-100KB gzipped)

### Build Profiles

```toml
[profile.release]
opt-level = "z"    # Optimize for size
lto = true         # Enable LTO
panic = "abort"    # Smaller panic handler
strip = true       # Remove debug symbols
```

## Integration Points

### JavaScript Interop

```rust
// Rust side
#[wasm_bindgen(start)]
pub fn init() {
    // Initialization
}

#[wasm_bindgen]
pub fn version() -> String {
    VERSION.to_string()
}
```

```javascript
// JS side
import init, { version } from './cloudscape_components.js';

await init();
console.log('Version:', version());
```

### CSS Integration

```html
<!-- Load shared CSS -->
<link rel="stylesheet" href="cloudscape-components.css">

<!-- WASM module will use these classes -->
<script type="module">
  import init from './cloudscape_components.js';
  await init();
</script>
```

## Security Architecture

### Memory Safety

- ✅ **No buffer overflows**: Rust bounds checking
- ✅ **No use-after-free**: Ownership system
- ✅ **No data races**: Borrow checker
- ✅ **No null pointers**: Option<T> type

### FFI Boundary

```rust
// Safe Rust (99% of code)
pub fn button(props: &ButtonProps) -> Html { ... }

// Unsafe only at FFI boundary (automatically generated)
#[wasm_bindgen]
extern "C" {
    fn console_log(s: &str);
}
```

**Principle**: Unsafe code limited to auto-generated wasm-bindgen glue

### Input Validation

All user inputs validated at component boundaries:
- Props validated by type system
- String inputs sanitized before DOM insertion
- Event data transformed to safe types

## Performance Optimizations

### Compile-Time Optimizations

1. **Dead code elimination**: Unused code removed
2. **Constant folding**: Compile-time evaluation
3. **Inline expansion**: Small functions inlined
4. **LLVM optimizations**: Full optimization pipeline

### Runtime Optimizations

1. **Zero-cost abstractions**: No runtime overhead for abstractions
2. **Monomorphization**: Generic functions specialized per type
3. **Efficient memory layout**: Compact data structures
4. **Direct DOM manipulation**: No virtual DOM overhead

### Bundle Size Optimizations

1. **LTO**: Cross-crate inlining and optimization
2. **Size-optimized compilation**: opt-level="z"
3. **Symbol stripping**: Remove debug info
4. **WASM optimization**: wasm-opt post-processing (optional)

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn badge_color_variants() {
        // Test logic
    }
}
```

**Location**: Alongside source files

### WASM Tests

```rust
#[wasm_bindgen_test]
fn button_renders_in_browser() {
    // DOM tests in actual browser
}
```

**Runs in**: Headless browser (Firefox)

### Integration Tests

```typescript
// src/__integ__/rust-components.test.ts
describe('Rust Button', () => {
  it('renders correctly', async () => {
    await initWasm();
    // Test with real browser
  });
});
```

**Location**: Existing test infrastructure

## Future Architecture Considerations

### Design Token Generator

```
style-dictionary/ (TS)
        │
        │ compile
        ▼
lib/style-dictionary/ (JSON)
        │
        │ read & transform
        ▼
shared-design-tokens/outputs/rust/ (RS)
        │
        │ include in build
        ▼
design-tokens crate
```

### Component Registry

Potential future enhancement:

```rust
ComponentRegistry::new()
    .register::<Button>("Button")
    .register::<Badge>("Badge")
    .build()
```

For dynamic component loading and hot reloading.

### Theming System

```rust
Theme::builder()
    .mode(Mode::Dark)
    .density(Density::Compact)
    .motion(Motion::Disabled)
    .apply_to_root()
```

For runtime theme switching.

## Deployment Architecture

### NPM Package Structure

```
@cloudscape-design/components/
├── lib/
│   ├── components/        # React (existing)
│   └── rust-components/   # Rust/WASM (new)
│       ├── cloudscape_components.js
│       ├── cloudscape_components_bg.wasm
│       └── cloudscape-components.css
├── package.json
└── README.md
```

### Import Paths

```javascript
// React components
import { Button } from '@cloudscape-design/components';

// Rust/WASM components
import init from '@cloudscape-design/components/rust-components/cloudscape_components.js';
```

### Browser Detection

```javascript
async function loadComponents() {
  if (typeof WebAssembly === 'object') {
    // Use Rust/WASM components
    await init();
    return 'wasm';
  } else {
    // Fallback to React components
    return 'react';
  }
}
```

## Maintenance Guidelines

### Adding New Components

1. Create `src/[component].rs`
2. Follow existing patterns (see `badge.rs` for simplest example)
3. Add to `lib.rs` exports
4. Write unit tests
5. Update documentation
6. Add to demo page

### Updating Existing Components

1. Modify source file
2. Run `cargo test`
3. Run `cargo fmt` and `cargo clippy`
4. Test with demo page
5. Verify CSS compatibility

### Breaking Changes

1. Update CHANGELOG
2. Increment version in Cargo.toml
3. Update migration guide
4. Deprecate old API (don't remove immediately)

