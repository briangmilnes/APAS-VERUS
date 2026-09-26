// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Single-threaded persistent reducer-augmented ordered table implementation.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod AugOrderedTableStPer {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::OrderedTableStPerLit;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::clone_plus::clone_plus::clone_fn2;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_lemmas,
};

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(F)]
    pub struct AugOrderedTableStPer<K: StT + Ord + TotalOrder, V: StT + Ord, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        pub base_table: OrderedTableStPer<K, V>,
        pub reducer: F,
        pub identity: V,
        pub cached_reduction: V,
    }

    pub type AugOrderedTablePer<K, V, F> = AugOrderedTableStPer<K, V, F>;

    //		Section 5. view impls


    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F> View for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
    }

    //		Section 7. proof fns/broadcast groups


    proof fn lemma_aug_view<K: StT + Ord + TotalOrder, V: StT + Ord, F: Fn(&V, &V) -> V + Clone>(
        t: &AugOrderedTableStPer<K, V, F>,
    )
        ensures t@ =~= t.base_table@
    {}

    //		Section 8. traits


    /// Trait defining all augmented ordered table operations (ADT 43.3)
    /// Extends ordered table operations with efficient reduction
    pub trait AugOrderedTableStPerTrait<K: StT + Ord + TotalOrder, V: StT + Ord, F>: Sized + View<V = Map<K::V, V::V>>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        spec fn spec_augorderedtablestper_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- delegates to base table size
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn size(&self) -> (count: usize)
            requires self.spec_augorderedtablestper_wf(),
            ensures count == self@.dom().len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- constructs empty base table with reducer/identity
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn empty(reducer: F, identity: V) -> (empty: Self)
            requires
                forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
                obeys_feq_fulls::<K, V>(),
                obeys_feq_full::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp::<K>(),
                view_ord_consistent::<K>(),
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- constructs singleton base table with reducer/identity
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            requires
                obeys_feq_clone::<Pair<K, V>>(),
                forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
                obeys_feq_fulls::<K, V>(),
                obeys_feq_full::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp::<K>(),
                view_ord_consistent::<K>(),
            ensures tree.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to TableStPer which uses linear scan
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 42.5 O(lg n); callee defect only (base is OrderedTableStPer, not TableStPer: OrdKeyMap find with a deep-copy expose per level); does not match old analysis: O(n) vs O(n h)
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_augorderedtablestper_wf(), obeys_view_eq::<K>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- clones base table (persistent), inserts linearly, recalculates reduction O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 42.5 O(lg n); base insert: O(n) clone of the whole tree for persistence + OrdKeyMap insert O(n h) (callee defect), then calculate_reduction recomputes the whole cached reduction O(n h) instead of a per-node augmented update (Def 43.3); does not match old analysis: O(n) vs O(n h)
        fn insert(&self, k: K, v: V) -> (updated: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
                self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                updated@.dom() =~= self@.dom().insert(k@),
                updated.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- clones base table (persistent), deletes linearly, recalculates reduction O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 42.5 O(lg n); base delete: O(n) clone of the whole tree for persistence + OrdKeyMap delete O(n h) (callee defect), then calculate_reduction O(n h) instead of a per-node augmented update (Def 43.3); does not match old analysis: O(n) vs O(n h)
        fn delete(&self, k: &K) -> (updated: Self)
            requires
                self.spec_augorderedtablestper_wf(),

                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures updated.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- extracts keys from base table entries
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2), Span O(n^2) — does not match textbook: CS 42.5 Work O(|a|), Span O(lg |a|); callee defect only (OrdKeyMap domain, O(i) ArraySetStEph insert per key); does not match old analysis: O(n) vs O(n^2)
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires self.spec_augorderedtablestper_wf(), obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- applies f to each key, then recalculates reduction O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 h + Σ W(f)), Span O(n^2 h + Σ S(f)) — does not match textbook: APAS line Work O(n log n), Span O(n) (CS 42.5 has no tabulate row); callee defect (base tabulate → sequential OrdKeyMap tabulate, O(i h) per insert) plus calculate_reduction O(n h); does not match old analysis: O(n) vs O(n^2 h + Σ W(f))
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
                keys@.len() < usize::MAX,
                obeys_feq_fulls::<K, V>(),
                obeys_feq_full::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp::<K>(),
                view_ord_consistent::<K>(),
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_augorderedtablestper_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- maps all values linearly, then recalculates reduction O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 + Σ W(f)), Span O(n^2 + Σ S(f)) — does not match textbook: CS 42.5 Work O(Σ W(f)), Span O(lg n + max S(f)); base map: in_order O(n h), then sequential sorted inserts into a fresh never-rebalanced ParamBST (O(i) each), plus calculate_reduction O(n h); does not match old analysis: O(n) vs O(n^2 + Σ W(f))
        fn map<G: Fn(&V) -> V>(&self, f: G) -> (mapped: Self)
            requires
                self.spec_augorderedtablestper_wf(),

                forall|v: &V| f.requires((v,)),
            ensures
                mapped@.dom() == self@.dom(),
                forall|k: K::V| #[trigger] mapped@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && mapped@[k] == result@),
                mapped.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- filters base table linearly, then recalculates reduction O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h^2 + Σ W(f)), Span O(n h^2 + Σ S(f)) — does not match textbook: CS 42.5 Work O(Σ W(f)), Span O(lg n + max S(f)); callee defect (base filter → OrdKeyMap filter, sequential ParamBST filter) plus calculate_reduction O(n h); does not match old analysis: O(n) vs O(n h^2 + Σ W(f))
        fn filter<G: Fn(&K, &V) -> bool>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
            requires
                self.spec_augorderedtablestper_wf(),

                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                filtered@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] filtered@.contains_key(k) ==> filtered@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] filtered@.dom().contains(k),
                filtered.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to base table intersection (linear scan), then recalculates reduction
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n m h + n h + Σ W(f)), Span O(n m h + n h + Σ S(f)) — does not match textbook: n = |self|, m = |other|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); callee defect (OrdKeyMap intersect_with) plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n m h + Σ W(f))
        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (common: Self)
            requires
                self.spec_augorderedtablestper_wf(),

                other.spec_augorderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
            ensures
                common@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] common@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && common@[k] == r@),
                common.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) -- delegates to base table union (linear merge), then recalculates reduction
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O((n + m)^2 h + Σ W(f)), Span O((n + m)^2 h + Σ S(f)) — does not match textbook: n = |self|, m = |other|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); callee defect (OrdKeyMap union_with) plus calculate_reduction O((n + m) h); does not match old analysis: O(n + m) vs O((n + m)^2 h)
        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (combined: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                other.spec_augorderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                self@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                combined@.dom() =~= self@.dom().union(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) && !other@.contains_key(k)
                    ==> combined@[k] == self@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !self@.contains_key(k)
                    ==> combined@[k] == other@[k],
                forall|k: K::V| #[trigger] self@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && combined@[k] == r@),
                combined.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to base table difference (linear scan), then recalculates reduction
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n m h + n^2), Span O(n m h + n^2) — does not match textbook: n = |self|, m = |other|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); callee defect (OrdKeyMap difference) plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n m h + n^2)
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                other.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                remaining@.dom() =~= self@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] remaining@.contains_key(k) ==> remaining@[k] == self@[k],
                remaining.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to base table restrict (linear scan), then recalculates reduction
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 + n m), Span O(n^2 + n m) — does not match textbook: n = |self|, m = |keys|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); base restrict: in_order, O(m) ArraySet membership per entry, sorted inserts into a fresh path (O(i) each), plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n^2 + n m)
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
            requires
                self.spec_augorderedtablestper_wf(),

            ensures
                restricted@.dom() =~= self@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] restricted@.contains_key(k) ==> restricted@[k] == self@[k],
                restricted.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to base table subtract (linear scan), then recalculates reduction
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 + n m), Span O(n^2 + n m) — does not match textbook: n = |self|, m = |keys|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); base subtract: in_order, O(m) ArraySet membership per entry, sorted inserts into a fresh path (O(i) each), plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n^2 + n m)
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
            requires
                self.spec_augorderedtablestper_wf(),

            ensures
                subtracted@.dom() =~= self@.dom().difference(keys@),
                forall|k: K::V| #[trigger] subtracted@.contains_key(k) ==> subtracted@[k] == self@[k],
                subtracted.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- collects base table entries into AVLTreeSeqStPer
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: APAS line Work O(n), Span O(log n); callee defect only (OrdKeyMap collect: ParamBST in_order with a deep-copy expose per node, then from_vec O(n)); does not match old analysis: O(n) vs O(n h)
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            requires self.spec_augorderedtablestper_wf(),
            ensures collected.spec_avltreeseqstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, returns first key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (base first_key → OrdKeyMap first_key → ParamBST min_key, no collect or sort); does not match old analysis: O(n log n) vs O(n h)
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablestper_wf(),
            ensures
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, returns last key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (base last_key → OrdKeyMap last_key → ParamBST max_key, no collect or sort); does not match old analysis: O(n log n) vs O(n h)
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablestper_wf(),
            ensures
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, finds predecessor
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (base previous_key → OrdKeyMap prev_key, no collect or sort); does not match old analysis: O(n log n) vs O(n h)
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablestper_wf(),
            ensures
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, finds successor
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (base next_key → OrdKeyMap next_key, no collect or sort); does not match old analysis: O(n log n) vs O(n h)
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablestper_wf(),
            ensures
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, partitions into two tables + recalculates reductions
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); base split_key → OrdKeyMap split (callee defect), then two calculate_reduction folds O(n h) instead of reading augmented subtree values; does not match old analysis: O(n log n) vs O(n h)
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            requires
                self.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
                parts.0.spec_augorderedtablestper_wf(),
                parts.2.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) -- delegates to base table union (linear merge), then recalculates reduction
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O((n + m)^2 h), Span O((n + m)^2 h) — does not match textbook: CS 43.2 join O(lg(n + m)); base join_key calls union (OrdKeyMap union_with) instead of a BST join; the reduction is not recalculated, the two cached values are combined with one reducer call, O(1); does not match old analysis: O(n + m) vs O((n + m)^2 h)
        fn join_key(left: &Self, right: &Self) -> (joined: Self)
            requires
                left.spec_augorderedtablestper_wf(),
                right.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                left@.dom().len() + right@.dom().len() < usize::MAX,
            ensures
                joined@.dom() =~= left@.dom().union(right@.dom()),
                joined.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, filters range, builds new table + recalculates reduction
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); base get_key_range → OrdKeyMap get_key_range (callee defect), then calculate_reduction O(n h); does not match old analysis: O(n log n) vs O(n h)
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires
                self.spec_augorderedtablestper_wf(),
            ensures
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, counts predecessors
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (base rank_key → OrdKeyMap rank_key, no collect or sort); does not match old analysis: O(n log n) vs O(n h)
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, selects by index
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (base select_key → OrdKeyMap select_key, no collect or sort); does not match old analysis: O(n log n) vs O(n h)
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            requires
                self.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collects entries, sorts, splits at rank into two tables + recalculates reductions
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); base split_rank_key: O(n) clone + OrdKeyMap split_rank_key (in_order walk + split, callee defect), then two calculate_reduction folds O(n h); does not match old analysis: O(n log n) vs O(n h)
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                self.spec_augorderedtablestper_wf(),
            ensures
                split.0@.dom().subset_of(self@.dom()),
                split.1@.dom().subset_of(self@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_augorderedtablestper_wf(),
                split.1.spec_augorderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 Def 43.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn reduce_val(&self) -> (reduced: V)
            requires self.spec_augorderedtablestper_wf(),;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n) -- split + cached reduction
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- get_key_range O(n log n) + calculate_reduction O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: Example 43.2 reduceVal(getRange) O(lg n); get_key_range copies the range (O(n h), callee defect) and refolds it with calculate_reduction O(n h) instead of combining O(lg n) augmented subtree values; does not match old analysis: O(n log n) vs O(n h)
        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            requires
                self.spec_augorderedtablestper_wf();
    }

    //		Section 9. impls


    // 7. free functions (calculate_reduction)

    /// Fold all values in `base` through `reducer`, returning `identity` for empty tables.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- collect O(n) + linear fold
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h + n W(r)), Span O(n h + n S(r)) — no textbook cost; does not match old analysis: O(n) vs O(n h); base collect O(n h) (ParamBST in_order with a deep-copy expose per node, callee defect), then a sequential fold with n AVLTreeSeqStPer nth calls, O(lg n) each; r = reducer
    pub fn calculate_reduction<K: StT + Ord + TotalOrder, V: StT + Ord, F>(
        base: &OrderedTableStPer<K, V>,
        reducer: &F,
        identity: &V,
    ) -> (reduced: V)
    where
        F: Fn(&V, &V) -> V + Clone,
        requires base.spec_orderedtablestper_wf(), forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
    {
        let pairs = base.collect();
        let sz = pairs.length();
        if sz == 0 {
            return identity.clone();
        }
        let mut reduced = pairs.nth(0).1.clone();
        let mut i: usize = 1;
        while i < sz
            invariant
                1 <= i <= pairs@.len(),
                sz as nat == pairs@.len(),
                pairs.spec_avltreeseqstper_wf(),
                forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
            decreases pairs@.len() - i,
        {
            let pair = pairs.nth(i);
            reduced = reducer(&reduced, &pair.1);
            i += 1;
        }
        reduced
    }


    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F> AugOrderedTableStPerTrait<K, V, F> for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        open spec fn spec_augorderedtablestper_wf(&self) -> bool {
            self.base_table.spec_orderedtablestper_wf()
            && forall|v1: &V, v2: &V| #[trigger] self.reducer.requires((v1, v2))
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len()
        {
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block (speed hint)
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_augorderedtablestper_wf()
        {
            let base = OrderedTableStPer::empty();
            let r = Self {
                base_table: base,
                cached_reduction: identity.clone(),
                reducer,
                identity,
            };
            // Veracity: NEEDED proof block (speed hint)
            // Veracity: NEEDED proof block
            proof { lemma_aug_view(&r); }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            ensures tree.spec_augorderedtablestper_wf()
        {
            let base = OrderedTableStPer::singleton(k, v.clone());
            let r = Self {
                base_table: base,
                cached_reduction: v,
                reducer,
                identity,
            // Veracity: NEEDED proof block
            };
            // Veracity: NEEDED proof block
            proof { lemma_aug_view(&r); }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to base_table find
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 42.5 O(lg n); callee defect only (OrdKeyMap find, deep-copy expose per level); does not match old analysis: O(lg n) vs O(n h)
        // Veracity: NEEDED proof block
        fn find(&self, k: &K) -> (found: Option<V>)
        {
            // Veracity: NEEDED proof block
            proof { lemma_aug_view(self); }
            self.base_table.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- base insert O(n) + recalculate O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 42.5 O(lg n); base insert: O(n) clone of the whole tree for persistence + OrdKeyMap insert O(n h) (callee defect), then calculate_reduction O(n h) instead of a per-node augmented update (Def 43.3); does not match old analysis: O(n) vs O(n h)
        fn insert(&self, k: K, v: V) -> (updated: Self)
        {
            let new_base = self.base_table.insert(k, v);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                // Veracity: NEEDED proof block
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            // Veracity: NEEDED proof block
            proof {
                lemma_aug_view(&r);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- base delete O(n) + recalculate O(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 42.5 O(lg n); base delete: O(n) clone of the whole tree for persistence + OrdKeyMap delete O(n h) (callee defect), then calculate_reduction O(n h) instead of a per-node augmented update (Def 43.3); does not match old analysis: O(n) vs O(n h)
        fn delete(&self, k: &K) -> (updated: Self)
            ensures updated.spec_augorderedtablestper_wf()
        {
            let new_base = self.base_table.delete(k);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                // Veracity: NEEDED proof block
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            // Veracity: NEEDED proof block
            proof {
                lemma_aug_view(&r);
            }
            r
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base_table domain
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2), Span O(n^2) — does not match textbook: CS 42.5 Work O(|a|), Span O(lg |a|); callee defect only (OrdKeyMap domain, O(i) ArraySetStEph insert per key); does not match old analysis: O(n) vs O(n^2)
        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            // Veracity: NEEDED proof block
            proof { lemma_aug_view(self); }
            self.base_table.domain()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- base tabulate + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 h + Σ W(f)), Span O(n^2 h + Σ S(f)) — does not match textbook: APAS line Work O(n log n), Span O(n) (CS 42.5 has no tabulate row); callee defect (base tabulate → sequential OrdKeyMap tabulate) plus calculate_reduction O(n h); does not match old analysis: O(n log n) vs O(n^2 h + Σ W(f))
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
        {
            let base_table = OrderedTableStPer::tabulate(f, keys);
            let cached_reduction = calculate_reduction(&base_table, &reducer, &identity);
// Veracity: UNNEEDED proof block 
            let r = Self {
                base_table,
                cached_reduction,
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- base map + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 + Σ W(f)), Span O(n^2 + Σ S(f)) — does not match textbook: CS 42.5 Work O(Σ W(f)), Span O(lg n + max S(f)); base map rebuilds by sorted inserts into a fresh never-rebalanced ParamBST (O(i) each), plus calculate_reduction O(n h); does not match old analysis: O(n log n) vs O(n^2 + Σ W(f))
        fn map<G: Fn(&V) -> V>(&self, f: G) -> (mapped: Self)
        {
            let new_base = self.base_table.map(f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);
// Veracity: NEEDED proof block

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- base filter + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h^2 + Σ W(f)), Span O(n h^2 + Σ S(f)) — does not match textbook: CS 42.5 Work O(Σ W(f)), Span O(lg n + max S(f)); callee defect (base filter → OrdKeyMap filter) plus calculate_reduction O(n h); does not match old analysis: O(n log n) vs O(n h^2 + Σ W(f))
        fn filter<G: Fn(&K, &V) -> bool>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
        {
            let new_base = self.base_table.filter(f, Ghost(spec_pred));
            // Veracity: NEEDED proof block
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- base intersection + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n m h + n h + Σ W(f)), Span O(n m h + n h + Σ S(f)) — does not match textbook: n = |self|, m = |other|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); callee defect (OrdKeyMap intersect_with) plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n m h + Σ W(f))
        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (common: Self)
        {
            // Veracity: NEEDED proof block
            let new_base = self.base_table.intersection(&other.base_table, f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- base union + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O((n + m)^2 h + Σ W(f)), Span O((n + m)^2 h + Σ S(f)) — does not match textbook: n = |self|, m = |other|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); callee defect (OrdKeyMap union_with) plus calculate_reduction O((n + m) h); does not match old analysis: O(n * m) vs O((n + m)^2 h)
        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (combined: Self)
        // Veracity: NEEDED proof block
        {
            let new_base = self.base_table.union(&other.base_table, f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- base difference + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n m h + n^2), Span O(n m h + n^2) — does not match textbook: n = |self|, m = |other|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); callee defect (OrdKeyMap difference) plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n m h + n^2)
        // Veracity: NEEDED proof block
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let new_base = self.base_table.difference(&other.base_table);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            }
            r
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- base restrict + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 + n m), Span O(n^2 + n m) — does not match textbook: n = |self|, m = |keys|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); base restrict rebuilds a path by sorted inserts after O(m) membership tests, plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n^2 + n m)
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
        {
            let new_base = self.base_table.restrict(keys);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            }
            r
        }
// Veracity: NEEDED proof block

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- base subtract + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n^2 + n m), Span O(n^2 + n m) — does not match textbook: n = |self|, m = |keys|; CS 42.5 Work O(m lg(1 + n/m)), Span O(lg(n + m)); base subtract rebuilds a path by sorted inserts after O(m) membership tests, plus calculate_reduction O(n h); does not match old analysis: O(n * m) vs O(n^2 + n m)
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
        {
            let new_base = self.base_table.subtract(keys);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                // Veracity: NEEDED proof block
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base collect
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: APAS line Work O(n), Span O(log n); callee defect only (OrdKeyMap collect: ParamBST in_order with a deep-copy expose per node, then from_vec O(n)); does not match old analysis: O(n) vs O(n h)
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            // Veracity: NEEDED proof block (speed hint)
            ensures collected.spec_avltreeseqstper_wf()
        {
            proof { lemma_aug_view(self); }
            self.base_table.collect()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base first_key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (OrdKeyMap first_key → ParamBST min_key); does not match old analysis: O(n) vs O(n h)
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().len() == 0 <==> first matches None,
                // Veracity: NEEDED proof block (speed hint)
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t),
        {
            proof { lemma_aug_view(self); }
            self.base_table.first_key()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base last_key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (OrdKeyMap last_key → ParamBST max_key); does not match old analysis: O(n) vs O(n h)
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                // Veracity: NEEDED proof block (speed hint)
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v),
        {
            proof { lemma_aug_view(self); }
            self.base_table.last_key()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base previous_key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (OrdKeyMap prev_key, deep-copy expose per level); does not match old analysis: O(n) vs O(n h)
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            proof { lemma_aug_view(self); }
            self.base_table.previous_key(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base next_key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (OrdKeyMap next_key, deep-copy expose per level); does not match old analysis: O(n) vs O(n h)
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            proof { lemma_aug_view(self); }
            self.base_table.next_key(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- base split_key + two recalculations
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); base split_key → OrdKeyMap split (callee defect), then two calculate_reduction folds O(n h) instead of reading augmented subtree values; does not match old analysis: O(n log n) vs O(n h)
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            ensures
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
                parts.0.spec_augorderedtablestper_wf(),
                parts.2.spec_augorderedtablestper_wf(),
        {
            let (left_base, middle, right_base) = self.base_table.split_key(k);
// Veracity: NEEDED proof block

            let left_reduction = calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
            (left, middle, right)
        }
// Veracity: NEEDED proof block

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- base join_key (union) + O(1) reduce
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O((n + m)^2 h), Span O((n + m)^2 h) — does not match textbook: CS 43.2 join O(lg(n + m)); base join_key calls union (OrdKeyMap union_with) instead of a BST join; the cached reductions combine in O(1); does not match old analysis: O(n * m) vs O((n + m)^2 h)
        fn join_key(left: &Self, right: &Self) -> (joined: Self)
        {
            let new_base = OrderedTableStPer::join_key(&left.base_table, &right.base_table);
            let new_reduction = if left.base_table.size() == 0 {
                right.cached_reduction.clone()
            } else if right.base_table.size() == 0 {
                left.cached_reduction.clone()
            } else {
                (left.reducer)(&left.cached_reduction, &right.cached_reduction)
            };

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&left.reducer),
                identity: left.identity.clone(),
            };
            proof {
                lemma_aug_view(&r);
            // Veracity: NEEDED proof block
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- base get_key_range + recalculate
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); base get_key_range → OrdKeyMap get_key_range (callee defect), then calculate_reduction O(n h); does not match old analysis: O(n log n) vs O(n h)
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_augorderedtablestper_wf(),
        {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            // Veracity: NEEDED proof block
            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(self);
                lemma_aug_view(&r);
            }
            r
        }
// Veracity: NEEDED proof block (speed hint)

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base rank_key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (OrdKeyMap rank_key, deep-copy expose per level); does not match old analysis: O(n) vs O(n h)
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len(),
        {
            proof { lemma_aug_view(self); }
            self.base_table.rank_key(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to base select_key
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); callee defect only (OrdKeyMap select_key, deep-copy expose per level); does not match old analysis: O(n) vs O(n h)
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int,
        {
            proof { lemma_aug_view(self); }
            self.base_table.select_key(i)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- base split_rank_key + two recalculations
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: CS 43.2 O(lg n); base split_rank_key: O(n) clone + OrdKeyMap split_rank_key (callee defect), then two calculate_reduction folds O(n h); does not match old analysis: O(n log n) vs O(n h)
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            ensures
                split.0@.dom().subset_of(self@.dom()),
                split.1@.dom().subset_of(self@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                // Veracity: NEEDED proof block
                forall|key| #[trigger] self@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_augorderedtablestper_wf(),
                split.1.spec_augorderedtablestper_wf(),
        {
            let (left_base, right_base) = self.base_table.split_rank_key(i);

            let left_reduction = calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                // Veracity: NEEDED proof block
                cached_reduction: left_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
// Veracity: NEEDED proof block

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
            (left, right)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- returns cached reduction clone
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — matches textbook
        fn reduce_val(&self) -> (reduced: V)
        {
            proof {
                lemma_aug_view(self);
                // wf chain: aug_wf → orderedtable_wf → bst_wf.
            }
            self.cached_reduction.clone()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- get_key_range + cached clone
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h), Span O(n h) — does not match textbook: Example 43.2 reduceVal(getRange) O(lg n); get_key_range copies the range (O(n h), callee defect) and refolds it with calculate_reduction O(n h) instead of combining O(lg n) augmented subtree values; does not match old analysis: O(n log n) vs O(n h)
        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
        {
            proof {
                lemma_aug_view(self);
                // wf chain: aug_wf → orderedtable_wf → bst_wf.
            }
            let range_table = self.get_key_range(k1, k2);
            range_table.cached_reduction.clone()
        }
    }


    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F: Fn(&V, &V) -> V + Clone> AugOrderedTableStPer<K, V, F> {
        /// Returns an iterator over the table entries via the base ordered table.
        pub fn iter(&self) -> (it: std::vec::IntoIter<Pair<K, V>>)
            requires self.spec_augorderedtablestper_wf(),
            ensures
                vstd::std_specs::vec::into_iter_elts(it) == IteratorSpec::remaining(&it),
                IteratorSpec::decrease(&it) is Some,
                vstd::std_specs::vec::into_iter_elts(it).len() == self.base_table.tree.inner@.len(),
        {
            self.base_table.iter()
        }
    }

    //		Section 10. iterators


    // r213 form C: this `IntoIterator` impl required `requires self.spec_augorderedtablestper_wf()`,
    // which verus 0.2026.09.13 rejects on an external trait's impl and no exec check
    // can establish; use `iter()`, which keeps the requires
    // (src/experiments/intoiter_form_c_no_impl.rs).
    /*
    impl<'a, K: StT + Ord + TotalOrder, V: StT + Ord, F: Fn(&V, &V) -> V + Clone> std::iter::IntoIterator for &'a AugOrderedTableStPer<K, V, F> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            // Veracity: NEEDED proof block
            requires self.spec_augorderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.base_table.tree.inner@.len(),
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }
    */

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F: Fn(&V, &V) -> V + Clone> PartialEqSpecImpl for AugOrderedTableStPer<K, V, F> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F: Fn(&V, &V) -> V + Clone> Eq for AugOrderedTableStPer<K, V, F> {}

    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F: Fn(&V, &V) -> V + Clone> PartialEq for AugOrderedTableStPer<K, V, F> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.base_table == other.base_table;
            proof { lemma_aug_view(self); lemma_aug_view(other); }
            equal
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F> Clone for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let r = Self {
                base_table: self.base_table.clone(),
                cached_reduction: self.cached_reduction.clone(),
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            r
        }
    }

    } // verus!

    //		Section 13. macros


    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableStPerLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::AugOrderedTableStPerTrait::empty($reducer, $identity);
            $(
                table = $crate::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::AugOrderedTableStPerTrait::insert(&table, $k, $v);
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::AugOrderedTableStPerTrait::empty($reducer, $identity)
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F> Display for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "AugOrderedTableStPer(size: {}, reduction: {})",
                self.size(),
                self.cached_reduction
            )
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT + Ord, F> Debug for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableStPer")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }
}
