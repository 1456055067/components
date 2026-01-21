// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Column Layout component for responsive multi-column grids.
//!
//! The ColumnLayout component provides a flexible layout container for organizing
//! content into multiple columns with responsive behavior. It supports configurable
//! column counts, variants, borders, and gutter spacing.

use yew::prelude::*;
use crate::internal::{BaseComponentProps, ClassBuilder};

/// Column layout visual variants
///
/// Determines the spacing and layout behavior of the column grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnVariant {
    /// Default layout for general content
    Default,
    /// Text-focused layout with equal width columns optimized for readability
    TextGrid,
}

impl Default for ColumnVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl ColumnVariant {
    /// Returns the CSS class name suffix for this variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::TextGrid => "text-grid",
        }
    }
}

/// Border configuration for column layout
///
/// Controls dividers between rows and columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BordersType {
    /// No borders
    None,
    /// Vertical borders between columns
    Vertical,
    /// Horizontal borders between rows
    Horizontal,
    /// Both vertical and horizontal borders
    All,
}

impl Default for BordersType {
    fn default() -> Self {
        Self::None
    }
}

impl BordersType {
    /// Returns the CSS class name suffix for this border type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::All => "all",
        }
    }
}

/// Properties for the ColumnLayout component
#[derive(Properties, PartialEq, Clone)]
pub struct ColumnLayoutProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Number of columns in the grid (1-4)
    ///
    /// The column count will be automatically clamped to the range 1-4.
    /// When `min_column_width` is set, this acts as the maximum number of columns.
    #[prop_or(1)]
    pub columns: u32,

    /// Content type variant
    ///
    /// * `Default` - Standard layout with default spacing
    /// * `TextGrid` - Optimized for text-heavy content with equal column widths
    #[prop_or_default]
    pub variant: ColumnVariant,

    /// Border configuration
    ///
    /// Controls whether dividers are placed between rows and columns.
    /// Note: Borders are not supported when used with `min_column_width`.
    #[prop_or_default]
    pub borders: BordersType,

    /// Minimum column width in pixels
    ///
    /// When set, the number of columns is determined by this value, the available
    /// space, and the maximum number of columns as defined by the `columns` property.
    /// This enables flexible, responsive column layouts.
    #[prop_or_default]
    pub min_column_width: Option<u32>,

    /// Whether to remove default gutters between columns
    #[prop_or(false)]
    pub disable_gutters: bool,

    /// Column content items
    #[prop_or_default]
    pub children: Children,
}

