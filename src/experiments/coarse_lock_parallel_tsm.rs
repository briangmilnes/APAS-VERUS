//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: coarse lock + TSM + parallel inside.
//!
//! Demonstrates all three layers of the target Mt architecture:
//! - Layer 1: One coarse RwLock wrapping an interior with data + TSM token.
//! - Layer 2: TSM token inside the lock; predicate ties token to data. Zero assumes.
//! - Layer 3: After acquire, call the Mt type's OWN parallel operations on owned data.
//!
//! Uses ArraySeqMtEphS<u64> as the inner Mt type. After acquiring the lock, the
//! experiment calls ArraySeqMtEphTrait::reduce and ArraySeqMtEphTrait::map directly —
//! the Mt type handles parallelism internally (D&C via join). The experiment does NOT
//! manually split, loop, or re-implement these operations.
//!
//! This is the key architectural point from architecture-coarse-lock-parallel-mt.md:
//! when M1 stores M2 inside its lock, M1's locked trait calls M2's unlocked trait
//! directly on the owned M2 data.
//!
//! Zero assumes, zero accepts, zero external_body (except inside the Mt type's internals).
//!
//! DO NOT register in lib.rs — experiments stay commented out.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 6. spec fns
// 9. impls

// 1. module

pub mod coarse_lock_parallel_tsm {

    use vstd::prelude::*;
    use vstd::rwlock::{RwLock, RwLockPredicate};

    use verus_state_machines_macros::tokenized_state_machine;

    // TSM: tracks sequence element count. Outside verus! (proc macro).
    tokenized_state_machine!(
        CollectionSM {
            fields {
                #[sharding(variable)]
                pub count: nat,
            }

            #[invariant]
            pub fn the_invariant(&self) -> bool { true }

            init!{
                initialize(n: nat) {
                    init count = n;
                }
            }

            // For write operations that preserve count (map).
            transition!{
                tr_noop() {
                }
            }

            #[inductive(initialize)]
            fn initialize_inductive(post: Self, n: nat) { }

            #[inductive(tr_noop)]
            fn tr_noop_inductive(pre: Self, post: Self) { }
        }
    );

