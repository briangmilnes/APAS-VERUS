// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `Clone` gives the prover on an enum.
//!
//! Question. APAS's tree modules are enums over per-variant structs
//! (`src/standards/multi_struct_standard.rs`). A derived `Clone` on an enum
//! expands to a `match` whose arms are constructors, which is the
//! `ExprX::Ctor` shape in `clone_add_post_condition`. Does Verus attach a
//! postcondition to that shape, and does adding `Copy` change the answer? This
//! file asks the first half: a non-`Copy` enum.
//!
//! RESULT: FAILS — 1 verified, 1 error. The warning differs from the struct
//!         case: "autoderive Clone impl does not take the form Verus expects",
//!         because the enum body is a `match`, not a constructor or a read of
//!         `self`. No specification is attached.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_clone_enum.20260922-112334.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_clone_enum

pub mod derive_0913_clone_enum {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone)]
    pub enum NodeS {
        LeafV(u64),
        PairV(u64, u64),
    }

    /// The postcondition a tree module needs from a node clone.
    pub fn clone_preserves_variant(n: &NodeS) -> (cloned: NodeS)
        ensures
            cloned == *n,
    {
        n.clone()
    }

    } // verus!
} // pub mod derive_0913_clone_enum
