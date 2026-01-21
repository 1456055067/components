// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Table component for displaying tabular data with sorting, filtering, selection, and pagination.
//!
//! A data table component that supports column definitions, row data, sorting,
//! selection (single or multiple), loading states, empty states, and pagination.

use crate::internal::{BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent};
use web_sys::MouseEvent;
use yew::prelude::*;

/// Direction for sorting
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SortDirection {
    /// Ascending sort (A-Z, 0-9)
    Ascending,
    /// Descending sort (Z-A, 9-0)
    Descending,
}

/// Sorting state for the table
#[derive(Debug, Clone, PartialEq)]
pub struct SortingState {
    /// ID of the column currently being sorted
    pub sort_column_id: Option<String>,
    /// Direction of the sort
    pub sort_direction: SortDirection,
}

impl Default for SortingState {
    fn default() -> Self {
        Self {
            sort_column_id: None,
            sort_direction: SortDirection::Ascending,
        }
    }
}

/// Event detail for sort change events
#[derive(Debug, Clone, PartialEq)]
pub struct TableSortDetail {
    /// ID of the column to sort by
    pub column_id: String,
    /// Direction to sort in
    pub direction: SortDirection,
}

/// Type of selection supported by the table
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SelectionType {
    /// Single row selection
    Single,
    /// Multiple row selection with checkboxes
    Multi,
}

/// Event detail for selection change events
#[derive(Debug, Clone, PartialEq)]
pub struct TableSelectionDetail<T: Clone + PartialEq> {
    /// List of selected items
    pub selected_items: Vec<T>,
}

/// Definition of a table column
///
/// A column specifies how data should be displayed, including the header text,
/// cell rendering function, sortability, and width constraints.
#[derive(Clone, Debug)]
pub struct TableColumn<T: Clone + PartialEq + 'static> {
    /// Unique identifier for this column
    pub id: String,
    /// Header text to display
    pub header: String,
    /// Function to render cell content for this column
    pub cell: fn(&T) -> Html,
    /// Whether this column can be sorted
    pub sortable: bool,
    /// CSS width for the column (e.g., "200px", "20%")
    pub width: Option<String>,
    /// Minimum CSS width for the column
    pub min_width: Option<String>,
}

impl<T: Clone + PartialEq + 'static> TableColumn<T> {
    /// Creates a new table column
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the column
    /// * `header` - Header text to display
    /// * `cell` - Function to render cell content
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::TableColumn;
    /// use yew::prelude::*;
    ///
    /// #[derive(Clone, PartialEq)]
    /// struct User {
    ///     name: String,
    /// }
    ///
    /// let column = TableColumn::new("name", "Name", |user: &User| html! { {&user.name} });
    /// ```
    pub fn new(id: impl Into<String>, header: impl Into<String>, cell: fn(&T) -> Html) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            cell,
            sortable: false,
            width: None,
            min_width: None,
        }
    }

    /// Sets whether this column is sortable
    pub fn with_sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Sets the width for this column
    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the minimum width for this column
    pub fn with_min_width(mut self, min_width: impl Into<String>) -> Self {
        self.min_width = Some(min_width.into());
        self
    }
}

// Manual PartialEq implementation that doesn't compare function pointers
impl<T: Clone + PartialEq + 'static> PartialEq for TableColumn<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.header == other.header
            && self.sortable == other.sortable
            && self.width == other.width
            && self.min_width == other.min_width
    }
}

