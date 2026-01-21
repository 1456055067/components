// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Auto-generated design tokens from style-dictionary
//! DO NOT EDIT MANUALLY

use std::fmt;

/// Color design tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorToken {
    ColorPrimary50,
    ColorPrimary100,
    ColorPrimary200,
    ColorPrimary300,
    ColorPrimary400,
    ColorPrimary500,
    ColorPrimary600,
    ColorPrimary700,
    ColorPrimary800,
    ColorPrimary900,
    ColorPrimary1000,
    ColorNeutral50,
    ColorNeutral100,
    ColorNeutral150,
    ColorNeutral200,
    ColorNeutral250,
    ColorNeutral300,
    ColorNeutral350,
    ColorNeutral400,
    ColorNeutral450,
    ColorNeutral500,
    ColorNeutral550,
    ColorNeutral600,
    ColorNeutral650,
    ColorNeutral700,
    ColorNeutral750,
    ColorNeutral800,
    ColorNeutral850,
    ColorNeutral900,
    ColorNeutral950,
    ColorNeutral1000,
    ColorError50,
    ColorError400,
    ColorError600,
    ColorError900,
    ColorError1000,
    ColorSuccess50,
    ColorSuccess500,
    ColorSuccess600,
    ColorSuccess1000,
    ColorWarning50,
    ColorWarning400,
    ColorWarning500,
    ColorWarning900,
    ColorWarning1000,
    ColorInfo50,
    ColorInfo300,
    ColorInfo400,
    ColorInfo600,
    ColorInfo1000,
    ColorGrey50,
    ColorGrey100,
    ColorGrey150,
    ColorGrey200,
    ColorGrey250,
    ColorGrey300,
    ColorGrey350,
    ColorGrey400,
    ColorGrey450,
    ColorGrey500,
    ColorGrey600,
    ColorGrey650,
    ColorGrey700,
    ColorGrey750,
    ColorGrey800,
    ColorGrey850,
    ColorGrey900,
    ColorGrey950,
    ColorGrey1000,
    ColorBlue50,
    ColorBlue100,
    ColorBlue200,
    ColorBlue300,
    ColorBlue400,
    ColorBlue600,
    ColorBlue700,
    ColorBlue900,
    ColorBlue1000,
    ColorGreen50,
    ColorGreen500,
    ColorGreen600,
    ColorGreen900,
    ColorGreen1000,
    ColorRed50,
    ColorRed400,
    ColorRed600,
    ColorRed900,
    ColorRed1000,
    ColorYellow50,
    ColorYellow400,
    ColorYellow500,
    ColorYellow900,
    ColorYellow1000,
    ColorPurple400,
    ColorPurple700,
    ColorAmber400,
    ColorAmber500,
    ColorAwsSquidInk,
    ColorTransparent,
    ColorBlack,
    ColorWhite,
    /// Color from the 'red' data visualization palette at a contrast ratio of 3:1
    ColorChartsRed300,
    /// Color from the 'red' data visualization palette at a contrast ratio of 4:1
    ColorChartsRed400,
    /// Color from the 'red' data visualization palette at a contrast ratio of 5:1
    ColorChartsRed500,
    /// Color from the 'red' data visualization palette at a contrast ratio of 6:1
    ColorChartsRed600,
    /// Color from the 'red' data visualization palette at a contrast ratio of 7:1
    ColorChartsRed700,
    /// Color from the 'red' data visualization palette at a contrast ratio of 8:1
    ColorChartsRed800,
    /// Color from the 'red' data visualization palette at a contrast ratio of 9:1
    ColorChartsRed900,
    /// Color from the 'red' data visualization palette at a contrast ratio of 10:1
    ColorChartsRed1000,
    /// Color from the 'red' data visualization palette at a contrast ratio of 11:1
    ColorChartsRed1100,
    /// Color from the 'red' data visualization palette at a contrast ratio of 12:1
    ColorChartsRed1200,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 3:1
    ColorChartsOrange300,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 4:1
    ColorChartsOrange400,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 5:1
    ColorChartsOrange500,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 6:1
    ColorChartsOrange600,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 7:1
    ColorChartsOrange700,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 8:1
    ColorChartsOrange800,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 9:1
    ColorChartsOrange900,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 10:1
    ColorChartsOrange1000,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 11:1
    ColorChartsOrange1100,
    /// Color from the 'orange' data visualization palette at a contrast ratio of 12:1
    ColorChartsOrange1200,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 3:1
    ColorChartsYellow300,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 4:1
    ColorChartsYellow400,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 5:1
    ColorChartsYellow500,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 6:1
    ColorChartsYellow600,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 7:1
    ColorChartsYellow700,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 8:1
    ColorChartsYellow800,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 9:1
    ColorChartsYellow900,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 10:1
    ColorChartsYellow1000,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 11:1
    ColorChartsYellow1100,
    /// Color from the 'yellow' data visualization palette at a contrast ratio of 12:1
    ColorChartsYellow1200,
    /// Color from the 'green' data visualization palette at a contrast ratio of 3:1
    ColorChartsGreen300,
    /// Color from the 'green' data visualization palette at a contrast ratio of 4:1
    ColorChartsGreen400,
    /// Color from the 'green' data visualization palette at a contrast ratio of 5:1
    ColorChartsGreen500,
    /// Color from the 'green' data visualization palette at a contrast ratio of 6:1
    ColorChartsGreen600,
    /// Color from the 'green' data visualization palette at a contrast ratio of 7:1
    ColorChartsGreen700,
    /// Color from the 'green' data visualization palette at a contrast ratio of 8:1
    ColorChartsGreen800,
    /// Color from the 'green' data visualization palette at a contrast ratio of 9:1
    ColorChartsGreen900,
    /// Color from the 'green' data visualization palette at a contrast ratio of 10:1
    ColorChartsGreen1000,
    /// Color from the 'green' data visualization palette at a contrast ratio of 11:1
    ColorChartsGreen1100,
    /// Color from the 'green' data visualization palette at a contrast ratio of 12:1
    ColorChartsGreen1200,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 3:1
    ColorChartsTeal300,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 4:1
    ColorChartsTeal400,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 5:1
    ColorChartsTeal500,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 6:1
    ColorChartsTeal600,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 7:1
    ColorChartsTeal700,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 8:1
    ColorChartsTeal800,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 9:1
    ColorChartsTeal900,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 10:1
    ColorChartsTeal1000,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 11:1
    ColorChartsTeal1100,
    /// Color from the 'teal' data visualization palette at a contrast ratio of 12:1
    ColorChartsTeal1200,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 3:1
    ColorChartsBlue1300,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 4:1
    ColorChartsBlue1400,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 5:1
    ColorChartsBlue1500,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 6:1
    ColorChartsBlue1600,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 7:1
    ColorChartsBlue1700,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 8:1
    ColorChartsBlue1800,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 9:1
    ColorChartsBlue1900,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 10:1
    ColorChartsBlue11000,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 11:1
    ColorChartsBlue11100,
    /// Color from the 'blue-1' data visualization palette at a contrast ratio of 12:1
    ColorChartsBlue11200,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 3:1
    ColorChartsBlue2300,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 4:1
    ColorChartsBlue2400,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 5:1
    ColorChartsBlue2500,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 6:1
    ColorChartsBlue2600,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 7:1
    ColorChartsBlue2700,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 8:1
    ColorChartsBlue2800,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 9:1
    ColorChartsBlue2900,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 10:1
    ColorChartsBlue21000,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 11:1
    ColorChartsBlue21100,
    /// Color from the 'blue-2' data visualization palette at a contrast ratio of 12:1
    ColorChartsBlue21200,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 3:1
    ColorChartsPurple300,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 4:1
    ColorChartsPurple400,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 5:1
    ColorChartsPurple500,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 6:1
    ColorChartsPurple600,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 7:1
    ColorChartsPurple700,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 8:1
    ColorChartsPurple800,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 9:1
    ColorChartsPurple900,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 10:1
    ColorChartsPurple1000,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 11:1
    ColorChartsPurple1100,
    /// Color from the 'purple' data visualization palette at a contrast ratio of 12:1
    ColorChartsPurple1200,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 3:1
    ColorChartsPink300,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 4:1
    ColorChartsPink400,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 5:1
    ColorChartsPink500,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 6:1
    ColorChartsPink600,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 7:1
    ColorChartsPink700,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 8:1
    ColorChartsPink800,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 9:1
    ColorChartsPink900,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 10:1
    ColorChartsPink1000,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 11:1
    ColorChartsPink1100,
    /// Color from the 'pink' data visualization palette at a contrast ratio of 12:1
    ColorChartsPink1200,
    /// Color to represent a critical error or a critically high-level of severity. For example: "Sev-1"
    ColorChartsStatusCritical,
    /// Color to represent an error status or a high-level of severity. Use this color to represent a default error status when there is only one applicable to a chart. For example: "Failed" or "Sev-2"
    ColorChartsStatusHigh,
    /// Color to represent a medium-level of severity. For example: "Sev-3"
    ColorChartsStatusMedium,
    /// Color to represent a warning or a low-level of severity. For example: "Warning" or "Sev-4"
    ColorChartsStatusLow,
    /// Color to represent a positive status. *For example: "Success" or "Running"
    ColorChartsStatusPositive,
    /// Color to represent an informational status. For example: "In-progress" or "Updating"
    ColorChartsStatusInfo,
    /// Color to represent a neutral status, a severity level of no impact, or the lowest-level of severity. For example: "Pending" or "Sev-5"
    ColorChartsStatusNeutral,
    /// The color to represent a threshold with a negative outcome. For example: A maximum limit
    ColorChartsThresholdNegative,
    /// The color to represent a threshold with a positive outcome. For example: A designated pass rate
    ColorChartsThresholdPositive,
    /// The color to represent an informational threshold to highlight special circumstances that may have or will occur. For example: A forecasted estimate
    ColorChartsThresholdInfo,
    /// The color to represent a threshold with a neutral outcome. For example: An average or baseline
    ColorChartsThresholdNeutral,
    /// Color of the grid lines in a chart.
    ColorChartsLineGrid,
    /// Color of the tick marks in a chart.
    ColorChartsLineTick,
    /// Color of the axis lines in a chart.
    ColorChartsLineAxis,
    /// Color #1 on the categorical data visualization palette.
    ColorChartsPaletteCategorical1,
    /// Color #2 on the categorical data visualization palette.
    ColorChartsPaletteCategorical2,
    /// Color #3 on the categorical data visualization palette.
    ColorChartsPaletteCategorical3,
    /// Color #4 on the categorical data visualization palette.
    ColorChartsPaletteCategorical4,
    /// Color #5 on the categorical data visualization palette.
    ColorChartsPaletteCategorical5,
    /// Color #6 on the categorical data visualization palette.
    ColorChartsPaletteCategorical6,
    /// Color #7 on the categorical data visualization palette.
    ColorChartsPaletteCategorical7,
    /// Color #8 on the categorical data visualization palette.
    ColorChartsPaletteCategorical8,
    /// Color #9 on the categorical data visualization palette.
    ColorChartsPaletteCategorical9,
    /// Color #10 on the categorical data visualization palette.
    ColorChartsPaletteCategorical10,
    /// Color #11 on the categorical data visualization palette.
    ColorChartsPaletteCategorical11,
    /// Color #12 on the categorical data visualization palette.
    ColorChartsPaletteCategorical12,
    /// Color #13 on the categorical data visualization palette.
    ColorChartsPaletteCategorical13,
    /// Color #14 on the categorical data visualization palette.
    ColorChartsPaletteCategorical14,
    /// Color #15 on the categorical data visualization palette.
    ColorChartsPaletteCategorical15,
    /// Color #16 on the categorical data visualization palette.
    ColorChartsPaletteCategorical16,
    /// Color #17 on the categorical data visualization palette.
    ColorChartsPaletteCategorical17,
    /// Color #18 on the categorical data visualization palette.
    ColorChartsPaletteCategorical18,
    /// Color #19 on the categorical data visualization palette.
    ColorChartsPaletteCategorical19,
    /// Color #20 on the categorical data visualization palette.
    ColorChartsPaletteCategorical20,
    /// Color #21 on the categorical data visualization palette.
    ColorChartsPaletteCategorical21,
    /// Color #22 on the categorical data visualization palette.
    ColorChartsPaletteCategorical22,
    /// Color #23 on the categorical data visualization palette.
    ColorChartsPaletteCategorical23,
    /// Color #24 on the categorical data visualization palette.
    ColorChartsPaletteCategorical24,
    /// Color #25 on the categorical data visualization palette.
    ColorChartsPaletteCategorical25,
    /// Color #26 on the categorical data visualization palette.
    ColorChartsPaletteCategorical26,
    /// Color #27 on the categorical data visualization palette.
    ColorChartsPaletteCategorical27,
    /// Color #28 on the categorical data visualization palette.
    ColorChartsPaletteCategorical28,
    /// Color #29 on the categorical data visualization palette.
    ColorChartsPaletteCategorical29,
    /// Color #30 on the categorical data visualization palette.
    ColorChartsPaletteCategorical30,
    /// Color #31 on the categorical data visualization palette.
    ColorChartsPaletteCategorical31,
    /// Color #32 on the categorical data visualization palette.
    ColorChartsPaletteCategorical32,
    /// Color #33 on the categorical data visualization palette.
    ColorChartsPaletteCategorical33,
    /// Color #34 on the categorical data visualization palette.
    ColorChartsPaletteCategorical34,
    /// Color #35 on the categorical data visualization palette.
    ColorChartsPaletteCategorical35,
    /// Color #36 on the categorical data visualization palette.
    ColorChartsPaletteCategorical36,
    /// Color #37 on the categorical data visualization palette.
    ColorChartsPaletteCategorical37,
    /// Color #38 on the categorical data visualization palette.
    ColorChartsPaletteCategorical38,
    /// Color #39 on the categorical data visualization palette.
    ColorChartsPaletteCategorical39,
    /// Color #40 on the categorical data visualization palette.
    ColorChartsPaletteCategorical40,
    /// Color #41 on the categorical data visualization palette.
    ColorChartsPaletteCategorical41,
    /// Color #42 on the categorical data visualization palette.
    ColorChartsPaletteCategorical42,
    /// Color #43 on the categorical data visualization palette.
    ColorChartsPaletteCategorical43,
    /// Color #44 on the categorical data visualization palette.
    ColorChartsPaletteCategorical44,
    /// Color #45 on the categorical data visualization palette.
    ColorChartsPaletteCategorical45,
    /// Color #46 on the categorical data visualization palette.
    ColorChartsPaletteCategorical46,
    /// Color #47 on the categorical data visualization palette.
    ColorChartsPaletteCategorical47,
    /// Color #48 on the categorical data visualization palette.
    ColorChartsPaletteCategorical48,
    /// Color #49 on the categorical data visualization palette.
    ColorChartsPaletteCategorical49,
    /// Color #50 on the categorical data visualization palette.
    ColorChartsPaletteCategorical50,
    /// Color for the error bar marker in charts.
    ColorChartsErrorBarMarker,
    ColorSeverityDarkRed,
    ColorSeverityRed,
    ColorSeverityOrange,
    ColorSeverityYellow,
    ColorSeverityGrey,
    /// Background color in a notification to represent a critical error or a critically high-level of severity. For example: "Sev-1"
    ColorBackgroundNotificationSeverityCritical,
    /// Background color in a notification to represent an error status or a high-level of severity. For example: "Failed" or "Sev-2"
    ColorBackgroundNotificationSeverityHigh,
    /// Background color in a notification to represent a medium-level of severity. For example: "Sev-3"
    ColorBackgroundNotificationSeverityMedium,
    /// Background color in a notification to represent a warning or a low-level of severity. For example: "Warning" or "Sev-4"
    ColorBackgroundNotificationSeverityLow,
    /// Background color in a notification to represent a neutral status, a severity level of no impact, or the lowest-level of severity. For example: "Pending" or "Sev-5"
    ColorBackgroundNotificationSeverityNeutral,
    /// Text color in a notification to represent a critical error or a critically high-level of severity. For example: "Sev-1"
    ColorTextNotificationSeverityCritical,
    /// Text color in a notification to represent an error status or a high-level of severity. For example: "Failed" or "Sev-2"
    ColorTextNotificationSeverityHigh,
    /// Text color in a notification to represent a medium-level of severity. For example: "Sev-3"
    ColorTextNotificationSeverityMedium,
    /// Text color in a notification to represent a warning or a low-level of severity. For example: "Warning" or "Sev-4"
    ColorTextNotificationSeverityLow,
    /// Text color in a notification to represent a neutral status, a severity level of no impact, or the lowest-level of severity. For example: "Pending" or "Sev-5"
    ColorTextNotificationSeverityNeutral,
    ColorGreyOpaque10,
    ColorGreyOpaque25,
    ColorGreyOpaque40,
    ColorGreyOpaque50,
    ColorGreyOpaque70,
    ColorGreyOpaque80,
    ColorGreyOpaque90,
    ColorGreyTransparent,
    ColorGreyTransparentHeavy,
    ColorGreyTransparentLight,
    ColorBackgroundBadgeIcon,
    /// The background color of link buttons in active state.
    ColorBackgroundButtonLinkActive,
    /// The background color of link buttons in hover state.
    ColorBackgroundButtonLinkHover,
    /// The background color of normal buttons in active state.
    ColorBackgroundButtonNormalActive,
    /// The default background color of normal buttons.
    ColorBackgroundButtonNormalDefault,
    /// The background color of normal buttons in disabled state.
    ColorBackgroundButtonNormalDisabled,
    /// The background color of normal buttons in hover state.
    ColorBackgroundButtonNormalHover,
    /// The background color of normal toggle buttons in pressed state.
    ColorBackgroundToggleButtonNormalPressed,
    /// The background color of primary buttons in active state.
    ColorBackgroundButtonPrimaryActive,
    /// The default background color of primary buttons.
    ColorBackgroundButtonPrimaryDefault,
    /// The background color of primary buttons in disabled state.
    ColorBackgroundButtonPrimaryDisabled,
    /// The background color of primary buttons in hover state.
    ColorBackgroundButtonPrimaryHover,
    ColorBackgroundDirectionButtonActive,
    ColorBackgroundDirectionButtonDefault,
    ColorBackgroundDirectionButtonDisabled,
    ColorBackgroundDirectionButtonHover,
    ColorTextDirectionButtonDefault,
    ColorTextDirectionButtonDisabled,
    ColorBackgroundCalendarCurrentDate,
    /// The background color of shaded table cells.
    ColorBackgroundCellShaded,
    ColorBackgroundCodeEditorGutterActiveLineDefault,
    ColorBackgroundCodeEditorGutterActiveLineError,
    ColorBackgroundCodeEditorGutterDefault,
    ColorBackgroundCodeEditorLoading,
    ColorBackgroundCodeEditorPaneItemHover,
    ColorBackgroundCodeEditorStatusBar,
    /// The background color of container main content areas. For example: content areas of form sections, containers, tables, and cards.
    ColorBackgroundContainerContent,
    /// The background color of container headers. For example: headers of form sections, containers, tables, and card collections.
    ColorBackgroundContainerHeader,
    /// The background color of a checked form control. For example: background fill of a selected radio button, checked checkbox, and toggle that is switched on.
    ColorBackgroundControlChecked,
    /// The default background color of form controls. For example: radio buttons and checkboxes default background fill color.
    ColorBackgroundControlDefault,
    /// The background color of a disabled form control. For example: background fill of a disabled radio button and checkbox.
    ColorBackgroundControlDisabled,
    /// The default background color of dropdown items. For example: select, multiselect, autosuggest, and datepicker dropdown backgrounds.
    ColorBackgroundDropdownItemDefault,
    ColorBackgroundDropdownItemDimmed,
    /// The background color of text that matches a user's query. For example: the background of text matching a query entered into a table filter, select, multiselect, or autosuggest.
    ColorBackgroundDropdownItemFilterMatch,
    /// The background color of dropdown items on hover. For example: background of hovered items in select, multiselect, autosuggest, and datepicker dropdowns.
    ColorBackgroundDropdownItemHover,
    /// The background color of selected dropdown items. For example: background of selected items in select, multiselect, autosuggest, and datepicker dropdowns.
    ColorBackgroundDropdownItemSelected,
    /// The background color of the home header, displayed on the Service's home page.
    ColorBackgroundHomeHeader,
    ColorBackgroundInlineCode,
    /// The default background color of form inputs. For example: background fill of an input, textarea, autosuggest, and trigger of a select and multiselect.
    ColorBackgroundInputDefault,
    /// The background color of a disabled form input. For example: background fill of a disabled input, textarea, autosuggest, and trigger of a select and multiselect.
    ColorBackgroundInputDisabled,
    /// The background color of a selected item. For example: tokens, selected table rows, cards, and tile backgrounds.
    ColorBackgroundItemSelected,
    /// The background color of the main content area on a page. For example: content area in app layout.
    ColorBackgroundLayoutMain,
    ColorBackgroundLayoutMobilePanel,
    /// The background color of app layout panel content area. For example: The side navigation and tools panel content background color.
    ColorBackgroundLayoutPanelContent,
    ColorBackgroundLayoutPanelHover,
    /// The background color of the app layout toggle button when it's active.
    ColorBackgroundLayoutToggleActive,
    /// The default background color of the app layout toggle button.
    ColorBackgroundLayoutToggleDefault,
    /// The background color of the app layout toggle button on hover.
    ColorBackgroundLayoutToggleHover,
    /// The background color of the app layout toggle button when it's selected and active.
    ColorBackgroundLayoutToggleSelectedActive,
    /// The default background color of the app layout toggle button when it's selected.
    ColorBackgroundLayoutToggleSelectedDefault,
    /// The background color of the app layout toggle button on hover when it's selected.
    ColorBackgroundLayoutToggleSelectedHover,
    ColorBackgroundModalOverlay,
    /// Background color for blue notifications. For example: blue badges and info flash messages.
    ColorBackgroundNotificationBlue,
    /// Background color for green notifications. For example: green badges and success flash messages.
    ColorBackgroundNotificationGreen,
    /// Background color for grey notifications. For example: grey badges.
    ColorBackgroundNotificationGrey,
    /// Background color for red notifications. For example: red badges and error flash messages.
    ColorBackgroundNotificationRed,
    /// Background color for yellow notifications. For example: yellow badges and warning flash messages.
    ColorBackgroundNotificationYellow,
    ColorBackgroundNotificationStackBar,
    ColorBackgroundNotificationStackBarActive,
    ColorBackgroundNotificationStackBarHover,
    /// Background color for the popover container.
    ColorBackgroundPopover,
    ColorBackgroundProgressBarContentDefault,
    ColorBackgroundProgressBarContentInFlash,
    ColorBackgroundProgressBarLayoutDefault,
    ColorBackgroundProgressBarLayoutInFlash,
    /// The background color of the active segment in a segmented control.
    ColorBackgroundSegmentActive,
    /// The background color of inactive segments in a segmented control.
    ColorBackgroundSegmentDefault,
    /// The background color of disabled segments in a segmented control.
    ColorBackgroundSegmentDisabled,
    /// The background color of inactive segments in a segmented control on hover.
    ColorBackgroundSegmentHover,
    /// The background color of segmented control wrapper.
    ColorBackgroundSegmentWrapper,
    /// The default background color of the slider range.
    ColorBackgroundSliderRangeDefault,
    /// The background color of the slider range in active state.
    ColorBackgroundSliderRangeActive,
    /// The default background color of the slider handle.
    ColorBackgroundSliderHandleDefault,
    /// The background color of the slider handle in active state.
    ColorBackgroundSliderHandleActive,
    /// The default background color of the slider track.
    ColorBackgroundSliderTrackDefault,
    ColorBackgroundSliderHandleRing,
    ColorBackgroundSliderHandleErrorDefault,
    ColorBackgroundSliderHandleErrorActive,
    ColorBackgroundSliderHandleWarningDefault,
    ColorBackgroundSliderHandleWarningActive,
    ColorBackgroundSliderRangeErrorDefault,
    ColorBackgroundSliderRangeErrorActive,
    ColorBackgroundSliderRangeWarningDefault,
    ColorBackgroundSliderRangeWarningActive,
    /// The background color of an item in error state. For example: error alerts.
    ColorBackgroundStatusError,
    /// The background color of an informational item. For example: information alerts.
    ColorBackgroundStatusInfo,
    /// The background color of the feedback/input dialogue box.
    ColorBackgroundDialog,
    /// The background color of an item in success state. For example: success alerts.
    ColorBackgroundStatusSuccess,
    /// The background color of an item in warning state. For example: warning alerts.
    ColorBackgroundStatusWarning,
    ColorBackgroundTableHeader,
    ColorBackgroundTilesDisabled,
    /// The background color of checked toggles in disabled state.
    ColorBackgroundToggleCheckedDisabled,
    ColorBackgroundToggleDefault,
    /// The gen-ai background color of avatars.
    ColorBackgroundAvatarGenAi,
    /// The default background color of avatars.
    ColorBackgroundAvatarDefault,
    /// The text and icon color of avatars.
    ColorTextAvatar,
    /// The background color of gen-ai loading bars.
    ColorBackgroundLoadingBarGenAi,
    /// The background color of `outgoing` chat bubble.
    ColorBackgroundChatBubbleOutgoing,
    /// The background color of `incoming` chat bubble.
    ColorBackgroundChatBubbleIncoming,
    /// Text color of `outgoing` chat bubble.
    ColorTextChatBubbleOutgoing,
    /// Text color of `incoming` chat bubble.
    ColorTextChatBubbleIncoming,
    /// The border color of normal buttons in active state.
    ColorBorderButtonNormalActive,
    /// The border color of normal buttons.
    ColorBorderButtonNormalDefault,
    /// The border color of normal toggle buttons in pressed state.
    ColorBorderToggleButtonNormalPressed,
    /// The border color of normal buttons in disabled state.
    ColorBorderButtonNormalDisabled,
    /// The text color of normal buttons in disabled state.
    ColorTextButtonNormalDisabled,
    /// The border color of normal buttons in hover state.
    ColorBorderButtonNormalHover,
    /// The color of icon buttons in disabled state.
    ColorTextButtonIconDisabled,
    /// The border color of primary buttons in disabled state.
    ColorBorderButtonPrimaryDisabled,
    /// The text color of primary buttons in disabled state.
    ColorTextButtonPrimaryDisabled,
    /// The highlight color for selected items. For example: borders of tokens and selected table rows, and check icons in selected dropdown items.
    ColorItemSelected,
    ColorBorderCalendarGrid,
    ColorBorderCalendarGridSelectedFocusRing,
    ColorBorderCellShaded,
    ColorBorderCodeEditorAceActiveLineLightTheme,
    ColorBorderCodeEditorAceActiveLineDarkTheme,
    ColorBorderCodeEditorDefault,
    ColorBorderCodeEditorPaneItemHover,
    ColorBorderContainerDivider,
    /// The top border color for containers and first item in dropdowns. For example: the top border of a card, dropdown, and table.
    ColorBorderContainerTop,
    ColorBorderControlChecked,
    /// The default border color of form controls. For example: radio buttons and checkboxes.
    ColorBorderControlDefault,
    /// The border color of disabled form controls. For example: disabled radio buttons and checkboxes.
    ColorBorderControlDisabled,
    ColorBorderDividerActive,
    /// The default color for dividers. For example: dividers in column layout, expanding sections, side nav, help panel, between table rows and dropdown items, and inside containers.
    ColorBorderDividerDefault,
    ColorBorderDividerPanelBottom,
    ColorBorderDividerPanelSide,
    /// The border color for row dividers. For example: row dividers for table and collection preferences.
    ColorBorderDividerSecondary,
    /// The border color of the dropdown container. For example: border color of the dropdown container in button dropdown, select, and multi-select.
    ColorBorderDropdownContainer,
    ColorBorderDropdownGroup,
    ColorBorderDropdownItemDefault,
    /// The border color of dropdown items on hover. For example: border of hovered items in select, multiselect, autosuggest, and hovered days in datepicker.
    ColorBorderDropdownItemHover,
    ColorBorderDropdownItemDimmedHover,
    ColorBorderDropdownItemSelected,
    ColorBorderDropdownItemTop,
    ColorBorderEditableCellHover,
    /// The default border color of form inputs. For example: input, textarea, autosuggest, datepicker, select, and multiselect.
    ColorBorderInputDefault,
    ColorBorderInputDisabled,
    /// The color of focus states for form inputs. For example: input, textarea, autosuggest, datepicker, select, and multiselect.
    ColorBorderInputFocused,
    /// The color of focus states. For example: the focus ring around interactive elements.
    ColorBorderItemFocused,
    /// The color of focus states for dropdown items. For example: the focus ring around selectable elements in the dropdown of button dropdown, select, and multi-select.
    ColorBorderDropdownItemFocused,
    /// The border color for dividers.
    ColorBorderItemPlaceholder,
    /// The border color of a selected item. For example: tokens, selected table rows, selected cards, and selected tile borders.
    ColorBorderItemSelected,
    ColorBorderLayout,
    ColorBorderNotificationStackBar,
    ColorBorderPanelHeader,
    /// The border color of the popover.
    ColorBorderPopover,
    /// Deprecated - this token is no longer in use.
    ColorBorderSegmentActive,
    /// Deprecated - this token is no longer in use.
    ColorBorderSegmentDefault,
    /// Deprecated - this token is no longer in use.
    ColorBorderSegmentDisabled,
    /// Deprecated - this token is no longer in use.
    ColorBorderSegmentHover,
    /// The border color of an item in error state. For example: error alerts.
    ColorBorderStatusError,
    /// The border color of an informational item. For example: information alerts.
    ColorBorderStatusInfo,
    /// The border color of an item in success state. For example: success alerts.
    ColorBorderStatusSuccess,
    /// The border color of an item in warning state. For example: warning alerts.
    ColorBorderStatusWarning,
    /// The border color of the feedback/input dialogue box.
    ColorBorderDialog,
    ColorBorderDividerInteractiveDefault,
    ColorBorderTabsDivider,
    ColorBorderTabsShadow,
    ColorBorderTabsUnderline,
    ColorBorderTilesDisabled,
    /// The border color of tutorials in the tutorials list in the tutorial panel.
    ColorBorderTutorial,
    /// The color used to mark enabled form controls. For example: the checkmark on checkboxes, inner circle on radio buttons, and handle on toggles.
    ColorForegroundControlDefault,
    /// The color used to mark disabled form controls. For example: the checkmark on checkboxes, inner circle on radio buttons, and handle on toggles.
    ColorForegroundControlDisabled,
    /// The color used to mark readonly form controls. For example: the checkmark on checkboxes, inner circle on radio buttons, and handle on toggles.
    ColorForegroundControlReadOnly,
    ColorShadowDefault,
    ColorShadowMedium,
    ColorShadowSide,
    ColorStrokeChartLine,
    ColorStrokeCodeEditorGutterActiveLineDefault,
    ColorStrokeCodeEditorGutterActiveLineHover,
    /// The accent text color used to guide a user. For example: the highlighted page in the side navigation, active tab text, selected day text color in date picker, and hover state in expandable section.
    ColorTextAccent,
    /// The default color of non-heading text and body content. For example: text in a paragraph tag, table cell data, form field labels and values.
    ColorTextBodyDefault,
    /// The color of text that is secondary to base text. For example: text in the navigation and tools panels.
    ColorTextBodySecondary,
    /// The text color that marks the breadcrumb item for the page the user is currently viewing.
    ColorTextBreadcrumbCurrent,
    /// The color used for the icon delimiter between breadcrumb items.
    ColorTextBreadcrumbIcon,
    ColorTextButtonInlineIconDefault,
    ColorTextButtonInlineIconDisabled,
    ColorTextButtonInlineIconHover,
    /// The active text color of normal buttons. For example: Active text color in normal and link buttons.
    ColorTextButtonNormalActive,
    /// The pressed text color of normal toggle buttons.
    ColorTextToggleButtonNormalPressed,
    /// The default text color of normal buttons.
    ColorTextButtonNormalDefault,
    /// The hover text color of normal buttons.
    ColorTextButtonNormalHover,
    /// The default text color of link buttons.
    ColorTextLinkButtonNormalDefault,
    /// The hover text color of link buttons.
    ColorTextLinkButtonNormalHover,
    /// The active text color of link buttons.
    ColorTextLinkButtonNormalActive,
    /// The active text color of primary buttons.
    ColorTextButtonPrimaryActive,
    /// The default text color of primary buttons.
    ColorTextButtonPrimaryDefault,
    /// The hover text color of primary buttons.
    ColorTextButtonPrimaryHover,
    ColorTextCalendarDateHover,
    ColorTextCalendarMonth,
    ColorTextCodeEditorGutterActiveLine,
    ColorTextCodeEditorGutterDefault,
    ColorTextCodeEditorStatusBarDisabled,
    ColorTextCodeEditorTabButtonError,
    ColorTextColumnHeader,
    ColorTextColumnSortingIcon,
    /// The text color of a disabled control. For example: the label and description on a disabled checkbox, radio button, or toggle.
    ColorTextControlDisabled,
    /// The default color for counters. For example: counters in table headers
    ColorTextCounter,
    ColorTextDisabled,
    ColorTextDisabledInlineEdit,
    /// The default color of dropdown footer text. For example: end of results text in an asynchrounous select and autosuggest.
    ColorTextDropdownFooter,
    ColorTextDropdownGroupLabel,
    /// The default text color of dropdown items. For example: label and label tag text color for autosuggest, select, and multiselect.
    ColorTextDropdownItemDefault,
    ColorTextDropdownItemDimmed,
    /// The text color of disabled dropdown items. For example: label, label tag, description, and tag text color of a disabled item in a select, multiselect, and autosuggest.
    ColorTextDropdownItemDisabled,
    /// The color of text matching a user's query. For example: the text matching a query entered into a table filter, select, multiselect, or autosuggest.
    ColorTextDropdownItemFilterMatch,
    /// The text color of hovered or selected dropdown items. For example: label text color of the item on hover in a select, multiselect, and autosuggest.
    ColorTextDropdownItemHighlighted,
    /// The text color of secondary information in dropdown items. For example: descriptions and tags text color in a select, multiselect, and autosuggest.
    ColorTextDropdownItemSecondary,
    ColorTextDropdownItemSecondaryHover,
    /// The color of text in non-dropdown empty states. For example: tables, card collections, and attribute editor empty state text.
    ColorTextEmpty,
    ColorTextExpandableSectionDefault,
    ColorTextExpandableSectionHover,
    ColorTextExpandableSectionNavigationIconDefault,
    /// The default color of form field labels and values. For example: the label in form fields, checkboxes, radio buttons, toggles, and the value in inputs and text areas.
    ColorTextFormDefault,
    /// Component specific: Special because control group is different.
    ColorTextFormLabel,
    /// The color of secondary text in form fields and controls. For example: the description and constraint text in form fields, the descriptions in checkboxes, radio buttons, toggles, and any additional info in an attribute editor.
    ColorTextFormSecondary,
    /// The default color for group labels. For example: group label in dropdown part of button dropdown, select, and multiselect, and group label in table and cards' preferences content selector.
    ColorTextGroupLabel,
    /// The default color for labels indicating that content is produced by generative AI.
    ColorTextLabelGenAi,
    /// The default color for headings 2-5. For example: headings in containers, form sections, forms, and app layout panels.
    ColorTextHeadingDefault,
    /// The default color for secondary heading text such as page and container descriptions. For example: descriptions in containers such as form sections, tables, and card collections, as well as form page descriptions.
    ColorTextHeadingSecondary,
    /// The color of the home header's text, displayed on the Service's home page.
    ColorTextHomeHeaderDefault,
    /// The color of the home header's secondary text, displayed on the Service's home page.
    ColorTextHomeHeaderSecondary,
    ColorTextIconCaret,
    ColorTextIconSubtle,
    /// The color of the text value in a disabled input. For example: text in a disabled input, autosuggest, datepicker, and the trigger of a select and multiselect.
    ColorTextInputDisabled,
    /// The color of placeholder text in an input. For example: placeholder text in an input, autosuggest, datepicker, and the trigger of a select and multiselect.
    ColorTextInputPlaceholder,
    ColorTextInputPlaceholderDisabled,
    /// The color of clickable elements in their active state. For example: tabs and icons.
    ColorTextInteractiveActive,
    /// The color of clickable elements in their default state. For example: tabs, and icons.
    ColorTextInteractiveDefault,
    /// The color of clickable elements in their disabled state. For example: disabled tabs, button text, and icons.
    ColorTextInteractiveDisabled,
    /// The color of clickable elements on hover. For example: icons on hover.
    ColorTextInteractiveHover,
    /// The pressed text color of icon toggle buttons.
    ColorTextToggleButtonIconPressed,
    /// The default color of clickable elements in the flashbar. For example: The dismiss icon button in a flashbar.
    ColorTextInteractiveInvertedDefault,
    /// The hover color of clickable elements in the flashbar. For example: The dismiss icon button in a flashbar.
    ColorTextInteractiveInvertedHover,
    ColorTextInverted,
    /// The default color for non-form labels. For example: the key in key/value pairs and card's sections labels.
    ColorTextLabel,
    /// The default color of the app layout toggle.
    ColorTextLayoutToggle,
    /// The color of the app layout toggle button when it's active.
    ColorTextLayoutToggleActive,
    /// The color of the app layout toggle button on hover.
    ColorTextLayoutToggleHover,
    /// The color of the app layout toggle button when it's selected.
    ColorTextLayoutToggleSelected,
    /// The default color for links. For example: text in an anchor tag, info links, breadcrumb links, and icon links.
    ColorTextLinkDefault,
    /// The hover color for links. For example: text in an anchor tag, info links, breadcrumb links, and icon links.
    ColorTextLinkHover,
    ColorTextLinkInvertedHover,
    ColorTextLinkButtonUnderline,
    ColorTextLinkButtonUnderlineHover,
    /// Default text color for notifications. For example: the text on badges and flashes.
    ColorTextNotificationDefault,
    ColorTextNotificationStackBar,
    ColorTextNotificationYellow,
    ColorTextPaginationPageNumberActiveDisabled,
    ColorTextPaginationPageNumberDefault,
    /// The text color of the active segment in a segmented control.
    ColorTextSegmentActive,
    /// The text color of inactive segments in a segmented control.
    ColorTextSegmentDefault,
    /// The text color of inactive segments in a segmented control on hover.
    ColorTextSegmentHover,
    ColorTextSmall,
    /// The color of error text and icons. For example: form field error text and error status indicators.
    ColorTextStatusError,
    /// The color of inactive and loading text and icons. For example: table and card collection loading states icon and text and inactive and pending status indicators.
    ColorTextStatusInactive,
    /// The color of info text and icons. For example: info status indicators and info alert icon.
    ColorTextStatusInfo,
    /// The color of success text and icons. For example: success status indicators and success alert icon.
    ColorTextStatusSuccess,
    /// The color of warning icons.
    ColorTextStatusWarning,
    /// The color of the title in the top navigation.
    ColorTextTopNavigationTitle,
    ColorTextTutorialHotspotDefault,
    ColorTextTutorialHotspotHover,
    /// The color of board placeholder in active state.
    ColorBoardPlaceholderActive,
    /// The color of board placeholder in hovered state.
    ColorBoardPlaceholderHover,
    /// The color of drag placeholder in active state.
    ColorDragPlaceholderActive,
    /// The color of drag placeholder in hovered state.
    ColorDragPlaceholderHover,
    /// The default color of file upload dropzone background.
    ColorDropzoneBackgroundDefault,
    /// The color of file upload dropzone background in hovered state.
    ColorDropzoneBackgroundHover,
    /// The default color of file upload dropzone text.
    ColorDropzoneTextDefault,
    /// The color of file upload dropzone text in hovered state.
    ColorDropzoneTextHover,
    /// The default color of file upload dropzone border.
    ColorDropzoneBorderDefault,
    /// The color of file upload dropzone border in hovered state.
    ColorDropzoneBorderHover,
    ColorGapGlobalDrawer,
    /// The color of the tree view connector lines.
    ColorTreeViewConnectorLine,
}

