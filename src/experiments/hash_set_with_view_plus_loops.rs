//! Testing HashSetWithViewPlus with for loops

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

fn test_hash_set_iter_view_only()
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

fn test_hash_set_to_hash_set_copy()
{
    let mut source = std::collections::HashSet::<u32>::new();
    source.insert(3);
    source.insert(6);
    
    let source_iter: std::collections::hash_set::Iter<'_, u32> = source.iter();
/*
    assert(source_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = source_iter@.1;
    
    let ghost iter_at_start: vstd::std_specs::hash::SetIterGhostIterator<u32> = source_iter.ghost_iter();
    assert(iter_at_start.pos == 0);
    assert(iter_at_start.elements == g_elements);
    
    assert(g_elements.to_set() =~= set![3u32, 6u32]);
    assert(g_elements.to_set() =~= source@);
*/
    
    let mut dest = std::collections::HashSet::<u32>::new();
    
    for item in iter: source_iter
        invariant
//            iter.elements == g_elements,
//            g_elements.to_set() =~= source@,
            dest@ == iter@.to_set(),
    {
        dest.insert(*item);
    }
    
    assert(dest@ =~= source@);
}

/*
fn test_vec_to_vec_copy()
{
    let mut source = Vec::<u32>::new();
    source.push(3);
    source.push(6);
    
    let source_iter = source.iter();
    assert(source_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = source_iter@.1;
    
    let ghost iter_at_start = source_iter.ghost_iter();
    assert(iter_at_start.pos == 0);
    assert(iter_at_start.elements == g_elements);
    assert(g_elements == source@);
    
    let mut dest = Vec::<u32>::new();
    
    for item in iter: source_iter
        invariant
            iter.elements == g_elements,
            dest@ == iter@,
    {
        dest.push(*item);
    }
    
    assert(dest@ == source@);
}

fn test_vec_to_vec_add_one()
{
    let mut source = Vec::<u32>::new();
    source.push(3);
    source.push(6);
    
    let source_iter = source.iter();
    assert(source_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = source_iter@.1;
    
    let ghost iter_at_start = source_iter.ghost_iter();
    assert(iter_at_start.pos == 0);
    assert(iter_at_start.elements == g_elements);
    assert(g_elements == source@);
    
    let mut dest = Vec::<u32>::new();
    
    for item in iter: source_iter
        invariant
            iter.elements == g_elements,
            dest@ == iter@.map(|i: int, x: u32| (x + 1) as u32),
            forall|i: int| 0 <= i < g_elements.len() ==> g_elements[i] < u32::MAX,
    {
        assert(*item < u32::MAX);
        dest.push(*item + 1);
    }
    
    assert(dest@ == source@.map(|i: int, x: u32| (x + 1) as u32));
    assert(dest@[0] == 4);
    assert(dest@[1] == 7);
}

fn test_hash_set_to_hash_set_add_one()
{
    let mut source = std::collections::HashSet::<u32>::new();
    source.insert(3);
    source.insert(6);
    
    let source_iter: std::collections::hash_set::Iter<'_, u32> = source.iter();
    assert(source_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = source_iter@.1;
    
    let ghost iter_at_start: vstd::std_specs::hash::SetIterGhostIterator<u32> = source_iter.ghost_iter();
    assert(iter_at_start.pos == 0);
    assert(iter_at_start.elements == g_elements);
    
    assert(g_elements.to_set() =~= set![3u32, 6u32]);
    assert(g_elements.to_set() =~= source@);
    
    let mut dest = std::collections::HashSet::<u32>::new();
    
    for item in iter: source_iter
        invariant
            iter.elements == g_elements,
            g_elements.to_set() =~= source@,
            forall|i: int| 0 <= i < g_elements.len() ==> g_elements[i] < u32::MAX,
            dest@ == iter@.map(|i: int, x: u32| (x + 1) as u32).to_set(),
    {
        assert(*item < u32::MAX);
        dest.insert(*item + 1);
    }
    
    assert(dest@ =~= source@.map(|x: u32| (x + 1) as u32));
    assert(dest@.contains(4));
    assert(dest@.contains(7));
}

fn test_hash_set_iter_vec_only()
{
    let mut m = std::collections::HashSet::<u32>::new();
    
    m.insert(3);
    m.insert(6);
    
    let m_iter: std::collections::hash_set::Iter<'_, u32> = m.iter();
    assert(m_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = m_iter@.1;
    
    let ghost iter_at_start: vstd::std_specs::hash::SetIterGhostIterator<u32> = m_iter.ghost_iter();
    assert(iter_at_start.pos == 0);
    assert(iter_at_start.elements == g_elements);
    
    // Assert relationship between sequence and set
    assert(g_elements.to_set() =~= set![3u32, 6u32]);
    assert(g_elements.to_set() =~= m@);
    
    // Build a result Vec by iterating
    let mut items = Vec::<u32>::new();
    
    for k in iter: m_iter
        invariant
            iter.elements == g_elements,
            g_elements.to_set() =~= m@,
            items@ == iter@,
    {
        assert(iter.elements.take(iter.pos).push(*k) =~= iter.elements.take(iter.pos + 1));
        items.push(*k);
    }
    
    assert(items@.to_set() =~= m@) by {
        assert(g_elements.take(g_elements.len() as int) =~= g_elements);
    }
}

fn test_hash_set_with_view_plus_iter()
{
    let mut m = HashSetWithViewPlus::<u32>::new();
    
    m.insert(3);
    m.insert(6);
    
    let m_iter: std::collections::hash_set::Iter<'_, u32> = m.iter();
    assert(m_iter@.0 == 0);
    
    let ghost g_elements: Seq<u32> = m_iter@.1;
    
    let ghost iter_at_start: vstd::std_specs::hash::SetIterGhostIterator<u32> = m_iter.ghost_iter();
    assert(iter_at_start.pos == 0);
    assert(iter_at_start.elements == g_elements);
    
    // Assert relationship between sequence and set
    assert(g_elements.to_set() =~= set![3u32, 6u32]);
    assert(g_elements.to_set() =~= m@);
    
    // Build a result set by iterating
    let mut result = HashSetWithViewPlus::<u32>::new();
    let mut items = Vec::<u32>::new();
    
    for k in iter: m_iter
        invariant
            iter.elements == g_elements,
            g_elements.to_set() =~= m@,
            items@ == iter@,
            result@ == items@.to_set(),
    {
        assert(iter.elements.take(iter.pos).push(*k) =~= iter.elements.take(iter.pos + 1));
        items.push(*k);
        result.insert(*k);
    }
    
    assert(items@.to_set() =~= m@) by {
        assert(g_elements.take(g_elements.len() as int) =~= g_elements);
    }
    assert(result@ =~= m@);
}
*/

} // verus!

