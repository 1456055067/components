// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating the Tabs component

use yew::prelude::*;
use cloudscape_components::{
    Tabs, Tab, TabsVariant, TabChangeDetail, TabDismissDetail,
    CustomEvent,
};

#[function_component(App)]
fn app() -> Html {
    let active_tab = use_state(|| "tab1".to_string());
    let dismissible_tabs = use_state(|| vec!["dismiss1", "dismiss2", "dismiss3"]);

    let on_change = {
        let active_tab = active_tab.clone();
        Callback::from(move |event: CustomEvent<TabChangeDetail>| {
            active_tab.set(event.detail.active_tab_id.clone());
        })
    };

    let on_dismiss = {
        let dismissible_tabs = dismissible_tabs.clone();
        Callback::from(move |event: CustomEvent<TabDismissDetail>| {
            let mut tabs = (*dismissible_tabs).clone();
            tabs.retain(|&id| id != event.detail.tab_id);
            dismissible_tabs.set(tabs);
        })
    };

    // Basic tabs example
    let basic_tabs = vec![
        Tab::new("tab1", html! { "First Tab" })
            .with_content(html! {
                <div>
                    <h3>{"First Tab Content"}</h3>
                    <p>{"This is the content of the first tab."}</p>
                </div>
            }),
        Tab::new("tab2", html! { "Second Tab" })
            .with_content(html! {
                <div>
                    <h3>{"Second Tab Content"}</h3>
                    <p>{"This is the content of the second tab."}</p>
                </div>
            }),
        Tab::new("tab3", html! { "Third Tab" })
            .with_content(html! {
                <div>
                    <h3>{"Third Tab Content"}</h3>
                    <p>{"This is the content of the third tab."}</p>
                </div>
            }),
    ];

    // Tabs with disabled tab
    let tabs_with_disabled = vec![
        Tab::new("enabled1", html! { "Enabled Tab" })
            .with_content(html! { <p>{"Content for enabled tab"}</p> }),
        Tab::new("disabled", html! { "Disabled Tab" })
            .with_disabled(true)
            .with_disabled_reason("This feature is not available")
            .with_content(html! { <p>{"This content won't be shown"}</p> }),
        Tab::new("enabled2", html! { "Another Enabled Tab" })
            .with_content(html! { <p>{"Content for another enabled tab"}</p> }),
    ];

    // Dismissible tabs
    let dismissible_tab_list = (*dismissible_tabs).iter().enumerate().map(|(i, &id)| {
        Tab::new(id, html! { format!("Tab {}", i + 1) })
            .with_content(html! {
                <div>
                    <h4>{format!("Dismissible Tab {} Content", i + 1)}</h4>
                    <p>{"Click the × button to dismiss this tab."}</p>
                </div>
            })
            .with_dismissible(true)
            .with_dismiss_label(format!("Close tab {}", i + 1))
    }).collect::<Vec<_>>();

    // Container variant tabs
    let container_tabs = vec![
        Tab::new("overview", html! { "Overview" })
            .with_content(html! {
                <div>
                    <h3>{"Overview"}</h3>
                    <p>{"This is an overview of the container variant."}</p>
                    <p>{"The container variant displays tabs with a border and container styling."}</p>
                </div>
            }),
        Tab::new("details", html! { "Details" })
            .with_content(html! {
                <div>
                    <h3>{"Details"}</h3>
                    <p>{"Detailed information goes here."}</p>
                    <ul>
                        <li>{"Detail 1"}</li>
                        <li>{"Detail 2"}</li>
                        <li>{"Detail 3"}</li>
                    </ul>
                </div>
            }),
        Tab::new("settings", html! { "Settings" })
            .with_content(html! {
                <div>
                    <h3>{"Settings"}</h3>
                    <p>{"Configuration options would appear here."}</p>
                </div>
            }),
    ];

    // Tabs with links
    let tabs_with_links = vec![
        Tab::new("home", html! { "Home" })
            .with_href("#home")
            .with_content(html! { <p>{"Home content"}</p> }),
        Tab::new("about", html! { "About" })
            .with_href("#about")
            .with_content(html! { <p>{"About content"}</p> }),
        Tab::new("contact", html! { "Contact" })
            .with_href("#contact")
            .with_content(html! { <p>{"Contact content"}</p> }),
    ];

    html! {
        <div style="padding: 20px; max-width: 1200px;">
            <h1>{"Tabs Component Examples"}</h1>

            <section style="margin-bottom: 40px;">
                <h2>{"Basic Tabs (Default Variant)"}</h2>
                <Tabs
                    tabs={basic_tabs}
                    active_tab_id={Some((*active_tab).clone())}
                    on_change={on_change.clone()}
                    aria_label="Basic navigation tabs"
                />
            </section>

            <section style="margin-bottom: 40px;">
                <h2>{"Tabs with Disabled Tab"}</h2>
                <p>{"Hover over the disabled tab to see the reason."}</p>
                <Tabs
                    tabs={tabs_with_disabled}
                    aria_label="Tabs with disabled option"
                />
            </section>

            <section style="margin-bottom: 40px;">
                <h2>{"Dismissible Tabs"}</h2>
                <p>{"Click the × button to dismiss tabs. Remaining tabs: "}{(*dismissible_tabs).len()}</p>
                {
                    if (*dismissible_tabs).is_empty() {
                        html! {
                            <p><em>{"All tabs have been dismissed!"}</em></p>
                        }
                    } else {
                        html! {
                            <Tabs
                                tabs={dismissible_tab_list}
                                on_dismiss={on_dismiss.clone()}
                                aria_label="Dismissible tabs example"
                            />
                        }
                    }
                }
            </section>

            <section style="margin-bottom: 40px;">
                <h2>{"Container Variant"}</h2>
                <Tabs
                    tabs={container_tabs}
                    variant={TabsVariant::Container}
                    aria_label="Container variant tabs"
                />
            </section>

            <section style="margin-bottom: 40px;">
                <h2>{"Tabs with Links"}</h2>
                <p>{"Each tab has an href attribute. Click with modifier keys to open in new tab."}</p>
                <Tabs
                    tabs={tabs_with_links}
                    aria_label="Tabs with links"
                />
            </section>

            <section style="margin-bottom: 40px;">
                <h2>{"Fit to Container"}</h2>
                <div style="border: 2px dashed #ccc; padding: 10px;">
                    <Tabs
                        tabs={vec![
                            Tab::new("wide1", html! { "Wide Tab 1" })
                                .with_content(html! { <p>{"Content 1"}</p> }),
                            Tab::new("wide2", html! { "Wide Tab 2" })
                                .with_content(html! { <p>{"Content 2"}</p> }),
                            Tab::new("wide3", html! { "Wide Tab 3" })
                                .with_content(html! { <p>{"Content 3"}</p> }),
                        ]}
                        fit_to_container={true}
                        aria_label="Tabs that fit to container"
                    />
                </div>
            </section>

            <section style="margin-bottom: 40px;">
                <h2>{"Without Content Padding"}</h2>
                <Tabs
                    tabs={vec![
                        Tab::new("no-pad-1", html! { "Tab 1" })
                            .with_content(html! {
                                <div style="background: #f0f0f0; padding: 20px;">
                                    {"This content has no default padding"}
                                </div>
                            }),
                        Tab::new("no-pad-2", html! { "Tab 2" })
                            .with_content(html! {
                                <div style="background: #e0e0e0; padding: 20px;">
                                    {"Custom padding can be applied"}
                                </div>
                            }),
                    ]}
                    disable_content_paddings={true}
                    aria_label="Tabs without content padding"
                />
            </section>

            <section style="margin-bottom: 40px;">
                <h2>{"Current State"}</h2>
                <p>
                    {"Active tab ID: "}
                    <strong>{(*active_tab).clone()}</strong>
                </p>
                <p>
                    {"Dismissible tabs remaining: "}
                    <strong>{(*dismissible_tabs).len()}</strong>
                </p>
            </section>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