impl ColorToken {
    /// Get the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
            Self::ColorPrimary50 => "--awsui-color-primary50",
            Self::ColorPrimary100 => "--awsui-color-primary100",
            Self::ColorPrimary200 => "--awsui-color-primary200",
            Self::ColorPrimary300 => "--awsui-color-primary300",
            Self::ColorPrimary400 => "--awsui-color-primary400",
            Self::ColorPrimary500 => "--awsui-color-primary500",
            Self::ColorPrimary600 => "--awsui-color-primary600",
            Self::ColorPrimary700 => "--awsui-color-primary700",
            Self::ColorPrimary800 => "--awsui-color-primary800",
            Self::ColorPrimary900 => "--awsui-color-primary900",
            Self::ColorPrimary1000 => "--awsui-color-primary1000",
            Self::ColorNeutral50 => "--awsui-color-neutral50",
            Self::ColorNeutral100 => "--awsui-color-neutral100",
            Self::ColorNeutral150 => "--awsui-color-neutral150",
            Self::ColorNeutral200 => "--awsui-color-neutral200",
            Self::ColorNeutral250 => "--awsui-color-neutral250",
            Self::ColorNeutral300 => "--awsui-color-neutral300",
            Self::ColorNeutral350 => "--awsui-color-neutral350",
            Self::ColorNeutral400 => "--awsui-color-neutral400",
            Self::ColorNeutral450 => "--awsui-color-neutral450",
            Self::ColorNeutral500 => "--awsui-color-neutral500",
            Self::ColorNeutral550 => "--awsui-color-neutral550",
            Self::ColorNeutral600 => "--awsui-color-neutral600",
            Self::ColorNeutral650 => "--awsui-color-neutral650",
            Self::ColorNeutral700 => "--awsui-color-neutral700",
            Self::ColorNeutral750 => "--awsui-color-neutral750",
            Self::ColorNeutral800 => "--awsui-color-neutral800",
            Self::ColorNeutral850 => "--awsui-color-neutral850",
            Self::ColorNeutral900 => "--awsui-color-neutral900",
            Self::ColorNeutral950 => "--awsui-color-neutral950",
            Self::ColorNeutral1000 => "--awsui-color-neutral1000",
            Self::ColorError50 => "--awsui-color-error50",
            Self::ColorError400 => "--awsui-color-error400",
            Self::ColorError600 => "--awsui-color-error600",
            Self::ColorError900 => "--awsui-color-error900",
            Self::ColorError1000 => "--awsui-color-error1000",
            Self::ColorSuccess50 => "--awsui-color-success50",
            Self::ColorSuccess500 => "--awsui-color-success500",
            Self::ColorSuccess600 => "--awsui-color-success600",
            Self::ColorSuccess1000 => "--awsui-color-success1000",
            Self::ColorWarning50 => "--awsui-color-warning50",
            Self::ColorWarning400 => "--awsui-color-warning400",
            Self::ColorWarning500 => "--awsui-color-warning500",
            Self::ColorWarning900 => "--awsui-color-warning900",
            Self::ColorWarning1000 => "--awsui-color-warning1000",
            Self::ColorInfo50 => "--awsui-color-info50",
            Self::ColorInfo300 => "--awsui-color-info300",
            Self::ColorInfo400 => "--awsui-color-info400",
            Self::ColorInfo600 => "--awsui-color-info600",
            Self::ColorInfo1000 => "--awsui-color-info1000",
            Self::ColorGrey50 => "--awsui-color-grey50",
            Self::ColorGrey100 => "--awsui-color-grey100",
            Self::ColorGrey150 => "--awsui-color-grey150",
            Self::ColorGrey200 => "--awsui-color-grey200",
            Self::ColorGrey250 => "--awsui-color-grey250",
            Self::ColorGrey300 => "--awsui-color-grey300",
            Self::ColorGrey350 => "--awsui-color-grey350",
            Self::ColorGrey400 => "--awsui-color-grey400",
            Self::ColorGrey450 => "--awsui-color-grey450",
            Self::ColorGrey500 => "--awsui-color-grey500",
            Self::ColorGrey600 => "--awsui-color-grey600",
            Self::ColorGrey650 => "--awsui-color-grey650",
            Self::ColorGrey700 => "--awsui-color-grey700",
            Self::ColorGrey750 => "--awsui-color-grey750",
            Self::ColorGrey800 => "--awsui-color-grey800",
            Self::ColorGrey850 => "--awsui-color-grey850",
            Self::ColorGrey900 => "--awsui-color-grey900",
            Self::ColorGrey950 => "--awsui-color-grey950",
            Self::ColorGrey1000 => "--awsui-color-grey1000",
            Self::ColorBlue50 => "--awsui-color-blue50",
            Self::ColorBlue100 => "--awsui-color-blue100",
            Self::ColorBlue200 => "--awsui-color-blue200",
            Self::ColorBlue300 => "--awsui-color-blue300",
            Self::ColorBlue400 => "--awsui-color-blue400",
            Self::ColorBlue600 => "--awsui-color-blue600",
            Self::ColorBlue700 => "--awsui-color-blue700",
            Self::ColorBlue900 => "--awsui-color-blue900",
            Self::ColorBlue1000 => "--awsui-color-blue1000",
            Self::ColorGreen50 => "--awsui-color-green50",
            Self::ColorGreen500 => "--awsui-color-green500",
            Self::ColorGreen600 => "--awsui-color-green600",
            Self::ColorGreen900 => "--awsui-color-green900",
            Self::ColorGreen1000 => "--awsui-color-green1000",
            Self::ColorRed50 => "--awsui-color-red50",
            Self::ColorRed400 => "--awsui-color-red400",
            Self::ColorRed600 => "--awsui-color-red600",
            Self::ColorRed900 => "--awsui-color-red900",
            Self::ColorRed1000 => "--awsui-color-red1000",
            Self::ColorYellow50 => "--awsui-color-yellow50",
            Self::ColorYellow400 => "--awsui-color-yellow400",
            Self::ColorYellow500 => "--awsui-color-yellow500",
            Self::ColorYellow900 => "--awsui-color-yellow900",
            Self::ColorYellow1000 => "--awsui-color-yellow1000",
            Self::ColorPurple400 => "--awsui-color-purple400",
            Self::ColorPurple700 => "--awsui-color-purple700",
            Self::ColorAmber400 => "--awsui-color-amber400",
            Self::ColorAmber500 => "--awsui-color-amber500",
            Self::ColorAwsSquidInk => "--awsui-color-aws-squid-ink",
            Self::ColorTransparent => "--awsui-color-transparent",
            Self::ColorBlack => "--awsui-color-black",
            Self::ColorWhite => "--awsui-color-white",
            Self::ColorChartsRed300 => "--awsui-color-charts-red300",
            Self::ColorChartsRed400 => "--awsui-color-charts-red400",
            Self::ColorChartsRed500 => "--awsui-color-charts-red500",
            Self::ColorChartsRed600 => "--awsui-color-charts-red600",
            Self::ColorChartsRed700 => "--awsui-color-charts-red700",
            Self::ColorChartsRed800 => "--awsui-color-charts-red800",
            Self::ColorChartsRed900 => "--awsui-color-charts-red900",
            Self::ColorChartsRed1000 => "--awsui-color-charts-red1000",
            Self::ColorChartsRed1100 => "--awsui-color-charts-red1100",
            Self::ColorChartsRed1200 => "--awsui-color-charts-red1200",
            Self::ColorChartsOrange300 => "--awsui-color-charts-orange300",
            Self::ColorChartsOrange400 => "--awsui-color-charts-orange400",
            Self::ColorChartsOrange500 => "--awsui-color-charts-orange500",
            Self::ColorChartsOrange600 => "--awsui-color-charts-orange600",
            Self::ColorChartsOrange700 => "--awsui-color-charts-orange700",
            Self::ColorChartsOrange800 => "--awsui-color-charts-orange800",
            Self::ColorChartsOrange900 => "--awsui-color-charts-orange900",
            Self::ColorChartsOrange1000 => "--awsui-color-charts-orange1000",
            Self::ColorChartsOrange1100 => "--awsui-color-charts-orange1100",
            Self::ColorChartsOrange1200 => "--awsui-color-charts-orange1200",
            Self::ColorChartsYellow300 => "--awsui-color-charts-yellow300",
            Self::ColorChartsYellow400 => "--awsui-color-charts-yellow400",
            Self::ColorChartsYellow500 => "--awsui-color-charts-yellow500",
            Self::ColorChartsYellow600 => "--awsui-color-charts-yellow600",
            Self::ColorChartsYellow700 => "--awsui-color-charts-yellow700",
            Self::ColorChartsYellow800 => "--awsui-color-charts-yellow800",
            Self::ColorChartsYellow900 => "--awsui-color-charts-yellow900",
            Self::ColorChartsYellow1000 => "--awsui-color-charts-yellow1000",
            Self::ColorChartsYellow1100 => "--awsui-color-charts-yellow1100",
            Self::ColorChartsYellow1200 => "--awsui-color-charts-yellow1200",
            Self::ColorChartsGreen300 => "--awsui-color-charts-green300",
            Self::ColorChartsGreen400 => "--awsui-color-charts-green400",
            Self::ColorChartsGreen500 => "--awsui-color-charts-green500",
            Self::ColorChartsGreen600 => "--awsui-color-charts-green600",
            Self::ColorChartsGreen700 => "--awsui-color-charts-green700",
            Self::ColorChartsGreen800 => "--awsui-color-charts-green800",
            Self::ColorChartsGreen900 => "--awsui-color-charts-green900",
            Self::ColorChartsGreen1000 => "--awsui-color-charts-green1000",
            Self::ColorChartsGreen1100 => "--awsui-color-charts-green1100",
            Self::ColorChartsGreen1200 => "--awsui-color-charts-green1200",
            Self::ColorChartsTeal300 => "--awsui-color-charts-teal300",
            Self::ColorChartsTeal400 => "--awsui-color-charts-teal400",
            Self::ColorChartsTeal500 => "--awsui-color-charts-teal500",
            Self::ColorChartsTeal600 => "--awsui-color-charts-teal600",
            Self::ColorChartsTeal700 => "--awsui-color-charts-teal700",
            Self::ColorChartsTeal800 => "--awsui-color-charts-teal800",
            Self::ColorChartsTeal900 => "--awsui-color-charts-teal900",
            Self::ColorChartsTeal1000 => "--awsui-color-charts-teal1000",
            Self::ColorChartsTeal1100 => "--awsui-color-charts-teal1100",
            Self::ColorChartsTeal1200 => "--awsui-color-charts-teal1200",
            Self::ColorChartsBlue1300 => "--awsui-color-charts-blue1300",
            Self::ColorChartsBlue1400 => "--awsui-color-charts-blue1400",
            Self::ColorChartsBlue1500 => "--awsui-color-charts-blue1500",
            Self::ColorChartsBlue1600 => "--awsui-color-charts-blue1600",
            Self::ColorChartsBlue1700 => "--awsui-color-charts-blue1700",
            Self::ColorChartsBlue1800 => "--awsui-color-charts-blue1800",
            Self::ColorChartsBlue1900 => "--awsui-color-charts-blue1900",
            Self::ColorChartsBlue11000 => "--awsui-color-charts-blue11000",
            Self::ColorChartsBlue11100 => "--awsui-color-charts-blue11100",
            Self::ColorChartsBlue11200 => "--awsui-color-charts-blue11200",
            Self::ColorChartsBlue2300 => "--awsui-color-charts-blue2300",
            Self::ColorChartsBlue2400 => "--awsui-color-charts-blue2400",
            Self::ColorChartsBlue2500 => "--awsui-color-charts-blue2500",
            Self::ColorChartsBlue2600 => "--awsui-color-charts-blue2600",
            Self::ColorChartsBlue2700 => "--awsui-color-charts-blue2700",
            Self::ColorChartsBlue2800 => "--awsui-color-charts-blue2800",
            Self::ColorChartsBlue2900 => "--awsui-color-charts-blue2900",
            Self::ColorChartsBlue21000 => "--awsui-color-charts-blue21000",
            Self::ColorChartsBlue21100 => "--awsui-color-charts-blue21100",
            Self::ColorChartsBlue21200 => "--awsui-color-charts-blue21200",
            Self::ColorChartsPurple300 => "--awsui-color-charts-purple300",
            Self::ColorChartsPurple400 => "--awsui-color-charts-purple400",
            Self::ColorChartsPurple500 => "--awsui-color-charts-purple500",
            Self::ColorChartsPurple600 => "--awsui-color-charts-purple600",
            Self::ColorChartsPurple700 => "--awsui-color-charts-purple700",
            Self::ColorChartsPurple800 => "--awsui-color-charts-purple800",
            Self::ColorChartsPurple900 => "--awsui-color-charts-purple900",
            Self::ColorChartsPurple1000 => "--awsui-color-charts-purple1000",
            Self::ColorChartsPurple1100 => "--awsui-color-charts-purple1100",
            Self::ColorChartsPurple1200 => "--awsui-color-charts-purple1200",
            Self::ColorChartsPink300 => "--awsui-color-charts-pink300",
            Self::ColorChartsPink400 => "--awsui-color-charts-pink400",
            Self::ColorChartsPink500 => "--awsui-color-charts-pink500",
            Self::ColorChartsPink600 => "--awsui-color-charts-pink600",
            Self::ColorChartsPink700 => "--awsui-color-charts-pink700",
            Self::ColorChartsPink800 => "--awsui-color-charts-pink800",
            Self::ColorChartsPink900 => "--awsui-color-charts-pink900",
            Self::ColorChartsPink1000 => "--awsui-color-charts-pink1000",
            Self::ColorChartsPink1100 => "--awsui-color-charts-pink1100",
            Self::ColorChartsPink1200 => "--awsui-color-charts-pink1200",
            Self::ColorChartsStatusCritical => "--awsui-color-charts-status-critical",
            Self::ColorChartsStatusHigh => "--awsui-color-charts-status-high",
            Self::ColorChartsStatusMedium => "--awsui-color-charts-status-medium",
            Self::ColorChartsStatusLow => "--awsui-color-charts-status-low",
            Self::ColorChartsStatusPositive => "--awsui-color-charts-status-positive",
            Self::ColorChartsStatusInfo => "--awsui-color-charts-status-info",
            Self::ColorChartsStatusNeutral => "--awsui-color-charts-status-neutral",
            Self::ColorChartsThresholdNegative => "--awsui-color-charts-threshold-negative",
            Self::ColorChartsThresholdPositive => "--awsui-color-charts-threshold-positive",
            Self::ColorChartsThresholdInfo => "--awsui-color-charts-threshold-info",
            Self::ColorChartsThresholdNeutral => "--awsui-color-charts-threshold-neutral",
            Self::ColorChartsLineGrid => "--awsui-color-charts-line-grid",
            Self::ColorChartsLineTick => "--awsui-color-charts-line-tick",
            Self::ColorChartsLineAxis => "--awsui-color-charts-line-axis",
            Self::ColorChartsPaletteCategorical1 => "--awsui-color-charts-palette-categorical1",
            Self::ColorChartsPaletteCategorical2 => "--awsui-color-charts-palette-categorical2",
            Self::ColorChartsPaletteCategorical3 => "--awsui-color-charts-palette-categorical3",
            Self::ColorChartsPaletteCategorical4 => "--awsui-color-charts-palette-categorical4",
            Self::ColorChartsPaletteCategorical5 => "--awsui-color-charts-palette-categorical5",
            Self::ColorChartsPaletteCategorical6 => "--awsui-color-charts-palette-categorical6",
            Self::ColorChartsPaletteCategorical7 => "--awsui-color-charts-palette-categorical7",
            Self::ColorChartsPaletteCategorical8 => "--awsui-color-charts-palette-categorical8",
            Self::ColorChartsPaletteCategorical9 => "--awsui-color-charts-palette-categorical9",
            Self::ColorChartsPaletteCategorical10 => "--awsui-color-charts-palette-categorical10",
            Self::ColorChartsPaletteCategorical11 => "--awsui-color-charts-palette-categorical11",
            Self::ColorChartsPaletteCategorical12 => "--awsui-color-charts-palette-categorical12",
            Self::ColorChartsPaletteCategorical13 => "--awsui-color-charts-palette-categorical13",
            Self::ColorChartsPaletteCategorical14 => "--awsui-color-charts-palette-categorical14",
            Self::ColorChartsPaletteCategorical15 => "--awsui-color-charts-palette-categorical15",
            Self::ColorChartsPaletteCategorical16 => "--awsui-color-charts-palette-categorical16",
            Self::ColorChartsPaletteCategorical17 => "--awsui-color-charts-palette-categorical17",
            Self::ColorChartsPaletteCategorical18 => "--awsui-color-charts-palette-categorical18",
            Self::ColorChartsPaletteCategorical19 => "--awsui-color-charts-palette-categorical19",
            Self::ColorChartsPaletteCategorical20 => "--awsui-color-charts-palette-categorical20",
            Self::ColorChartsPaletteCategorical21 => "--awsui-color-charts-palette-categorical21",
            Self::ColorChartsPaletteCategorical22 => "--awsui-color-charts-palette-categorical22",
            Self::ColorChartsPaletteCategorical23 => "--awsui-color-charts-palette-categorical23",
            Self::ColorChartsPaletteCategorical24 => "--awsui-color-charts-palette-categorical24",
            Self::ColorChartsPaletteCategorical25 => "--awsui-color-charts-palette-categorical25",
            Self::ColorChartsPaletteCategorical26 => "--awsui-color-charts-palette-categorical26",
            Self::ColorChartsPaletteCategorical27 => "--awsui-color-charts-palette-categorical27",
            Self::ColorChartsPaletteCategorical28 => "--awsui-color-charts-palette-categorical28",
            Self::ColorChartsPaletteCategorical29 => "--awsui-color-charts-palette-categorical29",
            Self::ColorChartsPaletteCategorical30 => "--awsui-color-charts-palette-categorical30",
            Self::ColorChartsPaletteCategorical31 => "--awsui-color-charts-palette-categorical31",
            Self::ColorChartsPaletteCategorical32 => "--awsui-color-charts-palette-categorical32",
            Self::ColorChartsPaletteCategorical33 => "--awsui-color-charts-palette-categorical33",
            Self::ColorChartsPaletteCategorical34 => "--awsui-color-charts-palette-categorical34",
            Self::ColorChartsPaletteCategorical35 => "--awsui-color-charts-palette-categorical35",
            Self::ColorChartsPaletteCategorical36 => "--awsui-color-charts-palette-categorical36",
            Self::ColorChartsPaletteCategorical37 => "--awsui-color-charts-palette-categorical37",
            Self::ColorChartsPaletteCategorical38 => "--awsui-color-charts-palette-categorical38",
            Self::ColorChartsPaletteCategorical39 => "--awsui-color-charts-palette-categorical39",
            Self::ColorChartsPaletteCategorical40 => "--awsui-color-charts-palette-categorical40",
            Self::ColorChartsPaletteCategorical41 => "--awsui-color-charts-palette-categorical41",
            Self::ColorChartsPaletteCategorical42 => "--awsui-color-charts-palette-categorical42",
            Self::ColorChartsPaletteCategorical43 => "--awsui-color-charts-palette-categorical43",
            Self::ColorChartsPaletteCategorical44 => "--awsui-color-charts-palette-categorical44",
            Self::ColorChartsPaletteCategorical45 => "--awsui-color-charts-palette-categorical45",
            Self::ColorChartsPaletteCategorical46 => "--awsui-color-charts-palette-categorical46",
            Self::ColorChartsPaletteCategorical47 => "--awsui-color-charts-palette-categorical47",
            Self::ColorChartsPaletteCategorical48 => "--awsui-color-charts-palette-categorical48",
            Self::ColorChartsPaletteCategorical49 => "--awsui-color-charts-palette-categorical49",
            Self::ColorChartsPaletteCategorical50 => "--awsui-color-charts-palette-categorical50",
            Self::ColorChartsErrorBarMarker => "--awsui-color-charts-error-bar-marker",
            Self::ColorSeverityDarkRed => "--awsui-color-severity-dark-red",
            Self::ColorSeverityRed => "--awsui-color-severity-red",
            Self::ColorSeverityOrange => "--awsui-color-severity-orange",
            Self::ColorSeverityYellow => "--awsui-color-severity-yellow",
            Self::ColorSeverityGrey => "--awsui-color-severity-grey",
            Self::ColorBackgroundNotificationSeverityCritical => "--awsui-color-background-notification-severity-critical",
            Self::ColorBackgroundNotificationSeverityHigh => "--awsui-color-background-notification-severity-high",
            Self::ColorBackgroundNotificationSeverityMedium => "--awsui-color-background-notification-severity-medium",
            Self::ColorBackgroundNotificationSeverityLow => "--awsui-color-background-notification-severity-low",
            Self::ColorBackgroundNotificationSeverityNeutral => "--awsui-color-background-notification-severity-neutral",
            Self::ColorTextNotificationSeverityCritical => "--awsui-color-text-notification-severity-critical",
            Self::ColorTextNotificationSeverityHigh => "--awsui-color-text-notification-severity-high",
            Self::ColorTextNotificationSeverityMedium => "--awsui-color-text-notification-severity-medium",
            Self::ColorTextNotificationSeverityLow => "--awsui-color-text-notification-severity-low",
            Self::ColorTextNotificationSeverityNeutral => "--awsui-color-text-notification-severity-neutral",
            Self::ColorGreyOpaque10 => "--awsui-color-grey-opaque10",
            Self::ColorGreyOpaque25 => "--awsui-color-grey-opaque25",
            Self::ColorGreyOpaque40 => "--awsui-color-grey-opaque40",
            Self::ColorGreyOpaque50 => "--awsui-color-grey-opaque50",
            Self::ColorGreyOpaque70 => "--awsui-color-grey-opaque70",
            Self::ColorGreyOpaque80 => "--awsui-color-grey-opaque80",
            Self::ColorGreyOpaque90 => "--awsui-color-grey-opaque90",
            Self::ColorGreyTransparent => "--awsui-color-grey-transparent",
            Self::ColorGreyTransparentHeavy => "--awsui-color-grey-transparent-heavy",
            Self::ColorGreyTransparentLight => "--awsui-color-grey-transparent-light",
            Self::ColorBackgroundBadgeIcon => "--awsui-color-background-badge-icon",
            Self::ColorBackgroundButtonLinkActive => "--awsui-color-background-button-link-active",
            Self::ColorBackgroundButtonLinkHover => "--awsui-color-background-button-link-hover",
            Self::ColorBackgroundButtonNormalActive => "--awsui-color-background-button-normal-active",
            Self::ColorBackgroundButtonNormalDefault => "--awsui-color-background-button-normal-default",
            Self::ColorBackgroundButtonNormalDisabled => "--awsui-color-background-button-normal-disabled",
            Self::ColorBackgroundButtonNormalHover => "--awsui-color-background-button-normal-hover",
            Self::ColorBackgroundToggleButtonNormalPressed => "--awsui-color-background-toggle-button-normal-pressed",
            Self::ColorBackgroundButtonPrimaryActive => "--awsui-color-background-button-primary-active",
            Self::ColorBackgroundButtonPrimaryDefault => "--awsui-color-background-button-primary-default",
            Self::ColorBackgroundButtonPrimaryDisabled => "--awsui-color-background-button-primary-disabled",
            Self::ColorBackgroundButtonPrimaryHover => "--awsui-color-background-button-primary-hover",
            Self::ColorBackgroundDirectionButtonActive => "--awsui-color-background-direction-button-active",
            Self::ColorBackgroundDirectionButtonDefault => "--awsui-color-background-direction-button-default",
            Self::ColorBackgroundDirectionButtonDisabled => "--awsui-color-background-direction-button-disabled",
            Self::ColorBackgroundDirectionButtonHover => "--awsui-color-background-direction-button-hover",
            Self::ColorTextDirectionButtonDefault => "--awsui-color-text-direction-button-default",
            Self::ColorTextDirectionButtonDisabled => "--awsui-color-text-direction-button-disabled",
            Self::ColorBackgroundCalendarCurrentDate => "--awsui-color-background-calendar-current-date",
            Self::ColorBackgroundCellShaded => "--awsui-color-background-cell-shaded",
            Self::ColorBackgroundCodeEditorGutterActiveLineDefault => "--awsui-color-background-code-editor-gutter-active-line-default",
            Self::ColorBackgroundCodeEditorGutterActiveLineError => "--awsui-color-background-code-editor-gutter-active-line-error",
            Self::ColorBackgroundCodeEditorGutterDefault => "--awsui-color-background-code-editor-gutter-default",
            Self::ColorBackgroundCodeEditorLoading => "--awsui-color-background-code-editor-loading",
            Self::ColorBackgroundCodeEditorPaneItemHover => "--awsui-color-background-code-editor-pane-item-hover",
            Self::ColorBackgroundCodeEditorStatusBar => "--awsui-color-background-code-editor-status-bar",
            Self::ColorBackgroundContainerContent => "--awsui-color-background-container-content",
            Self::ColorBackgroundContainerHeader => "--awsui-color-background-container-header",
            Self::ColorBackgroundControlChecked => "--awsui-color-background-control-checked",
            Self::ColorBackgroundControlDefault => "--awsui-color-background-control-default",
            Self::ColorBackgroundControlDisabled => "--awsui-color-background-control-disabled",
            Self::ColorBackgroundDropdownItemDefault => "--awsui-color-background-dropdown-item-default",
            Self::ColorBackgroundDropdownItemDimmed => "--awsui-color-background-dropdown-item-dimmed",
            Self::ColorBackgroundDropdownItemFilterMatch => "--awsui-color-background-dropdown-item-filter-match",
            Self::ColorBackgroundDropdownItemHover => "--awsui-color-background-dropdown-item-hover",
            Self::ColorBackgroundDropdownItemSelected => "--awsui-color-background-dropdown-item-selected",
            Self::ColorBackgroundHomeHeader => "--awsui-color-background-home-header",
            Self::ColorBackgroundInlineCode => "--awsui-color-background-inline-code",
            Self::ColorBackgroundInputDefault => "--awsui-color-background-input-default",
            Self::ColorBackgroundInputDisabled => "--awsui-color-background-input-disabled",
            Self::ColorBackgroundItemSelected => "--awsui-color-background-item-selected",
            Self::ColorBackgroundLayoutMain => "--awsui-color-background-layout-main",
            Self::ColorBackgroundLayoutMobilePanel => "--awsui-color-background-layout-mobile-panel",
            Self::ColorBackgroundLayoutPanelContent => "--awsui-color-background-layout-panel-content",
            Self::ColorBackgroundLayoutPanelHover => "--awsui-color-background-layout-panel-hover",
            Self::ColorBackgroundLayoutToggleActive => "--awsui-color-background-layout-toggle-active",
            Self::ColorBackgroundLayoutToggleDefault => "--awsui-color-background-layout-toggle-default",
            Self::ColorBackgroundLayoutToggleHover => "--awsui-color-background-layout-toggle-hover",
            Self::ColorBackgroundLayoutToggleSelectedActive => "--awsui-color-background-layout-toggle-selected-active",
            Self::ColorBackgroundLayoutToggleSelectedDefault => "--awsui-color-background-layout-toggle-selected-default",
            Self::ColorBackgroundLayoutToggleSelectedHover => "--awsui-color-background-layout-toggle-selected-hover",
            Self::ColorBackgroundModalOverlay => "--awsui-color-background-modal-overlay",
            Self::ColorBackgroundNotificationBlue => "--awsui-color-background-notification-blue",
            Self::ColorBackgroundNotificationGreen => "--awsui-color-background-notification-green",
            Self::ColorBackgroundNotificationGrey => "--awsui-color-background-notification-grey",
            Self::ColorBackgroundNotificationRed => "--awsui-color-background-notification-red",
            Self::ColorBackgroundNotificationYellow => "--awsui-color-background-notification-yellow",
            Self::ColorBackgroundNotificationStackBar => "--awsui-color-background-notification-stack-bar",
            Self::ColorBackgroundNotificationStackBarActive => "--awsui-color-background-notification-stack-bar-active",
            Self::ColorBackgroundNotificationStackBarHover => "--awsui-color-background-notification-stack-bar-hover",
            Self::ColorBackgroundPopover => "--awsui-color-background-popover",
            Self::ColorBackgroundProgressBarContentDefault => "--awsui-color-background-progress-bar-content-default",
            Self::ColorBackgroundProgressBarContentInFlash => "--awsui-color-background-progress-bar-content-in-flash",
            Self::ColorBackgroundProgressBarLayoutDefault => "--awsui-color-background-progress-bar-layout-default",
            Self::ColorBackgroundProgressBarLayoutInFlash => "--awsui-color-background-progress-bar-layout-in-flash",
            Self::ColorBackgroundSegmentActive => "--awsui-color-background-segment-active",
            Self::ColorBackgroundSegmentDefault => "--awsui-color-background-segment-default",
            Self::ColorBackgroundSegmentDisabled => "--awsui-color-background-segment-disabled",
            Self::ColorBackgroundSegmentHover => "--awsui-color-background-segment-hover",
            Self::ColorBackgroundSegmentWrapper => "--awsui-color-background-segment-wrapper",
            Self::ColorBackgroundSliderRangeDefault => "--awsui-color-background-slider-range-default",
            Self::ColorBackgroundSliderRangeActive => "--awsui-color-background-slider-range-active",
            Self::ColorBackgroundSliderHandleDefault => "--awsui-color-background-slider-handle-default",
            Self::ColorBackgroundSliderHandleActive => "--awsui-color-background-slider-handle-active",
            Self::ColorBackgroundSliderTrackDefault => "--awsui-color-background-slider-track-default",
            Self::ColorBackgroundSliderHandleRing => "--awsui-color-background-slider-handle-ring",
            Self::ColorBackgroundSliderHandleErrorDefault => "--awsui-color-background-slider-handle-error-default",
            Self::ColorBackgroundSliderHandleErrorActive => "--awsui-color-background-slider-handle-error-active",
            Self::ColorBackgroundSliderHandleWarningDefault => "--awsui-color-background-slider-handle-warning-default",
            Self::ColorBackgroundSliderHandleWarningActive => "--awsui-color-background-slider-handle-warning-active",
            Self::ColorBackgroundSliderRangeErrorDefault => "--awsui-color-background-slider-range-error-default",
            Self::ColorBackgroundSliderRangeErrorActive => "--awsui-color-background-slider-range-error-active",
            Self::ColorBackgroundSliderRangeWarningDefault => "--awsui-color-background-slider-range-warning-default",
            Self::ColorBackgroundSliderRangeWarningActive => "--awsui-color-background-slider-range-warning-active",
            Self::ColorBackgroundStatusError => "--awsui-color-background-status-error",
            Self::ColorBackgroundStatusInfo => "--awsui-color-background-status-info",
            Self::ColorBackgroundDialog => "--awsui-color-background-dialog",
            Self::ColorBackgroundStatusSuccess => "--awsui-color-background-status-success",
            Self::ColorBackgroundStatusWarning => "--awsui-color-background-status-warning",
            Self::ColorBackgroundTableHeader => "--awsui-color-background-table-header",
            Self::ColorBackgroundTilesDisabled => "--awsui-color-background-tiles-disabled",
            Self::ColorBackgroundToggleCheckedDisabled => "--awsui-color-background-toggle-checked-disabled",
            Self::ColorBackgroundToggleDefault => "--awsui-color-background-toggle-default",
            Self::ColorBackgroundAvatarGenAi => "--awsui-color-background-avatar-gen-ai",
            Self::ColorBackgroundAvatarDefault => "--awsui-color-background-avatar-default",
            Self::ColorTextAvatar => "--awsui-color-text-avatar",
            Self::ColorBackgroundLoadingBarGenAi => "--awsui-color-background-loading-bar-gen-ai",
            Self::ColorBackgroundChatBubbleOutgoing => "--awsui-color-background-chat-bubble-outgoing",
            Self::ColorBackgroundChatBubbleIncoming => "--awsui-color-background-chat-bubble-incoming",
            Self::ColorTextChatBubbleOutgoing => "--awsui-color-text-chat-bubble-outgoing",
            Self::ColorTextChatBubbleIncoming => "--awsui-color-text-chat-bubble-incoming",
            Self::ColorBorderButtonNormalActive => "--awsui-color-border-button-normal-active",
            Self::ColorBorderButtonNormalDefault => "--awsui-color-border-button-normal-default",
            Self::ColorBorderToggleButtonNormalPressed => "--awsui-color-border-toggle-button-normal-pressed",
            Self::ColorBorderButtonNormalDisabled => "--awsui-color-border-button-normal-disabled",
            Self::ColorTextButtonNormalDisabled => "--awsui-color-text-button-normal-disabled",
            Self::ColorBorderButtonNormalHover => "--awsui-color-border-button-normal-hover",
            Self::ColorTextButtonIconDisabled => "--awsui-color-text-button-icon-disabled",
            Self::ColorBorderButtonPrimaryDisabled => "--awsui-color-border-button-primary-disabled",
            Self::ColorTextButtonPrimaryDisabled => "--awsui-color-text-button-primary-disabled",
            Self::ColorItemSelected => "--awsui-color-item-selected",
            Self::ColorBorderCalendarGrid => "--awsui-color-border-calendar-grid",
            Self::ColorBorderCalendarGridSelectedFocusRing => "--awsui-color-border-calendar-grid-selected-focus-ring",
            Self::ColorBorderCellShaded => "--awsui-color-border-cell-shaded",
            Self::ColorBorderCodeEditorAceActiveLineLightTheme => "--awsui-color-border-code-editor-ace-active-line-light-theme",
            Self::ColorBorderCodeEditorAceActiveLineDarkTheme => "--awsui-color-border-code-editor-ace-active-line-dark-theme",
            Self::ColorBorderCodeEditorDefault => "--awsui-color-border-code-editor-default",
            Self::ColorBorderCodeEditorPaneItemHover => "--awsui-color-border-code-editor-pane-item-hover",
            Self::ColorBorderContainerDivider => "--awsui-color-border-container-divider",
            Self::ColorBorderContainerTop => "--awsui-color-border-container-top",
            Self::ColorBorderControlChecked => "--awsui-color-border-control-checked",
            Self::ColorBorderControlDefault => "--awsui-color-border-control-default",
            Self::ColorBorderControlDisabled => "--awsui-color-border-control-disabled",
            Self::ColorBorderDividerActive => "--awsui-color-border-divider-active",
            Self::ColorBorderDividerDefault => "--awsui-color-border-divider-default",
            Self::ColorBorderDividerPanelBottom => "--awsui-color-border-divider-panel-bottom",
            Self::ColorBorderDividerPanelSide => "--awsui-color-border-divider-panel-side",
            Self::ColorBorderDividerSecondary => "--awsui-color-border-divider-secondary",
            Self::ColorBorderDropdownContainer => "--awsui-color-border-dropdown-container",
            Self::ColorBorderDropdownGroup => "--awsui-color-border-dropdown-group",
            Self::ColorBorderDropdownItemDefault => "--awsui-color-border-dropdown-item-default",
            Self::ColorBorderDropdownItemHover => "--awsui-color-border-dropdown-item-hover",
            Self::ColorBorderDropdownItemDimmedHover => "--awsui-color-border-dropdown-item-dimmed-hover",
            Self::ColorBorderDropdownItemSelected => "--awsui-color-border-dropdown-item-selected",
            Self::ColorBorderDropdownItemTop => "--awsui-color-border-dropdown-item-top",
            Self::ColorBorderEditableCellHover => "--awsui-color-border-editable-cell-hover",
            Self::ColorBorderInputDefault => "--awsui-color-border-input-default",
            Self::ColorBorderInputDisabled => "--awsui-color-border-input-disabled",
            Self::ColorBorderInputFocused => "--awsui-color-border-input-focused",
            Self::ColorBorderItemFocused => "--awsui-color-border-item-focused",
            Self::ColorBorderDropdownItemFocused => "--awsui-color-border-dropdown-item-focused",
            Self::ColorBorderItemPlaceholder => "--awsui-color-border-item-placeholder",
            Self::ColorBorderItemSelected => "--awsui-color-border-item-selected",
            Self::ColorBorderLayout => "--awsui-color-border-layout",
            Self::ColorBorderNotificationStackBar => "--awsui-color-border-notification-stack-bar",
            Self::ColorBorderPanelHeader => "--awsui-color-border-panel-header",
            Self::ColorBorderPopover => "--awsui-color-border-popover",
            Self::ColorBorderSegmentActive => "--awsui-color-border-segment-active",
            Self::ColorBorderSegmentDefault => "--awsui-color-border-segment-default",
            Self::ColorBorderSegmentDisabled => "--awsui-color-border-segment-disabled",
            Self::ColorBorderSegmentHover => "--awsui-color-border-segment-hover",
            Self::ColorBorderStatusError => "--awsui-color-border-status-error",
            Self::ColorBorderStatusInfo => "--awsui-color-border-status-info",
            Self::ColorBorderStatusSuccess => "--awsui-color-border-status-success",
            Self::ColorBorderStatusWarning => "--awsui-color-border-status-warning",
            Self::ColorBorderDialog => "--awsui-color-border-dialog",
            Self::ColorBorderDividerInteractiveDefault => "--awsui-color-border-divider-interactive-default",
            Self::ColorBorderTabsDivider => "--awsui-color-border-tabs-divider",
            Self::ColorBorderTabsShadow => "--awsui-color-border-tabs-shadow",
            Self::ColorBorderTabsUnderline => "--awsui-color-border-tabs-underline",
            Self::ColorBorderTilesDisabled => "--awsui-color-border-tiles-disabled",
            Self::ColorBorderTutorial => "--awsui-color-border-tutorial",
            Self::ColorForegroundControlDefault => "--awsui-color-foreground-control-default",
            Self::ColorForegroundControlDisabled => "--awsui-color-foreground-control-disabled",
            Self::ColorForegroundControlReadOnly => "--awsui-color-foreground-control-read-only",
            Self::ColorShadowDefault => "--awsui-color-shadow-default",
            Self::ColorShadowMedium => "--awsui-color-shadow-medium",
            Self::ColorShadowSide => "--awsui-color-shadow-side",
            Self::ColorStrokeChartLine => "--awsui-color-stroke-chart-line",
            Self::ColorStrokeCodeEditorGutterActiveLineDefault => "--awsui-color-stroke-code-editor-gutter-active-line-default",
            Self::ColorStrokeCodeEditorGutterActiveLineHover => "--awsui-color-stroke-code-editor-gutter-active-line-hover",
            Self::ColorTextAccent => "--awsui-color-text-accent",
            Self::ColorTextBodyDefault => "--awsui-color-text-body-default",
            Self::ColorTextBodySecondary => "--awsui-color-text-body-secondary",
            Self::ColorTextBreadcrumbCurrent => "--awsui-color-text-breadcrumb-current",
            Self::ColorTextBreadcrumbIcon => "--awsui-color-text-breadcrumb-icon",
            Self::ColorTextButtonInlineIconDefault => "--awsui-color-text-button-inline-icon-default",
            Self::ColorTextButtonInlineIconDisabled => "--awsui-color-text-button-inline-icon-disabled",
            Self::ColorTextButtonInlineIconHover => "--awsui-color-text-button-inline-icon-hover",
            Self::ColorTextButtonNormalActive => "--awsui-color-text-button-normal-active",
            Self::ColorTextToggleButtonNormalPressed => "--awsui-color-text-toggle-button-normal-pressed",
            Self::ColorTextButtonNormalDefault => "--awsui-color-text-button-normal-default",
            Self::ColorTextButtonNormalHover => "--awsui-color-text-button-normal-hover",
            Self::ColorTextLinkButtonNormalDefault => "--awsui-color-text-link-button-normal-default",
            Self::ColorTextLinkButtonNormalHover => "--awsui-color-text-link-button-normal-hover",
            Self::ColorTextLinkButtonNormalActive => "--awsui-color-text-link-button-normal-active",
            Self::ColorTextButtonPrimaryActive => "--awsui-color-text-button-primary-active",
            Self::ColorTextButtonPrimaryDefault => "--awsui-color-text-button-primary-default",
            Self::ColorTextButtonPrimaryHover => "--awsui-color-text-button-primary-hover",
            Self::ColorTextCalendarDateHover => "--awsui-color-text-calendar-date-hover",
            Self::ColorTextCalendarMonth => "--awsui-color-text-calendar-month",
            Self::ColorTextCodeEditorGutterActiveLine => "--awsui-color-text-code-editor-gutter-active-line",
            Self::ColorTextCodeEditorGutterDefault => "--awsui-color-text-code-editor-gutter-default",
            Self::ColorTextCodeEditorStatusBarDisabled => "--awsui-color-text-code-editor-status-bar-disabled",
            Self::ColorTextCodeEditorTabButtonError => "--awsui-color-text-code-editor-tab-button-error",
            Self::ColorTextColumnHeader => "--awsui-color-text-column-header",
            Self::ColorTextColumnSortingIcon => "--awsui-color-text-column-sorting-icon",
            Self::ColorTextControlDisabled => "--awsui-color-text-control-disabled",
            Self::ColorTextCounter => "--awsui-color-text-counter",
            Self::ColorTextDisabled => "--awsui-color-text-disabled",
            Self::ColorTextDisabledInlineEdit => "--awsui-color-text-disabled-inline-edit",
            Self::ColorTextDropdownFooter => "--awsui-color-text-dropdown-footer",
            Self::ColorTextDropdownGroupLabel => "--awsui-color-text-dropdown-group-label",
            Self::ColorTextDropdownItemDefault => "--awsui-color-text-dropdown-item-default",
            Self::ColorTextDropdownItemDimmed => "--awsui-color-text-dropdown-item-dimmed",
            Self::ColorTextDropdownItemDisabled => "--awsui-color-text-dropdown-item-disabled",
            Self::ColorTextDropdownItemFilterMatch => "--awsui-color-text-dropdown-item-filter-match",
            Self::ColorTextDropdownItemHighlighted => "--awsui-color-text-dropdown-item-highlighted",
            Self::ColorTextDropdownItemSecondary => "--awsui-color-text-dropdown-item-secondary",
            Self::ColorTextDropdownItemSecondaryHover => "--awsui-color-text-dropdown-item-secondary-hover",
            Self::ColorTextEmpty => "--awsui-color-text-empty",
            Self::ColorTextExpandableSectionDefault => "--awsui-color-text-expandable-section-default",
            Self::ColorTextExpandableSectionHover => "--awsui-color-text-expandable-section-hover",
            Self::ColorTextExpandableSectionNavigationIconDefault => "--awsui-color-text-expandable-section-navigation-icon-default",
            Self::ColorTextFormDefault => "--awsui-color-text-form-default",
            Self::ColorTextFormLabel => "--awsui-color-text-form-label",
            Self::ColorTextFormSecondary => "--awsui-color-text-form-secondary",
            Self::ColorTextGroupLabel => "--awsui-color-text-group-label",
            Self::ColorTextLabelGenAi => "--awsui-color-text-label-gen-ai",
            Self::ColorTextHeadingDefault => "--awsui-color-text-heading-default",
            Self::ColorTextHeadingSecondary => "--awsui-color-text-heading-secondary",
            Self::ColorTextHomeHeaderDefault => "--awsui-color-text-home-header-default",
            Self::ColorTextHomeHeaderSecondary => "--awsui-color-text-home-header-secondary",
            Self::ColorTextIconCaret => "--awsui-color-text-icon-caret",
            Self::ColorTextIconSubtle => "--awsui-color-text-icon-subtle",
            Self::ColorTextInputDisabled => "--awsui-color-text-input-disabled",
            Self::ColorTextInputPlaceholder => "--awsui-color-text-input-placeholder",
            Self::ColorTextInputPlaceholderDisabled => "--awsui-color-text-input-placeholder-disabled",
            Self::ColorTextInteractiveActive => "--awsui-color-text-interactive-active",
            Self::ColorTextInteractiveDefault => "--awsui-color-text-interactive-default",
            Self::ColorTextInteractiveDisabled => "--awsui-color-text-interactive-disabled",
            Self::ColorTextInteractiveHover => "--awsui-color-text-interactive-hover",
            Self::ColorTextToggleButtonIconPressed => "--awsui-color-text-toggle-button-icon-pressed",
            Self::ColorTextInteractiveInvertedDefault => "--awsui-color-text-interactive-inverted-default",
            Self::ColorTextInteractiveInvertedHover => "--awsui-color-text-interactive-inverted-hover",
            Self::ColorTextInverted => "--awsui-color-text-inverted",
            Self::ColorTextLabel => "--awsui-color-text-label",
            Self::ColorTextLayoutToggle => "--awsui-color-text-layout-toggle",
            Self::ColorTextLayoutToggleActive => "--awsui-color-text-layout-toggle-active",
            Self::ColorTextLayoutToggleHover => "--awsui-color-text-layout-toggle-hover",
            Self::ColorTextLayoutToggleSelected => "--awsui-color-text-layout-toggle-selected",
            Self::ColorTextLinkDefault => "--awsui-color-text-link-default",
            Self::ColorTextLinkHover => "--awsui-color-text-link-hover",
            Self::ColorTextLinkInvertedHover => "--awsui-color-text-link-inverted-hover",
            Self::ColorTextLinkButtonUnderline => "--awsui-color-text-link-button-underline",
            Self::ColorTextLinkButtonUnderlineHover => "--awsui-color-text-link-button-underline-hover",
            Self::ColorTextNotificationDefault => "--awsui-color-text-notification-default",
            Self::ColorTextNotificationStackBar => "--awsui-color-text-notification-stack-bar",
            Self::ColorTextNotificationYellow => "--awsui-color-text-notification-yellow",
            Self::ColorTextPaginationPageNumberActiveDisabled => "--awsui-color-text-pagination-page-number-active-disabled",
            Self::ColorTextPaginationPageNumberDefault => "--awsui-color-text-pagination-page-number-default",
            Self::ColorTextSegmentActive => "--awsui-color-text-segment-active",
            Self::ColorTextSegmentDefault => "--awsui-color-text-segment-default",
            Self::ColorTextSegmentHover => "--awsui-color-text-segment-hover",
            Self::ColorTextSmall => "--awsui-color-text-small",
            Self::ColorTextStatusError => "--awsui-color-text-status-error",
            Self::ColorTextStatusInactive => "--awsui-color-text-status-inactive",
            Self::ColorTextStatusInfo => "--awsui-color-text-status-info",
            Self::ColorTextStatusSuccess => "--awsui-color-text-status-success",
            Self::ColorTextStatusWarning => "--awsui-color-text-status-warning",
            Self::ColorTextTopNavigationTitle => "--awsui-color-text-top-navigation-title",
            Self::ColorTextTutorialHotspotDefault => "--awsui-color-text-tutorial-hotspot-default",
            Self::ColorTextTutorialHotspotHover => "--awsui-color-text-tutorial-hotspot-hover",
            Self::ColorBoardPlaceholderActive => "--awsui-color-board-placeholder-active",
            Self::ColorBoardPlaceholderHover => "--awsui-color-board-placeholder-hover",
            Self::ColorDragPlaceholderActive => "--awsui-color-drag-placeholder-active",
            Self::ColorDragPlaceholderHover => "--awsui-color-drag-placeholder-hover",
            Self::ColorDropzoneBackgroundDefault => "--awsui-color-dropzone-background-default",
            Self::ColorDropzoneBackgroundHover => "--awsui-color-dropzone-background-hover",
            Self::ColorDropzoneTextDefault => "--awsui-color-dropzone-text-default",
            Self::ColorDropzoneTextHover => "--awsui-color-dropzone-text-hover",
            Self::ColorDropzoneBorderDefault => "--awsui-color-dropzone-border-default",
            Self::ColorDropzoneBorderHover => "--awsui-color-dropzone-border-hover",
            Self::ColorGapGlobalDrawer => "--awsui-color-gap-global-drawer",
            Self::ColorTreeViewConnectorLine => "--awsui-color-tree-view-connector-line",
        }
    }

    /// Get the CSS var() function for use in styles
    pub fn css_var(&self) -> String {
        format!("var({})", self.css_var_name())
    }
}

