#!/bin/bash
# sync-upstream.sh
# Syncs changes from the upstream cloudscape-design/components repository
# and identifies React changes that may need Rust/Yew updates

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
UPSTREAM_REMOTE="upstream"
UPSTREAM_BRANCH="main"
REACT_SRC_DIR="src"
RUST_COMPONENTS_DIR="rust-components/crates/components/src"

# Output file for the analysis report
REPORT_FILE="sync-report-$(date +%Y%m%d-%H%M%S).md"

echo -e "${BLUE}=== Cloudscape Upstream Sync & Analysis ===${NC}"
echo ""

# Function to check if upstream remote exists
check_upstream_remote() {
    if ! git remote get-url "$UPSTREAM_REMOTE" > /dev/null 2>&1; then
        echo -e "${YELLOW}Upstream remote not found. Adding it...${NC}"
        git remote add "$UPSTREAM_REMOTE" "https://github.com/cloudscape-design/components.git"
        echo -e "${GREEN}Added upstream remote${NC}"
    else
        echo -e "${GREEN}Upstream remote exists: $(git remote get-url $UPSTREAM_REMOTE)${NC}"
    fi
}

# Function to fetch upstream changes
fetch_upstream() {
    echo ""
    echo -e "${BLUE}Fetching upstream changes...${NC}"
    git fetch "$UPSTREAM_REMOTE" "$UPSTREAM_BRANCH"
    echo -e "${GREEN}Fetch complete${NC}"
}

# Function to get the list of implemented Yew components
get_implemented_components() {
    find "$RUST_COMPONENTS_DIR" -maxdepth 1 -name "*.rs" -type f | \
        xargs basename -a 2>/dev/null | \
        sed 's/\.rs$//' | \
        grep -v "^lib$" | \
        grep -v "^internal$" | \
        grep -v "^mod$" | \
        sort
}

# Function to map Rust component names to React component directories
map_component_to_react() {
    local rust_name="$1"
    case "$rust_name" in
        "box_component") echo "box" ;;
        "app_layout") echo "app-layout" ;;
        "content_layout") echo "content-layout" ;;
        "radio_group") echo "radio-group" ;;
        "status_indicator") echo "status-indicator" ;;
        "progress_bar") echo "progress-bar" ;;
        "form_field") echo "form-field" ;;
        "key_value_pairs") echo "key-value-pairs" ;;
        "expandable_section") echo "expandable-section" ;;
        "column_layout") echo "column-layout" ;;
        "space_between") echo "space-between" ;;
        "side_navigation") echo "side-navigation" ;;
        "top_navigation") echo "top-navigation" ;;
        "date_picker") echo "date-picker" ;;
        "date_range_picker") echo "date-range-picker" ;;
        "file_upload") echo "file-upload" ;;
        "token_group") echo "token-group" ;;
        "button_dropdown") echo "button-dropdown" ;;
        "copy_to_clipboard") echo "copy-to-clipboard" ;;
        *) echo "$rust_name" ;;
    esac
}

# Function to categorize the type of change
categorize_change() {
    local file="$1"
    local diff_content=$(git diff HEAD..."$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" -- "$file" 2>/dev/null || echo "")
    local categories=""

    if echo "$diff_content" | grep -qE "(interface.*Props|Props\s*=|export.*Props)"; then
        categories="${categories}props,"
    fi
    if echo "$diff_content" | grep -qE "^(\+.*export)"; then
        categories="${categories}exports,"
    fi
    if echo "$diff_content" | grep -qE "(onClick|onChange|onBlur|onFocus|onSubmit|onCancel|CancelableEventHandler)"; then
        categories="${categories}events,"
    fi
    if echo "$diff_content" | grep -qE "(className|styles\.|\.css|clsx)"; then
        categories="${categories}styles,"
    fi
    if echo "$diff_content" | grep -qE "(aria-|role=|tabIndex|a11y)"; then
        categories="${categories}a11y,"
    fi
    if echo "$diff_content" | grep -qE "(i18n|I18nStrings|formatMessage)"; then
        categories="${categories}i18n,"
    fi

    echo "${categories%,}"
}

# Function to generate detailed analysis for a component
generate_component_analysis() {
    local rust_component="$1"
    local react_component=$(map_component_to_react "$rust_component")
    local react_dir="$REACT_SRC_DIR/$react_component"

    if [ ! -d "$react_dir" ]; then
        return
    fi

    local upstream_changes=$(git diff --name-only HEAD..."$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" -- "$react_dir" 2>/dev/null | head -20)

    if [ -z "$upstream_changes" ]; then
        return
    fi

    echo ""
    echo "### $react_component (Rust: $rust_component)"
    echo ""
    echo "**Changed files:**"

    echo "$upstream_changes" | while read -r file; do
        if [ -n "$file" ]; then
            local categories=$(categorize_change "$file")
            local stat=$(git diff --stat HEAD..."$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" -- "$file" 2>/dev/null | tail -1 | sed 's/.*|/|/')
            echo "- \`$(basename "$file")\` $stat"
            if [ -n "$categories" ]; then
                echo "  - Categories: $categories"
            fi
        fi
    done

    echo ""
    echo "**Recommended actions:**"

    local full_diff=$(git diff HEAD..."$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" -- "$react_dir" 2>/dev/null || echo "")

    if echo "$full_diff" | grep -qE "interface.*Props"; then
        echo "- [ ] Review prop changes and update Rust \`${rust_component}Props\` struct"
    fi
    if echo "$full_diff" | grep -qE "(onClick|onChange|onBlur|onFocus)"; then
        echo "- [ ] Check event handler signatures in Rust component"
    fi
    if echo "$full_diff" | grep -qE "(className|styles\.)"; then
        echo "- [ ] Verify CSS class names match in Rust ClassBuilder usage"
    fi
    if echo "$full_diff" | grep -qE "aria-"; then
        echo "- [ ] Update ARIA attributes in Rust component"
    fi
}

