// Copyright (c) 2025 Brian G. Milnes
//! HashMapWithViewPlus - wrapper around vstd::hash_map::HashMapWithView.
//! Adds Clone, PartialEq, Eq that vstd's wrapper lacks (private inner field).

pub mod hash_map_with_view_plus {

use vstd::prelude::*;
use vstd::hash_map::HashMapWithView;
use std::collections::HashMap;
use core::hash::Hash;

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

verus! {

broadcast use vstd::map::group_map_axioms;

// 4. type definitions

#[verifier::reject_recursive_types(Key)]
#[verifier::reject_recursive_types(Value)]
pub struct HashMapWithViewPlus<Key: View + Eq + Hash, Value> {
    pub inner: HashMapWithView<Key, Value>,
}

// 5. view impls

impl<Key: View + Eq + Hash, Value> View for HashMapWithViewPlus<Key, Value> {
    type V = Map<<Key as View>::V, Value>;
    open spec fn view(&self) -> Self::V { self.inner@ }
}

// 8. traits

pub trait HashMapWithViewPlusTrait<Key: View + Eq + Hash, Value>: Sized + View<V = Map<<Key as View>::V, Value>> {
    fn new() -> (result: Self)
        requires
            obeys_key_model::<Key>(),
            forall|k1: Key, k2: Key| k1@ == k2@ ==> k1 == k2,
        ensures
            result@ == Map::<<Key as View>::V, Value>::empty();

    fn len(&self) -> (result: usize)
        ensures result == self@.len();

    fn is_empty(&self) -> (result: bool)
        ensures result == self@.is_empty();

    fn get<'a>(&'a self, k: &Key) -> (result: Option<&'a Value>)
        ensures
            match result {
                Some(v) => self@.contains_key(k@) && *v == self@[k@],
                None => !self@.contains_key(k@),
            };

    fn insert(&mut self, k: Key, v: Value)
        ensures self@ == old(self)@.insert(k@, v);

    fn clear(&mut self)
        ensures self@ == Map::<<Key as View>::V, Value>::empty();

    fn contains_key(&self, k: &Key) -> (result: bool)
        ensures result == self@.contains_key(k@);

    fn remove(&mut self, k: &Key) -> (result: Option<Value>)
        ensures
            match result {
                Some(v) => old(self)@.contains_key(k@) && v == old(self)@[k@] && self@ == old(self)@.remove(k@),
                None => !old(self)@.contains_key(k@) && self@ == old(self)@,
            };
}

// 9. impls

impl<Key: View + Eq + Hash, Value> HashMapWithViewPlusTrait<Key, Value> for HashMapWithViewPlus<Key, Value> {
    fn new() -> (result: Self) {
        HashMapWithViewPlus { inner: HashMapWithView::new() }
    }

    fn len(&self) -> (result: usize) {
        self.inner.len()
    }

    fn is_empty(&self) -> (result: bool) {
        self.inner.is_empty()
    }

    fn get<'a>(&'a self, k: &Key) -> (result: Option<&'a Value>) {
        self.inner.get(k)
    }

    fn insert(&mut self, k: Key, v: Value) {
        self.inner.insert(k, v);
    }

    fn clear(&mut self) {
        self.inner.clear();
    }

    fn contains_key(&self, k: &Key) -> (result: bool) {
        self.inner.contains_key(k)
    }

    fn remove(&mut self, k: &Key) -> (result: Option<Value>) {
        self.inner.remove(k)
    }
}

// 11. derive impls in verus!

impl<Key: View + Eq + Hash + Clone, Value: Clone> Clone for HashMapWithViewPlus<Key, Value> {
    #[verifier::external_body]
    fn clone(&self) -> (result: Self)
        ensures result@ == self@
    {
        HashMapWithViewPlus { inner: clone_hash_map_with_view(&self.inner) }
    }
}

impl<Key: View + Eq + Hash, Value: PartialEq> PartialEq for HashMapWithViewPlus<Key, Value> {
    #[verifier::external_body]
    fn eq(&self, other: &Self) -> bool {
        eq_hash_map_with_view(&self.inner, &other.inner)
    }
}

impl<Key: View + Eq + Hash, Value: Eq> Eq for HashMapWithViewPlus<Key, Value> {}

} // verus!

// 13. derive impls outside verus!

// HashMapWithView is a single-field newtype around HashMap with identical layout.
// These helpers use transmute to access the private inner field for Clone/PartialEq.

fn clone_hash_map_with_view<Key: View + Eq + Hash + Clone, Value: Clone>(
    inner: &HashMapWithView<Key, Value>,
) -> HashMapWithView<Key, Value> {
    unsafe {
        let raw: &HashMap<Key, Value> = std::mem::transmute(inner);
        std::mem::transmute(raw.clone())
    }
}

fn eq_hash_map_with_view<Key: View + Eq + Hash, Value: PartialEq>(
    a: &HashMapWithView<Key, Value>,
    b: &HashMapWithView<Key, Value>,
) -> bool {
    unsafe {
        let a_raw: &HashMap<Key, Value> = std::mem::transmute(a);
        let b_raw: &HashMap<Key, Value> = std::mem::transmute(b);
        a_raw == b_raw
    }
}

}
