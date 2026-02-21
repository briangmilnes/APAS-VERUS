// Copyright (c) 2025 Brian G. Milnes
//! APAS-VERUS library crate
// Verus's rustc exposes Arc<T, A> (allocator_api). Our assume_specification
// for Arc::clone in vstdplus/smart_ptrs.rs must match that exact signature.
// Verus's rustc exposes PointeeSized (sized_hierarchy). Our ExFeq trait in
// vstdplus/feq.rs uses it as a supertrait bound.
#![cfg_attr(verus_keep_ghost, feature(allocator_api))]
#![cfg_attr(verus_keep_ghost, feature(sized_hierarchy))]
#![allow(non_snake_case)]

// Foundation modules — always included unless experiments_only
pub mod Types;
pub mod Concurrency;
#[cfg(not(feature = "experiments_only"))]
pub mod ParaPairs;

pub mod experiments {
//    pub mod collect;
//    pub mod clone_fn;
//    pub mod baseviewtypes;
//    pub mod tcb_foul;  // TCB foul experiment - Verus blocks unspecified &mut self methods
//    pub mod pervasives;
//    pub mod sigma_pi;
//    pub mod abstract_set_iter;
//    pub mod SetLoops;
//    pub mod ToVecProof;
//    pub mod supertrait;
//    pub mod minimal_iter;
//    pub mod triangle;
//    pub mod verus_iterator;
//    pub mod verus_vec_iterator;
//    pub mod verus_vec_iterator_while_basic_proofs;
//    pub mod verus_vec_iterator_loop_basic_proofs;
//    pub mod verus_vec_iterator_for_basic_proofs;
//    pub mod verus_sum_loops_iterators;
//    pub mod seq_vec_equality;
//    pub mod executable_use_of_int;
//    pub mod VSTDLoopProofs;
//    pub mod seq_while_basic_proofs;
//    pub mod seq_for_basic_proofs;
//    pub mod seq_loop_basic_proofs;
//    pub mod verus_wrapped_iter_loops;
//    pub mod possession;
//    pub mod simple_seq_iter;
//    pub mod simple_set_iter;
//    pub mod simple_hash_set_iter;
//    pub mod invariant_proof_test;
//    pub mod assume_spec_test; 
//    pub mod struct_construction_test; 
//    pub mod pub_crate_test;
//    pub mod verus_pub_crate_test;  // Has type_invariant failure
//    pub mod hash_set_iter;
//    pub mod hash_set_with_view_plus_loops;
//    pub mod vec_if;
//    pub mod clone;
//    pub mod proof_fn_in_trait;
//    pub mod proven_partialeq;
//    pub mod use_proven_partialeq;
//    pub mod eq_rel;
//    pub mod total_ord_gen;
//    pub mod total_ord_gen_axioms;
//    pub mod test_feq;
//    pub mod test_feq_insertion_sort;
//    pub mod clone_plus;
//    pub mod CheckedI32;
//    pub mod unsigned_int;
//    pub mod signed_int;
//    pub mod checked_unsigned_int;
//    pub mod checked_signed_int;
//    pub mod checked_u32;
//    pub mod seq_set_exec;
//    pub mod hash_set_modern_pattern;  // WIP - uses vstd::std_specs not available in cargo
//    pub mod test_verify_one_file;     // WIP - uses rust_verify_test_macros (nightly)
//    pub mod ghost_type_invariant;     // FAILS - type_invariant makes struct opaque
//    pub mod modify_a_ghost_struct;
//    pub mod parapair_closure_ensures;
//    pub mod parapair_move_closure_ensures;
//    pub mod parapair_named_closure;
//    pub mod parapair_toplevel_closure;
//    pub mod arc_clone_deref;
//    pub mod spec_loop;  // RESULT: no cycle error in simple case
//   pub mod vec_filter;  // RESULT: use Anvil style multiset. 
//   pub mod vec_remove_duplicates;
//   pub mod deep_view_2_tuple;
//    pub mod deep_view_struct;
//    pub mod collect2;
//    pub mod collect_deep_view;
//    pub mod verus_keep_ghost_and_test;
//    pub mod biconditional_spec_fun;
//    pub mod arc_rwlock_ninject;
    // Hypothesis: Can Verus verify f64 sorting using bit-level ordering?
    // Result: Yes. Structural verification (loop invariants, sorted postcondition) works
    // with two classes of assumes: IEEE 754 ordering axioms and an external_body bridge
    // from exec `<=` to spec `to_bits_spec()` ordering. Multiset not proven (orthogonal).
    // pub mod f64_bits_sort;

