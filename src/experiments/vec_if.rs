// Copyright (c) 2025 Brian G. Milnes
//! Minimal experiment: Can we bridge Vec exec indexing to spec?
//!
//! GOAL: Use a broadcast axiom to avoid reveal(obeys_view_eq) in every function.

pub mod vec_if {
    use vstd::prelude::*;
    use vstd::std_specs::vec::vec_index;
    use vstd::laws_eq::*;
    use vstd::laws_cmp::*;
    use vstd::std_specs::cmp::{OrdSpec, PartialEqSpec, PartialOrdSpec};
    use core::cmp::Ordering;

    verus! {

    // Broadcast axiom to reveal obeys_view_eq for all types
    broadcast proof fn axiom_view_eq_auto_reveal<V: Eq + View + Ord>()
        requires 
            obeys_cmp_spec::<V>(),
            #[trigger] obeys_view_eq::<V>()
        ensures 
            forall|v1: V, v2: V|
              #![trigger v1.eq_spec(&v2)]
              v1.eq_spec(&v2) <==> v1@ == v2@
    {
        reveal(obeys_view_eq);
    }

    broadcast group group_view_eq_axioms { axiom_view_eq_auto_reveal}

    // Try WITHOUT reveal - using broadcast axiom
    fn vec_elem_equals<V: Eq + View + Ord>(vec: &Vec<V>, i: usize, v: &V) -> (result: bool)
        requires 
            i < vec.len(),
            obeys_cmp_spec::<V>(),
            obeys_view_eq::<V>(),
        ensures
            result  ==> vec@[i as int]@ == v@,
            !result ==> vec@[i as int]@ != v@,
    {
        broadcast use group_view_eq_axioms;
        
        if vec[i] == *v {
            assert(vec@[i as int]@ == v@);
            true
        } else {
            assert(vec@[i as int]@ != v@);
            false
        }
    }

    // Original version WITH reveal for comparison
    fn vec_elem_equals_with_reveal<V: Eq + View + Ord>(vec: &Vec<V>, i: usize, v: &V) -> (result: bool)
        requires 
            i < vec.len(),
            obeys_cmp_spec::<V>(),
            obeys_view_eq::<V>(),
        ensures
            result  ==> vec@[i as int]@ == v@,
            !result ==> vec@[i as int]@ != v@,
    {
        reveal(obeys_view_eq);
        
        if vec[i] == *v {
            assert(vec@[i as int]@ == v@);
            true
        } else {
            assert(vec@[i as int]@ != v@);
            false
        }
    }

    /// Try with concrete u64 type (like summer_school uses)
    fn vec_u64_equals(vec: Vec<u64>, i: usize, v: u64) -> (result: bool)
        requires i < vec.len(),
        ensures
            result  <==> vec@[i as int] == v,
    {
        if vec[i] == v {
            assert(vec@[i as int] == v);
            true
        } else {
            assert(vec@[i as int] != v);
            false
        }
    }

    } // verus!
}
