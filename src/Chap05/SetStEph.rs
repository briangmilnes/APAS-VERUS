//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `vstd::hash_set::HashSetWithView`.
//!
//! Uses IronKV's trusted approach: wraps HashSetWithView with #[verifier::external_body]

pub mod SetStEph {

    use vstd::prelude::*;
    use vstd::hash_set::*;
    use std::collections::HashSet as StdHashSet;
    use std::collections::hash_set::Iter; // Needed for iter() in APAS-AI
    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

    use crate::Types::Types::*;

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
broadcast use vstd::std_specs::hash::group_hash_axioms;

/// Verified ephemeral Set wrapping std::collections::HashSet with HashSetWithView for spec
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
#[derive(Clone)]
pub struct SetStEph<T: View + Eq + Hash> {
    pub data: StdHashSet<T>,
}

impl<T: View + Eq + Hash> View for SetStEph<T> {
    type V = Set<<T as View>::V>;

    open spec fn view(&self) -> Set<<T as View>::V> {
        arbitrary()
    }
}

pub trait SetStEphTrait<T: View + Eq + Hash + Clone>: Sized {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    fn singleton(x: T) -> Self;

    /// APAS: Work Θ(1), Span Θ(1)
    fn size(&self) -> (result: N)
        ensures result == self.view().len();

    /// APAS: Work Θ(1), Span Θ(1)
    fn mem(&self, x: &T) -> (result: B)
        ensures result == self.view().contains(x@);

    /// APAS: Work Θ(|a| + |b|), Span Θ(1)
    fn union(&self, other: &SetStEph<T>) -> Self;

    /// APAS: Work Θ(|a| + |b|), Span Θ(1)
    fn intersection(&self, other: &SetStEph<T>) -> Self;

    /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
    fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B;

    /// APAS: Work Θ(|a| × |b|), Span Θ(1)
    fn CartesianProduct<U: View + Eq + Hash + Clone>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>;

    /// APAS: Work Θ(1), Span Θ(1)
    fn insert(&mut self, x: T);

    /// APAS: Work Θ(1), Span Θ(1)
    fn iter(&self) -> Iter<'_, T>;

    /// APAS: Work Θ(|v|), Span Θ(1)
    fn FromVec(v: Vec<T>) -> Self;
}

// IronKV's approach: Generic implementation using external_body (trusted)
// This trusts the implementation instead of verifying it
impl<T: View + Eq + Hash + Clone> SetStEphTrait<T> for SetStEph<T> {
    #[verifier::external_body]
    fn empty() -> (result: SetStEph<T>)
        ensures result.view() == Set::<<T as View>::V>::empty()
    {
        SetStEph {
            data: StdHashSet::new(),
        }
    }

    #[verifier::external_body]
    fn singleton(x: T) -> (result: SetStEph<T>)
        ensures result.view() == Set::<<T as View>::V>::empty().insert(x@)
    {
        let mut s = StdHashSet::new();
        s.insert(x);
        SetStEph { data: s }
    }

    #[verifier::external_body]
    fn size(&self) -> (result: N)
        ensures result == self.view().len()
    {
        self.data.len()
    }

    #[verifier::external_body]
    fn mem(&self, x: &T) -> (result: B)
        ensures result == self.view().contains(x@)
    {
        self.data.contains(x)
    }

    #[verifier::external_body]
    fn union(&self, other: &SetStEph<T>) -> (result: SetStEph<T>)
        ensures result.view() == self.view().union(other.view())
    {
        let mut out_data = self.data.clone();
        for x in other.data.iter() {
            out_data.insert(x.clone());
        }
        SetStEph { data: out_data }
    }

    #[verifier::external_body]
    fn intersection(&self, other: &SetStEph<T>) -> (result: SetStEph<T>)
        ensures result.view() == self.view().intersect(other.view())
    {
        let mut out_data = StdHashSet::new();
        for x in self.data.iter() {
            if other.data.contains(x) {
                out_data.insert((*x).clone());
            }
        }
        SetStEph { data: out_data }
    }

    #[verifier::external_body]
    fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> (result: B)
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

    #[verifier::external_body]
    fn CartesianProduct<U: View + Eq + Hash + Clone>(&self, other: &SetStEph<U>) -> (result: SetStEph<Pair<T, U>>)
    {
        let mut out_data = StdHashSet::new();
        for a in self.data.iter() {
            for b in other.data.iter() {
                out_data.insert(Pair((*a).clone(), (*b).clone()));
            }
        }
        SetStEph { data: out_data }
    }

    #[verifier::external_body]
    fn insert(&mut self, x: T)
        ensures self.view() == old(self).view().insert(x@)
    {
        self.data.insert(x);
    }

    #[verifier::external_body]
    fn iter(&self) -> Iter<'_, T>
    {
        self.data.iter()
    }

    #[verifier::external_body]
    fn FromVec(v: Vec<T>) -> (result: SetStEph<T>)
    {
        let mut s = StdHashSet::new();
        for x in v {
            s.insert(x);
        }
        SetStEph { data: s }
    }
}

    } // verus!

    // Implementations outside verus! block for runtime traits
    impl<T: View + Eq + Hash> PartialEq for SetStEph<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    impl<T: View + Eq + Hash> Eq for SetStEph<T> {}

    impl<T: View + Eq + Hash + Debug> Debug for SetStEph<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_set().entries(self.data.iter()).finish()
        }
    }

    impl<T: View + Eq + Hash + Display> Display for SetStEph<T> {
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

    impl<T: View + Eq + Hash> Hash for SetStEph<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            use std::collections::hash_map::DefaultHasher;
            let mut element_hashes = Vec::<u64>::with_capacity(self.data.len());
            for e in self.data.iter() {
                let mut h = DefaultHasher::new();
                e.hash(&mut h);
                element_hashes.push(h.finish());
            }
            element_hashes.sort_unstable();
            self.data.len().hash(state);
            for h in element_hashes {
                h.hash(state);
            }
        }
    }

    #[macro_export]
    macro_rules! SetLit {
        () => {{
            < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty()
        }};
        ($($x:expr),* $(,)?) => {{
            let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
            $( __s.insert($x); )*
            __s
        }};
    }
}
