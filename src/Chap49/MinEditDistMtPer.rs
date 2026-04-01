//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 49: Minimum Edit Distance - persistent, multi-threaded.

pub mod MinEditDistMtPer {

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

    pub struct MinEditDistMtPerMemoInv;
    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for MinEditDistMtPerMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
            v@.dom().finite()
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct MinEditDistMtPerS<T: MtVal> {
        pub source: ArraySeqMtPerS<T>,
        pub target: ArraySeqMtPerS<T>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtPerMemoInv>>,
    }

    // 6. spec fns

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

    // 8. traits

    /// Trait for parallel minimum edit distance operations.
    pub trait MinEditDistMtPerTrait<T: MtVal>: Sized {
        /// Spec: source length.
        spec fn spec_source_len(&self) -> nat;

        /// Spec: target length.
        spec fn spec_target_len(&self) -> nat;

        /// Well-formedness: the memo lock carries the correct predicate.
        spec fn spec_mineditdistmtper_wf(&self) -> bool;

        /// Create new minimum edit distance solver.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures
                empty.spec_mineditdistmtper_wf(),
                empty.spec_source_len() == 0,
                empty.spec_target_len() == 0;

        /// Create from source and target sequences.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_sequences(source: ArraySeqMtPerS<T>, target: ArraySeqMtPerS<T>) -> (edit_dist: Self)
            ensures
                edit_dist.spec_mineditdistmtper_wf(),
                edit_dist.spec_source_len() == source.spec_len(),
                edit_dist.spec_target_len() == target.spec_len();

        /// Compute minimum edit distance.
        /// - Alg Analysis: APAS (Ch49 Alg 49.5): Work O(|S| * |T|), Span O(|S| + |T|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|·|T|), Span O(|S|+|T|) — parallel recursive memoized with join; matches APAS
        fn min_edit_distance(&self) -> (dist: usize)
        where
            T: Send + Sync + 'static
            requires
                self.spec_mineditdistmtper_wf(),
                self.spec_source_len() + self.spec_target_len() < usize::MAX;

        /// Get the source sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn source(&self) -> (s: &ArraySeqMtPerS<T>)
            ensures s.spec_len() == self.spec_source_len();

        /// Get the target sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn target(&self) -> (t: &ArraySeqMtPerS<T>)
            ensures t.spec_len() == self.spec_target_len();

        /// Get memoization table size.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// Create Arc-wrapped memo lock.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    fn new_arc_memo(
        val: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    ) -> (memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtPerMemoInv>>)
        requires val@.dom().finite(),
        ensures memo.pred() == MinEditDistMtPerMemoInv,
    {
        new_arc_rwlock(val, Ghost(MinEditDistMtPerMemoInv))
    }

    /// Clone Arc memo (reference count increment).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc/memo operations.
    fn clone_arc_memo<T: MtVal>(
        s: &MinEditDistMtPerS<T>,
    ) -> (cloned: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtPerMemoInv>>)
        requires s.memo.pred() == MinEditDistMtPerMemoInv,
        ensures cloned.pred() == s.memo.pred(),
    {
        clone_arc_rwlock(&s.memo)
    }

    /// Recursive memoized parallel minimum edit distance solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(|S|×|T|), Span O(|S|+|T|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|×|T|), Span O(|S|+|T|) — matches APAS
    fn min_edit_distance_rec<T: MtVal + Send + Sync + 'static>(
        source: &ArraySeqMtPerS<T>,
        target: &ArraySeqMtPerS<T>,
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtPerMemoInv>>,
        i: usize,
        j: usize,
    ) -> (dist: usize)
        requires
            i <= source.spec_len(),
            j <= target.spec_len(),
            source.spec_len() + target.spec_len() < usize::MAX,
            memo.pred() == MinEditDistMtPerMemoInv,
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

                let f1 = move || -> (r: usize)
                    requires
                        i - 1 <= source1.spec_len(),
                        j <= target1.spec_len(),
                        source1.spec_len() + target1.spec_len() < usize::MAX,
                        memo1.pred() == MinEditDistMtPerMemoInv,
                    ensures r <= (i - 1) + j,
                {
                    min_edit_distance_rec(&source1, &target1, &memo1, i - 1, j)
                };
                let f2 = move || -> (r: usize)
                    requires
                        i <= source2.spec_len(),
                        j - 1 <= target2.spec_len(),
                        source2.spec_len() + target2.spec_len() < usize::MAX,
                        memo2.pred() == MinEditDistMtPerMemoInv,
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

    impl<T: MtVal> MinEditDistMtPerTrait<T> for MinEditDistMtPerS<T> {
        open spec fn spec_source_len(&self) -> nat { self.source.spec_len() }

        open spec fn spec_target_len(&self) -> nat { self.target.spec_len() }

        open spec fn spec_mineditdistmtper_wf(&self) -> bool {
            self.memo.pred() == MinEditDistMtPerMemoInv
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new() -> Self
        where
            T: Default,
        {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source: ArraySeqMtPerS::new(0, T::default()),
                target: ArraySeqMtPerS::new(0, T::default()),
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction from components.
        fn from_sequences(source: ArraySeqMtPerS<T>, target: ArraySeqMtPerS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source,
                target,
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|*|T|), Span O(|S|+|T|) — clones + memoized recursive DP with parallel join; Mt parallel.
        fn min_edit_distance(&self) -> (dist: usize)
        where
            T: Send + Sync + 'static,
        {
            {
                let rwlock = arc_deref(&self.memo);
                let (mut memo, write_handle) = rwlock.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            let source_len = self.source.length();
            let target_len = self.target.length();

            min_edit_distance_rec(&self.source, &self.target, &self.memo, source_len, target_len)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn source(&self) -> (s: &ArraySeqMtPerS<T>) { &self.source }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn target(&self) -> (t: &ArraySeqMtPerS<T>) { &self.target }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read lock plus return cached size.
        fn memo_size(&self) -> (count: usize) {
            let handle = self.memo.acquire_read();
            let size = handle.borrow().len();
            handle.release_read();
            size
        }
    }

    // 11. derive impls in verus!

    impl<T: MtVal> Clone for MinEditDistMtPerS<T> {
        fn clone(&self) -> (cloned: Self) {
            MinEditDistMtPerS {
                source: self.source.clone(),
                target: self.target.clone(),
                memo: clone_arc_memo(self),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: MtVal> PartialEq for MinEditDistMtPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.source == other.source && self.target == other.target }
    }

    impl<T: MtVal> Eq for MinEditDistMtPerS<T> {}

    impl<T: MtVal> Debug for MinEditDistMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MinEditDistMtPerS")
                .field("source", &self.source)
                .field("target", &self.target)
                .finish()
        }
    }

    impl<T: MtVal> Display for MinEditDistMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let memo_size = {
                let handle = self.memo.acquire_read();
                let size = handle.borrow().len();
                handle.release_read();
                size
            };
            write!(
                f,
                "MinEditDistMtPer(source: {}, target: {}, memo_entries: {})",
                self.source, self.target, memo_size
            )
        }
    }
}

#[macro_export]
macro_rules! MinEditDistMtPerLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistMtPer::MinEditDistMtPer::MinEditDistMtPerS::from_sequences(
            <$crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS<_> as $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerBaseTrait<_>>::from_vec(vec![$($s),*]),
            <$crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS<_> as $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerBaseTrait<_>>::from_vec(vec![$($t),*])
        )
    };
    () => {
        $crate::Chap49::MinEditDistMtPer::MinEditDistMtPer::MinEditDistMtPerS::new()
    };
}
