//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 49: Minimum Edit Distance - ephemeral, multi-threaded.

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

pub mod MinEditDistMtEph {

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


    pub struct MinEditDistMtEphMemoInv;

    //		Section 6a. spec fns


    /// Recursive specification of minimum edit distance.
    /// Returns the minimum number of insert/delete operations to transform s[0..i) into t[0..j).
    pub open spec fn spec_med(s: Seq<int>, t: Seq<int>, i: nat, j: nat) -> nat
        decreases i + j,
    {
        if i == 0 { j }
        else if j == 0 { i }
        else if s[i as int - 1] == t[j as int - 1] {
            spec_med(s, t, (i - 1) as nat, (j - 1) as nat)
        } else {
            let delete_cost = spec_med(s, t, (i - 1) as nat, j);
            let insert_cost = spec_med(s, t, i, (j - 1) as nat);
            1 + if delete_cost <= insert_cost { delete_cost } else { insert_cost }
        }
    }

    //		Section 8a. traits


    /// Trait for parallel minimum edit distance operations.
    pub trait MinEditDistMtEphTrait<T: MtVal>: Sized {
        /// Spec: source length.
        spec fn spec_source_len(&self) -> nat;

        /// Spec: target length.
        spec fn spec_target_len(&self) -> nat;

        /// Well-formedness: the memo lock carries the correct predicate.
        spec fn spec_mineditdistmteph_wf(&self) -> bool;

        /// Create new minimum edit distance solver.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures
                empty.spec_mineditdistmteph_wf(),
                empty.spec_source_len() == 0,
                empty.spec_target_len() == 0;

        /// Create from source and target sequences.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_sequences(source: ArraySeqMtEphS<T>, target: ArraySeqMtEphS<T>) -> (edit_dist: Self)
            ensures
                edit_dist.spec_mineditdistmteph_wf(),
                edit_dist.spec_source_len() == source.spec_len(),
                edit_dist.spec_target_len() == target.spec_len();

        /// Compute minimum edit distance.
        /// - Alg Analysis: APAS (Ch49 Alg 49.5): Work O(|S| * |T|), Span O(|S| + |T|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|·|T|), Span O(|S|+|T|) — parallel recursive memoized with join; matches APAS
        fn min_edit_distance(&mut self) -> (dist: usize)
            requires
                old(self).spec_mineditdistmteph_wf(),
                old(self).spec_source_len() + old(self).spec_target_len() < usize::MAX,
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Get the source sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn source(&self) -> (s: &ArraySeqMtEphS<T>)
            ensures s.spec_len() == self.spec_source_len();

        /// Get the target sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn target(&self) -> (t: &ArraySeqMtEphS<T>)
            ensures t.spec_len() == self.spec_target_len();

        /// Set element in source sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn set_source(&mut self, index: usize, value: T)
            requires
                old(self).spec_mineditdistmteph_wf(),
                index < old(self).spec_source_len(),
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Set element in target sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn set_target(&mut self, index: usize, value: T)
            requires
                old(self).spec_mineditdistmteph_wf(),
                index < old(self).spec_target_len(),
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Clear memoization table.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn clear_memo(&mut self)
            requires old(self).spec_mineditdistmteph_wf(),
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Get memoization table size.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    //		Section 9a. impls


    /// Create Arc-wrapped memo lock.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    fn new_arc_memo(
        val: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    ) -> (memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>)
        requires val@.dom().finite(),
        ensures memo.pred() == MinEditDistMtEphMemoInv,
    {
        new_arc_rwlock(val, Ghost(MinEditDistMtEphMemoInv))
    }

