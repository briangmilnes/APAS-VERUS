// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether `#[derive(Clone, Copy)]` on a generic struct
//! still yields the `ensures ret == self` that the non-generic case gets.
//!
//! Question. `derive_0913_clone_generic_struct.rs` shows that a generic struct
//! deriving `Clone` alone gets no specification. rustc emits the `*self` clone
//! body whenever `Copy` is derived alongside, including for a generic struct,
//! so `clone_add_post_condition` should recognise the shape here too. Does the
//! postcondition survive the generic parameter?
//!
//! RESULT: FAILS — 1 verified, 1 error, with the
//!         `autoderive_clone_without_spec` warning. The `Copy` exemption does
//!         not reach a generic struct: rustc emits a constructor body rather
//!         than a read of `self`, so `clone_add_post_condition` takes the
//!         `ExprX::Ctor` branch and attaches nothing. This is the single
//!         upstream defect that blocks the derive for APAS's collections,
//!         which are all generic.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_copy_generic_struct.20260922-112727.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_copy_generic_struct

pub mod derive_0913_copy_generic_struct {

    use vstd::prelude::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    #[derive(Clone, Copy)]
    pub struct BoxS<T: Copy> {
        pub value: T,
    }

    pub fn clone_gives_structural_equality<T: Copy>(b: &BoxS<T>) -> (cloned: BoxS<T>)
        ensures
            cloned == *b,
    {
        b.clone()
    }

    } // verus!
} // pub mod derive_0913_copy_generic_struct
