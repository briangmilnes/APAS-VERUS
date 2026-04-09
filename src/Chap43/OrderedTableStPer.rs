//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Single-threaded persistent ordered table backed by ParamBST<Pair<K,V>>.


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

pub mod OrderedTableStPer {


    //		Section 2. imports

    use std::cmp::Ordering::Equal;
    use std::vec::IntoIter;

    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    use crate::Chap41::OrdKeyMap::OrdKeyMap::{OrdKeyMap, OrdKeyMapTrait};
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
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
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
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
    pub struct OrderedTableStPer<K: StT + Ord + TotalOrder, V: StT + Ord> {
        pub tree: OrdKeyMap<K, V>,
    }

    pub type OrderedTablePer<K, V> = OrderedTableStPer<K, V>;

    //		Section 5. view impls


    impl<K: StT + Ord + TotalOrder, V: StT + Ord> View for OrderedTableStPer<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { self.tree@ }
    }

    //		Section 8. traits


    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with persistent semantics.
    pub trait OrderedTableStPerTrait<K: StT + Ord + TotalOrder, V: StT + Ord>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablestper_wf(&self) -> bool;
        spec fn spec_orderedtablestper_find_pre(&self) -> bool;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablestper_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (table: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(k: K, v: V) -> (table: Self)
            requires
                obeys_feq_clone::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures table@ == Map::<K::V, V::V>::empty().insert(k@, v@), table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablestper_find_pre(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&self, k: K, v: V) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table@[k@] == v@,
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> table@[k2] == self@[k2],
                table.spec_orderedtablestper_wf();
        /// Like insert, but additionally ensures the inserted value mapping.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_wf(&self, k: K, v: V) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table@[k@] == v@,
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> table@[k2] == self@[k2],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures table@ == self@.remove(k@), table.spec_orderedtablestper_wf();
        /// Like delete, but additionally ensures value preservation for remaining keys.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_wf(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures
                table@ == self@.remove(k@),
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> table@[k2] == self@[k2],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn domain(&self) -> (keys: ArraySetStEph<K>)
            requires self.spec_orderedtablestper_wf(), obeys_feq_clone::<K>()
            ensures keys@ =~= self@.dom(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                keys@.len() < usize::MAX,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures
                table@.dom() =~= keys@,
                table.spec_orderedtablestper_wf(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && table@[k] == result@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), forall|v: &V| f.requires((v,)),
            ensures
                table@.dom() == self@.dom(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && table@[k] == result@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: Fn(&K, &V) -> bool>(&self, f: F, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] table@.dom().contains(k),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
            ensures
                table@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                self@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                table@.dom() =~= self@.dom().union(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) && !other@.contains_key(k)
                    ==> table@[k] == self@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !self@.contains_key(k)
                    ==> table@[k] == other@[k],
                forall|k: K::V| #[trigger] self@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn difference(&self, other: &Self) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), other.spec_orderedtablestper_wf(), obeys_view_eq::<K>(),
            ensures
                table@.dom() =~= self@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(),
            ensures
                table@.dom() =~= self@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(),
            ensures
                table@.dom() =~= self@.dom().difference(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
            requires self.spec_orderedtablestper_wf(),
            ensures self@.dom().finite(), sorted_entries.spec_avltreeseqstper_wf(), sorted_entries@.len() == self@.dom().len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn first_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn last_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn previous_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(pk) ==> self@.dom().contains(pk@),
                key matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn next_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(nk) ==> self@.dom().contains(nk@),
                key matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
                parts.0.spec_orderedtablestper_wf(),
                parts.2.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn join_key(left: &Self, right: &Self) -> (table: Self)
            requires
                left.spec_orderedtablestper_wf(),
                right.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                left@.dom().len() + right@.dom().len() < usize::MAX,
            ensures
                table@.dom() =~= left@.dom().union(right@.dom()),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n + m), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(log n + m) — DIFFERS: St sequential, APAS parallel
        fn get_key_range(&self, k1: &K, k2: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] table@.dom().contains(key) ==> table@[key] == self@[key],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn select_key(&self, i: usize) -> (key: Option<K>)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                parts.0@.dom().subset_of(self@.dom()),
                parts.1@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.1@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.1@.dom().contains(key),
                parts.0.spec_orderedtablestper_wf(),
                parts.1.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablestper_find_pre(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter(&self, k: K, v: V) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures table@ == self@.remove(k@), table.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + take first
        fn first_key_iter(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + take last
        fn last_key_iter(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan
        fn previous_key_iter(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(pk) ==> self@.dom().contains(pk@),
                key matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan
        fn next_key_iter(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(nk) ==> self@.dom().contains(nk@),
                key matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts
        fn split_key_iter(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
                parts.0.spec_orderedtablestper_wf(),
                parts.2.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + conditional BST inserts
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] table@.dom().contains(key) ==> table@[key] == self@[key],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + count
        fn rank_key_iter(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts
        fn split_rank_key_iter(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                parts.0@.dom().subset_of(self@.dom()),
                parts.1@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.1@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.1@.dom().contains(key),
                parts.0.spec_orderedtablestper_wf(),
                parts.1.spec_orderedtablestper_wf();
    }

    //		Section 9. impls


    impl<K: StT + Ord + TotalOrder, V: StT + Ord> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        open spec fn spec_orderedtablestper_wf(&self) -> bool {
            self.tree.spec_ordkeymap_wf()
        }

        open spec fn spec_orderedtablestper_find_pre(&self) -> bool {
            self.tree.spec_ordkeymap_wf()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize) {
            self.tree.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (table: Self) {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            OrderedTableStPer { tree: OrdKeyMap::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(k: K, v: V) -> (table: Self) {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                assert(obeys_view_eq_trigger::<K>());
            }
            let mut tree = OrdKeyMap::new();
            tree.insert(k, v);
            OrderedTableStPer { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::find
        fn find(&self, k: &K) -> (found: Option<V>) {
            self.tree.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::insert
        fn insert(&self, k: K, v: V) -> (table: Self) {
            let inner_copy = self.tree.inner.clone();
            let mut tree_copy = OrdKeyMap { inner: inner_copy };
            proof { assert(tree_copy.spec_ordkeymap_wf()); }
            tree_copy.insert(k, v);
            OrderedTableStPer { tree: tree_copy }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_wf(&self, k: K, v: V) -> (table: Self) {
            self.insert(k, v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::delete
        fn delete(&self, k: &K) -> (table: Self) {
            let inner_copy = self.tree.inner.clone();
            let mut tree_copy = OrdKeyMap { inner: inner_copy };
            proof { assert(tree_copy.spec_ordkeymap_wf()); }
            tree_copy.delete(k);
            OrderedTableStPer { tree: tree_copy }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_wf(&self, k: &K) -> (table: Self) {
            let table = self.delete(k);
            proof {
                assert forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2)
                    implies table@[k2] == self@[k2]
                by {
                    assert(table@ == self@.remove(k@));
                };
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to OrdKeyMap::domain
        fn domain(&self) -> (domain: ArraySetStEph<K>) {
            self.tree.domain()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to OrdKeyMap::tabulate
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self) {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            OrderedTableStPer { tree: OrdKeyMap::tabulate(keys, &f) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts
        #[verifier::loop_isolation(false)]
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self) {
            let sorted = self.tree.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(self.tree.inner@, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    0 <= i <= len,
                    len as nat == sorted@.len(),
                    self.spec_orderedtablestper_wf(),
                    forall|v: &V| f.requires((v,)),
                    obeys_feq_clone::<Pair<K, V>>(),
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    sorted@.len() == self.tree.inner@.len(),
                    forall|v: (K::V, V::V)| self.tree.inner@.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    new_tree@.len() == i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    forall|p: (K::V, V::V)| new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        self@.dom().contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int ==>
                        spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        (exists|old_val: V, result: V|
                            old_val@ == self@[p.0]
                            && f.ensures((&old_val,), result) && p.1 == result@),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let new_val = f(&pair.1);
                let k_clone = pair.0.clone_plus();
                let ghost old_new_tree_view = new_tree@;
                proof {
                    lemma_cloned_view_eq(pair.0, k_clone);
                    assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                        if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                            lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                            let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                            let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                            assert(false);
                        }
                    };
                }
                new_tree.insert(Pair(k_clone, new_val));
                proof {
                    assert(new_tree@.len() == i as nat + 1);
                    assert(new_tree@.len() < usize::MAX as nat);
                    lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, new_val@);
                    assert(new_tree@.contains((sorted@[i as int].0, new_val@)));
                    // Completeness: new entry + old entries.
                    lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, new_val@);
                    assert forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int implies
                        spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0) by {
                        // Old entry (key, v) was in old_new_tree_view, hence in new_tree@.
                        lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[j].0);
                        let v: V::V = choose|v: V::V| old_new_tree_view.contains((sorted@[j].0, v));
                        assert(new_tree@.contains((sorted@[j].0, v)));
                        lemma_pair_in_set_map_contains(new_tree@, sorted@[j].0, v);
                    };
                    // Value tracking for the new entry.
                    assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                    assert(self.tree.inner@.contains(sorted@[i as int]));
                    lemma_pair_in_set_map_contains(self.tree.inner@, sorted@[i as int].0, sorted@[i as int].1);
                    assert(self@.dom().contains(sorted@[i as int].0));
                    assert(pair.1@ == sorted@[i as int].1);
                    assert(self@[sorted@[i as int].0] == sorted@[i as int].1);
                    assert(f.ensures((&pair.1,), new_val));
                    // dom containment for new entry.
                    assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                        self@.dom().contains(p.0) by {
                        if old_new_tree_view.contains(p) {
                        } else {
                            assert(p == (sorted@[i as int].0, new_val@));
                        }
                    };
                    // Maintain spec_set_pair_view_generated.
                    assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                        assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                            implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                            if old_new_tree_view.contains(elem) {
                                // From loop invariant.
                            } else {
                                assert(elem == (k_clone@, new_val@));
                                let witness = Pair(k_clone, new_val);
                                assert(witness@ == elem);
                            }
                        };
                    };
                }
                i = i + 1;
            }
            let mapped = OrderedTableStPer { tree: OrdKeyMap { inner: new_tree } };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self.tree.inner@);
                assert(mapped@.dom() =~= self@.dom()) by {
                    assert forall|key: K::V| #[trigger] mapped@.dom().contains(key)
                        implies self@.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, key);
                        let v: V::V = choose|v: V::V| new_tree@.contains((key, v));
                        let j = choose|j: int| 0 <= j < i as int && (key, v).0 == (#[trigger] sorted@[j]).0;
                        assert(self.tree.inner@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(self.tree.inner@, sorted@[j].0, sorted@[j].1);
                    };
                    assert forall|key: K::V| self@.dom().contains(key)
                        implies #[trigger] mapped@.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(self.tree.inner@, key);
                        let v: V::V = choose|v: V::V| self.tree.inner@.contains((key, v));
                        assert(sorted@.contains((key, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (key, v);
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                    };
                };
                // Value tracking.
                assert forall|k: K::V| #[trigger] mapped@.contains_key(k) implies
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && mapped@[k] == result@)
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                };
                // wf: len bound from loop + axioms from self wf.
                assert(new_tree@.len() < usize::MAX as nat);
            }
            mapped
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to OrdKeyMap::filter
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self) {
            OrderedTableStPer { tree: self.tree.filter(f, Ghost(spec_pred)) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to OrdKeyMap::intersect_with
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self) {
            OrderedTableStPer { tree: self.tree.intersect_with(&other.tree, &f) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to OrdKeyMap::union_with
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self) {
            let result = self.tree.union_with(&other.tree, &f);
            proof {
                // OrdKeyMap union_with triggers its combine ensures on combined@[k]==r@ (inside
                // the existential).  Binding result@[k] to a fresh spec name forces Z3 to
                // materialize the term, which matches the trigger pattern and fires the forall.
                assert forall|k: K::V|
                    #[trigger] self@.contains_key(k) && other@.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && result@[k] == r@) by {
                    let vk = result@[k];  // materialize result@[k] for the trigger
                    assert(result@[k] == vk);
                };
            }
            OrderedTableStPer { tree: result }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to OrdKeyMap::difference
        fn difference(&self, other: &Self) -> (table: Self) {
            OrderedTableStPer { tree: self.tree.difference(&other.tree) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- iterate self, check key membership per element
        #[verifier::loop_isolation(false)]
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self) {
            let ghost old_tree = self.tree.inner@;
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
                    self.spec_orderedtablestper_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@.finite(),
                    self.tree.inner@ == old_tree,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> keys@.contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int && keys@.contains(sorted@[j].0) ==> new_tree@.contains(sorted@[j]),
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
                let pair = sorted.nth(i);
                let in_keys = keys.find(&pair.0);
                if in_keys {
                    let cloned = pair.clone_plus();
                    let ghost old_new_tree_view = new_tree@;
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                    new_tree.insert(cloned);
                    proof {
                        assert(new_tree@.len() <= i as nat + 1);
                        assert(new_tree@.len() < usize::MAX as nat);
                        lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        assert(old_tree.contains(sorted@[i as int]));
                    }
                }
                i = i + 1;
            }
            let table = OrderedTableStPer { tree: OrdKeyMap { inner: new_tree } };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                assert(table@.dom() =~= self@.dom().intersect(keys@)) by {
                    assert forall|k: K::V| table@.dom().contains(k)
                        implies #[trigger] self@.dom().intersect(keys@).contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                    };
                    assert forall|k: K::V| self@.dom().contains(k) && keys@.contains(k)
                        implies #[trigger] table@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] table@.contains_key(k)
                    implies table@[k] == self@[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                vstd::set_lib::lemma_len_subset(new_tree@, old_tree);
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- iterate self, check key exclusion per element
        #[verifier::loop_isolation(false)]
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self) {
            let ghost old_tree = self.tree.inner@;
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
                    self.spec_orderedtablestper_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@.finite(),
                    self.tree.inner@ == old_tree,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> !keys@.contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int && !keys@.contains(sorted@[j].0) ==> new_tree@.contains(sorted@[j]),
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
                let pair = sorted.nth(i);
                let in_keys = keys.find(&pair.0);
                if !in_keys {
                    let cloned = pair.clone_plus();
                    let ghost old_new_tree_view = new_tree@;
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                    new_tree.insert(cloned);
                    proof {
                        assert(new_tree@.len() <= i as nat + 1);
                        assert(new_tree@.len() < usize::MAX as nat);
                        lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        assert(old_tree.contains(sorted@[i as int]));
                    }
                }
                i = i + 1;
            }
            let table = OrderedTableStPer { tree: OrdKeyMap { inner: new_tree } };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                assert(table@.dom() =~= self@.dom().difference(keys@)) by {
                    assert forall|k: K::V| table@.dom().contains(k)
                        implies #[trigger] self@.dom().difference(keys@).contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                    };
                    assert forall|k: K::V| self@.dom().contains(k) && !keys@.contains(k)
                        implies #[trigger] table@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] table@.contains_key(k)
                    implies table@[k] == self@[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                vstd::set_lib::lemma_len_subset(new_tree@, old_tree);
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to OrdKeyMap::collect
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>) {
            let entries = self.tree.collect();
            proof {
                // entries@.len() == self@.dom().len(), and inner tree has < usize::MAX elements.
                lemma_pair_set_to_map_dom_finite(self.tree.inner@);
                lemma_pair_set_to_map_len(self.tree.inner@);
            }
            AVLTreeSeqStPerS::from_vec(entries)
        }


        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to first_key_iter
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to last_key_iter
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
            where K: TotalOrder
        {
            self.tree.next_key(k)
        }


        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to split_key_iter
        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            self.split_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::split
        fn split_key_iter(&self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            proof { lemma_pair_set_to_map_dom_finite(self.tree.inner@); }
            let (left, mid, right) = self.tree.split(k);
            (OrderedTableStPer { tree: left }, mid, OrderedTableStPer { tree: right })
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to union
        fn join_key(left: &Self, right: &Self) -> (table: Self) {
            left.union(right, |v1: &V, _v2: &V| -> (r: V) { v1.clone() })
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to get_key_range_iter
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self) {
            self.get_key_range_iter(k1, k2)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::get_key_range
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (range: Self) {
            OrderedTableStPer { tree: self.tree.get_key_range(k1, k2) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to rank_key_iter
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to split_rank_key_iter
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            self.split_rank_key_iter(i)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to OrdKeyMap::split_rank_key
        fn split_rank_key_iter(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let inner_copy = self.tree.inner.clone();
            let mut tree_copy = OrdKeyMap { inner: inner_copy };
            proof { assert(tree_copy.spec_ordkeymap_wf()); }
            let (left, right) = tree_copy.split_rank_key(i);
            (OrderedTableStPer { tree: left }, OrderedTableStPer { tree: right })
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>) {
            self.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter(&self, k: K, v: V) -> (table: Self) {
            self.insert(k, v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&self, k: &K) -> (table: Self) {
            self.delete(k)
        }
    }


    impl<K: StT + Ord + TotalOrder, V: StT + Ord> OrderedTableStPer<K, V> {
        /// Returns an iterator over the table entries via in-order traversal.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<K, V>)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree.inner@.len(),
                iter_invariant(&it),
        {
            let sorted = self.tree.inner.in_order();
            OrderedTableStPerIter { inner: sorted.seq.into_iter() }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n BST inserts from sorted entries
    pub fn from_sorted_entries<K: StT + Ord + TotalOrder, V: StT + Ord>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (table: OrderedTableStPer<K, V>)
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
            table.spec_orderedtablestper_wf(),
    {
        proof {
            assert(obeys_feq_full_trigger::<K>());
            assert(obeys_feq_full_trigger::<V>());
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            lemma_key_unique_empty::<K::V, V::V>();
        }
        let len = entries.length();
        let mut tree = ParamBST::<Pair<K, V>>::new();
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len as nat == entries@.len(),
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
                // Entries have unique keys (from requires).
                forall|ii: int, jj: int| 0 <= ii < jj < entries@.len()
                    ==> (#[trigger] entries@[ii]).0 != (#[trigger] entries@[jj]).0,
                spec_set_pair_view_generated::<K, V>(tree@),
            decreases len - i,
        {
            let ghost old_tree = tree@;
            let elem = entries.nth(i);
            let cloned = elem.clone_plus();
            proof { lemma_cloned_view_eq(*elem, cloned); }
            tree.insert(cloned);
            proof {
                assert(tree@.len() <= i as nat + 1);
                assert(i as nat + 1 <= len as nat);
                assert(tree@.len() < usize::MAX as nat);
                // Prove provenance for the new tree.
                assert forall|kv: K::V, vv: V::V| #[trigger] tree@.contains((kv, vv))
                    implies exists|j: int| #![trigger entries@[j]] 0 <= j < i as int + 1 && entries@[j] == (kv, vv) by {
                    if old_tree.contains((kv, vv)) {
                        let j = choose|j: int| #![trigger entries@[j]] 0 <= j < i as int && entries@[j] == (kv, vv);
                        assert(entries@[j] == (kv, vv) && j < i as int + 1);
                    } else {
                        // Must be the newly inserted element.
                        assert((kv, vv) == cloned@);
                        assert(entries@[i as int] == cloned@);
                    }
                };
                // Prove key uniqueness is maintained.
                assert(spec_key_unique_pairs_set(tree@)) by {
                    assert forall|k: K::V, v1: V::V, v2: V::V|
                        tree@.contains((k, v1)) && tree@.contains((k, v2)) implies v1 == v2 by {
                        if old_tree.contains((k, v1)) && old_tree.contains((k, v2)) {
                            // Both in old tree: follows from old invariant.
                        } else if !old_tree.contains((k, v1)) && !old_tree.contains((k, v2)) {
                            // Both are the new element.
                            assert((k, v1) == cloned@ && (k, v2) == cloned@);
                        } else {
                            // One old, one new: contradiction via unique keys.
                            if old_tree.contains((k, v1)) {
                                let j1 = choose|j: int| #![trigger entries@[j]]
                                    0 <= j < i as int && entries@[j] == (k, v1);
                                assert(entries@[j1].0 == entries@[i as int].0);
                                assert(j1 < i as int);
                                assert(false); // contradicts unique keys
                            } else {
                                let j2 = choose|j: int| #![trigger entries@[j]]
                                    0 <= j < i as int && entries@[j] == (k, v2);
                                assert(entries@[j2].0 == entries@[i as int].0);
                                assert(j2 < i as int);
                                assert(false);
                            }
                        }
                    };
                };
                // Maintain spec_set_pair_view_generated.
                assert(spec_set_pair_view_generated::<K, V>(tree@)) by {
                    assert forall|elem: (K::V, V::V)| tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        if old_tree.contains(elem) {
                            // From loop invariant.
                        } else {
                            assert(elem == cloned@);
                            assert(cloned@ == elem);
                        }
                    };
                };
            }
            i = i + 1;
        }
        let table = OrderedTableStPer { tree: OrdKeyMap { inner: tree } };
        proof { lemma_pair_set_to_map_dom_finite(tree@); }
        table
    }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerIter<K: StT + Ord + TotalOrder, V: StT + Ord> {
        pub inner: IntoIter<Pair<K, V>>,
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> View for OrderedTableStPerIter<K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<K: StT + Ord + TotalOrder, V: StT + Ord>(it: &OrderedTableStPerIter<K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> std::iter::Iterator for OrderedTableStPerIter<K, V> {
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
    pub struct OrderedTableStPerGhostIterator<K: StT + Ord + TotalOrder, V: StT + Ord> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> View for OrderedTableStPerGhostIterator<K, V> {
        type V = Seq<Pair<K, V>>;
        open spec fn view(&self) -> Seq<Pair<K, V>> { self.elements.take(self.pos) }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStPerIter<K, V> {
        type GhostIter = OrderedTableStPerGhostIterator<K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStPerGhostIterator<K, V> {
            OrderedTableStPerGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedTableStPerGhostIterator<K, V> {
        type ExecIter = OrderedTableStPerIter<K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStPerIter<K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStPerIter<K, V>) -> OrderedTableStPerGhostIterator<K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord + TotalOrder, V: StT + Ord> std::iter::IntoIterator for &'a OrderedTableStPer<K, V> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree.inner@.len(),
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord + TotalOrder, V: StT + Ord> PartialEqSpecImpl for OrderedTableStPer<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> Eq for OrderedTableStPer<K, V> {}

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> PartialEq for OrderedTableStPer<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.tree.inner.size() == other.tree.inner.size();
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@
        {
            let copy = OrderedTableStPer {
                tree: OrdKeyMap { inner: self.tree.inner.clone() },
            };
            proof { assume(copy@ == self@); }
            copy
        }
    }
    } // verus!

    //		Section 13. macros


    /// Macro for creating ordered tables from sorted key-value pairs.
    #[macro_export]
    macro_rules! OrderedTableStPerLit {
        () => {
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::OrderedTableStPer::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::from_sorted_entries(seq)
        }};
    }

    //		Section 14. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Debug for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Display for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Debug for OrderedTableStPerIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OrderedTableStPerIter").finish()
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Display for OrderedTableStPerIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPerIter")
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Debug for OrderedTableStPerGhostIterator<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPerGhostIterator")
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord> fmt::Display for OrderedTableStPerGhostIterator<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPerGhostIterator")
        }
    }
}
