//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Hypothesis: f32 total order properties verify via by(bit_vector).
//! f64 crashes Verus (bitvector_to_air.rs:424) — see f64_ieee_total_order.rs.
//! Testing f32 to confirm the mechanism works for the smaller float type.

pub mod f32_ieee_total_order {

    use vstd::prelude::*;
    use vstd::float::FloatBitsProperties;

    verus! {

    // Test 1: Reflexive.
    proof fn test_reflexive(x: f32)
        ensures !x.is_nan_spec() ==> x <= x,
    {
        assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
    }

    // Test 2: Transitive.
    proof fn test_transitive(x: f32, y: f32, z: f32)
        ensures !x.is_nan_spec() && !y.is_nan_spec() && !z.is_nan_spec()
            && x <= y && y <= z ==> x <= z,
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec() && !z.is_nan_spec()
            && x <= y && y <= z ==> x <= z) by(bit_vector);
    }

    // Test 3: Antisymmetric.
    proof fn test_antisymmetric(x: f32, y: f32)
        ensures !x.is_nan_spec() && !y.is_nan_spec()
            && x <= y && y <= x ==> x == y,
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec()
            && x <= y && y <= x ==> x == y) by(bit_vector);
    }

    // Test 4: Total.
    proof fn test_total(x: f32, y: f32)
        ensures !x.is_nan_spec() && !y.is_nan_spec() ==> (x <= y || y <= x),
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec() ==> (x <= y || y <= x)) by(bit_vector);
    }

    // Test 5: Concrete comparison.
    proof fn test_concrete()
        ensures 1.0f32 <= 2.0f32,
    {
        assert(1.0f32 <= 2.0f32) by(bit_vector);
    }

    // Test 6: Spec function wrapping.
    pub open spec fn f32_le(x: f32, y: f32) -> bool {
        x <= y
    }

    proof fn total_order_reflexive(x: f32)
        ensures !x.is_nan_spec() ==> f32_le(x, x),
    {
        assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
    }

    proof fn total_order_total(x: f32, y: f32)
        ensures !x.is_nan_spec() && !y.is_nan_spec()
            ==> f32_le(x, y) || f32_le(y, x),
    {
        assert(!x.is_nan_spec() && !y.is_nan_spec()
            ==> x <= y || y <= x) by(bit_vector);
    }

    } // verus!
}
