// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Pagination component
//!
//! Page navigation with next/previous and page numbers, enabling users to navigate
//! through large sets of data.

use yew::prelude::*;
use web_sys::MouseEvent;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ComponentStyles,
    AnalyticsMetadata, CustomEvent, ClassBuilder,
};

/// Event detail for pagination change events
#[derive(Debug, Clone, PartialEq)]
pub struct PaginationChangeDetail {
    /// The requested page index (1-indexed)
    pub requested_page_index: u32,
}

/// Event detail for page click events (next/previous)
#[derive(Debug, Clone, PartialEq)]
pub struct PaginationPageClickDetail {
    /// Whether the requested page is available
    pub requested_page_available: bool,
    /// The requested page index (1-indexed)
    pub requested_page_index: u32,
}

/// I18n strings for pagination component
#[derive(Clone, PartialEq)]
pub struct PaginationI18nStrings {
    /// Label for the pagination navigation
    pub pagination_label: Option<String>,
    /// Label for the previous page button
    pub previous_page_label: Option<String>,
    /// Label for the next page button
    pub next_page_label: Option<String>,
    /// Function to generate label for a specific page number
    pub page_label: Option<fn(u32) -> String>,
}

impl Default for PaginationI18nStrings {
    fn default() -> Self {
        Self {
            pagination_label: None,
            previous_page_label: Some("Previous page".to_string()),
            next_page_label: Some("Next page".to_string()),
            page_label: Some(|page_number| format!("Page {}", page_number)),
        }
    }
}

/// Properties for the Pagination component
#[derive(Properties, PartialEq, Clone)]
pub struct PaginationProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Index of the current page (1-indexed)
    ///
    /// The first page has an index of 1.
    #[prop_or(1)]
    pub current_page_index: u32,

    /// Total number of pages
    ///
    /// Only positive integers are allowed.
    #[prop_or_default]
    pub pages_count: u32,

    /// Whether the pagination is disabled
    ///
    /// Use this to prevent the user from changing pages before items are loaded.
    #[prop_or_default]
    pub disabled: bool,

    /// Sets the pagination variant to open-ended
    ///
    /// When true, always displays three dots before the next page icon.
    /// The next page button is never disabled. This enables lazy-loading scenarios
    /// where the total page count is unknown.
    #[prop_or_default]
    pub open_end: bool,

    /// I18n strings for ARIA labels and text
    #[prop_or_default]
    pub aria_labels: PaginationI18nStrings,

    /// Called when a user interaction causes a pagination change
    ///
    /// The event detail contains the new current_page_index.
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<PaginationChangeDetail>>>,

    /// Called when the previous page arrow is clicked
    ///
    /// The event detail contains:
    /// - requested_page_available: Always true
    /// - requested_page_index: The index of the requested page
    #[prop_or_default]
    pub on_previous_page_click: Option<Callback<CustomEvent<PaginationPageClickDetail>>>,

    /// Called when the next page arrow is clicked
    ///
    /// The event detail contains:
    /// - requested_page_available: false when open_end is true and at the last page
    /// - requested_page_index: The index of the requested page
    #[prop_or_default]
    pub on_next_page_click: Option<Callback<CustomEvent<PaginationPageClickDetail>>>,
}

