// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating the TokenGroup component

use cloudscape_components::{CustomEvent, Token, TokenDismissDetail, TokenGroup, TokenGroupAlignment};
use yew::prelude::*;

#[function_component(TokenGroupExample)]
pub fn token_group_example() -> Html {
    // State for managing tokens
    let tokens = use_state(|| {
        vec![
            Token::new("1", "Environment: Production").with_dismissible(true),
            Token::new("2", "Region: us-east-1").with_dismissible(true),
            Token::new("3", "Status: Active").with_dismissible(false),
            Token::new("4", "Type: EC2").with_dismissible(true),
            Token::new("5", "Owner: Team A").with_dismissible(true),
            Token::new("6", "Cost Center: 12345").with_dismissible(true),
        ]
    });

    // Handle token dismissal
    let on_dismiss = {
        let tokens = tokens.clone();
        Callback::from(move |event: CustomEvent<TokenDismissDetail>| {
            let dismissed_id = &event.detail.item_id;
            tokens.set(
                (*tokens)
                    .iter()
                    .filter(|t| t.id != *dismissed_id)
                    .cloned()
                    .collect(),
            );
            web_sys::console::log_1(&format!("Token dismissed: {}", dismissed_id).into());
        })
    };

    html! {
        <div style="padding: 20px;">
            <h1>{"TokenGroup Component Examples"}</h1>

            <h2>{"Basic Usage (Horizontal)"}</h2>
            <TokenGroup
                items={vec![
                    Token::new("t1", "Tag 1").with_dismissible(true),
                    Token::new("t2", "Tag 2").with_dismissible(true),
                    Token::new("t3", "Tag 3").with_dismissible(true),
                ]}
            />

            <h2 style="margin-top: 40px;">{"Vertical Alignment"}</h2>
            <TokenGroup
                items={vec![
                    Token::new("v1", "Vertical Tag 1").with_dismissible(true),
                    Token::new("v2", "Vertical Tag 2").with_dismissible(true),
                    Token::new("v3", "Vertical Tag 3").with_dismissible(true),
                ]}
                alignment={TokenGroupAlignment::Vertical}
            />

            <h2 style="margin-top: 40px;">{"With Limit (Show More)"}</h2>
            <TokenGroup
                items={vec![
                    Token::new("l1", "Tag 1").with_dismissible(true),
                    Token::new("l2", "Tag 2").with_dismissible(true),
                    Token::new("l3", "Tag 3").with_dismissible(true),
                    Token::new("l4", "Tag 4").with_dismissible(true),
                    Token::new("l5", "Tag 5").with_dismissible(true),
                    Token::new("l6", "Tag 6").with_dismissible(true),
                ]}
                limit={3}
            />

            <h2 style="margin-top: 40px;">{"Mixed Dismissible and Non-Dismissible"}</h2>
            <TokenGroup
                items={vec![
                    Token::new("m1", "Dismissible").with_dismissible(true),
                    Token::new("m2", "Not Dismissible"),
                    Token::new("m3", "Dismissible").with_dismissible(true),
                    Token::new("m4", "Not Dismissible"),
                ]}
            />

            <h2 style="margin-top: 40px;">{"Disabled Tokens"}</h2>
            <TokenGroup
                items={vec![
                    Token::new("d1", "Active").with_dismissible(true),
                    Token::new("d2", "Disabled")
                        .with_dismissible(true)
                        .with_disabled(true),
                    Token::new("d3", "Active").with_dismissible(true),
                ]}
            />

            <h2 style="margin-top: 40px;">{"Interactive Example with State"}</h2>
            <p>{"Click the X to dismiss tokens. Non-dismissible tokens cannot be removed."}</p>
            <TokenGroup
                items={(*tokens).clone()}
                on_dismiss={on_dismiss}
                limit={4}
            />
            <p style="margin-top: 10px;">
                {format!("Remaining tokens: {}", tokens.len())}
            </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<TokenGroupExample>::new().render();
}
