//! Proof time test: Does ParaPair propagate ensures through move closures?
//! RESULT: FAILS - Closure ensures don't propagate through ParaPair with move

use vstd::prelude::*;
use crate::Types::Types::Pair;
use crate::ParaPair;

verus! {

/// Closure that returns 42
fn return_42() -> (r: u64)
    ensures r == 42
{
    42
}

/// Closure that returns 99  
fn return_99() -> (r: u64)
    ensures r == 99
{
    99
}

/// Test: Can we prove literal values after ParaPair?
fn test_parapair_literal_return() {
    let Pair(a, b) = ParaPair!(move || return_42(), move || return_99());
    
    // ParaPair.ensures says: f1.ensures((), result.0) && f2.ensures((), result.1)
    // So we SHOULD get: a == 42 && b == 99 ... but we don't.
    assert(a == 42);  // FAILS
    assert(b == 99);  // FAILS
}

} // verus!
