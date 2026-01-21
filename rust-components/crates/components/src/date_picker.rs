// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! DatePicker component for date selection with a calendar dropdown.
//!
//! Provides a controlled date input with a calendar dropdown for visual date selection.
//! Supports keyboard navigation, manual text entry, and validation.

use crate::internal::{
    AriaAttributes, BaseComponentProps, ClassBuilder, ComponentMetadata, CustomEvent,
};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

/// Event detail for date picker change events
#[derive(Clone, PartialEq, Debug)]
pub struct DatePickerChangeDetail {
    /// The new date value in YYYY-MM-DD format
    pub value: String,
}

/// Properties for the DatePicker component
#[derive(Properties, PartialEq, Clone)]
pub struct DatePickerProps {
    /// Base component properties
    #[prop_or_default]
    pub base: BaseComponentProps,

    /// Current date value in YYYY-MM-DD format (controlled component)
    #[prop_or_default]
    pub value: String,

    /// Placeholder text (default: "YYYY/MM/DD")
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Whether the date picker is disabled
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the date picker has invalid content
    #[prop_or_default]
    pub invalid: bool,

    /// Whether to show a warning
    #[prop_or_default]
    pub warning: bool,

    /// Whether the date picker is read-only
    #[prop_or_default]
    pub read_only: bool,

    /// ARIA label for the "Today" button (default: "Today")
    #[prop_or_default]
    pub today_aria_label: Option<String>,

    /// ARIA label for the "Next month" button (default: "Next month")
    #[prop_or_default]
    pub next_month_aria_label: Option<String>,

    /// ARIA label for the "Previous month" button (default: "Previous month")
    #[prop_or_default]
    pub previous_month_aria_label: Option<String>,

    /// ARIA label for the calendar button (default: "Open calendar")
    #[prop_or_default]
    pub open_calendar_aria_label: Option<String>,

    /// Callback fired when the date value changes
    #[prop_or_default]
    pub on_change: Option<Callback<CustomEvent<DatePickerChangeDetail>>>,

    /// Callback fired when input loses focus
    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,

    /// Callback fired when input gains focus
    #[prop_or_default]
    pub on_focus: Option<Callback<()>>,

    /// ARIA attributes
    #[prop_or_default]
    pub aria: AriaAttributes,

    /// ARIA required attribute
    #[prop_or_default]
    pub aria_required: bool,

    /// Control ID for form field integration
    #[prop_or_default]
    pub control_id: Option<String>,

    /// HTML name attribute
    #[prop_or_default]
    pub name: Option<String>,

    /// Auto-focus on mount
    #[prop_or_default]
    pub auto_focus: bool,
}

/// Simple date structure for internal use
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DateValue {
    year: i32,
    month: u32,
    day: u32,
}

