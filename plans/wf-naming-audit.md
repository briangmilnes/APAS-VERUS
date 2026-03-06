# Well-Formedness Spec Naming Audit (revised 2026-03-06)

## Conventions

1. **Module-level wf**: `spec_{TYPE}_wf` — TYPE is the type name in lowercase, no internal underscores.
   For module-level wf, the type name typically matches the module/file name (e.g., `TableStEph` → `spec_tablesteph_wf`).

2. **Sub-property wf**: `spec_{TYPE}_{PROPERTY}_wf` — TYPE is the type, PROPERTY is what's being checked.
   Example: `spec_link_size_wf` (type=Link, property=size).

## Conforming (26)

| # | Chap | File | Function |
|---|------|------|----------|
| 1 | 37 | AVLTreeSeq.rs | `spec_avltreeseq_wf` |
| 2 | 37 | AVLTreeSeqStEph.rs | `spec_avltreeseqsteph_wf` |
| 3 | 37 | AVLTreeSeqStPer.rs | `spec_avltreeseqstper_wf` |
| 4 | 37 | AVLTreeSeqMtPer.rs | `spec_avltreeseqmtper_wf` |
| 5 | 37 | BSTRBMtEph.rs | `spec_bstrbmteph_wf` |
| 6 | 37 | BSTSplayMtEph.rs | `spec_bstsplaymteph_wf` |
| 7 | 39 | BSTTreapMtEph.rs | `spec_bsttreapmteph_wf` |
| 8 | 41 | ArraySetEnumMtEph.rs | `spec_arraysetenummteph_wf` |
| 9 | 41 | AVLTreeSetStPer.rs | `spec_avltreesetstper_wf` |
| 10 | 41 | AVLTreeSetMtEph.rs | `spec_avltreesetmteph_wf` |
| 11 | 42 | TableStEph.rs | `spec_tablesteph_wf` |
| 12 | 42 | TableStPer.rs | `spec_tablestper_wf` |
| 13 | 42 | TableMtEph.rs | `spec_tablemteph_wf` |
| 14 | 43 | OrderedTableStPer.rs | `spec_orderedtablestper_wf` |
| 15 | 43 | AugOrderedTableStPer.rs | `spec_augorderedtablestper_wf` |
| 16 | 54 | BFSStEph.rs | `spec_bfssteph_wf` |
| 17 | 54 | BFSStPer.rs | `spec_bfsstper_wf` |
| 18 | 54 | BFSMtEph.rs | `spec_bfsmteph_wf` |
| 19 | 54 | BFSMtPer.rs | `spec_bfsmtper_wf` |
| 20 | 56 | SSSPResultStEphI64.rs | `spec_ssspresultstephi64_wf` |
| 21 | 56 | SSSPResultStPerI64.rs | `spec_ssspresultstperi64_wf` |
| 22 | 56 | AllPairsResultStEphI64.rs | `spec_allpairsresultstephi64_wf` |
| 23 | 56 | AllPairsResultStPerI64.rs | `spec_allpairsresultstperi64_wf` |
| 24 | std | mod_standard.rs | `spec_modstandard_wf` |
| 25 | std | rwlock_standard.rs | `spec_rwlockstandard_wf` |
| 26 | std | table_of_contents_standard.rs | `spec_tableofcontentsstandard_wf` |

## Non-Conforming — Module-Level WF (14 renames)

| # | Chap | File | Current | Target |
|---|------|------|---------|--------|
| 1 | 39 | BSTTreapStEph.rs | `spec_wf` | `spec_bsttreapsteph_wf` |
| 2 | 40 | BSTKeyValueStEph.rs | `spec_wf` | `spec_bstkeyvaluesteph_wf` |
| 3 | 40 | BSTSizeStEph.rs | `spec_wf` | `spec_bstsizesteph_wf` |
| 4 | 40 | BSTReducedStEph.rs | `spec_wf` | `spec_bstreducedsteph_wf` |
| 5 | 41 | ArraySetStEph.rs | `spec_wf` | `spec_arraysetsteph_wf` |
| 6 | 43 | OrderedTableStEph.rs | `spec_wf` | `spec_orderedtablesteph_wf` |
| 7 | 43 | AugOrderedTableStEph.rs | `spec_wf` | `spec_augorderedtablesteph_wf` |
| 8 | 52 | AdjMatrixGraphStEph.rs | `spec_wf` | `spec_adjmatrixgraphsteph_wf` |
| 9 | 52 | AdjMatrixGraphStPer.rs | `spec_wf` | `spec_adjmatrixgraphstper_wf` |
| 10 | 52 | AdjMatrixGraphMtEph.rs | `spec_wf` | `spec_adjmatrixgraphmteph_wf` |
| 11 | 52 | AdjMatrixGraphMtPer.rs | `spec_wf` | `spec_adjmatrixgraphmtper_wf` |
| 12 | 55 | TopoSortStEph.rs | `spec_wf_adj_list` | `spec_toposortsteph_wf` |
| 13 | 55 | TopoSortStPer.rs | `spec_wf_adj_list_per` | `spec_toposortstper_wf` |
| 14 | 55 | CycleDetectStPer.rs | `spec_wf_adj_list_per` | `spec_cycledetectstper_wf` |

