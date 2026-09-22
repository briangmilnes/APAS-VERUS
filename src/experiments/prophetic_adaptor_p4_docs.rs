// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r207, agent 3 probe): obeys_true plus /// doc comments on the two impls. Do doc comments on impls inside verus! change the result?
//!
//! RESULT: SUCCEEDS — 1 verified, 0 errors
//! DATE: 2026-09-20
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-exp_adaptor_p4_docs.20260920-154644.log
//! LOG: logs/validate-experiment-prophetic_adaptor_p4_docs.20260920-160411.log (re-run from this path)
//! Context: docs/StandardsUpgrade.md section 2.3; the wrapping (adaptor) iterator standard.

pub mod prophetic_adaptor_p4_docs {

    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct OuterIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    /// `next` forwards to the inner iterator.
    impl<'a, T> Iterator for OuterIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (ret: Option<&'a T>) {
            self.inner.next()
        }
    }

    /// Every spec fn forwards to the inner iterator's.
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
