//! APAS-VERUS library crate
#![cfg_attr(verus_keep_ghost, feature(stmt_expr_attributes))]

pub mod Types;

pub mod experiments {
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
//    pub mod eq_rel;
//    pub mod total_ord_gen;
//    pub mod total_ord_gen_axioms;
//    pub mod test_feq;
//    pub mod test_feq_insertion_sort;
//      pub mod clone_plus;
//      pub mod CheckedI32;
//    pub mod unsigned_int;
//    pub mod signed_int;
//    pub mod checked_unsigned_int;
//    pub mod checked_signed_int;
//    pub mod checked_u32;
    pub mod seq_set_exec;
}

pub mod vstdplus {
    pub mod sigma_pi;
    pub mod set_axioms;
    pub mod seq_set;
    pub mod seq;
    pub mod hash_set_with_view_plus;
    pub mod hash_set_specs;
    pub mod total_order;
    pub mod partial_order;
    pub mod clone_view;
    pub mod feq;
    pub mod clone_plus;
    pub mod vec;
    pub mod checked_int;
    pub mod checked_nat;
    pub mod hashed_checked_u32;
}

pub mod Chap03 {
    pub mod InsertionSortStEph;
}

pub mod Chap05 {
    pub mod SetStEph;
    pub mod RelationStEph;
    pub mod MappingStEph;
}

pub mod Chap06 {
    pub mod DirGraphStEph;
    pub mod UnDirGraphStEph;
    pub mod LabDirGraphStEph;
    pub mod LabUnDirGraphStEph;
    pub mod WeightedDirGraphStEphInt;
    pub mod WeightedUnDirGraphStEphInt;
    pub mod WeightedDirGraphStEphU32;
    pub mod WeightedDirGraphStEphI32;
//     pub mod WeightedDirGraphStEphFloat;  // OrderedFloat - only PartialEq, no Eq
//     pub mod WeightedUnDirGraphStEphFloat;  // OrderedFloat - only PartialEq, no Eq


//     pub mod DirGraphMtEph;
//     pub mod UnDirGraphMtEph;
//     pub mod LabDirGraphMtEph;
//     pub mod LabUnDirGraphMtEph;
//     pub mod WeightedDirGraphMtEphInt;
//     pub mod WeightedDirGraphMtEphFloat;  // OrderedFloat - only PartialEq, no Eq
//     pub mod WeightedUnDirGraphMtEphInt;
//     pub mod WeightedUnDirGraphMtEphFloat;  // OrderedFloat - only PartialEq, no Eq
}
