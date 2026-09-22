// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r207, agent 3 probe): the wrapping standard minus InnerTrait, OuterTrait, and their impls. Does removing the traits make the adaptor's next() verify?
//!
//! RESULT: SUCCEEDS — 3 verified, 0 errors
//! DATE: 2026-09-20
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-exp_adaptor_pb_notraits.20260920-154755.log
//! LOG: logs/validate-experiment-prophetic_adaptor_pb_notraits.20260920-160414.log (re-run from this path)
//! Context: docs/StandardsUpgrade.md section 2.3; the wrapping (adaptor) iterator standard.

pub mod prophetic_adaptor_pb_notraits {

    use std::fmt::{Debug, Display, Formatter};
    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct InnerS<T> { pub seq: Vec<T> }

    impl<T> View for InnerS<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.seq@ }
    }

    impl<'a, T> std::iter::IntoIterator for &'a InnerS<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self.seq@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
        { self.seq.iter() }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OuterS<T> { pub data: InnerS<T> }

    impl<T> View for OuterS<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.data@ }
    }

    impl<'a, T> std::iter::IntoIterator for &'a OuterS<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self@,
                IteratorSpec::decrease(&it) is Some,
        { self.data.seq.iter() }
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

    impl<T: Debug> Debug for InnerS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "InnerS({:?})", self.seq) }
    }
    impl<T: Display> Display for InnerS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "InnerS") }
    }
    impl<T: Debug> Debug for OuterS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "OuterS({:?})", self.data) }
    }
    impl<T: Display> Display for OuterS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{}", self.data) }
    }
    impl<'a, T: Debug> Debug for OuterIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "OuterIter({:?})", self.inner) }
    }
    impl<'a, T> Display for OuterIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "OuterIter") }
    }
}
