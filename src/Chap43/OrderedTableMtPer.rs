// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Multi-threaded persistent ordered table using coarse RwLock over OrderedTableStPer.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 8b. traits
//	Section 9b. impls
//	Section 10b. iterators
//	Section 11a. top level coarse locking
//	Section 12b. derive impls in verus!
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!


//		Section 1. module

pub mod OrderedTableMtPer {

    //		Section 2. imports

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::accept::accept;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger, obeys_view_eq_trigger};
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::map::group_map_axioms,
    };

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtPerInv<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> {
        pub ghost expected_view: Map<K::V, V::V>,
    }

    //		Section 9a. impls


    /// Construct Mt wrapper from an St table.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- wraps inner in RwLock
    fn from_st_table<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static>(
        inner: OrderedTableStPer<K, V>,
    ) -> (s: OrderedTableMtPer<K, V>)
        requires inner.spec_orderedtablestper_wf()
        ensures s.spec_orderedtablemtper_wf(), s@ == inner@
    {
        let ghost view = inner@;
        // Veracity: NEEDED proof block
        proof {
            lemma_pair_set_to_map_dom_finite(inner.tree.inner@);
        }
        OrderedTableMtPer {
            locked_table: RwLock::new(inner, Ghost(OrderedTableMtPerInv { expected_view: view })),
            ghost_locked_table: Ghost(view),
        }
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtPer<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> {
        pub(crate) locked_table: RwLock<OrderedTableStPer<K, V>, OrderedTableMtPerInv<K, V>>,
        pub(crate) ghost_locked_table: Ghost<Map<K::V, V::V>>,
    }

    //		Section 5b. view impls


    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> View for OrderedTableMtPer<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            self.spec_ghost_locked_table()
        }
    }

    //		Section 8b. traits


    pub trait OrderedTableMtPerTrait<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablemtper_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- acquires read lock, delegates to StPer.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablemtper_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- constructs empty StPer + RwLock
        fn empty() -> (empty: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_orderedtablemtper_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- wraps StPer.singleton + RwLock
        fn singleton(k: K, v: V) -> (tree: Self)
            requires
                obeys_feq_clone::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree.spec_orderedtablemtper_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.find (linear scan)
        fn find(&self, k: &K) -> (found: Option<V>)
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.insert (linear dup check)
        fn insert(&self, k: K, v: V) -> (updated: Self)
            requires self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                updated@.dom() =~= self@.dom().insert(k@),
                updated.spec_orderedtablemtper_wf();

        /// Like insert, but additionally ensures the inserted value mapping.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to StPer insert_wf
        fn insert_wf(&self, k: K, v: V) -> (updated: Self)
            requires self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                updated@.dom() =~= self@.dom().insert(k@),
                updated@[k@] == v@,
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> updated@[k2] == self@[k2],
                updated.spec_orderedtablemtper_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.delete (linear scan)
        fn delete(&self, k: &K) -> (updated: Self)
            ensures
                updated@ == self@.remove(k@),
                updated.spec_orderedtablemtper_wf();

        /// Like delete, but additionally ensures value preservation for remaining keys.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to StPer delete_wf
        fn delete_wf(&self, k: &K) -> (updated: Self)
            ensures
                updated@ == self@.remove(k@),
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> updated@[k2] == self@[k2],
                updated.spec_orderedtablemtper_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.domain
        fn domain(&self) -> (domain: OrderedSetMtEph<K>)
            requires self.spec_orderedtablemtper_wf(), obeys_feq_clone::<K>()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.map
        fn map<G: Fn(&V) -> V + Send + Sync + 'static>(
            &self, f: G, Ghost(f_spec): Ghost<spec_fn(V::V) -> V::V>,
        ) -> (mapped: Self)
            requires
                forall|v: &V| f.requires((v,)),
                forall|v: V, r: V| f.ensures((&v,), r) ==> r@ == f_spec(v@),
            ensures
                mapped@.dom() =~= self@.dom(),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> mapped@[k] == f_spec(self@[k]),
                mapped.spec_orderedtablemtper_wf();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.filter
        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (filtered: Self)
            requires forall|p: &Pair<K, V>| f.requires((p,))
            ensures filtered@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + first)
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemtper_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + last)
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemtper_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + scan)
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemtper_wf()
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + scan)
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemtper_wf()
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + partition)
        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            requires self.spec_orderedtablemtper_wf(), obeys_view_eq::<K>()
            ensures self@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n + m), Span Θ(n + m) -- acquires lock, delegates to StPer.join_key (union)
        fn join_key(&self, other: &Self) -> (joined: Self)
            requires
                self.spec_orderedtablemtper_wf(),
                other.spec_orderedtablemtper_wf(),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
                self@.dom().len() + other@.dom().len() < usize::MAX,
            ensures joined@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n + m), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + m), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + filter)
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires self.spec_orderedtablemtper_wf()
            ensures range@.dom().finite();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + count)
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires self.spec_orderedtablemtper_wf(), obeys_view_eq::<K>()
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + index)
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablemtper_wf(), obeys_view_eq::<K>()
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + partition)
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires self.spec_orderedtablemtper_wf()
            ensures self@.dom().finite();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- acquires read lock, calls StPer iter, releases lock
        fn iter(&self) -> (it: OrderedTableMtPerIter<K, V>)
            requires self.spec_orderedtablemtper_wf()
            ensures it@.0 == 0, iter_invariant_orderedtablemtper(&it);
    }

    //		Section 9b. impls


    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> OrderedTableMtPer<K, V> {
        #[verifier::type_invariant]
        spec fn inv(self) -> bool {
            self.ghost_locked_table@.dom().finite()
            && self.locked_table.pred().expected_view == self.ghost_locked_table@
        }

        pub closed spec fn spec_ghost_locked_table(self) -> Map<K::V, V::V> {
            self.ghost_locked_table@
        }
    }


    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> OrderedTableMtPerTrait<K, V> for OrderedTableMtPer<K, V> {
        open spec fn spec_orderedtablemtper_wf(&self) -> bool {
            self@.dom().finite()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        // Veracity: NEEDED proof block
        fn size(&self) -> (count: usize) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            // Veracity: NEEDED proof block
            let inner = read_handle.borrow();
            let count = inner.size();
            proof {
                // Predicate chain: inv(pred, inner) → inner@ == pred.expected_view.
                // type_invariant: pred.expected_view == ghost_locked_table@ == self@.
                // inner.size() ensures: count == inner@.dom().len().
                // Veracity: NEEDED assert (speed hint)
                assert(inner@ == self@);
            }
            read_handle.release_read();
            count
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self) {
            let inner = OrderedTableStPer::empty();
            from_st_table(inner)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(k: K, v: V) -> (tree: Self) {
            let inner = OrderedTableStPer::singleton(k, v);
            from_st_table(inner)
        }
// Veracity: NEEDED proof block

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- RwLock wrapper, delegates to StPer find
        fn find(&self, k: &K) -> (found: Option<V>) {
            proof {
                use_type_invariant(self);
                // Veracity: NEEDED assert
                assert(obeys_view_eq_trigger::<K>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let found = inner.find(k);
// Veracity: UNNEEDED assert             proof { assert(inner@ == self@); }
            read_handle.release_read();
            found
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer insert
        fn insert(&self, k: K, v: V) -> (updated: Self) {
            proof {
                use_type_invariant(self);
                // Veracity: NEEDED assert
                // Veracity: NEEDED proof block
                assert(obeys_view_eq_trigger::<K>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED assert (speed hint)
            proof { assert(inner@ == self@); }
            let result = inner.insert(k, v);
            // Veracity: NEEDED proof block
            read_handle.release_read();
            from_st_table(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer insert_wf
        fn insert_wf(&self, k: K, v: V) -> (updated: Self) {
            proof {
                use_type_invariant(self);
                // Veracity: NEEDED assert
                assert(obeys_view_eq_trigger::<K>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
// Veracity: UNNEEDED assert             proof { assert(inner@ == self@); }
            // Veracity: NEEDED proof block
            let result = inner.insert_wf(k, v);
            read_handle.release_read();
            from_st_table(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer delete
        fn delete(&self, k: &K) -> (updated: Self) {
            proof {
                use_type_invariant(self);
                // Veracity: NEEDED assert
                assert(obeys_view_eq_trigger::<K>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert             proof { assert(inner@ == self@); }
            let result = inner.delete(k);
            read_handle.release_read();
            from_st_table(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer delete_wf
        // Veracity: NEEDED proof block
        fn delete_wf(&self, k: &K) -> (updated: Self) {
            proof {
                use_type_invariant(self);
                // Veracity: NEEDED assert
                assert(obeys_view_eq_trigger::<K>());
            }
            let read_handle = self.locked_table.acquire_read();
            // Veracity: NEEDED proof block
            let inner = read_handle.borrow();
            // Veracity: NEEDED assert (speed hint)
            proof { assert(inner@ == self@); }
            let result = inner.delete_wf(k);
            read_handle.release_read();
            // Veracity: NEEDED proof block
            from_st_table(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- collect O(n) + n OrderedSet inserts O(log n) each
        fn domain(&self) -> (domain: OrderedSetMtEph<K>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let entries = inner.collect();
            let len = entries.length();
            read_handle.release_read();
            proof {
                // entries.spec_avltreeseqstper_wf() is guaranteed by collect().
                // lemma_size_lt_usize_max: spec_cached_size(&entries.root) < usize::MAX.
                // lemma_size_eq_inorder_len: spec_cached_size(&entries.root) == entries@.len().
                // entries.length() ensures len as nat == entries@.len().
                // Therefore len < usize::MAX.
                lemma_size_lt_usize_max::<Pair<K, V>>(&entries.root);
                lemma_size_eq_inorder_len::<Pair<K, V>>(&entries.root);
// Veracity: UNNEEDED assert                 assert(len < usize::MAX);
            }
            let mut result = OrderedSetMtEph::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    entries.spec_avltreeseqstper_wf(),
                    i <= len,
                    len as nat == entries.spec_seq().len(),
                    len < usize::MAX,
                    result@.finite(),
                    result@.len() <= i as nat,
                decreases len - i,
            // Veracity: NEEDED proof block
            {
                let pair = entries.nth(i);
                result.insert(pair.0.clone());
                i += 1;
            }
            result
        }

        // Veracity: NEEDED proof block (speed hint)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StPer map
        fn map<G: Fn(&V) -> V + Send + Sync + 'static>(
            // Veracity: NEEDED proof block
            &self, f: G, Ghost(f_spec): Ghost<spec_fn(V::V) -> V::V>,
        ) -> (mapped: Self) {
            proof {
                use_type_invariant(self);
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_view_eq_trigger::<K>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let ghost inner_view = inner@;
            // Veracity: NEEDED assert (speed hint)
            proof { assert(inner@ == self@); }
            let st_result = inner.map(f);
            read_handle.release_read();
            proof {
                // StPer map ensures: st_result@.dom() == inner@.dom()
                // and forall|k| st_result@.contains_key(k) ==>
                //   exists|old_val, result| old_val@ == inner@[k] && f.ensures((&old_val,), result) && st_result@[k] == result@
                // Our requires: forall|v, r| f.ensures((&v,), r) ==> r@ == f_spec(v@)
                // Combined: st_result@[k] == f_spec(inner@[k]) == f_spec(self@[k])
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies st_result@[k] == f_spec(self@[k])
                by {
// Veracity: UNNEEDED assert                     assert(inner_view.contains_key(k));
                    // Veracity: NEEDED assert (speed hint)
                    assert(st_result@.contains_key(k));
                    // Veracity: NEEDED proof block
                    let (old_val, result): (V, V) = choose|old_val: V, result: V|
                        old_val@ == inner_view[k]
                        && f.ensures((&old_val,), result)
                        && st_result@[k] == result@;
                    // Veracity: NEEDED assert (speed hint)
                    assert(f.ensures((&old_val,), result));
                    // Veracity: NEEDED assert (speed hint)
                    assert(result@ == f_spec(old_val@));
                };
            }
            from_st_table(st_result)
        }
// Veracity: NEEDED proof block

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) -- collect O(n) + n StPer inserts each O(n)
        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (filtered: Self) {
            proof {
                // Veracity: NEEDED assert
                assert(obeys_view_eq_trigger::<K>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                use_type_invariant(self);
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let entries = inner.collect();
            read_handle.release_read();
            let mut result = OrderedTableStPer::empty();
            let len = entries.length();
            proof {
                lemma_size_lt_usize_max::<Pair<K, V>>(&entries.root);
                lemma_size_eq_inorder_len::<Pair<K, V>>(&entries.root);
// Veracity: UNNEEDED assert                 assert(len < usize::MAX);
            }
            let mut i: usize = 0;
            while i < len
                invariant
                    entries.spec_avltreeseqstper_wf(),
                    result.spec_orderedtablestper_wf(),
                    result@.dom().finite(),
                    result@.dom().len() <= i as nat,
                    i <= len,
                    len as nat == entries.spec_seq().len(),
                    len < usize::MAX,
                    forall|p: &Pair<K, V>| f.requires((p,)),
                    // Veracity: NEEDED proof block
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                decreases len - i,
            {
                let pair = entries.nth(i);
                if f(pair) {
                    result = result.insert(pair.0.clone(), pair.1.clone());
                }
                i += 1;
            }
            from_st_table(result)
        }
// Veracity: NEEDED proof block (speed hint)

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer first_key
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        // Veracity: NEEDED proof block
        {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let first = inner.first_key();
// Veracity: UNNEEDED assert             proof { assert(inner@ == self@); }
            read_handle.release_read();
            first
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer last_key
        fn last_key(&self) -> (last: Option<K>)
            // Veracity: NEEDED proof block (speed hint)
            where K: TotalOrder
        {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let last = inner.last_key();
            // Veracity: NEEDED assert (speed hint)
            proof { assert(inner@ == self@); }
            read_handle.release_read();
            last
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer previous_key
        // Veracity: NEEDED proof block
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let predecessor = inner.previous_key(k);
// Veracity: UNNEEDED proof block             // Veracity: NEEDED assert (speed hint)
            proof { assert(inner@ == self@); }
            read_handle.release_read();
            predecessor
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer next_key
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let successor = inner.next_key(k);
            // Veracity: NEEDED assert (speed hint)
            proof { assert(inner@ == self@); }
            read_handle.release_read();
            successor
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StPer split_key
        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self)) {
            // Veracity: NEEDED assert (speed hint)
            proof { assert(obeys_view_eq_trigger::<K>()); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let (left, val, right) = inner.split_key(k);
            proof { use_type_invariant(self); }
            read_handle.release_read();
            (from_st_table(left), val, from_st_table(right))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- RwLock wrapper, delegates to StPer join_key (union)
        fn join_key(&self, other: &Self) -> (joined: Self) {
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_view_eq_trigger::<K>());
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                 assert(obeys_feq_full_trigger::<Pair<K, V>>());
                use_type_invariant(self);
                use_type_invariant(other);
            }
            let self_read = self.locked_table.acquire_read();
            let self_inner = self_read.borrow();
            let other_read = other.locked_table.acquire_read();
            let other_inner = other_read.borrow();
            proof {
                // Veracity: NEEDED proof block
                // Veracity: NEEDED assert (speed hint)
                assert(self_inner@ == self@);
                // Veracity: NEEDED assert (speed hint)
                assert(other_inner@ == other@);
            }
            let result = OrderedTableStPer::join_key(self_inner, other_inner);
            other_read.release_read();
            self_read.release_read();
            from_st_table(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StPer get_key_range
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let range = inner.get_key_range(k1, k2);
            read_handle.release_read();
// Veracity: UNNEEDED proof block             proof {
// Veracity: UNNEEDED proof block                 lemma_pair_set_to_map_dom_finite(range.tree.inner@);
// Veracity: UNNEEDED proof block             }
            from_st_table(range)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer rank_key
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_view_eq_trigger::<K>());
                use_type_invariant(self);
            }
            let read_handle = self.locked_table.acquire_read();
            // Veracity: NEEDED proof block
            let inner = read_handle.borrow();
            let rank = inner.rank_key(k);
// Veracity: UNNEEDED assert             proof { assert(inner@ == self@); }
            read_handle.release_read();
            rank
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- RwLock wrapper, delegates to StPer select_key
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            proof {
// Veracity: UNNEEDED assert                 assert(obeys_view_eq_trigger::<K>());
                use_type_invariant(self);
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let selected = inner.select_key(i);
// Veracity: UNNEEDED assert             proof { assert(inner@ == self@); }
            read_handle.release_read();
            selected
        }
// Veracity: UNNEEDED proof block 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, delegates to StPer split_rank_key
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self)) {
            // Veracity: NEEDED proof block
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let (left, right) = inner.split_rank_key(i);
            proof { use_type_invariant(self); }
            read_handle.release_read();
            (from_st_table(left), from_st_table(right))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- acquires read lock, snapshots via StPer iter, releases lock
        fn iter(&self) -> (it: OrderedTableMtPerIter<K, V>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            proof { assert(inner.spec_orderedtablestper_wf()); }
            let st_iter = inner.iter();
            read_handle.release_read();
            OrderedTableMtPerIter { inner: st_iter }
        }
    }

    //		Section 10b. iterators


    pub open spec fn iter_invariant_orderedtablemtper<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static>(
        it: &OrderedTableMtPerIter<K, V>
    ) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtPerIter<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> {
        pub inner: OrderedTableStPerIter<K, V>,
    }

    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> View for OrderedTableMtPerIter<K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) {
            self.inner@
        }
    }

    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> std::iter::Iterator for OrderedTableMtPerIter<K, V> {
        type Item = Pair<K, V>;
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
            self.inner.next()
        }
    }

    /// Ghost iterator for for-loop support over OrderedTableMtPerIter.
    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtPerGhostIterator<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
    }

    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> View for OrderedTableMtPerGhostIterator<K, V> {
        type V = Seq<Pair<K, V>>;
        open spec fn view(&self) -> Seq<Pair<K, V>> { self.elements.take(self.pos) }
    }

    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableMtPerIter<K, V> {
        type GhostIter = OrderedTableMtPerGhostIterator<K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableMtPerGhostIterator<K, V> {
            OrderedTableMtPerGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> vstd::pervasive::ForLoopGhostIterator for OrderedTableMtPerGhostIterator<K, V> {
        type ExecIter = OrderedTableMtPerIter<K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableMtPerIter<K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableMtPerIter<K, V>) -> OrderedTableMtPerGhostIterator<K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> std::iter::IntoIterator for &'a OrderedTableMtPer<K, V> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableMtPerIter<K, V>;
        fn into_iter(self) -> (it: OrderedTableMtPerIter<K, V>)
            requires self.spec_orderedtablemtper_wf()
            ensures it@.0 == 0, iter_invariant_orderedtablemtper(&it),
        {
            self.iter()
        }
    }

    //		Section 11a. top level coarse locking


    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> RwLockPredicate<OrderedTableStPer<K, V>> for OrderedTableMtPerInv<K, V> {
        open spec fn inv(self, v: OrderedTableStPer<K, V>) -> bool {
            v.spec_orderedtablestper_wf()
            && v@ == self.expected_view
        }
    }

    //		Section 12b. derive impls in verus!


    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> Clone for OrderedTableMtPer<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow().clone();
            proof {
                // Predicate chain: borrow()@ == read_handle.view(),
                // inv(pred, borrow()@) gives borrow()@.spec_wf() && borrow()@@ == pred.expected_view.
                // Clone ensures inner@ == borrow()@@.
                // But clone of OrderedTableStPer... need to check its ensures.
                accept(inner@ == self@);
                accept(inner.spec_orderedtablestper_wf());
            }
            read_handle.release_read();
            from_st_table(inner)
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    use std::fmt;

    //		Section 14a. derive impls outside verus!


    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> fmt::Debug for OrderedTableMtPerInv<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtPerInv")
        }
    }

    impl<K: MtKey + TotalOrder + 'static, V: StTInMtT + Ord + 'static> fmt::Display for OrderedTableMtPerInv<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtPerInv")
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<K: MtKey + TotalOrder + 'static, V: MtKey + 'static> Default for OrderedTableMtPer<K, V> {
        fn default() -> Self { Self::empty() }
    }

    impl<K: MtKey + TotalOrder + 'static, V: MtKey + 'static> fmt::Debug for OrderedTableMtPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtPer(size: {})", self.size())
        }
    }

    impl<K: MtKey + TotalOrder + 'static, V: MtKey + 'static> fmt::Display for OrderedTableMtPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtPer(size: {})", self.size())
        }
    }
}
