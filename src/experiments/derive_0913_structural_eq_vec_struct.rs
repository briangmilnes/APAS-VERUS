// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether `#[derive(StructuralEq)]` applies to a struct
//! that owns a `Vec`.
//!
//! Question. `derive_0913_structural_eq_struct.rs` shows that `StructuralEq`
//! supplies the `PartialEqSpecImpl` a bare `PartialEq` derive lacks. Every APAS
//! collection that hand-writes `PartialEq` with one `accept` is backed by a
//! `Vec`, so the deletion depends on whether `Vec<u64>` is `Structural`. The
//! derive emits `let _: AssertParamIsStructural<Vec<u64>>` per field
//! (`builtin_macros/src/structural.rs`), which compiles only if the field type
//! carries the marker.
//!
//! RESULT: FAILS — does not compile. `error[E0277]: the trait bound
//!         `std::vec::Vec<u64>: vstd::prelude::Structural` is not satisfied`,
//!         required by `AssertParamIsStructural`. `StructuralEq` is
//!         unavailable to every `Vec`-backed APAS collection.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_structural_eq_vec_struct.20260922-112441.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_structural_eq_vec_struct

pub mod derive_0913_structural_eq_vec_struct {

    use vstd::prelude::*;

    verus! {

    #[derive(PartialEq, Eq, StructuralEq)]
    pub struct SeqS {
        pub seq: Vec<u64>,
    }

    impl View for SeqS {
        type V = Seq<u64>;

        open spec fn view(&self) -> Seq<u64> {
            self.seq@
        }
    }

    /// The postcondition every APAS collection's `PartialEq` states.
    pub fn eq_decides_view_equality(a: &SeqS, b: &SeqS) -> (equal: bool)
        ensures
            equal == (a@ == b@),
    {
        *a == *b
    }

    } // verus!
} // pub mod derive_0913_structural_eq_vec_struct
