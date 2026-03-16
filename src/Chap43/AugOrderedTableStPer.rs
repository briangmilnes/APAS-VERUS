//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent reducer-augmented ordered table implementation.

pub mod AugOrderedTableStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 7. proof fns, helper fns
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::OrderedTableStPerLit;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

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
    pub struct AugOrderedTableStPer<K: StT + Ord, V: StT, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        pub base_table: OrderedTableStPer<K, V>,
        pub reducer: F,
        pub identity: V,
        pub cached_reduction: V,
    }

    pub type AugOrderedTablePer<K, V, F> = AugOrderedTableStPer<K, V, F>;

    // 5. view impls

    impl<K: StT + Ord, V: StT, F> View for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
    }

    // 7. free functions (calculate_reduction)

    /// Fold all values in `base` through `reducer`, returning `identity` for empty tables.
    pub fn calculate_reduction<K: StT + Ord, V: StT, F>(
        base: &OrderedTableStPer<K, V>,
        reducer: &F,
        identity: &V,
    ) -> (reduced: V)
    where
        F: Fn(&V, &V) -> V + Clone,
        requires true,
        ensures base@.dom().finite(),
    {
        let pairs = base.collect();
        // collect ensures: base@.dom().finite(), pairs.spec_avltreeseqstper_wf()
        let sz = pairs.length();
        // length ensures: sz as nat == pairs@.len(), given spec_avltreeseqstper_wf()
        if sz == 0 {
            return identity.clone();
        }
        // sz > 0 so pairs@.len() > 0, safe to call nth(0).
        let mut reduced = pairs.nth(0).1.clone();
        let mut i: usize = 1;
        while i < sz
            invariant
                1 <= i <= pairs@.len(),
                sz as nat == pairs@.len(),
                pairs.spec_avltreeseqstper_wf(),
            decreases pairs@.len() - i,
        {
            let pair = pairs.nth(i);
            proof { assume(reducer.requires((&reduced, &pair.1))); }
            reduced = reducer(&reduced, &pair.1);
            i += 1;
        }
        reduced
    }

    // 7b. proof fns

    proof fn lemma_aug_view<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone>(
        t: &AugOrderedTableStPer<K, V, F>,
    )
        ensures t@ =~= t.base_table@
    {}

    // 8. traits

    /// Trait defining all augmented ordered table operations (ADT 43.3)
    /// Extends ordered table operations with efficient reduction
    pub trait AugOrderedTableStPerTrait<K: StT + Ord, V: StT, F>: Sized + View<V = Map<K::V, V::V>>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        spec fn spec_augorderedtablestper_wf(&self) -> bool;

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- delegates to base table size
        fn size(&self) -> (count: usize)
            requires self.spec_augorderedtablestper_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- constructs empty base table with reducer/identity
        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_augorderedtablestper_wf();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- constructs singleton base table with reducer/identity
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>(),
            ensures tree@.dom().finite(), tree.spec_augorderedtablestper_wf();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- delegates to TableStPer which uses linear scan
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_augorderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- clones base table (persistent), inserts linearly, recalculates reduction O(n)
        fn insert(&self, k: K, v: V) -> (updated: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                updated@.dom() =~= self@.dom().insert(k@),
                updated@.dom().finite(),
                updated.spec_augorderedtablestper_wf();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- clones base table (persistent), deletes linearly, recalculates reduction O(n)
        fn delete(&self, k: &K) -> (updated: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures updated@.dom().finite(), updated.spec_augorderedtablestper_wf();
        /// - APAS: Work O(n), Span O(n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- extracts keys from base table entries
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), self@.dom().finite();
        /// - APAS: Work O(n log n), Span O(n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- applies f to each key, then recalculates reduction O(n)
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>(),
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_augorderedtablestper_wf(),
                forall|k: K::V| #![auto] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@),
                tabulated@.dom().finite();
        /// - APAS: Work O(n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- maps all values linearly, then recalculates reduction O(n)
        fn map<G: Fn(&V) -> V>(&self, f: G) -> (mapped: Self)
            requires self.spec_augorderedtablestper_wf(), forall|v: &V| f.requires((v,)), obeys_feq_full::<K>(),
            ensures
                mapped@.dom() == self@.dom(),
                forall|k: K::V| #![auto] mapped@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && mapped@[k] == result@),
                mapped@.dom().finite(),
                mapped.spec_augorderedtablestper_wf();
        /// - APAS: Work O(n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- filters base table linearly, then recalculates reduction O(n)
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                obeys_feq_full::<Pair<K, V>>(),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                filtered@.dom().subset_of(self@.dom()),
                forall|k: K::V| #![auto] filtered@.contains_key(k) ==> filtered@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] filtered@.dom().contains(k),
                filtered@.dom().finite(),
                filtered.spec_augorderedtablestper_wf();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table intersection (linear scan), then recalculates reduction
        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (common: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                other.spec_augorderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<K>(),
            ensures
                common@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #![auto] common@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && common@[k] == r@),
                common@.dom().finite(),
                common.spec_augorderedtablestper_wf();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) -- delegates to base table union (linear merge), then recalculates reduction
        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (combined: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                other.spec_augorderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                combined@.dom() =~= self@.dom().union(other@.dom()),
                forall|k: K::V| #![auto] self@.contains_key(k) && !other@.contains_key(k)
                    ==> combined@[k] == self@[k],
                forall|k: K::V| #![auto] other@.contains_key(k) && !self@.contains_key(k)
                    ==> combined@[k] == other@[k],
                forall|k: K::V| #![auto] self@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && combined@[k] == r@),
                combined@.dom().finite(),
                combined.spec_augorderedtablestper_wf();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table difference (linear scan), then recalculates reduction
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                remaining@.dom() =~= self@.dom().difference(other@.dom()),
                forall|k: K::V| #![auto] remaining@.contains_key(k) ==> remaining@[k] == self@[k],
                remaining@.dom().finite(),
                remaining.spec_augorderedtablestper_wf();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table restrict (linear scan), then recalculates reduction
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                restricted@.dom() =~= self@.dom().intersect(keys@),
                forall|k: K::V| #![auto] restricted@.contains_key(k) ==> restricted@[k] == self@[k],
                restricted@.dom().finite(),
                restricted.spec_augorderedtablestper_wf();
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n * m), Span O(n * m) -- delegates to base table subtract (linear scan), then recalculates reduction
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                subtracted@.dom() =~= self@.dom().difference(keys@),
                forall|k: K::V| #![auto] subtracted@.contains_key(k) ==> subtracted@[k] == self@[k],
                subtracted@.dom().finite(),
                subtracted.spec_augorderedtablestper_wf();
        /// - APAS: Work O(n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- collects base table entries into AVLTreeSeqStPer
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, returns first key
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(v, t);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, returns last key
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(t, v);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, finds predecessor
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, finds successor
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, partitions into two tables + recalculates reductions
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            ensures
                self@.dom().finite(),
                parts.0@.dom().finite(),
                parts.2@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@;
        /// - APAS: Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) -- delegates to base table union (linear merge), then recalculates reduction
        fn join_key(left: &Self, right: &Self) -> (joined: Self)
            requires
                left.spec_augorderedtablestper_wf(),
                right.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                joined@.dom() =~= left@.dom().union(right@.dom()),
                joined@.dom().finite(),
                joined.spec_augorderedtablestper_wf();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, filters range, builds new table + recalculates reduction
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| range@.dom().contains(key) ==> range@[key] == self@[key];
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, counts predecessors
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, selects by index
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- collects entries, sorts, splits at rank into two tables + recalculates reductions
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures
                self@.dom().finite(),
                split.0@.dom().finite(),
                split.0@.dom().subset_of(self@.dom()),
                split.1@.dom().finite(),
                split.1@.dom().subset_of(self@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| self@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key);
        /// - APAS: Work O(1), Span O(1) -- augmented tables cache the reduction
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- calculate_reduction is external_body, iterates all values
        fn reduce_val(&self) -> (reduced: V)
            ensures self@.dom().finite();
        /// - APAS: Work O(log n), Span O(log n) -- split + cached reduction
        /// - Claude-Opus-4.6: Work O(n log n), Span O(n log n) -- get_key_range O(n log n) + calculate_reduction O(n)
        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT, F> AugOrderedTableStPerTrait<K, V, F> for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        open spec fn spec_augorderedtablestper_wf(&self) -> bool {
            self.base_table.spec_orderedtablestper_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

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
            proof { lemma_aug_view(&r); }
            r
        }

        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            ensures tree@.dom().finite(), tree.spec_augorderedtablestper_wf()
        {
            let base = OrderedTableStPer::singleton(k, v.clone());
            let r = Self {
                base_table: base,
                cached_reduction: v,
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn find(&self, k: &K) -> (found: Option<V>)
        {
            proof { lemma_aug_view(self); }
            self.base_table.find(k)
        }

        fn insert(&self, k: K, v: V) -> (updated: Self)
        {
            let new_base = self.base_table.insert(k, v);
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

        fn delete(&self, k: &K) -> (updated: Self)
            ensures updated@.dom().finite(), updated.spec_augorderedtablestper_wf()
        {
            let new_base = self.base_table.delete(k);
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

        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            proof { lemma_aug_view(self); }
            self.base_table.domain()
        }

        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
        {
            let base_table = OrderedTableStPer::tabulate(f, keys);
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

        fn map<G: Fn(&V) -> V>(&self, f: G) -> (mapped: Self)
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

        fn filter<G: Fn(&K, &V) -> B>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
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

        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (common: Self)
        {
            let new_base = self.base_table.intersection(&other.base_table, f);
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

        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (combined: Self)
        {
            let new_base = self.base_table.union(&other.base_table, f);
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

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let new_base = self.base_table.difference(&other.base_table);
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

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
        {
            let new_base = self.base_table.restrict(keys);
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

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
        {
            let new_base = self.base_table.subtract(keys);
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

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf()
        {
            proof { lemma_aug_view(self); }
            self.base_table.collect()
        }

        #[verifier::external_body]
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(v, t),
        {
            self.base_table.first_key()
        }

        #[verifier::external_body]
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(t, v),
        {
            self.base_table.last_key()
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            self.base_table.previous_key(k)
        }

        #[verifier::external_body]
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            self.base_table.next_key(k)
        }

        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            ensures
                self@.dom().finite(),
                parts.0@.dom().finite(),
                parts.2@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
        {
            let (left_base, middle, right_base) = self.base_table.split_key(k);

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

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
            (left, middle, right)
        }

        fn join_key(left: &Self, right: &Self) -> (joined: Self)
        {
            let new_base = OrderedTableStPer::join_key(&left.base_table, &right.base_table);
            let new_reduction = if left.base_table.size() == 0 {
                right.cached_reduction.clone()
            } else if right.base_table.size() == 0 {
                left.cached_reduction.clone()
            } else {
                proof { assume(left.reducer.requires(
                    (&left.cached_reduction, &right.cached_reduction))); }
                (left.reducer)(&left.cached_reduction, &right.cached_reduction)
            };

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: left.reducer.clone(),
                identity: left.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| range@.dom().contains(key) ==> range@[key] == self@[key],
        {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(self);
                lemma_aug_view(&r);
            }
            r
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, *k) && t@ != k@).len(),
        {
            self.base_table.rank_key(k)
        }

        #[verifier::external_body]
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int,
        {
            self.base_table.select_key(i)
        }

        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            ensures
                self@.dom().finite(),
                split.0@.dom().finite(),
                split.0@.dom().subset_of(self@.dom()),
                split.1@.dom().finite(),
                split.1@.dom().subset_of(self@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| self@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
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

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
            (left, right)
        }

        fn reduce_val(&self) -> (reduced: V)
            ensures self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.base_table.entries@);
            }
            self.cached_reduction.clone()
        }

        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            ensures self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.base_table.entries@);
            }
            let range_table = self.get_key_range(k1, k2);
            range_table.reduce_val()
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> AugOrderedTableStPer<K, V, F> {
        /// Returns an iterator over the table entries via the base ordered table.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.base_table.entries.seq@,
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }

    impl<'a, K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> std::iter::IntoIterator for &'a AugOrderedTableStPer<K, V, F> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.base_table.entries.seq@,
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }

    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> PartialEqSpecImpl for AugOrderedTableStPer<K, V, F> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> Eq for AugOrderedTableStPer<K, V, F> {}

    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> PartialEq for AugOrderedTableStPer<K, V, F> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.base_table == other.base_table;
            proof { lemma_aug_view(self); lemma_aug_view(other); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT, F> Clone for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let r = Self {
                base_table: self.base_table.clone(),
                cached_reduction: self.cached_reduction.clone(),
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            r
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<K: StT + Ord, V: StT, F> Display for AugOrderedTableStPer<K, V, F>
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

    impl<K: StT + Ord, V: StT, F> Debug for AugOrderedTableStPer<K, V, F>
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
}
