// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r217): whether the derived-`Clone` postcondition survives `f64`
//! fields, and whether a struct whose fields are themselves such a struct
//! inherits it.
//!
//! Question. `derive_0913_clone_copy_struct.rs` measures `#[derive(Clone,
//! Copy)]` on a struct of `u64`. `clone_add_post_condition`
//! (`rust_verify/src/automatic_derive.rs`) keys on the shape of the emitted
//! body — a read of `self` — and not on the field types, so `ensures ret ==
//! self` ought to attach to an `f64` struct too. But `f64` is not
//! `Structural` (`builtin/src/lib.rs` carries a TODO for it), so the spec
//! equality `cloned == *self` might not be available over `f64` fields at all.
//! `src/Chap26/ETSPStEph.rs`'s `Point` and `Edge` are exactly this shape, and
//! `Edge`'s fields are `Point`s, so the nested case is measured too.
//!
//! RESULT: SUCCEEDS — 6 verified, 0 errors, no warning. The derived `Clone` on
//!         a non-generic `Copy` struct of `f64` carries `ensures ret == self`,
//!         exactly as it does for `u64`; the field types do not matter,
//!         only the shape of the emitted body. Spec equality over `f64`
//!         fields is available even though `f64` is not `Structural`, because
//!         `Structural` governs the `==` operator and not `spec_eq`. A struct
//!         of such structs inherits both facts.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_clone_copy_f64_struct.20260922-130315.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_clone_copy_f64_struct

pub mod derive_0913_clone_copy_f64_struct {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone, Copy)]
    pub struct PointS {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Clone, Copy)]
    pub struct EdgeS {
        pub from: PointS,
        pub to: PointS,
    }

    /// The fact `src/Chap26/ETSPStEph.rs`'s hand-written `Clone for Point`
    /// stated: `ensures cloned == *self`.
    pub fn clone_gives_structural_equality(p: &PointS) -> (cloned: PointS)
        ensures
            cloned == *p,
    {
        p.clone()
    }

    /// The same fact for `Edge`, whose fields are themselves derived `Copy`
    /// structs of `f64`.
    pub fn clone_edge_gives_structural_equality(e: &EdgeS) -> (cloned: EdgeS)
        ensures
            cloned == *e,
    {
        e.clone()
    }

    /// Structural equality of the whole implies equality of each field, so a
    /// caller reasoning field-by-field keeps what it had.
    pub fn clone_preserves_fields(e: &EdgeS) -> (cloned: EdgeS)
        ensures
            cloned.from == e.from,
            cloned.to == e.to,
    {
        e.clone()
    }

    /// A copying move leaves an equal value, which is what deleting the
    /// hand-written `impl Copy` relies on.
    pub fn copy_equals_source(p: PointS) -> (copied: PointS)
        ensures
            copied == p,
    {
        let copied = p;
        copied
    }

    } // verus!
} // pub mod derive_0913_clone_copy_f64_struct
