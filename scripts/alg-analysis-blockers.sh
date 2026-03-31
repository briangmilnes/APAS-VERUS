#!/bin/bash
# Algorithmic analysis blockers report.
# Section 1: Summary table
# Section 2: Mt blockers by category (the real issues)
# Section 3: Full DIFFERS detail
# Usage: scripts/alg-analysis-blockers.sh [| tee analyses/alg-analysis-blockers.log]

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$PROJECT_ROOT/src"

echo "Algorithmic Analysis Blockers — $(date '+%Y-%m-%d %H:%M:%S')"
echo "============================================================="

# Gather totals
total_apas=$(grep -rc 'Alg Analysis: APAS' "$SRC/" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
total_filled=$(grep -rc 'Code review (Claude Opus 4.6):' "$SRC/" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
total_nones=$(grep -rc 'Claude-Opus-4.6 (1M): NONE' "$SRC/" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
total_matches=$(grep -rn 'matches APAS' "$SRC/" 2>/dev/null | grep 'Code review' | wc -l)
total_differs=$(grep -rn 'DIFFERS' "$SRC/" 2>/dev/null | grep 'Code review' | wc -l)
st_differs=$(grep -rn 'DIFFERS' "$SRC/" 2>/dev/null | grep 'Code review' | grep -E 'St(Eph|Per)\.' | wc -l)
mt_differs=$(grep -rn 'DIFFERS' "$SRC/" 2>/dev/null | grep 'Code review' | grep -E 'Mt(Eph|Per)\.' | wc -l)
other_differs=$(grep -rn 'DIFFERS' "$SRC/" 2>/dev/null | grep 'Code review' | grep -vE '(St|Mt)(Eph|Per)\.' | wc -l)

echo ""
echo "1. Summary"
echo "-------------------------------------------------------------"
echo ""
echo "  APAS annotations:    $total_apas"
echo "  Code reviews:        $total_filled"
echo "  NONEs remaining:     $total_nones"
echo "  Matches APAS:        $total_matches"
echo "  DIFFERS total:       $total_differs"
echo "    St (expected):     $st_differs"
echo "    Mt (blockers):     $mt_differs"
echo "    Other:             $other_differs"
echo ""

# Per-chapter table
printf "  %-8s  %5s  %5s  %5s  %5s  %5s  %5s\n" "Chapter" "APAS" "Done" "NONE" "Match" "StDif" "MtDif"
printf "  %-8s  %5s  %5s  %5s  %5s  %5s  %5s\n" "--------" "-----" "-----" "-----" "-----" "-----" "-----"

for d in "$SRC"/Chap*/; do
    chap=$(basename "$d")
    apas=$(grep -rc 'Alg Analysis: APAS' "$d" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
    [ "$apas" -eq 0 ] && continue
    done_c=$(grep -rc 'Code review (Claude Opus 4.6):' "$d" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
    nones=$(grep -rc 'Claude-Opus-4.6 (1M): NONE' "$d" 2>/dev/null | awk -F: '{s+=$NF} END {print s+0}')
    matches=$(grep -rn 'matches APAS' "$d" 2>/dev/null | grep 'Code review' | wc -l)
    st_d=$(grep -rn 'DIFFERS' "$d" 2>/dev/null | grep 'Code review' | grep -E 'St(Eph|Per)\.' | wc -l)
    mt_d=$(grep -rn 'DIFFERS' "$d" 2>/dev/null | grep 'Code review' | grep -E 'Mt(Eph|Per)\.' | wc -l)
    # Other differs (base modules in this chapter)
    oth_d=$(grep -rn 'DIFFERS' "$d" 2>/dev/null | grep 'Code review' | grep -vE '(St|Mt)(Eph|Per)\.' | wc -l)
    total_d=$((st_d + mt_d + oth_d))
    st_show=$st_d
    mt_show=$mt_d
    [ "$oth_d" -gt 0 ] && mt_show="$mt_d+$oth_d"
    printf "  %-8s  %5d  %5d  %5d  %5d  %5d  %5d\n" "$chap" "$apas" "$done_c" "$nones" "$matches" "$st_d" "$mt_d"
done

echo ""
echo "============================================================="
echo "2. Mt Blockers by Category"
echo "============================================================="
echo ""

# Extract Mt DIFFERS, categorize by common patterns
tmpfile=$(mktemp)
grep -rn 'DIFFERS' "$SRC/" 2>/dev/null | grep 'Code review' | grep -E 'Mt(Eph|Per)\.' > "$tmpfile"

# Categorize
cat_sequential_loop=$(grep -ci 'sequential loop\|sequential scan\|sequential filter\|sequential clone\|sequential tabulate\|sequential recursion\|sequential key\|sequential in-order\|sequential tree\|sequential split-join\|sequential insert\|sequential init\|sequential single\|sequential reduce\|sequential contraction\|sequential DP\|sequential partition\|sequential map\|sequential word' "$tmpfile" 2>/dev/null || echo 0)
cat_mt_not_parallel=$(grep -ci 'despite Mt naming\|no join/spawn' "$tmpfile" 2>/dev/null || echo 0)
cat_apas_cached=$(grep -ci 'APAS assumes cached\|APAS says O(1)\|APAS says O(n)' "$tmpfile" 2>/dev/null || echo 0)
cat_wrong_ds=$(grep -ci 'nested linear\|linear scan\|array\|Vec-backed\|copies elements\|clone entire\|copy with\|bit scan\|bit set\|bit clear' "$tmpfile" 2>/dev/null || echo 0)
cat_other=$((mt_differs - cat_sequential_loop - cat_mt_not_parallel - cat_apas_cached))
[ "$cat_other" -lt 0 ] && cat_other=0

printf "  %4d  Sequential impl where APAS assumes parallel (span issue)\n" "$cat_sequential_loop"
printf "  %4d  Mt file but no actual parallelism (no join/spawn)\n" "$cat_mt_not_parallel"
printf "  %4d  APAS assumes cached/stored value, impl recomputes\n" "$cat_apas_cached"
echo ""
echo "  Top reasons:"
grep -oP 'DIFFERS: \K.*' "$tmpfile" | sed 's/[;,].*//' | sort | uniq -c | sort -rn | head -20 | while read -r count reason; do
    printf "    %3d  %s\n" "$count" "$reason"
done

echo ""
echo "============================================================="
echo "3. Mt DIFFERS Detail (by chapter)"
echo "============================================================="
echo ""

printf "  # | %-6s | %-30s | %-4s | %s\n" "Chap" "File" "Line" "Reason"
printf "  # | %-6s | %-30s | %-4s | %s\n" "------" "------------------------------" "----" "----------"

idx=0
sort -t/ -k3,3 -k4,4 "$tmpfile" | while IFS= read -r line; do
    idx=$((idx + 1))
    filepath=$(echo "$line" | cut -d: -f1)
    lineno=$(echo "$line" | cut -d: -f2)
    file=$(basename "$filepath")
    chap=$(basename "$(dirname "$filepath")" | sed 's/Chap//')
    reason=$(echo "$line" | sed 's/.*DIFFERS: *//')
    printf "  %3d | %-6s | %-30s | %4s | %s\n" "$idx" "$chap" "$file" "$lineno" "$reason"
done

rm -f "$tmpfile"

echo ""
echo "============================================================="
echo "4. St DIFFERS Summary (expected, for reference)"
echo "============================================================="
echo ""
echo "  $st_differs St functions have sequential span where APAS specifies parallel."
echo "  This is by design — St variants are sequential implementations."
echo "  Mt variants should achieve the APAS parallel span."
echo ""
echo "End of report."
