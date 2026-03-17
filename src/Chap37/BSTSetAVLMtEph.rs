//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Set interface built atop the AVL multi-threaded BST implementation.

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

pub mod BSTSetAVLMtEph {

    use std::collections::BTreeSet;
    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

    //	4. type definitions

    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {
        tree: BSTAVLMtEph<T>,
    }

    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetAVLMtEphIter<T: StTInMtT + Ord> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetAVLMtEphGhostIter<T: StTInMtT + Ord> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord> View for BSTSetAVLMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord> View for BSTSetAVLMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn bstsetavlmteph_iter_invariant<T: StTInMtT + Ord>(it: &BSTSetAVLMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    //	8. traits

    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {
        spec fn spec_bstsetavlmteph_wf(&self) -> bool;

        fn empty() -> (set: Self)
            ensures set.spec_bstsetavlmteph_wf();
        fn singleton(value: T) -> (set: Self)
            ensures set.spec_bstsetavlmteph_wf();
        fn size(&self) -> (n: N)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn contains(&self, value: &T) -> (found: B)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsetavlmteph_wf()
            ensures self.spec_bstsetavlmteph_wf();
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetavlmteph_wf()
            ensures self.spec_bstsetavlmteph_wf();
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetavlmteph_wf(), other.spec_bstsetavlmteph_wf()
            ensures combined.spec_bstsetavlmteph_wf();
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetavlmteph_wf(), other.spec_bstsetavlmteph_wf()
            ensures common.spec_bstsetavlmteph_wf();
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetavlmteph_wf(), other.spec_bstsetavlmteph_wf()
            ensures diff.spec_bstsetavlmteph_wf();
        fn split(&self, pivot: &T) -> (parts: (Self, B, Self))
            requires self.spec_bstsetavlmteph_wf()
            ensures parts.0.spec_bstsetavlmteph_wf(), parts.2.spec_bstsetavlmteph_wf();
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetavlmteph_wf(), right.spec_bstsetavlmteph_wf()
            ensures joined.spec_bstsetavlmteph_wf();
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetavlmteph_wf(), right.spec_bstsetavlmteph_wf()
            ensures joined.spec_bstsetavlmteph_wf();
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires self.spec_bstsetavlmteph_wf()
            ensures filtered.spec_bstsetavlmteph_wf();
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn as_tree(&self) -> (tree: &BSTAVLMtEph<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures true;
        fn iter(&self) -> (it: BSTSetAVLMtEphIter<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures it@.0 == 0, bstsetavlmteph_iter_invariant(&it);
    }

    //	9. impls

    fn values_vec<T: StTInMtT + Ord>(tree: &BSTAVLMtEph<T>) -> (values: Vec<T>)
        requires tree.spec_bstavlmteph_wf(),
        ensures true,
    {
        tree.in_order().iter().cloned().collect()
    }

    fn rebuild_from_vec<T: StTInMtT + Ord>(values: Vec<T>) -> (tree: BSTAVLMtEph<T>)
        ensures true,
    {
        let mut tree = BSTAVLMtEph::new();
        for value in values {
            let _ = tree.insert(value);
        }
        tree
    }

    fn from_sorted_iter<T: StTInMtT + Ord, I>(values: I) -> (set: BSTSetAVLMtEph<T>)
    where
        I: IntoIterator<Item = T>,
        ensures true,
    {
        let mut tree = BSTAVLMtEph::new();
        for value in values {
            let _ = tree.insert(value);
        }
        BSTSetAVLMtEph { tree }
    }

    fn copy_set<T: StTInMtT + Ord>(set: &BSTSetAVLMtEph<T>) -> (out: BSTSetAVLMtEph<T>)
        requires set.spec_bstsetavlmteph_wf()
        ensures out.spec_bstsetavlmteph_wf()
    {
        from_sorted_iter(values_vec(&set.tree))
    }

    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {
        open spec fn spec_bstsetavlmteph_wf(&self) -> bool {
            self.tree.spec_bstavlmteph_wf()
        }

        fn empty() -> Self {
            Self {
                tree: BSTAVLMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let mut tree = BSTAVLMtEph::new();
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
            let filtered: Vec<T> = self
                .tree
                .in_order()
                .iter()
                .filter(|x| x != target)
                .cloned()
                .collect();
            self.tree = rebuild_from_vec(filtered);
        }

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
            let mut left = Vec::<T>::new();
            let mut right = Vec::<T>::new();
            let mut found = false;
            for value in self.tree.in_order().iter() {
                if value < pivot {
                    left.push(value.clone());
                } else if value > pivot {
                    right.push(value.clone());
                } else {
                    found = true;
                }
            }
            (from_sorted_iter(left), found, from_sorted_iter(right))
        }

        fn join_pair(left: Self, right: Self) -> Self {
            use crate::Types::Types::Pair;
            let Pair(left_values, right_values) = crate::ParaPair!(
                move || values_vec(&left.tree),
                move || values_vec(&right.tree)
            );

            let mut combined = left_values.into_iter().collect::<BTreeSet<T>>();
            for value in right_values {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            use crate::Types::Types::Pair;
            let Pair(left_values, right_values) = crate::ParaPair!(
                move || values_vec(&left.tree),
                move || values_vec(&right.tree)
            );

            let mut combined = left_values.into_iter().collect::<BTreeSet<T>>();
            combined.insert(pivot);
            for value in right_values {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        fn filter<F: FnMut(&T) -> bool + Send>(&self, mut predicate: F) -> Self {
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
        }

        fn reduce<F: FnMut(T, T) -> T + Send>(&self, mut op: F, base: T) -> T {
            self.tree
                .in_order()
                .iter()
                .fold(base, |acc, value| op(acc, value.clone()))
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }

        fn iter(&self) -> BSTSetAVLMtEphIter<T> {
            let values: Vec<T> = self.tree.in_order().iter().cloned().collect();
            BSTSetAVLMtEphIter { snapshot: values, pos: 0 }
        }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord> std::iter::Iterator for BSTSetAVLMtEphIter<T> {
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
                self.pos += 1;
                proof { accept(item == old(self)@.1[old(self)@.0]); }  // accept hole: Clone preserves value
                Some(item)
            }
        }
    }

    impl<T: StTInMtT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for BSTSetAVLMtEphIter<T> {
        type GhostIter = BSTSetAVLMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> BSTSetAVLMtEphGhostIter<T> {
            BSTSetAVLMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord> vstd::pervasive::ForLoopGhostIterator for BSTSetAVLMtEphGhostIter<T> {
        type ExecIter = BSTSetAVLMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSetAVLMtEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &BSTSetAVLMtEphIter<T>) -> BSTSetAVLMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord> std::iter::IntoIterator for &'a BSTSetAVLMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetAVLMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetAVLMtEphIter<T>)
            requires self.spec_bstsetavlmteph_wf()
            ensures it@.0 == 0, bstsetavlmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    impl<T: StTInMtT + Ord> IntoIterator for BSTSetAVLMtEph<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
    
        {
            let values: Vec<T> = self.tree.in_order().iter().cloned().collect();
            values.into_iter()
        }
    }

    } // verus!

    //	13. macros

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

    //	14. derive impls outside verus!

    impl<T: StTInMtT + Ord + fmt::Debug> fmt::Debug for BSTSetAVLMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetAVLMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord> fmt::Display for BSTSetAVLMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetAVLMtEph(size={})", self.size())
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSetAVLMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetAVLMtEphIter").field("pos", &self.pos).finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSetAVLMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetAVLMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSetAVLMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetAVLMtEphGhostIter").finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSetAVLMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetAVLMtEphGhostIter")
        }
    }
}
