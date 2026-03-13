# Agent 2 Work Plan — BST + Collections

## Fresh Baseline (scanned 2026-03-12)

| # | Chap | Holes | assume | ext_body | triv_wf | fn_missing_spec | Files |
|---|------|-------|--------|----------|---------|-----------------|-------|
| 1 | 37 | 39 | 7 | 31 | 1 | ~75 | 19 |
| 2 | 39 | 38 | 0 | 38 | 0 | 5 | 4 |
| 3 | 40 | 0 | 0 | 0 | 0 | 6 | 3 |
| 4 | 41 | 62 | 39 | 22 | 1 | 7 | 7 |
| 5 | 42 | 14 | 0 | 14 | 0 | 3 | 4 |
| | **Total** | **153** | **46** | **105** | **2** | **~96** | **37** |

## Progress Log

### Phase 1: Chap40 fn_missing_spec — COMPLETE
- Fixed 6 fn_missing_spec across 3 files (BSTSizeStEph, BSTKeyValueStEph, BSTReducedStEph).
- Pattern: `requires true` + simple provable ensures on `compare_*` and `clone_link`.
- Chap40: 3 clean modules, 0 holed, 0 holes, 215 complete specs. New clean chapter.

### Phase 2 (partial): Chap37 fn_missing_spec — COMPLETE
- Fixed 73 fn_missing_spec across 15 files:
  - BSTPlainMtEph, BSTAVLMtEph, BSTBBAlphaMtEph: min_node/max_node (6 fixes)
  - BSTRBMtEph: 20 functions (new_node through height_rec)
  - BSTSplayMtEph: 16 functions
  - BSTSplayStEph: 11 functions
  - BSTSet*MtEph (5 files): values_vec, rebuild_from_vec, from_sorted_iter, copy_set
  - AVLTreeSeq*: cached_height/h_fn/size_fn/height_fn (7 fixes across 4 files)
- Chap37: 14 clean modules (up from 4), 748/956 complete specs (78%), 2 unfixable fn_missing_spec (nested `rec` in external_body).
- 39 proof holes unchanged (7 assume, 31 external_body, 1 trivial_wf).

### Chap39 fn_missing_spec — COMPLETE
- Fixed 5 fn_missing_spec in BSTTreapMtEph.rs (clone_link, size_link, find_link, min_link, max_link).
- 38 proof holes unchanged.

### Chap41 fn_missing_spec — PARTIAL
- Fixed 4 fn_missing_spec in Example41_3.rs (example wrapper functions).
- 3 remaining: nested functions inside external_body (parallel_filter, parallel_intersect, parallel_sort).
- 62 proof holes unchanged.

### Chap42 fn_missing_spec — COMPLETE
- Fixed 3 fn_missing_spec in TableMtEph.rs, TableStEph.rs, TableStPer.rs (from_sorted_entries).
- Chap42: 2 clean modules (up from 1), 14 proof holes unchanged.

### Chap41 wf-bridge fix — COMPLETE (Round 4)
- Added `spec fn spec_avltreesetsteph_wf` to AVLTreeSetStEphTrait, propagating
  `self.elements.spec_avltreeseqsteph_wf()`.
- Added `requires wf` / `ensures wf` to all 11 trait methods.
- Removed 9 wf-bridge assumes from AVLTreeSetStEph impl (size, filter, intersection,
  difference, union×2, find, delete, insert).
- Added wf to 6 loop invariants (from_seq, filter, intersection, difference, union×2).
- Fixed cascade in Chap43/OrderedSetStEph.rs: updated trait requires/ensures, changed
  `spec_orderedsetsteph_wf` from `self@.finite()` to `self.base_set.spec_avltreesetsteph_wf()`.
- Fixed cascade in Chap53/GraphSearchStEph.rs: added `requires frontier.wf` to
  SelectionStrategy::select trait.
- No new holes introduced in cascade files. 0 errors, 3670 verified.

### Chap41 StPer from_vec wf — COMPLETE (Round 4)
- Removed 2 `assume(updated.spec_avltreesetstper_wf())` in AVLTreeSetStPer.rs (delete, insert
  not-found) — from_vec ensures wf, so construction-wf is provable.
