// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use crate::components::code_snippet::CodeSnippet;

#[function_component(NavigationComponents)]
pub fn navigation_components() -> Html {
    let active_tab_id = use_state(|| String::from("tab1"));
    let current_page = use_state(|| 1);

    let on_tab_change = {
        let active_tab_id = active_tab_id.clone();
        Callback::from(move |event: CustomEvent<TabChangeDetail>| {
            active_tab_id.set(event.detail.active_tab_id);
        })
    };

    let on_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |event: CustomEvent<PaginationChangeDetail>| {
            current_page.set(event.detail.requested_page_index);
        })
    };

    html! {
        <ContentLayout>
            <div slot="header">
                <Header variant={HeaderVariant::H1}>
                    {"Navigation Components"}
                    <div slot="description">
                        {"Components for navigation and page structure"}
                    </div>
                </Header>
            </div>

            <SpaceBetween size={SpaceBetweenSize::L}>
                // Tabs Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Tabs"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Tabbed Navigation"}</div>
                            <div class="demo-example-description">
                                {format!("Switch between views (active: {})", *active_tab_id)}
                            </div>
                            <div class="demo-preview">
                                <Tabs
                                    active_tab_id={(*active_tab_id).clone()}
                                    on_change={on_tab_change}
                                    tabs={vec![
                                        Tab::new("tab1", html!{"Overview"})
                                            .with_content(html! {
                                                <Container>
                                                    <TextContent>
                                                        <h3>{"Overview Content"}</h3>
                                                        <p>{"This is the overview tab content with general information."}</p>
                                                    </TextContent>
                                                </Container>
                                            }),
                                        Tab::new("tab2", html!{"Details"})
                                            .with_content(html! {
                                                <Container>
                                                    <TextContent>
                                                        <h3>{"Details Content"}</h3>
                                                        <p>{"Detailed information and specifications."}</p>
                                                    </TextContent>
                                                </Container>
                                            }),
                                        Tab::new("tab3", html!{"Configuration"})
                                            .with_content(html! {
                                                <Container>
                                                    <TextContent>
                                                        <h3>{"Configuration Content"}</h3>
                                                        <p>{"Configuration settings and options."}</p>
                                                    </TextContent>
                                                </Container>
                                            }),
                                        Tab::new("tab4", html!{"Disabled"})
                                            .with_content(html! { <div /> })
                                            .with_disabled(true),
                                    ]}
                                />
                            </div>
                            <CodeSnippet code={r#"<Tabs
    active_tab_id={active_tab_id}
    on_change={on_change}
    tabs={vec![
        Tab {
            id: "tab1".to_string(),
            label: "Tab 1".to_string(),
            content: html! { <div>{"Content 1"}</div> },
            ..Default::default()
        },
        Tab {
            id: "tab2".to_string(),
            label: "Tab 2".to_string(),
            content: html! { <div>{"Content 2"}</div> },
            ..Default::default()
        },
    ]}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // SideNavigation Component (already in use)
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"SideNavigation"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Sidebar Navigation"}</div>
                            <div class="demo-example-description">{"Hierarchical navigation menu"}</div>
                            <div class="demo-preview">
                                <Box variant={BoxVariant::P}>
                                    {"SideNavigation is currently being used in this demo application. "}
                                    {"You can see it in the left sidebar with all the component categories."}
                                </Box>
                                <Alert alert_type={AlertType::Info}>
                                    {"The sidebar on the left is a live example of the SideNavigation component with sections, links, and active state tracking."}
                                </Alert>
                            </div>
                            <CodeSnippet code={r#"<SideNavigation
    active_href={active_href}
    header={SideNavigationHeader {
        text: "Navigation".to_string(),
        href: "/".to_string(),
    }}
    items={vec![
        SideNavigationItem {
            type_: SideNavigationItemType::Link,
            text: "Home".to_string(),
            href: Some("/home".to_string()),
            ..Default::default()
        },
        SideNavigationItem {
            type_: SideNavigationItemType::Divider,
            ..Default::default()
        },
        SideNavigationItem {
            type_: SideNavigationItemType::Link,
            text: "Section".to_string(),
            href: Some("/section".to_string()),
            items: vec![
                SideNavigationItem {
                    type_: SideNavigationItemType::Link,
                    text: "Subsection".to_string(),
                    href: Some("/subsection".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ]}
    on_follow={on_follow}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Breadcrumbs Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Breadcrumbs"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Breadcrumb Trail"}</div>
                            <div class="demo-example-description">{"Navigation hierarchy path"}</div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::M}>
                                    <div>
                                        <Box variant={BoxVariant::P}>{"Simple breadcrumbs"}</Box>
                                        <Breadcrumbs
                                            items={vec![
                                                BreadcrumbItem {
                                                    text: "Home".to_string(),
                                                    href: "#".to_string(),
                                                },
                                                BreadcrumbItem {
                                                    text: "Components".to_string(),
                                                    href: "#".to_string(),
                                                },
                                                BreadcrumbItem {
                                                    text: "Navigation".to_string(),
                                                    href: "#".to_string(),
                                                },
                                            ]}
                                        />
                                    </div>

                                    <div>
                                        <Box variant={BoxVariant::P}>{"Deep hierarchy"}</Box>
                                        <Breadcrumbs
                                            items={vec![
                                                BreadcrumbItem {
                                                    text: "Services".to_string(),
                                                    href: "#".to_string(),
                                                },
                                                BreadcrumbItem {
                                                    text: "EC2".to_string(),
                                                    href: "#".to_string(),
                                                },
                                                BreadcrumbItem {
                                                    text: "Instances".to_string(),
                                                    href: "#".to_string(),
                                                },
                                                BreadcrumbItem {
                                                    text: "i-1234567890abcdef0".to_string(),
                                                    href: "#".to_string(),
                                                },
                                                BreadcrumbItem {
                                                    text: "Details".to_string(),
                                                    href: "#".to_string(),
                                                },
                                            ]}
                                        />
                                    </div>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r##"<Breadcrumbs
    items={vec![
        BreadcrumbItem {
            text: "Home".to_string(),
            href: "/".to_string(),
        },
        BreadcrumbItem {
            text: "Section".to_string(),
            href: "/section".to_string(),
        },
        BreadcrumbItem {
            text: "Current Page".to_string(),
            href: "#".to_string(),
        },
    ]}
    on_follow={on_follow}
/>"##} />
                        </div>
                    </SpaceBetween>
                </Container>

                // Pagination Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"Pagination"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Page Navigation"}</div>
                            <div class="demo-example-description">
                                {format!("Navigate between pages (current page: {})", *current_page)}
                            </div>
                            <div class="demo-preview">
                                <SpaceBetween size={SpaceBetweenSize::M}>
                                    <div>
                                        <Box variant={BoxVariant::P}>{"Default pagination"}</Box>
                                        <Pagination
                                            current_page_index={*current_page}
                                            pages_count={10}
                                            on_change={on_page_change.clone()}
                                        />
                                    </div>

                                    <div>
                                        <Box variant={BoxVariant::P}>{"With open end"}</Box>
                                        <Pagination
                                            current_page_index={1}
                                            pages_count={5}
                                            open_end={true}
                                        />
                                    </div>

                                    <div>
                                        <Box variant={BoxVariant::P}>{"Disabled"}</Box>
                                        <Pagination
                                            current_page_index={1}
                                            pages_count={1}
                                            disabled={true}
                                        />
                                    </div>
                                </SpaceBetween>
                            </div>
                            <CodeSnippet code={r#"<Pagination
    current_page_index={current_page}
    pages_count={10}
    on_change={on_change}
/>

<Pagination
    current_page_index={1}
    pages_count={5}
    open_end={true}
/>"#} />
                        </div>
                    </SpaceBetween>
                </Container>

                // TopNavigation Component
                <Container>
                    <div slot="header">
                        <Header variant={HeaderVariant::H2}>{"TopNavigation"}</Header>
                    </div>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <div class="demo-example">
                            <div class="demo-example-title">{"Top Application Bar"}</div>
                            <div class="demo-example-description">{"Global navigation with identity and utilities"}</div>
                            <div class="demo-preview">
                                <Box variant={BoxVariant::P}>
                                    {"TopNavigation provides a global application header with branding, search, and utility items. "}
                                    {"It's typically used at the very top of the page, above the AppLayout."}
                                </Box>
                                <Alert alert_type={AlertType::Info}>
                                    {"The TopNavigation component is designed to be placed outside the AppLayout, "}
                                    {"providing consistent branding and utilities across your entire application."}
                                </Alert>
                            </div>
                            <CodeSnippet code={r#"<TopNavigation
    identity={TopNavigationIdentity {
        title: "My Application".to_string(),
        href: "/".to_string(),
        logo: Some(TopNavigationLogo {
            src: "/logo.png".to_string(),
            alt: "Logo".to_string(),
        }),
    }}
    utilities={vec![
        TopNavigationUtility {
            type_: UtilityType::Button,
            text: Some("Notifications".to_string()),
            icon_name: Some("notification".to_string()),
            ..Default::default()
        },
        TopNavigationUtility {
            type_: UtilityType::MenuDropdown,
            text: Some("User Name".to_string()),
            icon_name: Some("user-profile".to_string()),
            items: vec![
                ButtonDropdownItem::new("profile", "Profile"),
                ButtonDropdownItem::new("signout", "Sign out"),
            ],
            ..Default::default()
        },
    ]}
/>"#} />
                        </div>

                        <div class="demo-example">
                            <div class="demo-example-title">{"Inline Example"}</div>
                            <div class="demo-preview">
                                <div style="border: 1px solid #e0e0e0; border-radius: 4px; overflow: hidden;">
                                    <TopNavigation
                                        identity={TopNavigationIdentity {
                                            title: Some("Demo Application".to_string()),
                                            href: "#".to_string(),
                                            logo: None,
                                            on_follow: None,
                                        }}
                                        utilities={vec![
                                            TopNavigationUtility::button("settings-button")
                                                .with_text("Settings")
                                                .with_icon_name("settings"),
                                        ]}
                                    />
                                </div>
                            </div>
                        </div>
                    </SpaceBetween>
                </Container>
            </SpaceBetween>
        </ContentLayout>
    }
}
