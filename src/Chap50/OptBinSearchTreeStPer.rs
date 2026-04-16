// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 50: Optimal Binary Search Tree - persistent, single-threaded.
//!
//! Memoized top-down DP for optimal BST cost.
//! Uses HashMapWithViewPlus for the memo table.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 8b. traits
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module

pub mod OptBinSearchTreeStPer {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Chap30::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::prob;
    use crate::vstdplus::accept::accept;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: StT> {
        pub key: T,
        pub prob: Probability,
    }

    //		Section 4b. type definitions


    /// Persistent single-threaded optimal binary search tree solver using dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTStPerS<T: StT> {
        pub keys: Vec<KeyProb<T>>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, Probability>,
    }

    //		Section 5b. view impls


    impl<T: StT> View for OBSTStPerS<T> {
        type V = OBSTStPerV<T>;
        open spec fn view(&self) -> Self::V {
            OBSTStPerV {
                keys: self.keys@,
                memo: self.memo@,
            }
        }
    }

    //		Section 8b. traits


    pub trait OBSTStPerTrait<T: StT>: Sized + View<V = OBSTStPerV<T>> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
            ensures
                empty@.keys.len() == 0,
                empty@.memo =~= Map::<(usize, usize), Probability>::empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self)
            requires keys@.len() == probs@.len(),
            ensures
                constructed@.keys.len() == keys@.len(),
                constructed@.memo =~= Map::<(usize, usize), Probability>::empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self)
            ensures
                constructed@.keys =~= key_probs@,
                constructed@.memo =~= Map::<(usize, usize), Probability>::empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3)
        fn optimal_cost(&self) -> (cost: Probability);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn keys(&self) -> (keys: &Vec<KeyProb<T>>)
            ensures keys@ =~= self@.keys;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_keys(&self) -> (count: usize)
            ensures count == self@.keys.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize)
            ensures count == self@.memo.len();
    }

    //		Section 9b. impls


    impl<T: StT> OBSTStPerTrait<T> for OBSTStPerS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self) {
            // Veracity: NEEDED proof block
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: Vec::new(),
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
            // Veracity: NEEDED proof block
            }
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: key_probs,
                memo: HashMapWithViewPlus::new(),
            }
        }

        // Veracity: NEEDED proof block (speed hint)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: key_probs,
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3)
        fn optimal_cost(&self) -> (cost: Probability) {
            if self.keys.len() == 0 {
                return Probability::zero();
            }

            let mut solver = self.clone();
            solver.memo.clear();

            let n = solver.keys.len();
            obst_rec_st_per(&mut solver, 0, n)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn keys(&self) -> (keys: &Vec<KeyProb<T>>) { &self.keys }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_keys(&self) -> (count: usize) { self.keys.len() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize) { self.memo.len() }
    }


    /// - Alg Analysis: APAS (Ch50 Alg 50.2): Work O(n^3), Span O(n lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — ACCEPTED DIFFERENCE: sequential DP table fill, APAS Span O(n lg n) assumes parallel
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

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    pub ghost struct OBSTStPerV<T: StT> {
        pub keys: Seq<KeyProb<T>>,
        pub memo: Map<(usize, usize), Probability>,
    }

    //		Section 12a. derive impls in verus!


    impl<T: StT> Clone for KeyProb<T> {
        fn clone(&self) -> (cloned: Self)
            // Veracity: NEEDED proof block (speed hint)
            ensures cloned == *self,
        {
            let cloned = KeyProb { key: self.key.clone(), prob: self.prob };
            proof { accept(cloned == *self); }  // accept hole: T::clone external_body
            cloned
        }
    }

    impl<T: StT> Eq for KeyProb<T> {}

    //		Section 12b. derive impls in verus!


    impl<T: StT> Clone for OBSTStPerS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            // Veracity: NEEDED proof block
            let cloned = OBSTStPerS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            };
            proof { accept(cloned@ == self@); }  // accept hole: Vec::clone external_body
            cloned
        }
    }
    } // verus!

    //		Section 14. derive impls outside verus!


    impl<'a, T: StT> IntoIterator for &'a OBSTStPerS<T> {
        type Item = KeyProb<T>;
        type IntoIter = Cloned<Iter<'a, KeyProb<T>>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — create cloned iterator adapter
        fn into_iter(self) -> Self::IntoIter { self.keys.iter().cloned() }
    }

    //		Section 14a. derive impls outside verus!

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

    //		Section 14b. derive impls outside verus!

    impl<T: StT> Display for OBSTStPerS<T> {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers
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

        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — move Vec into iterator
        fn into_iter(self) -> Self::IntoIter { self.keys.into_iter() }
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

    //		Section 14c. derive impls outside verus!

    impl<T: StT> Debug for OBSTStPerV<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OBSTStPerV") }
    }

    impl<T: StT> Display for OBSTStPerV<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "OBSTStPerV") }
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