- 1 remaining construction-wf assume (insert found-case): clone doesn't ensure wf.
- 10 remaining assumes: 9 operation-spec (set semantics) + 1 clone-wf.

### Chap37 wf size bound + compare_trees + clone_link — COMPLETE (Round 5)
- Added size bound `(node.left_size + node.right_size + 1 < usize::MAX)` to node-level
  `spec_avltreeseqsteph_wf` in AVLTreeSeqStEph.rs. Size bound is self-maintaining.
- Strengthened `size_link_fn` requires from `true` to `spec_avltreeseqsteph_wf(*n)`.
  Removed overflow assume at L313.
- Added size bound requires to `update_meta` and `rebalance_fn` (cascade from wf change).
- Ported verified `compare_trees` from shared AVLTreeSeq.rs: loop invariant with feq,
  removes external_body. PartialEq::eq became external_body bridge (net 0 on that item).
- Wrote verified `clone_link` recursive function: removes Clone external_body on
  AVLTreeSeqStEphS. Single accept for T::clone view bridge.
- Chap37: 39 → 37 holes (−1 assume, −1 external_body). 3672 verified, 0 errors.

## Current State (post Chap37 Round 5)

| # | Chap | Holes | assume | ext_body | triv_wf | Clean Mods | Total Mods |
|---|------|-------|--------|----------|---------|------------|------------|
| 1 | 37 | 37 | 6 | 30 | 1 | 14 | 19 |
| 2 | 39 | 38 | 0 | 38 | 0 | 1 | 4 |
| 3 | 40 | 0 | 0 | 0 | 0 | 3 | 3 |
| 4 | 41 | 51 | 28 | 22 | 1 | 1 | 7 |
| 5 | 42 | 14 | 0 | 14 | 0 | 2 | 4 |
| | **Total** | **140** | **34** | **104** | **2** | **21** | **37** |

Verification: 3672 verified, 0 errors.

## Remaining Work

### Chap37 (37 holes)
- 6 assume: 1 height overflow (StEph), 1 height overflow (shared), 1 next_key (StEph),
  1 from_vec spec + 1 wf-bridge (MtPer, blocked by build_balanced ext_body),
  1 eq wf-bridge (shared, PartialEq::eq)
- 30 external_body: 19 Arc path-copying (shared), 5 iterator (StEph/StPer),
  1 Clone for AVLTreeNode (shared, Arc), 1 PartialEq::eq (StEph, bridge),
  4 misc (StPer iter/clone)
- 1 trivial_wf: BSTSplayStEph (design choice, splay has no global invariant)

### Chap39 (37 holes)
- 37 external_body: BSTTreapMtEph (9), BSTSetTreapMtEph (10), BSTParaTreapMtEph (18)
- BSTTreapMtEph: Arc removed, now plain RwLock + ghost shadow. new() fully verified (-1 hole).
- BSTParaTreapMtEph is fine-grained concurrent — excluded from simple migration

### Chap41 (51 holes)
- 12 assume in AVLTreeSetStEph: 8 operation-spec (set semantics proofs), 2 overflow, 1 clone-wf, 1 from_seq-wf
- 10 assume in AVLTreeSetStPer: 9 operation-spec (set semantics), 1 clone-wf
- 5 assume in AVLTreeSetMtPer: 2 wf-bridge (size, find), 2 operation-spec (size, find), 1 finiteness
- 1 assume in ArraySetEnumMtEph: finiteness of bounded Set::new
- 22 external_body: mix of parallel (filter, intersection, union, difference) and delegated Mt ops
- 1 trivial_wf in AVLTreeSetMtEph: Unit Inv (no ghost↔exec fields to link)

### Chap42 (14 holes)
- 13 external_body in TableMtEph.rs: 8 join-based (must stay), 5 sequential (potentially provable)
- 1 external_body in TableStPer.rs: collect_by_key

## Priority Order: Chap40 → Chap37 → Chap42 → Chap39 → Chap41
