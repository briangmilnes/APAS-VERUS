//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent ordered table using coarse RwLock over OrderedTableStPer.

pub mod OrderedTableMtPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, obeys_view_eq_trigger};
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;

    verus! {

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::map::group_map_axioms,
    };

    // 4. type definitions

    pub struct OrderedTableMtPerInv;

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> RwLockPredicate<OrderedTableStPer<K, V>> for OrderedTableMtPerInv {
        open spec fn inv(self, v: OrderedTableStPer<K, V>) -> bool {
            v.spec_orderedtablestper_wf()
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtPer<K: MtKey + 'static, V: StTInMtT + Ord + 'static> {
        pub(crate) locked_table: RwLock<OrderedTableStPer<K, V>, OrderedTableMtPerInv>,
        pub(crate) ghost_locked_table: Ghost<Map<K::V, V::V>>,
    }

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> OrderedTableMtPer<K, V> {
        #[verifier::type_invariant]
        spec fn inv(self) -> bool {
            self.ghost_locked_table@.dom().finite()
        }

        pub closed spec fn spec_ghost_locked_table(self) -> Map<K::V, V::V> {
            self.ghost_locked_table@
        }
    }

    // 6. spec fns

    /// Construct Mt wrapper from an St table.
    fn from_st_table<K: MtKey + 'static, V: StTInMtT + Ord + 'static>(
        inner: OrderedTableStPer<K, V>,
    ) -> (s: OrderedTableMtPer<K, V>)
        requires inner@.dom().finite()
        ensures s@.dom().finite(), s.spec_orderedtablemtper_wf()
    {
        let ghost view = inner@;
        proof {
            assume(inner.spec_orderedtablestper_wf());
        }
        OrderedTableMtPer {
            locked_table: RwLock::new(inner, Ghost(OrderedTableMtPerInv)),
            ghost_locked_table: Ghost(view),
        }
    }

    // 5. view impls

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> View for OrderedTableMtPer<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            self.spec_ghost_locked_table()
        }
    }

    // 8. traits

    pub trait OrderedTableMtPerTrait<K: MtKey + 'static, V: StTInMtT + Ord + 'static>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablemtper_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- acquires read lock, delegates to StPer.size
        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty StPer + RwLock
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_orderedtablemtper_wf();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps StPer.singleton + RwLock
        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablemtper_wf();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.find (linear scan)
        fn find(&self, k: &K) -> (found: Option<V>);

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.insert (linear dup check)
        fn insert(&self, k: K, v: V) -> (updated: Self)
            ensures updated@.dom().finite();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.delete (linear scan)
        fn delete(&self, k: &K) -> (updated: Self)
            ensures updated@.dom().finite();

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.domain
        fn domain(&self) -> (domain: OrderedSetMtEph<K>)
            ensures self@.dom().finite();

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.map
        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (mapped: Self)
            requires forall|k: &K, v: &V| f.requires((k, v))
            ensures mapped@.dom().finite();

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- acquires read lock, delegates to StPer.filter
        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (filtered: Self)
            requires forall|p: &Pair<K, V>| f.requires((p,))
            ensures filtered@.dom().finite();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + first)
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + last)
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + scan)
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + scan)
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + partition)
        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();

        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- acquires lock, delegates to StPer.join_key (union)
        fn join_key(&self, other: &Self) -> (joined: Self)
            ensures joined@.dom().finite();

        /// - APAS: Work Θ(log n + m), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + filter)
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + count)
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + index)
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- acquires lock, delegates to StPer (collect + partition)
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> OrderedTableMtPerTrait<K, V> for OrderedTableMtPer<K, V> {
        open spec fn spec_orderedtablemtper_wf(&self) -> bool {
            self@.dom().finite()
        }

        fn size(&self) -> (count: usize) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let count = inner.size();
            proof { assume(count == self@.dom().len()); }
            proof { use_type_invariant(self); }
            read_handle.release_read();
            count
        }

        fn empty() -> (empty: Self) {
            let inner = OrderedTableStPer::empty();
            let ghost view = inner@;
            OrderedTableMtPer {
                locked_table: RwLock::new(inner, Ghost(OrderedTableMtPerInv)),
                ghost_locked_table: Ghost(view),
            }
        }

        fn singleton(k: K, v: V) -> (tree: Self) {
            proof { assert(obeys_feq_full_trigger::<Pair<K, V>>()); }
            let inner = OrderedTableStPer::singleton(k, v);
            let ghost view = inner@;
            OrderedTableMtPer {
                locked_table: RwLock::new(inner, Ghost(OrderedTableMtPerInv)),
                ghost_locked_table: Ghost(view),
            }
        }

        fn find(&self, k: &K) -> (found: Option<V>) {
            proof {
                assert(obeys_view_eq_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let found = inner.find(k);
            read_handle.release_read();
            found
        }

        fn insert(&self, k: K, v: V) -> (updated: Self) {
            proof {
                assert(obeys_view_eq_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let result = inner.insert(k, v);
            read_handle.release_read();
            let ghost view = result@;
            OrderedTableMtPer {
                locked_table: RwLock::new(result, Ghost(OrderedTableMtPerInv)),
                ghost_locked_table: Ghost(view),
            }
        }

        fn delete(&self, k: &K) -> (updated: Self) {
            proof {
                assert(obeys_view_eq_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let result = inner.delete(k);
            read_handle.release_read();
            let ghost view = result@;
            OrderedTableMtPer {
                locked_table: RwLock::new(result, Ghost(OrderedTableMtPerInv)),
                ghost_locked_table: Ghost(view),
            }
        }

        fn domain(&self) -> (domain: OrderedSetMtEph<K>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let entries = inner.collect();
            read_handle.release_read();
            let mut result = OrderedSetMtEph::empty();
            let len = entries.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    entries.spec_avltreeseqstper_wf(),
                    i <= len,
                    len as nat == entries.spec_seq().len(),
                    result@.finite(),
                decreases len - i,
            {
                let pair = entries.nth(i);
                result.insert(pair.0.clone());
                i += 1;
            }
            result
        }

        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (mapped: Self) {
            proof {
                assert(obeys_view_eq_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                use_type_invariant(self);
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let entries = inner.collect();
            read_handle.release_read();
            let mut result = OrderedTableStPer::empty();
            let len = entries.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    entries.spec_avltreeseqstper_wf(),
                    result.spec_orderedtablestper_wf(),
                    result@.dom().finite(),
                    i <= len,
                    len as nat == entries.spec_seq().len(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                decreases len - i,
            {
                let pair = entries.nth(i);
                let new_v = f(&pair.0, &pair.1);
                result = result.insert(pair.0.clone(), new_v);
                i += 1;
            }
            from_st_table(result)
        }

        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (filtered: Self) {
            proof {
                assert(obeys_view_eq_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                use_type_invariant(self);
            }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let entries = inner.collect();
            read_handle.release_read();
            let mut result = OrderedTableStPer::empty();
            let len = entries.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    entries.spec_avltreeseqstper_wf(),
                    result.spec_orderedtablestper_wf(),
                    result@.dom().finite(),
                    i <= len,
                    len as nat == entries.spec_seq().len(),
                    forall|p: &Pair<K, V>| f.requires((p,)),
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

        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let first = inner.first_key();
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            first
        }

        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let last = inner.last_key();
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            last
        }

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let predecessor = inner.previous_key(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            predecessor
        }

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let successor = inner.next_key(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            successor
        }

        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self)) {
            proof { assert(obeys_view_eq_trigger::<K>()); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let (left, val, right) = inner.split_key(k);
            proof { use_type_invariant(self); }
            read_handle.release_read();
            (from_st_table(left), val, from_st_table(right))
        }

        fn join_key(&self, other: &Self) -> (joined: Self) {
            proof {
                assert(obeys_view_eq_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let self_read = self.locked_table.acquire_read();
            let self_inner = self_read.borrow();
            let other_read = other.locked_table.acquire_read();
            let other_inner = other_read.borrow();
            let result = OrderedTableStPer::join_key(self_inner, other_inner);
            other_read.release_read();
            self_read.release_read();
            let ghost view = result@;
            OrderedTableMtPer {
                locked_table: RwLock::new(result, Ghost(OrderedTableMtPerInv)),
                ghost_locked_table: Ghost(view),
            }
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let range = inner.get_key_range(k1, k2);
            read_handle.release_read();
            from_st_table(range)
        }

        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            proof { assert(obeys_view_eq_trigger::<K>()); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let rank = inner.rank_key(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            rank
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            proof { assert(obeys_view_eq_trigger::<K>()); }
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let selected = inner.select_key(i);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            selected
        }

        fn split_rank_key(&self, i: usize) -> (split: (Self, Self)) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let (left, right) = inner.split_rank_key(i);
            proof { use_type_invariant(self); }
            read_handle.release_read();
            (from_st_table(left), from_st_table(right))
        }
    }

    // 11. derive impls in verus!

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> Clone for OrderedTableMtPer<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow().clone();
            proof { assume(inner@ == self@); }
            read_handle.release_read();
            let ghost view = inner@;
            proof {
                assume(view.dom().finite());
                assume(inner.spec_orderedtablestper_wf());
            }
            OrderedTableMtPer {
                locked_table: RwLock::new(inner, Ghost(OrderedTableMtPerInv)),
                ghost_locked_table: Ghost(view),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    use std::fmt;

    impl<K: MtKey + 'static, V: MtKey + 'static> Default for OrderedTableMtPer<K, V> {
        fn default() -> Self { Self::empty() }
    }

    impl<K: MtKey + 'static, V: MtKey + 'static> fmt::Debug for OrderedTableMtPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtPer(size: {})", self.size())
        }
    }

    impl<K: MtKey + 'static, V: MtKey + 'static> fmt::Display for OrderedTableMtPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableMtPer(size: {})", self.size())
        }
    }
}
