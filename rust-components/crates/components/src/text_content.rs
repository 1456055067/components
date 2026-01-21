// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! TextContent component
//!
//! A wrapper component that applies consistent typography styles to rich HTML content.
//! Automatically styles common text elements (headings, paragraphs, lists, code, etc.)
//! without requiring custom CSS.

use crate::internal::{BaseComponentProps, ClassBuilder};
use yew::prelude::*;

/// Properties for the TextContent component
#[derive(Properties, PartialEq, Clone)]
pub struct TextContentProps {
    /// Base component properties (id, class, etc.)
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Rich HTML content with automatic typography styling
    ///
    /// The TextContent component will automatically apply consistent styling
    /// to child elements including:
    /// - Headings (h1-h6)
    /// - Paragraphs (p)
    /// - Lists (ul, ol, li)
    /// - Inline code and code blocks (code, pre)
    /// - Text formatting (strong, em, mark, etc.)
    /// - Links (a)
    /// - Horizontal rules (hr)
    /// - Blockquotes
    #[prop_or_default]
    pub children: Children,
}

/// TextContent component for rich text content with typography styles
///
/// A wrapper component that provides consistent typography styling for rich HTML content.
/// All child elements are automatically styled according to Cloudscape design guidelines
/// without requiring additional CSS classes.
///
/// # Styled Elements
///
/// The TextContent component automatically applies styles to:
///
/// - **Headings**: `<h1>` through `<h6>` with appropriate sizing and spacing
/// - **Paragraphs**: `<p>` with consistent line height and spacing
/// - **Lists**: `<ul>` and `<ol>` with proper indentation and markers
/// - **Code**: `<code>` for inline code and `<pre>` for code blocks
/// - **Text formatting**: `<strong>`, `<em>`, `<mark>`, `<small>`, etc.
/// - **Links**: `<a>` with appropriate colors and hover states
/// - **Other elements**: `<blockquote>`, `<hr>`, tables, and more
///
/// # Example
///
/// ```rust
/// use cloudscape_components::TextContent;
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     html! {
///         <TextContent>
///             <h1>{"Main Title"}</h1>
///             <p>{"This is a paragraph with "}<strong>{"bold"}</strong>{" and "}<em>{"italic"}</em>{" text."}</p>
///             <ul>
///                 <li>{"First item"}</li>
///                 <li>{"Second item"}</li>
///                 <li>{"Third item"}</li>
///             </ul>
///             <p>{"Inline code: "}<code>{"const x = 42;"}</code></p>
///         </TextContent>
///     }
/// }
/// ```
///
/// # With Code Block
///
/// ```rust
/// use cloudscape_components::TextContent;
/// use yew::prelude::*;
///
/// #[function_component(CodeExample)]
/// fn code_example() -> Html {
///     html! {
///         <TextContent>
///             <h2>{"Function Example"}</h2>
///             <p>{"Here's how to use the function:"}</p>
///             <pre><code>
/// {"fn add(a: i32, b: i32) -> i32 {\n"}
/// {"    a + b\n"}
/// {"}"}
///             </code></pre>
///             <p>{"This function adds two numbers together."}</p>
///         </TextContent>
///     }
/// }
/// ```
///
/// # With Multiple Sections
///
/// ```rust
/// use cloudscape_components::TextContent;
/// use yew::prelude::*;
///
/// #[function_component(DocumentationPage)]
/// fn documentation_page() -> Html {
///     html! {
///         <TextContent>
///             <h1>{"User Guide"}</h1>
///
///             <h2>{"Getting Started"}</h2>
///             <p>{"Welcome to our application. This guide will help you get started."}</p>
///
///             <h2>{"Features"}</h2>
///             <ul>
///                 <li>{"Easy to use interface"}</li>
///                 <li>{"Real-time updates"}</li>
///                 <li>{"Secure data storage"}</li>
///             </ul>
///
///             <h2>{"Support"}</h2>
///             <p>{"For help, contact us at "}<a href="mailto:support@example.com">{"support@example.com"}</a>{"."}</p>
///         </TextContent>
///     }
/// }
/// ```
#[function_component(TextContent)]
pub fn text_content(props: &TextContentProps) -> Html {
    let class = ClassBuilder::new().add("awsui-text-content").build();

    let class = props.base.merge_classes(&class);

    html! {
        <div
            id={props.base.id.clone()}
            class={class}
        >
            { props.children.clone() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_content_props_default() {
        let props = TextContentProps {
            base: BaseComponentProps::default(),
            children: Children::default(),
        };

        assert!(props.base.id.is_none());
        assert!(props.base.class.is_none());
        assert!(props.children.is_empty());
    }

    #[test]
    fn test_text_content_with_base_props() {
        let base = BaseComponentProps {
            id: Some("test-id".to_string()),
            class: Some("custom-class".to_string()),
            data_attributes: None,
        };

        let props = TextContentProps {
            base: base.clone(),
            children: Children::default(),
        };

        assert_eq!(props.base.id, Some("test-id".to_string()));
        assert_eq!(props.base.class, Some("custom-class".to_string()));
    }

    #[test]
    fn test_text_content_class_builder() {
        let class = ClassBuilder::new().add("awsui-text-content").build();

        assert_eq!(class, "awsui-text-content");
    }

    #[test]
    fn test_text_content_merged_classes() {
        let base = BaseComponentProps {
            id: None,
            class: Some("user-class".to_string()),
            data_attributes: None,
        };

        let component_class = ClassBuilder::new().add("awsui-text-content").build();

        let merged = base.merge_classes(&component_class);

        assert_eq!(merged, "awsui-text-content user-class");
    }

    #[test]
    fn test_text_content_no_user_class() {
        let base = BaseComponentProps::default();
        let component_class = ClassBuilder::new().add("awsui-text-content").build();

        let merged = base.merge_classes(&component_class);

        assert_eq!(merged, "awsui-text-content");
    }

    #[test]
    fn test_text_content_with_children() {
        let children = html! {
            <>
                <h1>{"Title"}</h1>
                <p>{"Paragraph"}</p>
            </>
        };

        let props = TextContentProps {
            base: BaseComponentProps::default(),
            children: Children::new(vec![children]),
        };

        assert!(!props.children.is_empty());
    }
}
