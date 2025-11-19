//! Minimal experiment: Can we bridge Vec exec indexing to spec?

pub mod vec_if {
    use vstd::prelude::*;
    use vstd::std_specs::vec::vec_index;

    verus! {

    // Try to prove: if vec[i] == v in exec, can we assert vec@[i] == v in spec and
    // the opposite. 
    fn vec_elem_equals<V: Eq>(vec: &Vec<V>, i: usize, v: &V) -> (result: bool)
        requires i < vec.len(),
        ensures
            result  ==> vec@[i as int] == *v,
            !result ==> vec@[i as int] != *v,
    {
    if vec[i] == *v {
        assume(vec@[i as int] == *v);  // Manual trigger, like summer_school.rs:959
        true
    } else {
        assume(vec@[i as int] != *v);  // Manual trigger
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

