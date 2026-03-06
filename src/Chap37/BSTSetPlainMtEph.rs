//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the Plain multi-threaded BST implementation.

pub mod BSTSetPlainMtEph {

    use std::collections::BTreeSet;
    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTPlainMtEph::BSTPlainMtEph::*;
    use crate::Types::Types::*;

    verus! {

    #[derive(Clone)]
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord> {
        tree: BSTPlainMtEph<T>,
    }

    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetPlainMtEphIter<T: StTInMtT + Ord> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetPlainMtEphGhostIter<T: StTInMtT + Ord> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord> View for BSTSetPlainMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord> View for BSTSetPlainMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn bstsetplainmteph_iter_invariant<T: StTInMtT + Ord>(it: &BSTSetPlainMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord>: Sized {
        fn empty() -> (set: Self)
            ensures true;
        fn singleton(value: T) -> (set: Self)
            ensures true;
        fn size(&self) -> (n: N)
            ensures true;
        fn is_empty(&self) -> (b: B)
            ensures true;
        fn find(&self, value: &T) -> (found: Option<T>)
            ensures true;
        fn contains(&self, value: &T) -> (found: B)
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            ensures true;
        fn insert(&mut self, value: T)
            ensures true;
        fn delete(&mut self, target: &T)
            ensures true;
        fn union(&self, other: &Self) -> (combined: Self)
            ensures true;
        fn intersection(&self, other: &Self) -> (common: Self)
            ensures true;
        fn difference(&self, other: &Self) -> (diff: Self)
            ensures true;
        fn split(&self, pivot: &T) -> (parts: (Self, B, Self))
            ensures true;
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            ensures true;
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            ensures true;
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            ensures true;
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            ensures true;
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        fn as_tree(&self) -> (tree: &BSTPlainMtEph<T>)
            ensures true;
        fn iter(&self) -> (it: BSTSetPlainMtEphIter<T>)
            ensures it@.0 == 0, bstsetplainmteph_iter_invariant(&it);
    }

    fn values_vec<T: StTInMtT + Ord>(tree: &BSTPlainMtEph<T>) -> Vec<T> {
        tree.in_order().iter().cloned().collect()
    }
    fn rebuild_from_vec<T: StTInMtT + Ord>(values: Vec<T>) -> BSTPlainMtEph<T> {
        let tree = BSTPlainMtEph::new();
        for value in values { tree.insert(value); }
        tree
    }
    fn from_sorted_iter<T: StTInMtT + Ord, I: IntoIterator<Item = T>>(values: I) -> BSTSetPlainMtEph<T> {
        let tree = BSTPlainMtEph::new();
        for value in values { tree.insert(value); }
        BSTSetPlainMtEph { tree }
    }

    impl<T: StTInMtT + Ord> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {
        fn empty() -> Self {
            Self {
                tree: BSTPlainMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let tree = BSTPlainMtEph::new();
            tree.insert(value);
            Self { tree }
        }

        fn size(&self) -> N { self.tree.size() }

        fn is_empty(&self) -> B { self.tree.is_empty() }

        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        fn contains(&self, value: &T) -> B { self.tree.contains(value) }

        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        fn insert(&mut self, value: T) { self.tree.insert(value); }

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
            // Parallel divide-and-conquer using split/join
            if self.is_empty() {
                return other.clone();
            }
            if other.is_empty() {
                return self.clone();
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
            // Parallel divide-and-conquer using split/join
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
            // Parallel divide-and-conquer using split/join
            if self.is_empty() {
                return Self::empty();
            }
            if other.is_empty() {
                return self.clone();
            }
            
            let pivot = if self.size() <= other.size() {
                self.tree.minimum().unwrap()
            } else {
                other.tree.minimum().unwrap()
            };
            
            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, _, other_right) = other.split(&pivot);
            
            use crate::Types::Types::Pair;
            let Pair(left_diff, right_diff) = crate::ParaPair!(
                move || self_left.difference(&other_left),
                move || self_right.difference(&other_right)
            );
            
            if found_self {
                Self::join_pair(left_diff, right_diff)
            } else {
                Self::join_m(left_diff, pivot, right_diff)
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
            let mut combined = values_vec(&left.tree).into_iter().collect::<BTreeSet<T>>();
            for value in values_vec(&right.tree) {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let mut combined = values_vec(&left.tree).into_iter().collect::<BTreeSet<T>>();
            combined.insert(pivot);
            for value in values_vec(&right.tree) {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        fn filter<F>(&self, mut predicate: F) -> Self
        where
            F: FnMut(&T) -> bool,
        {
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
        }

        fn reduce<F>(&self, mut op: F, base: T) -> T
        where
            F: FnMut(T, T) -> T,
        {
            self.tree
                .in_order()
                .iter()
                .fold(base, |acc, value| op(acc, value.clone()))
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }

        fn iter(&self) -> BSTSetPlainMtEphIter<T> {
            let values: Vec<T> = self.tree.in_order().iter().cloned().collect();
            BSTSetPlainMtEphIter { snapshot: values, pos: 0 }
        }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord> std::iter::Iterator for BSTSetPlainMtEphIter<T> {
        type Item = T;

        #[verifier::external_body]
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
                Some(item)
            }
        }
    }

    impl<T: StTInMtT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for BSTSetPlainMtEphIter<T> {
        type GhostIter = BSTSetPlainMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> BSTSetPlainMtEphGhostIter<T> {
            BSTSetPlainMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord> vstd::pervasive::ForLoopGhostIterator for BSTSetPlainMtEphGhostIter<T> {
        type ExecIter = BSTSetPlainMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSetPlainMtEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &BSTSetPlainMtEphIter<T>) -> BSTSetPlainMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord> std::iter::IntoIterator for &'a BSTSetPlainMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetPlainMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetPlainMtEphIter<T>)
            ensures it@.0 == 0, bstsetplainmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    impl<T: StTInMtT + Ord> IntoIterator for BSTSetPlainMtEph<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures true,
        {
            let values: Vec<T> = self.tree.in_order().iter().cloned().collect();
            values.into_iter()
        }
    }

    } // verus!

    impl<T: StTInMtT + Ord + std::fmt::Debug> std::fmt::Debug for BSTSetPlainMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetPlainMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSetPlainMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetPlainMtEph(size={})", self.size())
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSetPlainMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetPlainMtEphIter").field("pos", &self.pos).finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSetPlainMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetPlainMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSetPlainMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetPlainMtEphGhostIter").finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSetPlainMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetPlainMtEphGhostIter")
        }
    }

    #[macro_export]
    macro_rules! BSTSetPlainMtEphLit {
        () => {
            < $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEph<_> as $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEph<_> as $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }
}
