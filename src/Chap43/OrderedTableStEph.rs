// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Single-threaded ephemeral ordered table backed by ParamBST<Pair<K,V>>.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod OrderedTableStEph {


    //		Section 2. imports

    use std::cmp::Ordering::Equal;
    use std::vec::IntoIter;

    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    use crate::Chap41::OrdKeyMap::OrdKeyMap::{OrdKeyMap, OrdKeyMapTrait};
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    pub use crate::Chap43::OrderedSpecsAndLemmas::OrderedSpecsAndLemmas::*;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
};

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEph<K: StT + Ord + TotalOrder, V: StT + Ord> {
        pub tree: OrdKeyMap<K, V>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    //		Section 5. view impls


    impl<K: StT + Ord + TotalOrder, V: StT + Ord> View for OrderedTableStEph<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { self.tree@ }
    }

    //		Section 8. traits


    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with ephemeral semantics.
    pub trait OrderedTableStEphTrait<K: StT + Ord + TotalOrder, V: StT + Ord>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablesteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- agrees with APAS
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablesteph_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- agrees with APAS
        fn empty() -> (empty: Self)
            requires
                obeys_feq_fulls::<K, V>(),
                obeys_feq_full::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures
                empty.spec_orderedtablesteph_wf(),
                empty@ == Map::<K::V, V::V>::empty();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- agrees with APAS
        fn singleton(k: K, v: V) -> (tree: Self)
            requires
                obeys_feq_clone::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST descent
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- agrees with APAS
        fn is_empty(&self) -> (is_empty: bool)
            requires self.spec_orderedtablesteph_wf(),
            ensures is_empty == self@.dom().is_empty();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST insert
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                !old(self)@.contains_key(k@) ==> old(self)@.dom().len() + 1 < usize::MAX as nat,
            ensures
                self@.contains_key(k@),
                self@.dom() =~= old(self)@.dom().insert(k@),
                forall|key: K::V| key != k@ && #[trigger] old(self)@.contains_key(key) ==> self@[key] == old(self)@[key],
                !old(self)@.contains_key(k@) ==> self@[k@] == v@,
                old(self)@.contains_key(k@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[k@] && combine.ensures((&old_v, &v), r) && self@[k@] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST delete
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite(), self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- collects keys from in_order
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires self.spec_orderedtablesteph_wf(), obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- inserts keys one by one
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
                obeys_feq_fulls::<K, V>(),
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_orderedtablesteph_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@),
                tabulated@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- collects, maps, rebuilds
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
            requires
                self.spec_orderedtablesteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                obeys_feq_clone::<Pair<K, V>>(),
            ensures mapped@.dom() =~= self@.dom(), mapped@.dom().finite(), mapped.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- collects, filters, rebuilds
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_orderedtablesteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                filtered@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] filtered@.contains_key(k) ==> filtered@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] filtered@.dom().contains(k),
                filtered@.dom().finite(),
                filtered.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- iterates all entries
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            requires self.spec_orderedtablesteph_wf(), forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n + m), Span Θ(n + m) -- iterative merge
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom() =~= old(self)@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old(self)@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && self@[k] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n + m), Span Θ(n + m) -- iterative merge
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
                old(self)@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) && !other@.contains_key(k)
                    ==> self@[k] == old(self)@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !old(self)@.contains_key(k)
                    ==> self@[k] == other@[k],
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old(self)@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && self@[k] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n + m), Span Θ(n + m) -- iterative difference
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedtablesteph_wf(), other.spec_orderedtablesteph_wf(),obeys_view_eq::<K>()
            ensures
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n * m), Span Θ(n * m) -- iterative restrict
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf()
            ensures
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n * m), Span Θ(n * m) -- iterative subtract
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf()
            ensures
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- collects from in_order
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            requires self.spec_orderedtablesteph_wf()
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST min
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST max
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST predecessor
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST successor
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST split
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@),
                !split.0@.dom().contains(k@),
                !split.2@.dom().contains(k@),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.2@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.2@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.2@.dom().contains(key) || key == k@,
                split.0.spec_orderedtablesteph_wf(),
                split.2.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n + m), Span Θ(n + m) -- delegates to union
        fn join_key(&mut self, other: Self)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
                old(self)@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n + m) where m = output size, Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST range
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST rank
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST select
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            requires
                self.spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- recursive BST split by rank
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_orderedtablesteph_wf(),
                split.1.spec_orderedtablesteph_wf();
        /// Iterative alternative to `find`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// Iterative alternative to `insert`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                !old(self)@.contains_key(k@) ==> old(self)@.dom().len() + 1 < usize::MAX as nat,
            ensures
                self@.contains_key(k@),
                self@.dom() =~= old(self)@.dom().insert(k@),
                forall|key: K::V| key != k@ && #[trigger] old(self)@.contains_key(key) ==> self@[key] == old(self)@[key],
                !old(self)@.contains_key(k@) ==> self@[k@] == v@,
                old(self)@.contains_key(k@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[k@] && combine.ensures((&old_v, &v), r) && self@[k@] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// Iterative alternative to `delete`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite(), self.spec_orderedtablesteph_wf();
        /// Iterative alternative to `first_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST min_key
        fn first_key_iter(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// Iterative alternative to `last_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST max_key
        fn last_key_iter(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// Iterative alternative to `previous_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST predecessor
        fn previous_key_iter(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// Iterative alternative to `next_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST successor
        fn next_key_iter(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// Iterative alternative to `split_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST key-only split
        fn split_key_iter(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@),
                !split.0@.dom().contains(k@),
                !split.2@.dom().contains(k@),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.2@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.2@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.2@.dom().contains(key) || key == k@,
                split.0.spec_orderedtablesteph_wf(),
                split.2.spec_orderedtablesteph_wf();
        /// Iterative alternative to `get_key_range`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- two BST key-only splits
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (range: Self)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_orderedtablesteph_wf();
        /// Iterative alternative to `rank_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST rank
        fn rank_key_iter(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// Iterative alternative to `split_rank_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST size-based select + split
        fn split_rank_key_iter(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_orderedtablesteph_wf(),
                split.1.spec_orderedtablesteph_wf();
    }

    //		Section 9. impls


    // DELETED R156: bst_find_by_key (~145 lines), bst_split_by_key (~450 lines).
    // Both moved to OrdKeyMap (ordkeymap_find, ordkeymap_split) in Chap38/OrdKeyMap.rs.
    // Dead after delegating split_key_iter, get_key_range_iter to OrdKeyMap.



    // BYPASSED: bst_rank_by_key, bst_select_by_rank — delegated to OrdKeyMap::rank_key, OrdKeyMap::select_key.

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        open spec fn spec_orderedtablesteph_wf(&self) -> bool {
            self.tree.spec_ordkeymap_wf()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            self.tree.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty()
        {
            OrderedTableStEph { tree: OrdKeyMap::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablesteph_wf()
        {
            let bst = ParamBST::singleton(Pair(k, v));
            // Veracity: NEEDED proof block
            proof {
                // bst@ == Set::empty().insert((k@, v@)).
                let s = Set::<(K::V, V::V)>::empty().insert((k@, v@));
                // Veracity: NEEDED assert (speed hint)
                assert(bst@ =~= s);
                // spec_pair_set_to_map(s) should be Map::empty().insert(k@, v@).
                lemma_set_to_map_empty::<K::V, V::V>();
                lemma_key_unique_empty::<K::V, V::V>();
                lemma_key_unique_insert(Set::<(K::V, V::V)>::empty(), k@, v@);
                lemma_set_to_map_insert(Set::empty(), k@, v@);
                lemma_pair_set_to_map_dom_finite(s);
                // Type axioms for wf: feq via broadcast, rest from requires.
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<V>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            OrderedTableStEph { tree: OrdKeyMap { inner: bst } }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::find
        fn find(&self, k: &K) -> (found: Option<V>)
        {
            self.tree.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>) {
            self.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == self@.dom().is_empty()
        {
            self.tree.is_empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>)
        {
            // Delegate to recursive find for now.
            self.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
        {
            // Delegate to recursive insert for now.
            self.insert(k, v, combine)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::find + OrdKeyMap::insert
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
        {
            let existing = self.find(&k);
            let ghost old_map = self@;
            match existing {
                Some(old_v) => {
                    let combined = combine(&old_v, &v);
                    self.tree.insert(k, combined);
                },
                None => {
                    self.tree.insert(k, v);
                },
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&mut self, k: &K) -> (updated: Option<V>)
        {
            self.delete(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::delete
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
        {
            let existing = self.find(k);
            self.tree.delete(k);
            existing
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to OrdKeyMap::domain
        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            self.tree.domain()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to OrdKeyMap::tabulate
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
            OrderedTableStEph { tree: OrdKeyMap::tabulate(keys, &f) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to OrdKeyMap::map_values
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
        {
            OrderedTableStEph { tree: self.tree.map_values(f) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to OrdKeyMap::filter
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
        {
            OrderedTableStEph { tree: self.tree.filter(f, Ghost(spec_pred)) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + fold
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite()
        {
            let sorted = self.tree.inner.in_order();
            let len = sorted.length();
            let mut reduced = init;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == sorted@.len(),
                    forall|r: R, k: &K, v: &V| f.requires((r, k, v)),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                reduced = f(reduced, &pair.0, &pair.1);
                i = i + 1;
            // Veracity: NEEDED proof block
            }
            proof { lemma_pair_set_to_map_dom_finite(self.tree.inner@); }
            reduced
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to OrdKeyMap::intersect_with
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        // Veracity: NEEDED proof block
        {
            self.tree = self.tree.intersect_with(&other.tree, &f);
            proof {
                lemma_pair_set_to_map_dom_finite(self.tree.inner@);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to OrdKeyMap::union_with
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            // Veracity: NEEDED proof block
            let ghost old_tree = self.tree;
            let combined = self.tree.union_with(&other.tree, &f);
            self.tree = combined;
            proof {
                lemma_pair_set_to_map_dom_finite(self.tree.inner@);
                // Bridge the "both keys" existential from OrdKeyMap ensures.
                // OrdKeyMap::union_with ensures: for k in both old_tree@ and other.tree@,
                // exists v1 v2 r with combine.ensures((&v1,&v2),r) && combined@[k]==r@.
                // We must relay with f.ensures since combine == &f.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] old_tree@.contains_key(k) && other.tree@.contains_key(k)
                    implies (exists|v1: V, v2: V, r: V|
                        v1@ == old_tree@[k] && v2@ == other.tree@[k]
                        && f.ensures((&v1, &v2), r) && combined@[k] == r@)
                by {
                    let ghost _v = combined@[k];
                };
            }
        }

        // BYPASSED R158: union old two-phase implementation (~390 lines).
        // Veracity: NEEDED proof block
        // Delegated to OrdKeyMap::union_with. See git history at commit before R158.
        #[cfg(never)] fn union_bypassed_r158() { let ghost old_tree = self.tree.inner@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            proof {
// Veracity: UNNEEDED assert                 assert(obeys_feq_full_trigger::<V>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                lemma_pair_set_to_map_len(old_tree);
                lemma_pair_set_to_map_len(other.tree.inner@);
            }
            // Veracity: NEEDED proof block
            // Phase 1: iterate self entries, merge with other where overlapping.
            let self_sorted = self.tree.inner.in_order();
            let self_len = self_sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, self_sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                // Empty set is trivially view-generated.
                // Veracity: NEEDED assert (speed hint)
                assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                    // Veracity: NEEDED assert (speed hint)
                    assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        // Veracity: NEEDED assert (speed hint)
                        assert(false);
                    };
                };
            }
            while i < self_len
                invariant
                    self.tree.inner@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    self_len as nat == self_sorted@.len(),
                    self_sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < self_sorted@.len() && 0 <= jj < self_sorted@.len() && ii != jj
                        ==> (#[trigger] self_sorted@[ii]).0 != (#[trigger] self_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] self_sorted@[j]).0,
                    0 <= i <= self_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() == i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                    old_map == spec_pair_set_to_map(old_tree),
                    other_map == other@,
                    // Phase 1 completeness.
                    forall|j2: int| 0 <= j2 < i as int
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0),
                    // Phase 1 value tracking (unified per-pair).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        old_map.dom().contains(p.0) &&
                        ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                        || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
// Veracity: UNNEEDED proof block                             v1@ == old_map[p.0] && v2@ == other_map[p.0]
                            && f.ensures((&v1, &v2), r) && p.1 == r@)),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                // Veracity: NEEDED proof block
                decreases self_len - i,
            {
                let pair = self_sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                let ghost old_new_tree_view = new_tree@;
                proof {
                    // Freshness: self_sorted@[i].0 not in new_tree.
// Veracity: UNNEEDED assert                     assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(self_sorted@[i as int].0)) by {
// Veracity: UNNEEDED assert                         if spec_pair_set_to_map(old_new_tree_view).dom().contains(self_sorted@[i as int].0) {
// Veracity: UNNEEDED assert                             lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[i as int].0);
// Veracity: UNNEEDED assert                             let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((self_sorted@[i as int].0, vv));
// Veracity: UNNEEDED assert                             let jj = choose|jj: int| 0 <= jj < i as int && (self_sorted@[i as int].0, vv).0 == (#[trigger] self_sorted@[jj]).0;
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(false);
// Veracity: UNNEEDED assert                         }
// Veracity: UNNEEDED assert                     };
                    // Link sorted entry to old_tree.
                    // Veracity: NEEDED assert (speed hint)
                    assert(self_sorted@.contains(self_sorted@[i as int])) by { assert(self_sorted@[i as int] == self_sorted@[i as int]); };
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                     assert(old_tree.contains(self_sorted@[i as int]));
                    lemma_pair_in_set_map_contains(old_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
                // Veracity: NEEDED proof block
                }
                match other_find {
                    Some(ov) => {
                        let combined = f(&pair.1, &ov);
                        let key_clone = pair.0.clone_plus();
                        proof { lemma_cloned_view_eq(pair.0, key_clone); }
                        new_tree.insert(Pair(key_clone, combined));
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, Pair(key_clone, combined));
// Veracity: UNNEEDED assert                             assert(new_tree@.len() == i as nat + 1);
// Veracity: UNNEEDED assert                             assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, combined@);
                            // Completeness maintenance.
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, combined@);
// Veracity: UNNEEDED assert                             assert forall|j2: int| 0 <= j2 < i as int
// Veracity: UNNEEDED assert                                 implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
// Veracity: UNNEEDED assert                             by {
// Veracity: UNNEEDED assert                                 lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
// Veracity: UNNEEDED assert                                 let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(new_tree@.contains((self_sorted@[j2].0, w)));
// Veracity: UNNEEDED assert                                 lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
// Veracity: UNNEEDED assert                             };
                            // Value tracking maintenance: new pair is combined.
// Veracity: UNNEEDED assert                             assert(old_map.dom().contains(self_sorted@[i as int].0)) by {
// Veracity: UNNEEDED assert                                 lemma_pair_in_set_map_contains(old_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
// Veracity: UNNEEDED assert                             };
                            // Veracity: NEEDED assert
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                old_map.dom().contains(p.0) &&
                                ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                                || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                    v1@ == old_map[p.0] && v2@ == other_map[p.0]
                                    && f.ensures((&v1, &v2), r) && p.1 == r@))
                            by {
                                if old_new_tree_view.contains(p) {
                                    // Existing pair — invariant held before insert.
                                } else {
                                    // New pair: p == (self_sorted@[i].0, combined@).
// Veracity: UNNEEDED assert                                     assert(p.0 == self_sorted@[i as int].0);
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                                     assert(p.1 == combined@);
// Veracity: UNNEEDED assert                                     assert(other_map.dom().contains(p.0));
// Veracity: UNNEEDED proof block                                 }
// Veracity: UNNEEDED proof block                             };
// Veracity: UNNEEDED proof block                         }
// Veracity: UNNEEDED proof block                     },
// Veracity: UNNEEDED proof block                     None => {
// Veracity: UNNEEDED proof block                         let cloned = pair.clone_plus();
// Veracity: UNNEEDED proof block                         proof { lemma_cloned_view_eq(*pair, cloned); }
// Veracity: UNNEEDED proof block                         new_tree.insert(cloned);
// Veracity: UNNEEDED proof block                         proof {
// Veracity: UNNEEDED proof block                             lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                             assert(new_tree@.len() == i as nat + 1);
// Veracity: UNNEEDED proof block                             // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block                             assert(new_tree@.len() < usize::MAX as nat);
// Veracity: UNNEEDED proof block                             lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, self_sorted@[i as int].1);
// Veracity: UNNEEDED proof block                             // Completeness maintenance.
// Veracity: UNNEEDED proof block                             lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, self_sorted@[i as int].1);
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                             assert forall|j2: int| 0 <= j2 < i as int
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                                 implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                             by {
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                                 lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                                 let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(new_tree@.contains((self_sorted@[j2].0, w)));
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                                 lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                             };
// Veracity: UNNEEDED proof block                             // Value tracking maintenance: new pair is self-only.
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                             assert(old_map.dom().contains(self_sorted@[i as int].0)) by {
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                                 lemma_pair_in_set_map_contains(old_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert                             };
// Veracity: UNNEEDED proof block                             // Veracity: NEEDED assert
// Veracity: UNNEEDED proof block                             assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
// Veracity: UNNEEDED proof block                                 old_map.dom().contains(p.0) &&
// Veracity: UNNEEDED proof block                                 ((!other_map.dom().contains(p.0) && old_tree.contains(p))
// Veracity: UNNEEDED proof block                                 || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
// Veracity: UNNEEDED proof block                                     v1@ == old_map[p.0] && v2@ == other_map[p.0]
// Veracity: UNNEEDED proof block                                     && f.ensures((&v1, &v2), r) && p.1 == r@))
// Veracity: UNNEEDED proof block                             by {
// Veracity: UNNEEDED proof block                                 if old_new_tree_view.contains(p) {
// Veracity: UNNEEDED proof block                                     // Existing pair — invariant held before insert.
// Veracity: UNNEEDED proof block                                 } else {
                                    // New pair: p == self_sorted@[i] (cloned).
// Veracity: UNNEEDED assert                                     assert(p.0 == self_sorted@[i as int].0);
// Veracity: UNNEEDED assert                                     assert(p.1 == self_sorted@[i as int].1);
// Veracity: UNNEEDED assert                                     assert(!other_map.dom().contains(p.0));
// Veracity: UNNEEDED assert                                     assert(old_tree.contains(p));
                                }
                            };
                        }
                    // Veracity: NEEDED proof block
                    },
                }
                i += 1;
            }
            // Phase 2: iterate other entries, add those not in self.
            let other_sorted = other.tree.inner.in_order();
            let other_len = other_sorted.length();
            let mut j: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(other.tree.inner@, other_sorted@);
                // Bridge: old keys preserved (Phase 1 completeness → per-key form).
                // Veracity: NEEDED assert
                assert forall|kv: K::V| #[trigger] old_map.dom().contains(kv)
                    implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                by {
                    lemma_map_contains_pair_in_set(old_tree, kv);
                    let vv: V::V = choose|vv: V::V| old_tree.contains((kv, vv));
// Veracity: UNNEEDED assert                     assert(self_sorted@.contains((kv, vv)));
                    let jx: int = choose|jx: int| 0 <= jx < self_sorted@.len() as int && self_sorted@[jx] == (kv, vv);
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[jx].0));
                };
                lemma_pair_set_to_map_len(old_tree);
                lemma_pair_set_to_map_len(other.tree.inner@);
            }
            while j < other_len
                invariant
                    self.tree.inner@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    other_map == other@,
                    old_map == spec_pair_set_to_map(old_tree),
                    other_len as nat == other_sorted@.len(),
                    other_sorted@.len() == other.tree.inner@.len(),
                    self_sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| other.tree.inner@.contains(v) <==> #[trigger] other_sorted@.contains(v),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < other_sorted@.len() && 0 <= jj < other_sorted@.len() && ii != jj
                        ==> (#[trigger] other_sorted@[ii]).0 != (#[trigger] other_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        old_map.dom().contains(p.0) ||
                        (exists|j2: int| 0 <= j2 < j as int && p.0 == (#[trigger] other_sorted@[j2]).0),
                    0 <= j <= other_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= self_sorted@.len() + j as nat,
                    self_sorted@.len() + other_sorted@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                    // Old keys preserved.
                    forall|kv: K::V| #[trigger] old_map.dom().contains(kv)
                        ==> spec_pair_set_to_map(new_tree@).dom().contains(kv),
                    // Other completeness.
                    forall|j2: int| 0 <= j2 < j as int && !old_map.dom().contains(other_sorted@[j2].0)
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j2].0),
                    // Phase 2 value tracking (3-way).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        (old_map.dom().contains(p.0) &&
                            // Veracity: NEEDED proof block
                            ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                            || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                v1@ == old_map[p.0] && v2@ == other_map[p.0]
                                && f.ensures((&v1, &v2), r) && p.1 == r@)))
                        || (!old_map.dom().contains(p.0) && other.tree.inner@.contains(p)),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                // Veracity: NEEDED proof block
                decreases other_len - j,
            {
                let pair = other_sorted.nth(j);
                proof { reveal(obeys_view_eq); }
                let in_self = self.find(&pair.0);
                match in_self {
                    None => {
                        // find returned None → !old_map.dom().contains(other_sorted@[j].0).
                        let cloned = pair.clone_plus();
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            // Freshness: other_sorted@[j].0 not already in new_tree.
// Veracity: UNNEEDED assert                             assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(other_sorted@[j as int].0)) by {
// Veracity: UNNEEDED assert                                 if spec_pair_set_to_map(old_new_tree_view).dom().contains(other_sorted@[j as int].0) {
// Veracity: UNNEEDED assert                                     lemma_map_contains_pair_in_set(old_new_tree_view, other_sorted@[j as int].0);
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                                     let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((other_sorted@[j as int].0, vv));
// Veracity: UNNEEDED assert                                     if old_map.dom().contains(other_sorted@[j as int].0) {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                         assert(false);
// Veracity: UNNEEDED assert                                     } else {
// Veracity: UNNEEDED assert                                         let j2 = choose|j2: int| 0 <= j2 < j as int && (other_sorted@[j as int].0, vv).0 == (#[trigger] other_sorted@[j2]).0;
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                         assert(false);
// Veracity: UNNEEDED assert                                     }
// Veracity: UNNEEDED assert                                 }
// Veracity: UNNEEDED assert                             };
                        }
                        new_tree.insert(cloned);
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
// Veracity: UNNEEDED assert                             assert(new_tree@.len() <= self_sorted@.len() + j as nat + 1);
                            lemma_key_unique_insert(old_new_tree_view, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            // Old keys preserved maintenance.
                            // Veracity: NEEDED assert
                            assert forall|kv: K::V| #[trigger] old_map.dom().contains(kv)
                                implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, kv);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((kv, w));
// Veracity: UNNEEDED assert                                 assert(new_tree@.contains((kv, w)));
                                lemma_pair_in_set_map_contains(new_tree@, kv, w);
                            };
                            // Other completeness maintenance.
// Veracity: UNNEEDED assert                             assert(other_sorted@.contains(other_sorted@[j as int])) by {
// Veracity: UNNEEDED assert                                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                                 assert(other_sorted@[j as int] == other_sorted@[j as int]);
// Veracity: UNNEEDED assert                             };
                            // Veracity: NEEDED assert (speed hint)
                            assert(other.tree.inner@.contains(other_sorted@[j as int]));
                            lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            // Veracity: NEEDED assert
                            assert forall|j2: int| 0 <= j2 < j as int + 1
                                && !old_map.dom().contains(other_sorted@[j2].0)
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j2].0)
                            by {
                                if j2 == j as int {
                                } else {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, other_sorted@[j2].0);
                                    let w: V::V = choose|w: V::V| old_new_tree_view.contains((other_sorted@[j2].0, w));
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(new_tree@.contains((other_sorted@[j2].0, w)));
                                    lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j2].0, w);
                                }
                            };
                            // Value tracking maintenance.
                            // Veracity: NEEDED assert
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                (old_map.dom().contains(p.0) &&
                                    ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                                    || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                        v1@ == old_map[p.0] && v2@ == other_map[p.0]
                                        && f.ensures((&v1, &v2), r) && p.1 == r@)))
                                || (!old_map.dom().contains(p.0) && other.tree.inner@.contains(p))
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(p.0 == other_sorted@[j as int].0);
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                                     assert(p.1 == other_sorted@[j as int].1);
// Veracity: UNNEEDED assert                                     assert(!old_map.dom().contains(p.0));
// Veracity: UNNEEDED assert                                     assert(other.tree.inner@.contains(p));
                                }
                            };
                        }
                    },
                    Some(_) => {},
                }
                j += 1;
            }
            self.tree = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_dom_finite(other.tree.inner@);
                // 1. Domain: self@.dom() =~= old_map.dom().union(other_map.dom()).
