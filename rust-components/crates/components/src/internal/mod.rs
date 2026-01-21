// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Internal utilities and shared component functionality

pub mod base_component;
pub mod classes;
pub mod events;
pub mod props;
pub mod styles;
pub mod analytics;
pub mod accessibility;

pub use base_component::{BaseComponentProps, ComponentMetadata};
pub use classes::{ClassBuilder, classes};
pub use events::{ClickEvent, CustomEvent, ClickDetail, FollowEvent};
pub use props::{StyleOverride, NativeAttributes, I18nStrings};
pub use styles::ComponentStyles;
pub use analytics::AnalyticsMetadata;
pub use accessibility::{AriaAttributes, FocusOptions};
