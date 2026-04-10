//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 49: Minimum Edit Distance - ephemeral, single-threaded.

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

pub mod MinEditDistStEph {

    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::iter::Map as IterMap;
    use std::iter::Zip;

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
    pub struct MinEditDistStEphS<T: StT> {
        pub source: ArraySeqStEphS<T>,
        pub target: ArraySeqStEphS<T>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    }

    //		Section 6. spec fns


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

    //		Section 8. traits


    /// Trait for minimum edit distance operations.
    pub trait MinEditDistStEphTrait<T: StT>: Sized {
        /// Spec: source length.
        spec fn spec_source_len(&self) -> nat;

        /// Spec: target length.
        spec fn spec_target_len(&self) -> nat;

        /// Create new minimum edit distance solver.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- allocate empty structures.
        fn new() -> (empty: Self)
        where
            T: Default
            requires obeys_feq_clone::<T>(),
            ensures empty.spec_source_len() == 0, empty.spec_target_len() == 0;

        /// Create from source and target sequences.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- move sequences into struct.
        fn from_sequences(source: ArraySeqStEphS<T>, target: ArraySeqStEphS<T>) -> (edit_dist: Self)
            ensures
                edit_dist.spec_source_len() == source.spec_len(),
                edit_dist.spec_target_len() == target.spec_len();

        /// Compute minimum edit distance.
        /// - Alg Analysis: APAS (Ch49 Alg 49.5): Work O(|S| * |T|), Span O(|S| + |T|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|·|T|), Span O(|S|·|T|) — ACCEPTED DIFFERENCE: sequential DP table fill, APAS Span O(|S|+|T|) assumes parallel
        fn min_edit_distance(&mut self) -> (dist: usize)
            requires old(self).spec_source_len() + old(self).spec_target_len() < usize::MAX,
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Get the source sequence.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return reference.
        fn source(&self) -> (s: &ArraySeqStEphS<T>)
            ensures s.spec_len() == self.spec_source_len();

        /// Get the target sequence.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return reference.
        fn target(&self) -> (t: &ArraySeqStEphS<T>)
            ensures t.spec_len() == self.spec_target_len();

        /// Set element in source sequence.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- array set O(1) plus memo clear O(n).
        fn set_source(&mut self, index: usize, value: T)
            requires index < old(self).spec_source_len(),
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Set element in target sequence.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- array set O(1) plus memo clear O(n).
        fn set_target(&mut self, index: usize, value: T)
            requires index < old(self).spec_target_len(),
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Clear memoization table.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- clear hash map.
        fn clear_memo(&mut self)
            ensures
                self.spec_source_len() == old(self).spec_source_len(),
                self.spec_target_len() == old(self).spec_target_len();

        /// Get memoization table size.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return cached length.
        fn memo_size(&self) -> (count: usize);
    }

    //		Section 9. impls


