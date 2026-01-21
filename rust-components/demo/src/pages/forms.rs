// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use crate::components::code_snippet::CodeSnippet;

#[function_component(FormComponents)]
pub fn form_components() -> Html {
    let input_value = use_state(|| String::from(""));
    let checkbox_checked = use_state(|| false);
    let toggle_checked = use_state(|| false);
    let radio_value = use_state(|| String::from("option1"));
    let select_value = use_state(|| String::from(""));
    let textarea_value = use_state(|| String::from(""));
    let multiselect_values: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let autosuggest_value = use_state(|| String::from(""));
    let tile_value = use_state(|| String::from("tile1"));

    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |event: CustomEvent<InputChangeDetail>| {
            input_value.set(event.detail.value);
        })
    };

    let on_checkbox_change = {
        let checkbox_checked = checkbox_checked.clone();
        Callback::from(move |event: CustomEvent<CheckboxChangeDetail>| {
            checkbox_checked.set(event.detail.checked);
        })
    };

    let on_toggle_change = {
        let toggle_checked = toggle_checked.clone();
        Callback::from(move |event: CustomEvent<ToggleChangeDetail>| {
            toggle_checked.set(event.detail.checked);
        })
    };

    let on_radio_change = {
        let radio_value = radio_value.clone();
        Callback::from(move |event: CustomEvent<RadioGroupChangeDetail>| {
            radio_value.set(event.detail.value);
        })
    };

    let on_select_change = {
        let select_value = select_value.clone();
        Callback::from(move |event: CustomEvent<SelectChangeDetail>| {
            select_value.set(event.detail.selected_option.value);
        })
    };

    let on_textarea_change = {
        let textarea_value = textarea_value.clone();
        Callback::from(move |event: CustomEvent<TextareaChangeDetail>| {
            textarea_value.set(event.detail.value);
        })
    };

    let on_tile_change = {
        let tile_value = tile_value.clone();
        Callback::from(move |event: CustomEvent<TilesChangeDetail>| {
            tile_value.set(event.detail.value);
        })
    };

    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Form Components"}
                    <div slot="description">
                        {"Interactive form controls for user input"}
                    </div>
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                // Input Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Input"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Text Input"}</div>
                            <div class="demo-example-description">
                                {format!("Single-line text input (current value: '{}')", *input_value)}
                            </div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <FormField
                                        label="Username"
                                        description={html!{"Enter your username"}}
                                    >
                                        <Input
                                            value={(*input_value).clone()}
                                            placeholder="Enter text here"
                                            on_change={on_input_change.clone()}
                                        />
                                    </FormField>
                                    <FormField
                                        label="Password"
                                        description={html!{"Must be at least 8 characters"}}
                                    >
                                        <Input
                                            input_type={InputType::Password}
                                            placeholder="Enter password"
                                        />
                                    </FormField>
                                    <FormField label="Email">
                                        <Input
                                            input_type={InputType::Email}
                                            placeholder="user@example.com"
                                        />
                                    </FormField>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Input
    value={value}
    placeholder="Enter text"
    on_change={on_change}
/>
<Input
    input_type={InputType::Password}
    placeholder="Password"
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Checkbox Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Checkbox"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Checkbox Input"}</div>
                            <div class="demo-example-description">
                                {format!("Checkbox control (checked: {})", *checkbox_checked)}
                            </div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <Checkbox
                                        checked={*checkbox_checked}
                                        on_change={on_checkbox_change}
                                    >
                                        {"I agree to the terms and conditions"}
                                    </Checkbox>
                                    <Checkbox checked={false}>
                                        {"Send me promotional emails"}
                                    </Checkbox>
                                    <Checkbox checked={true} disabled={true}>
                                        {"Disabled option"}
                                    </Checkbox>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Checkbox
    checked={checked}
    on_change={on_change}
>
    {"I agree to terms"}
