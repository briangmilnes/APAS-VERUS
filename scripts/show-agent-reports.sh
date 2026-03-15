#!/bin/bash
# Show agent reports for a given round.
# Usage: scripts/show-agent-reports.sh <round> [lines]
#   round  — round number (e.g. 17)
#   lines  — optional head limit (default: full file)

set -euo pipefail

ROUND=${1:?Usage: show-agent-reports.sh <round> [lines]}
LINES=${2:-0}

for i in 1 2 3 4; do
    DIR="/home/milnes/projects/APAS-VERUS-agent${i}"
    REPORT="plans/agent${i}-round${ROUND}-report.md"
    echo "=== agent${i} ==="
    if [[ -d "$DIR" ]]; then
        if [[ -f "$DIR/$REPORT" ]]; then
            if [[ "$LINES" -gt 0 ]]; then
                head -${LINES} "$DIR/$REPORT"
            else
                cat "$DIR/$REPORT"
            fi
        else
            echo "(no report: $REPORT)"
        fi
    else
        echo "(worktree not found: $DIR)"
    fi
    echo
done
