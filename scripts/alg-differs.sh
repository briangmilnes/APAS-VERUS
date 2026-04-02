#!/bin/bash
# alg-differs.sh — Find and classify algorithmic differences between
# implementation and APAS cost specifications.
#
# Reads the Code review annotations from source files and classifies
# Mt DIFFERS into actionable categories.
#
# Usage: scripts/alg-differs.sh [--refresh]
#   --refresh  Re-run veracity analysis first (slow)

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

LOG=analyses/veracity-analyze-alg-analysis.log

if [[ "${1:-}" == "--refresh" ]]; then
    echo "Refreshing analysis..."
    ~/projects/veracity/target/release/veracity-analyze-alg-analysis -c . > "$LOG" 2>&1 || true
fi

if [[ ! -f "$LOG" ]]; then
    echo "No analysis log found. Run: scripts/alg-differs.sh --refresh"
    exit 1
fi

# Extract Mt DIFFERS lines (the errors, not info St DIFFERS)
DIFFERS=$(grep "error: Mt fn" "$LOG" | grep "DIFFERS")

TOTAL=$(echo "$DIFFERS" | wc -l)

echo "================================================================="
echo "APAS-VERUS Algorithmic Differences Report"
echo "Generated: $(date '+%Y-%m-%d %H:%M')"
echo "================================================================="
echo ""
echo "Total Mt DIFFERS: $TOTAL"
echo ""

# Classify by reason pattern
echo "================================================================="
echo "1. By Category"
echo "================================================================="
echo ""

# Vec-backed (slice version exists)
VEC=$(echo "$DIFFERS" | grep -i "vec-backed\|sequential clone loop\|sequential.*loop\|sequential fold\|sequential loop\|sequential scan\|sequential.*tabulate\|sequential init\|sequential bit\|sequential word\|sequential.*insert\|sequential nested\|sequential apply" | grep -E "Chap18/ArraySeq|Chap19/ArraySeqMtEph\." | wc -l)
echo "Vec-backed sequences (slice version matches APAS):  $VEC"

# PRAM gap
PRAM=$(echo "$DIFFERS" | grep -i "PRAM\|assumes parallel\|word-AND\|word-OR\|word-AND-NOT\|bit scan\|bit set\|bit clear" | wc -l)
echo "PRAM model gap (fork-join can't match):             $PRAM"

# Parametric BST
PARA=$(echo "$DIFFERS" | grep -i "parametric" | wc -l)
echo "Parametric BST (by design):                         $PARA"

# Unordered table (array-backed)
UNORD=$(echo "$DIFFERS" | grep "Chap42/TableMtEph" | wc -l)
echo "Unordered table (array-backed, linear by design):   $UNORD"

# spec_fn not Send
SPECFN=$(echo "$DIFFERS" | grep -i "spec_fn not Send" | wc -l)
echo "spec_fn not Send (blocked on Verus):                $SPECFN"

# Sequential partition/filter in sort/select
PARTITION=$(echo "$DIFFERS" | grep -i "sequential.*partition\|sequential.*filter.*domin" | wc -l)
echo "Sequential partition dominates span:                $PARTITION"

# DP parallel
DP=$(echo "$DIFFERS" | grep -i "DP table\|DP fill\|sequential DP" | wc -l)
echo "Sequential DP (needs parallel DP):                  $DP"

# Graph operations
GRAPH=$(echo "$DIFFERS" | grep -E "Chap52|Chap62|Chap66" | wc -l)
echo "Graph operations (various):                         $GRAPH"

# AVL tree set (non-filter)
AVLSET=$(echo "$DIFFERS" | grep "Chap41/AVLTreeSet" | grep -v "filter\|PRAM\|bit\|word" | wc -l)
echo "AVL tree set (to_seq/from_seq sequential):          $AVLSET"

# Already categorized above — compute uncategorized
CATEGORIZED=$((VEC + PRAM + PARA + UNORD + SPECFN + PARTITION + DP + GRAPH + AVLSET))
UNCAT=$((TOTAL - CATEGORIZED))
echo "Other/uncategorized:                                $UNCAT"

echo ""
echo "================================================================="
echo "2. Real Targets (algorithmic fixes needed)"
echo "================================================================="
echo ""
echo "These DIFFERS represent genuine algorithmic gaps, not representation choices:"
echo ""

# Filter out the OK categories, show what's left
echo "$DIFFERS" | grep -v \
    -e "Chap18/ArraySeqMtEph\." \
    -e "Chap18/ArraySeqMtPer\." \
    -e "Chap19/ArraySeqMtEph\." \
    -e "Chap42/TableMtEph" \
    -e "ArraySetEnumMtEph" \
    -e "parametric" | \
    sed 's|.*/src/||; s|: error: Mt fn `\(.*\)` DIFFERS from APAS — : \(.*\)|\t\1\t\2|' | \
    awk -F'\t' '{printf "  %-45s %-25s %s\n", $1, $2, $3}' || true

echo ""
echo "================================================================="
echo "3. Full List"
echo "================================================================="
echo ""
echo "$DIFFERS" | sed 's|.*/src/||; s|: error: Mt fn `\(.*\)` DIFFERS from APAS — : \(.*\)|  \1  —  \2|' | \
    awk -F'  —  ' '{printf "  %-55s %s\n", $1, $2}' || true

echo ""
echo "================================================================="
echo "4. By File"
echo "================================================================="
echo ""
echo "$DIFFERS" | sed 's|.*src/||; s|:.*||' | sort | uniq -c | sort -rn | \
    awk '{printf "  %3d  %s\n", $1, $2}'
