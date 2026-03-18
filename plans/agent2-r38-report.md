# Agent 2 — R38 Report: Chap43 Ordered Set/Table Proofs

## Results Summary

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Verified | 4332 | 4334 | +2 |
| Holes | 15 | 9 | -6 |
| Warnings | 5 | 10 | +5 (reclassified) |
| RTT | 2613 pass | 2613 pass | 0 |

## Holes Closed (6)

| # | Chap | File | Hole Type | Technique |
|---|------|------|-----------|-----------|
| 1 | 43 | OrderedTableMtEph.rs | external_body (rank_key) | Rewrote body with while-loop + invariants; proved cardinality via `lemma_map_size` + `injective_on` |
| 2 | 43 | OrderedTableStEph.rs | assume (rank_key cardinality) | Replaced `assume(less_keys.len() == less_idx.len())` with `lemma_map_size` proof |
| 3 | 43 | OrderedTableStPer.rs | assume (rank_key cardinality) | Same technique as StEph |
| 4 | 43 | OrderedSetStEph.rs | assume (to_seq clone/view bridge) | Used `clone_plus()` + `lemma_cloned_view_eq` in loop invariant |
| 5 | 43 | OrderedTableStEph.rs | assume(false) (select_key pigeonhole) | Full pigeonhole proof: rank injectivity via TotalOrder + `lemma_subset_not_in_lt`, then `set_int_range` + `lemma_map_size` + subset-size equality |
| 6 | 43 | OrderedTableStPer.rs | assume(false) (select_key pigeonhole) | Same pigeonhole technique as StEph |

## Remaining 9 Holes

| # | Chap | File | Hole Type | Blocker |
|---|------|------|-----------|---------|
| 1 | 43 | AugOrderedTableMtEph.rs | assume (closure requires) | Cascading trait change needed across all callers |
| 2 | 43 | AugOrderedTableMtEph.rs | external_body (reduce_range_parallel) | Complex parallel reduction |
| 3 | 43 | AugOrderedTableStPer.rs | assume (closure clone totality) | Verus limitation: no `clone` spec for closures |
| 4 | 43 | OrderedSetMtEph.rs | external_body (to_seq) | RwLock ghost view bridge (inner@ == self@) |
| 5 | 43 | OrderedSetStEph.rs | assume (select filter cardinality) | Requires AVL sortedness invariant (Chap41) |
| 6 | 43 | OrderedSetStPer.rs | assume (select filter cardinality) | Requires AVL sortedness invariant (Chap41) |
| 7 | 43 | OrderedTableMtEph.rs | external_body (select_key) | Uses Vec::sort() (no Verus spec) |
| 8 | 43 | OrderedTableStEph.rs | external_body (collect) | Uses sort_by (no Verus spec) |
| 9 | 43 | OrderedTableStPer.rs | external_body (collect) | Uses sort_by (no Verus spec) |

## Key Techniques

1. **Cardinality via `lemma_map_size`**: The index-to-key-view map is injective
   (from `spec_keys_no_dups`), so `less_idx.map(f).len() == less_idx.len()`.
   Used in rank_key across MtEph, StEph, and StPer.

2. **Clone/view bridging via `clone_plus` + `lemma_cloned_view_eq`**: Replace `.clone()`
   with `.clone_plus()`, then in proof call `lemma_cloned_view_eq(*ref, cloned)` to get
   `cloned@ == ref@`. Track element-wise view equality in loop invariant.

3. **Pigeonhole via set theory**: Rank function is injective on `set_int_range(0, n)`
   (proved via TotalOrder transitivity + antisymmetry + `lemma_subset_not_in_lt`).
   Image has same size as domain (`lemma_map_size`). Image is a subset of domain.
   Subset with same size must equal the whole set. Therefore every rank value 0..n-1
   is achieved, including the target `i`.

## Files Modified

- `src/Chap43/OrderedTableMtEph.rs` — Added requires to trait rank_key/select_key; rewrote rank_key body; cfg-guarded import
- `src/Chap43/OrderedTableStEph.rs` — Proved rank_key cardinality assume; proved select_key pigeonhole
- `src/Chap43/OrderedTableStPer.rs` — Proved rank_key cardinality assume; proved select_key pigeonhole
- `src/Chap43/OrderedSetStEph.rs` — Proved to_seq clone/view bridge assume
