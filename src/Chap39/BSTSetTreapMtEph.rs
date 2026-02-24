//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface as a thin shim over BSTParaTreapMtEph.
//! All set algebra delegates to ParamTreap's split/join-based parallel algorithms.

pub mod BSTSetTreapMtEph {

    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetTreapMtEph<T: MtKey> {
        tree: ParamTreap<T>,
    }

    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;

    } // verus!

    // 6. helper functions

    impl<T: MtKey> Clone for BSTSetTreapMtEph<T> {
        fn clone(&self) -> Self {
            BSTSetTreapMtEph { tree: self.tree.clone() }
        }
    }

    fn minimum_inner<T: MtKey + 'static>(tree: &ParamTreap<T>) -> Option<T> {
        match tree.expose_with_priority() {
            None => None,
            Some((left, key, _, _)) => {
                if left.is_empty() { Some(key) }
                else { minimum_inner(&left) }
            }
        }
    }

    fn maximum_inner<T: MtKey + 'static>(tree: &ParamTreap<T>) -> Option<T> {
        match tree.expose_with_priority() {
            None => None,
            Some((_, key, _, right)) => {
                if right.is_empty() { Some(key) }
                else { maximum_inner(&right) }
            }
        }
    }

    // 8. traits

    pub trait BSTSetTreapMtEphTrait<T: MtKey>: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        fn empty()                                   -> Self;
        /// - APAS: Work O(log n), Span O(log n)
        fn singleton(value: T)                       -> Self;
        /// - APAS: Work Θ(1), Span Θ(1)
        fn size(&self)                               -> N;
        /// - APAS: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                           -> B;
        /// - APAS: Work O(log n), Span O(log n)
        fn find(&self, value: &T)                    -> Option<T>;
        /// - APAS: Work O(log n), Span O(log n)
        fn contains(&self, value: &T)                -> B;
        /// - APAS: Work O(log n), Span O(log n)
        fn minimum(&self)                            -> Option<T>;
        /// - APAS: Work O(log n), Span O(log n)
        fn maximum(&self)                            -> Option<T>;
        /// - APAS: Work O(log n), Span O(log n)
        fn insert(&mut self, value: T);
        /// - APAS: Work O(log n), Span O(log n)
        fn delete(&mut self, target: &T);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn union(&self, other: &Self)                -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn intersection(&self, other: &Self)         -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn difference(&self, other: &Self)           -> Self;
        /// - APAS: Work O(log n), Span O(log n)
        fn split(&self, pivot: &T)                   -> (Self, B, Self);
        /// - APAS: Work O(lg(|left| + |right|)), Span O(lg(|left| + |right|))
        fn join_pair(left: Self, right: Self)        -> Self;
        /// - APAS: Work O(lg(|left| + |right|)), Span O(lg(|left| + |right|))
        fn join_m(left: Self, pivot: T, right: Self) -> Self;
        /// - APAS: Work Θ(n), Span O(lg n)
        fn filter<F: Pred<T>>(&self, predicate: F)   -> Self;
        /// - APAS: Work Θ(n), Span O(lg n)
        fn reduce<F>(&self, op: F, base: T)          -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static;
        /// - APAS: Work Θ(n), Span Θ(n)
        fn iter_in_order(&self)                      -> ArraySeqStPerS<T>;
        /// - APAS: Work Θ(1), Span Θ(1)
        fn as_tree(&self)                            -> &ParamTreap<T>;
    }

    // 9. impls

    impl<T: MtKey + 'static> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {
        fn empty() -> Self {
            BSTSetTreapMtEph { tree: ParamTreap::new() }
        }

        fn singleton(value: T) -> Self {
            let tree = ParamTreap::new();
            tree.insert(value);
            BSTSetTreapMtEph { tree }
        }

        fn size(&self) -> N { self.tree.size() }

        fn is_empty(&self) -> B { self.tree.is_empty() }

        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        fn contains(&self, value: &T) -> B { self.find(value).is_some() }

        fn minimum(&self) -> Option<T> { minimum_inner(&self.tree) }

        fn maximum(&self) -> Option<T> { maximum_inner(&self.tree) }

        fn insert(&mut self, value: T) { self.tree.insert(value); }

        fn delete(&mut self, target: &T) { self.tree.delete(target); }

        fn union(&self, other: &Self) -> Self {
            BSTSetTreapMtEph { tree: self.tree.union(&other.tree) }
        }

        fn intersection(&self, other: &Self) -> Self {
            BSTSetTreapMtEph { tree: self.tree.intersect(&other.tree) }
        }

        fn difference(&self, other: &Self) -> Self {
            BSTSetTreapMtEph { tree: self.tree.difference(&other.tree) }
        }

        fn split(&self, pivot: &T) -> (Self, B, Self) {
            let (left, found, right) = self.tree.split(pivot);
            (BSTSetTreapMtEph { tree: left }, found, BSTSetTreapMtEph { tree: right })
        }

        fn join_pair(left: Self, right: Self) -> Self {
            BSTSetTreapMtEph { tree: left.tree.join_pair(right.tree) }
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            BSTSetTreapMtEph {
                tree: ParamTreap::join_mid(Exposed::Node(left.tree, pivot, right.tree)),
            }
        }

        fn filter<F: Pred<T>>(&self, predicate: F) -> Self {
            BSTSetTreapMtEph { tree: self.tree.filter(predicate) }
        }

        fn reduce<F>(&self, op: F, base: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            self.tree.reduce(op, base)
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &ParamTreap<T> { &self.tree }
    }

    // 13. derive impls outside verus!

    impl<T: MtKey + fmt::Debug> fmt::Debug for BSTSetTreapMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetTreapMtEph").finish()
        }
    }

    // 12. macros

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
