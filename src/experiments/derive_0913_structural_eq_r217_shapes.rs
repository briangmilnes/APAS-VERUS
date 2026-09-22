// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar and Guy Blelloch and Brian Milnes

//! Experiment (r217): what `#[derive(StructuralEq)]` proves for the four type
//! shapes r217 adds it to.
//!
//! Question. r216 measured `StructuralEq` on a struct of `u64`
//! (`derive_0913_structural_eq_struct.rs`), on a unit-variant enum
//! (`derive_0913_structural_eq_enum.rs`), and against a `View` whose type is
//! the field type itself (`derive_0913_structural_eq_view_bridge.rs`). r217
//! adds the derive to four shapes, one of which is not among those:
//! `Chap50`'s `MatrixDim` has a `View` of type `(nat, nat)` built by casting
//! both `usize` fields. A cast is a function application, so the view is
//! injective and the APAS postcondition `equal == (a@ == b@)` ought to follow,
//! but `as nat` is not the identity and was not measured. The other three
//! shapes are restated here so one file carries the whole claim: a struct of
//! two `usize` with no `View` (`Chap47`'s `LoadAndSize`), a unit-variant enum
//! with no `View` (`Chap37`'s `Color`), and a struct of `i64` and `usize`
//! whose `View` is the identity (`Chap57`'s `PQEntry`).
//!
//! RESULT: SUCCEEDS — 8 verified, 0 errors. `StructuralEq` decides spec
//!         equality for all four shapes. The `(nat, nat)` view is injective,
//!         so `MatrixDim`'s `==` decides view equality in both directions,
//!         which is the full APAS `PartialEq` postcondition. The enum's `==`
//!         also decides the variant test. Without the derive each `==` proves
//!         nothing, which `derive_0913_partial_eq_struct.rs` measures.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_structural_eq_r217_shapes.20260922-131253.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_structural_eq_r217_shapes

pub mod derive_0913_structural_eq_r217_shapes {

    use vstd::prelude::*;

    verus! {

    /// The shape of `src/Chap50/MatrixChain*.rs`'s `MatrixDim`.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, StructuralEq)]
    pub struct MatrixDimS {
        pub rows: usize,
        pub cols: usize,
    }

    impl View for MatrixDimS {
        type V = (nat, nat);

        open spec fn view(&self) -> (nat, nat) {
            (self.rows as nat, self.cols as nat)
        }
    }

    /// The shape of `src/Chap47/ParaHashTableStEph.rs`'s `LoadAndSize`, which
    /// has no `View`: only `PartialEq` is derived, not `Eq`.
    #[derive(Clone, Copy, PartialEq, StructuralEq)]
    pub struct LoadAndSizeS {
        pub load: usize,
        pub size: usize,
    }

    /// The shape of `src/Chap37/BSTRBMtEph.rs`'s `Color`.
    #[derive(Clone, Copy, PartialEq, Eq, StructuralEq)]
    pub enum ColorS {
        RedV,
        BlackV,
    }

    /// The shape of `src/Chap57/DijkstraStEphU64.rs`'s `PQEntry`.
    #[derive(Clone, Copy, PartialEq, Eq, StructuralEq)]
    pub struct PQEntryS {
        pub dist: i64,
        pub vertex: usize,
    }

    impl View for PQEntryS {
        type V = Self;

        open spec fn view(&self) -> Self {
            *self
        }
    }

    /// A `(nat, nat)` view built by casting is injective, so `==` decides view
    /// equality in both directions: the full APAS `PartialEq` postcondition.
    pub fn matrix_dim_eq_decides_view_equality(a: &MatrixDimS, b: &MatrixDimS) -> (equal: bool)
        ensures
            equal == (*a == *b),
            equal == (a@ == b@),
    {
        *a == *b
    }

    /// `PartialEq` alone, without `Eq`, is enough for the derive.
    pub fn load_and_size_eq_decides_spec_equality(a: &LoadAndSizeS, b: &LoadAndSizeS) -> (equal: bool)
        ensures
            equal == (*a == *b),
    {
        *a == *b
    }

    /// A colour comparison decides spec equality, and so the variant test that
    /// drives red-black rebalancing.
    pub fn color_eq_decides_variant(c: ColorS) -> (red: bool)
        ensures
            red == (c is RedV),
    {
        c == ColorS::RedV
    }

    /// An identity `View` makes spec equality and view equality the same
    /// predicate, which is what `Chap57`'s deleted hand-written
    /// `PartialEqSpecImpl` asserted without a check.
    pub fn pq_entry_eq_decides_view_equality(a: &PQEntryS, b: &PQEntryS) -> (equal: bool)
        ensures
            equal == (a@ == b@),
    {
        *a == *b
    }

    } // verus!
} // pub mod derive_0913_structural_eq_r217_shapes
