// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use cloudscape_components::*;
use crate::components::sidebar::Sidebar;
use crate::pages::{
    home::Home,
    basic::BasicComponents,
    forms::FormComponents,
    layout::LayoutComponents,
    navigation::NavigationComponents,
    data_display::DataDisplayComponents,
    overlay::OverlayComponents,
    notification::NotificationComponents,
};

#[derive(Clone, PartialEq)]
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

impl Route {
    pub fn as_str(&self) -> &'static str {
        match self {
            Route::Home => "home",
            Route::Basic => "basic",
            Route::Forms => "forms",
            Route::Layout => "layout",
            Route::Navigation => "navigation",
            Route::DataDisplay => "data-display",
            Route::Overlay => "overlay",
            Route::Notification => "notification",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "basic" => Route::Basic,
            "forms" => Route::Forms,
            "layout" => Route::Layout,
            "navigation" => Route::Navigation,
            "data-display" => Route::DataDisplay,
            "overlay" => Route::Overlay,
            "notification" => Route::Notification,
            _ => Route::Home,
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let current_route = use_state(|| Route::Home);

    // Theme state: "light" or "dark"
    let theme = use_state(|| {
        // Check localStorage for saved preference
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("cloudscape-theme") {
                    return saved_theme;
                }
            }
        }
        "light".to_string()
    });

    let on_navigate = {
        let current_route = current_route.clone();
        Callback::from(move |route: Route| {
            current_route.set(route);
        })
    };

    let navigation_open = use_state(|| true);
    let tools_open = use_state(|| false);

    let on_navigation_change = {
        let navigation_open = navigation_open.clone();
        Callback::from(move |event: CustomEvent<NavigationChangeDetail>| {
            navigation_open.set(event.detail.open);
        })
    };

    let on_tools_change = {
        let tools_open = tools_open.clone();
        Callback::from(move |event: CustomEvent<ToolsChangeDetail>| {
            tools_open.set(event.detail.open);
        })
    };

    // Apply theme to document element whenever it changes
    {
        let theme = theme.clone();
        use_effect_with((*theme).clone(), move |current_theme| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.document_element() {
                        let html_element = element.dyn_into::<web_sys::HtmlElement>().ok();
                        if let Some(html) = html_element {
                            let _ = html.set_attribute("data-awsui-theme", current_theme);

                            // Save to localStorage
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("cloudscape-theme", current_theme);
                            }
                        }
                    }
                }
            }
            || ()
        });
    }

    // Toggle theme callback
    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light" { "dark" } else { "light" };
            theme.set(new_theme.to_string());
        })
    };

    let content = match (*current_route).clone() {
        Route::Home => html! { <Home /> },
        Route::Basic => html! { <BasicComponents /> },
        Route::Forms => html! { <FormComponents /> },
        Route::Layout => html! { <LayoutComponents /> },
        Route::Navigation => html! { <NavigationComponents /> },
        Route::DataDisplay => html! { <DataDisplayComponents /> },
        Route::Overlay => html! { <OverlayComponents /> },
        Route::Notification => html! { <NotificationComponents /> },
    };

    let navigation = html! {
        <Sidebar current_route={(*current_route).clone()} {on_navigate} />
    };

    // Tools panel with theme switcher
    let tools = html! {
        <Container header={Some(html! {
            <Header variant={HeaderVariant::H2}>
                {"Settings"}
            </Header>
        })}>
            <SpaceBetween size={SpaceBetweenSize::L}>
                <FormField label="Theme" description={Some(html! { {"Switch between light and dark mode"} })}>
                    <Toggle
                        checked={*theme == "dark"}
                        on_change={Callback::from(move |_event: CustomEvent<ToggleChangeDetail>| {
                            toggle_theme.emit(());
                        })}
                    >
                        {if *theme == "dark" { html! { "Dark mode" } } else { html! { "Light mode" } }}
                    </Toggle>
                </FormField>

                <Box>
                    <TextContent>
                        <h3>{"About"}</h3>
                        <p>
                            {"This demo showcases Cloudscape Design System components "}
                            {"implemented in Rust/WebAssembly using the Yew framework."}
                        </p>
                        <p>
                            <strong>{"Features:"}</strong>
                        </p>
                        <ul>
                            <li>{"45 Cloudscape components"}</li>
                            <li>{"Design tokens from style-dictionary"}</li>
                            <li>{"Dark mode support"}</li>
                            <li>{"Type-safe Rust implementation"}</li>
                        </ul>
                    </TextContent>
                </Box>
            </SpaceBetween>
        </Container>
    };

    html! {
        <AppLayout
            navigation_open={*navigation_open}
            navigation_width={250}
            navigation={Some(navigation)}
            on_navigation_change={Some(on_navigation_change)}
            tools_open={*tools_open}
            tools={Some(tools)}
            on_tools_change={Some(on_tools_change)}
            content_type={ContentType::Default}
        >
            { content }
        </AppLayout>
    }
}
