//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral optimal binary search tree implementation using Vec and Arc<Mutex<Vec>> for mutable thread safety.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<Mutex<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod OptBinSearchTreeMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Chap50::Probability::Probability::Probability;
    use crate::Types::Types::*;

    // 4. type definitions
    #[derive(Clone, Debug)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    // Struct contains Arc<Mutex<HashMap>> for memoization — cannot be inside verus!.
    /// Ephemeral multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct OBSTMtEphS<T: MtVal> {
        pub keys: Arc<Mutex<Vec<KeyProb<T>>>>,
        pub memo: Arc<Mutex<HashMap<(usize, usize), Probability>>>,
    }

    // 8. traits
    /// Trait for parallel optimal BST operations
    pub trait OBSTMtEphTrait<T: MtVal>: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<Mutex> wrappers
        fn new()                                                  -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip n keys with probabilities then wrap in Arc<Mutex>
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<Mutex>
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — memoized DP with parallel min reduction
        fn optimal_cost(&mut self)                                -> Probability
        where
            T: Send + Sync + 'static;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn keys(&self)                                            -> Vec<KeyProb<T>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — field write under lock plus memo clear
        fn update_prob(&mut self, index: usize, prob: Probability);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under lock
        fn num_keys(&self)                                        -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under lock
        fn clear_memo(&mut self);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under lock
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
            let memo_guard = table.memo.lock().unwrap();
            if let Some(&result) = memo_guard.get(&(i, l)) {
                return result;
            }
        }

        let result = if l == 0 {
            Probability::zero()
        } else {
            let prob_sum = {
                let keys_guard = table.keys.lock().unwrap();
                (0..l)
                    .map(|k| keys_guard[i + k].prob)
                    .fold(Probability::zero(), |acc, p| acc + p)
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
            let mut memo_guard = table.memo.lock().unwrap();
            memo_guard.insert((i, l), result);
        }

        result
    }

    impl<T: MtVal> OBSTMtEphTrait<T> for OBSTMtEphS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<Mutex> wrappers
        fn new() -> Self {
            Self {
                keys: Arc::new(Mutex::new(Vec::new())),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip n keys then wrap in Arc<Mutex>
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self {
            let key_probs = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob }).collect::<Vec<KeyProb<T>>>();

            Self {
                keys: Arc::new(Mutex::new(key_probs)),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<Mutex>
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> Self {
            Self {
                keys: Arc::new(Mutex::new(key_probs)),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — clears memo, invokes obst_rec(0, n)
        fn optimal_cost(&mut self) -> Probability
        where
            T: Send + Sync + 'static,
        {
            let keys_len = {
                let keys_guard = self.keys.lock().unwrap();
                keys_guard.len()
            };

            if keys_len == 0 {
                return Probability::zero();
            }

            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            obst_rec(self, 0, keys_len)
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn keys(&self) -> Vec<KeyProb<T>> {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.clone()
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            {
                let mut keys_guard = self.keys.lock().unwrap();
                keys_guard[index] = key_prob;
            }
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — field write under lock plus memo clear
        fn update_prob(&mut self, index: usize, prob: Probability) {
            {
                let mut keys_guard = self.keys.lock().unwrap();
                keys_guard[index].prob = prob;
            }
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under lock
        fn num_keys(&self) -> usize {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.len()
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under lock
        fn clear_memo(&mut self) {
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under lock
        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    // 11. derive impls
    impl<T: MtVal> PartialEq for OBSTMtEphS<T> {
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — compare Vec contents under locks
        fn eq(&self, other: &Self) -> bool {
            let self_keys = self.keys.lock().unwrap();
            let other_keys = other.keys.lock().unwrap();
            *self_keys == *other_keys
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
    impl<T: MtVal> Display for OBSTMtEphS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers under locks
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_size = {
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
            };
            let keys_len = {
                let keys_guard = self.keys.lock().unwrap();
                keys_guard.len()
            };
            write!(f, "OBSTMtEph(keys: {keys_len}, memo_entries: {memo_size})")
        }
    }

    impl<T: MtVal> IntoIterator for OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — unwrap or clone Vec from Arc<Mutex>
        fn into_iter(self) -> Self::IntoIter {
            match Arc::try_unwrap(self.keys) {
                | Ok(mutex) => mutex.into_inner().unwrap().into_iter(),
                | Err(arc) => {
                    let keys_guard = arc.lock().unwrap();
                    keys_guard.clone().into_iter()
                }
            }
        }
    }

    impl<T: MtVal> IntoIterator for &OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn into_iter(self) -> Self::IntoIter {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.clone().into_iter()
        }
    }

    impl<T: MtVal> IntoIterator for &mut OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn into_iter(self) -> Self::IntoIter {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.clone().into_iter()
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
                vec![$($crate::Chap50::Probability::Probability::Probability::new($p)),*]
            )
        };
        () => {
            $crate::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::OBSTMtEphS::new()
        };
    }
}
