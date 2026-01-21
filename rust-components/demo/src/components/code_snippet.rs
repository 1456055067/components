// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use yew::prelude::*;
use cloudscape_components::*;

#[derive(Properties, PartialEq)]
pub struct CodeSnippetProps {
    pub code: String,
    #[prop_or_default]
    pub language: String,
}

#[function_component(CodeSnippet)]
pub fn code_snippet(props: &CodeSnippetProps) -> Html {
    let copied = use_state(|| false);

    let on_copy = {
        let copied = copied.clone();
        let code = props.code.clone();
        Callback::from(move |event: CustomEvent<CopyDetail>| {
            // In a real app, you'd use the clipboard API
            web_sys::console::log_1(&format!("Copied: {}", code).into());
            copied.set(true);

            let copied = copied.clone();
            gloo::timers::callback::Timeout::new(2000, move || {
                copied.set(false);
            }).forget();
        })
    };

    html! {
        <div class="demo-code-snippet">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                <span style="font-size: 12px; color: #666; font-weight: 600;">
                    { if !props.language.is_empty() { &props.language } else { "Rust" } }
                </span>
                <CopyToClipboard
                    copy_text={props.code.clone()}
                    on_copy={on_copy}
                    variant={CopyToClipboardVariant::Inline}
                />
            </div>
            <pre><code>{ &props.code }</code></pre>
        </div>
    }
}
