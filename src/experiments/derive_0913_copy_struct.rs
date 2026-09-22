// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether `#[derive(Copy)]` is accepted inside `verus!`,
//! and whether a copy is equal to its source.
//!
//! Question. `automatic_derive.rs` maps `RustItem::Copy` to
//! `AutomaticDeriveAction::VerifyAsIs`, so the derived `Copy` impl goes through
//! the verifier as an ordinary impl rather than being ignored. `Copy` has no
//! methods, so there is nothing to specify; the fact a caller needs is that a
//! copying move leaves an equal value behind. Does Verus know it?
//!
//! RESULT: SUCCEEDS — 2 verified, 0 errors. `Copy` is accepted inside `verus!`
//!         and a copying move gives an equal value, because Verus models a
//!         copy as the same value, not through the trait.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_copy_struct.20260922-112727.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_copy_struct

pub mod derive_0913_copy_struct {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone, Copy)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    /// A copying move: `p` is still usable, and the two values are equal.
    pub fn copy_equals_source(p: PointS) -> (copied: PointS)
        ensures
            copied == p,
    {
        let copied = p;
        assert(copied.x == p.x);
        copied
    }

    } // verus!
} // pub mod derive_0913_copy_struct
