// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use crate::components::code_snippet::CodeSnippet;

#[function_component(NotificationComponents)]
pub fn notification_components() -> Html {
    let flash_items = use_state(|| vec![
        FlashbarItem {
            id: Some("1".to_string()),
            flash_type: FlashbarType::Success,
            header: Some("Success".to_string()),
            content: html! { "Your changes have been saved successfully." },
            dismissible: true,
            loading: false,
            action: None,
            button_text: None,
            on_button_click: None,
        },
        FlashbarItem {
            id: Some("2".to_string()),
            flash_type: FlashbarType::Info,
            header: Some("Information".to_string()),
            content: html! { "New features are available. Check the release notes." },
            dismissible: true,
            loading: false,
            action: Some(html! {
                <Button variant={ButtonVariant::Link}>{"View details"}</Button>
            }),
            button_text: None,
            on_button_click: None,
        },
    ]);

    let add_success_message = {
        let flash_items = flash_items.clone();
        Callback::from(move |_| {
            let mut items = (*flash_items).clone();
            items.push(FlashbarItem {
                id: Some(format!("success-{}", items.len() + 1)),
                flash_type: FlashbarType::Success,
                header: Some("Success".to_string()),
                content: html!{format!("Operation {} completed successfully", items.len() + 1)},
                dismissible: true,
                loading: false,
                action: None,
                button_text: None,
                on_button_click: None,
            });
            flash_items.set(items);
        })
    };

    let add_error_message = {
        let flash_items = flash_items.clone();
        Callback::from(move |_| {
            let mut items = (*flash_items).clone();
            items.push(FlashbarItem {
                id: Some(format!("error-{}", items.len() + 1)),
                flash_type: FlashbarType::Error,
                header: Some("Error".to_string()),
                content: html! { "An error occurred while processing your request." },
                dismissible: true,
                loading: false,
                action: Some(html! {
                    <Button variant={ButtonVariant::Link}>{"Retry"}</Button>
                }),
                button_text: None,
                on_button_click: None,
            });
            flash_items.set(items);
        })
    };

    let add_warning_message = {
        let flash_items = flash_items.clone();
        Callback::from(move |_| {
            let mut items = (*flash_items).clone();
            items.push(FlashbarItem {
                id: Some(format!("warning-{}", items.len() + 1)),
                flash_type: FlashbarType::Warning,
                header: Some("Warning".to_string()),
                content: html! { "This action may have unintended consequences." },
                dismissible: true,
                loading: false,
                action: None,
                button_text: None,
                on_button_click: None,
            });
            flash_items.set(items);
        })
    };

    let add_loading_message = {
        let flash_items = flash_items.clone();
        Callback::from(move |_| {
            let mut items = (*flash_items).clone();
            items.push(FlashbarItem {
                id: Some(format!("loading-{}", items.len() + 1)),
                flash_type: FlashbarType::Info,
                header: Some("Processing".to_string()),
                content: html! { "Please wait while we process your request..." },
                dismissible: false,
                loading: true,
                action: None,
                button_text: None,
                on_button_click: None,
            });
            flash_items.set(items);
        })
    };

    let clear_messages = {
        let flash_items = flash_items.clone();
        Callback::from(move |_| {
            flash_items.set(vec![]);
        })
    };

    let on_dismiss = {
        let flash_items = flash_items.clone();
        Callback::from(move |event: CustomEvent<FlashbarDismissDetail>| {
            let items: Vec<FlashbarItem> = (*flash_items)
                .iter()
                .filter(|item| item.id.as_ref() != event.detail.item_id.as_ref())
                .cloned()
                .collect();
            flash_items.set(items);
        })
    };

    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Notification Components"}
                    <div slot="description">
                        {"Components for displaying notifications and flash messages"}
                    </div>
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                // Live Flashbar Demo
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>
                            {"Interactive Flashbar Demo"}
                            <div slot="description">
                                {format!("Currently showing {} message(s)", flash_items.len())}
                            </div>
                        </Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Add Messages"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    <Button
                                        variant={ButtonVariant::Normal}
                                        on_click={add_success_message}
                                    >
                                        {"Add Success"}
                                    </Button>
                                    <Button
                                        variant={ButtonVariant::Normal}
                                        on_click={add_error_message}
                                    >
                                        {"Add Error"}
                                    </Button>
                                    <Button
                                        variant={ButtonVariant::Normal}
                                        on_click={add_warning_message}
                                    >
                                        {"Add Warning"}
                                    </Button>
                                    <Button
                                        variant={ButtonVariant::Normal}
                                        on_click={add_loading_message}
                                    >
                                        {"Add Loading"}
                                    </Button>
                                    <Button
                                        variant={ButtonVariant::Normal}
                                        on_click={clear_messages}
                                        disabled={flash_items.is_empty()}
                                    >
                                        {"Clear All"}
                                    </Button>
                                </SpaceBetween>
                            </div>
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Active Messages"}</div>
                            <div class="demo-preview">
                                if flash_items.is_empty() {
                                    <Alert alert_type={AlertType::Info}>
                                        {"No messages to display. Click the buttons above to add notifications."}
                                    </Alert>
                                } else {
                                    <Flashbar
                                        items={(*flash_items).clone()}
                                        on_item_dismiss={on_dismiss}
                                    />
                                }
                            </div>
                        </div>
                    </SpaceBetween>
                </Container>

                // Flashbar Component Documentation
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Flashbar"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Flash Message Container"}</div>
                            <div class="demo-example-description">
                                {"Display multiple flash messages in a stacked layout"}
                            </div>
                            <div class="demo-preview">
                                <TextContent>
                                    <p>
                                        {"The Flashbar component displays multiple notification messages in a vertical stack. "}
                                        {"It's typically placed at the top of the page content or within a specific container."}
                                    </p>
                                </TextContent>
                            </div>
                            <CodeSnippet code={r#"<Flashbar
    items={vec![
        FlashbarItem {
            id: Some("1".to_string()),
            flash_type: FlashbarType::Success,
            header: Some("Success".to_string()),
            content: html! { "Operation completed" },
            dismissible: true,
            loading: false,
            action: None,
            button_text: None,
            on_button_click: None,
        },
    ]}
    on_item_dismiss={on_dismiss}
/>"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Message Types"}</div>
                            <div class="demo-example-description">{"Different flash message types and their use cases"}</div>
                            <div class="demo-preview">
                                <Flashbar
                                    items={vec![
                                        FlashbarItem {
                                            id: Some("type-success".to_string()),
                                            flash_type: FlashbarType::Success,
                                            header: Some("Success".to_string()),
                                            content: html! { "Use for successful operations and confirmations." },
                                            dismissible: true,
                                            loading: false,
                                            action: None,
                                            button_text: None,
                                            on_button_click: None,
                                        },
                                        FlashbarItem {
                                            id: Some("type-info".to_string()),
                                            flash_type: FlashbarType::Info,
                                            header: Some("Information".to_string()),
                                            content: html! { "Use for general information and updates." },
                                            dismissible: true,
                                            loading: false,
                                            action: None,
                                            button_text: None,
                                            on_button_click: None,
                                        },
                                        FlashbarItem {
                                            id: Some("type-warning".to_string()),
                                            flash_type: FlashbarType::Warning,
                                            header: Some("Warning".to_string()),
                                            content: html! { "Use for warnings and potential issues." },
                                            dismissible: true,
                                            loading: false,
                                            action: None,
                                            button_text: None,
                                            on_button_click: None,
                                        },
                                        FlashbarItem {
                                            id: Some("type-error".to_string()),
                                            flash_type: FlashbarType::Error,
                                            header: Some("Error".to_string()),
                                            content: html! { "Use for errors and failures that require attention." },
                                            dismissible: true,
                                            loading: false,
                                            action: None,
                                            button_text: None,
                                            on_button_click: None,
                                        },
                                    ]}
                                    on_item_dismiss={Callback::noop()}
                                />
                            </div>
                            <CodeSnippet code={r#"// Success message
FlashbarItem {
    flash_type: FlashbarType::Success,
    header: Some("Success".to_string()),
    content: html! { "Operation completed" },
    ..Default::default()
}

// Error message
FlashbarItem {
    flash_type: FlashbarType::Error,
    header: Some("Error".to_string()),
    content: html! { "Something went wrong" },
    ..Default::default()
}

// Warning message
FlashbarItem {
    flash_type: FlashbarType::Warning,
    header: Some("Warning".to_string()),
    content: html! { "Be careful" },
    ..Default::default()
}

// Info message
FlashbarItem {
    flash_type: FlashbarType::Info,
    header: Some("Info".to_string()),
    content: html! { "FYI" },
    ..Default::default()
}"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Messages with Actions"}</div>
                            <div class="demo-example-description">{"Flash messages can include action buttons"}</div>
                            <div class="demo-preview">
                                <Flashbar
                                    items={vec![
                                        FlashbarItem {
                                            id: Some("action-1".to_string()),
                                            flash_type: FlashbarType::Info,
                                            header: Some("Update Available".to_string()),
                                            content: html! { "A new version is available for download." },
                                            dismissible: true,
                                            loading: false,
                                            action: Some(html! {
                                                <Button variant={ButtonVariant::Primary}>
                                                    {"Update Now"}
                                                </Button>
                                            }),
                                            button_text: None,
                                            on_button_click: None,
                                        },
                                        FlashbarItem {
                                            id: Some("action-2".to_string()),
                                            flash_type: FlashbarType::Warning,
                                            header: Some("Backup Recommended".to_string()),
                                            content: html! { "Your data hasn't been backed up in 30 days." },
                                            dismissible: true,
                                            loading: false,
                                            action: Some(html! {
                                                <SpaceBetween size={SpaceBetweenSize::Xs} direction={SpaceBetweenDirection::Horizontal}>
                                                    <Button variant={ButtonVariant::Link}>
                                                        {"Remind me later"}
                                                    </Button>
                                                    <Button variant={ButtonVariant::Primary}>
                                                        {"Backup now"}
                                                    </Button>
                                                </SpaceBetween>
                                            }),
                                            button_text: None,
                                            on_button_click: None,
                                        },
                                    ]}
                                    on_item_dismiss={Callback::noop()}
                                />
                            </div>
                            <CodeSnippet code={r#"FlashbarItem {
    flash_type: FlashbarType::Info,
    header: Some("Update Available".to_string()),
    content: html! { "A new version is ready" },
    dismissible: true,
    action: Some(html! {
        <Button variant={ButtonVariant::Primary}>
            {"Update Now"}
        </Button>
    }),
    ..Default::default()
}"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Loading State"}</div>
                            <div class="demo-example-description">{"Show progress with loading indicator"}</div>
                            <div class="demo-preview">
                                <Flashbar
                                    items={vec![
                                        FlashbarItem {
                                            id: Some("loading-1".to_string()),
                                            flash_type: FlashbarType::Info,
                                            header: Some("Processing Request".to_string()),
                                            content: html! { "Please wait while we process your request..." },
                                            dismissible: false,
                                            loading: true,
                                            action: None,
                                            button_text: None,
                                            on_button_click: None,
                                        },
                                    ]}
                                    on_item_dismiss={Callback::noop()}
                                />
                            </div>
                            <CodeSnippet code={r#"FlashbarItem {
    flash_type: FlashbarType::Info,
    header: Some("Processing".to_string()),
    content: html! { "Please wait..." },
    dismissible: false,
    loading: true,
    ..Default::default()
}"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Best Practices
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Best Practices"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <ColumnLayout columns={2}>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::Xs}>
                                    <Box variant={BoxVariant::H3}>{"Do"}</Box>
                                    <TextContent>
                                        <ul>
                                            <li>{"Use appropriate message types for context"}</li>
                                            <li>{"Keep messages concise and actionable"}</li>
                                            <li>{"Provide clear actions when needed"}</li>
                                            <li>{"Allow users to dismiss non-critical messages"}</li>
                                            <li>{"Stack related messages together"}</li>
                                            <li>{"Use loading state for long operations"}</li>
                                        </ul>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                            <div>
                                <SpaceBetween size={SpaceBetweenSize::Xs}>
                                    <Box variant={BoxVariant::H3}>{"Don't"}</Box>
                                    <TextContent>
                                        <ul>
                                            <li>{"Don't show too many messages at once"}</li>
                                            <li>{"Don't use error messages for warnings"}</li>
                                            <li>{"Don't make critical errors dismissible"}</li>
                                            <li>{"Don't use flashbar for permanent content"}</li>
                                            <li>{"Don't overuse success messages"}</li>
                                            <li>{"Don't hide important actions in messages"}</li>
                                        </ul>
                                    </TextContent>
                                </SpaceBetween>
                            </div>
                        </ColumnLayout>

                        <Alert alert_type={AlertType::Info}>
                            {"Flashbar is typically placed at the top of the ContentLayout to ensure messages are immediately visible to users."}
                        </Alert>
                    </SpaceBetween>
                </Container>

                // Integration Example
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Integration Example"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <TextContent>
                            <p>{"Here's how to integrate Flashbar in your application:"}</p>
                        </TextContent>
                        <CodeSnippet code={r#"use yew::prelude::*;
use cloudscape_components::*;

#[function_component(MyPage)]
fn my_page() -> Html {
    let flash_items = use_state(|| vec![]);

    let add_message = {
        let flash_items = flash_items.clone();
        Callback::from(move |_| {
            let mut items = (*flash_items).clone();
            items.push(FlashbarItem {
                id: Some("msg-1".to_string()),
                flash_type: FlashbarType::Success,
                header: Some("Success".to_string()),
                content: html! { "Changes saved" },
                dismissible: true,
                loading: false,
                action: None,
                button_text: None,
                on_button_click: None,
            });
            flash_items.set(items);
        })
    };

    let on_dismiss = {
        let flash_items = flash_items.clone();
        Callback::from(move |event: CustomEvent<FlashbarDismissDetail>| {
            let items: Vec<FlashbarItem> = (*flash_items)
                .iter()
                .filter(|item| item.id.as_ref() != event.detail.item_id.as_ref())
                .cloned()
                .collect();
            flash_items.set(items);
        })
    };

    html! {
        <ContentLayout>
            <div slot="notifications">
                <Flashbar
                    items={(*flash_items).clone()}
                    on_item_dismiss={on_dismiss}
                />
            </div>

            <Container>
                <Button on_click={add_message}>
                    {"Trigger Notification"}
                </Button>
            </Container>
        </ContentLayout>
    }
}"#} language="Rust" />
                    </SpaceBetween>
                </Container>
            </SpaceBetween>
        </ContentLayout>
    }
}
