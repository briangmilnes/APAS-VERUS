//! Proof time test: Named closures with explicit specs (Fibonacci pattern)
//! RESULT: PASSES - Named closures with explicit ensures propagate through ParaPair

use vstd::prelude::*;
use crate::Types::Types::Pair;
use crate::ParaPair;

verus! {

/// Test: Does the Fibonacci pattern (named closures with specs) work?
fn test_parapair_named_closure() {
    // Fibonacci pattern: named closure with explicit ensures
    let f1 = move || -> (r: u64)
        ensures r == 42
    { 42 };
    
    let f2 = move || -> (r: u64)
        ensures r == 99
    { 99 };
    
    let Pair(a, b) = ParaPair!(f1, f2);
    
    // Does this prove with named closures?
    assert(a == 42);
    assert(b == 99);
}

} // verus!
