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
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;

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

        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablemteph_wf()
            ensures count == self@.dom().len(), self@.dom().finite();

        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_orderedtablemteph_wf();

        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablemteph_wf();

        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@),
                    None => !self@.contains_key(k@),
                };

        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_orderedtablemteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@),
                    None => !self@.contains_key(k@),
                };

        fn is_empty(&self) -> (is_empty: B)
            requires self.spec_orderedtablemteph_wf()
            ensures is_empty == self@.dom().is_empty();

        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: F)
            ensures self@.dom().finite();

        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablemteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<V>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite();

        fn domain(&self) -> (domain: ArraySetStEph<K>)
            ensures self@.dom().finite();

        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires keys@.finite()
            ensures tabulated@.dom().finite();

        fn map<F: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: F) -> (mapped: Self)
            ensures mapped@.dom().finite();

        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&self, f: F) -> (filtered: Self)
            ensures filtered@.dom().finite();

        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F)
            ensures self@.dom().finite();

        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: F)
            ensures self@.dom().finite();

        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite();

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();

        fn reduce<R: StTInMtT + 'static, F: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite();

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf();

        fn first_key(&self) -> (first: Option<K>)
            ensures self@.dom().finite();

        fn last_key(&self) -> (last: Option<K>)
            ensures self@.dom().finite();

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            ensures self@.dom().finite();

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            ensures self@.dom().finite();

        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();

        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite();

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite();

        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures self@.dom().finite();

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            ensures self@.dom().finite();

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

        #[verifier::external_body]
        fn map<F>(&self, f: F) -> (mapped: Self)
            where F: Fn(&K, &V) -> V + Send + Sync + 'static
            ensures mapped@.dom().finite()
        {
            let mut mapped = self.clone();
            let entries = self.collect();
            mapped.base_table = TableMtEph::empty();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                mapped
                    .base_table
                    .insert(pair.0.clone(), new_value, |_old, new| new.clone());
            }
            mapped
        }

        #[verifier::external_body]
        fn filter<F>(&self, f: F) -> (filtered: Self)
            where F: Fn(&K, &V) -> B + Send + Sync + 'static
            ensures filtered@.dom().finite()
        {
            let mut filtered = OrderedTableMtEph::empty();
            let entries = self.collect();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                if f(&pair.0, &pair.1) {
                    filtered
                        .base_table
                        .insert(pair.0.clone(), pair.1.clone(), |_old, new| new.clone());
                }
            }
            filtered
        }

        #[verifier::external_body]
        fn reduce<R: StTInMtT + 'static, F: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let mut reduced = init;
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                reduced = f(reduced, &pair.0, &pair.1);
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

        #[verifier::external_body]
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf()
        {
            let array_seq = self.base_table.entries();
            let len = array_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(elements)
        }

        fn first_key(&self) -> (first: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if entries.length() == 0 {
                None
            } else {
                Some(entries.nth(0).0.clone())
            }
        }

        fn last_key(&self) -> (last: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if size == 0 {
                None
            } else {
                Some(entries.nth(size - 1).0.clone())
            }
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();

            for i in (0..size).rev() {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    return Some(pair.0.clone());
                }
            }
            None
        }

        #[verifier::external_body]
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 > k {
                    return Some(pair.0.clone());
                }
            }
            None
        }

        #[verifier::external_body]
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut left_entries = Vec::new();
            let mut right_entries = Vec::new();
            let mut found_value = None;

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    left_entries.push(pair.clone());
                } else if &pair.0 > k {
                    right_entries.push(pair.clone());
                } else {
                    found_value = Some(pair.1.clone());
                }
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

        #[verifier::external_body]
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut range_entries = Vec::new();

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 >= k1 && &pair.0 <= k2 {
                    range_entries.push(pair.clone());
                }
            }

            let range_seq = AVLTreeSeqStPerS::from_vec(range_entries);
            from_sorted_entries(range_seq)
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut count = 0;

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    count += 1;
                } else {
                    break;
                }
            }
            count
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if i >= entries.length() {
                None
            } else {
                Some(entries.nth(i).0.clone())
            }
        }

        #[verifier::external_body]
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

            let mut left_entries = Vec::new();
            let mut right_entries = Vec::new();

            for j in 0..i {
                left_entries.push(entries.nth(j).clone());
            }
            for j in i..size {
                right_entries.push(entries.nth(j).clone());
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

    #[verifier::external_body]
    pub fn from_sorted_entries<K: MtKey, V: MtVal>(entries: AVLTreeSeqStPerS<Pair<K, V>>) -> (constructed: OrderedTableMtEph<K, V>)
        ensures constructed@.dom().finite()
    {
        let len = entries.length();
        let mut elements = Vec::new();
        for i in 0..len {
            elements.push(entries.nth(i).clone());
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
