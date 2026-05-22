// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: a from-scratch custom iterator under the verus 0.2026.05.21
//! prophetic iterator model (PR #2163).
//!
//! Unlike `prophetic_iter_slice_direct`, this iterator wraps no std iterator —
//! every `IteratorSpecImpl` method is written by hand. This is the pattern an
//! APAS tree iterator (in-order traversal, no slice underneath) must follow.
//! `CountIter` also yields **owned** `u64` values rather than `&T`.
//!
//! Components of the from-scratch pattern:
//!  - iterator struct + `#[verifier::type_invariant]`
//!  - stable `elts()` spec fn + spec constructor + `when_used_as_spec` ctor
//!  - `impl Iterator` (`next` — no `ensures`; the spec is the trait impl)
//!  - `impl IteratorSpecImpl` — 6 spec fns, prophetic `remaining()`
//!
//! RESULT: SUCCEEDS — 6 verified, 0 errors (verus 0.2026.05.21, standalone).
//! Finding: a from-scratch iterator needs private fields (a `type_invariant`
//! struct forbids crate-public ones), a `closed` constructor behind an `open`
//! spec ctor, and `next()` proves the prophetic postconditions with no asserts.

use vstd::prelude::*;
use vstd::std_specs::iter::*;

verus! {

// Yields the owned values start, start+1, ..., end-1.
// `start`/`end` are fixed at creation; `cur` advances.
// Fields are private: a `#[verifier::type_invariant]` struct may not expose
// crate-public fields, and the new model references `it.index()`/`it.seq()`
// rather than raw iterator fields.
pub struct CountIter {
    start: u64,
    cur: u64,
    end: u64,
}

impl CountIter {
    #[verifier::type_invariant]
    pub closed spec fn count_iter_type_inv(self) -> bool {
        self.start <= self.cur <= self.end
    }

    // Closed constructor: the only place the private fields are named in spec
    // mode. `count_iter_spec` (open) delegates here so it stays well-formed
    // for cross-module callers.
    pub closed spec fn spec_new(start: u64, end: u64) -> CountIter {
        CountIter { start, cur: start, end }
    }

    // The full, creation-time sequence — stable across `next()` calls.
    pub closed spec fn elts(self) -> Seq<u64> {
        Seq::new((self.end - self.start) as nat, |i: int| (self.start + i) as u64)
    }
}

pub open spec fn count_iter_spec(start: u64, end: u64) -> CountIter {
    CountIter::spec_new(start, end)
}

#[verifier::when_used_as_spec(count_iter_spec)]
pub fn count_iter(start: u64, end: u64) -> (it: CountIter)
    requires
        start <= end,
    ensures
        it == count_iter_spec(start, end),
        IteratorSpec::decrease(&it) is Some,
        IteratorSpec::initial_value_relation(&it, &it),
{
    CountIter { start, cur: start, end }
}

impl Iterator for CountIter {
    type Item = u64;

    fn next(&mut self) -> (ret: Option<u64>) {
        proof { use_type_invariant(&*self); }
        if self.cur < self.end {
            let v = self.cur;
            self.cur = self.cur + 1;
            Some(v)
        } else {
            None
        }
    }
}

impl IteratorSpecImpl for CountIter {
    open spec fn obeys_prophetic_iter_laws(&self) -> bool {
        true
    }

    closed spec fn remaining(&self) -> Seq<u64> {
        Seq::new((self.end - self.cur) as nat, |i: int| (self.cur + i) as u64)
    }

    closed spec fn will_return_none(&self) -> bool {
        true
    }

    closed spec fn decrease(&self) -> Option<nat> {
        Some((self.end - self.cur) as nat)
    }

    #[verifier::prophetic]
    open spec fn initial_value_relation(&self, init: &Self) -> bool {
        &&& IteratorSpec::remaining(init) == IteratorSpec::remaining(self)
        &&& init.elts() == self.elts()
    }

    open spec fn peek(&self, index: int) -> Option<u64> {
        if 0 <= index < self.elts().len() {
            Some(self.elts()[index])
        } else {
            None
        }
    }
}

// Form 1: `for x in it: count_iter(..)`. Collected vector equals 0..n.
fn test_for(n: u64) {
    let mut collected: Vec<u64> = Vec::new();
    for x in it: count_iter(0, n)
        invariant
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == it.seq()[i],
    {
        collected.push(x);
    }
    // `it` is out of scope after the loop; verus exports the invariant with the
    // iterator's spec value (`count_iter` has `when_used_as_spec`) substituted.
    assert(collected@ =~= Seq::new(n as nat, |i: int| i as u64));
}

// Form 2: manual `loop` driven by the iterator's non-prophetic `decrease()`.
fn test_loop(n: u64) {
    let mut collected: Vec<u64> = Vec::new();
    let mut it: VerusForLoopWrapper<'_, CountIter> =
        VerusForLoopWrapper::new(count_iter(0, n), Ghost(None));
    loop
        invariant
            it.wf(),
            IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
            IteratorSpec::decrease(&it.iter) is Some,
            it.seq() =~= Seq::new(n as nat, |i: int| i as u64),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == it.seq()[i],
        decreases IteratorSpec::decrease(&it.iter)->0,
    {
        match it.next() {
            Some(x) => { collected.push(x); },
            None => {
                assert(collected@ =~= Seq::new(n as nat, |i: int| i as u64));
                break;
            },
        }
    }
}

} // verus!

fn main() {}
