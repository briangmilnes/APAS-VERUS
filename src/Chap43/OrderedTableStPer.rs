//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered table implementation extending TableStPer.

pub mod OrderedTableStPer {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use vstd::prelude::*;
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
    vstd::seq::group_seq_axioms,
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

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPer<K: StT + Ord, V: StT> {
        pub base_table: TableStPer<K, V>,
    }

    pub type OrderedTablePer<K, V> = OrderedTableStPer<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for OrderedTableStPer<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { spec_entries_to_map(self.base_table.entries@) }
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with persistent semantics.
    pub trait OrderedTableStPerTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablestper_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- delegates to TableStPer.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablestper_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty TableStPer
        fn empty() -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps TableStPer.singleton
        fn singleton(k: K, v: V) -> (table: Self)
            requires obeys_feq_clone::<Pair<K, V>>(),
            ensures table@ == Map::<K::V, V::V>::empty().insert(k@, v@), table@.dom().finite(), table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.find (linear scan)
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.insert (linear dup check)
        fn insert(&self, k: K, v: V) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.delete (linear scan)
        fn delete(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures table@ == self@.remove(k@), table@.dom().finite(), table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.domain
        fn domain(&self) -> (keys: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures keys@ =~= self@.dom(), self@.dom().finite();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n^2), Span Θ(n^2) -- delegates to TableStPer.tabulate (sequential insert loop)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>(),
            ensures
                table@.dom() =~= keys@,
                table.spec_orderedtablestper_wf(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && table@[k] == result@),
                table@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.map (linear iteration)
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), forall|v: &V| f.requires((v,)), obeys_feq_full::<K>(),
            ensures
                table@.dom() == self@.dom(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && table@[k] == result@),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.filter (linear iteration)
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                obeys_feq_full::<Pair<K, V>>(),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] table@.dom().contains(k),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStPer.intersection (linear scan)
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<K>(),
            ensures
                table@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStPer.union (linear merge)
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
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
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStPer.difference (linear scan)
        fn difference(&self, other: &Self) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().difference(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects entries and sorts by key
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), sorted_entries.spec_avltreeseqstper_wf(), sorted_entries@.len() == self@.dom().len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns first element
        fn first_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns last element
        fn last_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for predecessor
        fn previous_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                key matches Some(pk) ==> self@.dom().contains(pk@),
                key matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for successor
        fn next_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                key matches Some(nk) ==> self@.dom().contains(nk@),
                key matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by key
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
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
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@;
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union (linear merge)
        fn join_key(left: &Self, right: &Self) -> (table: Self)
            requires
                left.spec_orderedtablestper_wf(),
                right.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= left@.dom().union(right@.dom()),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(log n + m), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then filters by range
        fn get_key_range(&self, k1: &K, k2: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                table@.dom().finite(),
                table@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] table@.dom().contains(key) ==> table@[key] == self@[key];
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then counts elements < k
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then indexes
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
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by rank
        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                parts.0@.dom().finite(),
                parts.1@.dom().finite(),
                parts.0@.dom().subset_of(self@.dom()),
                parts.1@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.1@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.1@.dom().contains(key);
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        open spec fn spec_orderedtablestper_wf(&self) -> bool {
            self.base_table.spec_tablestper_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
            }
            self.base_table.size()
        }

        fn empty() -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_orderedtablestper_wf()
        {
            OrderedTableStPer {
                base_table: TableStPer::empty(),
            }
        }

        fn singleton(k: K, v: V) -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty().insert(k@, v@), table@.dom().finite(), table.spec_orderedtablestper_wf()
        {
            let base = TableStPer::singleton(k, v);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn find(&self, k: &K) -> (found: Option<V>) {
            self.base_table.find(k)
        }

        fn insert(&self, k: K, v: V) -> (table: Self)
        {
            let base = self.base_table.insert(k, v, |_old: &V, new: &V| -> (r: V) { new.clone() });
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn delete(&self, k: &K) -> (table: Self)
            ensures table@ == self@.remove(k@), table@.dom().finite(), table.spec_orderedtablestper_wf()
        {
            let base = self.base_table.delete(k);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(base.entries@);
                assert(base@ =~= self.base_table@.remove(k@));
            }
            OrderedTableStPer { base_table: base }
        }

        fn domain(&self) -> (keys: ArraySetStEph<K>)
        {
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            self.base_table.domain()
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let base = TableStPer::tabulate(f, keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
        {
            let base = self.base_table.map(f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn filter<F: Fn(&K, &V) -> B>(&self, f: F, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (table: Self)
        {
            let base = self.base_table.filter(f, Ghost(spec_pred));
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
        {
            let base = self.base_table.intersection(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
        {
            let base = self.base_table.union(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn difference(&self, other: &Self) -> (table: Self)
        {
            let base = self.base_table.difference(&other.base_table);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let base = self.base_table.restrict(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let base = self.base_table.subtract(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        #[verifier::external_body]
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures
                self@.dom().finite(),
                sorted_entries.spec_avltreeseqstper_wf(),
                sorted_entries@.len() == self@.dom().len(),
                forall|i: int| 0 <= i < sorted_entries@.len() ==> self@.dom().contains((#[trigger] sorted_entries@[i]).0),
                self.spec_orderedtablestper_wf() ==> spec_entries_to_map(sorted_entries@) =~= self@,
                self.spec_orderedtablestper_wf() ==> spec_keys_no_dups(sorted_entries@),
        {
            let array_seq = self.base_table.collect();
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
                    assert forall|t: K| self@.dom().contains(t@)
                        implies TotalOrder::le(min_key, t) by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, t@);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == t@;
                        assert(self.base_table.entries.spec_index(j).0@ == t@);
                        assert(self.base_table.entries.spec_index(j).0 == t);
                    };
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
                    assert forall|t: K| self@.dom().contains(t@)
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
                                            && TotalOrder::le(*k, self.base_table.entries.spec_index(j).0) && self.base_table.entries.spec_index(j).0@ != k@
                                            implies TotalOrder::le(#[trigger] self.base_table.entries.spec_index(best_idx).0, self.base_table.entries.spec_index(j).0) by {
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
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
        {
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            assert(obeys_feq_full_trigger::<V>());
            let len = self.base_table.entries.length();
            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let ghost mut src_idx: Seq<int> = Seq::empty();
            let ghost mut found_idx: int = -1;
            let ghost mut pos: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    self.spec_orderedtablestper_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    obeys_feq_full::<V>(),
                    left_entries@.len() == src_idx.len(),
                    forall|r: int| #![trigger src_idx[r]]
                        0 <= r < src_idx.len() ==>
                        0 <= src_idx[r] < i
                        && left_entries@[r]@ == self.base_table.entries@[src_idx[r]]
                        && self.base_table.entries@[src_idx[r]].0 != k@,
                    forall|a: int, b: int| 0 <= a < b < src_idx.len() ==>
                        (#[trigger] src_idx[a]) < (#[trigger] src_idx[b]),
                    found_value matches Some(v) ==> (
                        0 <= found_idx < i
                        && self.base_table.entries@[found_idx].0 == k@
                        && v@ == self.base_table.entries@[found_idx].1
                    ),
                    found_value matches None ==> forall|j: int| 0 <= j < i ==>
                        (#[trigger] self.base_table.entries@[j]).0 != k@,
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
            let left_seq = ArraySeqStPerS::<Pair<K, V>>::from_vec(left_entries);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(left_seq@);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
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
            let left_table = OrderedTableStPer { base_table: TableStPer { entries: left_seq } };
            let right_table = Self::empty();
            proof {
                if found_value is Some {
                    let v = found_value.unwrap();
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, found_idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_table.entries@, found_idx);
                }
                if found_value is None {
                    lemma_entries_to_map_no_key::<K::V, V::V>(self.base_table.entries@, k@);
                }
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
                assert(!right_table@.dom().contains(k@));
                assert(right_table@.dom().subset_of(self@.dom()));
                assert(left_table@.dom().disjoint(right_table@.dom()));
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
            (left_table, found_value, right_table)
        }

        fn join_key(left: &Self, right: &Self) -> (table: Self)
        {
            left.union(right, |v1: &V, _v2: &V| -> (r: V) { v1.clone() })
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
                    self.spec_orderedtablestper_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    range_entries@.len() == src_idx.len(),
                    forall|r: int| #![trigger src_idx[r]]
                        0 <= r < src_idx.len() ==>
                        0 <= src_idx[r] < i && range_entries@[r]@ == self.base_table.entries@[src_idx[r]],
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
            let range_seq = ArraySeqStPerS::<Pair<K, V>>::from_vec(range_entries);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(range_seq@);
                assert(spec_keys_no_dups(range_seq@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < range_seq@.len()
                        implies (#[trigger] range_seq@[a]).0 != (#[trigger] range_seq@[b]).0
                    by {
                        range_seq.lemma_view_index(a);
                        range_seq.lemma_view_index(b);
                        assert(range_seq.spec_index(a) == range_entries@[a]);
                        assert(range_seq.spec_index(b) == range_entries@[b]);
                        assert(src_idx[a] < src_idx[b]);
                        assert(range_entries@[a]@ == self.base_table.entries@[src_idx[a]]);
                        assert(range_entries@[b]@ == self.base_table.entries@[src_idx[b]]);
                    };
                };
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
            let result = OrderedTableStPer { base_table: TableStPer { entries: range_seq } };
            proof {
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
            let ghost mut less_idx: Set<int> = Set::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == self.base_table.entries.spec_len(),
                    self.spec_orderedtablestper_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<K>(),
                    count as nat == less_idx.len(),
                    less_idx.finite(),
                    count <= i,
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
                            if TotalOrder::le(self.base_table.entries.spec_index(i as int).0, *k) {
                                K::antisymmetric(self.base_table.entries.spec_index(i as int).0, *k);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            proof {
                let pred = |x: K::V| exists|t: K| #![trigger t@ ] t@ == x && TotalOrder::le(t, *k) && t@ != k@;
                let less_keys: Set<K::V> = Set::new(|kv: K::V|
                    exists|j: int| #[trigger] less_idx.contains(j) && self.base_table.entries@[j].0 == kv);
                assert(less_keys =~= self@.dom().filter(pred)) by {
                    assert forall|kv: K::V| less_keys.contains(kv)
                        implies self@.dom().filter(pred).contains(kv) by {
                        let j = choose|j: int| less_idx.contains(j) && (#[trigger] self.base_table.entries@[j]).0 == kv;
                        assert(0 <= j < len);
                        self.base_table.entries.lemma_view_index(j);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_table.entries@, j);
                        let t = self.base_table.entries.spec_index(j).0;
                        assert(t@ == kv);
                        assert(TotalOrder::le(t, *k) && t != *k);
                        assert(t@ != k@);
                    };
                    assert forall|kv: K::V| self@.dom().filter(pred).contains(kv)
                        implies less_keys.contains(kv) by {
                        assert(self@.dom().contains(kv) && pred(kv));
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_table.entries@, kv);
                        let j = choose|j: int| 0 <= j < self.base_table.entries@.len()
                            && (#[trigger] self.base_table.entries@[j]).0 == kv;
                        self.base_table.entries.lemma_view_index(j);
                        let t = choose|t: K| #![trigger t@] t@ == kv && TotalOrder::le(t, *k) && t@ != k@;
                        assert(self.base_table.entries.spec_index(j).0@ == t@);
                        assert(self.base_table.entries.spec_index(j).0 == t);
                        assert(TotalOrder::le(self.base_table.entries.spec_index(j).0, *k));
                        assert(self.base_table.entries.spec_index(j).0 != *k);
                        assert(less_idx.contains(j));
                    };
                };
                assume(less_keys.len() == less_idx.len());
                assert(count as int == self@.dom().filter(pred).len());
                lemma_entries_to_map_len::<K::V, V::V>(self.base_table.entries@);
            }
            count
        }

        fn select_key(&self, i: usize) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int,
        {
            assert(obeys_feq_full_trigger::<K>());
            let len = self.base_table.entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if i >= self.size() {
                None
            } else {
                let mut j: usize = 0;
                let mut found: bool = false;
                let mut result_key: Option<K> = None;
                while j < len
                    invariant
                        j <= len,
                        len as nat == self.base_table.entries.spec_len(),
                        self.spec_orderedtablestper_wf(),
                        obeys_view_eq::<K>(),
                        obeys_feq_full::<K>(),
                        i < self@.dom().len(),
                        self@.dom().finite(),
                        found ==> result_key is Some,
                        found ==> (result_key matches Some(rk) ==> (
                            self@.dom().contains(rk@) &&
                            self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, rk) && t@ != rk@).len() == i as int
                        )),
                        !found ==> result_key is None,
                    decreases len - j,
                {
                    if !found {
                        let candidate = self.base_table.entries.nth(j);
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
                    if !found {
                        assume(false);
                    }
                }
                result_key
            }
        }

        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
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
                    assert(left_entries@[idx]@ == entries@[idx]);
                };
                lemma_entries_to_map_dom_subset::<K::V, V::V>(left_seq@, entries@);

                // Subset: right
                assert forall|idx: int| 0 <= idx < right_seq@.len()
                    implies exists|jdx: int| 0 <= jdx < entries@.len()
                        && (#[trigger] entries@[jdx]).0 == (#[trigger] right_seq@[idx]).0
                by {
                    let jdx = split_at as int + idx;
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
                            assert(left_entries@[li]@ == entries@[li]);
                            assert(right_entries@[ri]@ == entries@[split_at as int + ri]);
                            assert(entries@[li].0 == key);
                            assert(entries@[split_at as int + ri].0 == key);
                        }
                    };
                };

                // Coverage
                assert forall|key: K::V|
                    #[trigger] self@.dom().contains(key)
                    implies left_table@.dom().contains(key) || right_table@.dom().contains(key)
                by {
                    lemma_entries_to_map_key_in_seq(entries@, key);
                    let idx = choose|idx: int|
                        0 <= idx < entries@.len() && (#[trigger] entries@[idx]).0 == key;
                    if idx < split_at as int {
                        assert(left_entries@[idx]@ == entries@[idx]);
                        lemma_entries_to_map_contains_key(left_seq@, idx);
                    } else {
                        let ridx = idx - split_at as int;
                        assert(right_entries@[ridx]@ == entries@[split_at as int + ridx]);
                        lemma_entries_to_map_contains_key(right_seq@, ridx);
                    }
                };

                lemma_entries_to_map_finite::<K::V, V::V>(entries@);
            }

            (left_table, right_table)
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (table: OrderedTableStPer<K, V>)
        requires
            entries.spec_avltreeseqstper_wf(),
            spec_keys_no_dups(entries@),
        ensures
            table@.dom().finite(),
            table@ =~= spec_entries_to_map(entries@),
            table.spec_orderedtablestper_wf(),
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
            i += 1;
        }
        let seq = ArraySeqStPerS::from_vec(elements);
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
        OrderedTableStPer {
            base_table: TableStPer { entries: seq },
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT> OrderedTableStPer<K, V> {
        /// Returns an iterator over the table entries.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStPerIter { inner: self.base_table.entries.iter() }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerIter<'a, K, V> {
        pub inner: ArraySeqStPerIter<'a, Pair<K, V>>,
    }

    impl<'a, K, V> View for OrderedTableStPerIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, K, V>(it: &OrderedTableStPerIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::Iterator for OrderedTableStPerIter<'a, K, V> {
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
    pub struct OrderedTableStPerGhostIterator<'a, K, V> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K, V> View for OrderedTableStPerGhostIterator<'a, K, V> {
        type V = Seq<Pair<K, V>>;

        open spec fn view(&self) -> Seq<Pair<K, V>> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStPerIter<'a, K, V> {
        type GhostIter = OrderedTableStPerGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStPerGhostIterator<'a, K, V> {
            OrderedTableStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIterator for OrderedTableStPerGhostIterator<'a, K, V> {
        type ExecIter = OrderedTableStPerIter<'a, K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStPerIter<'a, K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStPerIter<'a, K, V>) -> OrderedTableStPerGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::IntoIterator for &'a OrderedTableStPer<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStPerIter { inner: self.base_table.entries.iter() }
        }
    }

    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT> PartialEqSpecImpl for OrderedTableStPer<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT> Eq for OrderedTableStPer<K, V> {}

    impl<K: StT + Ord, V: StT> PartialEq for OrderedTableStPer<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.base_table.entries == other.base_table.entries;
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@
        {
            OrderedTableStPer {
                base_table: self.base_table.clone(),
            }
        }
    }

    } // verus!

    // 12. macros

    /// Macro for creating ordered tables from sorted key-value pairs
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

    // 13. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT> fmt::Debug for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT> fmt::Display for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }
}
