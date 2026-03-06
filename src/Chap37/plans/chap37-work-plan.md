# Chapter 37 Work Plan — Binary Search Trees

Generated 2026-03-05 (rev 3). Post-Phase 1 update: items 1-3, 5-6, 8-9, 11-12 complete.

## Status Summary

- 19 source files, ~11,300 lines
- 40 holes (11 assume, 29 external_body) — 38 in AVLTreeSeq* family, 2 in BSTSplayStEph
- 9 proof fns, all clean
- 195 style warnings (100 in BSTSet* [12], 23 in AVLTreeSeq* [22], rest scattered)
- 4 fully clean files: BSTAVLMtEph, BSTBBAlphaMtEph, BSTPlainMtEph, BSTRBStEph
- Holes log: up to date (Mar 5)
- Style log: up to date (Mar 5)
- Fn-impls analysis: up to date (Mar 5)

## Completed (rev 2 items)

| # | Item | File(s) | Done |
|---|------|---------|------|
| 1 | 1 | BSTPlainMtEph.rs | Trait specs + TOC fix |
| 2 | 2 | BSTAVLMtEph.rs | Trait specs + TOC fix |
| 3 | 3 | BSTBBAlphaMtEph.rs | Trait specs |
| 4 | 5 | BSTRBMtEph.rs | Already vstd::rwlock; trait specs + TOC + style |
| 5 | 6 | BSTSplayMtEph.rs | Already vstd::rwlock; trait specs + TOC + style |
| 6 | 8 | BSTRBMtEph.rs | Eliminated size assume with runtime guard |
| 7 | 9 | BSTSplayMtEph.rs | Eliminated size assume with runtime guard |
| 8 | 11 | BSTPlainMtEph.rs | TOC fix (done with item 1) |
| 9 | 12 | BSTAVLMtEph.rs | TOC fix (done with item 2) |

## Skipped (rev 2 items)

| # | Item | File(s) | Reason |
|---|------|---------|--------|
| 1 | 4 | BSTBBAlphaStEph.rs | Rebuild algorithm not implemented; no rebalancing to prove through |
| 2 | 7 | BSTSplayStEph.rs | 120+ line &mut splay fn; complex proof |
| 3 | 10 | BSTRBStEph.rs | Color field cascades to BalBinTree (85 sites, 8 files); needs design decision |

## Downstream Dependency Analysis

Chapters blocked by Chap37: 41, 43, 44, 45, 52, 53, 55.

Almost ALL downstream imports go through AVLTreeSeq* files:

| # | Chap | Files Importing | From |
|---|------|-----------------|------|
| 1 | 41 | ArraySeq*, TreeSeq*, Seq* | AVLTreeSeqStEph, AVLTreeSeqStPer, AVLTreeSeqMtPer |
| 2 | 43 | OrderedTable*, AugOrderedTable*, OrderedSet*, Example43_1 | AVLTreeSeqStPer |
| 3 | 44 | DocumentIndex | AVLTreeSeqStPer |
| 4 | 45 | BalancedTreePQ, HeapsortExample | AVLTreeSeqStPer |
| 5 | 52 | AugOrderedTable*, OrderedTable* | AVLTreeSeqStEph, AVLTreeSeqStPer, AVLTreeSeqMtPer |
| 6 | 53 | AugOrderedTable*, OrderedTable* | AVLTreeSeqStEph, AVLTreeSeqStPer, AVLTreeSeqMtPer |
| 7 | 55 | GraphImplAdjTree*, GraphSeqAdjTree* | AVLTreeSeqStEph, AVLTreeSeqStPer |

No downstream chapter imports BST* or BSTSet* files directly.

## Current Holes by File

| # | Chap | File | Holes | Breakdown |
|---|------|------|------:|-----------|
| 1 | 37 | AVLTreeSeqStPer.rs | 14 | 3 assume, 11 external_body |
| 2 | 37 | AVLTreeSeqMtPer.rs | 12 | 1 assume, 11 external_body |
| 3 | 37 | AVLTreeSeqStEph.rs | 9 | 5 assume, 4 external_body |
| 4 | 37 | AVLTreeSeq.rs | 3 | 1 assume, 2 external_body |
| 5 | 37 | BSTSplayStEph.rs | 2 | 1 assume, 1 external_body |

