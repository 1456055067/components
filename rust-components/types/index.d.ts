// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * TypeScript Type Definitions for Cloudscape Rust/WASM Components
 *
 * These definitions describe the component API as implemented in Rust/Yew.
 * They can be used for:
 * - Documentation purposes
 * - Type checking in hybrid React/Rust codebases
 * - Future JavaScript binding generation
 *
 * @packageDocumentation
 */

// =============================================================================
// Base Types
// =============================================================================

/**
 * Common properties shared by all components
 */
export interface BaseComponentProps {
  /** Unique identifier for the component */
  id?: string;
  /** Additional CSS class names */
  className?: string;
  /** Inline styles as a string */
  style?: string;
  /** ARIA label for accessibility */
  ariaLabel?: string;
  /** ID of element that describes this component */
  ariaDescribedBy?: string;
  /** ID of element that labels this component */
  ariaLabelledBy?: string;
  /** Data attributes for testing or analytics */
  data?: Record<string, string>;
}

/**
 * Generic custom event wrapper
 */
export interface CustomEvent<T> {
  /** Event detail payload */
  detail: T;
  /** Whether the event can be cancelled */
  cancelable: boolean;
  /** Whether preventDefault() was called */
  defaultPrevented: boolean;
}

/**
 * Mouse click event details
 */
export interface ClickDetail {
  /** Mouse button (0 = left, 1 = middle, 2 = right) */
  button: number;
  /** Whether Ctrl key was pressed */
  ctrlKey: boolean;
  /** Whether Shift key was pressed */
  shiftKey: boolean;
  /** Whether Alt key was pressed */
  altKey: boolean;
  /** Whether Meta key was pressed */
  metaKey: boolean;
}

/**
 * Event handler type for component callbacks
 */
export type EventHandler<T> = (event: CustomEvent<T>) => void;

// =============================================================================
// Component Enums
// =============================================================================

/** Button visual variants */
export type ButtonVariant = 'normal' | 'primary' | 'link' | 'icon' | 'inline-icon' | 'inline-link';

/** Alert message types */
export type AlertType = 'info' | 'success' | 'warning' | 'error';

/** Spinner sizes */
export type SpinnerSize = 'normal' | 'big' | 'large';

/** Badge color variants */
export type BadgeColor = 'blue' | 'grey' | 'green' | 'red';

/** Input field types */
export type InputType = 'text' | 'password' | 'search' | 'number' | 'email' | 'url';

/** Box padding/margin variants */
export type BoxSpacing = 'n' | 'xxxs' | 'xxs' | 'xs' | 's' | 'm' | 'l' | 'xl' | 'xxl' | 'xxxl';

/** Box display variants */
export type BoxVariant =
  | 'div'
  | 'span'
  | 'h1'
  | 'h2'
  | 'h3'
  | 'h4'
  | 'h5'
  | 'p'
  | 'strong'
  | 'small'
  | 'code'
  | 'pre'
  | 'samp';

/** Modal sizes */
export type ModalSize = 'small' | 'medium' | 'large' | 'max';

/** Dismissal reasons for modals and overlays */
export type DismissReason = 'closeButton' | 'overlay' | 'keyboard';

/** Status indicator types */
export type StatusIndicatorType =
  | 'error'
  | 'warning'
  | 'success'
  | 'info'
  | 'stopped'
  | 'pending'
  | 'in-progress'
  | 'loading';

/** Header variants */
export type HeaderVariant = 'h1' | 'h2' | 'h3' | 'awsui-h1-sticky';

/** Space between sizes */
export type SpaceBetweenSize = 'xxxs' | 'xxs' | 'xs' | 's' | 'm' | 'l' | 'xl' | 'xxl';

/** Space between directions */
export type SpaceBetweenDirection = 'horizontal' | 'vertical';

