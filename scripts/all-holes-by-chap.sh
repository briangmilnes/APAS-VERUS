#!/bin/bash
# Run holes.sh on each chapter, writing output to each chapter's analyses/ log.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

for dir in src/Chap*/; do
    mkdir -p "$dir/analyses"
    scripts/holes.sh "$dir" | tee "$dir/analyses/veracity-review-verus-proof-holes.log"
done