    /// Clone Arc memo (reference count increment).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    fn clone_arc_memo<T: MtVal>(
        s: &MinEditDistMtEphS<T>,
    ) -> (cloned: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>)
        requires s.memo.pred() == MinEditDistMtEphMemoInv,
        ensures cloned.pred() == s.memo.pred(),
    {
        clone_arc_rwlock(&s.memo)
    }

    /// Recursive memoized parallel minimum edit distance solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(|S|×|T|), Span O(|S|+|T|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|×|T|), Span O(|S|+|T|) — matches APAS
    fn min_edit_distance_rec<T: MtVal>(
        source: &ArraySeqMtEphS<T>,
        target: &ArraySeqMtEphS<T>,
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>,
        i: usize,
        j: usize,
    ) -> (dist: usize)
        requires
            i <= source.spec_len(),
            j <= target.spec_len(),
            source.spec_len() + target.spec_len() < usize::MAX,
            memo.pred() == MinEditDistMtEphMemoInv,
        ensures
            dist <= i + j,
        decreases i + j,
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
                if result <= i + j {
                    return result;
                }
            }
        }

        let dist = if i == 0 {
            j
        } else if j == 0 {
            i
        } else {
            let source_char = source.nth(i - 1).clone();
            let target_char = target.nth(j - 1).clone();

            if source_char == target_char {
                min_edit_distance_rec(source, target, memo, i - 1, j - 1)
            } else {
                let source1 = source.clone();
                let target1 = target.clone();
                let memo1 = clone_arc_rwlock(memo);
                let source2 = source.clone();
                let target2 = target.clone();
                let memo2 = clone_arc_rwlock(memo);

                let ghost source_len = source.spec_len();
                let ghost target_len = target.spec_len();

                let f1 = move || -> (r: usize)
                    requires
                        i - 1 <= source1.spec_len(),
                        j <= target1.spec_len(),
                        source1.spec_len() + target1.spec_len() < usize::MAX,
                        memo1.pred() == MinEditDistMtEphMemoInv,
                    ensures r <= (i - 1) + j,
                {
                    min_edit_distance_rec(&source1, &target1, &memo1, i - 1, j)
                };
                let f2 = move || -> (r: usize)
                    requires
                        i <= source2.spec_len(),
                        j - 1 <= target2.spec_len(),
                        source2.spec_len() + target2.spec_len() < usize::MAX,
                        memo2.pred() == MinEditDistMtEphMemoInv,
                    ensures r <= i + (j - 1),
                {
                    min_edit_distance_rec(&source2, &target2, &memo2, i, j - 1)
                };
                let (delete_cost, insert_cost) = join(f1, f2);

                if delete_cost <= insert_cost {
                    1 + delete_cost
                } else {
                    1 + insert_cost
                }
            }
        };

        // Memo store.
        {
            let rwlock = arc_deref(memo);
            let (mut current, write_handle) = rwlock.acquire_write();
            current.insert(Pair(i, j), dist);
            write_handle.release_write(current);
        }

        dist
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct MinEditDistMtEphS<T: MtVal> {
        pub source: ArraySeqMtEphS<T>,
        pub target: ArraySeqMtEphS<T>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>,
    }

    //		Section 9b. impls


    impl<T: MtVal> MinEditDistMtEphTrait<T> for MinEditDistMtEphS<T> {
        open spec fn spec_source_len(&self) -> nat { self.source.spec_len() }

        open spec fn spec_target_len(&self) -> nat { self.target.spec_len() }

        open spec fn spec_mineditdistmteph_wf(&self) -> bool {
            self.memo.pred() == MinEditDistMtEphMemoInv
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new() -> Self
        where
            T: Default,
        {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source: ArraySeqMtEphS::new(0, T::default()),
                target: ArraySeqMtEphS::new(0, T::default()),
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction from components.
        fn from_sequences(source: ArraySeqMtEphS<T>, target: ArraySeqMtEphS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source,
                target,
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|*|T|), Span O(|S|+|T|) — memoized recursive DP with parallel join; Mt parallel.
        fn min_edit_distance(&mut self) -> (dist: usize)
        {
            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            let source_len = self.source.length();
            let target_len = self.target.length();

            min_edit_distance_rec(&self.source, &self.target, &self.memo, source_len, target_len)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn source(&self) -> (s: &ArraySeqMtEphS<T>) { &self.source }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn target(&self) -> (t: &ArraySeqMtEphS<T>) { &self.target }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — field write O(1) plus memo clear O(n).
        fn set_source(&mut self, index: usize, value: T) {
            let _ = self.source.set(index, value);
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — field write O(1) plus memo clear O(n).
        fn set_target(&mut self, index: usize, value: T) {
            let _ = self.target.set(index, value);
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


    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for MinEditDistMtEphMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
            v@.dom().finite()
        }
    }

    //		Section 12b. derive impls in verus!


    impl<T: MtVal> Clone for MinEditDistMtEphS<T> {
        fn clone(&self) -> (cloned: Self) {
            MinEditDistMtEphS {
                source: self.source.clone(),
                target: self.target.clone(),
                memo: clone_arc_memo(self),
            }
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    // 12. traits outside verus!

    /// Trait for methods returning &mut (not supported inside verus!).
    pub trait MinEditDistMtEphMutTrait<T: MtVal> {
        /// Get mutable source sequence (ephemeral allows mutation).
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn source_mut(&mut self) -> &mut ArraySeqMtEphS<T>;

        /// Get mutable target sequence (ephemeral allows mutation).
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn target_mut(&mut self) -> &mut ArraySeqMtEphS<T>;
    }

    //		Section 14a. derive impls outside verus!

    impl Debug for MinEditDistMtEphMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "MinEditDistMtEphMemoInv")
        }
    }

    impl Display for MinEditDistMtEphMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "MinEditDistMtEphMemoInv")
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: MtVal> MinEditDistMtEphMutTrait<T> for MinEditDistMtEphS<T> {
        fn source_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.source }
        fn target_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.target }
    }

    impl<T: MtVal> PartialEq for MinEditDistMtEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.source == other.source && self.target == other.target }
    }

    impl<T: MtVal> Eq for MinEditDistMtEphS<T> {}

    impl<T: MtVal> Debug for MinEditDistMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MinEditDistMtEphS")
                .field("source", &self.source)
                .field("target", &self.target)
                .finish()
        }
    }

    impl<T: MtVal> Display for MinEditDistMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let memo_size = {
                let handle = self.memo.acquire_read();
                let size = handle.borrow().len();
                handle.release_read();
                size
            };
            write!(
                f,
                "MinEditDistMtEph(source: {}, target: {}, memo_entries: {})",
                self.source, self.target, memo_size
            )
        }
    }
}

#[macro_export]
macro_rules! MinEditDistMtEphLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistMtEph::MinEditDistMtEph::MinEditDistMtEphS::from_sequences(
            $crate::ArraySeqMtEphChap19SLit![$($s),*],
            $crate::ArraySeqMtEphChap19SLit![$($t),*]
        )
    };
    () => {
        $crate::Chap49::MinEditDistMtEph::MinEditDistMtEph::MinEditDistMtEphS::new()
    };
}