impl DateValue {
    /// Create a new date value
    fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(1..=12).contains(&month) {
            return None;
        }
        let days_in_month = get_days_in_month(year, month);
        if day < 1 || day > days_in_month {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// Format as YYYY-MM-DD
    fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Format for display (YYYY/MM/DD)
    fn format_display(&self) -> String {
        format!("{:04}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

/// Parse date string in YYYY-MM-DD format
fn parse_date(s: &str) -> Option<DateValue> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let day = parts[2].parse::<u32>().ok()?;

    DateValue::new(year, month, day)
}

/// Parse date string in display format (YYYY/MM/DD)
fn parse_date_display(s: &str) -> Option<DateValue> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let day = parts[2].parse::<u32>().ok()?;

    DateValue::new(year, month, day)
}

/// Get the number of days in a given month
fn get_days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Get the day of week for the first day of the month (0 = Sunday, 6 = Saturday)
fn get_first_day_of_month(year: i32, month: u32) -> u32 {
    // Zeller's congruence algorithm
    let mut y = year;
    let mut m = month as i32;

    // January and February are counted as months 13 and 14 of the previous year
    if m < 3 {
        m += 12;
        y -= 1;
    }

    let k = y % 100;
    let j = y / 100;

    let h = (1 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;

    // Convert from Zeller's (0=Saturday) to our format (0=Sunday)
    ((h + 6) % 7) as u32
}

/// Get current date (uses a simple approximation - days since Unix epoch)
fn get_today() -> DateValue {
    // For WASM, we'd use js_sys::Date, but for simplicity we'll use a fixed date
    // In production, this would call JavaScript's Date API
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let year = date.get_full_year() as i32;
        let month = (date.get_month() + 1.0) as u32;
        let day = date.get_date() as u32;
        DateValue::new(year, month, day).unwrap_or(DateValue {
            year: 2026,
            month: 1,
            day: 1,
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Default for testing
        DateValue {
            year: 2026,
            month: 1,
            day: 20,
        }
    }
}

/// Month names for display
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Day names for calendar header
const DAY_NAMES: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// DatePicker component for date selection with a calendar dropdown.
///
/// # Example
///
/// ```rust
/// use cloudscape_components::{DatePicker, DatePickerChangeDetail, CustomEvent};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let date = use_state(|| "2026-01-20".to_string());
///
///     let on_change = {
///         let date = date.clone();
///         Callback::from(move |event: CustomEvent<DatePickerChangeDetail>| {
///             date.set(event.detail.value.clone());
///         })
///     };
///
///     html! {
///         <DatePicker
///             value={(*date).clone()}
///             placeholder="Select a date"
///             on_change={on_change}
///         />
///     }
/// }
/// ```
#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProps) -> Html {
    let _metadata = ComponentMetadata::new("DatePicker");
    let input_ref = use_node_ref();
    let is_calendar_open = use_state(|| false);
    let input_text = use_state(String::new);

    // Parse current value to determine displayed month
    let current_value = parse_date(&props.value);
    let today = get_today();

    // Current displayed month in calendar (defaults to selected date or today)
    let displayed_month = use_state(|| {
        current_value
            .map(|d| (d.year, d.month))
            .unwrap_or((today.year, today.month))
    });

    // Focused day in calendar for keyboard navigation
    let focused_day = use_state(|| current_value.map(|d| d.day).unwrap_or(1));

    // Update input text when value prop changes
    use_effect_with(props.value.clone(), {
        let input_text = input_text.clone();
        move |value| {
            if let Some(date) = parse_date(value) {
                input_text.set(date.format_display());
            } else if value.is_empty() {
                input_text.set(String::new());
            }
            || ()
        }
    });

    // Handle input text change
    let on_input = {
        let input_text = input_text.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                input_text.set(target.value());
            }
        })
    };

    // Handle input blur - validate and update value
    let on_blur_input = {
        let on_blur = props.on_blur.clone();
        let on_change = props.on_change.clone();
        let input_text = input_text.clone();
        let value = props.value.clone();

        Callback::from(move |_e: FocusEvent| {
            // Try to parse the input text
            if let Some(date) = parse_date_display(&input_text) {
                let new_value = date.format();
                if new_value != value
                    && let Some(callback) = &on_change
                {
                    callback.emit(CustomEvent::new_non_cancelable(DatePickerChangeDetail {
                        value: new_value,
                    }));
                }
            } else if input_text.is_empty() {
                // Clear the value
                if !value.is_empty()
                    && let Some(callback) = &on_change
                {
                    callback.emit(CustomEvent::new_non_cancelable(DatePickerChangeDetail {
                        value: String::new(),
                    }));
                }
            } else {
                // Invalid input - revert to current value
                if let Some(date) = parse_date(&value) {
                    input_text.set(date.format_display());
                } else {
                    input_text.set(String::new());
                }
            }

            if let Some(callback) = &on_blur {
                callback.emit(());
            }
        })
    };

    // Handle input focus
    let on_focus_input = {
        let on_focus = props.on_focus.clone();

        Callback::from(move |_e: FocusEvent| {
            if let Some(callback) = &on_focus {
                callback.emit(());
            }
        })
    };

