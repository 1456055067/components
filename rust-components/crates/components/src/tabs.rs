// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tabs component for tabbed navigation with active state.
//!
//! A tabbed navigation component that allows users to switch between different
//! views or content sections. Supports active tab state, disabled tabs,
//! dismissible tabs, and various visual variants.

use std::collections::HashSet;
use yew::prelude::*;
use web_sys::MouseEvent;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
    AriaAttributes,
};

/// Tab item configuration
#[derive(Clone, PartialEq, Debug)]
pub struct Tab {
    /// Unique identifier for the tab
    pub id: String,
    /// Display label for the tab
    pub label: Html,
    /// Content to display when tab is active
    pub content: Option<Html>,
    /// Whether this tab is disabled
    pub disabled: bool,
    /// Reason why the tab is disabled (shown as tooltip)
    pub disabled_reason: Option<String>,
    /// Whether the tab can be dismissed
    pub dismissible: bool,
    /// ARIA label for the dismiss button
    pub dismiss_label: Option<String>,
    /// Whether the dismiss button is disabled
    pub dismiss_disabled: bool,
    /// Optional href for the tab link
    pub href: Option<String>,
}

impl Tab {
    /// Creates a new tab with the given id and label
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::Tab;
    /// use yew::prelude::*;
    ///
    /// let tab = Tab::new("tab1", html! { "First Tab" });
    /// ```
    pub fn new(id: impl Into<String>, label: Html) -> Self {
        Self {
            id: id.into(),
            label,
            content: None,
            disabled: false,
            disabled_reason: None,
            dismissible: false,
            dismiss_label: None,
            dismiss_disabled: false,
            href: None,
        }
    }

    /// Sets the content for this tab
    pub fn with_content(mut self, content: Html) -> Self {
        self.content = Some(content);
        self
    }

    /// Sets whether this tab is disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the disabled reason for this tab
    pub fn with_disabled_reason(mut self, reason: impl Into<String>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    /// Sets whether this tab is dismissible
    pub fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Sets the dismiss label for this tab
    pub fn with_dismiss_label(mut self, label: impl Into<String>) -> Self {
        self.dismiss_label = Some(label.into());
        self
    }

    /// Sets whether the dismiss button is disabled
    pub fn with_dismiss_disabled(mut self, disabled: bool) -> Self {
        self.dismiss_disabled = disabled;
        self
    }

    /// Sets the href for this tab
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

/// Tabs variant types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsVariant {
    /// Default tabs styling
    #[default]
    Default,
    /// Container variant with border
    Container,
}

impl TabsVariant {
    fn as_str(&self) -> &'static str {
        match self {
            TabsVariant::Default => "default",
            TabsVariant::Container => "container",
        }
    }
}

/// Event detail for tab change events
#[derive(Clone, PartialEq, Debug)]
pub struct TabChangeDetail {
    /// The ID of the newly active tab
    pub active_tab_id: String,
    /// The href of the newly active tab, if defined
    pub active_tab_href: Option<String>,
}

/// Event detail for tab dismiss events
#[derive(Clone, PartialEq, Debug)]
pub struct TabDismissDetail {
    /// The ID of the dismissed tab
    pub tab_id: String,
}

/// Properties for the Tabs component
#[derive(Properties, PartialEq, Clone)]
pub struct TabsProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// The list of tab items to display
    #[prop_or_default]
    pub tabs: Vec<Tab>,

    /// The ID of the currently active tab (controlled component)
    ///
    /// If not provided, the first enabled tab will be selected by default.
    #[prop_or_default]
    pub active_tab_id: Option<String>,

    /// Callback fired when the active tab changes
    ///
    /// The event detail contains the new active tab ID and href.
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<TabChangeDetail>>>,

    /// Callback fired when a dismissible tab is dismissed
    ///
    /// The event detail contains the ID of the dismissed tab.
    #[prop_or_default]
    pub on_dismiss: Option<Callback<CustomEvent<TabDismissDetail>>>,

    /// Visual variant of the tabs
    #[prop_or_default]
    pub variant: TabsVariant,

    /// Whether tabs should fit to container width
    #[prop_or_default]
    pub fit_to_container: bool,

    /// Whether to disable padding on tab content
    #[prop_or_default]
    pub disable_content_paddings: bool,

    /// ARIA label for the tab list
    ///
    /// Use this when there's no visible label for the tabs.
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// ID of an element that labels the tab list
    ///
    /// Use this when there's a visible label element.
    #[prop_or_default]
    pub aria_labelledby: Option<String>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,
}

