//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 49: Subset Sum - ephemeral, single-threaded.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod SubsetSumStEph {

    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::ArraySeqStEphSLit;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::seq::group_seq_axioms,
        crate::Types::Types::group_Pair_axioms,
        vstd::std_specs::hash::group_hash_axioms,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct SubsetSumStEphS<T: StT> {
        pub multiset: ArraySeqStEphS<T>,
        pub memo: HashMapWithViewPlus<Pair<usize, i32>, bool>,
    }

    //		Section 6. spec fns


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

    //		Section 8. traits


    /// Trait for subset sum operations.
    pub trait SubsetSumStEphTrait<T: StT>: Sized {
        /// Spec: multiset length.
        spec fn spec_multiset_len(&self) -> nat;

        /// Create new subset sum solver.
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- allocate empty structures.
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures empty.spec_multiset_len() == 0;

        /// Create from multiset.
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- move multiset into struct.
        fn from_multiset(multiset: ArraySeqStEphS<T>) -> (subset_sum: Self)
            ensures subset_sum.spec_multiset_len() == multiset.spec_len();

        /// Solve subset sum for the given target.
        /// - Alg Analysis: APAS (Ch49 Alg 49.2): Work O(k * |S|), Span O(|S|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k·|S|), Span O(k·|S|) — DIFFERS: sequential DP table fill, APAS Span O(|S|) assumes parallel
        fn subset_sum(&mut self, target: i32) -> (found: bool)
        where
            T: Into<i32> + Copy
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Get the multiset.
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return reference.
        fn multiset(&self) -> (ms: &ArraySeqStEphS<T>)
            ensures ms.spec_len() == self.spec_multiset_len();

        /// Set element at index (ephemeral mutation).
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- array set O(1) plus memo clear O(n).
        fn set(&mut self, index: usize, value: T)
            requires index < old(self).spec_multiset_len(),
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Clear memoization table.
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- clear hash map.
        fn clear_memo(&mut self)
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Get memoization table size.
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return cached length.
        fn memo_size(&self) -> (count: usize);
    }

    //		Section 9. impls


    /// Recursive memoized subset sum solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(k*|S|), Span O(|S|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k*|S|), Span O(k*|S|) — DIFFERS: sequential recursive memoized, Span = Work; APAS Span O(|S|) assumes parallel
    fn subset_sum_rec<T: StT + Into<i32> + Copy>(
        table: &mut SubsetSumStEphS<T>,
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

    impl<T: StT> SubsetSumStEphTrait<T> for SubsetSumStEphS<T> {
        open spec fn spec_multiset_len(&self) -> nat { self.multiset.spec_len() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new() -> Self
        where
            T: Default,
        {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset: ArraySeqStEphS::new(0, T::default()),
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction from components.
        fn from_multiset(multiset: ArraySeqStEphS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset,
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k*|S|), Span O(k*|S|) — memoized recursive DP; St sequential.
        fn subset_sum(&mut self, target: i32) -> (found: bool)
        where
            T: Into<i32> + Copy,
        {
            if target < 0 {
                return false;
            }

            self.memo.clear();

            let n = self.multiset.length();
            subset_sum_rec(self, n, target)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn multiset(&self) -> (ms: &ArraySeqStEphS<T>) { &self.multiset }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — array set O(1) plus memo clear O(n).
        fn set(&mut self, index: usize, value: T) {
            let _ = self.multiset.set(index, value);
            self.memo.clear();
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clear hash map.
        fn clear_memo(&mut self) { self.memo.clear(); }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — returns cached size.
        fn memo_size(&self) -> (count: usize) { self.memo.len() }
    }

    //		Section 12. derive impls in verus!


    impl<T: StT> Clone for SubsetSumStEphS<T> {
        fn clone(&self) -> (cloned: Self) {
            SubsetSumStEphS {
                multiset: self.multiset.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    // 12. traits outside verus!

    /// Trait for methods returning &mut (not supported inside verus!).
    pub trait SubsetSumStEphMutTrait<T: StT> {
        /// Get mutable multiset (ephemeral allows mutation).
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return mutable reference.
        fn multiset_mut(&mut self) -> &mut ArraySeqStEphS<T>;
    }

    impl<T: StT> SubsetSumStEphMutTrait<T> for SubsetSumStEphS<T> {
        fn multiset_mut(&mut self) -> &mut ArraySeqStEphS<T> { &mut self.multiset }
    }

    impl<T: StT> Debug for SubsetSumStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumStEphS")
                .field("multiset", &self.multiset)
                .field("memo", &self.memo.inner)
                .finish()
        }
    }

    impl<T: StT> Display for SubsetSumStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(
                f,
                "SubsetSumStEph(multiset: {}, memo_entries: {})",
                self.multiset,
                self.memo.inner.len()
            )
        }
    }

    impl<T: StT> IntoIterator for SubsetSumStEphS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStEphS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.into_iter() }
    }

    impl<T: StT> IntoIterator for &SubsetSumStEphS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStEphS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.clone().into_iter() }
    }

    impl<T: StT> IntoIterator for &mut SubsetSumStEphS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStEphS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.clone().into_iter() }
    }
}

#[macro_export]
macro_rules! SubsetSumStEphLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumStEph::SubsetSumStEph::SubsetSumStEphS::from_multiset(
            $crate::ArraySeqStEphSLit![$($x),*]
        )
    };
    () => {
        $crate::Chap49::SubsetSumStEph::SubsetSumStEph::SubsetSumStEphS::new()
    };
}
