#!/bin/bash
# Compile PTT library then run proof time tests. ANSI stripped for emacs.
# Usage: ptt.sh [filter]  (e.g. ptt.sh Chap05)
# Filter is a case-insensitive substring match on test names.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERUS=~/projects/verus/source/target-verus/release/verus
RLIB_PATH="$PROJECT_ROOT/target/verus/libapas_verus.rlib"
VIR_PATH="$PROJECT_ROOT/target/verus/apas_verus.vir"

FILTER=()
if [ $# -gt 0 ]; then
    FILTER=(-E "test(/(?i)$1/)")
fi

mkdir -p "$PROJECT_ROOT/target/verus"

cd "$PROJECT_ROOT"
"$VERUS" \
    --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
    -o "$RLIB_PATH" \
    --export "$VIR_PATH" \
    2>&1 | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g'

cd "$PROJECT_ROOT/rust_verify_test"
cargo nextest run -j 6 --no-fail-fast --no-tests warn "${FILTER[@]}" 2>&1 \
    | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g'
