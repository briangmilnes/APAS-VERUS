// Copyright (c) 2025 Brian G. Milnes

//! HashMapWithViewPlus - wrapper around std::collections::HashMap.
//! Provides Clone, PartialEq, Eq, and iter() with Verus specs.

//  Table of Contents
//	1. module
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!

//		1. module


pub mod hash_map_with_view_plus {

use vstd::prelude::*;
use std::collections::HashMap;
use core::hash::Hash;

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;
#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::HashMapAdditionalSpecFns;

verus! {

//		3. broadcast use

broadcast use vstd::map::group_map_axioms;


//		4. type definitions

#[verifier::reject_recursive_types(Key)]
#[verifier::reject_recursive_types(Value)]
pub struct HashMapWithViewPlus<Key: View + Eq + Hash, Value> {
    pub inner: HashMap<Key, Value>,
}

//		5. view impls

impl<Key: View + Eq + Hash, Value> View for HashMapWithViewPlus<Key, Value> {
    type V = Map<<Key as View>::V, Value>;
    uninterp spec fn view(&self) -> Self::V;
}

//		8. traits

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

    fn iter(&self) -> (r: HashMapWithViewPlusIter<'_, Key, Value>)
        ensures
            r@.0 == 0,
            r@.1.no_duplicates(),
            obeys_key_model::<Key>() ==> {
                let (index, s) = r@;
                &&& forall|kv: (Key, Value)| #![trigger s.contains(kv)]
                        s.contains(kv) ==> self@.contains_key(kv.0@) && self@[kv.0@] == kv.1
                &&& forall|kv: Key::V| #![trigger self@.contains_key(kv)]
                        self@.contains_key(kv) ==>
                            exists|pair: (Key, Value)| #![trigger s.contains(pair)]
                                s.contains(pair) && pair.0@ == kv
            };
}

//		9. impls

impl<Key: View + Eq + Hash, Value> HashMapWithViewPlusTrait<Key, Value> for HashMapWithViewPlus<Key, Value> {
    #[verifier::external_body]
    fn new() -> (result: Self) {
        HashMapWithViewPlus { inner: HashMap::new() }
    }

    #[verifier::external_body]
    fn len(&self) -> (result: usize) {
        self.inner.len()
    }

    #[verifier::external_body]
    fn is_empty(&self) -> (result: bool) {
        self.inner.is_empty()
    }

    #[verifier::external_body]
    fn get<'a>(&'a self, k: &Key) -> (result: Option<&'a Value>) {
        self.inner.get(k)
    }

    #[verifier::external_body]
    fn insert(&mut self, k: Key, v: Value) {
        self.inner.insert(k, v);
    }

    #[verifier::external_body]
    fn clear(&mut self) {
        self.inner.clear();
    }

    #[verifier::external_body]
    fn contains_key(&self, k: &Key) -> (result: bool) {
        self.inner.contains_key(k)
    }

    #[verifier::external_body]
    fn remove(&mut self, k: &Key) -> (result: Option<Value>) {
        self.inner.remove(k)
    }

    #[verifier::external_body]
    fn iter(&self) -> (r: HashMapWithViewPlusIter<'_, Key, Value>) {
        HashMapWithViewPlusIter { inner: self.inner.iter() }
    }
}

//		10. iterators

/// Iterator wrapper with closed View for encapsulation.
#[verifier::reject_recursive_types(Key)]
#[verifier::reject_recursive_types(Value)]
pub struct HashMapWithViewPlusIter<'a, Key: View + Eq + Hash, Value> {
    inner: std::collections::hash_map::Iter<'a, Key, Value>,
}

impl<'a, Key: View + Eq + Hash, Value> View for HashMapWithViewPlusIter<'a, Key, Value> {
    type V = (int, Seq<(Key, Value)>);
    closed spec fn view(&self) -> (int, Seq<(Key, Value)>) {
        self.inner@
    }
}

pub open spec fn iter_invariant<'a, Key: View + Eq + Hash, Value>(it: &HashMapWithViewPlusIter<'a, Key, Value>) -> bool {
    0 <= it@.0 <= it@.1.len()
}

impl<'a, Key: View + Eq + Hash, Value> std::iter::Iterator for HashMapWithViewPlusIter<'a, Key, Value> {
    type Item = (&'a Key, &'a Value);

