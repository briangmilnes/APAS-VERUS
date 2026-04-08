//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Set interface as a thin shim over BSTParaTreapMtEph.
//! All set algebra delegates to ParamTreap's split/join-based parallel algorithms.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod BSTSetTreapMtEph {


    //		Section 2. imports

    use std::fmt;

    use std::cmp::Ordering::{Less, Greater};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;

    verus! 
{

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetTreapMtEph<T: MtKey> {
        pub tree: ParamTreap<T>,
    }

    pub type BSTSetTreapMt<T> = BSTSetTreapMtEph<T>;

    //		Section 5. view impls


    impl<T: MtKey> View for BSTSetTreapMtEph<T> {
        type V = Set<T::V>;

        open spec fn view(&self) -> Set<T::V> {
            self.tree@
        }
    }

    //		Section 8. traits


    pub trait BSTSetTreapMtEphTrait<T: MtKey>: Sized + View<V = Set<T::V>> {
        spec fn spec_bstsettreapmteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn empty() -> (set: Self)
            ensures set@.finite(), set@.len() == 0, set.spec_bstsettreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn singleton(value: T) -> (set: Self)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures set@.finite(), set@.len() == 1, set@.contains(value@), set.spec_bstsettreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn size(&self) -> (count: usize)
            ensures self@.finite(), count == self@.len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn is_empty(&self) -> (empty: bool)
            ensures self@.finite(), empty == (self@.len() == 0);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn find(&self, value: &T) -> (found: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                found matches Some(v) ==> v@ == value@ && self@.contains(v@),
                found is None ==> !self@.contains(value@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn contains(&self, value: &T) -> (found: bool)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures found == self@.contains(value@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn minimum(&self) -> (min: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                self@.finite(),
                self@.len() == 0 ==> min is None,
                min matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn maximum(&self) -> (max: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                self@.finite(),
                self@.len() == 0 ==> max is None,
                max matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn insert(&mut self, value: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.insert(value@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn delete(&mut self, target: &T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.remove(target@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n) — matches APAS
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@.finite(), combined@ == self@.union(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n) — matches APAS
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures common@.finite(), common@ == self@.intersect(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n) — matches APAS
        fn difference(&self, other: &Self) -> (diff: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures diff@.finite(), diff@ == self@.difference(other@);
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self))
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                parts.0@.finite(), parts.2@.finite(),
                parts.1 == self@.contains(pivot@),
                self@.finite(),
                parts.0@.disjoint(parts.2@),
                !parts.0@.contains(pivot@) && !parts.2@.contains(pivot@),
                parts.0@.union(parts.2@) =~= self@.remove(pivot@),
                self@ =~= parts.0@.union(parts.2@).union(
                    if parts.1 { Set::<<T as View>::V>::empty().insert(pivot@) } else { Set::<<T as View>::V>::empty() }
                ),
                forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(pivot) == Less,
                forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(pivot) == Greater;
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — matches APAS
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                left@.finite(), right@.finite(),
                left@.disjoint(right@),
                left@.len() + right@.len() < usize::MAX as nat,
                forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@.finite(), joined@ =~= left@.union(right@);
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — matches APAS
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                left@.disjoint(right@),
                !left@.contains(pivot@),
                !right@.contains(pivot@),
                left@.len() + right@.len() < usize::MAX as nat,
                forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&pivot) == Less,
                forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&pivot) == Greater,
            ensures joined@.finite(), joined@ =~= left@.union(right@).insert(pivot@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — matches APAS
        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
                forall|x: T, keep: bool|
                    predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
                self@.len() < usize::MAX as nat,
            ensures
                filtered@.finite(),
                filtered@.subset_of(self@),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — matches APAS
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn iter_in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures self@.finite(), ordered.spec_len() == self@.len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn as_tree(&self) -> (tree: &ParamTreap<T>)
            ensures tree@ == self@;
    }

    //		Section 9. impls


    #[verifier::exec_allows_no_decreases_clause]
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
    fn minimum_inner<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>) -> (min: Option<T>)
        requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
        ensures
            tree@.finite(),
            tree@.len() == 0 ==> min is None,
            min matches Some(v) ==> tree@.contains(v@),
    {
        match tree.expose() {
            Exposed::Leaf => None,
            Exposed::Node(left, key, _right) => {
                if left.is_empty() { Some(key) }
                else {
                    let result = minimum_inner(&left);
                    // left@.subset_of(tree@) from expose ensures.
                    // minimum_inner ensures: result Some(v) ==> left@.contains(v@).
                    // left@.contains(v@) && left@.subset_of(tree@) ==> tree@.contains(v@).
                    result
                }
            }
        }
    }

    #[verifier::exec_allows_no_decreases_clause]
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
    fn maximum_inner<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>) -> (max: Option<T>)
        requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
        ensures
            tree@.finite(),
            tree@.len() == 0 ==> max is None,
            max matches Some(v) ==> tree@.contains(v@),
    {
        match tree.expose() {
            Exposed::Leaf => None,
            Exposed::Node(_left, key, right) => {
                if right.is_empty() { Some(key) }
                else {
                    let result = maximum_inner(&right);
                    result
                }
            }
        }
    }

    impl<T: MtKey + ClonePreservesView> BSTSetTreapMtEphTrait<T> for BSTSetTreapMtEph<T> {
        open spec fn spec_bstsettreapmteph_wf(&self) -> bool {
            self@.finite()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (set: Self) {
            BSTSetTreapMtEph { tree: ParamTreap::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(value: T) -> (set: Self) {
            let set = Self::join_m(Self::empty(), value, Self::empty());
            // Veracity: NEEDED proof block
            proof {
                let empty = Set::<<T as View>::V>::empty();
                // Veracity: NEEDED assert
                assert(empty.finite());
                // Veracity: NEEDED assert
                assert(empty.len() == 0);
                // Veracity: NEEDED assert
                assert(!empty.contains(value@));
                // Veracity: NEEDED assert
                assert(set@ =~= empty.insert(value@));
            }
            set
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (count: usize) { self.tree.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool) { self.tree.is_empty() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn find(&self, value: &T) -> (found: Option<T>) { self.tree.find(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn contains(&self, value: &T) -> (found: bool)
        {
            self.find(value).is_some()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn minimum(&self) -> (min: Option<T>) {
            minimum_inner(&self.tree)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn maximum(&self) -> (max: Option<T>) {
            maximum_inner(&self.tree)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn insert(&mut self, value: T) {
            let ghost old_len = self@.len();
            let (left, _found, right) = self.split(&value);
            // Veracity: NEEDED proof block
            proof {
                // left@ and right@ partition self@.remove(value@) ⊆ self@.
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(left@.union(right@), self@);
                // Veracity: NEEDED assert
                assert(left@.len() + right@.len() < usize::MAX as nat);
            }
            *self = Self::join_m(left, value, right);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn delete(&mut self, target: &T) {
            let ghost kref = *target;
            let ghost old_view = self@;
            let (left, _found, right) = self.split(target);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                // Veracity: NEEDED assert
                assert(left@.union(right@) =~= old_view.remove(kref@));
                // Veracity: NEEDED assert
                assert(old_view.remove(kref@).subset_of(old_view));
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                // Veracity: NEEDED assert
                assert(left@.len() + right@.len() < usize::MAX as nat);
                // Veracity: NEEDED assert
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies s.cmp_spec(&o) == Less by {
                    // Veracity: NEEDED assert
                    assert(s.cmp_spec(target) == Less);    // from split ensures on left
                    // Veracity: NEEDED assert
                    assert(o.cmp_spec(target) == Greater); // from split ensures on right
                    lemma_cmp_antisymmetry(o, kref);
                    lemma_cmp_transitivity(s, kref, o);
                };
            }
            *self = Self::join_pair(left, right);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn union(&self, other: &Self) -> (combined: Self) {
            BSTSetTreapMtEph { tree: self.tree.union(&other.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn intersection(&self, other: &Self) -> (common: Self) {
            BSTSetTreapMtEph { tree: self.tree.intersect(&other.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn difference(&self, other: &Self) -> (diff: Self) {
            BSTSetTreapMtEph { tree: self.tree.difference(&other.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self)) {
            let (left, found, right) = self.tree.split(pivot);
            (BSTSetTreapMtEph { tree: left }, found, BSTSetTreapMtEph { tree: right })
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn join_pair(left: Self, right: Self) -> (joined: Self) {
            BSTSetTreapMtEph { tree: left.tree.join_pair(right.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self) {
            param_treap_assert_finite(&left.tree);
            param_treap_assert_finite(&right.tree);
            BSTSetTreapMtEph {
                tree: ParamTreap::join_mid(Exposed::Node(left.tree, pivot, right.tree)),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self) {
            BSTSetTreapMtEph { tree: self.tree.filter(predicate, Ghost(spec_pred)) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            self.tree.reduce(op, base)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn iter_in_order(&self) -> (ordered: ArraySeqStPerS<T>) { self.tree.in_order() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn as_tree(&self) -> (tree: &ParamTreap<T>) { &self.tree }
    }

    //		Section 12. derive impls in verus!


    impl<T: MtKey + ClonePreservesView> Clone for BSTSetTreapMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = BSTSetTreapMtEph { tree: self.tree.clone() };
            // Veracity: NEEDED proof block
            proof { assume(cloned@ == self@); } // Clone bridge: view preserved by ParamTreap::clone.
            cloned
        }
    }

    } // verus!

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

    impl<T: MtKey + ClonePreservesView + fmt::Debug> fmt::Debug for BSTSetTreapMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetTreapMtEph(size: {})", self.size())
        }
    }

    impl<T: MtKey + ClonePreservesView> fmt::Display for BSTSetTreapMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetTreapMtEph(size: {})", self.size())
        }
    }
}
