// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: the consuming iteration pattern under the verus 0.2026.05.21
//! prophetic iterator model (PR #2163).
//!
//! A `Vec`-backed collection exposes `IntoIterator for Self` by returning
//! `std::vec::IntoIter` directly — no custom iterator struct. The loop yields
//! **owned** `T` values. `vec::IntoIter` already implements `IteratorSpecImpl`
//! in vstd, so the collection adds nothing of its own.
//!
//! Tested loop forms:
//!  - `for x in it: coll.into_iter()`   (for-consume, explicit)
//!  - `for x in it: coll`               (for-consume, sugared)
//!  - manual `loop { match it.next() }` (loop-consume)
//!
//! RESULT: SUCCEEDS — 7 verified, 0 errors (verus 0.2026.05.21, standalone).
//! Finding: a Vec-backed collection exposes consuming iteration by returning
//! `std::vec::IntoIter` directly; `for x in coll` and a manual loop-consume
//! both verify.

use vstd::prelude::*;
use vstd::std_specs::iter::*;

verus! {

#[verifier::reject_recursive_types(T)]
pub struct ExampleS<T> {
    pub seq: Vec<T>,
}

// IntoIterator for Self: consuming iteration yielding owned T.
impl<T> std::iter::IntoIterator for ExampleS<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> (it: Self::IntoIter)
        ensures
            IteratorSpec::remaining(&it) == self.seq@,
            IteratorSpec::decrease(&it) is Some,
            IteratorSpec::initial_value_relation(&it, &it),
    {
        broadcast use vstd::std_specs::vec::axiom_spec_into_iter;
        self.seq.into_iter()
    }
}

// Form 1: `for x in it: coll.into_iter()`, explicit.
fn test_for_consume(a: ExampleS<u64>) {
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a.into_iter()
        invariant
            it.seq() == orig,
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == it.seq()[i],
    {
        collected.push(x);
    }
    assert(collected@ =~= orig);
}

// Form 2: `for x in it: coll`, sugared — the for loop calls into_iter itself.
fn test_for_consume_sugar(a: ExampleS<u64>) {
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a
        invariant
            it.seq() == orig,
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == it.seq()[i],
    {
        collected.push(x);
    }
    assert(collected@ =~= orig);
}

// Form 3: manual `loop` over the consuming iterator.
fn test_loop_consume(a: ExampleS<u64>) {
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: VerusForLoopWrapper<'_, std::vec::IntoIter<u64>> =
        VerusForLoopWrapper::new(a.into_iter(), Ghost(None));
    loop
        invariant
            it.wf(),
            IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
            IteratorSpec::decrease(&it.iter) is Some,
            it.seq() == orig,
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == it.seq()[i],
        decreases IteratorSpec::decrease(&it.iter)->0,
    {
        match it.next() {
            Some(x) => { collected.push(x); },
            None => {
                assert(collected@ =~= orig);
                break;
            },
        }
    }
}

} // verus!

fn main() {}
