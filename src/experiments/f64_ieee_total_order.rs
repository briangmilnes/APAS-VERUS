//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! RESULT: FAILS — f64 by(bit_vector) crashes Verus at bitvector_to_air.rs:424.
//! Hypothesis: Using the patterns from Verus test suite float.rs, can we prove
//! f64 total order properties? The test suite uses:
//!   - by(bit_vector) for IEEE float assertions
//!   - add_spec, lt_ensures, partial_cmp_spec from vstd::std_specs
//!   - is_nan_spec, is_finite_spec from FloatBitsProperties

pub mod f64_ieee_total_order {

    use vstd::prelude::*;
    use vstd::float::FloatBitsProperties;

    verus! {

    // Pattern from Verus test suite float.rs line 193:
    // assert(2.0f32 <= x <= 5.0f32 ==> x + x <= 10.0f32) by(bit_vector);

    // Test 1: f32 reflexive (known working pattern from test suite).
    proof fn test_f32_reflexive(x: f32)
        ensures !x.is_nan_spec() ==> x <= x,
    {
        assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
    }

    // Test 2: f32 total order.
    proof fn test_f32_total(x: f32, y: f32)
        ensures !x.is_nan_spec() && !y.is_nan_spec() ==> (x <= y || y <= x),
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec() ==> (x <= y || y <= x)) by(bit_vector);
    }

    // Test 3: f64 reflexive.
    proof fn test_f64_reflexive(x: f64)
        ensures !x.is_nan_spec() ==> x <= x,
    {
        assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
    }

    // Test 4: f64 total order.
    proof fn test_f64_total(x: f64, y: f64)
        ensures !x.is_nan_spec() && !y.is_nan_spec() ==> (x <= y || y <= x),
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec() ==> (x <= y || y <= x)) by(bit_vector);
    }

    // Test 5: f64 transitive.
    proof fn test_f64_transitive(x: f64, y: f64, z: f64)
        ensures !x.is_nan_spec() && !y.is_nan_spec() && !z.is_nan_spec()
            && x <= y && y <= z ==> x <= z,
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec() && !z.is_nan_spec()
            && x <= y && y <= z ==> x <= z) by(bit_vector);
    }

    // Test 6: f64 antisymmetric.
    proof fn test_f64_antisymmetric(x: f64, y: f64)
        ensures !x.is_nan_spec() && !y.is_nan_spec()
            && x <= y && y <= x ==> x == y,
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec()
            && x <= y && y <= x ==> x == y) by(bit_vector);
    }

    // Test 7: Concrete f64 comparison (no quantifiers).
    proof fn test_f64_concrete()
        ensures 1.0f64 <= 2.0f64,
    {
        assert(1.0f64 <= 2.0f64) by(bit_vector);
    }

    } // verus!
}