    // Hypothesis: Can Verus verify f64 sorting using native `<=` and uninterpreted le_ensures?
    // Result: Partially. Comparison bridge works, but le_ensures is hostile to invariant
    // maintenance across Vec::set mutations — the solver can't propagate ordering facts
    // through swaps. Requires assume for sorted-prefix maintenance. The bits approach is
    // cleaner for production use. Led to creation of vstdplus::float broadcast axiom group.
    // pub mod f64_float_cmp_sort;
    pub mod boxing_fns;
    // accept, accept_external_body — Veracity treatment (see Accepted.md)
    pub mod accept;
    // pub mod accept_external_body;  // FAILS - see file header
}

pub mod vstdplus {
    pub mod pervasives_plus;
    pub mod threads_plus;
    pub mod VecQueue;
    pub mod seq_set;
    pub mod seq;
    pub mod hash_set_with_view_plus;
    pub mod hash_set_specs;
    pub mod total_order;
    pub mod partial_order;
    pub mod feq;
    pub mod clone_plus;
    pub mod smart_ptrs;
    pub mod checked_int;
    pub mod checked_nat;
    pub mod hashed_checked_u32;
    pub mod sqrt;
    pub mod float;
    pub mod monoid;
    pub mod multiset;
    pub mod rand;
    pub mod arithmetic {
    pub mod power2_plus;
    }
}