/** Icon names (subset of commonly used) */
export type IconName =
  | 'add-plus'
  | 'angle-down'
  | 'angle-left'
  | 'angle-right'
  | 'angle-up'
  | 'calendar'
  | 'caret-down'
  | 'caret-left'
  | 'caret-right'
  | 'caret-up'
  | 'check'
  | 'close'
  | 'copy'
  | 'download'
  | 'edit'
  | 'envelope'
  | 'external'
  | 'file'
  | 'filter'
  | 'folder'
  | 'key'
  | 'lock-private'
  | 'menu'
  | 'notification'
  | 'refresh'
  | 'remove'
  | 'search'
  | 'settings'
  | 'share'
  | 'status-info'
  | 'status-negative'
  | 'status-pending'
  | 'status-positive'
  | 'status-warning'
  | 'treeview-collapse'
  | 'treeview-expand'
  | 'upload'
  | 'user-profile'
  | 'view-full'
  | 'zoom-in'
  | 'zoom-out'
  | string;

/** Icon sizes */
export type IconSize = 'small' | 'normal' | 'medium' | 'big' | 'large' | 'inherit';

/** Link variants */
export type LinkVariant = 'primary' | 'secondary' | 'info' | 'awsui-value-large';

/** Tabs variants */
export type TabsVariant = 'default' | 'container' | 'stacked';

/** Toggle variants */
export type ToggleVariant = 'normal' | 'disabled';

/** Progress bar variants */
export type ProgressBarVariant = 'standalone' | 'flash' | 'key-value';

/** Progress bar status */
export type ProgressBarStatus = 'in-progress' | 'success' | 'error';

// =============================================================================
// Event Detail Types
// =============================================================================

/** Input change event detail */
export interface InputChangeDetail {
  value: string;
}

/** Checkbox change event detail */
export interface CheckboxChangeDetail {
  checked: boolean;
  indeterminate: boolean;
}

/** Toggle change event detail */
export interface ToggleChangeDetail {
  checked: boolean;
}

/** Select change event detail */
export interface SelectChangeDetail {
  selectedOption: SelectOption | null;
}

/** Multiselect change event detail */
export interface MultiselectChangeDetail {
  selectedOptions: SelectOption[];
}

/** Select option definition */
export interface SelectOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
  iconName?: IconName;
  iconUrl?: string;
  iconAlt?: string;
  tags?: string[];
  filteringTags?: string[];
  labelTag?: string;
}

/** Radio group change event detail */
export interface RadioGroupChangeDetail {
  value: string;
}

/** Textarea change event detail */
export interface TextareaChangeDetail {
  value: string;
}

/** Tab change event detail */
export interface TabChangeDetail {
  activeTabId: string;
}

/** Modal dismiss event detail */
export interface ModalDismissDetail {
  reason: DismissReason;
}

/** Drawer dismiss event detail */
export interface DrawerDismissDetail {
  reason: DismissReason;
}

/** Expandable section change event detail */
export interface ExpandableSectionChangeDetail {
  expanded: boolean;
}

/** Pagination change event detail */
export interface PaginationChangeDetail {
  currentPageIndex: number;
}

/** Date picker change event detail */
export interface DatePickerChangeDetail {
  value: string;
}

/** Date range picker change event detail */
export interface DateRangePickerChangeDetail {
  value: DateRangeValue | null;
}

/** Date range value */
export interface DateRangeValue {
  type: 'absolute' | 'relative';
  startDate?: string;
  endDate?: string;
  amount?: number;
  unit?: 'second' | 'minute' | 'hour' | 'day' | 'week' | 'month' | 'year';
  key?: string;
}

/** File upload change event detail */
export interface FileUploadChangeDetail {
  value: File[];
}

/** Autosuggest change event detail */
export interface AutosuggestChangeDetail {
  value: string;
}

/** Autosuggest select event detail */
export interface AutosuggestSelectDetail {
  selectedOption: SelectOption | null;
}

/** Table selection change event detail */
export interface TableSelectionChangeDetail<T> {
  selectedItems: T[];
}

/** Table sort event detail */
export interface TableSortingChangeDetail<T> {
  sortingColumn: TableColumn<T>;
  sortingDescending: boolean;
}

/** Table column definition */
export interface TableColumn<T> {
  id: string;
  header: string;
  cell: (item: T) => string;
  sortingField?: string;
  sortingComparator?: (a: T, b: T) => number;
  width?: number | string;
  minWidth?: number | string;
}

