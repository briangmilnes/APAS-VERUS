//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface as a thin shim over BSTParaTreapMtEph.
//! All set algebra delegates to ParamTreap's split/join-based parallel algorithms.

//  Table of Contents
//	1. module
//	4. type definitions
//	5. view impls
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//	1. module


pub mod BSTSetTreapMtEph {

    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetTreapMtEph<T: MtKey> {
        pub tree: ParamTreap<T>,
    }

    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;

    // 5. view impls

    impl<T: MtKey> View for BSTSetTreapMtEph<T> {
        type V = Set<T::V>;

        open spec fn view(&self) -> Set<T::V> {
            self.tree@
        }
    }

    // 8. traits

    pub trait BSTSetTreapMtEphTrait<T: MtKey>: Sized + View<V = Set<T::V>> {
        spec fn spec_bstsettreapmteph_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (set: Self)
            ensures set@.finite(), set@.len() == 0, set.spec_bstsettreapmteph_wf();
        /// - APAS: Work O(log n), Span O(log n)
        fn singleton(value: T) -> (set: Self)
            ensures set@.finite(), set@.len() == 1, set@.contains(value@), set.spec_bstsettreapmteph_wf();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures self@.finite(), count == self@.len();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (empty: bool)
            ensures self@.finite(), empty == (self@.len() == 0);
        /// - APAS: Work O(log n), Span O(log n)
        fn find(&self, value: &T) -> (found: Option<T>)
            ensures found matches Some(v) ==> self@.contains(v@);
        /// - APAS: Work O(log n), Span O(log n)
        fn contains(&self, value: &T) -> (found: bool)
            ensures found == self@.contains(value@);
        /// - APAS: Work O(log n), Span O(log n)
        fn minimum(&self) -> (min: Option<T>)
            ensures
                self@.finite(),
                self@.len() == 0 ==> min is None,
                min matches Some(v) ==> self@.contains(v@);
        /// - APAS: Work O(log n), Span O(log n)
        fn maximum(&self) -> (max: Option<T>)
            ensures
                self@.finite(),
                self@.len() == 0 ==> max is None,
                max matches Some(v) ==> self@.contains(v@);
        /// - APAS: Work O(log n), Span O(log n)
        fn insert(&mut self, value: T)
            ensures self@.finite(), self@ =~= old(self)@.insert(value@);
        /// - APAS: Work O(log n), Span O(log n)
        fn delete(&mut self, target: &T)
            ensures self@.finite(), self@ =~= old(self)@.remove(target@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@.finite(), combined@ == self@.union(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn intersection(&self, other: &Self) -> (common: Self)
            ensures common@.finite(), common@ == self@.intersect(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn difference(&self, other: &Self) -> (diff: Self)
            ensures diff@.finite(), diff@ == self@.difference(other@);
        /// - APAS: Work O(log n), Span O(log n)
        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self))
            ensures
                parts.0@.finite(), parts.2@.finite(),
                parts.1 == self@.contains(pivot@);
        /// - APAS: Work O(lg(|left| + |right|)), Span O(lg(|left| + |right|))
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            ensures joined@.finite();
        /// - APAS: Work O(lg(|left| + |right|)), Span O(lg(|left| + |right|))
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            ensures joined@.finite();
        /// - APAS: Work Θ(n), Span O(lg n)
        fn filter<F: Pred<T>>(&self, predicate: F) -> (filtered: Self)
            ensures filtered@.finite();
        /// - APAS: Work Θ(n), Span O(lg n)
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
            ensures true;
        /// - APAS: Work Θ(n), Span Θ(n)
        fn iter_in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            ensures self@.finite(), ordered.spec_len() == self@.len();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn as_tree(&self) -> (tree: &ParamTreap<T>)
            ensures tree@ == self@;
    }

    // 9. impls

    #[verifier::exec_allows_no_decreases_clause]
    fn minimum_inner<T: MtKey + 'static>(tree: &ParamTreap<T>) -> (min: Option<T>)
        ensures
            tree@.finite(),
            tree@.len() == 0 ==> min is None,
            min matches Some(v) ==> tree@.contains(v@),
    {
        let min = match tree.expose_with_priority() {
            None => None,
            Some((left, key, _, _)) => {
                if left.is_empty() { Some(key) }
                else { minimum_inner(&left) }
            }
        };
        proof {
            accept(tree@.finite());
            accept(tree@.len() == 0 ==> min is None);
            accept(min matches Some(v) ==> tree@.contains(v@));
        }
        min
    }

