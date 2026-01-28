//! Proof time test: Does ParaPair propagate ensures through non-move closures?
//! RESULT: FAILS - Same as move version, closure ensures don't propagate

use vstd::prelude::*;
use crate::Types::Types::Pair;
use crate::ParaPair;

verus! {

fn return_42() -> (r: u64)
    ensures r == 42
{
    42
}

fn return_99() -> (r: u64)
    ensures r == 99
{
    99
}

/// Test: Without move, do ensures propagate?
fn test_parapair_closure_ensures() {
    let Pair(a, b) = ParaPair!(|| return_42(), || return_99());
    
    assert(a == 42);  // FAILS - same as move version
    assert(b == 99);  // FAILS
}

} // verus!
