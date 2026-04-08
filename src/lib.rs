// Copyright (c) 2025 Brian G. Milnes
//! APAS-VERUS library crate
// Verus's rustc exposes Arc<T, A> (allocator_api). Our assume_specification
// for Arc::clone in vstdplus/smart_ptrs.rs must match that exact signature.
// Verus's rustc exposes PointeeSized (sized_hierarchy). Our ExFeq trait in
// vstdplus/feq.rs uses it as a supertrait bound.
#![cfg_attr(verus_keep_ghost, feature(allocator_api))]
#![cfg_attr(verus_keep_ghost, feature(sized_hierarchy))]
// R115 EXPERIMENT: new-mut-ref encoding — compat layer so old ensures syntax works.
#![cfg_attr(verus_keep_ghost, verifier::deprecated_postcondition_mut_ref_style(true))]
// R115 EXPERIMENT: suppress macro_export lint triggered by new-mut-ref pipeline.
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Foundation modules
pub mod Types;
pub mod Concurrency;
#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap02")))]
pub mod ParaPairs;

pub mod experiments {
    // SUCCEEDS
//    pub mod accept;                                       // SUCCEEDS: accept(b) with admit replaces raw assume(b)
//    pub mod arc_clone_deref;                              // SUCCEEDS: Arc::clone preserves deref equality
//    pub mod ArrayVal;                                     // SUCCEEDS: array literals, mutation, and refs verify
//    pub mod ArrayVecSet;                                  // SUCCEEDS: for/while on arrays with length assumptions verify
//    pub mod baseviewtypes;                                // SUCCEEDS: all primitive Views are identity (type V = Self)
//    pub mod biconditional_spec_fun;                       // SUCCEEDS: biconditionals against spec functions
//    pub mod boxing_fns;                                   // SUCCEEDS: which Fn types Verus verifies when boxed
//    pub mod checked_comm;                                 // SUCCEEDS: commutativity lemmas for checked arithmetic
//    pub mod checked_signed_int;                           // SUCCEEDS: checked signed integer trait with overflow
//    pub mod checked_u32;                                  // SUCCEEDS: CheckedU32 implementing CheckedUnsignedInt
//    pub mod checked_unsigned_int;                         // SUCCEEDS: checked unsigned integer trait with overflow
//    pub mod CheckedI32;                                   // SUCCEEDS: checked integer tracking true mathematical value
//    pub mod clone;                                        // SUCCEEDS: broadcast axioms avoid explicit assert(cloned(...)) calls
//    pub mod clone_fn;                                     // SUCCEEDS: clone preserving closure specs
//    pub mod clone_plus_vs_deep_clone;                     // SUCCEEDS: DeepViewClone vs clone_plus comparison
//    pub mod collect;                                      // SUCCEEDS: collect (group-by-key) with two spec families
//    pub mod collect2;                                     // SUCCEEDS: dual loop collect
//    pub mod collect_deep_view;                            // SUCCEEDS: collect using DeepView trait
//    pub mod deep_view_2_tuple;                            // SUCCEEDS: DeepView for 2-tuples
//    pub mod deep_view_struct;                             // SUCCEEDS: DeepView on struct with Option<usize> field
//    pub mod derive_clone_enum_in_verus;                   // SUCCEEDS: #[derive(Clone)] on enum verifies
//    pub mod derive_clone_struct_in_verus;                 // SUCCEEDS: #[derive(Clone)] on struct verifies
//    pub mod derive_clone_struct_with_vec_in_verus;        // SUCCEEDS: #[derive(Clone)] on struct with Vec verifies
//    pub mod derive_copy_enum_in_verus;                    // SUCCEEDS: #[derive(Copy, Clone)] on enum verifies
//    pub mod derive_copy_struct_in_verus;                  // SUCCEEDS: #[derive(Copy, Clone)] on struct verifies
//    pub mod derive_debug_enum_in_verus;                   // SUCCEEDS: #[derive(Debug)] on enum verifies
//    pub mod derive_debug_struct_in_verus;                 // SUCCEEDS: #[derive(Debug)] on struct verifies
//    pub mod derive_debug_struct_with_vec_in_verus;        // SUCCEEDS: #[derive(Debug)] on struct with Vec verifies
//    pub mod derive_default_enum_in_verus;                 // SUCCEEDS: #[derive(Default)] on enum verifies
//    pub mod derive_default_struct_in_verus;               // SUCCEEDS: #[derive(Default)] on struct verifies
//    pub mod derive_default_struct_with_vec_in_verus;      // SUCCEEDS: #[derive(Default)] on struct with Vec verifies
//    pub mod derive_eq_enum_in_verus;                      // SUCCEEDS: #[derive(PartialEq, Eq)] on enum verifies
//    pub mod derive_eq_struct_in_verus;                    // SUCCEEDS: #[derive(PartialEq, Eq)] on struct verifies
//    pub mod derive_eq_struct_with_vec_in_verus;           // SUCCEEDS: #[derive(PartialEq, Eq)] on struct with Vec verifies
//    pub mod derive_hash_enum_in_verus;                    // SUCCEEDS: #[derive(Hash)] on enum verifies
//    pub mod derive_hash_struct_in_verus;                  // SUCCEEDS: #[derive(Hash)] on struct verifies
//    pub mod derive_hash_struct_with_vec_in_verus;         // SUCCEEDS: #[derive(Hash)] on struct with Vec verifies
//    pub mod derive_ord_enum_in_verus;                     // SUCCEEDS: #[derive(Ord)] on enum verifies
//    pub mod derive_ord_struct_in_verus;                   // SUCCEEDS: #[derive(Ord)] on struct verifies
//    pub mod derive_ord_struct_with_vec_in_verus;          // SUCCEEDS: #[derive(Ord)] on struct with Vec verifies
//    pub mod derive_partial_eq_enum_in_verus;              // SUCCEEDS: #[derive(PartialEq)] on enum verifies
//    pub mod derive_partial_eq_struct_in_verus;            // SUCCEEDS: #[derive(PartialEq)] on struct verifies
//    pub mod derive_partial_eq_struct_with_vec_in_verus;   // SUCCEEDS: #[derive(PartialEq)] on struct with Vec verifies
//    pub mod derive_partial_ord_enum_in_verus;             // SUCCEEDS: #[derive(PartialOrd)] on enum verifies
//    pub mod derive_partial_ord_struct_in_verus;           // SUCCEEDS: #[derive(PartialOrd)] on struct verifies
//    pub mod derive_partial_ord_struct_with_vec_in_verus;  // SUCCEEDS: #[derive(PartialOrd)] on struct with Vec verifies
//    pub mod eq_rel;                                       // SUCCEEDS: trait hierarchy for equality relations
//    pub mod exec_spec_verified_test;                      // SUCCEEDS: exec/spec/verified test patterns
//    pub mod external_body_accept_hole;                    // SUCCEEDS: veracity treats accept hole comments
//    pub mod f64_bits_sort;                                // SUCCEEDS: f64 sorting via bit-level ordering (with assumes for IEEE axioms)
//    pub mod ForLoops;                                     // SUCCEEDS: for loops with bounds and array access
//    pub mod hash_set_iter;                                // SUCCEEDS: hash set iteration patterns
//    pub mod hash_set_with_view_plus_loops;                // SUCCEEDS: loop patterns over HashSetWithViewPlus
//    pub mod minimal_iter;                                 // SUCCEEDS: minimal iterator verification
//    pub mod modify_a_ghost_struct;                        // SUCCEEDS: ghost structs can be created, modified, used in specs
//    pub mod mut_ref_hashtable;                            // SUCCEEDS: get_mut/entry patterns with -V new-mut-ref
//    pub mod mut_refs_and_mut_returns;                     // SUCCEEDS: &mut self with old/fin in ensures
//    pub mod parapair_named_closure;                       // SUCCEEDS: named closures with explicit ensures propagate
//    pub mod parapair_toplevel_closure;                    // SUCCEEDS: toplevel closures validate
//    pub mod pervasives;                                   // SUCCEEDS: pervasive utility tests
//    pub mod proof_fn_in_trait;                            // SUCCEEDS: proof fn in trait works
//    pub mod proven_partialeq;                             // SUCCEEDS: ProvenPartialEq pattern
//    pub mod pub_crate_test;                               // SUCCEEDS: pub(crate) test patterns
//    pub mod pub_crate_type_invariant_with_accessors;      // SUCCEEDS: closed spec fn accessors for pub(crate) fields
//    pub mod rwlock_inc_token_inside_lock;                 // SUCCEEDS: increment token inside lock
//    pub mod locked_wrapper_generic;                        // SUCCEEDS: generic LockedWrapper<T>, 1 assume for all Mt
//    pub mod pcell_rwlock_zero_assume;                      // TESTING: PCell+RwLock<PointsTo>, broken Arc type mismatch
//    pub mod state_machine_set_mt;                          // SUCCEEDS: tokenized_state_machine for Mt set, zero assumes
//    pub mod bst_plain_mt_tsm;                              // SUCCEEDS: TSM scaled to 10 ops, zero assumes
//    pub mod coarse_lock_parallel_tsm;                      // SUCCEEDS: coarse lock + TSM + parallel inside
//    pub mod bst_plain_mt_pcell;                            // SUCCEEDS: PCell+RwLock, zero assumes, View experiment
//    pub mod atomic_spinlock_pcell_mt;                      // SUCCEEDS: AtomicBool spinlock + PCell, zero assumes
//    pub mod closure_clone_first_class;                     // FAILS: Verus 3390e9af0 — Clone on closures still not recognized
//    pub mod named_fn_clone_bounds;                         // PARTIAL: named fns — tests 4,6 pass, 1,2,3,5 fail (Clone)
//    pub mod rwlock_no_ghost_field;                         // SUCCEEDS: PCell+PointsTo eliminates assumes
//    pub mod rwlock_tsm;                                   // SUCCEEDS: RwLock TSM pattern
//    pub mod rwlock_tsm_increment;                         // SUCCEEDS: coarse-grained lock caller sees specs via ghost TSM token
//    pub mod seq_array_equality;                           // SUCCEEDS: seq/array equality patterns
//    pub mod seq_for_basic_proofs;                         // SUCCEEDS: for loop over seq with basic proofs
//    pub mod seq_loop_basic_proofs;                        // SUCCEEDS: loop over seq with basic proofs
//    pub mod seq_vec_equality;                             // SUCCEEDS: seq/vec equality
//    pub mod seq_while_basic_proofs;                       // SUCCEEDS: while loop over seq with basic proofs
//    pub mod set_len_empty_both_ways;                      // SUCCEEDS: set length/empty equivalence
//    pub mod sigma_pi;                                     // SUCCEEDS: sigma/pi summation patterns
//    pub mod signed_int;                                   // SUCCEEDS: signed int verification
//    pub mod spec_fun_argument;                            // SUCCEEDS: ghost spec_fn predicate alongside exec closure
//    pub mod spec_loop;                                    // SUCCEEDS: no cycle error in simple case
//    pub mod struct_construction_test;                     // SUCCEEDS: struct construction patterns
//    pub mod struct_rwlock_type_invariant;                 // SUCCEEDS: type_invariant links ghost to locked value
//    pub mod test_feq;                                     // SUCCEEDS: feq test patterns
//    pub mod test_feq_insertion_sort;                      // SUCCEEDS: feq with insertion sort
//    pub mod test_test;                                    // SUCCEEDS: basic test patterns
//    pub mod trait_decreases;                              // SUCCEEDS: recursive trait dispatch with decreases
//    pub mod trait_iter_caller;                            // SUCCEEDS: trait iterator caller verifies
//    pub mod trait_rec_caller;                             // SUCCEEDS: trait recursive caller verifies
//    pub mod trait_rec_vs_iter;                            // SUCCEEDS: rec vs iter naming pattern comparison
//    pub mod tree_module_style;                            // SUCCEEDS: multi-struct spec style with recursive trait dispatch
//    pub mod tree_mut_data_updates;                        // SUCCEEDS: Box<Node> data field mutation verifies
//    pub mod tree_mut_structure_updates;                   // SUCCEEDS: rotations and insert-with-rotation verify
//    pub mod triangle;                                     // SUCCEEDS: triangle verification
//    pub mod unsigned_int;                                 // SUCCEEDS: unsigned int verification
//    pub mod use_proven_partialeq;                         // SUCCEEDS: ProvenPartialEq works as generic bound
//    pub mod vec_filter;                                   // SUCCEEDS: use Anvil style multiset for filter
//    pub mod vec_if;                                       // SUCCEEDS: vec conditional patterns
//    pub mod vec_length_while_rust;                        // SUCCEEDS: vec length while loop (Rust style)
//    pub mod vec_length_while_verus;                       // SUCCEEDS: vec length while loop (Verus style)
//    pub mod vec_remove_duplicates;                        // SUCCEEDS: vec remove duplicates
//    pub mod verus_iterator;                               // SUCCEEDS: Verus iterator patterns
//    pub mod verus_keep_ghost_and_test;                    // SUCCEEDS: verus_keep_ghost and test interaction
//    pub mod verus_sum_loops_iterators;                    // SUCCEEDS: sum loops with iterators
//    pub mod verus_vec_iterator_for_basic_proofs;          // SUCCEEDS: for loop vec iterator proofs
//    pub mod verus_vec_iterator_loop_basic_proofs;         // SUCCEEDS: loop vec iterator proofs
//    pub mod verus_wrapped_iter_loops;                     // SUCCEEDS: wrapped iterator loops
//    pub mod vstd_rwlock_example1;                         // SUCCEEDS: vstd rwlock example 1
//    pub mod vstd_rwlock_example2;                         // SUCCEEDS: vstd rwlock example 2
//    pub mod VSTDLoopProofs;                               // SUCCEEDS: VSTD loop proof patterns
//    pub mod WhileWhile;                                   // SUCCEEDS: nested while loops

