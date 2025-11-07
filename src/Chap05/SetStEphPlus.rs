//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set with SetView as supertrait (Option 1 approach)
//!
//! Demonstrates using SetView as a supertrait to provide verified core operations

pub mod SetStEphPlus {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

    use vstd::prelude::*;
    use vstd::hash_set::HashSetWithView;
    use std::collections::hash_set::Iter;


    use crate::Types::Types::*;
    use crate::vstdplus::SetView::SetView::SetView;

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
broadcast use vstd::std_specs::hash::group_hash_axioms;

/// Verified ephemeral Set wrapping HashSetWithView
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
// #[derive(Eq, PartialEq, Hash)]
pub struct SetStEphPlus<T: vstd::prelude::View + Eq + Hash> {
    pub data: HashSetWithView<T>,
}

impl<T: vstd::prelude::View + Eq + Hash> vstd::prelude::View for SetStEphPlus<T> {
    type V = Set<<T as vstd::prelude::View>::V>;

    open spec fn view(&self) -> Set<<T as vstd::prelude::View>::V> {
        self.data@
    }
}

// SetStEphPlusTrait extends SetView and adds APAS-specific operations
pub trait SetStEphPlusTrait<T: StT + Hash>: SetView<T> {
    /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
//    fn partition(&self, parts: &SetStEphPlus<SetStEphPlus<T>>) -> B;

    /// APAS: Work Θ(|a| × |b|), Span Θ(1)
    fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEphPlus<U>) -> SetStEphPlus<Pair<T, U>>;

    /// APAS: Work Θ(1), Span Θ(1)
    fn iter(&self) -> Iter<'_, T>;

    /// APAS: Work Θ(|v|), Span Θ(1)
    fn FromVec(v: Vec<T>) -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    fn singleton(x: T) -> Self;
}

// Implement SetView for the core operations
impl<T: StT + Hash> SetView<T> for SetStEphPlus<T> {
    // TRUSTED: Can't verify generic obeys_key_model
    #[verifier::external_body]
    fn empty() -> (result: Self)
        ensures result@ == Set::<T::V>::empty()
    {
        SetStEphPlus {
            data: HashSetWithView::new(),
        }
    }

    // VERIFIED: Direct call to HashSetWithView::len
    fn size(&self) -> (result: N)
        ensures result == self@.len()
    {
        self.data.len()
    }

    // VERIFIED: Direct call to HashSetWithView::contains
    fn mem(&self, x: &T) -> (result: B)
        ensures result == self@.contains(x@)
    {
        self.data.contains(x)
    }

    // VERIFIED: Direct call to HashSetWithView::insert
    fn insert(&mut self, x: T)
        ensures self@ == old(self)@.insert(x@)
    {
        self.data.insert(x);
    }

    // TRUSTED: HashSetWithView doesn't expose remove
    #[verifier::external_body]
    fn remove(&mut self, x: T)
        ensures self@ == old(self)@.remove(x@)
    {
        self.data.remove(&x);
    }

    // TRUSTED: HashSetWithView doesn't expose iterators
    #[verifier::external_body]
    fn union(&self, other: &Self) -> (result: Self)
        ensures result@ == self@.union(other@)
    {
        let mut out_data = self.data.clone();
        for x in other.data.iter() {
            out_data.insert(x.clone());
        }
        SetStEphPlus { data: out_data }
    }

    // TRUSTED: HashSetWithView doesn't expose iterators
    #[verifier::external_body]
    fn intersection(&self, other: &Self) -> (result: Self)
        ensures result@ == self@.intersect(other@)
    {
        let mut out_data = HashSetWithView::new();
        for x in self.data.iter() {
            if other.data.contains(x) {
                out_data.insert(x.clone());
            }
        }
        SetStEphPlus { data: out_data }
    }
}

// Implement SetStEphPlusTrait for APAS-specific operations
impl<T: StT + Hash> SetStEphPlusTrait<T> for SetStEphPlus<T> {
    // TRUSTED: HashSetWithView doesn't expose iterators
/*
    #[verifier::external_body]
    fn partition(&self, parts: &SetStEphPlus<SetStEphPlus<T>>) -> (result: B)
    {
        for x in self.data.iter() {
            let mut count: N = 0;
            for subset in parts.data.iter() {
                if subset.data.contains(x) {
                    count += 1;
                    if count > 1 {
                        return false;
                    }
                }
            }
            if count == 0 {
                return false;
            }
        }
        true
    }
*/

    // TRUSTED: HashSetWithView doesn't expose iterators
    #[verifier::external_body]
    fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEphPlus<U>) -> (result: SetStEphPlus<Pair<T, U>>)
    {
        let mut out_data = HashSetWithView::new();
        for a in self.data.iter() {
            for b in other.data.iter() {
                out_data.insert(Pair(a.clone(), b.clone()));
            }
        }
        SetStEphPlus { data: out_data }
    }

    // TRUSTED: HashSetWithView doesn't expose iterators
    #[verifier::external_body]
    fn iter(&self) -> Iter<'_, T>
    {
        self.data.iter()
    }

    // TRUSTED: HashSetWithView doesn't expose iterators
    #[verifier::external_body]
    fn FromVec(v: Vec<T>) -> (result: SetStEphPlus<T>)
    {
        let mut s = HashSetWithView::new();
        for x in v {
            s.insert(x);
        }
        SetStEphPlus { data: s }
    }

    // TRUSTED: Can't verify generic obeys_key_model
    #[verifier::external_body]
    fn singleton(x: T) -> (result: SetStEphPlus<T>)
        ensures result@ == Set::<T::V>::empty().insert(x@)
    {
        let mut s = HashSetWithView::new();
        s.insert(x);
        SetStEphPlus { data: s }
    }
}

    } // verus!
}

