// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * TypeScript Example - Using Cloudscape Rust Component Types
 *
 * This file demonstrates how to use the TypeScript type definitions
 * for the Cloudscape Rust/WASM components. These types can be used for:
 *
 * 1. Type-checking component configurations
 * 2. Building type-safe wrapper components
 * 3. Documenting component APIs
 * 4. IDE autocompletion and validation
 */

import type {
  // Component props
  AlertProps,
  // Enums
  AlertType,
  // Base types
  BaseComponentProps,
  ButtonProps,
  ButtonVariant,
  // Event details
  ClickDetail,
  CustomEvent,
  EventHandler,
  InputChangeDetail,
  InputProps,
  ModalDismissDetail,
  ModalProps,
  SelectChangeDetail,
  SelectOption,
  SelectProps,
  StatusIndicatorProps,
  StatusIndicatorType,
  TableColumn,
  TableProps,
  TableSelectionChangeDetail,
} from '../types';

// =============================================================================
// Example 1: Type-safe button configuration
// =============================================================================

const primaryButton: ButtonProps = {
  variant: 'primary',
  disabled: false,
  loading: false,
  iconName: 'add-plus',
  iconAlign: 'left',
  onClick: (event: CustomEvent<ClickDetail>) => {
    console.log('Button clicked:', event.detail);
    if (event.detail.ctrlKey) {
      console.log('Ctrl+Click detected');
    }
  },
};

// TypeScript will catch invalid variants:
// const invalidButton: ButtonProps = {
//   variant: 'invalid', // Error: Type '"invalid"' is not assignable
// };

// =============================================================================
// Example 2: Type-safe form input
// =============================================================================

const emailInput: InputProps = {
  type: 'email',
  placeholder: 'Enter your email',
  disabled: false,
  autoFocus: true,
  onChange: (event: CustomEvent<InputChangeDetail>) => {
    const newValue = event.detail.value;
    console.log('Input changed:', newValue);
  },
};

// =============================================================================
// Example 3: Type-safe select with options
// =============================================================================

const regionOptions: SelectOption[] = [
  { value: 'us-east-1', label: 'US East (N. Virginia)' },
  { value: 'us-west-2', label: 'US West (Oregon)' },
  { value: 'eu-west-1', label: 'Europe (Ireland)', description: 'EU data residency' },
  { value: 'ap-southeast-1', label: 'Asia Pacific (Singapore)', disabled: true },
];

const regionSelect: SelectProps = {
  selectedOption: regionOptions[0],
  options: regionOptions,
  placeholder: 'Select a region',
  filteringType: 'auto',
  onChange: (event: CustomEvent<SelectChangeDetail>) => {
    const selected = event.detail.selectedOption;
    if (selected) {
      console.log('Selected region:', selected.value, selected.label);
    }
  },
};

// =============================================================================
// Example 4: Type-safe table with generics
// =============================================================================

interface Instance {
  id: string;
  name: string;
  type: string;
  status: 'running' | 'stopped' | 'terminated';
  launchTime: Date;
}

const instanceColumns: TableColumn<Instance>[] = [
  {
    id: 'id',
    header: 'Instance ID',
    cell: item => item.id,
    sortingField: 'id',
    width: 200,
  },
  {
    id: 'name',
    header: 'Name',
    cell: item => item.name,
    sortingField: 'name',
  },
  {
    id: 'type',
    header: 'Instance Type',
    cell: item => item.type,
    sortingField: 'type',
  },
  {
    id: 'status',
    header: 'Status',
    cell: item => item.status,
    sortingField: 'status',
  },
];

const instanceTable: TableProps<Instance> = {
  items: [], // Would be populated with Instance[]
  columnDefinitions: instanceColumns,
  selectionType: 'multi',
  selectedItems: [],
  stickyHeader: true,
  stripedRows: true,
  onSelectionChange: (event: CustomEvent<TableSelectionChangeDetail<Instance>>) => {
    const selected = event.detail.selectedItems;
    console.log(
      'Selected instances:',
      selected.map(i => i.id)
    );
  },
};

// =============================================================================
// Example 5: Type-safe modal with dismissal handling
// =============================================================================

const deleteConfirmationModal: ModalProps = {
  visible: true,
  size: 'medium',
  header: 'Delete Instance',
  closeAriaLabel: 'Close modal',
  onDismiss: (event: CustomEvent<ModalDismissDetail>) => {
    const reason = event.detail.reason;
    switch (reason) {
      case 'closeButton':
        console.log('Closed via X button');
        break;
      case 'overlay':
        console.log('Closed via overlay click');
        break;
      case 'keyboard':
        console.log('Closed via Escape key');
        break;
    }
  },
};

// =============================================================================
// Example 6: Type-safe alert configuration
// =============================================================================

const alertTypes: AlertType[] = ['info', 'success', 'warning', 'error'];

const alerts: AlertProps[] = alertTypes.map(type => ({
  type,
  visible: true,
  dismissible: true,
  header: `${type.charAt(0).toUpperCase() + type.slice(1)} Alert`,
  onDismiss: () => {
    console.log(`Dismissed ${type} alert`);
  },
}));

// =============================================================================
// Example 7: Type-safe status indicators
// =============================================================================

type StatusMapping = Record<string, StatusIndicatorType>;

const statusMapping: StatusMapping = {
  running: 'success',
  stopped: 'stopped',
  pending: 'pending',
  starting: 'in-progress',
  error: 'error',
  warning: 'warning',
};

function getStatusIndicatorProps(status: string): StatusIndicatorProps {
  return {
    type: statusMapping[status] || 'info',
    wrapText: false,
  };
}

// =============================================================================
// Example 8: Component factory pattern with type safety
// =============================================================================

interface ComponentConfig<T extends BaseComponentProps> {
  props: T;
  testId: string;
}

function createButtonConfig(
  variant: ButtonVariant,
  text: string,
  onClick?: EventHandler<ClickDetail>
): ComponentConfig<ButtonProps> {
  return {
    props: {
      variant,
      onClick,
      ariaLabel: text,
    },
    testId: `button-${variant}`,
  };
}

// Example configurations using the factory pattern
export const primaryConfig = createButtonConfig('primary', 'Submit', e => {
  console.log('Primary clicked', e.detail);
});

export const linkConfig = createButtonConfig('link', 'Learn more');

// =============================================================================
// Example 9: Type guards for component props
// =============================================================================

function hasClickHandler(props: ButtonProps): props is ButtonProps & { onClick: EventHandler<ClickDetail> } {
  return props.onClick !== undefined;
}

function handleButtonProps(props: ButtonProps): void {
  if (hasClickHandler(props)) {
    // TypeScript knows onClick is defined here
    console.log('Button has click handler');
  } else {
    console.log('Button has no click handler (likely navigation)');
  }
}

// =============================================================================
// Exports for testing
// =============================================================================

export {
  primaryButton,
  emailInput,
  regionSelect,
  instanceTable,
  deleteConfirmationModal,
  alerts,
  getStatusIndicatorProps,
  createButtonConfig,
  handleButtonProps,
};
