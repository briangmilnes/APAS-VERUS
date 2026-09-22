// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Set interface built atop the BB-Alpha multi-threaded BST implementation.


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

pub mod BSTSetBBAlphaMtEph {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
    use crate::Types::Types::*;

    verus! 
{

    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub tree: BSTBBAlphaMtEph<T>,
    }

    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;

    //		Section 6. spec fns

    //		Section 8. traits


    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bstsetbbalphamteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(value: T) -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, value: &T) -> (found: bool)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>)
            requires old(self).spec_bstsetbbalphamteph_wf()
            ensures self.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetbbalphamteph_wf()
            ensures self.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures combined.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures common.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures diff.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self))
            requires self.spec_bstsetbbalphamteph_wf()
            ensures parts.0.spec_bstsetbbalphamteph_wf(), parts.2.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetbbalphamteph_wf(), right.spec_bstsetbbalphamteph_wf()
            ensures joined.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetbbalphamteph_wf(), right.spec_bstsetbbalphamteph_wf()
            ensures joined.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires
                self.spec_bstsetbbalphamteph_wf(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures filtered.spec_bstsetbbalphamteph_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires
                self.spec_bstsetbbalphamteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn as_tree(&self) -> (tree: &BSTBBAlphaMtEph<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn iter(&self) -> (it: std::vec::IntoIter<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures
                vstd::std_specs::vec::into_iter_elts(it) == IteratorSpec::remaining(&it),
                IteratorSpec::decrease(&it) is Some;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn copy_set(&self) -> (out: Self)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures out.spec_bstsetbbalphamteph_wf();
    }

    //		Section 9. impls


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn values_vec<T: StTInMtT + Ord + TotalOrder>(tree: &BSTBBAlphaMtEph<T>) -> (values: Vec<T>)
        requires tree.spec_bstbbalphamteph_wf(), obeys_feq_clone::<T>(),
        ensures true,
    {
        let seq = tree.in_order();
        let n = seq.length();
        let mut out = Vec::<T>::new();
        let mut i: usize = 0;
        while i < n
            invariant
                i <= n,
                n as int == seq.spec_len(),
            decreases n - i,
        {
            out.push(seq.nth(i).clone());
            i = i + 1;
        }
        out
    }
    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn rebuild_from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (tree: BSTBBAlphaMtEph<T>)
        ensures tree.spec_bstbbalphamteph_wf(),
    {
        let mut tree = BSTBBAlphaMtEph::new();
        let mut i: usize = 0;
        while i < values.len()
            invariant tree.spec_bstbbalphamteph_wf(),
                i <= values.len(),
            decreases values.len() - i,
        {
            let value = values[i].clone();
            let _ = tree.insert(value);
            i = i + 1;
        }
        tree
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
    fn from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (set: BSTSetBBAlphaMtEph<T>)
        requires obeys_feq_clone::<T>(),
        ensures set.spec_bstsetbbalphamteph_wf(),
    {
        let mut tree = BSTBBAlphaMtEph::new();
        let mut i: usize = 0;
        while i < values.len()
            invariant tree.spec_bstbbalphamteph_wf(),
                i <= values.len(),
            decreases values.len() - i,
        {
            let value = values[i].clone();
            let _ = tree.insert(value);
            i = i + 1;
        }
        BSTSetBBAlphaMtEph { tree }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {
        open spec fn spec_bstsetbbalphamteph_wf(&self) -> bool {
            self.tree.spec_bstbbalphamteph_wf() && obeys_feq_clone::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> Self {
            Self {
                tree: BSTBBAlphaMtEph::new(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(value: T) -> Self {
            let mut tree = BSTBBAlphaMtEph::new();
            let _ = tree.insert(value);
            Self { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> usize { self.tree.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> bool { self.tree.is_empty() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, value: &T) -> bool { self.tree.contains(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>) { self.tree.insert(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn delete(&mut self, target: &T) {
            if !self.contains(target) {
                return;
            }
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut filtered = Vec::<T>::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                decreases n - i,
            {
                if *seq.nth(i) != *target {
                    filtered.push(seq.nth(i).clone());
                }
                i = i + 1;
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

            let pivot = match self.tree.minimum() {
                Some(v) => v,
                None => { return other.copy_set(); }
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
            { self_left.union(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
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

            let pivot = match self.tree.minimum() {
                Some(v) => v,
                None => { return Self::empty(); }
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
            { self_left.intersection(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
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

            let (self_left, _found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
            { self_left.difference(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
            { self_right.difference(&other_right) };
            use crate::Types::Types::Pair;
            let Pair(left_diff, right_diff) = crate::ParaPair!(f1, f2);

            if found_other {
                Self::join_pair(left_diff, right_diff)
            } else {
                Self::join_m(left_diff, pivot, right_diff)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn split(&self, pivot: &T) -> (Self, bool, Self) {
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut left = Vec::<T>::new();
            let mut right = Vec::<T>::new();
            let mut found = false;
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                decreases n - i,
            {
                let value = seq.nth(i);
                match TotalOrder::cmp(value, pivot) {
                    core::cmp::Ordering::Less => { left.push(value.clone()); }
                    core::cmp::Ordering::Greater => { right.push(value.clone()); }
                    core::cmp::Ordering::Equal => { found = true; }
                }
                i = i + 1;
            }
            (from_vec(left), found, from_vec(right))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_pair(left: Self, right: Self) -> Self {
            let mut combined = values_vec(&left.tree);
            let right_vals = values_vec(&right.tree);
            let mut i: usize = 0;
            while i < right_vals.len()
                invariant i <= right_vals.len(),
                decreases right_vals.len() - i,
            {
                combined.push(right_vals[i].clone());
                i = i + 1;
            }
            from_vec(combined)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let mut combined = values_vec(&left.tree);
            combined.push(pivot);
            let right_vals = values_vec(&right.tree);
            let mut i: usize = 0;
            while i < right_vals.len()
                invariant i <= right_vals.len(),
                decreases right_vals.len() - i,
            {
                combined.push(right_vals[i].clone());
                i = i + 1;
            }
            from_vec(combined)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn filter<F>(&self, mut predicate: F) -> Self
        where
            F: FnMut(&T) -> bool,
        {
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut filtered = Vec::<T>::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                    forall|t: &T| #[trigger] predicate.requires((t,)),
                decreases n - i,
            {
                if predicate(seq.nth(i)) {
                    filtered.push(seq.nth(i).clone());
                }
                i = i + 1;
            }
            from_vec(filtered)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, mut op: F, base: T) -> T
        where
            F: FnMut(T, T) -> T,
        {
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut acc = base;
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                    forall|a: T, b: T| #[trigger] op.requires((a, b)),
                decreases n - i,
            {
                acc = op(acc, seq.nth(i).clone());
                i = i + 1;
            }
            acc
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }

        fn iter(&self) -> std::vec::IntoIter<T> {
            values_vec(&self.tree).into_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn copy_set(&self) -> (out: Self) {
            from_vec(values_vec(&self.tree))
        }
    }

    //		Section 10. iterators

    // r212 form C: this `IntoIterator` impl required `requires self.spec_bstsetbbalphamteph_wf()`, which
    // verus 0.2026.09.13 rejects on an external trait's impl and no exec check
    // can establish; use `iter()`, which keeps the requires
    // (src/experiments/intoiter_form_c_no_impl.rs).
    /*
    impl<'a, T: StTInMtT + Ord + TotalOrder + 'static> std::iter::IntoIterator for &'a BSTSetBBAlphaMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetBBAlphaMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetBBAlphaMtEphIter<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures it@.0 == 0, bstsetbbalphamteph_iter_invariant(&it),
        {
            self.iter()
        }
    }
    */

    // r212 form C: this `IntoIterator` impl required `requires self.spec_bstsetbbalphamteph_wf()`, which
    // verus 0.2026.09.13 rejects on an external trait's impl and no exec check
    // can establish; use `iter()`, which keeps the requires
    // (src/experiments/intoiter_form_c_no_impl.rs).
    /*
    impl<T: StTInMtT + Ord + TotalOrder> IntoIterator for BSTSetBBAlphaMtEph<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_bstsetbbalphamteph_wf()
        {
            values_vec(&self.tree).into_iter()
        }
    }
    */

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTSetBBAlphaMtEphLit {
        () => {
            < $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph<_> as $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph<_> as $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEphTrait<_> >::empty();
            $( let _ = __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder + fmt::Debug> fmt::Debug for BSTSetBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for BSTSetBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetBBAlphaMtEph(size={})", self.size())
        }
    }
}
