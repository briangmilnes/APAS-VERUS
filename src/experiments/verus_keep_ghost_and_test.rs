//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! # Experiment: `#[cfg(verus_keep_ghost)]` and `cargo test` compatibility
//!
//! ## Hypothesis
//!
//! A module inside `verus! { }` can compile under both `verus` (verification) and
//! `cargo test` (runtime testing) if ghost-only imports are guarded with
//! `#[cfg(verus_keep_ghost)]`. The `verus!` macro erases ghost/proof code
//! (requires, ensures, proof blocks, spec fns, Ghost parameters), keeping exec
//! code intact. The `#[cfg(verus_keep_ghost)]` guard removes imports that only
//! ghost code references, preventing "unresolved import" errors under `cargo test`.
//!
//! ## What needs guarding
//!
//! 1. **Imports** of vstd modules gated by `verus_keep_ghost` in vstd itself
//!    (e.g., `vstd::std_specs::vec`, `vstd::laws_eq`).
//! 2. **Impl blocks** for ghost-only traits (e.g., `impl PartialEqSpecImpl`).
//! 3. **Attributes** from the verifier (e.g., `#[verifier::loop_isolation(false)]`)
//!    -- use `#[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]`.
//!
//! The `verus!` macro does NOT erase `use` statements or `impl` blocks for
//! external traits, so those must be guarded manually.
//!
//! ## Result
//!
//! **Verus verification**: 1221 verified, 0 errors. This module verifies cleanly.
//!
//! **Cargo test**: This module is correctly structured for `cargo test`.
//! However, `cargo test` currently fails because OTHER modules in the codebase
//! have unguarded ghost-only imports (e.g., `use vstd::laws_eq::*` in ArraySeq.rs,
//! `use ...::{spec_iterate, spec_monoid}` in LinkedList/Mt files, unguarded
//! `impl PartialEqSpecImpl` blocks in ~15 files, and bare
//! `#[verifier::loop_isolation(false)]` attributes). The test file
//! `tests/experiments/TestVerusKeepGhostAndTest.rs` exercises the exec code.
//!
//! ## Conclusion
//!
//! The pattern works at the module level. To make `cargo test` work project-wide,
//! every module needs the same discipline applied. See `docs/VerusIfyAPASRust.md`
//! sections 1, 7, 14, and 16 for the full set of patterns.

pub mod verus_keep_ghost_and_test {

    use vstd::prelude::*;

    verus! {

    // This import is only needed for ghost specs (Vec::@ view axioms).
    // Under cargo test, verus_keep_ghost is NOT set, so this line is removed.
    // The exec code still compiles because verus! erases all ghost code that
    // references these symbols.
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::vec::*;

    /// A simple struct wrapping a Vec, with ghost spec.
    pub struct Bag<T> {
        pub elts: Vec<T>,
    }

    impl<T> View for Bag<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elts@ }
    }

    impl<T> Bag<T> {
        /// Create an empty bag.
        pub fn empty() -> (b: Bag<T>)
            ensures b@.len() == 0
        {
            Bag { elts: Vec::new() }
        }

        /// Push an item, with a spec that the new view ends with the item.
        pub fn push(&mut self, item: T)
            ensures
                self@.len() == old(self)@.len() + 1,
                self@.last() == item,
        {
            self.elts.push(item);
        }

        /// Return the number of elements.
        pub fn len(&self) -> (n: usize)
            ensures n == self@.len()
        {
            self.elts.len()
        }

        /// Get a reference to the i-th element.
        pub fn nth(&self, i: usize) -> (r: &T)
            requires i < self@.len()
            ensures *r == self@[i as int]
        {
            &self.elts[i]
        }

        /// Sum all elements (for i32 bags). The loop has ghost invariants
        /// that are erased under cargo test, leaving a plain while loop.
        pub fn sum(bag: &Bag<i32>) -> (total: i32)
            requires
                bag@.len() <= 1000,
                forall|i: int| 0 <= i < bag@.len() ==> -1000 <= #[trigger] bag@[i] <= 1000,
        {
            let mut total: i32 = 0;
            let mut i: usize = 0;
            while i < bag.elts.len()
                invariant
                    0 <= i <= bag@.len(),
                    bag@.len() <= 1000,
                    forall|j: int| 0 <= j < bag@.len() ==> -1000 <= #[trigger] bag@[j] <= 1000,
                    -1000 * (i as int) <= total <= 1000 * (i as int),
                decreases bag@.len() - i,
            {
                total = total + bag.elts[i];
                i += 1;
            }
            total
        }

        /// A proof function that only exists in ghost mode.
        /// Under cargo test, verus! erases this entirely.
        proof fn lemma_push_increases_len(pre: Seq<T>, post: Seq<T>, item: T)
            requires
                post.len() == pre.len() + 1,
                post.last() == item,
            ensures
                post.len() > pre.len(),
        {}
    }

    } // verus!
}
