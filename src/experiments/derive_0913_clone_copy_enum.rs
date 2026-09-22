// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `Clone` gives the prover on an enum that
//! is also `Copy`.
//!
//! Question. `derive_0913_clone_enum.rs` asks the non-`Copy` half. When `Copy`
//! is derived alongside, the compiler emits `*self` for the clone body instead
//! of a `match`, which is the shape `clone_add_post_condition` recognises. Does
//! the enum then get `ensures ret == self`, as the struct does in
//! `derive_0913_clone_copy_struct.rs`?
//!
//! RESULT: SUCCEEDS — 2 verified, 0 errors, no warning. Adding `Copy` changes
//!         the emitted clone body to a read of `self`, and the enum then gets
//!         `ensures ret == self`, exactly as the struct does.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_clone_copy_enum.20260922-112334.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_clone_copy_enum

pub mod derive_0913_clone_copy_enum {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone, Copy)]
    pub enum ColourS {
        RedV,
        BlackV,
        GradeV(u64),
    }

    pub fn clone_preserves_variant(c: &ColourS) -> (cloned: ColourS)
        ensures
            cloned == *c,
    {
        c.clone()
    }

    } // verus!
} // pub mod derive_0913_clone_copy_enum
