#!/usr/bin/env bash
# Watch progress of algorithmic analysis annotations across all chapters.
# Usage: watch -n5 scripts/watch-alg-analysis.sh
#    or: while true; do scripts/watch-alg-analysis.sh; sleep 5; done

set -euo pipefail
cd "$(git -C "$(dirname "$0")/.." rev-parse --show-toplevel)"

CHAPS=(02 03 04 05 06 07 08 09 10 11 12 13 14 15 17 18 19 20 21 22 23 24 26 27 28 29 30 31 32 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 61 62 63 64 65 66)

total_apas_new=0; total_apas_old=0
total_none=0; total_done_new=0; total_done_old=0
total_agrees=0; total_differs=0

printf "%-8s %4s %4s %4s %4s %4s %4s\n" "Chapter" "APAS" "Done" "NONE" "Agre" "Diff" "Pct"
printf "%-8s %4s %4s %4s %4s %4s %4s\n" "-------" "----" "----" "----" "----" "----" "---"

for d in "${CHAPS[@]}"; do
    dir="src/Chap${d}"
    [[ -d "$dir" ]] || continue

    apas_new=$(grep -r "Alg Analysis: APAS" "$dir" 2>/dev/null | wc -l)
    apas_old=$(grep -r "/// - APAS:" "$dir" 2>/dev/null | wc -l)
    apas=$((apas_new + apas_old))
    [[ $apas -eq 0 ]] && continue

    none=$(grep -r "Claude-Opus-4.6 (1M): NONE" "$dir" 2>/dev/null | wc -l)
    done_new=$(grep -r "Code review (Claude Opus" "$dir" 2>/dev/null | wc -l)
    done_old=$(grep -r "/// - Claude-Opus-4.6:" "$dir" 2>/dev/null | grep -v NONE | wc -l)
    done_total=$((done_new + done_old))

    agrees=$(grep -ri "agrees\|matches APAS" "$dir" 2>/dev/null | grep -i "Claude-Opus\|Code review" | wc -l)
    differs=$(grep -ri "differs\|DIFFERS" "$dir" 2>/dev/null | grep -i "Claude-Opus\|Code review" | wc -l)

    if [[ $apas -gt 0 ]]; then
        pct=$(( (done_total * 100) / apas ))
    else
        pct=0
    fi

    printf "Chap%-4s %4d %4d %4d %4d %4d %3d%%\n" "$d" "$apas" "$done_total" "$none" "$agrees" "$differs" "$pct"

    total_apas_new=$((total_apas_new + apas_new))
    total_apas_old=$((total_apas_old + apas_old))
    total_none=$((total_none + none))
    total_done_new=$((total_done_new + done_new))
    total_done_old=$((total_done_old + done_old))
    total_agrees=$((total_agrees + agrees))
    total_differs=$((total_differs + differs))
done

total_apas=$((total_apas_new + total_apas_old))
total_done=$((total_done_new + total_done_old))
if [[ $total_apas -gt 0 ]]; then
    total_pct=$(( (total_done * 100) / total_apas ))
else
    total_pct=0
fi

printf "%-8s %4s %4s %4s %4s %4s %4s\n" "-------" "----" "----" "----" "----" "----" "---"
printf "%-8s %4d %4d %4d %4d %4d %3d%%\n" "TOTAL" "$total_apas" "$total_done" "$total_none" "$total_agrees" "$total_differs" "$total_pct"
echo ""
echo "APAS = textbook annotations | Done = code reviews written | NONE = placeholders left"
echo "Agre = matches APAS | Diff = differs from APAS | Pct = Done/APAS"
