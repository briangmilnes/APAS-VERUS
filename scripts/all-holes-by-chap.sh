#!/bin/bash
# Run proof holes on each chapter and at the top level.
# Per-chapter logs go to src/ChapNN/analyses/.
# Top-level log goes to src/analyses/.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERACITY=~/projects/veracity/target/release/veracity-review-proof-holes

cd "$PROJECT_ROOT"

# Per-chapter runs.
for dir in src/Chap*/; do
    mkdir -p "$dir/analyses"
    "$VERACITY" "$dir" | tee "$dir/analyses/veracity-review-verus-proof-holes.log"
done

# Top-level run with excludes.
mkdir -p src/analyses
"$VERACITY" \
    -e benches -e tests -e rust_verify_test \
    -e src/vstdplus -e src/standards -e src/experiments \
    "$PROJECT_ROOT" \
    | tee src/analyses/veracity-review-verus-proof-holes.log
