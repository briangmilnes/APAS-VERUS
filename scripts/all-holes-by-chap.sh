#!/bin/bash
# Run veracity-review-proof-holes on each chapter directory, one at a time.
# Produces a per-chapter summary at the end.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERACITY=~/projects/veracity/target/release/veracity-review-proof-holes

cd "$PROJECT_ROOT"

total_clean=0
total_holed=0

for dir in src/Chap*/; do
    chap="$(basename "$dir")"
    output=$("$VERACITY" -d "$dir" 2>&1) || true

    clean=$(echo "$output" | grep -oP '\d+(?= clean \(no holes\))' || echo 0)
    holed=$(echo "$output" | grep -oP '\d+(?= holed \(contains holes\))' || echo 0)
    holes=$(echo "$output" | grep -oP 'Holes Found: \K\d+' || echo 0)

    if [ "$holed" -eq 0 ] && [ "$holes" -eq 0 ]; then
        printf "✅ %-12s  %s clean, 0 holes\n" "$chap" "$clean"
    else
        printf "❌ %-12s  %s clean, %s holed, %s holes\n" "$chap" "$clean" "$holed" "$holes"
    fi

    total_clean=$((total_clean + clean))
    total_holed=$((total_holed + holed))
done

echo ""
echo "Total: $total_clean clean, $total_holed holed"
