//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral ordered table implementation extending TableMtEph.

pub mod OrderedTableMtEph {

    use std::sync::Arc;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableMtEph::TableMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

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
    pub struct OrderedTableMtEph<K: MtKey, V: MtVal> {
        pub base_table: TableMtEph<K, V>,
    }

    pub type OrderedTableMt<K, V> = OrderedTableMtEph<K, V>;

    // 5. view impls

    impl<K: MtKey, V: MtVal> View for OrderedTableMtEph<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1 for keys) with multi-threaded ephemeral semantics
    pub trait OrderedTableMtEphTrait<K: MtKey, V: MtVal>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablemteph_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- delegates to TableMtEph.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablemteph_wf()
            ensures count == self@.dom().len(), self@.dom().finite();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty TableMtEph
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_orderedtablemteph_wf();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps TableMtEph.singleton
        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablemteph_wf();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableMtEph.find (linear scan)
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@),
                    None => !self@.contains_key(k@),
                };

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to find (linear scan)
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@),
                    None => !self@.contains_key(k@),
                };

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- compares size to 0
        fn is_empty(&self) -> (is_empty: B)
            requires self.spec_orderedtablemteph_wf()
            ensures is_empty == self@.dom().is_empty();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableMtEph.insert (linear dup check)
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: F)
            requires
                old(self).spec_orderedtablemteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_clone::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@.dom().finite();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableMtEph.delete (linear scan)
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablemteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<V>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite();

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableMtEph.domain
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            ensures self@.dom().finite();

        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n^2), Span Θ(n^2) -- delegates to TableMtEph.tabulate (sequential insert loop)
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
            ensures tabulated@.dom().finite();

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableMtEph.map (linear iteration)
        fn map<F: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: F) -> (mapped: Self)
            requires forall|k: &K, v: &V| f.requires((k, v))
            ensures mapped@.dom().finite();

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableMtEph.filter (linear iteration)
        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
            requires
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures filtered@.dom().finite();

        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableMtEph.intersection (linear scan)
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F)
            requires forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures self@.dom().finite();

        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableMtEph.union (linear merge)
        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F)
            requires forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures self@.dom().finite();

        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableMtEph.difference (linear scan)
        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite();

        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();

        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableMtEph.reduce (linear fold)
        fn reduce<R: StTInMtT + 'static, F: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: F) -> (reduced: R)
            requires forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();

        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects entries and sorts by key
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf();

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
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for predecessor
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for successor
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by key
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();

        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union (linear merge)
        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite();

        /// - APAS: Work Θ(log n + m), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then filters by range
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then counts elements < k
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then indexes
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by rank
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: MtKey, V: MtVal> OrderedTableMtEphTrait<K, V> for OrderedTableMtEph<K, V> {
        open spec fn spec_orderedtablemteph_wf(&self) -> bool {
            self@.dom().finite() && self.base_table.spec_tablemteph_wf()
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
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_orderedtablemteph_wf()
        {
            OrderedTableMtEph {
                base_table: TableMtEph::empty(),
            }
        }

        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablemteph_wf()
        {
            let base = TableMtEph::singleton(k, v);
            let tree = OrderedTableMtEph { base_table: base };
            proof {
                assert(tree@ =~= Map::<K::V, V::V>::empty().insert(k@, v@));
                lemma_entries_to_map_finite::<K::V, V::V>(tree.base_table.entries@);
            }
            tree
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

        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: F)
            ensures self@.dom().finite()
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
            ensures self@.dom().finite()
        {
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            self.base_table.domain()
        }

        fn tabulate<F>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            where F: Fn(&K) -> V + Send + Sync + 'static
            ensures tabulated@.dom().finite()
        {
            let base = TableMtEph::tabulate(f, keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableMtEph { base_table: base }
        }

        fn map<F>(&self, f: F) -> (mapped: Self)
            where F: Fn(&K, &V) -> V + Send + Sync + 'static
            ensures mapped@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut result_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                decreases size - i,
            {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                result_entries.push(Pair(pair.0.clone(), new_value));
                i += 1;
            }
            let result_seq = AVLTreeSeqStPerS::from_vec(result_entries);
            from_sorted_entries(result_seq)
        }

        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
        {
            let entries = self.collect();
            let size = entries.length();
            let mut result_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                decreases size - i,
            {
                let pair = entries.nth(i);
                if f(&pair.0, &pair.1) {
                    result_entries.push(Pair(pair.0.clone(), pair.1.clone()));
                }
                i += 1;
            }
            let result_seq = AVLTreeSeqStPerS::from_vec(result_entries);
            from_sorted_entries(result_seq)
        }

        fn reduce<R: StTInMtT + 'static, F: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: F) -> (reduced: R)
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

        fn intersection<F>(&mut self, other: &Self, f: F)
            where F: Fn(&V, &V) -> V + Send + Sync + 'static
            ensures self@.dom().finite()
        {
            self.base_table.intersection(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn union<F>(&mut self, other: &Self, f: F)
            where F: Fn(&V, &V) -> V + Send + Sync + 'static
            ensures self@.dom().finite()
        {
            self.base_table.union(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            self.base_table.difference(&other.base_table);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.restrict(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.subtract(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf()
        {
            let array_seq = self.base_table.entries();
            let len = array_seq.length();
            let mut elements: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as int == array_seq.spec_len(),
                decreases len - i,
            {
                let elem = array_seq.nth(i);
                elements.push(Pair(elem.0.clone(), elem.1.clone()));
                i += 1;
            }
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
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
            where Self: Sized
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut right_entries: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases size - i,
            {
                let pair = entries.nth(i);
                match pair.0.cmp(k) {
                    std::cmp::Ordering::Less => left_entries.push(Pair(pair.0.clone(), pair.1.clone())),
                    std::cmp::Ordering::Greater => right_entries.push(Pair(pair.0.clone(), pair.1.clone())),
                    std::cmp::Ordering::Equal => found_value = Some(pair.1.clone()),
                }
                i += 1;
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            *self = Self::empty();
            (from_sorted_entries(left_seq), found_value, from_sorted_entries(right_seq))
        }

        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite()
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut range_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases size - i,
            {
                let pair = entries.nth(i);
                let ge_k1 = match pair.0.cmp(k1) {
                    std::cmp::Ordering::Less => false,
                    _ => true,
                };
                let le_k2 = match pair.0.cmp(k2) {
                    std::cmp::Ordering::Greater => false,
                    _ => true,
                };
                if ge_k1 && le_k2 {
                    range_entries.push(Pair(pair.0.clone(), pair.1.clone()));
                }
                i += 1;
            }

            let range_seq = AVLTreeSeqStPerS::from_vec(range_entries);
            from_sorted_entries(range_seq)
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            let entries = self.base_table.entries();
            let n = entries.length();
            let mut count: usize = 0;
            for i in 0..n {
                let pair = entries.nth(i);
                if pair.0 < *k {
                    count += 1;
                }
            }
            count
        }

        #[verifier::external_body]
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            // Kept as external_body: uses Vec::sort() which has no Verus specs.
            let entries = self.base_table.entries();
            let n = entries.length();
            if i >= n {
                return None;
            }
            let mut keys: Vec<K> = Vec::new();
            let mut j: usize = 0;
            while j < n {
                keys.push(entries.nth(j).0.clone());
                j += 1;
            }
            keys.sort();
            Some(keys[i].clone())
        }

        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();

            if i >= size {
                let current = self.clone();
                *self = Self::empty();
                return (current, Self::empty());
            }

            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut right_entries: Vec<Pair<K, V>> = Vec::new();

            let mut j: usize = 0;
            while j < i
                invariant
                    j <= i,
                    i < size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases i - j,
            {
                let elem = entries.nth(j);
                left_entries.push(Pair(elem.0.clone(), elem.1.clone()));
                j += 1;
            }
            let mut j: usize = i;
            while j < size
                invariant
                    j <= size,
                    i <= j,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases size - j,
            {
                let elem = entries.nth(j);
                right_entries.push(Pair(elem.0.clone(), elem.1.clone()));
                j += 1;
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            *self = Self::empty();
            (from_sorted_entries(left_seq), from_sorted_entries(right_seq))
        }
    }

    // 10. iterators

    impl<K: MtKey, V: MtVal> OrderedTableMtEph<K, V> {
        /// Returns an iterator over the table entries.
        pub fn iter(&self) -> (it: OrderedTableMtEphIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableMtEphIter { inner: self.base_table.entries.iter() }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtEphIter<'a, K, V> {
        pub inner: ArraySeqMtEphIter<'a, Pair<K, V>>,
    }

    impl<'a, K, V> View for OrderedTableMtEphIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, K, V>(it: &OrderedTableMtEphIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: MtKey, V: MtVal> std::iter::Iterator for OrderedTableMtEphIter<'a, K, V> {
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
    pub struct OrderedTableMtEphGhostIterator<'a, K, V> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K, V> View for OrderedTableMtEphGhostIterator<'a, K, V> {
        type V = Seq<Pair<K, V>>;

        open spec fn view(&self) -> Seq<Pair<K, V>> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, K: MtKey, V: MtVal> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableMtEphIter<'a, K, V> {
        type GhostIter = OrderedTableMtEphGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableMtEphGhostIterator<'a, K, V> {
            OrderedTableMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: MtKey, V: MtVal> vstd::pervasive::ForLoopGhostIterator for OrderedTableMtEphGhostIterator<'a, K, V> {
        type ExecIter = OrderedTableMtEphIter<'a, K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableMtEphIter<'a, K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableMtEphIter<'a, K, V>) -> OrderedTableMtEphGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: MtKey, V: MtVal> std::iter::IntoIterator for &'a OrderedTableMtEph<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableMtEphIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableMtEphIter { inner: self.base_table.entries.iter() }
        }
    }

    // 11. derive impls in verus!

    impl<K: MtKey, V: MtVal> Clone for OrderedTableMtEph<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned_base = self.base_table.clone();
            proof { assert(cloned_base@ == self.base_table@); }
            OrderedTableMtEph {
                base_table: cloned_base,
            }
        }
    }

    pub fn from_sorted_entries<K: MtKey, V: MtVal>(entries: AVLTreeSeqStPerS<Pair<K, V>>) -> (constructed: OrderedTableMtEph<K, V>)
        requires entries.spec_avltreeseqstper_wf()
        ensures constructed@.dom().finite()
    {
        let len = entries.length();
        let mut elements: Vec<Pair<K, V>> = Vec::new();
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len as nat == entries.spec_seq().len(),
                entries.spec_avltreeseqstper_wf(),
            decreases len - i,
        {
            let elem = entries.nth(i);
            elements.push(Pair(elem.0.clone(), elem.1.clone()));
            i += 1;
        }
        OrderedTableMtEph {
            base_table: crate::Chap42::TableMtEph::TableMtEph::from_sorted_entries(elements),
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    use std::fmt;

    impl<K: MtKey, V: MtVal> PartialEq for OrderedTableMtEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
        }
    }

    impl<K: MtKey, V: MtVal> fmt::Debug for OrderedTableMtEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEph(size: {})", self.size())
        }
    }

    impl<K: MtKey, V: MtVal> fmt::Display for OrderedTableMtEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEph(size: {})", self.size())
        }
    }

    // 12. macros

    /// Macro for creating multi-threaded ephemeral ordered tables from sorted key-value pairs
    #[macro_export]
    macro_rules! OrderedTableMtEphLit {
        () => {
            $crate::Chap43::OrderedTableMtEph::OrderedTableMtEph::OrderedTableMtEph::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableMtEph::OrderedTableMtEph::from_sorted_entries(seq)
        }};
    }
}