Plus 9 eq/clone workaround assumes (tagged, accepted).

## Current Style Warnings by File

| # | Chap | File | Warnings | Categories |
|---|------|------|--------:|------------|
| 1 | 37 | BSTSetPlainMtEph.rs | 23 | 20×[12], 2×[17], 1×[22] |
| 2 | 37 | BSTSetAVLMtEph.rs | 23 | 20×[12], 2×[17], 1×[22] |
| 3 | 37 | BSTSetBBAlphaMtEph.rs | 23 | 20×[12], 2×[17], 1×[22] |
| 4 | 37 | BSTSetRBMtEph.rs | 23 | 20×[12], 2×[17], 1×[22] |
| 5 | 37 | BSTSetSplayMtEph.rs | 23 | 20×[12], 2×[17], 1×[22] |
| 6 | 37 | AVLTreeSeq.rs | 16 | 8×[22], 3×[13], 2×[18], 2×[17], 1×[12] |
| 7 | 37 | AVLTreeSeqStEph.rs | 14 | 5×[22], 2×[18], 2×[17], 2×[13], 1×[19], 1×[12], 1×[4] |
| 8 | 37 | AVLTreeSeqStPer.rs | 12 | 5×[22], 2×[18], 2×[17], 1×[19], 1×[13], 1×[12] |
| 9 | 37 | AVLTreeSeqMtPer.rs | 12 | 5×[22], 2×[17], 2×[18], 1×[19], 1×[13], 1×[12] |
| 10 | 37 | BSTSplayStEph.rs | 7 | 4×[12], 3×[22] |
| 11 | 37 | BSTPlainStEph.rs | 2 | 2×[22] |
| 12 | 37 | BSTAVLStEph.rs | 2 | 2×[22] |
| 13 | 37 | BSTBBAlphaStEph.rs | 2 | 2×[22] |
| 14 | 37 | BSTRBMtEph.rs | 4 | 2×[13], 1×[19], 1×[22] |
| 15 | 37 | BSTSplayMtEph.rs | 4 | 2×[13], 1×[19], 1×[22] |

## Proposed Work (rev 3)

Ordered by file simplicity. Severity: low/med/high relative to downstream unblocking.

### Tier 1 — Simple style fixes (2 warnings each, mechanical)

| # | Sev | Chap | File | Work | Warnings | Notes |
|---|-----|------|------|------|----------|-------|
| 1 | low | 37 | BSTPlainStEph.rs | Move 2 free spec fns to trait | 2 [22] | tree_contains, tree_is_bst already used as trait abstract sigs in other files |
| 2 | low | 37 | BSTAVLStEph.rs | Move 2 free spec fns to trait | 2 [22] | tree_is_avl, avl_balanced |
| 3 | low | 37 | BSTBBAlphaStEph.rs | Move 2 free spec fns to trait | 2 [22] | tree_is_bb, weight_balanced |

### Tier 2 — BSTSplayStEph (7 warnings, 2 holes)

| # | Sev | Chap | File | Work | Warnings | Notes |
|---|-----|------|------|------|----------|-------|
| 4 | med | 37 | BSTSplayStEph.rs | Eliminate size assume in update | 0 | Same runtime guard as BSTRBMtEph/BSTSplayMtEph |
| 5 | low | 37 | BSTSplayStEph.rs | Add ensures true to 4 trait fns | 4 [12] | insert, contains, in_order, pre_order |
| 6 | low | 37 | BSTSplayStEph.rs | Move 3 free spec fns to trait | 3 [22] | spec_size_link, spec_height_link, spec_contains_link |

### Tier 3 — BSTRBMtEph / BSTSplayMtEph residual (4 warnings each)