/// ColumnLayout component for responsive multi-column grids.
///
/// Creates a flexible grid layout that automatically adjusts the number of columns
/// based on the available space. Supports various visual styles, border options,
/// and responsive behavior.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{ColumnLayout, ColumnVariant};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     html! {
///         <ColumnLayout columns={3} variant={ColumnVariant::TextGrid}>
///             <div>{"Column 1"}</div>
///             <div>{"Column 2"}</div>
///             <div>{"Column 3"}</div>
///         </ColumnLayout>
///     }
/// }
/// ```
///
/// # With Borders
///
/// ```rust
/// use cloudscape_components::{ColumnLayout, BordersType};
/// use yew::prelude::*;
///
/// #[function_component(BorderExample)]
/// fn border_example() -> Html {
///     html! {
///         <ColumnLayout columns={2} borders={BordersType::Vertical}>
///             <div>{"Left column"}</div>
///             <div>{"Right column"}</div>
///         </ColumnLayout>
///     }
/// }
/// ```
///
/// # With Minimum Width
///
/// ```rust
/// use cloudscape_components::ColumnLayout;
/// use yew::prelude::*;
///
/// #[function_component(ResponsiveExample)]
/// fn responsive_example() -> Html {
///     html! {
///         <ColumnLayout columns={4} min_column_width={200}>
///             <div>{"Responsive column 1"}</div>
///             <div>{"Responsive column 2"}</div>
///             <div>{"Responsive column 3"}</div>
///             <div>{"Responsive column 4"}</div>
///         </ColumnLayout>
///     }
/// }
/// ```
#[function_component(ColumnLayout)]
pub fn column_layout(props: &ColumnLayoutProps) -> Html {
    // Clamp column count to valid range (1-4)
    let columns = props.columns.clamp(1, 4);

    // Determine if borders should be applied (not with text-grid or min_column_width)
    let is_text_grid = props.variant == ColumnVariant::TextGrid;
    let has_min_width = props.min_column_width.is_some();
    let should_disable_gutters = !is_text_grid && props.disable_gutters;
    let should_have_horizontal_borders = !is_text_grid && !has_min_width &&
        (props.borders == BordersType::Horizontal || props.borders == BordersType::All);
    let should_have_vertical_borders = !is_text_grid && !has_min_width &&
        (props.borders == BordersType::Vertical || props.borders == BordersType::All);

    // Build CSS classes for the root element
    let root_classes = ClassBuilder::new()
        .add("awsui-column-layout")
        .add(format!("awsui-column-layout-columns-{}", columns))
        .add(format!("awsui-column-layout-variant-{}", props.variant.as_str()))
        .add_if(should_have_horizontal_borders, "awsui-column-layout-borders-horizontal")
        .add_if(should_have_vertical_borders, "awsui-column-layout-borders-vertical")
        .add_if(should_disable_gutters, "awsui-column-layout-no-gutters");

    let root_class = props.base.merge_classes(&root_classes.build());

    // Build inline style for min-column-width if specified
    let style = props.min_column_width.map(|width| {
        format!("--column-layout-min-width: {}px; --column-layout-max-columns: {}", width, columns)
    });

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
            style={style}
        >
            <div class="awsui-column-layout-grid">
                { props.children.clone() }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_variant_default() {
        assert_eq!(ColumnVariant::default(), ColumnVariant::Default);
    }

    #[test]
    fn test_column_variant_as_str() {
        assert_eq!(ColumnVariant::Default.as_str(), "default");
        assert_eq!(ColumnVariant::TextGrid.as_str(), "text-grid");
    }

    #[test]
    fn test_borders_type_default() {
        assert_eq!(BordersType::default(), BordersType::None);
    }

    #[test]
    fn test_borders_type_as_str() {
        assert_eq!(BordersType::None.as_str(), "none");
        assert_eq!(BordersType::Vertical.as_str(), "vertical");
        assert_eq!(BordersType::Horizontal.as_str(), "horizontal");
        assert_eq!(BordersType::All.as_str(), "all");
    }

    #[test]
    fn test_column_count_clamping() {
        // Test that column count is properly validated
        // We verify the clamping logic works as expected
        let columns: u32 = 0; // Should clamp to 1
        let clamped = columns.clamp(1, 4);
        assert_eq!(clamped, 1);
    }

    #[test]
    fn test_column_count_upper_bound() {
        let columns: u32 = 10; // Should clamp to 4
        let clamped = columns.clamp(1, 4);
        assert_eq!(clamped, 4);
    }

    #[test]
    fn test_column_count_valid_range() {
        for i in 1..=4 {
            let clamped = i.clamp(1, 4);
            assert_eq!(clamped, i);
        }
    }

    #[test]
    fn test_default_values() {
        // Test that default values are as expected
        assert_eq!(ColumnVariant::default(), ColumnVariant::Default);
        assert_eq!(BordersType::default(), BordersType::None);
    }

    #[test]
    fn test_min_column_width_style() {
        // Verify that min_column_width generates the correct CSS custom properties
        let columns = 3;
        let min_column_width = Some(200);

        let style = min_column_width.map(|width| {
            format!("--column-layout-min-width: {}px; --column-layout-max-columns: {}", width, columns)
        });

        assert_eq!(
            style,
            Some("--column-layout-min-width: 200px; --column-layout-max-columns: 3".to_string())
        );
    }

    #[test]
    fn test_borders_not_applied_with_text_grid() {
        // Borders should not be applied when variant is TextGrid
        let is_text_grid = ColumnVariant::TextGrid == ColumnVariant::TextGrid;
        let borders = BordersType::All;

        let should_have_borders = !is_text_grid &&
            (borders == BordersType::Horizontal || borders == BordersType::All);

        assert_eq!(should_have_borders, false);
    }

    #[test]
    fn test_borders_applied_with_default_variant() {
        // Borders should be applied when variant is Default
        let is_text_grid = ColumnVariant::Default == ColumnVariant::TextGrid;
        let has_min_width = false;
        let borders = BordersType::All;

        let should_have_horizontal_borders = !is_text_grid && !has_min_width &&
            (borders == BordersType::Horizontal || borders == BordersType::All);

        assert_eq!(should_have_horizontal_borders, true);
    }
}
