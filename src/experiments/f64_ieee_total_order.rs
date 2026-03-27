//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Hypothesis: by(bit_vector) works for f32 (per Verus test suite). Does it work for f64?
//! Testing f32 first to confirm the mechanism, then f64.

pub mod f64_ieee_total_order {

    use vstd::prelude::*;
    use vstd::float::FloatBitsProperties;

    verus! {

    // Test with f32 first (known to work from Verus test suite).
    proof fn test_f32_reflexive(x: f32)
        ensures !x.is_nan_spec() ==> x <= x,
    {
        assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
    }

    } // verus!
}
