// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether `#[derive(StructuralEq)]` applies to a
//! non-generic enum.
//!
//! Question. `src/Chap37/BSTRBMtEph.rs` already writes
//! `#[derive(Clone, Copy, PartialEq, Eq)]` on its `Color` enum, so `==` on a
//! node colour currently proves nothing
//! (`derive_0913_partial_eq_struct.rs`). `StructuralEq` fixes that for a
//! struct; the `PartialEqSpecImpl` the macro emits names the bare type, and
//! the `Structural` impl goes through `synstructure`, which handles enums.
//! Does the pair compile and specify `==` for an enum?
//!
//! RESULT: SUCCEEDS — 3 verified, 0 errors. `StructuralEq` works on a
//!         non-generic enum exactly as on a struct, and a colour comparison
//!         then decides both spec equality and the variant test.
//!         `src/Chap37/BSTRBMtEph.rs` gains a specified `==` by adding one
//!         word to a derive list it already has.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_structural_eq_enum.20260922-113141.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_structural_eq_enum

pub mod derive_0913_structural_eq_enum {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone, Copy, PartialEq, Eq, StructuralEq)]
    pub enum ColourS {
        RedV,
        BlackV,
    }

    /// The postcondition a red-black tree needs when it compares colours.
    pub fn eq_decides_spec_equality(a: ColourS, b: ColourS) -> (equal: bool)
        ensures
            equal == (a == b),
    {
        a == b
    }

    /// A colour comparison drives the rebalancing case split.
    pub fn is_red(c: ColourS) -> (red: bool)
        ensures
            red == (c is RedV),
    {
        c == ColourS::RedV
    }

    } // verus!
} // pub mod derive_0913_structural_eq_enum
