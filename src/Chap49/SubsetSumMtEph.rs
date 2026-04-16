// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 49: Subset Sum - ephemeral, multi-threaded.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 6a. spec fns
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 9b. impls
//	Section 11a. top level coarse locking
//	Section 12b. derive impls in verus!
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!


//		Section 1. module

pub mod SubsetSumMtEph {

    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::ArraySeqMtEphChap19SLit;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::seq::group_seq_axioms,
        crate::Types::Types::group_Pair_axioms,
        vstd::std_specs::hash::group_hash_axioms,
    };

    //		Section 4a. type definitions


    pub struct SubsetSumMtEphMemoInv;

    //		Section 6a. spec fns


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

    //		Section 8a. traits


    /// Trait for parallel subset sum operations.
    pub trait SubsetSumMtEphTrait<T: MtVal>: Sized {
        /// Spec: multiset length.
        spec fn spec_multiset_len(&self) -> nat;

        /// Well-formedness: the memo lock carries the correct predicate.
        spec fn spec_subsetsummteph_wf(&self) -> bool;

        /// Create new subset sum solver.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures
                empty.spec_subsetsummteph_wf(),
                empty.spec_multiset_len() == 0;

        /// Create from multiset.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> (subset_sum: Self)
            ensures
                subset_sum.spec_subsetsummteph_wf(),
                subset_sum.spec_multiset_len() == multiset.spec_len();

        /// Solve subset sum for the given target.
        /// - Alg Analysis: APAS (Ch49 Alg 49.2): Work O(k * |S|), Span O(|S|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k·|S|), Span O(|S|) — parallel recursive memoized with join; matches APAS
        fn subset_sum(&mut self, target: i32) -> (found: bool)
            where T: Into<i32> + Copy,
            requires old(self).spec_subsetsummteph_wf(),
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Get the multiset.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn multiset(&self) -> (ms: &ArraySeqMtEphS<T>)
            ensures ms.spec_len() == self.spec_multiset_len();

        /// Set element at index (ephemeral mutation).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn set(&mut self, index: usize, value: T)
            requires
                old(self).spec_subsetsummteph_wf(),
                index < old(self).spec_multiset_len(),
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Clear memoization table.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn clear_memo(&mut self)
            requires old(self).spec_subsetsummteph_wf(),
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Get memoization table size.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    //		Section 9a. impls


    /// Create Arc-wrapped memo lock with empty map.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    fn new_arc_memo(
        val: HashMapWithViewPlus<Pair<usize, i32>, bool>,
    ) -> (memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtEphMemoInv>>)
        requires val@.dom().finite(),
        ensures memo.pred() == SubsetSumMtEphMemoInv,
    {
        new_arc_rwlock(val, Ghost(SubsetSumMtEphMemoInv))
    }

    /// Clone Arc memo (reference count increment).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    fn clone_arc_memo<T: MtVal>(
        s: &SubsetSumMtEphS<T>,
    ) -> (cloned: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtEphMemoInv>>)
        requires s.memo.pred() == SubsetSumMtEphMemoInv,
        ensures cloned.pred() == s.memo.pred(),
    {
        clone_arc_rwlock(&s.memo)
    }

    /// Recursive memoized parallel subset sum solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(k×|S|), Span O(|S|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k×|S|), Span O(|S|)
    fn subset_sum_rec<T: MtVal + Into<i32> + Copy>(
        multiset: &ArraySeqMtEphS<T>,
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtEphMemoInv>>,
        i: usize,
        j: i32,
    ) -> (found: bool)
        requires
            i <= multiset.spec_len(),
            memo.pred() == SubsetSumMtEphMemoInv,
        ensures true,
        decreases i,
    {
        // Memo lookup.
        {
            let rwlock = arc_deref(memo);
            let handle = rwlock.acquire_read();
            let found = match handle.borrow().get(&Pair(i, j)) {
                Some(v) => Some(*v),
                None => None,
            };
            handle.release_read();
            if let Some(result) = found {
                return result;
            }
        }

        let result = if j == 0 {
            true
        } else if i == 0 {
            false
        } else {
            let element_value: i32 = (*multiset.nth(i - 1)).clone().into();
            if element_value < 0 || element_value > j {
                subset_sum_rec(multiset, memo, i - 1, j)
            } else {
                let multiset1 = multiset.clone();
                let memo1 = clone_arc_rwlock(memo);
                let multiset2 = multiset.clone();
                let memo2 = clone_arc_rwlock(memo);

                let f1 = move || -> (r: bool)
                    requires
                        i - 1 <= multiset1.spec_len(),
                        memo1.pred() == SubsetSumMtEphMemoInv,
                {
                    subset_sum_rec(&multiset1, &memo1, i - 1, j - element_value)
                };
                let f2 = move || -> (r: bool)
                    requires
                        i - 1 <= multiset2.spec_len(),
                        memo2.pred() == SubsetSumMtEphMemoInv,
                {
                    subset_sum_rec(&multiset2, &memo2, i - 1, j)
                };
                let (result1, result2) = join(f1, f2);

                result1 || result2
            }
        };

        // Memo store.
        {
            let rwlock = arc_deref(memo);
            let (mut current, write_handle) = rwlock.acquire_write();
            current.insert(Pair(i, j), result);
            write_handle.release_write(current);
        }

        result
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct SubsetSumMtEphS<T: MtVal> {
        pub multiset: ArraySeqMtEphS<T>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtEphMemoInv>>,
    }

    //		Section 9b. impls


    impl<T: MtVal> SubsetSumMtEphTrait<T> for SubsetSumMtEphS<T> {
        open spec fn spec_multiset_len(&self) -> nat { self.multiset.spec_len() }

        open spec fn spec_subsetsummteph_wf(&self) -> bool {
            self.memo.pred() == SubsetSumMtEphMemoInv
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new() -> Self
        where
            T: Default,
        {
            // Veracity: NEEDED proof block
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset: ArraySeqMtEphS::new(0, T::default()),
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction from components.
        // Veracity: NEEDED proof block
        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset,
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k*|S|), Span O(|S|) — memoized recursive DP with parallel join; Mt parallel.
        fn subset_sum(&mut self, target: i32) -> (found: bool)
            where T: Into<i32> + Copy,
        {
            if target < 0 {
                return false;
            }

            {
                let rwlock = arc_deref(&self.memo);
                let (mut memo, write_handle) = rwlock.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            let n = self.multiset.length();
            subset_sum_rec(&self.multiset, &self.memo, n, target)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn multiset(&self) -> (ms: &ArraySeqMtEphS<T>) { &self.multiset }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — array set O(1) plus memo clear O(n).
        fn set(&mut self, index: usize, value: T) {
            let _ = self.multiset.set(index, value);
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clear hash map under lock.
        fn clear_memo(&mut self) {
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read lock plus return cached size.
        fn memo_size(&self) -> (count: usize) {
            let handle = self.memo.acquire_read();
            let size = handle.borrow().len();
            handle.release_read();
            size
        }
    }

    //		Section 11a. top level coarse locking


    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, i32>, bool>> for SubsetSumMtEphMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, i32>, bool>) -> bool {
            v@.dom().finite()
        }
    }

    //		Section 12b. derive impls in verus!


    impl<T: MtVal> Clone for SubsetSumMtEphS<T> {
        fn clone(&self) -> (cloned: Self) {
            SubsetSumMtEphS {
                multiset: self.multiset.clone(),
                memo: clone_arc_memo(self),
            }
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    // 12. traits outside verus!

    /// Trait for methods returning &mut (not supported inside verus!).
    pub trait SubsetSumMtEphMutTrait<T: MtVal> {
        /// Get mutable multiset (ephemeral allows mutation).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn multiset_mut(&mut self) -> &mut ArraySeqMtEphS<T>;
    }

    //		Section 14a. derive impls outside verus!

    impl Debug for SubsetSumMtEphMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "SubsetSumMtEphMemoInv")
        }
    }

    impl Display for SubsetSumMtEphMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "SubsetSumMtEphMemoInv")
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: MtVal> SubsetSumMtEphMutTrait<T> for SubsetSumMtEphS<T> {
        fn multiset_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.multiset }
    }

    impl<T: MtVal> PartialEq for SubsetSumMtEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.multiset == other.multiset }
    }

    impl<T: MtVal> Eq for SubsetSumMtEphS<T> {}

    impl<T: MtVal> Debug for SubsetSumMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumMtEphS")
                .field("multiset", &self.multiset)
                .finish()
        }
    }

    impl<T: MtVal> Display for SubsetSumMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let memo_size = {
                let handle = self.memo.acquire_read();
                let size = handle.borrow().len();
                handle.release_read();
                size
            };
            write!(
                f,
                "SubsetSumMtEph(multiset: {}, memo_entries: {})",
                self.multiset, memo_size
            )
        }
    }
}

#[macro_export]
macro_rules! SubsetSumMtEphLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumMtEph::SubsetSumMtEph::SubsetSumMtEphS::from_multiset(
            $crate::ArraySeqMtEphChap19SLit![$($x),*]
        )
    };
    () => {
        $crate::Chap49::SubsetSumMtEph::SubsetSumMtEph::SubsetSumMtEphS::new()
    };
}