    verus! {

    // 2. imports

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::monoid::monoid::*;

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    // 4. type definitions

    /// Lock interior: concrete sequence + ghost count token.
    pub struct CollectionInterior {
        pub seq: ArraySeqMtEphS<u64>,
        pub token: Tracked<CollectionSM::count>,
    }

    /// Predicate: token count == sequence length, sequence is well-formed.
    pub ghost struct CollectionInv {
        pub instance: CollectionSM::Instance,
    }

    impl RwLockPredicate<CollectionInterior> for CollectionInv {
        open spec fn inv(self, interior: CollectionInterior) -> bool {
            interior.seq.spec_arrayseqmteph_wf()
            && interior.token@.value() == interior.seq.spec_len()
            && interior.token@.instance_id() == self.instance.id()
        }
    }

    /// Coarse-locked collection with TSM tracking. Zero assumes.
    pub struct CollectionMt {
        pub lock: RwLock<CollectionInterior, CollectionInv>,
        pub inst: Tracked<CollectionSM::Instance>,
    }

    // 6. spec fns

    // 9. impls

    impl CollectionMt {
        pub open spec fn wf(&self) -> bool {
            self.lock.pred().instance == self.inst@
        }

        /// Construct empty collection.
        pub fn new_empty() -> (s: Self)
            ensures s.wf(),
        {
            let tracked (
                Tracked(instance),
                Tracked(count_token),
            ) = CollectionSM::Instance::initialize(0);

            let interior = CollectionInterior {
                seq: ArraySeqMtEphS::empty(),
                token: Tracked(count_token),
            };

            CollectionMt {
                lock: RwLock::new(interior, Ghost(CollectionInv { instance })),
                inst: Tracked(instance),
            }
        }

        /// Construct from a Vec.
        pub fn from_vec(v: Vec<u64>) -> (s: Self)
            ensures s.wf(),
        {
            let len = v.len();
            let tracked (
                Tracked(instance),
                Tracked(count_token),
            ) = CollectionSM::Instance::initialize(len as nat);

            let interior = CollectionInterior {
                seq: ArraySeqMtEphS::from_vec(v),
                token: Tracked(count_token),
            };

            CollectionMt {
                lock: RwLock::new(interior, Ghost(CollectionInv { instance })),
                inst: Tracked(instance),
            }
        }

        /// Read: return the length. Zero assumes.
        pub fn mt_size(&self) -> (n: usize)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let n = interior.seq.length();
            read_handle.release_read();
            n
        }

        /// Read + parallel: reduce via the Mt type's own parallel reduce.
        ///
        /// Acquires read lock, calls ArraySeqMtEphTrait::reduce on the interior
        /// data. The Mt type handles parallelism internally (D&C with join).
        /// This is the architectural pattern: call the inner type's unlocked
        /// operations after acquire.
        pub fn mt_parallel_reduce<F: Fn(&u64, &u64) -> u64 + Clone + Send + Sync + 'static>(
            &self, f: &F, Ghost(spec_f): Ghost<spec_fn(u64, u64) -> u64>, id: u64,
        ) -> (result: u64)
            requires
                self.wf(),
                obeys_feq_clone::<u64>(),
                spec_monoid(spec_f, id),
                forall|x: &u64, y: &u64| #[trigger] f.requires((x, y)),
                forall|x: u64, y: u64, ret: u64| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            // Call the Mt type's own parallel reduce — internally uses D&C + join.
            let result = <ArraySeqMtEphS<u64> as ArraySeqMtEphTrait<u64>>::reduce(&interior.seq, f, Ghost(spec_f), id);
            read_handle.release_read();
            result
        }

        /// Write + parallel: map via the Mt type's own parallel map.
        ///
        /// Acquires write lock, calls ArraySeqMtEphTrait::map on the interior
        /// data. Map preserves length, so the TSM token stays unchanged.
        /// The Mt type handles parallelism internally (D&C with join).
        pub fn mt_parallel_map<F: Fn(&u64) -> u64 + Clone + Send + Sync + 'static>(
            &self, f: &F,
        )
            requires
                self.wf(),
                obeys_feq_clone::<u64>(),
                forall|x: &u64| #[trigger] f.requires((x,)),
        {
            let (mut interior, write_handle) = self.lock.acquire_write();
            let ghost orig_len = interior.seq.spec_len();
            // Call the Mt type's own parallel map — internally uses D&C + join.
            let new_seq = <ArraySeqMtEphS<u64> as ArraySeqMtEphTrait<u64>>::map(&interior.seq, f);
            // Map preserves length: new_seq.spec_len() == orig_len.
            interior.seq = new_seq;
            proof {
                // Predicate: token value == seq length, both == orig_len.
                assert(interior.seq.spec_arrayseqmteph_wf());
                assert(interior.seq.spec_len() == orig_len);
                assert(interior.token@.value() == orig_len);
                assert(interior.token@.value() == interior.seq.spec_len());
            }
            write_handle.release_write(interior);
        }
    }

    } // verus!

    #[test]
    fn test_coarse_lock_parallel_tsm() {
        use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;
        set_parallelism(2);

        // Empty collection.
        let c = CollectionMt::new_empty();
        assert_eq!(c.mt_size(), 0);

        // Reduce on empty returns identity.
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, Ghost::assume_new(), 0);
        assert_eq!(sum, 0);

        // From vec.
        let c = CollectionMt::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(c.mt_size(), 8);

        // Parallel reduce: sum.
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, Ghost::assume_new(), 0);
        assert_eq!(sum, 36);

        // Parallel map: double each element.
        c.mt_parallel_map(&|x: &u64| -> u64 { *x * 2 });
        assert_eq!(c.mt_size(), 8);

        // Parallel reduce after map: doubled sum.
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, Ghost::assume_new(), 0);
        assert_eq!(sum, 72);

        // Parallel map: add 1 to each element.
        c.mt_parallel_map(&|x: &u64| -> u64 { *x + 1 });
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, Ghost::assume_new(), 0);
        assert_eq!(sum, 80);

        // Single element.
        let c = CollectionMt::from_vec(vec![42]);
        assert_eq!(c.mt_size(), 1);
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, Ghost::assume_new(), 0);
        assert_eq!(sum, 42);

        // Parallel map on single element.
        c.mt_parallel_map(&|x: &u64| -> u64 { *x * 10 });
        let val = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, Ghost::assume_new(), 0);
        assert_eq!(val, 420);
    }
}
