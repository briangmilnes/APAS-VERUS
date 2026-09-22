// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: which postconditions of APAS's `HashSetWithViewPlus`
//! (`src/vstdplus/hash_set_with_view_plus.rs`, 11 `external_body` methods and
//! one `admit` axiom) are now derivable from vstd 0.2026.09.13's
//! `std_specs::hash` specifications on `std::collections::HashSet` with no
//! trusted code in APAS. One fn per APAS method; each `ensures` is the APAS
//! postcondition, restated over the raw `HashSet` view `Set<Key>` and, where
//! APAS stated it over `Set<Key::V>`, over the mapped view `view_plus`.
//!
//! RESULT: SUCCEEDS — 11 verified, 0 errors. Every APAS set postcondition
//!         (new, with_capacity, len, contains, insert, iter, the for-loop
//!         pattern, and the mapped view) is derivable; vstd adds is_empty,
//!         remove, clear. Clone is the one gap: see vstd_hash_set_clone.rs.
//! DATE: 2026-09-20
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-vstd_hash_set_derived.20260920-222048.log
//! Run: scripts/validate-standard.sh --experiment vstd_hash_set_derived

pub mod vstd_hash_set_derived {
    use std::collections::HashSet;
    use std::hash::Hash;
    use vstd::prelude::*;
    use vstd::std_specs::hash::*;
    use vstd::std_specs::iter::*;

    verus! {

    broadcast use group_hash_axioms;

    /// APAS's view: `Set<Key::V>`, the image of the raw view under `@`.
    pub open spec fn view_plus<Key: View + Eq + Hash>(s: &HashSet<Key>) -> Set<Key::V> {
        s@.map(|k: Key| k@)
    }

    /// APAS `new`: `hash_set@ == Set::empty()`.
    pub fn derived_new<Key: View + Eq + Hash>() -> (s: HashSet<Key>)
        ensures
            s@ == Set::<Key>::empty(),
            view_plus(&s) == Set::<Key::V>::empty(),
    {
        let s = HashSet::<Key>::new();
        assert(view_plus(&s) =~= Set::<Key::V>::empty());
        s
    }

    /// APAS `with_capacity`: same postcondition as `new`.
    pub fn derived_with_capacity<Key: View + Eq + Hash>(capacity: usize) -> (s: HashSet<Key>)
        ensures
            s@ == Set::<Key>::empty(),
    {
        HashSet::<Key>::with_capacity(capacity)
    }

    /// APAS `len`: `len == self@.len()`.
    pub fn derived_len<Key: View + Eq + Hash>(s: &HashSet<Key>) -> (len: usize)
        requires
            obeys_key_model::<Key>(),
        ensures
            len == s@.len(),
    {
        s.len()
    }

    /// vstd extra: `is_empty`.
    pub fn derived_is_empty<Key: View + Eq + Hash>(s: &HashSet<Key>) -> (b: bool)
        ensures
            b == s@.is_empty(),
    {
        s.is_empty()
    }

    /// APAS `contains`: `contains == self@.contains(k)`.
    pub fn derived_contains<Key: View + Eq + Hash>(s: &HashSet<Key>, k: &Key) -> (contains: bool)
        requires
            obeys_key_model::<Key>(),
        ensures
            contains == s@.contains(*k),
    {
        s.contains(k)
    }

    /// APAS `insert`: `self@ == old(self)@.insert(k)` and `inserted == !old(self)@.contains(k)`.
    pub fn derived_insert<Key: View + Eq + Hash>(s: &mut HashSet<Key>, k: Key) -> (inserted: bool)
        requires
            obeys_key_model::<Key>(),
        ensures
            s@ == old(s)@.insert(k),
            inserted == !old(s)@.contains(k),
    {
        s.insert(k)
    }

    /// vstd extra, no APAS counterpart on the set wrapper: `remove`.
    pub fn derived_remove<Key: View + Eq + Hash>(s: &mut HashSet<Key>, k: &Key) -> (removed: bool)
        requires
            obeys_key_model::<Key>(),
        ensures
            s@ == old(s)@.remove(*k),
            removed == old(s)@.contains(*k),
    {
        s.remove(k)
    }

    /// vstd extra: `clear`.
    pub fn derived_clear<Key: View + Eq + Hash>(s: &mut HashSet<Key>)
        ensures
            s@ == Set::<Key>::empty(),
    {
        s.clear()
    }

    /// APAS `iter`: the iterator's elements are exactly the set's, without duplicates.
    /// APAS stated `forall k in seq ==> self@.contains(k@)` and the converse; vstd
    /// states `remaining.unref().to_set() == m@`, from which both follow.
    pub fn derived_iter<'a, Key: View + Eq + Hash>(s: &'a HashSet<Key>) -> (it: std::collections::hash_set::Iter<'a, Key>)
        requires
            obeys_key_model::<Key>(),
        ensures
            IteratorSpec::remaining(&it).unref().to_set() == s@,
            IteratorSpec::remaining(&it).no_duplicates(),
            IteratorSpec::remaining(&it).len() == s@.len(),
            IteratorSpec::decrease(&it) is Some,
    {
        s.iter()
    }

    /// APAS for-loop pattern over `iter()`: every visited key is in the set.
    pub fn derived_for_loop(s: &HashSet<u64>) -> (count: usize)
        ensures
            count == s@.len(),
    {
        let n = s.len();
        let mut count: usize = 0;
        for k in it: s.iter()
            invariant
                obeys_key_model::<u64>(),
                it.seq().unref().to_set() == s@,
                it.seq().len() == s@.len(),
                n == s@.len(),
                count == it.index(),
        {
            assert(s@.contains(*k));
            count = count + 1;
        }
        count
    }

    } // verus!
}
