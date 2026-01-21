// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! ExpandableSection component example
//!
//! This example demonstrates various uses of the ExpandableSection component:
//! - Uncontrolled mode with default_expanded
//! - Controlled mode with state management
//! - Different variants (default, container, footer, navigation)
//! - Header actions and descriptions
//! - Disabled content padding

use yew::prelude::*;
use cloudscape_components::{
    ExpandableSection, ExpandableSectionVariant, ExpandableSectionChangeDetail, CustomEvent,
    Button, ButtonVariant,
};

#[function_component(UncontrolledExample)]
fn uncontrolled_example() -> Html {
    html! {
        <div style="margin: 20px;">
            <h3>{"Uncontrolled ExpandableSection"}</h3>
            <ExpandableSection
                header="Click to expand"
                default_expanded={false}
            >
                <p>{"This is an uncontrolled expandable section."}</p>
                <p>{"It manages its own state internally."}</p>
                <p>{"The initial state is collapsed (default_expanded=false)."}</p>
            </ExpandableSection>
        </div>
    }
}

#[function_component(ControlledExample)]
fn controlled_example() -> Html {
    let expanded = use_state(|| false);
    let change_count = use_state(|| 0);

    let on_change = {
        let expanded = expanded.clone();
        let change_count = change_count.clone();
        Callback::from(move |event: CustomEvent<ExpandableSectionChangeDetail>| {
            expanded.set(event.detail.expanded);
            change_count.set(*change_count + 1);
        })
    };

    let toggle_button_click = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    html! {
        <div style="margin: 20px;">
            <h3>{"Controlled ExpandableSection"}</h3>
            <p>{"Times changed: "}{*change_count}</p>
            <Button
                variant={ButtonVariant::Primary}
                onclick={toggle_button_click}
            >
                {if *expanded { "Collapse from outside" } else { "Expand from outside" }}
            </Button>
            <br />
            <br />
            <ExpandableSection
                header_text={html! { "Controlled Section" }}
                expanded={Some(*expanded)}
                on_change={on_change}
            >
                <p>{"This is a controlled expandable section."}</p>
                <p>{"Its state is managed externally."}</p>
                <p>{"You can toggle it with the button above or by clicking the header."}</p>
            </ExpandableSection>
        </div>
    }
}

#[function_component(VariantsExample)]
fn variants_example() -> Html {
    html! {
        <div style="margin: 20px;">
            <h3>{"Different Variants"}</h3>

            <div style="margin-bottom: 20px;">
                <h4>{"Default Variant"}</h4>
                <ExpandableSection
                    header="Default variant"
                    variant={ExpandableSectionVariant::Default}
                    default_expanded={true}
                >
                    <p>{"This is the default variant."}</p>
                    <p>{"Use this in any general context."}</p>
                </ExpandableSection>
            </div>

            <div style="margin-bottom: 20px;">
                <h4>{"Container Variant"}</h4>
                <ExpandableSection
                    header_text={html! { "Container variant" }}
                    variant={ExpandableSectionVariant::Container}
                    default_expanded={true}
                >
                    <p>{"This is the container variant."}</p>
                    <p>{"Use this alongside other containers in detail pages."}</p>
                </ExpandableSection>
            </div>

            <div style="margin-bottom: 20px;">
                <h4>{"Footer Variant"}</h4>
                <ExpandableSection
                    header="Footer variant"
                    variant={ExpandableSectionVariant::Footer}
                    default_expanded={false}
                >
                    <p>{"This is the footer variant."}</p>
                    <p>{"Use this in container footers."}</p>
                </ExpandableSection>
            </div>

            <div style="margin-bottom: 20px;">
                <h4>{"Navigation Variant"}</h4>
                <ExpandableSection
                    header_text={html! { "Navigation variant" }}
                    variant={ExpandableSectionVariant::Navigation}
                    default_expanded={false}
                >
                    <ul>
                        <li><a href="#">{"Navigation Link 1"}</a></li>
                        <li><a href="#">{"Navigation Link 2"}</a></li>
                        <li><a href="#">{"Navigation Link 3"}</a></li>
                    </ul>
                </ExpandableSection>
            </div>
        </div>
    }
}

