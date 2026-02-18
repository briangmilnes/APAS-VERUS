#!/bin/bash
# Run veracity-review-proof-holes.
# Usage: holes.sh [dir-or-file]  (defaults to src/)
# Detects whether argument is a file or directory automatically.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERACITY=~/projects/veracity/target/release/veracity-review-proof-holes

TARGET="${1:-src/}"

cd "$PROJECT_ROOT"
if [ -f "$TARGET" ]; then
    "$VERACITY" "$TARGET"
elif [ -d "$TARGET" ]; then
    "$VERACITY" -d "$TARGET"
else
    echo "Not found: $TARGET"
    exit 1
fi
