// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
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
//	Section 10. iterators — BSTSetTreapMtEph
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module


pub mod BSTSetTreapMtEph {


    //		Section 2. imports

    use std::fmt;

    use std::cmp::Ordering::{Less, Greater};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::accept::accept;

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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn empty() -> (set: Self)
            ensures set@.len() == 0, set.spec_bstsettreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook (CS 38.11 singleton is O(1); this file's APAS line O(lg n) misreads it); does not match old analysis: O(lg n) vs new; join_mid of two leaves takes the lucky branch
        fn singleton(value: T) -> (set: Self)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures set@.len() == 1, set@.contains(value@), set.spec_bstsettreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn size(&self) -> (count: usize)
            ensures count == self@.len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: ParamTreap::find pays an O(size) deep-copy expose per level; does not match old analysis: O(lg n) vs new; expose clones subtrees
        fn find(&self, value: &T) -> (found: Option<T>)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures
                found matches Some(v) ==> v@ == value@ && self@.contains(v@),
                found is None ==> !self@.contains(value@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: calls find, which pays an O(size) deep-copy expose per level; does not match old analysis: O(lg n) vs new; expose clones subtrees
        fn contains(&self, value: &T) -> (found: bool)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures found == self@.contains(value@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: minimum_inner pays an O(size) deep-copy expose per level of the left spine; does not match old analysis: O(lg n) vs new; expose clones subtrees
        fn minimum(&self) -> (min: Option<T>)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures
                self@.len() == 0 ==> min is None,
                min matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: maximum_inner pays an O(size) deep-copy expose per level of the right spine; does not match old analysis: O(lg n) vs new; expose clones subtrees
        fn maximum(&self) -> (max: Option<T>)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures
                self@.len() == 0 ==> max is None,
                max matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: ParamTreap split and join_mid pay an O(size) deep-copy expose per level; does not match old analysis: O(lg n) vs new; expose clones subtrees
        fn insert(&mut self, value: T)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@ =~= old(self)@.insert(value@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n) expected, Span O(n lg n) expected — does not match textbook: ParamTreap join_pair clones and runs the sequential join_pair_inner union; does not match old analysis: O(lg n) vs new
        fn delete(&mut self, target: &T)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@ =~= old(self)@.remove(target@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O((m+n) lg m) expected, Span O(m + n lg m) expected, m = |self|, n = |other| — does not match textbook: ParamTreap union_inner pays deep-copy exposes, O(|b_i|) splits, and O(size) joins; does not match old analysis: O(m lg(n/m)), O(lg n) vs new; expose clones subtrees
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@ == self@.union(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(m lg^2 m + n lg m) expected, Span O(m lg m + n lg m) expected, m = |self|, n = |other| — does not match textbook: ParamTreap intersect_inner pays deep-copy exposes, splits, and sequential join_pair_inner; does not match old analysis: O(m lg(n/m)), O(lg n) vs new; expose clones subtrees
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures common@ == self@.intersect(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(m lg^2 m + n lg m) expected, Span O(m lg m + n lg m) expected, m = |self|, n = |other| — does not match textbook: ParamTreap difference_inner pays deep-copy exposes, splits, and sequential join_pair_inner; does not match old analysis: O(m lg(n/m)), O(lg n) vs new; expose clones subtrees
        fn difference(&self, other: &Self) -> (diff: Self)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures diff@ == self@.difference(other@);
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: ParamTreap split_inner pays an O(size) deep-copy expose per level; does not match old analysis: O(lg |t|) vs new; expose clones subtrees
        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self))
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures
                parts.1 == self@.contains(pivot@),
                parts.0@.disjoint(parts.2@),
                !parts.0@.contains(pivot@) && !parts.2@.contains(pivot@),
                parts.0@.union(parts.2@) =~= self@.remove(pivot@),
                self@ =~= parts.0@.union(parts.2@).union(
                    if parts.1 { Set::<<T as View>::V>::empty().insert(pivot@) } else { Set::<<T as View>::V>::empty() }
                ),
                forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(pivot) == Less,
                forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(pivot) == Greater;
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n) expected, Span O(n lg n) expected, n = |t1|+|t2| — does not match textbook: ParamTreap join_pair clones t1 and runs the sequential join_pair_inner union; does not match old analysis: O(lg(|t1|+|t2|)) vs new
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                left@.disjoint(right@),
                left@.len() + right@.len() < usize::MAX as nat,
                forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@ =~= left@.union(right@);
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(|t1|+|t2|) expected, Span O(|t1|+|t2|) expected — does not match textbook: ParamTreap join_mid descends with deep-copying expose_internal; does not match old analysis: O(lg(|t1|+|t2|)) vs new; expose clones subtrees
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                left@.disjoint(right@),
                !left@.contains(pivot@),
                !right@.contains(pivot@),
                left@.len() + right@.len() < usize::MAX as nat,
                forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&pivot) == Less,
                forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&pivot) == Greater,
            ensures joined@ =~= left@.union(right@).insert(pivot@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg^2 n + Σ W(f)) expected, Span O(n lg^2 n + Σ S(f)) expected — does not match textbook: ParamTreap filter runs the sequential filter_inner with deep-copy exposes and join_pair_inner; does not match old analysis: O(n), O(lg n) vs new
        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
                forall|x: T, keep: bool|
                    predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
                self@.len() < usize::MAX as nat,
            ensures
                filtered@.subset_of(self@),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n + Σ W(f)) expected, Span O(n + lg n max S(f)) expected — does not match textbook: ParamTreap reduce_inner forks, but pays an O(size) deep-copy expose per node; does not match old analysis: O(n), O(lg n) vs new; expose clones subtrees
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
            requires
                vstd::laws_cmp::obeys_cmp::<T>(),
                view_ord_consistent::<T>(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n) expected, Span O(n lg n) expected — does not match textbook: ParamTreap in_order pays an O(size) deep-copy expose per node; does not match old analysis: O(n) vs new; expose clones subtrees
        fn iter_in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures ordered.spec_len() == self@.len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn as_tree(&self) -> (tree: &ParamTreap<T>)
            ensures tree@ == self@;
    }

    //		Section 9. impls


    #[verifier::exec_allows_no_decreases_clause]
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — no textbook cost; does not match old analysis: O(lg n) expected vs new; tree.expose() deep-copies both subtrees at each level of the left spine
    fn minimum_inner<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>) -> (min: Option<T>)
        requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
        ensures
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
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — no textbook cost; does not match old analysis: O(lg n) expected vs new; tree.expose() deep-copies both subtrees at each level of the right spine
    fn maximum_inner<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>) -> (max: Option<T>)
        requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
        ensures
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
            true
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn empty() -> (set: Self) {
            BSTSetTreapMtEph { tree: ParamTreap::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn singleton(value: T) -> (set: Self) {
            let set = Self::join_m(Self::empty(), value, Self::empty());
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof {
                let empty = Set::<<T as View>::V>::empty();
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(set@ =~= empty.insert(value@));
            }
            set
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook; does not match old analysis: O(n) vs new; ParamTreap::size reads the cached size field
        fn size(&self) -> (count: usize) { self.tree.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn is_empty(&self) -> (empty: bool) { self.tree.is_empty() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: ParamTreap::find pays an O(size) deep-copy expose per level; does not match old analysis: O(lg n) expected vs new; expose clones subtrees
        fn find(&self, value: &T) -> (found: Option<T>) { self.tree.find(value) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: calls find, O(size) deep-copy expose per level; does not match old analysis: O(lg n) expected vs new; expose clones subtrees
        fn contains(&self, value: &T) -> (found: bool)
        {
            self.find(value).is_some()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: minimum_inner pays an O(size) deep-copy expose per level of the left spine; does not match old analysis: O(lg n) expected vs new; expose clones subtrees
        fn minimum(&self) -> (min: Option<T>) {
            minimum_inner(&self.tree)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: maximum_inner pays an O(size) deep-copy expose per level of the right spine; does not match old analysis: O(lg n) expected vs new; expose clones subtrees
        fn maximum(&self) -> (max: Option<T>) {
            maximum_inner(&self.tree)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: split and join_m pay an O(size) deep-copy expose per level; does not match old analysis: O(lg n) expected vs new; expose clones subtrees
        fn insert(&mut self, value: T) {
            let ghost old_len = self@.len();
            let (left, _found, right) = self.split(&value);
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof {
                // left@ and right@ partition self@.remove(value@) ⊆ self@.
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(left@.union(right@), self@);
            }
            *self = Self::join_m(left, value, right);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n) expected, Span O(n lg n) expected — does not match textbook: join_pair clones and runs the sequential join_pair_inner union; does not match old analysis: O(lg n) expected vs new
        fn delete(&mut self, target: &T) {
            let ghost kref = *target;
            let ghost old_view = self@;
            // Veracity: NEEDED proof block
            let (left, _found, right) = self.split(target);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry(o, kref);
                    lemma_cmp_transitivity(s, kref, o);
                };
            }
            *self = Self::join_pair(left, right);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O((m+n) lg m) expected, Span O(m + n lg m) expected, m = |self|, n = |other| — does not match textbook: ParamTreap union_inner pays deep-copy exposes, O(|b_i|) splits, and O(size) joins; does not match old analysis: O(n lg n), O(lg^2 n) vs new; expose clones subtrees
        fn union(&self, other: &Self) -> (combined: Self) {
            BSTSetTreapMtEph { tree: self.tree.union(&other.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(m lg^2 m + n lg m) expected, Span O(m lg m + n lg m) expected, m = |self|, n = |other| — does not match textbook: ParamTreap intersect_inner pays deep-copy exposes, splits, and sequential join_pair_inner; does not match old analysis: O(n lg n), O(lg^2 n) vs new; expose clones subtrees
        fn intersection(&self, other: &Self) -> (common: Self) {
            BSTSetTreapMtEph { tree: self.tree.intersect(&other.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(m lg^2 m + n lg m) expected, Span O(m lg m + n lg m) expected, m = |self|, n = |other| — does not match textbook: ParamTreap difference_inner pays deep-copy exposes, splits, and sequential join_pair_inner; does not match old analysis: O(n lg n), O(lg^2 n) vs new; expose clones subtrees
        fn difference(&self, other: &Self) -> (diff: Self) {
            BSTSetTreapMtEph { tree: self.tree.difference(&other.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n) expected, Span O(n) expected — does not match textbook: ParamTreap split_inner pays an O(size) deep-copy expose per level; does not match old analysis: O(lg n) expected vs new; expose clones subtrees
        fn split(&self, pivot: &T) -> (parts: (Self, bool, Self)) {
            let (left, found, right) = self.tree.split(pivot);
            (BSTSetTreapMtEph { tree: left }, found, BSTSetTreapMtEph { tree: right })
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n) expected, Span O(n lg n) expected, n = |t1|+|t2| — does not match textbook: ParamTreap join_pair clones t1 and runs the sequential join_pair_inner union; does not match old analysis: O(lg n) expected vs new
        fn join_pair(left: Self, right: Self) -> (joined: Self) {
            BSTSetTreapMtEph { tree: left.tree.join_pair(right.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(|t1|+|t2|) expected, Span O(|t1|+|t2|) expected — does not match textbook: ParamTreap join_mid descends with deep-copying expose_internal; does not match old analysis: O(lg n) expected vs new; expose clones subtrees
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self) {
            BSTSetTreapMtEph {
                tree: ParamTreap::join_mid(Exposed::Node(left.tree, pivot, right.tree)),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg^2 n + Σ W(f)) expected, Span O(n lg^2 n + Σ S(f)) expected — does not match textbook: ParamTreap filter runs the sequential filter_inner with deep-copy exposes and join_pair_inner; does not match old analysis: O(n lg n), O(lg^2 n) vs new
        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self) {
            BSTSetTreapMtEph { tree: self.tree.filter(predicate, Ghost(spec_pred)) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n + Σ W(f)) expected, Span O(n + lg n max S(f)) expected — does not match textbook: ParamTreap reduce_inner forks, but pays an O(size) deep-copy expose per node; does not match old analysis: O(n), O(n) vs new; expose clones subtrees
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            self.tree.reduce(op, base)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n) expected, Span O(n lg n) expected — does not match textbook: ParamTreap in_order pays an O(size) deep-copy expose per node; does not match old analysis: O(1) vs new; in_order traverses the whole tree
        fn iter_in_order(&self) -> (ordered: ArraySeqStPerS<T>) { self.tree.in_order() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn as_tree(&self) -> (tree: &ParamTreap<T>) { &self.tree }
    }

    //		Section 10. iterators — BSTSetTreapMtEph

    // r212 form C: this `IntoIterator` impl required `requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>()`, which
    // verus 0.2026.09.13 rejects on an external trait's impl and no exec check
    // can establish; use `iter()`, which keeps the requires
    // (src/experiments/intoiter_form_c_no_impl.rs).
    /*
    impl<'a, T: MtKey + ClonePreservesView> std::iter::IntoIterator for &'a BSTSetTreapMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetTreapMtEphIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures
                it@.0 == 0,
                it@.1.len() == self@.len(),
                iter_invariant_bststreapmteph(&it),
        {
            let in_ord = self.iter_in_order();
            BSTSetTreapMtEphIter { inner: in_ord.seq.into_iter() }
        }
    }
    */

    impl<T: MtKey + ClonePreservesView> BSTSetTreapMtEph<T> {
        /// Returns a snapshot iterator over the set elements in ascending key order.
        /// - Alg Analysis: Code review (Claude Fable 5.1): Work O(n), Span O(n) — in-order traversal.
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n lg n) expected, Span O(n lg n) expected — no textbook cost; does not match old analysis: O(n) vs new; iter_in_order pays an O(size) deep-copy expose per node
        pub fn iter(&self) -> (it: std::vec::IntoIter<T>)
            requires vstd::laws_cmp::obeys_cmp::<T>(), view_ord_consistent::<T>(),
            ensures
                IteratorSpec::remaining(&it).len() == self@.len(),
                vstd::std_specs::vec::into_iter_elts(it) == IteratorSpec::remaining(&it),
                IteratorSpec::decrease(&it) is Some,
        {
            let in_ord = self.iter_in_order();
            in_ord.seq.into_iter()
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: MtKey + ClonePreservesView> Clone for BSTSetTreapMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        // Veracity: NEEDED proof block
        {
            let cloned = BSTSetTreapMtEph { tree: self.tree.clone() };
            // Veracity: NEEDED proof block
            proof { accept(cloned@ == self@); } // Clone bridge: view preserved by ParamTreap::clone.
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

    //		Section 14b. derive impls outside verus!
}
