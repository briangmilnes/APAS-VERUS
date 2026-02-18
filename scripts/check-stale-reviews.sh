#!/usr/bin/env bash
set -euo pipefail

# Check which chapters have stale review-against-prose files.
# A review is stale if any of its inputs are newer than it.
#
# Usage:
#   scripts/check-stale-reviews.sh              # check all chapters
#   scripts/check-stale-reviews.sh 56 57 59     # check specific chapters

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

CHAPTERS=("$@")

if [ ${#CHAPTERS[@]} -eq 0 ]; then
    CHAPTERS=($(ls -d src/Chap*/analyses/review-against-prose.md 2>/dev/null | sed 's|src/Chap\([0-9]*\)/.*|\1|' | sort -n))
fi

stale=0
ok=0

for ch in "${CHAPTERS[@]}"; do
    review="src/Chap${ch}/analyses/review-against-prose.md"
    if [ ! -f "$review" ]; then
        echo "Chap${ch}: NO REVIEW"
        continue
    fi

    inputs=()
    for f in src/Chap${ch}/*.rs; do [ -f "$f" ] && inputs+=("$f"); done
    [ -f "prompts/Chap${ch}.txt" ] && inputs+=("prompts/Chap${ch}.txt")
    [ -f "src/Chap${ch}/analyses/veracity-review-verus-proof-holes.log" ] && inputs+=("src/Chap${ch}/analyses/veracity-review-verus-proof-holes.log")
    for f in tests/test_Chap${ch}*.rs; do [ -f "$f" ] && inputs+=("$f"); done
    for f in tests/Chap${ch}/*.rs; do [ -f "$f" ] && inputs+=("$f"); done

    if [ ${#inputs[@]} -eq 0 ]; then
        ok=$((ok + 1))
        continue
    fi

    changed=$(find "${inputs[@]}" -newer "$review" 2>/dev/null | head -5)

    if [ -n "$changed" ]; then
        echo "Chap${ch}: STALE"
        echo "$changed" | sed 's/^/  /'
        stale=$((stale + 1))
    else
        ok=$((ok + 1))
    fi
done

echo ""
echo "Summary: ${stale} stale, ${ok} up to date"