/** Follow event detail for navigation */
export interface FollowDetail {
  href: string;
  external: boolean;
}

/** Token dismiss event detail */
export interface TokenDismissDetail {
  itemIndex: number;
}

/** Copy to clipboard event detail */
export interface CopyToClipboardDetail {
  success: boolean;
  text: string;
}

// =============================================================================
// Component Props
// =============================================================================

/** Alert component props */
export interface AlertProps extends BaseComponentProps {
  type?: AlertType;
  visible?: boolean;
  dismissible?: boolean;
  header?: string;
  buttonText?: string;
  onDismiss?: EventHandler<void>;
  onButtonClick?: EventHandler<ClickDetail>;
}

/** Badge component props */
export interface BadgeProps extends BaseComponentProps {
  color?: BadgeColor;
}

/** Box component props */
export interface BoxProps extends BaseComponentProps {
  variant?: BoxVariant;
  padding?: BoxSpacing | { top?: BoxSpacing; bottom?: BoxSpacing; left?: BoxSpacing; right?: BoxSpacing };
  margin?: BoxSpacing | { top?: BoxSpacing; bottom?: BoxSpacing; left?: BoxSpacing; right?: BoxSpacing };
  display?: 'block' | 'inline' | 'inline-block' | 'none';
  float?: 'left' | 'right' | 'none';
  fontSize?:
    | 'body-s'
    | 'body-m'
    | 'heading-xs'
    | 'heading-s'
    | 'heading-m'
    | 'heading-l'
    | 'heading-xl'
    | 'display-l'
    | 'inherit';
  fontWeight?: 'light' | 'normal' | 'bold' | 'heavy' | 'inherit';
  color?:
    | 'inherit'
    | 'text-body-default'
    | 'text-body-secondary'
    | 'text-status-error'
    | 'text-status-success'
    | 'text-status-info'
    | 'text-status-inactive'
    | 'text-status-warning';
  textAlign?: 'left' | 'center' | 'right' | 'inherit';
}

/** Button component props */
export interface ButtonProps extends BaseComponentProps {
  variant?: ButtonVariant;
  disabled?: boolean;
  loading?: boolean;
  loadingText?: string;
  iconName?: IconName;
  iconUrl?: string;
  iconAlt?: string;
  iconAlign?: 'left' | 'right';
  href?: string;
  target?: '_blank' | '_self' | '_parent' | '_top' | string;
  rel?: string;
  download?: string | boolean;
  formAction?: string;
  ariaExpanded?: boolean;
  onClick?: EventHandler<ClickDetail>;
  onFollow?: EventHandler<FollowDetail>;
}

/** Checkbox component props */
export interface CheckboxProps extends BaseComponentProps {
  checked?: boolean;
  indeterminate?: boolean;
  disabled?: boolean;
  name?: string;
  description?: string;
  onChange?: EventHandler<CheckboxChangeDetail>;
}

/** Container component props */
export interface ContainerProps extends BaseComponentProps {
  header?: string;
  footer?: string;
  disableContentPaddings?: boolean;
  disableHeaderPaddings?: boolean;
  variant?: 'default' | 'stacked';
  fitHeight?: boolean;
}

/** Header component props */
export interface HeaderProps extends BaseComponentProps {
  variant?: HeaderVariant;
  description?: string;
  info?: string;
  counter?: string;
  actions?: string;
}

/** Icon component props */
export interface IconProps extends BaseComponentProps {
  name?: IconName;
  url?: string;
  alt?: string;
  size?: IconSize;
  variant?: 'normal' | 'subtle' | 'inverted' | 'disabled' | 'success' | 'error' | 'warning' | 'link';
}

/** Input component props */
export interface InputProps extends BaseComponentProps {
  type?: InputType;
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  readOnly?: boolean;
  autoFocus?: boolean;
  autoComplete?: boolean | string;
  name?: string;
  step?: number | 'any';
  inputMode?: 'none' | 'text' | 'decimal' | 'numeric' | 'tel' | 'search' | 'email' | 'url';
  spellcheck?: boolean;
  onChange?: EventHandler<InputChangeDetail>;
  onBlur?: EventHandler<void>;
  onFocus?: EventHandler<void>;
}

