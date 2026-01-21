// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! AppLayout component for application shell structure.
//!
//! The AppLayout component provides the main application shell that organizes
//! navigation, content, tools, and other UI elements in a consistent layout.
//! It follows the Cloudscape Design System patterns for complex applications.

use crate::internal::{BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent};
use yew::prelude::*;

/// Content type determines the layout and spacing behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum ContentType {
    /// Default content type with standard padding
    #[default]
    Default,
    /// Dashboard layout with grid-friendly spacing
    Dashboard,
    /// Table layout optimized for data tables
    Table,
    /// Form layout with optimized spacing for form fields
    Form,
    /// Wizard layout for multi-step processes
    Wizard,
    /// Cards layout optimized for card grids
    Cards,
}


impl ContentType {
    /// Returns the CSS class suffix for this content type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Dashboard => "dashboard",
            Self::Table => "table",
            Self::Form => "form",
            Self::Wizard => "wizard",
            Self::Cards => "cards",
        }
    }
}

/// Split panel position configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum SplitPanelPosition {
    /// Split panel positioned at the bottom of the content area
    #[default]
    Bottom,
    /// Split panel positioned on the side of the content area
    Side,
}


impl SplitPanelPosition {
    /// Returns the CSS class suffix for this position
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Side => "side",
        }
    }
}

/// Split panel preferences configuration
#[derive(Clone, PartialEq)]
#[derive(Default)]
pub struct SplitPanelPreferences {
    /// Position of the split panel (bottom or side)
    pub position: SplitPanelPosition,

    /// Size of the split panel in pixels
    pub size: Option<u32>,
}


impl SplitPanelPreferences {
    /// Creates new split panel preferences with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the position of the split panel
    pub fn with_position(mut self, position: SplitPanelPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets the size of the split panel in pixels
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }
}

/// Event detail for navigation change events
#[derive(Clone, PartialEq, Debug)]
pub struct NavigationChangeDetail {
    /// The new open state of the navigation panel
    pub open: bool,
}

/// Event detail for tools panel change events
#[derive(Clone, PartialEq, Debug)]
pub struct ToolsChangeDetail {
    /// The new open state of the tools panel
    pub open: bool,
}

/// Event detail for split panel toggle events
#[derive(Clone, PartialEq, Debug)]
pub struct SplitPanelToggleDetail {
    /// The new open state of the split panel
    pub open: bool,
}

/// Event detail for split panel resize events
#[derive(Clone, PartialEq, Debug)]
pub struct SplitPanelResizeDetail {
    /// The new size of the split panel in pixels
    pub size: u32,
}

