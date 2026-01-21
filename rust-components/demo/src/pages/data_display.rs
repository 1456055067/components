// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use crate::components::code_snippet::CodeSnippet;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct DemoItem {
    id: String,
    name: String,
    status: String,
    value: i32,
}

// Helper functions for table cells
fn render_id_cell(item: &DemoItem) -> Html {
    html! { <div>{&item.id}</div> }
}

fn render_name_cell(item: &DemoItem) -> Html {
    html! { <div>{&item.name}</div> }
}

fn render_status_cell(item: &DemoItem) -> Html {
    let indicator_type = match item.status.as_str() {
        "Active" => StatusIndicatorType::Success,
        "Pending" => StatusIndicatorType::Pending,
        _ => StatusIndicatorType::Stopped,
    };
    html! {
        <StatusIndicator status_type={indicator_type}>
            <div>{&item.status}</div>
        </StatusIndicator>
    }
}

fn render_value_cell(item: &DemoItem) -> Html {
    html! { <div>{item.value}</div> }
}

// Helper functions for card rendering
fn render_card_header(item: &serde_json::Value) -> Html {
    let name = item["name"].as_str().unwrap_or("");
    html! {
        <Link href="#" variant={LinkVariant::Primary}>
            <div>{name}</div>
        </Link>
    }
}

fn render_card_section(item: &serde_json::Value) -> Html {
    let status = item["status"].as_str().unwrap_or("Unknown");
    let indicator_type = match status {
        "Active" => StatusIndicatorType::Success,
        "Pending" => StatusIndicatorType::Pending,
        _ => StatusIndicatorType::Stopped,
    };
    html! {
        <SpaceBetween size={SpaceBetweenSize::Xs}>
            <div>
                <Box variant={BoxVariant::AwsuiKeyLabel}>{"Status"}</Box>
                <StatusIndicator status_type={indicator_type}>
                    <div>{status}</div>
                </StatusIndicator>
            </div>
            <div>
                <Box variant={BoxVariant::AwsuiKeyLabel}>{"Value"}</Box>
                <div>{item["value"].as_i64().unwrap_or(0)}</div>
            </div>
        </SpaceBetween>
    }
}

