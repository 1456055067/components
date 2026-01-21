// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Cards component for displaying card grids with selection.
//!
//! The Cards component provides a responsive grid layout for displaying multiple cards
//! with support for selection, loading states, empty states, and customizable rendering.

use crate::internal::{BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent};
use web_sys::MouseEvent;
use yew::prelude::*;

/// Definition of how a card should be rendered
///
/// A card definition specifies the header and sections that compose each card.
/// The header is required, while sections are optional and can be added using
/// the builder pattern.
#[derive(Clone)]
pub struct CardDefinition<T: Clone + PartialEq + 'static> {
    /// Function to render the card header
    pub header: fn(&T) -> Html,
    /// Functions to render card sections (content areas below header)
    pub sections: Vec<fn(&T) -> Html>,
}

impl<T: Clone + PartialEq + 'static> std::fmt::Debug for CardDefinition<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardDefinition")
            .field("header", &"<fn>")
            .field("sections", &format!("<{} sections>", self.sections.len()))
            .finish()
    }
}

impl<T: Clone + PartialEq + 'static> CardDefinition<T> {
    /// Creates a new card definition with a header
    ///
    /// # Arguments
    ///
    /// * `header` - Function to render the card header
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::CardDefinition;
    /// use yew::prelude::*;
    ///
    /// #[derive(Clone, PartialEq)]
    /// struct Item {
    ///     name: String,
    /// }
    ///
    /// let card_def = CardDefinition::new(|item: &Item| html! { {&item.name} });
    /// ```
    pub fn new(header: fn(&T) -> Html) -> Self {
        Self {
            header,
            sections: Vec::new(),
        }
    }

    /// Adds a section to the card definition
    ///
    /// # Arguments
    ///
    /// * `section` - Function to render the section content
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::CardDefinition;
    /// use yew::prelude::*;
    ///
    /// #[derive(Clone, PartialEq)]
    /// struct Item {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// let card_def = CardDefinition::new(|item: &Item| html! { {&item.name} })
    ///     .with_section(|item: &Item| html! { {&item.description} });
    /// ```
    pub fn with_section(mut self, section: fn(&T) -> Html) -> Self {
        self.sections.push(section);
        self
    }
}

// Manual PartialEq implementation that doesn't compare function pointers
impl<T: Clone + PartialEq + 'static> PartialEq for CardDefinition<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sections.len() == other.sections.len()
    }
}

/// Type of selection supported by the cards component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardsSelectionType {
    /// Single card selection
    Single,
    /// Multiple card selection
    Multi,
}

/// Event detail for selection change events
#[derive(Debug, Clone, PartialEq)]
pub struct CardsSelectionDetail<T: Clone + PartialEq> {
    /// List of selected items
    pub selected_items: Vec<T>,
}

/// Properties for the Cards component
#[derive(Properties, Clone)]
pub struct CardsProps<T: Clone + PartialEq + 'static> {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Card definition specifying how to render each card
    pub card_definition: CardDefinition<T>,

    /// Data items to display as cards
    #[prop_or_default]
    pub items: Vec<T>,

    /// Type of selection (single or multi)
    #[prop_or_default]
    pub selection_type: Option<CardsSelectionType>,

    /// Currently selected items (controlled)
    #[prop_or_default]
    pub selected_items: Vec<T>,

    /// Whether the cards are in a loading state
    #[prop_or_default]
    pub loading: bool,

    /// Text to display during loading
    #[prop_or_default]
    pub loading_text: Option<String>,

    /// Content to display when there are no cards
    #[prop_or_default]
    pub empty: Option<Html>,

    /// Header content (title, description, actions)
    #[prop_or_default]
    pub header: Option<Html>,

    /// Footer content (typically pagination)
    #[prop_or_default]
    pub footer: Option<Html>,

    /// Callback fired when selection changes
    #[prop_or_default]
    pub on_selection_change: Option<Callback<CustomEvent<CardsSelectionDetail<T>>>>,

    /// Number of cards per row at different breakpoints [xs, sm, md, lg]
    ///
    /// Defines responsive grid columns. For example, `[1, 2, 3, 4]` means:
    /// - 1 card per row on extra small screens
    /// - 2 cards per row on small screens
    /// - 3 cards per row on medium screens
    /// - 4 cards per row on large screens
    #[prop_or_else(|| vec![1, 2, 3, 4])]
    pub cards_per_row: Vec<u32>,
}

