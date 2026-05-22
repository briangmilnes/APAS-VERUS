// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Prophetic Iterator Standard: how to implement verified iterators in
//! APAS-VERUS under the verus 0.2026.05.21 iterator model (verus PR #2163,
//! "prophetic sequence encoding").
//!
//! This standard replaces the obsolete `iterators_standard.rs`, which was
//! written for the pre-#2163 `ForLoopGhostIterator` model. The old 10-component
//! pattern — custom `XxxIter` struct, `(int, Seq)` view, `XxxGhostIterator`,
//! `ForLoopGhostIteratorNew`/`ForLoopGhostIterator` impls — is gone. The new
//! model specifies iterators with the `IteratorSpec` external-trait extension
//! and drives `for` loops with `VerusForLoopWrapper`.
//!
//! Two iteration styles, both shown below.
//!
//! Delegated iteration — return a std iterator (the common case).
//! A `Vec`-backed collection needs no iterator type of its own. `iter()`
//! returns `std::slice::Iter` and `into_iter()` returns `std::vec::IntoIter`;
//! both already implement `IteratorSpecImpl` in vstd. The collection's only job
//! is an `ensures` pinning `IteratorSpec::remaining(&it)` to its contents.
//! See `struct ExampleS` below.
//!
//! Custom iteration — implement `IteratorSpecImpl` from scratch.
//! A collection with no slice underneath (e.g. a tree, traversed in order)
//! supplies its own iterator type and implements all six `IteratorSpecImpl`
//! spec functions by hand. See `struct CountIter` below.
//!
//! For-loop callers reference `it.index()` (items consumed) and `it.seq()`
//! (the prophetic full sequence). `it` is NOT in scope after the loop; verus
//! exports the invariant with the iterator's `when_used_as_spec` value
//! substituted. A manual `loop` drives termination with the non-prophetic
//! `IteratorSpec::decrease(&it.iter)->0` — `it.seq()` is prophetic and may not
//! appear in `decreases` — and must draw its conclusion before `break`.
//!
//! Loop forms exercised by the PTT (`Proveprophetic_iterators_standard.rs`):
//!   for-borrow-iter · for-borrow-into · for-consume · loop-borrow · loop-consume
//!
//!  Table of Contents
//!  1. module
//!  2. imports
//!  3. broadcast use
//!  4a. type definitions — struct CountIter (custom iteration)
//!  6a. spec fns — struct CountIter
//!  10a. iterators — struct CountIter
//!  4b. type definitions — struct ExampleS (delegated iteration)
//!  6b. spec fns — struct ExampleS
//!  8b. traits — struct ExampleS
//!  9b. impls — struct ExampleS
//!  10b. iterators — struct ExampleS
//!  14b. derive impls outside verus! — struct ExampleS

// 1. module
pub mod prophetic_iterators_standard {

    use std::fmt::{Debug, Formatter};

    // 2. imports
    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    // 3. broadcast use
    broadcast use vstd::std_specs::slice::axiom_spec_slice_iter;

    // Custom iteration: a from-scratch iterator. CountIter yields the owned values
    // start, start+1, ..., end-1. It wraps no std iterator, so every
    // IteratorSpecImpl method is written by hand. An APAS tree iterator
    // (in-order traversal) follows this same shape.

    // 4a. type definitions — struct CountIter
    //
    // Fields are private: a `#[verifier::type_invariant]` struct may not expose
    // crate-public fields, and the new model exposes nothing through raw
    // iterator fields — callers use `it.index()` and `it.seq()`.
    pub struct CountIter {
        start: u64,
        cur: u64,
        end: u64,
    }

    // 6a. spec fns — struct CountIter
    impl CountIter {
        // The type invariant: the cursor stays within [start, end].
        #[verifier::type_invariant]
        pub closed spec fn spec_countiter_wf(self) -> bool {
            self.start <= self.cur <= self.end
        }

        // Closed constructor — the only spec-mode mention of the private
        // fields, so `count_iter_spec` below can stay `open` for cross-module
        // callers without exposing the layout.
        pub closed spec fn spec_new(start: u64, end: u64) -> CountIter {
            CountIter { start, cur: start, end }
        }

        // The full creation-time sequence. Stable across `next()` — only `cur`
        // moves — so it anchors `peek` and `initial_value_relation`.
        pub closed spec fn elts(self) -> Seq<u64> {
            Seq::new((self.end - self.start) as nat, |i: int| (self.start + i) as u64)
        }
    }

    // Spec form of the `count_iter` constructor. `when_used_as_spec` on the
    // exec function points here, letting `for` loops reason about the iterator.
    pub open spec fn count_iter_spec(start: u64, end: u64) -> CountIter {
        CountIter::spec_new(start, end)
    }

