// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! DateRangePicker component for selecting a date range with start and end dates.
//!
//! Provides a controlled date range picker with support for absolute (calendar-based)
//! and relative (preset) date range selection modes. Supports validation, disabled states,
//! and custom relative options.

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::internal::{
    BaseComponentProps, ComponentMetadata, ClassBuilder, CustomEvent,
    AriaAttributes,
};

/// Time unit for relative date ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    /// Days
    Days,
    /// Weeks
    Weeks,
    /// Months
    Months,
}

impl TimeUnit {
    /// Returns the string representation of the time unit
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Days => "days",
            Self::Weeks => "weeks",
            Self::Months => "months",
        }
    }
}

/// Range selector mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeSelectorMode {
    /// Absolute mode: Two inputs (start, end) with shared calendar
    Absolute,
    /// Relative mode: Dropdown with presets (Last 7 days, Last 30 days, etc.)
    Relative,
}

impl Default for RangeSelectorMode {
    fn default() -> Self {
        Self::Absolute
    }
}

impl RangeSelectorMode {
    /// Returns the string representation of the mode
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Absolute => "absolute",
            Self::Relative => "relative",
        }
    }
}

/// A date range with start and end dates
#[derive(Clone, PartialEq, Debug)]
pub struct DateRange {
    /// Start date in YYYY-MM-DD format
    pub start: Option<String>,
    /// End date in YYYY-MM-DD format
    pub end: Option<String>,
}

impl DateRange {
    /// Creates a new date range
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::DateRange;
    ///
    /// let range = DateRange::new(
    ///     Some("2024-01-01".to_string()),
    ///     Some("2024-01-31".to_string())
    /// );
    /// ```
    pub fn new(start: Option<String>, end: Option<String>) -> Self {
        Self { start, end }
    }

    /// Creates a date range with start date
    pub fn with_start(start: impl Into<String>) -> Self {
        Self {
            start: Some(start.into()),
            end: None,
        }
    }

    /// Creates a date range with end date
    pub fn with_end(end: impl Into<String>) -> Self {
        Self {
            start: None,
            end: Some(end.into()),
        }
    }

    /// Sets the start date
    pub fn set_start(mut self, start: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self
    }

    /// Sets the end date
    pub fn set_end(mut self, end: impl Into<String>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Checks if the range is valid (end >= start)
    pub fn is_valid(&self) -> bool {
        match (&self.start, &self.end) {
            (Some(start), Some(end)) => end >= start,
            _ => true, // Incomplete ranges are considered valid
        }
    }

    /// Checks if the range is complete (both start and end are set)
    pub fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Checks if the range is empty (neither start nor end are set)
    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }
}

impl Default for DateRange {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}

/// A relative date range option
#[derive(Clone, PartialEq, Debug)]
pub struct RelativeOption {
    /// Unique key for this option
    pub key: String,
    /// Display label
    pub label: String,
    /// Amount of time units
    pub amount: i32,
    /// Time unit
    pub unit: TimeUnit,
}

impl RelativeOption {
    /// Creates a new relative option
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloudscape_components::{RelativeOption, TimeUnit};
    ///
    /// let option = RelativeOption::new("last-7-days", "Last 7 days", 7, TimeUnit::Days);
    /// ```
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        amount: i32,
        unit: TimeUnit,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            amount,
            unit,
        }
    }

    /// Sets the label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets the amount
    pub fn with_amount(mut self, amount: i32) -> Self {
        self.amount = amount;
        self
    }

    /// Sets the time unit
    pub fn with_unit(mut self, unit: TimeUnit) -> Self {
        self.unit = unit;
        self
    }
}

/// Event detail for date range picker change events
#[derive(Clone, PartialEq)]
pub struct DateRangePickerChangeDetail {
    /// The new date range value
    pub value: Option<DateRange>,
}

