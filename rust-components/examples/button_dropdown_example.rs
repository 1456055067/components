// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating the ButtonDropdown component

use cloudscape_components::{
    ButtonDropdown, ButtonDropdownItem, ButtonDropdownItemClickDetail, ButtonDropdownItemGroup,
    ButtonVariant, CustomEvent,
};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let last_action = use_state(|| "None".to_string());

    let on_item_click = {
        let last_action = last_action.clone();
        Callback::from(move |event: CustomEvent<ButtonDropdownItemClickDetail>| {
            last_action.set(event.detail.id.clone());
            web_sys::console::log_1(&format!("Item clicked: {}", event.detail.id).into());
        })
    };

    // Basic flat items
    let basic_items = vec![
        ButtonDropdownItem::new("action1", "Action 1"),
        ButtonDropdownItem::new("action2", "Action 2"),
        ButtonDropdownItem::new("action3", "Action 3").with_disabled(true),
    ];

    // Items with links
    let link_items = vec![
        ButtonDropdownItem::new("internal", "Internal Link")
            .with_href("https://example.com/internal"),
        ButtonDropdownItem::new("external", "External Link")
            .with_href("https://example.com/external")
            .with_external(true),
        ButtonDropdownItem::new("download", "Download")
            .with_href("https://example.com/file.pdf")
            .with_external(true),
    ];

    // Grouped items
    let item_groups = vec![
        ButtonDropdownItemGroup::new()
            .with_text("Edit Actions")
            .with_items(vec![
                ButtonDropdownItem::new("edit", "Edit"),
                ButtonDropdownItem::new("duplicate", "Duplicate"),
                ButtonDropdownItem::new("copy", "Copy"),
            ]),
        ButtonDropdownItemGroup::new()
            .with_text("Danger Zone")
            .with_items(vec![
                ButtonDropdownItem::new("archive", "Archive"),
                ButtonDropdownItem::new("delete", "Delete"),
            ]),
    ];

    // Items with icons
    let icon_items = vec![
        ButtonDropdownItem::new("save", "Save")
            .with_icon(html! { <span>{"💾"}</span> }),
        ButtonDropdownItem::new("share", "Share")
            .with_icon(html! { <span>{"🔗"}</span> }),
        ButtonDropdownItem::new("print", "Print")
            .with_icon(html! { <span>{"🖨️"}</span> }),
    ];

    html! {
        <div style="padding: 20px; max-width: 800px;">
            <h1>{"ButtonDropdown Examples"}</h1>

            <section style="margin-bottom: 30px;">
                <h2>{"Basic ButtonDropdown (Primary)"}</h2>
                <ButtonDropdown
                    items={basic_items.clone()}
                    variant={ButtonVariant::Primary}
                    on_item_click={on_item_click.clone()}
                >
                    {"Actions"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Normal Variant"}</h2>
                <ButtonDropdown
                    items={basic_items.clone()}
                    variant={ButtonVariant::Normal}
                    on_item_click={on_item_click.clone()}
                >
                    {"More Actions"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"With Links"}</h2>
                <ButtonDropdown
                    items={link_items}
                    variant={ButtonVariant::Normal}
                    on_item_click={on_item_click.clone()}
                >
                    {"Links"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Disabled State"}</h2>
                <ButtonDropdown
                    items={basic_items.clone()}
                    variant={ButtonVariant::Primary}
                    disabled={true}
                >
                    {"Disabled"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Loading State"}</h2>
                <ButtonDropdown
                    items={basic_items.clone()}
                    variant={ButtonVariant::Primary}
                    loading={true}
                >
                    {"Loading"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"With Item Groups"}</h2>
                <ButtonDropdown
                    item_groups={item_groups}
                    variant={ButtonVariant::Normal}
                    on_item_click={on_item_click.clone()}
                >
                    {"Grouped Actions"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"With Icons"}</h2>
                <ButtonDropdown
                    items={icon_items}
                    variant={ButtonVariant::Normal}
                    on_item_click={on_item_click.clone()}
                >
                    {"Actions with Icons"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Icon Variant"}</h2>
                <ButtonDropdown
                    items={basic_items.clone()}
                    variant={ButtonVariant::Icon}
                    on_item_click={on_item_click.clone()}
                    aria_label="More actions"
                    icon={html! { <span>{"⋮"}</span> }}
                >
                    {""}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px;">
                <h2>{"Expanded (Full Width)"}</h2>
                <ButtonDropdown
                    items={basic_items}
                    variant={ButtonVariant::Normal}
                    expanded={true}
                    on_item_click={on_item_click.clone()}
                >
                    {"Full Width Actions"}
                </ButtonDropdown>
            </section>

            <section style="margin-bottom: 30px; padding: 20px; background: #f5f5f5; border-radius: 8px;">
                <h2>{"Last Action"}</h2>
                <p>
                    {"Last clicked item: "}
                    <strong style="font-family: monospace; background: white; padding: 4px 8px; border-radius: 4px;">
                        { (*last_action).clone() }
                    </strong>
                </p>
            </section>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
