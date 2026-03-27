//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Testing what f32 by(bit_vector) can actually prove for algorithm use.
//! Need: unbounded non-negative finite floats, addition, comparison bridge.

pub mod f32_ieee_total_order {

    use vstd::prelude::*;
    use vstd::float::FloatBitsProperties;

    verus! {

    // TIER 1: Fixed range (known working)
    proof fn t1_reflexive_fixed(x: f32)
        ensures 0.0f32 <= x <= 100.0f32 ==> x <= x,
    {
        assert(0.0f32 <= x <= 100.0f32 ==> x <= x) by(bit_vector);
    }

    // TIER 2: Non-negative only (0.0 <= x, no upper bound).
    // This is what algorithms actually need.
    proof fn t2_reflexive_nonneg(x: f32)
        ensures 0.0f32 <= x ==> x <= x,
    {
        assert(0.0f32 <= x ==> x <= x) by(bit_vector);
    }

    proof fn t2_total_nonneg(x: f32, y: f32)
        ensures 0.0f32 <= x && 0.0f32 <= y ==> (x <= y || y <= x),
    {
        assert(0.0f32 <= x && 0.0f32 <= y ==> (x <= y || y <= x)) by(bit_vector);
    }

    proof fn t2_transitive_nonneg(x: f32, y: f32, z: f32)
        ensures 0.0f32 <= x && 0.0f32 <= y && 0.0f32 <= z
            && x <= y && y <= z ==> x <= z,
    {
        assert(0.0f32 <= x && 0.0f32 <= y && 0.0f32 <= z
            && x <= y && y <= z ==> x <= z) by(bit_vector);
    }

    proof fn t2_antisymmetric_nonneg(x: f32, y: f32)
        ensures 0.0f32 <= x && 0.0f32 <= y
            && x <= y && y <= x ==> x == y,
    {
        assert(0.0f32 <= x && 0.0f32 <= y
            && x <= y && y <= x ==> x == y) by(bit_vector);
    }

    // TIER 3: Addition properties for non-negative floats.
    proof fn t3_add_zero(x: f32)
        ensures 0.0f32 <= x ==> x + 0.0f32 == x,
    {
        assert(0.0f32 <= x ==> x + 0.0f32 == x) by(bit_vector);
    }

    proof fn t3_add_monotone(a: f32, b: f32, c: f32)
        ensures 0.0f32 <= a && 0.0f32 <= b && 0.0f32 <= c
            && b <= c
            && a + b <= 3.4e38f32 && a + c <= 3.4e38f32
            ==> a + b <= a + c,
    {
        assert(0.0f32 <= a && 0.0f32 <= b && 0.0f32 <= c
            && b <= c
            && a + b <= 3.4e38f32 && a + c <= 3.4e38f32
            ==> a + b <= a + c) by(bit_vector);
    }

    // TIER 4: is_finite_spec in by(bit_vector) — does this crash?
    // proof fn t4_reflexive_finite(x: f32)
    //     ensures x.is_finite_spec() && 0.0f32 <= x ==> x <= x,
    // {
    //     assert(x.is_finite_spec() && 0.0f32 <= x ==> x <= x) by(bit_vector);
    // }

    // TIER 5: Bridging by(bit_vector) results to regular proof context.
    pub open spec fn f32_le(x: f32, y: f32) -> bool {
        x <= y
    }

    proof fn t5_bridge_reflexive(x: f32)
        ensures 0.0f32 <= x ==> f32_le(x, x),
    {
        assert(0.0f32 <= x ==> x <= x) by(bit_vector);
    }

    proof fn t5_bridge_total(x: f32, y: f32)
        ensures 0.0f32 <= x && 0.0f32 <= y ==> f32_le(x, y) || f32_le(y, x),
    {
        assert(0.0f32 <= x && 0.0f32 <= y ==> x <= y || y <= x) by(bit_vector);
    }

    } // verus!
}
