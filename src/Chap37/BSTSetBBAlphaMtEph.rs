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

    #[derive(Clone)]
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord> {
        tree: BSTBBAlphaMtEph<T>,
    }

    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;

    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(value: T)                       -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                           -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn find(&self, value: &T)                    -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn contains(&self, value: &T)                -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn minimum(&self)                            -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn maximum(&self)                            -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn insert(&mut self, value: T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn delete(&mut self, target: &T);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self)                -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self)         -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self)           -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n)
        fn split(&self, pivot: &T)                   -> (Self, B, Self);
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|))
        fn join_pair(left: Self, right: Self)        -> Self;
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|))
        fn join_m(left: Self, pivot: T, right: Self) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> T;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn iter_in_order(&self)                      -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn as_tree(&self)                            -> &BSTBBAlphaMtEph<T>;
    }

    fn values_vec<T: StTInMtT + Ord>(tree: &BSTBBAlphaMtEph<T>) -> Vec<T> {
        tree.in_order().iter().cloned().collect()
    }
    fn rebuild_from_vec<T: StTInMtT + Ord>(values: Vec<T>) -> BSTBBAlphaMtEph<T> {
        let tree = BSTBBAlphaMtEph::new();
        for value in values { tree.insert(value); }
        tree
    }
    fn from_sorted_iter<T: StTInMtT + Ord, I: IntoIterator<Item = T>>(values: I) -> BSTSetBBAlphaMtEph<T> {
        let tree = BSTBBAlphaMtEph::new();
        for value in values { tree.insert(value); }
        BSTSetBBAlphaMtEph { tree }
    }

    impl<T: StTInMtT + Ord> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {
        fn empty() -> Self {
            Self {
                tree: BSTBBAlphaMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let tree = BSTBBAlphaMtEph::new();
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
    }

    }

    impl<T: StTInMtT + Ord + fmt::Debug> fmt::Debug for BSTSetBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEph").field("tree", &self.tree).finish()
        }
    }

    #[macro_export]
    macro_rules! BSTSetBBAlphaMtEphLit {
        () => {
            < $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph<_> as $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph<_> as $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }
}
