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
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn empty()                                   -> Self;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected; Span Θ(log n) expected — singleton inserts one element
        fn singleton(value: T)                       -> Self;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                               -> N;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                           -> B;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, value: &T)                    -> Option<T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, value: &T)                -> B;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn minimum(&self)                            -> Option<T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn maximum(&self)                            -> Option<T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&mut self, value: T);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — rebuild-based; not O(log n)
        fn delete(&mut self, target: &T);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential BTreeSet rebuild, not parallel
        fn union(&self, other: &Self)                -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential filter + rebuild, not parallel
        fn intersection(&self, other: &Self)         -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential filter + rebuild, not parallel
        fn difference(&self, other: &Self)           -> Self;
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential scan + rebuild, not O(log n)
        fn split(&self, pivot: &T)                   -> (Self, B, Self);
        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — BTreeSet rebuild, not O(log n)
        fn join_pair(left: Self, right: Self)        -> Self;
        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — BTreeSet rebuild, not O(log n)
        fn join_m(left: Self, pivot: T, right: Self) -> Self;
        /// - APAS: Work Θ(n), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential filter + rebuild
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;
        /// - APAS: Work Θ(n), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential fold
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn iter_in_order(&self)                      -> ArraySeqStPerS<T>;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn as_tree(&self)                            -> &BSTTreapMtEph<T>;
    }

    fn values_vec<T: StTInMtT + Ord>(tree: &BSTTreapMtEph<T>) -> Vec<T> {
        tree.in_order().iter().cloned().collect()
    }

    fn rebuild_from_vec<T: StTInMtT + Ord>(values: Vec<T>) -> BSTTreapMtEph<T> {
        let tree = BSTTreapMtEph::new();
        for value in values {
            tree.insert(value);
        }
        tree
    }

    fn from_sorted_iter<T: StTInMtT + Ord, I: IntoIterator<Item = T>>(values: I) -> BSTSetTreapMtEph<T> {
        let tree = BSTTreapMtEph::new();
        for value in values {
            tree.insert(value);
        }
        BSTSetTreapMtEph { tree }
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
            let mut values = values_vec(&self.tree);
            if let Some(pos) = values.iter().position(|x| x == target) {
                values.remove(pos);
                self.tree = rebuild_from_vec(values);
            }
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

        fn filter<F: FnMut(&T) -> bool>(&self, mut predicate: F) -> Self {
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
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
