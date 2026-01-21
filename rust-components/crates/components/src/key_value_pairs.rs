// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! KeyValuePairs component for displaying key-value pairs in a structured format.
//!
//! The KeyValuePairs component provides a flexible way to display structured data as
//! key-value pairs with support for multiple columns and semantic HTML for accessibility.

use yew::prelude::*;
use crate::internal::{BaseComponentProps, ClassBuilder};

/// A single key-value pair item
///
/// Represents one key-value pair with an optional info element.
#[derive(Clone, PartialEq)]
pub struct KeyValuePair {
    /// The label/key for this pair
    pub label: String,

    /// The value content (can be any renderable HTML)
    pub value: Html,

    /// Optional info element (e.g., info link or icon)
    pub info: Option<Html>,
}

impl KeyValuePair {
    /// Creates a new KeyValuePair with a label and value
    ///
    /// # Arguments
    ///
    /// * `label` - The label text for the key
    /// * `value` - The HTML content for the value
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::KeyValuePair;
    /// use yew::prelude::*;
    ///
    /// let pair = KeyValuePair::new("Name", html! { "John Doe" });
    /// ```
    pub fn new(label: impl Into<String>, value: Html) -> Self {
        Self {
            label: label.into(),
            value,
            info: None,
        }
    }

    /// Adds an info element to the key-value pair
    ///
    /// The info element is typically used for info links or icons that provide
    /// additional context about the key.
    ///
    /// # Arguments
    ///
    /// * `info` - The HTML content for the info element
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::KeyValuePair;
    /// use yew::prelude::*;
    ///
    /// let pair = KeyValuePair::new("Name", html! { "John Doe" })
    ///     .with_info(html! { <a href="#">{"Info"}</a> });
    /// ```
    pub fn with_info(mut self, info: Html) -> Self {
        self.info = Some(info);
        self
    }
}

/// Properties for the KeyValuePairs component
#[derive(Properties, PartialEq, Clone)]
pub struct KeyValuePairsProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Array of key-value pairs to display
    pub items: Vec<KeyValuePair>,

    /// Number of columns to display (1-4)
    ///
    /// Valid values are 1-4. Values outside this range will be clamped.
    /// Defaults to 1 column.
    #[prop_or(1)]
    pub columns: u32,

    /// Provides an aria-label for the key-value pairs container
    ///
    /// Don't use both `aria_label` and `aria_labelledby` at the same time.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Sets the aria-labelledby property on the container
    ///
    /// If there's a visible label element that you can reference, use this
    /// instead of `aria_label`. Don't use both at the same time.
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
}

