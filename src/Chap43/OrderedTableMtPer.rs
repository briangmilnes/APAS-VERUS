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
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

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

    // 6. spec fns

    /// Construct Mt wrapper from an St table.
    fn from_st_table<K: MtKey + 'static, V: StTInMtT + Ord + 'static>(
        inner: OrderedTableStPer<K, V>,
    ) -> (s: OrderedTableMtPer<K, V>)
        requires true
        ensures s@.dom().finite()
    {
        let ghost view = inner@;
        proof {
            accept(view.dom().finite());
            accept(inner.spec_orderedtablestper_wf());
        }
        OrderedTableMtPer {
            locked_table: RwLock::new(inner, Ghost(OrderedTableMtPerInv)),
            ghost_locked_table: Ghost(view),
        }
    }

    // 5. view impls

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> View for OrderedTableMtPer<K, V> {
        type V = Map<K::V, V::V>;
        closed spec fn view(&self) -> Map<K::V, V::V> {
            self.ghost_locked_table@
        }
    }

    // 8. traits

    pub trait OrderedTableMtPerTrait<K: MtKey + 'static, V: StTInMtT + Ord + 'static>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablemtper_wf(&self) -> bool;

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite();

        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_orderedtablemtper_wf();

        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablemtper_wf();

        fn find(&self, k: &K) -> (found: Option<V>);

        fn insert(&self, k: K, v: V) -> (updated: Self)
            ensures updated@.dom().finite();

        fn delete(&self, k: &K) -> (updated: Self)
            ensures updated@.dom().finite();

        fn domain(&self) -> (domain: OrderedSetMtEph<K>)
            ensures self@.dom().finite();

        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (mapped: Self)
            ensures mapped@.dom().finite();

        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (filtered: Self)
            ensures filtered@.dom().finite();

        fn first_key(&self) -> (first: Option<K>)
            ensures self@.dom().finite();

        fn last_key(&self) -> (last: Option<K>)
            ensures self@.dom().finite();

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            ensures self@.dom().finite();

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            ensures self@.dom().finite();

        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();

        fn join_key(&self, other: &Self) -> (joined: Self)
            ensures joined@.dom().finite();

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite();

        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures self@.dom().finite();

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            ensures self@.dom().finite();

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
            proof { accept(inner.spec_orderedtablestper_wf()); }
            let count = inner.size();
            proof { accept(count == self@.dom().len()); }
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            count
        }

        fn empty() -> (empty: Self) {
            let inner = OrderedTableStPer::empty();
            let ghost view = inner@;
            proof { accept(inner.spec_orderedtablestper_wf()); }
            OrderedTableMtPer {
                locked_table: RwLock::new(inner, Ghost(OrderedTableMtPerInv)),
                ghost_locked_table: Ghost(view),
            }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V) -> (tree: Self) {
            let inner = OrderedTableStPer::singleton(k, v);
            from_st_table(inner)
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (found: Option<V>) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let found = inner.find(k);
            read_handle.release_read();
            found
        }

        #[verifier::external_body]
        fn insert(&self, k: K, v: V) -> (updated: Self) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let result = inner.insert(k, v);
            read_handle.release_read();
            from_st_table(result)
        }

        #[verifier::external_body]
        fn delete(&self, k: &K) -> (updated: Self) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let result = inner.delete(k);
            read_handle.release_read();
            from_st_table(result)
        }

        #[verifier::external_body]
        fn domain(&self) -> (domain: OrderedSetMtEph<K>) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let inner_clone = inner.clone();
            read_handle.release_read();
            let mut result = OrderedSetMtEph::empty();
            for entry in &inner_clone {
                result.insert(entry.0.clone());
            }
            result
        }

        #[verifier::external_body]
        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (mapped: Self) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let inner_clone = inner.clone();
            read_handle.release_read();
            let mut result = OrderedTableStPer::empty();
            for entry in &inner_clone {
                let new_v = f(&entry.0, &entry.1);
                result = result.insert(entry.0.clone(), new_v);
            }
            from_st_table(result)
        }

        #[verifier::external_body]
        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (filtered: Self) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let inner_clone = inner.clone();
            read_handle.release_read();
            let mut result = OrderedTableStPer::empty();
            for entry in &inner_clone {
                if f(entry) {
                    result = result.insert(entry.0.clone(), entry.1.clone());
                }
            }
            from_st_table(result)
        }

        fn first_key(&self) -> (first: Option<K>) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let first = inner.first_key();
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            first
        }

        fn last_key(&self) -> (last: Option<K>) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let last = inner.last_key();
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            last
        }

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let predecessor = inner.previous_key(k);
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            predecessor
        }

        fn next_key(&self, k: &K) -> (successor: Option<K>) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let successor = inner.next_key(k);
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            successor
        }

        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self)) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let (left, val, right) = inner.split_key(k);
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            (from_st_table(left), val, from_st_table(right))
        }

        #[verifier::external_body]
        fn join_key(&self, other: &Self) -> (joined: Self) {
            let other_read = other.locked_table.acquire_read();
            let other_inner = other_read.borrow().clone();
            other_read.release_read();
            let self_read = self.locked_table.acquire_read();
            let self_inner = self_read.borrow();
            let result = OrderedTableStPer::join_key(self_inner, &other_inner);
            self_read.release_read();
            from_st_table(result)
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let range = inner.get_key_range(k1, k2);
            read_handle.release_read();
            from_st_table(range)
        }

        fn rank_key(&self, k: &K) -> (rank: usize) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let rank = inner.rank_key(k);
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            rank
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let selected = inner.select_key(i);
            proof { accept(self@.dom().finite()); }
            read_handle.release_read();
            selected
        }

        fn split_rank_key(&self, i: usize) -> (split: (Self, Self)) {
            let read_handle = self.locked_table.acquire_read();
            let inner = read_handle.borrow();
            let (left, right) = inner.split_rank_key(i);
            proof { accept(self@.dom().finite()); }
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
            proof { accept(inner@ == self@); }
            read_handle.release_read();
            let ghost view = inner@;
            proof {
                accept(view.dom().finite());
                accept(inner.spec_orderedtablestper_wf());
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