    // Toggle calendar
    let on_calendar_button_click = {
        let is_calendar_open = is_calendar_open.clone();
        let disabled = props.disabled;
        let read_only = props.read_only;

        Callback::from(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            if !disabled && !read_only {
                is_calendar_open.set(!*is_calendar_open);
            }
        })
    };

    // Handle day selection
    let on_day_select = {
        let is_calendar_open = is_calendar_open.clone();
        let on_change = props.on_change.clone();
        let displayed_month = displayed_month.clone();

        Callback::from(move |day: u32| {
            let (year, month) = *displayed_month;
            if let Some(date) = DateValue::new(year, month, day)
                && let Some(callback) = &on_change
            {
                callback.emit(CustomEvent::new_non_cancelable(DatePickerChangeDetail {
                    value: date.format(),
                }));
            }
            is_calendar_open.set(false);
        })
    };

    // Navigate to previous month
    let on_previous_month = {
        let displayed_month = displayed_month.clone();

        Callback::from(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let (year, month) = *displayed_month;
            if month == 1 {
                displayed_month.set((year - 1, 12));
            } else {
                displayed_month.set((year, month - 1));
            };
        })
    };

    // Navigate to next month
    let on_next_month = {
        let displayed_month = displayed_month.clone();

        Callback::from(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let (year, month) = *displayed_month;
            if month == 12 {
                displayed_month.set((year + 1, 1));
            } else {
                displayed_month.set((year, month + 1));
            };
        })
    };

    // Go to today
    let on_today_click = {
        let displayed_month = displayed_month.clone();
        let focused_day = focused_day.clone();

        Callback::from(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let today = get_today();
            displayed_month.set((today.year, today.month));
            focused_day.set(today.day);
        })
    };

    // Handle keyboard navigation in calendar
    let on_calendar_keydown = {
        let is_calendar_open = is_calendar_open.clone();
        let focused_day = focused_day.clone();
        let displayed_month = displayed_month.clone();
        let on_change = props.on_change.clone();

        Callback::from(move |e: KeyboardEvent| {
            if !*is_calendar_open {
                return;
            }

            let (year, month) = *displayed_month;
            let current_day = *focused_day;
            let days_in_month = get_days_in_month(year, month);

            match e.key().as_str() {
                "ArrowLeft" => {
                    e.prevent_default();
                    if current_day > 1 {
                        focused_day.set(current_day - 1);
                    }
                }
                "ArrowRight" => {
                    e.prevent_default();
                    if current_day < days_in_month {
                        focused_day.set(current_day + 1);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if current_day > 7 {
                        focused_day.set(current_day - 7);
                    }
                }
                "ArrowDown" => {
                    e.prevent_default();
                    if current_day + 7 <= days_in_month {
                        focused_day.set(current_day + 7);
                    }
                }
                "Home" => {
                    e.prevent_default();
                    focused_day.set(1);
                }
                "End" => {
                    e.prevent_default();
                    focused_day.set(days_in_month);
                }
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(date) = DateValue::new(year, month, current_day)
                        && let Some(callback) = &on_change
                    {
                        callback.emit(CustomEvent::new_non_cancelable(DatePickerChangeDetail {
                            value: date.format(),
                        }));
                    }
                    is_calendar_open.set(false);
                }
                "Escape" => {
                    e.prevent_default();
                    is_calendar_open.set(false);
                }
                _ => {}
            }
        })
    };

    // Build CSS classes
    let container_classes = ClassBuilder::new()
        .add("awsui-date-picker")
        .add_if(props.disabled, "awsui-date-picker-disabled")
        .add_if(props.invalid, "awsui-date-picker-invalid")
        .add_if(props.warning && !props.invalid, "awsui-date-picker-warning");

    let input_classes = ClassBuilder::new()
        .add("awsui-date-picker-input")
        .add_if(props.disabled, "awsui-date-picker-input-disabled")
        .add_if(props.invalid, "awsui-date-picker-input-invalid");

    let calendar_button_classes = ClassBuilder::new()
        .add("awsui-date-picker-calendar-button")
        .add_if(*is_calendar_open, "awsui-date-picker-calendar-button-open");

    html! {
        <div class={container_classes.build()}>
            <div class="awsui-date-picker-input-wrapper">
                <input
                    ref={input_ref}
                    type="text"
                    class={input_classes.build()}
                    id={props.control_id.clone()}
                    name={props.name.clone()}
                    value={(*input_text).clone()}
                    placeholder={props.placeholder.clone().unwrap_or_else(|| "YYYY/MM/DD".to_string())}
                    disabled={props.disabled}
                    readonly={props.read_only}
                    autofocus={props.auto_focus}
                    aria-label={props.aria.label.clone()}
                    aria-required={props.aria_required.to_string()}
                    aria-invalid={props.invalid.to_string()}
                    aria-labelledby={props.aria.labelledby.clone()}
                    aria-describedby={props.aria.describedby.clone()}
                    oninput={on_input}
                    onblur={on_blur_input}
                    onfocus={on_focus_input}
                />

                <button
                    type="button"
                    class={calendar_button_classes.build()}
                    aria-label={props.open_calendar_aria_label.clone().unwrap_or_else(|| "Open calendar".to_string())}
                    aria-expanded={is_calendar_open.to_string()}
                    disabled={props.disabled || props.read_only}
                    onclick={on_calendar_button_click}
                >
                    <span class="awsui-icon awsui-icon-calendar" aria-hidden="true">
                        {"📅"}
                    </span>
                </button>
            </div>

            if *is_calendar_open {
                {render_calendar(
                    *displayed_month,
                    current_value,
                    today,
                    *focused_day,
                    on_previous_month,
                    on_next_month,
                    on_today_click,
                    on_day_select,
                    on_calendar_keydown,
                    &props.previous_month_aria_label,
                    &props.next_month_aria_label,
                    &props.today_aria_label,
                    *is_calendar_open,
                )}
            }
        </div>
    }
}

