// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Iterative vs Recursive Standard: naming when both algorithm variants coexist.
//!
//! APAS presents most algorithms in a specific style — usually recursive (divide-and-conquer).
//! When our implementation uses a different style, both variants coexist in the same file,
//! same trait, same impl.
//!
//! Naming rule:
//! - The default name matches the textbook. If APAS says recursive, `fn find` is recursive.
//! - The alternative gets a suffix: `_iter` if iterative, `_rec` if recursive.
//! - If APAS says iterative (hash table probing, merge in merge sort), the default name
//!   is iterative and a recursive alternative (if one existed) would be `_rec`.
//!
//! One-directional rule:
//! - We only add a suffixed alternative when we HAVE one. If APAS says recursive and we
//!   have a recursive implementation, we're done. No obligation to write `_iter`.
//! - If APAS says recursive and we only have iterative, the iterative body gets `_iter`
//!   and the default name is reserved for the future recursive implementation. Until that
//!   recursive body exists, the default delegates to `_iter`.
//!
//! Both variants live in the trait:
//! - Same requires, same ensures, same spec function. The abstract behavior is identical.
//! - The trait is the reading surface. Putting both variants there lets the reader see
//!   what the module offers and compare the contracts side by side.
//! - Callers use the trait. Mt/Aug/delegation files dispatch through the trait.
//!   Putting `_iter` outside the trait would hide it from the delegation chain.
//!
//! No extra files:
//! - Both variants live in the same file, same trait, same impl block. No separate
//!   files for iterative vs recursive implementations.
//!
//! Phase 1 (rename only, no new proofs):
//! - Existing iterative body becomes `fn find_iter(...)` with the same ensures.
//! - Trait gets `fn find_iter(...)` with the same spec as `fn find(...)`.
//! - `fn find(...)` in the impl delegates: `self.find_iter(x)`.
//! - Callers are unaffected — they call the trait method `find`.
//! - Validate after each file.
//!
//! Phase 2 (write recursive implementations):
//! - Write recursive body under the default name: `fn find(...)`.
//! - The impl's `fn find` now calls the recursive logic directly.
//! - `fn find_iter` stays as the iterative alternative.
//! - Both are proven against the same spec function.
//!
//! What NOT to rename:
//! - Functions that match the textbook (both style and complexity). No suffix needed.
//! - Functions classified MATCH-DIFF-ALG (same complexity, different mechanism).
//!   Low priority; the algorithm is correct enough.
//! - Delegation functions (Mt wraps St, Aug wraps base). The fix propagates from the
//!   backing store. Don't rename at the delegation layer.
//! - Wrappers like `join` that delegate to `union`. When `union` becomes recursive,
//!   `join` inherits the fix automatically.
//!
//! Doc comments:
//! - The default (textbook) variant gets the APAS cost spec comment.
//! - The suffixed variant gets a one-line comment: `/// Iterative alternative to `find`.`
//!   or `/// Recursive alternative to `insert`.`
//!
//! RTT:
//! - Both variants need runtime tests. The default is tested through the trait.
//!   The `_iter` variant is tested by calling `_iter` directly.
//!
//! Reference implementation: `src/experiments/trait_rec_vs_iter.rs`.
//! Inventory of affected functions: `plans/iterative-vs-recursive-inventory-v2.md`.

pub mod iterative_vs_recursive_standard {

    use vstd::prelude::*;

    verus! {

    // Shared spec: both variants prove the same postcondition.
    pub open spec fn spec_sum(s: Seq<u64>, n: int) -> int
        decreases n,
    {
        if n <= 0 { 0 }
        else { spec_sum(s, n - 1) + s[n - 1] as int }
    }

    pub proof fn lemma_spec_sum_monotone(s: Seq<u64>, i: int, j: int)
        requires 0 <= i <= j, j <= s.len(),
        ensures spec_sum(s, i) <= spec_sum(s, j),
        decreases j - i,
    {
        if i < j { lemma_spec_sum_monotone(s, i, j - 1); }
    }

    pub struct Numbers {
        pub elements: Vec<u64>,
    }

    impl View for Numbers {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
    }

    // Recursive helper — private, called by the trait's default impl.
    fn rec_sum_helper(elements: &Vec<u64>, idx: usize) -> (total: u64)
        requires
            idx <= elements@.len(),
            elements@.len() <= u64::MAX,
            spec_sum(elements@, idx as int) <= u64::MAX,
        ensures total == spec_sum(elements@, idx as int),
        decreases idx,
    {
        if idx == 0 {
            0
        } else {
            let rest = rec_sum_helper(elements, idx - 1);
            rest + elements[idx - 1]
        }
    }

    // Both variants in the trait. Same spec. Reader sees them together.
    pub trait NumbersTrait: Sized + View<V = Seq<u64>> {
        spec fn spec_numbers_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s@ == Seq::<u64>::empty(), s.spec_numbers_wf();

        fn push(&mut self, val: u64)
            requires old(self).spec_numbers_wf(), old(self)@.len() < usize::MAX,
            ensures self@ == old(self)@.push(val), self.spec_numbers_wf();

        /// Sum — recursive (textbook default).
        /// - APAS: Work Θ(n), Span Θ(n)
        fn sum(&self) -> (total: u64)
            requires
                self.spec_numbers_wf(),
                self@.len() <= u64::MAX,
                spec_sum(self@, self@.len() as int) <= u64::MAX,
            ensures total == spec_sum(self@, self@.len() as int);

        /// Iterative alternative to `sum`.
        fn sum_iter(&self) -> (total: u64)
            requires
                self.spec_numbers_wf(),
                self@.len() <= u64::MAX,
                spec_sum(self@, self@.len() as int) <= u64::MAX,
            ensures total == spec_sum(self@, self@.len() as int);
    }

    impl NumbersTrait for Numbers {
        open spec fn spec_numbers_wf(&self) -> bool { true }

        fn new() -> (s: Self) {
            Numbers { elements: Vec::new() }
        }

        fn push(&mut self, val: u64) {
            self.elements.push(val);
        }

        // Default name = textbook style (recursive).
        fn sum(&self) -> (total: u64) {
            rec_sum_helper(&self.elements, self.elements.len())
        }

        // Suffixed name = alternative style (iterative).
        fn sum_iter(&self) -> (total: u64) {
            let mut total: u64 = 0;
            let mut i: usize = 0;
            while i < self.elements.len()
                invariant
                    0 <= i <= self.elements@.len(),
                    total == spec_sum(self.elements@, i as int),
                    spec_sum(self.elements@, i as int) <= u64::MAX,
                    spec_sum(self.elements@, self.elements@.len() as int) <= u64::MAX,
                    self.elements@.len() <= u64::MAX,
                decreases self.elements@.len() - i,
            {
                proof {
                    lemma_spec_sum_monotone(self.elements@, i + 1, self.elements@.len() as int);
                }
                total = total + self.elements[i];
                i = i + 1;
            }
            total
        }
    }

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for Numbers {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Numbers(len={})", self.elements.len())
        }
    }
    impl std::fmt::Display for Numbers {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Numbers(len={})", self.elements.len())
        }
    }
} // pub mod
