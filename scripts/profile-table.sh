#!/bin/bash
# Generate profile summary tables from SUMMARY-*.txt files.
# Usage: scripts/profile-table.sh
#
# Prints two tables:
# 1. Per-chapter totals sorted by instantiation count
# 2. Top quantifiers across all chapters (user + vstd only, no prelude/internal)

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROFILE_DIR="$PROJECT_ROOT/logs/profile"

if ! ls "$PROFILE_DIR"/SUMMARY-Chap*.txt >/dev/null 2>&1; then
    echo "No profile summaries found in $PROFILE_DIR"
    exit 1
fi

# Use only the latest summary per chapter.
declare -A LATEST
for f in "$PROFILE_DIR"/SUMMARY-Chap*.txt; do
    chap=$(basename "$f" | sed 's/SUMMARY-//;s/-20.*//')
    LATEST[$chap]="$f"
done
FILES=("${LATEST[@]}")

# Table 1: Per-chapter totals.
echo "## Per-Chapter Instantiation Totals"
echo ""
echo "| # | Chapter | Instantiations | Top Quantifier |"
echo "|---|---------|---------------|----------------|"

for f in "${FILES[@]}"; do
    chap=$(basename "$f" | sed 's/SUMMARY-//;s/-20.*//')

    total=$(grep "^=== Per-module totals" -A 999 "$f" \
        | grep -E "^\s+[0-9]" \
        | awk '{s+=$1} END {print s+0}')

    top=$(grep "^=== Grand top quantifiers" -A 999 "$f" \
        | grep -E "^\s+[0-9]" \
        | grep -v "prelude_\|constructor_\|internal_" \
        | head -1 \
        | sed 's/^\s*//' \
        | awk '{printf "%s %s", $1, $2}')

    [ -z "$top" ] && top="(prelude only)"

    printf "| 0 | %s | %s | %s |\n" "$chap" "$total" "$top"
done | sort -t'|' -k4 -rn | awk -F'|' 'BEGIN{n=0} {n++; printf "| %d |%s|%s|%s|\n", n, $3, $4, $5}'

# Table 2: Top quantifiers across all chapters.
echo ""
echo "## Top Quantifiers Across All Chapters (user + vstd)"
echo ""
echo "Units: quantifier instantiation counts (number of times Z3 fired each quantifier)."
echo ""
echo "| # | Instantiations | Quantifier |"
echo "|---|---------------|------------|"

for f in "${FILES[@]}"; do
    grep "^=== Grand top quantifiers" -A 999 "$f" \
        | grep -E "^\s+[0-9]" \
        | grep -v "prelude_\|constructor_\|internal_"
done \
    | awk '{
        # Strip trailing _NN numeric ID to group by short name.
        name = $2
        gsub(/_[0-9]+$/, "", name)
        counts[name] += $1
    }
    END {
        for (n in counts) printf "%10d  %s\n", counts[n], n
    }' \
    | sort -rn \
    | head -30 \
    | awk 'BEGIN{n=0} {n++; printf "| %d | %s | %s |\n", n, $1, $2}'
