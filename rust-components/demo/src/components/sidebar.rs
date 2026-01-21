// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;
use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub current_route: Route,
    pub on_navigate: Callback<Route>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let on_follow = {
        let on_navigate = props.on_navigate.clone();
        Callback::from(move |event: CustomEvent<SideNavigationFollowDetail>| {
            let route = Route::from_str(&event.detail.href);
            on_navigate.emit(route);
        })
    };

    let items = vec![
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Home".to_string(),
            href: Some("home".to_string()),
            external: false,
            items: vec![],
            info: None,
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Divider,
            text: String::new(),
            href: None,
            external: false,
            items: vec![],
            info: None,
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Basic Components".to_string(),
            href: Some("basic".to_string()),
            info: Some(html! { {"11 components"} }),
            external: false,
            items: vec![],
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Form Components".to_string(),
            href: Some("forms".to_string()),
            info: Some(html! { {"13 components"} }),
            external: false,
            items: vec![],
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Layout Components".to_string(),
            href: Some("layout".to_string()),
            info: Some(html! { {"6 components"} }),
            external: false,
            items: vec![],
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Navigation Components".to_string(),
            href: Some("navigation".to_string()),
            info: Some(html! { {"5 components"} }),
            external: false,
            items: vec![],
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Data Display".to_string(),
            href: Some("data-display".to_string()),
            info: Some(html! { {"6 components"} }),
            external: false,
            items: vec![],
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Overlay Components".to_string(),
            href: Some("overlay".to_string()),
            info: Some(html! { {"3 components"} }),
            external: false,
            items: vec![],
            default_expanded: None,
        },
        SideNavigationItem {
            item_type: SideNavigationItemType::Link,
            text: "Notification".to_string(),
            href: Some("notification".to_string()),
            info: Some(html! { {"1 component"} }),
            external: false,
            items: vec![],
            default_expanded: None,
        },
    ];

    html! {
        <SideNavigation
            active_href={props.current_route.as_str().to_string()}
            header={SideNavigationHeader {
                text: "Component Gallery".to_string(),
                href: "home".to_string(),
            }}
            {items}
            on_follow={on_follow}
        />
    }
}
