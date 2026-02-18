//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - persistent, single-threaded.

pub mod OptBinSearchTreeStPer {

    use std::cmp::min;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use crate::Chap50::Probability::Probability::Probability;
    use crate::Types::Types::*;
    use crate::prob;

    #[derive(Clone, Debug, PartialEq)]
    pub struct KeyProb<T: StT> {
        pub key: T,
        pub prob: Probability,
    }

    /// Persistent single-threaded optimal binary search tree solver using dynamic programming
    #[derive(Clone, Debug, PartialEq)]
    pub struct OBSTStPerS<T: StT> {
        keys: Vec<KeyProb<T>>,
        memo: HashMap<(usize, usize), Probability>,
    }

    /// Trait for optimal BST operations
    pub trait OBSTStPerTrait<T: StT> {
        /// Create new optimal BST solver
        fn new()                                                  -> Self;

        /// Create from keys and probabilities
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// Create from key-probability pairs
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// APAS: Work Θ(n³), Span Θ(n²)
        /// Claude-Opus-4.6: Work O(n³), Span O(n²)
        fn optimal_cost(&self)                                    -> Probability;

        /// Get the keys with probabilities
        fn keys(&self)                                            -> &Vec<KeyProb<T>>;

        /// Get number of keys
        fn num_keys(&self)                                        -> usize;

        /// Get memoization table size
        fn memo_size(&self)                                       -> usize;
    }

    impl<T: StT> OBSTStPerS<T> {
        /// APAS: Work Θ(n³), Span Θ(n²)
        /// Claude-Opus-4.6 Work: O(n³) - O(n²) subproblems, each O(n) work
        /// Claude-Opus-4.6 Span: O(n²) - recursion depth O(n), each level O(n) work
        fn obst_rec(&mut self, i: usize, l: usize) -> Probability {
            // Check memo first
            if let Some(&result) = self.memo.get(&(i, l)) {
                return result;
            }

            let result = if l == 0 {
                Probability::zero() // Base case: empty subsequence
            } else {
                // Sum probabilities for this subsequence
                let prob_sum = (0..l)
                    .map(|k| self.keys[i + k].prob)
                    .fold(Probability::zero(), |acc, p| acc + p);

                // Try each key as root and find minimum cost
                let min_cost = (0..l)
                    .map(|k| {
                        let left_cost = self.obst_rec(i, k);
                        let right_cost = self.obst_rec(i + k + 1, l - k - 1);
                        left_cost + right_cost
                    })
                    .fold(Probability::infinity(), min);

                prob_sum + min_cost
            };

            // Memoize result
            self.memo.insert((i, l), result);
            result
        }
    }

    impl<T: StT> OBSTStPerTrait<T> for OBSTStPerS<T> {
        fn new() -> Self {
            Self {
                keys: Vec::new(),
                memo: HashMap::new(),
            }
        }

        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self {
            let key_probs = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob })
                .collect();

            Self {
                keys: key_probs,
                memo: HashMap::new(),
            }
        }

        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> Self {
            Self {
                keys: key_probs,
                memo: HashMap::new(),
            }
        }

        fn optimal_cost(&self) -> Probability {
            if self.keys.is_empty() {
                return Probability::zero();
            }

            // Create mutable copy for memoization
            let mut solver = self.clone();
            solver.memo.clear(); // Fresh memo for each query

            let n = solver.keys.len();
            solver.obst_rec(0, n)
        }

        fn keys(&self) -> &Vec<KeyProb<T>> { &self.keys }

        fn num_keys(&self) -> usize { self.keys.len() }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    impl<T: StT> Display for OBSTStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "OBSTStPer(keys: {}, memo_entries: {})",
                self.keys.len(),
                self.memo.len()
            )
        }
    }

    impl<T: StT> IntoIterator for OBSTStPerS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        fn into_iter(self) -> Self::IntoIter { self.keys.into_iter() }
    }

    impl<'a, T: StT> IntoIterator for &'a OBSTStPerS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<T: StT> Display for KeyProb<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "({}: {:.3})", self.key, self.prob) }
    }

    impl<T: StT> Eq for KeyProb<T> {}
}

#[macro_export]
macro_rules! OBSTStPerLit {
    (keys: [$($k:expr),* $(,)?], probs: [$($p:expr),* $(,)?]) => {
        $crate::Chap50::OptBinSearchTreeStPer::OptBinSearchTreeStPer::OBSTStPerS::from_keys_probs(
            vec![$($k),*],
            vec![$(prob!($p)),*]
        )
    };
    () => {
        $crate::Chap50::OptBinSearchTreeStPer::OptBinSearchTreeStPer::OBSTStPerS::new()
    };
}
