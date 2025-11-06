//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Test for Chapter 5 SetStEph verified wrapper.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::axiom_u64_obeys_hash_table_key_model;

#[cfg(verus_keep_ghost)]
broadcast use axiom_u64_obeys_hash_table_key_model;

#[test]
fn test_set_u64_empty() {
    let s = SetStEph::<u64>::empty();
    assert(s.view() == Set::<u64>::empty());
}

#[test]
fn test_set_u64_singleton() {
    let s = SetStEph::<u64>::singleton(42);
    assert(s.view().contains(42));
    assert(s.view().len() == 1);
}

#[test]
fn test_set_u64_insert() {
    let mut s = SetStEph::<u64>::empty();
    s.insert(10);
    s.insert(20);
    assert(s.view().contains(10));
    assert(s.view().contains(20));
    assert(s.view().len() == 2);
}

#[test]
fn test_set_u64_mem() {
    let mut s = SetStEph::<u64>::empty();
    s.insert(42);
    let has_42 = s.mem(&42);
    let has_99 = s.mem(&99);
    assert(has_42);
    assert(!has_99);
}

} // verus!
