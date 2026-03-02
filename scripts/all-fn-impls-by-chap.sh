#!/bin/bash
# Run veracity-review-module-fn-impls on each chapter.
# The tool writes .md and .json to project-root analyses/; we copy them per-chapter.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERACITY=~/projects/veracity/target/release/veracity-review-module-fn-impls
cd "$PROJECT_ROOT"

for dir in src/Chap*/; do
    mkdir -p "$dir/analyses"
    "$VERACITY" -d "$dir"
    cp analyses/veracity-review-module-fn-impls.md "$dir/analyses/"
    cp analyses/veracity-review-module-fn-impls.json "$dir/analyses/"
done
