#!/usr/bin/env bash
# Check Alg Analysis annotations for APAS vs Code Review matches.
# Output in Emacs compilation-mode format:
#   file:line: info: ...     — APAS and code review match
#   file:line: warning: ...  — accepted difference
#   file:line: error: ...    — mismatch, missing review, or unresolved DIFFERS
#
# Usage: scripts/check-alg-analysis.sh [-e|--error] [-w|--warn] [ChapNN ...]
#        scripts/check-alg-analysis.sh                   (all levels, all chapters)
#        scripts/check-alg-analysis.sh -e                (errors only)
#        scripts/check-alg-analysis.sh -w                (warnings only)
#        scripts/check-alg-analysis.sh -e -w             (errors + warnings)
#        scripts/check-alg-analysis.sh -e 37 42          (errors in Chap37 and Chap42)

set -uo pipefail

SHOW_INFO=false
SHOW_WARN=false
SHOW_ERROR=false
FILTER=false
CHAPTERS=()

while [ $# -gt 0 ]; do
    case "$1" in
        -e|--error)  FILTER=true; SHOW_ERROR=true; shift ;;
        -w|--warn)   FILTER=true; SHOW_WARN=true; shift ;;
        *)           CHAPTERS+=("$1"); shift ;;
    esac
done

# No filter flags → show everything
if ! $FILTER; then
    SHOW_INFO=true; SHOW_WARN=true; SHOW_ERROR=true
fi

if [ ${#CHAPTERS[@]} -gt 0 ]; then
    DIRS=""
    for chap in "${CHAPTERS[@]}"; do
        DIRS="$DIRS src/Chap${chap}"
    done
else
    DIRS="src/Chap*"
fi

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
            # Strip leading comment/whitespace and "Alg Analysis: APAS (ChNN ...): "
            # The APAS ref can contain colons, so strip everything up to "): " then
            # the remaining prefix.
            sub(/^[[:space:]]*\/\/\/? *- *Alg Analysis: APAS */, "", apas_text)
            sub(/^\([^)]*\): */, "", apas_text)
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
                } else if (review_text ~ /DIFFERS.*St sequential/) {
                    printf "%s:%d: warning: DIFFERS (St sequential): %s\n", FILENAME, review_line, review_text
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
done | tee /tmp/check-alg-analysis.out | {
    if $SHOW_INFO && $SHOW_WARN && $SHOW_ERROR; then
        cat
    else
        grep -E "$(
            parts=()
            $SHOW_INFO  && parts+=(': info:')
            $SHOW_WARN  && parts+=(': warning:')
            $SHOW_ERROR && parts+=(': error:')
            IFS='|'; echo "${parts[*]}"
        )" || true
    fi
}

# Summary (always from full output)
awk '
/: info:/ { info++ }
/: warning:/ { warn++ }
/: error:/ { err++ }
END {
    printf "\nSummary: %d info (match), %d warning (accepted), %d error (mismatch/missing)\n", info, warn, err
}
' /tmp/check-alg-analysis.out
