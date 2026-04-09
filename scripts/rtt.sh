#!/bin/bash
# Runtime tests (RTTs) with -j 6 and 120s timeout. ANSI stripped for emacs.
# Usage: rtt.sh [filter]  (e.g. rtt.sh bst, rtt.sh Chap37)
# Filter is a case-insensitive substring match on test names.

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

FILTER=()
if [ $# -gt 0 ]; then
    FILTER=(-E "test(/(?i)$1/)")
fi

LOGDIR="$PROJECT_ROOT/logs"
mkdir -p "$LOGDIR"
# Detect agent name from worktree path (e.g., APAS-VERUS-agent3 → agent3).
AGENT_TAG=""
if [[ "$PROJECT_ROOT" =~ -agent([0-9]+)$ ]]; then
    AGENT_TAG=".agent${BASH_REMATCH[1]}"
fi
LOGFILE="$LOGDIR/rtt${AGENT_TAG}.$(date +%Y%m%d-%H%M%S).log"

START_SEC=$(date +%s)
echo "Starting RTT at $(date '+%H:%M:%S')"
timeout 120 cargo nextest run --release -j 6 --no-fail-fast --no-tests warn "${FILTER[@]}" 2>&1 \
    | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g' | tee "$LOGFILE"
RC=${PIPESTATUS[0]}
ELAPSED=$(( $(date +%s) - START_SEC ))
echo "Elapsed: ${ELAPSED}s" | tee -a "$LOGFILE"
exit $RC
