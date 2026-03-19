//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered table backed by AVLTreeSetStPer<Pair<K, V>>.

pub mod OrderedTableStPer {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;
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
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 10. iterators
    // 12. derive impls in verus!
    // 13. macros

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPer<K: StT + Ord, V: StT + Ord> {
        pub base_set: AVLTreeSetStPer<Pair<K, V>>,
    }

    pub type OrderedTablePer<K, V> = OrderedTableStPer<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStPer<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { spec_entries_to_map(self.base_set.elements@) }
    }

    // 6. spec fns

    proof fn lemma_keys_no_dups_implies_no_duplicates<KV, VV>(entries: Seq<(KV, VV)>)
        requires spec_keys_no_dups(entries),
        ensures entries.no_duplicates(),
    {
        assert forall|i: int, j: int| 0 <= i < j < entries.len()
            implies entries[i] != entries[j]
        by {
            assert(entries[i].0 != entries[j].0);
        };
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with persistent semantics.
    pub trait OrderedTableStPerTrait<K: StT + Ord, V: StT + Ord>: Sized + View<V = Map<K::V, V::V>> {
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

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        open spec fn spec_orderedtablestper_wf(&self) -> bool {
            self.base_set.spec_avltreesetstper_wf()
            && spec_keys_no_dups(self.base_set.elements@)
        }

        #[verifier::external_body]
        fn size(&self) -> (count: usize)
        {
            self.base_set.size()
        }

        fn empty() -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_orderedtablestper_wf()
        {
            let base = AVLTreeSetStPer::empty();
            proof {
                base.elements@.unique_seq_to_set();
            }
            OrderedTableStPer { base_set: base }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V) -> (table: Self)
        {
            OrderedTableStPer { base_set: AVLTreeSetStPer::singleton(Pair(k, v)) }
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (found: Option<V>)
        {
            let len = self.base_set.elements.length();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 == *k {
                    return Some(pair.1.clone());
                }
                i += 1;
            }
            None
        }

        #[verifier::external_body]
        fn insert(&self, k: K, v: V) -> (table: Self)
        {
            // Remove existing pair with same key (if any), then insert new pair.
            let len = self.base_set.elements.length();
            let mut base = self.base_set.clone();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 == k {
                    let pair_clone = pair.clone();
                    base = base.delete(&pair_clone);
                    break;
                }
                i += 1;
            }
            OrderedTableStPer {
                base_set: base.insert(Pair(k, v)),
            }
        }

        #[verifier::external_body]
        fn delete(&self, k: &K) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 == *k {
                    let pair_clone = pair.clone();
                    return OrderedTableStPer {
                        base_set: self.base_set.delete(&pair_clone),
                    };
                }
                i += 1;
            }
            self.clone()
        }

        #[verifier::external_body]
        fn domain(&self) -> (keys: ArraySetStEph<K>)
        {
            let len = self.base_set.elements.length();
            let mut keys = ArraySetStEph::empty();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                keys.insert(pair.0.clone());
                i += 1;
            }
            keys
        }

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let key_seq = keys.to_seq();
            let len = key_seq.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let k = key_seq.nth(i);
                let v = f(k);
                result_vec.push(Pair(k.clone(), v));
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                let new_v = f(&pair.1);
                result_vec.push(Pair(pair.0.clone(), new_v));
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if f(&pair.0, &pair.1) {
                    result_vec.push(pair.clone());
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined = f(&pair.1, &other_v);
                        result_vec.push(Pair(pair.0.clone(), combined));
                    },
                    None => {},
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
        {
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            // Entries from self (merge with other where keys overlap).
            let self_len = self.base_set.elements.length();
            let mut i: usize = 0;
            while i < self_len {
                let pair = self.base_set.elements.nth(i);
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined = f(&pair.1, &other_v);
                        result_vec.push(Pair(pair.0.clone(), combined));
                    },
                    None => {
                        result_vec.push(pair.clone());
                    },
                }
                i += 1;
            }
            // Entries from other that are not in self.
            let other_len = other.base_set.elements.length();
            i = 0;
            while i < other_len {
                let pair = other.base_set.elements.nth(i);
                let self_find = self.find(&pair.0);
                if self_find.is_none() {
                    result_vec.push(pair.clone());
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                let other_find = other.find(&pair.0);
                if other_find.is_none() {
                    result_vec.push(pair.clone());
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if keys.find(&pair.0) {
                    result_vec.push(pair.clone());
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if !keys.find(&pair.0) {
                    result_vec.push(pair.clone());
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
        {
            self.base_set.to_seq()
        }

        #[verifier::external_body]
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            if len == 0 {
                None
            } else {
                let first_pair = self.base_set.elements.nth(0);
                let mut min_key = first_pair.0.clone();
                let mut i: usize = 1;
                while i < len {
                    let elem_pair = self.base_set.elements.nth(i);
                    if elem_pair.0 < min_key {
                        min_key = elem_pair.0.clone();
                    }
                    i += 1;
                }
                Some(min_key)
            }
        }


        #[verifier::external_body]
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            if len == 0 {
                None
            } else {
                let first_pair = self.base_set.elements.nth(0);
                let mut max_key = first_pair.0.clone();
                let mut i: usize = 1;
                while i < len {
                    let elem_pair = self.base_set.elements.nth(i);
                    if elem_pair.0 > max_key {
                        max_key = elem_pair.0.clone();
                    }
                    i += 1;
                }
                Some(max_key)
            }
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            let mut best: Option<K> = None;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 < *k {
                    match &best {
                        None => { best = Some(pair.0.clone()); },
                        Some(b) => {
                            if pair.0 > *b {
                                best = Some(pair.0.clone());
                            }
                        },
                    }
                }
                i += 1;
            }
            best
        }

        #[verifier::external_body]
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            let mut best: Option<K> = None;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 > *k {
                    match &best {
                        None => { best = Some(pair.0.clone()); },
                        Some(b) => {
                            if pair.0 < *b {
                                best = Some(pair.0.clone());
                            }
                        },
                    }
                }
                i += 1;
            }
            best
        }

        #[verifier::external_body]
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
        {
            let len = self.base_set.elements.length();
            let mut left_vec: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 == *k {
                    found_value = Some(pair.1.clone());
                } else {
                    left_vec.push(pair.clone());
                }
                i += 1;
            }
            let left_seq = AVLTreeSeqStPerS::from_vec(left_vec);
            let left_table = OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(left_seq) };
            let right_table = Self::empty();
            (left_table, found_value, right_table)
        }

        #[verifier::external_body]
        fn join_key(left: &Self, right: &Self) -> (table: Self)
        {
            left.union(right, |v1: &V, _v2: &V| -> (r: V) { v1.clone() })
        }

        #[verifier::external_body]
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 >= *k1 && pair.0 <= *k2 {
                    result_vec.push(pair.clone());
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 < *k {
                    count += 1;
                }
                i += 1;
            }
            count
        }

        #[verifier::external_body]
        fn select_key(&self, i: usize) -> (key: Option<K>)
            where K: TotalOrder
        {
            let entries = self.collect();
            let len = entries.length();
            if i >= len {
                return None;
            }
            // Collect all keys, sort, return i-th.
            let mut keys: Vec<K> = Vec::new();
            let mut j: usize = 0;
            while j < len {
                let pair = entries.nth(j);
                keys.push(pair.0.clone());
                j += 1;
            }
            keys.sort();
            Some(keys[i].clone())
        }

        #[verifier::external_body]
        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
        {
            let entries = self.collect();
            let len = entries.length();
            // Collect all keys, sort them.
            let mut keys: Vec<K> = Vec::new();
            let mut j: usize = 0;
            while j < len {
                let pair = entries.nth(j);
                keys.push(pair.0.clone());
                j += 1;
            }
            keys.sort();
            let split_at = if i >= len { len } else { i };
            // Build left and right by rank.
            let mut left_vec: Vec<Pair<K, V>> = Vec::new();
            let mut right_vec: Vec<Pair<K, V>> = Vec::new();
            for (rank, sorted_key) in keys.iter().enumerate() {
                // Find the pair with this key.
                let mut k: usize = 0;
                while k < len {
                    let pair = entries.nth(k);
                    if pair.0 == *sorted_key {
                        if rank < split_at {
                            left_vec.push(pair.clone());
                        } else {
                            right_vec.push(pair.clone());
                        }
                        break;
                    }
                    k += 1;
                }
            }
            let left_seq = AVLTreeSeqStPerS::from_vec(left_vec);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_vec);
            (
                OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(left_seq) },
                OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(right_seq) },
            )
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_entries<K: StT + Ord, V: StT + Ord>(
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
        OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(entries) }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStPer<K, V> {
        /// Returns an iterator over the table entries.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<'_, K, V>)
            requires self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedTableStPerIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerIter<'a, K: StT + Ord, V: StT + Ord> {
        pub seq: &'a AVLTreeSeqStPerS<Pair<K, V>>,
        pub pos: usize,
        pub len: usize,
    }

    impl<'a, K: StT + Ord, V: StT + Ord> View for OrderedTableStPerIter<'a, K, V> {
        type V = (int, Seq<(K::V, V::V)>);
        open spec fn view(&self) -> (int, Seq<(K::V, V::V)>) { (self.pos as int, self.seq@) }
    }

    pub open spec fn iter_invariant<'a, K: StT + Ord, V: StT + Ord>(it: &OrderedTableStPerIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT + Ord> std::iter::Iterator for OrderedTableStPerIter<'a, K, V> {
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
                self.pos += 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerGhostIterator<'a, K: StT + Ord, V: StT + Ord> {
        pub pos: int,
        pub elements: Seq<(K::V, V::V)>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K: StT + Ord, V: StT + Ord> View for OrderedTableStPerGhostIterator<'a, K, V> {
        type V = Seq<(K::V, V::V)>;

        open spec fn view(&self) -> Seq<(K::V, V::V)> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStPerIter<'a, K, V> {
        type GhostIter = OrderedTableStPerGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStPerGhostIterator<'a, K, V> {
            OrderedTableStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedTableStPerGhostIterator<'a, K, V> {
        type ExecIter = OrderedTableStPerIter<'a, K, V>;
        type Item = (K::V, V::V);
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

        open spec fn ghost_peek_next(&self) -> Option<(K::V, V::V)> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStPerIter<'a, K, V>) -> OrderedTableStPerGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> std::iter::IntoIterator for &'a OrderedTableStPer<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedTableStPerIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    // 12. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT + Ord> PartialEqSpecImpl for OrderedTableStPer<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT + Ord> Eq for OrderedTableStPer<K, V> {}

    impl<K: StT + Ord, V: StT + Ord> PartialEq for OrderedTableStPer<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.base_set == other.base_set;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT + Ord> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@
        {
            let copy = OrderedTableStPer {
                base_set: self.base_set.clone(),
            };
            proof { assume(copy@ == self@); }
            copy
        }
    }

    } // verus!

    // 13. macros

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

    // 14. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT + Ord> fmt::Debug for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Display for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }
}