impl fmt::Display for ColorToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_var())
    }
}

/// Font design tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontToken {
    FontBoxValueLargeWeight,
    /// The default letter spacing for button text.
    FontButtonLetterSpacing,
    /// Used for secondary chart text, e.g. mixed chart axes and pie chart label descriptions.
    FontChartDetailSize,
    /// The default font weight for labels. For example, keys in key-value pairs, or form field labels.
    FontDisplayLabelWeight,
    FontExpandableHeadingSize,
    /// The default font family that will be applied globally to the product interface.
    FontFamilyBase,
    /// The monospace font family that will be applied globally to the product interface.
    FontFamilyMonospace,
    FontHeaderH2DescriptionLineHeight,
    FontHeaderH2DescriptionSize,
    FontLinkButtonLetterSpacing,
    FontLinkButtonWeight,
    FontPanelHeaderLineHeight,
    FontPanelHeaderSize,
    /// The default font size for regular body text. For example, <p> tags in text content, or button text.
    FontSizeBodyM,
    /// The default font size for small body text. For example, form field descriptions, or badge text.
    FontSizeBodyS,
    /// The default font size for large display text.
    FontSizeDisplayL,
    /// The default font size for h1s.
    FontSizeHeadingXl,
    /// The default font size for h2s.
    FontSizeHeadingL,
    /// The default font size for h3s.
    FontSizeHeadingM,
    /// The default font size for h4s.
    FontSizeHeadingS,
    /// The default font size for h5s.
    FontSizeHeadingXs,
    FontSmoothingMozOsx,
    FontSmoothingWebkit,
    FontTabsDisabledWeight,
    FontTabsLineHeight,
    FontTabsSize,
    FontWayfindingLinkActiveWeight,
    /// The default font weight for button text.
    FontWeightButton,
    /// The default font weight for h1s.
    FontWeightHeadingXl,
    /// The default font weight for h2s.
    FontWeightHeadingL,
    /// The default font weight for h3s.
    FontWeightHeadingM,
    /// The default font weight for h4s.
    FontWeightHeadingS,
    /// The default font weight for h5s.
    FontWeightHeadingXs,
    FontWeightHeavy,
}

