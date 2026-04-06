# R151 Agent 2 — Fix Mt Trait Bounds to Use StTInMtT. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Pay close attention to `mt_type_bounds_standard.rs`.

Report file: `plans/r151-agent2-sttinmtt-bounds-report.md`

## Problem

Veracity rule [23b] flags 15 instances where free functions in Mt modules require
`Send + Sync + 'static` but the trait only says `T: Eq + Clone` or similar
partial bounds. The Mt type bounds standard says: use `StTInMtT` (which expands
to `StT + Send + Sync + 'static`), not raw bounds.

## Files (13 files, 15 warnings)

```
Chap18/ArraySeqMtEph.rs
Chap18/ArraySeqMtEphSlice.rs
Chap19/ArraySeqMtEphSlice.rs
Chap27/ReduceContractMtEph.rs
Chap35/OrderStatSelectMtEph.rs
Chap35/OrderStatSelectMtPer.rs
Chap36/QuickSortMtEphSlice.rs
Chap39/BSTParaTreapMtEph.rs
Chap39/BSTSetTreapMtEph.rs
Chap49/MinEditDistMtEph.rs
Chap49/MinEditDistMtPer.rs
Chap49/SubsetSumMtEph.rs
Chap49/SubsetSumMtPer.rs
```

## What to do

For each file:

1. Find the trait declaration
2. Look at the type parameter bounds — they'll have partial bounds like
   `T: Eq + Clone` or `T: Clone + Eq + Send + Sync + 'static`
3. Replace with the appropriate alias from `src/Types.rs` / `src/Concurrency.rs`:
   - `Eq + Clone + Send + Sync + 'static` → `StTInMtT`
   - `Eq + Clone` on an Mt file → `StTInMtT` (the free fns already need it)
4. Check if `StTInMtT` is imported — it comes from `use crate::Types::Types::*;`
   which most files already have. If not, add it.
5. Update the impl block bounds to match if needed.
6. Validate with `scripts/validate.sh isolate ChapNN`

Example:

```rust
// Before:
pub trait ArraySeqMtEphSliceTrait<T: Eq + Clone>: Sized {

// After:
pub trait ArraySeqMtEphSliceTrait<T: StTInMtT>: Sized {
```

## Caution: Chap35/36 overlap with Agent 1

Agent 1 is adding `+ TotalOrder` to traits in Chap35 and Chap36. You are
changing the base bound (e.g., `Eq + Clone` → `StTInMtT`). If you both
modify the same trait, the merge will conflict. To minimize conflicts:

- For Chap35/36 files that need BOTH changes, apply both: `T: StTInMtT + Ord + TotalOrder`
- Read the current trait bound before editing — Agent 1 may have already
  touched it.

## Caution: StTInMtT is a superset

`StTInMtT` = `StT + Send + Sync + 'static` = `Eq + PartialEq + Clone +
Display + Debug + Sized + View + Send + Sync + 'static`. This is STRICTLY
more than `Eq + Clone`. Adding `Display + Debug + View` to the bound means
callers must satisfy those. Check:

- All callers already use concrete types that satisfy StTInMtT (u64, usize,
  String, etc.) — this is almost always true in Mt modules.
- If a caller uses a type that doesn't satisfy the new bound, the compiler
  will tell you. Fix the caller's bound too.

## Validation

Run `scripts/validate.sh isolate ChapNN` after each chapter. Run full
`scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or specs.
- Do NOT weaken ensures.
- All existing RTTs must pass.

## When done

RCP.
