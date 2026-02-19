//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral ordered set implementation using parallel Treap backing store.

pub mod OrderedSetMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    pub struct OrderedSetMtEph<T: MtKey + 'static> {
        tree: ParamTreap<T>,
    }

    pub type OrderedSetMt<T> = OrderedSetMtEph<T>;

    // 5. view impls

    impl<T: MtKey + 'static> View for OrderedSetMtEph<T> {
        type V = Set<T>;
        #[verifier::external_body]
        open spec fn view(&self) -> Set<T> { Set::empty() }
    }

    // 8. traits

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with multi-threaded ephemeral semantics
    pub trait OrderedSetMtEphTrait<T: MtKey + 'static> {
        // Base set operations (ADT 41.1) - ephemeral semantics with parallelism
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: N)
            ensures result == self@.len(), self@.finite();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Set::<T>::empty();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<T>::empty().insert(x), result@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(*x);
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(*x), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: Pred<T>>(&mut self, f: F)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&mut self, other: &Self)
            ensures self@ == old(self)@.intersect(other@), self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&mut self, other: &Self)
            ensures self@ == old(self)@.union(other@), self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&mut self, other: &Self)
            ensures self@ == old(self)@.difference(other@), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (result: ArraySeqStPerS<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(n lg n), Span Θ(lg n), Parallelism Θ(1)
        fn from_seq(seq: ArraySeqStPerS<T>) -> (result: Self)
            ensures result@.finite();

        // Ordering operations (ADT 43.1) - sequential (inherently sequential on trees)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn first(&self) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn last(&self) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn previous(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn next(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn split(&mut self, k: &T) -> (result: (Self, B, Self))
            where Self: Sized
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn join(&mut self, other: Self)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn get_range(&self, k1: &T, k2: &T) -> (result: Self)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, k: &T) -> (result: N)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn select(&self, i: N) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn split_rank(&mut self, i: N) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.finite();
    }

    // 9. impls

    impl<T: MtKey + 'static> OrderedSetMtEphTrait<T> for OrderedSetMtEph<T> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.len(), self@.finite()
        { self.tree.size() }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Set::<T>::empty()
        { OrderedSetMtEph { tree: ParamTreap::new() } }

        #[verifier::external_body]
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<T>::empty().insert(x), result@.finite()
        {
            let tree = ParamTreap::new();
            tree.insert(x);
            OrderedSetMtEph { tree }
        }

        #[verifier::external_body]
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(*x)
        { self.tree.find(x).is_some() }

        #[verifier::external_body]
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x), self@.finite()
        {
            self.tree.insert(x);
        }

        #[verifier::external_body]
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(*x), self@.finite()
        {
            self.tree.delete(x);
        }

        #[verifier::external_body]
        fn filter<F: PredMt<T>>(&mut self, f: F)
            ensures self@.finite()
        {
            self.tree = self.tree.filter(f);
        }

        #[verifier::external_body]
        fn intersection(&mut self, other: &Self)
            ensures self@ == old(self)@.intersect(other@), self@.finite()
        {
            self.tree = self.tree.intersect(&other.tree);
        }

        #[verifier::external_body]
        fn union(&mut self, other: &Self)
            ensures self@ == old(self)@.union(other@), self@.finite()
        {
            self.tree = self.tree.union(&other.tree);
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self)
            ensures self@ == old(self)@.difference(other@), self@.finite()
        {
            self.tree = self.tree.difference(&other.tree);
        }

        #[verifier::external_body]
        fn to_seq(&self) -> (result: ArraySeqStPerS<T>)
            ensures self@.finite()
        {
            self.tree.in_order()
        }

        #[verifier::external_body]
        fn from_seq(seq: ArraySeqStPerS<T>) -> (result: Self)
            ensures result@.finite()
        {
            let tree = ParamTreap::new();
            for i in 0..seq.length() {
                tree.insert(seq.nth(i).clone());
            }
            OrderedSetMtEph { tree }
        }

        #[verifier::external_body]
        fn first(&self) -> (result: Option<T>)
            ensures self@.finite()
        {
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

        #[verifier::external_body]
        fn last(&self) -> (result: Option<T>)
            ensures self@.finite()
        {
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

        #[verifier::external_body]
        fn previous(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite()
        {
            let (left, _found, _right) = self.tree.split(k);
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

        #[verifier::external_body]
        fn next(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite()
        {
            let (_left, _found, right) = self.tree.split(k);
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

        #[verifier::external_body]
        fn split(&mut self, k: &T) -> (result: (Self, B, Self))
            where Self: Sized
            ensures self@.finite()
        {
            let (left_tree, found, right_tree) = self.tree.split(k);
            *self = Self::empty();
            (
                OrderedSetMtEph { tree: left_tree },
                found,
                OrderedSetMtEph { tree: right_tree },
            )
        }

        #[verifier::external_body]
        fn join(&mut self, other: Self)
            ensures self@.finite()
        {
            self.tree = self.tree.join_pair(other.tree);
        }

        #[verifier::external_body]
        fn get_range(&self, k1: &T, k2: &T) -> (result: Self)
            ensures self@.finite()
        {
            let (_left, found1, right1) = self.tree.split(k1);
            let geq_k1 = if found1 {
                let singleton = ParamTreap::new();
                singleton.insert(k1.clone());
                singleton.join_pair(right1)
            } else {
                right1
            };
            let (mid, found2, _right2) = geq_k1.split(k2);
            let result_tree = if found2 {
                let singleton = ParamTreap::new();
                singleton.insert(k2.clone());
                mid.join_pair(singleton)
            } else {
                mid
            };
            OrderedSetMtEph { tree: result_tree }
        }

        #[verifier::external_body]
        fn rank(&self, k: &T) -> (result: N)
            ensures self@.finite()
        {
            let (left, _found, _right) = self.tree.split(k);
            left.size()
        }

        #[verifier::external_body]
        fn select(&self, i: N) -> (result: Option<T>)
            ensures self@.finite()
        {
            let seq = self.tree.in_order();
            if i < seq.length() {
                Some(seq.nth(i).clone())
            } else {
                None
            }
        }

        #[verifier::external_body]
        fn split_rank(&mut self, i: N) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.finite()
        {
            let seq = self.tree.in_order();
            if i == 0 {
                let result = (Self::empty(), OrderedSetMtEph { tree: self.tree.clone() });
                *self = Self::empty();
                return result;
            }
            if i >= seq.length() {
                let result = (OrderedSetMtEph { tree: self.tree.clone() }, Self::empty());
                *self = Self::empty();
                return result;
            }
            let pivot = seq.nth(i);
            let (left, found, right) = self.tree.split(pivot);
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

    } // verus!

    // 12. macros

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