impl FontToken {
    /// Get the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
            Self::FontBoxValueLargeWeight => "--awsui-font-box-value-large-weight",
            Self::FontButtonLetterSpacing => "--awsui-font-button-letter-spacing",
            Self::FontChartDetailSize => "--awsui-font-chart-detail-size",
            Self::FontDisplayLabelWeight => "--awsui-font-display-label-weight",
            Self::FontExpandableHeadingSize => "--awsui-font-expandable-heading-size",
            Self::FontFamilyBase => "--awsui-font-family-base",
            Self::FontFamilyMonospace => "--awsui-font-family-monospace",
            Self::FontHeaderH2DescriptionLineHeight => "--awsui-font-header-h2-description-line-height",
            Self::FontHeaderH2DescriptionSize => "--awsui-font-header-h2-description-size",
            Self::FontLinkButtonLetterSpacing => "--awsui-font-link-button-letter-spacing",
            Self::FontLinkButtonWeight => "--awsui-font-link-button-weight",
            Self::FontPanelHeaderLineHeight => "--awsui-font-panel-header-line-height",
            Self::FontPanelHeaderSize => "--awsui-font-panel-header-size",
            Self::FontSizeBodyM => "--awsui-font-size-body-m",
            Self::FontSizeBodyS => "--awsui-font-size-body-s",
            Self::FontSizeDisplayL => "--awsui-font-size-display-l",
            Self::FontSizeHeadingXl => "--awsui-font-size-heading-xl",
            Self::FontSizeHeadingL => "--awsui-font-size-heading-l",
            Self::FontSizeHeadingM => "--awsui-font-size-heading-m",
            Self::FontSizeHeadingS => "--awsui-font-size-heading-s",
            Self::FontSizeHeadingXs => "--awsui-font-size-heading-xs",
            Self::FontSmoothingMozOsx => "--awsui-font-smoothing-moz-osx",
            Self::FontSmoothingWebkit => "--awsui-font-smoothing-webkit",
            Self::FontTabsDisabledWeight => "--awsui-font-tabs-disabled-weight",
            Self::FontTabsLineHeight => "--awsui-font-tabs-line-height",
            Self::FontTabsSize => "--awsui-font-tabs-size",
            Self::FontWayfindingLinkActiveWeight => "--awsui-font-wayfinding-link-active-weight",
            Self::FontWeightButton => "--awsui-font-weight-button",
            Self::FontWeightHeadingXl => "--awsui-font-weight-heading-xl",
            Self::FontWeightHeadingL => "--awsui-font-weight-heading-l",
            Self::FontWeightHeadingM => "--awsui-font-weight-heading-m",
            Self::FontWeightHeadingS => "--awsui-font-weight-heading-s",
            Self::FontWeightHeadingXs => "--awsui-font-weight-heading-xs",
            Self::FontWeightHeavy => "--awsui-font-weight-heavy",
        }
    }

    /// Get the CSS var() function for use in styles
    pub fn css_var(&self) -> String {
        format!("var({})", self.css_var_name())
    }
}