# Main analysis function
run_analysis() {
    echo ""
    echo -e "${BLUE}Analyzing changes for implemented Rust/Yew components...${NC}"
    echo ""

    cat > "$REPORT_FILE" << 'HEADER'
# Upstream Sync Analysis Report

**Upstream:** cloudscape-design/components
**Branch:** main

## Summary

This report identifies React component changes in the upstream repository
that may require corresponding updates to the Rust/Yew implementations.

---

HEADER

    local implemented_components=$(get_implemented_components)
    local components_with_changes=""
    local total_components=0
    local changed_components=0

    echo "## Components Analysis" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"

    for rust_component in $implemented_components; do
        total_components=$((total_components + 1))
        local react_component=$(map_component_to_react "$rust_component")
        local react_dir="$REACT_SRC_DIR/$react_component"

        if [ -d "$react_dir" ]; then
            local upstream_changes=$(git diff --name-only HEAD..."$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" -- "$react_dir" 2>/dev/null || echo "")

            if [ -n "$upstream_changes" ]; then
                changed_components=$((changed_components + 1))
                components_with_changes="${components_with_changes}${rust_component}\n"
                echo -e "${YELLOW}⚠ ${react_component}${NC} has upstream changes"
                generate_component_analysis "$rust_component" >> "$REPORT_FILE"
            else
                echo -e "${GREEN}✓ ${react_component}${NC} is up to date"
            fi
        else
            echo -e "${BLUE}? ${rust_component}${NC} (React dir not found: $react_dir)"
        fi
    done

    echo ""
    echo "---" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "## Statistics" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "- **Total Rust components:** $total_components" >> "$REPORT_FILE"
    echo "- **Components with upstream changes:** $changed_components" >> "$REPORT_FILE"
    echo "- **Components up to date:** $((total_components - changed_components))" >> "$REPORT_FILE"

    echo ""
    echo -e "${GREEN}Analysis complete!${NC}"
    echo -e "Report saved to: ${BLUE}$REPORT_FILE${NC}"
}

# Function to show what would be merged
preview_merge() {
    echo ""
    echo -e "${BLUE}Preview of upstream changes:${NC}"
    echo ""

    local commit_count=$(git rev-list --count HEAD.."$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" 2>/dev/null || echo "0")
    echo -e "Commits behind upstream: ${YELLOW}$commit_count${NC}"
    echo ""

    if [ "$commit_count" -gt 0 ]; then
        echo "Recent upstream commits:"
        git log --oneline HEAD.."$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" | head -10
        if [ "$commit_count" -gt 10 ]; then
            echo "... and $((commit_count - 10)) more commits"
        fi
    fi
}

# Function to perform the merge
perform_merge() {
    echo ""
    echo -e "${YELLOW}Merging upstream changes...${NC}"

    local backup_branch="backup-before-sync-$(date +%Y%m%d-%H%M%S)"
    git branch "$backup_branch"
    echo -e "Created backup branch: ${GREEN}$backup_branch${NC}"

    if git merge "$UPSTREAM_REMOTE/$UPSTREAM_BRANCH" --no-edit; then
        echo -e "${GREEN}Merge successful!${NC}"
    else
        echo -e "${RED}Merge conflicts detected!${NC}"
        echo "Please resolve conflicts manually and then run:"
        echo "  git add ."
        echo "  git commit"
        echo ""
        echo "To abort the merge, run:"
        echo "  git merge --abort"
        return 1
    fi
}

# Parse command line arguments
ACTION="${1:-analyze}"

case "$ACTION" in
    "fetch")
        check_upstream_remote
        fetch_upstream
        ;;
    "analyze")
        check_upstream_remote
        fetch_upstream
        run_analysis
        ;;
    "preview")
        check_upstream_remote
        fetch_upstream
        preview_merge
        ;;
    "merge")
        check_upstream_remote
        fetch_upstream
        preview_merge
        read -p "Proceed with merge? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            perform_merge
            run_analysis
        fi
        ;;
    "full")
        check_upstream_remote
        fetch_upstream
        preview_merge
        read -p "Proceed with merge? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            perform_merge
        fi
        run_analysis
        ;;
    "help"|"--help"|"-h")
        echo "Usage: $0 [action]"
        echo ""
        echo "Actions:"
        echo "  fetch    - Fetch upstream changes only"
        echo "  analyze  - Fetch and analyze which Rust components need updates (default)"
        echo "  preview  - Show what would be merged"
        echo "  merge    - Merge upstream changes with confirmation"
        echo "  full     - Merge then analyze"
        echo "  help     - Show this help"
        ;;
    *)
        echo -e "${RED}Unknown action: $ACTION${NC}"
        echo "Run '$0 help' for usage"
        exit 1
        ;;
esac
