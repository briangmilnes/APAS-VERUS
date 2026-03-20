# Agent 2 Round 46 Report

## Summary

Proved `rank_key` and `select_key` in OrderedTableStPer.rs, removing 2 external_body
holes. Added missing `requires` to `reduce_range_parallel` trait in AugOrderedTableMtEph.rs.
Investigated all remaining holes; most are blocked by infrastructure gaps.

## Results

- Verification: 4400 verified, 0 errors
- RTT: 2613 tests passed
- Holes: 69 before, 67 after (2 closed)

## Holes Before/After by File

| # | Chap | File | Before | After | Notes |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStPer.rs | 2 | 0 | Proved rank_key and select_key |
| 2 | 43 | AugOrderedTableMtEph.rs | 1 | 1 | external_body on ParaPair! thread boundary |
| 3 | 43 | AugOrderedTableStPer.rs | 1 | 1 | Closure clone bridge assume (allowed pattern) |
| 4 | 43 | OrderedSetStEph.rs | 1 | 1 | select assume: needs sortedness in wf |
| 5 | 43 | OrderedSetStPer.rs | 1 | 1 | select assume: needs sortedness in wf |
| 6 | 59 | JohnsonMtEphI64.rs | 1 | 1 | external_body on ParaPair! thread boundary |
| 7 | 59 | JohnsonStEphI64.rs | 1 | 1 | assume: needs graph partition lemma |
| 8 | 41 | AVLTreeSetStEph.rs | 2 | 2 | Off-by-one: wf gives len<MAX, need len+1<MAX |
| 9 | 41 | Example41_3.rs | 4 | 4 | Skipped (Example file) |

## Techniques Used

1. **Ported StEph proof to StPer**: Adapted the verified `rank_key` proof from
   OrderedTableStEph.rs to OrderedTableStPer.rs. Key ingredients: ghost `counted` set,
   `TotalOrder::cmp` for spec-level comparison info, `spec_rank_pred` helper spec fn,
   set-filter equality proof at loop exit.

2. **select_key rewrite**: Replaced sort-based implementation (unverifiable due to
   `keys.sort()`) with iterate-and-rank approach matching StEph. For each entry, calls
   `rank_key` to find the element with rank == i.

3. **Spec improvement**: Added `requires self.spec_augorderedtablemteph_wf()` to
   `reduce_range_parallel` trait in AugOrderedTableMtEph (was missing, matching the
   sequential `reduce_range` which has the same requires).

## Remaining Holes: What Blocks Them

| # | Chap | File | Hole | Blocker |
|---|------|------|------|---------|
| 1 | 41 | AVLTreeSetStEph.rs | assume(new_vec@.len() < usize::MAX) x2 | Wf bound is len<MAX but from_vec needs len<MAX for len+1 elements. Fix: strengthen wf to len+1<MAX or add requires to insert/insert_sorted. Cascades to MtEph RwLock predicates. |
| 2 | 43 | AugOrderedTableMtEph.rs | external_body on reduce_range_parallel | Uses ParaPair! for thread spawning. Allowed pattern per CLAUDE.md. |
| 3 | 43 | AugOrderedTableStPer.rs | assume in lemma_reducer_clone_total | Closure clone bridge. Same Verus limitation as eq/clone workaround. |
| 4 | 43 | OrderedSetStEph.rs | assume in select | select returns i-th element, needs rank==i. Requires sortedness in wf, but sortedness is only defined under T: TotalOrder (method-level where clause). |
| 5 | 43 | OrderedSetStPer.rs | assume in select | Same as OrderedSetStEph. |
| 6 | 59 | JohnsonMtEphI64.rs | external_body on parallel_dijkstra_all | Uses ParaPair! for recursive divide-and-conquer. Thread boundary pattern. |
| 7 | 59 | JohnsonStEphI64.rs | assume in reweight_graph | Needs `reweighted@.A.len() <= graph@.A.len()`. Proof requires graph partition lemma (sum of out-degrees == |A|) not yet in library. from_weighed_edges also doesn't ensure output A.len(). |

## Warnings Investigated

| # | Chap | File | Warning | Assessment |
|---|------|------|---------|------------|
| 1 | 59 | JohnsonStEphI64.rs | fn_missing_requires: adjust_distance | Genuinely no precondition (all i64 inputs valid). Cannot add requires true or veracity annotation per rules. |
| 2 | 59 | JohnsonStEphI64.rs | fn_missing_requires: reweight_edge | Same: no precondition. Has `ensures true` which is a weak spec. |
| 3 | 43 | OrderedSetStEph.rs | fn_missing_requires: from_sorted_elements | Genuinely no precondition: from_vec (StPer) has no requires. |
| 4 | 43 | OrderedSetStPer.rs | fn_missing_requires: from_sorted_elements | Same as StEph version. |
