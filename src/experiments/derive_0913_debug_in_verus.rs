// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether `#[derive(Debug)]` may sit inside `verus!`.
//!
//! Question. APAS puts every `Debug` impl in section 14, outside `verus!`, and
//! hand-writes 245 of them. `automatic_derive.rs` maps `RustItem::Debug` to
//! `AutomaticDeriveAction::Ignore`, so the derived impl should pass through
//! untouched. Does a `#[derive(Debug)]` inside `verus!` compile on a struct, an
//! enum, and a generic struct, and is the resulting impl usable from ordinary
//! Rust outside the `verus!` block?
//!
//! RESULT: SUCCEEDS — 1 verified, 0 errors, no warning. `#[derive(Debug)]`
//!         compiles inside `verus!` on a struct, an enum, and a generic
//!         struct, and the impl is reachable from `format!` outside the block.
//!         The 245 hand-written `Debug` impls in section 14 are replaceable by
//!         one derive line in section 12.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_debug_in_verus.20260922-112547.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_debug_in_verus

pub mod derive_0913_debug_in_verus {

    use vstd::prelude::*;

    verus! {

    #[derive(Debug)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    #[derive(Debug)]
    pub enum NodeS {
        LeafV(u64),
        PairV(u64, u64),
    }

    #[verifier::reject_recursive_types(T)]
    #[derive(Debug)]
    pub struct BoxS<T> {
        pub value: T,
    }

    /// A derived `Debug` alongside a verified function on the same type.
    pub fn sum(p: &PointS) -> (total: u64)
        requires
            p.x + p.y <= u64::MAX,
        ensures
            total == p.x + p.y,
    {
        p.x + p.y
    }

    } // verus!

    // 14. derive impls outside verus!

    /// The derived impls are reachable from ordinary Rust.
    pub fn render(p: &PointS, n: &NodeS, b: &BoxS<u64>) -> String {
        format!("{:?} {:?} {:?}", p, n, b)
    }
} // pub mod derive_0913_debug_in_verus
