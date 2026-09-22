// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether `#[derive(StructuralEq)]` applies to a generic
//! struct.
//!
//! Question. Every APAS collection is generic in its element type, so a
//! deletion round needs `StructuralEq` to work on `S<T>`. The macro in
//! `builtin_macros/src/structural.rs` emits the `PartialEqSpecImpl` through
//! `quote!` on the bare identifier `#name`, with no generic parameters and no
//! where clause, while the `Structural` impl goes through
//! `synstructure::gen_impl`, which does carry them. Does the pair compile on a
//! generic struct?
//!
//! RESULT: FAILS — does not compile. `error[E0107]: missing generics for
//!         struct ... BoxS; expected 1 generic argument`, raised at the struct
//!         definition by the emitted `impl PartialEqSpecImpl for BoxS`.
//!         `StructuralEq` is unavailable to every generic APAS type.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_structural_eq_generic_struct.20260922-112441.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_structural_eq_generic_struct

pub mod derive_0913_structural_eq_generic_struct {

    use vstd::prelude::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    #[derive(PartialEq, Eq, StructuralEq)]
    pub struct BoxS<T> {
        pub value: T,
    }

    pub fn eq_decides_spec_equality<T: Eq>(a: &BoxS<T>, b: &BoxS<T>) -> (equal: bool)
        ensures
            equal == (*a == *b),
    {
        *a == *b
    }

    } // verus!
} // pub mod derive_0913_structural_eq_generic_struct
