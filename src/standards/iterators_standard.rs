// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Iterator Standard: delegated iteration for a Vec-backed collection under
//! the verus 0.2026.09.13 prophetic iterator model.
//!
//! Every APAS-VERUS collection supports verified iteration via both a manual
//! `loop` and a Verus `for x in it: ...`. A collection whose elements live in
//! a `Vec<T>` implements iteration by returning the std iterator directly:
//! vstd already supplies `IteratorSpecImpl` for `std::slice::Iter` and
//! `std::vec::IntoIter`, so the collection writes no iterator type, no `View`
//! for it, and no ghost iterator. The pre-#2163 10-component pattern is gone.
//!
//! Required components (all inside verus!):
//!  1. `iter(&self) -> std::slice::Iter<'_, T>` in the collection trait
//!     (section 8), with the three constructor postconditions below.
//!  2. `IntoIterator for &Self` returning the same std iterator with the same
//!     postconditions (section 10). Enables `for x in &coll`.
//!  3. Optional: `IntoIterator for Self` returning `std::vec::IntoIter<T>`
//!     (section 10). Enables `for x in coll`, yielding owned `T`.
//!
//! Constructor postconditions, for a borrowing iterator `it` over `self.seq`:
//!     IteratorSpec::remaining(&it) == self.seq@.as_ref(),      // prophetic
//!     vstd::std_specs::slice::into_iter_elts(it) == self.seq@, // what peek reads
//!     IteratorSpec::decrease(&it) is Some,                     // for-loop termination
//! For the consuming iterator drop `.as_ref()` and use
//! `vstd::std_specs::vec::into_iter_elts`.
//!
//! Loop idioms (proved in
//! rust_verify_test/tests/standards/Proveiterators_standard.rs):
//!
//!   for x in it: coll.iter()
//!       invariant
//!           it.seq() == orig.as_ref(),
//!           collected.len() == it.index(),
//!           forall|i: int| 0 <= i < collected.len()
//!               ==> #[trigger] collected@[i] == *it.seq()[i],
//!   {
//!       collected.push(*x);
//!   }
//!   assert(collected@ =~= orig);   // it.index() == it.seq().len() after the loop
//!
//!   let mut it = coll.iter();
//!   let ghost mut pos: int = 0;
//!   loop
//!       invariant
//!           IteratorSpec::obeys_prophetic_iter_laws(&it),
//!           IteratorSpec::decrease(&it) is Some,
//!           0 <= pos <= orig.len(),
//!           IteratorSpec::remaining(&it).len() == orig.len() - pos,
//!           forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
//!               ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
//!           collected.len() == pos,
//!           forall|i: int| 0 <= i < collected.len()
//!               ==> #[trigger] collected@[i] == orig[i],
//!       decreases IteratorSpec::decrease(&it)->0,
//!   {
//!       let ghost old_pos = pos;
//!       match it.next() {
//!           Some(x) => {
//!               proof { pos = pos + 1; assert(orig[old_pos] == *x); }
//!               collected.push(*x);
//!           },
//!           None => { assert(pos == orig.len()); assert(collected@ =~= orig); break; },
//!       }
//!   }
//!
//! A manual loop runs on the iterator's own `next()`, whose contract is the
//! `IteratorSpec` one (vstd/std_specs/iter.rs), not on `VerusForLoopWrapper`:
//! that wrapper lives in `vstd::std_specs`, which vstd declares only under
//! `verus_keep_ghost`, so a wrapper loop does not compile under `cargo`
//! (experiment: src/experiments/prophetic_manual_loop_next.rs). The ghost
//! counter `pos` is the number of items consumed; the two `remaining` clauses
//! are the wrapper's own `wf_inner`, written out. The `decreases` rule:
//! `remaining()` is prophetic and may not appear in `decreases`; a manual
//! loop measures the iterator's non-prophetic `IteratorSpec::decrease(&it)->0`,
//! and draws its conclusion before `break`, because the prophetic equality
//! does not survive the break. `it` is not in scope after a `for` loop.
//!
//! Custom iteration (a collection with no slice underneath) implements the
//! five `IteratorSpecImpl` spec fns by hand; see
//! prophetic_iterators_standard.rs. Wrapping another collection's iterator is
//! covered by wrapping_iterators_standard.rs.
//!
//! Note on iter_mut: vstd 0.2026.09.13 specifies `std::slice::IterMut`
//! (`vstd/std_specs/slice.rs`), so mutable iteration is now specifiable.
//! Modules that still carry an unspecified `iter_mut` or `into_iter_mut` keep
//! it as-is until a standard covers mutable iteration.
// 1. module
pub mod iterators_standard {

    use std::fmt::{Debug, Display, Formatter};

    // 2. imports
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    // 5. view impls

    impl<T> View for ExampleS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    // 8. traits

    pub trait ExampleTrait<T>: Sized + View<V = Seq<T>> {
        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends
                0 <= i < self.spec_len(),
        ;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;

        // Component 1: the borrowing entry point returns the std slice
        // iterator. The three postconditions are the constructor triple.
        fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self@,
                IteratorSpec::decrease(&it) is Some,
        ;
    }

    // 9. impls

    impl<T> ExampleTrait<T> for ExampleS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq@[i]
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

        fn iter(&self) -> (it: std::slice::Iter<'_, T>) {
            self.seq.iter()
        }
    }

    // 10. iterators

    // Component 2: IntoIterator for &Self. Enables `for x in &collection`.
    impl<'a, T> std::iter::IntoIterator for &'a ExampleS<T> {
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

    // Component 3 (optional): IntoIterator for Self. Yields owned T.
    impl<T> std::iter::IntoIterator for ExampleS<T> {
        type Item = T;

        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self.seq@,
                vstd::std_specs::vec::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
        {
            self.seq.into_iter()
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: Debug> Debug for ExampleS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ExampleS({:?})", self.seq)
        }
    }

    impl<T: Display> Display for ExampleS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }
} // pub mod iterators_standard
