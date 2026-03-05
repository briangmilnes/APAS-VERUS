//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Using Closures Standard: how to pass closures with specs in APAS-VERUS.
//!
//! Verus closures can carry `requires` and `ensures` clauses. The receiver
//! function uses `f.requires((...))` and `f.ensures((...), result)` to
//! propagate those specs through its own contract.
//!
//! This file shows three patterns:
//! - Pattern A: Let-bind a closure with ensures, pass by reference.
//! - Pattern B: Inline closure with requires+ensures inside a call.
//! - Pattern C: Ghost spec_fn companion for spec-level reasoning.
//!
//! References:
//! - src/Chap18/ArraySeqStEph.rs (tabulate)
//! - src/Chap18/ArraySeqStPer.rs (filter with Ghost spec_fn)
//! - src/Chap21/Algorithm21_5.rs (let-bind pred, ghost spec_pred)
// 1. module
pub mod using_closures_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

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

    /// Demonstrates higher-order functions that accept closures with specs.
    pub trait ExampleTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends
                0 <= i < self.spec_len(),
        ;

        /// Pattern A/B: Build a sequence by applying `f` to each index.
        ///
        /// The receiver propagates closure specs via `f.requires` and `f.ensures`.
        /// Callers let-bind a closure with ensures (Pattern A) or pass an inline
        /// closure with requires+ensures (Pattern B).
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (s: Self)
            requires
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                s.spec_len() == length as nat,
                forall|i: int| #![trigger s.spec_index(i)]
                    0 <= i < length ==> f.ensures((i as usize,), s.spec_index(i)),
        ;

        /// Apply a function to each element, producing a new sequence.
        fn map_apply<F: Fn(&T) -> T>(&self, f: &F) -> (mapped: Self)
            requires
                forall|i: int| 0 <= i < self.spec_len()
                    ==> #[trigger] f.requires((&self.spec_index(i),)),
            ensures
                mapped.spec_len() == self.spec_len(),
                forall|i: int| #![trigger mapped.spec_index(i)]
                    0 <= i < self.spec_len()
                    ==> f.ensures((&self.spec_index(i),), mapped.spec_index(i)),
        ;

        /// Pattern C: Filter with a ghost spec_fn companion.
        ///
        /// The exec closure `pred` decides which elements to keep.
        /// The ghost `spec_pred` mirrors it at spec level so ensures can
        /// reason about which elements survive.
        fn filter<F: Fn(&T) -> bool>(
            &self,
            pred: &F,
            Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: Self) where T: Copy
            requires
                forall|x: &T| #[trigger] pred.requires((x,)),
                forall|x: T, keep: bool|
                    pred.ensures((&x,), keep) ==> keep == spec_pred(x),
            ensures
                filtered.spec_len() <= self.spec_len(),
        ;
    }

    // 9. impls

    impl<T> ExampleTrait<T> for ExampleS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self@[i]
        }

        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (s: Self) {
            let mut v: Vec<T> = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    v@.len() == i as int,
                    forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                    forall|j: int| #![trigger v@[j]]
                        0 <= j < i ==> f.ensures((j as usize,), v@[j]),
                decreases length - i,
            {
                v.push(f(i));
                i += 1;
            }
            ExampleS { seq: v }
        }

        fn map_apply<F: Fn(&T) -> T>(&self, f: &F) -> (mapped: Self) {
            let len = self.seq.len();
            let mut v: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    len == self@.len(),
                    i <= len,
                    v@.len() == i as int,
                    forall|j: int| 0 <= j < self.spec_len()
                        ==> #[trigger] f.requires((&self.spec_index(j),)),
                    forall|j: int| #![trigger v@[j]]
                        0 <= j < i
                        ==> f.ensures((&self.spec_index(j),), v@[j]),
                decreases len - i,
            {
                assert(f.requires((&self.spec_index(i as int),)));
                v.push(f(&self.seq[i]));
                i += 1;
            }
            ExampleS { seq: v }
        }

        fn filter<F: Fn(&T) -> bool>(
            &self,
            pred: &F,
            Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: Self) where T: Copy {
            let len = self.seq.len();
            let mut v: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    len == self.seq@.len(),
                    i <= len,
                    v@.len() <= i as int,
                    forall|x: &T| #[trigger] pred.requires((x,)),
                decreases len - i,
            {
                if pred(&self.seq[i]) {
                    v.push(self.seq[i]);
                }
                i += 1;
            }
            ExampleS { seq: v }
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
} // pub mod using_closures_standard
