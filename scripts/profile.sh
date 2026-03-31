#!/bin/bash
# Profile Z3 instantiation counts.
# Usage: scripts/profile.sh [isolate ChapNN]
#        scripts/profile.sh                    # full crate profile
#        scripts/profile.sh isolate Chap65     # isolate + profile for Chap65
#        scripts/profile.sh summary <dir>      # summarize existing profile data
#
# Runs Verus with -V capture-profiles, compresses .profile and .graph files
# into logs/profile/<label>/<timestamp>/, then prints instantiation summary.

set -uo pipefail

# Acquire one of N exclusive slots (default 2) so concurrent agents don't OOM.
source "$(dirname "${BASH_SOURCE[0]}")/verus-lock.sh"

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERUS=~/projects/verus/source/target-verus/release/verus

# --- Summary function ---

readfile() {
    case "$1" in
        *.gz) zcat "$1" ;;
        *)    cat "$1" ;;
    esac
}

dispname() {
    local b
    b=$(basename "$1")
    b="${b%.gz}"
    b="${b%.profile}"
    echo "$b"
}

run_summary() {
    local TARGET="$1"
    local FILES=()

    if [ -d "$TARGET" ]; then
        FILES=("$TARGET"/*.profile)
        if [ ! -f "${FILES[0]:-}" ]; then
            FILES=("$TARGET"/*.profile.gz)
        fi
    else
        FILES=("$TARGET")
    fi

    echo "Units: quantifier instantiation counts (number of times Z3 fired each quantifier)."
    echo ""

    for pf in "${FILES[@]}"; do
        [ -f "$pf" ] || continue
        local fname
        fname=$(dispname "$pf")

        local result
        result=$(readfile "$pf" | awk '
        /^\[mk-quant\]/ { qname[$2] = $3 }
        /^\[new-match\]/ { qid = $3; count[qid]++; total++ }
        END {
            printf "TOTAL %d\n", total
            for (qid in count) {
                name = (qid in qname) ? qname[qid] : qid
                printf "%8d  %s  (%s)\n", count[qid], name, qid
            }
        }
        ' | sort -rn)

        local total
        total=$(echo "$result" | head -1 | awk '{print $2}')
        echo ""
        echo "=== $fname: $total instantiations ==="
        echo "$result" | tail -n +2 | head -30
    done

    echo ""
    echo "=== Per-module totals ==="
    for pf in "${FILES[@]}"; do
        [ -f "$pf" ] || continue
        local fname total
        fname=$(dispname "$pf")
        total=$(readfile "$pf" | awk '/^\[new-match\]/ { n++ } END { printf "%d", n+0 }')
        printf "%8d  %s\n" "$total" "$fname"
    done | sort -rn

    echo ""
    echo "=== Grand top quantifiers (top 40) ==="
    for pf in "${FILES[@]}"; do
        [ -f "$pf" ] || continue
        readfile "$pf" | awk '
        /^\[mk-quant\]/ { qname[$2] = $3 }
        /^\[new-match\]/ { qid = $3; count[qid]++ }
        END {
            for (qid in count) {
                name = (qid in qname) ? qname[qid] : qid
                printf "%8d  %s\n", count[qid], name
            }
        }
        '
    done | awk '{ counts[$2] += $1 } END { for (n in counts) printf "%8d  %s\n", counts[n], n }' \
        | sort -rn | head -40
}

# --- Main ---

MODE="${1:-full}"
shift 2>/dev/null || true

# Summary mode: just parse existing data.
if [ "$MODE" = "summary" ]; then
    if [ $# -lt 1 ]; then
        echo "Usage: $0 summary <profile-dir-or-file>"
        exit 1
    fi
    run_summary "$1"
    exit 0
fi

# Resolve Cargo feature dependencies for isolate mode (same as validate.sh).
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

EXTRA_FEATURES=()
for arg in "$@"; do
    if [[ "$arg" == Chap* ]]; then EXTRA_FEATURES+=("$arg"); fi
done

CFG_FLAGS=()
LABEL="full"

case "$MODE" in
    full)
        LABEL="full"
        ;;
    isolate)
        if [ ${#EXTRA_FEATURES[@]} -eq 0 ]; then
            echo "Usage: $0 isolate ChapNN"
            exit 1
        fi
        ALL_CHAPS=$(resolve_deps "${EXTRA_FEATURES[@]}")
        CFG_FLAGS=(--cfg 'feature="isolate"')
        for chap in $ALL_CHAPS; do
            CFG_FLAGS+=(--cfg "feature=\"$chap\"")
        done
        LABEL="${EXTRA_FEATURES[0]}"
        echo "Isolate: including $ALL_CHAPS"
        ;;
    *)
        echo "Usage: $0 [isolate ChapNN | summary <dir>]"
        exit 1
        ;;
esac

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
PROFILE_OUT="$PROJECT_ROOT/logs/profile"
SUMMARY_FILE="$PROFILE_OUT/SUMMARY-$LABEL-$TIMESTAMP.txt"

echo "=== Profiling $LABEL (capture-profiles) ==="

# Clean previous solver log.
rm -rf "$PROJECT_ROOT/.verus-solver-log"

cd "$PROJECT_ROOT"
START_SEC=$(date +%s)

timeout 300 "$VERUS" --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors \
    --num-threads 8 -V capture-profiles \
    "${CFG_FLAGS[@]}" 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | tee "$PROJECT_ROOT/logs/profile-$LABEL-$TIMESTAMP.log"
RC=${PIPESTATUS[0]}

ELAPSED=$(( $(date +%s) - START_SEC ))
echo "Elapsed: ${ELAPSED}s"

if [ -d "$PROJECT_ROOT/.verus-solver-log" ]; then
    mkdir -p "$PROFILE_OUT"
    echo ""

    # Run summary from raw solver log, save to file and print.
    run_summary "$PROJECT_ROOT/.verus-solver-log" | tee "$SUMMARY_FILE"

    # Delete raw data (can be GB). Summary is all we keep.
    rm -rf "$PROJECT_ROOT/.verus-solver-log"

    echo ""
    echo "=== Summary at: $SUMMARY_FILE ==="
else
    echo "WARNING: .verus-solver-log/ not created. Profile data unavailable."
fi

exit $RC
