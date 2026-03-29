#!/bin/bash
# Profile Z3 instantiation counts.
# Usage: scripts/profile.sh [isolate ChapNN]
#        scripts/profile.sh                    # full crate profile
#        scripts/profile.sh isolate Chap65     # isolate + profile for Chap65
#
# Runs Verus with -V capture-profiles, collects .profile and .graph files
# from .verus-solver-log/ into logs/profile/<label>/<timestamp>/.
# Use scripts/profile-summary.sh to parse the Z3 traces afterward.

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERUS=~/projects/verus/source/target-verus/release/verus

MODE="${1:-full}"
shift 2>/dev/null || true

EXTRA_FEATURES=()
for arg in "$@"; do
    if [[ "$arg" == Chap* ]]; then EXTRA_FEATURES+=("$arg"); fi
done

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
        echo "Usage: $0 [isolate ChapNN]"
        exit 1
        ;;
esac

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
PROFILE_DIR="$PROJECT_ROOT/logs/profile/$LABEL/$TIMESTAMP"

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

# Collect profile output.
if [ -d "$PROJECT_ROOT/.verus-solver-log" ]; then
    mkdir -p "$PROFILE_DIR"
    cp "$PROJECT_ROOT/.verus-solver-log"/*.profile "$PROFILE_DIR/" 2>/dev/null
    cp "$PROJECT_ROOT/.verus-solver-log"/*.graph "$PROFILE_DIR/" 2>/dev/null

    NPROFILES=$(ls "$PROFILE_DIR"/*.profile 2>/dev/null | wc -l)
    NGRAPHS=$(ls "$PROFILE_DIR"/*.graph 2>/dev/null | wc -l)
    echo ""
    echo "=== Collected $NPROFILES profile files, $NGRAPHS graph files ==="
    echo "=== Stored in $PROFILE_DIR ==="
    echo ""
    echo "Run: scripts/profile-summary.sh $PROFILE_DIR"
    echo "  or: scripts/profile-summary.sh $PROFILE_DIR/<module>.profile"
else
    echo "WARNING: .verus-solver-log/ not created. Profile data unavailable."
fi

exit $RC