    // FAILS
//    pub mod abstract_set_iter;                            // FAILS: vstdplus::vec, clone_view moved to attic
//    pub mod accept_external_body;                         // FAILS: macro cannot produce attribute attaching to item
//    pub mod arc_rwlock_ninject;                           // FAILS: threads contending for single RwLock
//    pub mod arc_rwlock_trivial;                           // FAILS: assumes/external_body at lock boundary
//    pub mod assume_spec_test;                             // FAILS: Verus panic (traits.rs assertion)
//    pub mod clone_plus;                                   // FAILS: postcondition not satisfied (feq_works)
//    pub mod copy_vs_clone_wars;                           // FAILS: Copy doesn't eliminate Clone/PartialEq workaround assumes
//    pub mod derive_display_enum_in_verus;                 // FAILS: derive_more can't link in Verus
//    pub mod derive_display_struct_in_verus;               // FAILS: derive_more can't link in Verus
//    pub mod executable_use_of_int;                        // FAILS: Verus disallows executable use of int
//    pub mod f64_float_cmp_sort;                           // FAILS: le_ensures hostile to invariant maintenance across swaps
//    pub mod f64_sort;                                     // FAILS: assertion failed (f64_le_spec in loop invariant)
//    pub mod ForFor;                                       // FAILS: precondition not satisfied, invariant not satisfied
//    pub mod generic_specs_to_prevent_cycles;              // FAILS: parse error (4c); only free fn (4a) works
//    pub mod ghost_type_invariant;                         // FAILS: type_invariant requires private fields, makes struct opaque
//    pub mod hash_set_modern_pattern;                      // FAILS: uses vstd::std_specs not available in cargo
//    pub mod HashCheckedU32;                               // FAILS: conflicting impls with vstdplus::hashed_checked_u32
//    pub mod invariant_proof_test;                         // FAILS: expected `!` in proof fn
//    pub mod iter_requires_on_external_trait;              // FAILS: Verus rejects requires on external trait impl
//    pub mod parapair_closure_ensures;                     // FAILS: closure ensures don't propagate through ParaPair
//    pub mod parapair_move_closure_ensures;                // FAILS: closure ensures don't propagate with move
//    pub mod possession;                                   // FAILS: missing field `a` in struct initializer
//    pub mod pub_crate_type_invariant;                     // FAILS: field expr for opaque datatype
//    pub mod seq_set_exec;                                 // FAILS: Chap05 not in experiments_only
//    pub mod SetLoops;                                     // FAILS: clone_view moved to attic
//    pub mod simple_hash_set_iter;                         // FAILS: obeys_feq_full, assertion failed
//    pub mod simple_seq_iter;                              // FAILS: clone_view moved to attic
//    pub mod simple_set_iter;                              // FAILS: clone_view, lemma_take_full_to_set
//    pub mod struct_rwlock_arc_with_fn_specs;              // FAILS: assumes/external_body at lock boundary
//    pub mod struct_rwlock_with_fn_specs;                  // FAILS: assumes/external_body at lock boundary
//    pub mod struct_rwlock_with_fn_specs_result;           // FAILS: assumes at lock boundary
//    pub mod struct_rwlock_with_fn_specs_result_handles;   // FAILS: assumes at lock boundary
//    pub mod supertrait;                                   // FAILS: multiple applicable items (supertrait foo)
//    pub mod tcb_foul;                                     // FAILS: Verus blocks unspecified &mut self methods
//    pub mod test_verify_one_file;                         // FAILS: uses rust_verify_test_macros (nightly)
//    pub mod total_ord_gen;                                // FAILS: assertion failed (axiom_cloned_view_eq)
//    pub mod total_ord_gen_axioms;                         // FAILS: trigger must be fn/field/arith
//    pub mod ToVecProof;                                   // FAILS: clone_view moved to attic
//    pub mod vec_clone_in_verus;                           // FAILS: Vec::clone alone cannot prove view equality for generic T
//    pub mod verus_pub_crate_test;                         // FAILS: field expr for opaque datatype
//    pub mod verus_vec_iterator;                           // FAILS: precondition not satisfied (exec_invariant)
//    pub mod verus_vec_iterator_while_basic_proofs;        // FAILS: depends on verus_vec_iterator
//    pub mod vstd_laws_eq_clone;                           // FAILS: reveal() E0401 on generic types
//    pub mod mut_struct_quantifier_limit;                   // FAILS: Z3 17.7GB on &mut 4-field struct with quantified maps
//    pub mod f32_ieee_total_order;                         // FAILS: by(bit_vector) flaky — antisymmetric/add_zero/add_monotone fail
//    pub mod f64_ieee_total_order;                         // FAILS: f64 by(bit_vector) crashes Verus — bitvector_to_air.rs:424
    pub mod new_mut_ref_matching_loop;                     // R115: new-mut-ref matching loop experiment
}

