// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Performance benchmarks for Cloudscape Rust components

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cloudscape_components::*;
use yew::prelude::*;

/// Benchmark component HTML construction
fn bench_component_html(c: &mut Criterion) {
    let mut group = c.benchmark_group("component_html");

    group.bench_function("badge", |b| {
        b.iter(|| {
            let html = html! {
                <Badge color={BadgeColor::Blue}>{"Test"}</Badge>
            };
            black_box(html);
        });
    });

    group.bench_function("button", |b| {
        b.iter(|| {
            let html = html! {
                <Button variant={ButtonVariant::Primary}>{"Click me"}</Button>
            };
            black_box(html);
        });
    });

    group.bench_function("spinner", |b| {
        b.iter(|| {
            let html = html! {
                <Spinner size={SpinnerSize::Normal} />
            };
            black_box(html);
        });
    });

    group.bench_function("alert", |b| {
        b.iter(|| {
            let html = html! {
                <Alert alert_type={AlertType::Info}>{"Message"}</Alert>
            };
            black_box(html);
        });
    });

    group.finish();
}

/// Benchmark complex component trees
fn bench_complex_trees(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_trees");

    group.bench_function("small_tree", |b| {
        b.iter(|| {
            let html = html! {
                <Container>
                    <SpaceBetween size={SpaceBetweenSize::M}>
                        <Badge color={BadgeColor::Blue}>{"Badge 1"}</Badge>
                        <Badge color={BadgeColor::Green}>{"Badge 2"}</Badge>
                        <Button variant={ButtonVariant::Primary}>{"Action"}</Button>
                    </SpaceBetween>
                </Container>
            };
            black_box(html);
        });
    });

    group.bench_function("medium_tree", |b| {
        b.iter(|| {
            let html = html! {
                <Container>
                    <SpaceBetween size={SpaceBetweenSize::L}>
                        <Header variant={HeaderVariant::H2}>{"Title"}</Header>
                        <ColumnLayout columns={3}>
                            <div>
                                <Badge color={BadgeColor::Blue}>{"Status"}</Badge>
                            </div>
                            <div>
                                <Badge color={BadgeColor::Green}>{"Count"}</Badge>
                            </div>
                            <div>
                                <Button variant={ButtonVariant::Primary}>{"Action"}</Button>
                            </div>
                        </ColumnLayout>
                    </SpaceBetween>
                </Container>
            };
            black_box(html);
        });
    });

    group.finish();
}

/// Benchmark design token operations
#[cfg(feature = "generated")]
fn bench_design_tokens(c: &mut Criterion) {
    use cloudscape_design_tokens::generated::*;

    let mut group = c.benchmark_group("design_tokens");

    group.bench_function("color_token_css_var_name", |b| {
        b.iter(|| {
            let token = ColorToken::ColorBackgroundButtonPrimaryDefault;
            black_box(token.css_var_name());
        });
    });

    group.bench_function("color_token_css_var", |b| {
        b.iter(|| {
            let token = ColorToken::ColorBackgroundButtonPrimaryDefault;
            black_box(token.css_var());
        });
    });

    group.bench_function("multiple_token_lookups", |b| {
        b.iter(|| {
            let tokens = vec![
                ColorToken::ColorPrimary600.css_var(),
                ColorToken::ColorBackgroundButtonPrimaryDefault.css_var(),
                OtherToken::SpaceScaledM.css_var(),
                BorderToken::BorderRadiusButton.css_var(),
            ];
            black_box(tokens);
        });
    });

    group.finish();
}

/// Benchmark string operations
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    group.bench_function("class_concat_format", |b| {
        b.iter(|| {
            let classes = format!("awsui-badge awsui-badge-color-{}", "blue");
            black_box(classes);
        });
    });

    group.bench_function("class_concat_join", |b| {
        b.iter(|| {
            let classes = vec![
                "awsui-button",
                "awsui-button-variant-primary",
                "awsui-button-size-normal",
            ].join(" ");
            black_box(classes);
        });
    });

    group.bench_function("string_clone", |b| {
        let s = String::from("Test content");
        b.iter(|| {
            black_box(s.clone());
        });
    });

    group.finish();
}

#[cfg(feature = "generated")]
criterion_group!(
    benches,
    bench_component_html,
    bench_complex_trees,
    bench_design_tokens,
    bench_string_operations
);

#[cfg(not(feature = "generated"))]
criterion_group!(
    benches,
    bench_component_html,
    bench_complex_trees,
    bench_string_operations
);

criterion_main!(benches);
