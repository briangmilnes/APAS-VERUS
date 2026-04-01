# R127 Agent 3 Report — Parallelize Chap18 ArraySeqMtPer

## Summary

Added parallel D&C helper methods (`map_inner`, `filter_inner`, `reduce_inner`,
`tabulate_inner`) to `ArraySeqMtPerS<T>` bare impl block. These use fork-join
parallelism via `join()` with named closures, following the existing `map_par`,
`filter_par`, `reduce_par` pattern.

Trait methods remain sequential because Rust's trait system requires impl method
bounds to match the trait declaration. Adding `Clone + Send + Sync + 'static`
to trait closure parameters would break all callers that capture local references
(21 compilation errors across 10+ files). The `_inner` helpers have the full
`Send + Sync + 'static` bounds needed for `join()`.

Also added `clone_fn_usize` to `vstdplus/clone_plus.rs` for cloning
`Fn(usize) -> T` closures with spec preservation.

## Functions parallelized

| # | Chap | File | Function | Parallel Helper | Old Span | New Span | Status |
|---|------|------|----------|-----------------|----------|----------|--------|
| 1 | 18 | ArraySeqMtPer.rs | map | map_inner | O(n) | O(log n) | verified |
| 2 | 18 | ArraySeqMtPer.rs | filter | filter_inner | O(n) | O(log n) | verified (no multiset ensures) |
| 3 | 18 | ArraySeqMtPer.rs | reduce | reduce_inner | O(n) | O(log n) | verified |
| 4 | 18 | ArraySeqMtPer.rs | tabulate | tabulate_inner | O(n) | O(log n) | verified |

## Functions not parallelized

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 5 | 18 | ArraySeqMtPer.rs | subseq | Vec-backed: O(1) requires tree representation |
| 6 | 18 | ArraySeqMtPer.rs | append | Vec-backed: O(1) requires tree representation |
| 7 | 18 | ArraySeqMtPer.rs | update | Vec-backed: O(1) requires tree representation |
| 8 | 18 | ArraySeqMtPer.rs | inject | Requires sort-by-position for parallel apply |
| 9 | 18 | ArraySeqMtPer.rs | scan | Requires upsweep/downsweep parallel prefix sum |
| 10 | 18 | ArraySeqMtPer.rs | flatten | D&C requires outer array cloning proof incompatible with subseq_copy |

## Code review annotations updated

All 10 DIFFERS functions had their annotations updated with specific explanations
for why they differ from APAS (Vec-backed vs tree representation, sequential
algorithm constraints, etc.).

## Verification

- Isolate Chap18: **1016 verified, 0 errors**
- RTT: **3533 tests passed**

## Notes

- Trait methods stay sequential; `_inner` helpers provide parallel implementations.
  Callers choose: use the trait method for general-purpose (any closure), or use
  the `_inner` helper for parallel execution (requires `Clone + Send + Sync + 'static`).
- The existing `map_par`, `filter_par`, `reduce_par` remain as-is. The new `_inner`
  helpers have richer ensures (per-element `f.ensures` for map, monoid fold for reduce).
- `filter_inner` does NOT include the multiset ensures from the trait's `filter`.
  The multiset proof requires sequential loop invariant structure. Callers needing
  multiset guarantees should use the trait's sequential `filter`.
