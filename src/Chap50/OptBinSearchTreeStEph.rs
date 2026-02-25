//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - ephemeral, single-threaded.
//!
//! Memoized top-down DP for optimal BST cost.
//! Uses HashMapWithViewPlus for the memo table.

pub mod OptBinSearchTreeStEph {

    use std::cmp::min;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Chap30::Probability::Probability::{Probability, ProbabilityTrait};
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

    /// Ephemeral single-threaded optimal binary search tree solver using dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTStEphS<T: StT> {
        pub keys: Vec<KeyProb<T>>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, Probability>,
    }

    impl<T: StT> Clone for OBSTStEphS<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            OBSTStEphS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    // 8. traits
    pub trait OBSTStEphTrait<T: StT>: Sized {
        fn new() -> (result: Self);
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (result: Self);
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (result: Self);
        fn optimal_cost(&mut self) -> (result: Probability);
        fn keys(&self) -> (result: &Vec<KeyProb<T>>);
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>);
        fn update_prob(&mut self, index: usize, prob: Probability);
        fn num_keys(&self) -> (result: usize);
        fn clear_memo(&mut self);
        fn memo_size(&self) -> (result: usize);
    }

    // 9. impls

    #[verifier::external_body]
    fn obst_rec_st_eph<T: StT>(s: &mut OBSTStEphS<T>, i: usize, l: usize) -> (result: Probability) {
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
                    let left_cost = obst_rec_st_eph(s, i, k);
                    let right_cost = obst_rec_st_eph(s, i + k + 1, l - k - 1);
                    left_cost + right_cost
                })
                .fold(Probability::infinity(), min);

            prob_sum + min_cost
        };

        s.memo.insert(Pair(i, l), result);
        result
    }

    impl<T: StT> OBSTStEphTrait<T> for OBSTStEphS<T> {
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
        fn optimal_cost(&mut self) -> (result: Probability) {
            if self.keys.is_empty() {
                return Probability::zero();
            }

            self.memo.clear();

            let n = self.keys.len();
            obst_rec_st_eph(self, 0, n)
        }

        fn keys(&self) -> (result: &Vec<KeyProb<T>>) { &self.keys }

        #[verifier::external_body]
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            if index < self.keys.len() {
                self.keys[index] = key_prob;
            }
            self.memo.clear();
        }

        #[verifier::external_body]
        fn update_prob(&mut self, index: usize, prob: Probability) {
            if index < self.keys.len() {
                self.keys[index].prob = prob;
            }
            self.memo.clear();
        }

        fn num_keys(&self) -> (result: usize) { self.keys.len() }

        fn clear_memo(&mut self) { self.memo.clear(); }

        fn memo_size(&self) -> (result: usize) { self.memo.len() }
    }

    // 11. derive impls in verus!
    impl<T: StT> Eq for KeyProb<T> {}

    } // verus!

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

    impl<T: StT> Debug for KeyProb<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "KeyProb({:?}, {:.3})", self.key, self.prob) }
    }

    impl<T: StT + PartialEq> PartialEq for KeyProb<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    impl<T: StT> Debug for OBSTStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "OBSTStEph(keys: {}, memo_entries: {})", self.keys.len(), self.memo.len())
        }
    }

    impl<T: StT + PartialEq> PartialEq for OBSTStEphS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.keys == other.keys && self.memo == other.memo
        }
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