    #[verifier::exec_allows_no_decreases_clause]
    fn maximum_inner<T: MtKey + 'static>(tree: &ParamTreap<T>) -> (max: Option<T>)
        ensures
            tree@.finite(),
            tree@.len() == 0 ==> max is None,
            max matches Some(v) ==> tree@.contains(v@),
    {
        let max = match tree.expose_with_priority() {
            None => None,
            Some((_, key, _, right)) => {
                if right.is_empty() { Some(key) }
                else { maximum_inner(&right) }
            }
        };
        proof {
            accept(tree@.finite());
            accept(tree@.len() == 0 ==> max is None);
            accept(max matches Some(v) ==> tree@.contains(v@));
        }
        max
    }

    impl<T: MtKey + 'static> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {
        open spec fn spec_bstsettreapmteph_wf(&self) -> bool {
            self@.finite()
        }

        fn empty() -> (set: Self) {
            BSTSetTreapMtEph { tree: ParamTreap::new() }
        }

        fn singleton(value: T) -> (set: Self) {
            let ghost v = value@;
            let tree = ParamTreap::new();
            tree.insert(value);
            let set = BSTSetTreapMtEph { tree };
            proof { accept(set@.finite() && set@.len() == 1 && set@.contains(v)); }
            set
        }

        fn size(&self) -> (count: usize) { self.tree.size() }

        fn is_empty(&self) -> (empty: bool) { self.tree.is_empty() }

        fn find(&self, value: &T) -> (found: Option<T>) { self.tree.find(value) }

        fn contains(&self, value: &T) -> (found: bool)
        {
            let result = self.find(value).is_some();
            proof { accept(result == self@.contains(value@)); }
            result
        }

        fn minimum(&self) -> (min: Option<T>) {
            let min = minimum_inner(&self.tree);
            proof { accept(self@.finite()); }
            min
        }

        fn maximum(&self) -> (max: Option<T>) {
            let max = maximum_inner(&self.tree);
            proof { accept(self@.finite()); }
            max
        }

        fn insert(&mut self, value: T) {
            let ghost v = value@;
            self.tree.insert(value);
            proof { accept(self@.finite() && self@ =~= old(self)@.insert(v)); }
        }

        fn delete(&mut self, target: &T) {
            self.tree.delete(target);
            proof { accept(self@.finite() && self@ =~= old(self)@.remove(target@)); }
        }

        fn union(&self, other: &Self) -> (combined: Self) {
            BSTSetTreapMtEph { tree: self.tree.union(&other.tree) }
        }

        fn intersection(&self, other: &Self) -> (common: Self) {
            BSTSetTreapMtEph { tree: self.tree.intersect(&other.tree) }
        }

        fn difference(&self, other: &Self) -> (diff: Self) {
            BSTSetTreapMtEph { tree: self.tree.difference(&other.tree) }
        }

        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self)) {
            let (left, found, right) = self.tree.split(pivot);
            (BSTSetTreapMtEph { tree: left }, found, BSTSetTreapMtEph { tree: right })
        }

        fn join_pair(left: Self, right: Self) -> (joined: Self) {
            BSTSetTreapMtEph { tree: left.tree.join_pair(right.tree) }
        }

        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self) {
            let joined = BSTSetTreapMtEph {
                tree: ParamTreap::join_mid(Exposed::Node(left.tree, pivot, right.tree)),
            };
            proof { accept(joined@.finite()); }
            joined
        }

        fn filter<F: Pred<T>>(&self, predicate: F) -> (filtered: Self) {
            BSTSetTreapMtEph { tree: self.tree.filter(predicate) }
        }

        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            self.tree.reduce(op, base)
        }

        fn iter_in_order(&self) -> (ordered: ArraySeqStPerS<T>) { self.tree.in_order() }

        fn as_tree(&self) -> (tree: &ParamTreap<T>) { &self.tree }
    }

    // 11. derive impls in verus!

    impl<T: MtKey> Clone for BSTSetTreapMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures true
        {
            BSTSetTreapMtEph { tree: self.tree.clone() }
        }
    }

    } // verus!

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


    // 13. derive impls outside verus!

    impl<T: MtKey + fmt::Debug> fmt::Debug for BSTSetTreapMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetTreapMtEph(size: {})", self.size())
        }
    }

    impl<T: MtKey> fmt::Display for BSTSetTreapMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetTreapMtEph(size: {})", self.size())
        }
    }
}
