# Agent 3 — Round 44 Report

## Assignment

Prove external_body functions in Chap65 (Kruskal + Prim MST) and Chap66 (Boruvka parallel MST).
Fix 5 warnings across the two chapters.

## Results Summary

- Verification: 4366 verified, 0 errors
- RTT: 2613 passed, 0 failed
- Holes: 125 → 124 (net -1)
- Warnings fixed: 4 of 5 (1 left is pq_entry_new fn_missing_requires — genuinely no precondition)

## Holes by File

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 65 | KruskalStEph.rs | 1 | 1 | 0 | kruskal_mst proved; sort helper added |
| 2 | 65 | PrimStEph.rs | 1 | 1 | 0 | prim_mst too complex to remove |
| 3 | 65 | UnionFindStEph.rs | 0 | 0 | 0 | Clean |
| 4 | 66 | BoruvkaMtEph.rs | 10 | 9 | -1 | boruvka_mst_mt_with_seed proved |
| 5 | 66 | BoruvkaStEph.rs | 0 | 0 | 0 | Clean |

## What Was Done

### Chap65/KruskalStEph.rs: kruskal_mst proved (net 0 holes)

Removed `external_body` from `kruskal_mst`. The original used `.iter().cloned().collect()`,
`for` loops over iterators, and `SetLit![]` — all cfg-gated out in Verus mode.

Rewrote with:
- Manual vertex insertion loop with UF wf invariant tracking `uf@.parent.contains_key`
- Manual edge collection loop preserving element identity via ghost sequence
- Factored `sort_by` closure into `sort_edges_by_weight` external_body helper with tight ensures
  (`edges@.len() == old(edges)@.len()`, permutation containment)
- Index-based greedy loop with proof block connecting edge provenance through sort to graph wf
  using `choose` operator

Added `ensures mst_edges.spec_setsteph_wf()` — flows from `SetStEph::empty()` wf + `insert` preserving wf.

Net 0 holes: removed kruskal_mst external_body (-1), added sort_edges_by_weight (+1).
Structural win: all algorithmic logic now verified; only sort closure remains external_body.

### Chap66/BoruvkaMtEph.rs: boruvka_mst_mt_with_seed proved (-1 hole)

Removed `external_body`. Replaced:
- `SetLit![]` → `SetStEph::empty()` (verified)
- `.iter().cloned().collect()` → manual iterator loops with `iter_invariant` and ghost sequences
- Added `requires vertices.spec_setsteph_wf(), edges.spec_setsteph_wf()`

Also removed `#[cfg(not(verus_keep_ghost))]` from `use std::sync::Arc` — validates clean.

### Warnings Fixed (4 of 5)

| # | Chap | File | Function | Warning | Fix |
|---|------|------|----------|---------|-----|
| 1 | 65 | KruskalStEph.rs | kruskal_mst | fn_missing_ensures | Added `ensures mst_edges.spec_setsteph_wf()` |
| 2 | 65 | KruskalStEph.rs | mst_weight | fn_missing_ensures | Added `ensures mst_edges@.len() == 0 ==> total@ == 0.0f64` |
| 3 | 65 | PrimStEph.rs | mst_weight | fn_missing_ensures | Same ensures pattern |
| 4 | 66 | BoruvkaMtEph.rs | mst_weight | fn_missing_ensures | Same ensures pattern |
| 5 | 65 | PrimStEph.rs | pq_entry_new | fn_missing_requires | Not fixed — genuinely no precondition |

### Not Attempted / Assessed

**prim_mst (Chap65):** External_body covers entire function with nested loops, PQ overflow
bounds, graph operation preconditions. The function uses `HashSetWithViewPlus` for visited
tracking and `BinaryHeapPQ` which lacks the ensures needed to prove PQ-based invariants.
Too complex for this round.

**9 remaining Boruvka Mt holes (Chap66):** All 7 leaf functions use `ParaPair!` which is
cfg-gated with `#[cfg(not(verus_keep_ghost))]` — they compile to nothing in Verus mode.
The 2 orchestrators (`bridge_star_partition_mt`, `boruvka_mst_mt`) call these external_body
functions and use iterator chains (`.keys().cloned().collect()`,
`Arc::try_unwrap().unwrap_or_else(closure)`) not expressible in Verus. These are genuinely
blocked by the cfg-gated ParaPair! macro.

## Remaining Warnings

| # | Chap | File | Warning | Status |
|---|------|------|---------|--------|
| 1 | 65 | PrimStEph.rs | pq_entry_new fn_missing_requires | No real precondition exists |
| 2 | 66 | BoruvkaMtEph.rs | boruvka_mst_mt_with_seed fn_missing_ensures | Can't prove — calls external_body boruvka_mst_mt |
| 3 | 66 | BoruvkaMtEph.rs | 2x assume_eq_clone_workaround | Standard workaround pattern |

## Techniques Used

- Manual iterator loops replacing `.iter().cloned().collect()` (Verus iterator standard)
- Ghost sequence tracking with `iter_invariant` predicate
- `choose` operator for existential witness in provenance proofs
- Sort factored into external_body helper with permutation ensures
- `SetStEph::empty()` replacing cfg-gated `SetLit![]`
