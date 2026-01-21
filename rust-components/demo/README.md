# Cloudscape Components Demo

An interactive WASM demo application showcasing all 44 Cloudscape Design System components implemented in Rust/Yew.

## Overview

This demo provides a comprehensive component gallery with:

- Live, interactive examples of all components
- Code snippets showing usage patterns
- Component configuration options
- Real-world use cases
- Clean, professional design using Cloudscape components

## Component Categories

### Basic Components (11)
- Badge, Spinner, Box, Button, ButtonDropdown, Alert, Link, Icon, ProgressBar, CopyToClipboard, TextContent

### Form Components (13)
- Input, Checkbox, Toggle, RadioGroup, Tiles, Select, Textarea, FormField, Multiselect, Autosuggest, DatePicker, DateRangePicker, FileUpload

### Layout Components (6)
- Container, Header, AppLayout, ContentLayout, ColumnLayout, SpaceBetween

### Navigation Components (5)
- Tabs, SideNavigation, Breadcrumbs, Pagination, TopNavigation

### Data Display Components (6)
- StatusIndicator, KeyValuePairs, ExpandableSection, Table, Cards, TokenGroup

### Overlay Components (3)
- Modal, Popover, Drawer

### Notification Components (1)
- Flashbar

## Prerequisites

Before running the demo, ensure you have:

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Trunk** (WASM build tool)
   ```bash
   cargo install trunk
   ```

3. **wasm32-unknown-unknown target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Building the Demo

### Development Build

For faster builds during development:

```bash
cd rust-components/demo
trunk serve
```

This will:
- Build the application in debug mode
- Start a development server at `http://127.0.0.1:8080`
- Watch for file changes and rebuild automatically
- Open your browser automatically

### Production Build

For optimized production builds:

```bash
cd rust-components/demo
trunk build --release
```

The optimized output will be in the `dist/` directory.

## Project Structure

```
demo/
├── Cargo.toml              # Demo project dependencies
├── Trunk.toml              # Trunk configuration
├── index.html              # HTML template
├── README.md               # This file
└── src/
    ├── main.rs             # Entry point
    ├── app.rs              # Main app component with routing
    ├── components/         # Demo-specific components
    │   ├── mod.rs
    │   ├── sidebar.rs      # Navigation sidebar
    │   └── code_snippet.rs # Code display component
    └── pages/              # Demo pages
        ├── mod.rs
        ├── home.rs         # Landing page
        ├── basic.rs        # Basic components demos
        ├── forms.rs        # Form components demos
        ├── layout.rs       # Layout components demos
        ├── navigation.rs   # Navigation components demos
        ├── data_display.rs # Data display demos
        ├── overlay.rs      # Overlay components demos
        └── notification.rs # Notification demos
```

## Architecture

### Routing

The demo uses a simple state-based routing system:

```rust
pub enum Route {
    Home,
    Basic,
    Forms,
    Layout,
    Navigation,
    DataDisplay,
    Overlay,
    Notification,
}
```

Navigation is handled through the `SideNavigation` component with callbacks that update the current route state.

### Component Organization

Each demo page follows a consistent structure:

1. **Header** - Page title and description
2. **Sections** - Grouped by component
3. **Examples** - Live component demonstrations
4. **Code Snippets** - Usage examples with syntax highlighting

### State Management

The demo uses Yew's built-in hooks for state management:

- `use_state` - Local component state
- `Callback` - Event handlers and callbacks
- Component properties for parent-child communication

## Features

### Interactive Examples

All components are fully functional and can be interacted with:

- Click buttons to see effects
- Fill out forms to test validation
- Toggle components to see state changes
- Trigger modals and drawers
- Add and dismiss flash messages

### Code Snippets

Each example includes a code snippet showing:

- Component instantiation
- Property configuration
- Event handlers
- Common patterns

### Responsive Design

The demo uses `AppLayout` which provides:

- Collapsible sidebar navigation
- Responsive breakpoints
- Mobile-friendly layout
- Consistent spacing

## Development Tips

### Hot Reload

Trunk provides hot reloading out of the box. Simply save your files and the browser will automatically refresh.

### Debugging

Use browser developer tools to:

- Inspect WASM modules
- View console logs
- Debug component state
- Monitor network requests

### Adding New Examples

To add a new component example:

1. Navigate to the appropriate page file (e.g., `src/pages/basic.rs`)
2. Add a new Container with the component demo
3. Include a CodeSnippet showing usage
4. Update the component count in the sidebar if needed

## Building from Parent Directory

You can also build the demo from the rust-components root:

```bash
# From rust-components/ directory
cd demo
trunk serve
```

Or integrate it into the main build system:

```bash
# From project root
cd rust-components/demo && trunk build --release
```

## Performance

The demo is optimized for performance:

- **Bundle Size**: Optimized with `opt-level = "z"` and LTO
- **Load Time**: Fast initial load with WASM compilation
- **Runtime**: Near-native performance with Rust/WASM
- **Memory**: Efficient memory usage with Rust's ownership system

## Browser Compatibility

The demo works in all modern browsers that support WebAssembly:

- Chrome/Edge 89+
- Firefox 78+
- Safari 15+

## Troubleshooting

### Build Errors

If you encounter build errors:

1. Ensure you're using the latest stable Rust:
   ```bash
   rustup update stable
   ```

2. Clean the build cache:
   ```bash
   trunk clean
   cargo clean
   ```

3. Verify the wasm target is installed:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

### CSS Not Loading

If styles aren't loading:

1. Build the parent components first:
   ```bash
   cd ..
   npm run rust:build
   ```

2. Ensure the CSS path in `index.html` is correct:
   ```html
   <link data-trunk rel="css" href="../dist/cloudscape-components.css" />
   ```

### Port Already in Use

If port 8080 is already in use:

```bash
trunk serve --port 8081
```

Or update `Trunk.toml`:

```toml
[serve]
port = 8081
```

## Contributing

To contribute to the demo:

1. Follow the existing code structure
2. Add examples for new components
3. Include code snippets for all examples
4. Test interactive functionality
5. Ensure responsive behavior

## Resources

- [Cloudscape Design System](https://cloudscape.design/)
- [Yew Framework](https://yew.rs/)
- [Trunk Build Tool](https://trunkrs.dev/)
- [WebAssembly](https://webassembly.org/)

## License

Apache-2.0

## Questions?

For questions or issues:

1. Check the main [rust-components README](../README.md)
2. Review component documentation in the source files
3. Open an issue in the repository
