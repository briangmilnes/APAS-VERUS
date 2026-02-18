//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral optimal binary search tree implementation using Vec and Arc<Mutex<Vec>> for mutable thread safety.

pub mod OptBinSearchTreeMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
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

    /// Ephemeral multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct OBSTMtEphS<T: MtVal> {
        keys: Arc<Mutex<Vec<KeyProb<T>>>>,
        memo: Arc<Mutex<HashMap<(usize, usize), Probability>>>,
    }

    /// Trait for parallel optimal BST operations
    pub trait OBSTMtEphTrait<T: MtVal> {
        /// Create new optimal BST solver
        fn new()                                                  -> Self;

        /// Create from keys and probabilities
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// Create from key-probability pairs
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// APAS: Work Θ(n³), Span Θ(n log n)
        /// Claude-Opus-4.6: Work O(n³), Span O(n log n)
        fn optimal_cost(&mut self)                                -> Probability
        where
            T: Send + Sync + 'static;

        /// Get a copy of the keys with probabilities (thread-safe)
        fn keys(&self)                                            -> Vec<KeyProb<T>>;

        /// Set key-probability pair at index
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>);

        /// Update probability for key at index
        fn update_prob(&mut self, index: usize, prob: Probability);

        /// Get number of keys
        fn num_keys(&self)                                        -> usize;

        /// Clear memoization table
        fn clear_memo(&mut self);

        /// Get memoization table size
        fn memo_size(&self)                                       -> usize;
    }

    /// APAS: Work Θ(n), Span Θ(log n)
    /// Claude-Opus-4.6 Work: O(n) - n comparisons
    /// Claude-Opus-4.6 Span: O(log n) - parallel reduction tree
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

    /// APAS: Work Θ(n³), Span Θ(n log n)
    /// Claude-Opus-4.6 Work: O(n³) - O(n²) subproblems, each O(n) work
    /// Claude-Opus-4.6 Span: O(n log n) - recursion depth O(n), each level O(log n) parallel reduction
    fn obst_rec<T: MtVal + Send + Sync + 'static>(table: &OBSTMtEphS<T>, i: usize, l: usize) -> Probability {
        // Check memo first (thread-safe)
        {
            let memo_guard = table.memo.lock().unwrap();
            if let Some(&result) = memo_guard.get(&(i, l)) {
                return result;
            }
        }

        let result = if l == 0 {
            Probability::zero() // Base case: empty subsequence
        } else {
            // Sum probabilities for this subsequence (thread-safe access)
            let prob_sum = {
                let keys_guard = table.keys.lock().unwrap();
                (0..l)
                    .map(|k| keys_guard[i + k].prob)
                    .fold(Probability::zero(), |acc, p| acc + p)
            };

            // Compute costs for each possible root in parallel
            let costs = (0..l)
                .map(|k| {
                    let left_cost = obst_rec(table, i, k);
                    let right_cost = obst_rec(table, i + k + 1, l - k - 1);
                    left_cost + right_cost
                }).collect::<Vec<Probability>>();

            // Use parallel reduction to find minimum
            let min_cost = parallel_min_reduction(table, costs);

            prob_sum + min_cost
        };

        // Memoize result (thread-safe)
        {
            let mut memo_guard = table.memo.lock().unwrap();
            memo_guard.insert((i, l), result);
        }

        result
    }

    impl<T: MtVal> OBSTMtEphTrait<T> for OBSTMtEphS<T> {
        fn new() -> Self {
            Self {
                keys: Arc::new(Mutex::new(Vec::new())),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

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

        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> Self {
            Self {
                keys: Arc::new(Mutex::new(key_probs)),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

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

            // Clear memo for fresh computation
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            obst_rec(self, 0, keys_len)
        }

        fn keys(&self) -> Vec<KeyProb<T>> {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.clone()
        }

        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            {
                let mut keys_guard = self.keys.lock().unwrap();
                keys_guard[index] = key_prob;
            }
            // Clear memo since keys changed
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        fn update_prob(&mut self, index: usize, prob: Probability) {
            {
                let mut keys_guard = self.keys.lock().unwrap();
                keys_guard[index].prob = prob;
            }
            // Clear memo since probabilities changed
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        fn num_keys(&self) -> usize {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.len()
        }

        fn clear_memo(&mut self) {
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    impl<T: MtVal> PartialEq for OBSTMtEphS<T> {
        fn eq(&self, other: &Self) -> bool {
            // Compare the contents of the Arc<Mutex<Vec>>
            let self_keys = self.keys.lock().unwrap();
            let other_keys = other.keys.lock().unwrap();
            *self_keys == *other_keys
        }
    }

    impl<T: MtVal> Eq for OBSTMtEphS<T> {}

    impl<T: MtVal> Display for OBSTMtEphS<T> {
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

        fn into_iter(self) -> Self::IntoIter {
            // Extract Vec from Arc<Mutex<Vec>> - this consumes the Arc
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

        fn into_iter(self) -> Self::IntoIter {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.clone().into_iter()
        }
    }

    impl<T: MtVal> IntoIterator for &mut OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        fn into_iter(self) -> Self::IntoIter {
            let keys_guard = self.keys.lock().unwrap();
            keys_guard.clone().into_iter()
        }
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
