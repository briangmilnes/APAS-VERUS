//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 50: Optimal Binary Search Tree - ephemeral, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 8c. traits
//	Section 9c. impls
//	Section 4d. type definitions
//	Section 5d. view impls
//	Section 9d. impls
//	Section 4e. type definitions
//	Section 11b. top level coarse locking
//	Section 11c. top level coarse locking
//	Section 12a. derive impls in verus!
//	Section 12d. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!
//	Section 14e. derive impls outside verus!

//		Section 1. module

pub mod OptBinSearchTreeMtEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::Arc;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap30::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    //		Section 4b. type definitions


        #[verifier::reject_recursive_types(T)]
        pub struct OptBSTMtEphKeysInv<T: MtVal> {
            pub ghost expected_keys: Seq<KeyProb<T>>,
        }

    //		Section 4c. type definitions


        pub struct OptBSTMtEphMemoInv;

    //		Section 8c. traits


    pub trait OBSTMtEphTrait<T: MtVal>: Sized + View<V = OBSTMtEphV<T>> {
        spec fn spec_optbinsearchtreemteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
            ensures empty@.keys.len() == 0, empty.spec_optbinsearchtreemteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self)
            requires keys@.len() == probs@.len(),
            ensures constructed@.keys.len() == keys@.len(), constructed.spec_optbinsearchtreemteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self)
            ensures constructed@.keys =~= key_probs@, constructed.spec_optbinsearchtreemteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n lg n)
        fn optimal_cost(&mut self) -> (cost: Probability) where T: Send + Sync + 'static
            requires old(self).spec_optbinsearchtreemteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn keys(&self) -> (keys: Vec<KeyProb<T>>)
            requires self.spec_optbinsearchtreemteph_wf(),
            ensures keys@ =~= self@.keys;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>)
            requires index < old(self)@.keys.len(), old(self).spec_optbinsearchtreemteph_wf(),
            ensures
                self@.keys =~= old(self)@.keys.update(index as int, key_prob),
                self.spec_optbinsearchtreemteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn update_prob(&mut self, index: usize, prob: Probability)
            requires index < old(self)@.keys.len(), old(self).spec_optbinsearchtreemteph_wf(),
            ensures
                self@.keys.len() == old(self)@.keys.len(),
                self.spec_optbinsearchtreemteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_keys(&self) -> (count: usize)
            requires self.spec_optbinsearchtreemteph_wf(),
            ensures count == self@.keys.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn clear_memo(&mut self)
            requires old(self).spec_optbinsearchtreemteph_wf(),
            ensures self@.keys =~= old(self)@.keys, self.spec_optbinsearchtreemteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    //		Section 9c. impls


    /// - Alg Analysis: APAS (Ch50 Alg 50.2): Work O(n^3), Span O(n lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n lg n) — parallel min reduction over split points via join, O(1) prefix sum lookup
    fn obst_rec(
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtEphMemoInv>>,
        prefix_sums: &Arc<Vec<Probability>>,
        n: usize,
        i: usize,
        l: usize,
    ) -> (cost: Probability)
        requires
            i + l <= n,
            memo.pred() == (OptBSTMtEphMemoInv),
            (*prefix_sums)@.len() == n + 1,
        ensures true,
        decreases l,
    {
        // Memo lookup.
        {
            let rwlock = arc_deref(memo);
            let handle = rwlock.acquire_read();
            let cached = match handle.borrow().get(&Pair(i, l)) {
                Some(v) => Some(*v),
                None => None,
            };
            handle.release_read();
            if let Some(cost) = cached {
                return cost;
            }
        }

        let cost = if l == 0 {
            Probability::zero()
        } else {
            // Probability sum from prefix sums: O(1).
            let ps = arc_deref(prefix_sums);
            let prob_sum = ps[i + l] - ps[i];

            // Parallel min reduction over split points: O(lg l) span.
            let min_cost = parallel_min_split_cost(memo, prefix_sums, n, i, l, 0, l);

            prob_sum + min_cost
        };

        // Memo store.
        {
            let rwlock = arc_deref(memo);
            let (mut memo_val, write_handle) = rwlock.acquire_write();
            memo_val.insert(Pair(i, l), cost);
            write_handle.release_write(memo_val);
        }

        cost
    }

    /// Parallel divide-and-conquer min reduction over split points k in [lo, hi).
    /// Returns the minimum of obst_rec(i, k) + obst_rec(i+k+1, l-k-1) for k in [lo, hi).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(l), Span O(lg l)
    fn parallel_min_split_cost(
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtEphMemoInv>>,
        prefix_sums: &Arc<Vec<Probability>>,
        n: usize,
        i: usize,
        l: usize,
        lo: usize,
        hi: usize,
    ) -> (cost: Probability)
        requires
            lo < hi,
            hi <= l,
            l > 0,
            i + l <= n,
            memo.pred() == (OptBSTMtEphMemoInv),
            (*prefix_sums)@.len() == n + 1,
        ensures true,
        decreases l, hi - lo,
    {
        if hi - lo == 1 {
            let left_cost = obst_rec(memo, prefix_sums, n, i, lo);
            let right_cost = obst_rec(memo, prefix_sums, n, i + lo + 1, l - lo - 1);
            left_cost + right_cost
        } else {
            let mid = lo + (hi - lo) / 2;
            let memo1 = clone_arc_rwlock(memo);
            let ps1 = clone_arc(prefix_sums);
            let memo2 = clone_arc_rwlock(memo);
            let ps2 = clone_arc(prefix_sums);

            let f1 = move || -> (r: Probability)
                requires
                    lo < mid,
                    mid <= l,
                    l > 0,
                    i + l <= n,
                    memo1.pred() == (OptBSTMtEphMemoInv),
                    (*ps1)@.len() == n + 1,
                ensures true
            {
                parallel_min_split_cost(&memo1, &ps1, n, i, l, lo, mid)
            };
            let f2 = move || -> (r: Probability)
                requires
                    mid < hi,
                    hi <= l,
                    l > 0,
                    i + l <= n,
                    memo2.pred() == (OptBSTMtEphMemoInv),
                    (*ps2)@.len() == n + 1,
                ensures true
            {
                parallel_min_split_cost(&memo2, &ps2, n, i, l, mid, hi)
            };
            let (left_min, right_min) = join(f1, f2);
            if left_min <= right_min { left_min } else { right_min }
        }
    }

    //		Section 4d. type definitions


    /// Ephemeral multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTMtEphS<T: MtVal> {
        pub keys: Arc<RwLock<Vec<KeyProb<T>>, OptBSTMtEphKeysInv<T>>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtEphMemoInv>>,
        pub ghost_keys: Ghost<Seq<KeyProb<T>>>,
    }

    //		Section 5d. view impls


    impl<T: MtVal> View for OBSTMtEphS<T> {
        type V = OBSTMtEphV<T>;
        open spec fn view(&self) -> Self::V {
            OBSTMtEphV { keys: self.ghost_keys@ }
        }
    }

    //		Section 9d. impls


    impl<T: MtVal> OBSTMtEphTrait<T> for OBSTMtEphS<T> {
        open spec fn spec_optbinsearchtreemteph_wf(&self) -> bool {
            &&& self.keys.pred().expected_keys =~= self.ghost_keys@
            &&& self.memo.pred() == (OptBSTMtEphMemoInv)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate empty Vec, two Arc<RwLock> wrappers
        fn new() -> (empty: Self) {
            // Veracity: NEEDED proof block
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: new_arc_rwlock(Vec::new(), Ghost(OptBSTMtEphKeysInv { expected_keys: Seq::empty() })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(Seq::empty()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — iterate keys/probs to build KeyProb vec
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self) {
            let mut key_probs: Vec<KeyProb<T>> = Vec::new();
            let mut idx: usize = 0;
            while idx < keys.len()
                invariant
                    idx <= keys@.len(),
                    keys@.len() == probs@.len(),
                    key_probs@.len() == idx as int,
                decreases keys@.len() - idx,
            {
                key_probs.push(KeyProb { key: keys[idx].clone(), prob: probs[idx] });
                idx += 1;
            }
            // Veracity: NEEDED proof block
            let ghost gk = key_probs@;
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: new_arc_rwlock(key_probs, Ghost(OptBSTMtEphKeysInv { expected_keys: gk })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(gk),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — wrap existing vec in Arc<RwLock>
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self) {
            // Veracity: NEEDED proof block
            let ghost gk = key_probs@;
            let _len = key_probs.len();
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: new_arc_rwlock(key_probs, Ghost(OptBSTMtEphKeysInv { expected_keys: gk })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(gk),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n lg n) — precomputes prefix sums, calls obst_rec with parallel min reduction
        fn optimal_cost(&mut self) -> (cost: Probability) where T: Send + Sync + 'static {
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let keys_ref = handle.borrow();
            let keys_len = keys_ref.len();

            if keys_len == 0 {
                handle.release_read();
                return Probability::zero();
            }

            // Precompute prefix sums of probabilities: prefix_sums[k] = sum(prob[0..k]).
            let mut prefix: Vec<Probability> = Vec::new();
            prefix.push(Probability::zero());
            let mut idx: usize = 0;
            while idx < keys_len
                invariant
                    idx <= keys_len,
                    prefix@.len() == idx as int + 1,
                    keys_len == keys_ref@.len(),
                decreases keys_len - idx,
            {
                let new_sum = prefix[idx] + keys_ref[idx].prob;
                prefix.push(new_sum);
                idx = idx + 1;
            }
            handle.release_read();

            let prefix_sums = Arc::new(prefix);

            // Clear memo.
            {
                let memo_arc = clone_arc_rwlock(&self.memo);
                let rwlock = arc_deref(&memo_arc);
                let (mut memo, write_handle) = rwlock.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }
            obst_rec(&self.memo, &prefix_sums, keys_len, 0, keys_len)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec under read lock
        fn keys(&self) -> (keys: Vec<KeyProb<T>>) {
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let borrowed = handle.borrow();
            let keys = borrowed.clone();
            handle.release_read();
            keys
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone keys vec, update one element, rebuild Arc<RwLock>
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            let ghost kp = key_prob;
            let ghost old_keys = self.ghost_keys@;
            let ghost new_keys_ghost = old_keys.update(index as int, kp);
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let borrowed = handle.borrow();
            // Veracity: NEEDED proof block
            let mut keys = borrowed.clone();
            handle.release_read();
            keys.set(index, key_prob);
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            *self = OBSTMtEphS {
                keys: new_arc_rwlock(keys, Ghost(OptBSTMtEphKeysInv { expected_keys: new_keys_ghost })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(new_keys_ghost),
            };
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone keys vec, update one probability, rebuild Arc<RwLock>
        fn update_prob(&mut self, index: usize, prob: Probability) {
            let ghost old_keys = self.ghost_keys@;
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let borrowed = handle.borrow();
            let mut keys = borrowed.clone();
            let new_kp = KeyProb { key: keys[index].key.clone(), prob };
            let ghost new_keys_ghost = old_keys.update(index as int, new_kp);
            keys.set(index, new_kp);
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            *self = OBSTMtEphS {
                keys: new_arc_rwlock(keys, Ghost(OptBSTMtEphKeysInv { expected_keys: new_keys_ghost })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(new_keys_ghost),
            };
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read lock, return Vec len
        fn num_keys(&self) -> (count: usize) {
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let count = handle.borrow().len();
            handle.release_read();
            count
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m) — clear memo hash map with m entries under write lock
        fn clear_memo(&mut self) {
            let memo_arc = self.memo.clone();
            let rwlock = arc_deref(&memo_arc);
            let (mut memo, write_handle) = rwlock.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read lock, return hash map len
        fn memo_size(&self) -> (count: usize) {
            let rwlock = arc_deref(&self.memo);
            let handle = rwlock.acquire_read();
            let count = handle.borrow().len();
            handle.release_read();
            count
        }
    }

    //		Section 4e. type definitions


    #[verifier::reject_recursive_types(T)]
    pub ghost struct OBSTMtEphV<T: MtVal> {
        pub keys: Seq<KeyProb<T>>,
    }

    //		Section 11b. top level coarse locking


        impl<T: MtVal> RwLockPredicate<Vec<KeyProb<T>>> for OptBSTMtEphKeysInv<T> {
            open spec fn inv(self, v: Vec<KeyProb<T>>) -> bool {
                v@ =~= self.expected_keys
            }
        }

    //		Section 11c. top level coarse locking


        impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, Probability>> for OptBSTMtEphMemoInv {
            open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, Probability>) -> bool {
                v@.dom().finite()
            }
        }

    //		Section 12a. derive impls in verus!


    impl<T: MtVal> Clone for KeyProb<T> {
        // Veracity: NEEDED proof block
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = KeyProb { key: self.key.clone(), prob: self.prob };
            proof { assume(cloned == *self); }
            cloned
        }
    }

    impl<T: MtVal> Eq for KeyProb<T> {}

    //		Section 12d. derive impls in verus!


    impl<T: MtVal> Clone for OBSTMtEphS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            // Veracity: NEEDED proof block
            let cloned = OBSTMtEphS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
                ghost_keys: Ghost(self.ghost_keys@),
            };
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: MtVal> PartialEqSpecImpl for OBSTMtEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: MtVal> PartialEq for OBSTMtEphS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let self_rwlock = arc_deref(&self.keys);
            // Veracity: NEEDED proof block
            let other_rwlock = arc_deref(&other.keys);
            let self_handle = self_rwlock.acquire_read();
            let other_handle = other_rwlock.acquire_read();
            let equal = *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: MtVal> Eq for OBSTMtEphS<T> {}
    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! OBSTMtEphLit {
        (keys: [$($k:expr),* $(,)?], probs: [$($p:expr),* $(,)?]) => {
            $crate::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::OBSTMtEphS::from_keys_probs(
                vec![$($k),*],
                vec![$(<$crate::Chap30::Probability::Probability::Probability as $crate::Chap30::Probability::Probability::ProbabilityTrait>::new($p)),*]
            )
        };
        () => {
            $crate::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::OBSTMtEphS::new()
        };
    }

    //		Section 14. derive impls outside verus!

    impl<T: MtVal> IntoIterator for &OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    impl<T: MtVal> IntoIterator for &mut OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    //		Section 14a. derive impls outside verus!

    impl<T: MtVal + PartialEq> PartialEq for KeyProb<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    impl<T: MtVal + Display> Display for KeyProb<T> {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format key and probability
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "({}: {:.3})", self.key, self.prob) }
    }

    impl<T: MtVal> Debug for KeyProb<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "KeyProb({:?}, {:.3})", self.key, self.prob) }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: MtVal> Debug for OptBSTMtEphKeysInv<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OptBSTMtEphKeysInv") }
    }

    impl<T: MtVal> Display for OptBSTMtEphKeysInv<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OptBSTMtEphKeysInv") }
    }

    //		Section 14c. derive impls outside verus!

    impl Debug for OptBSTMtEphMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OptBSTMtEphMemoInv") }
    }

    impl Display for OptBSTMtEphMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OptBSTMtEphMemoInv") }
    }

    //		Section 14d. derive impls outside verus!

    impl<T: MtVal> Debug for OBSTMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl<T: MtVal> Display for OBSTMtEphS<T> {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers under read locks
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_handle = self.memo.acquire_read();
            let memo_size = memo_handle.borrow().len();
            memo_handle.release_read();
            let keys_handle = self.keys.acquire_read();
            let keys_len = keys_handle.borrow().len();
            keys_handle.release_read();
            write!(f, "OBSTMtEph(keys: {keys_len}, memo_entries: {memo_size})")
        }
    }

    impl<T: MtVal> IntoIterator for OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec from Arc<RwLock>
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    //		Section 14e. derive impls outside verus!

    impl<T: MtVal> Debug for OBSTMtEphV<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OBSTMtEphV") }
    }

    impl<T: MtVal> Display for OBSTMtEphV<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OBSTMtEphV") }
    }
}