/** Link component props */
export interface LinkProps extends BaseComponentProps {
  variant?: LinkVariant;
  href?: string;
  target?: '_blank' | '_self' | '_parent' | '_top' | string;
  rel?: string;
  external?: boolean;
  externalIconAriaLabel?: string;
  fontSize?:
    | 'body-s'
    | 'body-m'
    | 'heading-xs'
    | 'heading-s'
    | 'heading-m'
    | 'heading-l'
    | 'heading-xl'
    | 'display-l'
    | 'inherit';
  color?: 'normal' | 'inverted';
  onFollow?: EventHandler<FollowDetail>;
}

/** Modal component props */
export interface ModalProps extends BaseComponentProps {
  visible?: boolean;
  size?: ModalSize;
  header?: string;
  footer?: string;
  closeAriaLabel?: string;
  onDismiss?: EventHandler<ModalDismissDetail>;
}

/** Progress bar component props */
export interface ProgressBarProps extends BaseComponentProps {
  value?: number;
  status?: ProgressBarStatus;
  variant?: ProgressBarVariant;
  resultText?: string;
  label?: string;
  description?: string;
  additionalInfo?: string;
}

/** Radio group component props */
export interface RadioGroupProps extends BaseComponentProps {
  value?: string | null;
  items?: RadioGroupItem[];
  name?: string;
  onChange?: EventHandler<RadioGroupChangeDetail>;
}

