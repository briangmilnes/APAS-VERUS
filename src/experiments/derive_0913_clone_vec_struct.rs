// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `Clone` gives the prover when the struct
//! owns a `Vec` and the clone is therefore not a copy.
//!
//! Question. Every APAS collection hand-writes `Clone` with
//! `ensures cloned@ == self@` and one `accept` for the inner `Vec::clone`
//! (`src/standards/partial_eq_eq_clone_standard.rs`). If the derived `Clone`
//! carried the same postcondition, 48 hand-written bodies could be deleted.
//! Does it? The struct here is the shape of `ArraySeqStEphS` with the generic
//! parameter fixed to `u64`.
//!
//! RESULT: FAILS — 1 verified, 1 error. Verus warns "does not (yet) support
//!         autoderive Clone impl when the clone is not a copy; continuing, but
//!         without adding a specification", and `cloned@ == s@` is then
//!         unprovable: "datatype is opaque here". The 48 hand-written `Clone`
//!         bodies stand.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_clone_vec_struct.20260922-112333.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_clone_vec_struct

pub mod derive_0913_clone_vec_struct {

    use vstd::prelude::*;

    verus! {

    #[derive(Clone)]
    pub struct SeqS {
        pub seq: Vec<u64>,
    }

    impl View for SeqS {
        type V = Seq<u64>;

        open spec fn view(&self) -> Seq<u64> {
            self.seq@
        }
    }

    /// The postcondition every APAS `Clone` states. If the derived impl carries
    /// a specification, this verifies with an empty proof.
    pub fn clone_preserves_view(s: &SeqS) -> (cloned: SeqS)
        ensures
            cloned@ == s@,
    {
        s.clone()
    }

    } // verus!
} // pub mod derive_0913_clone_vec_struct
