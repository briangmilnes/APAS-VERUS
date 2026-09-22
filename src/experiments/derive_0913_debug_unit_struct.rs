// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar and Guy Blelloch and Brian Milnes

//! Experiment (r217): whether `#[derive(Debug)]` on a unit struct inside
//! `verus!` prints exactly the type name, and whether it reaches a unit struct
//! that also carries `RwLockPredicate`.
//!
//! Question. `derive_0913_debug_in_verus.rs` shows `#[derive(Debug)]`
//! compiling inside `verus!` on a struct, an enum and a generic struct. The 86
//! constant-string `Debug` bodies `docs/DerivesInsideVerus.md` §5 counts are a
//! different shape: 61 of them are on a unit struct, `pub struct X;`, and
//! their body is `write!(f, "X")`. For those, and only those, a derive prints
//! the same text, so no test expectation moves. That claim needs measuring
//! rather than assuming, because `{:?}` on a derived unit struct could print
//! `X` or something else, and because many of the 61 are `RwLockPredicate`
//! invariant structs, which the derive must not disturb.
//!
//! RESULT: SUCCEEDS — 1 verified, 0 errors. `#[derive(Debug)]` on a unit
//!         struct compiles inside `verus!`, including on a unit struct that
//!         also implements `RwLockPredicate`, and the derived impl is
//!         reachable from `format!` outside the block. The printed text was
//!         measured separately in plain Rust
//!         (`scratch/r217-unit-struct-debug.rs`, compiled with `rustc -O`):
//!         a derived unit struct prints exactly its type name under both
//!         `{:?}` and `{:#?}`, character for character the same string as
//!         `write!(f, "X")`. The 58 non-`Example` unit structs among the 86
//!         constant-string bodies are therefore replaceable with no change in
//!         printed output and no test expectation to update.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_debug_unit_struct.20260922-131556.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_debug_unit_struct

pub mod derive_0913_debug_unit_struct {

    use vstd::prelude::*;
    use vstd::rwlock::RwLockPredicate;

    verus! {

    /// The shape of `src/Chap54/BFSStEph.rs`'s `BFSStEph`: a unit struct that
    /// carries a chapter's trait impl and holds no data.
    #[derive(Debug)]
    pub struct AlgorithmS;

    /// The shape of `src/Chap05/SetMtEph.rs`'s `SetMtEphInv`: a unit struct
    /// that is also an `RwLockPredicate`.
    #[derive(Debug)]
    pub struct LockInvS;

    impl RwLockPredicate<Vec<u64>> for LockInvS {
        open spec fn inv(self, v: Vec<u64>) -> bool {
            v@.len() >= 0
        }
    }

    pub fn sum_is_zero_when_empty(v: &Vec<u64>) -> (total: u64)
        requires
            v@.len() == 0,
        ensures
            total == 0,
    {
        let _ = v;
        0
    }

    } // verus!

    // 14. derive impls outside verus!

    /// The derived impls are reachable from ordinary Rust, and print the same
    /// text the hand-written `write!(f, "AlgorithmS")` bodies printed.
    pub fn render() -> (String, String, String) {
        (
            format!("{:?}", AlgorithmS),
            format!("{:#?}", AlgorithmS),
            format!("{:?}", LockInvS),
        )
    }

    #[test]
    fn derived_unit_struct_debug_prints_the_type_name() {
        let (plain, alternate, lock) = render();
        assert_eq!(plain, "AlgorithmS");
        assert_eq!(alternate, "AlgorithmS");
        assert_eq!(lock, "LockInvS");
    }
} // pub mod derive_0913_debug_unit_struct
