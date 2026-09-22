#!/bin/bash
# r212: run one chapter's proof-time tests through the real rust_verify_test
# harness while the rest of the crate still has type errors and cargo-nextest
# is absent. Steps, mirroring scripts/ptt.sh: (1) compile src/lib.rs with the
# isolate cfg flags of scripts/validate.sh to target/verus/libapas_verus.rlib
# and apas_verus.vir; (2) `cargo test --release` (not nextest) in
# rust_verify_test on every [[test]] whose path is tests/ChapNN/. ANSI stripped,
# logged to logs/ptt-ChapNN.<timestamp>.log. One Verus or cargo process at a
# time. Usage: scratch/r212-ptt.sh ChapNN
set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERUS=~/projects/verus/source/target-verus/release/verus
export VERUS_Z3_PATH=~/projects/verus/source/target-verus/release/z3
CHAP="${1:?usage: r212-ptt.sh ChapNN}"

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

ALL_CHAPS=$(resolve_deps "$CHAP")
CFG_FLAG=(--cfg 'feature="isolate"')
for chap in $ALL_CHAPS; do
    CFG_FLAG+=(--cfg "feature=\"$chap\"")
done

RLIB_PATH="$PROJECT_ROOT/target/verus/libapas_verus.rlib"
VIR_PATH="$PROJECT_ROOT/target/verus/apas_verus.vir"
mkdir -p "$PROJECT_ROOT/target/verus" "$PROJECT_ROOT/logs"
LOGFILE="$PROJECT_ROOT/logs/ptt-$CHAP.$(date +%Y%m%d-%H%M%S).log"

# The [[test]] names whose path is tests/ChapNN/.
mapfile -t TESTS < <(awk -v chap="$CHAP" '
    /^\[\[test\]\]/ { name=""; next }
    /^name = / { gsub(/name = |"/, ""); name=$0; next }
    /^path = / { gsub(/path = |"/, ""); if (index($0, "tests/" chap "/") == 1 && name != "") print name }
' "$PROJECT_ROOT/rust_verify_test/Cargo.toml")

cd "$PROJECT_ROOT"
START_SEC=$(date +%s)
{
    echo "r212 PTT for $CHAP (isolate: $ALL_CHAPS); tests: ${TESTS[*]}"
    echo "Starting PTT at $(date '+%H:%M:%S')"
    # `--no-verify`: this step only produces the rlib and the exported vir the
    # proof-time tests verify against; the crate itself is verified by
    # scripts/validate.sh isolate ChapNN (run first). Under `--crate-name
    # apas_verus` two pre-existing proofs that verify under validate.sh
    # (Chap18 LinkedListStEph.rs `scan` loop at 706, Chap37 AVLTreeSeq.rs
    # `insert_at_link` at 666) hit the rlimit, and the scan loop still does
    # at --rlimit 20 (matching loop over Seq::fold_left).
    "$VERUS" --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
        -o "$RLIB_PATH" --export "$VIR_PATH" --num-threads 8 --no-verify "${CFG_FLAG[@]}" 2>&1
    echo "compile rc: ${PIPESTATUS[0]:-$?}"
    cd "$PROJECT_ROOT/rust_verify_test"
    ARGS=()
    for t in "${TESTS[@]}"; do ARGS+=(--test "$t"); done
    cargo test --release --no-fail-fast "${ARGS[@]}" 2>&1
    echo "cargo test rc: $?"
} 2>&1 | sed 's/\x1b\[[0-9;]*[mGKHABCDEFJST]//g' | tee "$LOGFILE"
echo "Elapsed: $(( $(date +%s) - START_SEC ))s" | tee -a "$LOGFILE"
echo "Log: $LOGFILE"
