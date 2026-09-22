// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Set interface built atop the AVL multi-threaded BST implementation.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod BSTSetAVLMtEph {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    verus! 
{

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub tree: BSTAVLMtEph<T>,
    }

    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;

    //		Section 6. spec fns

    //		Section 8. traits


    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bstsetavlmteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(value: T) -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn contains(&self, value: &T) -> (found: bool)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>)
            requires old(self).spec_bstsetavlmteph_wf()
            ensures self.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetavlmteph_wf()
            ensures self.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetavlmteph_wf(), other.spec_bstsetavlmteph_wf()
            ensures combined.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetavlmteph_wf(), other.spec_bstsetavlmteph_wf()
            ensures common.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetavlmteph_wf(), other.spec_bstsetavlmteph_wf()
            ensures diff.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self))
            requires self.spec_bstsetavlmteph_wf()
            ensures parts.0.spec_bstsetavlmteph_wf(), parts.2.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetavlmteph_wf(), right.spec_bstsetavlmteph_wf()
            ensures joined.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetavlmteph_wf(), right.spec_bstsetavlmteph_wf()
            ensures joined.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires
                self.spec_bstsetavlmteph_wf(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures filtered.spec_bstsetavlmteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires
                self.spec_bstsetavlmteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn as_tree(&self) -> (tree: &BSTAVLMtEph<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn iter(&self) -> (it: std::vec::IntoIter<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures
                vstd::std_specs::vec::into_iter_elts(it) == IteratorSpec::remaining(&it),
                IteratorSpec::decrease(&it) is Some;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn copy_set(&self) -> (out: Self)
            requires self.spec_bstsetavlmteph_wf()
            ensures out.spec_bstsetavlmteph_wf();
    }

    //		Section 9. impls


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn values_vec<T: StTInMtT + Ord + TotalOrder>(tree: &BSTAVLMtEph<T>) -> (values: Vec<T>)
        requires tree.spec_bstavlmteph_wf(), obeys_feq_clone::<T>(),
        ensures true,
    {
        let sorted = tree.in_order();
        let n = sorted.length();
        let mut values: Vec<T> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                n as nat == sorted.spec_len(),
                0 <= i <= n,
            decreases n - i,
        {
            values.push(sorted.nth(i).clone());
            i += 1;
        }
        values
    }

    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn rebuild_from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (tree: BSTAVLMtEph<T>)
        ensures tree.spec_bstavlmteph_wf(),
    {
        let mut tree = BSTAVLMtEph::new();
        let n = values.len();
        let mut i: usize = 0;
        while i < n
            invariant
                tree.spec_bstavlmteph_wf(),
                0 <= i <= n,
                n == values@.len(),
            decreases n - i,
        {
            let _ = tree.insert(values[i].clone());
            i += 1;
        }
        tree
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn build_from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (set: BSTSetAVLMtEph<T>)
        requires obeys_feq_clone::<T>(),
        ensures set.spec_bstsetavlmteph_wf(),
    {
        BSTSetAVLMtEph { tree: rebuild_from_vec(values) }
    }

    impl<T: StTInMtT + Ord + TotalOrder> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {
        open spec fn spec_bstsetavlmteph_wf(&self) -> bool {
            self.tree.spec_bstavlmteph_wf() && obeys_feq_clone::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> Self {
            Self {
                tree: BSTAVLMtEph::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(value: T) -> Self {
            let mut tree = BSTAVLMtEph::new();
            let _ = tree.insert(value);
            Self { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> usize { self.tree.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> bool { self.tree.is_empty() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn contains(&self, value: &T) -> bool { self.tree.contains(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>) { self.tree.insert(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn delete(&mut self, target: &T) {
            if !self.contains(target) {
                return;
            }
            let sorted = self.tree.in_order();
            let n = sorted.length();
            let mut filtered: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    n as nat == sorted.spec_len(),
                    0 <= i <= n,
                decreases n - i,
            {
                let elem = sorted.nth(i);
                if *elem != *target {
                    filtered.push(elem.clone());
                }
                i += 1;
            }
            self.tree = rebuild_from_vec(filtered);
        }

        #[verifier::exec_allows_no_decreases_clause]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn union(&self, other: &Self) -> Self {
            if self.is_empty() {
                return other.copy_set();
            }
            if other.is_empty() {
                return self.copy_set();
            }

            let pivot = if self.size() <= other.size() {
                match self.tree.minimum() {
                    Some(v) => v,
                    None => { return other.copy_set(); }
                }
            } else {
                match other.tree.minimum() {
                    Some(v) => v,
                    None => { return self.copy_set(); }
                }
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetavlmteph_wf()
            { self_left.union(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetavlmteph_wf()
            { self_right.union(&other_right) };
            use crate::Types::Types::Pair;
            let Pair(left_union, right_union) = crate::ParaPair!(f1, f2);

            if found_self || found_other {
                Self::join_m(left_union, pivot, right_union)
            } else {
                Self::join_pair(left_union, right_union)
            }
        }

        #[verifier::exec_allows_no_decreases_clause]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn intersection(&self, other: &Self) -> Self {
            if self.is_empty() || other.is_empty() {
                return Self::empty();
            }

            let pivot = if self.size() <= other.size() {
                match self.tree.minimum() {
                    Some(v) => v,
                    None => { return Self::empty(); }
                }
            } else {
                match other.tree.minimum() {
                    Some(v) => v,
                    None => { return Self::empty(); }
                }
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetavlmteph_wf()
            { self_left.intersection(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetavlmteph_wf()
            { self_right.intersection(&other_right) };
            use crate::Types::Types::Pair;
            let Pair(left_inter, right_inter) = crate::ParaPair!(f1, f2);

            if found_self && found_other {
                Self::join_m(left_inter, pivot, right_inter)
            } else {
                Self::join_pair(left_inter, right_inter)
            }
        }

        #[verifier::exec_allows_no_decreases_clause]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn difference(&self, other: &Self) -> Self {
            if self.is_empty() {
                return Self::empty();
            }
            if other.is_empty() {
                return self.copy_set();
            }

            let pivot = match self.tree.minimum() {
                Some(v) => v,
                None => { return Self::empty(); }
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetavlmteph_wf()
            { self_left.difference(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetavlmteph_wf()
            { self_right.difference(&other_right) };
            use crate::Types::Types::Pair;
            let Pair(left_diff, right_diff) = crate::ParaPair!(f1, f2);

            if found_self && !found_other {
                Self::join_m(left_diff, pivot, right_diff)
            } else {
                Self::join_pair(left_diff, right_diff)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn split(&self, pivot: &T) -> (Self, bool, Self) {
            let sorted = self.tree.in_order();
            let n = sorted.length();
            let mut left: Vec<T> = Vec::new();
            let mut right: Vec<T> = Vec::new();
            let mut found = false;
            let mut i: usize = 0;
            while i < n
                invariant
                    n as nat == sorted.spec_len(),
                    0 <= i <= n,
                decreases n - i,
            {
                let elem = sorted.nth(i);
                if *elem < *pivot {
                    left.push(elem.clone());
                } else if *elem > *pivot {
                    right.push(elem.clone());
                } else {
                    found = true;
                }
                i += 1;
            }
            (build_from_vec(left), found, build_from_vec(right))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_pair(left: Self, right: Self) -> Self {
            let left_sorted = left.tree.in_order();
            let right_sorted = right.tree.in_order();
            let mut tree = BSTAVLMtEph::new();
            let n_left = left_sorted.length();
            let mut i: usize = 0;
            while i < n_left
                invariant
                    tree.spec_bstavlmteph_wf(),
                    n_left as nat == left_sorted.spec_len(),
                    0 <= i <= n_left,
                decreases n_left - i,
            {
                let _ = tree.insert(left_sorted.nth(i).clone());
                i += 1;
            }
            let n_right = right_sorted.length();
            let mut j: usize = 0;
            while j < n_right
                invariant
                    tree.spec_bstavlmteph_wf(),
                    n_right as nat == right_sorted.spec_len(),
                    0 <= j <= n_right,
                decreases n_right - j,
            {
                let _ = tree.insert(right_sorted.nth(j).clone());
                j += 1;
            }
            BSTSetAVLMtEph { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let left_sorted = left.tree.in_order();
            let right_sorted = right.tree.in_order();
            let mut tree = BSTAVLMtEph::new();
            let _ = tree.insert(pivot);
            let n_left = left_sorted.length();
            let mut i: usize = 0;
            while i < n_left
                invariant
                    tree.spec_bstavlmteph_wf(),
                    n_left as nat == left_sorted.spec_len(),
                    0 <= i <= n_left,
                decreases n_left - i,
            {
                let _ = tree.insert(left_sorted.nth(i).clone());
                i += 1;
            }
            let n_right = right_sorted.length();
            let mut j: usize = 0;
            while j < n_right
                invariant
                    tree.spec_bstavlmteph_wf(),
                    n_right as nat == right_sorted.spec_len(),
                    0 <= j <= n_right,
                decreases n_right - j,
            {
                let _ = tree.insert(right_sorted.nth(j).clone());
                j += 1;
            }
            BSTSetAVLMtEph { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn filter<F>(&self, mut predicate: F) -> Self
        where
            F: FnMut(&T) -> bool,
        {
            let sorted = self.tree.in_order();
            let n = sorted.length();
            let mut filtered: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == sorted.spec_len(),
                    forall|t: &T| #[trigger] predicate.requires((t,)),
                decreases n - i,
            {
                let elem = sorted.nth(i);
                if predicate(elem) {
                    filtered.push(elem.clone());
                }
                i += 1;
            }
            build_from_vec(filtered)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, mut op: F, base: T) -> T
        where
            F: FnMut(T, T) -> T,
        {
            let sorted = self.tree.in_order();
            let n = sorted.length();
            let mut acc = base;
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == sorted.spec_len(),
                    forall|a: T, b: T| #[trigger] op.requires((a, b)),
                decreases n - i,
            {
                acc = op(acc, sorted.nth(i).clone());
                i += 1;
            }
            acc
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn iter_in_order(&self) -> ArraySeqStPerS<T> {
            self.tree.in_order()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }

        fn iter(&self) -> std::vec::IntoIter<T> {
            let values = values_vec(&self.tree);
            values.into_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn copy_set(&self) -> (out: Self) {
            build_from_vec(values_vec(&self.tree))
        }
    }

    //		Section 10. iterators

    // r212 form C: this `IntoIterator` impl required `requires self.spec_bstsetavlmteph_wf()`, which
    // verus 0.2026.09.13 rejects on an external trait's impl and no exec check
    // can establish; use `iter()`, which keeps the requires
    // (src/experiments/intoiter_form_c_no_impl.rs).
    /*
    impl<'a, T: StTInMtT + Ord + TotalOrder> std::iter::IntoIterator for &'a BSTSetAVLMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetAVLMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetAVLMtEphIter<T>)
            requires self.spec_bstsetavlmteph_wf(),
            ensures it@.0 == 0, bstsetavlmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }
    */

    // r212 form C: this `IntoIterator` impl required `requires self.spec_bstsetavlmteph_wf()`, which
    // verus 0.2026.09.13 rejects on an external trait's impl and no exec check
    // can establish; use `iter()`, which keeps the requires
    // (src/experiments/intoiter_form_c_no_impl.rs).
    /*
    impl<T: StTInMtT + Ord + TotalOrder> IntoIterator for BSTSetAVLMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetAVLMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetAVLMtEphIter<T>)
            requires self.spec_bstsetavlmteph_wf(),
            ensures it@.0 == 0, bstsetavlmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }
    */

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTSetAVLMtEphLit {
        () => {
            < $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEph<_> as $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEph<_> as $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEphTrait<_> >::empty();
            $( let _ = __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder + fmt::Debug> fmt::Debug for BSTSetAVLMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetAVLMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> fmt::Display for BSTSetAVLMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetAVLMtEph(size={})", self.size())
        }
    }
}
