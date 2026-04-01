# R128 Agent 1 Report — Parallelize Chap18 MtPer map + reduce

## Summary

Wired the `map` and `reduce` trait methods in `ArraySeqMtPerRedefinableTrait` to
use divide-and-conquer via `join()`, matching the MtEph pattern from R127.

## Changes

### Chap18/ArraySeqMtPer.rs

| # | Chap | Method | Change |
|---|------|--------|--------|
| 1 | 18 | `reduce` trait decl | Added `F: Clone + Send + Sync + 'static`, `T: Eq + Send + Sync + 'static`, `obeys_feq_clone::<T>()` to requires |
| 2 | 18 | `reduce` trait impl | Replaced sequential fold with delegation to `reduce_inner` (D&C via join); handles empty case inline |
| 3 | 18 | `map` trait decl | Added `U: Eq + Send + Sync + 'static`, `F: Clone + Send + Sync + 'static`, `T: Clone + Eq + Send + Sync + 'static`, `obeys_feq_clone` requires |
| 4 | 18 | `map` trait impl | Replaced sequential loop with delegation to `map_inner` (D&C via join) |
| 5 | 18 | annotations | Updated Code review annotations: reduce Span O(n)→O(lg n), map Span O(n)→O(lg n) |

### Chap26/DivConReduceMtPer.rs

| # | Chap | Change |
|---|------|--------|
| 6 | 26 | Added 5 `external_body` bridge functions (`call_reduce_max`, `call_reduce_sum`, `call_reduce_product`, `call_reduce_or`, `call_reduce_and`) |
| 7 | 26 | Rewired trait impls to call bridge functions instead of inline closures |
| 8 | 26 | Preserved proof block in `max_element_parallel` (fold_left bounds + achievability) |

**Why the bridges**: Verus cannot verify `Clone` for closure or function-item types.
The `reduce` trait now requires `F: Clone + Send + Sync + 'static` for fork-join
parallelism. DivConReduceMtPer passes closures to `reduce`, but Verus rejects the
Clone bound check. The external_body bridges isolate this type-system limitation —
they're thread-spawn boundary wrappers, not algorithmic logic. The ensures specs are
tight (reduce's own postcondition guarantees the fold_left result).

## Holes

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 18 | ArraySeqMtPer.rs | 0 | 0 | 0 |
| 2 | 26 | DivConReduceMtPer.rs | 0 | 10 | +10 |

Chap26 delta breakdown: 5 external_body + 5 assume_new (inside external_body bodies).

## Verification

- **Validate**: 5504 verified, 0 errors
- **RTT**: 3534 passed, 0 skipped
- **PTT**: 221 passed, 0 skipped

## Techniques

- D&C delegation: trait methods delegate to existing `_inner` bare impl methods
- External_body bridge: isolates Verus Clone limitation at call sites
- Ghost conjunction: empty-case proof for reduce uses extensional equality + fold_left fuel

## Remaining work

The 10 holes in Chap26/DivConReduceMtPer are from Verus's inability to verify Clone
for function/closure types. These cannot be eliminated without either:
1. Verus adding Clone recognition for closure types, or
2. Restructuring DivConReduceMtPer to avoid calling `reduce` directly (e.g., inline D&C)