/// Properties for the DateRangePicker component
#[derive(Properties, PartialEq, Clone)]
pub struct DateRangePickerProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Current date range value (controlled component)
    #[prop_or_default]
    pub value: Option<DateRange>,

    /// Placeholder text for date inputs
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Whether the picker is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the picker has invalid content
    #[prop_or_default]
    pub invalid: bool,

    /// Range selector mode (absolute or relative)
    #[prop_or_default]
    pub range_selector_mode: RangeSelectorMode,

    /// Relative options for relative mode
    #[prop_or_default]
    pub relative_options: Vec<RelativeOption>,

    /// Callback fired when the date range changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<DateRangePickerChangeDetail>>>,

    /// Custom validation callback
    ///
    /// Receives (start, end) and returns true if the range is valid
    #[prop_or_default]
    pub is_valid_range: Option<Callback<(Option<String>, Option<String>), bool>>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA required attribute
    #[prop_or_default]
    pub aria_required: bool,

    /// Control ID for form field integration
    #[prop_or_default]
    pub control_id: Option<String>,

    /// HTML name attribute for form integration
    #[prop_or_default]
    pub name: Option<String>,

    /// Automatically focus the picker when component is mounted
    #[prop_or_default]
    pub auto_focus: bool,

    /// Whether the picker is read-only
    #[prop_or_default]
    pub read_only: bool,

    /// Start date label (for accessibility)
    #[prop_or_default]
    pub start_date_label: Option<String>,

    /// End date label (for accessibility)
    #[prop_or_default]
    pub end_date_label: Option<String>,
}

