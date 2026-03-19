//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered table backed by AVLTreeSeqStEphS<Pair<K,V>>.

pub mod OrderedTableStEph {

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::{spec_inorder_values, lemma_inorder_values_maps_to_views};
    #[cfg(verus_keep_ghost)]
    use crate::Chap42::TableStEph::TableStEph::{spec_entries_to_map, spec_keys_no_dups, lemma_entries_to_map_key_in_seq, lemma_entries_to_map_contains_key, lemma_entries_to_map_len, lemma_entries_to_map_no_key, lemma_entries_to_map_get, lemma_entries_to_map_dom_subset, lemma_entries_to_map_dom_same_keys, lemma_entries_to_map_finite};
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
        pub base_seq: AVLTreeSeqStEphS<Pair<K, V>>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for OrderedTableStEph<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { spec_entries_to_map(self.base_seq@) }
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
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStEph.delete which is O(n) linear scan
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<V>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite(), self.spec_orderedtablesteph_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- agrees with APAS; collects keys from entries
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires self.spec_orderedtablesteph_wf(), obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), self@.dom().finite();
        /// - APAS: Work Θ(n log n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) -- delegates to TableStEph.tabulate which inserts keys sequentially
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.len() < usize::MAX as nat,
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
            ensures mapped@.dom() =~= self@.dom(), mapped@.dom().finite(), mapped.spec_orderedtablesteph_wf();
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
                filtered@.dom().finite(),
                filtered.spec_orderedtablesteph_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- agrees with APAS; iterates all entries
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            requires self.spec_orderedtablesteph_wf(), forall|r: R, k: &K, v: &V| f.requires((r, k, v))
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
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
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
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStEph.difference which is linear scan
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedtablesteph_wf(), other.spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>(), obeys_view_eq::<K>()
            ensures
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- delegates to TableStEph.restrict which is linear scan per key
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- delegates to TableStEph.subtract which is linear scan per key
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- agrees with APAS; copies entries then sorts
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            requires self.spec_orderedtablesteph_wf()
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns first element
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns last element
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans backward
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans forward
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablesteph_wf()
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
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.2@.dom().contains(key) || key == k@,
                split.0.spec_orderedtablesteph_wf(),
                split.2.spec_orderedtablesteph_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union which is linear scan
        fn join_key(&mut self, other: Self)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                obeys_feq_clone::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                old(self)@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - APAS: Work Θ(log n + m) where m = output size, Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, filters, rebuilds
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_orderedtablesteph_wf();
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
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_orderedtablesteph_wf(),
                split.1.spec_orderedtablesteph_wf();
    }

    // 6. spec fns

    /// Proves that spec_keys_no_dups on a pair-view sequence implies no_duplicates.
    /// Different keys means different pairs.
    pub proof fn lemma_keys_no_dups_implies_no_duplicates<KV, VV>(s: Seq<(KV, VV)>)
        requires spec_keys_no_dups(s)
        ensures s.no_duplicates()
    {
        assert forall|i: int, j: int| 0 <= i < s.len() && 0 <= j < s.len() && i != j
            implies s[i] != s[j]
        by {
            if i < j {
                assert(s[i].0 != s[j].0);
            } else {
                assert(s[j].0 != s[i].0);
            }
        };
    }

    /// Spec predicate for rank_key: x is strictly less than k in the total order.
    pub open spec fn spec_rank_pred<K: StT + Ord + TotalOrder>(x: K::V, k: K) -> bool {
        exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, k) && t@ != k@
    }

    /// Get the length of the backing AVL sequence.
    fn avl_seq_length<K: StT + Ord, V: StT>(seq: &AVLTreeSeqStEphS<Pair<K, V>>) -> (len: usize)
        requires seq.spec_avltreeseqsteph_wf()
        ensures len as nat == seq@.len()
    {
        seq.length()
    }

    /// Get nth element from the AVL sequence.
    fn avl_seq_nth<'a, K: StT + Ord, V: StT>(seq: &'a AVLTreeSeqStEphS<Pair<K, V>>, index: usize) -> (elem: &'a Pair<K, V>)
        requires seq.spec_avltreeseqsteph_wf(), (index as int) < seq@.len()
        ensures elem@ == seq@[index as int]
    {
        seq.nth(index)
    }

    /// Check if a key exists in another table's backing sequence.
    /// Uses only key comparison (obeys_view_eq), no value cloning.
    fn key_in_other<K: StT + Ord, V: StT>(
        other: &AVLTreeSeqStEphS<Pair<K, V>>,
        k: &K,
    ) -> (found: bool)
        requires other.spec_avltreeseqsteph_wf(), obeys_view_eq::<K>()
        ensures found == spec_entries_to_map(other@).dom().contains(k@)
    {
        let other_len = avl_seq_length(other);
        let mut j: usize = 0;
        while j < other_len
            invariant
                j <= other_len,
                other_len as nat == other@.len(),
                other.spec_avltreeseqsteph_wf(),
                obeys_view_eq::<K>(),
                forall|q: int| 0 <= q < j as int ==> (#[trigger] other@[q]).0 != k@,
            decreases other_len - j,
        {
            let other_pair = avl_seq_nth(other, j);
            proof { reveal(obeys_view_eq); }
            if other_pair.0 == *k {
                proof {
                    lemma_entries_to_map_contains_key::<K::V, V::V>(other@, j as int);
                }
                return true;
            }
            j = j + 1;
        }
        proof {
            lemma_entries_to_map_no_key::<K::V, V::V>(other@, k@);
        }
        false
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        open spec fn spec_orderedtablesteph_wf(&self) -> bool {
            self.base_seq.spec_avltreeseqsteph_wf()
            && spec_keys_no_dups(self.base_seq@)
            && self.base_seq@.len() < usize::MAX as nat
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            let r = self.base_seq.length();
            proof {
                lemma_entries_to_map_len::<K::V, V::V>(self.base_seq@);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
                self.base_seq@.unique_seq_to_set();
            }
            r
        }

        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty()
        {
            OrderedTableStEph {
                base_seq: AVLTreeSeqStEphS::empty(),
            }
        }

        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite()
        {
            let base = AVLTreeSeqStEphS::singleton(Pair(k, v));
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(base@);
                assert(base@.len() == 1);
                assert(base@.last() == (k@, v@));
                assert(base@.drop_last().len() == 0);
                assert(spec_entries_to_map(base@.drop_last()) =~= Map::<K::V, V::V>::empty());
                assert(spec_entries_to_map(base@) =~= Map::<K::V, V::V>::empty().insert(k@, v@));
            }
            OrderedTableStEph { base_seq: base }
        }

        fn find(&self, k: &K) -> (found: Option<V>)
        {
            let len = self.base_seq.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_seq@.len(),
                    self.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    forall|j: int| 0 <= j < i as int ==> (#[trigger] self.base_seq@[j]).0 != k@,
                decreases len - i,
            {
                let pair = self.base_seq.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == *k {
                    let v_clone = pair.1.clone_plus();
                    proof {
                        lemma_cloned_view_eq(pair.1, v_clone);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, i as int);
                        lemma_entries_to_map_get::<K::V, V::V>(self.base_seq@, i as int);
                    }
                    return Some(v_clone);
                }
                i = i + 1;
            }
            proof {
                lemma_entries_to_map_no_key::<K::V, V::V>(self.base_seq@, k@);
            }
            None
        }

        fn lookup(&self, k: &K) -> (value: Option<V>) {
            self.find(k)
        }

        fn is_empty(&self) -> (is_empty: B)
            ensures is_empty == self@.dom().is_empty()
        {
            self.size() == 0
        }

        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let ghost old_seq = self.base_seq@;
            let ghost old_map = self@;
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut match_found = false;
            let ghost mut found_at: int = len as int;
            // Ghost witnesses for combine result.
            let ghost mut combined_old_v: V = v;
            let ghost mut combined_r: V = v;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == old_seq.len(),
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    spec_keys_no_dups(old_seq),
                    old_map == spec_entries_to_map(old_seq),
                    all@.len() == i as int,
                    match_found == (found_at < len as int),
                    found_at == len as int || (0 <= found_at < i as int && old_seq[found_at].0 == k@),
                    !match_found ==>
                        forall|j: int| 0 <= j < i as int ==> (#[trigger] old_seq[j]).0 != k@,
                    // Non-matching entries copied verbatim.
                    forall|j: int| 0 <= j < i as int && j != found_at ==>
                        (#[trigger] all@[j])@ == old_seq[j],
                    // Found entry: keys match, value is combine result.
                    match_found ==> all@[found_at]@.0 == old_seq[found_at].0,
                    match_found ==> combined_old_v@ == old_map[k@],
                    match_found ==> combine.ensures((&combined_old_v, &v), combined_r),
                    match_found ==> all@[found_at]@.1 == combined_r@,
                decreases len - i,
            {
                let pair = self.base_seq.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == k && !match_found {
                    match_found = true;
                    proof { found_at = i as int; }
                    let pair_clone = pair.clone_plus();
                    proof { lemma_cloned_view_eq(*pair, pair_clone); }
                    let final_value = combine(&pair.1, &v);
                    proof {
                        lemma_entries_to_map_get::<K::V, V::V>(old_seq, i as int);
                        combined_old_v = pair_clone.1;
                        combined_r = final_value;
                    }
                    all.push(Pair(pair_clone.0, final_value));
                } else {
                    let cloned = pair.clone_plus();
                    proof { lemma_cloned_view_eq(*pair, cloned); }
                    let ghost all_pre = all@;
                    all.push(cloned);
                    proof {
                        if match_found {
                            assert(found_at < all_pre.len() as int);
                            assert(all@[found_at] == all_pre[found_at]);
                        }
                    }
                }
                i = i + 1;
            }
            // Connect match_found to spec-level contains_key.
            proof {
                if match_found {
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, found_at);
                    assert(old_map.contains_key(k@));
                } else {
                    // No entry has key k@, so map doesn't contain it.
                    if old_map.contains_key(k@) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, k@);
                        let j = choose|j: int| 0 <= j < old_seq.len()
                            && (#[trigger] old_seq[j]).0 == k@;
                        assert(false); // contradicts !match_found invariant
                    }
                    assert(!old_map.contains_key(k@));
                }
            }
            if !match_found {
                proof {
                    lemma_entries_to_map_len::<K::V, V::V>(old_seq);
                    assert(!old_map.contains_key(k@));
                }
                all.push(Pair(k, v));
            }
            proof {
                if match_found {
                    assert(all@.len() == len as int);
                } else {
                    assert(all@.len() == len as int + 1);
                }
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            proof {
                let tree_seq = tree@;
                let new_map = spec_entries_to_map(tree_seq);
                // Prove no-dups.
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree_seq.len()
                        implies (#[trigger] tree_seq[i_]).0 != (#[trigger] tree_seq[j_]).0
                    by {
                        assert(tree_seq[i_] == all@[i_]@);
                        assert(tree_seq[j_] == all@[j_]@);
                        if i_ < len as int && j_ < len as int {
                            if i_ == found_at {
                                assert(all@[i_]@.0 == old_seq[found_at].0);
                            } else {
                                assert(all@[i_]@ == old_seq[i_]);
                            }
                            if j_ == found_at {
                                assert(all@[j_]@.0 == old_seq[found_at].0);
                            } else {
                                assert(all@[j_]@ == old_seq[j_]);
                            }
                        } else {
                            assert(j_ == len as int);
                            assert(all@[j_]@.0 == k@);
                            assert(!match_found);
                            assert(i_ != found_at);
                            assert(all@[i_]@ == old_seq[i_]);
                            assert(old_seq[i_].0 != k@);
                        }
                    };
                };
                // Prove wf.
                assert(tree_seq.len() == all@.len());
                if match_found {
                    assert(tree_seq.len() as nat == old_seq.len());
                }
                assert(self.spec_orderedtablesteph_wf());
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                lemma_entries_to_map_finite::<K::V, V::V>(old_seq);
                // Prove dom =~= old_dom.insert(k@).
                assert(new_map.dom() =~= old_map.dom().insert(k@)) by {
                    assert forall|key: K::V| #[trigger] new_map.dom().contains(key)
                        implies old_map.dom().contains(key) || key == k@
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, key);
                        let p = choose|p: int| 0 <= p < tree_seq.len()
                            && (#[trigger] tree_seq[p]).0 == key;
                        assert(tree_seq[p] == all@[p]@);
                        if p < len as int && p != found_at {
                            assert(all@[p]@ == old_seq[p]);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, p);
                        } else if p < len as int && p == found_at {
                            assert(all@[p]@.0 == old_seq[found_at].0);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, found_at);
                        } else {
                            assert(all@[p]@.0 == k@);
                        }
                    };
                    if found_at < len as int {
                        assert(tree_seq[found_at] == all@[found_at]@);
                        assert(all@[found_at]@.0 == k@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, found_at);
                    } else {
                        assert(tree_seq[len as int] == all@[len as int]@);
                        assert(all@[len as int]@.0 == k@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, len as int);
                    }
                    assert forall|key: K::V| old_map.dom().contains(key)
                        implies #[trigger] new_map.dom().contains(key)
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, key);
                        let q = choose|q: int| 0 <= q < old_seq.len()
                            && (#[trigger] old_seq[q]).0 == key;
                        assert(tree_seq[q] == all@[q]@);
                        if q == found_at {
                            assert(all@[q]@.0 == old_seq[found_at].0);
                        } else {
                            assert(all@[q]@ == old_seq[q]);
                        }
                        lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, q);
                    };
                };
                // Value: key != k@ and old_map.contains_key(key) => same value.
                assert forall|key: K::V| key != k@ && #[trigger] old_map.contains_key(key)
                    implies new_map[key] == old_map[key]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, key);
                    let q = choose|q: int| 0 <= q < old_seq.len()
                        && (#[trigger] old_seq[q]).0 == key;
                    assert(q != found_at);
                    assert(all@[q]@ == old_seq[q]);
                    assert(tree_seq[q] == all@[q]@);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, q);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, q);
                };
                // Value: !old_map.contains_key(k@) => new_map[k@] == v@.
                if !old_map.contains_key(k@) {
                    assert(all@[len as int]@.0 == k@);
                    assert(all@[len as int]@.1 == v@);
                    assert(tree_seq[len as int] == all@[len as int]@);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, len as int);
                }
                // Value: old_map.contains_key(k@) => combine witness.
                if old_map.contains_key(k@) {
                    assert(all@[found_at]@.0 == k@);
                    assert(tree_seq[found_at] == all@[found_at]@);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, found_at);
                }
            }
        }

        fn delete(&mut self, k: &K) -> (updated: Option<V>)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let ghost old_seq = self.base_seq@;
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let ghost mut found_at: int = len as int;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == old_seq.len(),
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    spec_keys_no_dups(old_seq),
                    found_at == len as int || (0 <= found_at < i as int && old_seq[found_at].0 == k@),
                    found_at == len as int ==>
                        forall|j: int| 0 <= j < i as int ==> (#[trigger] old_seq[j]).0 != k@,
                    found_value.is_some() == (found_at < len as int),
                    all@.len() == (if found_at < len as int { i as int - 1 } else { i as int }),
                    found_at == len as int ==>
                        forall|p: int| 0 <= p < all@.len() ==> (#[trigger] all@[p])@ == old_seq[p],
                    found_at < len as int ==>
                        forall|p: int| 0 <= p < found_at ==> (#[trigger] all@[p])@ == old_seq[p],
                    found_at < len as int ==>
                        forall|p: int| found_at <= p < all@.len() as int
                            ==> (#[trigger] all@[p])@ == old_seq[p + 1],
                decreases len - i,
            {
                let pair = self.base_seq.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == *k && found_value.is_none() {
                    found_value = Some(pair.1.clone_plus());
                    proof { found_at = i as int; }
                } else {
                    let cloned = pair.clone_plus();
                    proof { lemma_cloned_view_eq(*pair, cloned); }
                    all.push(cloned);
                }
                i = i + 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            proof {
                // No-dups on tree@.
                assert(spec_keys_no_dups(tree@)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree@.len()
                        implies (#[trigger] tree@[i_]).0 != (#[trigger] tree@[j_]).0
                    by {
                        let si = if found_at < len as int && i_ >= found_at { i_ + 1 } else { i_ };
                        let sj = if found_at < len as int && j_ >= found_at { j_ + 1 } else { j_ };
                        assert(tree@[i_] == old_seq[si]);
                        assert(tree@[j_] == old_seq[sj]);
                        assert(si < sj);
                    };
                };
                // No entry in tree@ has key k@.
                if found_at < len as int {
                    assert forall|p: int| 0 <= p < tree@.len()
                        implies (#[trigger] tree@[p]).0 != k@
                    by {
                        let sp = if p < found_at { p } else { p + 1 };
                        assert(tree@[p] == old_seq[sp]);
                        assert(sp != found_at);
                    };
                }
                lemma_entries_to_map_finite::<K::V, V::V>(tree@);
                // Prove map extensional equality: tree_map =~= old_map.remove(k@).
                let tree_map = spec_entries_to_map(tree@);
                let old_map = spec_entries_to_map(old_seq);
                // Forward: every key in tree_map is in old_map with same value, and key != k@.
                assert forall|key: K::V| #[trigger] tree_map.contains_key(key) implies
                    old_map.contains_key(key) && tree_map[key] == old_map[key] && key != k@
                by {
                    lemma_entries_to_map_key_in_seq(tree@, key);
                    let p = choose|p: int| 0 <= p < tree@.len() && (#[trigger] tree@[p]).0 == key;
                    lemma_entries_to_map_get(tree@, p);
                    let sp = if found_at < len as int && p >= found_at { p + 1 } else { p };
                    assert(tree@[p] == old_seq[sp]);
                    lemma_entries_to_map_contains_key(old_seq, sp);
                    lemma_entries_to_map_get(old_seq, sp);
                };
                // Backward: every key in old_map with key != k@ is in tree_map.
                assert forall|key: K::V| old_map.contains_key(key) && key != k@ implies
                    #[trigger] tree_map.contains_key(key) && tree_map[key] == old_map[key]
                by {
                    lemma_entries_to_map_key_in_seq(old_seq, key);
                    let q = choose|q: int| 0 <= q < old_seq.len() && (#[trigger] old_seq[q]).0 == key;
                    lemma_entries_to_map_get(old_seq, q);
                    assert(q != found_at);
                    let tp = if found_at < len as int && q > found_at { q - 1 } else { q };
                    assert(tree@[tp] == old_seq[q]);
                    lemma_entries_to_map_contains_key(tree@, tp);
                    lemma_entries_to_map_get(tree@, tp);
                };
                assert(tree_map =~= old_map.remove(k@));
            }
            found_value
        }

        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            let len = avl_seq_length(&self.base_seq);
            let mut domain = ArraySetStEph::empty();
            let mut i: usize = 0;
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
            }
            while i < len
                invariant
                    obeys_feq_clone::<K>(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    len as nat == self.base_seq@.len(),
                    0 <= i <= len,
                    domain.spec_arraysetsteph_wf(),
                    domain@.finite(),
                    forall|kv: K::V| domain@.contains(kv) ==>
                        #[trigger] self@.dom().contains(kv),
                    forall|j: int| 0 <= j < i ==>
                        domain@.contains(#[trigger] self.base_seq@[j].0),
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                let key_clone = pair.0.clone_plus();
                proof {
                    assert(cloned(pair.0, key_clone));
                    assert(obeys_feq_clone::<K>());
                    assert(key_clone@ == self.base_seq@[i as int].0);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(
                        self.base_seq@, i as int,
                    );
                }
                domain.insert(key_clone);
                i += 1;
            }
            proof {
                assert(domain@ =~= self@.dom()) by {
                    assert forall|kv: K::V| self@.dom().contains(kv)
                        implies domain@.contains(kv)
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            self.base_seq@, kv,
                        );
                    };
                };
            }
            domain
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let seq = keys.to_seq();
            let len = seq.length();
            let ghost seq_view = seq@;
            let mut all: Vec<Pair<K, V>> = Vec::new();
            // Ghost: key and result witnesses per index.
            let ghost mut key_args: Seq<K> = Seq::empty();
            let ghost mut results: Seq<V> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    0 <= i <= len,
                    len as int == seq_view.len(),
                    seq_view == seq@,
                    seq_view.no_duplicates(),
                    seq_view.to_set() =~= keys@,
                    forall|k: &K| f.requires((k,)),
                    obeys_feq_full::<K>(),
                    all@.len() == i as int,
                    key_args.len() == i as int,
                    results.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] all@[j])@.0 == seq_view[j],
                    forall|j: int| 0 <= j < i as int ==> {
                        &&& (#[trigger] key_args[j])@ == seq_view[j]
                        &&& f.ensures((&key_args[j],), results[j])
                        &&& all@[j]@.1 == results[j]@
                    },
                decreases len - i,
            {
                let k = seq.nth(i);
                let val = f(k);
                let k_clone = k.clone_plus();
                proof {
                    assert(cloned(*k, k_clone));
                    key_args = key_args.push(*k);
                    results = results.push(val);
                }
                all.push(Pair(k_clone, val));
                i += 1;
            }
            proof {
                assert(all@.len() as nat == seq_view.len());
                seq_view.unique_seq_to_set();
                assert(seq_view.len() == keys@.len());
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            let tabulated = OrderedTableStEph { base_seq: tree };
            proof {
                let tree_seq = tree@;
                let new_map = spec_entries_to_map(tree_seq);
                // Prove no-dups.
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree_seq.len()
                        implies (#[trigger] tree_seq[i_]).0 != (#[trigger] tree_seq[j_]).0
                    by {
                        assert(tree_seq[i_] == all@[i_]@);
                        assert(tree_seq[j_] == all@[j_]@);
                        assert(all@[i_]@.0 == seq_view[i_]);
                        assert(all@[j_]@.0 == seq_view[j_]);
                        // seq_view.no_duplicates() => seq_view[i_] != seq_view[j_].
                    };
                };
                assert(tabulated.spec_orderedtablesteph_wf());
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                // Prove dom =~= keys@.
                assert(new_map.dom() =~= keys@) by {
                    // Forward: every key in new_map is in keys@.
                    assert forall|key: K::V| #[trigger] new_map.dom().contains(key)
                        implies keys@.contains(key)
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, key);
                        let p = choose|p: int| 0 <= p < tree_seq.len()
                            && (#[trigger] tree_seq[p]).0 == key;
                        assert(tree_seq[p] == all@[p]@);
                        assert(all@[p]@.0 == seq_view[p]);
                        assert(seq_view.to_set().contains(seq_view[p]));
                    };
                    // Backward: every key in keys@ is in new_map.
                    assert forall|key: K::V| keys@.contains(key)
                        implies #[trigger] new_map.dom().contains(key)
                    by {
                        assert(seq_view.to_set().contains(key));
                        let q = choose|q: int| 0 <= q < seq_view.len()
                            && (#[trigger] seq_view[q]) == key;
                        assert(all@[q]@.0 == key);
                        assert(tree_seq[q] == all@[q]@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, q);
                    };
                };
                // Prove value witness.
                assert forall|key: K::V| #[trigger] new_map.contains_key(key)
                    implies (exists|key_arg: K, result: V|
                        key_arg@ == key && f.ensures((&key_arg,), result)
                        && new_map[key] == result@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, key);
                    let p = choose|p: int| 0 <= p < tree_seq.len()
                        && (#[trigger] tree_seq[p]).0 == key;
                    assert(tree_seq[p] == all@[p]@);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, p);
                    let ka = key_args[p];
                    let rv = results[p];
                    assert(ka@ == key);
                    assert(f.ensures((&ka,), rv));
                    assert(new_map[key] == rv@);
                };
            }
            tabulated
        }

        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_seq@.len(),
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                    all@.len() == i as int,
                    forall|j: int| 0 <= j < i ==> (#[trigger] all@[j])@.0 == self.base_seq@[j].0,
                decreases len - i,
            {
                let pair = self.base_seq.nth(i);
                let new_val = f(&pair.0, &pair.1);
                let cloned_pair = pair.clone_plus();
                proof { lemma_cloned_view_eq(*pair, cloned_pair); }
                all.push(Pair(cloned_pair.0, new_val));
                i = i + 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            proof {
                assert(spec_keys_no_dups(tree@)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree@.len()
                        implies (#[trigger] tree@[i_]).0 != (#[trigger] tree@[j_]).0
                    by {
                        assert(tree@[i_].0 == self.base_seq@[i_].0);
                        assert(tree@[j_].0 == self.base_seq@[j_].0);
                    };
                };
                lemma_entries_to_map_dom_same_keys::<K::V, V::V, V::V>(tree@, self.base_seq@);
                lemma_entries_to_map_finite::<K::V, V::V>(tree@);
            }
            OrderedTableStEph { base_seq: tree }
        }

        fn filter<F: Fn(&K, &V) -> B>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let len = avl_seq_length(&self.base_seq);
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            let ghost mut result_idx: Seq<int> = Seq::empty();
            while i < len
                invariant
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    len as nat == self.base_seq@.len(),
                    0 <= i <= len,
                    forall|k: &K, v: &V| f.requires((k, v)),
                    forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
                    result_src.len() == all@.len(),
                    result_idx.len() == i as int,
                    all@.len() <= i as int,
                    forall|j: int| 0 <= j < all@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && all@[j]@ == self.base_seq@[result_src[j]]
                        && spec_pred(self.base_seq@[result_src[j]].0, self.base_seq@[result_src[j]].1),
                    forall|m: int| #![trigger result_idx[m]]
                        0 <= m < i
                        && spec_pred(self.base_seq@[m].0, self.base_seq@[m].1)
                        ==> 0 <= result_idx[m] < all@.len()
                        && all@[result_idx[m]]@ == self.base_seq@[m],
                    forall|a: int, b: int| 0 <= a < b < all@.len() ==>
                        (#[trigger] all@[a])@.0 != (#[trigger] all@[b])@.0,
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                if f(&pair.0, &pair.1) {
                    let pair_clone = pair.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*pair, pair_clone);
                        assert(pair_clone@ == self.base_seq@[i as int]);
                        assert(spec_pred(self.base_seq@[i as int].0, self.base_seq@[i as int].1));
                        assert forall|a: int| 0 <= a < all@.len()
                            implies (#[trigger] all@[a])@.0 != self.base_seq@[i as int].0
                        by {
                            let src_a = result_src[a];
                            assert(all@[a]@ == self.base_seq@[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    let ghost new_result_idx = all@.len() as int;
                    all.push(pair_clone);
                    proof {
                        result_src = result_src.push(i as int);
                        result_idx = result_idx.push(new_result_idx);
                    }
                } else {
                    proof {
                        assert(f.ensures((&pair.0, &pair.1), false));
                        assert(!spec_pred(self.base_seq@[i as int].0, self.base_seq@[i as int].1));
                        result_idx = result_idx.push(0int);
                    }
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            proof {
                let tree_seq = tree@;
                assert(tree_seq =~= all@.map_values(|p: Pair<K, V>| p@));
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < tree_seq.len()
                        implies (#[trigger] tree_seq[a]).0 != (#[trigger] tree_seq[b]).0
                    by {
                        assert(tree_seq[a] == all@[a]@);
                        assert(tree_seq[b] == all@[b]@);
                    };
                };
                assert(tree_seq.len() < usize::MAX as nat);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                assert forall|k: K::V| spec_entries_to_map(tree_seq).dom().contains(k)
                    implies self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, k);
                    let j = choose|j: int| 0 <= j < tree_seq.len() && (#[trigger] tree_seq[j]).0 == k;
                    assert(tree_seq[j] == all@[j]@);
                    let m = result_src[j];
                    assert(all@[j]@ == self.base_seq@[m]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, m);
                };
                assert forall|k: K::V| #[trigger] spec_entries_to_map(tree_seq).contains_key(k)
                    implies spec_entries_to_map(tree_seq)[k] == self@[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, k);
                    let j = choose|j: int| 0 <= j < tree_seq.len() && (#[trigger] tree_seq[j]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, j);
                    assert(tree_seq[j] == all@[j]@);
                    let m = result_src[j];
                    assert(all@[j]@ == self.base_seq@[m]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_seq@, m);
                };
                assert forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    implies #[trigger] spec_entries_to_map(tree_seq).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_seq@, k);
                    let m = choose|m: int| 0 <= m < len as int
                        && (#[trigger] self.base_seq@[m]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_seq@, m);
                    assert(spec_pred(self.base_seq@[m].0, self.base_seq@[m].1));
                    let j = result_idx[m];
                    assert(all@[j]@ == self.base_seq@[m]);
                    assert(tree_seq[j] == all@[j]@);
                    assert(tree_seq[j].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, j);
                };
            }
            OrderedTableStEph { base_seq: tree }
        }

        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite()
        {
            let len = avl_seq_length(&self.base_seq);
            let mut reduced = init;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_seq@.len(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    forall|r: R, k: &K, v: &V| f.requires((r, k, v)),
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                reduced = f(reduced, &pair.0, &pair.1);
                i = i + 1;
            }
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@); }
            reduced
        }

        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let ghost old_seq = self.base_seq@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            let len = avl_seq_length(&self.base_seq);
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            let ghost mut result_idx: Seq<int> = Seq::empty();
            let ghost mut result_v1: Seq<V> = Seq::empty();
            let ghost mut result_v2: Seq<V> = Seq::empty();
            let ghost mut result_r: Seq<V> = Seq::empty();
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
            }
            while i < len
                invariant
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    spec_keys_no_dups(old_seq),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    old_map == spec_entries_to_map(old_seq),
                    other_map == other@,
                    len as nat == old_seq.len(),
                    0 <= i <= len,
                    result_src.len() == all@.len(),
                    result_idx.len() == i as int,
                    result_v1.len() == all@.len(),
                    result_v2.len() == all@.len(),
                    result_r.len() == all@.len(),
                    all@.len() <= i as int,
                    forall|j: int| 0 <= j < all@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && all@[j]@.0 == old_seq[result_src[j]].0
                        && other_map.contains_key(old_seq[result_src[j]].0),
                    forall|j: int| 0 <= j < all@.len() ==> {
                        let src = #[trigger] result_src[j];
                        &&& result_v1[j]@ == old_map[old_seq[src].0]
                        &&& result_v2[j]@ == other_map[old_seq[src].0]
                        &&& f.ensures((&result_v1[j], &result_v2[j]), result_r[j])
                        &&& all@[j]@.1 == result_r[j]@
                    },
                    forall|a: int, b: int| 0 <= a < b < all@.len() ==>
                        (#[trigger] all@[a])@.0 != (#[trigger] all@[b])@.0,
                    forall|m: int| #![trigger result_idx[m]]
                        0 <= m < i
                        && other_map.contains_key(old_seq[m].0)
                        ==> 0 <= result_idx[m] < all@.len()
                        && all@[result_idx[m]]@.0 == old_seq[m].0,
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined = f(&pair.1, &other_v);
                        let key_clone = pair.0.clone_plus();
                        proof {
                            assert(obeys_feq_full_trigger::<K>());
                            assert(key_clone@ == pair.0@);
                            assert forall|a: int| 0 <= a < all@.len()
                                implies (#[trigger] all@[a])@.0 != old_seq[i as int].0
                            by {
                                let src_a = result_src[a];
                                assert(all@[a]@.0 == old_seq[src_a].0);
                                assert(src_a != i as int);
                            };
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, i as int);
                            lemma_entries_to_map_get::<K::V, V::V>(old_seq, i as int);
                        }
                        let ghost new_result_idx = all@.len() as int;
                        all.push(Pair(key_clone, combined));
                        proof {
                            result_src = result_src.push(i as int);
                            result_idx = result_idx.push(new_result_idx);
                            result_v1 = result_v1.push(pair.1);
                            result_v2 = result_v2.push(other_v);
                            result_r = result_r.push(combined);
                        }
                    },
                    None => {
                        proof {
                            result_idx = result_idx.push(0int);
                        }
                    },
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            proof {
                let tree_seq = tree@;
                assert(tree_seq =~= all@.map_values(|p: Pair<K, V>| p@));
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < tree_seq.len()
                        implies (#[trigger] tree_seq[a]).0 != (#[trigger] tree_seq[b]).0
                    by {
                        assert(tree_seq[a] == all@[a]@);
                        assert(tree_seq[b] == all@[b]@);
                    };
                };
                assert(tree_seq.len() < usize::MAX as nat);
                lemma_entries_to_map_finite::<K::V, V::V>(old_seq);
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                assert forall|kv: K::V| #[trigger] spec_entries_to_map(tree_seq).dom().contains(kv)
                    <==> old_map.dom().contains(kv) && other_map.dom().contains(kv)
                by {
                    if spec_entries_to_map(tree_seq).dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, kv);
                        let j = choose|j: int| 0 <= j < tree_seq.len()
                            && (#[trigger] tree_seq[j]).0 == kv;
                        assert(tree_seq[j] == all@[j]@);
                        let m = result_src[j];
                        assert(all@[j]@.0 == old_seq[m].0);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, m);
                        assert(other_map.contains_key(old_seq[m].0));
                    }
                    if old_map.dom().contains(kv) && other_map.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, kv);
                        let m = choose|m: int| 0 <= m < len as int
                            && (#[trigger] old_seq[m]).0 == kv;
                        let ri = result_idx[m];
                        assert(all@[ri]@.0 == old_seq[m].0);
                        assert(tree_seq[ri] == all@[ri]@);
                        assert(tree_seq[ri].0 == kv);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, ri);
                    }
                };
                assert(spec_entries_to_map(tree_seq).dom() =~= old_map.dom().intersect(other_map.dom()));
                assert forall|kv: K::V| #[trigger] spec_entries_to_map(tree_seq).contains_key(kv)
                    implies (exists|v1: V, v2: V, r: V|
                        v1@ == old_map[kv] && v2@ == other_map[kv]
                        && f.ensures((&v1, &v2), r)
                        && spec_entries_to_map(tree_seq)[kv] == r@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, kv);
                    let j = choose|j: int| 0 <= j < tree_seq.len()
                        && (#[trigger] tree_seq[j]).0 == kv;
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, j);
                    assert(tree_seq[j] == all@[j]@);
                    let src = result_src[j];
                    assert(old_seq[src].0 == kv);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, src);
                    let v1 = result_v1[j];
                    let v2 = result_v2[j];
                    let r = result_r[j];
                    assert(v1@ == old_map[kv]);
                    assert(v2@ == other_map[kv]);
                    assert(f.ensures((&v1, &v2), r));
                    assert(spec_entries_to_map(tree_seq)[kv] == r@);
                };
            }
        }

        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                lemma_entries_to_map_len::<K::V, V::V>(self.base_seq@);
                lemma_entries_to_map_len::<K::V, V::V>(other.base_seq@);
            }
            let ghost old_seq = self.base_seq@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            // Phase 1: all entries from self (merged with other where keys overlap).
            let self_len = avl_seq_length(&self.base_seq);
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut self_in_other: Seq<bool> = Seq::empty();
            let ghost mut combined_v1: Seq<V> = Seq::empty();
            let ghost mut combined_v2: Seq<V> = Seq::empty();
            let ghost mut combined_r: Seq<V> = Seq::empty();
            while i < self_len
                invariant
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    spec_keys_no_dups(old_seq),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    self_len as nat == old_seq.len(),
                    0 <= i <= self_len,
                    all@.len() == i as int,
                    self_in_other.len() == i as int,
                    combined_v1.len() == i as int,
                    combined_v2.len() == i as int,
                    combined_r.len() == i as int,
                    old_map == spec_entries_to_map(old_seq),
                    other_map == spec_entries_to_map(other.base_seq@),
                    forall|j: int| 0 <= j < all@.len() ==>
                        (#[trigger] all@[j])@.0 == old_seq[j].0,
                    forall|j: int| 0 <= j < all@.len() ==>
                        (#[trigger] self_in_other[j]) == other_map.contains_key(old_seq[j].0),
                    forall|j: int| 0 <= j < all@.len() && !(#[trigger] self_in_other[j]) ==>
                        all@[j]@.1 == old_seq[j].1,
                    forall|j: int| 0 <= j < all@.len() && (#[trigger] self_in_other[j]) ==> {
                        &&& combined_v1[j]@ == old_map[old_seq[j].0]
                        &&& combined_v2[j]@ == other_map[old_seq[j].0]
                        &&& f.ensures((&combined_v1[j], &combined_v2[j]), combined_r[j])
                        &&& all@[j]@.1 == combined_r[j]@
                    },
                    forall|a: int, b: int| 0 <= a < b < all@.len() ==>
                        (#[trigger] all@[a])@.0 != (#[trigger] all@[b])@.0,
                decreases self_len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(ov) => {
                        let combined = f(&pair.1, &ov);
                        let key_clone = pair.0.clone_plus();
                        proof {
                            lemma_cloned_view_eq(pair.0, key_clone);
                            assert forall|a: int| 0 <= a < all@.len()
                                implies (#[trigger] all@[a])@.0 != old_seq[i as int].0
                            by {
                                assert(all@[a]@.0 == old_seq[a].0);
                                assert(a != i as int);
                            };
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, i as int);
                            lemma_entries_to_map_get::<K::V, V::V>(old_seq, i as int);
                        }
                        all.push(Pair(key_clone, combined));
                        proof {
                            self_in_other = self_in_other.push(true);
                            combined_v1 = combined_v1.push(pair.1);
                            combined_v2 = combined_v2.push(ov);
                            combined_r = combined_r.push(combined);
                        }
                    },
                    None => {
                        let cloned = pair.clone_plus();
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            assert forall|a: int| 0 <= a < all@.len()
                                implies (#[trigger] all@[a])@.0 != old_seq[i as int].0
                            by {
                                assert(all@[a]@.0 == old_seq[a].0);
                                assert(a != i as int);
                            };
                        }
                        all.push(cloned);
                        proof {
                            self_in_other = self_in_other.push(false);
                            combined_v1 = combined_v1.push(pair.1);
                            combined_v2 = combined_v2.push(pair.1);
                            combined_r = combined_r.push(pair.1);
                        }
                    },
                }
                i += 1;
            }
            // Phase 2: entries from other not in self.
            let ghost phase1_len = all@.len();
            let other_len = avl_seq_length(&other.base_seq);
            let ghost mut phase2_src: Seq<int> = Seq::empty();
            let ghost mut phase2_idx: Seq<int> = Seq::empty();
            let mut j: usize = 0;
            proof {
                assert forall|j_: int| 0 <= j_ < phase1_len
                    implies (#[trigger] all@[j_])@.0 == old_seq[j_].0
                by {};
            }
            while j < other_len
                invariant
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    other.base_seq.spec_avltreeseqsteph_wf(),
                    spec_keys_no_dups(old_seq),
                    spec_keys_no_dups(other.base_seq@),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    old_map == spec_entries_to_map(old_seq),
                    other_map == spec_entries_to_map(other.base_seq@),
                    self_len as nat == old_seq.len(),
                    other_len as nat == other.base_seq@.len(),
                    0 <= j <= other_len,
                    phase1_len == self_len as int,
                    all@.len() >= phase1_len,
                    all@.len() <= self_len + j,
                    phase2_src.len() == all@.len() - phase1_len,
                    phase2_idx.len() == j as int,
                    forall|j_: int| 0 <= j_ < phase1_len ==>
                        (#[trigger] all@[j_])@.0 == old_seq[j_].0,
                    forall|j_: int| 0 <= j_ < phase1_len ==>
                        (#[trigger] self_in_other[j_]) == other_map.contains_key(old_seq[j_].0),
                    forall|j_: int| 0 <= j_ < phase1_len && !(#[trigger] self_in_other[j_]) ==>
                        all@[j_]@.1 == old_seq[j_].1,
                    forall|j_: int| 0 <= j_ < phase1_len && (#[trigger] self_in_other[j_]) ==> {
                        &&& combined_v1[j_]@ == old_map[old_seq[j_].0]
                        &&& combined_v2[j_]@ == other_map[old_seq[j_].0]
                        &&& f.ensures((&combined_v1[j_], &combined_v2[j_]), combined_r[j_])
                        &&& all@[j_]@.1 == combined_r[j_]@
                    },
                    forall|p: int| 0 <= p < phase2_src.len() ==>
                        0 <= (#[trigger] phase2_src[p]) < j
                        && all@[phase1_len + p]@ == other.base_seq@[phase2_src[p]]
                        && !old_map.contains_key(other.base_seq@[phase2_src[p]].0),
                    forall|m: int| #![trigger phase2_idx[m]]
                        0 <= m < j
                        && !old_map.contains_key(other.base_seq@[m].0)
                        ==> 0 <= phase2_idx[m] < phase2_src.len()
                        && all@[phase1_len + phase2_idx[m]]@ == other.base_seq@[m],
                    forall|a: int, b: int| 0 <= a < b < all@.len() ==>
                        (#[trigger] all@[a])@.0 != (#[trigger] all@[b])@.0,
                decreases other_len - j,
            {
                let pair = avl_seq_nth(&other.base_seq, j);
                let in_self = key_in_other(&self.base_seq, &pair.0);
                if !in_self {
                    let cloned = pair.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        assert forall|a: int| 0 <= a < all@.len()
                            implies (#[trigger] all@[a])@.0 != other.base_seq@[j as int].0
                        by {
                            if a < phase1_len {
                                assert(all@[a]@.0 == old_seq[a].0);
                                lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, a);
                            } else {
                                let p2idx = a - phase1_len;
                                let src = phase2_src[p2idx];
                                assert(all@[a]@ == other.base_seq@[src]);
                                assert(src != j as int);
                            }
                        };
                    }
                    let ghost new_p2_idx = phase2_src.len() as int;
                    all.push(cloned);
                    proof {
                        phase2_src = phase2_src.push(j as int);
                        phase2_idx = phase2_idx.push(new_p2_idx);
                    }
                } else {
                    proof {
                        phase2_idx = phase2_idx.push(0int);
                    }
                }
                j += 1;
            }
            proof {
                assert(all@.len() <= self_len + other_len);
                lemma_entries_to_map_len::<K::V, V::V>(old_seq);
                lemma_entries_to_map_len::<K::V, V::V>(other.base_seq@);
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            proof {
                let tree_seq = tree@;
                let new_map = spec_entries_to_map(tree_seq);
                // Prove no-dups.
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree_seq.len()
                        implies (#[trigger] tree_seq[i_]).0 != (#[trigger] tree_seq[j_]).0
                    by {
                        assert(tree_seq[i_] == all@[i_]@);
                        assert(tree_seq[j_] == all@[j_]@);
                    };
                };
                // Prove wf: tree was built by from_vec, no-dups shown above, size bounded.
                assert(tree_seq.len() == all@.len());
                assert(all@.len() as nat <= old_seq.len() + other.base_seq@.len());
                assert(self.spec_orderedtablesteph_wf());
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                lemma_entries_to_map_finite::<K::V, V::V>(old_seq);
                lemma_entries_to_map_finite::<K::V, V::V>(other.base_seq@);
                // Prove dom =~= old_dom.union(other_dom).
                assert forall|kv: K::V| #[trigger] new_map.dom().contains(kv)
                    <==> old_map.dom().contains(kv) || other_map.dom().contains(kv)
                by {
                    if new_map.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, kv);
                        let idx = choose|idx: int| 0 <= idx < tree_seq.len()
                            && (#[trigger] tree_seq[idx]).0 == kv;
                        assert(tree_seq[idx] == all@[idx]@);
                        if idx < phase1_len {
                            assert(all@[idx]@.0 == old_seq[idx].0);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, idx);
                        } else {
                            let p2idx = idx - phase1_len;
                            let src = phase2_src[p2idx];
                            assert(all@[idx]@ == other.base_seq@[src]);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(
                                other.base_seq@, src,
                            );
                        }
                    }
                    if old_map.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, kv);
                        let m = choose|m: int| 0 <= m < old_seq.len()
                            && (#[trigger] old_seq[m]).0 == kv;
                        assert(all@[m]@.0 == kv);
                        assert(tree_seq[m] == all@[m]@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, m);
                    }
                    if other_map.dom().contains(kv) && !old_map.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(other.base_seq@, kv);
                        let m = choose|m: int| 0 <= m < other.base_seq@.len()
                            && (#[trigger] other.base_seq@[m]).0 == kv;
                        let p2i = phase2_idx[m];
                        assert(all@[phase1_len + p2i]@ == other.base_seq@[m]);
                        assert(tree_seq[phase1_len + p2i] == all@[phase1_len + p2i]@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            tree_seq, phase1_len + p2i,
                        );
                    }
                };
                assert(new_map.dom() =~= old_map.dom().union(other_map.dom()));
                // Value case 1: key in old(self), not in other.
                assert forall|kv: K::V| #[trigger] old_map.contains_key(kv) && !other_map.contains_key(kv)
                    implies new_map[kv] == old_map[kv]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, kv);
                    let m = choose|m: int| 0 <= m < old_seq.len()
                        && (#[trigger] old_seq[m]).0 == kv;
                    assert(!self_in_other[m]);
                    assert(all@[m]@.1 == old_seq[m].1);
                    assert(tree_seq[m] == all@[m]@);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, m);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, m);
                };
                // Value case 2: key in other, not in old(self).
                assert forall|kv: K::V| #[trigger] other_map.contains_key(kv) && !old_map.contains_key(kv)
                    implies new_map[kv] == other_map[kv]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(other.base_seq@, kv);
                    let m = choose|m: int| 0 <= m < other.base_seq@.len()
                        && (#[trigger] other.base_seq@[m]).0 == kv;
                    let p2i = phase2_idx[m];
                    let idx = phase1_len + p2i;
                    assert(all@[idx]@ == other.base_seq@[m]);
                    assert(tree_seq[idx] == all@[idx]@);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(other.base_seq@, m);
                };
                // Value case 3: key in both.
                assert forall|kv: K::V| #[trigger] old_map.contains_key(kv) && other_map.contains_key(kv)
                    implies (exists|v1: V, v2: V, r: V|
                        v1@ == old_map[kv] && v2@ == other_map[kv]
                        && f.ensures((&v1, &v2), r)
                        && new_map[kv] == r@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, kv);
                    let m = choose|m: int| 0 <= m < old_seq.len()
                        && (#[trigger] old_seq[m]).0 == kv;
                    assert(self_in_other[m]);
                    assert(all@[m]@.0 == kv);
                    assert(tree_seq[m] == all@[m]@);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, m);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, m);
                    let v1 = combined_v1[m];
                    let v2 = combined_v2[m];
                    let r = combined_r[m];
                    assert(v1@ == old_map[kv]);
                    assert(v2@ == other_map[kv]);
                    assert(f.ensures((&v1, &v2), r));
                    assert(new_map[kv] == r@);
                };
            }
        }

        fn difference(&mut self, other: &Self)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let ghost old_seq = self.base_seq@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            let len = avl_seq_length(&self.base_seq);
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let ghost mut kept: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == old_seq.len(),
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    other.base_seq.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    obeys_view_eq::<K>(),
                    spec_keys_no_dups(old_seq),
                    other_map == spec_entries_to_map(other.base_seq@),
                    all@.len() == kept.len(),
                    kept.len() <= i as int,
                    forall|p: int| 0 <= p < kept.len() ==> 0 <= #[trigger] kept[p] < i as int,
                    forall|p: int, q: int| 0 <= p < q < kept.len()
                        ==> #[trigger] kept[p] < #[trigger] kept[q],
                    forall|p: int| 0 <= p < all@.len()
                        ==> (#[trigger] all@[p])@ == old_seq[kept[p]],
                    // Key at kept[p] is not in other's domain.
                    forall|p: int| 0 <= p < kept.len()
                        ==> !other_map.dom().contains(old_seq[#[trigger] kept[p]].0),
                    // Completeness: every source index < i not in other was kept.
                    forall|q: int| 0 <= q < i as int
                        && !other_map.dom().contains(old_seq[q].0)
                        ==> #[trigger] kept.contains(q),
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                let in_other = key_in_other(&other.base_seq, &pair.0);
                if !in_other {
                    let cloned = pair.clone_plus();
                    proof { lemma_cloned_view_eq(*pair, cloned); }
                    all.push(cloned);
                    proof {
                        let old_kept = kept;
                        let ghost new_idx = old_kept.len() as int;
                        kept = kept.push(i as int);
                        assert(0 <= new_idx < kept.len());
                        assert(kept[new_idx] == i as int);
                        // Old witnesses still valid: kept is an extension of old_kept.
                        assert forall|q: int| 0 <= q < (i + 1) as int
                            && !other_map.dom().contains(old_seq[q].0)
                            implies #[trigger] kept.contains(q)
                        by {
                            if q == i as int {
                                assert(kept[new_idx] == q);
                            } else {
                                assert(old_kept.contains(q));
                                let witness = choose|w: int| 0 <= w < old_kept.len() && old_kept[w] == q;
                                assert(kept[witness] == q);
                            }
                        };
                    }
                } else {
                    proof {
                        assert(other_map.dom().contains(old_seq[i as int].0));
                    }
                }
                i = i + 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            proof {
                let tree_seq = tree@;
                let new_map = spec_entries_to_map(tree_seq);
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree_seq.len()
                        implies (#[trigger] tree_seq[i_]).0 != (#[trigger] tree_seq[j_]).0
                    by {
                        assert(tree_seq[i_] == old_seq[kept[i_]]);
                        assert(tree_seq[j_] == old_seq[kept[j_]]);
                        assert(kept[i_] < kept[j_]);
                    };
                };
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                assert forall|k: K::V| #[trigger] new_map.contains_key(k)
                    implies old_map.dom().contains(k) && !other_map.dom().contains(k) && new_map[k] == old_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, k);
                    let idx = choose|idx: int| 0 <= idx < tree_seq.len() && (#[trigger] tree_seq[idx]).0 == k;
                    assert(tree_seq[idx] == old_seq[kept[idx]]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, kept[idx]);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, kept[idx]);
                };
                assert forall|k: K::V|
                    old_map.dom().contains(k) && !other_map.dom().contains(k)
                    implies #[trigger] new_map.contains_key(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, k);
                    let src_idx = choose|idx: int| 0 <= idx < old_seq.len() && (#[trigger] old_seq[idx]).0 == k;
                    assert(kept.contains(src_idx));
                    let p = choose|p: int| 0 <= p < kept.len() && kept[p] == src_idx;
                    assert(tree_seq[p].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, p);
                };
                assert(new_map.dom() =~= old_map.dom().difference(other_map.dom()));
            }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let ghost old_seq = self.base_seq@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let len = avl_seq_length(&self.base_seq);
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let ghost mut kept: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == old_seq.len(),
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    spec_keys_no_dups(old_seq),
                    keys@ == keys_set,
                    keys@.finite(),
                    all@.len() == kept.len(),
                    kept.len() <= i as int,
                    forall|p: int| 0 <= p < kept.len() ==> 0 <= #[trigger] kept[p] < i as int,
                    forall|p: int, q: int| 0 <= p < q < kept.len()
                        ==> #[trigger] kept[p] < #[trigger] kept[q],
                    forall|p: int| 0 <= p < all@.len()
                        ==> (#[trigger] all@[p])@ == old_seq[kept[p]],
                    forall|p: int| 0 <= p < kept.len()
                        ==> keys_set.contains(old_seq[#[trigger] kept[p]].0),
                    forall|q: int| 0 <= q < i as int
                        && keys_set.contains(old_seq[q].0)
                        ==> #[trigger] kept.contains(q),
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                let in_keys = keys.find(&pair.0);
                if in_keys {
                    let cloned = pair.clone_plus();
                    proof { lemma_cloned_view_eq(*pair, cloned); }
                    all.push(cloned);
                    proof {
                        let old_kept = kept;
                        let ghost new_idx = old_kept.len() as int;
                        kept = kept.push(i as int);
                        assert(0 <= new_idx < kept.len());
                        assert(kept[new_idx] == i as int);
                        assert forall|q: int| 0 <= q < (i + 1) as int
                            && keys_set.contains(old_seq[q].0)
                            implies #[trigger] kept.contains(q)
                        by {
                            if q == i as int {
                                assert(kept[new_idx] == q);
                            } else {
                                assert(old_kept.contains(q));
                                let witness = choose|w: int| 0 <= w < old_kept.len() && old_kept[w] == q;
                                assert(kept[witness] == q);
                            }
                        };
                    }
                } else {
                    proof {
                        assert(!keys_set.contains(old_seq[i as int].0));
                    }
                }
                i = i + 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            proof {
                let tree_seq = tree@;
                let new_map = spec_entries_to_map(tree_seq);
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree_seq.len()
                        implies (#[trigger] tree_seq[i_]).0 != (#[trigger] tree_seq[j_]).0
                    by {
                        assert(tree_seq[i_] == old_seq[kept[i_]]);
                        assert(tree_seq[j_] == old_seq[kept[j_]]);
                        assert(kept[i_] < kept[j_]);
                    };
                };
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                assert forall|k: K::V| #[trigger] new_map.contains_key(k)
                    implies old_map.dom().contains(k) && keys_set.contains(k) && new_map[k] == old_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, k);
                    let idx = choose|idx: int| 0 <= idx < tree_seq.len() && (#[trigger] tree_seq[idx]).0 == k;
                    assert(tree_seq[idx] == old_seq[kept[idx]]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, kept[idx]);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, kept[idx]);
                };
                assert forall|k: K::V|
                    old_map.dom().contains(k) && keys_set.contains(k)
                    implies #[trigger] new_map.contains_key(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, k);
                    let src_idx = choose|idx: int| 0 <= idx < old_seq.len() && (#[trigger] old_seq[idx]).0 == k;
                    assert(kept.contains(src_idx));
                    let p = choose|p: int| 0 <= p < kept.len() && kept[p] == src_idx;
                    assert(tree_seq[p].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, p);
                };
                assert(new_map.dom() =~= old_map.dom().intersect(keys_set));
            }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let ghost old_seq = self.base_seq@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let len = avl_seq_length(&self.base_seq);
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let ghost mut kept: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == old_seq.len(),
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    spec_keys_no_dups(old_seq),
                    keys@ == keys_set,
                    keys@.finite(),
                    all@.len() == kept.len(),
                    kept.len() <= i as int,
                    forall|p: int| 0 <= p < kept.len() ==> 0 <= #[trigger] kept[p] < i as int,
                    forall|p: int, q: int| 0 <= p < q < kept.len()
                        ==> #[trigger] kept[p] < #[trigger] kept[q],
                    forall|p: int| 0 <= p < all@.len()
                        ==> (#[trigger] all@[p])@ == old_seq[kept[p]],
                    forall|p: int| 0 <= p < kept.len()
                        ==> !keys_set.contains(old_seq[#[trigger] kept[p]].0),
                    forall|q: int| 0 <= q < i as int
                        && !keys_set.contains(old_seq[q].0)
                        ==> #[trigger] kept.contains(q),
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                let in_keys = keys.find(&pair.0);
                if !in_keys {
                    let cloned = pair.clone_plus();
                    proof { lemma_cloned_view_eq(*pair, cloned); }
                    all.push(cloned);
                    proof {
                        let old_kept = kept;
                        let ghost new_idx = old_kept.len() as int;
                        kept = kept.push(i as int);
                        assert(0 <= new_idx < kept.len());
                        assert(kept[new_idx] == i as int);
                        assert forall|q: int| 0 <= q < (i + 1) as int
                            && !keys_set.contains(old_seq[q].0)
                            implies #[trigger] kept.contains(q)
                        by {
                            if q == i as int {
                                assert(kept[new_idx] == q);
                            } else {
                                assert(old_kept.contains(q));
                                let witness = choose|w: int| 0 <= w < old_kept.len() && old_kept[w] == q;
                                assert(kept[witness] == q);
                            }
                        };
                    }
                } else {
                    proof {
                        assert(keys_set.contains(old_seq[i as int].0));
                    }
                }
                i = i + 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            proof {
                let tree_seq = tree@;
                let new_map = spec_entries_to_map(tree_seq);
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|i_: int, j_: int|
                        0 <= i_ < j_ < tree_seq.len()
                        implies (#[trigger] tree_seq[i_]).0 != (#[trigger] tree_seq[j_]).0
                    by {
                        assert(tree_seq[i_] == old_seq[kept[i_]]);
                        assert(tree_seq[j_] == old_seq[kept[j_]]);
                        assert(kept[i_] < kept[j_]);
                    };
                };
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                assert forall|k: K::V| #[trigger] new_map.contains_key(k)
                    implies old_map.dom().contains(k) && !keys_set.contains(k) && new_map[k] == old_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, k);
                    let idx = choose|idx: int| 0 <= idx < tree_seq.len() && (#[trigger] tree_seq[idx]).0 == k;
                    assert(tree_seq[idx] == old_seq[kept[idx]]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, kept[idx]);
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, kept[idx]);
                };
                assert forall|k: K::V|
                    old_map.dom().contains(k) && !keys_set.contains(k)
                    implies #[trigger] new_map.contains_key(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, k);
                    let src_idx = choose|idx: int| 0 <= idx < old_seq.len() && (#[trigger] old_seq[idx]).0 == k;
                    assert(kept.contains(src_idx));
                    let p = choose|p: int| 0 <= p < kept.len() && kept[p] == src_idx;
                    assert(tree_seq[p].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(tree_seq, p);
                };
                assert(new_map.dom() =~= old_map.dom().difference(keys_set));
            }
        }

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures
                self@.dom().finite(),
                collected.spec_avltreeseqstper_wf(),
                collected@.len() == self@.dom().len(),
                forall|i: int| 0 <= i < collected@.len() ==> self@.dom().contains((#[trigger] collected@[i]).0),
                self.spec_orderedtablesteph_wf() ==> spec_entries_to_map(collected@) =~= self@,
                self.spec_orderedtablesteph_wf() ==> spec_keys_no_dups(collected@),
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let len = avl_seq_length(&self.base_seq);
            let mut elements: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_seq@.len(),
                    self.spec_orderedtablesteph_wf(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    elements@.len() == i as int,
                    forall|j: int| 0 <= j < i ==> (#[trigger] elements@[j])@ == self.base_seq@[j],
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                let cloned = pair.clone_plus();
                proof { lemma_cloned_view_eq(*pair, cloned); }
                elements.push(cloned);
                i = i + 1;
            }
            let result = AVLTreeSeqStPerS::from_vec(elements);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
                lemma_entries_to_map_len::<K::V, V::V>(self.base_seq@);
                self.base_seq@.unique_seq_to_set();
                // result@ =~= elements@.map_values(|p| p@) =~= self.base_seq@
                assert(result@ =~= self.base_seq@) by {
                    assert forall|j: int| 0 <= j < result@.len()
                        implies #[trigger] result@[j] == self.base_seq@[j]
                    by {};
                };
                // Each entry's key is in the map domain.
                assert forall|i_: int| 0 <= i_ < result@.len()
                    implies self@.dom().contains((#[trigger] result@[i_]).0)
                by {
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, i_);
                };
            }
            result
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
            let len = avl_seq_length(&self.base_seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
            }
            if len == 0 {
                proof { assert(self.base_seq@ =~= Seq::<(K::V, V::V)>::empty()); }
                None
            } else {
                let ghost vals = spec_inorder_values::<Pair<K, V>>(self.base_seq.root);
                proof { lemma_inorder_values_maps_to_views::<Pair<K, V>>(self.base_seq.root); }
                let first_pair = avl_seq_nth(&self.base_seq, 0);
                let mut min_key = first_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(vals[0int].0, min_key);
                    K::reflexive(min_key);
                }
                let ghost mut min_idx: int = 0;
                let mut i: usize = 1;
                while i < len
                    invariant
                        obeys_feq_full::<K>(),
                        self.base_seq.spec_avltreeseqsteph_wf(),
                        1 <= i, i <= len,
                        len as nat == self.base_seq@.len(),
                        0 <= min_idx, min_idx < i,
                        vals == spec_inorder_values::<Pair<K, V>>(self.base_seq.root),
                        vals.len() == self.base_seq@.len(),
                        vals.map_values(|p: Pair<K, V>| p@) =~= self.base_seq@,
                        min_key@ == self.base_seq@[min_idx].0,
                        min_key == vals[min_idx].0,
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i ==> TotalOrder::le(min_key, vals[j].0),
                    decreases len - i,
                {
                    let elem_pair = avl_seq_nth(&self.base_seq, i);
                    proof {
                        assert(elem_pair.0@ == vals[i as int].0@);
                        assert(elem_pair.0 == vals[i as int].0);
                    }
                    let c = TotalOrder::cmp(&elem_pair.0, &min_key);
                    match c {
                        core::cmp::Ordering::Less => {
                            let ghost old_min = min_key;
                            min_key = elem_pair.0.clone_plus();
                            proof {
                                lemma_cloned_view_eq(vals[i as int].0, min_key);
                                min_idx = i as int;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies TotalOrder::le(min_key, #[trigger] vals[j].0) by {
                                    if j == i as int {
                                        K::reflexive(min_key);
                                    } else {
                                        K::transitive(min_key, old_min, vals[j].0);
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
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, min_idx);
                    assert(self@.dom().contains(min_key@));
                    assert forall|t: K| #[trigger] self@.dom().contains(t@)
                        implies TotalOrder::le(min_key, t) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_seq@, t@);
                        let j = choose|j: int| 0 <= j < self.base_seq@.len()
                            && (#[trigger] self.base_seq@[j]).0 == t@;
                        assert(vals[j].0@ == t@);
                        assert(vals[j].0 == t);
                    };
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, 0);
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
            let len = avl_seq_length(&self.base_seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
            }
            if len == 0 {
                proof { assert(self.base_seq@ =~= Seq::<(K::V, V::V)>::empty()); }
                None
            } else {
                let ghost vals = spec_inorder_values::<Pair<K, V>>(self.base_seq.root);
                proof { lemma_inorder_values_maps_to_views::<Pair<K, V>>(self.base_seq.root); }
                let first_pair = avl_seq_nth(&self.base_seq, 0);
                let mut max_key = first_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(vals[0int].0, max_key);
                    K::reflexive(max_key);
                }
                let ghost mut max_idx: int = 0;
                let mut i: usize = 1;
                while i < len
                    invariant
                        obeys_feq_full::<K>(),
                        self.base_seq.spec_avltreeseqsteph_wf(),
                        1 <= i, i <= len,
                        len as nat == self.base_seq@.len(),
                        0 <= max_idx, max_idx < i,
                        vals == spec_inorder_values::<Pair<K, V>>(self.base_seq.root),
                        vals.len() == self.base_seq@.len(),
                        vals.map_values(|p: Pair<K, V>| p@) =~= self.base_seq@,
                        max_key@ == self.base_seq@[max_idx].0,
                        max_key == vals[max_idx].0,
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i ==> TotalOrder::le(vals[j].0, max_key),
                    decreases len - i,
                {
                    let elem_pair = avl_seq_nth(&self.base_seq, i);
                    proof {
                        assert(elem_pair.0@ == vals[i as int].0@);
                        assert(elem_pair.0 == vals[i as int].0);
                    }
                    let c = TotalOrder::cmp(&elem_pair.0, &max_key);
                    match c {
                        core::cmp::Ordering::Greater => {
                            let ghost old_max = max_key;
                            max_key = elem_pair.0.clone_plus();
                            proof {
                                lemma_cloned_view_eq(vals[i as int].0, max_key);
                                max_idx = i as int;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies TotalOrder::le(#[trigger] vals[j].0, max_key) by {
                                    if j == i as int {
                                        K::reflexive(max_key);
                                    } else {
                                        K::transitive(vals[j].0, old_max, max_key);
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
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, max_idx);
                    assert(self@.dom().contains(max_key@));
                    assert forall|t: K| #[trigger] self@.dom().contains(t@)
                        implies TotalOrder::le(t, max_key) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_seq@, t@);
                        let j = choose|j: int| 0 <= j < self.base_seq@.len()
                            && (#[trigger] self.base_seq@[j]).0 == t@;
                        assert(vals[j].0@ == t@);
                        assert(vals[j].0 == t);
                    };
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, 0);
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
            let len = avl_seq_length(&self.base_seq);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@); }
            let ghost vals = spec_inorder_values::<Pair<K, V>>(self.base_seq.root);
            proof { lemma_inorder_values_maps_to_views::<Pair<K, V>>(self.base_seq.root); }
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    obeys_feq_full::<K>(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    0 <= i, i <= len,
                    len as nat == self.base_seq@.len(),
                    vals == spec_inorder_values::<Pair<K, V>>(self.base_seq.root),
                    vals.len() == self.base_seq@.len(),
                    vals.map_values(|p: Pair<K, V>| p@) =~= self.base_seq@,
                    !found ==> forall|j: int| #![trigger vals[j]]
                        0 <= j < i ==> !(TotalOrder::le(vals[j].0, *k) && vals[j].0@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(vals[best_idx].0, *k) && vals[best_idx].0@ != k@ &&
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i && TotalOrder::le(vals[j].0, *k) && vals[j].0@ != k@
                            ==> TotalOrder::le(vals[j].0, vals[best_idx].0)
                    ),
                decreases len - i,
            {
                let elem_pair = avl_seq_nth(&self.base_seq, i);
                proof {
                    assert(elem_pair.0@ == vals[i as int].0@);
                    assert(elem_pair.0 == vals[i as int].0);
                }
                let c = TotalOrder::cmp(&elem_pair.0, k);
                match c {
                    core::cmp::Ordering::Less => {
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                K::reflexive(vals[i as int].0);
                            }
                        } else {
                            let best_pair = avl_seq_nth(&self.base_seq, best_pos);
                            let c2 = TotalOrder::cmp(&elem_pair.0, &best_pair.0);
                            match c2 {
                                core::cmp::Ordering::Greater => {
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(vals[j].0, *k) && vals[j].0@ != k@
                                            implies TotalOrder::le(#[trigger] vals[j].0, vals[best_idx].0) by {
                                            if j == i as int {
                                                K::reflexive(vals[i as int].0);
                                            } else {
                                                K::transitive(vals[j].0, vals[old_best].0, vals[i as int].0);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        K::total(vals[i as int].0, vals[best_idx].0);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                    },
                    core::cmp::Ordering::Greater => {
                        proof {
                            if TotalOrder::le(vals[i as int].0, *k) {
                                K::antisymmetric(vals[i as int].0, *k);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            if !found {
                None
            } else {
                let result_pair = avl_seq_nth(&self.base_seq, best_pos);
                let result = result_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(vals[best_idx].0, result);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, best_idx);
                    assert forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@
                        implies TotalOrder::le(t, result) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_seq@, t@);
                        let j = choose|j: int| 0 <= j < self.base_seq@.len()
                            && (#[trigger] self.base_seq@[j]).0 == t@;
                        assert(vals[j].0@ == t@);
                        assert(vals[j].0 == t);
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
            let len = avl_seq_length(&self.base_seq);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@); }
            let ghost vals = spec_inorder_values::<Pair<K, V>>(self.base_seq.root);
            proof { lemma_inorder_values_maps_to_views::<Pair<K, V>>(self.base_seq.root); }
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    obeys_feq_full::<K>(),
                    self.base_seq.spec_avltreeseqsteph_wf(),
                    0 <= i, i <= len,
                    len as nat == self.base_seq@.len(),
                    vals == spec_inorder_values::<Pair<K, V>>(self.base_seq.root),
                    vals.len() == self.base_seq@.len(),
                    vals.map_values(|p: Pair<K, V>| p@) =~= self.base_seq@,
                    !found ==> forall|j: int| #![trigger vals[j]]
                        0 <= j < i ==> !(TotalOrder::le(*k, vals[j].0) && vals[j].0@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(*k, vals[best_idx].0) && vals[best_idx].0@ != k@ &&
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i && TotalOrder::le(*k, vals[j].0) && vals[j].0@ != k@
                            ==> TotalOrder::le(vals[best_idx].0, vals[j].0)
                    ),
                decreases len - i,
            {
                let elem_pair = avl_seq_nth(&self.base_seq, i);
                proof {
                    assert(elem_pair.0@ == vals[i as int].0@);
                    assert(elem_pair.0 == vals[i as int].0);
                }
                let c = TotalOrder::cmp(&elem_pair.0, k);
                match c {
                    core::cmp::Ordering::Greater => {
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                K::reflexive(vals[i as int].0);
                            }
                        } else {
                            let best_pair = avl_seq_nth(&self.base_seq, best_pos);
                            let c2 = TotalOrder::cmp(&elem_pair.0, &best_pair.0);
                            match c2 {
                                core::cmp::Ordering::Less => {
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(*k, (#[trigger] vals[j]).0) && vals[j].0@ != k@
                                            implies TotalOrder::le(vals[best_idx].0, vals[j].0) by {
                                            if j == i as int {
                                                K::reflexive(vals[i as int].0);
                                            } else {
                                                K::transitive(vals[i as int].0, vals[old_best].0, vals[j].0);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        K::total(vals[best_idx].0, vals[i as int].0);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                    },
                    core::cmp::Ordering::Less => {
                        proof {
                            if TotalOrder::le(*k, vals[i as int].0) {
                                K::antisymmetric(*k, vals[i as int].0);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            if !found {
                None
            } else {
                let result_pair = avl_seq_nth(&self.base_seq, best_pos);
                let result = result_pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq(vals[best_idx].0, result);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, best_idx);
                    assert forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@
                        implies TotalOrder::le(result, t) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_seq@, t@);
                        let j = choose|j: int| 0 <= j < self.base_seq@.len()
                            && (#[trigger] self.base_seq@[j]).0 == t@;
                        assert(vals[j].0@ == t@);
                        assert(vals[j].0 == t);
                    };
                }
                Some(result)
            }
        }

        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let ghost old_seq = self.base_seq@;
            let ghost old_map = self@;
            let len = avl_seq_length(&self.base_seq);
            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let mut i: usize = 0;
            let ghost mut left_src: Seq<int> = Seq::empty();
            let ghost mut left_idx: Seq<int> = Seq::empty();
            let ghost mut found: bool = false;
            let ghost mut found_idx: int = 0;
            proof {
                assert(obeys_feq_full_trigger::<V>());
            }
            while i < len
                invariant
                    self.base_seq@ == old_seq,
                    old(self).base_seq@ == old_seq,
                    old(self).spec_orderedtablesteph_wf(),
                    spec_keys_no_dups(old_seq),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    old_map == spec_entries_to_map(old_seq),
                    len as nat == old_seq.len(),
                    0 <= i <= len,
                    left_src.len() == left_entries@.len(),
                    left_idx.len() == i as int,
                    left_entries@.len() <= i as int,
                    forall|j: int| 0 <= j < left_entries@.len() ==>
                        0 <= (#[trigger] left_src[j]) < i
                        && left_entries@[j]@ == old_seq[left_src[j]]
                        && old_seq[left_src[j]].0 != k@,
                    forall|a: int, b: int| 0 <= a < b < left_entries@.len() ==>
                        (#[trigger] left_entries@[a])@.0 != (#[trigger] left_entries@[b])@.0,
                    forall|m: int| #![trigger left_idx[m]]
                        0 <= m < i
                        && old_seq[m].0 != k@
                        ==> 0 <= left_idx[m] < left_entries@.len()
                        && left_entries@[left_idx[m]]@ == old_seq[m],
                    found_value is Some <==> found,
                    found ==> 0 <= found_idx < i as int,
                    found ==> old_seq[found_idx].0 == k@,
                    found ==> found_value->Some_0@ == old_seq[found_idx].1,
                    !found ==> forall|m: int| 0 <= m < i as int ==>
                        (#[trigger] old_seq[m]).0 != k@,
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == *k {
                    let v_clone = pair.1.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<V>());
                        assert(v_clone@ == pair.1@);
                        found = true;
                        found_idx = i as int;
                        left_idx = left_idx.push(0int);
                    }
                    found_value = Some(v_clone);
                } else {
                    let pair_clone = pair.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*pair, pair_clone);
                        assert(pair_clone@ == old_seq[i as int]);
                        assert forall|a: int| 0 <= a < left_entries@.len()
                            implies (#[trigger] left_entries@[a])@.0 != old_seq[i as int].0
                        by {
                            let src_a = left_src[a];
                            assert(left_entries@[a]@ == old_seq[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    let ghost new_left_idx = left_entries@.len() as int;
                    left_entries.push(pair_clone);
                    proof {
                        left_src = left_src.push(i as int);
                        left_idx = left_idx.push(new_left_idx);
                    }
                }
                i += 1;
            }
            let left_tree = AVLTreeSeqStEphS::from_vec(left_entries);
            let left_table = OrderedTableStEph { base_seq: left_tree };
            let right_tree = AVLTreeSeqStEphS::<Pair<K, V>>::empty();
            let right_table = OrderedTableStEph { base_seq: right_tree };
            *self = Self::empty();
            proof {
                // right_table wf: empty tree is wf, empty seq has no dups and len 0.
                assert(spec_keys_no_dups(right_tree@)) by {
                    assert(right_tree@ =~= Seq::<(K::V, V::V)>::empty());
                };
                let left_seq = left_tree@;
                assert(left_seq =~= left_entries@.map_values(|p: Pair<K, V>| p@));
                assert(spec_keys_no_dups(left_seq)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < left_seq.len()
                        implies (#[trigger] left_seq[a]).0 != (#[trigger] left_seq[b]).0
                    by {
                        assert(left_seq[a] == left_entries@[a]@);
                        assert(left_seq[b] == left_entries@[b]@);
                    };
                };
                assert(left_seq.len() < usize::MAX as nat);
                lemma_entries_to_map_finite::<K::V, V::V>(old_seq);
                lemma_entries_to_map_finite::<K::V, V::V>(left_seq);
                // Found value postconditions.
                if found {
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, found_idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_seq, found_idx);
                } else {
                    lemma_entries_to_map_no_key::<K::V, V::V>(old_seq, k@);
                }
                // !left_table@.dom().contains(k@).
                assert(!spec_entries_to_map(left_seq).dom().contains(k@)) by {
                    if spec_entries_to_map(left_seq).dom().contains(k@) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq, k@);
                        let j = choose|j: int| 0 <= j < left_seq.len()
                            && (#[trigger] left_seq[j]).0 == k@;
                        assert(left_seq[j] == left_entries@[j]@);
                        let src_j = left_src[j];
                        assert(left_entries@[j]@ == old_seq[src_j]);
                        assert(old_seq[src_j].0 != k@);
                    }
                };
                // left_table@.dom().subset_of(old_map.dom()).
                assert forall|key: K::V| #![trigger old_map.dom().contains(key)]
                    spec_entries_to_map(left_seq).dom().contains(key)
                    implies old_map.dom().contains(key)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq, key);
                    let j = choose|j: int| 0 <= j < left_seq.len()
                        && (#[trigger] left_seq[j]).0 == key;
                    assert(left_seq[j] == left_entries@[j]@);
                    let m = left_src[j];
                    assert(left_entries@[j]@ == old_seq[m]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_seq, m);
                };
                // Completeness.
                assert forall|key: K::V| #[trigger] old_map.dom().contains(key)
                    implies spec_entries_to_map(left_seq).dom().contains(key)
                        || right_table@.dom().contains(key) || key == k@
                by {
                    if key != k@ {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_seq, key);
                        let m = choose|m: int| 0 <= m < len as int
                            && (#[trigger] old_seq[m]).0 == key;
                        let li = left_idx[m];
                        assert(left_entries@[li]@ == old_seq[m]);
                        assert(left_seq[li] == left_entries@[li]@);
                        assert(left_seq[li].0 == key);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(left_seq, li);
                    }
                };
                assert(right_table.spec_orderedtablesteph_wf());
            }
            (left_table, found_value, right_table)
        }

        fn join_key(&mut self, other: Self)
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let len = avl_seq_length(&self.base_seq);
            let mut range_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            while i < len
                invariant
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    len as nat == self.base_seq@.len(),
                    0 <= i <= len,
                    result_src.len() == range_entries@.len(),
                    range_entries@.len() <= i as int,
                    forall|j: int| 0 <= j < range_entries@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && range_entries@[j]@ == self.base_seq@[result_src[j]],
                    forall|a: int, b: int| 0 <= a < b < range_entries@.len() ==>
                        (#[trigger] range_entries@[a])@.0 != (#[trigger] range_entries@[b])@.0,
                decreases len - i,
            {
                let pair = avl_seq_nth(&self.base_seq, i);
                let ge_k1 = match pair.0.cmp(k1) {
                    std::cmp::Ordering::Less => false,
                    _ => true,
                };
                let le_k2 = match pair.0.cmp(k2) {
                    std::cmp::Ordering::Greater => false,
                    _ => true,
                };
                if ge_k1 && le_k2 {
                    let pair_clone = pair.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*pair, pair_clone);
                        assert(pair_clone@ == self.base_seq@[i as int]);
                        assert forall|a: int| 0 <= a < range_entries@.len()
                            implies (#[trigger] range_entries@[a])@.0 != self.base_seq@[i as int].0
                        by {
                            let src_a = result_src[a];
                            assert(range_entries@[a]@ == self.base_seq@[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    range_entries.push(pair_clone);
                    proof {
                        result_src = result_src.push(i as int);
                    }
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(range_entries);
            proof {
                let tree_seq = tree@;
                assert(tree_seq =~= range_entries@.map_values(|p: Pair<K, V>| p@));
                assert(spec_keys_no_dups(tree_seq)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < tree_seq.len()
                        implies (#[trigger] tree_seq[a]).0 != (#[trigger] tree_seq[b]).0
                    by {
                        assert(tree_seq[a] == range_entries@[a]@);
                        assert(tree_seq[b] == range_entries@[b]@);
                    };
                };
                assert(tree_seq.len() < usize::MAX as nat);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
                lemma_entries_to_map_finite::<K::V, V::V>(tree_seq);
                assert forall|k: K::V| spec_entries_to_map(tree_seq).dom().contains(k)
                    implies self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, k);
                    let j = choose|j: int| 0 <= j < tree_seq.len() && (#[trigger] tree_seq[j]).0 == k;
                    assert(tree_seq[j] == range_entries@[j]@);
                    let m = result_src[j];
                    assert(range_entries@[j]@ == self.base_seq@[m]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, m);
                };
                assert forall|key: K::V| #[trigger] spec_entries_to_map(tree_seq).dom().contains(key)
                    implies spec_entries_to_map(tree_seq)[key] == self@[key]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(tree_seq, key);
                    let j = choose|j: int| 0 <= j < tree_seq.len() && (#[trigger] tree_seq[j]).0 == key;
                    lemma_entries_to_map_get::<K::V, V::V>(tree_seq, j);
                    assert(tree_seq[j] == range_entries@[j]@);
                    let m = result_src[j];
                    assert(range_entries@[j]@ == self.base_seq@[m]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_seq@, m);
                };
            }
            OrderedTableStEph { base_seq: tree }
        }

        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                // obeys_feq_full::<K>() now holds, providing obeys_feq_view_injective::<K>():
                // forall|x: K, y: K| x@ == y@ ==> x == y
                // Contrapositive: x != y ==> x@ != y@
            }
            let len = self.base_seq.length();
            let ghost seq = self.base_seq@;
            let ghost dom = self@.dom();
            let ghost pred = |x: K::V| -> bool
                { exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@ };
            let mut count: usize = 0;
            let ghost mut counted: Set<K::V> = Set::empty();
            let mut i: usize = 0;
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full::<K>());
                lemma_entries_to_map_finite::<K::V, V::V>(seq);
                assert(counted.finite());
                assert(counted.len() == 0);
            }
            while i < len
                invariant
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<K>(),
                    len as nat == seq.len(),
                    seq == self.base_seq@,
                    dom == self@.dom(),
                    spec_keys_no_dups(seq),
                    0 <= i <= len,
                    0 <= count <= i,
                    count as int == counted.len(),
                    counted.finite(),
                    forall|x: K::V| #[trigger] counted.contains(x)
                        ==> spec_rank_pred::<K>(x, *k),
                    forall|x: K::V| #[trigger] counted.contains(x) ==> dom.contains(x),
                    forall|j: int| 0 <= j < i as int && spec_rank_pred::<K>(seq[j].0, *k)
                        ==> #[trigger] counted.contains(seq[j].0),
                    forall|x: K::V| counted.contains(x) ==>
                        exists|j: int| 0 <= j < i as int && #[trigger] seq[j].0 == x,
                decreases len - i,
            {
                let pair = self.base_seq.nth(i);
                let c = TotalOrder::cmp(&pair.0, k);
                match c {
                    core::cmp::Ordering::Less => {
                        // cmp Less: le(pair.0, *k) && pair.0 != *k.
                        // view_injective (from obeys_feq_full): pair.0@ == k@ ==> pair.0 == *k.
                        // Contrapositive: pair.0 != *k ==> pair.0@ != k@.
                        proof {
                            assert(pair.0@ != k@);
                            assert(TotalOrder::le(pair.0, *k));
                            assert(spec_rank_pred::<K>(pair.0@, *k));
                            lemma_entries_to_map_contains_key::<K::V, V::V>(seq, i as int);
                            if counted.contains(pair.0@) {
                                let witness = choose|j: int| 0 <= j < i as int && seq[j].0 == pair.0@;
                                assert(seq[witness].0 == seq[i as int].0);
                            }
                            assert(!counted.contains(pair.0@));
                            counted = counted.insert(pair.0@);
                        }
                        count = count + 1;
                    },
                    _ => {
                        // cmp Equal or Greater. Need !spec_rank_pred(seq[i].0, *k).
                        proof {
                            if spec_rank_pred::<K>(seq[i as int].0, *k) {
                                let t_wit: K = choose|t: K| #![trigger t@]
                                    t@ == seq[i as int].0 && TotalOrder::le(t, *k) && t@ != k@;
                                // t_wit@ == pair.0@. view_injective: t_wit == pair.0.
                                assert(t_wit == pair.0);
                                assert(TotalOrder::le(pair.0, *k));
                                match c {
                                    core::cmp::Ordering::Equal => {
                                        // pair.0 == *k, so pair.0@ == k@ by view_injective.
                                        // But t_wit@ != k@ and t_wit@ == pair.0@ == k@. Contradiction.
                                    },
                                    core::cmp::Ordering::Greater => {
                                        // le(*k, pair.0) && le(pair.0, *k).
                                        K::antisymmetric(pair.0, *k);
                                    },
                                    _ => {},
                                }
                            }
                        }
                    },
                }
                i = i + 1;
            }
            proof {
                assert(counted =~= dom.filter(pred)) by {
                    assert forall|x: K::V| counted.contains(x)
                        implies #[trigger] dom.filter(pred).contains(x) by {
                        assert(dom.contains(x));
                        assert(spec_rank_pred::<K>(x, *k));
                    };
                    assert forall|x: K::V| dom.filter(pred).contains(x)
                        implies #[trigger] counted.contains(x) by {
                        assert(dom.contains(x));
                        assert(spec_rank_pred::<K>(x, *k));
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq, x);
                        let idx = choose|idx: int| 0 <= idx < seq.len()
                            && (#[trigger] seq[idx]).0 == x;
                        assert(spec_rank_pred::<K>(seq[idx].0, *k));
                        assert(counted.contains(seq[idx].0));
                    };
                };
                dom.lemma_len_filter(pred);
            }
            count
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_seq@);
            }
            let len = self.base_seq.length();
            if i >= self.size() {
                None
            } else {
                let mut j: usize = 0;
                let mut result_key: Option<K> = None;
                while j < len
                    invariant
                        j <= len,
                        len as nat == self.base_seq@.len(),
                        self.spec_orderedtablesteph_wf(),
                        obeys_view_eq::<K>(),
                        obeys_feq_full::<K>(),
                        obeys_feq_full::<Pair<K, V>>(),
                        self@.dom().finite(),
                        i < self@.dom().len(),
                        match result_key {
                            None => true,
                            Some(rk) =>
                                self@.dom().contains(rk@)
                                && self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, rk) && t@ != rk@).len() == i as int,
                        },
                    decreases len - j,
                {
                    let candidate = self.base_seq.nth(j);
                    let rank_val = self.rank_key(&candidate.0);
                    if rank_val == i && result_key.is_none() {
                        let pair_clone = candidate.clone_plus();
                        proof {
                            lemma_cloned_view_eq(*candidate, pair_clone);
                            // pair_clone.0@ == candidate.0@ from cloned view eq.
                            // view_injective: pair_clone.0 == candidate.0 (structural).
                            let ghost _pv = pair_clone.0@;
                            let ghost _cv = candidate.0@;
                            assert(pair_clone.0 == candidate.0);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_seq@, j as int);
                        }
                        result_key = Some(pair_clone.0);
                    }
                    j = j + 1;
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

            // Build left entries [0..split_at).
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

            // Build right entries [split_at..size).
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
                // spec_keys_no_dups for left.
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

                // spec_keys_no_dups for right.
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

            proof {
                lemma_entries_to_map_len::<K::V, V::V>(old(self).base_seq@);
                assert(entries@.len() < usize::MAX as nat);
                assert(left_seq@.len() < usize::MAX as nat);
                assert(right_seq@.len() < usize::MAX as nat);
            }
            let left_table = from_sorted_entries(left_seq);
            let right_table = from_sorted_entries(right_seq);

            proof {
                // Subset: left.
                assert forall|idx: int| 0 <= idx < left_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < entries@.len()
                        && (#[trigger] entries@[jdx]).0 == (#[trigger] left_seq@[idx]).0
                by {
                    assert(left_seq@[idx] == left_entries@[idx]@);
                    assert(left_entries@[idx]@ == entries@[idx]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(left_seq@, entries@);

                // Subset: right.
                assert forall|idx: int| 0 <= idx < right_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < entries@.len()
                        && (#[trigger] entries@[jdx]).0 == (#[trigger] right_seq@[idx]).0
                by {
                    let jdx = split_at as int + idx;
                    assert(right_seq@[idx] == right_entries@[idx]@);
                    assert(right_entries@[idx]@ == entries@[jdx]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(right_seq@, entries@);

                // Disjoint.
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

                // Coverage.
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
            requires self.spec_orderedtablesteph_wf()
            ensures
                it@.0 == 0,
                it@.1 == self.base_seq@,
                iter_invariant(&it),
        {
            let len = avl_seq_length(&self.base_seq);
            OrderedTableStEphIter { seq: &self.base_seq, pos: 0, len }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphIter<'a, K: StT + Ord, V: StT> {
        pub seq: &'a AVLTreeSeqStEphS<Pair<K, V>>,
        pub pos: usize,
        pub len: usize,
    }

    impl<'a, K: StT + Ord, V: StT> View for OrderedTableStEphIter<'a, K, V> {
        type V = (int, Seq<(K::V, V::V)>);
        open spec fn view(&self) -> (int, Seq<(K::V, V::V)>) { (self.pos as int, self.seq@) }
    }

    pub open spec fn iter_invariant<'a, K: StT + Ord, V: StT>(it: &OrderedTableStEphIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::Iterator for OrderedTableStEphIter<'a, K, V> {
        type Item = &'a Pair<K, V>;

        #[verifier::external_body]
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
                        &&& element@ == old_seq[old_index]
                    },
                }
            })
        {
            if self.pos < self.len {
                let elem = self.seq.nth(self.pos);
                self.pos = self.pos + 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphGhostIterator<'a, K: StT + Ord, V: StT> {
        pub pos: int,
        pub elements: Seq<(K::V, V::V)>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K: StT + Ord, V: StT> View for OrderedTableStEphGhostIterator<'a, K, V> {
        type V = Seq<(K::V, V::V)>;

        open spec fn view(&self) -> Seq<(K::V, V::V)> {
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
        type Item = (K::V, V::V);
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

        open spec fn ghost_peek_next(&self) -> Option<(K::V, V::V)> {
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
            requires self.spec_orderedtablesteph_wf()
            ensures
                it@.0 == 0,
                it@.1 == self.base_seq@,
                iter_invariant(&it),
        {
            let len = avl_seq_length(&self.base_seq);
            OrderedTableStEphIter { seq: &self.base_seq, pos: 0, len }
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> (cloned: Self) {
            OrderedTableStEph {
                base_seq: self.base_seq.clone(),
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
            entries@.len() < usize::MAX as nat,
        ensures
            cloned@.dom().finite(),
            cloned@ =~= spec_entries_to_map(entries@),
            cloned.spec_orderedtablesteph_wf(),
    {
        assert(obeys_feq_full_trigger::<Pair<K, V>>());
        let len = entries.length();
        let mut elements: Vec<Pair<K, V>> = Vec::new();
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len as nat == entries@.len(),
                entries.spec_avltreeseqstper_wf(),
                obeys_feq_full::<Pair<K, V>>(),
                elements@.len() == i as int,
                forall|j: int| 0 <= j < i ==> (#[trigger] elements@[j])@ == entries@[j],
            decreases len - i,
        {
            let elem = entries.nth(i);
            let cloned_elem = elem.clone_plus();
            proof { lemma_cloned_view_eq(*elem, cloned_elem); }
            elements.push(cloned_elem);
            i = i + 1;
        }
        let tree = AVLTreeSeqStEphS::from_vec(elements);
        proof {
            assert(tree@ =~= entries@) by {
                assert forall|j: int| 0 <= j < tree@.len()
                    implies #[trigger] tree@[j] == entries@[j]
                by {};
            };
            assert(spec_keys_no_dups(tree@));
            lemma_entries_to_map_finite::<K::V, V::V>(entries@);
        }
        OrderedTableStEph { base_seq: tree }
    }

    } // verus!

    // 13. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT> PartialEq for OrderedTableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            let len = self.base_seq.length();
            if len != other.base_seq.length() { return false; }
            for i in 0..len {
                if self.base_seq.nth(i) != other.base_seq.nth(i) { return false; }
            }
            true
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
}
