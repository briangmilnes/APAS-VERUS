//! Explicit demonstration of what Verus auto-generates for for-loop invariants
//! Based on builtin_macros/src/syntax.rs and vstd/std_specs/hash.rs

pub mod SetLoops {

use vstd::prelude::*;
//use std::hash::Hash;

//use crate::Types::Types::*;
//use crate::Chap05::SetStEph::SetStEph::*;
//use crate::vstdplus::set_with_view::SetWithView::SetWithView;

verus! {

#[cfg(verus_keep_ghost)]
broadcast use {vstd::std_specs::hash::group_hash_axioms, crate::vstdplus::clone_view::clone_view::group_clone_view_axioms};

fn walkvec_while<T>(v: &Vec<T>) -> ()
{
    let mut i: usize = 0;
    while i < v.len()
        invariant
            i <= v.len(),
        decreases v.len() - i,
    {
        i = i + 1;
    }
}

/*
fn walkvec<T>(v: &Vec<T>) -> ()
{
    for x in it: v.iter(){}
}
*/

/*
fn walkset<T: StT + Hash>(s: &SetStEph<T>) -> ()
//      requires s@.len() < usize::MAX,
//    ensures result == s@.len()
{
//    let mut count: usize = 0;
    for x in it: s.iter()
    {
//        assume(count < usize::MAX);
//        count = count + 1;
    }
//    count
}
*/

/*

/// FromVec - working version that verifies (same as SetStEph.rs)
fn FromVecWorking<T: StT + Hash>(v: Vec<T>) -> (result: SetStEph<T>)
    requires vstd::std_specs::hash::obeys_key_model::<T>(),
             forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
    ensures forall |i: int| #![trigger result@.contains(v@[i]@)] 0 <= i < v@.len() ==> result@.contains(v@[i]@)
{
    let mut s = SetStEph::empty();
    for idx in iter: 0..v.len()
        invariant
            forall |j: int| #![trigger s@.contains(v@[j]@)] 0 <= j < iter.cur ==> s@.contains(v@[j]@),
    {
        s.insert(v[idx].clone());
    }
    s
}

/// FromVec with #[verifier::no_auto_loop_invariant] - need to add bounds manually
fn FromVecNoAuto<T: StT + Hash>(v: Vec<T>) -> (result: SetStEph<T>)
    requires vstd::std_specs::hash::obeys_key_model::<T>(),
             forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
    ensures forall |i: int| #![trigger result@.contains(v@[i]@)] 0 <= i < v@.len() ==> result@.contains(v@[i]@)
{
    let mut s = SetStEph::empty();
    #[verifier::no_auto_loop_invariant]
    for idx in iter: 0..v.len()
        invariant
            // MUST manually add what ghost_invariant would have given us:
            iter.start == 0,                 // Iterator started at 0
            iter.end == v@.len(),            // Iterator ends at v.len()
            0 <= iter.cur <= iter.end,       // Current position is in bounds
            
            // MANUAL invariant:
            forall |j: int| #![trigger s@.contains(v@[j]@)] 0 <= j < iter.cur ==> s@.contains(v@[j]@),
    {
        s.insert(v[idx].clone());
    }
    s
}


/// Cardinality - simple count with one for loop, auto-generated invariants
fn Cardinality<T: StT + Hash>(s: &SetStEph<T>) -> (result: usize)
      requires s@.len() < usize::MAX,
//    ensures result == s@.len()
{
    let mut count: usize = 0;
    for _x in it: s.iter()
    {
        assume(count < usize::MAX);
        count = count + 1;
    }
    count
}

*/

// CartesianProduct with #[verifier::no_auto_loop_invariant] 
// Shows what invariants we must add explicitly when disabling auto-generation
/*
fn CartesianProductExplicit<T: StT + Hash, U: StT + Hash>(self_set: &SetStEph<T>,other_set: &SetStEph<U>) -> (result: SetStEph<Pair<T, U>>)
    requires vstd::std_specs::hash::obeys_key_model::<Pair<T, U>>(),
             forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
             forall|u1: U, u2: U| u1@ == u2@ ==> u1 == u2,
             forall|p1: Pair<T, U>, p2: Pair<T, U>| p1@ == p2@ ==> p1 == p2,
    ensures forall|t: T::V, u: U::V| 
        #![trigger result@.contains((t, u))]
        result@.contains((t, u)) <==> (self_set@.contains(t) && other_set@.contains(u))
{
    let mut result = SetStEph::empty();
    let self_iter = self_set.iter();
    #[verifier::no_auto_loop_invariant]
    for x in it_x: self_iter
        invariant
            // MUST add what ghost_invariant would have given us:
            0 <= it_x.pos <= it_x.elements.len(),
            
            // MANUAL invariants:
            forall |i: int, v: U::V| 
                #![trigger result@.contains((it_x.elements[i]@, v))]
                0 <= i < it_x.pos && other_set@.contains(v) ==> 
                    result@.contains((it_x.elements[i]@, v)),
    {
        let other_iter = other_set.iter();
        #[verifier::no_auto_loop_invariant]
        for y in it_y: other_iter
            invariant
                // MUST add what ghost_invariant would have given us:
                0 <= it_x.pos <= it_x.elements.len(),
                0 <= it_y.pos <= it_y.elements.len(),
                
                // MANUAL invariants:
                forall |i: int, v: U::V| 
                    #![trigger result@.contains((it_x.elements[i]@, v))]
                    0 <= i < it_x.pos && other_set@.contains(v) ==> 
                        result@.contains((it_x.elements[i]@, v)),
                forall |j: int| 
                    #![trigger result@.contains((x@, it_y.elements[j]@))]
                    0 <= j < it_y.pos ==>
                        result@.contains((x@, it_y.elements[j]@)),
        {
            result.insert(Pair(x.clone(), y.clone()));
        }
    }
    result
}

// The same function but with #[verifier::no_auto_loop_invariant]
// This disables the AUTO-GENERATED ghost_invariant but keeps exec_invariant
//#[verifier::loop_isolation(false)]
fn CartesianProductNoAuto<T: StT + Hash, U: StT + Hash>(
    self_set: &SetStEph<T>, 
    other_set: &SetStEph<U>
) -> (result: SetStEph<Pair<T, U>>)
    requires vstd::std_specs::hash::obeys_key_model::<Pair<T, U>>(),
             forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
             forall|u1: U, u2: U| u1@ == u2@ ==> u1 == u2,
             forall|p1: Pair<T, U>, p2: Pair<T, U>| p1@ == p2@ ==> p1 == p2,
    ensures forall|t: T::V, u: U::V| 
        #![trigger result@.contains((t, u))]
        result@.contains((t, u)) <==> (self_set@.contains(t) && other_set@.contains(u))
{
    let mut result = SetStEph::empty();
    let self_iter = self_set.iter();
    #[verifier::no_auto_loop_invariant]
    for x in it_x: self_iter
        invariant
            // ONLY exec_invariant remains (automatically):
            // it_x.pos == self_iter@.0,
            // it_x.elements == self_iter@.1,
            
            // NO ghost_invariant (disabled by no_auto_loop_invariant)
            
            // MANUAL invariants (with bounds for iterator access):
            0 <= it_x.pos <= it_x.elements.len(),
            forall |i: int, v: U::V| 
                #![trigger result@.contains((it_x.elements[i]@, v))]
                0 <= i < it_x.pos && other_set@.contains(v) ==> 
                    result@.contains((it_x.elements[i]@, v)),
    {
        let other_iter = other_set.iter();
        #[verifier::no_auto_loop_invariant]
        for y in it_y: other_iter
            invariant
                // ONLY exec_invariant remains
                
                // MANUAL invariants (with bounds for iterator access):
                0 <= it_x.pos <= it_x.elements.len(),
                0 <= it_y.pos <= it_y.elements.len(),
                forall |i: int, v: U::V| 
                    #![trigger result@.contains((it_x.elements[i]@, v))]
                    0 <= i < it_x.pos && other_set@.contains(v) ==> 
                        result@.contains((it_x.elements[i]@, v)),
                forall |j: int| 
                    #![trigger result@.contains((x@, it_y.elements[j]@))]
                    0 <= j < it_y.pos ==>
                        result@.contains((x@, it_y.elements[j]@)),
        {
            result.insert(Pair(x.clone(), y.clone()));
        }
    }
    result
}
*/

} // verus!
}

