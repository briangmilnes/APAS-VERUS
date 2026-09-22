// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r212): `IntoIterator` form C for a slice-backed collection.
//!
//! Question. Verus 0.2026.09.13 rejects `requires` on an impl of an external
//! trait's method (`src/experiments/iter_requires_on_external_trait.rs`), so
//! `IntoIterator for &SliceS` cannot require `spec_slices_wf()`. Form C
//! writes no `IntoIterator` impl at all: `iter()` keeps its `requires`, and
//! every caller writes `for x in it: s.iter()`. A `for x in &s` does not
//! compile, since `&SliceS` is not `IntoIterator`.
//!
//! Cost at the definer: the impl is deleted (nothing to prove). Cost at the
//! call sites: every `for x in &coll` and `(&coll).into_iter()` in the crate,
//! its run-time tests and its proof-time tests becomes `coll.iter()`; the
//! `loop-borrow-into` and `for-borrow-into` proof-time patterns of
//! `src/standards/iterator_ptt_standard.rs` cannot be written.
//!
//! RESULT: SUCCEEDS — 4 verified, 0 errors
//! DATE: 2026-09-21
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-intoiter_form_c_no_impl.20260921-164448.log
//! Finding: nothing to prove at the definer; the cost is the 266 `(&a).into_iter()`
//! proof-time-test sites, 46 `for x in &coll` run-time-test loops and 16 source
//! sites that would have to change, and the loss of two of the six PTT patterns.

pub mod intoiter_form_c_no_impl {

    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct SliceS<T> {
        pub data: Vec<T>,
        pub start: usize,
        pub len: usize,
    }

    impl<T> SliceS<T> {
        pub open spec fn spec_slices_wf(self) -> bool {
            self.start + self.len <= self.data@.len()
        }

        pub open spec fn spec_backing_seq(self) -> Seq<T> {
            self.data@.subrange(self.start as int, (self.start + self.len) as int)
        }

        pub fn new(data: Vec<T>, start: usize, len: usize) -> (s: Self)
            requires
                start + len <= data@.len(),
            ensures
                s.spec_slices_wf(),
                s.spec_backing_seq() == data@.subrange(start as int, (start + len) as int),
        {
            SliceS { data, start, len }
        }

        pub fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            requires
                self.spec_slices_wf(),
            ensures
                IteratorSpec::remaining(&it) == self.spec_backing_seq().as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.spec_backing_seq(),
                IteratorSpec::decrease(&it) is Some,
        {
            let sl: &[T] = &self.data.as_slice()[self.start..][..self.len];
            sl.iter()
        }
    }

    impl<T> View for SliceS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.spec_backing_seq()
        }
    }

    /// The only consumer form: through `iter()`.
    pub fn count_iter(s: &SliceS<u64>) -> (n: usize)
        requires
            s.spec_slices_wf(),
        ensures
            n == s.spec_backing_seq().len(),
    {
        let mut n: usize = 0;
        for x in it: s.iter()
            invariant
                it.seq() == s.spec_backing_seq().as_ref(),
                n == it.index(),
                s.spec_slices_wf(),
        {
            n = n + 1;
        }
        n
    }

    } // verus!
} // pub mod intoiter_form_c_no_impl
