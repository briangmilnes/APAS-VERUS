#!/bin/bash
# Verify the path/ parallel build. Run from fixture root.
# Usage: scripts/validate-path.sh [full|dev_only|exp] [--time]

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERUS=~/projects/verus/source/target-verus/release/verus

MODE="${1:-full}"
shift 2>/dev/null || true

USE_TIME=false
for arg in "$@"; do
    if [ "$arg" = "--time" ]; then USE_TIME=true; fi
done

case "$MODE" in
    full)     CFG_FLAG=() ;;
    dev_only) CFG_FLAG=(--cfg 'feature="dev_only"') ;;
    exp)      CFG_FLAG=(--cfg 'feature="experiments_only"') ;;
    *)        echo "Usage: validate-path.sh [full|dev_only|exp] [--time]"; exit 1 ;;
esac

TIME_FLAG=()
if $USE_TIME; then
    TIME_FLAG=(--time)
fi

cd "$PROJECT_ROOT"
if [ ! -f path/lib.rs ]; then
    echo "path/lib.rs not found. Run scripts/regenerate-path.sh or copy _path files to path/"
    exit 1
fi
echo "Verifying path/ (parallel build)"
time timeout 120 "$VERUS" --crate-type=lib path/lib.rs --multiple-errors 20 --expand-errors \
    "${CFG_FLAG[@]}" "${TIME_FLAG[@]}" 2>&1 | sed 's/\x1b\[[0-9;]*m//g'
