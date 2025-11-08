//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `vstd::hash_set::HashSetWithView`.
//!
//! SetStEph implements SetWithView (verified specs) and extends with APAS-specific API.

pub mod SetStEph {

    use vstd::prelude::*;
    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

    use crate::Types::Types::*;
    use crate::vstdplus::set_with_view::SetWithView::SetWithView;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
broadcast use {vstd::std_specs::hash::group_hash_axioms, crate::vstdplus::clone_view::clone_view::group_clone_view_axioms, crate::Types::Types::group_pair_axioms};

pub trait SetStEphTrait<T: StT + Hash>: SetWithView<T> {
    fn singleton(x: T) -> (result: Self)
        requires vstd::std_specs::hash::obeys_key_model::<T>(),
                 forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
        ensures result@ == Set::<<T as View>::V>::empty().insert(x@)
    {
        let mut s = Self::empty();
        s.insert(x);
        s
    }

    fn size(&self) -> (result: N)
        ensures result == self@.len()
    {
        self.len()
    }

    fn mem(&self, x: &T) -> (result: B)
        ensures result == self@.contains(x@)
    {
        self.contains(x)
    }

    fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;

    fn FromVec(v: Vec<T>) -> Self
        requires vstd::std_specs::hash::obeys_key_model::<T>(),
                 forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2;

    fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>
        requires vstd::std_specs::hash::obeys_key_model::<Pair<T, U>>();

    fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B;
}


/// Verified ephemeral Set wrapping HashSetWithViewPlus
/// Implements SetWithView for verified specs, extends with APAS API
#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
pub struct SetStEph<T: View + Eq + Hash + Clone> {
    pub data: HashSetWithViewPlus<T>,
}

impl<T: View + Eq + Hash + Clone> SetStEph<T> {
    pub open spec fn view(&self) -> Set<<T as View>::V> {
        self.data@
    }
}

impl<T: View + Eq + Hash + Clone> View for SetStEph<T> {
    type V = Set<<T as View>::V>;

    open spec fn view(&self) -> Set<<T as View>::V> {
        Self::view(self)
    }
}


// Implement SetWithView trait for verified specs
impl<T: StT + Hash> SetWithView<T> for SetStEph<T> {
    fn empty() -> (result: Self) {
        SetStEph { data: HashSetWithViewPlus::new() }
    }

    fn contains(&self, x: &T) -> (result: bool) {
        self.data.contains(x)
    }

    fn insert(&mut self, x: T) {
        self.data.insert(x);
    }

    fn remove(&mut self, x: &T) {
        self.data.remove(x);
    }

    fn union(&self, other: &Self) -> (result: Self) {
        SetStEph { data: self.data.union(&other.data) }
    }

    fn intersect(&self, other: &Self) -> (result: Self) {
        SetStEph { data: self.data.intersection(&other.data) }
    }

    fn difference(&self, other: &Self) -> (result: Self) {
        SetStEph { data: self.data.difference(&other.data) }
    }

    fn len(&self) -> (result: usize) {
        self.data.len()
    }

    #[verifier::external_body]
    fn is_empty(&self) -> (result: bool) {
        // TODO: Need axiom that self@.len() == 0 <==> (forall a, !self@.contains(a))
        // Currently vstd has axiom_set_empty_len but not the reverse direction
        self.data.len() == 0
    }
}

// SetStEph implements SetStEphTrait (default implementations only, no verus specs)
impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {
    fn iter(&self) -> std::collections::hash_set::Iter<'_, T> {
        self.data.iter()
    }

    fn FromVec(v: Vec<T>) -> (result: Self)
        ensures forall |i: int| #![trigger result@.contains(v@[i]@)] 0 <= i < v@.len() ==> result@.contains(v@[i]@)
    {
        let mut s = Self::empty();
        let mut i: usize = 0;
        while i < v.len()
            invariant
                i <= v.len(),
                forall |j: int| #![trigger s@.contains(v@[j]@)] 0 <= j < i ==> s@.contains(v@[j]@),
            decreases v.len() - i
        {
            let x = v[i].clone();
            // axiom_clone_preserves_view should apply here
            s.insert(x);
            i += 1;
        }
        s
    }

    #[verifier::external_body]
    fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>> {
        let mut result = SetStEph::empty();
        for x in self.iter() {
            for y in other.iter() {
                result.insert(Pair(x.clone(), y.clone()));
            }
        }
        result
    }

    #[verifier::external_body]
    fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B {
        for x in self.iter() {
            let mut count: N = 0;
            for part in parts.iter() {
                if part.contains(x) {
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
}

// Additional convenience methods outside the trait
impl<T: StT + Hash> SetStEph<T> {
    pub fn intersection(&self, other: &Self) -> Self {
        self.intersect(other)
    }
}

    // Pedagogical runtime trait implementations that CAN be verified
    impl<T: StT + Hash> PartialEq for SetStEph<T> {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> (result: bool)
            ensures result == (self@ == other@)
        {
            // TODO: Verus ForLoopGhostIterator for HashSet exists, but tracking checked elements is challenging
            // Need invariant: "all elements seen so far are in other", but can't easily express without
            // referencing iterator's ghost state (which isn't directly accessible in invariant scope)
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

    impl<T: StT + Hash> Eq for SetStEph<T> {}

    impl<T: StT + Hash> Clone for SetStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            SetStEph { data: self.data.clone() }
        }
    }

    } // verus!

    // Pedagogical runtime trait implementations
    // Note: These use types (Formatter, Hasher) that Verus doesn't understand,
    // so they must be outside the verus! block

    impl<T: StT + Hash> Debug for SetStEph<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_set().entries(self.data.iter()).finish()
        }
    }

    impl<T: StT + Hash> Display for SetStEph<T> {
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

    impl<T: StT + Hash> Hash for SetStEph<T> {
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
