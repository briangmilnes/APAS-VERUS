//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 49: Subset Sum - persistent, single-threaded.

pub mod SubsetSumStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::ArraySeqStPerSLit;

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::seq::group_seq_axioms,
        crate::Types::Types::group_Pair_axioms,
        vstd::std_specs::hash::group_hash_axioms,
    };

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct SubsetSumStPerS<T: StT> {
        pub multiset: ArraySeqStPerS<T>,
        pub memo: HashMapWithViewPlus<Pair<usize, i32>, bool>,
    }

    // 6. spec fns

    /// Recursive specification of the subset sum problem.
    /// Returns true iff some subset of s[0..i) sums to j.
    pub open spec fn spec_subset_sum(s: Seq<int>, i: nat, j: int) -> bool
        decreases i,
    {
        if j == 0 { true }
        else if i == 0 { false }
        else {
            let elem = s[i as int - 1];
            if elem > j {
                spec_subset_sum(s, (i - 1) as nat, j)
            } else {
                spec_subset_sum(s, (i - 1) as nat, j - elem)
                || spec_subset_sum(s, (i - 1) as nat, j)
            }
        }
    }

    // 8. traits

    /// Trait for subset sum operations.
    pub trait SubsetSumStPerTrait<T: StT>: Sized {
        /// Spec: multiset length.
        spec fn spec_multiset_len(&self) -> nat;

        /// Create new subset sum solver.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures empty.spec_multiset_len() == 0;

        /// Create from multiset.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_multiset(multiset: ArraySeqStPerS<T>) -> (subset_sum: Self)
            ensures subset_sum.spec_multiset_len() == multiset.spec_len();

        /// Solve subset sum for the given target.
        /// - Alg Analysis: APAS (Ch49 Alg 49.2): Work O(k * |S|), Span O(|S|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k·|S|), Span O(k·|S|) — DIFFERS: sequential DP table fill, APAS Span O(|S|) assumes parallel
        fn subset_sum(&self, target: i32) -> (found: bool)
        where
            T: Into<i32> + Copy;

        /// Get the multiset.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn multiset(&self) -> (ms: &ArraySeqStPerS<T>)
            ensures ms.spec_len() == self.spec_multiset_len();

        /// Get memoization table size.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// Recursive memoized subset sum solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(k×|S|), Span O(|S|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k×|S|), Span O(k×|S|) — DIFFERS: sequential recursive memoized, Span = Work; APAS Span O(|S|) assumes parallel
    fn subset_sum_rec<T: StT + Into<i32> + Copy>(
        table: &mut SubsetSumStPerS<T>,
        i: usize,
        j: i32,
    ) -> (found: bool)
        requires
            i <= old(table).multiset.spec_len(),
        ensures
            table.multiset.spec_len() == old(table).multiset.spec_len(),
        decreases i,
    {
        if let Some(cached) = table.memo.get(&Pair(i, j)) {
            return *cached;
        }

        let found = if j == 0 {
            true
        } else if i == 0 {
            false
        } else {
            let element_value: i32 = (*table.multiset.nth(i - 1)).into();
            if element_value < 0 || element_value > j {
                subset_sum_rec(table, i - 1, j)
            } else {
                // 0 <= element_value <= j, so j - element_value fits in i32.
                subset_sum_rec(table, i - 1, j - element_value)
                || subset_sum_rec(table, i - 1, j)
            }
        };

        table.memo.insert(Pair(i, j), found);
        found
    }

    impl<T: StT> SubsetSumStPerTrait<T> for SubsetSumStPerS<T> {
        open spec fn spec_multiset_len(&self) -> nat { self.multiset.spec_len() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new() -> Self
        where
            T: Default,
        {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset: ArraySeqStPerS::new(0, T::default()),
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction from components.
        fn from_multiset(multiset: ArraySeqStPerS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset,
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k*|S|), Span O(k*|S|) — clones + memoized recursive DP; St sequential.
        fn subset_sum(&self, target: i32) -> (found: bool)
        where
            T: Into<i32> + Copy,
        {
            if target < 0 {
                return false;
            }

            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            let mut solver = SubsetSumStPerS {
                multiset: self.multiset.clone(),
                memo: HashMapWithViewPlus::new(),
            };

            let n = solver.multiset.length();
            subset_sum_rec(&mut solver, n, target)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn multiset(&self) -> (ms: &ArraySeqStPerS<T>) { &self.multiset }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — returns cached size.
        fn memo_size(&self) -> (count: usize) { self.memo.len() }
    }

    // 11. derive impls in verus!

    impl<T: StT> Clone for SubsetSumStPerS<T> {
        fn clone(&self) -> (cloned: Self) {
            SubsetSumStPerS {
                multiset: self.multiset.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> PartialEq for SubsetSumStPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.multiset == other.multiset && self.memo.inner == other.memo.inner
        }
    }

    impl<T: StT> Eq for SubsetSumStPerS<T> {}

    impl<T: StT> Debug for SubsetSumStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumStPerS")
                .field("multiset", &self.multiset)
                .field("memo", &self.memo.inner)
                .finish()
        }
    }

    impl<T: StT> Display for SubsetSumStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(
                f,
                "SubsetSumStPer(multiset: {}, memo_entries: {})",
                self.multiset,
                self.memo.inner.len()
            )
        }
    }

    impl<T: StT> IntoIterator for SubsetSumStPerS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStPerS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.into_iter() }
    }

    impl<T: StT> IntoIterator for &SubsetSumStPerS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStPerS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.clone().into_iter() }
    }
}

#[macro_export]
macro_rules! SubsetSumStPerLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumStPer::SubsetSumStPer::SubsetSumStPerS::from_multiset(
            $crate::ArraySeqStPerSLit![$($x),*]
        )
    };
    () => {
        $crate::Chap49::SubsetSumStPer::SubsetSumStPer::SubsetSumStPerS::new()
    };
}
