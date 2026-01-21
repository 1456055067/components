// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod app;
mod components;
mod pages;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