/// Tabs component for tabbed navigation with active state.
///
/// A controlled component that displays a set of tabs and allows users to switch
/// between different content views. The component handles keyboard navigation,
/// accessibility, and supports disabled and dismissible tabs.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{Tabs, Tab, TabsVariant, TabChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let active_tab = use_state(|| "tab1".to_string());
///
///     let tabs = vec![
///         Tab::new("tab1", html! { "First Tab" })
///             .with_content(html! { <div>{"Content for first tab"}</div> }),
///         Tab::new("tab2", html! { "Second Tab" })
///             .with_content(html! { <div>{"Content for second tab"}</div> }),
///         Tab::new("tab3", html! { "Third Tab" })
///             .with_content(html! { <div>{"Content for third tab"}</div> })
///             .with_disabled(true),
///         Tab::new("tab4", html! { "Dismissible Tab" })
///             .with_content(html! { <div>{"This tab can be closed"}</div> })
///             .with_dismissible(true)
///             .with_dismiss_label("Close tab"),
///     ];
///
///     let on_change = {
///         let active_tab = active_tab.clone();
///         Callback::from(move |event: CustomEvent<TabChangeDetail>| {
///             active_tab.set(event.detail.active_tab_id.clone());
///         })
///     };
///
///     html! {
///         <Tabs
///             tabs={tabs}
///             active_tab_id={Some((*active_tab).clone())}
///             variant={TabsVariant::Container}
///             on_change={on_change}
///             aria_label="Navigation tabs"
///         />
///     }
/// }
/// ```
#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let _metadata = ComponentMetadata::new("Tabs");

    // Track which tabs have been viewed for lazy loading
    let viewed_tabs = use_state(|| HashSet::new());

    // Determine the active tab ID
    let active_tab_id = if let Some(ref id) = props.active_tab_id {
        id.clone()
    } else {
        // Find first enabled tab
        props.tabs
            .iter()
            .find(|tab| !tab.disabled)
            .map(|tab| tab.id.clone())
            .unwrap_or_default()
    };

    // Mark active tab as viewed
    {
        let viewed_tabs = viewed_tabs.clone();
        let active_tab_id = active_tab_id.clone();
        use_effect_with(active_tab_id.clone(), move |id| {
            let mut tabs = (*viewed_tabs).clone();
            tabs.insert(id.clone());
            viewed_tabs.set(tabs);
            || ()
        });
    }

    // Find the active tab
    let active_tab = props.tabs
        .iter()
        .find(|tab| tab.id == active_tab_id);

    // Handle tab click
    let on_tab_click = {
        let on_change = props.on_change.clone();
        let tabs = props.tabs.clone();

        Callback::from(move |tab_id: String| {
            if let Some(tab) = tabs.iter().find(|t| t.id == tab_id) {
                if !tab.disabled {
                    if let Some(ref callback) = on_change {
                        callback.emit(CustomEvent::new_non_cancelable(TabChangeDetail {
                            active_tab_id: tab_id,
                            active_tab_href: tab.href.clone(),
                        }));
                    }
                }
            }
        })
    };

    // Handle tab dismiss
    let on_tab_dismiss = {
        let on_dismiss = props.on_dismiss.clone();

        Callback::from(move |tab_id: String| {
            if let Some(ref callback) = on_dismiss {
                callback.emit(CustomEvent::new_non_cancelable(TabDismissDetail {
                    tab_id,
                }));
            }
        })
    };

    // Build container classes
    let container_classes = ClassBuilder::new()
        .add("awsui-tabs")
        .add(&format!("awsui-tabs-variant-{}", props.variant.as_str()))
        .add_if(props.fit_to_container, "awsui-tabs-fit-container");

    // Build tab list classes
    let tab_list_classes = ClassBuilder::new()
        .add("awsui-tabs-tab-list")
        .add_if(props.fit_to_container, "awsui-tabs-tab-list-fit");

    // Build content wrapper classes
    let content_wrapper_classes = ClassBuilder::new()
        .add("awsui-tabs-content-wrapper")
        .add_if(!props.disable_content_paddings, "awsui-tabs-content-paddings");

    html! {
        <div
            id={props.base.id.clone()}
            class={props.base.merge_classes(&container_classes.build())}
        >
            // Tab list
            <div
                class={tab_list_classes.build()}
                role="tablist"
                aria-label={props.aria_label.clone()}
                aria-labelledby={props.aria_labelledby.clone()}
            >
                {
                    props.tabs.iter().map(|tab| {
                        render_tab_header(
                            tab,
                            &active_tab_id,
                            &on_tab_click,
                            &on_tab_dismiss,
                        )
                    }).collect::<Html>()
                }
            </div>

            // Tab content
            <div class={content_wrapper_classes.build()}>
                {
                    if let Some(active) = active_tab {
                        render_tab_content(active, &active_tab_id)
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}

/// Renders a single tab header
fn render_tab_header(
    tab: &Tab,
    active_tab_id: &str,
    on_tab_click: &Callback<String>,
    on_tab_dismiss: &Callback<String>,
) -> Html {
    let is_active = tab.id == active_tab_id;

    let tab_classes = ClassBuilder::new()
        .add("awsui-tabs-tab")
        .add_if(is_active, "awsui-tabs-tab-active")
        .add_if(tab.disabled, "awsui-tabs-tab-disabled")
        .add_if(tab.dismissible, "awsui-tabs-tab-dismissible");

    let tab_id = tab.id.clone();
    let tab_click = on_tab_click.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        tab_click.emit(tab_id.clone());
    });

    let tab_link_classes = ClassBuilder::new()
        .add("awsui-tabs-tab-link")
        .add_if(is_active, "awsui-tabs-tab-link-active");

    // Build dismiss button if needed
    let dismiss_button = if tab.dismissible {
        let tab_id = tab.id.clone();
        let dismiss_callback = on_tab_dismiss.clone();
        let dismiss_label = tab.dismiss_label.clone()
            .unwrap_or_else(|| "Dismiss tab".to_string());

        let on_dismiss_click = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            dismiss_callback.emit(tab_id.clone());
        });

        html! {
            <button
                class="awsui-tabs-tab-dismiss"
                aria-label={dismiss_label}
                disabled={tab.dismiss_disabled}
                onclick={on_dismiss_click}
                type="button"
            >
                {"×"}
            </button>
        }
    } else {
        html! {}
    };

    // Wrap in tooltip if disabled with reason
    let tab_element = html! {
        <div
            class={tab_classes.build()}
            role="presentation"
        >
            <a
                class={tab_link_classes.build()}
                href={tab.href.clone().unwrap_or_else(|| "#".to_string())}
                role="tab"
                id={format!("awsui-tabs-{}-tab", tab.id)}
                aria-selected={is_active.to_string()}
                aria-disabled={tab.disabled.to_string()}
                aria-controls={format!("awsui-tabs-{}-panel", tab.id)}
                tabindex={if is_active && !tab.disabled { "0" } else { "-1" }}
                onclick={onclick}
            >
                <span class="awsui-tabs-tab-label">
                    { tab.label.clone() }
                </span>
            </a>
            { dismiss_button }
        </div>
    };

    if let Some(ref reason) = tab.disabled_reason {
        html! {
            <div
                class="awsui-tabs-tab-tooltip-wrapper"
                title={reason.clone()}
            >
                { tab_element }
            </div>
        }
    } else {
        tab_element
    }
}