impl fmt::Display for FontToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_var())
    }
}

/// Other design tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OtherToken {
    /// The default letter spacing for small body text.
    LetterSpacingBodyS,
    /// The default letter spacing for large display text.
    LetterSpacingDisplayL,
    /// The default letter spacing for h1s.
    LetterSpacingHeadingXl,
    /// The default letter spacing for h2s.
    LetterSpacingHeadingL,
    /// The default letter spacing for h3s.
    LetterSpacingHeadingM,
    /// The default letter spacing for h4s.
    LetterSpacingHeadingS,
    /// The default line height for regular body text.
    LineHeightBodyM,
    /// The default line height for small body text.
    LineHeightBodyS,
    /// The default line height for large display text.
    LineHeightDisplayL,
    /// The default line height for h1s.
    LineHeightHeadingXl,
    /// The default line height for h2s.
    LineHeightHeadingL,
    /// The default line height for h3s.
    LineHeightHeadingM,
    /// The default line height for h4s.
    LineHeightHeadingS,
    /// The default line height for h5s.
    LineHeightHeadingXs,
    SpaceAlertActionLeft,
    SpaceAlertHorizontal,
    SpaceAlertMessageRight,
    SpaceAlertVertical,
    SpaceButtonFocusOutlineGutter,
    SpaceButtonHorizontal,
    SpaceButtonIconFocusOutlineGutterVertical,
    SpaceButtonIconOnlyHorizontal,
    SpaceButtonInlineIconFocusOutlineGutter,
    SpaceButtonModalDismissVertical,
    SpaceCalendarGridFocusOutlineGutter,
    SpaceCalendarGridSelectedFocusOutlineGutter,
    SpaceCalendarGridGutter,
    SpaceCardHorizontal,
    SpaceCardVertical,
    SpaceCodeEditorStatusFocusOutlineGutter,
    SpaceContainerContentTop,
    SpaceContainerHeaderTop,
    SpaceContainerHeaderBottom,
    /// The horizontal padding inside a container.
    SpaceContainerHorizontal,
    SpaceContentHeaderPaddingBottom,
    SpaceDarkHeaderOverlapDistance,
    SpaceExpandableSectionIconOffsetTop,
    /// The horizontal padding inside field components.
    SpaceFieldHorizontal,
    SpaceFieldIconOffset,
    SpaceFilteringTokenDismissButtonFocusOutlineGutter,
    SpaceFilteringTokenOperationSelectFocusOutlineGutter,
    SpaceFlashbarActionLeft,
    SpaceFlashbarDismissRight,
    SpaceFlashbarHorizontal,
    SpaceFlashbarVertical,
    SpaceGridGutter,
    SpaceKeyValueGap,
    SpaceLayoutContentBottom,
    SpaceLayoutContentHorizontal,
    SpaceLayoutToggleDiameter,
    SpaceLayoutTogglePadding,
    SpaceModalContentBottom,
    SpaceModalHorizontal,
    SpacePanelContentBottom,
    SpacePanelContentTop,
    SpacePanelDividerMarginHorizontal,
    SpacePanelHeaderVertical,
    SpacePanelNavLeft,
    SpacePanelSideLeft,
    SpacePanelSideRight,
    SpacePanelSplitTop,
    SpacePanelSplitBottom,
    SpaceSegmentedControlFocusOutlineGutter,
    SpaceTabsContentTop,
    SpaceTabsFocusOutlineGutter,
    SpaceTableContentBottom,
    SpaceTableEmbeddedHeaderTop,
    SpaceTableFooterHorizontal,
    SpaceTableHeaderFocusOutlineGutter,
    SpaceTableHeaderHorizontal,
    SpaceTableHeaderToolsBottom,
    SpaceTableHeaderToolsFullPageBottom,
    SpaceTableHorizontal,
    /// The indentation of tree view items.
    SpaceTreeViewIndentation,
    SpaceTileGutter,
    SpaceScaled2xNone,
    SpaceScaled2xXxxs,
    SpaceScaled2xXxs,
    SpaceScaled2xXs,
    SpaceScaled2xS,
    SpaceScaled2xM,
    SpaceScaled2xL,
    SpaceScaled2xXl,
    SpaceScaled2xXxl,
    SpaceScaled2xXxxl,
    SpaceScaledNone,
    /// The XXXS spacing unit which scales between modes.
    SpaceScaledXxxs,
    /// The XXS spacing unit which scales between modes. For example: vertical space between form field label and control.
    SpaceScaledXxs,
    /// The XS spacing unit which scales between modes. For example: horizontal space between buttons in an action stripe.
    SpaceScaledXs,
    /// The S spacing unit which scales between modes. For example: vertical padding inside a popover.
    SpaceScaledS,
    /// The M spacing unit which scales between modes. For example: horizontal padding inside a popover.
    SpaceScaledM,
    /// The L spacing unit which scales between modes. For example: vertical space between form fields.
    SpaceScaledL,
    /// The XL spacing unit which scales between modes. For example: horizontal space between wizard navigation and content.
    SpaceScaledXl,
    /// The XXL spacing unit which scales between modes. For example: left indentation of grouped options in a select.
    SpaceScaledXxl,
    /// The XXXL spacing unit which scales between modes. For example: horizontal padding for app layout and split panel content on large screens.
    SpaceScaledXxxl,
    /// The static XXXS spacing unit.
    SpaceStaticXxxs,
    /// The static XXS spacing unit.
    SpaceStaticXxs,
    /// The static XS spacing unit.
    SpaceStaticXs,
    /// The static S spacing unit.
    SpaceStaticS,
    /// The static M spacing unit.
    SpaceStaticM,
    /// The static L spacing unit.
    SpaceStaticL,
    /// The static XL spacing unit.
    SpaceStaticXl,
    /// The static XXL spacing unit.
    SpaceStaticXxl,
    /// The static XXXL spacing unit.
    SpaceStaticXxxl,
    SpaceNone,
    SpaceXxxs,
    SpaceXxs,
    SpaceXs,
    SpaceS,
    SpaceM,
    SpaceL,
    SpaceXl,
    SpaceXxl,
    SpaceXxxl,
}

