// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r212): `IntoIterator` form A for a slice-backed collection.
//!
//! Question. Verus 0.2026.09.13 rejects `requires` on an impl of an external
//! trait's method (`src/experiments/iter_requires_on_external_trait.rs`), so
//! `IntoIterator for &SliceS` cannot require `spec_slices_wf()`, yet its body
//! indexes `data[start..start + len]`, which needs `start + len <= data.len()`.
//! Form A carries the well-formedness as a `#[verifier::type_invariant]` on
//! the struct: the fields become private, `open spec fn view` goes through a
//! closed accessor, and `into_iter` obtains the bound from
//! `use_type_invariant(self)` with no `requires`.
//!
//! Cost at the definer: `pub` dropped from three fields, one attribute, one
//! closed accessor `spec_backing_seq`, one `proof { use_type_invariant }` line
//! in `iter` and in `into_iter`; every constructor must establish the
//! invariant, and every struct literal outside a constructor must too.
//!
//! RESULT: SUCCEEDS — 8 verified, 0 errors
//! DATE: 2026-09-21
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-intoiter_form_a_type_invariant.20260921-164420.log
//! Finding: `use_type_invariant(self)` discharges the slice bound in an
//! external-trait impl with no `requires`; but the closed accessor hides the
//! sequence's length, so a caller's `usize` counter needs `lemma_backing_seq_len`
//! (a `tracked &self` proof fn) and its bound as a loop invariant, and every
//! struct literal and field reader outside the impl must change.

pub mod intoiter_form_a_type_invariant {

    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct SliceS<T> {
        data: Vec<T>,
        start: usize,
        len: usize,
    }

    impl<T> SliceS<T> {
        #[verifier::type_invariant]
        pub closed spec fn spec_slices_wf(self) -> bool {
            self.start + self.len <= self.data@.len()
        }

        /// The elements the slice covers; the only spec-mode reader of the
        /// private fields.
        pub closed spec fn spec_backing_seq(self) -> Seq<T> {
            self.data@.subrange(self.start as int, (self.start + self.len) as int)
        }

        /// A closed accessor hides the length bound a caller's `usize`
        /// counter needs; form A pays for it with a lemma.
        pub proof fn lemma_backing_seq_len(tracked &self)
            ensures
                self.spec_backing_seq().len() <= usize::MAX,
        {
            use_type_invariant(self);
        }

        pub fn new(data: Vec<T>, start: usize, len: usize) -> (s: Self)
            requires
                start + len <= data@.len(),
            ensures
                s.spec_backing_seq() == data@.subrange(start as int, (start + len) as int),
        {
            SliceS { data, start, len }
        }

        pub fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self.spec_backing_seq().as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.spec_backing_seq(),
                IteratorSpec::decrease(&it) is Some,
        {
            proof { use_type_invariant(self); }
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
                IteratorSpec::remaining(&it) == self.spec_backing_seq().as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.spec_backing_seq(),
                IteratorSpec::decrease(&it) is Some,
        {
            proof { use_type_invariant(self); }
            let sl: &[T] = &self.data.as_slice()[self.start..][..self.len];
            sl.iter()
        }
    }

    /// A consumer through `iter()`.
    pub fn count_iter(s: &SliceS<u64>) -> (n: usize)
        ensures
            n == s.spec_backing_seq().len(),
    {
        proof { s.lemma_backing_seq_len(); }
        let mut n: usize = 0;
        for x in it: s.iter()
            invariant
                it.seq() == s.spec_backing_seq().as_ref(),
                n == it.index(),
                s.spec_backing_seq().len() <= usize::MAX,
        {
            n = n + 1;
        }
        n
    }

    /// A consumer through `IntoIterator for &SliceS`, the `for x in &s` form.
    pub fn count_into(s: &SliceS<u64>) -> (n: usize)
        ensures
            n == s.spec_backing_seq().len(),
    {
        proof { s.lemma_backing_seq_len(); }
        let mut n: usize = 0;
        for x in it: (&*s).into_iter()
            invariant
                it.seq() == s.spec_backing_seq().as_ref(),
                n == it.index(),
                s.spec_backing_seq().len() <= usize::MAX,
        {
            n = n + 1;
        }
        n
    }

    } // verus!
} // pub mod intoiter_form_a_type_invariant
