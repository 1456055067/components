// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Internal utilities and shared component functionality

pub mod accessibility;
pub mod analytics;
pub mod base_component;
pub mod classes;
pub mod events;
pub mod props;
pub mod styles;

pub use accessibility::{AriaAttributes, FocusOptions};
pub use analytics::AnalyticsMetadata;
pub use base_component::{BaseComponentProps, ComponentMetadata};
pub use classes::{ClassBuilder, classes};
pub use events::{ClickDetail, ClickEvent, CustomEvent, FollowEvent};
pub use props::{I18nStrings, NativeAttributes, StyleOverride};
pub use styles::ComponentStyles;