    /// Recursive memoized minimum edit distance solver.
    /// - Alg Analysis: APAS (Ch49 ref): Work O(|S|*|T|), Span O(|S|+|T|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|*|T|), Span O(|S|*|T|) — ACCEPTED DIFFERENCE: sequential recursive memoized, Span = Work; APAS Span O(|S|+|T|) assumes parallel diagonal
    fn min_edit_distance_rec<T: StT>(
        table: &mut MinEditDistStEphS<T>,
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

    impl<T: StT> MinEditDistStEphTrait<T> for MinEditDistStEphS<T> {
        open spec fn spec_source_len(&self) -> nat { self.source.spec_len() }

        open spec fn spec_target_len(&self) -> nat { self.target.spec_len() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new() -> Self
        where
            T: Default,
        {
            // Veracity: NEEDED proof block
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source: ArraySeqStEphS::new(0, T::default()),
                target: ArraySeqStEphS::new(0, T::default()),
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction from components.
        // Veracity: NEEDED proof block
        fn from_sequences(source: ArraySeqStEphS<T>, target: ArraySeqStEphS<T>) -> Self {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                source,
                target,
                memo: HashMapWithViewPlus::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|*|T|), Span O(|S|*|T|) — memoized recursive DP; St sequential.
        fn min_edit_distance(&mut self) -> (dist: usize) {
            self.memo.clear();

            let source_len = self.source.length();
            let target_len = self.target.length();

            min_edit_distance_rec(self, source_len, target_len)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn source(&self) -> (s: &ArraySeqStEphS<T>) { &self.source }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn target(&self) -> (t: &ArraySeqStEphS<T>) { &self.target }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — array set O(1) plus memo clear O(n).
        fn set_source(&mut self, index: usize, value: T) {
            let _ = self.source.set(index, value);
            self.memo.clear();
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — array set O(1) plus memo clear O(n).
        fn set_target(&mut self, index: usize, value: T) {
            let _ = self.target.set(index, value);
            self.memo.clear();
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clear hash map.
        fn clear_memo(&mut self) { self.memo.clear(); }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — returns cached size.
        fn memo_size(&self) -> (count: usize) { self.memo.len() }
    }

    //		Section 12. derive impls in verus!


    impl<T: StT> Clone for MinEditDistStEphS<T> {
        fn clone(&self) -> (cloned: Self) {
            MinEditDistStEphS {
                source: self.source.clone(),
                target: self.target.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    // 12. traits outside verus!

    /// Trait for methods returning &mut (not supported inside verus!).
    pub trait MinEditDistStEphMutTrait<T: StT> {
        /// Get mutable source sequence (ephemeral allows mutation).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return mutable reference.
        fn source_mut(&mut self) -> &mut ArraySeqStEphS<T>;

        /// Get mutable target sequence (ephemeral allows mutation).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return mutable reference.
        fn target_mut(&mut self) -> &mut ArraySeqStEphS<T>;
    }

    impl<T: StT> MinEditDistStEphMutTrait<T> for MinEditDistStEphS<T> {
        fn source_mut(&mut self) -> &mut ArraySeqStEphS<T> { &mut self.source }
        fn target_mut(&mut self) -> &mut ArraySeqStEphS<T> { &mut self.target }
    }

    impl<T: StT> Debug for MinEditDistStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MinEditDistStEphS")
                .field("source", &self.source)
                .field("target", &self.target)
                .field("memo", &self.memo.inner)
                .finish()
        }
    }

    impl<T: StT> Display for MinEditDistStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(
                f,
                "MinEditDistStEph(source: {}, target: {}, memo_entries: {})",
                self.source,
                self.target,
                self.memo.inner.len()
            )
        }
    }

    impl<T: StT> IntoIterator for MinEditDistStEphS<T> {
        type Item = Pair<T, T>;
        type IntoIter = IterMap<
            Zip<<ArraySeqStEphS<T> as IntoIterator>::IntoIter, <ArraySeqStEphS<T> as IntoIterator>::IntoIter>,
            fn((T, T)) -> Pair<T, T>,
        >;

        fn into_iter(self) -> Self::IntoIter { self.source.into_iter().zip(self.target).map(|(a, b)| Pair(a, b)) }
    }

    impl<T: StT> IntoIterator for &MinEditDistStEphS<T> {
        type Item = Pair<T, T>;
        type IntoIter = IterMap<
            Zip<<ArraySeqStEphS<T> as IntoIterator>::IntoIter, <ArraySeqStEphS<T> as IntoIterator>::IntoIter>,
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

    impl<T: StT> IntoIterator for &mut MinEditDistStEphS<T> {
        type Item = Pair<T, T>;
        type IntoIter = IterMap<
            Zip<<ArraySeqStEphS<T> as IntoIterator>::IntoIter, <ArraySeqStEphS<T> as IntoIterator>::IntoIter>,
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
macro_rules! MinEditDistStEphLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistStEph::MinEditDistStEph::MinEditDistStEphS::from_sequences(
            $crate::ArraySeqStEphSLit![$($s),*],
            $crate::ArraySeqStEphSLit![$($t),*]
        )
    };
    () => {
        $crate::Chap49::MinEditDistStEph::MinEditDistStEph::MinEditDistStEphS::new()
    };
}
