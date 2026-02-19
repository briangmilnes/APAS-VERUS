//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral optimal binary search tree implementation using Vec and Arc<RwLock<Vec>> for mutable thread safety.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<RwLock<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod OptBinSearchTreeMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::Arc;
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap50::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;

    // 4. type definitions
    #[derive(Clone, Debug)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    /// Ephemeral multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[derive(Clone)]
    pub struct OBSTMtEphS<T: MtVal> {
        pub keys: Arc<RwLock<Vec<KeyProb<T>>, ObstEphKeysWf>>,
        pub memo: Arc<RwLock<HashMap<(usize, usize), Probability>, ObstEphMemoWf>>,
    }

    verus! {
        #[verifier::external_type_specification]
        pub struct ExProbability(Probability);

        #[verifier::reject_recursive_types(T)]
        #[verifier::external_type_specification]
        pub struct ExKeyProb<T: MtVal>(KeyProb<T>);

        pub struct ObstEphKeysWf;
        impl<T: MtVal> RwLockPredicate<Vec<KeyProb<T>>> for ObstEphKeysWf {
            open spec fn inv(self, v: Vec<KeyProb<T>>) -> bool { true }
        }
        #[verifier::external_body]
        fn new_obst_eph_keys_lock<T: MtVal>(val: Vec<KeyProb<T>>) -> (lock: RwLock<Vec<KeyProb<T>>, ObstEphKeysWf>) {
            RwLock::new(val, Ghost(ObstEphKeysWf))
        }

        pub struct ObstEphMemoWf;
        impl RwLockPredicate<HashMap<(usize, usize), Probability>> for ObstEphMemoWf {
            open spec fn inv(self, v: HashMap<(usize, usize), Probability>) -> bool { true }
        }
        #[verifier::external_body]
        fn new_obst_eph_memo_lock(val: HashMap<(usize, usize), Probability>) -> (lock: RwLock<HashMap<(usize, usize), Probability>, ObstEphMemoWf>) {
            RwLock::new(val, Ghost(ObstEphMemoWf))
        }
    }

    // 8. traits
    /// Trait for parallel optimal BST operations
    pub trait OBSTMtEphTrait<T: MtVal>: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<RwLock> wrappers
        fn new()                                                  -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip n keys with probabilities then wrap in Arc<RwLock>
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<RwLock>
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — memoized DP with parallel min reduction
        fn optimal_cost(&mut self)                                -> Probability
        where
            T: Send + Sync + 'static;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn keys(&self)                                            -> Vec<KeyProb<T>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — field write under lock plus memo clear
        fn update_prob(&mut self, index: usize, prob: Probability);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under read lock
        fn num_keys(&self)                                        -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under write lock
        fn clear_memo(&mut self);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under read lock
        fn memo_size(&self)                                       -> usize;
    }

    // 9. impls
    /// - APAS: Work Θ(n), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — parallel divide-and-conquer min reduction
    fn parallel_min_reduction<T: MtVal>(table: &OBSTMtEphS<T>, costs: Vec<Probability>) -> Probability {
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

    /// - APAS: Work Θ(n³), Span Θ(n² lg n)
    /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — memoized DP per Algorithm 50.3, parallel min reduction per subproblem
    fn obst_rec<T: MtVal + Send + Sync + 'static>(table: &OBSTMtEphS<T>, i: usize, l: usize) -> Probability {
        {
            let handle = table.memo.acquire_read();
            let cached = handle.borrow().get(&(i, l)).copied();
            handle.release_read();
            if let Some(result) = cached {
                return result;
            }
        }

        let result = if l == 0 {
            Probability::zero()
        } else {
            let prob_sum = {
                let handle = table.keys.acquire_read();
                let keys = handle.borrow();
                let sum = (0..l)
                    .map(|k| keys[i + k].prob)
                    .fold(Probability::zero(), |acc, p| acc + p);
                handle.release_read();
                sum
            };

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
            memo.insert((i, l), result);
            write_handle.release_write(memo);
        }

        result
    }

    impl<T: MtVal> OBSTMtEphTrait<T> for OBSTMtEphS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<RwLock> wrappers
        fn new() -> Self {
            Self {
                keys: Arc::new(new_obst_eph_keys_lock(Vec::new())),
                memo: Arc::new(new_obst_eph_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip n keys then wrap in Arc<RwLock>
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self {
            let key_probs = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob }).collect::<Vec<KeyProb<T>>>();

            Self {
                keys: Arc::new(new_obst_eph_keys_lock(key_probs)),
                memo: Arc::new(new_obst_eph_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<RwLock>
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> Self {
            Self {
                keys: Arc::new(new_obst_eph_keys_lock(key_probs)),
                memo: Arc::new(new_obst_eph_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — clears memo, invokes obst_rec(0, n)
        fn optimal_cost(&mut self) -> Probability
        where
            T: Send + Sync + 'static,
        {
            let keys_len = {
                let handle = self.keys.acquire_read();
                let len = handle.borrow().len();
                handle.release_read();
                len
            };

            if keys_len == 0 {
                return Probability::zero();
            }

            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            obst_rec(self, 0, keys_len)
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn keys(&self) -> Vec<KeyProb<T>> {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            {
                let (mut keys, write_handle) = self.keys.acquire_write();
                keys[index] = key_prob;
                write_handle.release_write(keys);
            }
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — field write under lock plus memo clear
        fn update_prob(&mut self, index: usize, prob: Probability) {
            {
                let (mut keys, write_handle) = self.keys.acquire_write();
                keys[index].prob = prob;
                write_handle.release_write(keys);
            }
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under read lock
        fn num_keys(&self) -> usize {
            let handle = self.keys.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under write lock
        fn clear_memo(&mut self) {
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under read lock
        fn memo_size(&self) -> usize {
            let handle = self.memo.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }
    }

    // 11. derive impls
    impl<T: MtVal> PartialEq for OBSTMtEphS<T> {
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — compare Vec contents under read locks
        fn eq(&self, other: &Self) -> bool {
            let self_handle = self.keys.acquire_read();
            let other_handle = other.keys.acquire_read();
            let result = *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            result
        }
    }

    impl<T: MtVal> Eq for OBSTMtEphS<T> {}

    impl<T: MtVal + PartialEq> PartialEq for KeyProb<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — compare key and probability
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    impl<T: MtVal> Eq for KeyProb<T> {}

    // 13. derive impls outside verus!
    impl<T: MtVal> Debug for OBSTMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl<T: MtVal> Display for OBSTMtEphS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers under read locks
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec from Arc<RwLock>
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    impl<T: MtVal> IntoIterator for &OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    impl<T: MtVal + Display> Display for KeyProb<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format key and probability
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "({}: {:.3})", self.key, self.prob) }
    }

    // 12. macros
    #[macro_export]
    macro_rules! OBSTMtEphLit {
        (keys: [$($k:expr),* $(,)?], probs: [$($p:expr),* $(,)?]) => {
            $crate::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::OBSTMtEphS::from_keys_probs(
                vec![$($k),*],
                vec![$(<$crate::Chap50::Probability::Probability::Probability as $crate::Chap50::Probability::Probability::ProbabilityTrait>::new($p)),*]
            )
        };
        () => {
            $crate::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::OBSTMtEphS::new()
        };
    }
}
