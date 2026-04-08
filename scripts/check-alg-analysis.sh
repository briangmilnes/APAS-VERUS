#!/usr/bin/env bash
# Check Alg Analysis annotations for APAS vs Code Review matches.
# Output in Emacs compilation-mode format:
#   file:line: info: ...     — APAS and code review match
#   file:line: warning: ...  — accepted difference
#   file:line: error: ...    — mismatch, missing review, or unresolved DIFFERS
#
# Usage: scripts/check-alg-analysis.sh [ChapNN ...]
#        scripts/check-alg-analysis.sh              (all chapters)

set -uo pipefail

if [ $# -gt 0 ]; then
    DIRS=""
    for chap in "$@"; do
        DIRS="$DIRS src/Chap${chap}"
    done
else
    DIRS="src/Chap*"
fi

INFO=0
WARN=0
ERROR=0

for dir in $DIRS; do
    [ -d "$dir" ] || continue
    for f in "$dir"/*.rs; do
        [ -f "$f" ] || continue
        # Skip Example and Problem files
        case "$(basename "$f")" in
            Example*|Problem*) continue ;;
        esac

        # Process pairs: APAS line followed by Code review line
        awk '
        /Alg Analysis: APAS/ {
            apas_line = NR
            apas_text = $0
            # Strip leading comment/whitespace
            sub(/^[[:space:]]*\/\/\/? *- *Alg Analysis: APAS[^:]*: */, "", apas_text)
            next
        }
        /Alg Analysis: Code review/ {
            review_line = NR
            review_text = $0
            sub(/^[[:space:]]*\/\/\/? *- *Alg Analysis: Code review[^:]*: */, "", review_text)

            if (apas_line > 0 && (review_line - apas_line) <= 3) {
                # We have a pair
                if (review_text ~ /ACCEPTED DIFFERENCE/) {
                    printf "%s:%d: warning: ACCEPTED DIFFERENCE: %s\n", FILENAME, review_line, review_text
                } else if (review_text ~ /DIFFERS/) {
                    printf "%s:%d: error: DIFFERS (not accepted): %s\n", FILENAME, review_line, review_text
                } else if (review_text ~ /matches APAS/ || review_text ~ /matches/) {
                    printf "%s:%d: info: match: APAS: %s | Review: %s\n", FILENAME, apas_line, apas_text, review_text
                } else {
                    # Has both but no explicit match/differs tag — compare Work specs
                    # Extract Work O(...) from both
                    apas_work = apas_text
                    sub(/,.*/, "", apas_work)
                    review_work = review_text
                    sub(/,.*/, "", review_work)
                    if (apas_work == review_work) {
                        printf "%s:%d: info: match (implicit): APAS: %s | Review: %s\n", FILENAME, apas_line, apas_text, review_text
                    } else {
                        printf "%s:%d: error: possible mismatch: APAS: %s | Review: %s\n", FILENAME, apas_line, apas_text, review_text
                    }
                }
            } else if (apas_line == 0) {
                # Code review without APAS — just informational
                # (many functions have no APAS cost spec)
            }
            apas_line = 0
            apas_text = ""
        }
        # APAS line with no following Code review within 3 lines
        /Alg Analysis: APAS/ { }
        !/Alg Analysis/ {
            if (apas_line > 0 && NR > apas_line + 3) {
                printf "%s:%d: error: APAS annotation with no code review: %s\n", FILENAME, apas_line, apas_text
                apas_line = 0
                apas_text = ""
            }
        }
        ' "$f"
    done
done | tee /tmp/check-alg-analysis.out

# Summary
awk '
/: info:/ { info++ }
/: warning:/ { warn++ }
/: error:/ { err++ }
END {
    printf "\nSummary: %d info (match), %d warning (accepted), %d error (mismatch/missing)\n", info, warn, err
}
' /tmp/check-alg-analysis.out