</Checkbox>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Toggle Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Toggle"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Toggle Switch"}</div>
                            <div class="demo-example-description">
                                {format!("On/off toggle control (checked: {})", *toggle_checked)}
                            </div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <FormField
                                        label="Enable notifications"
                                        description={html!{"Receive real-time notifications"}}
                                    >
                                        <Toggle
                                            checked={*toggle_checked}
                                            on_change={on_toggle_change}
                                        />
                                    </FormField>
                                    <FormField label="Dark mode">
                                        <Toggle checked={false} />
                                    </FormField>
                                    <FormField label="Disabled toggle">
                                        <Toggle checked={true} disabled={true} />
                                    </FormField>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Toggle
    checked={checked}
    on_change={on_change}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // RadioGroup Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"RadioGroup"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Radio Button Group"}</div>
                            <div class="demo-example-description">
                                {format!("Single selection from multiple options (selected: {})", *radio_value)}
                            </div>
                            <div class="demo-preview">
                                <FormField label="Select a plan">
                                    <RadioGroup
                                        value={(*radio_value).clone()}
                                        on_change={on_radio_change}
                                        items={vec![
                                            RadioGroupItem::new("option1", html! { "Basic Plan" })
                                                .with_description(html! { "Free tier with limited features" }),
                                            RadioGroupItem::new("option2", html! { "Pro Plan" })
                                                .with_description(html! { "$9.99/month with all features" }),
                                            RadioGroupItem::new("option3", html! { "Enterprise Plan" })
                                                .with_description(html! { "Custom pricing and support" }),
                                        ]}
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<RadioGroup
    value={value}
    on_change={on_change}
    items={vec![
        RadioGroupItem::new("option1", html! { "Option 1" }),
    ]}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Tiles Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Tiles"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Selectable Tiles"}</div>
                            <div class="demo-example-description">
                                {format!("Radio-style selection with visual tiles (selected: {})", *tile_value)}
                            </div>
                            <div class="demo-preview">
                                <FormField label="Choose your deployment type">
                                    <Tiles
                                        value={(*tile_value).clone()}
                                        on_change={on_tile_change}
                                        columns={3}
                                        items={vec![
                                            TileItem::new("tile1", "Single Instance")
                                                .with_description(html! { "Deploy on a single EC2 instance" }),
                                            TileItem::new("tile2", "Auto Scaling")
                                                .with_description(html! { "Automatically scale based on load" }),
                                            TileItem::new("tile3", "Container")
                                                .with_description(html! { "Deploy with Docker containers" }),
                                        ]}
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<Tiles
    value={value}
    on_change={on_change}
    columns={3}
    items={vec![
        TileItem::new("tile1", "Option 1")
            .with_description(html! { "Description" }),
    ]}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Select Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Select"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Dropdown Selection"}</div>
                            <div class="demo-example-description">
                                {format!("Select from dropdown menu (selected: '{}')", *select_value)}
                            </div>
                            <div class="demo-preview">
                                <FormField label="Select a region">
                                    <Select
                                        selected_option={if !select_value.is_empty() {
                                            Some(SelectOption::new(&*select_value)
                                                .with_label(&*select_value))
                                        } else {
                                            None
                                        }}
                                        on_change={on_select_change}
                                        options={vec![
                                            SelectOption::new("us-east-1")
                                                .with_label("US East (N. Virginia)"),
                                            SelectOption::new("us-west-2")
                                                .with_label("US West (Oregon)"),
                                            SelectOption::new("eu-west-1")
                                                .with_label("Europe (Ireland)"),
                                            SelectOption::new("ap-southeast-1")
                                                .with_label("Asia Pacific (Singapore)"),
                                        ]}
                                        placeholder="Choose a region"
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<Select
    selected_option={selected}
    on_change={on_change}
    options={vec![
        SelectOption::new("opt1")
            .with_label("Option 1"),
    ]}
    placeholder="Choose an option"
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Textarea Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Textarea"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Multi-line Text Input"}</div>
                            <div class="demo-example-description">
                                {format!("Text area with {} characters", textarea_value.len())}
                            </div>
                            <div class="demo-preview">
                                <FormField
                                    label="Description"
                                    description={html!{"Enter a detailed description"}}
                                    constraint_text={html!{format!("{} / 500 characters", textarea_value.len())}}
                                >
                                    <Textarea
                                        value={(*textarea_value).clone()}
                                        on_change={on_textarea_change}
                                        placeholder="Enter your description here..."
                                        rows={5}
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<Textarea
    value={value}
    on_change={on_change}
    placeholder="Enter text..."
    rows={5}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // FormField Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"FormField"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Form Field Wrapper"}</div>
                            <div class="demo-example-description">{"Wraps form controls with labels and validation"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::S}>
                                    <FormField
                                        label="Instance name"
                                        description={html!{"Enter a unique name for your instance"}}
                                        constraint_text={html!{"Must be alphanumeric"}}
                                    >
                                        <Input placeholder="my-instance-1" />
                                    </FormField>
                                    <FormField
                                        label="Instance type"
                                        description={html!{"Select the instance size"}}
                                        error_text={html!{"This field is required"}}
                                    >
                                        <Select
                                            selected_option={SelectOption::new("")}
                                            options={vec![]}
                                            placeholder="Choose instance type"
                                        />
                                    </FormField>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<FormField
    label="Field label"
    description={html!{"Helper text"}}
    error_text="Validation error"
