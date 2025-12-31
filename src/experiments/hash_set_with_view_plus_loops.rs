// Copyright (c) 2025 Brian G. Milnes
//! Testing HashSetWithViewPlus with for loops
//!
//! STATUS: Potentially obsolete - investigating removing HashSetWithViewPlus
//! in favor of using std::collections::HashSet directly with deep_view().

pub mod hash_set_with_view_plus_loops {

use vstd::prelude::*;
use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
use vstd::std_specs::hash::obeys_key_model;
use vstd::std_specs::hash::SetIterAdditionalSpecFns;

#[cfg(verus_keep_ghost)]
use vstd::pervasive::ForLoopGhostIteratorNew;
#[cfg(verus_keep_ghost)]
use vstd::pervasive::ForLoopGhostIterator;

verus! {

broadcast use vstd::seq_lib::group_seq_properties;

#[verifier::external_body]
fn vec_to_vec_copy_fails(source: &Vec<u32>) -> (dest: Vec<u32>)
    ensures dest@ == source@
{
    let mut dest = Vec::<u32>::new();
    for item in iter: source.iter()
        invariant
            dest@ == iter@,
    {
        dest.push(*item);
    }
    
    dest
}

fn vec_to_vec_copy(source: &Vec<u32>) -> (dest: Vec<u32>)
    ensures dest@ == source@
{
    let mut dest = Vec::<u32>::new();
    
    let source_iter = source.iter();
    let ghost g_elements: Seq<u32> = source_iter@.1;

    for item in iter: source_iter
        invariant
            iter.elements == g_elements,
            dest@ == iter@,
    {
        dest.push(*item);
    }
    
    dest
}

fn vec_to_vec_copy_stepped()
{
    let mut source = Vec::<u32>::new();
    source.push(2);
    source.push(3);
    
    let mut source_iter = source.iter();
    assert(source_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = source_iter@.1;
    assert(g_elements == source@);
    
    let mut dest = Vec::<u32>::new();
    
    // Step 0: Initial state
    let ghost iter_ghost_0 = source_iter.ghost_iter();
    assert(iter_ghost_0.pos == source_iter@.0);
    assert(iter_ghost_0.elements == source_iter@.1);
    assert(iter_ghost_0.exec_invariant(&source_iter));
    assert(iter_ghost_0.pos == 0);
    assert(iter_ghost_0.elements == g_elements);
    assert(iter_ghost_0@ == g_elements.take(0));
    assert(iter_ghost_0@.len() == 0);
    assert(dest@ == iter_ghost_0@);  // Initial: empty vec == empty sequence
    
    // Step 1: After first next()
    let ghost old_source_iter_view = source_iter@;
    let item1 = source_iter.next();
    let ghost iter_ghost_1 = iter_ghost_0.ghost_advance(&source_iter);
    proof {
        assert(iter_ghost_0.pos == old_source_iter_view.0);
        if item1.is_some() {
            let (old_index, old_seq) = old_source_iter_view;
            let (new_index, new_seq) = source_iter@;
            assert(new_index == old_index + 1);
            assert(iter_ghost_1.pos == iter_ghost_0.pos + 1);
            assert(iter_ghost_1.pos == new_index);
            assert(iter_ghost_1.pos == source_iter@.0);
        }
    }
    assert(iter_ghost_1.elements == source_iter@.1);
    assert(iter_ghost_1.exec_invariant(&source_iter));
    assert(iter_ghost_1.pos == 1);
    assert(iter_ghost_1.elements == g_elements);
    assert(iter_ghost_1@ == g_elements.take(1));
    if item1.is_some() {
        assert(g_elements.len() >= 1);
        assert(iter_ghost_1@.len() == 1);
        assert(iter_ghost_1@[0] == *item1.unwrap());
        dest.push(*item1.unwrap());
        assert(dest@ == iter_ghost_1@);
    }
    
    // Step 2: After second next()
    let ghost old_source_iter_view_2 = source_iter@;
    let item2 = source_iter.next();
    let ghost iter_ghost_2 =
        if item2.is_some() {
            iter_ghost_1.ghost_advance(&source_iter)
        } else {
            iter_ghost_1 
        };
    
    if item2.is_some() {
        proof {
            assert(iter_ghost_1.pos == old_source_iter_view_2.0);
            let (old_index, old_seq) = old_source_iter_view_2;
            let (new_index, new_seq) = source_iter@;
            assert(new_index == old_index + 1); 
            assert(iter_ghost_2.pos == iter_ghost_1.pos + 1);
            assert(iter_ghost_2.pos == new_index);
            assert(iter_ghost_2.pos == source_iter@.0);
        }
        assert(iter_ghost_2.elements == source_iter@.1);
        assert(iter_ghost_2.exec_invariant(&source_iter));
        assert(iter_ghost_2.pos == 2);
        assert(iter_ghost_2.elements == g_elements);
        assert(iter_ghost_2@ == g_elements.take(2));
        assert(g_elements.len() >= 2);
        assert(iter_ghost_2@.len() == 2);
        assert(iter_ghost_2@[1] == *item2.unwrap());
        
        dest.push(*item2.unwrap());
        assert(dest@ == iter_ghost_2@);
    }
    
    // Step 3: Third next() should return None
    if item2.is_some() {
        assert(source_iter@.0 == 2);
        assert(source_iter@.1 == g_elements);
        let item3 = source_iter.next();
        if item3.is_none() {
            assert(2 >= g_elements.len());
            assert(g_elements.len() == 2);
            assert(iter_ghost_2@ == g_elements.take(2));
            assert(g_elements.take(2) == g_elements);
            assert(iter_ghost_2@ == g_elements);
            assert(dest@ == iter_ghost_2@);
            assert(dest@ == source@);
        }
    }
}


fn vec_add_one(source: &Vec<u32>) -> (dest: Vec<u32>)
    requires forall|i: int| 0 <= i < source@.len() ==> source@[i] < u32::MAX
    ensures dest@ == source@.map(|i: int, x: u32| (x + 1) as u32)
{
    let source_iter = source.iter();
    let ghost g_elements: Seq<u32> = source_iter@.1;
    
    let mut dest = Vec::<u32>::new();
    
    for item in iter: source_iter
        invariant
            iter.elements == g_elements,
            dest@ == iter@.map(|i: int, x: u32| (x + 1) as u32),
            forall|i: int| 0 <= i < g_elements.len() ==> g_elements[i] < u32::MAX,
    {
        dest.push(*item + 1);
    }
    
    dest
}

fn hash_set_copy(source: &std::collections::HashSet<u32>) -> (dest: std::collections::HashSet<u32>)
    ensures dest@ == source@
{
    let source_iter = source.iter();
    let ghost original_seq = source_iter@.1;
    
    let mut dest = std::collections::HashSet::<u32>::new();
    
    for item in iter: source_iter
        invariant
            iter.elements == original_seq,
            dest@ == iter@.to_set(),
    {
        dest.insert(*item);
    }
    
    dest
}

fn hash_set_add_one(source: &std::collections::HashSet<u32>) -> (dest: std::collections::HashSet<u32>)
    requires forall|x: u32| source@.contains(x) ==> x < u32::MAX
    ensures forall|x: u32| source@.contains(x) ==> dest@.contains((x + 1) as u32)
{
    let source_iter = source.iter();
    let ghost original_seq = source_iter@.1;
    
    let mut dest = std::collections::HashSet::<u32>::new();
    
    for item in iter: source_iter
        invariant
            iter.elements == original_seq,
            original_seq.to_set() == source@,
            forall|x: u32| source@.contains(x) ==> x < u32::MAX,
            forall|i: int| 0 <= i < iter.pos ==> #[trigger] dest@.contains((original_seq[i] + 1) as u32),
    {
        assert(source@.contains(*item)); // Need to know it's not going to overflow.
        dest.insert(*item + 1);
    }
    
    dest
}

fn hash_set_copy_stepped ()
{

    let mut source: std::collections::HashSet<u32> = std::collections::HashSet::<u32>::new();
    source.insert(3);
    source.insert(6);
    
    let mut source_iter: std::collections::hash_set::Iter<'_, u32> = source.iter();
    assert(source_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = source_iter@.1;
    assert(g_elements.to_set() =~= source@);
    
    let mut dest: std::collections::HashSet<u32> = std::collections::HashSet::<u32>::new();
    
    // Step 0: Initial state
    let ghost iter_ghost_0: vstd::std_specs::hash::SetIterGhostIterator<u32> = source_iter.ghost_iter();
    assert(iter_ghost_0.pos == source_iter@.0);
    assert(iter_ghost_0.elements == source_iter@.1);
    assert(iter_ghost_0.exec_invariant(&source_iter));
    assert(iter_ghost_0.pos == 0);
    assert(iter_ghost_0.elements == g_elements);
    assert(iter_ghost_0@ == g_elements.take(0));
    assert(iter_ghost_0@.len() == 0);
    assert(dest@ == iter_ghost_0@.to_set());  // Initial: empty set == empty sequence
    
    // Step 1: After first next()
    let ghost old_source_iter_view = source_iter@;
    let item1: Option<&u32> = source_iter.next();
    let ghost iter_ghost_1: vstd::std_specs::hash::SetIterGhostIterator<u32> = iter_ghost_0.ghost_advance(&source_iter);
    proof {
        assert(iter_ghost_0.pos == old_source_iter_view.0);
        if item1.is_some() {
            let (old_index, old_seq) = old_source_iter_view;
            let (new_index, new_seq) = source_iter@;
            assert(new_index == old_index + 1);
            assert(iter_ghost_1.pos == iter_ghost_0.pos + 1);
            assert(iter_ghost_1.pos == new_index);
            assert(iter_ghost_1.pos == source_iter@.0);
        }
    }
    assert(iter_ghost_1.elements == source_iter@.1);
    assert(iter_ghost_1.exec_invariant(&source_iter));
    assert(iter_ghost_1.pos == 1);
    assert(iter_ghost_1.elements == g_elements);
    assert(iter_ghost_1@ == g_elements.take(1));
    if item1.is_some() {
        assert(g_elements.len() >= 1);
        assert(iter_ghost_1@.len() == 1);  // Now the lemma can apply!
        assert(iter_ghost_1@[0] == *item1.unwrap());
        let ghost old_dest = dest@;
        dest.insert(*item1.unwrap());
        assert(dest@ == iter_ghost_1@.to_set());
    }
    
    // Step 2: After second next()
    let ghost old_source_iter_view_2 = source_iter@;
    let item2: Option<&u32> = source_iter.next();
    let ghost iter_ghost_2: vstd::std_specs::hash::SetIterGhostIterator<u32> =
        if item2.is_some() {
            iter_ghost_1.ghost_advance(&source_iter)
        } else {
            iter_ghost_1 
        };
    
    if item2.is_some() {
        // Prove synchronization (same as step 1):
        proof {
            assert(iter_ghost_1.pos == old_source_iter_view_2.0);
            let (old_index, old_seq) = old_source_iter_view_2;
            let (new_index, new_seq) = source_iter@;
            assert(new_index == old_index + 1); 
            assert(iter_ghost_2.pos == iter_ghost_1.pos + 1);
            assert(iter_ghost_2.pos == new_index);
            assert(iter_ghost_2.pos == source_iter@.0);
        }
        assert(iter_ghost_2.elements == source_iter@.1);
        assert(iter_ghost_2.exec_invariant(&source_iter));
        assert(iter_ghost_2.pos == 2);
        assert(iter_ghost_2.elements == g_elements);
        assert(iter_ghost_2@ == g_elements.take(2));
        assert(g_elements.len() >= 2);
        assert(iter_ghost_2@.len() == 2);
        assert(iter_ghost_2@[1] == *item2.unwrap());
        
        let ghost old_dest = dest@;
        dest.insert(*item2.unwrap());
        assert(dest@ == iter_ghost_2@.to_set());
    }
    
    // Step 3: Third next() should return None (iterator exhausted)
    if item2.is_some() {
        assert(source_iter@.0 == 2); 
        assert(source_iter@.1 == g_elements); 
        let item3: Option<&u32> = source_iter.next();
        if item3.is_none() {
            assert(2 >= g_elements.len());
            assert(g_elements.len() == 2);
            assert(iter_ghost_2@ == g_elements.take(2));
            assert(g_elements.take(2) == g_elements);
            assert(iter_ghost_2@ == g_elements);
            
            // Verify we've consumed expected elements!
            assert(iter_ghost_2@.to_set() == g_elements.to_set());
            assert(g_elements.to_set() == source@);
            assert(iter_ghost_2@.to_set() == source@);
            
            // Final verification: dest contains all source elements!
            assert(dest@ == iter_ghost_2@.to_set());
            assert(dest@ == source@);
        }
    }
}

} // verus!

}
