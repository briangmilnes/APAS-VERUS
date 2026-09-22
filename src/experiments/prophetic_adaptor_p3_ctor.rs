// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r207, agent 3 probe): obeys_true plus a free-fn constructor whose ensures name IteratorSpec::remaining and decrease on the adaptor. Does a free fn naming the adaptor's IteratorSpec fns break next()?
//!
//! RESULT: SUCCEEDS — 2 verified, 0 errors
//! DATE: 2026-09-20
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-exp_adaptor_p3_ctor.20260920-154643.log
//! LOG: logs/validate-experiment-prophetic_adaptor_p3_ctor.20260920-160409.log (re-run from this path)
//! Context: docs/StandardsUpgrade.md section 2.3; the wrapping (adaptor) iterator standard.

pub mod prophetic_adaptor_p3_ctor {

    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    pub fn make<'a, T>(v: &'a Vec<T>) -> (it: OuterIter<'a, T>)
        ensures
            IteratorSpec::remaining(&it) == v@.as_ref(),
            IteratorSpec::decrease(&it) is Some,
    {
        OuterIter { inner: v.iter() }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OuterIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> Iterator for OuterIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (ret: Option<&'a T>) {
            self.inner.next()
        }
    }

    impl<'a, T> IteratorSpecImpl for OuterIter<'a, T> {
        open spec fn obeys_prophetic_iter_laws(&self) -> bool {
            true
        }

        #[verifier::prophetic]
        open spec fn remaining(&self) -> Seq<&'a T> {
            IteratorSpec::remaining(&self.inner)
        }

        #[verifier::prophetic]
        open spec fn will_return_none(&self) -> bool {
            IteratorSpec::will_return_none(&self.inner)
        }

        open spec fn decrease(&self) -> Option<nat> {
            IteratorSpec::decrease(&self.inner)
        }

        open spec fn peek(&self, index: int) -> Option<&'a T> {
            IteratorSpec::peek(&self.inner, index)
        }
    }

    } // verus!
}
