#!/bin/bash
# WASM Optimization Script
# Applies wasm-opt optimizations to reduce bundle size further

set -e

WASM_FILE="demo/dist/cloudscape-demo-12da8de75993b5b2_bg.wasm"
BACKUP_FILE="${WASM_FILE}.backup"

echo "🔧 WASM Optimization Script"
echo "============================"
echo ""

# Check if wasm-opt is available
if ! command -v wasm-opt &> /dev/null; then
    echo "⚠️  wasm-opt not found. Installing via Homebrew..."
    echo ""
    echo "Run: brew install binaryen"
    echo ""
    echo "After installation, run this script again."
    exit 1
fi

# Check if WASM file exists
if [ ! -f "$WASM_FILE" ]; then
    echo "❌ WASM file not found: $WASM_FILE"
    echo "Build the project first with: cd demo && trunk build --release"
    exit 1
fi

# Get original size
ORIGINAL_SIZE=$(stat -f%z "$WASM_FILE")
ORIGINAL_SIZE_MB=$(echo "scale=2; $ORIGINAL_SIZE / 1048576" | bc)

echo "📦 Original WASM size: ${ORIGINAL_SIZE_MB}MB"
echo ""

# Backup original file
cp "$WASM_FILE" "$BACKUP_FILE"
echo "✓ Created backup: ${BACKUP_FILE}"

# Apply wasm-opt optimizations
echo "🔧 Applying wasm-opt optimizations..."
wasm-opt -Oz --enable-bulk-memory "$BACKUP_FILE" -o "$WASM_FILE"

# Get optimized size
OPTIMIZED_SIZE=$(stat -f%z "$WASM_FILE")
OPTIMIZED_SIZE_MB=$(echo "scale=2; $OPTIMIZED_SIZE / 1048576" | bc)
REDUCTION=$(echo "scale=1; ($ORIGINAL_SIZE - $OPTIMIZED_SIZE) * 100 / $ORIGINAL_SIZE" | bc)

echo ""
echo "✅ Optimization complete!"
echo ""
echo "Original:  ${ORIGINAL_SIZE_MB}MB"
echo "Optimized: ${OPTIMIZED_SIZE_MB}MB"
echo "Savings:   ${REDUCTION}%"
echo ""
echo "Backup saved to: ${BACKUP_FILE}"
echo "(You can delete the backup if the optimized version works correctly)"
