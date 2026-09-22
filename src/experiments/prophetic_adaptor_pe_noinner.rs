// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r207, agent 3 probe): pa_nointoiter minus InnerTrait, keeping OuterTrait with iter and iter_adapted. Does a trait method whose ensures name IteratorSpec::remaining(&it) on a same-crate iterator type make that type's next() fail?
//!
//! RESULT: FAILS — 2 verified, 1 errors; next() fails its contract check
//! DATE: 2026-09-20
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-exp_adaptor_pe_noinner.20260920-154840.log
//! LOG: logs/validate-experiment-prophetic_adaptor_pe_noinner.20260920-160419.log (re-run from this path)
//! Context: docs/StandardsUpgrade.md section 2.3; the wrapping (adaptor) iterator standard.

pub mod prophetic_adaptor_pe_noinner {

    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct InnerS<T> { pub seq: Vec<T> }

    impl<T> View for InnerS<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.seq@ }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OuterS<T> { pub data: InnerS<T> }

    impl<T> View for OuterS<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.data@ }
    }

    pub trait OuterTrait<T>: Sized + View<V = Seq<T>> {
        spec fn spec_len(&self) -> nat;
        fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self@,
                IteratorSpec::decrease(&it) is Some;
        fn iter_adapted(&self) -> (it: OuterIter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                it.elts() == self@,
                IteratorSpec::decrease(&it) is Some;
    }

    impl<T> OuterTrait<T> for OuterS<T> {
        open spec fn spec_len(&self) -> nat { self@.len() }
        fn iter(&self) -> (it: std::slice::Iter<'_, T>) { self.data.seq.iter() }
        fn iter_adapted(&self) -> (it: OuterIter<'_, T>) { OuterIter { inner: self.data.seq.iter() } }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OuterIter<'a, T> { pub inner: std::slice::Iter<'a, T> }

    impl<'a, T> OuterIter<'a, T> {
        pub open spec fn elts(self) -> Seq<T> { vstd::std_specs::slice::into_iter_elts(self.inner) }
    }

    impl<'a, T> Iterator for OuterIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> (ret: Option<&'a T>) { self.inner.next() }
    }

    impl<'a, T> IteratorSpecImpl for OuterIter<'a, T> {
        open spec fn obeys_prophetic_iter_laws(&self) -> bool { true }
        #[verifier::prophetic]
        open spec fn remaining(&self) -> Seq<&'a T> { IteratorSpec::remaining(&self.inner) }
        #[verifier::prophetic]
        open spec fn will_return_none(&self) -> bool { IteratorSpec::will_return_none(&self.inner) }
        open spec fn decrease(&self) -> Option<nat> { IteratorSpec::decrease(&self.inner) }
        open spec fn peek(&self, index: int) -> Option<&'a T> { IteratorSpec::peek(&self.inner, index) }
    }

    } // verus!
}