/// Properties for the Table component
#[derive(Properties, Clone)]
pub struct TableProps<T: Clone + PartialEq + 'static> {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Column definitions for the table
    pub columns: Vec<TableColumn<T>>,

    /// Data items to display in the table
    #[prop_or_default]
    pub items: Vec<T>,

    /// Type of selection (single or multi)
    #[prop_or_default]
    pub selection_type: Option<SelectionType>,

    /// Currently selected items (controlled)
    #[prop_or_default]
    pub selected_items: Vec<T>,

    /// Whether the table is in a loading state
    #[prop_or_default]
    pub loading: bool,

    /// Text to display during loading
    #[prop_or_default]
    pub loading_text: Option<String>,

    /// Content to display when the table is empty
    #[prop_or_default]
    pub empty: Option<Html>,

    /// Header content (title, description, actions)
    #[prop_or_default]
    pub header: Option<Html>,

    /// Footer content (typically pagination)
    #[prop_or_default]
    pub footer: Option<Html>,

    /// Current sorting state
    #[prop_or_default]
    pub sorting_state: Option<SortingState>,

    /// Whether to make the header sticky (remains visible on scroll)
    #[prop_or_default]
    pub sticky_header: bool,

    /// Callback fired when selection changes
    #[prop_or_default]
    pub on_selection_change: Option<Callback<CustomEvent<TableSelectionDetail<T>>>>,

    /// Callback fired when sort changes
    #[prop_or_default]
    pub on_sort_change: Option<Callback<CustomEvent<TableSortDetail>>>,
}

impl<T: Clone + PartialEq + 'static> PartialEq for TableProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.columns == other.columns
            && self.items == other.items
            && self.selection_type == other.selection_type
            && self.selected_items == other.selected_items
            && self.loading == other.loading
            && self.loading_text == other.loading_text
            && self.sorting_state == other.sorting_state
            && self.sticky_header == other.sticky_header
    }
}

