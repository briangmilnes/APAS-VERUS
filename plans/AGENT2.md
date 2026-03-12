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

## Current State (post fn_missing_spec sweep)

| # | Chap | Holes | Clean Mods | Total Mods | Spec% | Remaining fn_miss |
|---|------|-------|------------|------------|-------|-------------------|
| 1 | 37 | 39 | 14 | 19 | 78% | 2 (unfixable) |
| 2 | 39 | 38 | 1 | 4 | 97% | 0 |
| 3 | 40 | 0 | 3 | 3 | 100% | 0 |
| 4 | 41 | 62 | 1 | 7 | 74% | 3 (unfixable) |
| 5 | 42 | 14 | 2 | 4 | 97% | 0 |
| | **Total** | **153** | **21** | **37** | | **5** |

Verification: 3670 verified, 0 errors (stable throughout).

## Remaining Work

The fn_missing_spec sweep is complete. Remaining work is proof holes:

### Chap37 (39 holes)
- 7 assume: mostly eq/clone workarounds in AVLTreeSeq* → classify for accept
- 31 external_body: 13 in TableMtEph are algorithmic parallel code (NOT coarse RwLock)
- 1 trivial_wf: vec-backed type, user confirmed `{ true }` is correct

### Chap39 (38 holes)
- 38 external_body: BSTTreapMtEph (10), BSTSetTreapMtEph (10), BSTParaTreapMtEph (18)
- BSTParaTreapMtEph is fine-grained concurrent BST — excluded from simple migration

### Chap41 (62 holes)
- 39 assume: need classification (lock-boundary, eq/clone, algorithmic)
- 22 external_body: mix of Mt migration candidates and algorithmic
- 1 trivial_wf

### Chap42 (14 holes)
- 13 external_body in TableMtEph.rs: algorithmic parallel implementations with join()
- 1 external_body in TableStPer.rs: collect_by_key

## Priority Order: Chap40 → Chap37 → Chap42 → Chap39 → Chap41
