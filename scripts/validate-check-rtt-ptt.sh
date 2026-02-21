#!/bin/bash
# Run full validation pipeline: validate, RTT, PTT. Stops on first failure.
# Usage: validate-check-rtt-ptt.sh [full|dev|exp] [--time] [filter]
#   mode defaults to full, filter is optional test name substring for RTT/PTT.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

MODE="${1:-full}"
shift 2>/dev/null || true

VALIDATE_ARGS=("$MODE")
FILTER=""

for arg in "$@"; do
    if [ "$arg" = "--time" ]; then
        VALIDATE_ARGS+=("--time")
    else
        FILTER="$arg"
    fi
done

RTT_ARGS=()
PTT_ARGS=()
if [ -n "$FILTER" ]; then
    RTT_ARGS=("$FILTER")
    PTT_ARGS=("$FILTER")
fi

"$SCRIPT_DIR/validate.sh" "${VALIDATE_ARGS[@]}"
"$SCRIPT_DIR/rtt.sh" "${RTT_ARGS[@]}"
"$SCRIPT_DIR/ptt.sh" "${PTT_ARGS[@]}"
