#!/bin/bash
# Verify one file from src/standards/ or src/experiments/ on its own, through a
# generated crate root that carries lib.rs's crate attributes. Logs to logs/.
#
# Usage:
#   scripts/validate-standard.sh <standard> [deps|nodeps] [ptt]
#   scripts/validate-standard.sh --experiment <experiment> [deps|nodeps]
#   scripts/validate-standard.sh BASELINE deps
#
#   <standard>    a file name under src/standards/ without .rs
#   <experiment>  a file name under src/experiments/ without .rs
#   deps          also declare Types, Concurrency, ParaPairs,
#                 Chap02::HFSchedulerMtEph, and the vstdplus modules the
#                 standards import (179 functions; verify alone with BASELINE)
#   ptt           also declare scratch/ptt_bodies/ptt_<standard>.rs as `pub mod ptt`
#
# The crate root is written to scratch/harness-<name>/lib.rs (gitignored) and
# the log to logs/validate-standard-<name>.<timestamp>.log. One Verus process
# at a time, through scripts/verus-lock.sh.

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERUS=~/projects/verus/source/target-verus/release/verus
export VERUS_Z3_PATH=~/projects/verus/source/target-verus/release/z3

KIND=standards
if [ "${1:-}" = "--experiment" ]; then
    KIND=experiments
    shift
fi
NAME="${1:?usage: validate-standard.sh [--experiment] <name> [deps|nodeps] [ptt]}"
WITH_DEPS="${2:-nodeps}"
WITH_PTT="${3:-}"

if [ "$NAME" != "BASELINE" ] && [ ! -f "$PROJECT_ROOT/src/$KIND/$NAME.rs" ]; then
    echo "No such file: src/$KIND/$NAME.rs" >&2
    exit 2
fi
PTT_BODY="$PROJECT_ROOT/scratch/ptt_bodies/ptt_$NAME.rs"
if [ "$WITH_PTT" = "ptt" ] && [ ! -f "$PTT_BODY" ]; then
    echo "No PTT body module: scratch/ptt_bodies/ptt_$NAME.rs" >&2
    exit 2
fi

export VERUS_LOCK_WEIGHT=1
source "$PROJECT_ROOT/scripts/verus-lock.sh"

HARNESS_DIR="$PROJECT_ROOT/scratch/harness-$NAME"
mkdir -p "$HARNESS_DIR"
ROOT="$HARNESS_DIR/lib.rs"

{
cat <<'EOF'
#![cfg_attr(verus_keep_ghost, feature(allocator_api))]
#![cfg_attr(verus_keep_ghost, feature(sized_hierarchy))]
#![cfg_attr(verus_keep_ghost, verifier::deprecated_postcondition_mut_ref_style(true))]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
EOF
if [ "$WITH_DEPS" = "deps" ]; then
cat <<EOF
#[path = "$PROJECT_ROOT/src/Types.rs"]
pub mod Types;
#[path = "$PROJECT_ROOT/src/Concurrency.rs"]
pub mod Concurrency;
#[path = "$PROJECT_ROOT/src/ParaPairs.rs"]
pub mod ParaPairs;
pub mod Chap02 {
    #[path = "$PROJECT_ROOT/src/Chap02/HFSchedulerMtEph.rs"]
    pub mod HFSchedulerMtEph;
}
pub mod vstdplus {
    #[path = "$PROJECT_ROOT/src/vstdplus/accept.rs"]
    pub mod accept;
    #[path = "$PROJECT_ROOT/src/vstdplus/clone_view.rs"]
    pub mod clone_view;
    #[path = "$PROJECT_ROOT/src/vstdplus/feq.rs"]
    pub mod feq;
    #[path = "$PROJECT_ROOT/src/vstdplus/hash_specs_plus.rs"]
    pub mod hash_specs_plus;
    #[path = "$PROJECT_ROOT/src/vstdplus/total_order.rs"]
    pub mod total_order;
    #[path = "$PROJECT_ROOT/src/vstdplus/threads_plus.rs"]
    pub mod threads_plus;
}
EOF
fi
if [ "$NAME" != "BASELINE" ]; then
cat <<EOF
pub mod $KIND {
    #[path = "$PROJECT_ROOT/src/$KIND/$NAME.rs"]
    pub mod $NAME;
}
EOF
fi
if [ "$WITH_PTT" = "ptt" ]; then
cat <<EOF
#[path = "$PTT_BODY"]
pub mod ptt;
EOF
fi
} > "$ROOT"

LOGDIR="$PROJECT_ROOT/logs"
mkdir -p "$LOGDIR"
LOGFILE="$LOGDIR/validate-standard-$NAME.$(date +%Y%m%d-%H%M%S).log"

cd "$PROJECT_ROOT"
{
    echo "File: src/$KIND/$NAME.rs (deps: $WITH_DEPS; ptt: ${WITH_PTT:-no})"
    echo "Harness root: $ROOT"
    "$VERUS" --version | sed -n '2p'
} | tee "$LOGFILE"
echo "Starting verification at $(date '+%H:%M:%S')" | tee -a "$LOGFILE"
START_SEC=$(date +%s)
timeout 600 "$VERUS" --crate-type=lib "$ROOT" --multiple-errors 20 --expand-errors \
    --num-threads 8 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | tee -a "$LOGFILE"
RC=${PIPESTATUS[0]}
echo "Elapsed: $(( $(date +%s) - START_SEC ))s" | tee -a "$LOGFILE"
echo "Log: $LOGFILE"
exit $RC