/// KeyValuePairs component for displaying key-value pairs in a structured format.
///
/// This component uses semantic HTML (dl, dt, dd elements) for accessibility and
/// supports displaying pairs in 1-4 columns with a responsive grid layout.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{KeyValuePairs, KeyValuePair};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let items = vec![
///         KeyValuePair::new("Name", html! { "John Doe" }),
///         KeyValuePair::new("Email", html! { "john@example.com" }),
///         KeyValuePair::new("Status", html! { "Active" }),
///     ];
///
///     html! {
///         <KeyValuePairs items={items} columns={2} />
///     }
/// }
/// ```
///
/// # With Info Links
///
/// ```rust
/// use cloudscape_components::{KeyValuePairs, KeyValuePair};
/// use yew::prelude::*;
///
/// #[function_component(InfoExample)]
/// fn info_example() -> Html {
///     let items = vec![
///         KeyValuePair::new("API Endpoint", html! { "https://api.example.com" })
///             .with_info(html! { <a href="#">{"Learn more"}</a> }),
///         KeyValuePair::new("Region", html! { "us-east-1" }),
///     ];
///
///     html! {
///         <KeyValuePairs
///             items={items}
///             columns={1}
///             aria_label="API Configuration"
///         />
///     }
/// }
/// ```
#[function_component(KeyValuePairs)]
pub fn key_value_pairs(props: &KeyValuePairsProps) -> Html {
    // Clamp columns to valid range (1-4)
    let columns = props.columns.clamp(1, 4);

    // Build CSS classes for the root element
    let root_classes = ClassBuilder::new()
        .add("awsui-key-value-pairs")
        .add(format!("awsui-key-value-pairs-columns-{}", columns));

    let root_class = props.base.merge_classes(&root_classes.build());

    // Build the pairs as dt/dd elements
    let pairs = props.items.iter().enumerate().map(|(index, pair)| {
        let pair_classes = ClassBuilder::new()
            .add("awsui-key-value-pair")
            .build();

        let label_classes = ClassBuilder::new()
            .add("awsui-key-value-pair-label")
            .build();

        let value_classes = ClassBuilder::new()
            .add("awsui-key-value-pair-value")
            .build();

        let info_element = pair.info.as_ref().map(|info| {
            let info_classes = ClassBuilder::new()
                .add("awsui-key-value-pair-info")
                .build();

            html! {
                <span class={info_classes}>
                    { info.clone() }
                </span>
            }
        });

        html! {
            <div key={index} class={pair_classes}>
                <dt class={label_classes}>
                    <div class="awsui-key-value-pair-label-text">
                        { &pair.label }
                    </div>
                    { info_element }
                </dt>
                <dd class={value_classes}>
                    { pair.value.clone() }
                </dd>
            </div>
        }
    });

    html! {
        <dl
            id={props.base.id.clone()}
            class={root_class}
            aria-label={props.aria_label.clone()}
            aria-labelledby={props.aria_labelledby.clone()}
        >
            { for pairs }
        </dl>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_value_pair_new() {
        let pair = KeyValuePair::new("Test Label", html! { "Test Value" });

        assert_eq!(pair.label, "Test Label");
        assert!(pair.info.is_none());
    }

    #[test]
    fn test_key_value_pair_with_info() {
        let pair = KeyValuePair::new("Label", html! { "Value" })
            .with_info(html! { <a>{"Info"}</a> });

        assert_eq!(pair.label, "Label");
        assert!(pair.info.is_some());
    }

    #[test]
    fn test_key_value_pair_builder_pattern() {
        let pair = KeyValuePair::new("Name", html! { "John" })
            .with_info(html! { <span>{"i"}</span> });

        assert_eq!(pair.label, "Name");
        assert!(pair.info.is_some());
    }

    #[test]
    fn test_columns_default() {
        let props = KeyValuePairsProps {
            base: BaseComponentProps::default(),
            items: vec![],
            columns: 1,
            aria_label: None,
            aria_labelledby: None,
        };

        assert_eq!(props.columns, 1);
    }

    #[test]
    fn test_columns_clamping_below_range() {
        // Test that columns below 1 would be clamped to 1
        let columns = 0u32.clamp(1, 4);
        assert_eq!(columns, 1);
    }

    #[test]
    fn test_columns_clamping_above_range() {
        // Test that columns above 4 would be clamped to 4
        let columns = 5u32.clamp(1, 4);
        assert_eq!(columns, 4);
    }

    #[test]
    fn test_columns_within_range() {
        // Test that valid columns remain unchanged
        for col in 1..=4 {
            let clamped = col.clamp(1, 4);
            assert_eq!(clamped, col);
        }
    }

    #[test]
    fn test_props_with_aria_label() {
        let props = KeyValuePairsProps {
            base: BaseComponentProps::default(),
            items: vec![],
            columns: 2,
            aria_label: Some("Test Label".to_string()),
            aria_labelledby: None,
        };

        assert_eq!(props.aria_label, Some("Test Label".to_string()));
        assert_eq!(props.aria_labelledby, None);
    }

    #[test]
    fn test_props_with_aria_labelledby() {
        let props = KeyValuePairsProps {
            base: BaseComponentProps::default(),
            items: vec![],
            columns: 3,
            aria_label: None,
            aria_labelledby: Some("label-id".to_string()),
        };

        assert_eq!(props.aria_label, None);
        assert_eq!(props.aria_labelledby, Some("label-id".to_string()));
    }

    #[test]
    fn test_multiple_items() {
        let items = vec![
            KeyValuePair::new("Key 1", html! { "Value 1" }),
            KeyValuePair::new("Key 2", html! { "Value 2" }),
            KeyValuePair::new("Key 3", html! { "Value 3" }),
        ];

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].label, "Key 1");
        assert_eq!(items[1].label, "Key 2");
        assert_eq!(items[2].label, "Key 3");
    }

    #[test]
    fn test_item_with_complex_value() {
        let complex_value = html! {
            <div>
                <span>{"Complex"}</span>
                <strong>{"Value"}</strong>
            </div>
        };

        let pair = KeyValuePair::new("Complex", complex_value);
        assert_eq!(pair.label, "Complex");
    }

    #[test]
    fn test_string_into_label() {
        let owned = String::from("Owned String");
        let pair = KeyValuePair::new(owned, html! { "Value" });
        assert_eq!(pair.label, "Owned String");

        let borrowed = "Borrowed String";
        let pair2 = KeyValuePair::new(borrowed, html! { "Value" });
        assert_eq!(pair2.label, "Borrowed String");
    }
}
