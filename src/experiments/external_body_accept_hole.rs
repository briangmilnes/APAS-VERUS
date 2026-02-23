// Copyright (c) 2025 Brian G. Milnes
//! Hypothesis: Does Veracity use `// accept hole` comments after #[verifier::external_body]?
//!
//! Per vstdplus/accept.rs: we cannot write a macro for external_body, so Veracity
//! will use `// accept hole` comments after the attribute to indicate accepted holes.
//!
//! RESULT: Verus verifies. Veracity still reports 1× external_body hole — it does
//! not yet recognize `// accept hole` to downgrade. The vstdplus comment is aspirational.
//!
//! Run: veracity-review-proof-holes -d src/experiments/external_body_accept_hole.rs

use vstd::prelude::*;

verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

#[verifier::reject_recursive_types(T)]
pub struct ExternalBodyAcceptHoleBox<T> {
    pub data: Vec<T>,
}

impl<T> View for ExternalBodyAcceptHoleBox<T> {
    type V = Seq<T>;
    open spec fn view(&self) -> Seq<T> { self.data@ }
}

impl<T: Clone> Clone for ExternalBodyAcceptHoleBox<T> {
    #[verifier::external_body]  // accept hole
    fn clone(&self) -> (result: Self)
        ensures result@ == self@
    {
        ExternalBodyAcceptHoleBox { data: self.data.clone() }
    }
}

} // verus!
