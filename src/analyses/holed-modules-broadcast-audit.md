<style>

body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }

.markdown-body { max-width: 100% !important; width: 100% !important; }

.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }

table { width: 100% !important; table-layout: fixed; }

</style>

# Broadcast Axiom Audit for Holed Modules

| Chapter | File | Holes | Spec Types Used | vstd Broadcasts | vstdplus Broadcasts |
|---------|------|-------|-----------------|-----------------|---------------------|
| Chap35 | OrderStatSelectMtEph.rs | 1 | Seq | Multiset, Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap35 | OrderStatSelectMtPer.rs | 1 | Seq | Multiset, Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap41 | AVLTreeSetMtEph.rs | 14 | Set | None | None |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap41 | AVLTreeSetMtPer.rs | 18 | Set | None | None |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap41 | AVLTreeSetStEph.rs | 34 | Set | None | None |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap41 | AVLTreeSetStPer.rs | 30 | Set | Multiset, Seq, Set | Yes |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap41 | ArraySetEnumMtEph.rs | 15 | Set | None | None |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap41 | ArraySetStEph.rs | 21 | Seq, Set | Seq, Set | None |
| | *↳ Proposed* | | | `vstd::multiset::group_multiset_axioms`<br>`vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures`<br>`vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::multiset::*`<br>`crate::vstdplus::seq_set::*` |
| Chap41 | Example41_3.rs | 4 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap42 | TableMtEph.rs | 15 | Map, Seq | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default`<br>`vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap42 | TableStEph.rs | 14 | Map, Seq | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default`<br>`vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap26 | ETSPMtEph.rs | 2 | Seq | Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap26 | ETSPStEph.rs | 2 | Seq | Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap47 | ChainedHashTable.rs | 1 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap52 | AdjTableGraphMtPer.rs | 1 | Map, Set | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default`<br>`vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap52 | EdgeSetGraphMtPer.rs | 1 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap39 | BSTSetTreapMtEph.rs | 14 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap39 | BSTTreapMtEph.rs | 2 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap39 | BSTTreapStEph.rs | 9 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap50 | MatrixChainStEph.rs | 2 | Map, Seq | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default`<br>`vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap50 | MatrixChainStPer.rs | 2 | Map, Seq | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default`<br>`vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap50 | OptBinSearchTreeMtEph.rs | 1 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap50 | OptBinSearchTreeStEph.rs | 2 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap50 | OptBinSearchTreeStPer.rs | 2 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap50 | Probability.rs | 15 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap12 | Exercise12_5.rs | 4 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap40 | BSTKeyValueStEph.rs | 9 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap40 | BSTReducedStEph.rs | 12 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap40 | BSTSizeStEph.rs | 9 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap56 | Example56_1.rs | 3 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap56 | Example56_3.rs | 2 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap43 | AugOrderedTableMtEph.rs | 5 | Map | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | AugOrderedTableStEph.rs | 5 | Map | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | AugOrderedTableStPer.rs | 4 | Map | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | OrderedSetMtEph.rs | 23 | Set | None | None |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | OrderedSetStEph.rs | 12 | Set | None | None |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | OrderedSetStPer.rs | 10 | Set | None | None |
| | *↳ Proposed* | | | `vstd::set::group_set_axioms`<br>`vstd::set_lib::group_set_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | OrderedTableMtEph.rs | 17 | Map | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | OrderedTableMtPer.rs | 21 | Map | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | OrderedTableStEph.rs | 16 | Map | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap43 | OrderedTableStPer.rs | 27 | Map | None | None |
| | *↳ Proposed* | | | `vstd::map::group_map_axioms`<br>`vstd::map_lib::group_map_lib_default` | `crate::vstdplus::feq::feq::group_feq_axioms` |
| Chap18 | ArraySeq.rs | 2 | Seq | Multiset, Seq | Yes |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap37 | AVLTreeSeq.rs | 16 | Seq | Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap37 | AVLTreeSeqMtPer.rs | 12 | Seq | Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap37 | AVLTreeSeqStEph.rs | 15 | Seq | Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap37 | AVLTreeSeqStPer.rs | 14 | Seq | Seq | None |
| | *↳ Proposed* | | | `vstd::seq::group_seq_axioms`<br>`vstd::seq_lib::group_seq_properties`<br>`vstd::seq_lib::group_to_multiset_ensures` | `crate::vstdplus::feq::feq::group_feq_axioms`<br>`crate::vstdplus::seq_set::*` |
| Chap37 | BSTSplayStEph.rs | 11 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap53 | GraphSearchMtPer.rs | 4 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap53 | GraphSearchStEph.rs | 4 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap53 | GraphSearchStPer.rs | 4 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap53 | PQMinStEph.rs | 4 | None | None | None |
| | *↳ Proposed* | | | None | None |
| Chap53 | PQMinStPer.rs | 4 | None | None | None |
| | *↳ Proposed* | | | None | None |