    fn next(&mut self) -> (next: Option<(&'a Key, &'a Value)>)
        ensures ({
            let (old_index, old_seq) = old(self)@;
            match next {
                None => {
                    &&& self@ == old(self)@
                    &&& old_index >= old_seq.len()
                },
                Some((k, v)) => {
                    let (new_index, new_seq) = self@;
                    let (old_k, old_v) = old_seq[old_index];
                    &&& 0 <= old_index < old_seq.len()
                    &&& new_seq == old_seq
                    &&& new_index == old_index + 1
                    &&& k == old_k
                    &&& v == old_v
                    &&& old_seq.to_set().contains((*k, *v))
                },
            }
        })
    {
        self.inner.next()
    }
}

/// Ghost iterator for ForLoopGhostIterator support (for-iter, for-borrow patterns).
#[verifier::reject_recursive_types(Key)]
#[verifier::reject_recursive_types(Value)]
pub struct HashMapWithViewPlusGhostIterator<'a, Key: View + Eq + Hash, Value> {
    pub pos: int,
    pub kv_pairs: Seq<(Key, Value)>,
    pub phantom: core::marker::PhantomData<&'a (Key, Value)>,
}

impl<'a, Key: View + Eq + Hash, Value> vstd::pervasive::ForLoopGhostIteratorNew for HashMapWithViewPlusIter<'a, Key, Value> {
    type GhostIter = HashMapWithViewPlusGhostIterator<'a, Key, Value>;

    open spec fn ghost_iter(&self) -> HashMapWithViewPlusGhostIterator<'a, Key, Value> {
        HashMapWithViewPlusGhostIterator { pos: self@.0, kv_pairs: self@.1, phantom: core::marker::PhantomData }
    }
}

impl<'a, Key: View + Eq + Hash, Value> vstd::pervasive::ForLoopGhostIterator for HashMapWithViewPlusGhostIterator<'a, Key, Value> {
    type ExecIter = HashMapWithViewPlusIter<'a, Key, Value>;
    type Item = (Key, Value);
    type Decrease = int;

    open spec fn exec_invariant(&self, exec_iter: &HashMapWithViewPlusIter<'a, Key, Value>) -> bool {
        &&& self.pos == exec_iter@.0
        &&& self.kv_pairs == exec_iter@.1
    }

    open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
        init matches Some(init) ==> {
            &&& init.pos == 0
            &&& init.kv_pairs == self.kv_pairs
            &&& 0 <= self.pos <= self.kv_pairs.len()
        }
    }

    open spec fn ghost_ensures(&self) -> bool {
        self.pos == self.kv_pairs.len()
    }

    open spec fn ghost_decrease(&self) -> Option<int> {
        Some(self.kv_pairs.len() - self.pos)
    }

    open spec fn ghost_peek_next(&self) -> Option<(Key, Value)> {
        if 0 <= self.pos < self.kv_pairs.len() {
            Some(self.kv_pairs[self.pos])
        } else {
            None
        }
    }

    open spec fn ghost_advance(&self, _exec_iter: &HashMapWithViewPlusIter<'a, Key, Value>) -> HashMapWithViewPlusGhostIterator<'a, Key, Value> {
        Self { pos: self.pos + 1, ..*self }
    }
}

impl<'a, Key: View + Eq + Hash, Value> View for HashMapWithViewPlusGhostIterator<'a, Key, Value> {
    type V = Seq<(Key, Value)>;

    open spec fn view(&self) -> Seq<(Key, Value)> {
        self.kv_pairs.take(self.pos)
    }
}

//		11. derive impls in verus!

impl<Key: View + Eq + Hash + Clone, Value: Clone> Clone for HashMapWithViewPlus<Key, Value> {
    #[verifier::external_body]
    fn clone(&self) -> (result: Self)
        ensures result@ == self@
    {
        HashMapWithViewPlus { inner: self.inner.clone() }
    }
}

impl<Key: View + Eq + Hash, Value: PartialEq> PartialEq for HashMapWithViewPlus<Key, Value> {
    #[verifier::external_body]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<Key: View + Eq + Hash, Value: Eq> Eq for HashMapWithViewPlus<Key, Value> {}

} // verus!

}
