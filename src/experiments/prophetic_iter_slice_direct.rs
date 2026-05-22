// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: a Vec-backed collection that exposes iteration by returning
//! `std::slice::Iter` directly, with no custom iterator struct.
//!
//! This explores the verus 0.2026.05.21 prophetic iterator model (PR #2163)
//! and the central APAS migration question: can a Vec-backed collection drop
//! its hand-written `XxxIter` wrapper entirely and lean on the vstd
//! `IteratorSpecImpl` for `std::slice::Iter`?
//!
//! Tested loop forms over `coll.iter()`:
//!  - `for x in it: coll.iter()` with index()/seq() invariants
//!  - manual `loop { match it.next() }` via VerusForLoopWrapper
//!  - `for x in &coll` through `IntoIterator for &Coll`
//!
//! RESULT: SUCCEEDS — 8 verified, 0 errors (verus 0.2026.05.21, standalone).
//! Finding: a Vec-backed collection needs no custom iterator type. `for` loops
//! verify cleanly; a manual `loop` must draw its conclusion before `break`
//! because the prophetic `seq()` equality does not propagate past `break`.

use vstd::prelude::*;
use vstd::std_specs::iter::*;

verus! {

#[verifier::reject_recursive_types(T)]
pub struct ExampleS<T> {
    pub seq: Vec<T>,
}

impl<T> ExampleS<T> {
    /// Iteration entry point. Returns the bare slice iterator; the collection
    /// adds no iterator type of its own. The ensures pins `remaining()` to the
    /// collection contents so for-loop invariants can name `coll.seq@`.
    pub fn iter(&self) -> (it: std::slice::Iter<'_, T>)
        ensures
            IteratorSpec::remaining(&it) == self.seq@.as_ref(),
            IteratorSpec::decrease(&it) is Some,
            IteratorSpec::initial_value_relation(&it, &it),
    {
        broadcast use vstd::std_specs::slice::axiom_spec_slice_iter;
        self.seq.iter()
    }
}

// IntoIterator for &Self, enabling `for x in &coll`.
impl<'a, T> std::iter::IntoIterator for &'a ExampleS<T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> (it: Self::IntoIter)
        ensures
            IteratorSpec::remaining(&it) == self.seq@.as_ref(),
            IteratorSpec::decrease(&it) is Some,
            IteratorSpec::initial_value_relation(&it, &it),
    {
        broadcast use vstd::std_specs::slice::axiom_spec_slice_iter;
        self.seq.iter()
    }
}

// Form 1: `for x in it: coll.iter()`. The collected vector must equal the
// collection contents on loop exit.
fn test_for_borrow_iter(a: &ExampleS<u64>) {
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a.iter()
        invariant
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == *it.seq()[i],
    {
        collected.push(*x);
    }
    assert(collected@ =~= orig);
}

// Form 2: manual `loop` over a VerusForLoopWrapper around the slice iterator.
// `seq()` is prophetic and cannot drive `decreases`, so the loop measure is the
// underlying iterator's non-prophetic `decrease()` metric.
fn test_loop_borrow_iter(a: &ExampleS<u64>) {
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: VerusForLoopWrapper<'_, std::slice::Iter<'_, u64>> =
        VerusForLoopWrapper::new(a.iter(), Ghost(None));
    loop
        invariant
            it.wf(),
            IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
            IteratorSpec::decrease(&it.iter) is Some,
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == *it.seq()[i],
        decreases IteratorSpec::decrease(&it.iter)->0,
    {
        match it.next() {
            Some(x) => { collected.push(*x); },
            None => {
                // `next()` returning None pins the index to the sequence end.
                // The conclusion is drawn here because the prophetic `seq()`
                // equality does not propagate cleanly past a `break`.
                assert(it.index() == it.seq().len());
                assert(collected@ =~= orig);
                break;
            },
        }
    }
}

// Form 3: `for x in &coll` through `IntoIterator for &Self`.
fn test_for_borrow_into(a: &ExampleS<u64>) {
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a
        invariant
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == *it.seq()[i],
    {
        collected.push(*x);
    }
    assert(collected@ =~= orig);
}

} // verus!

fn main() {}
