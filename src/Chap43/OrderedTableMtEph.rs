//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Multi-threaded ephemeral ordered table using coarse RwLock over OrderedTableStEph.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 10a. iterators
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 8b. traits
//	Section 9b. impls
//	Section 10b. iterators
//	Section 11a. top level coarse locking
//	Section 12b. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!


//		Section 1. module

pub mod OrderedTableMtEph {

    //		Section 2. imports

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::vstdplus::accept::accept;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger, obeys_view_eq_trigger};
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_fulls;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
};

    //		Section 4a. type definitions


    pub struct OrderedTableMtEphInv;

    pub type OrderedTableMt<K, V> = OrderedTableMtEph<K, V>;

    //		Section 10a. iterators


    pub open spec fn iter_invariant<'a, K, V>(it: &OrderedTableMtEphIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: MtKey, V: MtVal + Ord> IntoIterator for &'a OrderedTableMtEph<K, V> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableMtEphIter<'a, K, V>;

        fn into_iter(self) -> (it: OrderedTableMtEphIter<'a, K, V>)
            requires self.spec_orderedtablemteph_wf(),
            ensures
                it@.0 == 0,
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtEph<K: MtKey, V: MtVal + Ord> {
        pub locked_table: RwLock<OrderedTableStEph<K, V>, OrderedTableMtEphInv>,
        pub ghost_locked_table: Ghost<Map<K::V, V::V>>,
    }

    //		Section 5b. view impls


    impl<K: MtKey, V: MtVal + Ord> View for OrderedTableMtEph<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            self.spec_ghost_locked_table()
        }
    }

    //		Section 8b. traits


    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1 for keys) with multi-threaded ephemeral semantics.
    pub trait OrderedTableMtEphTrait<K: MtKey, V: MtVal + Ord>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablemteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- acquires read lock, delegates to StEph.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablemteph_wf()
            ensures count == self@.dom().len(), self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty StEph + RwLock
        fn empty() -> (empty: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_orderedtablemteph_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps StEph.singleton + RwLock
        fn singleton(k: K, v: V) -> (tree: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablemteph_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StEph.find
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- compares size to 0
        fn is_empty(&self) -> (is_empty: bool)
            requires self.spec_orderedtablemteph_wf()
            ensures is_empty == self@.dom().is_empty();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires write lock, delegates to StEph.insert
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: F)
            requires
                old(self).spec_orderedtablemteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_clone::<K>(),
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires write lock, delegates to StEph.delete
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablemteph_wf(),
                obeys_view_eq::<K>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StEph.domain
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires self.spec_orderedtablemteph_wf(), obeys_feq_clone::<K>()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n^2), Span Θ(n^2) -- delegates to StEph.tabulate (sequential insert loop)
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
                obeys_feq_fulls::<K, V>(),
            ensures tabulated@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires read lock, delegates to StEph.map
        fn map<F: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: F) -> (mapped: Self)
            requires forall|k: &K, v: &V| f.requires((k, v))
            ensures mapped@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires read lock, delegates to StEph.filter
        fn filter<F: Fn(&K, &V) -> bool + Send + Sync + 'static>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
            requires
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures filtered@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- acquires locks, delegates to StEph.intersection
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F)
            requires forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- acquires locks, delegates to StEph.union
        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F)
            requires forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- acquires locks, delegates to StEph.difference
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedtablemteph_wf()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- acquires write lock, delegates to StEph.restrict
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablemteph_wf()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- acquires write lock, delegates to StEph.subtract
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablemteph_wf()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StEph.reduce
        fn reduce<R: StTInMtT + 'static, F: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: F) -> (reduced: R)
            requires forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires read lock, delegates to StEph.collect
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StEph.first_key
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StEph.last_key
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StEph.previous_key
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemteph_wf()
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StEph.next_key
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemteph_wf()
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires write lock, delegates to StEph.split_key
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            requires old(self).spec_orderedtablemteph_wf(), obeys_view_eq::<K>()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union
        fn join_key(&mut self, other: Self)
            requires old(self).spec_orderedtablemteph_wf()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n + m), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + m), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires read lock, delegates to StEph.get_key_range
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires self.spec_orderedtablemteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_orderedtablemteph_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires read lock, delegates to StEph.rank_key
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>()
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires read lock, delegates to StEph.select_key
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>()
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires write lock, delegates to StEph.split_rank_key
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires old(self).spec_orderedtablemteph_wf()
            ensures self@.dom().finite();

        fn iter<'a>(&'a self) -> (it: OrderedTableMtEphIter<'a, K, V>)
            requires self.spec_orderedtablemteph_wf(),
            ensures
                it@.0 == 0,
                iter_invariant(&it);
    }

    //		Section 9b. impls


    impl<K: MtKey, V: MtVal + Ord> OrderedTableMtEph<K, V> {
        pub open spec fn spec_ghost_locked_table(self) -> Map<K::V, V::V> {
            self.ghost_locked_table@
        }
    }


    impl<K: MtKey, V: MtVal + Ord> OrderedTableMtEphTrait<K, V> for OrderedTableMtEph<K, V> {
        open spec fn spec_orderedtablemteph_wf(&self) -> bool {
            self@.dom().finite()
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper, delegates to StEph
        fn size(&self) -> (count: usize) {

            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let count = inner.size();
            proof { assume(count == self@.dom().len()); }
            read_handle.release_read();
            count
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self) {
                      assert(obeys_feq_full_trigger::<K>());
           assert(obeys_feq_full_trigger::<V>());
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let inner = OrderedTableStEph::empty();
            from_st(inner)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(k: K, v: V) -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<K>());
           assert(obeys_feq_full_trigger::<V>());
            proof { assert(obeys_feq_full_trigger::<Pair<K, V>>()); }
            let inner = OrderedTableStEph::singleton(k, v);
            from_st(inner)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- RwLock wrapper, delegates to StEph find
        fn find(&self, k: &K) -> (found: Option<V>) {

            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let found = inner.find(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            found
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>) {
            self.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == self@.dom().is_empty()
        {
            self.size() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph insert
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: F) {
            let ghost old_view = self.ghost_locked_table@;
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            proof { assume(!locked_val@.contains_key(k@) ==> locked_val@.dom().len() + 1 < usize::MAX as nat); }
            locked_val.insert(k, v, combine);
            proof { assume(locked_val.spec_orderedtablesteph_wf()); }
            let ghost new_view = locked_val@;
            write_handle.release_write(locked_val);
            self.ghost_locked_table = Ghost(new_view);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph delete
        fn delete(&mut self, k: &K) -> (updated: Option<V>) {
            proof {

                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let ghost old_view = self.ghost_locked_table@;
            let ghost k_view = k@;
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            let updated = locked_val.delete(k);
            proof { assume(locked_val.spec_orderedtablesteph_wf()); }
            write_handle.release_write(locked_val);
            self.ghost_locked_table = Ghost(old_view.remove(k_view));
            updated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph domain
        fn domain(&self) -> (domain: ArraySetStEph<K>) {
            proof {
                assume(self@.dom().finite());
                assert(obeys_feq_full_trigger::<K>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner.spec_orderedtablesteph_wf()); }
            let domain = inner.domain();
            read_handle.release_read();
            domain
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to StEph tabulate
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self) {
            let inner = OrderedTableStEph::tabulate(f, keys);
            from_st(inner)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StEph map
        fn map<F: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: F) -> (mapped: Self) {
            proof {

                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let result = inner.map(f);
            read_handle.release_read();
            from_st(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StEph filter
        fn filter<F: Fn(&K, &V) -> bool + Send + Sync + 'static>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self) {

            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let result = inner.filter(f, Ghost(spec_pred));
            read_handle.release_read();
            from_st(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- RwLock wrapper, delegates to StEph intersection
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F) {
            proof {

                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_view_eq_trigger::<K>());
            }
            let other_read = other.locked_table.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            locked_val.intersection(other_ref, f);
            proof { assume(locked_val.spec_orderedtablesteph_wf()); }
            let ghost new_view = locked_val@;
            write_handle.release_write(locked_val);
            other_read.release_read();
            self.ghost_locked_table = Ghost(new_view);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- RwLock wrapper, delegates to StEph union
        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F) {
            proof {

                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_view_eq_trigger::<K>());
            }
            let other_read = other.locked_table.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            proof { assume(locked_val@.dom().len() + other_ref@.dom().len() < usize::MAX); }
            locked_val.union(other_ref, f);
            proof { assume(locked_val.spec_orderedtablesteph_wf()); }
            let ghost new_view = locked_val@;
            write_handle.release_write(locked_val);
            other_read.release_read();
            self.ghost_locked_table = Ghost(new_view);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- RwLock wrapper, delegates to StEph difference
        fn difference(&mut self, other: &Self) {
            proof {

                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                assert(obeys_view_eq_trigger::<K>());
            }
            let other_read = other.locked_table.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            proof { assume(other_ref.spec_orderedtablesteph_wf()); }
            locked_val.difference(other_ref);
            proof { assume(locked_val.spec_orderedtablesteph_wf()); }
            let ghost new_view = locked_val@;
            write_handle.release_write(locked_val);
            other_read.release_read();
            self.ghost_locked_table = Ghost(new_view);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- RwLock wrapper, delegates to StEph restrict
        fn restrict(&mut self, keys: &ArraySetStEph<K>) {
            proof {

                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            locked_val.restrict(keys);
            proof { assume(locked_val.spec_orderedtablesteph_wf()); }
            let ghost new_view = locked_val@;
            write_handle.release_write(locked_val);
            self.ghost_locked_table = Ghost(new_view);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- RwLock wrapper, delegates to StEph subtract
        fn subtract(&mut self, keys: &ArraySetStEph<K>) {
            proof {

                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            locked_val.subtract(keys);
            proof { assume(locked_val.spec_orderedtablesteph_wf()); }
            let ghost new_view = locked_val@;
            write_handle.release_write(locked_val);
            self.ghost_locked_table = Ghost(new_view);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph reduce
        fn reduce<R: StTInMtT + 'static, F: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: F) -> (reduced: R) {
            proof { assume(self@.dom().finite()); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner.spec_orderedtablesteph_wf()); }
            let reduced = inner.reduce(init, f);
            read_handle.release_read();
            reduced
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph collect
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>) {
            proof { assume(self@.dom().finite()); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let collected = inner.collect();
            read_handle.release_read();
            collected
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph first_key
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner.spec_orderedtablesteph_wf()); }
            let first = inner.first_key();
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            first
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph last_key
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner.spec_orderedtablesteph_wf()); }
            let last = inner.last_key();
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            last
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph previous_key
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner.spec_orderedtablesteph_wf()); }
            let predecessor = inner.previous_key(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            predecessor
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph next_key
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner.spec_orderedtablesteph_wf()); }
            let successor = inner.next_key(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            successor
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StEph split_key
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            proof { assert(obeys_view_eq_trigger::<K>()); }
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            let (left_inner, found_value, right_inner) = locked_val.split_key(k);
            // Release write lock with empty.
            let empty_val = OrderedTableStEph::empty();
            self.ghost_locked_table = Ghost(empty_val@);
            proof { assume(empty_val.spec_orderedtablesteph_wf()); }
            write_handle.release_write(empty_val);
            proof {
                assume(left_inner.spec_orderedtablesteph_wf());
                assume(right_inner.spec_orderedtablesteph_wf());
            }
            (from_st(left_inner), found_value, from_st(right_inner))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to union
        fn join_key(&mut self, other: Self) {
            self.union(&other, |v1: &V, _v2: &V| v1.clone());
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StEph get_key_range
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self) {

                      assert(obeys_feq_full_trigger::<K>());
           assert(obeys_feq_full_trigger::<V>());
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner@ =~= self@); }
            let st_range = inner.get_key_range(k1, k2);
            read_handle.release_read();
            proof { assume(st_range.spec_orderedtablesteph_wf()); }
            from_st(st_range)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph rank_key
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {

            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let rank = inner.rank_key(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            rank
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StEph select_key
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {

            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let selected = inner.select_key(i);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            selected
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StEph split_rank_key
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let (mut locked_val, write_handle) = self.locked_table.acquire_write();
            let (left, right) = locked_val.split_rank_key(i);
            let empty_val = OrderedTableStEph::empty();
            self.ghost_locked_table = Ghost(empty_val@);
            proof { assume(empty_val.spec_orderedtablesteph_wf()); }
            write_handle.release_write(empty_val);
            proof {
                assume(left.spec_orderedtablesteph_wf());
                assume(right.spec_orderedtablesteph_wf());
            }
            (from_st(left), from_st(right))
        }

        fn iter<'a>(&'a self) -> (it: OrderedTableMtEphIter<'a, K, V>)
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner.spec_orderedtablesteph_wf()); }
            let sorted = inner.tree.inner.in_order();
            let n = sorted.length();
            let mut snapshot: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    n as nat == sorted.spec_len(),
                    0 <= i <= n,
                    snapshot@.len() == i as int,
                decreases n - i,
            {
                let e = sorted.nth(i);
                snapshot.push(Pair(e.0.clone(), e.1.clone()));
                i += 1;
            }
            read_handle.release_read();
            OrderedTableMtEphIter { snapshot, pos: 0, _phantom: core::marker::PhantomData }
        }
    }


    /// Construct Mt wrapper from an St table.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- wraps inner in RwLock
    fn from_st<K: MtKey, V: MtVal + Ord>(inner: OrderedTableStEph<K, V>) -> (s: OrderedTableMtEph<K, V>)
        requires inner@.dom().finite()
        ensures s@ =~= inner@, s@.dom().finite(), s.spec_orderedtablemteph_wf()
    {
        assert(obeys_feq_full_trigger::<K>());
        assert(obeys_feq_full_trigger::<V>());
        assert(obeys_feq_full_trigger::<Pair<K, V>>());
        let ghost view = inner@;
        proof { assume(inner.spec_orderedtablesteph_wf()); }
        OrderedTableMtEph {
            locked_table: RwLock::new(inner, Ghost(OrderedTableMtEphInv)),
            ghost_locked_table: Ghost(view),
        }
    }

    /// Build an MtEph table from entries (used by macro and tests).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to StEph from_sorted_entries
    pub fn from_sorted_entries<K: MtKey, V: MtVal + Ord>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (constructed: OrderedTableMtEph<K, V>)
        requires
            entries.spec_avltreeseqstper_wf(),
            entries@.len() < usize::MAX as nat,
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
            view_ord_consistent::<Pair<K, V>>(),
            spec_pair_key_determines_order::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            view_ord_consistent::<K>(),
            forall|ii: int, jj: int| 0 <= ii < jj < entries@.len()
                ==> (#[trigger] entries@[ii]).0 != (#[trigger] entries@[jj]).0,
        ensures constructed@.dom().finite(), constructed.spec_orderedtablemteph_wf()
    {
        assert(obeys_feq_full_trigger::<K>());
        assert(obeys_feq_full_trigger::<V>());
        assert(obeys_feq_full_trigger::<Pair<K, V>>());
        let inner = crate::Chap43::OrderedTableStEph::OrderedTableStEph::from_sorted_entries(entries);
        from_st(inner)
    }

    //		Section 10b. iterators


    // Entries are behind an RwLock, so the iterator collects a snapshot.
    // All iterator infrastructure uses external_body since entries cannot be
    // borrowed through the lock.

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtEphIter<'a, K, V> {
        pub snapshot: Vec<Pair<K, V>>,
        pub pos: usize,
        pub _phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K, V> View for OrderedTableMtEphIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) {
            (self.pos as int, self.snapshot@)
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

    impl<'a, K: MtKey, V: MtVal + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableMtEphIter<'a, K, V> {
        type GhostIter = OrderedTableMtEphGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableMtEphGhostIterator<'a, K, V> {
            OrderedTableMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: MtKey, V: MtVal + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedTableMtEphGhostIterator<'a, K, V> {
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

    impl<'a, K: MtKey, V: MtVal + Ord> Iterator for OrderedTableMtEphIter<'a, K, V> {
        type Item = Pair<K, V>;

        #[verifier::external_body]
        fn next(&mut self) -> (next: Option<Pair<K, V>>)
            ensures
                ({
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
                }),
        {
            if self.pos < self.snapshot.len() {
                let item = self.snapshot[self.pos].clone();
                self.pos += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    //		Section 11a. top level coarse locking


    impl<K: MtKey, V: MtVal + Ord> RwLockPredicate<OrderedTableStEph<K, V>> for OrderedTableMtEphInv {
        open spec fn inv(self, v: OrderedTableStEph<K, V>) -> bool {
            v.spec_orderedtablesteph_wf()
        }
    }

    //		Section 12b. derive impls in verus!


    impl<K: MtKey, V: MtVal + Ord> Clone for OrderedTableMtEph<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow().clone();
            proof { accept(inner@ == self@); }
            read_handle.release_read();
            let ghost view = inner@;
            proof {
                assume(view.dom().finite());
                assume(inner.spec_orderedtablesteph_wf());
            }
            OrderedTableMtEph {
                locked_table: RwLock::new(inner, Ghost(OrderedTableMtEphInv)),
                ghost_locked_table: Ghost(view),
            }
        }
    }

    } // verus!

    //		Section 13. macros


    /// Macro for creating multi-threaded ephemeral ordered tables from sorted key-value pairs.
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

    //		Section 14. derive impls outside verus!

    use std::fmt;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBSTTrait;
    use crate::Chap41::OrdKeyMap::OrdKeyMap::OrdKeyMapTrait;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;

    //		Section 14a. derive impls outside verus!

    impl fmt::Debug for OrderedTableMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEphInv")
        }
    }

    impl fmt::Display for OrderedTableMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEphInv")
        }
    }

    //		Section 14b. derive impls outside verus!

    // Ghost<Map<K::V, V::V>> contains FnSpec which is not Send/Sync at the type level,
    // but Ghost is erased at runtime (zero-sized). Safe because no actual data crosses threads.
    unsafe impl<K: MtKey, V: MtVal + Ord> Send for OrderedTableMtEph<K, V> {}
    unsafe impl<K: MtKey, V: MtVal + Ord> Sync for OrderedTableMtEph<K, V> {}

    impl<K: MtKey, V: MtVal + Ord> PartialEq for OrderedTableMtEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            let self_read = self.locked_table.acquire_read();
            let other_read = other.locked_table.acquire_read();
            let result = self_read.borrow().tree.size() == other_read.borrow().tree.size();
            other_read.release_read();
            self_read.release_read();
            result
        }
    }

    impl<K: MtKey, V: MtVal + Ord> Default for OrderedTableMtEph<K, V> {
        fn default() -> Self { Self::empty() }
    }

    impl<K: MtKey, V: MtVal + Ord> fmt::Debug for OrderedTableMtEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEph(size: {})", self.size())
        }
    }

    impl<K: MtKey, V: MtVal + Ord> fmt::Display for OrderedTableMtEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEph(size: {})", self.size())
        }
    }

    impl<'a, K: MtKey, V: MtVal + Ord> fmt::Debug for OrderedTableMtEphIter<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OrderedTableMtEphIter").finish()
        }
    }

    impl<'a, K: MtKey, V: MtVal + Ord> fmt::Display for OrderedTableMtEphIter<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEphIter")
        }
    }

    impl<'a, K: MtKey, V: MtVal + Ord> fmt::Debug for OrderedTableMtEphGhostIterator<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OrderedTableMtEphGhostIterator").finish()
        }
    }

    impl<'a, K: MtKey, V: MtVal + Ord> fmt::Display for OrderedTableMtEphGhostIterator<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtEphGhostIterator")
        }
    }
}
