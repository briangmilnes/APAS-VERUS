//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Hypothesis: f32 by(bit_vector) works for concrete ranges (per Verus test suite)
//! but crashes on is_nan_spec(). Test without is_nan_spec.

pub mod f32_ieee_total_order {

    use vstd::prelude::*;

    verus! {

    // Test 1: Concrete — known to work from Verus test suite.
    proof fn test_concrete()
        ensures 1.0f32 <= 2.0f32,
    {
        assert(1.0f32 <= 2.0f32) by(bit_vector);
    }

    // Test 2: Range-bounded reflexive (no is_nan_spec).
    proof fn test_bounded_reflexive(x: f32)
        ensures 0.0f32 <= x <= 100.0f32 ==> x <= x,
    {
        assert(0.0f32 <= x <= 100.0f32 ==> x <= x) by(bit_vector);
    }

    // Test 3: Range-bounded total.
    proof fn test_bounded_total(x: f32, y: f32)
        ensures 0.0f32 <= x <= 100.0f32 && 0.0f32 <= y <= 100.0f32
            ==> (x <= y || y <= x),
    {
        assert(0.0f32 <= x <= 100.0f32 && 0.0f32 <= y <= 100.0f32
            ==> (x <= y || y <= x)) by(bit_vector);
    }

    // Test 4: Range-bounded transitive.
    proof fn test_bounded_transitive(x: f32, y: f32, z: f32)
        ensures 0.0f32 <= x <= 100.0f32 && 0.0f32 <= y <= 100.0f32 && 0.0f32 <= z <= 100.0f32
            && x <= y && y <= z ==> x <= z,
    {
        assert(0.0f32 <= x <= 100.0f32 && 0.0f32 <= y <= 100.0f32 && 0.0f32 <= z <= 100.0f32
            && x <= y && y <= z ==> x <= z) by(bit_vector);
    }

    // Test 5: Addition from Verus test suite pattern.
    proof fn test_add_bounded(x: f32)
        ensures 2.0f32 <= x <= 5.0f32 ==> x + x <= 10.0f32,
    {
        assert(2.0f32 <= x <= 5.0f32 ==> x + x <= 10.0f32) by(bit_vector);
    }

    } // verus!
}
