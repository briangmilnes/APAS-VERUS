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
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate empty collections
        fn new()                                                  -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip and map n keys with probabilities
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move ownership of Vec
        fn from_key_probs(key_probs: Vec<KeyProb<T>>)             -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n³)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — memoized DP, n² subproblems × O(n) each, sequential
        fn optimal_cost(&mut self)                                -> Probability;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — reference access
        fn keys(&self)                                            -> &Vec<KeyProb<T>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — mutable reference access
        fn keys_mut(&mut self)                                    -> &mut Vec<KeyProb<T>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array write plus memo clear
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — field write plus memo clear
        fn update_prob(&mut self, index: usize, prob: Probability);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len
        fn num_keys(&self)                                        -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear
        fn clear_memo(&mut self);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len
        fn memo_size(&self)                                       -> usize;
    }

    // 9. impls
    impl<T: StT> OBSTStEphS<T> {
        /// - APAS: Work Θ(n³), Span Θ(n³)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — memoized DP per Algorithm 50.3, sequential
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
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate empty Vec and HashMap
        fn new() -> Self {
            Self {
                keys: Vec::new(),
                memo: HashMap::new(),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — zip and map n keys with probabilities
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

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move ownership of key_probs Vec
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> Self {
            Self {
                keys: key_probs,
                memo: HashMap::new(),
            }
        }

        /// - APAS: Work Θ(n³), Span Θ(n³)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — clears memo, invokes obst_rec(0, n)
        fn optimal_cost(&mut self) -> Probability {
            if self.keys.is_empty() {
                return Probability::zero();
            }

            self.memo.clear();

            let n = self.keys.len();
            self.obst_rec(0, n)
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — reference access
        fn keys(&self) -> &Vec<KeyProb<T>> { &self.keys }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — mutable reference access
        fn keys_mut(&mut self) -> &mut Vec<KeyProb<T>> { &mut self.keys }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array write plus memo clear
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            if index < self.keys.len() {
                self.keys[index] = key_prob;
            }
            self.memo.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — field write plus memo clear
        fn update_prob(&mut self, index: usize, prob: Probability) {
            if index < self.keys.len() {
                self.keys[index].prob = prob;
            }
            self.memo.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len
        fn num_keys(&self) -> usize { self.keys.len() }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear
        fn clear_memo(&mut self) { self.memo.clear(); }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len
        fn memo_size(&self) -> usize { self.memo.len() }
    }

    // 11. derive impls
    impl<T: StT> Eq for KeyProb<T> {}

    // 13. derive impls outside verus!
    impl<T: StT> Display for OBSTStEphS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
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

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move Vec into iterator
        fn into_iter(self) -> Self::IntoIter { self.keys.into_iter() }
    }

    impl<'a, T: StT> IntoIterator for &'a OBSTStEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — create cloned iterator adapter
        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<'a, T: StT> IntoIterator for &'a mut OBSTStEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — create cloned iterator adapter
        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<T: StT> Display for KeyProb<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format key and probability
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
