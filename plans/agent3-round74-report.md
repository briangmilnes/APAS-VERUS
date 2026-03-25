# Agent 3 — Round 74 Report

## Summary

Eliminated **17 assume holes** across 3 files. All 17 were actionable proof targets;
all 17 are now proven or replaced with proper requires propagation.

- **Validation**: 4735 verified, 0 errors
- **RTT**: 2619 passed, 0 skipped
- **PTT**: 157 passed, 0 skipped

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | BSTSetPlainMtEph.rs | 6 | 0 | -6 |
| 2 | 37 | BSTSetBBAlphaMtEph.rs | 6 | 0 | -6 |
| 3 | 41 | AVLTreeSetMtPer.rs | 5 | 0 | -5 |
| | | **Total** | **17** | **0** | **-17** |

## Techniques Used

### BSTSetPlainMtEph.rs and BSTSetBBAlphaMtEph.rs (12 holes, identical pattern)

**Problem**: After `ParaPair!` macro calls in `union`, `intersection`, and `difference`,
6 assumes each asserted `r.spec_wf()` for the two parallel results. The ParaPair macro
propagates closure ensures via `f1.ensures((), pair.0)`, but inline closures had no
explicit ensures clause — so wf wasn't available after the join.

**Fix**: Replaced inline closures with named closures carrying explicit `ensures r.spec_wf()`:

```rust
let f1 = move || -> (r: Self)
    ensures r.spec_bstsetplainmteph_wf()
{ self_left.union(&other_left) };
let f2 = move || -> (r: Self)
    ensures r.spec_bstsetplainmteph_wf()
{ self_right.union(&other_right) };
let Pair(left_union, right_union) = crate::ParaPair!(f1, f2);
// No assumes needed — wf flows through ParaPair ensures.
```

### AVLTreeSetMtPer.rs (5 holes)

**Problem 1 — Type axioms (4 holes)**: `from_seq` and `find` assumed
`obeys_cmp_spec::<T>()` and `view_ord_consistent::<T>()`. These have broadcast proofs
only for primitive types, not generic T.

**Fix**: Propagated as `requires` clauses on the trait methods. Only callers are RTT
test files (no verified code callers), so no proof obligations were broken.

**Problem 2 — Capacity bound (1 hole)**: `from_seq` assumed
`st@.len() + 1 < usize::MAX`. The `values_in_order()` ensures only `true`, so
seq length can't be bounded at spec level.

**Fix**: Added runtime capacity guard `if n > usize::MAX - 2 { return empty }` plus
loop invariant `st@.len() <= i as nat, n <= usize::MAX - 2`.

## Remaining Holes in Assigned Files

All assigned holes are eliminated. Remaining assumes in AVLTreeSetMtPer.rs are
pre-existing `structural_false_positive RWLOCK_GHOST` boundary assumes (25 total)
which were not part of this assignment.
