# R89 — Integerize Example56_1 + Example56_3, STEP 10

## Objective

Fix Example56_1.rs and Example56_3.rs in Chap56. They are commented out as BROKEN
because they used `ordered_float` (removed). Convert their float usage to integer (i64).

## CRITICAL: Work from the working integer code

PathWeightUtilsStEph.rs already has BOTH `path_weight_int` (working, i64) and
`path_weight_float` (WrappedF64, may have issues). The examples should use
`path_weight_int` exclusively.

Read these first:
- `src/Chap56/PathWeightUtilsStEph.rs` — has both int and float path weight functions
- `src/Chap56/SSSPResultStEphI64.rs` — working integer SSSP result for pattern reference

## What to fix

### Example56_1.rs

1. Remove `use crate::vstdplus::float::float::*;`
2. `example_path_weight_int` — already uses i64, probably fine
3. `example_path_weight_float` — convert to i64:
   - Replace `dist(2.5)` etc with integer weights like `3i64`, `5i64`
   - Replace `unreachable_dist()` with `i64::MAX` or a sentinel
   - Call `path_weight_int` instead of `path_weight_float`
   - Rename to `example_path_weight_i64` or just fold into the int example
4. `example_negative_weights` — already uses i64, probably fine

### Example56_3.rs

Same pattern. Convert any float usage to i64. These are Example files — the
textbook demonstrations just need to compile and run, they don't need deep proofs.

## lib.rs — uncomment your files

Uncomment BOTH files in lib.rs. They are currently commented out with `// BROKEN`.

## Isolation

```bash
scripts/validate.sh isolate Chap56
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- These are Example files — keep them simple demonstration code.
- Do NOT modify PathWeightUtilsStEph.rs or any other Chap56 file.
- Do NOT add assume or accept.
- The functions are `#[verifier::external]` (I/O demonstration) — that's fine, leave them.
- If `path_weight_float` is called nowhere else after this change, leave it alone
  (don't delete it from PathWeightUtils).

## STEP 10

## Report

Write `plans/agent-r89-example56-report.md`.