#[function_component(WithHeaderActionsExample)]
fn with_header_actions_example() -> Html {
    let action_clicked = use_state(|| 0);

    let on_action_click = {
        let action_clicked = action_clicked.clone();
        Callback::from(move |_| {
            action_clicked.set(*action_clicked + 1);
        })
    };

    html! {
        <div style="margin: 20px;">
            <h3>{"ExpandableSection with Header Actions"}</h3>
            <p>{"Action clicked: "}{*action_clicked}{" times"}</p>
            <ExpandableSection
                header_text={html! { "Section with Actions" }}
                header_actions={html! {
                    <Button
                        variant={ButtonVariant::Normal}
                        onclick={on_action_click}
                    >
                        {"Action"}
                    </Button>
                }}
                default_expanded={true}
            >
                <p>{"This section has an action button in the header."}</p>
                <p>{"The button is separate from the expand/collapse functionality."}</p>
            </ExpandableSection>
        </div>
    }
}

#[function_component(WithDescriptionExample)]
fn with_description_example() -> Html {
    html! {
        <div style="margin: 20px;">
            <h3>{"ExpandableSection with Description"}</h3>
            <ExpandableSection
                header_text={html! { "Configuration Settings" }}
                header_description={html! {
                    {"Configure advanced options for your application"}
                }}
                variant={ExpandableSectionVariant::Container}
                default_expanded={true}
            >
                <div>
                    <label>{"Setting 1: "}<input type="text" /></label>
                    <br />
                    <label>{"Setting 2: "}<input type="checkbox" /></label>
                    <br />
                    <label>{"Setting 3: "}<input type="number" /></label>
                </div>
            </ExpandableSection>
        </div>
    }
}

#[function_component(WithoutPaddingExample)]
fn without_padding_example() -> Html {
    html! {
        <div style="margin: 20px;">
            <h3>{"ExpandableSection without Content Padding"}</h3>
            <ExpandableSection
                header="Full-width content"
                disable_content_paddings={true}
                default_expanded={true}
            >
                <div style="background: #e7f3ff; padding: 10px;">
                    <p>{"This content has no default padding."}</p>
                    <p>{"The blue background extends to the edges."}</p>
                    <p>{"Useful for custom layouts or tables."}</p>
                </div>
            </ExpandableSection>
        </div>
    }
}

#[function_component(NestedExample)]
fn nested_example() -> Html {
    html! {
        <div style="margin: 20px;">
            <h3>{"Nested ExpandableSections"}</h3>
            <ExpandableSection
                header="Parent Section"
                default_expanded={true}
            >
                <p>{"This is the parent section."}</p>

                <ExpandableSection
                    header="Child Section 1"
                    default_expanded={false}
                >
                    <p>{"This is the first nested section."}</p>
                </ExpandableSection>

                <br />

                <ExpandableSection
                    header="Child Section 2"
                    default_expanded={false}
                >
                    <p>{"This is the second nested section."}</p>

                    <ExpandableSection
                        header="Grandchild Section"
                        default_expanded={false}
                    >
                        <p>{"You can nest expandable sections multiple levels deep."}</p>
                    </ExpandableSection>
                </ExpandableSection>
            </ExpandableSection>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="max-width: 1200px; margin: 0 auto; padding: 20px;">
            <h1>{"ExpandableSection Component Examples"}</h1>
            <p>{"This page demonstrates various uses of the ExpandableSection component."}</p>

            <hr />
            <UncontrolledExample />

            <hr />
            <ControlledExample />

            <hr />
            <VariantsExample />

            <hr />
            <WithHeaderActionsExample />

            <hr />
            <WithDescriptionExample />

            <hr />
            <WithoutPaddingExample />

            <hr />
            <NestedExample />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
