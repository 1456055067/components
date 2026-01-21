// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod app;
mod components;
mod pages;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    // Set up better panic messages for debugging
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Initialize Cloudscape components library
    cloudscape_components::init();

    yew::Renderer::<App>::new().render();
}
