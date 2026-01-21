// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Cloudscape Design System Components for Yew
//!
//! This library provides Rust/WASM implementations of Cloudscape components
//! using the Yew framework, optimized for performance and security.

use wasm_bindgen::prelude::*;

// Re-export design tokens
pub use cloudscape_design_tokens as tokens;

// Component modules
pub mod alert;
pub mod app_layout;
pub mod autosuggest;
pub mod badge;
pub mod box_component;
pub mod breadcrumbs;
pub mod button;
pub mod button_dropdown;
pub mod cards;
pub mod checkbox;
pub mod column_layout;
pub mod container;
pub mod content_layout;
pub mod copy_to_clipboard;
pub mod date_picker;
pub mod date_range_picker;
pub mod drawer;
pub mod expandable_section;
pub mod file_upload;
pub mod flashbar;
pub mod form_field;
pub mod header;
pub mod icon;
pub mod input;
pub mod key_value_pairs;
pub mod link;
pub mod modal;
pub mod multiselect;
pub mod pagination;
pub mod popover;
pub mod progress_bar;
pub mod radio_group;
pub mod select;
pub mod side_navigation;
pub mod space_between;
pub mod spinner;
pub mod status_indicator;
pub mod table;
pub mod tabs;
pub mod text_content;
pub mod textarea;
pub mod tiles;
pub mod toggle;
pub mod token_group;
pub mod top_navigation;

// Internal utilities
pub mod internal;

// Re-export components for convenient access
pub use alert::{Alert, AlertI18nStrings, AlertProps, AlertType, DismissDetail};
pub use app_layout::{
    AppLayout, AppLayoutProps, ContentType, NavigationChangeDetail, SplitPanelPosition,
    SplitPanelPreferences, SplitPanelResizeDetail, SplitPanelToggleDetail, ToolsChangeDetail,
};
pub use autosuggest::{
    Autosuggest, AutosuggestChangeDetail, AutosuggestOption, AutosuggestProps,
    AutosuggestSelectDetail,
};
pub use badge::{Badge, BadgeColor, BadgeProps};
pub use box_component::{Box, BoxProps, BoxVariant, SpacingSize};
pub use breadcrumbs::{BreadcrumbFollowEvent, BreadcrumbItem, Breadcrumbs, BreadcrumbsProps};
pub use button::{Button, ButtonProps, ButtonVariant, FormAction, IconAlign};
pub use button_dropdown::{
    ButtonDropdown, ButtonDropdownItem, ButtonDropdownItemClickDetail, ButtonDropdownItemGroup,
    ButtonDropdownProps,
};
pub use cards::{CardDefinition, Cards, CardsProps, CardsSelectionDetail, CardsSelectionType};
pub use checkbox::{Checkbox, CheckboxChangeDetail, CheckboxProps};
pub use column_layout::{BordersType, ColumnLayout, ColumnLayoutProps, ColumnVariant};
pub use container::{Container, ContainerProps, ContainerVariant, Media, MediaPosition};
pub use content_layout::{ContentLayout, ContentLayoutProps};
pub use copy_to_clipboard::{
    CopyDetail, CopyStatus, CopyToClipboard, CopyToClipboardProps, CopyToClipboardVariant,
};
pub use date_picker::{DatePicker, DatePickerChangeDetail, DatePickerProps};
pub use date_range_picker::{
    DateRange, DateRangePicker, DateRangePickerChangeDetail, DateRangePickerProps,
    RangeSelectorMode, RelativeOption, TimeUnit,
};
pub use drawer::{Drawer, DrawerDismissDetail, DrawerProps, DrawerSize};
pub use expandable_section::{
    ExpandableSection, ExpandableSectionChangeDetail, ExpandableSectionProps,
    ExpandableSectionVariant,
};
pub use file_upload::{
    FileUpload, FileUploadChangeDetail, FileUploadFile, FileUploadI18nStrings, FileUploadProps,
};
pub use flashbar::{Flashbar, FlashbarDismissDetail, FlashbarItem, FlashbarProps, FlashbarType};
pub use form_field::{FormField, FormFieldProps};
pub use header::{Header, HeaderProps, HeaderVariant};
pub use icon::{Icon, IconProps, IconSize, IconVariant};
pub use input::{Input, InputChangeDetail, InputProps, InputType};
pub use key_value_pairs::{KeyValuePair, KeyValuePairs, KeyValuePairsProps};
pub use link::{FollowDetail, FollowEvent, Link, LinkColor, LinkFontSize, LinkProps, LinkVariant};
pub use modal::{DismissReason, Modal, ModalDismissDetail, ModalProps, ModalSize};
pub use multiselect::{
    FilteringType, Multiselect, MultiselectChangeDetail, MultiselectOption, MultiselectProps,
};
pub use pagination::{
    Pagination, PaginationChangeDetail, PaginationI18nStrings, PaginationPageClickDetail,
    PaginationProps,
};
pub use popover::{Popover, PopoverDismissDetail, PopoverPosition, PopoverProps, PopoverSize};
pub use progress_bar::{ProgressBar, ProgressBarProps, ProgressBarStatus, ProgressBarVariant};
pub use radio_group::{
    RadioGroup, RadioGroupChangeDetail, RadioGroupDirection, RadioGroupItem, RadioGroupProps,
};
pub use select::{Select, SelectChangeDetail, SelectOption, SelectProps};
pub use side_navigation::{
    ChangeDetail as SideNavigationChangeDetail, FollowDetail as SideNavigationFollowDetail,
    SideNavigation, SideNavigationHeader, SideNavigationItem, SideNavigationItemType,
    SideNavigationProps,
};
pub use space_between::{
    SpaceBetween, SpaceBetweenAlignment, SpaceBetweenDirection, SpaceBetweenProps,
    SpaceBetweenSize,
};
pub use spinner::{Spinner, SpinnerProps, SpinnerSize, SpinnerVariant};
pub use status_indicator::{
    StatusIndicator, StatusIndicatorColor, StatusIndicatorProps, StatusIndicatorType,
};
pub use table::{
    SelectionType, SortDirection, SortingState, Table, TableColumn, TableProps,
    TableSelectionDetail, TableSortDetail,
};
pub use tabs::{Tab, TabChangeDetail, TabDismissDetail, Tabs, TabsProps, TabsVariant};
pub use text_content::{TextContent, TextContentProps};
pub use textarea::{Textarea, TextareaChangeDetail, TextareaProps};
pub use tiles::{TileItem, Tiles, TilesChangeDetail, TilesProps};
pub use toggle::{Toggle, ToggleChangeDetail, ToggleProps};
pub use token_group::{Token, TokenDismissDetail, TokenGroup, TokenGroupAlignment, TokenGroupProps};
pub use top_navigation::{
    IdentityFollowDetail, TopNavigation, TopNavigationI18nStrings, TopNavigationIdentity,
    TopNavigationLogo, TopNavigationProps, TopNavigationUtility, UtilityButtonVariant,
    UtilityClickDetail, UtilityFollowDetail, UtilityType,
};

// Re-export commonly used internal types
pub use internal::CustomEvent;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the Cloudscape components library
///
/// This should be called once when your application starts.
/// It sets up any necessary global state or configuration.
pub fn init() {
    // Initialization code can be added here as needed
    // For example: panic hooks, logging setup, etc.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }
}