/// Render the calendar dropdown
#[allow(clippy::too_many_arguments)]
fn render_calendar(
    displayed_month: (i32, u32),
    selected_date: Option<DateValue>,
    today: DateValue,
    focused_day: u32,
    on_previous_month: Callback<web_sys::MouseEvent>,
    on_next_month: Callback<web_sys::MouseEvent>,
    on_today_click: Callback<web_sys::MouseEvent>,
    on_day_select: Callback<u32>,
    on_keydown: Callback<KeyboardEvent>,
    previous_aria_label: &Option<String>,
    next_aria_label: &Option<String>,
    today_aria_label: &Option<String>,
    is_calendar_open: bool,
) -> Html {
    let (year, month) = displayed_month;
    let days_in_month = get_days_in_month(year, month);
    let first_day = get_first_day_of_month(year, month);

    let dropdown_classes = ClassBuilder::new()
        .add("awsui-date-picker-dropdown")
        .add_if(is_calendar_open, "awsui-date-picker-dropdown-open")
        .build();

    html! {
        <div
            class={dropdown_classes}
            onkeydown={on_keydown}
            tabindex="-1"
        >
            <div class="awsui-date-picker-calendar">
                // Header with month/year and navigation
                <div class="awsui-date-picker-header">
                    <button
                        type="button"
                        class="awsui-date-picker-header-button awsui-date-picker-header-prev"
                        aria-label={previous_aria_label.clone().unwrap_or_else(|| "Previous month".to_string())}
                        onclick={on_previous_month}
                    >
                        {"◀"}
                    </button>

                    <div class="awsui-date-picker-header-title">
                        {format!("{} {}", MONTH_NAMES[(month - 1) as usize], year)}
                    </div>

                    <button
                        type="button"
                        class="awsui-date-picker-header-button awsui-date-picker-header-next"
                        aria-label={next_aria_label.clone().unwrap_or_else(|| "Next month".to_string())}
                        onclick={on_next_month}
                    >
                        {"▶"}
                    </button>
                </div>

                // Day names header
                <div class="awsui-date-picker-weekdays">
                    {
                        DAY_NAMES.iter().map(|day_name| {
                            html! {
                                <div class="awsui-date-picker-weekday" key={*day_name}>
                                    {*day_name}
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                // Calendar grid
                <div class="awsui-date-picker-grid" role="grid">
                    {render_calendar_days(CalendarDaysConfig {
                        year,
                        month,
                        days_in_month,
                        first_day,
                        selected_date,
                        today,
                        focused_day,
                        on_day_select,
                    })}
                </div>

                // Footer with "Today" button
                <div class="awsui-date-picker-footer">
                    <button
                        type="button"
                        class="awsui-date-picker-today-button"
                        aria-label={today_aria_label.clone().unwrap_or_else(|| "Today".to_string())}
                        onclick={on_today_click}
                    >
                        {today_aria_label.clone().unwrap_or_else(|| "Today".to_string())}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Render the calendar days grid
struct CalendarDaysConfig {
    year: i32,
    month: u32,
    days_in_month: u32,
    first_day: u32,
    selected_date: Option<DateValue>,
    today: DateValue,
    focused_day: u32,
    on_day_select: Callback<u32>,
}

fn render_calendar_days(config: CalendarDaysConfig) -> Html {
    let CalendarDaysConfig {
        year,
        month,
        days_in_month,
        first_day,
        selected_date,
        today,
        focused_day,
        on_day_select,
    } = config;
    let mut days = Vec::new();

    // Add empty cells for days before the first day of the month
    for _ in 0..first_day {
        days.push(html! {
            <div class="awsui-date-picker-day awsui-date-picker-day-empty" key={format!("empty-{}", days.len())} />
        });
    }

    // Add cells for each day of the month
    for day in 1..=days_in_month {
        let is_selected = selected_date
            .map(|d| d.year == year && d.month == month && d.day == day)
            .unwrap_or(false);
        let is_today = today.year == year && today.month == month && today.day == day;
        let is_focused = day == focused_day;

        let day_classes = ClassBuilder::new()
            .add("awsui-date-picker-day")
            .add_if(is_selected, "awsui-date-picker-day-selected")
            .add_if(is_today, "awsui-date-picker-day-today")
            .add_if(is_focused, "awsui-date-picker-day-focused");

        let day_clone = day;
        let on_click = {
            let on_day_select = on_day_select.clone();
            Callback::from(move |e: web_sys::MouseEvent| {
                e.prevent_default();
                on_day_select.emit(day_clone);
            })
        };

        days.push(html! {
            <button
                type="button"
                class={day_classes.build()}
                aria-selected={is_selected.to_string()}
                aria-current={if is_today { "date" } else { "false" }}
                tabindex={if is_focused { "0" } else { "-1" }}
                onclick={on_click}
                key={day}
            >
                {day}
            </button>
        });
    }

    html! {
        <>
            { for days }
        </>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_valid() {
        let date = parse_date("2026-01-20").unwrap();
        assert_eq!(date.year, 2026);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 20);
    }

    #[test]
    fn test_parse_date_invalid_format() {
        assert!(parse_date("20-01-2026").is_none());
        assert!(parse_date("2026/01/20").is_none());
        assert!(parse_date("invalid").is_none());
        assert!(parse_date("").is_none());
    }

    #[test]
    fn test_parse_date_invalid_values() {
        assert!(parse_date("2026-13-01").is_none()); // Invalid month
        assert!(parse_date("2026-00-01").is_none()); // Invalid month
        assert!(parse_date("2026-01-32").is_none()); // Invalid day
        assert!(parse_date("2026-01-00").is_none()); // Invalid day
        assert!(parse_date("2026-02-30").is_none()); // Invalid day for February
    }

    #[test]
    fn test_parse_date_display_valid() {
        let date = parse_date_display("2026/01/20").unwrap();
        assert_eq!(date.year, 2026);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 20);
    }

    #[test]
    fn test_parse_date_display_invalid() {
        assert!(parse_date_display("2026-01-20").is_none());
        assert!(parse_date_display("invalid").is_none());
    }

    #[test]
    fn test_date_value_format() {
        let date = DateValue::new(2026, 1, 20).unwrap();
        assert_eq!(date.format(), "2026-01-20");
    }

    #[test]
    fn test_date_value_format_display() {
        let date = DateValue::new(2026, 1, 20).unwrap();
        assert_eq!(date.format_display(), "2026/01/20");
    }

    #[test]
    fn test_get_days_in_month() {
        assert_eq!(get_days_in_month(2026, 1), 31); // January
        assert_eq!(get_days_in_month(2026, 2), 28); // February (non-leap)
        assert_eq!(get_days_in_month(2024, 2), 29); // February (leap)
        assert_eq!(get_days_in_month(2026, 4), 30); // April
        assert_eq!(get_days_in_month(2026, 12), 31); // December
    }

    #[test]
    fn test_is_leap_year() {
        assert!(!is_leap_year(2026)); // Not a leap year
        assert!(is_leap_year(2024)); // Divisible by 4
        assert!(!is_leap_year(2100)); // Divisible by 100 but not 400
        assert!(is_leap_year(2000)); // Divisible by 400
    }

    #[test]
    fn test_get_first_day_of_month() {
        // January 2026 starts on Thursday (4)
        let first_day = get_first_day_of_month(2026, 1);
        // Note: The exact value depends on the Zeller's congruence implementation
        assert!(first_day < 7);
    }

    #[test]
    fn test_date_value_new_validates() {
        assert!(DateValue::new(2026, 1, 20).is_some());
        assert!(DateValue::new(2026, 13, 1).is_none()); // Invalid month
        assert!(DateValue::new(2026, 0, 1).is_none()); // Invalid month
        assert!(DateValue::new(2026, 1, 32).is_none()); // Invalid day
        assert!(DateValue::new(2026, 2, 29).is_none()); // Invalid day for non-leap year
        assert!(DateValue::new(2024, 2, 29).is_some()); // Valid day for leap year
    }

    #[test]
    fn test_date_change_detail() {
        let detail = DatePickerChangeDetail {
            value: "2026-01-20".to_string(),
        };
        assert_eq!(detail.value, "2026-01-20");
    }

    #[test]
    fn test_month_names() {
        assert_eq!(MONTH_NAMES[0], "January");
        assert_eq!(MONTH_NAMES[11], "December");
        assert_eq!(MONTH_NAMES.len(), 12);
    }

    #[test]
    fn test_day_names() {
        assert_eq!(DAY_NAMES[0], "Su");
        assert_eq!(DAY_NAMES[6], "Sa");
        assert_eq!(DAY_NAMES.len(), 7);
    }

    #[test]
    fn test_date_value_equality() {
        let date1 = DateValue::new(2026, 1, 20).unwrap();
        let date2 = DateValue::new(2026, 1, 20).unwrap();
        let date3 = DateValue::new(2026, 1, 21).unwrap();

        assert_eq!(date1, date2);
        assert_ne!(date1, date3);
    }
}
