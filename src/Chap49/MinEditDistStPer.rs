//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - persistent, single-threaded.

pub mod MinEditDistStPer {

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
    use std::iter::Map as IterMap;
    use std::iter::Zip;

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
    pub struct MinEditDistStPerS<T: StT> {
        pub source: ArraySeqStPerS<T>,
        pub target: ArraySeqStPerS<T>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, usize>,
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

    /// Every memo entry at key (i, j) stores a value <= i + j.
    pub open spec fn spec_memo_bounded(memo_view: Map<(usize, usize), usize>) -> bool {
        forall|k: (usize, usize)| #[trigger] memo_view.contains_key(k) ==>
            (memo_view[k] as int) <= (k.0 + k.1) as int
    }

    // 8. traits

    /// Trait for minimum edit distance operations.
    pub trait MinEditDistStPerTrait<T: StT>: Sized {
        /// Spec: source length.
        spec fn spec_source_len(&self) -> nat;

        /// Spec: target length.
        spec fn spec_target_len(&self) -> nat;

        /// Create new minimum edit distance solver.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures empty.spec_source_len() == 0, empty.spec_target_len() == 0;

        /// Create from source and target sequences.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_sequences(source: ArraySeqStPerS<T>, target: ArraySeqStPerS<T>) -> (edit_dist: Self)
            ensures
                edit_dist.spec_source_len() == source.spec_len(),
                edit_dist.spec_target_len() == target.spec_len();

        /// Compute minimum edit distance.
        /// - Alg Analysis: APAS (Ch49 Alg 49.5): Work O(|S| * |T|), Span O(|S| + |T|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|·|T|), Span O(|S|·|T|) — DIFFERS: sequential DP table fill, APAS Span O(|S|+|T|) assumes parallel
        fn min_edit_distance(&self) -> (dist: usize)
            requires self.spec_source_len() + self.spec_target_len() < usize::MAX;

        /// Get the source sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn source(&self) -> (s: &ArraySeqStPerS<T>)
            ensures s.spec_len() == self.spec_source_len();

        /// Get the target sequence.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn target(&self) -> (t: &ArraySeqStPerS<T>)
            ensures t.spec_len() == self.spec_target_len();

        /// Get memoization table size.
        /// - Alg Analysis: APAS (Ch49 ref): not specified
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// Recursive memoized minimum edit distance solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(|S|×|T|), Span O(|S|+|T|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|×|T|), Span O(|S|+|T|) — matches APAS
    fn min_edit_distance_rec<T: StT>(
        table: &mut MinEditDistStPerS<T>,
        i: usize,
        j: usize,
    ) -> (dist: usize)
        requires
            i <= old(table).source.spec_len(),
            j <= old(table).target.spec_len(),
            old(table).source.spec_len() + old(table).target.spec_len() < usize::MAX,
            spec_memo_bounded(old(table).memo@),
        ensures
            table.source.spec_len() == old(table).source.spec_len(),
            table.target.spec_len() == old(table).target.spec_len(),
            dist <= i + j,
            spec_memo_bounded(table.memo@),
        decreases i + j,
    {
        if let Some(cached) = table.memo.get(&Pair(i, j)) {
            return *cached;
        }

        let dist = if i == 0 {
            j
        } else if j == 0 {
            i
        } else {
            let source_char = table.source.nth(i - 1);
            let target_char = table.target.nth(j - 1);

            if *source_char == *target_char {
                min_edit_distance_rec(table, i - 1, j - 1)
            } else {
                let delete_cost = min_edit_distance_rec(table, i - 1, j);
                let insert_cost = min_edit_distance_rec(table, i, j - 1);

                if delete_cost <= insert_cost {
                    1 + delete_cost
                } else {
                    1 + insert_cost
                }
            }
        };

        table.memo.insert(Pair(i, j), dist);
        dist
    }

    impl<T: StT> MinEditDistStPerTrait<T> for MinEditDistStPerS<T> {
        open spec fn spec_source_len(&self) -> nat { self.source.spec_len() }

        open spec fn spec_target_len(&self) -> nat { self.target.spec_len() }

        fn new() -> Self
        where
            T: Default,
        {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source: ArraySeqStPerS::new(0, T::default()),
                target: ArraySeqStPerS::new(0, T::default()),
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn from_sequences(source: ArraySeqStPerS<T>, target: ArraySeqStPerS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source,
                target,
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn min_edit_distance(&self) -> (dist: usize) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            let mut solver = MinEditDistStPerS {
                source: self.source.clone(),
                target: self.target.clone(),
                memo: HashMapWithViewPlus::new(),
            };

            let source_len = solver.source.length();
            let target_len = solver.target.length();

            min_edit_distance_rec(&mut solver, source_len, target_len)
        }

        fn source(&self) -> (s: &ArraySeqStPerS<T>) { &self.source }

        fn target(&self) -> (t: &ArraySeqStPerS<T>) { &self.target }

        fn memo_size(&self) -> (count: usize) { self.memo.len() }
    }

    // 11. derive impls in verus!

    impl<T: StT> Clone for MinEditDistStPerS<T> {
        fn clone(&self) -> (cloned: Self) {
            MinEditDistStPerS {
                source: self.source.clone(),
                target: self.target.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> PartialEq for MinEditDistStPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.source == other.source
                && self.target == other.target
                && self.memo.inner == other.memo.inner
        }
    }

    impl<T: StT> Eq for MinEditDistStPerS<T> {}

    impl<T: StT> Debug for MinEditDistStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MinEditDistStPerS")
                .field("source", &self.source)
                .field("target", &self.target)
                .field("memo", &self.memo.inner)
                .finish()
        }
    }

    impl<T: StT> Display for MinEditDistStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(
                f,
                "MinEditDistStPer(source: {}, target: {}, memo_entries: {})",
                self.source,
                self.target,
                self.memo.inner.len()
            )
        }
    }

    impl<T: StT> IntoIterator for MinEditDistStPerS<T> {
        type Item = Pair<T, T>;
        type IntoIter = IterMap<
            Zip<<ArraySeqStPerS<T> as IntoIterator>::IntoIter, <ArraySeqStPerS<T> as IntoIterator>::IntoIter>,
            fn((T, T)) -> Pair<T, T>,
        >;

        fn into_iter(self) -> Self::IntoIter { self.source.into_iter().zip(self.target).map(|(a, b)| Pair(a, b)) }
    }

    impl<T: StT> IntoIterator for &MinEditDistStPerS<T> {
        type Item = Pair<T, T>;
        type IntoIter = IterMap<
            Zip<<ArraySeqStPerS<T> as IntoIterator>::IntoIter, <ArraySeqStPerS<T> as IntoIterator>::IntoIter>,
            fn((T, T)) -> Pair<T, T>,
        >;

        fn into_iter(self) -> Self::IntoIter {
            self.source
                .clone()
                .into_iter()
                .zip(self.target.clone())
                .map(|(a, b)| Pair(a, b))
        }
    }
}

#[macro_export]
macro_rules! MinEditDistStPerLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistStPer::MinEditDistStPer::MinEditDistStPerS::from_sequences(
            $crate::ArraySeqStPerSLit![$($s),*],
            $crate::ArraySeqStPerSLit![$($t),*]
        )
    };
    () => {
        $crate::Chap49::MinEditDistStPer::MinEditDistStPer::MinEditDistStPerS::new()
    };
}
