//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral ordered set implementation using parallel Treap backing store.

pub mod OrderedSetMtEph {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Types::Types::*;

    pub struct OrderedSetMtEph<T: MtKey + 'static> {
        tree: ParamTreap<T>,
    }

    pub type OrderedSetMt<T> = OrderedSetMtEph<T>;

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with multi-threaded ephemeral semantics
    pub trait OrderedSetMtEphTrait<T: MtKey + 'static> {
        // Base set operations (ADT 41.1) - ephemeral semantics with parallelism
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                        -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                            -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                    -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                 -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn insert(&mut self, x: T);
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn delete(&mut self, x: &T);
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: Pred<T>>(&mut self, f: F);
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&mut self, other: &Self);
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&mut self, other: &Self);
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&mut self, other: &Self);
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self)                      -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n lg n), Span Θ(lg n), Parallelism Θ(1)
        fn from_seq(seq: ArraySeqStPerS<T>) -> Self;

        // Ordering operations (ADT 43.1) - sequential (inherently sequential on trees)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn first(&self)                       -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn last(&self)                        -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn previous(&self, k: &T)             -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn next(&self, k: &T)                 -> Option<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn split(&mut self, k: &T)            -> (Self, B, Self)
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn join(&mut self, other: Self);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn get_range(&self, k1: &T, k2: &T)   -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, k: &T)                 -> N;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn select(&self, i: N)                -> Option<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn split_rank(&mut self, i: N)        -> (Self, Self)
        where
            Self: Sized;
    }

    impl<T: MtKey + 'static> OrderedSetMtEphTrait<T> for OrderedSetMtEph<T> {
        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N { self.tree.size() }

        /// Claude Work: O(1), Span: O(1)
        fn empty() -> Self { OrderedSetMtEph { tree: ParamTreap::new() } }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn singleton(x: T) -> Self {
            let tree = ParamTreap::new();
            tree.insert(x);
            OrderedSetMtEph { tree }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn find(&self, x: &T) -> B { self.tree.find(x).is_some() }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn insert(&mut self, x: T) {
            self.tree.insert(x);
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn delete(&mut self, x: &T) {
            self.tree.delete(x);
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn filter<F: PredMt<T>>(&mut self, f: F) {
            self.tree = self.tree.filter(f);
        }

        /// Claude Work: O(m lg(n/m)), Span: O(lg n)
        fn intersection(&mut self, other: &Self) {
            self.tree = self.tree.intersect(&other.tree);
        }

        /// Claude Work: O(m lg(n/m)), Span: O(lg n)
        fn union(&mut self, other: &Self) {
            self.tree = self.tree.union(&other.tree);
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn difference(&mut self, other: &Self) {
            self.tree = self.tree.difference(&other.tree);
        }

        /// Claude Work: O(n), Span: O(n)
        fn to_seq(&self) -> ArraySeqStPerS<T> {
            self.tree.in_order()
        }

        /// Claude Work: O(n lg n), Span: O(lg n)
        fn from_seq(seq: ArraySeqStPerS<T>) -> Self {
            let tree = ParamTreap::new();
            for i in 0..seq.length() {
                tree.insert(seq.nth(i).clone());
            }
            OrderedSetMtEph { tree }
        }

        // Ordering operations (ADT 43.1)

        /// Claude Work: O(lg n), Span: O(lg n)
        fn first(&self) -> Option<T> {
            // Find minimum by traversing left spine
            match self.tree.expose() {
                Exposed::Leaf => None,
                Exposed::Node(left, key, _right) => {
                    let mut current_left = left;
                    let mut current_key = key;
                    loop {
                        match current_left.expose() {
                            Exposed::Leaf => return Some(current_key),
                            Exposed::Node(next_left, next_key, _) => {
                                current_left = next_left;
                                current_key = next_key;
                            }
                        }
                    }
                }
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn last(&self) -> Option<T> {
            // Find maximum by traversing right spine
            match self.tree.expose() {
                Exposed::Leaf => None,
                Exposed::Node(_left, key, right) => {
                    let mut current_right = right;
                    let mut current_key = key;
                    loop {
                        match current_right.expose() {
                            Exposed::Leaf => return Some(current_key),
                            Exposed::Node(_, next_key, next_right) => {
                                current_right = next_right;
                                current_key = next_key;
                            }
                        }
                    }
                }
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn previous(&self, k: &T) -> Option<T> {
            // Split at k, then find max of left tree
            let (left, _found, _right) = self.tree.split(k);
            // Find maximum in left tree (rightmost element)
            match left.expose() {
                Exposed::Leaf => None,
                Exposed::Node(_left_sub, key, right_sub) => {
                    let mut current_right = right_sub;
                    let mut current_key = key;
                    loop {
                        match current_right.expose() {
                            Exposed::Leaf => return Some(current_key),
                            Exposed::Node(_, next_key, next_right) => {
                                current_right = next_right;
                                current_key = next_key;
                            }
                        }
                    }
                }
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn next(&self, k: &T) -> Option<T> {
            // Split at k, then find min of right tree
            let (_left, _found, right) = self.tree.split(k);
            // Find minimum in right tree (leftmost element)
            match right.expose() {
                Exposed::Leaf => None,
                Exposed::Node(left_sub, key, _right_sub) => {
                    let mut current_left = left_sub;
                    let mut current_key = key;
                    loop {
                        match current_left.expose() {
                            Exposed::Leaf => return Some(current_key),
                            Exposed::Node(next_left, next_key, _) => {
                                current_left = next_left;
                                current_key = next_key;
                            }
                        }
                    }
                }
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn split(&mut self, k: &T) -> (Self, B, Self) {
            let (left_tree, found, right_tree) = self.tree.split(k);
            *self = Self::empty();
            (
                OrderedSetMtEph { tree: left_tree },
                found,
                OrderedSetMtEph { tree: right_tree },
            )
        }

        /// Claude Work: O(lg(m + n)), Span: O(lg(m + n))
        fn join(&mut self, other: Self) {
            self.tree = self.tree.join_pair(other.tree);
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn get_range(&self, k1: &T, k2: &T) -> Self {
            // Split at k1 to get elements >= k1
            let (_left, found1, right1) = self.tree.split(k1);
            // If k1 was found, rejoin it to right1
            let geq_k1 = if found1 {
                let singleton = ParamTreap::new();
                singleton.insert(k1.clone());
                singleton.join_pair(right1)
            } else {
                right1
            };
            // Split at k2 to get elements <= k2
            let (mid, found2, _right2) = geq_k1.split(k2);
            // If k2 was found, rejoin it to mid
            let result_tree = if found2 {
                let singleton = ParamTreap::new();
                singleton.insert(k2.clone());
                mid.join_pair(singleton)
            } else {
                mid
            };
            OrderedSetMtEph { tree: result_tree }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn rank(&self, k: &T) -> N {
            // Count elements < k using split
            let (left, _found, _right) = self.tree.split(k);
            left.size()
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn select(&self, i: N) -> Option<T> {
            // Select ith smallest element (0-indexed)
            // Convert sequence to sorted array and index
            let seq = self.tree.in_order();
            if i < seq.length() {
                Some(seq.nth(i).clone())
            } else {
                None
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn split_rank(&mut self, i: N) -> (Self, Self) {
            // Split at rank i: left has elements [0..i), right has elements [i..)
            let seq = self.tree.in_order();
            if i == 0 {
                // All elements go to right
                let result = (Self::empty(), OrderedSetMtEph { tree: self.tree.clone() });
                *self = Self::empty();
                return result;
            }
            if i >= seq.length() {
                // All elements go to left
                let result = (OrderedSetMtEph { tree: self.tree.clone() }, Self::empty());
                *self = Self::empty();
                return result;
            }
            let pivot = seq.nth(i);
            let (left, found, right) = self.tree.split(pivot);
            // Pivot should be included in right, so rejoin it if found
            let right_tree = if found {
                let singleton = ParamTreap::new();
                singleton.insert(pivot.clone());
                singleton.join_pair(right)
            } else {
                right
            };
            *self = Self::empty();
            (OrderedSetMtEph { tree: left }, OrderedSetMtEph { tree: right_tree })
        }
    }

    /// Macro for creating ordered sets from literals
    #[macro_export]
    macro_rules! OrderedSetMtEphLit {
        ($($x:expr),* $(,)?) => {
            {
                let mut set = $crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEph::empty();
                $(
                    set.insert($x);
                )*
                set
            }
        };
    }
}
