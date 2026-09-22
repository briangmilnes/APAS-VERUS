// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `Clone` gives the prover when the struct
//! is also `Copy`.
//!
//! Question. Verus 0.2026.09.13 treats `#[derive(Clone)]` as a special case
//! (`rust_verify/src/automatic_derive.rs`, `clone_add_post_condition`): when the
//! derived body reads `self` — the shape the compiler emits for a `Copy` type —
//! Verus attaches `ensures ret == self` to the derived `clone`. Does that
//! postcondition reach a caller, and does it carry through `View`?
//!
//! RESULT: SUCCEEDS — 2 verified, 0 errors, no warning. The derived `clone`
//!         on a non-generic `Copy` struct carries `ensures ret == self`, and
//!         view equality follows by congruence. No `accept`, no hand-written
//!         `Clone`.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_clone_copy_struct.20260922-112332.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_clone_copy_struct

pub mod derive_0913_clone_copy_struct {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone, Copy)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    impl View for PointS {
        type V = (int, int);

        open spec fn view(&self) -> (int, int) {
            (self.x as int, self.y as int)
        }
    }

    /// The derived `clone` carries `ensures ret == self`, so the caller gets
    /// structural equality and, by congruence, view equality, with no `accept`.
    pub fn clone_gives_structural_equality(p: &PointS) -> (cloned: PointS)
        ensures
            cloned == *p,
            cloned@ == p@,
    {
        p.clone()
    }

    } // verus!
} // pub mod derive_0913_clone_copy_struct