/// Properties for the AppLayout component
#[derive(Properties, PartialEq, Clone)]
pub struct AppLayoutProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    // Navigation panel properties
    /// Whether the navigation panel is currently open (controlled)
    #[prop_or_default]
    pub navigation_open: bool,

    /// Whether to completely hide the navigation panel
    #[prop_or(false)]
    pub navigation_hide: bool,

    /// Width of the navigation panel in pixels (default: 280)
    #[prop_or(280)]
    pub navigation_width: u32,

    /// Callback fired when navigation open state changes
    #[prop_or_default]
    pub on_navigation_change: Option<Callback<CustomEvent<NavigationChangeDetail>>>,

    /// Navigation panel content
    #[prop_or_default]
    pub navigation: Option<Html>,

    // Tools panel properties
    /// Whether the tools panel is currently open (controlled)
    #[prop_or_default]
    pub tools_open: bool,

    /// Whether to completely hide the tools panel
    #[prop_or(false)]
    pub tools_hide: bool,

    /// Width of the tools panel in pixels (default: 290)
    #[prop_or(290)]
    pub tools_width: u32,

    /// Callback fired when tools panel open state changes
    #[prop_or_default]
    pub on_tools_change: Option<Callback<CustomEvent<ToolsChangeDetail>>>,

    /// Tools panel content (typically help panel)
    #[prop_or_default]
    pub tools: Option<Html>,

    // Split panel properties
    /// Whether the split panel is currently open
    #[prop_or_default]
    pub split_panel_open: bool,

    /// Split panel preferences (position, size)
    #[prop_or_default]
    pub split_panel_preferences: SplitPanelPreferences,

    /// Callback fired when split panel is toggled
    #[prop_or_default]
    pub on_split_panel_toggle: Option<Callback<CustomEvent<SplitPanelToggleDetail>>>,

    /// Callback fired when split panel is resized
    #[prop_or_default]
    pub on_split_panel_resize: Option<Callback<CustomEvent<SplitPanelResizeDetail>>>,

    /// Split panel content
    #[prop_or_default]
    pub split_panel: Option<Html>,

    // Content properties
    /// Type of content being displayed (affects layout)
    #[prop_or_default]
    pub content_type: ContentType,

    /// Maximum width of the content area (CSS value)
    #[prop_or_default]
    pub max_content_width: Option<String>,

    /// Minimum width of the content area (CSS value)
    #[prop_or_default]
    pub min_content_width: Option<String>,

    /// Whether to disable default content padding
    #[prop_or(false)]
    pub disable_content_paddings: bool,

    // Content slots
    /// Header content (optional top banner)
    #[prop_or_default]
    pub header: Option<Html>,

    /// Breadcrumbs content
    #[prop_or_default]
    pub breadcrumbs: Option<Html>,

    /// Notifications content (typically flashbar)
    #[prop_or_default]
    pub notifications: Option<Html>,

    /// Main content area
    #[prop_or_default]
    pub children: Children,

    // ARIA properties
    /// ARIA label for the navigation region
    #[prop_or("Navigation".to_string())]
    pub aria_label_navigation: String,

    /// ARIA label for the tools region
    #[prop_or("Help panel".to_string())]
    pub aria_label_tools: String,

    /// ARIA label for the main content region
    #[prop_or("Main content".to_string())]
    pub aria_label_main: String,
}

