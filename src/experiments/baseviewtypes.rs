// Copyright (c) 2025 Brian G. Milnes
//! Experiment: What are the View types of primitives?
//! Testing whether u64@, i64@, bool@, ()@ give identity or int/bool/unit.
//!
//! RESULT: All primitive Views are IDENTITY (type V = Self)
//!         NO implicit coercion from u64 to int
//!         Must use explicit `as int` conversion

use vstd::prelude::*;

verus! {

// Identity views for all primitives

// Test u64 view type - IDENTITY
proof fn test_u64_view() {
    let x: u64 = 42u64;
    let v: u64 = x@;           // ✓ u64@ : u64
    let lit: u64 = 42u64@;     // ✓ literal view
}

// Test i64 view type - IDENTITY  
proof fn test_i64_view() {
    let x: i64 = -42i64;
    let v: i64 = x@;           // ✓ i64@ : i64
}

// Test bool view type - IDENTITY
proof fn test_bool_view() {
    let x: bool = true;
    let v: bool = x@;          // ✓ bool@ : bool
    let lit: bool = true@;     // ✓ literal view
}

// Test unit view type - IDENTITY
proof fn test_unit_view() {
    let x: () = ();
    let v: () = x@;            // ✓ ()@ : ()
}

// No implicit coercion: u64 does NOT auto-convert to int

proof fn test_explicit_conversion() {
    let x: u64 = 42u64;
    
    // FAILS: let as_int: int = x@;   // No implicit coercion!
    // FAILS: let as_int: int = x;    // No implicit widening!
    
    // WORKS: explicit cast
    let as_int: int = x@ as int;      // ✓ explicit conversion
    let as_int2: int = x as int;      // ✓ explicit conversion
}

// Test Vec<u64> view - elements are u64, not int
proof fn test_vec_u64_view(v: Vec<u64>)
    requires v.len() > 0
{
    let seq      : Seq<u64> = v@;              // Seq<u64>
    let seq_int  : Seq<int> = v@.map(|i: int, x: u64| x as int);

    let elem : u64      = seq[0];          // ✓ Element is u64
    
    // FAILS: let elem2: int = seq[0];  // No coercion!
    
    // WORKS: explicit
    let elem_int: int = seq[0] as int;  // ✓
}

// Comparison across types requires explicit conversion
proof fn test_cross_type_comparison() {
    let x: u64 = 42u64;
    let y: int = 100;
    
    // FAILS: assert(x < y);  // Can't compare u64 and int directly!
    
    // WORKS: explicit conversion
    assert((x as int) < y);   // ✓
}

} // verus!
