// Copyright (c) 2025 Brian G. Milnes

//! HashSetWithViewPlus - wrapper around std::collections::HashSet
//! Provides View that gives Set<Key::V> and iter() method.
//! Bypasses vstd::hash_set::HashSetWithView to avoid pub m dependency.

//  Table of Contents
//	1. module
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!

//		1. module


pub mod hash_set_with_view_plus {

use vstd::prelude::*;
use std::collections::HashSet;

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;
use core::hash::Hash;
use crate::vstdplus::feq::feq::*;

verus! {

//		3. broadcast use

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};


//		4. type definitions

// Direct wrapper around std::collections::HashSet
// View gives Set<Key::V> (mapped view)
#[verifier::reject_recursive_types(Key)]
pub struct HashSetWithViewPlus<Key: View + Eq + Hash> {
    pub inner: HashSet<Key>,
}


//		5. view impls

impl<Key: View + Eq + Hash> View for HashSetWithViewPlus<Key> {
    type V = Set<<Key as View>::V>;
    
    // Map the raw HashSet view (Set<Key>) to Set<Key::V>
    open spec fn view(&self) -> Self::V { 
        self.inner@.map(|k: Key| k@)
    }
}


//		7. proof fns/broadcast groups

/// A HashSetWithViewPlus is always finite (it's backed by a finite HashSet)
pub broadcast proof fn axiom_hash_set_with_view_plus_finite<Key: View + Eq + Hash>(s: &HashSetWithViewPlus<Key>)
    ensures
        #[trigger] s@.finite(),
{
    admit();
}

pub broadcast group group_hash_set_with_view_plus_axioms {
    axiom_hash_set_with_view_plus_finite,
}


//		8. traits

pub trait HashSetWithViewPlusTrait<Key: View + Eq + Hash>: View<V = Set<<Key as View>::V>> {
    fn iter(&self) -> (r: HashSetWithViewPlusIter<'_, Key>)
        ensures
            r@.0 == 0,
            r@.1.no_duplicates(),
            obeys_key_model::<Key>() ==> {
                let (index, s) = r@;
                &&& forall|k: Key| #![trigger s.contains(k)] s.contains(k) ==> self@.contains(k@)
                &&& forall|kv: Key::V| #![trigger self@.contains(kv)] self@.contains(kv) ==> exists|k: Key| #![trigger s.contains(k)] s.contains(k) && k@ == kv
            };
}


//		9. impls

impl<Key: View + Eq + Hash + Clone> HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    pub fn new() -> (hash_set: Self)
        requires
            obeys_key_model::<Key>(),
            obeys_feq_full::<Key>(),
        ensures
            hash_set@ == Set::<<Key as View>::V>::empty(),
    { 
        HashSetWithViewPlus { inner: HashSet::new() }
    }

    #[verifier::external_body]
    pub fn with_capacity(capacity: usize) -> (hash_set: Self)
        requires
            obeys_key_model::<Key>(),
            obeys_feq_full::<Key>(),
        ensures
            hash_set@ == Set::<<Key as View>::V>::empty(),
    { 
        HashSetWithViewPlus { inner: HashSet::with_capacity(capacity) }
    }

    #[verifier::external_body]
    pub fn len(&self) -> (len: usize)
        ensures
            len == self@.len(),
    {
        self.inner.len()
    }

    #[verifier::external_body]
    pub fn contains(&self, k: &Key) -> (contains: bool)
        requires
            obeys_key_model::<Key>(),
        ensures
            contains == self@.contains(k@),
    { 
        self.inner.contains(k) 
    }

    #[verifier::external_body]
    pub fn insert(&mut self, k: Key) -> (inserted: bool)
        requires
            obeys_key_model::<Key>(),
            obeys_feq_full::<Key>(),
        ensures
            self@ == old(self)@.insert(k@),
            inserted == !old(self)@.contains(k@),
    { 
        self.inner.insert(k)
    }
}

impl<Key: View + Eq + Hash> HashSetWithViewPlusTrait<Key> for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn iter(&self) -> (r: HashSetWithViewPlusIter<'_, Key>)
    { HashSetWithViewPlusIter { inner: self.inner.iter() } }
}


//		10. iterators

#[verifier::reject_recursive_types(Key)]
pub struct HashSetWithViewPlusIter<'a, Key: View + Eq + Hash> {
    pub inner: std::collections::hash_set::Iter<'a, Key>,
}

