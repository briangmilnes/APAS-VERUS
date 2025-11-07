//! HashSetWithViewPlus - extends HashSetWithView with clone and iterators
//!
//! This fills a gap in vstd's HashSetWithView which doesn't expose clone() or iter().
//! We provide these as trusted external_body functions with appropriate specs.

pub mod HashSetWithViewPlus {
    use vstd::prelude::*;
    use std::collections::HashSet;
    use std::collections::hash_set::Iter;
    use std::hash::Hash;

    verus! {

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    #[cfg(verus_keep_ghost)]
    broadcast use vstd::std_specs::hash::group_hash_axioms;

    /// Extended wrapper around std::collections::HashSet with View and additional methods
    #[verifier::ext_equal]
    #[verifier::reject_recursive_types(Key)]
    pub struct HashSetWithViewPlus<Key> where Key: View + Eq + Hash + Clone {
        inner: HashSet<Key>,
    }

    impl<Key> View for HashSetWithViewPlus<Key> where Key: View + Eq + Hash + Clone {
        type V = Set<<Key as View>::V>;

        uninterp spec fn view(&self) -> Self::V;
    }

    impl<Key> HashSetWithViewPlus<Key> where Key: View + Eq + Hash + Clone {
        /// Creates an empty HashSetWithViewPlus
        #[verifier::external_body]
        pub fn new() -> (result: Self)
            requires
                obeys_key_model::<Key>(),
                forall|k1: Key, k2: Key| k1@ == k2@ ==> k1 == k2,
            ensures
                result@ == Set::<<Key as View>::V>::empty(),
        {
            Self { inner: HashSet::new() }
        }

        /// Returns the number of elements in the set
        #[verifier::external_body]
        pub fn len(&self) -> (result: usize)
            ensures
                result == self@.len(),
        {
            self.inner.len()
        }

        /// Returns true if the set contains the key
        #[verifier::external_body]
        pub fn contains(&self, key: &Key) -> (result: bool)
            ensures
                result == self@.contains(key@),
        {
            self.inner.contains(key)
        }

        /// Inserts a key into the set
        #[verifier::external_body]
        pub fn insert(&mut self, key: Key)
            ensures
                self@ == old(self)@.insert(key@),
        {
            self.inner.insert(key);
        }

        /// Removes a key from the set
        #[verifier::external_body]
        pub fn remove(&mut self, key: &Key)
            ensures
                self@ == old(self)@.remove(key@),
        {
            self.inner.remove(key);
        }

        /// Clone the set (TRUSTED: external_body)
        #[verifier::external_body]
        pub fn clone(&self) -> (result: Self)
            ensures
                result@ == self@,
        {
            Self { inner: self.inner.clone() }
        }

        /// Returns an iterator over the set
        pub fn iter(&self) -> Iter<'_, Key>
        {
            self.inner.iter()
        }
    }

    } // verus!
}

