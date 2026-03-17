# Fix: veracity-review-proof-holes Default Excludes

## Problem

Running `veracity-review-proof-holes` against `~/projects/APAS-VERUS`
without explicit `-e` flags produces 259 holes. The correct count is
~159. The `scripts/all-holes-by-chap.sh` gets the right answer because
it passes:

```bash
-e benches -e tests -e rust_verify_test \
-e src/vstdplus -e src/standards -e src/experiments
```

When agents or users run the tool bare (`veracity-review-proof-holes .`),
they get inflated numbers because vstdplus/experiments/standards are
included.

## Root Cause

vstdplus contains 100 foundation holes (admit, assume_specification,
external_body for std library bridges). These are infrastructure, not
algorithmic proof work. experiments/ and standards/ contain test code.

## Fix

Add default excludes to the tool so that known non-algorithmic
directories are excluded automatically. After parsing user `-e` flags,
append defaults for directories that exist:

- `experiments`, `src/experiments`
- `standards`, `src/standards`
- `vstdplus`, `src/vstdplus`

User `-e` flags should ADD to (not replace) these defaults. The
defaults should only apply when the directory actually exists in the
target path.

## Validation

After fix, running bare against `~/projects/APAS-VERUS`:
- Holes Found: ~159 (not 259+)
- Warnings: ~40
- Accepted: ~327
- Structural FPs: ~24

The per-chapter scripts (`scripts/holes.sh src/ChapNN/`) should be
unaffected since they target specific chapter directories.
