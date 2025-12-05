//! Axioms connecting Clone and View
//!
//! For types that implement both Clone and View, we need axioms that
//! connect the executable clone() operation to the spec-level view.
//!
//! NOTE: vstd provides `cloned(a, b)` predicate but does NOT provide
//! the bridge to View: `cloned(a, b) ==> a@ == b@`. This axiom fills that gap.

pub mod clone_view {
    use vstd::prelude::*;

    verus! {

    /// Axiom: Cloning preserves the view for types with Clone + View
    /// 
    /// For any value x of type T where T: Clone + View,
    /// if y is the result of cloning x, then x@ == y@
    ///
    /// This axiom is fundamental because:
    /// - Clone creates a new instance with the same logical value
    /// - View maps values to their spec-level representation
    /// - The spec-level representation should be preserved across cloning
    ///
    /// Usage in proofs:
    /// ```
    /// let y = x.clone();
    /// proof {
    ///     axiom_clone_preserves_view(x, y);
    ///     assert(x@ == y@);
    /// }
    /// ```
    // UNUSED: only used in experiments (commented out)
    #[verifier::external_body]
    pub broadcast proof fn lemma_clone_preserves_view<T: Clone + View>(x: &T, y: &T)
        requires
            vstd::pervasive::cloned(*x, *y),
        ensures
            #[trigger] x@ == #[trigger] y@,
    {
    }

    // UNUSED: only used in experiments (commented out)
    pub broadcast group group_clone_view_axioms {
        lemma_clone_preserves_view,
    }

    } // verus!
}

