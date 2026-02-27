// Copyright (c) 2025 Brian G. Milnes
//! HashSetWithViewPlus - wrapper around std::collections::HashSet
//! Provides View that gives Set<Key::V> and iter() method.
//! Bypasses vstd::hash_set::HashSetWithView to avoid pub m dependency.

pub mod hash_set_with_view_plus {

use vstd::prelude::*;
use std::collections::HashSet;

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;
use core::hash::Hash;
use crate::vstdplus::feq::feq::*;

verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

// Direct wrapper around std::collections::HashSet
// View gives Set<Key::V> (mapped view)
#[verifier::reject_recursive_types(Key)]
pub struct HashSetWithViewPlus<Key: View + Eq + Hash> {
    pub inner: HashSet<Key>,
}

impl<Key: View + Eq + Hash> View for HashSetWithViewPlus<Key> {
    type V = Set<<Key as View>::V>;
    
    // Map the raw HashSet view (Set<Key>) to Set<Key::V>
    open spec fn view(&self) -> Self::V { 
        self.inner@.map(|k: Key| k@)
    }
}

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

impl<Key: View + Eq + Hash + Clone> Clone for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn clone(&self) -> (clone: Self)
        ensures clone@ == self@
    {
        HashSetWithViewPlus { inner: self.inner.clone() }
    }
}

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

pub trait HashSetWithViewPlusTrait<Key: View + Eq + Hash>: View<V = Set<<Key as View>::V>> {
    fn iter(&self) -> (r: std::collections::hash_set::Iter<'_, Key>)
        ensures
            r@.0 == 0,
            r@.1.no_duplicates(),
            obeys_key_model::<Key>() ==> {
                let (index, s) = r@;
                &&& forall|k: Key| #![trigger s.contains(k)] s.contains(k) ==> self@.contains(k@)
                &&& forall|kv: Key::V| #![trigger self@.contains(kv)] self@.contains(kv) ==> exists|k: Key| #![trigger s.contains(k)] s.contains(k) && k@ == kv
            };
}

impl<Key: View + Eq + Hash> HashSetWithViewPlusTrait<Key> for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn iter(&self) -> (r: std::collections::hash_set::Iter<'_, Key>)
    { self.inner.iter() }
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
