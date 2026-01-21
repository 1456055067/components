// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Cloudscape Components Demo"}
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                <Container>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div>
                            <Header variant={HeaderVariant::H2}>
                                {"Welcome to the Cloudscape Rust/Yew Component Gallery"}
                            </Header>
                        </div>
                        <TextContent>
                            <p>
                                {"This interactive demo showcases all 44 implemented Cloudscape Design System components built with Rust and Yew for WebAssembly."}
                            </p>
                            <p>
                                {"These components provide the same look and feel as the React implementation, but with the performance and security benefits of Rust/WASM."}
                            </p>
                        </TextContent>
                    </SpaceBetween>
                </Container>

                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>
                            {"Component Categories"}
                        </Header>
                    </div>
                    <ColumnLayout columns={2} variant={ColumnVariant::TextGrid}>
                        <div>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::H3}>
                                    {"Basic Components (11)"}
                                </Box>
                                <TextContent>
                                    <p>{"Badge, Spinner, Box, Button, ButtonDropdown, Alert, Link, Icon, ProgressBar, CopyToClipboard, TextContent"}</p>
                                </TextContent>
                            </SpaceBetween>
                        </div>

                        <div>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::H3}>
                                    {"Form Components (13)"}
                                </Box>
                                <TextContent>
                                    <p>{"Input, Checkbox, Toggle, RadioGroup, Tiles, Select, Textarea, FormField, Multiselect, Autosuggest, DatePicker, DateRangePicker, FileUpload"}</p>
                                </TextContent>
                            </SpaceBetween>
                        </div>

                        <div>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::H3}>
                                    {"Layout Components (6)"}
                                </Box>
                                <TextContent>
                                    <p>{"Container, Header, AppLayout, ContentLayout, ColumnLayout, SpaceBetween"}</p>
                                </TextContent>
                            </SpaceBetween>
                        </div>

                        <div>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::H3}>
                                    {"Navigation Components (5)"}
                                </Box>
                                <TextContent>
                                    <p>{"Tabs, SideNavigation, Breadcrumbs, Pagination, TopNavigation"}</p>
                                </TextContent>
                            </SpaceBetween>
                        </div>

                        <div>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::H3}>
                                    {"Data Display Components (6)"}
                                </Box>
                                <TextContent>
                                    <p>{"StatusIndicator, KeyValuePairs, ExpandableSection, Table, Cards, TokenGroup"}</p>
                                </TextContent>
                            </SpaceBetween>
                        </div>

                        <div>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::H3}>
                                    {"Overlay Components (3)"}
                                </Box>
                                <TextContent>
                                    <p>{"Modal, Popover, Drawer"}</p>
                                </TextContent>
                            </SpaceBetween>
                        </div>

                        <div>
                            <SpaceBetween size={SpaceBetweenSize::Xs}>
                                <Box variant={BoxVariant::H3}>
                                    {"Notification Components (1)"}
                                </Box>
                                <TextContent>
                                    <p>{"Flashbar"}</p>
                                </TextContent>
                            </SpaceBetween>
                        </div>
                    </ColumnLayout>
                </Container>

                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>
                            {"Getting Started"}
                        </Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <TextContent>
                            <p>{"Use the sidebar navigation to explore different component categories. Each page contains:"}</p>
                            <ul>
                                <li>{"Live, interactive component examples"}</li>
                                <li>{"Code snippets showing usage"}</li>
                                <li>{"Configuration options and variants"}</li>
                            </ul>
                        </TextContent>

                        <Alert alert_type={AlertType::Info}>
                            {"All components are fully functional and can be interacted with directly in this demo."}
                        </Alert>
                    </SpaceBetween>
                </Container>

                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>
                            {"Key Features"}
                        </Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <ColumnLayout columns={2}>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::AwsuiKeyLabel}>
                                        {"Performance"}
                                    </Box>
                                    <TextContent>
                                        <p>{"Zero-cost abstractions and WASM compilation for fast runtime performance"}</p>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::AwsuiKeyLabel}>
                                        {"Security"}
                                    </Box>
                                    <TextContent>
                                        <p>{"Memory safety guarantees from Rust reduce vulnerabilities"}</p>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::AwsuiKeyLabel}>
                                        {"Compatibility"}
                                    </Box>
                                    <TextContent>
                                        <p>{"Shares design tokens and styles with the React implementation"}</p>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::AwsuiKeyLabel}>
                                        {"Type Safety"}
                                    </Box>
                                    <TextContent>
                                        <p>{"Strong typing catches errors at compile time"}</p>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                        </ColumnLayout>
                    </SpaceBetween>
                </Container>
            </SpaceBetween>
        </ContentLayout>
    }
}
