//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: Copy vs Clone in Verus — the clone wars.
//!
//! Questions and results:
//!   1. Does derive(Copy, Clone) on a generic struct work inside verus!?
//!      RESULT: COMPILES but warns "does not (yet) support autoderive Clone
//!      impl when the clone is not a copy" — Verus does not recognize that
//!      Copy implies Clone is a bitwise copy. The warning fires even when
//!      the type IS Copy.
//!
//!   2. Does Copy eliminate the Clone assume workaround?
//!      RESULT: NO. Calling .clone() on a Copy type still has no spec.
//!      `cloned@ == p@` is unprovable after `.clone()` without assume.
//!      The derived Clone is treated as external (no spec generated).
//!
//!   3. Does Copy eliminate the PartialEq assume workaround?
//!      RESULT: NO. derive(PartialEq) on a Copy type is treated as external.
//!      Calling `==` on a derived-PartialEq Copy type gives: "cannot use
//!      function which is ignored because it is declared outside verus! or
//!      marked external." Manual PartialEq with PartialEqSpecImpl is still
//!      required.
//!
//!   4. Does derive(Copy, Clone) with View work?
//!      RESULT: YES. View impl on Copy structs works normally.
//!
//!   5. Does a Copy type used in a Vec still need assume for Vec::clone?
//!      RESULT: YES. Vec is not Copy regardless of element type. Vec::clone
//!      still needs the assume workaround.
//!
//!   6. Can we derive(Copy, Clone, Eq, PartialEq) all at once?
//!      RESULT: COMPILES (with warnings) but PartialEq is external/unusable.
//!
//!   7. Does Verus generate a spec for derived Clone when the type is Copy?
//!      RESULT: NO. The warning says Verus skips the Clone spec entirely.
//!
//!   8. Does implicit copy (let a = p; — no .clone()) preserve View?
//!      RESULT: YES. Implicit copy on Copy types preserves View provably.
//!      This is the one genuine win: use implicit copy, avoid .clone().
//!
//!   9. Does manual PartialEq on a Copy type with primitive fields work
//!      without assume?
//!      RESULT: YES for CopyPoint (i64 fields). Field-by-field comparison
//!      on primitives proves `r == (self@ == other@)` without assume.
//!
//! BOTTOM LINE: Copy gives you implicit duplication (no .clone() needed)
//! and manual PartialEq on primitive fields works without assume. But
//! derive(Clone) and derive(PartialEq) remain unspecified by Verus even
//! on Copy types. The 101 workaround assumes are NOT eliminated by Copy.
//! The current PartialEq/Clone pattern (manual impls with assumes) remains
//! necessary for generic types. For concrete types with only primitive
//! fields, Copy + manual field-by-field eq can avoid the assume.
//!
//! RESULT: FAILS (test 4: .clone() on Copy type has no spec)

pub mod copy_vs_clone_wars {
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::{PartialEqSpec, PartialEqSpecImpl};

    verus! {

    // Test 1: Basic generic Copy struct inside verus! — PASSES (with warning).
    #[derive(Copy, Clone)]
    pub struct CopyPair<A: Copy, B: Copy> {
        pub first: A,
        pub second: B,
    }

    // Test 2 + 4: Copy struct with View — PASSES.
    impl<A: Copy, B: Copy> View for CopyPair<A, B> {
        type V = (A, B);
        open spec fn view(&self) -> (A, B) {
            (self.first, self.second)
        }
    }

    // Test 3: Implicit copy preserves View — PASSES (no assume needed).
    fn test_copy_clone_no_assume(p: CopyPair<u64, u64>) -> (cloned: CopyPair<u64, u64>)
        ensures cloned@ == p@,
    {
        let cloned = p;  // Copy — implicit bitwise copy, no .clone() needed.
        cloned
    }

    // Test 4: Explicit .clone() on Copy type — FAILS.
    // Verus has no spec for derived Clone, even on Copy types.
    // Uncomment to see: "postcondition not satisfied: cloned@ == p@"
    //
    // fn test_copy_explicit_clone_no_assume(p: &CopyPair<u64, u64>)
    //     -> (cloned: CopyPair<u64, u64>)
    //     ensures cloned@ == p@,
    // {
    //     let cloned = p.clone();
    //     cloned
    // }

    // Test 5: derive(Copy, Clone, Eq, PartialEq) — compiles but PartialEq is external.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CopyTriple<A: Copy + Eq, B: Copy + Eq, C: Copy + Eq> {
        pub a: A,
        pub b: B,
        pub c: C,
    }

    impl<A: Copy + Eq, B: Copy + Eq, C: Copy + Eq> View for CopyTriple<A, B, C> {
        type V = (A, B, C);
        open spec fn view(&self) -> (A, B, C) {
            (self.a, self.b, self.c)
        }
    }

    // Test 6: Derived PartialEq on Copy type — FAILS (external, unusable).
    // Verus treats derive(PartialEq) as external even on Copy types.
    // Error: "cannot use function which is ignored because it is either
    // declared outside the verus! macro or it is marked as external"
    // SKIPPED.

    // Test 7 + 9: Manual PartialEq on Copy type with primitive fields — PASSES.
    #[derive(Copy, Clone)]
    pub struct CopyPoint {
        pub x: i64,
        pub y: i64,
    }

    impl View for CopyPoint {
        type V = (i64, i64);
        open spec fn view(&self) -> (i64, i64) { (self.x, self.y) }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for CopyPoint {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    // No assume needed! Field-by-field comparison on primitives is provable.
    impl PartialEq for CopyPoint {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@),
        {
            let r = self.x == other.x && self.y == other.y;
            r
        }
    }

    impl Eq for CopyPoint {}

    // Test 8: Implicit copy preserves View across multiple uses — PASSES.
    fn test_copy_in_function(p: CopyPoint) -> (result: (CopyPoint, CopyPoint))
        ensures result.0@ == p@, result.1@ == p@,
    {
        let a = p;   // Copy
        let b = p;   // Copy again — not a use-after-move error
        (a, b)
    }

    // Test 9: Vec of Copy type clone — still needs assume — expected.
    fn test_vec_of_copy_clone(v: &Vec<CopyPoint>) -> (cloned: Vec<CopyPoint>)
        ensures cloned@ == v@,
    {
        let cloned = v.clone();
        proof { assume(cloned@ == v@); }
        cloned
    }

    // Test 10: Copy type avoids .clone() entirely — PASSES.
    fn test_copy_avoids_clone(points: &Vec<CopyPoint>) -> (sum_x: i64)
        requires points@.len() > 0,
    {
        let p = points[0];     // Copy — no clone needed
        let q = p;             // Copy again
        let _r = q;            // And again
        p.x                    // p is still usable
    }

    } // verus!
} // pub mod
