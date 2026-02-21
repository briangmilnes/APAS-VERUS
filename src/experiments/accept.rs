// Copyright (c) 2025 Brian G. Milnes
//! Experiment: assume vs accept — Veracity treatment
//!
//! Per veracity/docs/Accepted.md.
//! 1. assume() — Veracity: 2× assume_eq_clone_workaround (warning)
//! 2. accept() — Veracity: call sites not flagged; 1× admit() in accept body
//!
//! Belt: proof { accept(...); }. Suspenders: assert(...) by { accept(...); } — no proof block.
//!
//! Run: veracity-review-proof-holes -d src/experiments/accept.rs

use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use vstd::std_specs::cmp::PartialEqSpecImpl;

verus! {

/// Intentional proof hole — use instead of assume() for accepted workarounds.
pub proof fn accept(b: bool)
    ensures b,
{
    admit();
}

proof fn accept_propagates_like_assume() {
    let x: int = 5;
    accept(x == 5);
    assert(x == 5);
}

#[verifier::reject_recursive_types(T)]
pub struct AssumeBox<T> {
    pub data: Vec<T>,
}

impl<T> View for AssumeBox<T> {
    type V = Seq<T>;
    open spec fn view(&self) -> Seq<T> { self.data@ }
}

#[cfg(verus_keep_ghost)]
impl<T: PartialEq> PartialEqSpecImpl for AssumeBox<T> {
    open spec fn obeys_eq_spec() -> bool { true }
    open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
}

impl<T: Clone> Clone for AssumeBox<T> {
    fn clone(&self) -> (result: Self)
        ensures result@ == self@
    {
        let result = AssumeBox { data: self.data.clone() };
        proof { assume(result@ == self@); }
        result
    }
}

impl<T: PartialEq> core::cmp::PartialEq for AssumeBox<T> {
    fn eq(&self, other: &Self) -> (r: bool)
        ensures r == (self@ == other@)
    {
        let r = self.data == other.data;
        proof { assume(r == (self@ == other@)); }
        r
    }
}

impl<T: Eq> core::cmp::Eq for AssumeBox<T> {}

#[verifier::reject_recursive_types(T)]
pub struct AcceptBox<T> {
    pub data: Vec<T>,
}

impl<T> View for AcceptBox<T> {
    type V = Seq<T>;
    open spec fn view(&self) -> Seq<T> { self.data@ }
}

#[cfg(verus_keep_ghost)]
impl<T: PartialEq> PartialEqSpecImpl for AcceptBox<T> {
    open spec fn obeys_eq_spec() -> bool { true }
    open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
}

impl<T: Clone> Clone for AcceptBox<T> {
    fn clone(&self) -> (result: Self)
        ensures result@ == self@
    {
        let result = AcceptBox { data: self.data.clone() };
        proof { accept(result@ == self@); }  // belt
        result
    }
}

impl<T: PartialEq> core::cmp::PartialEq for AcceptBox<T> {
    fn eq(&self, other: &Self) -> (r: bool)
        ensures r == (self@ == other@)
    {
        let r = self.data == other.data;
        assert(r == (self@ == other@)) by {
            accept(r == (self@ == other@));  // suspenders: by-block, no proof { }
        }
        r
    }
}

impl<T: Eq> core::cmp::Eq for AcceptBox<T> {}

} // verus!
