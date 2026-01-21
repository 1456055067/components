// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Example usage of the Breadcrumbs component

use yew::prelude::*;
use cloudscape_components::{Breadcrumbs, BreadcrumbItem, BreadcrumbFollowEvent};

#[function_component(BreadcrumbsExample)]
fn breadcrumbs_example() -> Html {
    // State to track current page for demonstration
    let current_page = use_state(|| "/products/details".to_string());

    // Handler for breadcrumb navigation
    let on_follow = {
        let current_page = current_page.clone();
        Callback::from(move |mut event: BreadcrumbFollowEvent| {
            // Prevent default browser navigation
            event.prevent_default();

            // Update current page state
            current_page.set(event.detail.href.clone());

            // Log the navigation
            web_sys::console::log_1(
                &format!(
                    "Navigating to: {} ({})",
                    event.detail.text,
                    event.detail.href
                ).into()
            );
        })
    };

    // Handler for all clicks (including those with modifiers)
    let on_click = Callback::from(|event: BreadcrumbFollowEvent| {
        web_sys::console::log_1(
            &format!(
                "Breadcrumb clicked: {} at index {}",
                event.detail.text,
                event.detail.item_index
            ).into()
        );
    });

    html! {
        <div style="padding: 20px;">
            <h1>{"Breadcrumbs Component Examples"}</h1>

            <div style="margin-bottom: 40px;">
                <h2>{"Basic Breadcrumbs"}</h2>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::new("Products", "/products"),
                        BreadcrumbItem::new("Details", "/products/details"),
                    ]}
                />
            </div>

            <div style="margin-bottom: 40px;">
                <h2>{"Interactive Breadcrumbs with Navigation"}</h2>
                <p>{"Current page: "}{(*current_page).clone()}</p>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::new("Products", "/products"),
                        BreadcrumbItem::new("Electronics", "/products/electronics"),
                        BreadcrumbItem::new("Laptops", "/products/electronics/laptops"),
                        BreadcrumbItem::new("Details", "/products/electronics/laptops/123"),
                    ]}
                    on_follow={on_follow.clone()}
                    on_click={on_click.clone()}
                    aria_label="Product navigation breadcrumbs"
                />
            </div>

            <div style="margin-bottom: 40px;">
                <h2>{"Breadcrumbs with Builder Pattern"}</h2>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Dashboard", "/dashboard")
                            .with_text("Home Dashboard"),
                        BreadcrumbItem::new("", "")
                            .with_text("Settings")
                            .with_href("/settings"),
                        BreadcrumbItem::new("", "")
                            .with_text("User Preferences")
                            .with_href("/settings/preferences"),
                    ]}
                />
            </div>

            <div style="margin-bottom: 40px;">
                <h2>{"Deep Hierarchy (Many Items)"}</h2>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::new("Level 1", "/level1"),
                        BreadcrumbItem::new("Level 2", "/level1/level2"),
                        BreadcrumbItem::new("Level 3", "/level1/level2/level3"),
                        BreadcrumbItem::new("Level 4", "/level1/level2/level3/level4"),
                        BreadcrumbItem::new("Level 5", "/level1/level2/level3/level4/level5"),
                        BreadcrumbItem::new("Current", "/level1/level2/level3/level4/level5/current"),
                    ]}
                    expand_aria_label="Show full navigation path"
                />
            </div>

            <div style="margin-bottom: 40px;">
                <h2>{"Two Items Only"}</h2>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::new("Current Page", "/current"),
                    ]}
                />
            </div>

            <div style="margin-bottom: 40px;">
                <h2>{"Single Item (Current Page Only)"}</h2>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Home", "/"),
                    ]}
                />
            </div>

            <div style="margin-bottom: 40px;">
                <h2>{"Empty Breadcrumbs"}</h2>
                <Breadcrumbs items={vec![]} />
            </div>

            <div style="margin-bottom: 40px;">
                <h2>{"Custom ARIA Label"}</h2>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Documentation", "/docs"),
                        BreadcrumbItem::new("API Reference", "/docs/api"),
                        BreadcrumbItem::new("Components", "/docs/api/components"),
                    ]}
                    aria_label="Documentation navigation"
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<BreadcrumbsExample>::new().render();
}