| # | Sev | Chap | File | Work | Warnings | Notes |
|---|-----|------|------|------|----------|-------|
| 7 | low | 37 | BSTRBMtEph.rs | Move trait impl inside verus! | 2 [13] | Trait impl + Default currently outside verus! |
| 8 | low | 37 | BSTRBMtEph.rs | Move link_wf to trait; fix return name | 2 [19,22] | link_wf free spec, reduce return 'result' |
| 9 | low | 37 | BSTSplayMtEph.rs | Move trait impl inside verus! | 2 [13] | Trait impl + Default currently outside verus! |
| 10 | low | 37 | BSTSplayMtEph.rs | Move link_wf to trait; fix return name | 2 [19,22] | link_wf free spec, reduce return 'result' |

### Tier 4 — AVLTreeSeq* style (54 warnings across 4 files, HIGH downstream impact)

| # | Sev | Chap | File | Work | Warnings | Notes |
|---|-----|------|------|------|----------|-------|
| 11 | high | 37 | AVLTreeSeq.rs | Move 8 free spec fns to traits | 8 [22] | Critical: downstream chapters import these |
| 12 | high | 37 | AVLTreeSeq.rs | Fix TOC ordering, move impls inside verus! | 5 [13,18] | 3×[13] Default/Iterator, 2×[18] ordering |
| 13 | high | 37 | AVLTreeSeq.rs | Add iterator/IntoIterator impls | 2 [17] | Per collection-iterator standard |
| 14 | high | 37 | AVLTreeSeqStEph.rs | Move 5 free spec fns to traits | 5 [22] | Imported by Chap41,52,53,55 |
| 15 | high | 37 | AVLTreeSeqStEph.rs | Fix TOC, move impls inside verus!, fix return name | 5 [13,18,19] | 2×[13], 2×[18], 1×[19] |
| 16 | high | 37 | AVLTreeSeqStEph.rs | Fix iterator impl | 2 [17] | Has iterator but not per standard |
| 17 | high | 37 | AVLTreeSeqStPer.rs | Move 5 free spec fns to traits | 5 [22] | Most imported file (Chap43,44,45,52,53,55) |
| 18 | high | 37 | AVLTreeSeqStPer.rs | Fix TOC, move impl inside verus!, fix return name | 4 [13,18,19] | 1×[13], 2×[18], 1×[19] |
| 19 | high | 37 | AVLTreeSeqStPer.rs | Fix iterator impl | 2 [17] | Has iterator but not per standard |
| 20 | high | 37 | AVLTreeSeqMtPer.rs | Move 5 free spec fns to traits | 5 [22] | Imported by Chap41,52,53 |
| 21 | high | 37 | AVLTreeSeqMtPer.rs | Fix TOC, move impl inside verus!, fix return name | 4 [13,18,19] | 1×[13], 2×[18], 1×[19] |
| 22 | high | 37 | AVLTreeSeqMtPer.rs | Fix iterator impl | 2 [17] | Has iterator but not per standard |

### Tier 5 — AVLTreeSeq* holes (38 holes, critical for downstream)