/// Pagination component for page navigation
///
/// Enables users to navigate through large sets of data with page numbers,
/// previous/next buttons, and ellipsis for many pages.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Pagination, PaginationChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let current_page = use_state(|| 1);
///
///     let on_change = {
///         let current_page = current_page.clone();
///         Callback::from(move |event: CustomEvent<PaginationChangeDetail>| {
///             current_page.set(event.detail.requested_page_index);
///         })
///     };
///
///     html! {
///         <Pagination
///             current_page_index={*current_page}
///             pages_count={10}
///             on_change={on_change}
///         />
///     }
/// }
/// ```
///
/// # Open-ended Example
///
/// ```rust
/// use cloudscape_components::{Pagination, PaginationChangeDetail, PaginationPageClickDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(LazyLoadedList)]
/// fn lazy_loaded_list() -> Html {
///     let current_page = use_state(|| 1);
///     let pages_count = use_state(|| 5);
///
///     let on_change = {
///         let current_page = current_page.clone();
///         Callback::from(move |event: CustomEvent<PaginationChangeDetail>| {
///             current_page.set(event.detail.requested_page_index);
///         })
///     };
///
///     let on_next_page_click = {
///         let pages_count = pages_count.clone();
///         let current_page = current_page.clone();
///         Callback::from(move |event: CustomEvent<PaginationPageClickDetail>| {
///             if !event.detail.requested_page_available {
///                 // Load more pages
///                 pages_count.set(*pages_count + 5);
///             }
///         })
///     };
///
///     html! {
///         <Pagination
///             current_page_index={*current_page}
///             pages_count={*pages_count}
///             open_end={true}
///             on_change={on_change}
///             on_next_page_click={on_next_page_click}
///         />
///     }
/// }
/// ```
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let _metadata = ComponentMetadata::new("Pagination");

    // Get pagination state (which pages to show, where to show dots)
    let pagination_state = get_pagination_state(
        props.current_page_index,
        props.pages_count,
        props.open_end,
    );

    // Get i18n strings with defaults
    let previous_page_label = props.aria_labels.previous_page_label
        .as_ref()
        .cloned()
        .unwrap_or_else(|| "Previous page".to_string());

    let next_page_label = props.aria_labels.next_page_label
        .as_ref()
        .cloned()
        .unwrap_or_else(|| "Next page".to_string());

    let page_label_fn = props.aria_labels.page_label
        .unwrap_or(|page_number| format!("Page {}", page_number));

    // Create handlers
    let handle_page_click = {
        let on_change = props.on_change.clone();
        Callback::from(move |requested_page_index: u32| {
            if let Some(ref callback) = on_change {
                let detail = PaginationChangeDetail {
                    requested_page_index,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }
        })
    };

    let handle_prev_page_click = {
        let on_change = props.on_change.clone();
        let on_previous_page_click = props.on_previous_page_click.clone();
        let current_page_index = props.current_page_index;

        Callback::from(move |_e: MouseEvent| {
            let requested_page_index = current_page_index.saturating_sub(1);

            // Fire onChange
            if let Some(ref callback) = on_change {
                let detail = PaginationChangeDetail {
                    requested_page_index,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }

            // Fire onPreviousPageClick
            if let Some(ref callback) = on_previous_page_click {
                let detail = PaginationPageClickDetail {
                    requested_page_available: true,
                    requested_page_index,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }
        })
    };

    let handle_next_page_click = {
        let on_change = props.on_change.clone();
        let on_next_page_click = props.on_next_page_click.clone();
        let current_page_index = props.current_page_index;
        let pages_count = props.pages_count;

        Callback::from(move |_e: MouseEvent| {
            let requested_page_index = current_page_index + 1;
            let requested_page_available = current_page_index < pages_count;

            // Fire onChange
            if let Some(ref callback) = on_change {
                let detail = PaginationChangeDetail {
                    requested_page_index,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }

            // Fire onNextPageClick
            if let Some(ref callback) = on_next_page_click {
                let detail = PaginationPageClickDetail {
                    requested_page_available,
                    requested_page_index,
                };
                callback.emit(CustomEvent::new_non_cancelable(detail));
            }
        })
    };

    // Determine button states
    let previous_button_disabled = props.disabled || props.current_page_index == 1;
    let next_button_disabled = props.disabled ||
        (!props.open_end && (props.pages_count == 0 || props.current_page_index == props.pages_count));

    // Build component styles
    let mut styles = ComponentStyles::new();
    styles.add_class("awsui-pagination");
    if props.disabled {
        styles.add_class("awsui-pagination-disabled");
    }

    let class = props.base.merge_classes(&styles.class_attr());
    let style_attr = styles.style_attr();

    // Create analytics metadata
    let analytics = AnalyticsMetadata {
        component: Some(crate::internal::analytics::ComponentAnalytics {
            name: "awsui.Pagination".to_string(),
            label: crate::internal::analytics::LabelIdentifier::Root,
            properties: Some(serde_json::json!({
                "openEnd": props.open_end.to_string(),
                "pagesCount": props.pages_count.to_string(),
                "currentPageIndex": props.current_page_index.to_string(),
            })),
        }),
        ..Default::default()
    };
    let analytics_attr = analytics.to_data_attribute();

    html! {
        <ul
            id={props.base.id.clone()}
            class={class}
            style={style_attr}
            aria-label={props.aria_labels.pagination_label.clone()}
            data-analytics-metadata={analytics_attr}
        >
            // Previous button
            <PageButton
                class_name="awsui-pagination-arrow"
                aria_label={previous_page_label}
                disabled={previous_button_disabled}
                on_click={handle_prev_page_click}
                position="prev"
            >
                <span class="awsui-pagination-icon-prev">{"◀"}</span>
            </PageButton>

            // First page
            <PageNumber
                page_index={1}
                is_current={props.current_page_index == 1}
                disabled={props.disabled}
                aria_label={page_label_fn(1)}
                on_click={handle_page_click.clone()}
            />

            // Left dots
            if pagination_state.left_dots {
                <li class="awsui-pagination-page-item">
                    <span class="awsui-pagination-dots">{"..."}</span>
                </li>
            }

            // Middle pages
            {
                range(pagination_state.left_index, pagination_state.right_index)
                    .into_iter()
                    .map(|page_index| {
                        html! {
                            <PageNumber
                                key={page_index}
                                page_index={page_index}
                                is_current={props.current_page_index == page_index}
                                disabled={props.disabled}
                                aria_label={page_label_fn(page_index)}
                                on_click={handle_page_click.clone()}
                            />
                        }
                    })
                    .collect::<Html>()
            }

            // Right dots
            if pagination_state.right_dots {
                <li class="awsui-pagination-page-item">
                    <span class="awsui-pagination-dots">{"..."}</span>
                </li>
            }

            // Last page (not shown in open-end mode)
            if !props.open_end && props.pages_count > 1 {
                <PageNumber
                    page_index={props.pages_count}
                    is_current={props.current_page_index == props.pages_count}
                    disabled={props.disabled}
                    aria_label={page_label_fn(props.pages_count)}
                    on_click={handle_page_click.clone()}
                />
            }

            // Next button
            <PageButton
                class_name="awsui-pagination-arrow"
                aria_label={next_page_label}
                disabled={next_button_disabled}
                on_click={handle_next_page_click}
                position="next"
            >
                <span class="awsui-pagination-icon-next">{"▶"}</span>
            </PageButton>
        </ul>
    }
}

/// Properties for page button component
#[derive(Properties, PartialEq, Clone)]
struct PageButtonProps {
    #[prop_or_default]
    class_name: String,
    aria_label: String,
    #[prop_or_default]
    disabled: bool,
    on_click: Callback<MouseEvent>,
    #[prop_or_default]
    position: String,
    children: Children,
}

/// Internal component for previous/next buttons
#[function_component(PageButton)]
fn page_button(props: &PageButtonProps) -> Html {
    let on_click = {
        let disabled = props.disabled;
        let callback = props.on_click.clone();

        Callback::from(move |e: MouseEvent| {
            if !disabled {
                e.prevent_default();
                callback.emit(e);
            }
        })
    };

    let button_classes = ClassBuilder::new()
        .add(&props.class_name)
        .add("awsui-pagination-button")
        .add_if(props.disabled, "awsui-pagination-button-disabled");

    html! {
        <li class="awsui-pagination-page-item">
            <button
                type="button"
                class={button_classes.build()}
                aria-label={props.aria_label.clone()}
                disabled={props.disabled}
                onclick={on_click}
            >
                { props.children.clone() }
            </button>
        </li>
    }
}

/// Properties for page number component
#[derive(Properties, PartialEq, Clone)]
struct PageNumberProps {
    page_index: u32,
    #[prop_or_default]
    is_current: bool,
    #[prop_or_default]
    disabled: bool,
    aria_label: String,
    on_click: Callback<u32>,
}

/// Internal component for page number buttons
#[function_component(PageNumber)]
fn page_number(props: &PageNumberProps) -> Html {
    let on_click = {
        let disabled = props.disabled;
        let page_index = props.page_index;
        let callback = props.on_click.clone();

        Callback::from(move |e: MouseEvent| {
            if !disabled {
                e.prevent_default();
                callback.emit(page_index);
            }
        })
    };

    let button_classes = ClassBuilder::new()
        .add("awsui-pagination-button")
        .add("awsui-pagination-page-number")
        .add_if(props.disabled, "awsui-pagination-button-disabled")
        .add_if(props.is_current, "awsui-pagination-button-current");

    html! {
        <li class="awsui-pagination-page-item">
            <button
                type="button"
                class={button_classes.build()}
                aria-label={props.aria_label.clone()}
                aria-current={props.is_current.then_some("page")}
                disabled={props.disabled}
                onclick={on_click}
            >
                { props.page_index }
            </button>
        </li>
    }
}

/// State of pagination controls (which pages to show, where to show dots)
#[derive(Debug, Clone, PartialEq)]
struct PaginationState {
    left_dots: bool,
    right_dots: bool,
    left_index: u32,
    right_index: u32,
}

/// Calculate pagination state based on current page, total pages, and open-end mode
///
/// This determines which page numbers to display and where to show ellipsis.
/// The algorithm tries to show 7 controls total (current page +/- 3 neighbors).
fn get_pagination_state(current_page_index: u32, total_pages_count: u32, is_open_end: bool) -> PaginationState {
    // Total number of page number elements to display (excluding first, last, and arrows)
    const NUMBER_OF_CONTROLS: u32 = 7;

    // Max number of controls that can be displayed on left and right of current page
    let left_delta = NUMBER_OF_CONTROLS / 2;
    let mut right_delta = left_delta;

    // Upper and lower limits for page range
    let lower_limit = 2;
    let mut upper_limit = total_pages_count.saturating_sub(1);

    if is_open_end {
        right_delta += 1;
        upper_limit = total_pages_count + 1;
    }

    // Left and right indices based on delta calculation
    let mut left_index = current_page_index.saturating_sub(left_delta);
    let mut right_index = current_page_index + right_delta;

    // Adjust page indexes if page index is too small
    if left_index < lower_limit {
        right_index += lower_limit - left_index;
        left_index = lower_limit;
    }

    // Adjust page indexes if page index is too big
    if right_index > upper_limit {
        left_index = left_index.saturating_sub(right_index - upper_limit);
        right_index = upper_limit;
    }

    // Adjust indexes one more time to avoid out of range errors
    left_index = left_index.max(2);
    right_index = right_index.min(upper_limit);

    // Consider adding dots
    let left_dots = left_index > 2;
    let right_dots = is_open_end || right_index < upper_limit;

    if left_dots {
        left_index += 1;
    }

    if right_dots {
        right_index = right_index.saturating_sub(1);
    }

    PaginationState {
        left_dots,
        right_dots,
        left_index,
        right_index,
    }
}

/// Generate a range of page numbers
fn range(from: u32, to: u32) -> Vec<u32> {
    if from > to {
        vec![]
    } else {
        (from..=to).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_change_detail_equality() {
        let detail1 = PaginationChangeDetail {
            requested_page_index: 5,
        };

        let detail2 = PaginationChangeDetail {
            requested_page_index: 5,
        };

        assert_eq!(detail1, detail2);
    }

    #[test]
    fn pagination_page_click_detail_equality() {
        let detail1 = PaginationPageClickDetail {
            requested_page_available: true,
            requested_page_index: 3,
        };

        let detail2 = PaginationPageClickDetail {
            requested_page_available: true,
            requested_page_index: 3,
        };

        assert_eq!(detail1, detail2);
    }

    #[test]
    fn test_range_generation() {
        assert_eq!(range(1, 5), vec![1, 2, 3, 4, 5]);
        assert_eq!(range(3, 3), vec![3]);
        assert_eq!(range(5, 3), Vec::<u32>::new()); // Invalid range
        assert_eq!(range(1, 1), vec![1]);
    }

    #[test]
    fn test_pagination_state_basic() {
        let state = get_pagination_state(1, 10, false);

        // On page 1, should show pages 2-5 (left_index to right_index)
        // with right dots, no left dots
        assert!(!state.left_dots);
        assert!(state.right_dots);
        assert_eq!(state.left_index, 2);
        assert!(state.right_index <= 6); // Should be around 5 or 6
    }

    #[test]
    fn test_pagination_state_middle_page() {
        let state = get_pagination_state(6, 12, false);

        // On page 6 of 12, should show dots on both sides
        // left_index = 6 - 3 = 3, right_index = 6 + 3 = 9
        // left_dots = 3 > 2 = true, right_dots = 9 < 11 = true
        assert!(state.left_dots);
        assert!(state.right_dots);
        assert!(state.left_index > 2);
        assert!(state.right_index < 11);
    }

    #[test]
    fn test_pagination_state_last_page() {
        let state = get_pagination_state(10, 10, false);

        // On last page, should show left dots, no right dots
        assert!(state.left_dots);
        assert!(!state.right_dots);
        assert!(state.left_index > 2);
    }

    #[test]
    fn test_pagination_state_open_end() {
        let state = get_pagination_state(5, 10, true);

        // In open-end mode, should always show right dots
        assert!(state.right_dots);
    }

    #[test]
    fn test_pagination_state_few_pages() {
        let state = get_pagination_state(2, 3, false);

        // With only 3 pages, might not need dots
        // The exact behavior depends on the algorithm, but it should handle small counts
        assert!(state.left_index >= 2);
        assert!(state.right_index <= 3);
    }

    #[test]
    fn test_i18n_strings_default() {
        let i18n = PaginationI18nStrings::default();

        assert_eq!(i18n.previous_page_label, Some("Previous page".to_string()));
        assert_eq!(i18n.next_page_label, Some("Next page".to_string()));
        assert!(i18n.page_label.is_some());

        if let Some(page_label_fn) = i18n.page_label {
            assert_eq!(page_label_fn(1), "Page 1");
            assert_eq!(page_label_fn(10), "Page 10");
        }
    }

    #[test]
    fn test_pagination_state_edge_case_zero_pages() {
        let state = get_pagination_state(1, 0, false);

        // Should handle gracefully
        assert_eq!(state.left_index, 2);
        assert!(state.right_index == 0 || state.right_index == 1);
    }
}