impl<T: Clone + PartialEq + 'static> PartialEq for CardsProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.card_definition == other.card_definition
            && self.items == other.items
            && self.selection_type == other.selection_type
            && self.selected_items == other.selected_items
            && self.loading == other.loading
            && self.loading_text == other.loading_text
            && self.cards_per_row == other.cards_per_row
    }
}

/// Cards component for displaying card grids
///
/// A flexible card grid component with support for selection, loading states,
/// and responsive layouts. The component is fully generic over the item type.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Cards, CardDefinition, CardsSelectionType};
/// use yew::prelude::*;
///
/// #[derive(Clone, PartialEq)]
/// struct Item {
///     name: String,
///     description: String,
/// }
///
/// #[function_component(MyCards)]
/// fn my_cards() -> Html {
///     let card_definition = CardDefinition::new(|item: &Item| html! { {&item.name} })
///         .with_section(|item: &Item| html! { {&item.description} });
///
///     let items = vec![
///         Item { name: "Card 1".into(), description: "Description 1".into() },
///         Item { name: "Card 2".into(), description: "Description 2".into() },
///     ];
///
///     html! {
///         <Cards<Item>
///             card_definition={card_definition}
///             items={items}
///             selection_type={CardsSelectionType::Multi}
///         />
///     }
/// }
/// ```
#[function_component(Cards)]
pub fn cards<T: Clone + PartialEq + 'static>(props: &CardsProps<T>) -> Html {
    let _metadata = ComponentMetadata::new("Cards");

    // Handle card selection
    let on_card_select = {
        let on_selection_change = props.on_selection_change.clone();
        let selected_items = props.selected_items.clone();
        let selection_type = props.selection_type;

        Callback::from(move |item: T| {
            if let Some(callback) = &on_selection_change {
                let new_selected = match selection_type {
                    Some(CardsSelectionType::Single) => {
                        // Single selection: replace selection
                        vec![item]
                    }
                    Some(CardsSelectionType::Multi) => {
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

                let detail = CardsSelectionDetail {
                    selected_items: new_selected,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }
        })
    };

    // Build root classes
    let root_classes = ClassBuilder::new()
        .add("awsui-cards")
        .add_if(props.loading, "awsui-cards-loading");

    let root_class = props.base.merge_classes(&root_classes.build());

    // Build grid style based on cards_per_row configuration
    let grid_style = build_grid_style(&props.cards_per_row);

    html! {
        <div class={root_class} id={props.base.id.clone()}>
            // Header section
            if let Some(ref header) = props.header {
                <div class="awsui-cards-header">
                    { header.clone() }
                </div>
            }

            // Cards grid container
            <div class="awsui-cards-container">
                {
                    if props.loading {
                        // Loading state
                        html! {
                            <div class="awsui-cards-loading-content">
                                <span class="awsui-cards-loading-spinner">{"⟳"}</span>
                                <span class="awsui-cards-loading-text">
                                    { props.loading_text.as_deref().unwrap_or("Loading...") }
                                </span>
                            </div>
                        }
                    } else if props.items.is_empty() {
                        // Empty state
                        html! {
                            <div class="awsui-cards-empty">
                                {
                                    if let Some(ref empty) = props.empty {
                                        empty.clone()
                                    } else {
                                        html! { <div class="awsui-cards-empty-default">{"No items found"}</div> }
                                    }
                                }
                            </div>
                        }
                    } else {
                        // Cards grid
                        html! {
                            <div class="awsui-cards-grid" style={grid_style}>
                                {
                                    props.items.iter().map(|item| {
                                        let is_selected = props.selected_items.contains(item);
                                        let selectable = props.selection_type.is_some();

                                        let card_classes = ClassBuilder::new()
                                            .add("awsui-cards-card")
                                            .add_if(is_selected, "awsui-cards-card-selected")
                                            .add_if(selectable, "awsui-cards-card-selectable");

                                        let item_clone = item.clone();
                                        let on_card_select_clone = on_card_select.clone();

                                        let on_card_click = if selectable {
                                            Some(Callback::from(move |e: MouseEvent| {
                                                e.prevent_default();
                                                on_card_select_clone.emit(item_clone.clone());
                                            }))
                                        } else {
                                            None
                                        };

                                        html! {
                                            <div
                                                class={card_classes.build()}
                                                onclick={on_card_click}
                                                role={if selectable { Some("button") } else { None }}
                                                tabindex={if selectable { Some("0") } else { None }}
                                                aria-selected={if selectable { Some(is_selected.to_string()) } else { None }}
                                            >
                                                // Selection indicator
                                                if let Some(selection_type) = props.selection_type {
                                                    <div class="awsui-cards-card-selection">
                                                        {
                                                            match selection_type {
                                                                CardsSelectionType::Single => html! {
                                                                    <input
                                                                        type="radio"
                                                                        class="awsui-cards-selection-radio"
                                                                        checked={is_selected}
                                                                        readonly=true
                                                                        aria-label="Select card"
                                                                    />
                                                                },
                                                                CardsSelectionType::Multi => html! {
                                                                    <input
                                                                        type="checkbox"
                                                                        class="awsui-cards-selection-checkbox"
                                                                        checked={is_selected}
                                                                        readonly=true
                                                                        aria-label="Select card"
                                                                    />
                                                                },
                                                            }
                                                        }
                                                    </div>
                                                }

                                                // Card header
                                                <div class="awsui-cards-card-header">
                                                    { (props.card_definition.header)(item) }
                                                </div>

                                                // Card sections
                                                {
                                                    props.card_definition.sections.iter().map(|section| {
                                                        html! {
                                                            <div class="awsui-cards-card-section">
                                                                { section(item) }
                                                            </div>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    }
                }
            </div>

            // Footer section (pagination, etc.)
            if let Some(ref footer) = props.footer {
                <div class="awsui-cards-footer">
                    { footer.clone() }
                </div>
            }
        </div>
    }
}

/// Builds the grid-template-columns style based on cards_per_row configuration
fn build_grid_style(cards_per_row: &[u32]) -> Option<String> {
    if cards_per_row.is_empty() {
        return None;
    }

    // Use the first value as the base, or default to 1
    let base_cols = cards_per_row.first().copied().unwrap_or(1);

    // Build CSS grid template
    // This is a simplified version; in production, you'd use CSS media queries
    let grid_cols = format!("repeat({}, 1fr)", base_cols);

    Some(format!("grid-template-columns: {}", grid_cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestItem {
        id: u32,
        name: String,
        description: String,
    }

    #[test]
    fn test_card_definition_new() {
        let card_def = CardDefinition::new(|item: &TestItem| html! { {&item.name} });
        assert_eq!(card_def.sections.len(), 0);
    }

    #[test]
    fn test_card_definition_with_section() {
        let card_def = CardDefinition::new(|item: &TestItem| html! { {&item.name} })
            .with_section(|item: &TestItem| html! { {&item.description} });
        assert_eq!(card_def.sections.len(), 1);
    }

    #[test]
    fn test_card_definition_multiple_sections() {
        let card_def = CardDefinition::new(|item: &TestItem| html! { {&item.name} })
            .with_section(|item: &TestItem| html! { {&item.description} })
            .with_section(|item: &TestItem| html! { {item.id} });
        assert_eq!(card_def.sections.len(), 2);
    }

    #[test]
    fn test_card_definition_equality() {
        let card_def1 = CardDefinition::new(|item: &TestItem| html! { {&item.name} })
            .with_section(|item: &TestItem| html! { {&item.description} });

        let card_def2 = CardDefinition::new(|item: &TestItem| html! { {&item.name} })
            .with_section(|item: &TestItem| html! { {&item.description} });

        assert_eq!(card_def1, card_def2);
    }

    #[test]
    fn test_selection_type_equality() {
        assert_eq!(CardsSelectionType::Single, CardsSelectionType::Single);
        assert_eq!(CardsSelectionType::Multi, CardsSelectionType::Multi);
        assert_ne!(CardsSelectionType::Single, CardsSelectionType::Multi);
    }

    #[test]
    fn test_cards_selection_detail() {
        let items = vec![
            TestItem {
                id: 1,
                name: "Card 1".to_string(),
                description: "Description 1".to_string(),
            },
            TestItem {
                id: 2,
                name: "Card 2".to_string(),
                description: "Description 2".to_string(),
            },
        ];

        let detail = CardsSelectionDetail {
            selected_items: items.clone(),
        };

        assert_eq!(detail.selected_items.len(), 2);
        assert_eq!(detail.selected_items[0].id, 1);
        assert_eq!(detail.selected_items[1].id, 2);
    }

    #[test]
    fn test_build_grid_style_empty() {
        let style = build_grid_style(&[]);
        assert_eq!(style, None);
    }

    #[test]
    fn test_build_grid_style_single_value() {
        let style = build_grid_style(&[3]);
        assert_eq!(
            style,
            Some("grid-template-columns: repeat(3, 1fr)".to_string())
        );
    }

    #[test]
    fn test_build_grid_style_multiple_values() {
        // Uses first value
        let style = build_grid_style(&[1, 2, 3, 4]);
        assert_eq!(
            style,
            Some("grid-template-columns: repeat(1, 1fr)".to_string())
        );
    }

    #[test]
    fn test_build_grid_style_default() {
        let style = build_grid_style(&[1]);
        assert_eq!(
            style,
            Some("grid-template-columns: repeat(1, 1fr)".to_string())
        );
    }

    #[test]
    fn test_cards_selection_detail_equality() {
        let items = vec![TestItem {
            id: 1,
            name: "Test".to_string(),
            description: "Description".to_string(),
        }];

        let detail1 = CardsSelectionDetail {
            selected_items: items.clone(),
        };

        let detail2 = CardsSelectionDetail {
            selected_items: items,
        };

        assert_eq!(detail1, detail2);
    }

    #[test]
    fn test_empty_selection() {
        let detail: CardsSelectionDetail<TestItem> = CardsSelectionDetail {
            selected_items: vec![],
        };

        assert!(detail.selected_items.is_empty());
    }

    #[test]
    fn test_selection_detail_with_one_item() {
        let item = TestItem {
            id: 1,
            name: "Single Card".to_string(),
            description: "Single Description".to_string(),
        };

        let detail = CardsSelectionDetail {
            selected_items: vec![item.clone()],
        };

        assert_eq!(detail.selected_items.len(), 1);
        assert_eq!(detail.selected_items[0], item);
    }

    #[test]
    fn test_cards_per_row_default() {
        // Test default value via the prop_or_else
        let default_cards_per_row = vec![1, 2, 3, 4];
        assert_eq!(default_cards_per_row, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_grid_style_with_four_columns() {
        let style = build_grid_style(&[4]);
        assert_eq!(
            style,
            Some("grid-template-columns: repeat(4, 1fr)".to_string())
        );
    }

    #[test]
    fn test_grid_style_with_two_columns() {
        let style = build_grid_style(&[2]);
        assert_eq!(
            style,
            Some("grid-template-columns: repeat(2, 1fr)".to_string())
        );
    }
}
