//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - ephemeral, single-threaded.

pub mod OptBinSearchTreeStEph {

    use std::cmp::min;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Chap50::Probability::Probability::Probability;
    use crate::Types::Types::*;
    use crate::prob;

    verus! {
    } // verus!

    // 4. type definitions
    #[derive(Clone, Debug, PartialEq)]
    pub struct KeyProb<T: StT> {
        pub key: T,
        pub prob: Probability,
    }

    /// Ephemeral single-threaded optimal binary search tree solver using dynamic programming
    #[derive(Clone, Debug, PartialEq)]
    pub struct OBSTStEphS<T: StT> {
        pub keys: Vec<KeyProb<T>>,
        pub memo: HashMap<(usize, usize), Probability>,
    }

    // 8. traits
    /// Trait for optimal BST operations
    pub trait OBSTStEphTrait<T: StT>: Sized {
        /// Create new optimal BST solver
        fn new()                                                  -> Self;

        /// Create from keys and probabilities
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// Create from key-probability pairs
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// APAS: Work Θ(n³), Span Θ(n²)
        /// Claude-Opus-4.6: Work O(n³), Span O(n²)
        fn optimal_cost(&mut self)                                -> Probability;

        /// Get the keys with probabilities
        fn keys(&self)                                            -> &Vec<KeyProb<T>>;

        /// Get mutable keys (ephemeral allows mutation)
        fn keys_mut(&mut self)                                    -> &mut Vec<KeyProb<T>>;

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

    // 9. impls
    impl<T: StT> OBSTStEphS<T> {
        /// APAS: Work Θ(n³), Span Θ(n²)
        /// Claude-Opus-4.6 Work: O(n³) - O(n²) subproblems, each O(n) work
        /// Claude-Opus-4.6 Span: O(n²) - recursion depth O(n), each level O(n) work
        fn obst_rec(&mut self, i: usize, l: usize) -> Probability {
            if let Some(&result) = self.memo.get(&(i, l)) {
                return result;
            }

            let result = if l == 0 {
                Probability::zero()
            } else {
                let prob_sum = (0..l)
                    .map(|k| self.keys[i + k].prob)
                    .fold(Probability::zero(), |acc, p| acc + p);

                let min_cost = (0..l)
                    .map(|k| {
                        let left_cost = self.obst_rec(i, k);
                        let right_cost = self.obst_rec(i + k + 1, l - k - 1);
                        left_cost + right_cost
                    })
                    .fold(Probability::infinity(), min);

                prob_sum + min_cost
            };

            self.memo.insert((i, l), result);
            result
        }
    }

    impl<T: StT> OBSTStEphTrait<T> for OBSTStEphS<T> {
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

        fn optimal_cost(&mut self) -> Probability {
            if self.keys.is_empty() {
                return Probability::zero();
            }

            self.memo.clear();

            let n = self.keys.len();
            self.obst_rec(0, n)
        }

        fn keys(&self) -> &Vec<KeyProb<T>> { &self.keys }

        fn keys_mut(&mut self) -> &mut Vec<KeyProb<T>> { &mut self.keys }

        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            if index < self.keys.len() {
                self.keys[index] = key_prob;
            }
            self.memo.clear();
        }

        fn update_prob(&mut self, index: usize, prob: Probability) {
            if index < self.keys.len() {
                self.keys[index].prob = prob;
            }
            self.memo.clear();
        }

        fn num_keys(&self) -> usize { self.keys.len() }

        fn clear_memo(&mut self) { self.memo.clear(); }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    // 11. derive impls
    impl<T: StT> Eq for KeyProb<T> {}

    // 13. derive impls outside verus!
    impl<T: StT> Display for OBSTStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "OBSTStEph(keys: {}, memo_entries: {})",
                self.keys.len(),
                self.memo.len()
            )
        }
    }

    impl<T: StT> IntoIterator for OBSTStEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        fn into_iter(self) -> Self::IntoIter { self.keys.into_iter() }
    }

    impl<'a, T: StT> IntoIterator for &'a OBSTStEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<'a, T: StT> IntoIterator for &'a mut OBSTStEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<T: StT> Display for KeyProb<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "({}: {:.3})", self.key, self.prob) }
    }
}

#[macro_export]
macro_rules! OBSTStEphLit {
    (keys: [$($k:expr),* $(,)?], probs: [$($p:expr),* $(,)?]) => {
        $crate::Chap50::OptBinSearchTreeStEph::OptBinSearchTreeStEph::OBSTStEphS::from_keys_probs(
            vec![$($k),*],
            vec![$(prob!($p)),*]
        )
    };
    () => {
        $crate::Chap50::OptBinSearchTreeStEph::OptBinSearchTreeStEph::OBSTStEphS::new()
    };
}
