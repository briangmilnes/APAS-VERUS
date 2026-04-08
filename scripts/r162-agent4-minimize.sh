#!/usr/bin/env bash
# R162 Agent4: veracity-minimize-lib on Chap37.
#
# Chap37 has 1184 asserts, isolate time ~17s.
# Estimated runtime: ~335 min (~5.5 hours).

set -uo pipefail

AGENT="/home/milnes/projects/APAS-VERUS-agent4"
VERACITY="/home/milnes/projects/veracity/target/release/veracity-minimize-lib"
VERACITY_DIR="/home/milnes/projects/veracity"
CHAP="37"
CHAPDIR="${AGENT}/src/Chap${CHAP}"

log() { echo "[$(date '+%H:%M:%S')] $*"; }
die() { log "FATAL: $*"; exit 1; }

log "══════════════════════════════════════════"
log "Chap${CHAP}: starting (~335 min estimated)"
log "══════════════════════════════════════════"

[ -d "$CHAPDIR" ] || die "Chapter directory not found: $CHAPDIR"

# Confirm clean git state (veracity requires it).
cd "$AGENT"
DIRTY=$(git status --porcelain | wc -l)
[ "$DIRTY" -eq 0 ] || die "Worktree has uncommitted changes. Stopping."

# Run veracity-minimize-lib.
log "  Running veracity-minimize-lib..."
cd "$VERACITY_DIR"
"$VERACITY" \
    -c "$AGENT" \
    -l "${AGENT}/src/vstdplus" \
    --no-lib-min --project APAS --chapter "Chap${CHAP}" -a

# Find the log veracity just wrote (newest by mtime).
VLOG=$(ls -t "${AGENT}/analyses/veracity-minimize-lib."*.log 2>/dev/null | head -1)
[ -n "$VLOG" ] || die "No veracity log found after run."

grep -q "Phase 10" "$VLOG" || die "Phase 10 not found in $VLOG — veracity aborted. Fix verification errors first."

# Strip UNNEEDED markers.
cd "$AGENT"
REMOVED=0
for f in "${CHAPDIR}"/*.rs; do
    [ -f "$f" ] || continue
    COUNT=$(grep -c "// Veracity: UNNEEDED assert" "$f" || true)
    if [ "$COUNT" -gt 0 ]; then
        grep -v "// Veracity: UNNEEDED assert" "$f" > "${f}.tmp"
        mv "${f}.tmp" "$f"
        REMOVED=$((REMOVED + COUNT))
        log "  $(basename "$f"): -${COUNT} asserts"
    fi
done
log "  Total markers removed: ${REMOVED}"

if [ "$REMOVED" -eq 0 ]; then
    log "  No UNNEEDED markers found. Nothing to commit."
    exit 0
fi

# Validate.
log "  Validating isolate Chap${CHAP}..."
VALLOG="/tmp/r162-validate-chap${CHAP}.log"
scripts/validate.sh isolate "Chap${CHAP}" 2>&1 | tee "$VALLOG" || true

if grep -q ", 0 errors" "$VALLOG"; then
    log "  Validation: PASSED"
else
    log "  Validation: FAILED — check $VALLOG"
    log "  Veracity log: $VLOG"
    die "Stopping on validation failure."
fi

# Commit and push.
git add -A
git commit -m "$(cat <<EOF
R162 Agent 4: minimize Chap${CHAP} (-${REMOVED} asserts)

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
git push

log "  Chap${CHAP} complete: -${REMOVED} asserts committed and pushed."
