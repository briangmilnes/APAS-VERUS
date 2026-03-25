#!/bin/bash
# Compile PTT library then run proof time tests. ANSI stripped for emacs.
# Usage: ptt.sh [filter]  (e.g. ptt.sh Chap05)
# Filter is a case-insensitive substring match on test names.

set -uo pipefail

# Wait for 5GB free RAM before running. Poll every 5s, give up after 120s.
wait_for_ram() {
    local need_kb=$((9 * 1024 * 1024))
    local waited=0
    while true; do
        local avail_kb
        avail_kb=$(awk '/MemAvailable/ {print $2}' /proc/meminfo)
        if [ "$avail_kb" -ge "$need_kb" ]; then
            return 0
        fi
        if [ "$waited" -ge 120 ]; then
            echo "ERROR: only $((avail_kb / 1024))MB free after 120s (need 9GB). Aborting."
            exit 1
        fi
        echo "Waiting for RAM: $((avail_kb / 1024))MB free, need 9216MB (${waited}s elapsed)..."
        sleep 5
        waited=$((waited + 5))
    done
}

wait_for_ram

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERUS=~/projects/verus/source/target-verus/release/verus
RLIB_PATH="$PROJECT_ROOT/target/verus/libapas_verus.rlib"
VIR_PATH="$PROJECT_ROOT/target/verus/apas_verus.vir"

FILTER=()
if [ $# -gt 0 ]; then
    FILTER=(-E "test(/(?i)$1/)")
fi

mkdir -p "$PROJECT_ROOT/target/verus"

LOGDIR="$PROJECT_ROOT/logs"
mkdir -p "$LOGDIR"
LOGFILE="$LOGDIR/ptt.$(date +%Y%m%d-%H%M%S).log"

cd "$PROJECT_ROOT"
START_SEC=$(date +%s)
echo "Starting PTT at $(date '+%H:%M:%S')"
{
"$VERUS" \
    --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
    -o "$RLIB_PATH" \
    --export "$VIR_PATH" \
    2>&1 | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g'

cd "$PROJECT_ROOT/rust_verify_test"
cargo nextest run --release -j 6 --no-fail-fast --no-tests warn "${FILTER[@]}" 2>&1 \
    | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g'
} | tee "$LOGFILE"
RC=${PIPESTATUS[0]}
ELAPSED=$(( $(date +%s) - START_SEC ))
echo "Elapsed: ${ELAPSED}s" | tee -a "$LOGFILE"
exit $RC
