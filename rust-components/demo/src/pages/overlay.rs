// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use cloudscape_components::internal::ClickDetail;
use crate::components::code_snippet::CodeSnippet;

#[function_component(OverlayComponents)]
pub fn overlay_components() -> Html {
    let modal_visible = use_state(|| false);
    let drawer_visible = use_state(|| false);
    let popover_visible = use_state(|| false);

    let show_modal = {
        let modal_visible = modal_visible.clone();
        Callback::from(move |_| {
            modal_visible.set(true);
        })
    };

    let hide_modal = {
        let modal_visible = modal_visible.clone();
        Callback::from(move |_: CustomEvent<ModalDismissDetail>| {
            modal_visible.set(false);
        })
    };

    let hide_modal_button = {
        let modal_visible = modal_visible.clone();
        Callback::from(move |_: CustomEvent<ClickDetail>| {
            modal_visible.set(false);
        })
    };

    let show_drawer = {
        let drawer_visible = drawer_visible.clone();
        Callback::from(move |_| {
            drawer_visible.set(true);
        })
    };

    let hide_drawer = {
        let drawer_visible = drawer_visible.clone();
        Callback::from(move |_: CustomEvent<DrawerDismissDetail>| {
            drawer_visible.set(false);
        })
    };

    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Overlay Components"}
                    <div slot="description">
                        {"Components that overlay content on top of the page"}
                    </div>
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                // Modal Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Modal"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Dialog / Modal"}</div>
                            <div class="demo-example-description">{"Overlay dialog for focused interactions"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    <Button variant={ButtonVariant::Primary} on_click={show_modal.clone()}>
                                        {"Open Modal"}
                                    </Button>
                                </SpaceBetween>

                                if *modal_visible {
                                    <Modal
                                        visible={true}
                                        on_dismiss={hide_modal.clone()}
                                        header="Modal Title"
                                        size={ModalSize::Medium}
                                    >
                                        <div slot="footer">
                                            <SpaceBetween size={SpaceBetweenSize::Xs} direction={SpaceBetweenDirection::Horizontal}>
                                                <Button variant={ButtonVariant::Link} on_click={hide_modal_button.clone()}>
                                                    {"Cancel"}
                                                </Button>
                                                <Button variant={ButtonVariant::Primary}>
                                                    {"Confirm"}
                                                </Button>
                                            </SpaceBetween>
                                        </div>

                                        <SpaceBetween size={SpaceBetweenSize::M}>
                                            <TextContent>
                                                <p>{"This is the modal content. Modals are used for important interactions that require user attention."}</p>
                                                <p>{"They overlay the main content and prevent interaction with the page until dismissed."}</p>
                                            </TextContent>

                                            <FormField label="Example field">
                                                <Input placeholder="Enter some text" />
                                            </FormField>

                                            <Alert alert_type={AlertType::Info}>
                                                {"This is an informational message inside the modal."}
                                            </Alert>
                                        </SpaceBetween>
                                    </Modal>
                                }
                            </div>
                            <CodeSnippet code={r#"<Button on_click={show_modal}>
    {"Open Modal"}
</Button>

<Modal
    visible={visible}
    on_dismiss={on_dismiss}
    header="Modal Title"
    size={ModalSize::Medium}
>
    <div slot="footer">
        <SpaceBetween
            size={SpaceBetweenSize::Xs}
            direction={SpaceBetweenDirection::Horizontal}
        >
            <Button variant={ButtonVariant::Link}>
                {"Cancel"}
            </Button>
            <Button variant={ButtonVariant::Primary}>
                {"Confirm"}
            </Button>
        </SpaceBetween>
    </div>

    <div>{"Modal content"}</div>
</Modal>"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Modal Sizes"}</div>
                            <div class="demo-example-description">{"Different modal size variants"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::P}>
                                        {"Modals support different sizes: Small, Medium, Large, and Max"}
                                    </Box>
                                    <SpaceBetween size={SpaceBetweenSize::Xs} direction={SpaceBetweenDirection::Horizontal}>
                                        <Badge color={BadgeColor::Blue}>{"Small"}</Badge>
                                        <Badge color={BadgeColor::Blue}>{"Medium"}</Badge>
                                        <Badge color={BadgeColor::Blue}>{"Large"}</Badge>
                                        <Badge color={BadgeColor::Blue}>{"Max"}</Badge>
                                    </SpaceBetween>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Modal size={ModalSize::Small}>...</Modal>
<Modal size={ModalSize::Medium}>...</Modal>
<Modal size={ModalSize::Large}>...</Modal>
<Modal size={ModalSize::Max}>...</Modal>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Drawer Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Drawer"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Side Drawer Panel"}</div>
                            <div class="demo-example-description">{"Slides in from the edge for supplementary content"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    <Button variant={ButtonVariant::Normal} on_click={show_drawer.clone()}>
                                        {"Open Drawer"}
                                    </Button>
                                </SpaceBetween>

                                if *drawer_visible {
                                    <Drawer
                                        visible={true}
                                        on_dismiss={hide_drawer.clone()}
                                        header="Drawer Title"
                                        size={DrawerSize::Medium}
                                    >
                                        <SpaceBetween size={SpaceBetweenSize::M}>
                                            <TextContent>
                                                <h3>{"Drawer Content"}</h3>
                                                <p>{"Drawers slide in from the side of the screen and are useful for displaying detailed information or additional controls without leaving the current page."}</p>
                                            </TextContent>

                                            <Container>
                                                <div slot="header">
                                                    <Header variant={HeaderVariant::H3}>{"Details"}</Header>
                                                </div>
                                                <KeyValuePairs
                                                    columns={1}
                                                    items={vec![
                                                        KeyValuePair {
                                                            label: "Property 1".to_string(),
                                                            value: html! { <div>{"Value 1"}</div> },
                                                            info: None,
                                                        },
                                                        KeyValuePair {
                                                            label: "Property 2".to_string(),
                                                            value: html! { <div>{"Value 2"}</div> },
                                                            info: None,
                                                        },
                                                        KeyValuePair {
                                                            label: "Status".to_string(),
                                                            value: html! {
                                                                <StatusIndicator status_type={StatusIndicatorType::Success}>
                                                                    {"Active"}
                                                                </StatusIndicator>
                                                            },
                                                            info: None,
                                                        },
                                                    ]}
                                                />
                                            </Container>

                                            <ExpandableSection header_text={html!{"Additional Information"}}>
                                                <TextContent>
                                                    <p>{"More details can be placed inside expandable sections to keep the drawer organized."}</p>
                                                </TextContent>
                                            </ExpandableSection>
                                        </SpaceBetween>
                                    </Drawer>
                                }
                            </div>
                            <CodeSnippet code={r#"<Button on_click={show_drawer}>
    {"Open Drawer"}
</Button>

<Drawer
    visible={visible}
    on_dismiss={on_dismiss}
    header="Drawer Title"
    size={DrawerSize::Medium}
>
    <div>{"Drawer content"}</div>
</Drawer>"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Drawer Sizes"}</div>
                            <div class="demo-example-description">{"Different drawer width options"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::P}>
                                        {"Drawers support different sizes: Small, Medium, and Large"}
                                    </Box>
                                    <SpaceBetween size={SpaceBetweenSize::Xs} direction={SpaceBetweenDirection::Horizontal}>
                                        <Badge color={BadgeColor::Green}>{"Small"}</Badge>
                                        <Badge color={BadgeColor::Green}>{"Medium"}</Badge>
                                        <Badge color={BadgeColor::Green}>{"Large"}</Badge>
                                    </SpaceBetween>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Drawer size={DrawerSize::Small}>...</Drawer>
<Drawer size={DrawerSize::Medium}>...</Drawer>
<Drawer size={DrawerSize::Large}>...</Drawer>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Popover Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Popover"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Popover Overlay"}</div>
                            <div class="demo-example-description">{"Contextual content overlay with positioning"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::M}>
                                    <div style="display: flex; gap: 16px; align-items: center;">
                                        <span>{"Click for more information: "}</span>
                                        <Popover
                                            header="Popover Title"
                                            size={PopoverSize::Medium}
                                            position={PopoverPosition::Top}
                                            dismissible={true}
                                        >
                                            <div slot="trigger">
                                                <Button variant={ButtonVariant::Icon} icon={html!{<Icon name="status-info" />}} aria_label="Info" />
                                            </div>

                                            <SpaceBetween size={SpaceBetweenSize::S}>
                                                <TextContent>
                                                    <p>{"This is a popover with additional information. Popovers are useful for providing context-sensitive help or details."}</p>
                                                </TextContent>
                                                <Link href="#" external={true}>
                                                    {"Learn more"}
                                                </Link>
                                            </SpaceBetween>
                                        </Popover>
                                    </div>

                                    <div style="display: flex; gap: 16px; align-items: center;">
                                        <span>{"Hover for tooltip: "}</span>
                                        <Popover
                                            size={PopoverSize::Small}
                                            position={PopoverPosition::Right}
                                            trigger_type="hover"
                                        >
                                            <div slot="trigger">
                                                <Badge color={BadgeColor::Blue}>{"Hover me"}</Badge>
                                            </div>
                                            <div>{"This is a tooltip-style popover"}</div>
                                        </Popover>
                                    </div>

                                    <div>
                                        <Box variant={BoxVariant::P}>{"With form content"}</Box>
                                        <Popover
                                            header="Quick Edit"
                                            size={PopoverSize::Large}
                                            dismissible={true}
                                        >
                                            <div slot="trigger">
                                                <Button variant={ButtonVariant::Normal}>{"Edit"}</Button>
                                            </div>

                                            <SpaceBetween size={SpaceBetweenSize::S}>
                                                <FormField label="Name">
                                                    <Input placeholder="Enter name" />
                                                </FormField>
                                                <FormField label="Description">
                                                    <Textarea placeholder="Enter description" rows={3} />
                                                </FormField>
                                                <SpaceBetween size={SpaceBetweenSize::Xs} direction={SpaceBetweenDirection::Horizontal}>
                                                    <Button variant={ButtonVariant::Link}>{"Cancel"}</Button>
                                                    <Button variant={ButtonVariant::Primary}>{"Save"}</Button>
                                                </SpaceBetween>
                                            </SpaceBetween>
                                        </Popover>
                                    </div>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Popover
    header="Popover Title"
    size={PopoverSize::Medium}
    position={PopoverPosition::Top}
    dismissible={true}
>
    <div slot="trigger">
        <Button icon_name="status-info" />
    </div>

    <div>{"Popover content"}</div>
</Popover>

// Tooltip-style popover
<Popover
    size={PopoverSize::Small}
    trigger_type="hover"
>
    <div slot="trigger">
        <span>{"Hover me"}</span>
    </div>
    <div>{"Tooltip text"}</div>
</Popover>"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Popover Positions"}</div>
                            <div class="demo-example-description">{"Control where the popover appears"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::P}>
                                        {"Popovers can be positioned relative to their trigger: Top, Bottom, Left, Right"}
                                    </Box>
                                    <ColumnLayout columns={4}>
                                        <Badge color={BadgeColor::Grey}>{"Top"}</Badge>
                                        <Badge color={BadgeColor::Grey}>{"Bottom"}</Badge>
                                        <Badge color={BadgeColor::Grey}>{"Left"}</Badge>
                                        <Badge color={BadgeColor::Grey}>{"Right"}</Badge>
                                    </ColumnLayout>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Popover position={PopoverPosition::Top}>...</Popover>
