// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use crate::components::code_snippet::CodeSnippet;

#[function_component(BasicComponents)]
pub fn basic_components() -> Html {
    let button_clicks = use_state(|| 0);
    let spinner_visible = use_state(|| true);
    let alert_visible = use_state(|| true);

    let on_button_click = {
        let button_clicks = button_clicks.clone();
        Callback::from(move |_| {
            button_clicks.set(*button_clicks + 1);
        })
    };

    let on_alert_dismiss = {
        let alert_visible = alert_visible.clone();
        Callback::from(move |_: CustomEvent<DismissDetail>| {
            alert_visible.set(false);
        })
    };

    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Basic Components"}
                    <div slot="info">
                        <Link variant={LinkVariant::Info}>{"Info"}</Link>
                    </div>
                    <div slot="description">
                        {"Core components for building user interfaces"}
                    </div>
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                // Badge Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Badge"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Badge Colors"}</div>
                            <div class="demo-example-description">{"Small visual indicators for labels and metadata"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    <Badge color={BadgeColor::Blue}>{"Blue"}</Badge>
                                    <Badge color={BadgeColor::Grey}>{"Grey"}</Badge>
                                    <Badge color={BadgeColor::Green}>{"Green"}</Badge>
                                    <Badge color={BadgeColor::Red}>{"Red"}</Badge>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Badge color={BadgeColor::Blue}>{"New"}</Badge>
<Badge color={BadgeColor::Grey}>{"Draft"}</Badge>
<Badge color={BadgeColor::Green}>{"Active"}</Badge>
<Badge color={BadgeColor::Red}>{"Critical"}</Badge>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Button Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Button"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Button Variants"}</div>
                            <div class="demo-example-description">
                                {format!("Interactive buttons with multiple variants (clicked {} times)", *button_clicks)}
                            </div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    <Button variant={ButtonVariant::Primary} on_click={on_button_click.clone()}>
                                        {"Primary"}
                                    </Button>
                                    <Button variant={ButtonVariant::Normal} on_click={on_button_click.clone()}>
                                        {"Normal"}
                                    </Button>
                                    <Button variant={ButtonVariant::Link}>
                                        {"Link"}
                                    </Button>
                                    <Button variant={ButtonVariant::Primary} disabled={true}>
                                        {"Disabled"}
                                    </Button>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Button variant={ButtonVariant::Primary} on_click={on_click}>
    {"Primary"}
</Button>
<Button variant={ButtonVariant::Normal}>
    {"Normal"}
</Button>
<Button variant={ButtonVariant::Link}>
    {"Link"}
</Button>"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Button with Icon"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    <Button
                                        variant={ButtonVariant::Primary}
                                        icon={html! { "+" }}
                                    >
                                        {"Add Item"}
                                    </Button>
                                    <Button
                                        variant={ButtonVariant::Normal}
                                        icon={html! { "↓" }}
                                        icon_align={IconAlign::Right}
                                    >
                                        {"Download"}
                                    </Button>
                                    <Button
                                        variant={ButtonVariant::Icon}
                                        icon={html! { "⚙" }}
                                        aria_label={"Settings"}
                                    />
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Button
    variant={ButtonVariant::Primary}
    icon={"add-plus"}
>
    {"Add Item"}
</Button>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // ButtonDropdown Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"ButtonDropdown"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Button with Dropdown Menu"}</div>
                            <div class="demo-example-description">{"Button with multiple action options"}</div>
                            <div class="demo-preview">
                                <ButtonDropdown
                                    items={vec![
                                        ButtonDropdownItem::new("action1", "Create Resource"),
                                        ButtonDropdownItem::new("action2", "Edit Resource"),
                                        ButtonDropdownItem::new("action3", "Delete Resource")
                                            .with_disabled(true),
                                    ]}
                                    variant={ButtonVariant::Primary}
                                >
                                    {"Actions"}
                                </ButtonDropdown>
                            </div>
                            <CodeSnippet code={r#"<ButtonDropdown
    items={vec![
        ButtonDropdownItem::new("action1", "Create Resource"),
    ]}
    variant={ButtonVariant::Primary}
>
    {"Actions"}
</ButtonDropdown>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Spinner Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Spinner"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Loading Indicators"}</div>
                            <div class="demo-example-description">{"Animated loading spinners"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    if *spinner_visible {
                                        <Spinner size={SpinnerSize::Normal} />
                                        <Spinner size={SpinnerSize::Large} />
                                        <Spinner size={SpinnerSize::Big} />
                                    }
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Spinner size={SpinnerSize::Normal} />
<Spinner size={SpinnerSize::Large} />
<Spinner size={SpinnerSize::Big} />"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Alert Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Alert"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Contextual Messages"}</div>
                            <div class="demo-example-description">{"Feedback messages with different severity levels"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Alert alert_type={AlertType::Success}>
                                        {"Your changes have been saved successfully."}
                                    </Alert>
                                    <Alert alert_type={AlertType::Info}>
                                        {"This feature is currently in beta."}
                                    </Alert>
                                    <Alert alert_type={AlertType::Warning}>
                                        {"This action cannot be undone."}
                                    </Alert>
                                    <Alert alert_type={AlertType::Error}>
                                        {"An error occurred while processing your request."}
                                    </Alert>
                                    if *alert_visible {
                                        <Alert
                                            alert_type={AlertType::Info}
                                            dismissible={true}
                                            on_dismiss={on_alert_dismiss}
                                        >
                                            {"This alert can be dismissed."}
                                        </Alert>
                                    }
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Alert alert_type={AlertType::Success}>
    {"Changes saved successfully"}
</Alert>
<Alert alert_type={AlertType::Warning}>
    {"This action cannot be undone"}
</Alert>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Link Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Link"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Hyperlinks"}</div>
                            <div class="demo-example-description">{"Links with external icon support"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Link href="#" variant={LinkVariant::Primary}>
                                        {"Primary link"}
                                    </Link>
                                    <Link href="#" external={true}>
                                        {"External link"}
                                    </Link>
                                    <Link href="#" variant={LinkVariant::Info}>
                                        {"Info link"}
                                    </Link>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r##"<Link href="#" variant={LinkVariant::Primary}>
    {"Primary link"}
</Link>
<Link href="#" external={true}>
    {"External link"}
</Link>"##} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Icon Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Icon"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Icon Variants"}</div>
                            <div class="demo-example-description">{"SVG icons with built-in support"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S} direction={SpaceBetweenDirection::Horizontal}>
                                    <Icon name="settings" size={IconSize::Normal} />
                                    <Icon name="search" size={IconSize::Normal} />
                                    <Icon name="notification" size={IconSize::Normal} />
                                    <Icon name="download" size={IconSize::Normal} />
                                    <Icon name="upload" size={IconSize::Normal} />
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Icon name="settings" size={IconSize::Normal} />
<Icon name="search" size={IconSize::Normal} />
<Icon name="notification" size={IconSize::Normal} />"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // ProgressBar Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"ProgressBar"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Progress Indicators"}</div>
                            <div class="demo-example-description">{"Visual progress indicators with status"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <ProgressBar
                                        value={25.0}
                                        label="Uploading file"
                                        description={html! { "25% complete" }}
                                    />
                                    <ProgressBar
                                        value={75.0}
                                        status={ProgressBarStatus::InProgress}
                                        label="Processing"
                                    />
                                    <ProgressBar
                                        value={100.0}
                                        status={ProgressBarStatus::Success}
                                        label="Complete"
                                    />
                                    <ProgressBar
                                        value={50.0}
                                        status={ProgressBarStatus::Error}
                                        label="Upload failed"
                                    />
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<ProgressBar
    value={75.0}
    status={ProgressBarStatus::InProgress}
    label="Processing"
/>
<ProgressBar
    value={100.0}
    status={ProgressBarStatus::Success}
    label="Complete"
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // CopyToClipboard Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"CopyToClipboard"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Copy Button"}</div>
                            <div class="demo-example-description">{"Button that copies text with feedback"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <div>
                                        <Box variant={BoxVariant::AwsuiKeyLabel}>{"API Endpoint"}</Box>
                                        <div style="display: flex; align-items: center; gap: 8px; margin-top: 4px;">
                                            <code>{"https://api.example.com/v1/resources"}</code>
                                            <CopyToClipboard
                                                copy_text="https://api.example.com/v1/resources"
                                                variant={CopyToClipboardVariant::Inline}
                                            />
                                        </div>
                                    </div>
                                    <div>
                                        <Box variant={BoxVariant::AwsuiKeyLabel}>{"Access Key"}</Box>
                                        <div style="display: flex; align-items: center; gap: 8px; margin-top: 4px;">
                                            <code>{"AKIAIOSFODNN7EXAMPLE"}</code>
                                            <CopyToClipboard
                                                copy_text="AKIAIOSFODNN7EXAMPLE"
                                                variant={CopyToClipboardVariant::Icon}
                                            />
                                        </div>
                                    </div>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<CopyToClipboard
    copy_text="https://api.example.com/v1/resources"
    variant={CopyToClipboardVariant::Inline}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Box Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Box"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Layout Utility"}</div>
                            <div class="demo-example-description">{"Flexible box for spacing and typography"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Box variant={BoxVariant::H1}>{"Heading 1"}</Box>
                                    <Box variant={BoxVariant::H2}>{"Heading 2"}</Box>
                                    <Box variant={BoxVariant::H3}>{"Heading 3"}</Box>
                                    <Box variant={BoxVariant::P}>{"Paragraph text with normal styling"}</Box>
                                    <Box variant={BoxVariant::Samp}>{"Samp text variant"}</Box>
                                    <Box variant={BoxVariant::Code}>{"Code snippet example"}</Box>
                                    <Box variant={BoxVariant::AwsuiKeyLabel}>{"Key Label"}</Box>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Box variant={BoxVariant::H1}>{"Heading 1"}</Box>
<Box variant={BoxVariant::P}>{"Paragraph"}</Box>
<Box variant={BoxVariant::Code}>{"Code"}</Box>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // TextContent Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"TextContent"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Rich Text Content"}</div>
                            <div class="demo-example-description">{"Container for formatted text with typography styles"}</div>
                            <div class="demo-preview">
                                <TextContent>
                                    <h3>{"Section Title"}</h3>
                                    <p>
                                        {"This is a paragraph with "}
                                        <strong>{"bold text"}</strong>
                                        {" and "}
                                        <em>{"italic text"}</em>
                                        {"."}
                                    </p>
                                    <ul>
                                        <li>{"First list item"}</li>
                                        <li>{"Second list item"}</li>
                                        <li>{"Third list item"}</li>
                                    </ul>
                                </TextContent>
                            </div>
                            <CodeSnippet code={r#"<TextContent>
    <h3>{"Section Title"}</h3>
    <p>{"This is formatted text."}</p>
    <ul>
        <li>{"List item"}</li>
    </ul>
</TextContent>"#} />
                        </div>
                    </SpaceBetween>
                </Container>
            </SpaceBetween>
        </ContentLayout>
    }
}
