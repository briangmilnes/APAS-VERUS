#!/usr/bin/env bash
# Process a partial veracity-minimize-lib run: restore TESTING, mark NEEDED
# using UNNEEDED markers as boundaries, then strip UNNEEDED.
#
# For each file in the chapter:
#   1. Restore any // Veracity: TESTING lines (uncomment the code)
#   2. Find the last // Veracity: UNNEEDED assert line (the boundary)
#   3. Mark every unmarked assert and proof block ABOVE that boundary
#      as // Veracity: NEEDED assert / // Veracity: NEEDED proof block
#   4. Strip all // Veracity: UNNEEDED assert lines
#
# Usage:  scripts/mark-needed-from-partial.sh <worktree> <ChapNN>
#   e.g.  scripts/mark-needed-from-partial.sh /home/milnes/projects/APAS-VERUS-agent1 Chap43

set -uo pipefail

WORKTREE="${1:?Usage: $0 <worktree> <ChapNN>}"
CHAP="${2:?Usage: $0 <worktree> <ChapNN>}"
CHAPDIR="${WORKTREE}/src/${CHAP}"

[ -d "$CHAPDIR" ] || { echo "ERROR: $CHAPDIR not found"; exit 1; }

TOTAL_TESTING=0
TOTAL_NEEDED_ASSERTS=0
TOTAL_NEEDED_PROOFS=0
TOTAL_STRIPPED=0

for f in "${CHAPDIR}"/*.rs; do
    [ -f "$f" ] || continue
    FNAME=$(basename "$f")

    # Step 1: Restore TESTING lines (uncomment the code).
    # Format: // Veracity: TESTING assert    assert(foo);
    #      or // Veracity: TESTING <code>
    TESTING_COUNT=$(grep -c "// Veracity: TESTING" "$f" || true)
    if [ "$TESTING_COUNT" -gt 0 ]; then
        # Strip the // Veracity: TESTING prefix, preserving indentation and code.
        # The format is: <indent>// Veracity: TESTING <kind>    <original code>
        # We need to extract the original code after the marker+kind.
        sed -i 's|^\([[:space:]]*\)// Veracity: TESTING[^[:space:]]* *|\1|' "$f"
        TOTAL_TESTING=$((TOTAL_TESTING + TESTING_COUNT))
        echo "  $FNAME: restored $TESTING_COUNT TESTING lines"
    fi

    # Step 2: Find last UNNEEDED line number (the boundary).
    LAST_UNNEEDED=$(grep -n "// Veracity: UNNEEDED assert" "$f" | tail -1 | cut -d: -f1)

    if [ -z "$LAST_UNNEEDED" ]; then
        # No UNNEEDED markers in this file. Either:
        #   - veracity never reached this file (don't mark anything)
        #   - all asserts were NEEDED (but we can't tell without the log)
        # Safe choice: don't mark. Skip.
        continue
    fi

    # Step 3: Mark NEEDED above the boundary.
    # Use awk: for lines before LAST_UNNEEDED, mark unmarked asserts and proof blocks.
    COUNTS=$(awk -v boundary="$LAST_UNNEEDED" '
    BEGIN { asserts = 0; proofs = 0 }
    {
        if (NR < boundary && $0 !~ /^[[:space:]]*\/\//) {
            if ($0 ~ /^[[:space:]]*(assert\(|assert |assert!)/ &&
                prev !~ /Veracity: NEEDED assert/) {
                match($0, /^[[:space:]]*/)
                indent = substr($0, RSTART, RLENGTH)
                print indent "// Veracity: NEEDED assert"
                asserts++
            }
            if ($0 ~ /^[[:space:]]*proof[[:space:]]*\{/ &&
                prev !~ /Veracity: NEEDED proof block/) {
                match($0, /^[[:space:]]*/)
                indent = substr($0, RSTART, RLENGTH)
                print indent "// Veracity: NEEDED proof block"
                proofs++
            }
        }
        print $0
        prev = $0
    }
    END { printf "%d %d\n", asserts, proofs > "/dev/stderr" }
    ' "$f" > "${f}.tmp" 2>/tmp/mark-counts.txt)
    mv "${f}.tmp" "$f"

    ASSERT_COUNT=$(awk '{print $1}' /tmp/mark-counts.txt)
    PROOF_COUNT=$(awk '{print $2}' /tmp/mark-counts.txt)

    # Step 4: Strip UNNEEDED lines.
    STRIP_COUNT=$(grep -c "// Veracity: UNNEEDED assert" "$f" || true)
    if [ "$STRIP_COUNT" -gt 0 ]; then
        grep -v "// Veracity: UNNEEDED assert" "$f" > "${f}.tmp"
        mv "${f}.tmp" "$f"
    fi

    if [ "$ASSERT_COUNT" -gt 0 ] || [ "$PROOF_COUNT" -gt 0 ] || [ "$STRIP_COUNT" -gt 0 ]; then
        MSG="$FNAME:"
        [ "$ASSERT_COUNT" -gt 0 ] && MSG="$MSG $ASSERT_COUNT NEEDED asserts,"
        [ "$PROOF_COUNT" -gt 0 ] && MSG="$MSG $PROOF_COUNT NEEDED proof blocks,"
        [ "$STRIP_COUNT" -gt 0 ] && MSG="$MSG -$STRIP_COUNT UNNEEDED stripped"
        echo "  $MSG"
    fi

    TOTAL_NEEDED_ASSERTS=$((TOTAL_NEEDED_ASSERTS + ASSERT_COUNT))
    TOTAL_NEEDED_PROOFS=$((TOTAL_NEEDED_PROOFS + PROOF_COUNT))
    TOTAL_STRIPPED=$((TOTAL_STRIPPED + STRIP_COUNT))
done

echo ""
echo "Summary for ${CHAP}:"
echo "  TESTING restored:     $TOTAL_TESTING"
echo "  NEEDED asserts:       $TOTAL_NEEDED_ASSERTS"
echo "  NEEDED proof blocks:  $TOTAL_NEEDED_PROOFS"
echo "  UNNEEDED stripped:    $TOTAL_STRIPPED"
