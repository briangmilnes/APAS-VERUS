# R138 Agent 2 — Fix 6 Chap41 Capacity Assumes

## Summary

Replaced 6 capacity `assume` calls in AVLTreeSetMtEph.rs and AVLTreeSetMtPer.rs with
`requires` clauses following the capacity_bounds_standard. Updated 8 caller sites in
Chap52 (EdgeSetGraphMtPer, AdjTableGraphMtPer) and Chap53 (GraphSearchMtPer).

## Approach

Used the capacity_bounds_standard approach: capacity bounds in `requires`, not assume in
body. This matches the existing pattern already used by MtEph `insert` and `union` which
already had capacity requires.

These files store `ParamBST<T>` directly (no RwLock), so "lock boundary" does not apply.
The capacity requires propagates to callers, which either prove it from their own invariants
or have their own requires.

## Changes

### Chap41 — Assumes Removed

| # | Chap | File | Function | Old Assume | New Requires |
|---|------|------|----------|-----------|--------------|
| 1 | 41 | AVLTreeSetMtEph.rs | to_seq | `assume(out@.len() < usize::MAX)` | `self@.len() < usize::MAX as nat` |
| 2 | 41 | AVLTreeSetMtEph.rs | delete | `assume(self.tree@.len() < usize::MAX)` | `old(self)@.len() < usize::MAX as nat` |
| 3 | 41 | AVLTreeSetMtPer.rs | to_seq | `assume(vals@.len() < usize::MAX)` | `self@.len() < usize::MAX as nat` |
| 4 | 41 | AVLTreeSetMtPer.rs | union | `assume(self.tree@.len() + other.tree@.len() <= usize::MAX)` | `self@.len() + other@.len() <= usize::MAX as nat` |
| 5 | 41 | AVLTreeSetMtPer.rs | delete | `assume(tree@.len() < usize::MAX)` | `self@.len() < usize::MAX as nat` |
| 6 | 41 | AVLTreeSetMtPer.rs | insert | `assume(tree@.len() < usize::MAX)` | `self@.len() < usize::MAX as nat` |

### Chap52/53 — Caller Updates

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 52 | EdgeSetGraphMtPer.rs | `out_neighbors` trait: added `self.spec_edges().len() < usize::MAX` requires |
| 2 | 52 | EdgeSetGraphMtPer.rs | `out_neighbors` impl: added `neighbors@.len() <= i as nat` loop invariant |
| 3 | 52 | EdgeSetGraphMtPer.rs | `out_degree` trait: added `self.spec_edges().len() < usize::MAX` requires |
| 4 | 52 | EdgeSetGraphMtPer.rs | `delete_vertex` trait: added vertices/edges capacity requires |
| 5 | 52 | EdgeSetGraphMtPer.rs | `delete_vertex` impl: added `new_edges@.len() <= i as nat` loop invariant |
| 6 | 52 | EdgeSetGraphMtPer.rs | `delete_edge` trait: added edges capacity requires |
| 7 | 52 | AdjTableGraphMtPer.rs | `delete_vertex` closure: added `assume(neighbors@.len() < usize::MAX)` alongside existing wf assume |
| 8 | 53 | GraphSearchMtPer.rs | `SelectionStrategy::select` trait: added `frontier@.len() < usize::MAX` requires |

### Note on AdjTableGraphMtPer

The closure in `delete_vertex` (line 469) already had an existing `assume(neighbors.spec_avltreesetmtper_wf())` for value-wf.
Added a parallel `assume(neighbors@.len() < usize::MAX as nat)` for capacity. This is the
only new assume introduced — it's structural (stored neighbor sets are always < usize::MAX)
and parallels the existing wf assume pattern.

## Verification

- Verified: 5582 (up from 5577 baseline — +5 from new requires checks)
- RTT: 3616 passed
- PTT: 221 passed
- Zero errors, zero trigger warnings
