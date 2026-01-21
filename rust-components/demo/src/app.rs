// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
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

    html! {
        <AppLayout
            navigation_open={*navigation_open}
            navigation_width={250}
            on_navigation_change={Some(on_navigation_change)}
            tools_open={*tools_open}
            on_tools_change={Some(on_tools_change)}
            content_type={ContentType::Default}
        >
            <div slot="navigation">
                <Sidebar current_route={(*current_route).clone()} {on_navigate} />
            </div>
            <div slot="content">
                { content }
            </div>
        </AppLayout>
    }
}
