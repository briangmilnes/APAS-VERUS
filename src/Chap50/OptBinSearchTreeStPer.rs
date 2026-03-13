//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - persistent, single-threaded.
//!
//! Memoized top-down DP for optimal BST cost.
//! Uses HashMapWithViewPlus for the memo table.

pub mod OptBinSearchTreeStPer {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Chap30::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
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
            proof { accept(cloned == *self); }  // accept hole: T::clone external_body
            cloned
        }
    }

    /// Persistent single-threaded optimal binary search tree solver using dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTStPerS<T: StT> {
        pub keys: Vec<KeyProb<T>>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, Probability>,
    }

    impl<T: StT> Clone for OBSTStPerS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = OBSTStPerS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            };
            proof { accept(cloned@ == self@); }  // accept hole: Vec::clone external_body
            cloned
        }
    }

    // 5. view impls
    #[verifier::reject_recursive_types(T)]
    pub ghost struct OBSTStPerV<T: StT> {
        pub keys: Seq<KeyProb<T>>,
        pub memo: Map<(usize, usize), Probability>,
    }

    impl<T: StT> View for OBSTStPerS<T> {
        type V = OBSTStPerV<T>;
        open spec fn view(&self) -> Self::V {
            OBSTStPerV {
                keys: self.keys@,
                memo: self.memo@,
            }
        }
    }

    // 8. traits
    pub trait OBSTStPerTrait<T: StT>: Sized + View<V = OBSTStPerV<T>> {
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

        fn optimal_cost(&self) -> (cost: Probability);

        fn keys(&self) -> (keys: &Vec<KeyProb<T>>)
            ensures keys@ =~= self@.keys;

        fn num_keys(&self) -> (count: usize)
            ensures count == self@.keys.len();

        fn memo_size(&self) -> (count: usize)
            ensures count == self@.memo.len();
    }

    // 9. impls

    fn obst_rec_st_per<T: StT>(s: &mut OBSTStPerS<T>, i: usize, l: usize) -> (cost: Probability)
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
                let left_cost = obst_rec_st_per(s, i, k);
                let right_cost = obst_rec_st_per(s, i + k + 1, l - k - 1);
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

    impl<T: StT> OBSTStPerTrait<T> for OBSTStPerS<T> {
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

        fn optimal_cost(&self) -> (cost: Probability) {
            if self.keys.len() == 0 {
                return Probability::zero();
            }

            let mut solver = self.clone();
            solver.memo.clear();

            let n = solver.keys.len();
            obst_rec_st_per(&mut solver, 0, n)
        }

        fn keys(&self) -> (keys: &Vec<KeyProb<T>>) { &self.keys }

        fn num_keys(&self) -> (count: usize) { self.keys.len() }

        fn memo_size(&self) -> (count: usize) { self.memo.len() }
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
