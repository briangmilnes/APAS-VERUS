// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Test: can we add `requires` to `std::iter::Iterator::next` in Verus?
//!
//! RESULT: NO.  Verus error:
//!   "trait method implementation cannot declare requires clauses;
//!    these can only be inherited from the trait declaration"
//!
//! This confirms that the `assume(iter_invariant(self))` in hand-rolled iterators
//! (OrderedTableStEph, OrderedTableStPer, simple_seq_iter) is a necessary workaround.
//! The assume cannot be eliminated without either:
//!   (a) Verus adding requires support on external trait impls, or
//!   (b) wrapping a vstd-specified iterator (slice::Iter) instead of hand-rolling next().
//!
//! The standard iterator pattern (ArraySeqStEph) avoids this by delegating next() to
//! `self.inner.next()` where inner is a `std::slice::Iter` with vstd assume_specification.
//! Hand-rolled iterators that do manual pos/len indexing cannot use that delegation pattern
//! and are stuck with the assume.
//!
//! Test 1 (FAILS): requires on Iterator::next — Verus rejects it.
//! Test 2 (VERIFIES): assume workaround — the pattern used in production code.
//! Test 3 (VERIFIES): free function with requires — proves the invariant round-trips,
//!   confirming the assume is sound (just not provable through the trait interface).

pub mod iter_requires_on_external_trait {
    use vstd::prelude::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct MyIter<T> {
        pub data: Vec<T>,
        pub pos: usize,
    }

    impl<T> View for MyIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.data@)
        }
    }

    pub open spec fn iter_invariant<T>(it: &MyIter<T>) -> bool {
        it.pos <= it.data@.len()
    }

    // Test 1: FAILS — requires on external trait impl is rejected by Verus.
    // Uncomment to see the error.
    /*
    impl<T: Clone> Iterator for MyIter<T> {
        type Item = T;
        fn next(&mut self) -> (result: Option<T>)
            requires iter_invariant(&old(self)),
            ensures iter_invariant(self),
        {
            if self.pos < self.data.len() {
                let elem = self.data[self.pos].clone();
                self.pos = self.pos + 1;
                Some(elem)
            } else {
                None
            }
        }
    }
    */

    // Test 2: assume workaround — the production pattern.
    impl<T: Clone> Iterator for MyIter<T> {
        type Item = T;

        fn next(&mut self) -> (result: Option<T>)
            ensures
                iter_invariant(self),
                ({
                    let (old_index, old_seq) = old(self)@;
                    match result {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index >= old_seq.len()
                        },
                        Some(ref element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                        },
                    }
                }),
        {
            assume(iter_invariant(self));
            if self.pos < self.data.len() {
                let elem = self.data[self.pos].clone();
                self.pos = self.pos + 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    // Test 3: free function with requires — proves the invariant round-trips.
    fn assumption_free_next<T: Clone>(it: &mut MyIter<T>) -> (result: Option<T>)
        requires
            iter_invariant(&old(it)),
        ensures
            iter_invariant(it),
            ({
                let (old_index, old_seq) = old(it)@;
                match result {
                    None => {
                        &&& it@ == old(it)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(ref element) => {
                        let (new_index, new_seq) = it@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                    },
                }
            }),
    {
        if it.pos < it.data.len() {
            let elem = it.data[it.pos].clone();
            it.pos = it.pos + 1;
            Some(elem)
        } else {
            None
        }
    }

    // Test 4: caller using the assume-based Iterator impl.
    fn test_manual_loop() {
        let mut it = MyIter::<u64> { data: vec![1u64, 2, 3], pos: 0 };
        loop
            invariant
                iter_invariant(&it),
                it.data@ == seq![1u64, 2, 3],
            decreases it.data@.len() - it.pos,
        {
            match it.next() {
                Some(_val) => {},
                None => { break; },
            }
        }
    }

    } // verus!
}
