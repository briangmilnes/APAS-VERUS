//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! RwLock Standard: using vstd's RwLock with a real predicate.
//!
//! vstd's RwLock gives verified concurrent access to shared mutable state.
//! The proof bracket: acquire guarantees the invariant holds, release demands
//! you re-establish it. This is genuine verified concurrency — not a `true`
//! predicate that proves nothing.
//!
//! This standard shows:
//! - Defining a predicate struct with a ghost field for construction-time context.
//! - Implementing `RwLockPredicate` with a real invariant.
//! - acquire_write / modify / prove invariant maintained / release_write.
//! - acquire_read / borrow / release_read.
//!
//! Caveats Verus does NOT prove:
//! - Deadlock freedom (you can forget to release).
//! - Auto-release (handles are not RAII — you must call release manually).
//! - Arc::clone correctness (unspecified in vstd).
//!
//! Anti-pattern: `inv(self, v) -> bool { true }` proves nothing — any value
//! satisfies the predicate, so acquire tells you nothing useful.
//!
//! References:
//! - src/Chap37/BSTPlainMtEph.rs (gold standard — no external_body)
//! - src/Chap18/ArraySeqMtEph.rs (ghost fields in predicate, complex invariant)
//! - vstd::rwlock (RwLock is itself a TSM internally)

//  Table of Contents
//	1. module
//	2. imports
//	4. type definitions
//	8. traits
//	9. impls
//	13. derive impls outside verus!

//		1. module

pub mod rwlock_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use vstd::rwlock::{RwLock, RwLockPredicate, ReadHandle, WriteHandle};

    //		4. type definitions

    /// Lock predicate with a ghost field carrying construction-time context.
    /// Naming follows the ModuleInv convention: module is RwLockStandard, struct is RwLockStandardInv.
    pub struct RwLockStandardInv {
        pub ghost max_val: u64,
    }

    /// The locked value is a plain u64. The predicate constrains it to <= max_val.
    /// BoundedCounter wraps the lock and stores max_val as an exec field for runtime
    /// comparison, linked to the predicate's ghost field by spec_rwlockstandard_wf.
    pub struct BoundedCounter {
        pub max_val: u64,
        pub lock: RwLock<u64, RwLockStandardInv>,
    }

    //		8. traits

    pub trait BoundedCounterTrait: Sized {
        /// The maximum value this counter can hold (from the predicate's ghost field).
        spec fn spec_max_val(&self) -> u64;

        /// Well-formedness: the exec max_val field matches the predicate's ghost max_val.
        spec fn spec_rwlockstandard_wf(&self) -> bool;

        fn new(max_val: u64, init_val: u64) -> (s: Self)
            requires
                init_val <= max_val,
            ensures
                s.spec_max_val() == max_val,
                s.spec_rwlockstandard_wf(),
        ;

        /// Acquire write lock, increment if below max, release.
        /// The proof bracket: acquire gives val <= max, release demands new_val <= max.
        fn try_increment(&self) -> (incremented: bool)
            requires
                self.spec_rwlockstandard_wf(),
        ;

        /// Acquire read lock, borrow value, release.
        fn read_val(&self) -> (val: u64)
            requires
                self.spec_rwlockstandard_wf(),
            ensures
                val <= self.spec_max_val(),
        ;
    }

    //		9. impls

    impl RwLockPredicate<u64> for RwLockStandardInv {
        open spec fn inv(self, val: u64) -> bool {
            val <= self.max_val
        }
    }

    impl BoundedCounterTrait for BoundedCounter {
        open spec fn spec_max_val(&self) -> u64 {
            self.lock.pred().max_val
        }

        open spec fn spec_rwlockstandard_wf(&self) -> bool {
            self.max_val == self.lock.pred().max_val
        }

        fn new(max_val: u64, init_val: u64) -> (s: Self) {
            let ghost pred = RwLockStandardInv { max_val: max_val };
            BoundedCounter {
                max_val: max_val,
                lock: RwLock::new(init_val, Ghost(pred)),
            }
        }

        fn try_increment(&self) -> (incremented: bool) {
            // Acquire: Verus guarantees self.lock.inv(val), i.e. val <= max_val.
            let (val, write_handle) = self.lock.acquire_write();

            if val < self.max_val {
                // Proof: val < max_val implies val + 1 <= max_val, so inv(val + 1) holds.
                write_handle.release_write(val + 1);
                true
            } else {
                // Value unchanged — inv trivially maintained.
                write_handle.release_write(val);
                false
            }
        }

        fn read_val(&self) -> (val: u64) {
            // Acquire: Verus guarantees self.lock.inv(read_handle.view()).
            let read_handle = self.lock.acquire_read();
            // Borrow: *ref == read_handle.view(), and inv says it's <= max_val.
            let val = *read_handle.borrow();
            read_handle.release_read();
            val
        }
    }

    } // verus!

    //		13. derive impls outside verus!

    impl Debug for RwLockStandardInv {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "RwLockStandardInv")
        }
    }

    impl Display for RwLockStandardInv {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "RwLockStandardInv")
        }
    }

    impl Debug for BoundedCounter {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "BoundedCounter(max={})", self.max_val)
        }
    }

    impl Display for BoundedCounter {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "BoundedCounter(max={})", self.max_val)
        }
    }
} // pub mod rwlock_standard