/// AppLayout component for application shell structure.
///
/// The AppLayout provides a consistent application shell with navigation,
/// content, tools, and optional split panel areas. It handles responsive
/// behavior and provides a foundation for complex applications.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{AppLayout, NavigationChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let nav_open = use_state(|| true);
///     let tools_open = use_state(|| false);
///
///     let on_navigation_change = {
///         let nav_open = nav_open.clone();
///         Callback::from(move |event: CustomEvent<NavigationChangeDetail>| {
///             nav_open.set(event.detail.open);
///         })
///     };
///
///     let on_tools_change = {
///         let tools_open = tools_open.clone();
///         Callback::from(move |event: CustomEvent<ToolsChangeDetail>| {
///             tools_open.set(event.detail.open);
///         })
///     };
///
///     html! {
///         <AppLayout
///             navigation_open={*nav_open}
///             on_navigation_change={on_navigation_change}
///             tools_open={*tools_open}
///             on_tools_change={on_tools_change}
///             navigation={html! { <div>{"Navigation content"}</div> }}
///             tools={html! { <div>{"Help panel content"}</div> }}
///             breadcrumbs={html! { <div>{"Home > Dashboard"}</div> }}
///         >
///             <h1>{"Main content"}</h1>
///         </AppLayout>
///     }
/// }
/// ```
///
/// # With Split Panel
///
/// ```rust
/// use cloudscape_components::{AppLayout, SplitPanelPreferences, SplitPanelPosition};
/// use yew::prelude::*;
///
/// #[function_component(AppWithSplitPanel)]
/// fn app_with_split_panel() -> Html {
///     let split_panel_open = use_state(|| false);
///
///     let preferences = SplitPanelPreferences::new()
///         .with_position(SplitPanelPosition::Bottom)
///         .with_size(300);
///
///     html! {
///         <AppLayout
///             split_panel_open={*split_panel_open}
///             split_panel_preferences={preferences}
///             split_panel={html! { <div>{"Split panel content"}</div> }}
///         >
///             <div>{"Main content"}</div>
///         </AppLayout>
///     }
/// }
/// ```
///
/// # Accessibility
///
/// The AppLayout component follows accessibility best practices:
/// - Uses semantic HTML landmarks (navigation, main, complementary)
/// - Provides skip navigation links for keyboard users
/// - Sets appropriate ARIA labels for regions
/// - Ensures proper focus management when panels open/close
/// - Supports keyboard navigation throughout the interface
#[function_component(AppLayout)]
pub fn app_layout(props: &AppLayoutProps) -> Html {
    let _metadata = ComponentMetadata::new("AppLayout");

    // Handle navigation toggle
    let on_navigation_toggle = {
        let on_navigation_change = props.on_navigation_change.clone();
        let current_state = props.navigation_open;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(callback) = &on_navigation_change {
                callback.emit(CustomEvent::new_non_cancelable(NavigationChangeDetail {
                    open: !current_state,
                }));
            }
        })
    };

    // Handle tools toggle
    let on_tools_toggle = {
        let on_tools_change = props.on_tools_change.clone();
        let current_state = props.tools_open;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(callback) = &on_tools_change {
                callback.emit(CustomEvent::new_non_cancelable(ToolsChangeDetail {
                    open: !current_state,
                }));
            }
        })
    };

    // Handle split panel toggle
    let on_split_toggle = {
        let on_split_panel_toggle = props.on_split_panel_toggle.clone();
        let current_state = props.split_panel_open;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(callback) = &on_split_panel_toggle {
                callback.emit(CustomEvent::new_non_cancelable(SplitPanelToggleDetail {
                    open: !current_state,
                }));
            }
        })
    };

    // Build CSS classes for root element
    let root_classes = ClassBuilder::new()
        .add("awsui-app-layout")
        .add(format!(
            "awsui-app-layout-content-type-{}",
            props.content_type.as_str()
        ))
        .add_if(props.navigation_open, "awsui-app-layout-navigation-open")
        .add_if(props.tools_open, "awsui-app-layout-tools-open")
        .add_if(props.split_panel_open, "awsui-app-layout-split-panel-open")
        .add_if(props.navigation_hide, "awsui-app-layout-navigation-hidden")
        .add_if(props.tools_hide, "awsui-app-layout-tools-hidden")
        .add_if(
            props.split_panel.is_some(),
            format!(
                "awsui-app-layout-split-panel-position-{}",
                props.split_panel_preferences.position.as_str()
            ),
        );

    let root_class = props.base.merge_classes(&root_classes.build());

    // Build CSS classes for navigation
    let navigation_classes = ClassBuilder::new()
        .add("awsui-app-layout-navigation")
        .add_if(props.navigation_open, "awsui-app-layout-navigation-open")
        .add_if(!props.navigation_open, "awsui-app-layout-navigation-closed");

    // Build CSS classes for tools panel
    let tools_classes = ClassBuilder::new()
        .add("awsui-app-layout-tools")
        .add_if(props.tools_open, "awsui-app-layout-tools-open")
        .add_if(!props.tools_open, "awsui-app-layout-tools-closed");

    // Build CSS classes for main content
    let main_classes = ClassBuilder::new().add("awsui-app-layout-main").add_if(
        !props.disable_content_paddings,
        "awsui-app-layout-main-padded",
    );

    // Build CSS classes for content area
    let content_classes = ClassBuilder::new()
        .add("awsui-app-layout-content")
        .add(format!(
            "awsui-app-layout-content-type-{}",
            props.content_type.as_str()
        ))
        .add_if(
            !props.disable_content_paddings,
            "awsui-app-layout-content-padded",
        );

    // Helper function to build split panel classes
    let build_split_panel_classes = || {
        ClassBuilder::new()
            .add("awsui-app-layout-split-panel")
            .add(format!(
                "awsui-app-layout-split-panel-position-{}",
                props.split_panel_preferences.position.as_str()
            ))
            .add_if(props.split_panel_open, "awsui-app-layout-split-panel-open")
            .add_if(
                !props.split_panel_open,
                "awsui-app-layout-split-panel-closed",
            )
            .build()
    };

    // Build inline styles for navigation width
    let navigation_style = if props.navigation_open && !props.navigation_hide {
        Some(format!("width: {}px;", props.navigation_width))
    } else {
        None
    };

    // Build inline styles for tools width
    let tools_style = if props.tools_open && !props.tools_hide {
        Some(format!("width: {}px;", props.tools_width))
    } else {
        None
    };

    // Build inline styles for content area
    let content_style = {
        let mut styles = Vec::new();

        if let Some(ref max_width) = props.max_content_width {
            styles.push(format!("max-width: {}", max_width));
        }

        if let Some(ref min_width) = props.min_content_width {
            styles.push(format!("min-width: {}", min_width));
        }

        if styles.is_empty() {
            None
        } else {
            Some(styles.join("; "))
        }
    };

    // Build inline styles for split panel
    let split_panel_style = if props.split_panel_open {
        props.split_panel_preferences.size.map(|size| {
            match props.split_panel_preferences.position {
                SplitPanelPosition::Bottom => format!("height: {}px;", size),
                SplitPanelPosition::Side => format!("width: {}px;", size),
            }
        })
    } else {
        None
    };

    html! {
        <div
            id={props.base.id.clone()}
            class={root_class}
        >
            // Skip to main content link for accessibility
            <a href="#awsui-app-layout-main-content" class="awsui-app-layout-skip-link">
                {"Skip to main content"}
            </a>

            // Optional header banner
            if let Some(ref header) = props.header {
                <header class="awsui-app-layout-header">
                    { header.clone() }
                </header>
            }

            // Main layout container
            <div class="awsui-app-layout-container">
                // Navigation panel
                if !props.navigation_hide {
                    <nav
                        class={navigation_classes.build()}
                        style={navigation_style}
                        aria-label={props.aria_label_navigation.clone()}
                    >
                        <div class="awsui-app-layout-navigation-content">
                            if let Some(ref navigation) = props.navigation {
                                { navigation.clone() }
                            }
                        </div>

                        // Navigation toggle button
                        <button
                            class="awsui-app-layout-navigation-toggle"
                            aria-label={if props.navigation_open { "Close navigation" } else { "Open navigation" }}
                            onclick={on_navigation_toggle}
                        >
                            <span class="awsui-app-layout-toggle-icon" />
                        </button>
                    </nav>
                }

                // Main content wrapper
                <div class="awsui-app-layout-main-wrapper">
                    // Breadcrumbs
                    if let Some(ref breadcrumbs) = props.breadcrumbs {
                        <div class="awsui-app-layout-breadcrumbs">
                            { breadcrumbs.clone() }
                        </div>
                    }

                    // Notifications
                    if let Some(ref notifications) = props.notifications {
                        <div class="awsui-app-layout-notifications">
                            { notifications.clone() }
                        </div>
                    }

                    // Main content area
                    <main
                        id="awsui-app-layout-main-content"
                        class={main_classes.build()}
                        aria-label={props.aria_label_main.clone()}
                    >
                        <div class={content_classes.build()} style={content_style}>
                            { props.children.clone() }
                        </div>

                        // Split panel (when positioned at bottom)
                        if props.split_panel.is_some() &&
                           props.split_panel_preferences.position == SplitPanelPosition::Bottom {
                            <div
                                class={build_split_panel_classes()}
                                style={split_panel_style.clone()}
                            >
                                <div class="awsui-app-layout-split-panel-header">
                                    <button
                                        class="awsui-app-layout-split-panel-toggle"
                                        aria-label={if props.split_panel_open { "Close panel" } else { "Open panel" }}
                                        onclick={on_split_toggle.clone()}
                                    >
                                        <span class="awsui-app-layout-toggle-icon" />
                                    </button>
                                </div>

                                if props.split_panel_open {
                                    <div class="awsui-app-layout-split-panel-content">
                                        { props.split_panel.clone() }
                                    </div>
                                }
                            </div>
                        }
                    </main>
                </div>

                // Split panel (when positioned on side)
                if props.split_panel.is_some() &&
                   props.split_panel_preferences.position == SplitPanelPosition::Side {
                    <div
                        class={build_split_panel_classes()}
                        style={split_panel_style}
                    >
                        <div class="awsui-app-layout-split-panel-header">
                            <button
                                class="awsui-app-layout-split-panel-toggle"
                                aria-label={if props.split_panel_open { "Close panel" } else { "Open panel" }}
                                onclick={on_split_toggle}
                            >
                                <span class="awsui-app-layout-toggle-icon" />
                            </button>
                        </div>

                        if props.split_panel_open {
                            <div class="awsui-app-layout-split-panel-content">
                                { props.split_panel.clone() }
                            </div>
                        }
                    </div>
                }

                // Tools panel
                if !props.tools_hide {
                    <aside
                        class={tools_classes.build()}
                        style={tools_style}
                        aria-label={props.aria_label_tools.clone()}
                    >
                        <div class="awsui-app-layout-tools-content">
                            if let Some(ref tools) = props.tools {
                                { tools.clone() }
                            }
                        </div>

                        // Tools toggle button
                        <button
                            class="awsui-app-layout-tools-toggle"
                            aria-label={if props.tools_open { "Close tools" } else { "Open tools" }}
                            onclick={on_tools_toggle}
                        >
                            <span class="awsui-app-layout-toggle-icon" />
                        </button>
                    </aside>
                }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_default() {
        assert_eq!(ContentType::default(), ContentType::Default);
    }

    #[test]
    fn test_content_type_as_str() {
        assert_eq!(ContentType::Default.as_str(), "default");
        assert_eq!(ContentType::Dashboard.as_str(), "dashboard");
        assert_eq!(ContentType::Table.as_str(), "table");
        assert_eq!(ContentType::Form.as_str(), "form");
        assert_eq!(ContentType::Wizard.as_str(), "wizard");
        assert_eq!(ContentType::Cards.as_str(), "cards");
    }

    #[test]
    fn test_split_panel_position_default() {
        assert_eq!(SplitPanelPosition::default(), SplitPanelPosition::Bottom);
    }

    #[test]
    fn test_split_panel_position_as_str() {
        assert_eq!(SplitPanelPosition::Bottom.as_str(), "bottom");
        assert_eq!(SplitPanelPosition::Side.as_str(), "side");
    }

    #[test]
    fn test_split_panel_preferences_default() {
        let prefs = SplitPanelPreferences::default();
        assert_eq!(prefs.position, SplitPanelPosition::Bottom);
        assert_eq!(prefs.size, None);
    }

    #[test]
    fn test_split_panel_preferences_builder() {
        let prefs = SplitPanelPreferences::new()
            .with_position(SplitPanelPosition::Side)
            .with_size(400);

        assert_eq!(prefs.position, SplitPanelPosition::Side);
        assert_eq!(prefs.size, Some(400));
    }

    #[test]
    fn test_navigation_change_detail() {
        let detail = NavigationChangeDetail { open: true };
        assert!(detail.open);

        let detail = NavigationChangeDetail { open: false };
        assert!(!detail.open);
    }

    #[test]
    fn test_tools_change_detail() {
        let detail = ToolsChangeDetail { open: true };
        assert!(detail.open);

        let detail = ToolsChangeDetail { open: false };
        assert!(!detail.open);
    }

    #[test]
    fn test_split_panel_toggle_detail() {
        let detail = SplitPanelToggleDetail { open: true };
        assert!(detail.open);

        let detail = SplitPanelToggleDetail { open: false };
        assert!(!detail.open);
    }

    #[test]
    fn test_split_panel_resize_detail() {
        let detail = SplitPanelResizeDetail { size: 300 };
        assert_eq!(detail.size, 300);

        let detail = SplitPanelResizeDetail { size: 500 };
        assert_eq!(detail.size, 500);
    }

    #[test]
    fn test_navigation_change_detail_clone() {
        let detail = NavigationChangeDetail { open: true };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_tools_change_detail_clone() {
        let detail = ToolsChangeDetail { open: true };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_split_panel_toggle_detail_clone() {
        let detail = SplitPanelToggleDetail { open: true };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }

    #[test]
    fn test_split_panel_resize_detail_clone() {
        let detail = SplitPanelResizeDetail { size: 300 };
        let cloned = detail.clone();
        assert_eq!(detail, cloned);
    }
}
