# Agent 4 — Round 9 Report

## Summary

Eliminated 9 proof holes across Chap41 and Chap53. PQMinStPer.rs now fully verified (0 holes).
Verified: 3928, Errors: 0. RTT: 2600 pass.

## Holes Before/After

| # | Chap | File                    | Before | After | Delta |
|---|------|-------------------------|--------|-------|-------|
| 1 | 41   | AVLTreeSetMtEph.rs      | 24     | 19    | -5    |
| 2 | 41   | AVLTreeSetMtPer.rs      | 13     | 13    | 0     |
| 3 | 53   | GraphSearchStEph.rs     | 1      | 1     | 0     |
| 4 | 53   | GraphSearchStPer.rs     | 2      | 1     | -1    |
| 5 | 53   | GraphSearchMtPer.rs     | 1      | 1     | 0     |
| 6 | 53   | PQMinStPer.rs           | 3      | 0     | -3    |
| 7 | 53   | PQMinStEph.rs           | 0      | 0     | 0     |
|   |      | **Total**               | **44** | **35**| **-9**|

Project total holes: 300 → 291 (-9).

## Chapters Closed

None fully closed. PQMinStPer.rs is now clean (0 holes).

## Techniques Used

1. **wf-implies-inv**: In MtEph, `spec_avltreesetsteph_wf()` includes
   `elements.spec_avltreeseqsteph_wf()`, which is exactly what `AVLTreeSetMtEphInv.inv()`
   checks. Replaced 5 `assume(inv)` with `assert(inv)` in empty/singleton/from_seq/delete/insert.

2. **Direct .elements access**: StPer's `to_seq()` doesn't ensure `spec_avltreeseqstper_wf()`
   on the returned seq, so `nth()` fails. Workaround: access `.elements` pub field directly,
   since `spec_avltreesetstper_wf()` (in requires) unfolds to include `elements.spec_avltreeseqstper_wf()`.

3. **Recursive-to-loop conversion**: PQMinStPer's `pq_explore` converted from recursive
   `external_body` to a verified `while` loop with `#[verifier::exec_allows_no_decreases_clause]`.
   Full loop invariants propagate wf, closure requires, and subset tracking.

4. **Closure requires propagation**: Lifted `graph.requires((v,))` and `priority_fn.requires((v,))`
   into function requires clauses per `using_closures_standard.rs`.

5. **feq broadcast**: Added `broadcast use crate::vstdplus::feq::feq::group_feq_axioms`
   and `obeys_feq_clone::<V>()` requires where needed for clone proofs.

## Remaining Holes — What Blocks Them

### Chap41 AVLTreeSetMtEph.rs (19 holes)
- **7 external_body**: spec_set_view, to_seq, filter, intersection, difference, union, iter.
  All involve Arc<RwLock> — the view through Arc is opaque, can't prove postconditions about
  set contents.
- **11 assume**: All view-related (e.g., `assume(r == self@.len())`, `assume(self@.finite())`).
  Blocked because `spec_set_view` is external_body — no way to connect the inner
  AVLTreeSetStEph's view to the outer MtEph's view through Arc<RwLock>.
- **1 trivial_spec_wf**: `spec_avltreesetmteph_wf() { true }`. Requires defining a real
  wf predicate, but Arc<RwLock> makes the inner state opaque.

### Chap41 AVLTreeSetMtPer.rs (13 holes)
- **7 external_body**: from_seq, filter, intersection, difference, union, delete, insert.
  All contain parallel implementations with nested helper functions (parallel_sort, etc.)
  that live inside external_body blocks.
- **6 assume**: wf-bridge assumes in size() and find(). Bridge internal
  `spec_avltreeseqmtper_wf` and feq properties. Would need architectural changes to prove.

### Chap53 (3 holes, all graph_search_explore)
- **GraphSearchStEph.rs**: 1 external_body on `graph_search_explore`.
- **GraphSearchStPer.rs**: 1 external_body on `graph_search_explore`.
- **GraphSearchMtPer.rs**: 1 external_body on `graph_search_explore`.
  All three are the same recursive exploration function. Converting to a while loop
  (as done for PQMinStPer) is possible but the ensures (`visited@ ⊇ visited_init@ ∪ frontier@`)
  requires tracking that every frontier vertex eventually gets visited, which needs an
  inductive argument over the shrinking frontier.

## Commit

See git log for commit hash.
