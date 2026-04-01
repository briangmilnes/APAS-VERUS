//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 49: Subset Sum - persistent, multi-threaded.

pub mod SubsetSumMtPer {

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
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::seq::group_seq_axioms,
        crate::Types::Types::group_Pair_axioms,
        vstd::std_specs::hash::group_hash_axioms,
    };

    // 4. type definitions

    pub struct SubsetSumMtPerMemoInv;
    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, i32>, bool>> for SubsetSumMtPerMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, i32>, bool>) -> bool {
            v@.dom().finite()
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct SubsetSumMtPerS<T: MtVal> {
        pub multiset: ArraySeqMtPerS<T>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtPerMemoInv>>,
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

    /// Trait for parallel subset sum operations.
    pub trait SubsetSumMtPerTrait<T: MtVal>: Sized {
        /// Spec: multiset length.
        spec fn spec_multiset_len(&self) -> nat;

        /// Well-formedness: the memo lock carries the correct predicate.
        spec fn spec_subsetsummtper_wf(&self) -> bool;

        /// Create new subset sum solver.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures
                empty.spec_subsetsummtper_wf(),
                empty.spec_multiset_len() == 0;

        /// Create from multiset.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_multiset(multiset: ArraySeqMtPerS<T>) -> (subset_sum: Self)
            ensures
                subset_sum.spec_subsetsummtper_wf(),
                subset_sum.spec_multiset_len() == multiset.spec_len();

        /// Solve subset sum for the given target.
        /// - Alg Analysis: APAS (Ch49 Alg 49.2): Work O(k * |S|), Span O(|S|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k·|S|), Span O(k·|S|) — DIFFERS: sequential DP table fill, APAS Span O(|S|) assumes parallel
        fn subset_sum(&self, target: i32) -> (found: bool)
        where
            T: Into<i32> + Copy + Send + Sync + 'static
            requires self.spec_subsetsummtper_wf();

        /// Get the multiset.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn multiset(&self) -> (ms: &ArraySeqMtPerS<T>)
            ensures ms.spec_len() == self.spec_multiset_len();

        /// Get memoization table size.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// Create Arc-wrapped memo lock with empty map.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    fn new_arc_memo(
        val: HashMapWithViewPlus<Pair<usize, i32>, bool>,
    ) -> (memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtPerMemoInv>>)
        requires val@.dom().finite(),
        ensures memo.pred() == SubsetSumMtPerMemoInv,
    {
        new_arc_rwlock(val, Ghost(SubsetSumMtPerMemoInv))
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    /// Clone Arc memo (reference count increment).
    fn clone_arc_memo<T: MtVal>(
        s: &SubsetSumMtPerS<T>,
    ) -> (cloned: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtPerMemoInv>>)
        requires s.memo.pred() == SubsetSumMtPerMemoInv,
        ensures cloned.pred() == s.memo.pred(),
    {
        clone_arc_rwlock(&s.memo)
    }

    /// Recursive memoized parallel subset sum solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(k×|S|), Span O(|S|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k×|S|), Span O(|S|) — matches APAS
    fn subset_sum_rec<T: MtVal + Into<i32> + Copy + Send + Sync + 'static>(
        multiset: &ArraySeqMtPerS<T>,
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtPerMemoInv>>,
        i: usize,
        j: i32,
    ) -> (found: bool)
        requires
            i <= multiset.spec_len(),
            memo.pred() == SubsetSumMtPerMemoInv,
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
                        memo1.pred() == SubsetSumMtPerMemoInv,
                {
                    subset_sum_rec(&multiset1, &memo1, i - 1, j - element_value)
                };
                let f2 = move || -> (r: bool)
                    requires
                        i - 1 <= multiset2.spec_len(),
                        memo2.pred() == SubsetSumMtPerMemoInv,
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

    impl<T: MtVal> SubsetSumMtPerTrait<T> for SubsetSumMtPerS<T> {
        open spec fn spec_multiset_len(&self) -> nat { self.multiset.spec_len() }

        open spec fn spec_subsetsummtper_wf(&self) -> bool {
            self.memo.pred() == SubsetSumMtPerMemoInv
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        }

        fn new() -> Self
        where
            T: Default,
        {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset: ArraySeqMtPerS::new(0, T::default()),
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction from components.
            }
        }

        fn from_multiset(multiset: ArraySeqMtPerS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset,
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n*m), Span O(n+m) — DP with parallel diagonal wavefront; Mt parallel.
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        fn subset_sum(&self, target: i32) -> (found: bool)
        where
            T: Into<i32> + Copy + Send + Sync + 'static,
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
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — returns cached size.
            let n = self.multiset.length();
            subset_sum_rec(&self.multiset, &self.memo, n, target)
        }

        fn multiset(&self) -> (ms: &ArraySeqMtPerS<T>) { &self.multiset }

        fn memo_size(&self) -> (count: usize) {
            let handle = self.memo.acquire_read();
            let size = handle.borrow().len();
            handle.release_read();
            size
        }
    }

    // 11. derive impls in verus!

    impl<T: MtVal> Clone for SubsetSumMtPerS<T> {
        fn clone(&self) -> (cloned: Self) {
            SubsetSumMtPerS {
                multiset: self.multiset.clone(),
                memo: clone_arc_memo(self),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: MtVal> PartialEq for SubsetSumMtPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.multiset == other.multiset }
    }

    impl<T: MtVal> Eq for SubsetSumMtPerS<T> {}

    impl<T: MtVal> Debug for SubsetSumMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumMtPerS")
                .field("multiset", &self.multiset)
                .finish()
        }
    }

    impl<T: MtVal> Display for SubsetSumMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let memo_size = {
                let handle = self.memo.acquire_read();
                let size = handle.borrow().len();
                handle.release_read();
                size
            };
            write!(
                f,
                "SubsetSumMtPer(multiset: {}, memo_entries: {})",
                self.multiset, memo_size
            )
        }
    }
}

#[macro_export]
macro_rules! SubsetSumMtPerLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumMtPer::SubsetSumMtPer::SubsetSumMtPerS::from_multiset(
            <$crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS<_> as $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerBaseTrait<_>>::from_vec(vec![$($x),*])
        )
    };
    () => {
        $crate::Chap49::SubsetSumMtPer::SubsetSumMtPer::SubsetSumMtPerS::new()
    };
}