/** Radio group item definition */
export interface RadioGroupItem {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

/** Select component props */
export interface SelectProps extends BaseComponentProps {
  selectedOption?: SelectOption | null;
  options?: SelectOption[];
  placeholder?: string;
  disabled?: boolean;
  filteringType?: 'none' | 'auto' | 'manual';
  triggerVariant?: 'label' | 'option';
  onChange?: EventHandler<SelectChangeDetail>;
}

/** Multiselect component props */
export interface MultiselectProps extends BaseComponentProps {
  selectedOptions?: SelectOption[];
  options?: SelectOption[];
  placeholder?: string;
  disabled?: boolean;
  filteringType?: 'none' | 'auto' | 'manual';
  tokenLimit?: number;
  hideTokens?: boolean;
  onChange?: EventHandler<MultiselectChangeDetail>;
}

/** Space between component props */
export interface SpaceBetweenProps extends BaseComponentProps {
  size?: SpaceBetweenSize;
  direction?: SpaceBetweenDirection;
  alignItems?: 'center' | 'start' | 'end';
}

/** Spinner component props */
export interface SpinnerProps extends BaseComponentProps {
  size?: SpinnerSize;
  variant?: 'normal' | 'disabled' | 'inverted';
}

/** Status indicator component props */
export interface StatusIndicatorProps extends BaseComponentProps {
  type?: StatusIndicatorType;
  colorOverride?: 'red' | 'green' | 'blue' | 'grey';
  wrapText?: boolean;
}

/** Tabs component props */
export interface TabsProps extends BaseComponentProps {
  activeTabId?: string;
  tabs?: TabDefinition[];
  variant?: TabsVariant;
  disableContentPaddings?: boolean;
  onChange?: EventHandler<TabChangeDetail>;
}

/** Tab definition */
export interface TabDefinition {
  id: string;
  label: string;
  disabled?: boolean;
  href?: string;
}

/** Textarea component props */
export interface TextareaProps extends BaseComponentProps {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  readOnly?: boolean;
  autoFocus?: boolean;
  name?: string;
  rows?: number;
  spellcheck?: boolean;
  onChange?: EventHandler<TextareaChangeDetail>;
  onBlur?: EventHandler<void>;
  onFocus?: EventHandler<void>;
}

/** Toggle component props */
export interface ToggleProps extends BaseComponentProps {
  checked?: boolean;
  disabled?: boolean;
  description?: string;
  onChange?: EventHandler<ToggleChangeDetail>;
}

/** Column layout component props */
export interface ColumnLayoutProps extends BaseComponentProps {
  columns?: 1 | 2 | 3 | 4;
  variant?: 'default' | 'text-grid';
  borders?: 'none' | 'horizontal' | 'vertical' | 'all';
  minColumnWidth?: number;
  disableGutters?: boolean;
}

/** Expandable section component props */
export interface ExpandableSectionProps extends BaseComponentProps {
  variant?: 'default' | 'container' | 'footer' | 'navigation' | 'stacked';
  expanded?: boolean;
  defaultExpanded?: boolean;
  headerText?: string;
  headerDescription?: string;
  headingTagOverride?: 'h1' | 'h2' | 'h3' | 'h4' | 'h5';
  onChange?: EventHandler<ExpandableSectionChangeDetail>;
}

/** Pagination component props */
export interface PaginationProps extends BaseComponentProps {
  currentPageIndex?: number;
  pagesCount?: number;
  disabled?: boolean;
  openEnd?: boolean;
  ariaLabels?: {
    nextPageLabel?: string;
    previousPageLabel?: string;
    pageLabel?: (pageNumber: number) => string;
  };
  onChange?: EventHandler<PaginationChangeDetail>;
}

/** Form field component props */
export interface FormFieldProps extends BaseComponentProps {
  label?: string;
  description?: string;
  info?: string;
  constraintText?: string;
  errorText?: string;
  stretch?: boolean;
}

/** Drawer component props */
export interface DrawerProps extends BaseComponentProps {
  visible?: boolean;
  header?: string;
  loading?: boolean;
  onDismiss?: EventHandler<DrawerDismissDetail>;
}

/** Flashbar component props */
export interface FlashbarProps extends BaseComponentProps {
  items?: FlashbarItem[];
  stackItems?: boolean;
}

/** Flashbar item definition */
export interface FlashbarItem {
  type?: AlertType;
  header?: string;
  content?: string;
  buttonText?: string;
  dismissible?: boolean;
  dismissLabel?: string;
  loading?: boolean;
  id?: string;
  onDismiss?: () => void;
  onButtonClick?: () => void;
}

/** Token group component props */
export interface TokenGroupProps extends BaseComponentProps {
  items?: TokenItem[];
  limit?: number;
  alignment?: 'horizontal' | 'vertical';
  onDismiss?: EventHandler<TokenDismissDetail>;
}

/** Token item definition */
export interface TokenItem {
  label: string;
  description?: string;
  iconName?: IconName;
  iconUrl?: string;
  iconAlt?: string;
  disabled?: boolean;
  dismissLabel?: string;
  tags?: string[];
}

/** Popover component props */
export interface PopoverProps extends BaseComponentProps {
  position?: 'top' | 'right' | 'bottom' | 'left';
  size?: 'small' | 'medium' | 'large';
  fixedWidth?: boolean;
  triggerType?: 'text' | 'custom';
  header?: string;
  dismissAriaLabel?: string;
  renderWithPortal?: boolean;
}

/** Breadcrumb group component props */
export interface BreadcrumbGroupProps extends BaseComponentProps {
  items?: BreadcrumbItem[];
  ariaLabel?: string;
  expandAriaLabel?: string;
  onFollow?: EventHandler<FollowDetail>;
}

/** Breadcrumb item definition */
export interface BreadcrumbItem {
  text: string;
  href: string;
}

/** Key value pairs component props */
export interface KeyValuePairsProps extends BaseComponentProps {
  items?: KeyValuePairItem[];
  columns?: number;
}

/** Key value pair item definition */
export interface KeyValuePairItem {
  label: string;
  value: string;
  info?: string;
}

/** Copy to clipboard component props */
export interface CopyToClipboardProps extends BaseComponentProps {
  copyButtonText?: string;
  copySuccessText?: string;
  copyErrorText?: string;
  textToCopy: string;
  variant?: 'button' | 'icon' | 'inline';
  onCopy?: EventHandler<CopyToClipboardDetail>;
}

/** File upload component props */
export interface FileUploadProps extends BaseComponentProps {
  value?: File[];
  multiple?: boolean;
  accept?: string;
  showFileLastModified?: boolean;
  showFileSize?: boolean;
  showFileThumbnail?: boolean;
  tokenLimit?: number;
  constraintText?: string;
  errorText?: string;
  fileErrors?: (string | null)[];
  i18nStrings?: FileUploadI18nStrings;
  onChange?: EventHandler<FileUploadChangeDetail>;
}

/** File upload i18n strings */
export interface FileUploadI18nStrings {
  uploadButtonText?: (multiple: boolean) => string;
  dropzoneText?: (multiple: boolean) => string;
  removeFileAriaLabel?: (fileIndex: number) => string;
  limitShowFewer?: string;
  limitShowMore?: string;
  errorIconAriaLabel?: string;
}

/** Date picker component props */
export interface DatePickerProps extends BaseComponentProps {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  readOnly?: boolean;
  locale?: string;
  startOfWeek?: 0 | 1 | 2 | 3 | 4 | 5 | 6;
  isDateEnabled?: (date: Date) => boolean;
  nextMonthAriaLabel?: string;
  previousMonthAriaLabel?: string;
  todayAriaLabel?: string;
  onChange?: EventHandler<DatePickerChangeDetail>;
}

/** Date range picker component props */
export interface DateRangePickerProps extends BaseComponentProps {
  value?: DateRangeValue | null;
  placeholder?: string;
  disabled?: boolean;
  readOnly?: boolean;
  locale?: string;
  startOfWeek?: 0 | 1 | 2 | 3 | 4 | 5 | 6;
  isDateEnabled?: (date: Date) => boolean;
  dateOnly?: boolean;
  rangeSelectorMode?: 'default' | 'absolute-only' | 'relative-only';
  relativeOptions?: RelativeTimeOption[];
  i18nStrings?: DateRangePickerI18nStrings;
  onChange?: EventHandler<DateRangePickerChangeDetail>;
}

/** Relative time option for date range picker */
export interface RelativeTimeOption {
  key: string;
  amount: number;
  unit: 'second' | 'minute' | 'hour' | 'day' | 'week' | 'month' | 'year';
  type: 'relative';
}

/** Date range picker i18n strings */
export interface DateRangePickerI18nStrings {
  todayAriaLabel?: string;
  nextMonthAriaLabel?: string;
  previousMonthAriaLabel?: string;
  startDateLabel?: string;
  endDateLabel?: string;
  startTimeLabel?: string;
  endTimeLabel?: string;
  clearButtonLabel?: string;
  cancelButtonLabel?: string;
  applyButtonLabel?: string;
  customRelativeRangeOptionLabel?: string;
  customRelativeRangeOptionDescription?: string;
  customRelativeRangeDurationLabel?: string;
  customRelativeRangeDurationPlaceholder?: string;
  customRelativeRangeUnitLabel?: string;
  formatRelativeRange?: (value: DateRangeValue) => string;
  formatUnit?: (unit: string, value: number) => string;
  dateTimeConstraintText?: string;
  modeSelectionLabel?: string;
  relativeModeTitle?: string;
  absoluteModeTitle?: string;
  relativeRangeSelectionHeading?: string;
  renderSelectedAbsoluteRangeAriaLive?: (startDate: string, endDate: string) => string;
}

/** Autosuggest component props */
export interface AutosuggestProps extends BaseComponentProps {
  value?: string;
  options?: SelectOption[];
  placeholder?: string;
  disabled?: boolean;
  readOnly?: boolean;
  filteringType?: 'none' | 'auto' | 'manual';
  statusType?: 'pending' | 'loading' | 'finished' | 'error';
  empty?: string;
  loadingText?: string;
  finishedText?: string;
  errorText?: string;
  recoveryText?: string;
  enteredTextLabel?: (value: string) => string;
  onChange?: EventHandler<AutosuggestChangeDetail>;
  onSelect?: EventHandler<AutosuggestSelectDetail>;
  onLoadItems?: EventHandler<{ filteringText: string; firstPage: boolean; samePage: boolean }>;
}

/** Table component props */
export interface TableProps<T> extends BaseComponentProps {
  items?: T[];
  columnDefinitions?: TableColumn<T>[];
  header?: string;
  footer?: string;
  empty?: string;
  loading?: boolean;
  loadingText?: string;
  filter?: string;
  pagination?: string;
  preferences?: string;
  selectionType?: 'single' | 'multi';
  selectedItems?: T[];
  sortingColumn?: TableColumn<T>;
  sortingDescending?: boolean;
  sortingDisabled?: boolean;
  stickyHeader?: boolean;
  stickyHeaderVerticalOffset?: number;
  stripedRows?: boolean;
  wrapLines?: boolean;
  trackBy?: string | ((item: T) => string);
  ariaLabels?: {
    allItemsSelectionLabel?: (data: { selectedItems: T[] }) => string;
    itemSelectionLabel?: (data: { selectedItems: T[] }, item: T) => string;
    selectionGroupLabel?: string;
    tableLabel?: string;
    resizerRoleDescription?: string;
    activateEditLabel?: (column: TableColumn<T>, item: T) => string;
    cancelEditLabel?: (column: TableColumn<T>) => string;
    submitEditLabel?: (column: TableColumn<T>) => string;
  };
  onSelectionChange?: EventHandler<TableSelectionChangeDetail<T>>;
  onSortingChange?: EventHandler<TableSortingChangeDetail<T>>;
}

/** Cards component props */
export interface CardsProps<T> extends BaseComponentProps {
  items?: T[];
  cardDefinition?: CardDefinition<T>;
  header?: string;
  empty?: string;
  loading?: boolean;
  loadingText?: string;
  filter?: string;
  pagination?: string;
  preferences?: string;
  selectionType?: 'single' | 'multi';
  selectedItems?: T[];
  cardsPerRow?: { minWidth: number; cards: number }[];
  stickyHeader?: boolean;
  stickyHeaderVerticalOffset?: number;
  trackBy?: string | ((item: T) => string);
  onSelectionChange?: EventHandler<TableSelectionChangeDetail<T>>;
}

/** Card definition for Cards component */
export interface CardDefinition<T> {
  header?: (item: T) => string;
  sections?: Array<{
    id?: string;
    header?: string;
    content?: (item: T) => string;
    width?: number;
  }>;
}

/** Tiles component props */
export interface TilesProps extends BaseComponentProps {
  items?: TileItem[];
  value?: string | null;
  columns?: number;
  onChange?: EventHandler<{ value: string }>;
}

/** Tile item definition */
export interface TileItem {
  value: string;
  label: string;
  description?: string;
  image?: string;
  disabled?: boolean;
}

/** App layout component props */
export interface AppLayoutProps extends BaseComponentProps {
  content?: string;
  navigation?: string;
  navigationOpen?: boolean;
  navigationHide?: boolean;
  navigationWidth?: number;
  tools?: string;
  toolsOpen?: boolean;
  toolsHide?: boolean;
  toolsWidth?: number;
  breadcrumbs?: string;
  notifications?: string;
  headerSelector?: string;
  footerSelector?: string;
  stickyNotifications?: boolean;
  disableBodyScroll?: boolean;
  disableContentPaddings?: boolean;
  disableContentHeaderOverlap?: boolean;
  maxContentWidth?: number;
  minContentWidth?: number;
  splitPanel?: string;
  splitPanelOpen?: boolean;
  splitPanelPreferences?: { position: 'bottom' | 'side' };
  drawers?: DrawerDefinition[];
  activeDrawerId?: string;
  onNavigationChange?: EventHandler<{ open: boolean }>;
  onToolsChange?: EventHandler<{ open: boolean }>;
  onSplitPanelToggle?: EventHandler<{ open: boolean }>;
  onSplitPanelPreferencesChange?: EventHandler<{ position: 'bottom' | 'side' }>;
  onDrawerChange?: EventHandler<{ activeDrawerId: string | null }>;
}

/** Drawer definition for AppLayout */
export interface DrawerDefinition {
  id: string;
  content: string;
  trigger?: {
    iconName?: IconName;
    iconUrl?: string;
    iconAlt?: string;
  };
  ariaLabels?: {
    drawerName?: string;
    closeButton?: string;
    triggerButton?: string;
    resizeHandle?: string;
  };
  badge?: boolean;
  resizable?: boolean;
  defaultSize?: number;
  onResize?: EventHandler<{ size: number }>;
}

/** Content layout component props */
export interface ContentLayoutProps extends BaseComponentProps {
  header?: string;
  disableOverlap?: boolean;
  maxContentWidth?: number;
  headerVariant?: 'default' | 'high-contrast';
}

/** Side navigation component props */
export interface SideNavigationProps extends BaseComponentProps {
  activeHref?: string;
  header?: {
    text: string;
    href: string;
    logo?: {
      src: string;
      alt: string;
    };
  };
  items?: SideNavigationItem[];
  onFollow?: EventHandler<FollowDetail>;
  onChange?: EventHandler<{ expandedSections: string[] }>;
}

/** Side navigation item */
export type SideNavigationItem =
  | {
      type: 'link';
      text: string;
      href: string;
      external?: boolean;
      externalIconAriaLabel?: string;
      info?: string;
      badge?: boolean;
    }
  | { type: 'section'; text: string; defaultExpanded?: boolean; items: SideNavigationItem[] }
  | { type: 'section-group'; title: string; items: SideNavigationItem[] }
  | {
      type: 'expandable-link-group';
      text: string;
      href: string;
      defaultExpanded?: boolean;
      items: SideNavigationItem[];
    }
  | { type: 'link-group'; text: string; href: string; items: SideNavigationItem[] }
  | { type: 'divider' };

/** Top navigation component props */
export interface TopNavigationProps extends BaseComponentProps {
  identity?: {
    title: string;
    href: string;
    logo?: {
      src: string;
      alt: string;
    };
    onFollow?: EventHandler<FollowDetail>;
  };
  utilities?: TopNavigationUtility[];
  i18nStrings?: {
    searchIconAriaLabel?: string;
    searchDismissIconAriaLabel?: string;
    overflowMenuTriggerText?: string;
    overflowMenuTitleText?: string;
    overflowMenuBackIconAriaLabel?: string;
    overflowMenuDismissIconAriaLabel?: string;
  };
}

/** Top navigation utility */
export type TopNavigationUtility =
  | {
      type: 'button';
      iconName?: IconName;
      text?: string;
      title?: string;
      ariaLabel?: string;
      badge?: boolean;
      disableTextCollapse?: boolean;
      disableUtilityCollapse?: boolean;
      onClick?: () => void;
    }
  | {
      type: 'menu-dropdown';
      iconName?: IconName;
      text?: string;
      title?: string;
      ariaLabel?: string;
      badge?: boolean;
      disableTextCollapse?: boolean;
      disableUtilityCollapse?: boolean;
      items: MenuDropdownItem[];
    };

/** Menu dropdown item */
export interface MenuDropdownItem {
  id: string;
  text: string;
  description?: string;
  href?: string;
  external?: boolean;
  externalIconAriaLabel?: string;
  disabled?: boolean;
  iconName?: IconName;
  iconUrl?: string;
  iconAlt?: string;
  items?: MenuDropdownItem[];
}

/** Button dropdown component props */
export interface ButtonDropdownProps extends BaseComponentProps {
  items?: ButtonDropdownItem[];
  disabled?: boolean;
  loading?: boolean;
  variant?: 'normal' | 'primary' | 'icon' | 'inline-icon';
  mainAction?: {
    text: string;
    disabled?: boolean;
    loading?: boolean;
    external?: boolean;
    externalIconAriaLabel?: string;
    href?: string;
    iconName?: IconName;
    onClick?: EventHandler<ClickDetail>;
    onFollow?: EventHandler<FollowDetail>;
  };
  expandToViewport?: boolean;
  expandableGroups?: boolean;
  onItemClick?: EventHandler<{ id: string }>;
  onItemFollow?: EventHandler<{ id: string; href: string; external?: boolean }>;
}

/** Button dropdown item */
export type ButtonDropdownItem =
  | {
      id: string;
      text: string;
      description?: string;
      href?: string;
      external?: boolean;
      externalIconAriaLabel?: string;
      disabled?: boolean;
      disabledReason?: string;
      iconName?: IconName;
      iconUrl?: string;
      iconAlt?: string;
    }
  | { id: string; text: string; items: ButtonDropdownItem[] };
