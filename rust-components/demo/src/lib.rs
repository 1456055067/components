// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod app;
mod components;
mod pages;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    // Set up better panic messages for debugging
    console_error_panic_hook::set_once();

    log("WASM module started");

    // Initialize Cloudscape components library
    cloudscape_components::init();

    log("Cloudscape components initialized");

    yew::Renderer::<App>::new().render();

    log("Yew renderer created");
}
