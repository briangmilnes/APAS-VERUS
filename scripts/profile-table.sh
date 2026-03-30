#!/bin/bash
# Generate profile summary table from SUMMARY-*.txt files.
# Usage: scripts/profile-table.sh
#
# Reads logs/profile/SUMMARY-Chap*.txt, extracts total instantiations
# per chapter, and prints a markdown table sorted by instantiation count.

set -uo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROFILE_DIR="$PROJECT_ROOT/logs/profile"

if ! ls "$PROFILE_DIR"/SUMMARY-Chap*.txt >/dev/null 2>&1; then
    echo "No profile summaries found in $PROFILE_DIR"
    exit 1
fi

echo "| # | Chapter | Instantiations | Top Quantifier |"
echo "|---|---------|---------------|----------------|"

N=0
for f in "$PROFILE_DIR"/SUMMARY-Chap*.txt; do
    chap=$(basename "$f" | sed 's/SUMMARY-//;s/-20.*//')

    # Total instantiations: sum the Per-module totals section.
    total=$(grep "^=== Per-module totals" -A 999 "$f" \
        | grep -E "^\s+[0-9]" \
        | awk '{s+=$1} END {print s+0}')

    # Top user quantifier (skip prelude/internal/constructor).
    top=$(grep "^=== Grand top quantifiers" -A 999 "$f" \
        | grep -E "^\s+[0-9]" \
        | grep -v "prelude_\|constructor_\|internal_" \
        | head -1 \
        | sed 's/^\s*//' \
        | awk '{printf "%s %s", $1, $2}')

    [ -z "$top" ] && top="(prelude only)"

    N=$((N + 1))
    printf "| %d | %s | %s | %s |\n" "$N" "$chap" "$total" "$top"
done | sort -t'|' -k4 -rn | awk -F'|' 'BEGIN{n=0} {n++; printf "| %d %s|%s|%s|%s|\n", n, "", $3, $4, $5}'
