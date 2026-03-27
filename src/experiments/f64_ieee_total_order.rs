//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Hypothesis: With Verus ff454ab0f (IEEE float SMT), Z3 can prove that finite f64
//! values form a total order using IeeeFloat::ieee_le — reflexive, transitive,
//! antisymmetric, total — without any assumes or axioms.
//!
//! Key insight: raw `<=` on f64 goes through PartialOrd (uninterpreted in Z3).
//! Must use `IeeeFloat::ieee_le` to get the IEEE SMT encoding.

pub mod f64_ieee_total_order {

    use vstd::prelude::*;
    use vstd::float::FloatBitsProperties;

    verus! {

    // Test 1: Basic IEEE comparison. Does Z3 prove reflexive?
    proof fn test_reflexive(x: f64)
        requires x.is_finite_spec(),
        ensures x.ieee_le(x),
    {
    }

    proof fn test_transitive(x: f64, y: f64, z: f64)
        requires x.is_finite_spec(), y.is_finite_spec(), z.is_finite_spec(),
            x.ieee_le(y), y.ieee_le(z),
        ensures x.ieee_le(z),
    {
    }

    proof fn test_antisymmetric(x: f64, y: f64)
        requires x.is_finite_spec(), y.is_finite_spec(),
            x.ieee_le(y), y.ieee_le(x),
        ensures x.ieee_eq(y),
    {
    }

    proof fn test_total(x: f64, y: f64)
        requires x.is_finite_spec(), y.is_finite_spec(),
        ensures x.ieee_le(y) || y.ieee_le(x),
    {
    }

    // Test 2: IEEE addition properties.
    proof fn test_add_identity(a: f64)
        requires a.is_finite_spec(),
        ensures a.ieee_add(0.0).ieee_eq(a),
    {
    }

    // Test 3: Exec comparison via ieee_le.
    fn exec_compare(x: f64, y: f64) -> (r: bool)
        requires x.is_finite_spec(), y.is_finite_spec(),
        ensures r == x.ieee_le(y),
    {
        x.ieee_le(y)
    }

    // Test 4: Spec function wrapping ieee_le.
    pub open spec fn f64_le(x: f64, y: f64) -> bool {
        x.ieee_le(y)
    }

    proof fn total_order_reflexive(x: f64)
        requires x.is_finite_spec(),
        ensures f64_le(x, x),
    {
    }

    proof fn total_order_transitive(x: f64, y: f64, z: f64)
        requires x.is_finite_spec(), y.is_finite_spec(), z.is_finite_spec(),
            f64_le(x, y), f64_le(y, z),
        ensures f64_le(x, z),
    {
    }

    proof fn total_order_antisymmetric(x: f64, y: f64)
        requires x.is_finite_spec(), y.is_finite_spec(),
            f64_le(x, y), f64_le(y, x),
        ensures x.ieee_eq(y),
    {
    }

    proof fn total_order_total(x: f64, y: f64)
        requires x.is_finite_spec(), y.is_finite_spec(),
        ensures f64_le(x, y) || f64_le(y, x),
    {
    }

    } // verus!
}