/// Renders the content for a tab
fn render_tab_content(tab: &Tab, active_tab_id: &str) -> Html {
    let is_active = tab.id == active_tab_id;

    let content_classes = ClassBuilder::new()
        .add("awsui-tabs-content")
        .add_if(is_active, "awsui-tabs-content-active");

    html! {
        <div
            class={content_classes.build()}
            role="tabpanel"
            id={format!("awsui-tabs-{}-panel", tab.id)}
            aria-labelledby={format!("awsui-tabs-{}-tab", tab.id)}
            tabindex={if is_active { "0" } else { "-1" }}
            hidden={!is_active}
        >
            { tab.content.clone().unwrap_or_default() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_new() {
        let tab = Tab::new("test-id", html! { "Test Label" });
        assert_eq!(tab.id, "test-id");
        assert!(!tab.disabled);
        assert!(!tab.dismissible);
        assert_eq!(tab.content, None);
    }

    #[test]
    fn test_tab_builder() {
        let tab = Tab::new("tab1", html! { "Label" })
            .with_content(html! { <div>{"Content"}</div> })
            .with_disabled(true)
            .with_disabled_reason("Not available")
            .with_dismissible(true)
            .with_dismiss_label("Close")
            .with_dismiss_disabled(true)
            .with_href("/path");

        assert_eq!(tab.id, "tab1");
        assert!(tab.disabled);
        assert_eq!(tab.disabled_reason, Some("Not available".to_string()));
        assert!(tab.dismissible);
        assert_eq!(tab.dismiss_label, Some("Close".to_string()));
        assert!(tab.dismiss_disabled);
        assert_eq!(tab.href, Some("/path".to_string()));
        assert!(tab.content.is_some());
    }

    #[test]
    fn test_tabs_variant_as_str() {
        assert_eq!(TabsVariant::Default.as_str(), "default");
        assert_eq!(TabsVariant::Container.as_str(), "container");
    }

    #[test]
    fn test_tab_change_detail() {
        let detail = TabChangeDetail {
            active_tab_id: "tab1".to_string(),
            active_tab_href: Some("/path".to_string()),
        };

        assert_eq!(detail.active_tab_id, "tab1");
        assert_eq!(detail.active_tab_href, Some("/path".to_string()));
    }

    #[test]
    fn test_tab_dismiss_detail() {
        let detail = TabDismissDetail {
            tab_id: "tab1".to_string(),
        };

        assert_eq!(detail.tab_id, "tab1");
    }

    #[test]
    fn test_tab_equality() {
        let tab1 = Tab::new("id1", html! { "Label" });
        let tab2 = Tab::new("id1", html! { "Label" });
        let tab3 = Tab::new("id2", html! { "Label" });

        assert_eq!(tab1.id, tab2.id);
        assert_ne!(tab1.id, tab3.id);
    }

    #[test]
    fn test_tab_with_content() {
        let tab = Tab::new("tab1", html! { "Label" })
            .with_content(html! { <div>{"Test content"}</div> });

        assert!(tab.content.is_some());
    }

    #[test]
    fn test_tabs_variant_default() {
        let variant = TabsVariant::default();
        assert_eq!(variant, TabsVariant::Default);
    }
}
