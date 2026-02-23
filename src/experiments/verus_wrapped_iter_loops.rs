// Copyright (c) 2025 Brian G. Milnes
#![allow(unused_imports)]
use vstd::prelude::*;

verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

pub open spec fn seq_i64_mem(s: Seq<i64>, elt: i64) -> bool {
    exists|i: int| 0 <= i < s.len() && s[i] == elt
}

proof fn lemma_found_at_pos(original_seq: Seq<i64>, elt: i64, pos: int)
    requires
        0 <= pos < original_seq.len(),
        original_seq[pos] == elt,
    ensures
        seq_i64_mem(original_seq, elt) == true,
{
    admit();
}

proof fn lemma_not_found(original_seq: Seq<i64>, elt: i64)
    requires
        forall|j: int| 0 <= j < original_seq.len() ==> original_seq[j] != elt,
    ensures
        seq_i64_mem(original_seq, elt) == false,
{
    admit();
}

// While version - matches seq_while_basic_proofs::i64_vec_mem_while
pub fn i64_vec_mem_while(s: Vec<i64>, elt: i64) -> (result: bool)
    ensures result == seq_i64_mem(s@, elt)
{
    let mut i: usize = 0;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> s@[j] != elt,
        decreases s@.len() - i,
    {
        if s[i] == elt {
            return true;
        }
        i += 1;
    }
    false
}

// Loop version - matches seq_loop_basic_proofs::i64_vec_mem_loop
pub fn i64_vec_mem_loop(s: Vec<i64>, elt: i64) -> (result: bool)
    ensures result == seq_i64_mem(s@, elt)
{
    let mut i: usize = 0;
    loop
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> s@[j] != elt,
        decreases s@.len() - i,
    {
        if i >= s.len() {
            return false;
        }
        if s[i] == elt {
            return true;
        }
        i += 1;
    }
}

// For loop version - matches seq_for_basic_proofs::i64_vec_mem_for
pub fn i64_vec_mem_for(s: Vec<i64>, elt: i64) -> (result: bool)
    ensures result == seq_i64_mem(s@, elt)
{
    for i in 0..s.len()
        invariant
            forall|j: int| 0 <= j < i ==> s@[j] != elt,
    {
        if s[i] == elt {
            return true;
        }
    }
    false
}

// For loop with no_auto_loop_invariant - full manual expansion of what Verus generates
// This demonstrates what Verus auto invariants do for `for i in 0..s.len()` loops:
//
// 1. Creates explicit Range<usize> iterator: range_iter
// 2. Creates ghost iterator: RangeGhostIterator<usize> linked to range_iter
// 3. Maintains exec_invariant: connects ghost state to executable iterator state
// 4. Maintains ghost_invariant: tracks relationship to initial ghost state
// 5. Tracks bounds: ghost_iter.cur <= ghost_iter.end, ghost_iter.end == s@.len()
// 6. Advances ghost_iter after each next() call (in Some branch)
// 7. Uses return (not break) to avoid break-invariant problems
//
// Note: The connection between "range_iter.next() == None" and "ghost_iter.cur == ghost_iter.end"
// requires understanding ForLoopGhostIterator::ghost_peek_next and ghost_ensures specs.
pub fn i64_vec_mem_for_no_auto(s: Vec<i64>, elt: i64) -> (result: bool)
    ensures result == seq_i64_mem(s@, elt)
{
    use vstd::pervasive::{ForLoopGhostIteratorNew, ForLoopGhostIterator};
    use vstd::std_specs::range::RangeGhostIterator;
    use std::ops::Range;
    
    let mut range_iter: Range<usize> = 0..s.len();
    let ghost mut ghost_iter: RangeGhostIterator<usize> = ForLoopGhostIteratorNew::ghost_iter(&range_iter);
    let ghost init_ghost_iter = ghost_iter;
    
    loop
        invariant
            ForLoopGhostIterator::exec_invariant(&ghost_iter, &range_iter),
            ForLoopGhostIterator::ghost_invariant(&ghost_iter, Some(&init_ghost_iter)),
            ghost_iter.cur <= ghost_iter.end,
            ghost_iter.end == s@.len(),
            0 <= ghost_iter.cur,
            forall|j: int| 0 <= j < ghost_iter.cur ==> s@[j] != elt,
        decreases ghost_iter.end - ghost_iter.cur,
    {
        match range_iter.next() {
            Some(i) => {
                proof { ghost_iter = ForLoopGhostIterator::ghost_advance(&ghost_iter, &range_iter); }
                if s[i] == elt {
                    return true;
                }
            }
            None => {
                // Iterator exhausted - all elements checked
                assert(ghost_iter.cur == ghost_iter.end);
                assert(forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt);
                return false;
            }
        }
    }
}

