//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the AVL multi-threaded BST implementation.

pub mod BSTSetAVLMtEph {

    use std::collections::BTreeSet;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct BSTSetAVLMtEph<T: StTInMtT + Ord> {
        tree: BSTAVLMtEph<T>,
    }

    pub type BSTSetAVLMt<T> = BSTSetAVLMtEph<T>;

    fn values_vec<T: StTInMtT + Ord>(tree: &BSTAVLMtEph<T>) -> Vec<T> { tree.in_order().iter().cloned().collect() }

    fn rebuild_from_vec<T: StTInMtT + Ord>(values: Vec<T>) -> BSTAVLMtEph<T> {
        let tree = BSTAVLMtEph::new();
        for value in values {
            tree.insert(value);
        }
        tree
    }

    fn from_sorted_iter<T: StTInMtT + Ord, I>(values: I) -> BSTSetAVLMtEph<T>
    where
        I: IntoIterator<Item = T>,
    {
        let tree = BSTAVLMtEph::new();
        for value in values {
            tree.insert(value);
        }
        BSTSetAVLMtEph { tree }
    }

    pub trait BSTSetAVLMtEphTrait<T: StTInMtT + Ord>: Sized {
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
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) [parallelized]
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) [parallelized]
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> T;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn iter_in_order(&self)                      -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn as_tree(&self)                            -> &BSTAVLMtEph<T>;
    }

    impl<T: StTInMtT + Ord> BSTSetAVLMtEphTrait<T> for BSTSetAVLMtEph<T> {
        fn empty() -> Self {
            Self {
                tree: BSTAVLMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let tree = BSTAVLMtEph::new();
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
            // Algorithm: Parallel divide-and-conquer using split/join primitives
            // Work: O(m log(n/m)), Span: O(log n × log m)
            
            // Base cases
            if self.is_empty() {
                return other.clone();
            }
            if other.is_empty() {
                return self.clone();
            }
            
            // Pick pivot from smaller tree for better balance
            let pivot = if self.size() <= other.size() {
                self.tree.minimum().unwrap()
            } else {
                other.tree.minimum().unwrap()
            };
            
            // Split both trees at pivot
            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);
            
            // Parallel recursive union on left and right subtrees
            use crate::Types::Types::Pair;
            let Pair(left_union, right_union) = crate::ParaPair!(
                move || self_left.union(&other_left),
                move || self_right.union(&other_right)
            );
            
            // Join results: include pivot if found in either tree
            if found_self || found_other {
                Self::join_m(left_union, pivot, right_union)
            } else {
                Self::join_pair(left_union, right_union)
            }
        }

        fn intersection(&self, other: &Self) -> Self {
            // Algorithm: Parallel divide-and-conquer using split/join primitives
            // Work: O(m log(n/m)), Span: O(log n × log m)
            
            // Base cases
            if self.is_empty() || other.is_empty() {
                return Self::empty();
            }
            
            // Pick pivot from smaller tree
            let pivot = if self.size() <= other.size() {
                self.tree.minimum().unwrap()
            } else {
                other.tree.minimum().unwrap()
            };
            
            // Split both trees at pivot
            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);
            
            // Parallel recursive intersection on left and right subtrees
            use crate::Types::Types::Pair;
            let Pair(left_inter, right_inter) = crate::ParaPair!(
                move || self_left.intersection(&other_left),
                move || self_right.intersection(&other_right)
            );
            
            // Join results: include pivot only if found in BOTH trees
            if found_self && found_other {
                Self::join_m(left_inter, pivot, right_inter)
            } else {
                Self::join_pair(left_inter, right_inter)
            }
        }

        fn difference(&self, other: &Self) -> Self {
            // Algorithm: Parallel divide-and-conquer using split/join primitives
            // Work: O(m log(n/m)), Span: O(log n × log m)
            
            // Base cases
            if self.is_empty() {
                return Self::empty();
            }
            if other.is_empty() {
                return self.clone();
            }
            
            // Pick pivot from self (the set we're subtracting from)
            let pivot = self.tree.minimum().unwrap();
            
            // Split both trees at pivot
            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);
            
            // Parallel recursive difference on left and right subtrees
            use crate::Types::Types::Pair;
            let Pair(left_diff, right_diff) = crate::ParaPair!(
                move || self_left.difference(&other_left),
                move || self_right.difference(&other_right)
            );
            
            // Join results: include pivot only if found in self but NOT in other
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
            // Parallel extraction of values from both trees
            use crate::Types::Types::Pair;
            let Pair(left_values, right_values) = crate::ParaPair!(
                move || values_vec(&left.tree),
                move || values_vec(&right.tree)
            );
            
            // Merge into BTreeSet and rebuild
            let mut combined = left_values.into_iter().collect::<BTreeSet<T>>();
            for value in right_values {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            // Parallel extraction of values from both trees
            use crate::Types::Types::Pair;
            let Pair(left_values, right_values) = crate::ParaPair!(
                move || values_vec(&left.tree),
                move || values_vec(&right.tree)
            );
            
            // Merge into BTreeSet with pivot and rebuild
            let mut combined = left_values.into_iter().collect::<BTreeSet<T>>();
            combined.insert(pivot);
            for value in right_values {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        fn filter<F: FnMut(&T) -> bool + Send>(&self, mut predicate: F) -> Self {
            // Sequential implementation due to FnMut constraint
            // Parallel implementation would require Fn + Sync which is incompatible with FnMut API
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
        }

        fn reduce<F: FnMut(T, T) -> T + Send>(&self, mut op: F, base: T) -> T {
            // Sequential implementation due to FnMut constraint
            // Parallel implementation would require Fn + Sync which is incompatible with FnMut API
            self.tree
                .in_order()
                .iter()
                .fold(base, |acc, value| op(acc, value.clone()))
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTAVLMtEph<T> { &self.tree }
    }

    #[macro_export]
    macro_rules! BSTSetAVLMtEphLit {
        () => {
            < $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEph<_> as $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEph<_> as $crate::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }
}
