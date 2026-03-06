# Well-Formedness Spec Naming Audit

Convention: `spec_{module name lowercase no underscores}_wf`

## Module WF Specs

| # | Chap | File | Function | Conforms? |
|---|------|------|----------|-----------|
| 1 | 37 | AVLTreeSeq.rs | `spec_avltreeseq_wf` | Yes |
| 2 | 37 | AVLTreeSeqStEph.rs | `spec_avltreeseqsteph_wf` | Yes |
| 3 | 37 | AVLTreeSeqStPer.rs | `spec_avltreeseqstper_wf` | Yes |
| 4 | 37 | AVLTreeSeqMtPer.rs | `spec_avltreeseqmtper_wf` | Yes |
| 5 | 37 | BSTRBMtEph.rs | `link_wf` | No — helper, not module wf |
| 6 | 37 | BSTSplayMtEph.rs | `link_wf` | No — helper, not module wf |
| 7 | 39 | BSTTreapStEph.rs | `spec_wf` | No — bare |
| 8 | 39 | BSTTreapStEph.rs | `spec_size_wf_link` | Sub-wf helper |
| 9 | 39 | BSTTreapMtEph.rs | `spec_size_wf_link` | Sub-wf helper |
| 10 | 40 | BSTKeyValueStEph.rs | `spec_wf` | No — bare |
| 11 | 40 | BSTSizeStEph.rs | `spec_wf`, `spec_size_wf` | No — bare |
| 12 | 40 | BSTReducedStEph.rs | `spec_wf`, `spec_size_wf` | No — bare |
| 13 | 41 | ArraySetStEph.rs | `spec_wf` | No — bare |
| 14 | 41 | AVLTreeSetStPer.rs | `spec_avltreesetstper_wf` | Yes |
| 15 | 42 | TableStEph.rs | `spec_tablesteph_wf` | Yes |
| 16 | 42 | TableStPer.rs | `spec_tablestper_wf` | Yes |
| 17 | 42 | TableMtEph.rs | `spec_tablemteph_wf` | Yes |
| 18 | 43 | OrderedTableStEph.rs | `spec_wf` | No — bare |
| 19 | 43 | OrderedTableStPer.rs | `spec_orderedtablestper_wf` | Yes |
| 20 | 43 | AugOrderedTableStEph.rs | `spec_wf` | No — bare |
| 21 | 43 | AugOrderedTableStPer.rs | `spec_augorderedtablestper_wf` | Yes |
| 22 | 52 | AdjMatrixGraphStEph.rs | `spec_wf` | No — bare |
| 23 | 52 | AdjMatrixGraphStPer.rs | `spec_wf` | No — bare |
| 24 | 52 | AdjMatrixGraphMtEph.rs | `spec_wf` | No — bare |
| 25 | 52 | AdjMatrixGraphMtPer.rs | `spec_wf` | No — bare |
| 26 | 54 | BFSStEph.rs | `spec_bfssteph_wf` | Yes |
| 27 | 54 | BFSStPer.rs | `spec_bfsstper_wf` | Yes |
| 28 | 54 | BFSMtEph.rs | `spec_bfsmteph_wf` | Yes |
| 29 | 54 | BFSMtPer.rs | `spec_bfsmtper_wf` | Yes |
| 30 | 55 | TopoSortStEph.rs | `spec_wf_adj_list` | No — inverted |
| 31 | 55 | TopoSortStPer.rs | `spec_wf_adj_list_per` | No — inverted |
| 32 | 55 | CycleDetectStPer.rs | `spec_wf_adj_list_per` | No — inverted |
| 33 | 56 | SSSPResultStEphI64.rs | `spec_ssspresultstephi64_wf` | Yes |
| 34 | 56 | SSSPResultStPerI64.rs | `spec_ssspresultstperi64_wf` | Yes |
| 35 | 56 | AllPairsResultStEphI64.rs | `spec_allpairsresultstephi64_wf` | Yes |
| 36 | 56 | AllPairsResultStPerI64.rs | `spec_allpairsresultstperi64_wf` | Yes |
| 37 | 65 | UnionFindStEph.rs | `wf` | No — bare, no `spec_` |
| 38 | 12 | Exercise12_5.rs | `wf` | No — bare, no `spec_` |
| 39 | 19 | ArraySeqMtEphSlice.rs | `slice_wf` | Sub-wf helper |
| 40 | — | Types.rs | `wf_graph_view` | No — no `spec_` |
| 41 | — | Types.rs | `wf_lab_graph_view` | No — no `spec_` |
| 42 | — | vstdplus/float.rs | `float_wf` | No — no `spec_` |

## Standards (all conform except one)

| # | File | Function | Conforms? |
|---|------|----------|-----------|
| 1 | mod_standard.rs | `spec_modstandard_wf` | Yes |
| 2 | rwlock_standard.rs | `spec_rwlockstandard_wf` | Yes |
| 3 | table_of_contents_standard.rs | `spec_tableofcontentsstandard_wf` | Yes |
| 4 | arc_rwlock_coarse_standard.rs | `spec_wf` | No — BST node wf, not module wf |

## Summary

- 21 conform
- 17 do not (mostly bare `spec_wf` or missing the `spec_` prefix)
- Convention established after Chap39-43 and graph chapters were written
- Chap54/56 files and standards all follow it
