// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Wrapping Iterators Standard: iterating a collection that wraps another.
//!
//! When a module wraps another module's collection (MappingStEph wraps
//! RelationStEph, RelationStEph wraps SetStEph, the Chap42 tables wrap an
//! ArraySeq), the outer collection's iterator is the inner collection's
//! iterator. Under the verus 0.2026.09.13 prophetic iterator model there are
//! two ways to expose it, shown below on InnerS (Vec-backed) and OuterS
//! (wraps InnerS).
//!
//! Pattern A — re-expose (the default). `OuterS::iter()` returns exactly the
//! type `InnerS::iter()` returns (here `std::slice::Iter`) by calling it, and
//! restates the three constructor postconditions over OuterS's own view. No
//! new type, no new spec, no proof obligation: vstd's `IteratorSpecImpl` for
//! the std iterator carries through. Fifteen APAS iterators re-expose a
//! backing collection's iterator this way (docs/PropheticIterators.md, row 6).
//!
//! Pattern B — adaptor by delegation. When the outer module must own its
//! iterator type (to keep the inner representation out of its public API, or
//! to adapt items), it defines `OuterIter { inner }`, forwards `next()` to
//! `inner.next()`, and implements `IteratorSpecImpl` by forwarding
//! `remaining`, `will_return_none`, `decrease` and `peek` to the inner
//! iterator's, as vstd does for `&mut I` (vstd/std_specs/iter.rs,
//! `impl IteratorSpecImpl for &mut I`). `obeys_prophetic_iter_laws` is
//! written as the inner iterator's value, the constant `true` for a std
//! iterator, not as a forwarding call (see the comment on the impl). The two
//! prophetic spec fns, `remaining` and `will_return_none`, carry
//! `#[verifier::prophetic]` because they call prophetic spec fns. Verus then
//! re-checks the `Iterator::next` contract for the adaptor once, and the
//! delegation discharges it.
//!
//! Choose A unless B's reason applies. Both have the same cost: `iter()` is
//! O(1) and `next()` adds one O(1) forwarding call.
//!
//! One rule for B's constructor: when it is a trait method, its `ensures`
//! name the inner iterator (`IteratorSpec::remaining(&it.inner)`), not the
//! adaptor (`IteratorSpec::remaining(&it)`). See the comment on
//! `OuterTrait::iter_adapted` and docs/StandardsUpgrade.md for the measured
//! behaviour behind this rule.
//!
//! The old model's `OuterIter`/`OuterGhostIterator` pair, its `(int, Seq)`
//! view, `iter_invariant`, and the `ForLoopGhostIterator` impls are gone.
//!
//! References:
//! - src/standards/iterators_standard.rs (the delegated style and loop idioms)
//! - src/standards/prophetic_iterators_standard.rs (the custom style)
//! - src/Chap05/MappingStEph.rs (wraps RelationStEph)
//! - src/Chap05/RelationStEph.rs (wraps SetStEph)

//  Table of Contents
//	1. module
//	2. imports
//	4a. type definitions — struct InnerS
//	5a. view impls — struct InnerS
//	8a. traits — struct InnerS
//	9a. impls — struct InnerS
//	10a. iterators — struct InnerS
//	4b. type definitions — struct OuterS
//	5b. view impls — struct OuterS
//	8b. traits — struct OuterS
//	9b. impls — struct OuterS
//	10b. iterators — struct OuterS
//	14a. derive impls outside verus! — struct InnerS
//	14b. derive impls outside verus! — struct OuterS

//		1. module

pub mod wrapping_iterators_standard {

    use std::fmt::{Debug, Display, Formatter};

    //		2. imports
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;

