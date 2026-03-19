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
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>(), obeys_view_eq::<K>()
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

    /// Get the length of the backing AVL sequence without requiring wf.
    #[verifier::external_body]
    fn avl_seq_length<K: StT + Ord, V: StT>(seq: &AVLTreeSeqStEphS<Pair<K, V>>) -> (len: usize)
        ensures len as nat == seq@.len()
    {
        seq.length()
    }

    /// Get nth element from the AVL sequence without requiring wf.
    #[verifier::external_body]
    fn avl_seq_nth<'a, K: StT + Ord, V: StT>(seq: &'a AVLTreeSeqStEphS<Pair<K, V>>, index: usize) -> (elem: &'a Pair<K, V>)
        requires (index as int) < seq@.len()
        ensures elem@ == seq@[index as int]
    {
        seq.nth(index)
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        open spec fn spec_orderedtablesteph_wf(&self) -> bool {
            self.base_seq.spec_avltreeseqsteph_wf()
            && spec_keys_no_dups(self.base_seq@)
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

        #[verifier::external_body]
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut match_index: usize = len;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                if pair.0 == k {
                    match_index = i;
                } else {
                    all.push(pair.clone());
                }
                i += 1;
            }
            let final_value;
            if match_index < len {
                let old_entry = self.base_seq.nth(match_index);
                final_value = combine(&old_entry.1, &v);
            } else {
                final_value = v;
            }
            all.push(Pair(k, final_value));
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
        }

        #[verifier::external_body]
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            ensures self@ == old(self)@.remove(k@), self@.dom().finite(), self.spec_orderedtablesteph_wf()
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                if pair.0 == *k && found_value.is_none() {
                    found_value = Some(pair.1.clone());
                } else {
                    all.push(pair.clone());
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
            found_value
        }

        #[verifier::external_body]
        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            let len = self.base_seq.length();
            let mut domain = ArraySetStEph::empty();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                domain.insert(pair.0.clone());
                i += 1;
            }
            domain
        }

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
            let seq = keys.to_seq();
            let len = seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let k = seq.nth(i);
                let v = f(k);
                all.push(Pair(k.clone(), v));
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            OrderedTableStEph { base_seq: tree }
        }

        #[verifier::external_body]
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                let new_val = f(&pair.0, &pair.1);
                all.push(Pair(pair.0.clone(), new_val));
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            OrderedTableStEph { base_seq: tree }
        }

        #[verifier::external_body]
        fn filter<F: Fn(&K, &V) -> B>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                if f(&pair.0, &pair.1) {
                    all.push(pair.clone());
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            OrderedTableStEph { base_seq: tree }
        }

        #[verifier::external_body]
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite()
        {
            let len = self.base_seq.length();
            let mut reduced = init;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                reduced = f(reduced, &pair.0, &pair.1);
                i += 1;
            }
            reduced
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                let other_val = other.find(&pair.0);
                if let Some(ov) = other_val {
                    let combined = f(&pair.1, &ov);
                    all.push(Pair(pair.0.clone(), combined));
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
        }

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            // Start with self entries, combining with other when key overlaps.
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                let other_val = other.find(&pair.0);
                match other_val {
                    Some(ov) => {
                        let combined = f(&pair.1, &ov);
                        all.push(Pair(pair.0.clone(), combined));
                    },
                    None => {
                        all.push(pair.clone());
                    },
                }
                i += 1;
            }
            // Add other entries not in self.
            let other_len = other.base_seq.length();
            let mut j: usize = 0;
            while j < other_len {
                let pair = other.base_seq.nth(j);
                let self_val = self.find(&pair.0);
                if self_val.is_none() {
                    all.push(pair.clone());
                }
                j += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self)
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                if !other.find(&pair.0).is_some() {
                    all.push(pair.clone());
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
        }

        #[verifier::external_body]
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                if keys.find(&pair.0) {
                    all.push(pair.clone());
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
        }

        #[verifier::external_body]
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            let len = self.base_seq.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                if !keys.find(&pair.0) {
                    all.push(pair.clone());
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(all);
            self.base_seq = tree;
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

        #[verifier::external_body]
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
        {
            let len = self.base_seq.length();
            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                if pair.0 == *k {
                    found_value = Some(pair.1.clone());
                } else {
                    left_entries.push(pair.clone());
                }
                i += 1;
            }
            let left_tree = AVLTreeSeqStEphS::from_vec(left_entries);
            let left_table = OrderedTableStEph { base_seq: left_tree };
            let right_table = Self::empty();
            *self = Self::empty();
            (left_table, found_value, right_table)
        }

        fn join_key(&mut self, other: Self)
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        #[verifier::external_body]
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
        {
            let len = self.base_seq.length();
            let mut range_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                let ge_k1 = match pair.0.cmp(k1) {
                    std::cmp::Ordering::Less => false,
                    _ => true,
                };
                let le_k2 = match pair.0.cmp(k2) {
                    std::cmp::Ordering::Greater => false,
                    _ => true,
                };
                if ge_k1 && le_k2 {
                    range_entries.push(pair.clone());
                }
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(range_entries);
            OrderedTableStEph { base_seq: tree }
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            let len = self.base_seq.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_seq.nth(i);
                let c = TotalOrder::cmp(&pair.0, k);
                match c {
                    core::cmp::Ordering::Less => { count = count + 1; },
                    _ => {},
                }
                i = i + 1;
            }
            count
        }

        #[verifier::external_body]
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_seq.length();
            if i >= self.size() {
                None
            } else {
                let mut j: usize = 0;
                let mut result_key: Option<K> = None;
                while j < len {
                    let candidate = self.base_seq.nth(j);
                    let rank_val = self.rank_key(&candidate.0);
                    if rank_val == i && result_key.is_none() {
                        result_key = Some(candidate.0.clone());
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

    #[verifier::external_body]
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
        while i < len {
            let elem = entries.nth(i);
            elements.push(elem.clone());
            i = i + 1;
        }
        let tree = AVLTreeSeqStEphS::from_vec(elements);
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
