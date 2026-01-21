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
    // Simple test to verify rendering works
    html! {
        <div style="padding: 20px; font-family: sans-serif;">
            <h1>{"Cloudscape Components Demo"}</h1>
            <p>{"If you can see this, the WASM app is working!"}</p>
        </div>
    }
}