    verus! {

    //		4a. type definitions — struct InnerS

    // Inner layer: a Vec-backed collection (like SetStEph or ArraySeqStEph).
    #[verifier::reject_recursive_types(T)]
    pub struct InnerS<T> {
        pub seq: Vec<T>,
    }

    //		5a. view impls — struct InnerS

    impl<T> View for InnerS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    //		8a. traits — struct InnerS

    pub trait InnerTrait<T>: Sized + View<V = Seq<T>> {
        spec fn spec_len(&self) -> nat;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;

        // Delegated iteration: the std slice iterator, with the constructor
        // triple from iterators_standard.rs.
        fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self@,
                IteratorSpec::decrease(&it) is Some,
        ;
    }

    //		9a. impls — struct InnerS

    impl<T> InnerTrait<T> for InnerS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
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
                i += 1;
            }
            InnerS { seq: v }
        }

        fn iter(&self) -> (it: std::slice::Iter<'_, T>) {
            self.seq.iter()
        }
    }

    //		10a. iterators — struct InnerS

    impl<'a, T> std::iter::IntoIterator for &'a InnerS<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self.seq@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
        {
            self.seq.iter()
        }
    }

    //		4b. type definitions — struct OuterS

    // Outer layer: wraps InnerS (like MappingStEph wraps RelationStEph).
    #[verifier::reject_recursive_types(T)]
    pub struct OuterS<T> {
        pub data: InnerS<T>,
    }

    //		5b. view impls — struct OuterS

    /// OuterS View delegates to the inner collection.
    impl<T> View for OuterS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.data@
        }
    }

    //		8b. traits — struct OuterS

    pub trait OuterTrait<T>: Sized + View<V = Seq<T>> {
        spec fn spec_len(&self) -> nat;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;

        /// Pattern A: re-expose the inner iterator. Same type, same triple,
        /// stated over the outer view.
        fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self@,
                IteratorSpec::decrease(&it) is Some,
        ;

        /// Pattern B: an adaptor type of the module's own. The postconditions
        /// are stated over the inner std iterator, `it.inner`, not over the
        /// adaptor's own `IteratorSpec` fns: a trait method whose `ensures`
        /// names `IteratorSpec::remaining(&it)` for a type whose `next` is
        /// verified in this crate makes that `next` check fail on verus
        /// 0.2026.09.13 (the adaptor's spec fns are then treated as opaque
        /// there); an inherent method or a free function does not. Callers
        /// lose nothing, because the adaptor's `remaining` is `open` and
        /// unfolds to `IteratorSpec::remaining(&it.inner)`. The middle clause
        /// names the adaptor's `elts()`, the non-prophetic contents `peek`
        /// reads.
        fn iter_adapted(&self) -> (it: OuterIter<'_, T>)
            ensures
                IteratorSpec::remaining(&it.inner) == self@.as_ref(),
                it.elts() == self@,
                IteratorSpec::decrease(&it.inner) is Some,
        ;
    }

    //		9b. impls — struct OuterS

    impl<T> OuterTrait<T> for OuterS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
        }

        fn new(length: usize, init: T) -> (s: Self) where T: Copy {
            OuterS { data: InnerS::new(length, init) }
        }

        fn iter(&self) -> (it: std::slice::Iter<'_, T>) {
            self.data.iter()
        }

        fn iter_adapted(&self) -> (it: OuterIter<'_, T>) {
            OuterIter { inner: self.data.iter() }
        }
    }

    //		10b. iterators — struct OuterS

    // Pattern A: `for x in &outer` re-exposes the inner iterator.
    impl<'a, T> std::iter::IntoIterator for &'a OuterS<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self@,
                IteratorSpec::decrease(&it) is Some,
        {
            self.data.iter()
        }
    }

    // Pattern B: the adaptor. It owns nothing but the inner iterator.
    #[verifier::reject_recursive_types(T)]
    pub struct OuterIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> OuterIter<'a, T> {
        /// The non-prophetic contents `peek` reads: the inner iterator's.
        pub open spec fn elts(self) -> Seq<T> {
            vstd::std_specs::slice::into_iter_elts(self.inner)
        }
    }

    /// `next` forwards to the inner iterator. Its contract is the
    /// `IteratorSpec` one, checked against the delegating spec below.
    impl<'a, T> Iterator for OuterIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (ret: Option<&'a T>) {
            self.inner.next()
        }
    }

    /// Every spec fn forwards to the inner iterator's, as vstd does for `&mut I`.
    /// Spec-only, so it exists only under Verus (the same gate as PartialEqSpecImpl).
    #[cfg(verus_keep_ghost)]
    impl<'a, T> IteratorSpecImpl for OuterIter<'a, T> {
        // The inner iterator's value, written out: vstd defines
        // `obeys_prophetic_iter_laws` for `std::slice::Iter` as `true`
        // (vstd/std_specs/slice.rs). The forwarding form
        // `IteratorSpec::obeys_prophetic_iter_laws(&self.inner)` does not
        // discharge the `next` contract on verus 0.2026.09.13 (the prover
        // does not unfold it twice); vstd's own `MyTake`/`MySkip` adaptors in
        // rust_verify_test/tests/iterators.rs also write the constant.
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

    //		14a. derive impls outside verus! — struct InnerS

    impl<T: Debug> Debug for InnerS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InnerS({:?})", self.seq)
        }
    }

    impl<T: Display> Display for InnerS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    //		14b. derive impls outside verus! — struct OuterS

    impl<T: Debug> Debug for OuterS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterS({:?})", self.data)
        }
    }

    impl<T: Display> Display for OuterS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.data)
        }
    }

    impl<'a, T: Debug> Debug for OuterIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for OuterIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterIter")
        }
    }
} // pub mod wrapping_iterators_standard