// Vec's IntoIter version - uses Vec::into_iter() instead of Range
// This is what `for val in vec` desugars to in Rust (consumes the Vec)
// In Verus, for loops over Vec use IntoIterGhostIterator ghost state
//
// Key differences from Range version:
// - Ghost iterator is IntoIterGhostIterator<i64, Global> not RangeGhostIterator<usize>
// - Fields are .pos and .elements instead of .cur and .end
// - Loop variable is the actual value (val: i64) not index (i: usize)
// - Vec is consumed by into_iter(), so we capture s@ as original_seq first
//
// The postcondition refers to s@, but s is consumed by into_iter().
// We prove properties about original_seq but must bridge to s@ in ensures.
// The bridge invariant (original_seq == s@) is essential.

/* VERUS DOESN'T SUPPORT THIS - see error below
// The natural for-in loop over Vec - what we'd write in normal Rust:
pub fn i64_vec_mem_for_into_iter_natural(s: Vec<i64>, elt: i64) -> (result: bool)
    ensures result == seq_i64_mem(s@, elt)
{
    for val in s {
        if val == elt {
            return true;
        }
    }
    false
}

// Verus error:
// error: `alloc::vec::impl&%17::into_iter` is not supported
//   --> src/experiments/verus_wrapped_iter_loops.rs:XX:YY
//    |
// XX |     for val in s {
//    |  _____^          -
//    |
//    = help: The following declaration may resolve this error:
//            pub assume_specification<'a, T, A> [<&'a std::vec::Vec<T, A> as std::iter::IntoIterator>::into_iter]
//
// Verus requires an assume_specification for Vec::into_iter to use for-in loops.
// Instead, we manually desugar to loop + next() below.
*/

pub fn i64_vec_mem_for_into_iter(s: Vec<i64>, elt: i64) -> (result: bool)
    ensures result == seq_i64_mem(s@, elt)
{
    use vstd::pervasive::{ForLoopGhostIteratorNew, ForLoopGhostIterator};
    use vstd::std_specs::vec::IntoIterGhostIterator;
    use std::alloc::Global;
    
    let ghost original_seq = s@;
    let mut vec_iter = s.into_iter();
    let ghost mut ghost_iter: IntoIterGhostIterator<i64, Global> = ForLoopGhostIteratorNew::ghost_iter(&vec_iter);
    let ghost init_ghost_iter = ghost_iter;
    
    // After into_iter(), IntoIterGhostIterator should have ghost_iter.elements == original_seq
    // but the postcondition refers to s@ (which was moved). Need axiom that view is preserved.
    assert(ghost_iter.elements == original_seq);
    
    loop
        invariant
            ForLoopGhostIterator::exec_invariant(&ghost_iter, &vec_iter),
            ForLoopGhostIterator::ghost_invariant(&ghost_iter, Some(&init_ghost_iter)),
            ghost_iter.pos <= ghost_iter.elements.len(),
            ghost_iter.elements == original_seq,
            forall|j: int| 0 <= j < ghost_iter.pos ==> ghost_iter.elements[j] != elt,
            original_seq == s@,
        decreases ghost_iter.elements.len() - ghost_iter.pos,
    {
        match vec_iter.next() {
            Some(val) => {
                proof { ghost_iter = ForLoopGhostIterator::ghost_advance(&ghost_iter, &vec_iter); }
                if val == elt {
                    // val came from ghost_iter.elements which equals original_seq
                    assert(seq_i64_mem(original_seq, elt));
                    return true;
                }
            }
            None => {
                // Iterator exhausted - all elements in original_seq checked
                assert(forall|j: int| 0 <= j < original_seq.len() ==> original_seq[j] != elt);
                assert(!seq_i64_mem(original_seq, elt));
                return false;
            }
        }
    }
}

}
