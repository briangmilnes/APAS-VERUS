#!/bin/bash
# Track alg analysis progress across agent worktrees (live, not just pushed).
# Shows NONE placeholders remaining vs filled per agent.
# Usage: scripts/watch-alg-analysis.sh

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Alg Analysis Progress — $(date '+%H:%M:%S')"
echo "================================================"

main_nones=$(grep -rc 'Claude-Opus-4.6 (1M): NONE' "$PROJECT_ROOT/src/" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
main_filled=$(grep -rc 'Code review (Claude Opus 4.6):' "$PROJECT_ROOT/src/" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
echo ""
echo "Main: ${main_nones} NONEs, ${main_filled} filled"
echo ""

printf "%-8s  %6s  %6s  %5s  %5s  %s\n" "Agent" "NONEs" "Filled" "Done%" "Dirty" "Status"
printf "%-8s  %6s  %6s  %5s  %5s  %s\n" "--------" "------" "------" "-----" "-----" "----------"

for i in 1 2 3 4; do
    worktree="$PROJECT_ROOT/../APAS-VERUS-agent$i"
    if [ ! -d "$worktree/src" ]; then
        printf "%-8s  %6s  %6s  %5s  %5s  %s\n" "agent$i" "-" "-" "-" "-" "no worktree"
        continue
    fi

    nones=$(grep -rc 'Claude-Opus-4.6 (1M): NONE' "$worktree/src/" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
    filled=$(grep -rc 'Code review (Claude Opus 4.6):' "$worktree/src/" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
    dirty=$(cd "$worktree" && git status -s src/ 2>/dev/null | wc -l)

    total=$((nones + filled))
    if [ "$total" -gt 0 ]; then
        pct=$((filled * 100 / total))
    else
        pct=0
    fi

    if [ "$filled" -eq 0 ] && [ "$dirty" -eq 0 ]; then
        status="not started"
    elif [ "$nones" -eq 0 ]; then
        status="done"
    else
        status="working"
    fi

    printf "%-8s  %6d  %6d  %4d%%  %5d  %s\n" "agent$i" "$nones" "$filled" "$pct" "$dirty" "$status"
done

echo ""
echo "Total slots: $((main_nones + main_filled))"
