// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use crate::components::code_snippet::CodeSnippet;

#[function_component(LayoutComponents)]
pub fn layout_components() -> Html {
    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Layout Components"}
                    <div slot="description">
                        {"Components for structuring page layout and content organization"}
                    </div>
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                // Container Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Container"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Content Container"}</div>
                            <div class="demo-example-description">{"Wrapper with header, footer, and media slots"}</div>
                            <div class="demo-preview">
                                <Container variant={ContainerVariant::Default}>
                                    <div slot="header">
                                        <Header variant={HeaderVariant::H2}>
                                            {"Container Header"}
                                            <div slot="actions">
                                                <Button variant={ButtonVariant::Primary}>{"Action"}</Button>
                                            </div>
                                        </Header>
                                    </div>
                                    <TextContent>
                                        <p>{"This is the main content area of the container. It can hold any content including text, forms, or other components."}</p>
                                    </TextContent>
                                    <div slot="footer">
                                        <Box variant={BoxVariant::P}>{"Footer content"}</Box>
                                    </div>
                                </Container>
                            </div>
                            <CodeSnippet code={r#"<Container variant={ContainerVariant::Default}>
    <div slot="header">
        <Header variant={HeaderVariant::H2}>
            {"Header"}
        </Header>
    </div>
    <p>{"Content goes here"}</p>
    <div slot="footer">
        {"Footer"}
    </div>
</Container>"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Container Variants"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Container variant={ContainerVariant::Stacked}>
                                        <div slot="header">
                                            <Header variant={HeaderVariant::H3}>{"Stacked Container"}</Header>
                                        </div>
                                        <TextContent>
                                            <p>{"Stacked variant for compact layouts"}</p>
                                        </TextContent>
                                    </Container>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Container variant={ContainerVariant::Stacked}>
    {"Content"}
</Container>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Header Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Header"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Header Variants"}</div>
                            <div class="demo-example-description">{"Page and section headers with actions and info"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::M}>
                                    <Header variant={HeaderVariant::H1}>
                                        {"Page Title (H1)"}
                                        <div slot="actions">
                                            <SpaceBetween size={SpaceBetweenSize::Xs} direction={SpaceBetweenDirection::Horizontal}>
                                                <Button>{"Action 1"}</Button>
                                                <Button variant={ButtonVariant::Primary}>{"Action 2"}</Button>
                                            </SpaceBetween>
                                        </div>
                                        <div slot="description">
                                            {"Page description with additional context"}
                                        </div>
                                        <div slot="info">
                                            <Link variant={LinkVariant::Info}>{"Info"}</Link>
                                        </div>
                                    </Header>

                                    <Header variant={HeaderVariant::H2}>
                                        {"Section Title (H2)"}
                                        <div slot="description">
                                            {"Section description"}
                                        </div>
                                    </Header>

                                    <Header variant={HeaderVariant::H3}>
                                        {"Subsection Title (H3)"}
                                    </Header>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Header variant={HeaderVariant::H1}>
    {"Page Title"}
    <div slot="actions">
        <Button>{"Action"}</Button>
    </div>
    <div slot="description">
        {"Description"}
    </div>
</Header>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // SpaceBetween Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"SpaceBetween"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Layout Spacing Utility"}</div>
                            <div class="demo-example-description">{"Consistent spacing between elements"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::M}>
                                    <Box variant={BoxVariant::P}>{"Vertical spacing (default)"}</Box>
                                    <Container>
                                        <SpaceBetween size={SpaceBetweenSize::S}>
                                            <Badge color={BadgeColor::Blue}>{"Item 1"}</Badge>
                                            <Badge color={BadgeColor::Green}>{"Item 2"}</Badge>
                                            <Badge color={BadgeColor::Red}>{"Item 3"}</Badge>
                                        </SpaceBetween>
                                    </Container>

                                    <Box variant={BoxVariant::P}>{"Horizontal spacing"}</Box>
                                    <Container>
                                        <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                            <Badge color={BadgeColor::Blue}>{"Item 1"}</Badge>
                                            <Badge color={BadgeColor::Green}>{"Item 2"}</Badge>
                                            <Badge color={BadgeColor::Red}>{"Item 3"}</Badge>
                                        </SpaceBetween>
                                    </Container>

                                    <Box variant={BoxVariant::P}>{"Size variants"}</Box>
                                    <Container>
                                        <SpaceBetween size={SpaceBetweenSize::Xxxs}>
                                            <Box variant={BoxVariant::P}>{"Size: xxxs"}</Box>
                                            <Box variant={BoxVariant::P}>{"Minimal spacing"}</Box>
                                        </SpaceBetween>
                                    </Container>
                                    <Container>
                                        <SpaceBetween size={SpaceBetweenSize::L}>
                                            <Box variant={BoxVariant::P}>{"Size: l"}</Box>
                                            <Box variant={BoxVariant::P}>{"Large spacing"}</Box>
                                        </SpaceBetween>
                                    </Container>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<SpaceBetween size={SpaceBetweenSize::M}>
    <div>{"Item 1"}</div>
    <div>{"Item 2"}</div>
</SpaceBetween>

<SpaceBetween
    size={SpaceBetweenSize::S}
    direction={SpaceBetweenDirection::Horizontal}
>
    <Badge>{"Badge 1"}</Badge>
    <Badge>{"Badge 2"}</Badge>
</SpaceBetween>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // ColumnLayout Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"ColumnLayout"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Multi-column Layout"}</div>
                            <div class="demo-example-description">{"Responsive grid layout with multiple columns"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::M}>
                                    <Box variant={BoxVariant::P}>{"Two columns"}</Box>
                                    <ColumnLayout columns={2}>
                                        <Container>
                                            <Box variant={BoxVariant::P}>{"Column 1"}</Box>
                                        </Container>
                                        <Container>
                                            <Box variant={BoxVariant::P}>{"Column 2"}</Box>
                                        </Container>
                                    </ColumnLayout>

                                    <Box variant={BoxVariant::P}>{"Three columns"}</Box>
                                    <ColumnLayout columns={3}>
                                        <div>
                                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                                <Box variant={BoxVariant::H3}>{"Feature 1"}</Box>
                                                <TextContent>
                                                    <p>{"Description for feature 1"}</p>
                                                </TextContent>
                                            </SpaceBetween>
                                        </div>
                                        <div>
                                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                                <Box variant={BoxVariant::H3}>{"Feature 2"}</Box>
                                                <TextContent>
                                                    <p>{"Description for feature 2"}</p>
                                                </TextContent>
                                            </SpaceBetween>
                                        </div>
                                        <div>
                                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                                <Box variant={BoxVariant::H3}>{"Feature 3"}</Box>
                                                <TextContent>
                                                    <p>{"Description for feature 3"}</p>
                                                </TextContent>
                                            </SpaceBetween>
                                        </div>
                                    </ColumnLayout>

                                    <Box variant={BoxVariant::P}>{"Four columns"}</Box>
                                    <ColumnLayout columns={4}>
                                        <Container><Badge color={BadgeColor::Blue}>{"Col 1"}</Badge></Container>
                                        <Container><Badge color={BadgeColor::Green}>{"Col 2"}</Badge></Container>
                                        <Container><Badge color={BadgeColor::Red}>{"Col 3"}</Badge></Container>
                                        <Container><Badge color={BadgeColor::Grey}>{"Col 4"}</Badge></Container>
                                    </ColumnLayout>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<ColumnLayout columns={3}>
    <div>{"Column 1"}</div>
    <div>{"Column 2"}</div>
    <div>{"Column 3"}</div>
</ColumnLayout>

<ColumnLayout
    columns={2}
    variant={ColumnVariant::TextGrid}
>
    <div>{"Content"}</div>
</ColumnLayout>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // ContentLayout Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"ContentLayout"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Content Wrapper"}</div>
                            <div class="demo-example-description">{"Page content wrapper with header and notifications"}</div>
                            <div class="demo-preview">
                                <Box variant={BoxVariant::P}>
                                    {"ContentLayout is used as the root wrapper for page content. "}
                                    {"This demo application uses ContentLayout for all pages. "}
                                    {"It provides consistent spacing and structure with slots for header and notifications."}
                                </Box>
                            </div>
                            <CodeSnippet code={r#"<ContentLayout>
    <div slot="header">
        <Header variant={HeaderVariant::H1}>
            {"Page Title"}
        </Header>
    </div>

    <SpaceBetween size={SpaceBetweenSize::L}>
        <Container>
            {"Content"}
        </Container>
    </SpaceBetween>
</ContentLayout>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // AppLayout Component (already in use)
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"AppLayout"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Application Shell"}</div>
                            <div class="demo-example-description">{"Main application layout with navigation, content, and tools"}</div>
                            <div class="demo-preview">
                                <Box variant={BoxVariant::P}>
                                    {"AppLayout is the main shell of this demo application. "}
                                    {"It provides the sidebar navigation, main content area, and optional tools panel. "}
                                    {"The layout automatically handles responsive behavior and navigation state."}
                                </Box>
                                <Alert alert_type={AlertType::Info}>
                                    {"You're currently viewing content inside an AppLayout component. "}
                                    {"The sidebar navigation on the left is part of the AppLayout structure."}
                                </Alert>
                            </div>
                            <CodeSnippet code={r#"<AppLayout
    navigation_open={true}
    navigation_width={250}
    on_navigation_change={on_nav_change}
    tools_open={false}
    on_tools_change={on_tools_change}
    content_type={ContentType::Default}
>
    <div slot="navigation">
        <SideNavigation ... />
    </div>

    <div slot="content">
        <ContentLayout>
            {"Page content"}
        </ContentLayout>
    </div>

    <div slot="tools">
        {"Help panel content"}
    </div>
</AppLayout>"#} />
                        </div>
                    </SpaceBetween>
                </Container>
            </SpaceBetween>
        </ContentLayout>
    }
}
