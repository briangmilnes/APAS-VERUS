// Copyright (c) 2025 Brian G. Milnes
use vstd::prelude::*;
use std::collections::HashSet;
#[cfg(verus_keep_ghost)]
use vstd::pervasive::ForLoopGhostIteratorNew;

verus! {

fn test_hash_set_iter()
{
    let mut m = HashSet::<u32>::new();

    m.insert(3);
    m.insert(6);
    let m_iter = m.iter();
    assert(m_iter@.0 == 0);
    assert(m_iter@.1.to_set() =~= set![3u32, 6u32]);
    let ghost g_elements = m_iter@.1;

    let mut items = Vec::<u32>::new();
    assert(items@ =~= g_elements.take(0));

    let ghost iter_at_start: vstd::std_specs::hash::SetIterGhostIterator<u32> = m_iter.ghost_iter();
    assert(iter_at_start.pos == 0);
    assert(iter_at_start.elements == g_elements);

    for k in iter: m_iter
        invariant
            iter.elements == g_elements,
            g_elements.to_set() =~= set![3u32, 6u32],
            items@ == iter@,
    {
        assert(iter.elements.take(iter.pos).push(*k) =~= iter.elements.take(iter.pos + 1));
        items.push(*k);
    }
    assert(items@.to_set() =~= set![3u32, 6u32]) by {
        assert(g_elements.take(g_elements.len() as int) =~= g_elements);
    }
}

} // verus!
