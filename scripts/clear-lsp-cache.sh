#!/bin/bash
# Clear LSP and Verus analyzer cache

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Clearing LSP cache for APAS-VERUS..."

# Clear Verus log directory
if [ -d "$PROJECT_ROOT/.verus-log" ]; then
    rm -rf "$PROJECT_ROOT/.verus-log"
    echo "✓ Cleared .verus-log"
fi

# Clear rust-analyzer cache in target
if [ -d "$PROJECT_ROOT/target" ]; then
    find "$PROJECT_ROOT/target" -name ".fingerprint" -type d -exec rm -rf {} + 2>/dev/null
    echo "✓ Cleared target/.fingerprint caches"
fi

# Clear any rust-analyzer state files
if [ -f "$PROJECT_ROOT/.rust-analyzer" ]; then
    rm -rf "$PROJECT_ROOT/.rust-analyzer"
    echo "✓ Cleared .rust-analyzer"
fi

echo "LSP cache cleared successfully!"

