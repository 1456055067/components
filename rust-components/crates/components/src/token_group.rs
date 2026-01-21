// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! TokenGroup component for displaying dismissible tokens/tags.
//!
//! A component for displaying a collection of tokens (tags) with optional dismiss
//! functionality. Tokens can have icons, disabled states, and support both horizontal
//! and vertical alignment. Includes a "show more" feature to limit visible tokens.

use crate::internal::{BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent};
use web_sys::MouseEvent;
use yew::prelude::*;

/// A single token item
#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    /// Unique identifier for the token
    pub id: String,
    /// Display label for the token
    pub label: String,
    /// Whether the token can be dismissed
    pub dismissible: bool,
    /// Whether the token is disabled
    pub disabled: bool,
    /// Optional icon to display before the label
    pub icon: Option<Html>,
}

impl Token {
    /// Creates a new token with the given id and label
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::Token;
    ///
    /// let token = Token::new("tag-1", "Environment: Production");
    /// ```
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            dismissible: false,
            disabled: false,
            icon: None,
        }
    }

    /// Sets whether this token is dismissible
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::Token;
    ///
    /// let token = Token::new("tag-1", "Tag 1")
    ///     .with_dismissible(true);
    /// ```
    pub fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Sets whether this token is disabled
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::Token;
    ///
    /// let token = Token::new("tag-1", "Tag 1")
    ///     .with_disabled(true);
    /// ```
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the icon for this token
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use cloudscape_components::{Token, Icon};
    /// use yew::prelude::*;
    ///
    /// let token = Token::new("tag-1", "Tag 1")
    ///     .with_icon(html! { <Icon name="check" /> });
    /// ```
    pub fn with_icon(mut self, icon: Html) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Alignment options for the token group
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TokenGroupAlignment {
    /// Tokens are displayed in a horizontal row
    #[default]
    Horizontal,
    /// Tokens are displayed in a vertical column
    Vertical,
}

impl TokenGroupAlignment {
    fn as_str(&self) -> &'static str {
        match self {
            TokenGroupAlignment::Horizontal => "horizontal",
            TokenGroupAlignment::Vertical => "vertical",
        }
    }
}

/// Event detail for token dismiss events
#[derive(Clone, PartialEq, Debug)]
pub struct TokenDismissDetail {
    /// The ID of the dismissed token
    pub item_id: String,
}

/// Properties for the TokenGroup component
#[derive(Properties, PartialEq, Clone)]
pub struct TokenGroupProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// The list of tokens to display
    #[prop_or_default]
    pub items: Vec<Token>,

    /// Alignment of tokens (horizontal or vertical)
    ///
    /// Defaults to Horizontal.
    #[prop_or_default]
    pub alignment: TokenGroupAlignment,

    /// Maximum number of tokens to display before showing "show more"
    ///
    /// If not specified, all tokens are displayed.
    #[prop_or_default]
    pub limit: Option<u32>,

    /// Callback fired when a token is dismissed
    ///
    /// The event detail contains the ID of the dismissed token.
    #[prop_or_default]
    pub on_dismiss: Option<Callback<CustomEvent<TokenDismissDetail>>>,
}

