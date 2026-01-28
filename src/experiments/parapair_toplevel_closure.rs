//! Proof time test: Top-level closures with explicit specs
//! RESULT: Validates.

use vstd::prelude::*;
use crate::Types::Types::Pair;
use crate::ParaPair;

verus! {

fn f42 () -> (r: u64)
   ensures r == 42
{ 42 }
    
fn f99 () -> (r: u64)
   ensures r == 99
{ 99 }

fn test_parapair_toplevel_closure() {
    let Pair(a, b) = ParaPair!(f42, f99);
    
    assert(a == 42);
    assert(b == 99);
}

} // verus!