#[cfg(not(feature = "experiments_only"))]
pub mod Chap02 {
    pub mod HFSchedulerMtEph;
    pub mod FibonacciHFScheduler;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap03 {
    pub mod InsertionSortStEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap05 {
    pub mod SetStEph;
    pub mod SetMtEph;
    pub mod RelationStEph;
    pub mod MappingStEph;
    pub mod KleeneStPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap06 {
    pub mod DirGraphStEph;
    pub mod UnDirGraphStEph;
    pub mod LabDirGraphStEph;
    pub mod LabUnDirGraphStEph;
    pub mod WeightedDirGraphStEphU32;
    // MtEph graph modules
    pub mod DirGraphMtEph;
    pub mod UnDirGraphMtEph;
    pub mod LabDirGraphMtEph;
    pub mod LabUnDirGraphMtEph;
    // WeightedDirGraphStEph per-type variants, 
    // just one being verified but they all work.
    pub mod WeightedDirGraphStEphU8;
    pub mod WeightedDirGraphStEphU16;
    pub mod WeightedDirGraphStEphU64;
    pub mod WeightedDirGraphStEphU128;
    pub mod WeightedDirGraphStEphUsize;
    pub mod WeightedDirGraphStEphI8;
    pub mod WeightedDirGraphStEphI16;
    pub mod WeightedDirGraphStEphI32;
    pub mod WeightedDirGraphStEphI64;
    pub mod WeightedDirGraphStEphI128;
    pub mod WeightedDirGraphStEphIsize;
    // Int/Float aggregate graph modules removed: Rust lacks sum types
    // (no `impl Trait for i8 | i16 | i32 | ...`), so these can't be expressed cleanly.
    // Use per-type variants above instead.
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap11 {
    pub mod FibonacciStEph;
    pub mod FibonacciMtPerAllThreads;
    pub mod FibonacciMtPerTSM;
    pub mod FibonacciMtEph2Threads;
    pub mod FibonacciMtEphRecomputes;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap12 {
    pub mod Exercise12_1;
    pub mod Exercise12_2;
    pub mod Exercise12_5;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap17 {
    pub mod MathSeq;
}

#[cfg(not(feature = "experiments_only"))]
pub mod Chap18 {
    pub mod ArraySeq;
    pub mod ArraySeqStPer;
    pub mod ArraySeqStEph;
    pub mod LinkedListStPer;
    pub mod LinkedListStEph;
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtPer;
}

#[cfg(not(feature = "experiments_only"))]
pub mod Chap19 {
    pub mod ArraySeqStPer;
    pub mod ArraySeqStEph;
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtEphSlice;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap21 {
    pub mod Algorithm21_1;
    pub mod Algorithm21_2;
    pub mod Algorithm21_5;
    pub mod Algorithm21_6;
    pub mod Exercise21_5;
    pub mod Exercise21_6;
    pub mod Exercise21_7;
    pub mod Exercise21_8;
    pub mod Exercise21_9;
    pub mod Problem21_1;
//    pub mod Problem21_3;
    pub mod Problem21_4;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap23 {
    pub mod PrimTreeSeqStPer;
    pub mod BalBinTreeStEph;
}

#[cfg(not(feature = "experiments_only"))]
pub mod Chap26 {
    pub mod DivConReduceStPer;
    pub mod MergeSortStPer;
    pub mod ScanDCStPer;
    pub mod ETSPStEph;
    pub mod DivConReduceMtPer;
    pub mod MergeSortMtPer;
    pub mod ScanDCMtPer;
    pub mod ETSPMtEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap27 {
    pub mod ReduceContractStEph;
    pub mod ReduceContractMtEph;
    pub mod ScanContractStEph;
    pub mod ScanContractMtEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap28 {
    pub mod MCSSSpec;
    pub mod MaxContigSubSumBruteStEph;
    pub mod MaxContigSubSumReducedStEph;
    pub mod MaxContigSubSumOptStEph;
    pub mod MaxContigSubSumDivConStEph;
    pub mod MaxContigSubSumIterStEph;
    pub mod MaxContigSubSumReducedMcsseStEph;
    pub mod MaxContigSubSumDivConOptStEph;
    pub mod MaxContigSubSumOptMtEph;
    pub mod MaxContigSubSumDivConMtEph;
    pub mod MaxContigSubSumDivConOptMtEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap35 {
    pub mod OrderStatSelectStEph;
    pub mod OrderStatSelectStPer;
    pub mod OrderStatSelectMtEph;
    pub mod OrderStatSelectMtPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap36 {
    pub mod QuickSortStEph;
    pub mod QuickSortMtEph;
    // pub mod QuickSortMtEphSlice;  // uses rand (Verus can't link)
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap37 {
    pub mod AVLTreeSeq;
    pub mod AVLTreeSeqStEph;
    pub mod AVLTreeSeqStPer;
    #[cfg(feature = "all_chapters")]
    pub mod AVLTreeSeqMtPer;
    pub mod BSTPlainStEph;
    // pub mod BSTPlainMtEph;  // stale imports (was Verus-only, never cargo-compiled)
    pub mod BSTAVLStEph;
    // pub mod BSTAVLMtEph;  // stale imports (was Verus-only, never cargo-compiled)
    pub mod BSTRBStEph;
    #[cfg(feature = "all_chapters")]
    pub mod BSTRBMtEph;
    pub mod BSTSplayStEph;
    #[cfg(feature = "all_chapters")]
    pub mod BSTSplayMtEph;
    pub mod BSTBBAlphaStEph;
    // pub mod BSTBBAlphaMtEph;  // stale imports (was Verus-only, never cargo-compiled)
    #[cfg(feature = "all_chapters")]
    pub mod BSTSetPlainMtEph;
    #[cfg(feature = "all_chapters")]
    pub mod BSTSetAVLMtEph;
    #[cfg(feature = "all_chapters")]
    pub mod BSTSetRBMtEph;
    #[cfg(feature = "all_chapters")]
    pub mod BSTSetSplayMtEph;
    #[cfg(feature = "all_chapters")]
    pub mod BSTSetBBAlphaMtEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap38 {
    pub mod BSTParaStEph;
    pub mod BSTParaMtEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap39 {
    pub mod BSTTreapStEph;
    pub mod BSTTreapMtEph;
    pub mod BSTParaTreapMtEph;
    pub mod BSTSetTreapMtEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap40 {
    pub mod BSTKeyValueStEph;
    pub mod BSTSizeStEph;
    pub mod BSTReducedStEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap41 {
    pub mod ArraySetStEph;
    // pub mod ArraySetEnumMtEph;  // uses bitvec (Verus can't link)
    pub mod AVLTreeSetStEph;
    pub mod AVLTreeSetStPer;
    pub mod AVLTreeSetMtEph;
    #[cfg(feature = "all_chapters")]
    pub mod AVLTreeSetMtPer;
    pub mod Example41_3;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap42 {
    pub mod TableStEph;
    pub mod TableStPer;
    pub mod TableMtEph;
    pub mod Example42_1;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap43 {
    // Wave 1: OrderedTable + AugOrderedTable (depend on ArraySetStEph, AVLTreeSeqStEph/StPer)
    pub mod OrderedTableStEph;
    pub mod OrderedTableMtEph;
    pub mod AugOrderedTableStEph;
    pub mod AugOrderedTableMtEph;
    // Wave 2: StPer variants (depend on TableStPer)
    pub mod OrderedTableStPer;
    pub mod AugOrderedTableStPer;
    // Wave 3: OrderedSet (depend on AVLTreeSetStEph/StPer)
    pub mod OrderedSetStEph;
    pub mod OrderedSetStPer;
    // Wave 4: ParamTreap now uses vstd::rwlock::RwLock, declared inside verus!
    pub mod OrderedSetMtEph;
    pub mod OrderedTableMtPer;
    pub mod Example43_1;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap44 {
    // pub mod DocumentIndex;  // depends on Chap41::AVLTreeSet (types outside verus!)
    // pub mod Example44_1;  // depends on DocumentIndex
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap45 {
    pub mod UnsortedListPQ;
    pub mod SortedListPQ;
    pub mod BinaryHeapPQ;
    pub mod BalancedTreePQ;
    pub mod LeftistHeapPQ;
    pub mod HeapsortExample;
    pub mod Example45_2;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap47 {
    pub mod ChainedHashTable;
    pub mod StructChainedHashTable;
    pub mod VecChainedHashTableStEph;
    pub mod LinkedListChainedHashTableStEph;
    pub mod FlatHashTable;
    pub mod LinProbFlatHashTableStEph;
    pub mod QuadProbFlatHashTableStEph;
    pub mod DoubleHashFlatHashTableStEph;
    pub mod ParaHashTableStEph;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap49 {
    pub mod SubsetSumStEph;
    pub mod SubsetSumStPer;
    pub mod SubsetSumMtEph;
    pub mod SubsetSumMtPer;
    pub mod MinEditDistStEph;
    pub mod MinEditDistStPer;
    pub mod MinEditDistMtEph;
    pub mod MinEditDistMtPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap50 {
    pub mod Probability;
    pub mod MatrixChainStEph;
    pub mod MatrixChainStPer;
    pub mod MatrixChainMtEph;
    pub mod MatrixChainMtPer;
    pub mod OptBinSearchTreeStEph;
    pub mod OptBinSearchTreeStPer;
    pub mod OptBinSearchTreeMtEph;
    pub mod OptBinSearchTreeMtPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap51 {
    pub mod BottomUpDPStEph;
    pub mod BottomUpDPStPer;
    pub mod BottomUpDPMtEph;
    pub mod BottomUpDPMtPer;
    pub mod TopDownDPStEph;
    pub mod TopDownDPStPer;
    pub mod TopDownDPMtEph;
    pub mod TopDownDPMtPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap52 {
    // All graph files use types from Chap37/41/42 declared outside verus!
    // pub mod AdjSeqGraphStEph;
    // pub mod AdjSeqGraphStPer;
    // pub mod AdjSeqGraphMtEph;
    // pub mod AdjSeqGraphMtPer;
    // pub mod AdjTableGraphStEph;
    // pub mod AdjTableGraphStPer;
    // pub mod AdjTableGraphMtPer;
    // pub mod AdjMatrixGraphStEph;
    // pub mod AdjMatrixGraphStPer;
    // pub mod AdjMatrixGraphMtEph;
    // pub mod AdjMatrixGraphMtPer;
    // pub mod EdgeSetGraphStEph;
    // pub mod EdgeSetGraphStPer;
    // pub mod EdgeSetGraphMtPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap53 {
    // All files use types from Chap41/42/45 declared outside verus!
    // pub mod PQMinStEph;
    // pub mod PQMinStPer;
    // pub mod GraphSearchStEph;
    // pub mod GraphSearchStPer;
    // pub mod GraphSearchMtPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap54 {
    pub mod BFSStEph;
    pub mod BFSStPer;
    pub mod BFSMtEph;
    pub mod BFSMtPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap55 {
    // All files use types from Chap37/41 declared outside verus!
    // pub mod DFSStEph;
    // pub mod DFSStPer;
    // pub mod TopoSortStEph;
    // pub mod TopoSortStPer;
    // pub mod CycleDetectStEph;
    // pub mod CycleDetectStPer;
    // pub mod SCCStEph;
    // pub mod SCCStPer;
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap56 {
    pub mod SSSPResultStEphI64;
    pub mod SSSPResultStPerI64;
    pub mod AllPairsResultStEphI64;
    pub mod AllPairsResultStPerI64;
    pub mod PathWeightUtilsStEph;
    pub mod PathWeightUtilsStPer;
    pub mod SSSPResultStEphF64;
    pub mod SSSPResultStPerF64;
    pub mod AllPairsResultStEphF64;
    pub mod AllPairsResultStPerF64;
    // pub mod Example56_1;  // uses ordered_float (removed)
    // pub mod Example56_3;  // uses ordered_float (removed)
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap57 {
    pub mod StackStEph;
    // pub mod DijkstraStEphI64;  // depends on Chap45::BinaryHeapPQ (types outside verus!)
    // pub mod DijkstraStEphF64;  // blocked: no WeightedDirGraphStEphF64 + BinaryHeapPQ
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap58 {
    pub mod BellmanFordStEphI64;
    // pub mod BellmanFordStEphF64;  // blocked: no WeightedDirGraphStEphF64
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap59 {
    // pub mod JohnsonStEphI64;  // depends on DijkstraStEphI64
    // pub mod JohnsonMtEphI64;  // depends on DijkstraStEphI64
    // pub mod JohnsonStEphF64;  // blocked: no WeightedDirGraphStEphF64
    // pub mod JohnsonMtEphF64;  // blocked: no WeightedDirGraphStEphF64
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap61 {
    // pub mod EdgeContractionStEph;  // depends on VertexMatchingStEph
    // pub mod EdgeContractionMtEph;  // depends on VertexMatchingMtEph
    // pub mod VertexMatchingStEph;  // uses rand (Verus can't link)
    // pub mod VertexMatchingMtEph;  // uses rand (Verus can't link)
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap62 {
    pub mod StarPartitionStEph;
    // pub mod StarPartitionMtEph;  // uses rand (Verus can't link)
    pub mod StarContractionStEph;
    // pub mod StarContractionMtEph;  // depends on StarPartitionMtEph
}

#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]
pub mod Chap63 {
    pub mod ConnectivityStEph;
    // pub mod ConnectivityMtEph;  // depends on StarPartitionMtEph
}

#[cfg(feature = "all_chapters")]
pub mod Chap64 {
    pub mod SpanTreeStEph;
    pub mod SpanTreeMtEph;
    // pub mod TSPApproxStEph;  // uses ordered_float (removed)
}

#[cfg(feature = "all_chapters")]
pub mod Chap65 {
    pub mod UnionFindStEph;
    // pub mod KruskalStEph;  // uses ordered_float (removed)
    // pub mod PrimStEph;  // uses ordered_float (removed)
}

#[cfg(feature = "all_chapters")]
pub mod Chap66 {
    // pub mod BoruvkaStEph;  // uses rand + ordered_float (removed)
    // pub mod BoruvkaMtEph;  // uses rand + ordered_float (removed)
}
