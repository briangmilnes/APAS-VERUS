// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether APAS's `TotalOrder` can be discharged for a
//! struct whose `Ord` is derived.
//!
//! Question. `src/vstdplus/total_order.rs` carries a comment saying that user
//! types without an `OrdSpecImpl` fall back to the trait's default
//! `cmp_spec_less_implies_le` and `cmp_spec_greater_implies_le` bodies, which
//! `assume`. `derive_0913_ord_with_ordspecimpl.rs` shows that a hand-written
//! `OrdSpecImpl` beside a derived `Ord` does specify `cmp`. Does that make all
//! six `TotalOrder` members provable with empty proof bodies, so the two
//! assuming defaults are never taken?
//!
//! RESULT: SUCCEEDS — 197 verified, 0 errors, of which 179 are the `deps`
//!         baseline and 18 are this file. All six `TotalOrder` members take
//!         empty proof bodies, including the two that would otherwise fall
//!         back to the trait's assuming defaults, so
//!         `src/vstdplus/total_order.rs`'s comment about user types is out of
//!         date. The `OrdSpecImpl` remains trusted, per
//!         `derive_0913_ord_with_ordspecimpl.rs`.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_ord_totalorder_bridge.20260922-112641.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_ord_totalorder_bridge deps

pub mod derive_0913_ord_totalorder_bridge {

    use core::cmp::Ordering;

    use vstd::prelude::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus! {

    use vstd::std_specs::cmp::{OrdSpecImpl, PartialOrdSpecImpl};

    #[derive(PartialEq, Eq, PartialOrd, Ord, StructuralEq)]
    pub struct RankS {
        pub major: u64,
        pub minor: u64,
    }

    pub open spec fn spec_cmp(a: RankS, b: RankS) -> Ordering {
        if a.major < b.major {
            Ordering::Less
        } else if a.major > b.major {
            Ordering::Greater
        } else if a.minor < b.minor {
            Ordering::Less
        } else if a.minor > b.minor {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    impl PartialOrdSpecImpl for RankS {
        open spec fn obeys_partial_cmp_spec() -> bool {
            true
        }

        open spec fn partial_cmp_spec(&self, other: &Self) -> Option<Ordering> {
            Some(spec_cmp(*self, *other))
        }
    }

    impl OrdSpecImpl for RankS {
        open spec fn obeys_cmp_spec() -> bool {
            true
        }

        open spec fn cmp_spec(&self, other: &Self) -> Ordering {
            spec_cmp(*self, *other)
        }
    }

    impl TotalOrder for RankS {
        open spec fn le(self, other: Self) -> bool {
            self.major < other.major || (self.major == other.major && self.minor <= other.minor)
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        proof fn total(x: Self, y: Self) {
        }

        fn cmp(&self, other: &Self) -> (c: Ordering) {
            Ord::cmp(self, other)
        }

        proof fn cmp_spec_less_implies_le(a: Self, b: Self) {
        }

        proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {
        }
    }

    } // verus!
} // pub mod derive_0913_ord_totalorder_bridge
