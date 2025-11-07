//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set implementing SetTrait
//!
//! Uses SetTrait which matches vstd::set::Set<A> API

pub mod SetStEphPlus {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

    use vstd::prelude::*;
    use std::collections::hash_set::Iter;

    use crate::Types::Types::*;
    use crate::vstdplus::SetTrait::SetTrait::SetTrait;
    use crate::vstdplus::HashSetWithViewPlus::HashSetWithViewPlus::HashSetWithViewPlus;

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
broadcast use vstd::std_specs::hash::group_hash_axioms;

/// Verified ephemeral Set wrapping HashSetWithViewPlus
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
pub struct SetStEphPlus<T: vstd::prelude::View + Eq + Hash + Clone> {
    data: HashSetWithViewPlus<T>,
}

// SetStEphPlusTrait adds APAS-specific operations beyond SetTrait
pub trait SetStEphPlusTrait<T: StT + Hash + Clone>: SetTrait<T> {
    /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
    // Commented out: requires nesting HashSetWithViewPlus which needs Hash on SetStEphPlus
    // fn partition(&self, parts: &SetStEphPlus<SetStEphPlus<T>>) -> B;

    /// APAS: Work Θ(|a| × |b|), Span Θ(1)
    fn CartesianProduct<U: StT + Hash + Clone>(&self, other: &SetStEphPlus<U>) -> SetStEphPlus<Pair<T, U>>;

    /// APAS: Work Θ(1), Span Θ(1)
    fn iter(&self) -> Iter<'_, T>;

    /// APAS: Work Θ(|v|), Span Θ(1)
    fn FromVec(v: Vec<T>) -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    fn singleton(x: T) -> Self;
}

// Implement SetTrait for the core operations
impl<T: StT + Hash + Clone> SetTrait<T> for SetStEphPlus<T> {
    closed spec fn view(&self) -> Set<<T as View>::V> {
        self.data@
    }
    // TRUSTED: Can't verify generic obeys_key_model
    #[verifier::external_body]
    fn empty() -> (result: Self)
        ensures Self::view(&result) == Set::<<T as View>::V>::empty()
    {
        SetStEphPlus {
            data: HashSetWithViewPlus::new(),
        }
    }

    // VERIFIED: Direct call to HashSetWithViewPlus::contains
    fn contains(&self, x: &T) -> (result: B)
        ensures result == Self::view(self).contains(x@)
    {
        self.data.contains(x)
    }

    // VERIFIED: Direct call to HashSetWithViewPlus::insert
    fn insert(&mut self, x: T)
        ensures Self::view(self) == Self::view(&old(self)).insert(x@)
    {
        self.data.insert(x);
    }

    // VERIFIED: Direct call to HashSetWithViewPlus::remove
    fn remove(&mut self, x: &T)
        ensures Self::view(self) == Self::view(&old(self)).remove(x@)
    {
        self.data.remove(x);
    }

    // TRUSTED: external_body for clone + iteration
    #[verifier::external_body]
    fn union(&self, other: &Self) -> (result: Self)
        ensures Self::view(&result) == Self::view(self).union(Self::view(other))
    {
        let mut out_data = self.data.clone();
        for x in other.data.iter() {
            out_data.insert(x.clone());
        }
        SetStEphPlus { data: out_data }
    }

    // TRUSTED: external_body for iteration
    #[verifier::external_body]
    fn intersect(&self, other: &Self) -> (result: Self)
        ensures Self::view(&result) == Self::view(self).intersect(Self::view(other))
    {
        let mut out_data = HashSetWithViewPlus::new();
        for x in self.data.iter() {
            if other.data.contains(x) {
                out_data.insert(x.clone());
            }
        }
        SetStEphPlus { data: out_data }
    }

    // TRUSTED: external_body for iteration
    #[verifier::external_body]
    fn difference(&self, other: &Self) -> (result: Self)
        ensures Self::view(&result) == Self::view(self).difference(Self::view(other))
    {
        let mut out_data = HashSetWithViewPlus::new();
        for x in self.data.iter() {
            if !other.data.contains(x) {
                out_data.insert(x.clone());
            }
        }
        SetStEphPlus { data: out_data }
    }

    // VERIFIED: Direct call to HashSetWithViewPlus::len
    fn len(&self) -> (result: usize)
        ensures result == Self::view(self).len()
    {
        self.data.len()
    }

    // TRUSTED: Can't prove len() == 0 <==> view() == empty() without axioms
    #[verifier::external_body]
    fn is_empty(&self) -> (result: bool)
        ensures result <==> Self::view(self) == Set::<<T as View>::V>::empty()
    {
        self.data.len() == 0
    }
}

// Implement SetStEphPlusTrait for APAS-specific operations
impl<T: StT + Hash + Clone> SetStEphPlusTrait<T> for SetStEphPlus<T> {
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

    // TRUSTED: external_body for iteration
    #[verifier::external_body]
    fn CartesianProduct<U: StT + Hash + Clone>(&self, other: &SetStEphPlus<U>) -> (result: SetStEphPlus<Pair<T, U>>)
    {
        let mut out_data = HashSetWithViewPlus::new();
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

    // TRUSTED: external_body for iteration
    #[verifier::external_body]
    fn FromVec(v: Vec<T>) -> (result: SetStEphPlus<T>)
    {
        let mut s = HashSetWithViewPlus::new();
        for x in v {
            s.insert(x);
        }
        SetStEphPlus { data: s }
    }

    // TRUSTED: Can't verify generic obeys_key_model
    #[verifier::external_body]
    fn singleton(x: T) -> (result: SetStEphPlus<T>)
        ensures SetStEphPlus::view(&result) == Set::<<T as View>::V>::empty().insert(x@)
    {
        let mut s = HashSetWithViewPlus::new();
        s.insert(x);
        SetStEphPlus { data: s }
    }
}

    } // verus!

    // Runtime trait implementations outside verus! block
    impl<T: StT + Hash + Clone> PartialEq for SetStEphPlus<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.data.len() != other.data.len() {
                return false;
            }
            for x in self.data.iter() {
                if !other.data.contains(x) {
                    return false;
                }
            }
            true
        }
    }

    impl<T: StT + Hash + Clone> Eq for SetStEphPlus<T> {}

    impl<T: StT + Hash + Clone> Debug for SetStEphPlus<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_set().entries(self.data.iter()).finish()
        }
    }

    impl<T: StT + Hash + Clone> Display for SetStEphPlus<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{{")?;
            let mut first = true;
            for x in self.data.iter() {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{x}")?;
            }
            write!(f, "}}")
        }
    }
}