## Non-Conforming — No `spec_` Prefix (3 renames)

| # | Chap | File | Current | Target |
|---|------|------|---------|--------|
| 15 | 65 | UnionFindStEph.rs | `wf` | `spec_unionfindsteph_wf` |
| 16 | — | Types.rs | `wf_graph_view` | `spec_graphview_wf` |
| 17 | — | Types.rs | `wf_lab_graph_view` | `spec_labgraphview_wf` |

## Non-Conforming — Sub-Property WF (6 renames)

Convention: `spec_{TYPE}_{PROPERTY}_wf`

| # | Chap | File | Current | Target |
|---|------|------|---------|--------|
| 18 | 39 | BSTTreapStEph.rs | `spec_size_wf_link` | `spec_link_size_wf` |
| 19 | 39 | BSTTreapMtEph.rs | `spec_size_wf_link` | `spec_link_size_wf` |
| 20 | 40 | BSTSizeStEph.rs | `spec_size_wf_link` | `spec_link_size_wf` |
| 21 | 40 | BSTSizeStEph.rs | `spec_size_wf` | `spec_bstsizesteph_size_wf` |
| 22 | 40 | BSTReducedStEph.rs | `spec_size_wf_link` | `spec_link_size_wf` |
| 23 | 40 | BSTReducedStEph.rs | `spec_size_wf` | `spec_bstreducedsteph_size_wf` |

## Excluded from Rename

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 12 | Exercise12_5.rs | `wf` | Exercise file |
| 2 | 19 | ArraySeqMtEphSlice.rs | `slice_wf` | Slice sub-wf helper |
| 3 | — | vstdplus/float.rs | `float_wf` | Trait method, per-element wf |
| 4 | — | vstdplus/float.rs | `all_float_wf` | Derived from `float_wf` trait |
| 5 | std | arc_rwlock_coarse_standard.rs | `spec_wf` | Per-node wf with (lo,hi) params |
| 6 | exp | ghost_type_invariant.rs | `wf` | Experiment file |
| 7 | exp | abstract_set_iter.rs | `wf` | Experiment file |

## Execution Plan

23 renames across 16 source files + Types.rs.

### Ordering by blast radius (smallest first)

**Batch 1 — Self-contained files (4 renames):**
Chap65 UnionFindStEph (#15), Chap55 TopoSortStEph (#12), TopoSortStPer (#13), CycleDetectStPer (#14)

**Batch 2 — Chap40 BST variants (6 renames):**
BSTKeyValueStEph (#2), BSTSizeStEph (#3, #20, #21), BSTReducedStEph (#4, #22, #23).
Includes both module-level and sub-property wf renames. Standalone files.

**Batch 3 — Chap39 BSTTreapStEph (2 renames):**
Module-level (#1) and sub-property (#18). BSTTreapMtEph sub-property (#19).

**Batch 4 — Chap41 ArraySetStEph (1 rename):**
#5 only.

**Batch 5 — Chap43 StEph files (2 renames):**
OrderedTableStEph (#6), AugOrderedTableStEph (#7).

**Batch 6 — Chap52 AdjMatrixGraph (4 renames):**
All four variants (#8-#11). Cross-file callers likely in Chap54-56 graph algorithms.

**Batch 7 — Types.rs (2 renames, highest blast radius):**
`wf_graph_view` (#16), `wf_lab_graph_view` (#17). Imported by Chap52-56 graph modules.

### Per-file procedure

1. Rename in trait declaration
2. Rename in trait impl
3. Rename free spec fn (if exists)
4. Find and update all call sites across `src/`, `tests/`, `rust_verify_test/`
5. `scripts/validate.sh`
6. Fix any breakage, re-validate

## Summary

- **26 conform**
- **23 need rename** (14 module-level, 3 no-prefix, 6 sub-property)
- **7 excluded** (exercises, experiments, float trait, arc_rwlock standard)