/// Table component for displaying tabular data
///
/// A flexible data table with support for sorting, selection, loading states,
/// and pagination. The component is fully generic over the item type.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Table, TableColumn, SortDirection, SortingState};
/// use yew::prelude::*;
///
/// #[derive(Clone, PartialEq)]
/// struct User {
///     id: u32,
///     name: String,
///     email: String,
/// }
///
/// #[function_component(MyTable)]
/// fn my_table() -> Html {
///     let columns = vec![
///         TableColumn::new("name", "Name", |user: &User| html! { {&user.name} })
///             .with_sortable(true),
///         TableColumn::new("email", "Email", |user: &User| html! { {&user.email} }),
///     ];
///
///     let items = vec![
///         User { id: 1, name: "Alice".into(), email: "alice@example.com".into() },
///         User { id: 2, name: "Bob".into(), email: "bob@example.com".into() },
///     ];
///
///     html! {
///         <Table<User>
///             columns={columns}
///             items={items}
///             loading={false}
///         />
///     }
/// }
/// ```
#[function_component(Table)]
pub fn table<T: Clone + PartialEq + 'static>(props: &TableProps<T>) -> Html {
    let _metadata = ComponentMetadata::new("Table");

    // Handle sort column click
    let on_sort_click = {
        let on_sort_change = props.on_sort_change.clone();
        let sorting_state = props.sorting_state.clone();

        Callback::from(move |column_id: String| {
            if let Some(callback) = &on_sort_change {
                // Determine new direction
                let new_direction = if let Some(ref state) = sorting_state {
                    if state.sort_column_id.as_ref() == Some(&column_id) {
                        // Toggle direction for same column
                        match state.sort_direction {
                            SortDirection::Ascending => SortDirection::Descending,
                            SortDirection::Descending => SortDirection::Ascending,
                        }
                    } else {
                        // Default to ascending for new column
                        SortDirection::Ascending
                    }
                } else {
                    SortDirection::Ascending
                };

                let detail = TableSortDetail {
                    column_id,
                    direction: new_direction,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }
        })
    };

    // Handle row selection
    let on_row_select = {
        let on_selection_change = props.on_selection_change.clone();
        let selected_items = props.selected_items.clone();
        let selection_type = props.selection_type;

        Callback::from(move |item: T| {
            if let Some(callback) = &on_selection_change {
                let new_selected = match selection_type {
                    Some(SelectionType::Single) => {
                        // Single selection: replace selection
                        vec![item]
                    }
                    Some(SelectionType::Multi) => {
                        // Multi selection: toggle item
                        if selected_items.contains(&item) {
                            selected_items
                                .iter()
                                .filter(|i| *i != &item)
                                .cloned()
                                .collect()
                        } else {
                            let mut new = selected_items.clone();
                            new.push(item);
                            new
                        }
                    }
                    None => vec![],
                };

                let detail = TableSelectionDetail {
                    selected_items: new_selected,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }
        })
    };

    // Handle select all (for multi-select)
    let on_select_all = {
        let on_selection_change = props.on_selection_change.clone();
        let selected_items = props.selected_items.clone();
        let items = props.items.clone();

        Callback::from(move |_e: MouseEvent| {
            if let Some(callback) = &on_selection_change {
                let all_selected = selected_items.len() == items.len() && !items.is_empty();
                let new_selected = if all_selected {
                    // Deselect all
                    vec![]
                } else {
                    // Select all
                    items.clone()
                };

                let detail = TableSelectionDetail {
                    selected_items: new_selected,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }
        })
    };

    // Build root classes
    let root_classes = ClassBuilder::new()
        .add("awsui-table")
        .add_if(props.loading, "awsui-table-loading");

    // Build container classes
    let container_classes = ClassBuilder::new()
        .add("awsui-table-container")
        .add_if(props.sticky_header, "awsui-table-sticky-header");

    // Check if all items are selected (for multi-select checkbox state)
    let all_selected = props.selection_type == Some(SelectionType::Multi)
        && !props.items.is_empty()
        && props.selected_items.len() == props.items.len();

    let some_selected = props.selection_type == Some(SelectionType::Multi)
        && !props.selected_items.is_empty()
        && props.selected_items.len() < props.items.len();

    html! {
        <div class={root_classes.build()} id={props.base.id.clone()}>
            // Header section
            if let Some(ref header) = props.header {
                <div class="awsui-table-header">
                    { header.clone() }
                </div>
            }

            // Table container
            <div class={container_classes.build()}>
                <table class="awsui-table-element" role="table">
                    // Table head
                    <thead class="awsui-table-thead">
                        <tr class="awsui-table-row">
                            // Selection column header (for multi-select)
                            if props.selection_type == Some(SelectionType::Multi) {
                                <th class="awsui-table-header-cell awsui-table-selection-header" scope="col">
                                    <div class="awsui-table-header-cell-content">
                                        <input
                                            type="checkbox"
                                            class="awsui-table-selection-checkbox"
                                            checked={all_selected}
                                            indeterminate={some_selected.to_string()}
                                            onclick={on_select_all}
                                            aria-label="Select all items"
                                        />
                                    </div>
                                </th>
                            } else if props.selection_type == Some(SelectionType::Single) {
                                // Empty header cell for single selection
                                <th class="awsui-table-header-cell awsui-table-selection-header" scope="col">
                                </th>
                            }

                            // Column headers
                            {
                                props.columns.iter().map(|column| {
                                    let is_sorted = props.sorting_state.as_ref()
                                        .and_then(|s| s.sort_column_id.as_ref())
                                        .map(|id| id == &column.id)
                                        .unwrap_or(false);

                                    let sort_direction = if is_sorted {
                                        props.sorting_state.as_ref().map(|s| s.sort_direction)
                                    } else {
                                        None
                                    };

                                    let header_classes = ClassBuilder::new()
                                        .add("awsui-table-header-cell")
                                        .add_if(column.sortable, "awsui-table-header-cell-sortable");

                                    let column_id = column.id.clone();
                                    let on_sort_click_clone = on_sort_click.clone();

                                    let on_header_click = Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();
                                        on_sort_click_clone.emit(column_id.clone());
                                    });

                                    let cell_style = build_cell_style(&column.width, &column.min_width);

                                    // Determine aria-sort attribute
                                    let aria_sort = if is_sorted {
                                        match sort_direction {
                                            Some(SortDirection::Ascending) => Some("ascending"),
                                            Some(SortDirection::Descending) => Some("descending"),
                                            None => None,
                                        }
                                    } else if column.sortable {
                                        Some("none")
                                    } else {
                                        None
                                    };

                                    html! {
                                        <th
                                            key={column.id.clone()}
                                            class={header_classes.build()}
                                            scope="col"
                                            style={cell_style}
                                            aria-sort={aria_sort}
                                        >
                                            if column.sortable {
                                                <button
                                                    type="button"
                                                    class="awsui-table-header-cell-button"
                                                    onclick={on_header_click}
                                                >
                                                    <div class="awsui-table-header-cell-content">
                                                        <span class="awsui-table-header-cell-text">
                                                            { &column.header }
                                                        </span>
                                                        if is_sorted {
                                                            <span class="awsui-table-sort-icon" aria-hidden="true">
                                                                {
                                                                    match sort_direction {
                                                                        Some(SortDirection::Ascending) => "▲",
                                                                        Some(SortDirection::Descending) => "▼",
                                                                        None => "",
                                                                    }
                                                                }
                                                            </span>
                                                        }
                                                    </div>
                                                </button>
                                            } else {
                                                <div class="awsui-table-header-cell-content">
                                                    <span class="awsui-table-header-cell-text">
                                                        { &column.header }
                                                    </span>
                                                </div>
                                            }
                                        </th>
                                    }
                                }).collect::<Html>()
                            }
                        </tr>
                    </thead>

                    // Table body
                    <tbody class="awsui-table-tbody">
                        {
                            if props.loading {
                                // Loading state
                                html! {
                                    <tr class="awsui-table-row">
                                        <td
                                            class="awsui-table-cell awsui-table-loading-cell"
                                            colspan={calculate_colspan(props)}
                                        >
                                            <div class="awsui-table-loading-content">
                                                <span class="awsui-table-loading-spinner">{"⟳"}</span>
                                                <span class="awsui-table-loading-text">
                                                    { props.loading_text.as_deref()
                                                        .unwrap_or("Loading...") }
                                                </span>
                                            </div>
                                        </td>
                                    </tr>
                                }
                            } else if props.items.is_empty() {
                                // Empty state
                                html! {
                                    <tr class="awsui-table-row">
                                        <td
                                            class="awsui-table-cell awsui-table-empty"
                                            colspan={calculate_colspan(props)}
                                        >
                                            {
                                                if let Some(ref empty) = props.empty {
                                                    empty.clone()
                                                } else {
                                                    html! { <div class="awsui-table-empty-default">{"No items found"}</div> }
                                                }
                                            }
                                        </td>
                                    </tr>
                                }
                            } else {
                                // Data rows
                                props.items.iter().map(|item| {
                                    let is_selected = props.selected_items.contains(item);

                                    let row_classes = ClassBuilder::new()
                                        .add("awsui-table-row")
                                        .add_if(is_selected, "awsui-table-row-selected");

                                    let item_clone = item.clone();
                                    let on_row_select_clone = on_row_select.clone();

                                    html! {
                                        <tr class={row_classes.build()} role="row">
                                            // Selection cell
                                            if let Some(selection_type) = props.selection_type {
                                                <td class="awsui-table-cell awsui-table-selection-cell">
                                                    {
                                                        match selection_type {
                                                            SelectionType::Single => {
                                                                let item_for_radio = item_clone.clone();
                                                                let on_select = Callback::from(move |e: MouseEvent| {
                                                                    e.prevent_default();
                                                                    on_row_select_clone.emit(item_for_radio.clone());
                                                                });
                                                                html! {
                                                                    <input
                                                                        type="radio"
                                                                        class="awsui-table-selection-radio"
                                                                        checked={is_selected}
                                                                        onclick={on_select}
                                                                        aria-label="Select item"
                                                                    />
                                                                }
                                                            }
                                                            SelectionType::Multi => {
                                                                let item_for_checkbox = item_clone.clone();
                                                                let on_select = Callback::from(move |e: MouseEvent| {
                                                                    e.prevent_default();
                                                                    on_row_select_clone.emit(item_for_checkbox.clone());
                                                                });
                                                                html! {
                                                                    <input
                                                                        type="checkbox"
                                                                        class="awsui-table-selection-checkbox"
                                                                        checked={is_selected}
                                                                        onclick={on_select}
                                                                        aria-label="Select item"
                                                                    />
                                                                }
                                                            }
                                                        }
                                                    }
                                                </td>
                                            }

                                            // Data cells
                                            {
                                                props.columns.iter().map(|column| {
                                                    let cell_content = (column.cell)(item);
                                                    let cell_style = build_cell_style(&column.width, &column.min_width);

                                                    html! {
                                                        <td
                                                            key={column.id.clone()}
                                                            class="awsui-table-cell"
                                                            style={cell_style}
                                                        >
                                                            { cell_content }
                                                        </td>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </tbody>
                </table>
            </div>

            // Footer section (pagination, etc.)
            if let Some(ref footer) = props.footer {
                <div class="awsui-table-footer">
                    { footer.clone() }
                </div>
            }
        </div>
    }
}

/// Calculates the colspan for loading and empty states
fn calculate_colspan<T: Clone + PartialEq + 'static>(props: &TableProps<T>) -> String {
    let selection_col = if props.selection_type.is_some() { 1 } else { 0 };
    (props.columns.len() + selection_col).to_string()
}

/// Builds the style attribute for a table cell based on width constraints
fn build_cell_style(width: &Option<String>, min_width: &Option<String>) -> Option<String> {
    let mut styles = Vec::new();

    if let Some(w) = width {
        styles.push(format!("width: {}", w));
    }

    if let Some(min_w) = min_width {
        styles.push(format!("min-width: {}", min_w));
    }

    if styles.is_empty() {
        None
    } else {
        Some(styles.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestItem {
        id: u32,
        name: String,
        value: i32,
    }

    #[test]
    fn test_sort_direction_equality() {
        assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
        assert_eq!(SortDirection::Descending, SortDirection::Descending);
        assert_ne!(SortDirection::Ascending, SortDirection::Descending);
    }

    #[test]
    fn test_sorting_state_default() {
        let state = SortingState::default();
        assert_eq!(state.sort_column_id, None);
        assert_eq!(state.sort_direction, SortDirection::Ascending);
    }

    #[test]
    fn test_table_column_builder() {
        let column = TableColumn::new("test", "Test Column", |item: &TestItem| {
            html! { {&item.name} }
        })
        .with_sortable(true)
        .with_width("200px")
        .with_min_width("100px");

        assert_eq!(column.id, "test");
        assert_eq!(column.header, "Test Column");
        assert!(column.sortable);
        assert_eq!(column.width, Some("200px".to_string()));
        assert_eq!(column.min_width, Some("100px".to_string()));
    }

    #[test]
    fn test_table_column_equality() {
        let col1 = TableColumn::new("id", "ID", |item: &TestItem| html! { {item.id} })
            .with_sortable(true);
        let col2 = TableColumn::new("id", "ID", |item: &TestItem| html! { {item.id} })
            .with_sortable(true);

        assert_eq!(col1, col2);
    }

    #[test]
    fn test_selection_type_equality() {
        assert_eq!(SelectionType::Single, SelectionType::Single);
        assert_eq!(SelectionType::Multi, SelectionType::Multi);
        assert_ne!(SelectionType::Single, SelectionType::Multi);
    }

    #[test]
    fn test_table_sort_detail() {
        let detail = TableSortDetail {
            column_id: "name".to_string(),
            direction: SortDirection::Ascending,
        };

        assert_eq!(detail.column_id, "name");
        assert_eq!(detail.direction, SortDirection::Ascending);
    }

    #[test]
    fn test_table_selection_detail() {
        let items = vec![
            TestItem {
                id: 1,
                name: "Item 1".to_string(),
                value: 10,
            },
            TestItem {
                id: 2,
                name: "Item 2".to_string(),
                value: 20,
            },
        ];

        let detail = TableSelectionDetail {
            selected_items: items.clone(),
        };

        assert_eq!(detail.selected_items.len(), 2);
        assert_eq!(detail.selected_items[0].id, 1);
        assert_eq!(detail.selected_items[1].id, 2);
    }

    #[test]
    fn test_build_cell_style_with_width() {
        let style = build_cell_style(&Some("200px".to_string()), &None);
        assert_eq!(style, Some("width: 200px".to_string()));
    }

    #[test]
    fn test_build_cell_style_with_min_width() {
        let style = build_cell_style(&None, &Some("100px".to_string()));
        assert_eq!(style, Some("min-width: 100px".to_string()));
    }

    #[test]
    fn test_build_cell_style_with_both() {
        let style = build_cell_style(&Some("200px".to_string()), &Some("100px".to_string()));
        assert_eq!(style, Some("width: 200px; min-width: 100px".to_string()));
    }

    #[test]
    fn test_build_cell_style_with_neither() {
        let style = build_cell_style(&None, &None);
        assert_eq!(style, None);
    }

    #[test]
    fn test_sort_direction_toggle() {
        // Test ascending to descending
        let current = SortDirection::Ascending;
        let next = match current {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        };
        assert_eq!(next, SortDirection::Descending);

        // Test descending to ascending
        let current = SortDirection::Descending;
        let next = match current {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        };
        assert_eq!(next, SortDirection::Ascending);
    }

    #[test]
    fn test_sorting_state_equality() {
        let state1 = SortingState {
            sort_column_id: Some("name".to_string()),
            sort_direction: SortDirection::Ascending,
        };

        let state2 = SortingState {
            sort_column_id: Some("name".to_string()),
            sort_direction: SortDirection::Ascending,
        };

        let state3 = SortingState {
            sort_column_id: Some("value".to_string()),
            sort_direction: SortDirection::Ascending,
        };

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_selection_detail_equality() {
        let items = vec![TestItem {
            id: 1,
            name: "Test".to_string(),
            value: 10,
        }];

        let detail1 = TableSelectionDetail {
            selected_items: items.clone(),
        };

        let detail2 = TableSelectionDetail {
            selected_items: items,
        };

        assert_eq!(detail1, detail2);
    }

    #[test]
    fn test_empty_selection() {
        let detail: TableSelectionDetail<TestItem> = TableSelectionDetail {
            selected_items: vec![],
        };

        assert!(detail.selected_items.is_empty());
    }

    #[test]
    fn test_column_width_percentage() {
        let column = TableColumn::new("test", "Test", |item: &TestItem| html! { {item.id} })
            .with_width("20%");

        assert_eq!(column.width, Some("20%".to_string()));
    }

    #[test]
    fn test_column_width_pixels() {
        let column = TableColumn::new("test", "Test", |item: &TestItem| html! { {item.id} })
            .with_width("150px");

        assert_eq!(column.width, Some("150px".to_string()));
    }

    #[test]
    fn test_sorting_state_with_column() {
        let state = SortingState {
            sort_column_id: Some("name".to_string()),
            sort_direction: SortDirection::Descending,
        };

        assert!(state.sort_column_id.is_some());
        assert_eq!(state.sort_column_id.unwrap(), "name");
        assert_eq!(state.sort_direction, SortDirection::Descending);
    }

    #[test]
    fn test_sorting_state_without_column() {
        let state = SortingState {
            sort_column_id: None,
            sort_direction: SortDirection::Ascending,
        };

        assert!(state.sort_column_id.is_none());
    }
}
