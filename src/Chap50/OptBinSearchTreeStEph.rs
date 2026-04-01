//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 50: Optimal Binary Search Tree - ephemeral, single-threaded.
//!
//! Memoized top-down DP for optimal BST cost.
//! Uses HashMapWithViewPlus for the memo table.

pub mod OptBinSearchTreeStEph {

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

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 5. view impls
// 8. traits
// 9. impls
// 11. derive impls in verus!

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: StT> {
        pub key: T,
        pub prob: Probability,
    }

    impl<T: StT> Clone for KeyProb<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self,
        {
            let cloned = KeyProb { key: self.key.clone(), prob: self.prob };
            proof { assume(cloned == *self); }  // accept hole: T::clone external_body
            cloned
        }
    }

    /// Ephemeral single-threaded optimal binary search tree solver using dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTStEphS<T: StT> {
        pub keys: Vec<KeyProb<T>>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, Probability>,
    }

    impl<T: StT> Clone for OBSTStEphS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = OBSTStEphS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            };
            proof { assume(cloned@ == self@); }  // accept hole: Vec::clone external_body
            cloned
        }
    }

    // 5. view impls
    #[verifier::reject_recursive_types(T)]
    pub ghost struct OBSTStEphV<T: StT> {
        pub keys: Seq<KeyProb<T>>,
        pub memo: Map<(usize, usize), Probability>,
    }

    impl<T: StT> View for OBSTStEphS<T> {
        type V = OBSTStEphV<T>;
        open spec fn view(&self) -> Self::V {
            OBSTStEphV {
                keys: self.keys@,
                memo: self.memo@,
            }
        }
    }

    // 8. traits
    pub trait OBSTStEphTrait<T: StT>: Sized + View<V = OBSTStEphV<T>> {
        fn new() -> (empty: Self)
            ensures
                empty@.keys.len() == 0,
                empty@.memo =~= Map::<(usize, usize), Probability>::empty();

        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self)
            requires keys@.len() == probs@.len(),
            ensures
                constructed@.keys.len() == keys@.len(),
                constructed@.memo =~= Map::<(usize, usize), Probability>::empty();

        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self)
            ensures
                constructed@.keys =~= key_probs@,
                constructed@.memo =~= Map::<(usize, usize), Probability>::empty();

        fn optimal_cost(&mut self) -> (cost: Probability);

        fn keys(&self) -> (keys: &Vec<KeyProb<T>>)
            ensures keys@ =~= self@.keys;

        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>)
            requires index < old(self)@.keys.len(),
            ensures
                self@.keys =~= old(self)@.keys.update(index as int, key_prob),
                self@.memo =~= Map::<(usize, usize), Probability>::empty();

        fn update_prob(&mut self, index: usize, prob: Probability)
            requires index < old(self)@.keys.len(),
            ensures
                self@.keys.len() == old(self)@.keys.len(),
                self@.memo =~= Map::<(usize, usize), Probability>::empty();

        fn num_keys(&self) -> (count: usize)
            ensures count == self@.keys.len();

        fn clear_memo(&mut self)
            ensures
                self@.keys =~= old(self)@.keys,
                self@.memo =~= Map::<(usize, usize), Probability>::empty();

        fn memo_size(&self) -> (count: usize)
            ensures count == self@.memo.len();
    }

    // 9. impls

    /// - Alg Analysis: APAS (Ch50 Alg 50.2): Work O(n^3), Span O(n lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — DIFFERS: sequential DP table fill, APAS Span O(n lg n) assumes parallel
    fn obst_rec_st_eph<T: StT>(s: &mut OBSTStEphS<T>, i: usize, l: usize) -> (cost: Probability)
        requires
            i + l <= old(s)@.keys.len(),
            old(s)@.memo.dom().finite(),
        ensures
            s@.keys =~= old(s)@.keys,
            s@.memo.dom().finite(),
        decreases l,
    {
        let cached = match s.memo.get(&Pair(i, l)) {
            Some(v) => Some(*v),
            None => None,
        };
        if let Some(cost) = cached {
            return cost;
        }

        let cost = if l == 0 {
            Probability::zero()
        } else {
            let n = s.keys.len();

            // Sum probabilities of keys[i..i+l].
            let mut prob_sum = Probability::zero();
            let mut k: usize = 0;
            while k < l
                invariant
                    k <= l,
                    i + l <= n,
                    n == s@.keys.len(),
                    s@.keys =~= old(s)@.keys,
                    s@.memo.dom().finite(),
                decreases l - k,
            {
                prob_sum = prob_sum + s.keys[i + k].prob;
                k = k + 1;
            }

            // Find minimum cost over all split points.
            let mut min_cost = Probability::infinity();
            let mut k: usize = 0;
            while k < l
                invariant
                    k <= l,
                    i + l <= n,
                    n == s@.keys.len(),
                    i + l <= s@.keys.len(),
                    s@.keys =~= old(s)@.keys,
                    s@.memo.dom().finite(),
                decreases l - k,
            {
                let left_cost = obst_rec_st_eph(s, i, k);
                let right_cost = obst_rec_st_eph(s, i + k + 1, l - k - 1);
                let split_cost = left_cost + right_cost;
                if split_cost <= min_cost {
                    min_cost = split_cost;
                }
                k = k + 1;
            }

            prob_sum + min_cost
        };

        s.memo.insert(Pair(i, l), cost);
        cost
    }

    impl<T: StT> OBSTStEphTrait<T> for OBSTStEphS<T> {
        fn new() -> (empty: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: Vec::new(),
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self) {
            let n = keys.len();
            let mut key_probs: Vec<KeyProb<T>> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    n == keys@.len(),
                    keys@.len() == probs@.len(),
                    i <= n,
                    key_probs@.len() == i as int,
                decreases n - i,
            {
                key_probs.push(KeyProb { key: keys[i].clone(), prob: probs[i] });
                i += 1;
            }
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: key_probs,
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: key_probs,
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn optimal_cost(&mut self) -> (cost: Probability) {
            if self.keys.len() == 0 {
                return Probability::zero();
            }

            self.memo.clear();

            let n = self.keys.len();
            obst_rec_st_eph(self, 0, n)
        }

        fn keys(&self) -> (keys: &Vec<KeyProb<T>>) { &self.keys }

        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            self.keys.set(index, key_prob);
            self.memo.clear();
        }

        fn update_prob(&mut self, index: usize, prob: Probability) {
            let key = self.keys[index].key.clone();
            self.keys.set(index, KeyProb { key, prob });
            self.memo.clear();
        }

        fn num_keys(&self) -> (count: usize) { self.keys.len() }

        fn clear_memo(&mut self) { self.memo.clear(); }

        fn memo_size(&self) -> (count: usize) { self.memo.len() }
    }

    // 11. derive impls in verus!
    impl<T: StT> Eq for KeyProb<T> {}

    } // verus!

    // 13. derive impls outside verus!
    impl<T: StT> Display for OBSTStEphS<T> {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers
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

        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — move Vec into iterator
        fn into_iter(self) -> Self::IntoIter { self.keys.into_iter() }
    }

    impl<'a, T: StT> IntoIterator for &'a OBSTStEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — create cloned iterator adapter
        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<'a, T: StT> IntoIterator for &'a mut OBSTStEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — create cloned iterator adapter
        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    impl<T: StT> Display for KeyProb<T> {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format key and probability
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
