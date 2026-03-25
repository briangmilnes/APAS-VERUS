//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Set interface built atop the Splay multi-threaded BST implementation.

//  Table of Contents
//	1. module
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	10. iterators
//	13. macros
//	14. derive impls outside verus!

//	1. module

pub mod BSTSetSplayMtEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    verus! {

    //	4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub tree: BSTSplayMtEph<T>,
    }

    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetSplayMtEphIter<T: StTInMtT + Ord + TotalOrder> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetSplayMtEphGhostIter<T: StTInMtT + Ord + TotalOrder> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetSplayMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetSplayMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn bstsetsplaymteph_iter_invariant<T: StTInMtT + Ord + TotalOrder>(it: &BSTSetSplayMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    //	8. traits

    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bstsetsplaymteph_wf(&self) -> bool;

        fn empty() -> (set: Self)
            ensures set.spec_bstsetsplaymteph_wf();
        fn singleton(value: T) -> (set: Self)
            ensures set.spec_bstsetsplaymteph_wf();
        fn size(&self) -> (n: N)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn contains(&self, value: &T) -> (found: B)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsetsplaymteph_wf()
            ensures self.spec_bstsetsplaymteph_wf();
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetsplaymteph_wf()
            ensures self.spec_bstsetsplaymteph_wf();
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetsplaymteph_wf(), other.spec_bstsetsplaymteph_wf()
            ensures combined.spec_bstsetsplaymteph_wf();
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetsplaymteph_wf(), other.spec_bstsetsplaymteph_wf()
            ensures common.spec_bstsetsplaymteph_wf();
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetsplaymteph_wf(), other.spec_bstsetsplaymteph_wf()
            ensures diff.spec_bstsetsplaymteph_wf();
        fn split(&self, pivot: &T) -> (parts: (Self, B, Self))
            requires self.spec_bstsetsplaymteph_wf()
            ensures parts.0.spec_bstsetsplaymteph_wf(), parts.2.spec_bstsetsplaymteph_wf();
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetsplaymteph_wf(), right.spec_bstsetsplaymteph_wf()
            ensures joined.spec_bstsetsplaymteph_wf();
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetsplaymteph_wf(), right.spec_bstsetsplaymteph_wf()
            ensures joined.spec_bstsetsplaymteph_wf();
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires self.spec_bstsetsplaymteph_wf()
            ensures filtered.spec_bstsetsplaymteph_wf();
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn as_tree(&self) -> (tree: &BSTSplayMtEph<T>)
            requires self.spec_bstsetsplaymteph_wf()
            ensures true;
        fn iter(&self) -> (it: BSTSetSplayMtEphIter<T>)
            requires self.spec_bstsetsplaymteph_wf()
            ensures it@.0 == 0, bstsetsplaymteph_iter_invariant(&it);
    }

    //	9. impls

    fn values_vec<T: StTInMtT + Ord + TotalOrder + 'static>(tree: &BSTSplayMtEph<T>) -> (values: Vec<T>)
        requires tree.spec_bstsplaymteph_wf(), obeys_feq_clone::<T>(),
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
    fn rebuild_from_vec<T: StTInMtT + Ord + TotalOrder + 'static>(values: Vec<T>) -> (tree: BSTSplayMtEph<T>)
        ensures tree.spec_bstsplaymteph_wf(),
    {
        let mut tree = BSTSplayMtEph::new();
        let n = values.len();
        let mut i: usize = 0;
        while i < n
            invariant
                tree.spec_bstsplaymteph_wf(),
                0 <= i <= n,
                n == values@.len(),
            decreases n - i,
        {
            let _ = tree.insert(values[i].clone());
            i += 1;
        }
        tree
    }

    fn build_from_vec<T: StTInMtT + Ord + TotalOrder + 'static>(values: Vec<T>) -> (set: BSTSetSplayMtEph<T>)
        requires obeys_feq_clone::<T>(),
        ensures set.spec_bstsetsplaymteph_wf(),
    {
        BSTSetSplayMtEph { tree: rebuild_from_vec(values) }
    }

    fn copy_set<T: StTInMtT + Ord + TotalOrder + 'static>(set: &BSTSetSplayMtEph<T>) -> (out: BSTSetSplayMtEph<T>)
        requires set.spec_bstsetsplaymteph_wf()
        ensures out.spec_bstsetsplaymteph_wf()
    {
        build_from_vec(values_vec(&set.tree))
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {
        open spec fn spec_bstsetsplaymteph_wf(&self) -> bool {
            self.tree.spec_bstsplaymteph_wf() && obeys_feq_clone::<T>()
        }

        fn empty() -> Self {
            proof { assume(obeys_feq_clone::<T>()); }
            Self {
                tree: BSTSplayMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            proof { assume(obeys_feq_clone::<T>()); }
            let mut tree = BSTSplayMtEph::new();
            let _ = tree.insert(value);
            Self { tree }
        }

        fn size(&self) -> N { self.tree.size() }

        fn is_empty(&self) -> B { self.tree.is_empty() }

        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        fn contains(&self, value: &T) -> B { self.tree.contains(value) }

        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        fn insert(&mut self, value: T) -> (r: Result<(), ()>) { self.tree.insert(value) }

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

        #[verifier::external_body]
        fn union(&self, other: &Self) -> Self {
            if self.is_empty() {
                return copy_set(other);
            }
            if other.is_empty() {
                return copy_set(self);
            }

            let pivot = if self.size() <= other.size() {
                self.tree.minimum().unwrap()
            } else {
                other.tree.minimum().unwrap()
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            use crate::Types::Types::Pair;
            let Pair(left_union, right_union) = crate::ParaPair!(
                move || self_left.union(&other_left),
                move || self_right.union(&other_right)
            );

            if found_self || found_other {
                Self::join_m(left_union, pivot, right_union)
            } else {
                Self::join_pair(left_union, right_union)
            }
        }

        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> Self {
            if self.is_empty() || other.is_empty() {
                return Self::empty();
            }

            let pivot = if self.size() <= other.size() {
                self.tree.minimum().unwrap()
            } else {
                other.tree.minimum().unwrap()
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            use crate::Types::Types::Pair;
            let Pair(left_inter, right_inter) = crate::ParaPair!(
                move || self_left.intersection(&other_left),
                move || self_right.intersection(&other_right)
            );

            if found_self && found_other {
                Self::join_m(left_inter, pivot, right_inter)
            } else {
                Self::join_pair(left_inter, right_inter)
            }
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> Self {
            if self.is_empty() {
                return Self::empty();
            }
            if other.is_empty() {
                return copy_set(self);
            }

            let pivot = self.tree.minimum().unwrap();

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            use crate::Types::Types::Pair;
            let Pair(left_diff, right_diff) = crate::ParaPair!(
                move || self_left.difference(&other_left),
                move || self_right.difference(&other_right)
            );

            if found_self && !found_other {
                Self::join_m(left_diff, pivot, right_diff)
            } else {
                Self::join_pair(left_diff, right_diff)
            }
        }

        fn split(&self, pivot: &T) -> (Self, B, Self) {
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

        fn join_pair(left: Self, right: Self) -> Self {
            let left_sorted = left.tree.in_order();
            let right_sorted = right.tree.in_order();
            let mut tree = BSTSplayMtEph::new();
            let n_left = left_sorted.length();
            let mut i: usize = 0;
            while i < n_left
                invariant
                    tree.spec_bstsplaymteph_wf(),
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
                    tree.spec_bstsplaymteph_wf(),
                    n_right as nat == right_sorted.spec_len(),
                    0 <= j <= n_right,
                decreases n_right - j,
            {
                let _ = tree.insert(right_sorted.nth(j).clone());
                j += 1;
            }
            BSTSetSplayMtEph { tree }
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let left_sorted = left.tree.in_order();
            let right_sorted = right.tree.in_order();
            let mut tree = BSTSplayMtEph::new();
            let _ = tree.insert(pivot);
            let n_left = left_sorted.length();
            let mut i: usize = 0;
            while i < n_left
                invariant
                    tree.spec_bstsplaymteph_wf(),
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
                    tree.spec_bstsplaymteph_wf(),
                    n_right as nat == right_sorted.spec_len(),
                    0 <= j <= n_right,
                decreases n_right - j,
            {
                let _ = tree.insert(right_sorted.nth(j).clone());
                j += 1;
            }
            BSTSetSplayMtEph { tree }
        }

        #[verifier::external_body]
        fn filter<F: FnMut(&T) -> bool + Send>(&self, mut predicate: F) -> Self {
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
                if predicate(elem) {
                    filtered.push(elem.clone());
                }
                i += 1;
            }
            build_from_vec(filtered)
        }

        #[verifier::external_body]
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, mut op: F, base: T) -> T {
            let sorted = self.tree.in_order();
            let n = sorted.length();
            let mut acc = base;
            let mut i: usize = 0;
            while i < n
                invariant
                    n as nat == sorted.spec_len(),
                    0 <= i <= n,
                decreases n - i,
            {
                acc = op(acc, sorted.nth(i).clone());
                i += 1;
            }
            acc
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> {
            self.tree.in_order()
        }

        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }

        fn iter(&self) -> BSTSetSplayMtEphIter<T> {
            let values = values_vec(&self.tree);
            BSTSetSplayMtEphIter { snapshot: values, pos: 0 }
        }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord + TotalOrder> std::iter::Iterator for BSTSetSplayMtEphIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            if self.pos >= self.snapshot.len() {
                None
            } else {
                let item = self.snapshot[self.pos].clone();
                self.pos = self.pos + 1;
                proof { assume(item == old(self)@.1[old(self)@.0]); }  // accept hole: Clone preserves value
                Some(item)
            }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIteratorNew for BSTSetSplayMtEphIter<T> {
        type GhostIter = BSTSetSplayMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> BSTSetSplayMtEphGhostIter<T> {
            BSTSetSplayMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIterator for BSTSetSplayMtEphGhostIter<T> {
        type ExecIter = BSTSetSplayMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSetSplayMtEphIter<T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &BSTSetSplayMtEphIter<T>) -> BSTSetSplayMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + TotalOrder + 'static> std::iter::IntoIterator for &'a BSTSetSplayMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetSplayMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetSplayMtEphIter<T>)
            requires self.spec_bstsetsplaymteph_wf()
            ensures it@.0 == 0, bstsetsplaymteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> IntoIterator for BSTSetSplayMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetSplayMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetSplayMtEphIter<T>)
            requires self.spec_bstsetsplaymteph_wf(),
            ensures it@.0 == 0, bstsetsplaymteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    } // verus!

    //	13. macros

    #[macro_export]
    macro_rules! BSTSetSplayMtEphLit {
        () => {
            < $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEph<_> as $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEph<_> as $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEphTrait<_> >::empty();
            $( let _ = __set.insert($x); )*
            __set
        }};
    }

    //	14. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder + fmt::Debug> fmt::Debug for BSTSetSplayMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetSplayMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for BSTSetSplayMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetSplayMtEph(size={})", self.size())
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSetSplayMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetSplayMtEphIter").field("pos", &self.pos).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTSetSplayMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetSplayMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSetSplayMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetSplayMtEphGhostIter").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTSetSplayMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetSplayMtEphGhostIter")
        }
    }
}

