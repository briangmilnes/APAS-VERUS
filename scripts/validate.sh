#!/bin/bash
# Verus verification. Usage: validate.sh [full|dev_only|exp|union_find] [--time] [--profile] [--profile-all]

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
USE_PROFILE=false
USE_PROFILE_ALL=false
EXTRA_FEATURES=()
for arg in "$@"; do
    if [ "$arg" = "--time" ]; then USE_TIME=true;
    elif [ "$arg" = "--profile-all" ]; then USE_PROFILE_ALL=true;
    elif [ "$arg" = "--profile" ]; then USE_PROFILE=true;
    elif [[ "$arg" == Chap* ]]; then EXTRA_FEATURES+=("$arg");
    fi
done

# Resolve Cargo feature dependencies for isolate mode.
# Reads [features] from Cargo.toml and computes transitive closure.
resolve_deps() {
    local -A deps resolved
    local key vals
    while IFS='=' read -r key vals; do
        key=$(echo "$key" | tr -d ' "')
        vals=$(echo "$vals" | tr -d '[]"' | tr ',' ' ')
        deps[$key]="$vals"
    done < <(sed -n '/^\[features\]/,/^\[/p' "$PROJECT_ROOT/Cargo.toml" | grep '^Chap')

    local queue=("$@")
    while [ ${#queue[@]} -gt 0 ]; do
        local cur="${queue[0]}"
        queue=("${queue[@]:1}")
        [ -n "${resolved[$cur]:-}" ] && continue
        resolved[$cur]=1
        for dep in ${deps[$cur]:-}; do
            [ -z "${resolved[$dep]:-}" ] && queue+=("$dep")
        done
    done
    echo "${!resolved[@]}"
}

case "$MODE" in
    full)         CFG_FLAG=() ;;
    dev_only)     CFG_FLAG=(--cfg 'feature="dev_only"') ;;
    exp)          CFG_FLAG=(--cfg 'feature="experiments_only"') ;;
    wf)           CFG_FLAG=(--cfg 'feature="wf"') ;;
    isolate)
        ALL_CHAPS=$(resolve_deps "${EXTRA_FEATURES[@]}")
        CFG_FLAG=(--cfg 'feature="isolate"')
        for chap in $ALL_CHAPS; do
            CFG_FLAG+=(--cfg "feature=\"$chap\"")
        done
        echo "Isolate: including $ALL_CHAPS"
        ;;
    *)
        echo "WARNING: unknown mode '$MODE' — treating as Cargo feature name"
        CFG_FLAG=(--cfg "feature=\"$MODE\"")
        ;;
esac

TIME_FLAG=()
if $USE_TIME; then
    TIME_FLAG=(--time)
fi

PROFILE_FLAG=()
if $USE_PROFILE_ALL; then
    PROFILE_FLAG=(--profile-all)
elif $USE_PROFILE; then
    PROFILE_FLAG=(--profile)
fi

LOGDIR="$PROJECT_ROOT/logs"
mkdir -p "$LOGDIR"
LOGFILE="$LOGDIR/validate.$(date +%Y%m%d-%H%M%S).log"

cd "$PROJECT_ROOT"
START_SEC=$(date +%s)
echo "Starting verification at $(date '+%H:%M:%S')"

# Memory monitor: sample peak RSS of rust_verify and z3 children every 2s.
# Writes peaks to a temp file so the parent can read them after verus exits.
MEM_STATS=$(mktemp)
echo "0 0 $(awk '/MemAvailable/ {print $2}' /proc/meminfo) 0 0" > "$MEM_STATS"
CLK_TCK=$(getconf CLK_TCK)
(
    peak_rv=0; peak_z3=0; rv_cpu=0; z3_cpu=0
    min_free=$(awk '/MemAvailable/ {print $2}' /proc/meminfo)
    while true; do
        sleep 2
        rv_kb=0; z3_kb=0; rv_ticks=0; z3_ticks=0
        for pid in $(pgrep -f rust_verify 2>/dev/null); do
            rss=$(awk '/^VmRSS:/ {print $2}' /proc/$pid/status 2>/dev/null || echo 0)
            rv_kb=$((rv_kb + rss))
            # fields 14+15 = own utime+stime; 16+17 = children's cumulative utime+stime
            ticks=$(awk '{print $14 + $15}' /proc/$pid/stat 2>/dev/null || echo 0)
            rv_ticks=$((rv_ticks + ticks))
            child_ticks=$(awk '{print $16 + $17}' /proc/$pid/stat 2>/dev/null || echo 0)
            z3_ticks=$((z3_ticks + child_ticks))
        done
        for pid in $(pgrep -f 'z3 -smt2' 2>/dev/null); do
            rss=$(awk '/^VmRSS:/ {print $2}' /proc/$pid/status 2>/dev/null || echo 0)
            z3_kb=$((z3_kb + rss))
        done
        free_kb=$(awk '/MemAvailable/ {print $2}' /proc/meminfo)
        [ "$rv_kb" -gt "$peak_rv" ] && peak_rv=$rv_kb
        [ "$z3_kb" -gt "$peak_z3" ] && peak_z3=$z3_kb
        [ "$free_kb" -lt "$min_free" ] && min_free=$free_kb
        [ "$rv_ticks" -gt "$rv_cpu" ] && rv_cpu=$rv_ticks
        [ "$z3_ticks" -gt "$z3_cpu" ] && z3_cpu=$z3_ticks
        echo "$peak_rv $peak_z3 $min_free $rv_cpu $z3_cpu" > "$MEM_STATS"
    done
) &
MONITOR_PID=$!

# Limit parallelism to 8 threads (default is num_cpus-1, can lock machine)
# Extra Verus flags via VERUS_EXTRA_ARGS env var (e.g. VERUS_EXTRA_ARGS="-V new-mut-ref")
VERUS_EXTRA_ARGS="${VERUS_EXTRA_ARGS:-}"
timeout 300 "$VERUS" --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors \
    --num-threads 8 \
    "${CFG_FLAG[@]}" "${TIME_FLAG[@]}" "${PROFILE_FLAG[@]}" $VERUS_EXTRA_ARGS 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | tee "$LOGFILE"
RC=${PIPESTATUS[0]}

kill "$MONITOR_PID" 2>/dev/null; wait "$MONITOR_PID" 2>/dev/null

read PEAK_RV_KB PEAK_Z3_KB MIN_FREE_KB RV_TICKS Z3_TICKS < "$MEM_STATS"
rm -f "$MEM_STATS"

ELAPSED=$(( $(date +%s) - START_SEC ))
RV_CPU=$((RV_TICKS / CLK_TCK))
Z3_CPU=$((Z3_TICKS / CLK_TCK))
echo "Elapsed: ${ELAPSED}s" | tee -a "$LOGFILE"
echo "Sampled Memory Usage: peak rust_verify RSS: $((PEAK_RV_KB / 1024))MB, peak z3 RSS: $((PEAK_Z3_KB / 1024))MB, min free: $((MIN_FREE_KB / 1024))MB" | tee -a "$LOGFILE"
echo "Sampled CPU Usage: rust_verify: ${RV_CPU}s, z3 children: ${Z3_CPU}s" | tee -a "$LOGFILE"
exit $RC
