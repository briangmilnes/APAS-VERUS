#!/usr/bin/env bash
# Add "// Veracity: NEEDED assert" and "// Veracity: NEEDED proof block"
# markers before every assert and proof block in the given chapter
# directories.  Only marks items that:
#   - are not inside comments
#   - are not already preceded by a NEEDED marker
#
# Usage:  scripts/mark-needed-asserts.sh Chap05 Chap06 ...
#         scripts/mark-needed-asserts.sh --all-minimized

set -uo pipefail

if [ "${1:-}" = "--all-minimized" ]; then
    # Chapters that have been through veracity-minimize-lib as of R162.
    CHAPTERS=(05 06 17 18 19 21 23 26 27 28 35 36 37 38 39 40 41 42 54)
    shift
else
    CHAPTERS=("$@")
fi

[ ${#CHAPTERS[@]} -eq 0 ] && { echo "Usage: $0 Chap05 Chap06 ... | --all-minimized"; exit 1; }

TOTAL_ASSERTS=0
TOTAL_PROOFS=0

for CHAP in "${CHAPTERS[@]}"; do
    CHAPDIR="src/Chap${CHAP}"
    [ -d "$CHAPDIR" ] || { echo "SKIP: $CHAPDIR not found"; continue; }

    for f in "${CHAPDIR}"/*.rs; do
        [ -f "$f" ] || continue

        COUNTS=$(awk '
        /^[[:space:]]*\/\// { prev = $0; next }
        /^[[:space:]]*(assert\(|assert |assert!)/ {
            if (prev !~ /Veracity: NEEDED assert/) asserts++
        }
        /^[[:space:]]*proof[[:space:]]*\{/ {
            if (prev !~ /Veracity: NEEDED proof block/) proofs++
        }
        { prev = $0 }
        END { print asserts+0 " " proofs+0 }
        ' "$f")

        ASSERT_COUNT=${COUNTS%% *}
        PROOF_COUNT=${COUNTS##* }

        if [ "$ASSERT_COUNT" -gt 0 ] || [ "$PROOF_COUNT" -gt 0 ]; then
            awk '
            {
                # Not a comment line
                if ($0 !~ /^[[:space:]]*\/\//) {
                    # Assert: not already marked
                    if ($0 ~ /^[[:space:]]*(assert\(|assert |assert!)/ &&
                        prev !~ /Veracity: NEEDED assert/) {
                        match($0, /^[[:space:]]*/)
                        indent = substr($0, RSTART, RLENGTH)
                        print indent "// Veracity: NEEDED assert"
                    }
                    # Proof block: not already marked
                    if ($0 ~ /^[[:space:]]*proof[[:space:]]*\{/ &&
                        prev !~ /Veracity: NEEDED proof block/) {
                        match($0, /^[[:space:]]*/)
                        indent = substr($0, RSTART, RLENGTH)
                        print indent "// Veracity: NEEDED proof block"
                    }
                }
                print $0
                prev = $0
            }
            ' "$f" > "${f}.tmp"
            mv "${f}.tmp" "$f"
            MSG=""
            [ "$ASSERT_COUNT" -gt 0 ] && MSG="${ASSERT_COUNT} asserts"
            [ "$PROOF_COUNT" -gt 0 ] && { [ -n "$MSG" ] && MSG="$MSG, "; MSG="${MSG}${PROOF_COUNT} proof blocks"; }
            echo "  $(basename "$f"): $MSG"
            TOTAL_ASSERTS=$((TOTAL_ASSERTS + ASSERT_COUNT))
            TOTAL_PROOFS=$((TOTAL_PROOFS + PROOF_COUNT))
        fi
    done
done

echo "Total: $TOTAL_ASSERTS asserts + $TOTAL_PROOFS proof blocks marked as NEEDED"
