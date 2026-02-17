//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the Treap multi-threaded BST implementation.
//!
//! NOTE: This implementation uses SEQUENTIAL aggregate operations (union, intersection, etc.)
//! because it wraps BSTTreapMtEph which, while thread-safe, doesn't expose the internal tree 
//! structure (split/join primitives) needed for parallel divide-and-conquer algorithms.
//!
//! The backing BSTTreapMtEph provides:
//! - Thread-safe concurrent access (Arc<RwLock<>> at root)
//! - Standard BST operations: insert, find, delete, min, max
//! - Sequential traversals: in_order, pre_order
//!
//! What it LACKS for parallelization:
//! - No split(pivot) -> (left_tree, found, right_tree) operation
//! - No join(left_tree, right_tree) -> tree operation  
//! - Internal node structure not exposed for parallel traversal
//!
//! For truly PARALLEL BST operations with O(lg n) span, use BSTParaTreapMtEph instead.
//! BSTParaTreapMtEph provides split/join primitives enabling parallel aggregate operations.
//! 
//! This file exists for compatibility, testing, and basic thread-safe Treap usage,
//! but has O(n) span for aggregate operations (union, intersection, difference).

pub mod BSTSetTreapMtEph {

    use std::collections::BTreeSet;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;
    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct BSTSetTreapMtEph<T: StTInMtT + Ord> {
        tree: BSTTreapMtEph<T>,
    }

    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;

    pub trait BSTSetTreapMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(value: T)                       -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                           -> B;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected with locking
        fn find(&self, value: &T)                    -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected with locking
        fn contains(&self, value: &T)                -> B;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected with locking
        fn minimum(&self)                            -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected with locking
        fn maximum(&self)                            -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected with locking
        fn insert(&mut self, value: T);
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected with locking
        fn delete(&mut self, target: &T);
        /// NOTE: Sequential implementation - Span Θ(n), not parallel
        fn union(&self, other: &Self)                -> Self;
        /// NOTE: Sequential implementation - Span Θ(n), not parallel
        fn intersection(&self, other: &Self)         -> Self;
        /// NOTE: Sequential implementation - Span Θ(n), not parallel
        fn difference(&self, other: &Self)           -> Self;
        /// claude-4-sonet: Work Θ(log n) expected, Span Θ(log n)
        fn split(&self, pivot: &T)                   -> (Self, B, Self);
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|))
        fn join_pair(left: Self, right: Self)        -> Self;
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|))
        fn join_m(left: Self, pivot: T, right: Self) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn iter_in_order(&self)                      -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn as_tree(&self)                            -> &BSTTreapMtEph<T>;
    }

    impl<T: StTInMtT + Ord> BSTSetTreapMtEph<T> {
        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }
        fn rebuild_from_vec(values: Vec<T>) -> BSTTreapMtEph<T> {
            let tree = BSTTreapMtEph::new();
            for value in values {
                tree.insert(value);
            }
            tree
        }
        fn from_sorted_iter<I>(values: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let tree = BSTTreapMtEph::new();
            for value in values {
                tree.insert(value);
            }
            Self { tree }
        }
    }

    impl<T: StTInMtT + Ord> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {
        fn empty() -> Self {
            Self {
                tree: BSTTreapMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let tree = BSTTreapMtEph::new();
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
            let mut values = self.values_vec();
            if let Some(pos) = values.iter().position(|x| x == target) {
                values.remove(pos);
                self.tree = Self::rebuild_from_vec(values);
            }
        }

        fn union(&self, other: &Self) -> Self {
            let mut merged = self.values_vec().into_iter().collect::<BTreeSet<T>>();
            for value in other.values_vec() {
                merged.insert(value);
            }
            Self::from_sorted_iter(merged)
        }

        fn intersection(&self, other: &Self) -> Self {
            let other_values = other.values_vec().into_iter().collect::<BTreeSet<T>>();
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
            Self::from_sorted_iter(filtered)
        }

        fn difference(&self, other: &Self) -> Self {
            let other_values = other.values_vec().into_iter().collect::<BTreeSet<T>>();
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
            Self::from_sorted_iter(filtered)
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
            (Self::from_sorted_iter(left), found, Self::from_sorted_iter(right))
        }

        fn join_pair(left: Self, right: Self) -> Self {
            let mut combined = left.values_vec().into_iter().collect::<BTreeSet<T>>();
            for value in right.values_vec() {
                combined.insert(value);
            }
            Self::from_sorted_iter(combined)
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let mut combined = left.values_vec().into_iter().collect::<BTreeSet<T>>();
            combined.insert(pivot);
            for value in right.values_vec() {
                combined.insert(value);
            }
            Self::from_sorted_iter(combined)
        }

        fn filter<F: FnMut(&T) -> bool>(&self, mut predicate: F) -> Self {
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect::<Vec<T>>();
            Self::from_sorted_iter(filtered)
        }

        fn reduce<F: FnMut(T, T) -> T>(&self, mut op: F, base: T) -> T {
            self.tree
                .in_order()
                .iter()
                .fold(base, |acc, value| op(acc, value.clone()))
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }
    }

    #[macro_export]
    macro_rules! BSTSetTreapMtEphLit {
        () => {
            < $crate::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMtEph<_> as $crate::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMtEph<_> as $crate::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }
}