#[cfg(all(not(feature = "experiments_only"), not(feature = "isolate")))]
pub mod standards {
    pub mod arc_usage_standard;
    pub mod capacity_bounds_standard;
//  pub mod constructor_feq_standard;              // comment-only
    pub mod deep_view_standard;
    pub mod finite_sets_standard;
//  pub mod helper_function_placement_standard;    // comment-only
    pub mod hfscheduler_standard;
    pub mod iterative_vs_recursive_standard;
    pub mod iterators_standard;
    pub mod mod_standard;
    pub mod multi_struct_standard;
    pub mod mut_standard;
    pub mod partial_eq_eq_clone_standard;
    pub mod spec_naming_convention;
    pub mod spec_wf_standard;
    pub mod table_of_contents_standard;
    pub mod toplevel_coarse_rwlocks_for_mt_modules;
    pub mod rwlock_tsm_standard;
//  pub mod total_order_standard;                  // comment-only
    pub mod tsm_standard;
    pub mod using_closures_standard;
//  pub mod using_hashmap_standard;                // comment-only
//  pub mod using_rand_standard;                   // comment-only
    pub mod view_standard;
    pub mod wrapping_iterators_standard;
}

pub mod vstdplus {
    pub mod accept;
    pub mod pervasives_plus;
    pub mod threads_plus;
    pub mod VecQueue;
    pub mod seq_set;
    pub mod seq;
    pub mod hash_set_with_view_plus;
    pub mod hash_map_with_view_plus;
    pub mod hash_set_specs;
    pub mod total_order;
    pub mod partial_order;
    pub mod feq;
    pub mod clone_plus;
    pub mod clone_view;
    pub mod smart_ptrs;
    pub mod arc_rwlock;
    pub mod checked_int;
    pub mod checked_nat;
    pub mod hashed_checked_u32;
    pub mod sqrt;
    pub mod float;
    pub mod monoid;
    pub mod multiset;
    pub mod rand;
    pub mod strings;
    pub mod arithmetic {
    pub mod power2_plus;
    }
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap02")))]
pub mod Chap02 {
    pub mod HFSchedulerMtEph;
    pub mod FibonacciHFScheduler;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap03")))]
pub mod Chap03 {
    pub mod InsertionSortStEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap05")))]
pub mod Chap05 {
    pub mod SetStEph;
    pub mod SetMtEph;
    pub mod RelationStEph;
    pub mod MappingStEph;
    pub mod KleeneStPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap06")))]
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
    pub mod WeightedDirGraphStEphF64;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap11")))]
pub mod Chap11 {
    pub mod FibonacciStEph;
    pub mod FibonacciMtPerAllThreads;
    pub mod FibonacciMtPerTSM;
    pub mod FibonacciMtEph2Threads;
    pub mod FibonacciMtEphRecomputes;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap12")))]
pub mod Chap12 {
    pub mod Exercise12_1;
    pub mod Exercise12_2;
    pub mod Exercise12_5;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap17")))]
pub mod Chap17 {
    pub mod MathSeq;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap18")))]
pub mod Chap18 {
    pub mod ArraySeq;
    pub mod ArraySeqStPer;
    pub mod ArraySeqStEph;
    pub mod LinkedListStPer;
    pub mod LinkedListStEph;
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtEphSlice;
    pub mod ArraySeqMtPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap19")))]
pub mod Chap19 {
    pub mod ArraySeqStPer;
    pub mod ArraySeqStEph;
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtEphSlice;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap21")))]
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
    pub mod Problem21_3;
    pub mod Problem21_4;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap23")))]
pub mod Chap23 {
    pub mod PrimTreeSeqStPer;
    pub mod BalBinTreeStEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap26")))]
pub mod Chap26 {
    pub mod DivConReduceStPer;
    pub mod MergeSortStPer;
    pub mod ScanDCStPer;
    pub mod ETSPStEph;
    pub mod ETSPMtEph;
    pub mod DivConReduceMtPer;
    pub mod MergeSortMtPer;
    pub mod ScanDCMtPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap27")))]
pub mod Chap27 {
    pub mod ReduceContractStEph;
    pub mod ReduceContractMtEph;
    pub mod ScanContractStEph;
    pub mod ScanContractMtEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap28")))]
pub mod Chap28 {
    pub mod MCSSSpec;
    pub mod MaxContigSubSumBruteStEph;
    pub mod MaxContigSubSumReducedStEph;
    pub mod MaxContigSubSumDivConStEph;
    pub mod MaxContigSubSumIterStEph;
    pub mod MaxContigSubSumReducedMcsseStEph;
    pub mod MaxContigSubSumDivConOptStEph;
    pub mod MaxContigSubSumOptStEph;
    pub mod MaxContigSubSumOptMtEph;
    pub mod MaxContigSubSumDivConMtEph;
    pub mod MaxContigSubSumDivConOptMtEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap30")))]
pub mod Chap30 {
    pub mod Probability;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap35")))]
pub mod Chap35 {
    pub mod OrderStatSelectStEph;
    pub mod OrderStatSelectStPer;
    pub mod OrderStatSelectMtEph;
    pub mod OrderStatSelectMtPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap36")))]
pub mod Chap36 {
    pub mod QuickSortStEph;
    pub mod QuickSortMtEph;
    pub mod QuickSortMtEphSlice;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap37")))]
pub mod Chap37 {
    pub mod AVLTreeSeq;
    pub mod AVLTreeSeqStEph;
    pub mod AVLTreeSeqStPer;
    pub mod AVLTreeSeqMtPer;
    pub mod BSTPlainStEph;
    pub mod BSTPlainMtEph;
    pub mod BSTAVLStEph;
    pub mod BSTAVLMtEph;
    pub mod BSTRBStEph;
    pub mod BSTRBMtEph;
    pub mod BSTSplayStEph;
    pub mod BSTSplayMtEph;
    pub mod BSTBBAlphaStEph;
    pub mod BSTBBAlphaMtEph;
    pub mod BSTSetPlainMtEph;
    pub mod BSTSetAVLMtEph;
    pub mod BSTSetRBMtEph;
    pub mod BSTSetSplayMtEph;
    pub mod BSTSetBBAlphaMtEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap38")))]
pub mod Chap38 {
    pub mod BSTParaStEph;
    pub mod BSTParaMtEph;
}


#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap39")))]
pub mod Chap39 {
    pub mod BSTTreapStEph;
    pub mod BSTTreapMtEph;
    pub mod BSTParaTreapMtEph;
    pub mod BSTSetTreapMtEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap40")))]
pub mod Chap40 {
    pub mod BSTKeyValueStEph;
    pub mod BSTSizeStEph;
    pub mod BSTReducedStEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap41")))]
pub mod Chap41 {
    pub mod OrdKeyMap;
    pub mod ArraySetStEph;
    pub mod ArraySetEnumMtEph;
    pub mod AVLTreeSetStEph;
    pub mod AVLTreeSetStPer;
    pub mod AVLTreeSetMtEph;
    pub mod AVLTreeSetMtPer;
    pub mod Example41_3;
}


#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap42")))]
pub mod Chap42 {
    pub mod TableSpecsAndLemmas;
    pub mod TableStEph;
    pub mod TableStPer;
    pub mod TableMtEph;
    pub mod Example42_1;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap43")))]
pub mod Chap43 {
    pub mod OrderedTableStEph;
    pub mod OrderedTableMtEph;
    pub mod AugOrderedTableStEph;
    pub mod AugOrderedTableMtEph;
    pub mod OrderedTableStPer;
    pub mod AugOrderedTableStPer;
    pub mod OrderedSetStEph;
    pub mod OrderedSetMtEph;
    pub mod OrderedSetStPer;
    pub mod OrderedTableMtPer;
    pub mod Example43_1;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap44")))]
pub mod Chap44 {
    pub mod DocumentIndex;
    pub mod Example44_1;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap45")))]
pub mod Chap45 {
    pub mod UnsortedListPQ;
    pub mod SortedListPQ;
    pub mod BinaryHeapPQ;
    pub mod BalancedTreePQ;
    pub mod LeftistHeapPQ;
    pub mod HeapsortExample;
    pub mod Example45_2;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap47")))]
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

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap49")))]
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

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap50")))]
pub mod Chap50 {
    pub mod MatrixChainStEph;
    pub mod MatrixChainStPer;
    pub mod MatrixChainMtEph;
    pub mod MatrixChainMtPer;
    pub mod OptBinSearchTreeStEph;
    pub mod OptBinSearchTreeStPer;
    pub mod OptBinSearchTreeMtEph;
    pub mod OptBinSearchTreeMtPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap51")))]
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


#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap52")))]
pub mod Chap52 {
    pub mod AdjTableGraphSpecsAndLemmas;
    pub mod AdjSeqGraphStEph;
    pub mod AdjSeqGraphStPer;
    pub mod AdjSeqGraphMtEph;
    pub mod AdjSeqGraphMtPer;
    pub mod AdjMatrixGraphStEph;
    pub mod AdjMatrixGraphStPer;
    pub mod AdjMatrixGraphMtEph;
    pub mod AdjMatrixGraphMtPer;
    pub mod AdjTableGraphStEph;
    pub mod AdjTableGraphStPer;
    pub mod AdjTableGraphMtPer;
    pub mod EdgeSetGraphStEph;
    pub mod EdgeSetGraphStPer;
    pub mod EdgeSetGraphMtPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap53")))]
pub mod Chap53 {
    pub mod PQMinStEph;
    pub mod PQMinStPer;
    pub mod GraphSearchStEph;
    pub mod GraphSearchStPer;
    pub mod GraphSearchMtPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap54")))]
pub mod Chap54 {
    pub mod BFSStEph;
    pub mod BFSStPer;
    pub mod BFSMtEph;
    pub mod BFSMtPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap55")))]
pub mod Chap55 {
    pub mod DFSStEph;
    pub mod DFSStPer;
    pub mod TopoSortStEph;
    pub mod TopoSortStPer;
    pub mod CycleDetectStEph;
    pub mod CycleDetectStPer;
    pub mod SCCStEph;
    pub mod SCCStPer;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap56")))]
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
    pub mod Example56_1;
    pub mod Example56_3;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap57")))]
pub mod Chap57 {
    pub mod StackStEph;
    pub mod DijkstraStEphU64;
    pub mod DijkstraStEphF64;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap58")))]
pub mod Chap58 {
    pub mod BellmanFordStEphI64;
    pub mod BellmanFordStEphF64;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap59")))]
pub mod Chap59 {
    pub mod JohnsonStEphI64;
    pub mod JohnsonMtEphI64;
    pub mod JohnsonStEphF64;
    pub mod JohnsonMtEphF64;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap61")))]
pub mod Chap61 {
    pub mod EdgeContractionStEph;
    pub mod EdgeContractionMtEph;
    pub mod VertexMatchingStEph;
    pub mod VertexMatchingMtEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap62")))]
pub mod Chap62 {
    pub mod StarPartitionStEph;
    pub mod StarPartitionMtEph;
    pub mod StarContractionStEph;
    pub mod StarContractionMtEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap63")))]
pub mod Chap63 {
    pub mod ConnectivityStEph;
    pub mod ConnectivityMtEph;
}

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap64")))]
pub mod Chap64 {
    pub mod SpanTreeStEph;
    pub mod SpanTreeMtEph;
    pub mod TSPApproxStEph;
}

// Chap65: commented out — union matching loop OOMs full validate (16GB+, killed at 300s).
// Use `scripts/validate.sh isolate Chap65` for development.
// #[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap65")))]
// pub mod Chap65 {
//     pub mod UnionFindStEph;
//     pub mod KruskalStEph;
//     pub mod PrimStEph;
// }

#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap66")))]
pub mod Chap66 {
    pub mod BoruvkaStEph;
    pub mod BoruvkaMtEph;
}
