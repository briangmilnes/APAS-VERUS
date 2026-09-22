// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether a hand-written `OrdSpecImpl` beside a derived
//! `Ord` specifies the comparison, and what that costs in trust.
//!
//! Question. `derive_0913_ord_obeys_cmp.rs` shows a derived `Ord` gives the
//! prover nothing, because `obeys_cmp_spec` has no definition. `StructuralEq`
//! solves the same problem for equality by emitting a `PartialEqSpecImpl`
//! (`builtin_macros/src/structural.rs`); there is no `StructuralOrd`. Writing
//! the `OrdSpecImpl` and `PartialOrdSpecImpl` by hand is the obvious
//! substitute. Does it compile and specify `cmp`, `lt` and `le`?
//!
//! The answer matters mostly as a warning: the derived `Ord` body is never
//! checked against `cmp_spec`, so an `obeys_cmp_spec() == true` written beside
//! a derived impl is an unchecked claim, of the same standing as an `assume`.
//!
//! RESULT: SUCCEEDS — 1 verified, 0 errors — but the result is trust, not
//!         proof. `obeys_cmp_spec() == true` beside a derived `Ord` asserts
//!         that the derived body agrees with `spec_cmp`, and Verus never
//!         checks it, because the derived impl is ignored. Prefer a
//!         hand-written `Ord` whose body Verus does check.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_ord_with_ordspecimpl.20260922-112613.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_ord_with_ordspecimpl

pub mod derive_0913_ord_with_ordspecimpl {

    use core::cmp::Ordering;

    use vstd::prelude::*;

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

    /// The postcondition `derive_0913_ord_obeys_cmp.rs` could not prove.
    pub fn le_decides_lexicographic_order(a: &RankS, b: &RankS) -> (ordered: bool)
        ensures
            ordered == (spec_cmp(*a, *b) is Less || spec_cmp(*a, *b) is Equal),
    {
        *a <= *b
    }

    } // verus!
} // pub mod derive_0913_ord_with_ordspecimpl
