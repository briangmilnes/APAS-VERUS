//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.5 ephemeral Mapping - verified wrapper around vstd::hash_map::HashMapWithView.

pub mod MappingStEph {
    use vstd::prelude::*;
    use vstd::hash_map::*;
    use std::hash::Hash;

    use crate::Types::Types::*;

    verus! {

/// Verified ephemeral Mapping wrapping HashMapWithView
#[verifier::ext_equal]
#[verifier::reject_recursive_types(K)]
#[verifier::reject_recursive_types(V)]
pub struct MappingStEph<K: View + Eq + Hash, V> {
    pub data: HashMapWithView<K, V>,
}

pub trait MappingStEphTrait<K: View + Eq + Hash, V: PartialEq>: Sized {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self
        requires vstd::std_specs::hash::obeys_key_model::<K>();

    /// APAS: Work Θ(1), Span Θ(1)
    fn size(&self) -> (result: N)
        ensures result == self.view().len();

    /// APAS: Work Θ(1), Span Θ(1)
    fn mem(&self, k: &K, v: &V) -> (result: B)
        ensures result == (self.view().contains_key(k@) && self.view()[k@] == *v);

    /// APAS: Work Θ(1), Span Θ(1)
    fn get(&self, k: &K) -> (result: Option<&V>)
        ensures
            match result {
                Some(v) => self.view().contains_key(k@) && self.view()[k@] == *v,
                None => !self.view().contains_key(k@),
            };

    /// APAS: Work Θ(1), Span Θ(1)
    fn insert(&mut self, k: K, v: V)
        ensures self.view() == old(self).view().insert(k@, v);

    spec fn view(&self) -> Map<<K as View>::V, V>;
}

impl<K: View + Eq + Hash, V: PartialEq> MappingStEphTrait<K, V> for MappingStEph<K, V> {
    fn empty() -> (result: MappingStEph<K, V>)
        ensures result.view() == Map::<<K as View>::V, V>::empty()
    {
        MappingStEph {
            data: HashMapWithView::new(),
        }
    }

    fn size(&self) -> (result: N)
        ensures result == self.view().len()
    {
        self.data.len()
    }

    fn mem(&self, k: &K, v: &V) -> (result: B)
        ensures result == (self.view().contains_key(k@) && self.view()[k@] == *v)
    {
        match self.data.get(k) {
            Some(val) => *val == *v,
            None => false,
        }
    }

    fn get(&self, k: &K) -> (result: Option<&V>)
        ensures
            match result {
                Some(v) => self.view().contains_key(k@) && self.view()[k@] == *v,
                None => !self.view().contains_key(k@),
            }
    {
        self.data.get(k)
    }

    fn insert(&mut self, k: K, v: V)
        ensures self.view() == old(self).view().insert(k@, v)
    {
        self.data.insert(k, v);
    }

    open spec fn view(&self) -> Map<<K as View>::V, V> {
        self.data@
    }
}

    } // verus!

    // Simplified MappingLit macro for our verified wrapper
    // Note: Does not support literal syntax yet - need to add FromVec/FromPairs methods
    #[macro_export]
    macro_rules! MappingLit {
        () => {{
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> >::empty()
        }};
        // TODO: Add syntax for ($( ($k:expr, $v:expr) ),*) once we have FromVec
    }
}
