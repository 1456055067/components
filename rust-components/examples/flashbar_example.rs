// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Example usage of the Flashbar component

use cloudscape_components::{
    Button, ButtonVariant, CustomEvent, Flashbar, FlashbarDismissDetail, FlashbarItem,
    FlashbarType,
};
use yew::prelude::*;

#[function_component(FlashbarExample)]
pub fn flashbar_example() -> Html {
    // State for managing flashbar items
    let items = use_state(|| {
        vec![
            FlashbarItem::new(
                FlashbarType::Success,
                html! { "Your changes have been saved successfully." },
            )
            .with_id("success-1")
            .with_header("Success")
            .with_dismissible(true),
            FlashbarItem::new(
                FlashbarType::Info,
                html! { "This is an informational message with additional details." },
            )
            .with_id("info-1")
            .with_header("Information")
            .with_dismissible(true),
        ]
    });

    // Handler for dismissing items
    let on_item_dismiss = {
        let items = items.clone();
        Callback::from(move |event: CustomEvent<FlashbarDismissDetail>| {
            let item_id = event.detail.item_id.clone();
            items.set(
                (*items)
                    .iter()
                    .filter(|item| item.id != item_id)
                    .cloned()
                    .collect(),
            );
        })
    };

    // Handler for adding new items
    let add_success = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut new_items = (*items).clone();
            new_items.push(
                FlashbarItem::new(
                    FlashbarType::Success,
                    html! { "Operation completed successfully." },
                )
                .with_id(format!("success-{}", new_items.len()))
                .with_header("Success")
                .with_dismissible(true),
            );
            items.set(new_items);
        })
    };

    let add_error = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut new_items = (*items).clone();
            new_items.push(
                FlashbarItem::new(
                    FlashbarType::Error,
                    html! { "An error occurred while processing your request." },
                )
                .with_id(format!("error-{}", new_items.len()))
                .with_header("Error")
                .with_dismissible(true),
            );
            items.set(new_items);
        })
    };

    let add_warning = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut new_items = (*items).clone();
            new_items.push(
                FlashbarItem::new(
                    FlashbarType::Warning,
                    html! { "Please review your settings before continuing." },
                )
                .with_id(format!("warning-{}", new_items.len()))
                .with_header("Warning")
                .with_dismissible(true),
            );
            items.set(new_items);
        })
    };

    let add_loading = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut new_items = (*items).clone();
            new_items.push(
                FlashbarItem::new(
                    FlashbarType::Info,
                    html! { "Processing your request..." },
                )
                .with_id(format!("loading-{}", new_items.len()))
                .with_header("Processing")
                .with_loading(true)
                .with_dismissible(false),
            );
            items.set(new_items);
        })
    };

    let add_with_action = {
        let items = items.clone();
        Callback::from(move |_| {
            let mut new_items = (*items).clone();
            let on_button_click = Callback::from(|_| {
                web_sys::console::log_1(&"Action button clicked!".into());
            });

            new_items.push(
                FlashbarItem::new(
                    FlashbarType::Info,
                    html! { "New update available." },
                )
                .with_id(format!("action-{}", new_items.len()))
                .with_header("Update")
                .with_dismissible(true)
                .with_button_text("Update now")
                .with_on_button_click(on_button_click),
            );
            items.set(new_items);
        })
    };

    html! {
        <div style="padding: 20px; max-width: 800px; margin: 0 auto;">
            <h1>{"Flashbar Component Examples"}</h1>

            <div style="margin-bottom: 20px;">
                <h2>{"Controls"}</h2>
                <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                    <Button variant={ButtonVariant::Primary} on_click={add_success}>
                        {"Add Success"}
                    </Button>
                    <Button variant={ButtonVariant::Normal} on_click={add_error}>
                        {"Add Error"}
                    </Button>
                    <Button variant={ButtonVariant::Normal} on_click={add_warning}>
                        {"Add Warning"}
                    </Button>
                    <Button variant={ButtonVariant::Normal} on_click={add_loading}>
                        {"Add Loading"}
                    </Button>
                    <Button variant={ButtonVariant::Normal} on_click={add_with_action}>
                        {"Add with Action"}
                    </Button>
                </div>
            </div>

            <h2>{"Flashbar (Default)"}</h2>
            <Flashbar
                items={(*items).clone()}
                on_item_dismiss={on_item_dismiss.clone()}
            />

            <h2 style="margin-top: 40px;">{"Flashbar (Stacked)"}</h2>
            <Flashbar
                items={(*items).clone()}
                stack_items={true}
                on_item_dismiss={on_item_dismiss}
            />

            <h2 style="margin-top: 40px;">{"Static Examples"}</h2>
            <Flashbar
                items={vec![
                    FlashbarItem::new(
                        FlashbarType::Success,
                        html! { "File uploaded successfully." },
                    )
                    .with_header("Upload Complete"),

                    FlashbarItem::new(
                        FlashbarType::Warning,
                        html! { "Your session will expire in 5 minutes." },
                    )
                    .with_header("Session Warning")
                    .with_dismissible(true),

                    FlashbarItem::new(
                        FlashbarType::Error,
                        html! { "Failed to connect to the server. Please check your connection." },
                    )
                    .with_header("Connection Error")
                    .with_dismissible(true),
                ]}
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<FlashbarExample>::new().render();
}
