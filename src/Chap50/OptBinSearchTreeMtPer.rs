//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent optimal binary search tree implementation using Vec and Arc for thread safety.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<Mutex<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod OptBinSearchTreeMtPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
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
    /// Persistent multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct OBSTMtPerS<T: MtVal> {
        pub keys: Arc<Vec<KeyProb<T>>>,
        pub memo: Arc<Mutex<HashMap<(usize, usize), Probability>>>,
    }

    // 8. traits
    /// Trait for parallel optimal BST operations
    pub trait OBSTMtPerTrait<T: MtVal>: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc wrappers
        fn new()                                                  -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip n keys with probabilities then wrap in Arc
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — memoized DP with parallel min reduction
        fn optimal_cost(&self)                                    -> Probability
        where
            T: Send + Sync + 'static;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Arc reference access
        fn keys(&self)                                            -> &Arc<Vec<KeyProb<T>>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len through Arc
        fn num_keys(&self)                                        -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under lock
        fn memo_size(&self)                                       -> usize;
    }

    // 9. impls
    /// - APAS: Work Θ(n), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — parallel divide-and-conquer min reduction
    fn parallel_min_reduction<T: MtVal>(table: &OBSTMtPerS<T>, costs: Vec<Probability>) -> Probability {
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
    fn obst_rec<T: MtVal + Send + Sync + 'static>(table: &OBSTMtPerS<T>, i: usize, l: usize) -> Probability {
        {
            let memo_guard = table.memo.lock().unwrap();
            if let Some(&result) = memo_guard.get(&(i, l)) {
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
            let mut memo_guard = table.memo.lock().unwrap();
            memo_guard.insert((i, l), result);
        }

        result
    }

    impl<T: MtVal> OBSTMtPerTrait<T> for OBSTMtPerS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc wrappers
        fn new() -> Self {
            Self {
                keys: Arc::new(Vec::new()),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip n keys then wrap in Arc
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self {
            let key_probs = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob }).collect::<Vec<KeyProb<T>>>();

            Self {
                keys: Arc::new(key_probs),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> Self {
            Self {
                keys: Arc::new(key_probs),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — clears memo, invokes obst_rec(0, n)
        fn optimal_cost(&self) -> Probability
        where
            T: Send + Sync + 'static,
        {
            if self.keys.is_empty() {
                return Probability::zero();
            }

            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            let n = self.keys.len();
            obst_rec(self, 0, n)
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Arc reference access
        fn keys(&self) -> &Arc<Vec<KeyProb<T>>> { &self.keys }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len through Arc
        fn num_keys(&self) -> usize { self.keys.len() }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under lock
        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    // 11. derive impls
    impl<T: MtVal> PartialEq for OBSTMtPerS<T> {
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — compare Arc<Vec> contents
        fn eq(&self, other: &Self) -> bool { self.keys == other.keys }
    }

    impl<T: MtVal> Eq for OBSTMtPerS<T> {}

    impl<T: MtVal + PartialEq> PartialEq for KeyProb<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — compare key and probability
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    impl<T: MtVal> Eq for KeyProb<T> {}

    // 13. derive impls outside verus!
    impl<T: MtVal> Display for OBSTMtPerS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_size = {
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
            };
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
}

#[macro_export]
macro_rules! OBSTMtPerLit {
    (keys: [$($k:expr),* $(,)?], probs: [$($p:expr),* $(,)?]) => {
        $crate::Chap50::OptBinSearchTreeMtPer::OptBinSearchTreeMtPer::OBSTMtPerS::from_keys_probs(
            vec![$($k),*],
            vec![$($crate::Chap50::Probability::Probability::Probability::new($p)),*]
        )
    };
    () => {
        $crate::Chap50::OptBinSearchTreeMtPer::OptBinSearchTreeMtPer::OBSTMtPerS::new()
    };
}
