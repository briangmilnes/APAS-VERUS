#!/usr/bin/env bash
# R162 Agent3 sequential veracity-minimize-lib loop.
#
# Chapters (rows 1-12 from timing table, skipping done/running):
#   05 06 17 18 19 21 23 27 28
#
# Skipped:
#   26 — veracity running
#   35 — agent2 running
#   36 — done (-219 asserts)
#   37 — deferred (335 min alone, run separately)
#
# Estimated runtime: ~187 min (~3 hours sequential).

set -uo pipefail

AGENT="/home/milnes/projects/APAS-VERUS-agent3"
VERACITY="/home/milnes/projects/veracity/target/release/veracity-minimize-lib"
VERACITY_DIR="/home/milnes/projects/veracity"

CHAPTERS=(05 06 17 18 19 21 23 27 28)

log() { echo "[$(date '+%H:%M:%S')] $*"; }

die() { log "FATAL: $*"; exit 1; }

for CHAP in "${CHAPTERS[@]}"; do
    log "══════════════════════════════════════════"
    log "Chap${CHAP}: starting"
    log "══════════════════════════════════════════"

    CHAPDIR="${AGENT}/src/Chap${CHAP}"
    [ -d "$CHAPDIR" ] || die "Chapter directory not found: $CHAPDIR"

    # Confirm clean git state (veracity requires it).
    cd "$AGENT"
    DIRTY=$(git status --porcelain | wc -l)
    [ "$DIRTY" -eq 0 ] || die "Chap${CHAP}: worktree has uncommitted changes. Stopping."

    # Run veracity-minimize-lib. It writes its own timestamped log to analyses/.
    log "  Running veracity-minimize-lib..."
    cd "$VERACITY_DIR"
    "$VERACITY" \
        -c "$AGENT" \
        -l "${AGENT}/src/vstdplus" \
        --no-lib-min --project APAS --chapter "Chap${CHAP}" -a

    # Find the log veracity just wrote (newest by mtime).
    VLOG=$(ls -t "${AGENT}/analyses/veracity-minimize-lib."*.log 2>/dev/null | head -1)
    [ -n "$VLOG" ] || die "No veracity log found after Chap${CHAP} run."

    # Confirm Phase 10 ran — if it aborted at Phase 1 we should stop.
    grep -q "Phase 10" "$VLOG" || die "Phase 10 not found in $VLOG — veracity aborted. Fix verification errors first."

    # Strip UNNEEDED markers from all .rs files in the chapter directory.
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
        log "  No UNNEEDED markers found. Nothing to commit — skipping."
        continue
    fi

    # Validate isolate ChapNN. Must show 0 errors.
    log "  Validating isolate Chap${CHAP}..."
    VALLOG="/tmp/r162-validate-chap${CHAP}.log"
    scripts/validate.sh isolate "Chap${CHAP}" 2>&1 | tee "$VALLOG" || true

    if grep -q ", 0 errors" "$VALLOG"; then
        log "  Validation: PASSED"
    else
        log "  Validation: FAILED — check $VALLOG"
        log "  The veracity log is at: $VLOG"
        log "  Restore any incorrectly removed asserts from the log, then re-run."
        die "Stopping on Chap${CHAP} validation failure."
    fi

    # Commit and push.
    git add -A
    git commit -m "$(cat <<EOF
R162 Agent 3: minimize Chap${CHAP} (-${REMOVED} asserts)

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
    git push

    log "  Chap${CHAP} complete: -${REMOVED} asserts committed and pushed."
done

log "══════════════════════════════════════════"
log "All chapters complete."
log "══════════════════════════════════════════"