<Popover position={PopoverPosition::Bottom}>...</Popover>
<Popover position={PopoverPosition::Left}>...</Popover>
<Popover position={PopoverPosition::Right}>...</Popover>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Usage Guidelines
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Usage Guidelines"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <ColumnLayout columns={3}>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::Xs}>
                                    <Box variant={BoxVariant::H3}>{"Modal"}</Box>
                                    <TextContent>
                                        <p><strong>{"Use for:"}</strong></p>
                                        <ul>
                                            <li>{"Critical actions"}</li>
                                            <li>{"Confirmations"}</li>
                                            <li>{"Complex forms"}</li>
                                            <li>{"Blocking interactions"}</li>
                                        </ul>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::Xs}>
                                    <Box variant={BoxVariant::H3}>{"Drawer"}</Box>
                                    <TextContent>
                                        <p><strong>{"Use for:"}</strong></p>
                                        <ul>
                                            <li>{"Detailed views"}</li>
                                            <li>{"Secondary content"}</li>
                                            <li>{"Non-blocking info"}</li>
                                            <li>{"Multi-step workflows"}</li>
                                        </ul>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::Xs}>
                                    <Box variant={BoxVariant::H3}>{"Popover"}</Box>
                                    <TextContent>
                                        <p><strong>{"Use for:"}</strong></p>
                                        <ul>
                                            <li>{"Contextual help"}</li>
                                            <li>{"Quick actions"}</li>
                                            <li>{"Tooltips"}</li>
                                            <li>{"Brief information"}</li>
                                        </ul>
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
