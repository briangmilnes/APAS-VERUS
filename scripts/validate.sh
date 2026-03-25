#!/bin/bash
# Verus verification. Usage: validate.sh [full|dev_only|exp] [--time]

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
    wf)       CFG_FLAG=(--cfg 'feature="wf"') ;;
    *)        echo "Usage: validate.sh [full|dev_only|exp|wf] [--time]"; exit 1 ;;
esac

TIME_FLAG=()
if $USE_TIME; then
    TIME_FLAG=(--time)
fi

LOGDIR="$PROJECT_ROOT/logs"
mkdir -p "$LOGDIR"
LOGFILE="$LOGDIR/validate.$(date +%Y%m%d-%H%M%S).log"

cd "$PROJECT_ROOT"
START_SEC=$(date +%s)
echo "Starting verification at $(date '+%H:%M:%S')"
# Limit parallelism to 8 threads (default is num_cpus-1, can lock machine)
timeout 300 "$VERUS" --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors \
    --num-threads 8 \
    "${CFG_FLAG[@]}" "${TIME_FLAG[@]}" 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | tee "$LOGFILE"
RC=${PIPESTATUS[0]}
ELAPSED=$(( $(date +%s) - START_SEC ))
echo "Elapsed: ${ELAPSED}s" | tee -a "$LOGFILE"
exit $RC