// Veracity: UNNEEDED assert                 assert(self@.dom() =~= old_map.dom().union(other_map.dom())) by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                     assert forall|kv: K::V| self@.dom().contains(kv)
// Veracity: UNNEEDED assert                         implies #[trigger] old_map.dom().union(other_map.dom()).contains(kv)
// Veracity: UNNEEDED assert                     by {
// Veracity: UNNEEDED assert                         lemma_map_contains_pair_in_set(self.tree.inner@, kv);
// Veracity: UNNEEDED assert                         let vv: V::V = choose|vv: V::V| self.tree.inner@.contains((kv, vv));
// Veracity: UNNEEDED assert                         if !old_map.dom().contains(kv) {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(other.tree.inner@.contains((kv, vv)));
// Veracity: UNNEEDED assert                             lemma_pair_in_set_map_contains(other.tree.inner@, kv, vv);
// Veracity: UNNEEDED assert                         }
// Veracity: UNNEEDED assert                     };
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                     assert forall|kv: K::V| #[trigger] old_map.dom().union(other_map.dom()).contains(kv)
// Veracity: UNNEEDED assert                         implies self@.dom().contains(kv)
// Veracity: UNNEEDED assert                     by {
// Veracity: UNNEEDED assert                         if old_map.dom().contains(kv) {
// Veracity: UNNEEDED assert                         } else {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(other_map.dom().contains(kv));
// Veracity: UNNEEDED assert                             lemma_map_contains_pair_in_set(other.tree.inner@, kv);
// Veracity: UNNEEDED assert                             let vv: V::V = choose|vv: V::V| other.tree.inner@.contains((kv, vv));
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(other_sorted@.contains((kv, vv)));
// Veracity: UNNEEDED assert                             let jx: int = choose|jx: int| 0 <= jx < other_sorted@.len() as int && other_sorted@[jx] == (kv, vv);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(!old_map.dom().contains(other_sorted@[jx].0));
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[jx].0));
// Veracity: UNNEEDED assert                         }
// Veracity: UNNEEDED assert                     };
// Veracity: UNNEEDED assert                 };
                // 2. Self-only values.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] old_map.contains_key(k) && !other_map.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(self.tree.inner@, k);
                    let vv: V::V = choose|vv: V::V| self.tree.inner@.contains((k, vv));
                    // Veracity: NEEDED assert (speed hint)
                    assert(old_tree.contains((k, vv)));
                    lemma_pair_in_set_map_contains(self.tree.inner@, k, vv);
                    lemma_pair_in_set_map_contains(old_tree, k, vv);
                };
                // 3. Other-only values.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] other_map.contains_key(k) && !old_map.contains_key(k)
                    implies self@[k] == other_map[k]
                by {
                    lemma_map_contains_pair_in_set(self.tree.inner@, k);
                    let vv: V::V = choose|vv: V::V| self.tree.inner@.contains((k, vv));
// Veracity: UNNEEDED assert                     assert(other.tree.inner@.contains((k, vv)));
                    lemma_pair_in_set_map_contains(self.tree.inner@, k, vv);
                    lemma_pair_in_set_map_contains(other.tree.inner@, k, vv);
                };
                // 4. Both values.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] old_map.contains_key(k) && other_map.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old_map[k] && v2@ == other_map[k]
                        && f.ensures((&v1, &v2), r)
                        && self@[k] == r@)
                by {
// Veracity: UNNEEDED proof block                     lemma_map_contains_pair_in_set(self.tree.inner@, k);
                    let vv: V::V = choose|vv: V::V| self.tree.inner@.contains((k, vv));
                    lemma_pair_in_set_map_contains(self.tree.inner@, k, vv);
                };
                // 5. wf.
