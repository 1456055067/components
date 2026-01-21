// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating the RadioGroup component

use yew::prelude::*;
use cloudscape_components::{
    RadioGroup, RadioGroupItem, RadioGroupChangeDetail, RadioGroupDirection,
    CustomEvent,
};

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| Some("option1".to_string()));

    let on_change = {
        let selected = selected.clone();
        Callback::from(move |event: CustomEvent<RadioGroupChangeDetail>| {
            selected.set(Some(event.detail.value.clone()));
        })
    };

    // Basic example
    let basic_items = vec![
        RadioGroupItem::new("option1", html! { "First option" }),
        RadioGroupItem::new("option2", html! { "Second option" }),
        RadioGroupItem::new("option3", html! { "Third option" }),
    ];

    // Example with descriptions
    let descriptive_items = vec![
        RadioGroupItem::new("small", html! { "Small" })
            .with_description(html! { "Suitable for light workloads" }),
        RadioGroupItem::new("medium", html! { "Medium" })
            .with_description(html! { "Recommended for most workloads" }),
        RadioGroupItem::new("large", html! { "Large" })
            .with_description(html! { "For compute-intensive workloads" }),
    ];

    // Example with disabled option
    let items_with_disabled = vec![
        RadioGroupItem::new("enabled1", html! { "Enabled option 1" }),
        RadioGroupItem::new("disabled", html! { "Disabled option" })
            .with_disabled(true),
        RadioGroupItem::new("enabled2", html! { "Enabled option 2" }),
    ];

    html! {
        <div style="padding: 20px; max-width: 600px;">
            <h1>{"RadioGroup Examples"}</h1>

            <section style="margin-bottom: 30px;">
                <h2>{"Basic RadioGroup"}</h2>
                <RadioGroup
                    value={(*selected).clone()}
                    items={basic_items}
                    on_change={on_change.clone()}
                />
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"RadioGroup with Descriptions"}</h2>
                <RadioGroup
                    value={None}
                    items={descriptive_items}
                    on_change={on_change.clone()}
                />
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"RadioGroup with Disabled Option"}</h2>
                <RadioGroup
                    value={None}
                    items={items_with_disabled}
                    on_change={on_change.clone()}
                />
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Horizontal RadioGroup"}</h2>
                <RadioGroup
                    value={None}
                    items={vec![
                        RadioGroupItem::new("yes", html! { "Yes" }),
                        RadioGroupItem::new("no", html! { "No" }),
                        RadioGroupItem::new("maybe", html! { "Maybe" }),
                    ]}
                    direction={RadioGroupDirection::Horizontal}
                    on_change={on_change.clone()}
                />
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Read-only RadioGroup"}</h2>
                <RadioGroup
                    value={Some("readonly-option".to_string())}
                    items={vec![
                        RadioGroupItem::new("readonly-option", html! { "Selected (read-only)" }),
                        RadioGroupItem::new("other", html! { "Other option" }),
                    ]}
                    read_only={true}
                />
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Current Selection"}</h2>
                <p>
                    {"Selected value: "}
                    <strong>
                        {
                            selected.as_ref()
                                .map(|s| s.clone())
                                .unwrap_or_else(|| "None".to_string())
                        }
                    </strong>
                </p>
            </section>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
