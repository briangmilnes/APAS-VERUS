// Copyright (c) 2025 Brian G. Milnes
//! APAS-VERUS library crate
#![cfg_attr(verus_keep_ghost, feature(sized_hierarchy))]

pub mod Types;
pub mod Concurrency;
pub mod ParaPairs;

pub mod experiments {
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
    pub mod checked_int;
    pub mod checked_nat;
    pub mod hashed_checked_u32;
    pub mod arithmetic {
    pub mod power2_plus;
    }
}

pub mod Chap02 {
    pub mod WSSchedulerMtEph;
    pub mod FibonacciWSScheduler;
}

pub mod Chap03 {
    pub mod InsertionSortStEph;
}

pub mod Chap05 {
    pub mod SetStEph;
    pub mod SetMtEph;
    pub mod RelationStEph;
    pub mod MappingStEph;
}

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
//    pub mod WeightedDirGraphStEphU16;
//    pub mod WeightedDirGraphStEphU64;
//    pub mod WeightedDirGraphStEphU128;
//    pub mod WeightedDirGraphStEphUsize;
//    pub mod WeightedDirGraphStEphI8;
//    pub mod WeightedDirGraphStEphI16;
//    pub mod WeightedDirGraphStEphI32;
//    pub mod WeightedDirGraphStEphI64;
//    pub mod WeightedDirGraphStEphI128;
//    pub mod WeightedDirGraphStEphIsize;
}

pub mod Chap11 {
    pub mod FibonacciStEph;
    pub mod FibonacciMtPerAllThreads;
    pub mod FibonacciMtPerTSM;
    pub mod FibonacciMtEph2Threads;
    pub mod FibonacciMtEphRecomputes;
}

pub mod Chap12 {
    pub mod Exercise12_1;
    pub mod Exercise12_2;
    pub mod Exercise12_5;
}

pub mod Chap17 {
    pub mod MathSeq;
}

pub mod Chap18 {
    pub mod ArraySeq;
    pub mod ArraySeqStPer;
    pub mod ArraySeqStEph;
    pub mod LinkedListStPer;
    pub mod LinkedListStEph;
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtPer;
}

pub mod Chap19 {
    pub mod ArraySeqStPer;
    pub mod ArraySeqStEph;
    pub mod ArraySeqMtEph;
}

pub mod Chap21 {
    pub mod Algorithm21_1;
    // pub mod Algorithm21_2;
    // pub mod Algorithm21_5;
    // pub mod Algorithm21_6;
    // pub mod Exercise21_5;
    // pub mod Exercise21_6;
    // pub mod Exercise21_7;
    // pub mod Exercise21_8;
    // pub mod Exercise21_9;
    // pub mod Problem21_1;
    // pub mod Problem21_3;
    // pub mod Problem21_4;
}