impl OtherToken {
    /// Get the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
            Self::LetterSpacingBodyS => "--awsui-letter-spacing-body-s",
            Self::LetterSpacingDisplayL => "--awsui-letter-spacing-display-l",
            Self::LetterSpacingHeadingXl => "--awsui-letter-spacing-heading-xl",
            Self::LetterSpacingHeadingL => "--awsui-letter-spacing-heading-l",
            Self::LetterSpacingHeadingM => "--awsui-letter-spacing-heading-m",
            Self::LetterSpacingHeadingS => "--awsui-letter-spacing-heading-s",
            Self::LineHeightBodyM => "--awsui-line-height-body-m",
            Self::LineHeightBodyS => "--awsui-line-height-body-s",
            Self::LineHeightDisplayL => "--awsui-line-height-display-l",
            Self::LineHeightHeadingXl => "--awsui-line-height-heading-xl",
            Self::LineHeightHeadingL => "--awsui-line-height-heading-l",
            Self::LineHeightHeadingM => "--awsui-line-height-heading-m",
            Self::LineHeightHeadingS => "--awsui-line-height-heading-s",
            Self::LineHeightHeadingXs => "--awsui-line-height-heading-xs",
            Self::SpaceAlertActionLeft => "--awsui-space-alert-action-left",
            Self::SpaceAlertHorizontal => "--awsui-space-alert-horizontal",
            Self::SpaceAlertMessageRight => "--awsui-space-alert-message-right",
            Self::SpaceAlertVertical => "--awsui-space-alert-vertical",
            Self::SpaceButtonFocusOutlineGutter => "--awsui-space-button-focus-outline-gutter",
            Self::SpaceButtonHorizontal => "--awsui-space-button-horizontal",
            Self::SpaceButtonIconFocusOutlineGutterVertical => "--awsui-space-button-icon-focus-outline-gutter-vertical",
            Self::SpaceButtonIconOnlyHorizontal => "--awsui-space-button-icon-only-horizontal",
            Self::SpaceButtonInlineIconFocusOutlineGutter => "--awsui-space-button-inline-icon-focus-outline-gutter",
            Self::SpaceButtonModalDismissVertical => "--awsui-space-button-modal-dismiss-vertical",
            Self::SpaceCalendarGridFocusOutlineGutter => "--awsui-space-calendar-grid-focus-outline-gutter",
            Self::SpaceCalendarGridSelectedFocusOutlineGutter => "--awsui-space-calendar-grid-selected-focus-outline-gutter",
            Self::SpaceCalendarGridGutter => "--awsui-space-calendar-grid-gutter",
            Self::SpaceCardHorizontal => "--awsui-space-card-horizontal",
            Self::SpaceCardVertical => "--awsui-space-card-vertical",
            Self::SpaceCodeEditorStatusFocusOutlineGutter => "--awsui-space-code-editor-status-focus-outline-gutter",
            Self::SpaceContainerContentTop => "--awsui-space-container-content-top",
            Self::SpaceContainerHeaderTop => "--awsui-space-container-header-top",
            Self::SpaceContainerHeaderBottom => "--awsui-space-container-header-bottom",
            Self::SpaceContainerHorizontal => "--awsui-space-container-horizontal",
            Self::SpaceContentHeaderPaddingBottom => "--awsui-space-content-header-padding-bottom",
            Self::SpaceDarkHeaderOverlapDistance => "--awsui-space-dark-header-overlap-distance",
            Self::SpaceExpandableSectionIconOffsetTop => "--awsui-space-expandable-section-icon-offset-top",
            Self::SpaceFieldHorizontal => "--awsui-space-field-horizontal",
            Self::SpaceFieldIconOffset => "--awsui-space-field-icon-offset",
            Self::SpaceFilteringTokenDismissButtonFocusOutlineGutter => "--awsui-space-filtering-token-dismiss-button-focus-outline-gutter",
            Self::SpaceFilteringTokenOperationSelectFocusOutlineGutter => "--awsui-space-filtering-token-operation-select-focus-outline-gutter",
            Self::SpaceFlashbarActionLeft => "--awsui-space-flashbar-action-left",
            Self::SpaceFlashbarDismissRight => "--awsui-space-flashbar-dismiss-right",
            Self::SpaceFlashbarHorizontal => "--awsui-space-flashbar-horizontal",
            Self::SpaceFlashbarVertical => "--awsui-space-flashbar-vertical",
            Self::SpaceGridGutter => "--awsui-space-grid-gutter",
            Self::SpaceKeyValueGap => "--awsui-space-key-value-gap",
            Self::SpaceLayoutContentBottom => "--awsui-space-layout-content-bottom",
            Self::SpaceLayoutContentHorizontal => "--awsui-space-layout-content-horizontal",
            Self::SpaceLayoutToggleDiameter => "--awsui-space-layout-toggle-diameter",
            Self::SpaceLayoutTogglePadding => "--awsui-space-layout-toggle-padding",
            Self::SpaceModalContentBottom => "--awsui-space-modal-content-bottom",
            Self::SpaceModalHorizontal => "--awsui-space-modal-horizontal",
            Self::SpacePanelContentBottom => "--awsui-space-panel-content-bottom",
            Self::SpacePanelContentTop => "--awsui-space-panel-content-top",
            Self::SpacePanelDividerMarginHorizontal => "--awsui-space-panel-divider-margin-horizontal",
            Self::SpacePanelHeaderVertical => "--awsui-space-panel-header-vertical",
            Self::SpacePanelNavLeft => "--awsui-space-panel-nav-left",
            Self::SpacePanelSideLeft => "--awsui-space-panel-side-left",
            Self::SpacePanelSideRight => "--awsui-space-panel-side-right",
            Self::SpacePanelSplitTop => "--awsui-space-panel-split-top",
            Self::SpacePanelSplitBottom => "--awsui-space-panel-split-bottom",
            Self::SpaceSegmentedControlFocusOutlineGutter => "--awsui-space-segmented-control-focus-outline-gutter",
            Self::SpaceTabsContentTop => "--awsui-space-tabs-content-top",
            Self::SpaceTabsFocusOutlineGutter => "--awsui-space-tabs-focus-outline-gutter",
            Self::SpaceTableContentBottom => "--awsui-space-table-content-bottom",
            Self::SpaceTableEmbeddedHeaderTop => "--awsui-space-table-embedded-header-top",
            Self::SpaceTableFooterHorizontal => "--awsui-space-table-footer-horizontal",
            Self::SpaceTableHeaderFocusOutlineGutter => "--awsui-space-table-header-focus-outline-gutter",
            Self::SpaceTableHeaderHorizontal => "--awsui-space-table-header-horizontal",
            Self::SpaceTableHeaderToolsBottom => "--awsui-space-table-header-tools-bottom",
            Self::SpaceTableHeaderToolsFullPageBottom => "--awsui-space-table-header-tools-full-page-bottom",
            Self::SpaceTableHorizontal => "--awsui-space-table-horizontal",
            Self::SpaceTreeViewIndentation => "--awsui-space-tree-view-indentation",
            Self::SpaceTileGutter => "--awsui-space-tile-gutter",
            Self::SpaceScaled2xNone => "--awsui-space-scaled2x-none",
            Self::SpaceScaled2xXxxs => "--awsui-space-scaled2x-xxxs",
            Self::SpaceScaled2xXxs => "--awsui-space-scaled2x-xxs",
            Self::SpaceScaled2xXs => "--awsui-space-scaled2x-xs",
            Self::SpaceScaled2xS => "--awsui-space-scaled2x-s",
            Self::SpaceScaled2xM => "--awsui-space-scaled2x-m",
            Self::SpaceScaled2xL => "--awsui-space-scaled2x-l",
            Self::SpaceScaled2xXl => "--awsui-space-scaled2x-xl",
            Self::SpaceScaled2xXxl => "--awsui-space-scaled2x-xxl",
            Self::SpaceScaled2xXxxl => "--awsui-space-scaled2x-xxxl",
            Self::SpaceScaledNone => "--awsui-space-scaled-none",
            Self::SpaceScaledXxxs => "--awsui-space-scaled-xxxs",
            Self::SpaceScaledXxs => "--awsui-space-scaled-xxs",
            Self::SpaceScaledXs => "--awsui-space-scaled-xs",
            Self::SpaceScaledS => "--awsui-space-scaled-s",
            Self::SpaceScaledM => "--awsui-space-scaled-m",
            Self::SpaceScaledL => "--awsui-space-scaled-l",
            Self::SpaceScaledXl => "--awsui-space-scaled-xl",
            Self::SpaceScaledXxl => "--awsui-space-scaled-xxl",
            Self::SpaceScaledXxxl => "--awsui-space-scaled-xxxl",
            Self::SpaceStaticXxxs => "--awsui-space-static-xxxs",
            Self::SpaceStaticXxs => "--awsui-space-static-xxs",
            Self::SpaceStaticXs => "--awsui-space-static-xs",
            Self::SpaceStaticS => "--awsui-space-static-s",
            Self::SpaceStaticM => "--awsui-space-static-m",
            Self::SpaceStaticL => "--awsui-space-static-l",
            Self::SpaceStaticXl => "--awsui-space-static-xl",
            Self::SpaceStaticXxl => "--awsui-space-static-xxl",
            Self::SpaceStaticXxxl => "--awsui-space-static-xxxl",
            Self::SpaceNone => "--awsui-space-none",
            Self::SpaceXxxs => "--awsui-space-xxxs",
            Self::SpaceXxs => "--awsui-space-xxs",
            Self::SpaceXs => "--awsui-space-xs",
            Self::SpaceS => "--awsui-space-s",
            Self::SpaceM => "--awsui-space-m",
            Self::SpaceL => "--awsui-space-l",
            Self::SpaceXl => "--awsui-space-xl",
            Self::SpaceXxl => "--awsui-space-xxl",
            Self::SpaceXxxl => "--awsui-space-xxxl",
        }
    }

    /// Get the CSS var() function for use in styles
    pub fn css_var(&self) -> String {
        format!("var({})", self.css_var_name())
    }
}

