// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r212): `IntoIterator` form B for a slice-backed collection.
//!
//! Question. Verus 0.2026.09.13 rejects `requires` on an impl of an external
//! trait's method (`src/experiments/iter_requires_on_external_trait.rs`), so
//! `IntoIterator for &SliceS` cannot require `spec_slices_wf()`, yet its body
//! indexes `data[start..start + len]`, which needs `start + len <= data.len()`.
//! Form B makes `into_iter` total: one O(1) bounds check in exec, the empty
//! slice on the non-well-formed branch, and the constructor triple in the
//! `ensures` under the premise `self.spec_slices_wf() ==>`. The fields stay
//! `pub`, `view` stays `open`, and `iter()` keeps its `requires`.
//!
//! Cost at the definer: two exec lines (the branch), one premise on the
//! `ensures`. Cost at a call site: none; a well-formed collection gets the
//! triple as before. Work and span of `into_iter` stay O(1).
//!
//! RESULT: SUCCEEDS — 7 verified, 0 errors
//! DATE: 2026-09-21
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-intoiter_form_b_total_body.20260921-164442.log
//! Finding: the conditional triple verifies on both branches; a consumer's
//! `for` loop carries `s.spec_slices_wf()` in its invariant (the loop is
//! isolated), which the old `requires` form needed as well. Chosen for r212.

pub mod intoiter_form_b_total_body {

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

    impl<'a, T> std::iter::IntoIterator for &'a SliceS<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                self.spec_slices_wf() ==> {
                    &&& IteratorSpec::remaining(&it) == self.spec_backing_seq().as_ref()
                    &&& vstd::std_specs::slice::into_iter_elts(it) == self.spec_backing_seq()
                    &&& IteratorSpec::decrease(&it) is Some
                },
        {
            // The bound the deleted `requires` carried, checked in O(1) with
            // no overflow: `len <= data.len()` first, then `start <= data.len() - len`.
            let sl: &[T] = if self.len <= self.data.len() && self.start <= self.data.len() - self.len {
                &self.data.as_slice()[self.start..][..self.len]
            } else {
                &self.data.as_slice()[0..0]
            };
            sl.iter()
        }
    }

    /// A consumer through `iter()`.
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

    /// A consumer through `IntoIterator for &SliceS`, the `for x in &s` form.
    pub fn count_into(s: &SliceS<u64>) -> (n: usize)
        requires
            s.spec_slices_wf(),
        ensures
            n == s.spec_backing_seq().len(),
    {
        let mut n: usize = 0;
        for x in it: (&*s).into_iter()
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
} // pub mod intoiter_form_b_total_body