impl<'a, Key: View + Eq + Hash> View for HashSetWithViewPlusIter<'a, Key> {
    type V = (int, Seq<Key>);
    open spec fn view(&self) -> (int, Seq<Key>) {
        self.inner@
    }
}

pub open spec fn iter_invariant<'a, Key: View + Eq + Hash>(it: &HashSetWithViewPlusIter<'a, Key>) -> bool {
    0 <= it@.0 <= it@.1.len()
}

impl<'a, Key: View + Eq + Hash> std::iter::Iterator for HashSetWithViewPlusIter<'a, Key> {
    type Item = &'a Key;

    fn next(&mut self) -> (next: Option<&'a Key>)
        ensures ({
            let (old_index, old_seq) = old(self)@;
            match next {
                None => {
                    &&& self@ == old(self)@
                    &&& old_index >= old_seq.len()
                },
                Some(element) => {
                    let (new_index, new_seq) = self@;
                    &&& 0 <= old_index < old_seq.len()
                    &&& new_seq == old_seq
                    &&& new_index == old_index + 1
                    &&& element == old_seq[old_index]
                },
            }
        })
    {
        self.inner.next()
    }
}

/// Ghost iterator for ForLoopGhostIterator support (for-iter, for-borrow patterns).
#[verifier::reject_recursive_types(Key)]
pub struct HashSetWithViewPlusGhostIterator<'a, Key: View + Eq + Hash> {
    pub pos: int,
    pub elements: Seq<Key>,
    pub phantom: core::marker::PhantomData<&'a Key>,
}

impl<'a, Key: View + Eq + Hash> vstd::pervasive::ForLoopGhostIteratorNew for HashSetWithViewPlusIter<'a, Key> {
    type GhostIter = HashSetWithViewPlusGhostIterator<'a, Key>;

    open spec fn ghost_iter(&self) -> HashSetWithViewPlusGhostIterator<'a, Key> {
        HashSetWithViewPlusGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
    }
}

impl<'a, Key: View + Eq + Hash> vstd::pervasive::ForLoopGhostIterator for HashSetWithViewPlusGhostIterator<'a, Key> {
    type ExecIter = HashSetWithViewPlusIter<'a, Key>;
    type Item = Key;
    type Decrease = int;

    open spec fn exec_invariant(&self, exec_iter: &HashSetWithViewPlusIter<'a, Key>) -> bool {
        &&& self.pos == exec_iter@.0
        &&& self.elements == exec_iter@.1
    }

    open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
        init matches Some(init) ==> {
            &&& init.pos == 0
            &&& init.elements == self.elements
            &&& 0 <= self.pos <= self.elements.len()
        }
    }

    open spec fn ghost_ensures(&self) -> bool {
        self.pos == self.elements.len()
    }

    open spec fn ghost_decrease(&self) -> Option<int> {
        Some(self.elements.len() - self.pos)
    }

    open spec fn ghost_peek_next(&self) -> Option<Key> {
        if 0 <= self.pos < self.elements.len() {
            Some(self.elements[self.pos])
        } else {
            None
        }
    }

    open spec fn ghost_advance(&self, _exec_iter: &HashSetWithViewPlusIter<'a, Key>) -> HashSetWithViewPlusGhostIterator<'a, Key> {
        Self { pos: self.pos + 1, ..*self }
    }
}

impl<'a, Key: View + Eq + Hash> View for HashSetWithViewPlusGhostIterator<'a, Key> {
    type V = Seq<Key>;

    open spec fn view(&self) -> Seq<Key> {
        self.elements.take(self.pos)
    }
}


//		11. derive impls in verus!

impl<Key: View + Eq + Hash + Clone> Clone for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn clone(&self) -> (clone: Self)
        ensures clone@ == self@
    {
        HashSetWithViewPlus { inner: self.inner.clone() }
    }
}

impl<Key: View + Eq + Hash> std::hash::Hash for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for key in self.inner.iter() {
            key.hash(state);
        }
    }
}

impl<Key: View + Eq + Hash> PartialEq for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn eq(&self, other: &Self) -> bool { self.inner == other.inner }
}

impl<Key: View + Eq + Hash> Eq for HashSetWithViewPlus<Key> {}

} // verus!

}