impl fmt::Display for OtherToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_var())
    }
}

/// Border design tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderToken {
    /// The height of the active tab indicator.
    BorderActiveWidth,
    BorderCodeEditorStatusDividerWidth,
    BorderContainerStickyWidth,
    BorderContainerTopWidth,
    BorderControlFocusRingShadowSpread,
    BorderControlInvalidFocusRingShadowSpread,
    /// Used for divider between repeated items such as dropdowns and tables.
    BorderDividerListWidth,
    /// The default system divider width - used for dividers between sections of content such as key/value pairs and tabs, for both full width and inset dividers.
    BorderDividerSectionWidth,
    BorderDropdownVirtualOffsetWidth,
    /// Used for invalid input left border width.
    BorderInvalidWidth,
    BorderItemWidth,
    BorderLineChartDashArray,
    BorderLineChartLineJoin,
    BorderLineChartWidth,
    /// The split panel header bottom divider width.
    BorderPanelHeaderWidth,
    /// The split panel top border width.
    BorderPanelTopWidth,
    /// The border radius of alerts.
    BorderRadiusAlert,
    /// The border radius of badges.
    BorderRadiusBadge,
    /// The border radius of buttons and segmented control's segments.
    BorderRadiusButton,
    /// The border radius of the focused day in date picker and date range picker.
    BorderRadiusCalendarDayFocusRing,
    /// The border radius of code editors.
    BorderRadiusCodeEditor,
    /// The border radius of containers. Also used in container-based components like table, cards, expandable section, and modal.
    BorderRadiusContainer,
    /// The border radius of the focus indicator of circular controls. For example: radio button.
    BorderRadiusControlCircularFocusRing,
    /// The border radius of the focus indicator of interactive elements. For example: button, link, toggle, pagination controls, expandable section header, popover trigger.
    BorderRadiusControlDefaultFocusRing,
    /// The border radius of dropdown containers. For example: button dropdown, select, multiselect, autosuggest, date picker, date range picker.
    BorderRadiusDropdown,
    /// The border radius of file upload dropzone.
    BorderRadiusDropzone,
    /// The border radius of flash messages in flashbars.
    BorderRadiusFlashbar,
    /// The border radius of items that can be selected from a list. For example: select options, table rows, calendar days.
    BorderRadiusItem,
    /// The border radius of form controls. For example: input, select.
    BorderRadiusInput,
    /// The border radius of popovers.
    BorderRadiusPopover,
    /// The border radius of tabs' focus indicator. Used in tabs and in the code editor status bar.
    BorderRadiusTabsFocusRing,
    /// The border radius of tiles.
    BorderRadiusTiles,
    /// The border radius of tokens. For example: token groups, multiselect tokens, property filter tokens.
    BorderRadiusToken,
    /// The border radius of chat bubbles.
    BorderRadiusChatBubble,
    /// The border radius of tutorials inside a tutorial panel.
    BorderRadiusTutorialPanelItem,
    BorderTableStickyWidth,
    BorderLinkFocusRingOutline,
    BorderLinkFocusRingShadowSpread,
    /// The border width of alerts.
    BorderWidthAlert,
    /// The border width of buttons.
    BorderWidthButton,
    /// The border width of dropdowns.
    BorderWidthDropdown,
    /// The border width of form fields.
    BorderWidthField,
    /// The border width of popovers.
    BorderWidthPopover,
    /// The border width of tokens.
    BorderWidthToken,
    /// The visual stroke width of small icons.
    BorderWidthIconSmall,
    /// The visual stroke width of normal icons.
    BorderWidthIconNormal,
    /// The visual stroke width of medium icons.
    BorderWidthIconMedium,
    /// The visual stroke width of big icons.
    BorderWidthIconBig,
    /// The visual stroke width of large icons.
    BorderWidthIconLarge,
}