/// DateRangePicker component for selecting a date range with start and end dates.
///
/// A controlled component that allows users to select a date range using either
/// absolute dates (with calendar inputs) or relative presets (like "Last 7 days").
/// Supports validation, disabled states, and custom relative options.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{DateRangePicker, DateRange, RelativeOption, TimeUnit};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let value = use_state(|| None);
///
///     let relative_options = vec![
///         RelativeOption::new("last-7-days", "Last 7 days", 7, TimeUnit::Days),
///         RelativeOption::new("last-30-days", "Last 30 days", 30, TimeUnit::Days),
///         RelativeOption::new("last-3-months", "Last 3 months", 3, TimeUnit::Months),
///     ];
///
///     let on_change = {
///         let value = value.clone();
///         Callback::from(move |event: CustomEvent<DateRangePickerChangeDetail>| {
///             value.set(event.detail.value.clone());
///         })
///     };
///
///     html! {
///         <DateRangePicker
///             value={(*value).clone()}
///             placeholder="Select date range"
///             relative_options={relative_options}
///             on_change={on_change}
///         />
///     }
/// }
/// ```
#[function_component(DateRangePicker)]
pub fn date_range_picker(props: &DateRangePickerProps) -> Html {
    let _metadata = ComponentMetadata::new("DateRangePicker");
    let start_input_ref = use_node_ref();
    let end_input_ref = use_node_ref();
    let current_mode = use_state(|| props.range_selector_mode);
    let selected_relative = use_state(|| None::<String>);

    // Handle start date change
    let on_start_change = {
        let value = props.value.clone();
        let on_change = props.on_change.clone();
        let is_valid_range = props.is_valid_range.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                let start = if target.value().is_empty() {
                    None
                } else {
                    Some(target.value())
                };

                let end = value.as_ref().and_then(|v| v.end.clone());

                // Validate range
                let is_valid = if let Some(validator) = &is_valid_range {
                    validator.emit((start.clone(), end.clone()))
                } else {
                    match (&start, &end) {
                        (Some(s), Some(e)) => e >= s,
                        _ => true,
                    }
                };

                if is_valid {
                    let new_range = if start.is_none() && end.is_none() {
                        None
                    } else {
                        Some(DateRange { start, end })
                    };

                    if let Some(callback) = &on_change {
                        callback.emit(CustomEvent::new_non_cancelable(
                            DateRangePickerChangeDetail { value: new_range }
                        ));
                    }
                }
            }
        })
    };

    // Handle end date change
    let on_end_change = {
        let value = props.value.clone();
        let on_change = props.on_change.clone();
        let is_valid_range = props.is_valid_range.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                let end = if target.value().is_empty() {
                    None
                } else {
                    Some(target.value())
                };

                let start = value.as_ref().and_then(|v| v.start.clone());

                // Validate range
                let is_valid = if let Some(validator) = &is_valid_range {
                    validator.emit((start.clone(), end.clone()))
                } else {
                    match (&start, &end) {
                        (Some(s), Some(e)) => e >= s,
                        _ => true,
                    }
                };

                if is_valid {
                    let new_range = if start.is_none() && end.is_none() {
                        None
                    } else {
                        Some(DateRange { start, end })
                    };

                    if let Some(callback) = &on_change {
                        callback.emit(CustomEvent::new_non_cancelable(
                            DateRangePickerChangeDetail { value: new_range }
                        ));
                    }
                }
            }
        })
    };

    // Handle mode change
    let on_mode_change = {
        let current_mode = current_mode.clone();

        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                let mode = match target.value().as_str() {
                    "relative" => RangeSelectorMode::Relative,
                    _ => RangeSelectorMode::Absolute,
                };
                current_mode.set(mode);
            }
        })
    };

    // Handle relative option selection
    let on_relative_change = {
        let on_change = props.on_change.clone();
        let selected_relative = selected_relative.clone();

        Callback::from(move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                let key = target.value();
                selected_relative.set(Some(key.clone()));

                // In a real implementation, you would calculate the actual date range
                // based on the relative option. For now, we'll emit None to indicate
                // that relative mode is selected but we're not calculating dates.
                if let Some(callback) = &on_change {
                    callback.emit(CustomEvent::new_non_cancelable(
                        DateRangePickerChangeDetail { value: None }
                    ));
                }
            }
        })
    };

    // Check if current range is invalid
    let is_range_invalid = props.value.as_ref()
        .map(|v| !v.is_valid())
        .unwrap_or(false) || props.invalid;

    // Build root classes
    let root_classes = ClassBuilder::new()
        .add("awsui-date-range-picker")
        .add_if(props.disabled, "awsui-date-range-picker-disabled")
        .add_if(is_range_invalid, "awsui-date-range-picker-invalid")
        .add_if(props.read_only, "awsui-date-range-picker-readonly");

    // Build input classes closure - creates a fresh builder each time it's called
    let build_input_classes = || {
        ClassBuilder::new()
            .add("awsui-input")
            .add("awsui-input-type-date")
            .add_if(props.disabled, "awsui-input-disabled")
            .add_if(props.read_only, "awsui-input-readonly")
            .add_if(is_range_invalid, "awsui-input-invalid")
            .build()
    };

    // Get placeholder text
    let placeholder_text = props.placeholder.clone()
        .unwrap_or_else(|| "YYYY-MM-DD".to_string());

    // Get start and end values
    let start_value = props.value.as_ref()
        .and_then(|v| v.start.clone())
        .unwrap_or_default();
    let end_value = props.value.as_ref()
        .and_then(|v| v.end.clone())
        .unwrap_or_default();

    html! {
        <div class={root_classes.build()}>
            // Mode toggle
            <div class="awsui-date-range-picker-mode-toggle">
                <label class="awsui-date-range-picker-mode-label">
                    <input
                        type="radio"
                        name={format!("{}-mode", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                        value="absolute"
                        checked={*current_mode == RangeSelectorMode::Absolute}
                        disabled={props.disabled}
                        onchange={on_mode_change.clone()}
                    />
                    <span class="awsui-date-range-picker-mode-label-text">
                        { "Absolute" }
                    </span>
                </label>
                <label class="awsui-date-range-picker-mode-label">
                    <input
                        type="radio"
                        name={format!("{}-mode", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                        value="relative"
                        checked={*current_mode == RangeSelectorMode::Relative}
                        disabled={props.disabled}
                        onchange={on_mode_change}
                    />
                    <span class="awsui-date-range-picker-mode-label-text">
                        { "Relative" }
                    </span>
                </label>
            </div>

            // Content based on mode
            if *current_mode == RangeSelectorMode::Absolute {
                <div class="awsui-date-range-picker-inputs">
                    // Start date input
                    <div class="awsui-date-range-picker-start-input">
                        <label
                            class="awsui-date-range-picker-input-label"
                            for={format!("{}-start", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                        >
                            { props.start_date_label.clone().unwrap_or_else(|| "Start date".to_string()) }
                        </label>
                        <input
                            ref={start_input_ref}
                            type="date"
                            class={build_input_classes()}
                            id={format!("{}-start", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                            name={props.name.clone().map(|n| format!("{}-start", n))}
                            value={start_value}
                            placeholder={placeholder_text.clone()}
                            disabled={props.disabled}
                            readonly={props.read_only}
                            autofocus={props.auto_focus}
                            aria-label={props.aria.label.clone().or_else(|| Some("Start date".to_string()))}
                            aria-required={props.aria_required.to_string()}
                            aria-invalid={is_range_invalid.to_string()}
                            aria-labelledby={props.aria.labelledby.clone()}
                            aria-describedby={props.aria.describedby.clone()}
                            oninput={on_start_change}
                        />
                    </div>

                    // End date input
                    <div class="awsui-date-range-picker-end-input">
                        <label
                            class="awsui-date-range-picker-input-label"
                            for={format!("{}-end", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                        >
                            { props.end_date_label.clone().unwrap_or_else(|| "End date".to_string()) }
                        </label>
                        <input
                            ref={end_input_ref}
                            type="date"
                            class={build_input_classes()}
                            id={format!("{}-end", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                            name={props.name.clone().map(|n| format!("{}-end", n))}
                            value={end_value}
                            placeholder={placeholder_text}
                            disabled={props.disabled}
                            readonly={props.read_only}
                            aria-label={props.aria.label.clone().or_else(|| Some("End date".to_string()))}
                            aria-required={props.aria_required.to_string()}
                            aria-invalid={is_range_invalid.to_string()}
                            aria-labelledby={props.aria.labelledby.clone()}
                            aria-describedby={props.aria.describedby.clone()}
                            oninput={on_end_change}
                        />
                    </div>
                </div>
            } else {
                // Relative mode dropdown
                <div class="awsui-date-range-picker-relative-dropdown">
                    <label
                        class="awsui-date-range-picker-input-label"
                        for={format!("{}-relative", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                    >
                        { "Select time range" }
                    </label>
                    <select
                        class="awsui-select-trigger"
                        id={format!("{}-relative", props.control_id.clone().unwrap_or_else(|| "date-range".to_string()))}
                        disabled={props.disabled}
                        aria-label={props.aria.label.clone().or_else(|| Some("Select relative date range".to_string()))}
                        aria-required={props.aria_required.to_string()}
                        aria-invalid={is_range_invalid.to_string()}
                        onchange={on_relative_change}
                    >
                        <option value="" selected={selected_relative.is_none()}>
                            { "Choose a time range" }
                        </option>
                        {
                            props.relative_options.iter().map(|option| {
                                let is_selected = selected_relative.as_ref()
                                    .map(|s| s == &option.key)
                                    .unwrap_or(false);

                                html! {
                                    <option
                                        key={option.key.clone()}
                                        value={option.key.clone()}
                                        selected={is_selected}
                                    >
                                        { &option.label }
                                    </option>
                                }
                            }).collect::<Html>()
                        }
                    </select>
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_unit_as_str() {
        assert_eq!(TimeUnit::Days.as_str(), "days");
        assert_eq!(TimeUnit::Weeks.as_str(), "weeks");
        assert_eq!(TimeUnit::Months.as_str(), "months");
    }

    #[test]
    fn test_range_selector_mode_default() {
        assert_eq!(RangeSelectorMode::default(), RangeSelectorMode::Absolute);
    }

    #[test]
    fn test_range_selector_mode_as_str() {
        assert_eq!(RangeSelectorMode::Absolute.as_str(), "absolute");
        assert_eq!(RangeSelectorMode::Relative.as_str(), "relative");
    }

    #[test]
    fn test_date_range_new() {
        let range = DateRange::new(
            Some("2024-01-01".to_string()),
            Some("2024-01-31".to_string())
        );
        assert_eq!(range.start, Some("2024-01-01".to_string()));
        assert_eq!(range.end, Some("2024-01-31".to_string()));
    }

    #[test]
    fn test_date_range_builder() {
        let range = DateRange::with_start("2024-01-01")
            .set_end("2024-01-31");
        assert_eq!(range.start, Some("2024-01-01".to_string()));
        assert_eq!(range.end, Some("2024-01-31".to_string()));
    }

    #[test]
    fn test_date_range_is_valid() {
        let valid_range = DateRange::new(
            Some("2024-01-01".to_string()),
            Some("2024-01-31".to_string())
        );
        assert!(valid_range.is_valid());

        let invalid_range = DateRange::new(
            Some("2024-01-31".to_string()),
            Some("2024-01-01".to_string())
        );
        assert!(!invalid_range.is_valid());

        let incomplete_range = DateRange::new(
            Some("2024-01-01".to_string()),
            None
        );
        assert!(incomplete_range.is_valid());
    }

    #[test]
    fn test_date_range_is_complete() {
        let complete_range = DateRange::new(
            Some("2024-01-01".to_string()),
            Some("2024-01-31".to_string())
        );
        assert!(complete_range.is_complete());

        let incomplete_range = DateRange::new(
            Some("2024-01-01".to_string()),
            None
        );
        assert!(!incomplete_range.is_complete());
    }

    #[test]
    fn test_date_range_is_empty() {
        let empty_range = DateRange::default();
        assert!(empty_range.is_empty());

        let non_empty_range = DateRange::new(
            Some("2024-01-01".to_string()),
            None
        );
        assert!(!non_empty_range.is_empty());
    }

    #[test]
    fn test_relative_option_new() {
        let option = RelativeOption::new("last-7-days", "Last 7 days", 7, TimeUnit::Days);
        assert_eq!(option.key, "last-7-days");
        assert_eq!(option.label, "Last 7 days");
        assert_eq!(option.amount, 7);
        assert_eq!(option.unit, TimeUnit::Days);
    }

    #[test]
    fn test_relative_option_builder() {
        let option = RelativeOption::new("test", "Test", 1, TimeUnit::Days)
            .with_label("Last 30 days")
            .with_amount(30)
            .with_unit(TimeUnit::Weeks);

        assert_eq!(option.label, "Last 30 days");
        assert_eq!(option.amount, 30);
        assert_eq!(option.unit, TimeUnit::Weeks);
    }

    #[test]
    fn test_date_range_picker_change_detail() {
        let range = DateRange::new(
            Some("2024-01-01".to_string()),
            Some("2024-01-31".to_string())
        );
        let detail = DateRangePickerChangeDetail {
            value: Some(range.clone()),
        };

        assert_eq!(detail.value.unwrap().start, Some("2024-01-01".to_string()));
    }

    #[test]
    fn test_date_range_equality() {
        let range1 = DateRange::new(
            Some("2024-01-01".to_string()),
            Some("2024-01-31".to_string())
        );
        let range2 = DateRange::new(
            Some("2024-01-01".to_string()),
            Some("2024-01-31".to_string())
        );
        let range3 = DateRange::new(
            Some("2024-02-01".to_string()),
            Some("2024-02-28".to_string())
        );

        assert_eq!(range1, range2);
        assert_ne!(range1, range3);
    }

    #[test]
    fn test_relative_option_equality() {
        let option1 = RelativeOption::new("last-7-days", "Last 7 days", 7, TimeUnit::Days);
        let option2 = RelativeOption::new("last-7-days", "Last 7 days", 7, TimeUnit::Days);
        let option3 = RelativeOption::new("last-30-days", "Last 30 days", 30, TimeUnit::Days);

        assert_eq!(option1, option2);
        assert_ne!(option1, option3);
    }
}
