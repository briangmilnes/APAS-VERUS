//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - ephemeral, multi-threaded.

pub mod SubsetSumMtEph {

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
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::ArraySeqMtEphChap19SLit;

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::seq::group_seq_axioms,
        crate::Types::Types::group_Pair_axioms,
        vstd::std_specs::hash::group_hash_axioms,
    };

    // 4. type definitions

    pub struct SubsetSumMtEphMemoInv;
    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, i32>, bool>> for SubsetSumMtEphMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, i32>, bool>) -> bool {
            v@.dom().finite()
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct SubsetSumMtEphS<T: MtVal> {
        pub multiset: ArraySeqMtEphS<T>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtEphMemoInv>>,
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
    pub trait SubsetSumMtEphTrait<T: MtVal>: Sized {
        /// Spec: multiset length.
        spec fn spec_multiset_len(&self) -> nat;

        /// Well-formedness: the memo lock carries the correct predicate.
        spec fn spec_subsetsummteph_wf(&self) -> bool;

        /// Create new subset sum solver.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures
                empty.spec_subsetsummteph_wf(),
                empty.spec_multiset_len() == 0;

        /// Create from multiset.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> (subset_sum: Self)
            ensures
                subset_sum.spec_subsetsummteph_wf(),
                subset_sum.spec_multiset_len() == multiset.spec_len();

        /// Solve subset sum for the given target.
        /// - Alg Analysis: APAS (Ch49 Alg 49.2): Work O(k * |S|), Span O(|S|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k·|S|), Span O(k·|S|) — DIFFERS: sequential DP table fill, APAS Span O(|S|) assumes parallel
        fn subset_sum(&mut self, target: i32) -> (found: bool)
        where
            T: Into<i32> + Copy + Send + Sync + 'static
            requires old(self).spec_subsetsummteph_wf(),
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Get the multiset.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn multiset(&self) -> (ms: &ArraySeqMtEphS<T>)
            ensures ms.spec_len() == self.spec_multiset_len();

        /// Set element at index (ephemeral mutation).
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn set(&mut self, index: usize, value: T)
            requires
                old(self).spec_subsetsummteph_wf(),
                index < old(self).spec_multiset_len(),
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Clear memoization table.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn clear_memo(&mut self)
            requires old(self).spec_subsetsummteph_wf(),
            ensures self.spec_multiset_len() == old(self).spec_multiset_len();

        /// Get memoization table size.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// Create Arc-wrapped memo lock with empty map.
    fn new_arc_memo(
        val: HashMapWithViewPlus<Pair<usize, i32>, bool>,
    ) -> (memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, i32>, bool>, SubsetSumMtEphMemoInv>>)
        requires val@.dom().finite(),
        ensures memo.pred() == SubsetSumMtEphMemoInv,
    {
        new_arc_rwlock(val, Ghost(SubsetSumMtEphMemoInv))
    }

    /// Clone Arc memo (reference count increment).
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k×|S|), Span O(|S|) — matches APAS
    fn subset_sum_rec<T: MtVal + Into<i32> + Copy + Send + Sync + 'static>(
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

    impl<T: MtVal> SubsetSumMtEphTrait<T> for SubsetSumMtEphS<T> {
        open spec fn spec_multiset_len(&self) -> nat { self.multiset.spec_len() }

        open spec fn spec_subsetsummteph_wf(&self) -> bool {
            self.memo.pred() == SubsetSumMtEphMemoInv
        }

        fn new() -> Self
        where
            T: Default,
        {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset: ArraySeqMtEphS::new(0, T::default()),
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, i32>(); }
            Self {
                multiset,
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        fn subset_sum(&mut self, target: i32) -> (found: bool)
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

            let n = self.multiset.length();
            subset_sum_rec(&self.multiset, &self.memo, n, target)
        }

        fn multiset(&self) -> (ms: &ArraySeqMtEphS<T>) { &self.multiset }

        fn set(&mut self, index: usize, value: T) {
            let _ = self.multiset.set(index, value);
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        fn clear_memo(&mut self) {
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        fn memo_size(&self) -> (count: usize) {
            let handle = self.memo.acquire_read();
            let size = handle.borrow().len();
            handle.release_read();
            size
        }
    }

    // 11. derive impls in verus!

    impl<T: MtVal> Clone for SubsetSumMtEphS<T> {
        fn clone(&self) -> (cloned: Self) {
            SubsetSumMtEphS {
                multiset: self.multiset.clone(),
                memo: clone_arc_memo(self),
            }
        }
    }

    } // verus!

    // 12. traits outside verus!

    /// Trait for methods returning &mut (not supported inside verus!).
    pub trait SubsetSumMtEphMutTrait<T: MtVal> {
        /// Get mutable multiset (ephemeral allows mutation).
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn multiset_mut(&mut self) -> &mut ArraySeqMtEphS<T>;
    }

    impl<T: MtVal> SubsetSumMtEphMutTrait<T> for SubsetSumMtEphS<T> {
        fn multiset_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.multiset }
    }

    // 13. derive impls outside verus!

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
