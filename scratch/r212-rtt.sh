#!/bin/bash
# r212: run one chapter's run-time tests with `cargo test --release` (the
# 1.98.1 toolchain has no cargo-nextest, so scripts/rtt.sh cannot run) on
# every [[test]] of Cargo.toml whose path is tests/ChapNN/. ANSI stripped,
# logged to logs/rtt.<timestamp>.log. Usage: scratch/r212-rtt.sh ChapNN
set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHAP="${1:?usage: r212-rtt.sh ChapNN}"
mkdir -p "$PROJECT_ROOT/logs"
LOGFILE="$PROJECT_ROOT/logs/rtt.$(date +%Y%m%d-%H%M%S).log"

mapfile -t TESTS < <(awk -v chap="$CHAP" '
    /^\[\[test\]\]/ { name=""; next }
    /^name = / { gsub(/name = |"/, ""); name=$0; next }
    /^path = / { gsub(/path = |"/, ""); if (index($0, "tests/" chap "/") == 1 && name != "") print name }
' "$PROJECT_ROOT/Cargo.toml")

cd "$PROJECT_ROOT"
START_SEC=$(date +%s)
{
    echo "r212 RTT for $CHAP: cargo test --release on ${#TESTS[@]} targets: ${TESTS[*]}"
    echo "Starting RTT at $(date '+%H:%M:%S')"
    ARGS=()
    for t in "${TESTS[@]}"; do ARGS+=(--test "$t"); done
    cargo test --release --no-fail-fast "${ARGS[@]}" 2>&1
    echo "cargo test rc: $?"
} 2>&1 | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g' | tee "$LOGFILE"
echo "Elapsed: $(( $(date +%s) - START_SEC ))s" | tee -a "$LOGFILE"
echo "Log: $LOGFILE"
