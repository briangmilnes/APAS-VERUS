//! HashSetWithViewPlus - wrapper around vstd::hash_set::HashSetWithView
//! Adds iter() method since HashSetWithView.m is now public

pub mod hash_set_with_view_plus {

use vstd::prelude::*;
use vstd::hash_set::HashSetWithView;

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;
#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::SetIterAdditionalSpecFns;
use core::hash::Hash;
use crate::vstdplus::feq::feq::*;

verus! {

// Step 1: Simple wrapper with View
#[verifier::reject_recursive_types(Key)]
pub struct HashSetWithViewPlus<Key: View + Eq + Hash> {
    pub inner: HashSetWithView<Key>,
}

impl<Key: View + Eq + Hash> View for HashSetWithViewPlus<Key> {
    type V = Set<<Key as View>::V>;
    
    open spec fn view(&self) -> Self::V { self.inner@}
}

impl<Key: View + Eq + Hash + Clone> Clone for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn clone(&self) -> (result: Self)
        ensures result@ == self@
    {
        HashSetWithViewPlus { inner: HashSetWithView { m: self.inner.m.clone() } }
    }
}

impl<Key: View + Eq + Hash + Clone> HashSetWithViewPlus<Key> {
    pub fn new() -> (result: Self)
        requires
            obeys_key_model::<Key>(),
            obeys_feq_full::<Key>(),
        ensures
            result@ == Set::<<Key as View>::V>::empty(),
    { HashSetWithViewPlus { inner: HashSetWithView::new() } }

    pub fn len(&self) -> (len: usize)
        ensures
            len == self@.len(),
    {
        self.inner.len()
    }

    pub fn contains(&self, k: &Key) -> (contains: bool)
        requires
            obeys_key_model::<Key>(),
        ensures
            contains == self@.contains(k@),
    { self.inner.contains(k) }

    pub fn insert(&mut self, k: Key) -> (inserted: bool)
        requires
            obeys_key_model::<Key>(),
            obeys_feq_full::<Key>(),
        ensures
            self@ == old(self)@.insert(k@),
            inserted == !old(self)@.contains(k@),
    { self.inner.insert(k) }
}

pub trait HashSetWithViewPlusTrait<Key: View + Eq + Hash>: View<V = Set<<Key as View>::V>> {
    fn iter(&self) -> (r: std::collections::hash_set::Iter<'_, Key>)
        ensures
            r@.0 == 0,
            r@.1.no_duplicates(),
            obeys_key_model::<Key>() ==> {
                let (index, s) = r@;
                &&& forall|k: Key| #![auto] s.contains(k) ==> self@.contains(k@)
                &&& forall|kv: Key::V| #![auto] self@.contains(kv) ==> exists|k: Key| #![auto] s.contains(k) && k@ == kv
            };
}

impl<Key: View + Eq + Hash> HashSetWithViewPlusTrait<Key> for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn iter(&self) -> (r: std::collections::hash_set::Iter<'_, Key>)
    { self.inner.m.iter() }
}

impl<Key: View + Eq + Hash> std::hash::Hash for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for key in self.inner.m.iter() {
            key.hash(state);
        }
    }
}

impl<Key: View + Eq + Hash> PartialEq for HashSetWithViewPlus<Key> {
    #[verifier::external_body]
    fn eq(&self, other: &Self) -> bool { self.inner.m == other.inner.m }
}

impl<Key: View + Eq + Hash> Eq for HashSetWithViewPlus<Key> {}

} // verus!

}

