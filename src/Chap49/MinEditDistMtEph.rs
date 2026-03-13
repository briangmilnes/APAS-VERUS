//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - ephemeral, multi-threaded.

pub mod MinEditDistMtEph {

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

    pub struct MinEditDistMtEphMemoInv;
    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for MinEditDistMtEphMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
            v@.dom().finite()
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct MinEditDistMtEphS<T: MtVal> {
        pub source: ArraySeqMtEphS<T>,
        pub target: ArraySeqMtEphS<T>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>,
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
    pub trait MinEditDistMtEphTrait<T: MtVal>: Sized {
        /// Spec: source length.
        spec fn spec_source_len(&self) -> nat;

        /// Spec: target length.
        spec fn spec_target_len(&self) -> nat;

        /// Create new minimum edit distance solver.
        /// - APAS: not specified
        fn new() -> (result: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures result.spec_source_len() == 0, result.spec_target_len() == 0;

        /// Create from source and target sequences.
        /// - APAS: not specified
        fn from_sequences(source: ArraySeqMtEphS<T>, target: ArraySeqMtEphS<T>) -> (result: Self)
            ensures
                result.spec_source_len() == source.spec_len(),
                result.spec_target_len() == target.spec_len();

        /// Compute minimum edit distance.
        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
        fn min_edit_distance(&mut self) -> (dist: usize)
        where
            T: Send + Sync + 'static
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Get the source sequence.
        /// - APAS: not specified
        fn source(&self) -> (s: &ArraySeqMtEphS<T>)
            ensures s.spec_len() == self.spec_source_len();

        /// Get the target sequence.
        /// - APAS: not specified
        fn target(&self) -> (t: &ArraySeqMtEphS<T>)
            ensures t.spec_len() == self.spec_target_len();

        /// Set element in source sequence.
        /// - APAS: not specified
        fn set_source(&mut self, index: usize, value: T)
            requires index < old(self).spec_source_len(),
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Set element in target sequence.
        /// - APAS: not specified
        fn set_target(&mut self, index: usize, value: T)
            requires index < old(self).spec_target_len(),
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Clear memoization table.
        /// - APAS: not specified
        fn clear_memo(&mut self)
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Get memoization table size.
        /// - APAS: not specified
        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// Create Arc-wrapped memo lock.
    fn new_arc_memo(
        val: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    ) -> (memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>)
        requires val@.dom().finite(),
        ensures memo.pred() == MinEditDistMtEphMemoInv,
    {
        new_arc_rwlock(val, Ghost(MinEditDistMtEphMemoInv))
    }

    /// Clone Arc memo (reference count increment).
    fn clone_arc_memo<T: MtVal>(
        s: &MinEditDistMtEphS<T>,
    ) -> (cloned: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>)
        ensures cloned.pred() == s.memo.pred(),
    {
        clone_arc_rwlock(&s.memo)
    }

    /// Recursive memoized parallel minimum edit distance solver.
    /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
    #[verifier::external_body]
    fn min_edit_distance_rec<T: MtVal + Send + Sync + 'static>(
        source: &ArraySeqMtEphS<T>,
        target: &ArraySeqMtEphS<T>,
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MinEditDistMtEphMemoInv>>,
        i: usize,
        j: usize,
    ) -> (dist: usize) {
        {
            let handle = memo.acquire_read();
            let found = handle.borrow().get(&Pair(i, j)).copied();
            handle.release_read();
            if let Some(result) = found {
                return result;
            }
        }

        let result = if i == 0 {
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
                let memo1 = Arc::clone(memo);
                let source2 = source.clone();
                let target2 = target.clone();
                let memo2 = Arc::clone(memo);

                let f1 = move || min_edit_distance_rec(&source1, &target1, &memo1, i - 1, j);
                let f2 = move || min_edit_distance_rec(&source2, &target2, &memo2, i, j - 1);
                let (delete_cost, insert_cost) = join(f1, f2);

                if delete_cost <= insert_cost {
                    1 + delete_cost
                } else {
                    1 + insert_cost
                }
            }
        };

        {
            let (mut current, write_handle) = memo.acquire_write();
            current.insert(Pair(i, j), result);
            write_handle.release_write(current);
        }

        result
    }

    impl<T: MtVal> MinEditDistMtEphTrait<T> for MinEditDistMtEphS<T> {
        open spec fn spec_source_len(&self) -> nat { self.source.spec_len() }

        open spec fn spec_target_len(&self) -> nat { self.target.spec_len() }

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

        fn from_sequences(source: ArraySeqMtEphS<T>, target: ArraySeqMtEphS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source,
                target,
                memo: new_arc_memo(HashMapWithViewPlus::new()),
            }
        }

        fn min_edit_distance(&mut self) -> (dist: usize)
        where
            T: Send + Sync + 'static,
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

        fn source(&self) -> (s: &ArraySeqMtEphS<T>) { &self.source }

        fn target(&self) -> (t: &ArraySeqMtEphS<T>) { &self.target }

        fn set_source(&mut self, index: usize, value: T) {
            let _ = self.source.set(index, value);
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        fn set_target(&mut self, index: usize, value: T) {
            let _ = self.target.set(index, value);
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

    // 12. traits outside verus!

    /// Trait for methods returning &mut (not supported inside verus!).
    pub trait MinEditDistMtEphMutTrait<T: MtVal> {
        /// Get mutable source sequence (ephemeral allows mutation).
        /// - APAS: not specified
        fn source_mut(&mut self) -> &mut ArraySeqMtEphS<T>;

        /// Get mutable target sequence (ephemeral allows mutation).
        /// - APAS: not specified
        fn target_mut(&mut self) -> &mut ArraySeqMtEphS<T>;
    }

    impl<T: MtVal> MinEditDistMtEphMutTrait<T> for MinEditDistMtEphS<T> {
        fn source_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.source }
        fn target_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.target }
    }

    // 13. derive impls outside verus!

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
