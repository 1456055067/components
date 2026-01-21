# Cloudscape Design System - Rust/Yew Components

This directory contains Rust/WebAssembly implementations of Cloudscape Design System components using the [Yew](https://yew.rs/) framework.

## Overview

The Rust/Yew components are designed to:

- **Improve Performance**: Leverage Rust's zero-cost abstractions and WASM for faster runtime performance
- **Enhance Security**: Reduce vulnerabilities through Rust's memory safety guarantees
- **Maintain Compatibility**: Share design tokens and styles with the React implementation
- **Enable Gradual Migration**: Coexist with React components in the same application

## Project Structure

```text
rust-components/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── components/         # Main Yew components library
│   │   ├── src/
│   │   │   ├── badge.rs    # Badge component
│   │   │   ├── spinner.rs  # Spinner component
│   │   │   ├── box_component.rs # Box layout component
│   │   │   ├── button.rs   # Button component
│   │   │   └── internal/   # Shared utilities
│   │   └── Cargo.toml
│   └── design-tokens/      # Design token types
│       ├── src/
│       └── Cargo.toml
├── examples/               # Demo applications (coming soon)
└── dist/                   # Build output (WASM, JS bindings)

```

## Prerequisites

1. **Rust**: Install from [rustup.rs](https://rustup.rs/)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. **wasm-pack**: Install for building WASM

   ```bash
   cargo install wasm-pack
   ```

1. **Node.js**: Required for the design token generator (already installed for React build)

## Building

The Rust components are integrated into the main Gulp build system.

### Build Commands

```bash
# Build Rust components (production) - includes CSS extraction and WASM compilation
npm run rust:build
# or
npx gulp rust:build

# Development build (faster, larger bundle)
npx gulp rust:build:dev

# Build only CSS bundle
npx gulp rust:styles

# Watch for changes
npx gulp rust:watch

# Run tests
npx gulp rust:test

# Check code without building
npx gulp rust:check

# Clean build artifacts
npx gulp rust:clean
```

### Build Process

The build automatically:

1. **Extracts CSS** from React component builds (`lib/components/*/styles.css.js`)
2. **Combines CSS** into a single bundle (`cloudscape-components.css`)
3. **Minifies CSS** for production (`cloudscape-components.min.css`)
4. **Compiles Rust to WASM** using wasm-pack with size optimizations
5. **Copies outputs** to `dist/` and `lib/rust-components/`

If React components aren't built yet, placeholder styles with design tokens are generated.

## Usage Example

```rust
use yew::prelude::*;
use cloudscape_components::{Button, ButtonVariant, Badge, BadgeColor};

#[function_component(App)]
fn app() -> Html {
    let on_click = Callback::from(|event| {
        web_sys::console::log_1(&"Button clicked!".into());
    });

    html! {
        <div>
            <Button
                variant={ButtonVariant::Primary}
                on_click={on_click}
            >
                {"Click me"}
            </Button>

            <Badge color={BadgeColor::Blue}>
                {"New"}
            </Badge>
        </div>
    }
}
```

## Components

### Implemented (37 Total)

#### Basic Components

- ✅ **Badge** - Small visual indicator for labels and metadata
- ✅ **Spinner** - Loading indicator with animation
- ✅ **Box** - Layout utility for spacing and typography
- ✅ **Button** - Interactive button with multiple variants
- ✅ **Alert** - Contextual feedback messages with different severity levels
- ✅ **Link** - Hyperlink with external icon support
- ✅ **Icon** - SVG icon component with built-in and custom icon support

#### Form Components

- ✅ **Input** - Text input field with validation and multiple types
- ✅ **Checkbox** - Checkbox input with indeterminate state support
- ✅ **Toggle** - Toggle switch component (on/off control)
- ✅ **RadioGroup** - Radio button group with multiple options
- ✅ **Select** - Dropdown selection with keyboard navigation
- ✅ **Textarea** - Multi-line text input with auto-resize
- ✅ **FormField** - Form field wrapper with label, description, and validation
- ✅ **Multiselect** - Multiple selection dropdown with token display
- ✅ **Autosuggest** - Input with suggestions dropdown
- ✅ **DatePicker** - Date selection with calendar
- ✅ **DateRangePicker** - Date range selection (absolute/relative)
- ✅ **FileUpload** - File upload with drag-and-drop

#### Layout Components

- ✅ **Container** - Content container with header, footer, and media slots
- ✅ **Header** - Page and section headers (H1/H2/H3)
- ✅ **AppLayout** - Main application shell with navigation, tools, and split panel
- ✅ **ContentLayout** - Content wrapper with header and notifications
- ✅ **ColumnLayout** - Multi-column layout wrapper with responsive grids

#### Navigation Components

- ✅ **Tabs** - Tabbed navigation with active state and dismissible tabs
- ✅ **SideNavigation** - Sidebar navigation with hierarchical items
- ✅ **Breadcrumbs** - Breadcrumb trail for navigation hierarchy
- ✅ **Pagination** - Page navigation with next/previous and page numbers
- ✅ **TopNavigation** - Top application bar with menu items and utilities

#### Data Display Components

- ✅ **StatusIndicator** - Status badges with color variants and icons
- ✅ **KeyValuePairs** - Structured key-value pair display
- ✅ **ExpandableSection** - Collapsible content sections
- ✅ **Table** - Data table with sorting, filtering, selection, and pagination

#### Overlay Components

- ✅ **Modal** - Dialog/modal component with size variants and dismiss handling
- ✅ **Popover** - Popover overlay with positioning and dismissible content
- ✅ **Drawer** - Side drawer panel that slides in from the edge

#### Notification Components

- ✅ **Flashbar** - Notification message container that displays multiple flash messages

### Planned

- ⏳ **Cards** - Card grid with selection
- ⏳ **PropertyFilter** - Advanced filtering component
- ⏳ More components coming...

## Design Tokens

Design tokens are shared between React and Rust implementations through a code generation process:

1. Tokens are defined in `style-dictionary/` (TypeScript)
2. The generator script (`shared-design-tokens/generate-rust-tokens.js`) creates Rust types
3. Both React and Rust components use the same CSS custom properties

To regenerate Rust tokens:

```bash
node shared-design-tokens/generate-rust-tokens.js
```

## Integration with React

The Rust/WASM components can be used alongside React components in the same application:

```javascript
import init, { Badge } from './rust-components/dist/wasm/cloudscape_components.js';

// Initialize WASM module
await init();

// Use in your app (integration layer needed)
```

## Performance

Initial benchmarks (coming soon) will compare:

- Initial render time
- Re-render performance
- Memory usage
- Bundle size
- Time to interactive

## Demo Page

An interactive demo is available at [examples/demo.html](examples/demo.html).

To view:

```bash
# Build components first
npx gulp rust:build

# Serve with any static server from project root
python3 -m http.server 8000

# Open http://localhost:8000/rust-components/examples/demo.html
```

The demo page showcases all implemented components with visual examples.

## Development

### Code Style

Follow Rust best practices:

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check without building
cargo check
```

## Contributing

When adding new components:

1. Create a new module in `crates/components/src/`
2. Follow the existing component patterns (see `badge.rs` as the simplest example)
3. Export from `lib.rs`
4. Add tests
5. Update this README

## License

Apache-2.0

## Resources

- [Yew Documentation](https://yew.rs/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Cloudscape Design System](https://cloudscape.design/)
