// Copyright (c) 2025 Brian G. Milnes
//! TCB Foul Experiment
//!
//! Demonstrates how the Trusted Computing Base (TCB) can be compromised.
//!
//! ## Attempt 1: Call unspecified Rust std methods (BLOCKED)
//!
//! We tried calling Vec::reverse(), Vec::sort() etc. which have no vstd specs.
//! Verus BLOCKS this with: "complex arguments to &mut parameters are currently unsupported"
//! 
//! This error comes from:
//!   - rust_verify/src/rust_to_vir_expr.rs:3725
//!   - vir/src/well_formed.rs:480
//!
//! The check in well_formed.rs ensures &mut args are "simple" (variables, fields).
//! Methods without specs fail this check, preventing TCB fouls via unspecified methods.
//!
//! ## Attempt 2: Use external_body with lying specs (WORKS - demonstrates TCB hole)
//!
//! The fundamental TCB hole: external_body specs are TRUSTED, not verified.
//! If the human writes a wrong spec, Verus proves false things.

use vstd::prelude::*;

verus! {

// TCB FOUL via external_body (the only way that works)

/// - This function LIES about what it returns.
/// - The spec says it returns 42, but it actually returns 0.
/// - Verus trusts the spec - it cannot see the body.
#[verifier::external_body]
pub fn lying_function() -> (result: u64)
    ensures result == 42  // LIE! Actually returns 0
{
    0  // The truth that Verus cannot see
}

/// Caller "proves" false things based on the lying spec
pub fn tcb_foul_lying_spec() {
    let x = lying_function();
    
    proof {
        // Verus proves this because lying_function's spec says result == 42
        assert(x == 42);  // UNSOUND: x is actually 0 at runtime!
        
        // We can now derive any falsehood
        assert(x + x == 84);  // False at runtime (0 + 0 = 0)
    }
}

/// Claims to increment, but actually decrements
#[verifier::external_body]
pub fn lying_increment(x: &mut u64)
    requires *x < u64::MAX
    ensures *x == *old(x) + 1  // LIE! Actually decrements
{
    *x = x.wrapping_sub(1);  // Decrement, not increment
}

pub fn tcb_foul_mutation() {
    let mut val: u64 = 100;
    
    lying_increment(&mut val);
    
    proof {
        // Verus thinks val == 101
        assert(val == 101);  // UNSOUND: val is actually 99
    }
}

/// Direct use of assume - the most obvious TCB foul
pub fn tcb_foul_assume() {
    let x: u64 = 0;
    
    proof {
        assume(x == 42);  // Just assume a falsehood
        assert(x == 42);  // Now we can "prove" it
    }
}

// BLOCKED ATTEMPTS (left as documentation)

/*
// These DON'T compile - Verus blocks unspecified &mut self methods

pub fn tcb_foul_vec_reverse() {
    let mut v: Vec<u64> = Vec::new();
    v.push(100);
    v.reverse();  // ERROR: complex arguments to &mut parameters are currently unsupported
}

pub fn tcb_foul_vec_sort() {
    let mut v: Vec<u64> = Vec::new();
    v.push(3);
    v.push(1);
    v.sort();  // ERROR: complex arguments to &mut parameters are currently unsupported
}
*/

} // verus!

// KEY INSIGHT
//
// Verus has good defenses against TCB fouls:
// 1. Blocks calls to unspecified &mut self methods (the "complex arguments" check)
// 2. Blocks unsafe code (transmute, raw pointers)
// 3. Requires explicit external_body or assume to bypass verification
//
// The TCB consists of:
// - All external_body function specs (trusted without proof)
// - All assume statements
// - All axioms
// - The Verus tool itself
//
// Running with --no-cheating disables assume/external_body,
// but then you can't interface with unverified Rust libraries.
