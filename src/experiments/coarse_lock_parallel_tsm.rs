//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: coarse lock + TSM + parallel inside.
//!
//! Demonstrates all three layers of the target Mt architecture:
//! - Layer 1: One coarse RwLock wrapping an interior with data + TSM token.
//! - Layer 2: TSM token inside the lock; predicate ties token to data. Zero assumes.
//! - Layer 3: Parallel operations inside acquire via join() on owned O(1) slices.
//!
//! Uses ArraySeqMtEphSliceS<u64> for O(1) slicing (Arc<Vec> sharing).
//! Parallel reduce and map use HFScheduler join() with named closures.
//! Zero assumes, zero accepts, zero external_body (except HFScheduler join and clone_fn).
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

    use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

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
        pub seq: ArraySeqMtEphSliceS<u64>,
        pub token: Tracked<CollectionSM::count>,
    }

    /// Predicate: token count == sequence length, sequence is well-formed.
    pub ghost struct CollectionInv {
        pub instance: CollectionSM::Instance,
    }

    impl RwLockPredicate<CollectionInterior> for CollectionInv {
        open spec fn inv(self, interior: CollectionInterior) -> bool {
            interior.seq.spec_arrayseqmtephslice_wf()
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

    /// Sequential reduce over a slice-backed sequence.
    fn seq_reduce_u64<F: Fn(&u64, &u64) -> u64>(
        seq: &ArraySeqMtEphSliceS<u64>, f: &F, id: u64,
    ) -> (result: u64)
        requires
            seq.spec_arrayseqmtephslice_wf(),
            forall|x: &u64, y: &u64| #[trigger] f.requires((x, y)),
            obeys_feq_clone::<u64>(),
    {
        let len = seq.length();
        let mut acc: u64 = id;
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len == seq.spec_len(),
                seq.spec_arrayseqmtephslice_wf(),
                forall|x: &u64, y: &u64| #[trigger] f.requires((x, y)),
                obeys_feq_clone::<u64>(),
            decreases len - i,
        {
            let elem = seq.nth_cloned(i);
            acc = f(&acc, &elem);
            i += 1;
        }
        acc
    }

    /// Sequential map over a slice-backed sequence, producing a Vec.
    fn seq_map_u64<F: Fn(&u64) -> u64>(
        seq: &ArraySeqMtEphSliceS<u64>, f: &F,
    ) -> (result: Vec<u64>)
        requires
            seq.spec_arrayseqmtephslice_wf(),
            forall|x: &u64| #[trigger] f.requires((x,)),
            obeys_feq_clone::<u64>(),
        ensures
            result@.len() == seq.spec_len(),
    {
        let len = seq.length();
        let mut v: Vec<u64> = Vec::with_capacity(len);
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len == seq.spec_len(),
                v@.len() == i as int,
                seq.spec_arrayseqmtephslice_wf(),
                forall|x: &u64| #[trigger] f.requires((x,)),
                obeys_feq_clone::<u64>(),
            decreases len - i,
        {
            let elem = seq.nth_cloned(i);
            v.push(f(&elem));
            i += 1;
        }
        v
    }

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
                seq: ArraySeqMtEphSliceS::empty(),
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
                seq: ArraySeqMtEphSliceS::from_vec(v),
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

        /// Read + parallel: reduce via O(1) slice split + join. Zero assumes.
        ///
        /// Acquires read lock, creates two owned slices via O(1) Arc::clone,
        /// releases the lock, then forks reduce over both halves.
        pub fn mt_parallel_reduce<F: Fn(&u64, &u64) -> u64 + Clone + Send + 'static>(
            &self, f: &F, id: u64,
        ) -> (result: u64)
            requires
                self.wf(),
                forall|x: &u64, y: &u64| #[trigger] f.requires((x, y)),
                obeys_feq_clone::<u64>(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let len = interior.seq.length();

            if len == 0 {
                read_handle.release_read();
                return id;
            }

            let mid = len / 2;
            // O(1) slice: Arc::clone + window adjust. Owned, Send, 'static.
            let left_slice = interior.seq.slice(0, mid);
            let right_slice = interior.seq.slice(mid, len - mid);
            // Slices hold their own Arc — safe to release the lock.
            read_handle.release_read();

            // Clone the combiner for each join arm.
            let f1 = clone_fn2(f);
            let f2 = clone_fn2(f);
            let id1 = id;
            let id2 = id;

            // Named closures with explicit ensures (standard 8).
            let fa = move || -> (r: u64)
                requires
                    left_slice.spec_arrayseqmtephslice_wf(),
                    forall|x: &u64, y: &u64| #[trigger] f1.requires((x, y)),
                    obeys_feq_clone::<u64>(),
            {
                seq_reduce_u64(&left_slice, &f1, id1)
            };

            let fb = move || -> (r: u64)
                requires
                    right_slice.spec_arrayseqmtephslice_wf(),
                    forall|x: &u64, y: &u64| #[trigger] f2.requires((x, y)),
                    obeys_feq_clone::<u64>(),
            {
                seq_reduce_u64(&right_slice, &f2, id2)
            };

            let (left_result, right_result) = join(fa, fb);
            f(&left_result, &right_result)
        }

        /// Write + parallel: map via O(1) slice split + join. Zero assumes.
        ///
        /// Acquires write lock, creates two owned slices, forks map over both,
        /// combines results, rebuilds sequence, releases write lock. Count
        /// unchanged (map preserves length), so TSM token stays the same.
        pub fn mt_parallel_map<F: Fn(&u64) -> u64 + Clone + Send + 'static>(
            &self, f: &F,
        )
            requires
                self.wf(),
                forall|x: &u64| #[trigger] f.requires((x,)),
                obeys_feq_clone::<u64>(),
        {
            let (mut interior, write_handle) = self.lock.acquire_write();

            // Save ghost facts from the predicate for the release proof.
            let ghost orig_token_val = interior.token@.value();
            let len = interior.seq.length();
            // Predicate guarantees: orig_token_val == len as nat.

            if len == 0 {
                write_handle.release_write(interior);
                return;
            }

            let mid = len / 2;
            let right_len = len - mid;

            // O(1) slice: Arc::clone + window adjust.
            let left_slice = interior.seq.slice(0, mid);
            let right_slice = interior.seq.slice(mid, right_len);

            let ghost left_len = left_slice.spec_len();
            let ghost right_len_spec = right_slice.spec_len();

            let f1 = clone_fn(f);
            let f2 = clone_fn(f);

            let fa = move || -> (r: Vec<u64>)
                requires
                    left_slice.spec_arrayseqmtephslice_wf(),
                    forall|x: &u64| #[trigger] f1.requires((x,)),
                    obeys_feq_clone::<u64>(),
                ensures
                    r@.len() == left_len,
            {
                seq_map_u64(&left_slice, &f1)
            };

            let fb = move || -> (r: Vec<u64>)
                requires
                    right_slice.spec_arrayseqmtephslice_wf(),
                    forall|x: &u64| #[trigger] f2.requires((x,)),
                    obeys_feq_clone::<u64>(),
                ensures
                    r@.len() == right_len_spec,
            {
                seq_map_u64(&right_slice, &f2)
            };

            let (mut left_vec, right_vec) = join(fa, fb);

            // Combine: append right results to left.
            let rlen = right_vec.len();
            let mut i: usize = 0;
            while i < rlen
                invariant
                    i <= rlen,
                    rlen == right_vec@.len(),
                    rlen as int == right_len_spec,
                    left_vec@.len() == mid as int + i as int,
                decreases rlen - i,
            {
                left_vec.push(right_vec[i]);
                i += 1;
            }

            // Build new sequence from combined results.
            assert(left_vec@.len() == mid as int + rlen as int);
            assert(mid + right_len == len);
            assert(left_vec@.len() == len as int);

            let new_seq = ArraySeqMtEphSliceS::from_vec(left_vec);

            // Replace seq; token unchanged. Predicate still holds:
            // token.value() == orig_token_val == len == new_seq.spec_len().
            interior.seq = new_seq;

            proof {
                assert(interior.seq.spec_arrayseqmtephslice_wf());
                assert(interior.token@.value() == orig_token_val);
                assert(orig_token_val == len as nat);
                assert(interior.seq.spec_len() == len as nat);
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
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, 0);
        assert_eq!(sum, 0);

        // From vec.
        let c = CollectionMt::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(c.mt_size(), 8);

        // Parallel reduce: sum.
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, 0);
        assert_eq!(sum, 36);

        // Parallel map: double each element.
        c.mt_parallel_map(&|x: &u64| -> u64 { *x * 2 });
        assert_eq!(c.mt_size(), 8);

        // Parallel reduce after map: doubled sum.
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, 0);
        assert_eq!(sum, 72);

        // Parallel map: add 1 to each element.
        c.mt_parallel_map(&|x: &u64| -> u64 { *x + 1 });
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, 0);
        assert_eq!(sum, 80);

        // Single element.
        let c = CollectionMt::from_vec(vec![42]);
        assert_eq!(c.mt_size(), 1);
        let sum = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, 0);
        assert_eq!(sum, 42);

        // Parallel map on single element.
        c.mt_parallel_map(&|x: &u64| -> u64 { *x * 10 });
        let val = c.mt_parallel_reduce(&|a: &u64, b: &u64| -> u64 { *a + *b }, 0);
        assert_eq!(val, 420);
    }
}
