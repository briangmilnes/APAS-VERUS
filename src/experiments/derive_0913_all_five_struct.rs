// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): the exact line `#[derive(Clone, Copy, PartialEq, Eq,
//! Debug)]` on one struct inside `verus!`.
//!
//! Question. All five derives on one struct, inside `verus!`, on
//! 0.2026.09.13: does it compile and verify, and which of the two facts a
//! caller wants does it supply? The two facts are `cloned == *self`, which the
//! `Copy` shape of the derived `Clone` should give
//! (`derive_0913_clone_copy_struct.rs`), and `equal == (*a == *b)`, which the
//! bare `PartialEq` derive should not
//! (`derive_0913_partial_eq_struct.rs`).
//!
//! RESULT: PARTIAL — 2 verified, 1 error, no warning. The line compiles inside
//!         `verus!` and the clone fact holds; the equality fact does not.
//!         `#[derive(StructuralEq)]` is the missing sixth derive; see
//!         `derive_0913_all_five_plus_structural.rs`.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_all_five_struct.20260922-112703.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_all_five_struct

pub mod derive_0913_all_five_struct {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    /// Fact one: the clone equals its source.
    pub fn clone_gives_structural_equality(p: &PointS) -> (cloned: PointS)
        ensures
            cloned == *p,
    {
        p.clone()
    }

    /// Fact two: `==` decides spec equality.
    pub fn eq_decides_spec_equality(a: &PointS, b: &PointS) -> (equal: bool)
        ensures
            equal == (*a == *b),
    {
        *a == *b
    }

    } // verus!

    // 14. derive impls outside verus!

    /// The derived `Debug` is reachable from ordinary Rust.
    pub fn render(p: &PointS) -> String {
        format!("{:?}", p)
    }
} // pub mod derive_0913_all_five_struct
