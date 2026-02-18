#!/bin/bash
# Runtime tests (RTTs) with -j 6 and 120s timeout. ANSI stripped for emacs.
# Usage: rtt.sh [filter]  (e.g. rtt.sh bst, rtt.sh Chap37)
# Filter is a case-insensitive substring match on test names.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

FILTER=()
if [ $# -gt 0 ]; then
    FILTER=(-E "test(/(?i)$1/)")
fi

timeout 120 cargo nextest run -j 6 --no-fail-fast --no-tests warn "${FILTER[@]}" 2>&1 \
    | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g'
