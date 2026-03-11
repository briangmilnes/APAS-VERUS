//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the BB-Alpha multi-threaded BST implementation.

pub mod BSTSetBBAlphaMtEph {

    use std::collections::BTreeSet;
    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
    use crate::Types::Types::*;

    verus! {

    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {
        tree: BSTBBAlphaMtEph<T>,
    }

    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetBBAlphaMtEphIter<T: StTInMtT + Ord> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetBBAlphaMtEphGhostIter<T: StTInMtT + Ord> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord> View for BSTSetBBAlphaMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord> View for BSTSetBBAlphaMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn bstsetbbalphamteph_iter_invariant<T: StTInMtT + Ord>(it: &BSTSetBBAlphaMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {
        spec fn spec_bstsetbbalphamteph_wf(&self) -> bool;

        fn empty() -> (set: Self)
            ensures set.spec_bstsetbbalphamteph_wf();
        fn singleton(value: T) -> (set: Self)
            ensures set.spec_bstsetbbalphamteph_wf();
        fn size(&self) -> (n: N)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn contains(&self, value: &T) -> (found: B)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsetbbalphamteph_wf()
            ensures self.spec_bstsetbbalphamteph_wf();
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetbbalphamteph_wf()
            ensures self.spec_bstsetbbalphamteph_wf();
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures combined.spec_bstsetbbalphamteph_wf();
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures common.spec_bstsetbbalphamteph_wf();
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures diff.spec_bstsetbbalphamteph_wf();
        fn split(&self, pivot: &T) -> (parts: (Self, B, Self))
            requires self.spec_bstsetbbalphamteph_wf()
            ensures parts.0.spec_bstsetbbalphamteph_wf(), parts.2.spec_bstsetbbalphamteph_wf();
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetbbalphamteph_wf(), right.spec_bstsetbbalphamteph_wf()
            ensures joined.spec_bstsetbbalphamteph_wf();
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetbbalphamteph_wf(), right.spec_bstsetbbalphamteph_wf()
            ensures joined.spec_bstsetbbalphamteph_wf();
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures filtered.spec_bstsetbbalphamteph_wf();
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn as_tree(&self) -> (tree: &BSTBBAlphaMtEph<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn iter(&self) -> (it: BSTSetBBAlphaMtEphIter<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures it@.0 == 0, bstsetbbalphamteph_iter_invariant(&it);
    }

    fn values_vec<T: StTInMtT + Ord>(tree: &BSTBBAlphaMtEph<T>) -> Vec<T> {
        tree.in_order().iter().cloned().collect()
    }
    fn rebuild_from_vec<T: StTInMtT + Ord>(values: Vec<T>) -> BSTBBAlphaMtEph<T> {
        let mut tree = BSTBBAlphaMtEph::new();
        for value in values {
            let _ = tree.insert(value);
        }
        tree
    }

    fn from_sorted_iter<T: StTInMtT + Ord, I>(values: I) -> BSTSetBBAlphaMtEph<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut tree = BSTBBAlphaMtEph::new();
        for value in values {
            let _ = tree.insert(value);
        }
        BSTSetBBAlphaMtEph { tree }
    }

    fn copy_set<T: StTInMtT + Ord>(set: &BSTSetBBAlphaMtEph<T>) -> BSTSetBBAlphaMtEph<T> {
        from_sorted_iter(values_vec(&set.tree))
    }

    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {
        open spec fn spec_bstsetbbalphamteph_wf(&self) -> bool {
            self.tree.spec_bstbbalphamteph_wf()
        }

        fn empty() -> Self {
            Self {
                tree: BSTBBAlphaMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let mut tree = BSTBBAlphaMtEph::new();
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
            let mut merged = values_vec(&self.tree).into_iter().collect::<BTreeSet<T>>();
            for value in values_vec(&other.tree) {
                merged.insert(value);
            }
            from_sorted_iter(merged)
        }

        fn intersection(&self, other: &Self) -> Self {
            let other_values = values_vec(&other.tree).into_iter().collect::<BTreeSet<T>>();
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| {
                    if other_values.contains(v) {
                        Some(v.clone())
                    } else {
                        None
                    }
                }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
        }

        fn difference(&self, other: &Self) -> Self {
            let other_values = values_vec(&other.tree).into_iter().collect::<BTreeSet<T>>();
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| {
                    if !other_values.contains(v) {
                        Some(v.clone())
                    } else {
                        None
                    }
                }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
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

        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }

        fn iter(&self) -> BSTSetBBAlphaMtEphIter<T> {
            let values: Vec<T> = self.tree.in_order().iter().cloned().collect();
            BSTSetBBAlphaMtEphIter { snapshot: values, pos: 0 }
        }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord> std::iter::Iterator for BSTSetBBAlphaMtEphIter<T> {
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

    impl<T: StTInMtT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for BSTSetBBAlphaMtEphIter<T> {
        type GhostIter = BSTSetBBAlphaMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> BSTSetBBAlphaMtEphGhostIter<T> {
            BSTSetBBAlphaMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord> vstd::pervasive::ForLoopGhostIterator for BSTSetBBAlphaMtEphGhostIter<T> {
        type ExecIter = BSTSetBBAlphaMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSetBBAlphaMtEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &BSTSetBBAlphaMtEphIter<T>) -> BSTSetBBAlphaMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord> std::iter::IntoIterator for &'a BSTSetBBAlphaMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetBBAlphaMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetBBAlphaMtEphIter<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures it@.0 == 0, bstsetbbalphamteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    impl<T: StTInMtT + Ord> IntoIterator for BSTSetBBAlphaMtEph<T> {
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

    impl<T: StTInMtT + Ord + fmt::Debug> fmt::Debug for BSTSetBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord> fmt::Display for BSTSetBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetBBAlphaMtEph(size={})", self.size())
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSetBBAlphaMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEphIter").field("pos", &self.pos).finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSetBBAlphaMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetBBAlphaMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSetBBAlphaMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEphGhostIter").finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSetBBAlphaMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetBBAlphaMtEphGhostIter")
        }
    }

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
}