/// TokenGroup component for displaying dismissible tokens/tags.
///
/// A controlled component that displays a collection of tokens (tags) with
/// optional dismiss functionality. Each token can have an icon, be disabled,
/// and be dismissible. The component supports both horizontal and vertical
/// alignment and can limit the number of visible tokens with a "show more" feature.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{TokenGroup, Token, TokenDismissDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let items = vec![
///         Token::new("1", "Tag 1").with_dismissible(true),
///         Token::new("2", "Tag 2").with_dismissible(true),
///         Token::new("3", "Tag 3"),
///     ];
///
///     let on_dismiss = Callback::from(move |event: CustomEvent<TokenDismissDetail>| {
///         // Handle token dismissal
///         web_sys::console::log_1(&format!("Token dismissed: {}", event.detail.item_id).into());
///     });
///
///     html! {
///         <TokenGroup
///             items={items}
///             on_dismiss={on_dismiss}
///         />
///     }
/// }
/// ```
///
/// # With Limit and Vertical Alignment
///
/// ```rust
/// use cloudscape_components::{TokenGroup, Token, TokenGroupAlignment};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let items = vec![
///         Token::new("1", "Tag 1").with_dismissible(true),
///         Token::new("2", "Tag 2").with_dismissible(true),
///         Token::new("3", "Tag 3").with_dismissible(true),
///         Token::new("4", "Tag 4").with_dismissible(true),
///         Token::new("5", "Tag 5").with_dismissible(true),
///     ];
///
///     html! {
///         <TokenGroup
///             items={items}
///             alignment={TokenGroupAlignment::Vertical}
///             limit={3}
///         />
///     }
/// }
/// ```
#[function_component(TokenGroup)]
pub fn token_group(props: &TokenGroupProps) -> Html {
    let _metadata = ComponentMetadata::new("TokenGroup");
    let show_all = use_state(|| false);

    // Determine which tokens to show based on limit
    let visible_items = use_memo(
        (props.items.clone(), props.limit, *show_all),
        |(items, limit, show_all)| {
            if *show_all || limit.is_none() {
                items.clone()
            } else {
                let limit_val = limit.unwrap() as usize;
                items.iter().take(limit_val).cloned().collect::<Vec<_>>()
            }
        },
    );

    let remaining_count = if let Some(limit) = props.limit {
        if !*show_all && props.items.len() > limit as usize {
            props.items.len() - limit as usize
        } else {
            0
        }
    } else {
        0
    };

    // Toggle show more/less
    let on_toggle_show = {
        let show_all = show_all.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            show_all.set(!*show_all);
        })
    };

    // Handle token dismiss
    let on_token_dismiss = {
        let on_dismiss = props.on_dismiss.clone();

        Callback::from(move |item_id: String| {
            if let Some(callback) = &on_dismiss {
                callback.emit(CustomEvent::new_non_cancelable(TokenDismissDetail {
                    item_id,
                }));
            }
        })
    };

    // Build root classes
    let root_classes = ClassBuilder::new()
        .add("awsui-token-group")
        .add(format!(
            "awsui-token-group-alignment-{}",
            props.alignment.as_str()
        ));

    let class = props.base.merge_classes(&root_classes.build());

    html! {
        <div
            id={props.base.id.clone()}
            class={class}
        >
            <div class="awsui-token-group-items">
                {
                    visible_items.iter().map(|token| {
                        let token_id = token.id.clone();
                        let token_classes = ClassBuilder::new()
                            .add("awsui-token")
                            .add_if(token.dismissible, "awsui-token-dismissible")
                            .add_if(token.disabled, "awsui-token-disabled");

                        let on_dismiss = {
                            let on_token_dismiss = on_token_dismiss.clone();
                            let token_disabled = token.disabled;
                            Callback::from(move |e: MouseEvent| {
                                if !token_disabled {
                                    e.prevent_default();
                                    e.stop_propagation();
                                    on_token_dismiss.emit(token_id.clone());
                                }
                            })
                        };

                        html! {
                            <div key={token.id.clone()} class={token_classes.build()}>
                                if let Some(ref icon) = token.icon {
                                    <span class="awsui-token-icon">
                                        { icon.clone() }
                                    </span>
                                }
                                <span class="awsui-token-label">
                                    { &token.label }
                                </span>
                                if token.dismissible {
                                    <button
                                        type="button"
                                        class="awsui-token-dismiss-button"
                                        aria-label={format!("Remove {}", token.label)}
                                        onclick={on_dismiss}
                                        disabled={token.disabled}
                                    >
                                        { "×" }
                                    </button>
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }

                // Show more/less button
                if remaining_count > 0 || *show_all && props.limit.is_some() {
                    <button
                        type="button"
                        class="awsui-token-group-toggle"
                        onclick={on_toggle_show}
                        aria-expanded={show_all.to_string()}
                    >
                        if *show_all {
                            { "Show less" }
                        } else {
                            { format!("Show {} more", remaining_count) }
                        }
                    </button>
                }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new() {
        let token = Token::new("test-id", "Test Label");
        assert_eq!(token.id, "test-id");
        assert_eq!(token.label, "Test Label");
        assert!(!token.dismissible);
        assert!(!token.disabled);
        assert!(token.icon.is_none());
    }

    #[test]
    fn test_token_builder_pattern() {
        let token = Token::new("id-1", "Label 1")
            .with_dismissible(true)
            .with_disabled(true);

        assert_eq!(token.id, "id-1");
        assert_eq!(token.label, "Label 1");
        assert!(token.dismissible);
        assert!(token.disabled);
    }

    #[test]
    fn test_token_with_icon() {
        let icon = html! { <span>{"✓"}</span> };
        let token = Token::new("id-1", "Label 1").with_icon(icon);

        assert!(token.icon.is_some());
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new("id-1", "Label 1");
        let token2 = Token::new("id-1", "Label 1");
        let token3 = Token::new("id-2", "Label 2");

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new("id-1", "Label 1")
            .with_dismissible(true)
            .with_disabled(true);
        let token2 = token1.clone();

        assert_eq!(token1, token2);
        assert_eq!(token1.id, token2.id);
        assert_eq!(token1.label, token2.label);
        assert_eq!(token1.dismissible, token2.dismissible);
        assert_eq!(token1.disabled, token2.disabled);
    }

    #[test]
    fn test_token_group_alignment_as_str() {
        assert_eq!(TokenGroupAlignment::Horizontal.as_str(), "horizontal");
        assert_eq!(TokenGroupAlignment::Vertical.as_str(), "vertical");
    }

    #[test]
    fn test_token_group_alignment_default() {
        let alignment = TokenGroupAlignment::default();
        assert_eq!(alignment, TokenGroupAlignment::Horizontal);
    }

    #[test]
    fn test_token_group_alignment_equality() {
        assert_eq!(
            TokenGroupAlignment::Horizontal,
            TokenGroupAlignment::Horizontal
        );
        assert_eq!(TokenGroupAlignment::Vertical, TokenGroupAlignment::Vertical);
        assert_ne!(
            TokenGroupAlignment::Horizontal,
            TokenGroupAlignment::Vertical
        );
    }

    #[test]
    fn test_token_dismiss_detail() {
        let detail = TokenDismissDetail {
            item_id: "test-id".to_string(),
        };

        assert_eq!(detail.item_id, "test-id");
    }

    #[test]
    fn test_token_dismiss_detail_clone() {
        let detail1 = TokenDismissDetail {
            item_id: "test-id".to_string(),
        };
        let detail2 = detail1.clone();

        assert_eq!(detail1, detail2);
        assert_eq!(detail1.item_id, detail2.item_id);
    }
}
