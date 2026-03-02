#!/bin/bash
# Run veracity-review-verus-style on each chapter.
# The tool writes its own log to $dir/analyses/veracity-review-verus-style.log.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERACITY=~/projects/veracity/target/release/veracity-review-verus-style
cd "$PROJECT_ROOT"

for dir in src/Chap*/; do
    mkdir -p "$dir/analyses"
    "$VERACITY" -c "$PROJECT_ROOT" "$dir"
done
