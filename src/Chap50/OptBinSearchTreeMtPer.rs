//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 50: Optimal Binary Search Tree - persistent, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.

pub mod OptBinSearchTreeMtPer {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
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

    verus! {

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 5. view impls
// 8. traits
// 9. impls
// 11. derive impls in verus!

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    impl<T: MtVal> Clone for KeyProb<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = KeyProb { key: self.key.clone(), prob: self.prob };
            proof { assume(cloned == *self); }
            cloned
        }
    }

        pub struct OptBSTMtPerMemoInv;
        impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, Probability>> for OptBSTMtPerMemoInv {
            open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, Probability>) -> bool {
                v@.dom().finite()
            }
        }

    /// Persistent multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTMtPerS<T: MtVal> {
        pub keys: Arc<Vec<KeyProb<T>>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtPerMemoInv>>,
    }

    // 5. view impls
    #[verifier::reject_recursive_types(T)]
    pub ghost struct OBSTMtPerV<T: MtVal> {
        pub keys: Seq<KeyProb<T>>,
    }

    impl<T: MtVal> View for OBSTMtPerS<T> {
        type V = OBSTMtPerV<T>;
        open spec fn view(&self) -> Self::V {
            OBSTMtPerV { keys: self.keys@ }
        }
    }

    // 8. traits
    pub trait OBSTMtPerTrait<T: MtVal>: Sized + View<V = OBSTMtPerV<T>> {
        spec fn spec_optbinsearchtreemtper_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
            ensures empty@.keys.len() == 0, empty.spec_optbinsearchtreemtper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self)
            requires keys@.len() == probs@.len(),
            ensures constructed@.keys.len() == keys@.len(), constructed.spec_optbinsearchtreemtper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self)
            ensures constructed@.keys =~= key_probs@, constructed.spec_optbinsearchtreemtper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n lg n)
        fn optimal_cost(&self) -> (cost: Probability) where T: Send + Sync + 'static
            requires self.spec_optbinsearchtreemtper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn keys(&self) -> (keys: &Arc<Vec<KeyProb<T>>>)
            ensures keys@ =~= self@.keys;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_keys(&self) -> (count: usize)
            ensures count == self@.keys.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// Clone Arc<Vec<Probability>> preserving the view.
    #[verifier::external_body]
    fn clone_arc_prob_vec(arc: &Arc<Vec<Probability>>) -> (cloned: Arc<Vec<Probability>>)
        ensures (*cloned)@ =~= (*arc)@,
    {
        arc.clone()
    }

    /// - Alg Analysis: APAS (Ch50 Alg 50.2): Work O(n^3), Span O(n lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n lg n) — parallel min reduction over split points via join, O(1) prefix sum lookup
    fn obst_rec(
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtPerMemoInv>>,
        prefix_sums: &Arc<Vec<Probability>>,
        n: usize,
        i: usize,
        l: usize,
    ) -> (cost: Probability)
        requires
            i + l <= n,
            memo.pred() == (OptBSTMtPerMemoInv),
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
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtPerMemoInv>>,
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
            memo.pred() == (OptBSTMtPerMemoInv),
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
            let ps1 = clone_arc_prob_vec(prefix_sums);
            let memo2 = clone_arc_rwlock(memo);
            let ps2 = clone_arc_prob_vec(prefix_sums);

            let f1 = move || -> (r: Probability)
                requires
                    lo < mid,
                    mid <= l,
                    l > 0,
                    i + l <= n,
                    memo1.pred() == (OptBSTMtPerMemoInv),
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
                    memo2.pred() == (OptBSTMtPerMemoInv),
                    (*ps2)@.len() == n + 1,
                ensures true
            {
                parallel_min_split_cost(&memo2, &ps2, n, i, l, mid, hi)
            };
            let (left_min, right_min) = join(f1, f2);
            if left_min <= right_min { left_min } else { right_min }
        }
    }

    impl<T: MtVal> OBSTMtPerTrait<T> for OBSTMtPerS<T> {
        open spec fn spec_optbinsearchtreemtper_wf(&self) -> bool {
            self.memo.pred() == (OptBSTMtPerMemoInv)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate empty Vec, wrap in Arc + Arc<RwLock>
        fn new() -> (empty: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: Arc::new(Vec::new()),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtPerMemoInv)),
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
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: Arc::new(key_probs),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtPerMemoInv)),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — wrap existing vec in Arc
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: Arc::new(key_probs),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtPerMemoInv)),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n lg n) — precomputes prefix sums, calls obst_rec with parallel min reduction
        fn optimal_cost(&self) -> (cost: Probability) where T: Send + Sync + 'static {
            let keys_ref = arc_deref(&self.keys);
            let keys_len = keys_ref.len();
            if keys_len == 0 { return Probability::zero(); }

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

            let prefix_sums = Arc::new(prefix);

            // Clear memo.
            {
                let memo_arc = clone_arc_rwlock(&self.memo);
                let rwlock = arc_deref(&memo_arc);
                let (mut memo, write_handle) = rwlock.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }
            assert(self.memo.pred() == (OptBSTMtPerMemoInv));
            obst_rec(&self.memo, &prefix_sums, keys_len, 0, keys_len)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — return reference to Arc field
        fn keys(&self) -> (keys: &Arc<Vec<KeyProb<T>>>) { &self.keys }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — deref Arc, return Vec len
        fn num_keys(&self) -> (count: usize) {
            let keys = arc_deref(&self.keys);
            keys.len()
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

    // 11. derive impls in verus!
    impl<T: MtVal> Clone for OBSTMtPerS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = OBSTMtPerS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            };
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: MtVal> PartialEqSpecImpl for OBSTMtPerS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: MtVal> PartialEq for OBSTMtPerS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let self_keys = arc_deref(&self.keys);
            let other_keys = arc_deref(&other.keys);
            let equal = *self_keys == *other_keys;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: MtVal> Eq for OBSTMtPerS<T> {}

    impl<T: MtVal> Eq for KeyProb<T> {}

    } // verus!

    impl<T: MtVal + PartialEq> PartialEq for KeyProb<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    // 13. derive impls outside verus!
    impl<T: MtVal> Debug for OBSTMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl<T: MtVal> Display for OBSTMtPerS<T> {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_handle = self.memo.acquire_read();
            let memo_size = memo_handle.borrow().len();
            memo_handle.release_read();
            write!(f, "OBSTMtPer(keys: {}, memo_entries: {})", self.keys.len(), memo_size)
        }
    }

    impl<T: MtVal> IntoIterator for OBSTMtPerS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — unwrap or clone Vec from Arc
        fn into_iter(self) -> Self::IntoIter {
            match Arc::try_unwrap(self.keys) {
                | Ok(vec) => vec.into_iter(),
                | Err(arc) => (*arc).clone().into_iter(),
            }
        }
    }

    impl<'a, T: MtVal> IntoIterator for &'a OBSTMtPerS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — create cloned iterator adapter over Arc<Vec>
        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<T: MtVal + Display> Display for KeyProb<T> {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format key and probability
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "({}: {:.3})", self.key, self.prob) }
    }

    impl<T: MtVal> Debug for KeyProb<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "KeyProb({:?}, {:.3})", self.key, self.prob) }
    }
}

#[macro_export]
macro_rules! OBSTMtPerLit {
    (keys: [$($k:expr),* $(,)?], probs: [$($p:expr),* $(,)?]) => {
        $crate::Chap50::OptBinSearchTreeMtPer::OptBinSearchTreeMtPer::OBSTMtPerS::from_keys_probs(
            vec![$($k),*],
            vec![$(<$crate::Chap30::Probability::Probability::Probability as $crate::Chap30::Probability::Probability::ProbabilityTrait>::new($p)),*]
        )
    };
    () => {
        $crate::Chap50::OptBinSearchTreeMtPer::OptBinSearchTreeMtPer::OBSTMtPerS::new()
    };
}
