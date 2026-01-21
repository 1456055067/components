// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating the SpaceBetween component

use yew::prelude::*;
use cloudscape_components::{
    SpaceBetween, SpaceBetweenAlignment, SpaceBetweenDirection, SpaceBetweenSize,
    Button, Box, BoxVariant, SpacingSize, Container, Header,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="padding: 20px; max-width: 1200px;">
            <h1>{"SpaceBetween Examples"}</h1>

            <SpaceBetween size={SpaceBetweenSize::L}>
                <section>
                    <h2>{"Vertical Spacing (Default)"}</h2>
                    <Container header={html! { <Header>{"Button Stack"}</Header> }}>
                        <SpaceBetween size={SpaceBetweenSize::M}>
                            <Button>{"First Button"}</Button>
                            <Button>{"Second Button"}</Button>
                            <Button>{"Third Button"}</Button>
                            <Button>{"Fourth Button"}</Button>
                        </SpaceBetween>
                    </Container>
                </section>

                <section>
                    <h2>{"Horizontal Spacing"}</h2>
                    <Container header={html! { <Header>{"Button Group"}</Header> }}>
                        <SpaceBetween
                            size={SpaceBetweenSize::S}
                            direction={SpaceBetweenDirection::Horizontal}
                        >
                            <Button>{"Cancel"}</Button>
                            <Button>{"Save Draft"}</Button>
                            <Button>{"Publish"}</Button>
                        </SpaceBetween>
                    </Container>
                </section>

                <section>
                    <h2>{"Size Variants"}</h2>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <Container header={html! { <Header>{"Extra Small (Xs)"}</Header> }}>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::P}>{"Item 1"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 2"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 3"}</Box>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Small (S)"}</Header> }}>
                            <SpaceBetween size={SpaceBetweenSize::S}>
                                <Box variant={BoxVariant::P}>{"Item 1"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 2"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 3"}</Box>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Medium (M) - Default"}</Header> }}>
                            <SpaceBetween size={SpaceBetweenSize::M}>
                                <Box variant={BoxVariant::P}>{"Item 1"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 2"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 3"}</Box>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Large (L)"}</Header> }}>
                            <SpaceBetween size={SpaceBetweenSize::L}>
                                <Box variant={BoxVariant::P}>{"Item 1"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 2"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 3"}</Box>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Extra Large (Xl)"}</Header> }}>
                            <SpaceBetween size={SpaceBetweenSize::Xl}>
                                <Box variant={BoxVariant::P}>{"Item 1"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 2"}</Box>
                                <Box variant={BoxVariant::P}>{"Item 3"}</Box>
                            </SpaceBetween>
                        </Container>
                    </SpaceBetween>
                </section>

                <section>
                    <h2>{"Alignment Options"}</h2>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <Container header={html! { <Header>{"Horizontal - Center Vertical Alignment"}</Header> }}>
                            <SpaceBetween
                                size={SpaceBetweenSize::S}
                                direction={SpaceBetweenDirection::Horizontal}
                                alignment_vertical={Some(SpaceBetweenAlignment::Center)}
                            >
                                <Box variant={BoxVariant::P}>{"Small text"}</Box>
                                <Box variant={BoxVariant::H3}>{"Large text"}</Box>
                                <Box variant={BoxVariant::P}>{"Small text again"}</Box>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Horizontal - End Vertical Alignment"}</Header> }}>
                            <SpaceBetween
                                size={SpaceBetweenSize::S}
                                direction={SpaceBetweenDirection::Horizontal}
                                alignment_vertical={Some(SpaceBetweenAlignment::End)}
                            >
                                <Box variant={BoxVariant::P}>{"Small text"}</Box>
                                <Box variant={BoxVariant::H3}>{"Large text"}</Box>
                                <Box variant={BoxVariant::P}>{"Small text again"}</Box>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Vertical - Center Horizontal Alignment"}</Header> }}>
                            <SpaceBetween
                                size={SpaceBetweenSize::M}
                                direction={SpaceBetweenDirection::Vertical}
                                alignment_horizontal={Some(SpaceBetweenAlignment::Center)}
                            >
                                <Button>{"Button 1"}</Button>
                                <Button>{"Button 2"}</Button>
                                <Button>{"Button 3"}</Button>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Vertical - End Horizontal Alignment"}</Header> }}>
                            <SpaceBetween
                                size={SpaceBetweenSize::M}
                                direction={SpaceBetweenDirection::Vertical}
                                alignment_horizontal={Some(SpaceBetweenAlignment::End)}
                            >
                                <Button>{"Button 1"}</Button>
                                <Button>{"Button 2"}</Button>
                                <Button>{"Button 3"}</Button>
                            </SpaceBetween>
                        </Container>
                    </SpaceBetween>
                </section>

                <section>
                    <h2>{"Nested SpaceBetween"}</h2>
                    <Container header={html! { <Header>{"Form Layout Example"}</Header> }}>
                        <SpaceBetween size={SpaceBetweenSize::L}>
                            <Box variant={BoxVariant::H3}>{"Account Settings"}</Box>

                            <SpaceBetween size={SpaceBetweenSize::M}>
                                <Box variant={BoxVariant::P}>
                                    {"Configure your account preferences below."}
                                </Box>

                                <Box padding={SpacingSize::M}>
                                    <SpaceBetween size={SpaceBetweenSize::S}>
                                        <Box variant={BoxVariant::P}>{"Setting 1: Enabled"}</Box>
                                        <Box variant={BoxVariant::P}>{"Setting 2: Disabled"}</Box>
                                        <Box variant={BoxVariant::P}>{"Setting 3: Enabled"}</Box>
                                    </SpaceBetween>
                                </Box>

                                <SpaceBetween
                                    size={SpaceBetweenSize::S}
                                    direction={SpaceBetweenDirection::Horizontal}
                                    alignment_horizontal={Some(SpaceBetweenAlignment::End)}
                                >
                                    <Button>{"Cancel"}</Button>
                                    <Button>{"Save Changes"}</Button>
                                </SpaceBetween>
                            </SpaceBetween>
                        </SpaceBetween>
                    </Container>
                </section>

                <section>
                    <h2>{"Compact Spacing (Xxxs, Xxs)"}</h2>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <Container header={html! { <Header>{"Extra Extra Extra Small (Xxxs)"}</Header> }}>
                            <SpaceBetween size={SpaceBetweenSize::Xxxs}>
                                <Box variant={BoxVariant::P}>{"Very compact item 1"}</Box>
                                <Box variant={BoxVariant::P}>{"Very compact item 2"}</Box>
                                <Box variant={BoxVariant::P}>{"Very compact item 3"}</Box>
                            </SpaceBetween>
                        </Container>

                        <Container header={html! { <Header>{"Extra Extra Small (Xxs)"}</Header> }}>
                            <SpaceBetween size={SpaceBetweenSize::Xxs}>
                                <Box variant={BoxVariant::P}>{"Compact item 1"}</Box>
                                <Box variant={BoxVariant::P}>{"Compact item 2"}</Box>
                                <Box variant={BoxVariant::P}>{"Compact item 3"}</Box>
                            </SpaceBetween>
                        </Container>
                    </SpaceBetween>
                </section>

                <section>
                    <h2>{"Extra Large Spacing (Xxl)"}</h2>
                    <Container header={html! { <Header>{"Extra Extra Large (Xxl)"}</Header> }}>
                        <SpaceBetween size={SpaceBetweenSize::Xxl}>
                            <Box variant={BoxVariant::H3}>{"Section 1"}</Box>
                            <Box variant={BoxVariant::P}>
                                {"This spacing is useful for separating major sections of content."}
                            </Box>
                            <Box variant={BoxVariant::H3}>{"Section 2"}</Box>
                            <Box variant={BoxVariant::P}>
                                {"The extra large gap creates clear visual separation."}
                            </Box>
                        </SpaceBetween>
                    </Container>
                </section>
            </SpaceBetween>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