>
    <Input placeholder="..." />
</FormField>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Multiselect Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Multiselect"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Multiple Selection"}</div>
                            <div class="demo-example-description">{"Select multiple options from dropdown"}</div>
                            <div class="demo-preview">
                                <FormField label="Select security groups">
                                    <Multiselect
                                        selected_options={vec![]}
                                        options={vec![
                                            MultiselectOption::new("sg-1")
                                                .with_label("Default Security Group"),
                                            MultiselectOption::new("sg-2")
                                                .with_label("Web Server Security Group"),
                                            MultiselectOption::new("sg-3")
                                                .with_label("Database Security Group"),
                                        ]}
                                        placeholder="Choose security groups"
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<Multiselect
    selected_options={selected}
    on_change={on_change}
    options={vec![
        MultiselectOption::new("opt1")
            .with_label("Option 1"),
    ]}
    placeholder="Choose options"
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Autosuggest Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Autosuggest"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Input with Suggestions"}</div>
                            <div class="demo-example-description">{"Type to see suggestions"}</div>
                            <div class="demo-preview">
                                <FormField label="Search resources">
                                    <Autosuggest
                                        value={(*autosuggest_value).clone()}
                                        options={vec![
                                            AutosuggestOption::new("ec2-instance-1")
                                                .with_label("EC2 Instance 1"),
                                            AutosuggestOption::new("s3-bucket-1")
                                                .with_label("S3 Bucket 1"),
                                            AutosuggestOption::new("rds-database-1")
                                                .with_label("RDS Database 1"),
                                        ]}
                                        placeholder="Search..."
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<Autosuggest
    value={value}
    on_change={on_change}
    options={vec![
        AutosuggestOption::new("suggestion1")
            .with_label("Suggestion 1"),
    ]}
    placeholder="Type to search..."
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // DatePicker Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"DatePicker"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Date Selection"}</div>
                            <div class="demo-example-description">{"Pick a date from calendar"}</div>
                            <div class="demo-preview">
                                <FormField label="Select start date">
                                    <DatePicker
                                        value="2024-01-15"
                                        placeholder="YYYY-MM-DD"
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<DatePicker
    value={value}
    on_change={on_change}
    placeholder="YYYY-MM-DD"
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // DateRangePicker Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"DateRangePicker"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Date Range Selection"}</div>
                            <div class="demo-example-description">{"Pick start and end dates (absolute or relative)"}</div>
                            <div class="demo-preview">
                                <FormField label="Select date range">
                                    <DateRangePicker
                                        value={DateRange {
                                            start: Some("2024-01-01".to_string()),
                                            end: Some("2024-01-31".to_string()),
                                        }}
                                        placeholder="Select range"
                                        range_selector_mode={RangeSelectorMode::Absolute}
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<DateRangePicker
    value={DateRange {
        type_: "absolute".to_string(),
        start_date: Some("2024-01-01".to_string()),
        end_date: Some("2024-01-31".to_string()),
        ..Default::default()
    }}
    on_change={on_change}
    placeholder="Select range"
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // FileUpload Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"FileUpload"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"File Upload"}</div>
                            <div class="demo-example-description">{"Upload files with drag-and-drop support"}</div>
                            <div class="demo-preview">
                                <FormField
                                    label="Upload documents"
                                    description={html!{"Supported formats: PDF, DOC, DOCX"}}
                                >
                                    <FileUpload
                                        value={vec![]}
                                        multiple={true}
                                        accept=".pdf,.doc,.docx"
                                    />
                                </FormField>
                            </div>
                            <CodeSnippet code={r#"<FileUpload
    value={files}
    on_change={on_change}
    multiple={true}
    accept=".pdf,.doc,.docx"
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>
            </SpaceBetween>
        </ContentLayout>
    }
}
