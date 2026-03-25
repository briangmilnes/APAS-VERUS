//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral reducer-augmented ordered table implementation.
//!
//! reduce_range_parallel() uses verified ParaPair! with named closures for range reductions.

pub mod AugOrderedTableMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 7. free functions (calculate_reduction, recalculate_reduction)
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 13. derive impls outside verus!
    // 14. macros

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::Arc;

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::spec_pair_key_determines_order;
    use crate::Concurrency::Concurrency::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::vstdplus::clone_plus::clone_plus::clone_fn2;
    use crate::{OrderedTableMtEphLit, ParaPair};
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_fulls;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(F)]
    pub struct AugOrderedTableMtEph<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> {
        pub base_table: OrderedTableMtEph<K, V>,
        pub reducer: F,
        pub identity: V,
        pub cached_reduction: V,
    }

    pub type AugOrderedTableMt<K, V, F> = AugOrderedTableMtEph<K, V, F>;

    // 5. view impls

    impl<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> View for AugOrderedTableMtEph<K, V, F> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
    }

    // 7. free functions (calculate_reduction, recalculate_reduction)

    pub fn recalculate_reduction<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>>(
        table: &AugOrderedTableMtEph<K, V, F>,
    ) -> (reduced: V)
    requires table.spec_augorderedtablemteph_wf()
    ensures table@.dom().finite()
    {
        let reduced = calculate_reduction(&table.base_table, &table.reducer, &table.identity);
        proof { lemma_aug_view(table); }
        reduced
    }

    pub fn calculate_reduction<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>>(
        base: &OrderedTableMtEph<K, V>,
        reducer: &F,
        identity: &V,
    ) -> (reduced: V)
    requires forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2))
    ensures base@.dom().finite()
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
            i = i + 1;
        }
        reduced
    }

    // 7b. proof fns

    proof fn lemma_aug_view<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>>(
        t: &AugOrderedTableMtEph<K, V, F>,
    )
        ensures t@ =~= t.base_table@
    {}

    // 8. traits

    /// Trait defining all augmented ordered table operations (ADT 43.3) with multi-threaded ephemeral semantics
    /// Extends ordered table operations with efficient reduction and thread-safe operations
    pub trait AugOrderedTableMtEphTrait<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_augorderedtablemteph_wf(&self) -> bool;

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- delegates to base table size
        fn size(&self) -> (count: usize)
            requires self.spec_augorderedtablemteph_wf()
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- constructs empty base table with reducer/identity
        fn empty(reducer: F, identity: V) -> (empty: Self)
            requires forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2))
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_augorderedtablemteph_wf();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- constructs singleton base table with reducer/identity
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            requires
                forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures tree.spec_augorderedtablemteph_wf();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- delegates to TableMtEph which uses linear scan
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_augorderedtablemteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@),
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- delegates to TableMtEph which uses linear scan
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_augorderedtablemteph_wf(), obeys_view_eq::<K>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@),
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- delegates to base table is_empty
        fn is_empty(&self) -> (is_empty: B)
            requires self.spec_augorderedtablemteph_wf()
            ensures is_empty == self@.dom().is_empty(), self@.dom().finite();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- mutates base table (linear scan), then recalculates reduction O(n)
        fn insert<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: G)
            requires
                old(self).spec_augorderedtablemteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_clone::<K>(),
            ensures self@.dom().finite();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- mutates base table (linear scan), then recalculates reduction O(n)
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_augorderedtablemteph_wf(),
                obeys_view_eq::<K>(),
            ensures self@.dom().finite();
        /// - APAS: Work O(n), Span O(n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- extracts keys from base table entries
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            ensures self@.dom().finite();
        /// - APAS: Work O(n log n), Span O(n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- applies f to each key, then recalculates reduction O(n)
        fn tabulate<G: Fn(&K) -> V + Send + Sync + 'static>(
            f: G,
            keys: &ArraySetStEph<K>,
            reducer: F,
            identity: V,
        ) -> (domain: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
                obeys_feq_full::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
                obeys_feq_fulls::<K, V>(),
            ensures domain@.dom().finite();
        /// - APAS: Work O(n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- maps all values linearly, then recalculates reduction O(n)
        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (mapped: Self)
            requires
                self.spec_augorderedtablemteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
            ensures mapped@.dom().finite();
        /// - APAS: Work O(n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- filters base table linearly, then recalculates reduction O(n)
        fn filter<G: Fn(&K, &V) -> B + Send + Sync + 'static>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
            requires
                self.spec_augorderedtablemteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures filtered@.dom().finite();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table intersection (linear scan), then recalculates reduction
        fn intersection<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            requires
                old(self).spec_augorderedtablemteph_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures self@.dom().finite();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) -- delegates to base table union (linear merge), then recalculates reduction
        fn union<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            requires
                old(self).spec_augorderedtablemteph_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures self@.dom().finite();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table difference (linear scan), then recalculates reduction
        fn difference(&mut self, other: &Self)
            requires old(self).spec_augorderedtablemteph_wf()
            ensures self@.dom().finite();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table restrict (linear scan), then recalculates reduction
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_augorderedtablemteph_wf()
            ensures self@.dom().finite();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table subtract (linear scan), then recalculates reduction
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_augorderedtablemteph_wf()
            ensures self@.dom().finite();
        /// - APAS: Work O(n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- iterates all entries applying f sequentially
        fn reduce<R: StTInMtT + 'static, G: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: G) -> (reduced: R)
            requires forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();
        /// - APAS: Work O(n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- collects base table entries into AVLTreeSeqStPer
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- external_body, delegates to base table which collects+sorts
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- external_body, delegates to base table which collects+sorts
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- external_body, delegates to base table which collects+sorts
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- external_body, delegates to base table which collects+sorts
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- delegates to base table split + recalculates reductions
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized,
            requires old(self).spec_augorderedtablemteph_wf(), obeys_view_eq::<K>(),
            ensures self@.dom().finite();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) -- delegates to base table join + recalculates reduction
        fn join_key(&mut self, other: Self)
            requires old(self).spec_augorderedtablemteph_wf()
            ensures self@.dom().finite();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- delegates to base table get_key_range + recalculates reduction
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires self.spec_augorderedtablemteph_wf()
            ensures range.spec_augorderedtablemteph_wf();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- external_body, delegates to base table which collects+sorts+counts
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires self.spec_augorderedtablemteph_wf(), obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- external_body, delegates to base table which collects+sorts+selects
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablemteph_wf(), obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- delegates to base table split_rank + recalculates reductions
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized,
            requires old(self).spec_augorderedtablemteph_wf()
            ensures self@.dom().finite();
        /// - APAS: Work O(1), Span O(1) -- augmented tables cache the reduction
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- returns clone of cached_reduction
        fn reduce_val(&self) -> (reduced: V)
            requires self@.dom().finite()
            ensures self@.dom().finite();
        /// - APAS: Work O(log n), Span O(log n) -- split + cached reduction
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- get_key_range O(n log n) + reduce_val O(1)
        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            requires self.spec_augorderedtablemteph_wf()
            ensures self@.dom().finite();
        /// - APAS: Work O(log n), Span O(log n) -- split + cached reduction
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- parallel via ParaPair! with named closures
        fn reduce_range_parallel(&self, k1: &K, k2: &K) -> (reduced: V)
            where K: TotalOrder
            requires
                self.spec_augorderedtablemteph_wf(),
                obeys_view_eq::<K>(),
            ensures self@.dom().finite();

        fn iter<'a>(&'a self) -> (it: OrderedTableMtEphIter<'a, K, V>)
            requires self.spec_augorderedtablemteph_wf(),
            ensures
                it@.0 == 0,
                iter_invariant(&it);
    }

    // 9. impls

    impl<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> AugOrderedTableMtEphTrait<K, V, F> for AugOrderedTableMtEph<K, V, F> {
        open spec fn spec_augorderedtablemteph_wf(&self) -> bool {
            self@.dom().finite() && self.base_table.spec_orderedtablemteph_wf()
            && forall|v1: &V, v2: &V| #[trigger] self.reducer.requires((v1, v2))
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_augorderedtablemteph_wf()
        {
                      assert(obeys_feq_full_trigger::<K>());
           assert(obeys_feq_full_trigger::<V>());
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let base = OrderedTableMtEph::empty();
            let r = Self {
                base_table: base,
                cached_reduction: identity.clone(),
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            ensures tree.spec_augorderedtablemteph_wf()
        {
                      assert(obeys_feq_full_trigger::<K>());
           assert(obeys_feq_full_trigger::<V>());
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let base = OrderedTableMtEph::singleton(k, v.clone());
            let r = Self {
                base_table: base,
                cached_reduction: v,
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn find(&self, k: &K) -> (found: Option<V>) {
            self.base_table.find(k)
        }

        fn lookup(&self, k: &K) -> (value: Option<V>) {
            self.base_table.lookup(k)
        }

        fn is_empty(&self) -> (is_empty: B)
            ensures is_empty == self@.dom().is_empty(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.is_empty()
        }

        fn insert<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: G)
            ensures self@.dom().finite()
        {
            self.base_table.insert(k, v, combine);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            ensures self@.dom().finite()
        {
            let updated = self.base_table.delete(k);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
            updated
        }

        fn domain(&self) -> (domain: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.domain()
        }

        fn tabulate<G: Fn(&K) -> V + Send + Sync + 'static>(
            f: G,
            keys: &ArraySetStEph<K>,
            reducer: F,
            identity: V,
        ) -> (domain: Self)
            ensures domain@.dom().finite()
        {
            let base_table = OrderedTableMtEph::tabulate(f, keys);
            let cached_reduction = calculate_reduction(&base_table, &reducer, &identity);

            let r = Self {
                base_table,
                cached_reduction,
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (mapped: Self)
            ensures mapped@.dom().finite()
        {
            let new_base = self.base_table.map(f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn filter<G: Fn(&K, &V) -> B + Send + Sync + 'static>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
            ensures filtered@.dom().finite()
        {
            let new_base = self.base_table.filter(f, Ghost(spec_pred));
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn intersection<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn union<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.union(&other.base_table, f);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            self.base_table.difference(&other.base_table);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.restrict(keys);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.subtract(keys);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn reduce<R: StTInMtT + 'static, G: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: G) -> (reduced: R)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.reduce(init, f)
        }

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf()
        {
            proof { lemma_aug_view(self); }
            self.base_table.collect()
        }

        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            proof { lemma_aug_view(self); }
            self.base_table.first_key()
        }

        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            proof { lemma_aug_view(self); }
            self.base_table.last_key()
        }

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            proof { lemma_aug_view(self); }
            self.base_table.previous_key(k)
        }

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            proof { lemma_aug_view(self); }
            self.base_table.next_key(k)
        }

        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            ensures self@.dom().finite()
        {
            let (left_base, found_value, right_base) = self.base_table.split_key(k);

            let left_reduction = calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            proof { lemma_aug_view(self); }
            (left, found_value, right)
        }

        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite()
        {
            self.base_table.join_key(other.base_table);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range.spec_augorderedtablemteph_wf()
        {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: clone_fn2(&self.reducer),
                identity: self.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            proof { lemma_aug_view(self); }
            self.base_table.rank_key(k)
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            proof { lemma_aug_view(self); }
            self.base_table.select_key(i)
        }

        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            ensures self@.dom().finite()
        {
            let (left_base, right_base) = self.base_table.split_rank_key(i);

            let left_reduction = calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            proof { lemma_aug_view(self); }
            (left, right)
        }

        fn reduce_val(&self) -> (reduced: V)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.cached_reduction.clone()
        }

        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            let range_table = self.get_key_range(k1, k2);
            range_table.reduce_val()
        }

        fn reduce_range_parallel(&self, k1: &K, k2: &K) -> (reduced: V)
            where K: TotalOrder
        {
            let range_table = self.get_key_range(k1, k2);

            if range_table.size() == 0 {
                return range_table.identity.clone();
            }
            if range_table.size() == 1 {
                return range_table.reduce_val();
            }

            let mid_rank = range_table.size() / 2;
            match range_table.select_key(mid_rank) {
                Some(mid_key) => {
                    let left_table = range_table.get_key_range(k1, &mid_key);
                    let right_start = match range_table.next_key(&mid_key) {
                        Some(k) => k,
                        None => mid_key.clone(),
                    };
                    let right_table = range_table.get_key_range(&right_start, k2);

                    let reducer = clone_fn2(&range_table.reducer);

                    let mid_val = match range_table.find(&mid_key) {
                        Some(v) => v,
                        None => range_table.identity.clone(),
                    };

                    let f1 = move || -> (r: V)
                        requires left_table@.dom().finite()
                    {
                        left_table.reduce_val()
                    };

                    let f2 = move || -> (r: V)
                        requires right_table@.dom().finite()
                    {
                        right_table.reduce_val()
                    };

                    let Pair(left_val, right_val) = ParaPair!(f1, f2);
                    let left_mid = reducer(&left_val, &mid_val);
                    reducer(&left_mid, &right_val)
                }
                None => {
                    range_table.reduce_val()
                }
            }
        }

        #[verifier::external_body] // veracity: accept — iterator boundary
        fn iter<'a>(&'a self) -> (it: OrderedTableMtEphIter<'a, K, V>)
        {
            self.base_table.iter()
        }
    }

    // 10. iterators

    impl<'a, K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> IntoIterator for &'a AugOrderedTableMtEph<K, V, F> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableMtEphIter<'a, K, V>;

        #[verifier::external_body] // veracity: accept — iterator boundary
        fn into_iter(self) -> (it: OrderedTableMtEphIter<'a, K, V>)
            ensures
                it@.0 == 0,
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    // 11. derive impls in verus!

    impl<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> Clone for AugOrderedTableMtEph<K, V, F> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned_base = self.base_table.clone();
            proof { assert(cloned_base@ == self.base_table@); }
            Self {
                base_table: cloned_base,
                cached_reduction: self.cached_reduction.clone(),
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> PartialEq for AugOrderedTableMtEph<K, V, F> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
                && self.cached_reduction == other.cached_reduction
        }
    }

    impl<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> Display for AugOrderedTableMtEph<K, V, F> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "AugOrderedTableMtEph(size: {}, reduction: {})",
                self.size(),
                self.cached_reduction
            )
        }
    }

    impl<K: MtKey, V: MtVal + Ord, F: MtReduceFn<V>> Debug for AugOrderedTableMtEph<K, V, F> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableMtEph")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }

    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableMtEphLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::empty($reducer, $identity);
            $(
                $crate::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::insert(&mut table, $k, $v, |_old, new| new.clone());
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::empty($reducer, $identity)
        }};
    }
}
