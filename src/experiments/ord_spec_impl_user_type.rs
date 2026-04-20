// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//!
//! Experiment: Can we implement OrdSpecImpl for a user-defined type?
//! RESULT: FAILS — Verus panics at vir/src/ast_util.rs:734:
//!   "mask_spec_or_default should not be called for TraitMethodImpl"
//!
//! The workaround is to keep Ord::cmp as external_body and assume the
//! bridge lemmas. This experiment documents the Verus bug.

pub mod ord_spec_impl_user_type {

    use std::cmp::Ordering;
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpecImpl;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdSpecImpl;

    verus! {

    #[derive(Eq, PartialEq)]
    pub struct MyPair {
        pub first: u64,
        pub second: u64,
    }

    impl View for MyPair {
        type V = Self;
        open spec fn view(&self) -> Self { *self }
    }

    // This is what we WANT to write — but it crashes Verus.
    #[cfg(verus_keep_ghost)]
    impl OrdSpecImpl for MyPair {
        open spec fn obeys_cmp() -> bool { true }
        open spec fn cmp_spec(&self, other: &Self) -> Ordering {
            if self.first < other.first { Ordering::Less }
            else if self.first > other.first { Ordering::Greater }
            else if self.second < other.second { Ordering::Less }
            else if self.second > other.second { Ordering::Greater }
            else { Ordering::Equal }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialOrdSpecImpl for MyPair {
        open spec fn obeys_partial_cmp_spec() -> bool { true }
        open spec fn partial_cmp_spec(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp_spec(other))
        }
    }

    impl Ord for MyPair {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.first < other.first { Ordering::Less }
            else if self.first > other.first { Ordering::Greater }
            else if self.second < other.second { Ordering::Less }
            else if self.second > other.second { Ordering::Greater }
            else { Ordering::Equal }
        }
    }

    impl PartialOrd for MyPair {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(Ord::cmp(self, other))
        }
    }

    } // verus!
}
