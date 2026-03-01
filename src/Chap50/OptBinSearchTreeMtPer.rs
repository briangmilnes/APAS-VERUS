//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - persistent, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.

pub mod OptBinSearchTreeMtPer {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::sync::Arc;
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap30::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    verus! {

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    impl<T: MtVal> Clone for KeyProb<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            KeyProb { key: self.key.clone(), prob: self.prob }
        }
    }

        pub struct OptBSTMtPerMemoInv;
        impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, Probability>> for OptBSTMtPerMemoInv {
            open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, Probability>) -> bool {
                v@.dom().finite()
            }
        }
        #[verifier::external_body]
        fn new_obst_per_memo_lock(val: HashMapWithViewPlus<Pair<usize, usize>, Probability>) -> (lock: RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtPerMemoInv>)
            requires val@.dom().finite()
        {
            RwLock::new(val, Ghost(OptBSTMtPerMemoInv))
        }

    /// Persistent multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTMtPerS<T: MtVal> {
        pub keys: Arc<Vec<KeyProb<T>>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtPerMemoInv>>,
    }

    impl<T: MtVal> Clone for OBSTMtPerS<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            OBSTMtPerS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    // 8. traits
    pub trait OBSTMtPerTrait<T: MtVal>: Sized {
        fn new() -> (result: Self);
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (result: Self);
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (result: Self);
        fn optimal_cost(&self) -> (result: Probability) where T: Send + Sync + 'static;
        fn keys(&self) -> (result: &Arc<Vec<KeyProb<T>>>);
        fn num_keys(&self) -> (result: usize);
        fn memo_size(&self) -> (result: usize);
    }

    // 9. impls

    #[verifier::external_body]
    fn parallel_min_reduction<T: MtVal>(table: &OBSTMtPerS<T>, costs: Vec<Probability>) -> (result: Probability) {
        if costs.is_empty() {
            return Probability::infinity();
        }
        if costs.len() == 1 {
            return costs[0];
        }

        let mid = costs.len() / 2;
        let left_costs = costs[..mid].to_vec();
        let right_costs = costs[mid..].to_vec();

        let table_clone1 = table.clone();
        let table_clone2 = table.clone();

        let handle1 = thread::spawn(move || parallel_min_reduction(&table_clone1, left_costs));
        let handle2 = thread::spawn(move || parallel_min_reduction(&table_clone2, right_costs));

        let left_min = handle1.join().unwrap();
        let right_min = handle2.join().unwrap();

        std::cmp::min(left_min, right_min)
    }

    #[verifier::external_body]
    fn obst_rec<T: MtVal + Send + Sync + 'static>(table: &OBSTMtPerS<T>, i: usize, l: usize) -> (result: Probability) {
        {
            let handle = table.memo.acquire_read();
            let cached = handle.borrow().get(&Pair(i, l)).copied();
            handle.release_read();
            if let Some(result) = cached {
                return result;
            }
        }

        let result = if l == 0 {
            Probability::zero()
        } else {
            let prob_sum = (0..l)
                .map(|k| table.keys[i + k].prob)
                .fold(Probability::zero(), |acc, p| acc + p);

            let costs = (0..l)
                .map(|k| {
                    let left_cost = obst_rec(table, i, k);
                    let right_cost = obst_rec(table, i + k + 1, l - k - 1);
                    left_cost + right_cost
                }).collect::<Vec<Probability>>();

            let min_cost = parallel_min_reduction(table, costs);

            prob_sum + min_cost
        };

        {
            let (mut memo, write_handle) = table.memo.acquire_write();
            memo.insert(Pair(i, l), result);
            write_handle.release_write(memo);
        }

        result
    }

    impl<T: MtVal> OBSTMtPerTrait<T> for OBSTMtPerS<T> {
        #[verifier::external_body]
        fn new() -> (result: Self) {
            Self {
                keys: Arc::new(Vec::new()),
                memo: Arc::new(new_obst_per_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (result: Self) {
            let key_probs = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob }).collect::<Vec<KeyProb<T>>>();
            Self {
                keys: Arc::new(key_probs),
                memo: Arc::new(new_obst_per_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (result: Self) {
            Self {
                keys: Arc::new(key_probs),
                memo: Arc::new(new_obst_per_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn optimal_cost(&self) -> (result: Probability) where T: Send + Sync + 'static {
            if self.keys.is_empty() { return Probability::zero(); }
            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }
            let n = self.keys.len();
            obst_rec(self, 0, n)
        }

        #[verifier::external_body]
        fn keys(&self) -> (result: &Arc<Vec<KeyProb<T>>>) { &self.keys }

        #[verifier::external_body]
        fn num_keys(&self) -> (result: usize) { self.keys.len() }

        #[verifier::external_body]
        fn memo_size(&self) -> (result: usize) {
            let handle = self.memo.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }
    }

    // 11. derive impls in verus!
    impl<T: MtVal> PartialEq for OBSTMtPerS<T> {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> bool { self.keys == other.keys }
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
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — unwrap or clone Vec from Arc
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

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — create cloned iterator adapter over Arc<Vec>
        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<T: MtVal + Display> Display for KeyProb<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format key and probability
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