#[function_component(DataDisplayComponents)]
pub fn data_display_components() -> Html {
    let expanded = use_state(|| false);
    let selected_items: UseStateHandle<Vec<DemoItem>> = use_state(|| vec![]);

    let on_toggle_expand = {
        let expanded = expanded.clone();
        Callback::from(move |event: CustomEvent<ExpandableSectionChangeDetail>| {
            expanded.set(event.detail.expanded);
        })
    };

    let items = vec![
        DemoItem {
            id: "1".to_string(),
            name: "Item 1".to_string(),
            status: "Active".to_string(),
            value: 100,
        },
        DemoItem {
            id: "2".to_string(),
            name: "Item 2".to_string(),
            status: "Pending".to_string(),
            value: 75,
        },
        DemoItem {
            id: "3".to_string(),
            name: "Item 3".to_string(),
            status: "Inactive".to_string(),
            value: 50,
        },
    ];

    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Data Display Components"}
                    <div slot="description">
                        {"Components for displaying structured data and information"}
                    </div>
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                // StatusIndicator Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"StatusIndicator"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Status Badges"}</div>
                            <div class="demo-example-description">{"Visual status indicators with colors"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <StatusIndicator status_type={StatusIndicatorType::Success}>
                                        {"Success - Operation completed"}
                                    </StatusIndicator>
                                    <StatusIndicator status_type={StatusIndicatorType::Error}>
                                        {"Error - Operation failed"}
                                    </StatusIndicator>
                                    <StatusIndicator status_type={StatusIndicatorType::Warning}>
                                        {"Warning - Requires attention"}
                                    </StatusIndicator>
                                    <StatusIndicator status_type={StatusIndicatorType::Info}>
                                        {"Info - Additional information"}
                                    </StatusIndicator>
                                    <StatusIndicator status_type={StatusIndicatorType::Pending}>
                                        {"Pending - In progress"}
                                    </StatusIndicator>
                                    <StatusIndicator status_type={StatusIndicatorType::InProgress}>
                                        {"In Progress - Processing"}
                                    </StatusIndicator>
                                    <StatusIndicator status_type={StatusIndicatorType::Stopped}>
                                        {"Stopped - Not running"}
                                    </StatusIndicator>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<StatusIndicator status_type={StatusIndicatorType::Success}>
    {"Success"}
</StatusIndicator>
<StatusIndicator status_type={StatusIndicatorType::Error}>
    {"Error"}
</StatusIndicator>
<StatusIndicator status_type={StatusIndicatorType::Warning}>
    {"Warning"}
</StatusIndicator>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // KeyValuePairs Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"KeyValuePairs"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Key-Value Display"}</div>
                            <div class="demo-example-description">{"Structured display of key-value pairs"}</div>
                            <div class="demo-preview">
                                <KeyValuePairs
                                    columns={2}
                                    items={vec![
                                        KeyValuePair {
                                            label: "Instance ID".to_string(),
                                            value: html! { <div>{"i-1234567890abcdef0"}</div> },
                                            info: None,
                                        },
                                        KeyValuePair {
                                            label: "Instance Type".to_string(),
                                            value: html! { <div>{"t3.micro"}</div> },
                                            info: None,
                                        },
                                        KeyValuePair {
                                            label: "Availability Zone".to_string(),
                                            value: html! { <div>{"us-east-1a"}</div> },
                                            info: None,
                                        },
                                        KeyValuePair {
                                            label: "State".to_string(),
                                            value: html! {
                                                <StatusIndicator status_type={StatusIndicatorType::Success}>
                                                    {"Running"}
                                                </StatusIndicator>
                                            },
                                            info: None,
                                        },
                                        KeyValuePair {
                                            label: "Launch Time".to_string(),
                                            value: html! { <div>{"2024-01-15 10:30:00"}</div> },
                                            info: None,
                                        },
                                        KeyValuePair {
                                            label: "Public IP".to_string(),
                                            value: html! {
                                                <div style="display: flex; align-items: center; gap: 8px;">
                                                    <code>{"54.123.45.67"}</code>
                                                    <CopyToClipboard
                                                        copy_text="54.123.45.67"
                                                        variant={CopyToClipboardVariant::Icon}
                                                    />
                                                </div>
                                            },
                                            info: None,
                                        },
                                    ]}
                                />
                            </div>
                            <CodeSnippet code={r#"<KeyValuePairs
    columns={2}
    items={vec![
        KeyValuePair {
            label: "Name".to_string(),
            value: html! { <div>{"Value"}</div> },
        },
        KeyValuePair {
            label: "Status".to_string(),
            value: html! {
                <StatusIndicator status_type={StatusIndicatorType::Success}>
                    {"Active"}
                </StatusIndicator>
            },
        },
    ]}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // ExpandableSection Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"ExpandableSection"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Collapsible Content"}</div>
                            <div class="demo-example-description">
                                {format!("Expand/collapse content sections (expanded: {})", *expanded)}
                            </div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <ExpandableSection
                                        header_text={html!{"Advanced Settings"}}
                                        expanded={*expanded}
                                        on_change={on_toggle_expand}
                                    >
                                        <SpaceBetween size={SpaceBetweenSize::S}>
                                            <FormField label="Option 1">
                                                <Input placeholder="Enter value" />
                                            </FormField>
                                            <FormField label="Option 2">
                                                <Select
                                                    selected_option={SelectOption::new("")}
                                                    options={vec![]}
                                                    placeholder="Choose option"
                                                />
                                            </FormField>
                                            <Checkbox checked={false}>
                                                {"Enable advanced mode"}
                                            </Checkbox>
                                        </SpaceBetween>
                                    </ExpandableSection>

                                    <ExpandableSection
                                        header_text={html!{"Additional Information"}}
                                        variant={ExpandableSectionVariant::Footer}
                                        expanded={false}
                                    >
                                        <TextContent>
                                            <p>{"This section contains additional details and supplementary information."}</p>
                                        </TextContent>
                                    </ExpandableSection>

                                    <ExpandableSection
                                        header_text={html!{"Expanded by Default"}}
                                        expanded={true}
                                    >
                                        <Box variant={BoxVariant::P}>
                                            {"This section is expanded by default."}
                                        </Box>
                                    </ExpandableSection>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<ExpandableSection
    header_text="Advanced Settings"
    expanded={expanded}
    on_change={on_change}
>
    <div>{"Content inside section"}</div>
</ExpandableSection>

<ExpandableSection
    header_text="Footer Section"
    variant={ExpandableSectionVariant::Footer}
>
    <div>{"Footer content"}</div>
</ExpandableSection>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Table Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Table"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Data Table"}</div>
                            <div class="demo-example-description">{"Display tabular data with sorting and selection"}</div>
                            <div class="demo-preview">
                                <Table<DemoItem>
                                    columns={vec![
                                        TableColumn::new("id", "ID", render_id_cell)
                                            .with_sortable(true),
                                        TableColumn::new("name", "Name", render_name_cell)
                                            .with_sortable(true),
                                        TableColumn::new("status", "Status", render_status_cell)
                                            .with_sortable(true),
                                        TableColumn::new("value", "Value", render_value_cell)
                                            .with_sortable(true),
                                    ]}
                                    items={items.clone()}
                                    loading={false}
                                    selection_type={SelectionType::Multi}
                                    selected_items={vec![]}
                                />
                            </div>
                            <CodeSnippet code={r#"<Table
    columns={vec![
        TableColumn {
            id: "name".to_string(),
            header: "Name".to_string(),
            cell: Box::new(|item| {
                html! { <div>{item["name"]}</div> }
            }),
            sortable: true,
            ..Default::default()
        },
    ]}
    items={items}
    loading={false}
    selection_type={SelectionType::Multi}
    selected_items={selected}
    on_selection_change={on_change}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Cards Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Cards"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Card Grid"}</div>
                            <div class="demo-example-description">{"Display items in a card grid layout"}</div>
                            <div class="demo-preview">
                                <Cards<serde_json::Value>
                                    card_definition={CardDefinition {
                                        header: render_card_header,
                                        sections: vec![render_card_section],
                                    }}
                                    items={items.iter().map(|item| serde_json::to_value(item).unwrap()).collect::<Vec<_>>()}
                                    cards_per_row={vec![3]}
                                    selection_type={CardsSelectionType::Multi}
                                    selected_items={vec![]}
                                />
                            </div>
                            <CodeSnippet code={r##"<Cards
    card_definition={CardDefinition {
        header: Box::new(|item| {
            html! { <Link href="#">{item["name"]}</Link> }
        }),
        sections: vec![
            Box::new(|item| {
                html! { <div>{item["description"]}</div> }
            }),
        ],
    }}
    items={items}
    cards_per_row={3}
    selection_type={CardsSelectionType::Multi}
    selected_items={selected}
/>"##} />
                        </div>
                    </SpaceBetween>
                </Container>

                // TokenGroup Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"TokenGroup"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Token Display"}</div>
                            <div class="demo-example-description">{"Display dismissible tokens/tags"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::M}>
                                    <div>
                                        <Box variant={BoxVariant::P}>{"Tags"}</Box>
                                        <TokenGroup
                                            items={vec![
                                                Token::new("tag1", "Production").with_dismissible(true),
                                                Token::new("tag2", "us-east-1").with_dismissible(true),
                                                Token::new("tag3", "Active").with_dismissible(false),
                                            ]}
                                        />
                                    </div>

                                    <div>
                                        <Box variant={BoxVariant::P}>{"Filters with icons"}</Box>
                                        <TokenGroup
                                            items={vec![
                                                Token::new("filter1", "Filter 1")
                                                    .with_dismissible(true),
                                                Token::new("filter2", "Filter 2")
                                                    .with_dismissible(true),
                                            ]}
                                        />
                                    </div>

                                    <div>
                                        <Box variant={BoxVariant::P}>{"With limit"}</Box>
                                        <TokenGroup
                                            items={vec![
                                                Token::new("token1", "Token 1").with_dismissible(true),
                                                Token::new("token2", "Token 2").with_dismissible(true),
                                                Token::new("token3", "Token 3").with_dismissible(true),
                                                Token::new("token4", "Token 4").with_dismissible(true),
                                                Token::new("token5", "Token 5").with_dismissible(true),
                                            ]}
                                            limit={3}
                                        />
                                    </div>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<TokenGroup
    items={vec![
        Token {
            label: "Tag 1".to_string(),
            dismissible: true,
            ..Default::default()
        },
        Token {
            label: "Tag 2".to_string(),
            dismissible: true,
            icon_name: Some("search".to_string()),
            ..Default::default()
        },
    ]}
    on_dismiss={on_dismiss}
    limit={5}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>
            </SpaceBetween>
        </ContentLayout>
    }
}
