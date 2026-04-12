#!/bin/bash
# Run criterion benchmarks. Supports isolation by chapter.
# Usage:
#   bench.sh                    — run all benchmarks
#   bench.sh isolate ChapNN     — run benchmarks for ChapNN only
#   bench.sh ChapNN             — same as isolate ChapNN
#   bench.sh --list             — list all registered bench names

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

LOGDIR="$PROJECT_ROOT/logs"
mkdir -p "$LOGDIR"
AGENT_TAG=""
if [[ "$PROJECT_ROOT" =~ -agent([0-9]+)$ ]]; then
    AGENT_TAG=".agent${BASH_REMATCH[1]}"
fi
LOGFILE="$LOGDIR/bench${AGENT_TAG}.$(date +%Y%m%d-%H%M%S).log"

MODE="${1:-all}"
shift 2>/dev/null || true

# Resolve which bench files belong to a given chapter.
# Bench files live in benches/ChapNN/Bench*.rs; we collect all for the chapter.
bench_names_for_chapter() {
    local chap="$1"
    local dir="$PROJECT_ROOT/benches/${chap}"
    if [ ! -d "$dir" ]; then
        echo ""
        return
    fi
    for f in "$dir"/Bench*.rs; do
        [ -f "$f" ] || continue
        basename "$f" .rs
    done
}

case "$MODE" in
    --list)
        grep '^name' "$PROJECT_ROOT/Cargo.toml" | grep -i bench | awk -F'"' '{print $2}' | sort
        exit 0
        ;;
    isolate|Chap*)
        # Accept: bench.sh isolate ChapNN  OR  bench.sh ChapNN
        if [ "$MODE" = "isolate" ]; then
            CHAP="${1:-}"
            [ -z "$CHAP" ] && { echo "Usage: bench.sh isolate ChapNN"; exit 1; }
        else
            CHAP="$MODE"
        fi
        BENCH_NAMES=$(bench_names_for_chapter "$CHAP")
        if [ -z "$BENCH_NAMES" ]; then
            echo "No benchmarks found for $CHAP (looked in benches/$CHAP/)" >&2
            exit 1
        fi
        BENCH_FLAGS=()
        for name in $BENCH_NAMES; do
            BENCH_FLAGS+=(--bench "$name")
        done
        echo "Isolate $CHAP: running ${BENCH_FLAGS[*]}"
        echo "Starting bench at $(date '+%H:%M:%S')"
        START_SEC=$(date +%s)
        cargo bench "${BENCH_FLAGS[@]}" 2>&1 | tee "$LOGFILE"
        RC=${PIPESTATUS[0]}
        ;;
    all)
        echo "Starting bench (all) at $(date '+%H:%M:%S')"
        START_SEC=$(date +%s)
        cargo bench 2>&1 | tee "$LOGFILE"
        RC=${PIPESTATUS[0]}
        ;;
    *)
        echo "Unknown mode: $MODE" >&2
        echo "Usage: bench.sh [all|isolate ChapNN|ChapNN|--list]" >&2
        exit 1
        ;;
esac

ELAPSED=$(( $(date +%s) - START_SEC ))
echo "Elapsed: ${ELAPSED}s" | tee -a "$LOGFILE"
exit $RC
