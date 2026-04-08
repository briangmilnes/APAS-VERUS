#!/usr/bin/env bash
# Add "// Veracity: NEEDED assert" markers before every assert in the given
# chapter directories.  Only marks asserts that:
#   - are not inside comments
#   - are not already preceded by a NEEDED marker
#
# Usage:  scripts/mark-needed-asserts.sh Chap05 Chap06 ...
#         scripts/mark-needed-asserts.sh --all-minimized

set -uo pipefail

if [ "${1:-}" = "--all-minimized" ]; then
    # Chapters that have been through veracity-minimize-lib as of R162.
    CHAPTERS=(05 06 17 18 19 21 23 26 27 28 35 36 40 54)
    shift
else
    CHAPTERS=("$@")
fi

[ ${#CHAPTERS[@]} -eq 0 ] && { echo "Usage: $0 Chap05 Chap06 ... | --all-minimized"; exit 1; }

TOTAL=0

for CHAP in "${CHAPTERS[@]}"; do
    CHAPDIR="src/Chap${CHAP}"
    [ -d "$CHAPDIR" ] || { echo "SKIP: $CHAPDIR not found"; continue; }

    for f in "${CHAPDIR}"/*.rs; do
        [ -f "$f" ] || continue

        COUNT=$(awk '
        /^[[:space:]]*\/\// { next }
        /^[[:space:]]*(assert\(|assert |assert!)/ {
            if (prev !~ /Veracity: NEEDED assert/) count++
        }
        { prev = $0 }
        END { print count+0 }
        ' "$f")

        if [ "$COUNT" -gt 0 ]; then
            awk '
            {
                skip = 0
                # Not a comment line, matches assert, previous line not already marked
                if ($0 !~ /^[[:space:]]*\/\// &&
                    $0 ~ /^[[:space:]]*(assert\(|assert |assert!)/ &&
                    prev !~ /Veracity: NEEDED assert/) {
                    # Extract leading whitespace
                    match($0, /^[[:space:]]*/)
                    indent = substr($0, RSTART, RLENGTH)
                    print indent "// Veracity: NEEDED assert"
                }
                print $0
                prev = $0
            }
            ' "$f" > "${f}.tmp"
            mv "${f}.tmp" "$f"
            echo "  $(basename "$f"): $COUNT asserts marked"
            TOTAL=$((TOTAL + COUNT))
        fi
    done
done

echo "Total: $TOTAL asserts marked as NEEDED"
