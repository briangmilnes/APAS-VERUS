//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral reducer-augmented ordered table implementation.

pub mod AugOrderedTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 7. free functions (calculate_reduction)
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
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::*;
    use crate::OrderedTableStEphLit;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;

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
    pub struct AugOrderedTableStEph<K: StT + Ord, V: StT + Ord, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        pub base_table: OrderedTableStEph<K, V>,
        pub reducer: F,
        pub identity: V,
        pub cached_reduction: V,
    }

    pub type AugOrderedTableEph<K, V, F> = AugOrderedTableStEph<K, V, F>;

    // 5. view impls

    impl<K: StT + Ord, V: StT + Ord, F> View for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
    }

    // 7. free functions (calculate_reduction)

    pub fn calculate_reduction<K: StT + Ord, V: StT + Ord, F>(
        base: &OrderedTableStEph<K, V>,
        reducer: &F,
        identity: &V,
    ) -> (reduced: V)
    where
        F: Fn(&V, &V) -> V + Clone,
        requires
            forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
            base.spec_orderedtablesteph_wf(),
        ensures base@.dom().finite(),
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

    // 7b. proof fns

    proof fn lemma_aug_view<K: StT + Ord, V: StT + Ord, F: Fn(&V, &V) -> V + Clone>(
        t: &AugOrderedTableStEph<K, V, F>,
    )
        ensures t@ =~= t.base_table@
    {}

    // 8. traits

    /// Trait defining all augmented ordered table operations (ADT 43.3) with ephemeral semantics
    /// Extends ordered table operations with efficient reduction and in-place mutations
    pub trait AugOrderedTableStEphTrait<K: StT + Ord, V: StT + Ord, F>: Sized + View<V = Map<K::V, V::V>>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        spec fn spec_augorderedtablesteph_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- delegates to OrderedTableStEph.size
        fn size(&self) -> (count: usize)
            requires self.spec_augorderedtablesteph_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty base table + stores reducer
        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps OrderedTableStEph.singleton
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures tree@.dom().finite();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to OrderedTableStEph.find (linear scan)
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_augorderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_augorderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- compares size to 0
        fn is_empty(&self) -> (is_empty: B)
            requires self.spec_augorderedtablesteph_wf(),
            ensures is_empty == self@.dom().is_empty(), self@.dom().finite();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to OrderedTableStEph.insert (linear dup check)
        fn insert<G: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: G)
            requires
                old(self).spec_augorderedtablesteph_wf(),
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
                self@.dom().finite();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to OrderedTableStEph.delete (linear scan)
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_augorderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures self@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to OrderedTableStEph.domain
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires self.spec_augorderedtablesteph_wf(), obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), self@.dom().finite();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n^2), Span Θ(n^2) -- delegates to OrderedTableStEph.tabulate (sequential insert loop)
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                forall|v1: &V, v2: &V| #[trigger] reducer.requires((v1, v2)),
                keys@.len() < usize::MAX as nat,
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_augorderedtablesteph_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@);
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- delegates to OrderedTableStEph.map (collect + rebuild)
        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> (mapped: Self)
            requires self.spec_augorderedtablesteph_wf(), forall|k: &K, v: &V| f.requires((k, v)), obeys_feq_clone::<Pair<K, V>>()
            ensures mapped@.dom() =~= self@.dom(), mapped@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- delegates to OrderedTableStEph.filter (collect + filter + rebuild)
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
            requires
                self.spec_augorderedtablesteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                filtered@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] filtered@.contains_key(k) ==> filtered@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] filtered@.dom().contains(k),
                filtered@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to OrderedTableStEph.reduce (linear fold)
        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> (reduced: R)
            requires self.spec_augorderedtablesteph_wf(), forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to OrderedTableStEph.intersection (linear scan)
        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            requires
                old(self).spec_augorderedtablesteph_wf(),
                other.spec_augorderedtablesteph_wf(),
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
                self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to OrderedTableStEph.union (linear merge)
        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            requires
                old(self).spec_augorderedtablesteph_wf(),
                other.spec_augorderedtablesteph_wf(),
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
                self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to OrderedTableStEph.difference (linear scan)
        fn difference(&mut self, other: &Self)
            requires old(self).spec_augorderedtablesteph_wf(), other.spec_augorderedtablesteph_wf(),obeys_view_eq::<K>()
            ensures
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_augorderedtablesteph_wf()
            ensures
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_augorderedtablesteph_wf()
            ensures
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects entries and sorts by key
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            requires self.spec_augorderedtablesteph_wf()
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns first element
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns last element
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for predecessor
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for successor
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by key
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized,
            requires old(self).spec_augorderedtablesteph_wf(), obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@);
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union (linear merge)
        fn join_key(&mut self, other: Self)
            requires
                old(self).spec_augorderedtablesteph_wf(),
                other.spec_augorderedtablesteph_wf(),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
                old(self)@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                self@.dom().finite();
        /// - APAS: Work Θ(log n + m), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then filters by range
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires self.spec_augorderedtablesteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom());
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then counts elements < k
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires self.spec_augorderedtablesteph_wf(), obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then indexes
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            requires self.spec_augorderedtablesteph_wf(), obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by rank
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized,
            requires old(self).spec_augorderedtablesteph_wf(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom());
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- calculate_reduction is external_body (iterates all entries)
        fn reduce_val(&self) -> (reduced: V)
            requires self.spec_augorderedtablesteph_wf(),
            ensures self@.dom().finite();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- get_key_range + calculate_reduction
        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            requires self.spec_augorderedtablesteph_wf(),
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT + Ord, F> AugOrderedTableStEphTrait<K, V, F> for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        open spec fn spec_augorderedtablesteph_wf(&self) -> bool {
            self.base_table.spec_orderedtablesteph_wf()
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
            ensures empty@ == Map::<K::V, V::V>::empty()
        {
            let base = OrderedTableStEph::empty();
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
            ensures tree@.dom().finite()
        {
            let base = OrderedTableStEph::singleton(k, v.clone());
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
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                }
        {
            self.base_table.find(k)
        }

        fn lookup(&self, k: &K) -> (value: Option<V>)
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                }
        {
            self.base_table.lookup(k)
        }

        fn is_empty(&self) -> (is_empty: B)
            ensures is_empty == self@.dom().is_empty(), self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                lemma_pair_set_to_map_dom_finite(self.base_table.tree@);
            }
            self.base_table.is_empty()
        }

        fn insert<G: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: G)
        {
            self.base_table.insert(k, v, combine);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            ensures self@.dom().finite()
        {
            let updated = self.base_table.delete(k);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
            updated
        }

        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            proof { lemma_aug_view(self); }
            self.base_table.domain()
        }

        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
        {
                      assert(obeys_feq_full_trigger::<K>());
           assert(obeys_feq_full_trigger::<V>());
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let base_table = OrderedTableStEph::tabulate(f, keys);
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

        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> (mapped: Self)
        {
            let new_base = self.base_table.map(f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof { lemma_aug_view(self); lemma_aug_view(&r); }
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

        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> (reduced: R)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.reduce(init, f)
        }

        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
        {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
        {
            self.base_table.union(&other.base_table, f);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn difference(&mut self, other: &Self)
        {
            self.base_table.difference(&other.base_table);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            self.base_table.restrict(keys);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            self.base_table.subtract(keys);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len()
        {
            proof { lemma_aug_view(self); }
            self.base_table.collect()
        }

        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t),
        {
            proof { lemma_aug_view(self); }
            self.base_table.first_key()
        }

        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v),
        {
            proof { lemma_aug_view(self); }
            self.base_table.last_key()
        }

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            proof { lemma_aug_view(self); }
            self.base_table.previous_key(k)
        }

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            proof { lemma_aug_view(self); }
            self.base_table.next_key(k)
        }

        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@),
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

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
            (left, found_value, right)
        }

        fn join_key(&mut self, other: Self)
        {
            let self_size = self.base_table.size();
            let other_size = other.base_table.size();
            let old_reduction = self.cached_reduction.clone();
            let other_reduction = other.cached_reduction.clone();

            self.base_table.join_key(other.base_table);

            if self_size == 0 {
                self.cached_reduction = other_reduction;
            } else if other_size == 0 {
                self.cached_reduction = old_reduction;
            } else {
                self.cached_reduction = (self.reducer)(&old_reduction, &other_reduction);
            }
            proof { lemma_aug_view(self); }
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
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

        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len(),
        {
            proof { lemma_aug_view(self); }
            self.base_table.rank_key(k)
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int,
        {
            proof { lemma_aug_view(self); }
            self.base_table.select_key(i)
        }

        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom()),
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
                // wf chain: aug_wf → orderedtable_wf → bst_wf → tree@.finite().
                lemma_pair_set_to_map_dom_finite(self.base_table.tree@);
            }
            self.cached_reduction.clone()
        }

        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            ensures self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                // wf chain: aug_wf → orderedtable_wf → bst_wf → tree@.finite().
                lemma_pair_set_to_map_dom_finite(self.base_table.tree@);
            }
            let range_table = self.get_key_range(k1, k2);
            range_table.cached_reduction.clone()
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT + Ord, F: Fn(&V, &V) -> V + Clone> AugOrderedTableStEph<K, V, F> {
        /// Returns an iterator over the table entries via the base ordered table.
        pub fn iter(&self) -> (it: OrderedTableStEphIter<K, V>)
            requires self.spec_augorderedtablesteph_wf()
            ensures
                it@.0 == 0,
                it@.1.len() == self.base_table.tree@.len(),
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord, F: Fn(&V, &V) -> V + Clone> std::iter::IntoIterator for &'a AugOrderedTableStEph<K, V, F> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableStEphIter<K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_augorderedtablesteph_wf()
            ensures
                it@.0 == 0,
                it@.1.len() == self.base_table.tree@.len(),
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT + Ord, F> Clone for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let base_cloned = self.base_table.clone();
            proof { assume(base_cloned@ == self.base_table@); }
            Self {
                base_table: base_cloned,
                cached_reduction: self.cached_reduction.clone(),
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus! (Debug/Display must stay outside per Verus limitation)

    impl<K: StT + Ord, V: StT + Ord, F> PartialEq for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
                && self.cached_reduction == other.cached_reduction
        }
    }

    impl<K: StT + Ord, V: StT + Ord, F> Display for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "AugOrderedTableStEph(size: {}, reduction: {})",
                self.size(),
                self.cached_reduction
            )
        }
    }

    impl<K: StT + Ord, V: StT + Ord, F> Debug for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableStEph")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }

    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableStEphLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::empty($reducer, $identity);
            $(
                $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::insert(&mut table, $k, $v, |_old, new| new.clone());
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::empty($reducer, $identity)
        }};
    }
}
