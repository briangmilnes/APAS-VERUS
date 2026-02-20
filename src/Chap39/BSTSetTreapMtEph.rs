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
    use std::collections::hash_map::DefaultHasher;
    use std::fmt;
    use std::hash::{Hash, Hasher};

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;
    use crate::Types::Types::*;

    verus! {

    #[verifier::external_body]
    fn priority_for<T: Hash>(key: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetTreapMtEph<T: StTInMtT + Ord + Hash> {
        tree: BSTTreapMtEph<T>,
    }

    impl<T: StTInMtT + Ord + Hash> Clone for BSTSetTreapMtEph<T> {
        fn clone(&self) -> (result: Self)
            ensures true,
        {
            BSTSetTreapMtEph { tree: self.tree.clone() }
        }
    }

    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;  // T: StTInMtT + Ord + Hash

    pub trait BSTSetTreapMtEphTrait<T: StTInMtT + Ord + Hash>: Sized {
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

    #[verifier::external_body]
    fn values_vec<T: StTInMtT + Ord>(tree: &BSTTreapMtEph<T>) -> Vec<T> {
        tree.in_order().iter().cloned().collect()
    }

    fn rebuild_from_vec<T: StTInMtT + Ord + Hash>(values: Vec<T>) -> BSTTreapMtEph<T> {
        let tree = BSTTreapMtEph::new();
        for value in values {
            let p = priority_for(&value);
            tree.insert(value, p);
        }
        tree
    }

    #[verifier::external_body]
    fn from_sorted_iter<T: StTInMtT + Ord + Hash, I: IntoIterator<Item = T>>(values: I) -> BSTSetTreapMtEph<T> {
        let tree = BSTTreapMtEph::new();
        for value in values {
            let p = priority_for(&value);
            tree.insert(value, p);
        }
        BSTSetTreapMtEph { tree }
    }

    impl<T: StTInMtT + Ord + Hash> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {
        fn empty() -> Self {
            Self {
                tree: BSTTreapMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let tree = BSTTreapMtEph::new();
            let p = priority_for(&value);
            tree.insert(value, p);
            Self { tree }
        }

        fn size(&self) -> N { self.tree.size() }

        fn is_empty(&self) -> B { self.tree.is_empty() }

        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        fn contains(&self, value: &T) -> B { self.tree.contains(value) }

        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        fn insert(&mut self, value: T) {
            let p = priority_for(&value);
            self.tree.insert(value, p);
        }

        #[verifier::external_body]
        fn delete(&mut self, target: &T) {
            let mut values = values_vec(&self.tree);
            if let Some(pos) = values.iter().position(|x| x == target) {
                values.remove(pos);
                self.tree = rebuild_from_vec(values);
            }
        }

        #[verifier::external_body]
        fn union(&self, other: &Self) -> Self {
            let mut merged = values_vec(&self.tree).into_iter().collect::<BTreeSet<T>>();
            for value in values_vec(&other.tree) {
                merged.insert(value);
            }
            from_sorted_iter(merged)
        }

        #[verifier::external_body]
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

        #[verifier::external_body]
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

        #[verifier::external_body]
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

        #[verifier::external_body]
        #[verifier::external_body]
        fn join_pair(left: Self, right: Self) -> Self {
            let mut combined = values_vec(&left.tree).into_iter().collect::<BTreeSet<T>>();
            for value in values_vec(&right.tree) {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        #[verifier::external_body]
        #[verifier::external_body]
        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let mut combined = values_vec(&left.tree).into_iter().collect::<BTreeSet<T>>();
            combined.insert(pivot);
            for value in values_vec(&right.tree) {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        #[verifier::external_body]
        fn filter<F: FnMut(&T) -> bool>(&self, mut predicate: F) -> Self {
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
        }

        #[verifier::external_body]
        fn reduce<F: FnMut(T, T) -> T>(&self, mut op: F, base: T) -> T {
            self.tree
                .in_order()
                .iter()
                .fold(base, |acc, value| op(acc, value.clone()))
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTTreapMtEph<T> { &self.tree }
    }

    }

    impl<T: StTInMtT + Ord + Hash + fmt::Debug> fmt::Debug for BSTSetTreapMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetTreapMtEph").field("tree", &self.tree).finish()
        }
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