impl BorderToken {
    /// Get the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
            Self::BorderActiveWidth => "--awsui-border-active-width",
            Self::BorderCodeEditorStatusDividerWidth => "--awsui-border-code-editor-status-divider-width",
            Self::BorderContainerStickyWidth => "--awsui-border-container-sticky-width",
            Self::BorderContainerTopWidth => "--awsui-border-container-top-width",
            Self::BorderControlFocusRingShadowSpread => "--awsui-border-control-focus-ring-shadow-spread",
            Self::BorderControlInvalidFocusRingShadowSpread => "--awsui-border-control-invalid-focus-ring-shadow-spread",
            Self::BorderDividerListWidth => "--awsui-border-divider-list-width",
            Self::BorderDividerSectionWidth => "--awsui-border-divider-section-width",
            Self::BorderDropdownVirtualOffsetWidth => "--awsui-border-dropdown-virtual-offset-width",
            Self::BorderInvalidWidth => "--awsui-border-invalid-width",
            Self::BorderItemWidth => "--awsui-border-item-width",
            Self::BorderLineChartDashArray => "--awsui-border-line-chart-dash-array",
            Self::BorderLineChartLineJoin => "--awsui-border-line-chart-line-join",
            Self::BorderLineChartWidth => "--awsui-border-line-chart-width",
            Self::BorderPanelHeaderWidth => "--awsui-border-panel-header-width",
            Self::BorderPanelTopWidth => "--awsui-border-panel-top-width",
            Self::BorderRadiusAlert => "--awsui-border-radius-alert",
            Self::BorderRadiusBadge => "--awsui-border-radius-badge",
            Self::BorderRadiusButton => "--awsui-border-radius-button",
            Self::BorderRadiusCalendarDayFocusRing => "--awsui-border-radius-calendar-day-focus-ring",
            Self::BorderRadiusCodeEditor => "--awsui-border-radius-code-editor",
            Self::BorderRadiusContainer => "--awsui-border-radius-container",
            Self::BorderRadiusControlCircularFocusRing => "--awsui-border-radius-control-circular-focus-ring",
            Self::BorderRadiusControlDefaultFocusRing => "--awsui-border-radius-control-default-focus-ring",
            Self::BorderRadiusDropdown => "--awsui-border-radius-dropdown",
            Self::BorderRadiusDropzone => "--awsui-border-radius-dropzone",
            Self::BorderRadiusFlashbar => "--awsui-border-radius-flashbar",
            Self::BorderRadiusItem => "--awsui-border-radius-item",
            Self::BorderRadiusInput => "--awsui-border-radius-input",
            Self::BorderRadiusPopover => "--awsui-border-radius-popover",
            Self::BorderRadiusTabsFocusRing => "--awsui-border-radius-tabs-focus-ring",
            Self::BorderRadiusTiles => "--awsui-border-radius-tiles",
            Self::BorderRadiusToken => "--awsui-border-radius-token",
            Self::BorderRadiusChatBubble => "--awsui-border-radius-chat-bubble",
            Self::BorderRadiusTutorialPanelItem => "--awsui-border-radius-tutorial-panel-item",
            Self::BorderTableStickyWidth => "--awsui-border-table-sticky-width",
            Self::BorderLinkFocusRingOutline => "--awsui-border-link-focus-ring-outline",
            Self::BorderLinkFocusRingShadowSpread => "--awsui-border-link-focus-ring-shadow-spread",
            Self::BorderWidthAlert => "--awsui-border-width-alert",
            Self::BorderWidthButton => "--awsui-border-width-button",
            Self::BorderWidthDropdown => "--awsui-border-width-dropdown",
            Self::BorderWidthField => "--awsui-border-width-field",
            Self::BorderWidthPopover => "--awsui-border-width-popover",
            Self::BorderWidthToken => "--awsui-border-width-token",
            Self::BorderWidthIconSmall => "--awsui-border-width-icon-small",
            Self::BorderWidthIconNormal => "--awsui-border-width-icon-normal",
            Self::BorderWidthIconMedium => "--awsui-border-width-icon-medium",
            Self::BorderWidthIconBig => "--awsui-border-width-icon-big",
            Self::BorderWidthIconLarge => "--awsui-border-width-icon-large",
        }
    }

    /// Get the CSS var() function for use in styles
    pub fn css_var(&self) -> String {
        format!("var({})", self.css_var_name())
    }
}

impl fmt::Display for BorderToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_var())
    }
}

/// Motion design tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionToken {
    MotionDurationExtraFast,
    MotionDurationExtraSlow,
    MotionDurationFast,
    MotionDurationModerate,
    MotionDurationRefreshOnlyAmbient,
    MotionDurationRefreshOnlyFast,
    MotionDurationRefreshOnlyMedium,
    MotionDurationRefreshOnlySlow,
    /// The duration for gradient motion of gen-ai avatars.
    MotionDurationAvatarGenAiGradient,
    /// The duration for loading motion of avatars.
    MotionDurationAvatarLoadingDots,
    MotionDurationRotate180,
    MotionDurationRotate90,
    MotionDurationShowPaced,
    MotionDurationShowQuick,
    MotionDurationSlow,
    MotionDurationTransitionQuick,
    MotionDurationTransitionShowPaced,
    MotionDurationTransitionShowQuick,
    MotionEasingEaseOutQuart,
    MotionEasingRefreshOnlyA,
    MotionEasingRefreshOnlyB,
    MotionEasingRefreshOnlyC,
    MotionEasingRefreshOnlyD,
    /// The easing curve for gradient motion of gen-ai avatars.
    MotionEasingAvatarGenAiGradient,
    MotionEasingRotate180,
    MotionEasingRotate90,
    MotionEasingShowPaced,
    MotionEasingShowQuick,
    MotionEasingTransitionQuick,
    MotionEasingTransitionShowPaced,
    MotionEasingTransitionShowQuick,
    /// The easing curve for providing responsive yet smooth visual feedback.
    MotionEasingResponsive,
    /// The easing curve for making an element sticky to a certain state.
    MotionEasingSticky,
    /// The easing curve for drawing a user's attention in an expressive way.
    MotionEasingExpressive,
    /// The duration for making the motion feel quick and responsive.
    MotionDurationResponsive,
    /// The duration for accommodating the motion with more expressiveness.
    MotionDurationExpressive,
    /// The duration for drawing more attention or accommodating for more complexity.
    MotionDurationComplex,
    /// The CSS keyframes for showing an element.
    MotionKeyframesFadeIn,
    /// The CSS keyframes for hiding an element.
    MotionKeyframesFadeOut,
    /// The CSS keyframes applied to an error status icon to draw the user's attention.
    MotionKeyframesStatusIconError,
    /// The CSS keyframes for scaling up an element to draw the user's attention.
    MotionKeyframesScalePopup,
}

impl MotionToken {
    /// Get the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
            Self::MotionDurationExtraFast => "--awsui-motion-duration-extra-fast",
            Self::MotionDurationExtraSlow => "--awsui-motion-duration-extra-slow",
            Self::MotionDurationFast => "--awsui-motion-duration-fast",
            Self::MotionDurationModerate => "--awsui-motion-duration-moderate",
            Self::MotionDurationRefreshOnlyAmbient => "--awsui-motion-duration-refresh-only-ambient",
            Self::MotionDurationRefreshOnlyFast => "--awsui-motion-duration-refresh-only-fast",
            Self::MotionDurationRefreshOnlyMedium => "--awsui-motion-duration-refresh-only-medium",
            Self::MotionDurationRefreshOnlySlow => "--awsui-motion-duration-refresh-only-slow",
            Self::MotionDurationAvatarGenAiGradient => "--awsui-motion-duration-avatar-gen-ai-gradient",
            Self::MotionDurationAvatarLoadingDots => "--awsui-motion-duration-avatar-loading-dots",
            Self::MotionDurationRotate180 => "--awsui-motion-duration-rotate180",
            Self::MotionDurationRotate90 => "--awsui-motion-duration-rotate90",
            Self::MotionDurationShowPaced => "--awsui-motion-duration-show-paced",
            Self::MotionDurationShowQuick => "--awsui-motion-duration-show-quick",
            Self::MotionDurationSlow => "--awsui-motion-duration-slow",
            Self::MotionDurationTransitionQuick => "--awsui-motion-duration-transition-quick",
            Self::MotionDurationTransitionShowPaced => "--awsui-motion-duration-transition-show-paced",
            Self::MotionDurationTransitionShowQuick => "--awsui-motion-duration-transition-show-quick",
            Self::MotionEasingEaseOutQuart => "--awsui-motion-easing-ease-out-quart",
            Self::MotionEasingRefreshOnlyA => "--awsui-motion-easing-refresh-only-a",
            Self::MotionEasingRefreshOnlyB => "--awsui-motion-easing-refresh-only-b",
            Self::MotionEasingRefreshOnlyC => "--awsui-motion-easing-refresh-only-c",
            Self::MotionEasingRefreshOnlyD => "--awsui-motion-easing-refresh-only-d",
            Self::MotionEasingAvatarGenAiGradient => "--awsui-motion-easing-avatar-gen-ai-gradient",
            Self::MotionEasingRotate180 => "--awsui-motion-easing-rotate180",
            Self::MotionEasingRotate90 => "--awsui-motion-easing-rotate90",
            Self::MotionEasingShowPaced => "--awsui-motion-easing-show-paced",
            Self::MotionEasingShowQuick => "--awsui-motion-easing-show-quick",
            Self::MotionEasingTransitionQuick => "--awsui-motion-easing-transition-quick",
            Self::MotionEasingTransitionShowPaced => "--awsui-motion-easing-transition-show-paced",
            Self::MotionEasingTransitionShowQuick => "--awsui-motion-easing-transition-show-quick",
            Self::MotionEasingResponsive => "--awsui-motion-easing-responsive",
            Self::MotionEasingSticky => "--awsui-motion-easing-sticky",
            Self::MotionEasingExpressive => "--awsui-motion-easing-expressive",
            Self::MotionDurationResponsive => "--awsui-motion-duration-responsive",
            Self::MotionDurationExpressive => "--awsui-motion-duration-expressive",
            Self::MotionDurationComplex => "--awsui-motion-duration-complex",
            Self::MotionKeyframesFadeIn => "--awsui-motion-keyframes-fade-in",
            Self::MotionKeyframesFadeOut => "--awsui-motion-keyframes-fade-out",
            Self::MotionKeyframesStatusIconError => "--awsui-motion-keyframes-status-icon-error",
            Self::MotionKeyframesScalePopup => "--awsui-motion-keyframes-scale-popup",
        }
    }

    /// Get the CSS var() function for use in styles
    pub fn css_var(&self) -> String {
        format!("var({})", self.css_var_name())
    }
}

impl fmt::Display for MotionToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_var())
    }
}

/// Size design tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizeToken {
    SizeCalendarGridWidth,
    SizeControl,
    SizeIconBig,
    SizeIconLarge,
    SizeIconMedium,
    SizeIconNormal,
    SizeTableSelectionHorizontal,
    SizeVerticalInput,
    SizeVerticalPanelIconOffset,
}

impl SizeToken {
    /// Get the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
            Self::SizeCalendarGridWidth => "--awsui-size-calendar-grid-width",
            Self::SizeControl => "--awsui-size-control",
            Self::SizeIconBig => "--awsui-size-icon-big",
            Self::SizeIconLarge => "--awsui-size-icon-large",
            Self::SizeIconMedium => "--awsui-size-icon-medium",
            Self::SizeIconNormal => "--awsui-size-icon-normal",
            Self::SizeTableSelectionHorizontal => "--awsui-size-table-selection-horizontal",
            Self::SizeVerticalInput => "--awsui-size-vertical-input",
            Self::SizeVerticalPanelIconOffset => "--awsui-size-vertical-panel-icon-offset",
        }
    }

    /// Get the CSS var() function for use in styles
    pub fn css_var(&self) -> String {
        format!("var({})", self.css_var_name())
    }
}

impl fmt::Display for SizeToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_var())
    }
}

/// Shadow design tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShadowToken {
    /// Shadow for containers and cards.
    ShadowContainer,
    /// Shadow for containers and cards in active state.
    ShadowContainerActive,
    ShadowDropdown,
    /// Shadow for dropdown elements that pop up above the trigger, for example a dropdown at the bottom of the screen.
    ShadowDropup,
    ShadowFlashCollapsed,
    ShadowFlashSticky,
    ShadowModal,
    /// Shadow for global elements like app layout panels and top navigation.
    ShadowPanel,
    /// Shadow for circular toggles in visual refresh.
    ShadowPanelToggle,
    ShadowPopover,
    /// Adjustment to the panel shadow so it displays the same for panels positioned at the bottom of the screen.
    ShadowSplitBottom,
    /// Adjustment to the panel shadow so it does not bleed onto adjacent panels to the right of it.
    ShadowSplitSide,
    /// Shadow for sticky elements or inner elements that already have shadow around.
    ShadowSticky,
    ShadowStickyEmbedded,
    ShadowStickyColumnFirst,
    ShadowStickyColumnLast,
}

impl ShadowToken {
    /// Get the CSS custom property name for this token
    pub fn css_var_name(&self) -> &'static str {
        match self {
            Self::ShadowContainer => "--awsui-shadow-container",
            Self::ShadowContainerActive => "--awsui-shadow-container-active",
            Self::ShadowDropdown => "--awsui-shadow-dropdown",
            Self::ShadowDropup => "--awsui-shadow-dropup",
            Self::ShadowFlashCollapsed => "--awsui-shadow-flash-collapsed",
            Self::ShadowFlashSticky => "--awsui-shadow-flash-sticky",
            Self::ShadowModal => "--awsui-shadow-modal",
            Self::ShadowPanel => "--awsui-shadow-panel",
            Self::ShadowPanelToggle => "--awsui-shadow-panel-toggle",
            Self::ShadowPopover => "--awsui-shadow-popover",
            Self::ShadowSplitBottom => "--awsui-shadow-split-bottom",
            Self::ShadowSplitSide => "--awsui-shadow-split-side",
            Self::ShadowSticky => "--awsui-shadow-sticky",
            Self::ShadowStickyEmbedded => "--awsui-shadow-sticky-embedded",
            Self::ShadowStickyColumnFirst => "--awsui-shadow-sticky-column-first",
            Self::ShadowStickyColumnLast => "--awsui-shadow-sticky-column-last",
        }
    }

    /// Get the CSS var() function for use in styles
    pub fn css_var(&self) -> String {
        format!("var({})", self.css_var_name())
    }
}

impl fmt::Display for ShadowToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_var())
    }
}

