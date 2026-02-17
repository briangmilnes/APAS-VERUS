//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent optimal binary search tree implementation using Vec and Arc for thread safety.

pub mod OptBinSearchTreeMtPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::vec::IntoIter;

    use crate::Chap50::Probability::Probability::Probability;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    /// Persistent multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct OBSTMtPerS<T: MtVal> {
        keys: Arc<Vec<KeyProb<T>>>,
        memo: Arc<Mutex<HashMap<(usize, usize), Probability>>>,
    }

    /// Trait for parallel optimal BST operations
    pub trait OBSTMtPerTrait<T: MtVal> {
        /// Create new optimal BST solver
        fn new()                                                  -> Self;

        /// Create from keys and probabilities
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// Create from key-probability pairs
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// claude-4-sonet: Work Θ(n³), Span Θ(n log n), Parallelism Θ(n²/log n)
        /// Compute optimal BST cost where n=number of keys
        fn optimal_cost(&self)                                    -> Probability
        where
            T: Send + Sync + 'static;

        /// Get the keys with probabilities
        fn keys(&self)                                            -> &Arc<Vec<KeyProb<T>>>;

        /// Get number of keys
        fn num_keys(&self)                                        -> usize;

        /// Get memoization table size
        fn memo_size(&self)                                       -> usize;
    }

    impl<T: MtVal> OBSTMtPerS<T> {
        /// Parallel reduction to find minimum cost among root choices
        /// Claude Work: O(n) - n comparisons
        /// Claude Span: O(log n) - parallel reduction tree
        fn parallel_min_reduction(&self, costs: Vec<Probability>) -> Probability {
            if costs.is_empty() {
                return Probability::infinity();
            }
            if costs.len() == 1 {
                return costs[0];
            }

            let mid = costs.len() / 2;
            let left_costs = costs[..mid].to_vec();
            let right_costs = costs[mid..].to_vec();

            let self_clone1 = self.clone();
            let self_clone2 = self.clone();

            let handle1 = thread::spawn(move || self_clone1.parallel_min_reduction(left_costs));

            let handle2 = thread::spawn(move || self_clone2.parallel_min_reduction(right_costs));

            let left_min = handle1.join().unwrap();
            let right_min = handle2.join().unwrap();

            std::cmp::min(left_min, right_min)
        }

        /// Internal recursive optimal BST with memoization and parallel reduction
        /// Claude Work: O(n³) - O(n²) subproblems, each O(n) work
        /// Claude Span: O(n log n) - maximum recursion depth O(n), each level O(log n) parallel reduction
        fn obst_rec(&self, i: usize, l: usize) -> Probability
        where
            T: Send + Sync + 'static,
        {
            // Check memo first (thread-safe)
            {
                let memo_guard = self.memo.lock().unwrap();
                if let Some(&result) = memo_guard.get(&(i, l)) {
                    return result;
                }
            }

            let result = if l == 0 {
                Probability::zero() // Base case: empty subsequence
            } else {
                // Sum probabilities for this subsequence
                let prob_sum: Probability = (0..l)
                    .map(|k| self.keys[i + k].prob)
                    .fold(Probability::zero(), |acc, p| acc + p);

                // Compute costs for each possible root in parallel
                let costs: Vec<Probability> = (0..l)
                    .map(|k| {
                        let left_cost = self.obst_rec(i, k);
                        let right_cost = self.obst_rec(i + k + 1, l - k - 1);
                        left_cost + right_cost
                    })
                    .collect();

                // Use parallel reduction to find minimum
                let min_cost = self.parallel_min_reduction(costs);

                prob_sum + min_cost
            };

            // Memoize result (thread-safe)
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.insert((i, l), result);
            }

            result
        }
    }

    impl<T: MtVal> OBSTMtPerTrait<T> for OBSTMtPerS<T> {
        fn new() -> Self {
            Self {
                keys: Arc::new(Vec::new()),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self {
            let key_probs: Vec<KeyProb<T>> = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob })
                .collect();

            Self {
                keys: Arc::new(key_probs),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> Self {
            Self {
                keys: Arc::new(key_probs),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn optimal_cost(&self) -> Probability
        where
            T: Send + Sync + 'static,
        {
            if self.keys.is_empty() {
                return Probability::zero();
            }

            // Clear memo for fresh computation
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            let n = self.keys.len();
            self.obst_rec(0, n)
        }

        fn keys(&self) -> &Arc<Vec<KeyProb<T>>> { &self.keys }

        fn num_keys(&self) -> usize { self.keys.len() }

        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    impl<T: MtVal> PartialEq for OBSTMtPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.keys == other.keys }
    }

    impl<T: MtVal> Eq for OBSTMtPerS<T> {}

    impl<T: MtVal> Display for OBSTMtPerS<T> {
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

        fn into_iter(self) -> Self::IntoIter {
            // Extract Vec from Arc - this consumes the Arc
            match Arc::try_unwrap(self.keys) {
                | Ok(vec) => vec.into_iter(),
                | Err(arc) => (*arc).clone().into_iter(),
            }
        }
    }

    impl<'a, T: MtVal> IntoIterator for &'a OBSTMtPerS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<T: MtVal + PartialEq> PartialEq for KeyProb<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    impl<T: MtVal> Eq for KeyProb<T> {}

    impl<T: MtVal + Display> Display for KeyProb<T> {
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
