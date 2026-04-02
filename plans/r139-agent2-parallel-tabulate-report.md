# R139 Agent 2 Report — Parallel Tabulate for ArraySeqMtEphSlice

## Summary

Implemented parallel `tabulate` for `ArraySeqMtEphSliceS<T>` using D&C with `join()`,
following the existing `map_dc_vec`/`filter_dc_vec` pattern in the same file.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 19 | ArraySeqMtEphSlice.rs | Added `tabulate` to trait + impl |
| 2 | 19 | ArraySeqMtEphSlice.rs | Added `tabulate_dc_vec` D&C helper |
| 3 | 19 | TestArraySeqMtEphSlice.rs | Added 6 RTTs for tabulate |

## Implementation

- **Trait method**: `tabulate<F: MtTabulateFn<T>>(f: &F, length: usize) -> Self`
  - Requires: `obeys_feq_clone::<T>()`, `f.requires` for all indices
  - Ensures: wf, length matches, element-level `f.ensures` for each index
- **D&C helper**: `tabulate_dc_vec(f, start, count) -> Vec<T>`
  - Base: empty (count=0), singleton (count=1, call `f(start)`)
  - Recursive: split at midpoint, `join()` both halves, concatenate Vecs
  - Uses `clone_fn_usize` to clone the tabulate function for both arms
  - Full element-level proof through the combine loop
- **Alg Analysis**: Work O(n * W(f)), Span O(lg n * S(f)) — D&C + join, O(n) rejoin

## RTTs Added

| # | Test | What it checks |
|---|------|---------------|
| 1 | test_tabulate_empty | length 0 |
| 2 | test_tabulate_singleton | length 1, correct value |
| 3 | test_tabulate_small | 5 elements, i^2 values |
| 4 | test_tabulate_identity | 100 elements, identity function |
| 5 | test_tabulate_large | 10000 elements, sum via reduce |
| 6 | test_tabulate_then_map | tabulate + map composition |

## Verification

- Isolate Chap19: 865 verified, 0 errors (6s)
- Full: 5589 verified, 0 errors (111s)
- RTT: 3622 passed, 0 skipped
- PTT: 221 passed, 0 skipped

## Techniques

- Named closures with explicit requires/ensures for join arms
- Ghost `v_view` binding to preserve Vec view across `from_vec` move
- Three-part loop invariant: left elements proven, right elements copied, right source proven
- Final `assert forall` proof block connecting combined Vec to spec