// Veracity: UNNEEDED assert                 assert(self.tree.inner@.len() < usize::MAX as nat);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to OrdKeyMap::difference
        fn difference(&mut self, other: &Self)
        {
            self.tree = self.tree.difference(&other.tree);
            // Veracity: NEEDED proof block
            proof { lemma_pair_set_to_map_dom_finite(self.tree.inner@); }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- filter by key set membership
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_tree = self.tree.inner@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let sorted = self.tree.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.tree.inner@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@ == keys_set,
                    keys@.finite(),
                    old_map == spec_pair_set_to_map(old_tree),
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    // Entries in new_tree have keys in keys_set.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        keys_set.contains(p.0),
                    // Completeness: processed entries in keys_set are in new_tree.
                    forall|j: int| 0 <= j < i as int && keys_set.contains(sorted@[j].0)
                        ==> #[trigger] new_tree@.contains(sorted@[j]),
                    0 <= i <= len,
                    // Veracity: NEEDED proof block
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let in_keys = keys.find(&pair.0);
                if in_keys {
                    // Veracity: NEEDED proof block
                    let cloned = pair.clone_plus();
                    let ghost old_new_tree_view = new_tree@;
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        // Veracity: NEEDED assert (speed hint)
                        assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
// Veracity: UNNEEDED assert                                 assert(false);
                            }
                        };
                    }
                    // Veracity: NEEDED proof block
                    new_tree.insert(cloned);
                    proof {
// Veracity: UNNEEDED assert                         assert(new_tree@.len() <= i as nat + 1);
// Veracity: UNNEEDED assert                         assert(new_tree@.len() < usize::MAX as nat);
                        lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                        // Veracity: NEEDED assert
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_tree.contains(sorted@[i as int]));
                        // Veracity: NEEDED assert (speed hint)
                        assert(new_tree@.contains(sorted@[i as int]));
                    }
                }
                i = i + 1;
            }
            self.tree = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                // Prove: self@.dom() =~= old_map.dom().intersect(keys_set)
                // Veracity: NEEDED assert
                assert(self@.dom() =~= old_map.dom().intersect(keys_set)) by {
                    // Forward: k in self dom ==> k in old dom and k in keys_set.
                    // Veracity: NEEDED assert
                    assert forall|k: K::V| #[trigger] self@.dom().contains(k)
                        implies old_map.dom().contains(k) && keys_set.contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
// Veracity: UNNEEDED assert                         assert(old_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
// Veracity: UNNEEDED assert                         assert(keys_set.contains(k));
                    };
                    // Backward: k in old dom and k in keys_set ==> k in self dom.
                    // Veracity: NEEDED assert
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && keys_set.contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
// Veracity: UNNEEDED assert                         assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
// Veracity: UNNEEDED assert                         assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                // Prove: values preserved.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
// Veracity: UNNEEDED assert                     assert(old_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                // Type axioms flow from old(self).spec_orderedtablesteph_wf().
                // Veracity: NEEDED assert (speed hint)
                assert(spec_pair_key_determines_order::<K, V>());
                // Veracity: NEEDED assert (speed hint)
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
// Veracity: UNNEEDED proof block                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block                 assert(view_ord_consistent::<K>());
// Veracity: UNNEEDED proof block                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block                 assert(obeys_feq_fulls::<K, V>());
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- filter by key set exclusion
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_tree = self.tree.inner@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let sorted = self.tree.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.tree.inner@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@ == keys_set,
                    keys@.finite(),
                    old_map == spec_pair_set_to_map(old_tree),
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    // Entries in new_tree have keys NOT in keys_set.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        !keys_set.contains(p.0),
                    // Completeness: processed entries not in keys_set are in new_tree.
                    // Veracity: NEEDED proof block
                    forall|j: int| 0 <= j < i as int && !keys_set.contains(sorted@[j].0)
                        ==> #[trigger] new_tree@.contains(sorted@[j]),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                decreases len - i,
            {
                // Veracity: NEEDED proof block
                let pair = sorted.nth(i);
                let in_keys = keys.find(&pair.0);
                if !in_keys {
                    let cloned = pair.clone_plus();
                    let ghost old_new_tree_view = new_tree@;
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        // Veracity: NEEDED assert (speed hint)
                        assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
// Veracity: UNNEEDED assert                                 assert(false);
                            // Veracity: NEEDED proof block
                            }
                        };
                    }
                    new_tree.insert(cloned);
                    proof {
// Veracity: UNNEEDED assert                         assert(new_tree@.len() <= i as nat + 1);
                        // Veracity: NEEDED assert (speed hint)
                        assert(new_tree@.len() < usize::MAX as nat);
                        lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                        // Veracity: NEEDED assert
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_tree.contains(sorted@[i as int]));
// Veracity: UNNEEDED assert                         assert(new_tree@.contains(sorted@[i as int]));
                    }
                }
                i = i + 1;
            }
            self.tree = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                // Prove: self@.dom() =~= old_map.dom().difference(keys_set)
                // Veracity: NEEDED assert
                assert(self@.dom() =~= old_map.dom().difference(keys_set)) by {
                    // Forward: k in self dom ==> k in old dom and k not in keys_set.
                    // Veracity: NEEDED assert
                    assert forall|k: K::V| #[trigger] self@.dom().contains(k)
                        implies old_map.dom().contains(k) && !keys_set.contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
// Veracity: UNNEEDED assert                         assert(!keys_set.contains(k));
                    };
                    // Backward: k in old dom and k not in keys_set ==> k in self dom.
                    // Veracity: NEEDED assert
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && !keys_set.contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
// Veracity: UNNEEDED assert                         assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
// Veracity: UNNEEDED assert                         assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                // Prove: values preserved.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                     assert(old_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                // Type axioms flow from old(self).spec_orderedtablesteph_wf().
// Veracity: UNNEEDED assert                 assert(spec_pair_key_determines_order::<K, V>());
// Veracity: UNNEEDED assert                 assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
// Veracity: UNNEEDED assert                 assert(view_ord_consistent::<K>());
// Veracity: UNNEEDED assert                 assert(obeys_feq_fulls::<K, V>());
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to OrdKeyMap::collect + from_vec
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures
                self@.dom().finite(),
                collected.spec_avltreeseqstper_wf(),
                collected@.len() == self@.dom().len(),
        {
            let entries = self.tree.collect();
            proof {
                lemma_pair_set_to_map_len(self.tree.inner@);
                lemma_pair_set_to_map_dom_finite(self.tree.inner@);
                // entries@.len() == self@.dom().len() == self.tree.inner@.len() < usize::MAX.
            }
            AVLTreeSeqStPerS::from_vec(entries)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to first_key_iter
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            self.first_key_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::first_key
        fn first_key_iter(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            self.tree.first_key()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to last_key_iter
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            self.last_key_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::last_key
        fn last_key_iter(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            self.tree.last_key()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to previous_key_iter
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            self.previous_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::prev_key
        fn previous_key_iter(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            self.tree.prev_key(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to next_key_iter
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            self.next_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::next_key
        fn next_key_iter(&self, k: &K) -> (successor: Option<K>)
// Veracity: UNNEEDED proof block             where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            self.tree.next_key(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to split_key_iter
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            self.split_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST key-only split via expose + join_mid
        fn split_key_iter(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            proof { lemma_pair_set_to_map_dom_finite(self.tree.inner@); }
            let (left_map, found_val, right_map) = self.tree.split(k);
            *self = Self::empty();
            let left_table = OrderedTableStEph { tree: left_map };
            let right_table = OrderedTableStEph { tree: right_map };
            (left_table, found_val, right_table)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to union
        fn join_key(&mut self, other: Self)
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to get_key_range_iter
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
        {
            self.get_key_range_iter(k1, k2)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- two BST key-only splits
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (range: Self)
        {
            let range_map = self.tree.get_key_range(k1, k2);
            OrderedTableStEph { tree: range_map }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to rank_key_iter
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            self.rank_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::rank_key
        fn rank_key_iter(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            self.tree.rank_key(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::select_key
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            self.tree.select_key(i)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to split_rank_key_iter
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            self.split_rank_key_iter(i)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST size-based select + key-only split
        fn split_rank_key_iter(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let (left_map, right_map) = self.tree.split_rank_key(i);
            *self = Self::empty();
            let left_table = OrderedTableStEph { tree: left_map };
            let right_table = OrderedTableStEph { tree: right_map };
            (left_table, right_table)
        }
    }


    impl<K: StT + Ord + TotalOrder, V: StT + Ord> OrderedTableStEph<K, V> {
        /// Returns an iterator over the table entries via in-order traversal.
        pub fn iter(&self) -> (it: OrderedTableStEphIter<K, V>)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree.inner@.len(),
                iter_invariant(&it),
        {
            let sorted = self.tree.inner.in_order();
            OrderedTableStEphIter { inner: sorted.seq.into_iter() }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n BST inserts from sorted entries
    #[verifier::loop_isolation(false)]
    // Veracity: NEEDED proof block
    pub fn from_sorted_entries<K: StT + Ord + TotalOrder, V: StT + Ord>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (table: OrderedTableStEph<K, V>)
        requires
            entries.spec_avltreeseqstper_wf(),
            obeys_feq_clone::<Pair<K, V>>(),
            obeys_feq_full::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
            view_ord_consistent::<Pair<K, V>>(),
            spec_pair_key_determines_order::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            entries@.len() < usize::MAX as nat,
            // Entries must have unique keys.
            forall|ii: int, jj: int| 0 <= ii < jj < entries@.len()
                ==> (#[trigger] entries@[ii]).0 != (#[trigger] entries@[jj]).0,
        ensures
            table@.dom().finite(),
            table.spec_orderedtablesteph_wf(),
    {
        proof {
// Veracity: UNNEEDED assert             assert(obeys_feq_full_trigger::<K>());
// Veracity: UNNEEDED assert             assert(obeys_feq_full_trigger::<V>());
            // Veracity: NEEDED assert (speed hint)
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            lemma_key_unique_empty::<K::V, V::V>();
        }
        let len = entries.length();
        let mut tree = ParamBST::<Pair<K, V>>::new();
        let mut i: usize = 0;
        while i < len
            invariant
// Veracity: UNNEEDED proof block                 i <= len,
                len as nat == entries@.len(),
                // Veracity: NEEDED proof block
                entries@.len() < usize::MAX as nat,
                entries.spec_avltreeseqstper_wf(),
                tree.spec_bstparasteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                tree@.len() <= i as nat,
                tree@.len() < usize::MAX as nat,
                spec_key_unique_pairs_set(tree@),
                // Provenance: every element in the tree came from entries[0..i].
                forall|kv: K::V, vv: V::V| #[trigger] tree@.contains((kv, vv)) ==>
                    exists|j: int| #![trigger entries@[j]] 0 <= j < i as int && entries@[j] == (kv, vv),
                spec_set_pair_view_generated::<K, V>(tree@),
                // Entries have unique keys (from requires).
                forall|ii: int, jj: int| 0 <= ii < jj < entries@.len()
                    ==> (#[trigger] entries@[ii]).0 != (#[trigger] entries@[jj]).0,
            decreases len - i,
        {
            let ghost old_tree = tree@;
            let elem = entries.nth(i);
            let cloned = elem.clone_plus();
            proof { lemma_cloned_view_eq(*elem, cloned); }
            tree.insert(cloned);
            proof {
// Veracity: UNNEEDED assert                 assert(tree@.len() <= i as nat + 1);
// Veracity: UNNEEDED assert                 assert(i as nat + 1 <= len as nat);
// Veracity: UNNEEDED assert                 assert(tree@.len() < usize::MAX as nat);
                // Prove provenance for the new tree.
// Veracity: UNNEEDED assert                 assert forall|kv: K::V, vv: V::V| #[trigger] tree@.contains((kv, vv))
// Veracity: UNNEEDED assert                     implies exists|j: int| #![trigger entries@[j]] 0 <= j < i as int + 1 && entries@[j] == (kv, vv) by {
// Veracity: UNNEEDED assert                     if old_tree.contains((kv, vv)) {
// Veracity: UNNEEDED assert                         let j = choose|j: int| #![trigger entries@[j]] 0 <= j < i as int && entries@[j] == (kv, vv);
// Veracity: UNNEEDED assert                         // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                         assert(entries@[j] == (kv, vv) && j < i as int + 1);
// Veracity: UNNEEDED assert                     } else {
// Veracity: UNNEEDED assert                         // Must be the newly inserted element.
// Veracity: UNNEEDED assert                         // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                         assert((kv, vv) == cloned@);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         assert(entries@[i as int] == cloned@);
// Veracity: UNNEEDED assert                     }
// Veracity: UNNEEDED assert                 };
                // Prove key uniqueness is maintained.
// Veracity: UNNEEDED assert                 assert(spec_key_unique_pairs_set(tree@)) by {
// Veracity: UNNEEDED assert 
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                     assert forall|k: K::V, v1: V::V, v2: V::V|
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         tree@.contains((k, v1)) && tree@.contains((k, v2)) implies v1 == v2 by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         if old_tree.contains((k, v1)) && old_tree.contains((k, v2)) {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             // Both in old tree: follows from old invariant.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         } else if !old_tree.contains((k, v1)) && !old_tree.contains((k, v2)) {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             // Both are the new element.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert((k, v1) == cloned@ && (k, v2) == cloned@);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         } else {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             // One old, one new: contradiction via unique keys.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             if old_tree.contains((k, v1)) {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // (k, v2) == cloned@, so k == cloned@.0 == entries@[i].0.
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 let j1 = choose|j: int| #![trigger entries@[j]]
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                     0 <= j < i as int && entries@[j] == (k, v1);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // entries@[j1].0 == k == entries@[i].0, but j1 < i.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(entries@[j1].0 == entries@[i as int].0);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(j1 < i as int);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(false); // contradicts unique keys
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             } else {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // (k, v1) == cloned@
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 let j2 = choose|j: int| #![trigger entries@[j]]
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                     0 <= j < i as int && entries@[j] == (k, v2);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(entries@[j2].0 == entries@[i as int].0);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(j2 < i as int);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(false);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             }
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         }
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                     };
// Veracity: UNNEEDED assert                 };
            }
            i = i + 1;
        }
        let table = OrderedTableStEph { tree: OrdKeyMap { inner: tree } };
        proof { lemma_pair_set_to_map_dom_finite(tree@); }
        table
    }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphIter<K: StT + Ord + TotalOrder, V: StT + Ord> {
        pub inner: IntoIter<Pair<K, V>>,
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> View for OrderedTableStEphIter<K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<K: StT + Ord + TotalOrder, V: StT + Ord>(it: &OrderedTableStEphIter<K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> std::iter::Iterator for OrderedTableStEphIter<K, V> {
        type Item = Pair<K, V>;

        fn next(&mut self) -> (next: Option<Pair<K, V>>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphGhostIterator<K: StT + Ord + TotalOrder, V: StT + Ord> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> View for OrderedTableStEphGhostIterator<K, V> {
        type V = Seq<Pair<K, V>>;
        open spec fn view(&self) -> Seq<Pair<K, V>> { self.elements.take(self.pos) }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStEphIter<K, V> {
        type GhostIter = OrderedTableStEphGhostIterator<K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStEphGhostIterator<K, V> {
            OrderedTableStEphGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedTableStEphGhostIterator<K, V> {
        type ExecIter = OrderedTableStEphIter<K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStEphIter<K, V>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<Pair<K, V>> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStEphIter<K, V>) -> OrderedTableStEphGhostIterator<K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord + TotalOrder, V: StT + Ord> std::iter::IntoIterator for &'a OrderedTableStEph<K, V> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableStEphIter<K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree.inner@.len(),
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    //		Section 12. derive impls in verus!


    impl<K: StT + Ord + TotalOrder, V: StT + Ord> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> (cloned: Self) {
            OrderedTableStEph { tree: self.tree.clone() }
        }
    }
    } // verus!

    //		Section 13. macros


    /// Macro for creating ephemeral ordered tables from sorted key-value pairs.
    #[macro_export]
    macro_rules! OrderedTableStEphLit {
        () => {
            $crate::Chap43::OrderedTableStEph::OrderedTableStEph::OrderedTableStEph::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableStEph::OrderedTableStEph::from_sorted_entries(seq)
        }};
    }

    //		Section 14. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> PartialEq for OrderedTableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.size() == other.size()
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Debug for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Display for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Debug for OrderedTableStEphIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OrderedTableStEphIter").finish()
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Display for OrderedTableStEphIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEphIter")
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Debug for OrderedTableStEphGhostIterator<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEphGhostIterator")
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Display for OrderedTableStEphGhostIterator<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEphGhostIterator")
        }
    }
}
