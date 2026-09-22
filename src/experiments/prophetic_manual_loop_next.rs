// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r211): a manual `loop { match it.next() { .. } }` over
//! `std::slice::Iter` and `std::vec::IntoIter` on the iterator's own `next()`,
//! with no `VerusForLoopWrapper`.
//!
//! Question. `VerusForLoopWrapper` lives in `vstd::std_specs::iter`, and vstd
//! declares `#[cfg(verus_keep_ghost)] pub mod std_specs;` (vstd/vstd.rs:96),
//! so a manual loop written on the wrapper does not compile under `cargo`
//! (`logs/rtt.20260921-112745.log`, 119 `cannot find type VerusForLoopWrapper`).
//! Which manual-loop form verifies on the iterator's own `next()`, whose
//! contract is the `IteratorSpec` one (vstd/std_specs/iter.rs:35)?
//!
//! Form. A ghost counter `pos` of consumed items and two invariants over the
//! prophetic `IteratorSpec::remaining(&it)`, index-wise (the wrapper's own
//! `wf_inner`, vstd/std_specs/iter.rs:848):
//!     IteratorSpec::obeys_prophetic_iter_laws(&it),
//!     IteratorSpec::decrease(&it) is Some,
//!     0 <= pos <= orig.len(),
//!     IteratorSpec::remaining(&it).len() == orig.len() - pos,
//!     forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
//!         ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
//! with `decreases IteratorSpec::decrease(&it)->0` (the non-prophetic metric;
//! `remaining()` is prophetic and may not appear in `decreases`). `Some(x)`
//! advances `pos` in a proof block; `None` pins `pos == orig.len()`.
//!
//! Also exercised: the `skip` form of the fourth and fifth invariants as one
//! clause, `IteratorSpec::remaining(&it).unref() == orig.skip(pos)`; an early
//! `return` from the `Some` arm (the `all_nonempty` shape in
//! src/Chap05/SetStEph.rs); the consuming `std::vec::IntoIter`; and the
//! `for x in it: coll.iter()` form, which the `verus!` macro desugars in both
//! build modes.
//!
//! RESULT: SUCCEEDS — 10 verified, 0 errors
//! DATE: 2026-09-21
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-prophetic_manual_loop_next.20260921-145935.log
//! Finding: the index-wise form verifies with no seq lemma and no `=~=`; a
//! `return` inside the (isolated) loop needs `orig == a.seq@` as an invariant.

pub mod prophetic_manual_loop_next {

    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    impl<T> ExampleS<T> {
        pub fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self.seq@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
        {
            self.seq.iter()
        }
    }

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

    // Form 1: the index-wise invariants (the wrapper's wf_inner, written out).
    fn test_loop_borrow_next(a: &ExampleS<u64>) {
        let ghost orig: Seq<u64> = a.seq@;
        let mut collected: Vec<u64> = Vec::new();
        let mut it = a.iter();
        let ghost mut pos: int = 0;
        loop
            invariant
                IteratorSpec::obeys_prophetic_iter_laws(&it),
                IteratorSpec::decrease(&it) is Some,
                0 <= pos <= orig.len(),
                IteratorSpec::remaining(&it).len() == orig.len() - pos,
                forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                    ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
                collected.len() == pos,
                forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == orig[i],
            decreases IteratorSpec::decrease(&it)->0,
        {
            let ghost old_pos = pos;
            match it.next() {
                Some(x) => {
                    proof {
                        pos = pos + 1;
                        assert(orig[old_pos] == *x);
                    }
                    collected.push(*x);
                },
                None => {
                    assert(pos == orig.len());
                    assert(collected@ =~= orig);
                    break;
                },
            }
        }
    }

    // Form 2 (the `skip` clause in place of the two index-wise invariants) is
    // the separate experiment prophetic_manual_loop_next_skip.rs: FAILS.

    // Form 3: early `return` from the `Some` arm (the all_nonempty shape). The
    // loop is isolated by default, so the postcondition proved at a `return`
    // inside it sees only the invariants: `orig == a.seq@` is one of them.
    fn test_loop_borrow_next_return(a: &ExampleS<u64>) -> (all_nonzero: bool)
        ensures
            all_nonzero == forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] a.seq@[i] != 0,
    {
        let ghost orig: Seq<u64> = a.seq@;
        let mut it = a.iter();
        let ghost mut pos: int = 0;
        loop
            invariant
                orig == a.seq@,
                IteratorSpec::obeys_prophetic_iter_laws(&it),
                IteratorSpec::decrease(&it) is Some,
                0 <= pos <= orig.len(),
                IteratorSpec::remaining(&it).len() == orig.len() - pos,
                forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                    ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
                forall|i: int| 0 <= i < pos ==> #[trigger] orig[i] != 0,
            decreases IteratorSpec::decrease(&it)->0,
        {
            let ghost old_pos = pos;
            match it.next() {
                Some(x) => {
                    proof {
                        pos = pos + 1;
                        assert(orig[old_pos] == *x);
                    }
                    if *x == 0 {
                        return false;
                    }
                },
                None => {
                    return true;
                },
            }
        }
    }

    // Form 4: the consuming iterator, owned items (no deref).
    fn test_loop_consume_next(a: ExampleS<u64>) {
        let ghost orig: Seq<u64> = a.seq@;
        let mut collected: Vec<u64> = Vec::new();
        let mut it = a.into_iter();
        let ghost mut pos: int = 0;
        loop
            invariant
                IteratorSpec::obeys_prophetic_iter_laws(&it),
                IteratorSpec::decrease(&it) is Some,
                0 <= pos <= orig.len(),
                IteratorSpec::remaining(&it).len() == orig.len() - pos,
                forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                    ==> #[trigger] IteratorSpec::remaining(&it)[i] == orig[pos + i],
                collected.len() == pos,
                forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == orig[i],
            decreases IteratorSpec::decrease(&it)->0,
        {
            let ghost old_pos = pos;
            match it.next() {
                Some(x) => {
                    proof {
                        pos = pos + 1;
                        assert(orig[old_pos] == x);
                    }
                    collected.push(x);
                },
                None => {
                    assert(pos == orig.len());
                    assert(collected@ =~= orig);
                    break;
                },
            }
        }
    }

    // Form 5: `for x in it: coll.iter()`, which the verus! macro desugars in
    // both build modes.
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

    } // verus!
} // pub mod prophetic_manual_loop_next
