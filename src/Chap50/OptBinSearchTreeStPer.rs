//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - persistent, single-threaded.
//!
//! Memoized top-down DP for optimal BST cost.
//! Uses HashMapWithViewPlus for the memo table.

pub mod OptBinSearchTreeStPer {

    use std::cmp::min;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Chap50::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::prob;

    verus! {

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: StT> {
        pub key: T,
        pub prob: Probability,
    }

    impl<T: StT> Clone for KeyProb<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            KeyProb { key: self.key.clone(), prob: self.prob }
        }
    }

    /// Persistent single-threaded optimal binary search tree solver using dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTStPerS<T: StT> {
        pub keys: Vec<KeyProb<T>>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, Probability>,
    }

    impl<T: StT> Clone for OBSTStPerS<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            OBSTStPerS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    // 8. traits
    pub trait OBSTStPerTrait<T: StT>: Sized {
        fn new() -> (result: Self);
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (result: Self);
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (result: Self);
        fn optimal_cost(&self) -> (result: Probability);
        fn keys(&self) -> (result: &Vec<KeyProb<T>>);
        fn num_keys(&self) -> (result: usize);
        fn memo_size(&self) -> (result: usize);
    }

    // 9. impls

    #[verifier::external_body]
    fn obst_rec_st_per<T: StT>(s: &mut OBSTStPerS<T>, i: usize, l: usize) -> (result: Probability) {
        if let Some(&result) = s.memo.get(&Pair(i, l)) {
            return result;
        }

        let result = if l == 0 {
            Probability::zero()
        } else {
            let prob_sum = (0..l)
                .map(|k| s.keys[i + k].prob)
                .fold(Probability::zero(), |acc, p| acc + p);

            let min_cost = (0..l)
                .map(|k| {
                    let left_cost = obst_rec_st_per(s, i, k);
                    let right_cost = obst_rec_st_per(s, i + k + 1, l - k - 1);
                    left_cost + right_cost
                })
                .fold(Probability::infinity(), min);

            prob_sum + min_cost
        };

        s.memo.insert(Pair(i, l), result);
        result
    }

    impl<T: StT> OBSTStPerTrait<T> for OBSTStPerS<T> {
        #[verifier::external_body]
        fn new() -> (result: Self) {
            Self {
                keys: Vec::new(),
                memo: HashMapWithViewPlus::new(),
            }
        }

        #[verifier::external_body]
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (result: Self) {
            let key_probs = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob })
                .collect();

            Self {
                keys: key_probs,
                memo: HashMapWithViewPlus::new(),
            }
        }

        #[verifier::external_body]
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (result: Self) {
            Self {
                keys: key_probs,
                memo: HashMapWithViewPlus::new(),
            }
        }

        #[verifier::external_body]
        fn optimal_cost(&self) -> (result: Probability) {
            if self.keys.is_empty() {
                return Probability::zero();
            }

            let mut solver = self.clone();
            solver.memo.clear();

            let n = solver.keys.len();
            obst_rec_st_per(&mut solver, 0, n)
        }

        fn keys(&self) -> (result: &Vec<KeyProb<T>>) { &self.keys }

        fn num_keys(&self) -> (result: usize) { self.keys.len() }

        fn memo_size(&self) -> (result: usize) { self.memo.len() }
    }

    // 11. derive impls in verus!
    impl<T: StT> Eq for KeyProb<T> {}

    } // verus!

    // 13. derive impls outside verus!
    impl<T: StT> Display for OBSTStPerS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
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

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move Vec into iterator
        fn into_iter(self) -> Self::IntoIter { self.keys.into_iter() }
    }

    impl<'a, T: StT> IntoIterator for &'a OBSTStPerS<T> {
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

    impl<T: StT> Debug for KeyProb<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "KeyProb({:?}, {:.3})", self.key, self.prob) }
    }

    impl<T: StT + PartialEq> PartialEq for KeyProb<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    impl<T: StT> Debug for OBSTStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "OBSTStPer(keys: {}, memo_entries: {})", self.keys.len(), self.memo.len())
        }
    }

    impl<T: StT + PartialEq> PartialEq for OBSTStPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.keys == other.keys && self.memo == other.memo
        }
    }
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
