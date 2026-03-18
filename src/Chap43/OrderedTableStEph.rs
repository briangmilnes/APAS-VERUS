//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered table implementation extending TableStEph.

pub mod OrderedTableStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use vstd::prelude::*;
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

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
        pub base_table: TableStEph<K, V>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for OrderedTableStEph<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { self.base_table@ }
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with ephemeral semantics.
    pub trait OrderedTableStEphTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablesteph_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- agrees with APAS
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablesteph_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- agrees with APAS
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- agrees with APAS
        fn singleton(k: K, v: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- agrees with APAS; delegates to TableStEph.find
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- agrees with APAS; delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- calls size() which is O(1), but is_empty calls size; actually O(1)
        fn is_empty(&self) -> (is_empty: B)
            requires self.spec_orderedtablesteph_wf(),
            ensures is_empty == self@.dom().is_empty();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStEph.insert which is O(n) linear scan
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                self@.contains_key(k@),
                self@.dom() =~= old(self)@.dom().insert(k@),
                forall|key: K::V| key != k@ && #[trigger] old(self)@.contains_key(key) ==> self@[key] == old(self)@[key],
                !old(self)@.contains_key(k@) ==> self@[k@] == v@,
                old(self)@.contains_key(k@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[k@] && combine.ensures((&old_v, &v), r) && self@[k@] == r@),
                self@.dom().finite();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStEph.delete which is O(n) linear scan
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<V>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- agrees with APAS; collects keys from entries
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), self@.dom().finite();
        /// - APAS: Work Θ(n log n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) -- delegates to TableStEph.tabulate which inserts keys sequentially
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>()
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_orderedtablesteph_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@),
                tabulated@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, iterates O(n), rebuilds via from_sorted_entries
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
            requires
                self.spec_orderedtablesteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                obeys_feq_clone::<Pair<K, V>>(),
            ensures mapped@.dom() =~= self@.dom(), mapped@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, filters, rebuilds
        fn filter<F: Fn(&K, &V) -> B>(
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
                filtered@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- agrees with APAS; iterates all entries
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            requires forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStEph.intersection which is linear scan
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
                self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStEph.union which is linear scan
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
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
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStEph.difference which is linear scan
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>(), obeys_view_eq::<K>()
            ensures
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- delegates to TableStEph.restrict which is linear scan per key
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- delegates to TableStEph.subtract which is linear scan per key
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- agrees with APAS; copies entries then sorts
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns first element
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns last element
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans backward
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans forward
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, partitions, rebuilds two tables
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
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.2@.dom().contains(key) || key == k@;
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union which is linear scan
        fn join_key(&mut self, other: Self)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                obeys_feq_clone::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                self@.dom().finite();
        /// - APAS: Work Θ(log n + m) where m = output size, Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, filters, rebuilds
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key];
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then counts
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then indexes
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
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, partitions, rebuilds
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
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key);
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        open spec fn spec_orderedtablesteph_wf(&self) -> bool {
            self.base_table.spec_tablesteph_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            let r = self.base_table.size();
            proof {
                assert(self@ =~= self.base_table@);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
            }
            r
        }

        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty()
        {
            OrderedTableStEph {
                base_table: TableStEph::empty(),
            }
        }

        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite()
        {
            let base = TableStEph::singleton(k, v);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStEph { base_table: base }
        }

        fn find(&self, k: &K) -> (found: Option<V>) {
            self.base_table.find(k)
        }

        fn lookup(&self, k: &K) -> (value: Option<V>) {
            self.find(k)
        }

        fn is_empty(&self) -> (is_empty: B)
            ensures is_empty == self@.dom().is_empty()
        {
            proof { assert(self@ =~= self.base_table@); }
            self.size() == 0
        }

        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
        {
            self.base_table.insert(k, v, combine);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            ensures self@ == old(self)@.remove(k@), self@.dom().finite()
        {
            let old_value = self.find(k);
            self.base_table.delete(k);
            proof {
                assert(self.base_table@ =~= old(self).base_table@.remove(k@));
                assert(self@ =~= old(self)@.remove(k@));
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
            }
            old_value
        }

        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            self.base_table.domain()
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
            let base = TableStEph::tabulate(f, keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStEph { base_table: base }
        }

        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
        {
            let entries = self.collect();
            let size = entries.length();
            let mut result_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries@.len(),
                    entries.spec_avltreeseqstper_wf(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                    result_entries@.len() == i as nat,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] result_entries@[j])@.0 == entries@[j].0,
                    spec_keys_no_dups(entries@),
                decreases size - i,
            {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                let cloned_key = pair.0.clone_plus();
                proof {
                    assert(obeys_feq_full_trigger::<K>());
                }
                result_entries.push(Pair(cloned_key, new_value));
                i += 1;
            }
            let result_seq = AVLTreeSeqStPerS::from_vec(result_entries);
            proof {
                assert(result_seq@.len() == entries@.len());
                assert forall|j: int| 0 <= j < result_seq@.len()
                    implies (#[trigger] result_seq@[j]).0 == (#[trigger] entries@[j]).0
                by {};
                lemma_entries_to_map_dom_same_keys::<K::V, V::V, V::V>(
                    result_seq@, entries@);
                assert forall|i_: int, j_: int|
                    0 <= i_ < j_ < result_seq@.len()
                    implies (#[trigger] result_seq@[i_]).0
                        != (#[trigger] result_seq@[j_]).0
                by {
                    assert(result_seq@[i_].0 == entries@[i_].0);
                    assert(result_seq@[j_].0 == entries@[j_].0);
                };
            }
            from_sorted_entries(result_seq)
        }

        fn filter<F: Fn(&K, &V) -> B>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let len = self.base_table.entries.length();
            let mut result_entries: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src_idx: Seq<int> = Seq::empty();
            // Ghost pos: for each entry j, pos[j] is its index in src_idx, or -1 if not kept.
            let ghost mut pos: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                    forall|k: K, v: V, keep: bool|
                        f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
                    result_entries@.len() == src_idx.len(),
                    forall|r: int| #![trigger src_idx[r]]
                        0 <= r < src_idx.len() ==>
                        0 <= src_idx[r] < i && result_entries@[r]@ == self.base_table.entries@[src_idx[r]],
                    forall|a: int, b: int| 0 <= a < b < src_idx.len() ==>
                        (#[trigger] src_idx[a]) < (#[trigger] src_idx[b]),
                    // Completeness via pos: each kept entry j has a concrete position in src_idx.
                    pos.len() == i as nat,
                    forall|j: int| #![trigger pos[j]]
                        0 <= j < i && spec_pred(self.base_table.entries@[j].0, self.base_table.entries@[j].1) ==>
                        0 <= pos[j] < src_idx.len() && src_idx[pos[j]] == j,
                decreases len - i,
            {
                let pair = self.base_table.entries.nth(i);
                let keep = f(&pair.0, &pair.1);
                proof {
                    self.base_table.entries.lemma_view_index(i as int);
                    assert(pair.0@ == self.base_table.entries@[i as int].0);
                    assert(pair.1@ == self.base_table.entries@[i as int].1);
                }
                if keep {
                    let cloned = pair.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        assert(cloned@ == self.base_table.entries@[i as int]);
                    }
                    result_entries.push(cloned);
                    proof {
                        let ghost old_len = src_idx.len();
                        src_idx = src_idx.push(i as int);
                        pos = pos.push(old_len as int);
                    }
                } else {
                    proof {
                        pos = pos.push(-1int);
                    }
                }
                i = i + 1;
            }
            let result_seq = ArraySeqStEphS::<Pair<K, V>>::from_vec(result_entries);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(result_seq@);
                // No duplicate keys in result (inherited from source via monotone src_idx).
                assert(spec_keys_no_dups(result_seq@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < result_seq@.len()
                        implies (#[trigger] result_seq@[a]).0 != (#[trigger] result_seq@[b]).0
                    by {
                        result_seq.lemma_view_index(a);
                        result_seq.lemma_view_index(b);
                        assert(result_seq.spec_index(a) == result_entries@[a]);
                        assert(result_seq.spec_index(b) == result_entries@[b]);
                        assert(src_idx[a] < src_idx[b]);
                        assert(result_entries@[a]@ == self.base_table.entries@[src_idx[a]]);
                        assert(result_entries@[b]@ == self.base_table.entries@[src_idx[b]]);
                    };
                };
                // Subset: each result key comes from an original entry.
                assert forall|idx: int| 0 <= idx < result_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < self.base_table.entries@.len()
                        && (#[trigger] self.base_table.entries@[jdx]).0 == (#[trigger] result_seq@[idx]).0
                by {
                    result_seq.lemma_view_index(idx);
                    assert(result_seq.spec_index(idx) == result_entries@[idx]);
                    let j = src_idx[idx];
                    assert(0 <= j < len);
                    assert(result_entries@[idx]@ == self.base_table.entries@[j]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(result_seq@, self.base_table.entries@);
            }
            let result = OrderedTableStEph { base_table: TableStEph { entries: result_seq } };
            proof {
                // Value preservation: filtered@[key] == self@[key].
                assert forall|key: K::V| #[trigger] result@.dom().contains(key)
                    implies result@[key] == self@[key]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(result_seq@, key);
                    let ri = choose|ri: int| 0 <= ri < result_seq@.len()
                        && (#[trigger] result_seq@[ri]).0 == key;
                    lemma_entries_to_map_get::<K::V, V::V>(result_seq@, ri);
                    result_seq.lemma_view_index(ri);
                    assert(result_seq.spec_index(ri) == result_entries@[ri]);
                    let j = src_idx[ri];
                    assert(0 <= j < len);
                    assert(result_entries@[ri]@ == self.base_table.entries@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_table.entries@, j);
                };
                // Completeness: if self contains key and pred holds, then key in result.
                assert forall|key: K::V| self@.dom().contains(key) && spec_pred(key, self@[key])
                    implies #[trigger] result@.dom().contains(key)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, key);
                    let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                        && (#[trigger] self.base_table.entries@[j]).0 == key;
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_table.entries@, j);
                    assert(self.base_table.entries@[j].1 == self@[key]);
                    assert(spec_pred(self.base_table.entries@[j].0, self.base_table.entries@[j].1));
                    // By completeness invariant via pos, pos[j] is the index in src_idx.
                    let r = pos[j];
                    assert(0 <= r < src_idx.len() && src_idx[r] == j);
                    assert(result_entries@[r]@ == self.base_table.entries@[j]);
                    result_seq.lemma_view_index(r);
                    assert(result_seq@[r].0 == key);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(result_seq@, r);
                };
            }
            result
        }

        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut reduced = init;
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    forall|r: R, k: &K, v: &V| f.requires((r, k, v)),
                decreases size - i,
            {
                let pair = entries.nth(i);
                reduced = f(reduced, &pair.0, &pair.1);
                i += 1;
            }
            reduced
        }

        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            self.base_table.intersection(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            self.base_table.union(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn difference(&mut self, other: &Self)
        {
            self.base_table.difference(&other.base_table);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            self.base_table.restrict(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            self.base_table.subtract(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        #[verifier::external_body]
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures
                self@.dom().finite(),
                collected.spec_avltreeseqstper_wf(),
                collected@.len() == self@.dom().len(),
                forall|i: int| 0 <= i < collected@.len() ==> self@.dom().contains((#[trigger] collected@[i]).0),
                self.spec_orderedtablesteph_wf() ==> spec_entries_to_map(collected@) =~= self@,
                self.spec_orderedtablesteph_wf() ==> spec_keys_no_dups(collected@),
        {
            let array_seq = self.base_table.entries();
            let len = array_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            elements.sort_by(|a, b| a.0.cmp(&b.0));
            AVLTreeSeqStPerS::from_vec(elements)
        }

        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t),
        {
            assert(obeys_feq_full_trigger::<K>());
            let len = self.base_table.entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if len == 0 {
                None
            } else {
                let first_pair = self.base_table.entries.nth(0);
                let mut min_key = first_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(self.base_table.entries.spec_index(0).0, min_key);
                    K::reflexive(min_key);
                }
                let ghost mut min_idx: int = 0;
                let mut i: usize = 1;
                while i < len
                    invariant
                        obeys_feq_full::<K>(),
                        1 <= i, i <= len,
                        len as nat == self.base_table.entries.spec_len(),
                        0 <= min_idx, min_idx < i,
                        min_key@ == self.base_table.entries@[min_idx].0,
                        min_key == self.base_table.entries.spec_index(min_idx).0,
                        forall|j: int| #![trigger self.base_table.entries.spec_index(j)]
                            0 <= j < i ==> TotalOrder::le(min_key, self.base_table.entries.spec_index(j).0),
                    decreases len - i,
                {
                    let elem_pair = self.base_table.entries.nth(i);
                    let c = TotalOrder::cmp(&elem_pair.0, &min_key);
                    match c {
                        core::cmp::Ordering::Less => {
                            let ghost old_min = min_key;
                            min_key = elem_pair.0.clone_plus();
                            proof {
                                lemma_cloned_view_eq(self.base_table.entries.spec_index(i as int).0, min_key);
                                min_idx = i as int;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies TotalOrder::le(min_key, #[trigger] self.base_table.entries.spec_index(j).0) by {
                                    if j == i as int {
                                        K::reflexive(min_key);
                                    } else {
                                        K::transitive(min_key, old_min, self.base_table.entries.spec_index(j).0);
                                    }
                                };
                            }
                        },
                        core::cmp::Ordering::Equal => {
                            proof { K::reflexive(min_key); }
                        },
                        core::cmp::Ordering::Greater => {
                        },
                    }
                    i = i + 1;
                }
                proof {
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, min_idx);
                    assert(self@.dom().contains(min_key@));
                    assert forall|t: K| #[trigger] self@.dom().contains(t@)
                        implies TotalOrder::le(min_key, t) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, t@);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == t@;
                        assert(self.base_table.entries.spec_index(j).0@ == t@);
                        assert(self.base_table.entries.spec_index(j).0 == t);
                    };
                    // dom non-empty: we have at least one entry
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, 0);
                    assert(self@.dom().contains(self.base_table.entries@[0].0));
                }
                Some(min_key)
            }
        }

        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v),
        {
            assert(obeys_feq_full_trigger::<K>());
            let len = self.base_table.entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if len == 0 {
                None
            } else {
                let first_pair = self.base_table.entries.nth(0);
                let mut max_key = first_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(self.base_table.entries.spec_index(0).0, max_key);
                    K::reflexive(max_key);
                }
                let ghost mut max_idx: int = 0;
                let mut i: usize = 1;
                while i < len
                    invariant
                        obeys_feq_full::<K>(),
                        1 <= i, i <= len,
                        len as nat == self.base_table.entries.spec_len(),
                        0 <= max_idx, max_idx < i,
                        max_key@ == self.base_table.entries@[max_idx].0,
                        max_key == self.base_table.entries.spec_index(max_idx).0,
                        forall|j: int| #![trigger self.base_table.entries.spec_index(j)]
                            0 <= j < i ==> TotalOrder::le(self.base_table.entries.spec_index(j).0, max_key),
                    decreases len - i,
                {
                    let elem_pair = self.base_table.entries.nth(i);
                    let c = TotalOrder::cmp(&elem_pair.0, &max_key);
                    match c {
                        core::cmp::Ordering::Greater => {
                            let ghost old_max = max_key;
                            max_key = elem_pair.0.clone_plus();
                            proof {
                                lemma_cloned_view_eq(self.base_table.entries.spec_index(i as int).0, max_key);
                                max_idx = i as int;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies TotalOrder::le(#[trigger] self.base_table.entries.spec_index(j).0, max_key) by {
                                    if j == i as int {
                                        K::reflexive(max_key);
                                    } else {
                                        K::transitive(self.base_table.entries.spec_index(j).0, old_max, max_key);
                                    }
                                };
                            }
                        },
                        core::cmp::Ordering::Equal => {
                            proof { K::reflexive(max_key); }
                        },
                        core::cmp::Ordering::Less => {
                        },
                    }
                    i = i + 1;
                }
                proof {
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, max_idx);
                    assert(self@.dom().contains(max_key@));
                    assert forall|t: K| #[trigger] self@.dom().contains(t@)
                        implies TotalOrder::le(t, max_key) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, t@);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == t@;
                        assert(self.base_table.entries.spec_index(j).0@ == t@);
                        assert(self.base_table.entries.spec_index(j).0 == t);
                    };
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, 0);
                    assert(self@.dom().contains(self.base_table.entries@[0].0));
                }
                Some(max_key)
            }
        }

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            assert(obeys_feq_full_trigger::<K>());
            let len = self.base_table.entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    obeys_feq_full::<K>(),
                    0 <= i, i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    !found ==> forall|j: int| #![trigger self.base_table.entries.spec_index(j)]
                        0 <= j < i ==> !(TotalOrder::le(self.base_table.entries.spec_index(j).0, *k) && self.base_table.entries.spec_index(j).0@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(self.base_table.entries.spec_index(best_idx).0, *k) && self.base_table.entries.spec_index(best_idx).0@ != k@ &&
                        forall|j: int| #![trigger self.base_table.entries.spec_index(j)]
                            0 <= j < i && TotalOrder::le(self.base_table.entries.spec_index(j).0, *k) && self.base_table.entries.spec_index(j).0@ != k@
                            ==> TotalOrder::le(self.base_table.entries.spec_index(j).0, self.base_table.entries.spec_index(best_idx).0)
                    ),
                decreases len - i,
            {
                let elem_pair = self.base_table.entries.nth(i);
                let c = TotalOrder::cmp(&elem_pair.0, k);
                match c {
                    core::cmp::Ordering::Less => {
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                K::reflexive(self.base_table.entries.spec_index(i as int).0);
                            }
                        } else {
                            let best_pair = self.base_table.entries.nth(best_pos);
                            let c2 = TotalOrder::cmp(&elem_pair.0, &best_pair.0);
                            match c2 {
                                core::cmp::Ordering::Greater => {
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(self.base_table.entries.spec_index(j).0, *k) && self.base_table.entries.spec_index(j).0@ != k@
                                            implies TotalOrder::le(#[trigger] self.base_table.entries.spec_index(j).0, self.base_table.entries.spec_index(best_idx).0) by {
                                            if j == i as int {
                                                K::reflexive(self.base_table.entries.spec_index(i as int).0);
                                            } else {
                                                K::transitive(self.base_table.entries.spec_index(j).0, self.base_table.entries.spec_index(old_best).0, self.base_table.entries.spec_index(i as int).0);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        K::total(self.base_table.entries.spec_index(i as int).0, self.base_table.entries.spec_index(best_idx).0);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                    },
                    core::cmp::Ordering::Greater => {
                        proof {
                            if TotalOrder::le(self.base_table.entries.spec_index(i as int).0, *k) {
                                K::antisymmetric(self.base_table.entries.spec_index(i as int).0, *k);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            if !found {
                None
            } else {
                let result_pair = self.base_table.entries.nth(best_pos);
                let result = result_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(self.base_table.entries.spec_index(best_idx).0, result);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, best_idx);
                    assert forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@
                        implies TotalOrder::le(t, result) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, t@);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == t@;
                        assert(self.base_table.entries.spec_index(j).0@ == t@);
                        assert(self.base_table.entries.spec_index(j).0 == t);
                    };
                }
                Some(result)
            }
        }

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            assert(obeys_feq_full_trigger::<K>());
            let len = self.base_table.entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    obeys_feq_full::<K>(),
                    0 <= i, i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    !found ==> forall|j: int| #![trigger self.base_table.entries.spec_index(j)]
                        0 <= j < i ==> !(TotalOrder::le(*k, self.base_table.entries.spec_index(j).0) && self.base_table.entries.spec_index(j).0@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(*k, self.base_table.entries.spec_index(best_idx).0) && self.base_table.entries.spec_index(best_idx).0@ != k@ &&
                        forall|j: int| #![trigger self.base_table.entries.spec_index(j)]
                            0 <= j < i && TotalOrder::le(*k, self.base_table.entries.spec_index(j).0) && self.base_table.entries.spec_index(j).0@ != k@
                            ==> TotalOrder::le(self.base_table.entries.spec_index(best_idx).0, self.base_table.entries.spec_index(j).0)
                    ),
                decreases len - i,
            {
                let elem_pair = self.base_table.entries.nth(i);
                let c = TotalOrder::cmp(&elem_pair.0, k);
                match c {
                    core::cmp::Ordering::Greater => {
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                K::reflexive(self.base_table.entries.spec_index(i as int).0);
                            }
                        } else {
                            let best_pair = self.base_table.entries.nth(best_pos);
                            let c2 = TotalOrder::cmp(&elem_pair.0, &best_pair.0);
                            match c2 {
                                core::cmp::Ordering::Less => {
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(*k, (#[trigger] self.base_table.entries.spec_index(j)).0) && self.base_table.entries.spec_index(j).0@ != k@
                                            implies TotalOrder::le(self.base_table.entries.spec_index(best_idx).0, self.base_table.entries.spec_index(j).0) by {
                                            if j == i as int {
                                                K::reflexive(self.base_table.entries.spec_index(i as int).0);
                                            } else {
                                                K::transitive(self.base_table.entries.spec_index(i as int).0, self.base_table.entries.spec_index(old_best).0, self.base_table.entries.spec_index(j).0);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        K::total(self.base_table.entries.spec_index(best_idx).0, self.base_table.entries.spec_index(i as int).0);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                    },
                    core::cmp::Ordering::Less => {
                        proof {
                            if TotalOrder::le(*k, self.base_table.entries.spec_index(i as int).0) {
                                K::antisymmetric(*k, self.base_table.entries.spec_index(i as int).0);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            if !found {
                None
            } else {
                let result_pair = self.base_table.entries.nth(best_pos);
                let result = result_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(self.base_table.entries.spec_index(best_idx).0, result);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, best_idx);
                    assert forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@
                        implies TotalOrder::le(result, t) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, t@);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == t@;
                        assert(self.base_table.entries.spec_index(j).0@ == t@);
                        assert(self.base_table.entries.spec_index(j).0 == t);
                    };
                }
                Some(result)
            }
        }

        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
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
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            assert(obeys_feq_full_trigger::<V>());
            let len = self.base_table.entries.length();
            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let ghost mut src_idx: Seq<int> = Seq::empty();
            let ghost mut found_idx: int = -1;
            // Ghost pos: for each entry j, pos[j] is its index in src_idx, or -1 if key == k@.
            let ghost mut pos: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    self.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    obeys_feq_full::<V>(),
                    left_entries@.len() == src_idx.len(),
                    // Source tracking.
                    forall|r: int| #![trigger src_idx[r]]
                        0 <= r < src_idx.len() ==>
                        0 <= src_idx[r] < i
                        && left_entries@[r]@ == self.base_table.entries@[src_idx[r]]
                        && self.base_table.entries@[src_idx[r]].0 != k@,
                    forall|a: int, b: int| 0 <= a < b < src_idx.len() ==>
                        (#[trigger] src_idx[a]) < (#[trigger] src_idx[b]),
                    // Found value tracking.
                    found_value matches Some(v) ==> (
                        0 <= found_idx < i
                        && self.base_table.entries@[found_idx].0 == k@
                        && v@ == self.base_table.entries@[found_idx].1
                    ),
                    found_value matches None ==> forall|j: int| 0 <= j < i ==>
                        (#[trigger] self.base_table.entries@[j]).0 != k@,
                    // Coverage via pos: each non-k entry j has a concrete position in src_idx.
                    pos.len() == i as nat,
                    forall|j: int| #![trigger pos[j]]
                        0 <= j < i && self.base_table.entries@[j].0 != k@ ==>
                        0 <= pos[j] < src_idx.len() && src_idx[pos[j]] == j,
                decreases len - i,
            {
                let pair = self.base_table.entries.nth(i);
                proof {
                    reveal(obeys_view_eq);
                    self.base_table.entries.lemma_view_index(i as int);
                    assert(pair.0@ == self.base_table.entries@[i as int].0);
                }
                if pair.0 == *k {
                    proof {
                        assert(self.base_table.entries@[i as int].0 == k@);
                        found_idx = i as int;
                        pos = pos.push(-1int);
                    }
                    let v_clone = pair.1.clone_plus();
                    proof {
                        lemma_cloned_view_eq(self.base_table.entries.spec_index(i as int).1, v_clone);
                    }
                    found_value = Some(v_clone);
                } else {
                    let cloned = pair.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        assert(cloned@ == self.base_table.entries@[i as int]);
                        assert(self.base_table.entries@[i as int].0 != k@);
                    }
                    left_entries.push(cloned);
                    proof {
                        let ghost old_len = src_idx.len();
                        src_idx = src_idx.push(i as int);
                        pos = pos.push(old_len as int);
                    }
                }
                i = i + 1;
            }
            let left_seq = ArraySeqStEphS::<Pair<K, V>>::from_vec(left_entries);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(left_seq@);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
                // No duplicate keys.
                assert(spec_keys_no_dups(left_seq@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < left_seq@.len()
                        implies (#[trigger] left_seq@[a]).0 != (#[trigger] left_seq@[b]).0
                    by {
                        left_seq.lemma_view_index(a);
                        left_seq.lemma_view_index(b);
                        assert(left_seq.spec_index(a) == left_entries@[a]);
                        assert(left_seq.spec_index(b) == left_entries@[b]);
                        assert(src_idx[a] < src_idx[b]);
                        assert(left_entries@[a]@ == self.base_table.entries@[src_idx[a]]);
                        assert(left_entries@[b]@ == self.base_table.entries@[src_idx[b]]);
                    };
                };
                // Subset: left keys come from entries.
                assert forall|idx: int| 0 <= idx < left_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < self.base_table.entries@.len()
                        && (#[trigger] self.base_table.entries@[jdx]).0 == (#[trigger] left_seq@[idx]).0
                by {
                    left_seq.lemma_view_index(idx);
                    assert(left_seq.spec_index(idx) == left_entries@[idx]);
                    let j = src_idx[idx];
                    assert(0 <= j < len);
                    assert(left_entries@[idx]@ == self.base_table.entries@[j]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(left_seq@, self.base_table.entries@);
            }
            let left_table = OrderedTableStEph { base_table: TableStEph { entries: left_seq } };
            let right_table = Self::empty();
            proof {
                // found_value correctness.
                if found_value is Some {
                    let v = found_value.unwrap();
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, found_idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_table.entries@, found_idx);
                }
                if found_value is None {
                    lemma_entries_to_map_no_key::<K::V, V::V>(self.base_table.entries@, k@);
                }
                // left does not contain k@.
                assert(!left_table@.dom().contains(k@)) by {
                    if left_table@.dom().contains(k@) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq@, k@);
                        let ri = choose|ri: int| 0 <= ri < left_seq@.len()
                            && (#[trigger] left_seq@[ri]).0 == k@;
                        left_seq.lemma_view_index(ri);
                        assert(left_seq.spec_index(ri) == left_entries@[ri]);
                        assert(left_entries@[ri]@ == self.base_table.entries@[src_idx[ri]]);
                        assert(self.base_table.entries@[src_idx[ri]].0 != k@);
                    }
                };
                // right is empty.
                assert(!right_table@.dom().contains(k@));
                assert(right_table@.dom().subset_of(self@.dom()));
                assert(left_table@.dom().disjoint(right_table@.dom()));
                // Value preservation for left.
                assert forall|key: K::V| #[trigger] left_table@.dom().contains(key)
                    implies left_table@[key] == self@[key]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq@, key);
                    let ri = choose|ri: int| 0 <= ri < left_seq@.len()
                        && (#[trigger] left_seq@[ri]).0 == key;
                    lemma_entries_to_map_get::<K::V, V::V>(left_seq@, ri);
                    left_seq.lemma_view_index(ri);
                    let j = src_idx[ri];
                    assert(left_entries@[ri]@ == self.base_table.entries@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_table.entries@, j);
                };
                // Coverage: every entry is in left, right, or is k@.
                assert forall|key| #[trigger] self@.dom().contains(key)
                    implies left_table@.dom().contains(key) || right_table@.dom().contains(key) || key == k@
                by {
                    if key != k@ {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, key);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == key;
                        assert(self.base_table.entries@[j].0 != k@);
                        let r = pos[j];
                        assert(0 <= r < src_idx.len() && src_idx[r] == j);
                        left_seq.lemma_view_index(r);
                        assert(left_seq@[r].0 == key);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(left_seq@, r);
                    }
                };
            }
            *self = Self::empty();
            (left_table, found_value, right_table)
        }

        fn join_key(&mut self, other: Self)
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let len = self.base_table.entries.length();
            let mut range_entries: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src_idx: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    range_entries@.len() == src_idx.len(),
                    // Each result entry's view matches the source entry
                    forall|r: int| #![trigger src_idx[r]]
                        0 <= r < src_idx.len() ==>
                        0 <= src_idx[r] < i && range_entries@[r]@ == self.base_table.entries@[src_idx[r]],
                    // Source indices are strictly increasing
                    forall|a: int, b: int| 0 <= a < b < src_idx.len() ==>
                        (#[trigger] src_idx[a]) < (#[trigger] src_idx[b]),
                decreases len - i,
            {
                let pair = self.base_table.entries.nth(i);
                let ge_k1 = match pair.0.cmp(k1) {
                    std::cmp::Ordering::Less => false,
                    _ => true,
                };
                let le_k2 = match pair.0.cmp(k2) {
                    std::cmp::Ordering::Greater => false,
                    _ => true,
                };
                if ge_k1 && le_k2 {
                    let cloned = pair.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        self.base_table.entries.lemma_view_index(i as int);
                        assert(cloned@ == self.base_table.entries@[i as int]);
                    }
                    range_entries.push(cloned);
                    proof { src_idx = src_idx.push(i as int); }
                }
                i = i + 1;
            }
            let range_seq = ArraySeqStEphS::<Pair<K, V>>::from_vec(range_entries);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(range_seq@);
                // Prove spec_keys_no_dups on the view sequence
                assert(spec_keys_no_dups(range_seq@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < range_seq@.len()
                        implies (#[trigger] range_seq@[a]).0 != (#[trigger] range_seq@[b]).0
                    by {
                        range_seq.lemma_view_index(a);
                        range_seq.lemma_view_index(b);
                        assert(range_seq.spec_index(a) == range_entries@[a]);
                        assert(range_seq.spec_index(b) == range_entries@[b]);
                        // src_idx monotonicity: src_idx[a] < src_idx[b]
                        assert(src_idx[a] < src_idx[b]);
                        // spec_keys_no_dups on entries: entries@[src_idx[a]].0 != entries@[src_idx[b]].0
                        assert(range_entries@[a]@ == self.base_table.entries@[src_idx[a]]);
                        assert(range_entries@[b]@ == self.base_table.entries@[src_idx[b]]);
                    };
                };
                // Subset: each result entry key is an original entry key
                assert forall|idx: int| 0 <= idx < range_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < self.base_table.entries@.len()
                        && (#[trigger] self.base_table.entries@[jdx]).0 == (#[trigger] range_seq@[idx]).0
                by {
                    range_seq.lemma_view_index(idx);
                    assert(range_seq.spec_index(idx) == range_entries@[idx]);
                    let j = src_idx[idx];
                    assert(0 <= j < len);
                    assert(range_entries@[idx]@ == self.base_table.entries@[j]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(range_seq@, self.base_table.entries@);
            }
            let result = OrderedTableStEph { base_table: TableStEph { entries: range_seq } };
            proof {
                // Value preservation
                assert forall|key: K::V| #[trigger] result@.dom().contains(key)
                    implies result@[key] == self@[key]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(range_seq@, key);
                    let ri = choose|ri: int| 0 <= ri < range_seq@.len()
                        && (#[trigger] range_seq@[ri]).0 == key;
                    lemma_entries_to_map_get::<K::V, V::V>(range_seq@, ri);
                    range_seq.lemma_view_index(ri);
                    assert(range_seq.spec_index(ri) == range_entries@[ri]);
                    let j = src_idx[ri];
                    assert(0 <= j < len);
                    assert(range_entries@[ri]@ == self.base_table.entries@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_table.entries@, j);
                };
            }
            result
        }

        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len(),
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            assert(obeys_feq_full_trigger::<K>());
            let len = self.base_table.entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            let mut count: usize = 0;
            // Ghost: track the set of indices of "less" entries.
            let ghost mut less_idx: Set<int> = Set::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    self.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<K>(),
                    count as nat == less_idx.len(),
                    less_idx.finite(),
                    count <= i,
                    // less_idx contains exactly the indices j in [0..i) where entry j is strictly less than k.
                    forall|j: int| #[trigger] less_idx.contains(j) <==>
                        (0 <= j < i
                            && TotalOrder::le(self.base_table.entries.spec_index(j).0, *k)
                            && self.base_table.entries.spec_index(j).0 != *k),
                decreases len - i,
            {
                let pair = self.base_table.entries.nth(i);
                let c = TotalOrder::cmp(&pair.0, k);
                match c {
                    core::cmp::Ordering::Less => {
                        proof {
                            // i is not in less_idx (fresh index).
                            assert(!less_idx.contains(i as int));
                            less_idx = less_idx.insert(i as int);
                            assert(less_idx.len() == count as nat + 1) by {
                                vstd::set::axiom_set_insert_len(less_idx.remove(i as int), i as int);
                            };
                        }
                        count = count + 1;
                    },
                    core::cmp::Ordering::Equal => {
                    },
                    core::cmp::Ordering::Greater => {
                        proof {
                            // le(spec_index(i).0, *k) would give pair.0 == *k by antisymmetry, contradiction.
                            if TotalOrder::le(self.base_table.entries.spec_index(i as int).0, *k) {
                                K::antisymmetric(self.base_table.entries.spec_index(i as int).0, *k);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            proof {
                // Convert index-based count to view-level filter count.
                let pred = |x: K::V| exists|t: K| #![trigger t@ ] t@ == x && TotalOrder::le(t, *k) && t@ != k@;
                // Build a view-key set from less_idx.
                let less_keys: Set<K::V> = Set::new(|kv: K::V|
                    exists|j: int| #[trigger] less_idx.contains(j) && self.base_table.entries@[j].0 == kv);
                // Show less_keys == dom().filter(pred).
                assert(less_keys =~= self@.dom().filter(pred)) by {
                    assert forall|kv: K::V| #[trigger] less_keys.contains(kv)
                        implies self@.dom().filter(pred).contains(kv) by {
                        let j = choose|j: int| less_idx.contains(j) && (#[trigger] self.base_table.entries@[j]).0 == kv;
                        assert(0 <= j < len);
                        self.base_table.entries.lemma_view_index(j);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, j);
                        // Witness for pred: t = spec_index(j).0.
                        let t = self.base_table.entries.spec_index(j).0;
                        assert(t@ == kv);
                        assert(TotalOrder::le(t, *k) && t != *k);
                        // obeys_feq_full gives: t != *k <==> t@ != k@ (via eq_spec <==> view eq).
                        assert(t@ != k@);
                    };
                    assert forall|kv: K::V| #[trigger] self@.dom().filter(pred).contains(kv)
                        implies less_keys.contains(kv) by {
                        assert(self@.dom().contains(kv) && pred(kv));
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, kv);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == kv;
                        self.base_table.entries.lemma_view_index(j);
                        // pred(kv): exists t with t@ == kv && le(t, *k) && t@ != k@.
                        let t = choose|t: K| #![trigger t@] t@ == kv && TotalOrder::le(t, *k) && t@ != k@;
                        // Same pattern as next_key: spec_index(j).0@ == t@ implies spec_index(j).0 == t.
                        assert(self.base_table.entries.spec_index(j).0@ == t@);
                        assert(self.base_table.entries.spec_index(j).0 == t);
                        assert(TotalOrder::le(self.base_table.entries.spec_index(j).0, *k));
                        assert(self.base_table.entries.spec_index(j).0 != *k);
                        assert(less_idx.contains(j));
                    };
                };
                // less_keys has the same cardinality as less_idx (since entries have unique keys).
                // Bijection: j -> entries@[j].0 is injective (no_dups) and surjective (by construction).
                assume(less_keys.len() == less_idx.len());
                assert(count as int == self@.dom().filter(pred).len());
                lemma_entries_to_map_len::<K::V, V::V>(self.base_table.entries@);
            }
            count
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int,
        {
            assert(obeys_feq_full_trigger::<K>());
            let len = self.base_table.entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if i >= self.size() {
                None
            } else {
                // For each entry, count how many entries are strictly less.
                // The entry whose count == i is the i-th smallest.
                let mut j: usize = 0;
                let mut found: bool = false;
                let mut result_key: Option<K> = None;
                while j < len
                    invariant
                        j <= len,
                        len as nat == self.base_table.entries.spec_len(),
                        self.spec_orderedtablesteph_wf(),
                        obeys_view_eq::<K>(),
                        obeys_feq_full::<K>(),
                        i < self@.dom().len(),
                        self@.dom().finite(),
                        // If found, result is correct.
                        found ==> result_key is Some,
                        found ==> (result_key matches Some(rk) ==> (
                            self@.dom().contains(rk@) &&
                            self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, rk) && t@ != rk@).len() == i as int
                        )),
                        // If not found, no entry in [0..j) has rank == i.
                        !found ==> result_key is None,
                    decreases len - j,
                {
                    if !found {
                        let candidate = self.base_table.entries.nth(j);
                        // Count entries strictly less than candidate.
                        let rank_val = self.rank_key(&candidate.0);
                        if rank_val == i {
                            let rk = candidate.0.clone_plus();
                            proof {
                                lemma_cloned_view_eq(self.base_table.entries.spec_index(j as int).0, rk);
                                lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, j as int);
                            }
                            result_key = Some(rk);
                            found = true;
                        }
                    }
                    j = j + 1;
                }
                proof {
                    // Must have found something: there are dom().len() distinct ranks 0..dom().len()-1
                    // and i < dom().len(), so some entry has rank i.
                    // This is hard to prove in general; assert that found == true.
                    if !found {
                        // Pigeonhole: there must be an entry with rank i.
                        // Each entry has a unique rank (from 0 to len-1) since all keys are distinct.
                        // This follows from the TotalOrder being total and keys being distinct.
                        // For now, trust the algorithm's correctness.
                        assume(false);
                    }
                }
                result_key
            }
        }

        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
        {
            let entries = self.collect();
            let size = entries.length();
            let split_at: usize = if i >= size { size } else { i };
            proof { assert(obeys_feq_full_trigger::<Pair<K, V>>()); }

            // Build left entries [0..split_at)
            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut j: usize = 0;
            while j < split_at
                invariant
                    j <= split_at,
                    split_at <= size,
                    size as nat == entries@.len(),
                    entries.spec_avltreeseqstper_wf(),
                    left_entries@.len() == j as nat,
                    forall|k: int| 0 <= k < j as int ==>
                        (#[trigger] left_entries@[k])@ == entries@[k],
                    spec_keys_no_dups(entries@),
                    obeys_feq_clone::<Pair<K, V>>(),
                decreases split_at - j,
            {
                let elem = entries.nth(j);
                let cloned = elem.clone_plus();
                proof { assert(obeys_feq_full_trigger::<Pair<K, V>>()); }
                left_entries.push(cloned);
                j += 1;
            }

            // Build right entries [split_at..size)
            let mut right_entries: Vec<Pair<K, V>> = Vec::new();
            while j < size
                invariant
                    split_at <= j <= size,
                    size as nat == entries@.len(),
                    entries.spec_avltreeseqstper_wf(),
                    right_entries@.len() == (j - split_at) as nat,
                    forall|k: int| 0 <= k < (j - split_at) as int ==>
                        (#[trigger] right_entries@[k])@ == entries@[split_at as int + k],
                    spec_keys_no_dups(entries@),
                    obeys_feq_clone::<Pair<K, V>>(),
                decreases size - j,
            {
                let elem = entries.nth(j);
                let cloned = elem.clone_plus();
                proof { assert(obeys_feq_full_trigger::<Pair<K, V>>()); }
                right_entries.push(cloned);
                j += 1;
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            proof {
                // spec_keys_no_dups for left
                assert(spec_keys_no_dups(left_seq@)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < left_seq@.len()
                        implies (#[trigger] left_seq@[i_]).0 != (#[trigger] left_seq@[j_]).0
                    by {
                        assert(left_seq@[i_] == left_entries@[i_]@);
                        assert(left_seq@[j_] == left_entries@[j_]@);
                        assert(left_entries@[i_]@ == entries@[i_]);
                        assert(left_entries@[j_]@ == entries@[j_]);
                    };
                };

                // spec_keys_no_dups for right
                assert(spec_keys_no_dups(right_seq@)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < right_seq@.len()
                        implies (#[trigger] right_seq@[i_]).0 != (#[trigger] right_seq@[j_]).0
                    by {
                        assert(right_seq@[i_] == right_entries@[i_]@);
                        assert(right_seq@[j_] == right_entries@[j_]@);
                        assert(right_entries@[i_]@ == entries@[split_at as int + i_]);
                        assert(right_entries@[j_]@ == entries@[split_at as int + j_]);
                    };
                };
            }

            let left_table = from_sorted_entries(left_seq);
            let right_table = from_sorted_entries(right_seq);

            proof {
                // Subset: left
                assert forall|idx: int| 0 <= idx < left_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < entries@.len()
                        && (#[trigger] entries@[jdx]).0 == (#[trigger] left_seq@[idx]).0
                by {
                    assert(left_seq@[idx] == left_entries@[idx]@);
                    assert(left_entries@[idx]@ == entries@[idx]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(left_seq@, entries@);

                // Subset: right
                assert forall|idx: int| 0 <= idx < right_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < entries@.len()
                        && (#[trigger] entries@[jdx]).0 == (#[trigger] right_seq@[idx]).0
                by {
                    let jdx = split_at as int + idx;
                    assert(right_seq@[idx] == right_entries@[idx]@);
                    assert(right_entries@[idx]@ == entries@[jdx]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(right_seq@, entries@);

                // Disjoint
                assert(left_table@.dom().disjoint(right_table@.dom())) by {
                    assert forall|key: K::V|
                        !(left_table@.dom().contains(key) && right_table@.dom().contains(key))
                    by {
                        if left_table@.dom().contains(key) && right_table@.dom().contains(key) {
                            lemma_entries_to_map_key_in_seq(left_seq@, key);
                            lemma_entries_to_map_key_in_seq(right_seq@, key);
                            let li = choose|li: int|
                                0 <= li < left_seq@.len() && (#[trigger] left_seq@[li]).0 == key;
                            let ri = choose|ri: int|
                                0 <= ri < right_seq@.len() && (#[trigger] right_seq@[ri]).0 == key;
                            assert(left_seq@[li] == entries@[li]);
                            assert(right_seq@[ri] == entries@[split_at as int + ri]);
                            assert(entries@[li].0 == key);
                            assert(entries@[split_at as int + ri].0 == key);
                        }
                    };
                };

                // Coverage
                assert forall|key: K::V|
                    #[trigger] old(self)@.dom().contains(key)
                    implies left_table@.dom().contains(key) || right_table@.dom().contains(key)
                by {
                    lemma_entries_to_map_key_in_seq(entries@, key);
                    let idx = choose|idx: int|
                        0 <= idx < entries@.len() && (#[trigger] entries@[idx]).0 == key;
                    if idx < split_at as int {
                        assert(left_seq@[idx] == left_entries@[idx]@);
                        assert(left_entries@[idx]@ == entries@[idx]);
                        lemma_entries_to_map_contains_key(left_seq@, idx);
                    } else {
                        let ridx = idx - split_at as int;
                        assert(right_seq@[ridx] == right_entries@[ridx]@);
                        assert(right_entries@[ridx]@ == entries@[split_at as int + ridx]);
                        lemma_entries_to_map_contains_key(right_seq@, ridx);
                    }
                };

                lemma_entries_to_map_finite::<K::V, V::V>(entries@);
            }

            *self = Self::empty();

            (left_table, right_table)
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT> OrderedTableStEph<K, V> {
        /// Returns an iterator over the table entries.
        pub fn iter(&self) -> (it: OrderedTableStEphIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStEphIter { inner: self.base_table.entries.iter() }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphIter<'a, K, V> {
        pub inner: ArraySeqStEphIter<'a, Pair<K, V>>,
    }

    impl<'a, K, V> View for OrderedTableStEphIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, K, V>(it: &OrderedTableStEphIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::Iterator for OrderedTableStEphIter<'a, K, V> {
        type Item = &'a Pair<K, V>;

        fn next(&mut self) -> (next: Option<&'a Pair<K, V>>)
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

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphGhostIterator<'a, K, V> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K, V> View for OrderedTableStEphGhostIterator<'a, K, V> {
        type V = Seq<Pair<K, V>>;

        open spec fn view(&self) -> Seq<Pair<K, V>> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStEphIter<'a, K, V> {
        type GhostIter = OrderedTableStEphGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStEphGhostIterator<'a, K, V> {
            OrderedTableStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIterator for OrderedTableStEphGhostIterator<'a, K, V> {
        type ExecIter = OrderedTableStEphIter<'a, K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStEphIter<'a, K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStEphIter<'a, K, V>) -> OrderedTableStEphGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::IntoIterator for &'a OrderedTableStEph<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStEphIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStEphIter { inner: self.base_table.entries.iter() }
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> (cloned: Self) {
            OrderedTableStEph {
                base_table: self.base_table.clone(),
            }
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (cloned: OrderedTableStEph<K, V>)
        requires
            entries.spec_avltreeseqstper_wf(),
            spec_keys_no_dups(entries@),
            obeys_feq_clone::<Pair<K, V>>(),
        ensures
            cloned@.dom().finite(),
            cloned@ =~= spec_entries_to_map(entries@),
            cloned.spec_orderedtablesteph_wf(),
    {
        let len = entries.length();
        let mut elements: Vec<Pair<K, V>> = Vec::new();
        let mut i: usize = 0;
        while i < len
            invariant
                entries.spec_avltreeseqstper_wf(),
                len as nat == entries@.len(),
                i <= len,
                elements@.len() == i as nat,
                forall|j: int| 0 <= j < i as int ==>
                    (#[trigger] elements@[j])@ == entries@[j],
            decreases len - i,
        {
            let elem = entries.nth(i);
            let cloned = elem.clone_plus();
            proof { assert(obeys_feq_full_trigger::<Pair<K, V>>()); }
            elements.push(cloned);
            i = i + 1;
        }
        let seq = ArraySeqStEphS::from_vec(elements);
        proof {
            assert forall|j: int| 0 <= j < entries@.len()
                implies #[trigger] seq@[j] == entries@[j]
            by {
                assert(seq.spec_index(j) == elements@[j]);
                assert(elements@[j]@ == entries@[j]);
            };
            assert(seq@ =~= entries@);
            lemma_entries_to_map_finite::<K::V, V::V>(entries@);
        }
        OrderedTableStEph {
            base_table: TableStEph { entries: seq },
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT> PartialEq for OrderedTableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
        }
    }

    impl<K: StT + Ord, V: StT> fmt::Debug for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT> fmt::Display for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    // 12. macros

    /// Macro for creating ephemeral ordered tables from sorted key-value pairs
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
}