    // 10a. iterators — struct CountIter

    // Exec constructor. `when_used_as_spec` ties it to `count_iter_spec` so the
    // loop header `for x in it: count_iter(a, b)` has a spec value.
    //
    // The `ensures` states `remaining()` explicitly. `remaining()` is a closed
    // spec fn, so a caller in another module cannot unfold it — the contract
    // must hand the sequence over directly, exactly as `<[T]>::iter` does.
    #[verifier::when_used_as_spec(count_iter_spec)]
    pub fn count_iter(start: u64, end: u64) -> (it: CountIter)
        requires
            start <= end,
        ensures
            it == count_iter_spec(start, end),
            IteratorSpec::remaining(&it)
                == Seq::new((end - start) as nat, |i: int| (start + i) as u64),
            IteratorSpec::decrease(&it) is Some,
            IteratorSpec::initial_value_relation(&it, &it),
    {
        CountIter { start, cur: start, end }
    }

    // `Iterator::next` carries no `ensures` of its own — the specification is
    // the `IteratorSpecImpl` block below, against which verus checks this body.
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

    // The six-function prophetic specification.
    impl IteratorSpecImpl for CountIter {
        // CountIter always terminates and applies no fallible closure.
        open spec fn obeys_prophetic_iter_laws(&self) -> bool {
            true
        }

        // The items still to be returned: a shrinking suffix of `elts()`.
        closed spec fn remaining(&self) -> Seq<u64> {
            Seq::new((self.end - self.cur) as nat, |i: int| (self.cur + i) as u64)
        }

        // CountIter always finishes with a `None`, never hangs.
        closed spec fn will_return_none(&self) -> bool {
            true
        }

        // Non-prophetic termination metric — usable in a manual `decreases`.
        closed spec fn decrease(&self) -> Option<nat> {
            Some((self.end - self.cur) as nat)
        }

        // Relates a live iterator to the value it was created from.
        #[verifier::prophetic]
        open spec fn initial_value_relation(&self, init: &Self) -> bool {
            &&& IteratorSpec::remaining(init) == IteratorSpec::remaining(self)
            &&& init.elts() == self.elts()
        }

        // A guess at the index-th item, drawn from the stable `elts()`.
        open spec fn peek(&self, index: int) -> Option<u64> {
            if 0 <= index < self.elts().len() {
                Some(self.elts()[index])
            } else {
                None
            }
        }
    }

    // Delegated iteration: a Vec-backed collection. It defines no iterator type; `iter`
    // and `into_iter` return the vstd-specified std iterators directly.

    // 4b. type definitions — struct ExampleS
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    // 6b. spec fns — struct ExampleS
    impl<T> ExampleS<T> {
        pub open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        pub open spec fn spec_index(&self, i: int) -> T
            recommends
                0 <= i < self.spec_len(),
        {
            self.seq@[i]
        }
    }

    // 8b. traits — struct ExampleS
    pub trait ExampleTrait<T>: Sized + View<V = Seq<T>> {
        // Borrowing iterator entry point. The `ensures` pins the iterator's
        // prophetic `remaining()` to the collection contents, so a `for` loop
        // invariant can name `self.seq@` through `it.seq()`.
        fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                IteratorSpec::decrease(&it) is Some,
                IteratorSpec::initial_value_relation(&it, &it),
        ;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s@.len() == length as nat,
        ;
    }

    // 9b. impls — struct ExampleS
    impl<T> View for ExampleS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    impl<T> ExampleTrait<T> for ExampleS<T> {
        fn iter(&self) -> (it: std::slice::Iter<'_, T>) {
            self.seq.iter()
        }

        fn new(length: usize, init: T) -> (s: Self) where T: Copy {
            let mut v: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    v@.len() == i as int,
                decreases length - i,
            {
                v.push(init);
                i = i + 1;
            }
            ExampleS { seq: v }
        }
    }

    // 10b. iterators — struct ExampleS

    // `for x in &coll` — borrowing. Returns the std slice iterator directly.
    impl<'a, T> std::iter::IntoIterator for &'a ExampleS<T> {
        type Item = &'a T;

        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self.seq@.as_ref(),
                IteratorSpec::decrease(&it) is Some,
                IteratorSpec::initial_value_relation(&it, &it),
        {
            self.seq.iter()
        }
    }

    // `for x in coll` — consuming. Returns the std vec iterator directly,
    // yielding owned `T`.
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

    } // verus!

    // 14b. derive impls outside verus! — struct ExampleS
    impl<T: Debug> Debug for ExampleS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleS({:?})", self.seq)
        }
    }
} // pub mod prophetic_iterators_standard
