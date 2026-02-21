// Copyright (c) 2025 Brian G. Milnes
//! Experiment: accepted_external_body declarative macro — per veracity/docs/Accepted.md
//!
//! Hypothesis: A declarative macro_rules! can expand to #[verifier::external_body] and attach
//! to function items, providing a concise "accepted" wrapper for external_body functions.
//!
//! Result: FAILS — declarative macro_rules! cannot produce attributes that attach to items.
//! Attempt 1: macro! {} expands to #[verifier::external_body] — "expected item after attributes".
//! Attempt 2: structured macro (capture+reemit fn) — expansion `(result: Self)` rejected at `:`.
//! Use proc-macro instead.
//!
//! Run: veracity-review-proof-holes -d src/experiments/accept_external_body.rs

use vstd::prelude::*;

/// Attempt 1: {} no-args, expands to attribute only.
macro_rules! accepted_external_body_attr {
    {} => {
        #[verifier::external_body]
    };
}

macro_rules! accepted_external_body {
    (
        fn $name:ident ( $($arg:tt)* ) -> ( $ret:ident : $ret_ty:ty )
        ensures $($rest:tt)*
    ) => {
        #[verifier::external_body]
        fn $name ( $($arg)* ) -> ( $ret : $ret_ty )
            ensures $($rest)*
    };
}

verus! {

#[verifier::reject_recursive_types(T)]
pub struct AcceptExternalBodyBox<T> {
    pub data: Vec<T>,
}

impl<T> View for AcceptExternalBodyBox<T> {
    type V = Seq<T>;
    open spec fn view(&self) -> Seq<T> { self.data@ }
}

impl<T: Clone> Clone for AcceptExternalBodyBox<T> {
    accepted_external_body_attr! {}
    fn clone(&self) -> (result: Self)
        ensures result@ == self@
    {
        AcceptExternalBodyBox { data: self.data.clone() }
    }
}

/// Ensures from external_body clone are propagated to callers.
fn clone_ensures_propagates(x: &AcceptExternalBodyBox<int>) -> (y: AcceptExternalBodyBox<int>)
    ensures y@ == x@
{
    let y = x.clone();
    y
}

} // verus!
