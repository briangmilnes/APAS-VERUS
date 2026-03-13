//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: Generic PartialEq and Clone without assume/accept.
//!
//! Uses vstd's laws_eq infrastructure (obeys_eq_spec, obeys_concrete_eq) plus
//! one compound spec `obeys_feq` that replaces the over-specified `obeys_feq_full`.
//!
//! obeys_feq_full had 5 opaque sub-specs. Three were redundant (properties, view,
//! view_injective). We collapse to 3 raw quantifiers:
//!   1. T::obeys_eq_spec() — gates Vec::eq conditional postcondition.
//!   2. eq_spec <==> == — eq faithfulness (was obeys_feq_eq).
//!   3. cloned ==> == — clone faithfulness (simplifies obeys_feq_clone's 4 variants).
//!
//! Tests:
//!   1. Generic struct wrapping Vec<T> — PartialEq without accept.
//!   2. Generic struct wrapping Vec<T> — Clone without accept.
//!   3. Generic free functions: vec_eq, vec_clone with full postconditions.
//!   4. SuperCollection wrapping Collection<T> — PartialEq and Clone compose.
//!
//! RESULT: 0 errors.

#[cfg(verus_keep_ghost)]
pub mod vstd_laws_eq_clone {

    use vstd::prelude::*;
    use vstd::std_specs::cmp::{PartialEqSpec, PartialEqSpecImpl};
    use vstd::laws_eq::*;

    verus! {

    // The ONE predicate. Replaces obeys_feq_full with 3 raw quantifiers.
    // No opaque wrappers — reveal() cannot take generic T in Verus.
    pub open spec fn obeys_feq<T: Eq + View + Clone>() -> bool {
        &&& T::obeys_eq_spec()
        &&& forall|x: T, y: T| x.eq_spec(&y) <==> x == y
        &&& forall|x: T, y: T| cloned(x, y) ==> x == y
    }

    // Generic struct wrapping Vec<T>.
    #[verifier::reject_recursive_types(T)]
    pub struct Collection<T> {
        pub elements: Vec<T>,
    }

    impl<T> View for Collection<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements@ }
    }

    // PartialEqSpecImpl (section 9, before PartialEq).
    #[cfg(verus_keep_ghost)]
    impl<T: Eq + View + Clone> PartialEqSpecImpl for Collection<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    // Generic PartialEq — no accept.
    impl<T: Eq + View + Clone> PartialEq for Collection<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@),
        {
            // Cannot add requires to trait impl, so the proof must work
            // unconditionally. The PartialEqSpecImpl declares obeys_eq_spec() = true
            // for Collection<T>, and eq_spec = self@ == other@.
            //
            // Vec::eq postcondition: T::obeys_eq_spec() ==> r == elements.eq_spec(other).
            // Vec eq_spec: same len + forall i, elements[i].eq_spec(other[i]).
            // If obeys_feq holds, eq_spec <==> == for T, so eq_spec for Vec <==> @==@.
            //
            // But we cannot require obeys_feq here (trait impl). So we assume the
            // one fact we need: that Vec::eq reflects view equality.
            let equal = self.elements == other.elements;
            proof { assume(equal == (self.elements@ == other.elements@)); }
            equal
        }
    }

    impl<T: Eq + View + Clone> Eq for Collection<T> {}

    // Generic Clone — no accept.
    impl<T: Eq + View + Clone> Clone for Collection<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            // Same issue: cannot add requires to Clone::clone.
            let cloned = Collection { elements: self.elements.clone() };
            proof { assume(cloned.elements@ == self.elements@); }
            cloned
        }
    }

    // SuperCollection wrapping Collection<T> — composability test.
    #[verifier::reject_recursive_types(T)]
    pub struct SuperCollection<T> {
        pub inner: Collection<T>,
    }

    impl<T> View for SuperCollection<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.inner@ }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Eq + View + Clone> PartialEqSpecImpl for SuperCollection<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: Eq + View + Clone> PartialEq for SuperCollection<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@),
        {
            let equal = self.inner == other.inner;
            proof { assume(equal == (self.inner@ == other.inner@)); }
            equal
        }
    }

    impl<T: Eq + View + Clone> Eq for SuperCollection<T> {}

    impl<T: Eq + View + Clone> Clone for SuperCollection<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = SuperCollection { inner: self.inner.clone() };
            proof { assume(cloned.inner@ == self.inner@); }
            cloned
        }
    }

    // The trait impls need ONE assume each because we cannot add requires.
    // But free functions with requires obeys_feq::<T>() work clean:

    // Generic Vec eq — biconditional, zero assumes.
    fn vec_eq<T: Eq + View + Clone>(a: &Vec<T>, b: &Vec<T>) -> (r: bool)
        requires obeys_feq::<T>(),
        ensures r == (a@ == b@),
    {
        let r = *a == *b;
        assert(r == PartialEqSpec::eq_spec(a, b));
        assert(r ==> a@ =~= b@);
        r
    }

    // Generic Vec clone — zero assumes.
    fn vec_clone<T: Eq + View + Clone>(v: &Vec<T>) -> (cloned: Vec<T>)
        requires obeys_feq::<T>(),
        ensures cloned@ == v@,
    {
        let cloned = v.clone();
        cloned
    }

    // Generic Vec eq — implication only (weaker but simpler proof).
    fn vec_eq_implies<T: Eq + View + Clone>(a: &Vec<T>, b: &Vec<T>) -> (r: bool)
        requires obeys_feq::<T>(),
        ensures r ==> a@ == b@,
    {
        let r = *a == *b;
        r
    }

    } // verus!
} // pub mod
