//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! PartialEq, Eq, and Clone Standard for APAS-VERUS modules.
//!
//! These derive-style impls live in section 12 (derive impls in verus!) inside verus!.
//! Each has a specific accept pattern for bridging Verus limitations.
//!
//! Why accepts are necessary:
//! - Verus cannot verify through the `==` operator on inner containers (Vec, HashSet,
//!   HashMap) because their PartialEq impls are external_body in vstd.
//! - Verus cannot verify that Vec::clone() preserves the view because Vec::clone()
//!   is external_body in vstd.
//! - These are Verus/vstd limitations, not algorithmic gaps. The Rust standard library
//!   guarantees these properties.
//!
//! Accept categories:
//! 1. PartialEq: `accept(equal == (self@ == other@))` — delegates to inner container ==.
//! 2. Clone view: `accept(cloned@ == self@)` — delegates to inner container clone.
//! 3. Clone structural: `accept(cloned == *self)` — for types without View.
//! 4. Clone feq: `accept(obeys_feq_clone::<T>())` — clone preserves function equality.
//!
//! These accepts are safe because:
//! - PartialEq on Vec/HashSet is implemented by Rust's standard library. If the
//!   element type's PartialEq is correct, the container's PartialEq is correct.
//! - Clone on Vec/HashSet produces a value with identical contents. If the element
//!   type's Clone is correct, the container's Clone preserves the view.
//! - PartialEqSpecImpl connects the spec-level eq_spec to the exec-level == operator.
//!   Verus trusts this bridge when obeys_eq_spec() returns true.
//!
//! Eq: marker trait, no body, no assumes. Just `impl<T: Eq + View> Eq for MyType<T> {}`.
//!
//! CRITICAL RULE: assume() and accept() for Clone, PartialEq, and Eq bridges

//! They must NEVER appear in algorithmic code (trait methods, helper functions,
//! proof functions, etc.). If algorithmic code needs these properties, it must
//! obtain them through the ensures clauses of clone() and eq(), not by assuming
//! or accepting them directly.
//!
//! 5. Propagating clone assumptions through requires (feq broadcast pattern).
//!
//!    When algorithmic code clones generic `T` values (e.g., `Arc<Node<T>>` in tree
//!    rotations), each clone site needs `obeys_feq_clone::<T>()` to prove the cloned
//!    value equals the original.  Rather than `assume(obeys_feq_clone::<T>())` at
//!    every clone, add `obeys_feq_clone::<T>()` to the function's `requires` and
//!    assume it once at the top-level entry point.
//!
//!    ```
//!    // Internal helper: requires propagates the assumption.
//!    fn rotate_right(y: Arc<Node<T>>) -> (out: Arc<Node<T>>)
//!        requires spec_wf(Some(y)), obeys_feq_clone::<T>(),
//!        ensures  spec_wf(Some(out)),
//!    {
//!        let x = y.left.as_ref().unwrap().clone();  // clone is safe: feq in scope
//!        // ... no per-clone assume needed
//!    }
//!
//!    // Top-level entry point: single assume.
//!    fn insert(&mut self, value: T) {
//!        proof { assume(obeys_feq_clone::<T>()); }  // ONE assume for entire call tree
//!        self.root = insert_at(self.root.clone(), value);
//!    }
//!    ```
//!
//!    This replaces N per-clone assumes with 1 entry-point assume. The `group_feq_axioms`
//!    broadcast group (`axiom_cloned_implies_eq`, `axiom_cloned_implies_eq_owned`) fires
//!    automatically when `obeys_feq_clone::<T>()` is in scope, so clone results satisfy
//!    `x == y` without explicit lemma calls.
//!
//!    Import pattern:
//!    ```
//!    use crate::vstdplus::feq::feq::feq;
//!    #[cfg(verus_keep_ghost)]
//!    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full};
//!    ```
//!    And in broadcast use (section 3):
//!    ```
//!    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
//!    ```
//!
//! Section ordering within section 12:
//! 1. Clone
//! 2. PartialEq
//! 3. Eq
//!
//! PartialEqSpecImpl goes in section 9 (impls) because the style checker treats
//! cfg-gated trait impls as regular impls, which must precede section 10.

pub mod partial_eq_eq_clone_standard {

    use vstd::prelude::*;
    use crate::vstdplus::accept::accept;

    verus! {

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    #[verifier::reject_recursive_types(T)]
    pub struct Collection<T> {
        pub elements: Vec<T>,
    }

    impl<T> View for Collection<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements@ }
    }

    //      9. impls (PartialEqSpecImpl goes here)

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for Collection<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    //      12. derive impls in verus!

    // Clone: accept that the inner container clone preserves the view.
    impl<T: Clone> Clone for Collection<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = Collection { elements: self.elements.clone() };
            proof { accept(cloned@ == self@); }  // accept hole: Vec::clone external_body
            cloned
        }
    }

    // PartialEq: accept that the inner container == reflects view equality.
    impl<T: PartialEq + View> PartialEq for Collection<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@),
        {
            let equal = self.elements == other.elements;
            proof { accept(equal == (self@ == other@)); }  // accept hole: Vec::eq external_body
            equal
        }
    }

    // Eq: marker trait, no body, no accept.
    impl<T: Eq + View> Eq for Collection<T> {}

    } // verus!
} // pub mod