| # | Sev | Chap | File | Work | Holes | Notes |
|---|-----|------|------|------|------:|-------|
| 23 | high | 37 | AVLTreeSeq.rs | Lift insert_at_link assume | 1 | nat_max assume in rebalance height |
| 24 | high | 37 | AVLTreeSeq.rs | Lift iterator external_body | 1 | next() on AVLTreeSeqIter |
| 25 | high | 37 | AVLTreeSeq.rs | Lift clone external_body | 1 | Clone for AVLTreeSeqS |
| 26 | high | 37 | AVLTreeSeqStEph.rs | Eliminate 3 size/height assumes | 3 | size_link_fn, update_meta, from_vec |
| 27 | high | 37 | AVLTreeSeqStEph.rs | Eliminate 2 push_back assumes | 2 | next_key + cached_size overflow |
| 28 | high | 37 | AVLTreeSeqStEph.rs | Lift 2 clone external_body | 2 | Clone for node + AVLTreeSeqStEphS |
| 29 | high | 37 | AVLTreeSeqStEph.rs | Lift iterator external_body | 1 | iter() returns AVLTreeSeqIterStEph |
| 30 | high | 37 | AVLTreeSeqStEph.rs | Lift compare_trees external_body | 1 | Used in PartialEq |
| 31 | high | 37 | AVLTreeSeqStPer.rs | Eliminate 3 assumes | 3 | subseq_copy, values_in_order, to_arrayseq wf assumes |
| 32 | high | 37 | AVLTreeSeqStPer.rs | Lift 8 Arc external_body fns | 8 | mk, rotate_*, rebalance, nth_ref, set_rec, inorder_collect, build_balanced |
| 33 | high | 37 | AVLTreeSeqStPer.rs | Lift 3 remaining external_body | 3 | compare_trees, iter, clone |
| 34 | high | 37 | AVLTreeSeqMtPer.rs | Eliminate values_in_order assume | 1 | spec_well_formed wf assume |
| 35 | high | 37 | AVLTreeSeqMtPer.rs | Lift 8 Arc external_body fns | 8 | Same as StPer: mk, rotate_*, rebalance, etc. |
| 36 | high | 37 | AVLTreeSeqMtPer.rs | Lift 3 remaining external_body | 3 | subseq_copy, clone, compare_trees |

### Tier 6 — BSTSet* files (115 warnings, no downstream deps)

| # | Sev | Chap | File(s) | Work | Warnings | Notes |
|---|-----|------|---------|------|----------|-------|
| 37 | low | 37 | BSTSet*MtEph.rs (5) | Add ensures true to 100 trait fns | 100 [12] | 20 per file, mechanical |
| 38 | low | 37 | BSTSet*MtEph.rs (5) | Add iterator/IntoIterator | 10 [17] | 2 per file |
| 39 | low | 37 | BSTSet*MtEph.rs (5) | Move free spec fns to traits | 5 [22] | 1 per file |

### Tier 7 — Deferred complex items (from rev 2)

| # | Sev | Chap | File | Work | Notes |
|---|-----|------|------|------|-------|
| 40 | med | 37 | BSTBBAlphaStEph.rs | Prove weight_balanced through insert | Needs rebuild algorithm first |
| 41 | med | 37 | BSTSplayStEph.rs | Prove BST ordering on splay/insert | 120+ line &mut splay fn; complex proof |
| 42 | med | 37 | BSTRBStEph.rs | Add RB color to BalBinTree | Cascades to Chap23 (85 sites, 8 files) |
| 43 | low | 37 | BSTSplayStEph.rs | Lift Node clone external_body | 1 hole |
| 44 | low | 37 | BSTSet*MtEph.rs (5) | O(lg n) split/join | Algorithmic; current O(n) |
| 45 | low | 37 | BSTSet*MtEph.rs (5) | Parallel filter/reduce | Textbook says O(lg n) span |

## Recommended Order

**Phase 2 — Quick wins (Tiers 1-3):** Items 1-10.
Mechanical style fixes. No proof changes. Low risk. Gets 6 more files to zero warnings.

**Phase 3 — AVLTreeSeq style (Tier 4):** Items 11-22.
High downstream impact. Move free spec fns to traits, fix TOC/ordering, iterator standard. Touches the 4 files that ALL downstream chapters import.

**Phase 4 — AVLTreeSeq holes (Tier 5):** Items 23-36.
The real unblocking work. 38 holes across 4 files. Start with size assumes (runtime guards), then Arc external_body (per arc_standard), then clone/iterator.

**Phase 5 — BSTSet style (Tier 6):** Items 37-39.
115 warnings but zero downstream impact. Mechanical.

**Phase 6 — Complex proof work (Tier 7):** Items 40-45.
Hard proofs, cross-chapter cascades, algorithmic redesigns. Deferred.

## Blockers

Proving Chap37 blocks downstream chapters 41, 43, 44, 45, 52, 53, 55.
Critical path: AVLTreeSeq* files (Tiers 4-5, items 11-36).
