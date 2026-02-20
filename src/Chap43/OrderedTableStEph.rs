//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered table implementation extending TableStEph.

pub mod OrderedTableStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Types::Types::*;
    use vstd::prelude::*;

    verus! {

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
        pub base_table: TableStEph<K, V>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for OrderedTableStEph<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { self.base_table@ }
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with ephemeral semantics.
    pub trait OrderedTableStEphTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite();
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();
        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite();
        fn find(&self, k: &K) -> (result: Option<V>);
        fn lookup(&self, k: &K) -> (result: Option<V>);
        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty();
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            ensures self@.dom().finite();
        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@ == old(self)@.remove(k@), self@.dom().finite();
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite();
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite();
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (result: R)
            ensures self@.dom().finite();
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            ensures self@.dom().finite();
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            ensures self@.dom().finite();
        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite();
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite();
        fn first_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn last_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn previous_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn next_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn split_key(&mut self, k: &K) -> (result: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();
        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite();
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite();
        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite();
        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn split_rank_key(&mut self, i: N) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite()
        {
            self.base_table.size()
        }

        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            OrderedTableStEph {
                base_table: TableStEph::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite()
        {
            OrderedTableStEph {
                base_table: TableStEph::singleton(k, v),
            }
        }

        fn find(&self, k: &K) -> (result: Option<V>) {
            self.base_table.find(k)
        }

        fn lookup(&self, k: &K) -> (result: Option<V>) {
            self.find(k)
        }

        #[verifier::external_body]
        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty()
        {
            self.size() == 0
        }

        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            ensures self@.dom().finite()
        {
            self.base_table.insert(k, v, combine);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        #[verifier::external_body]
        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@ == old(self)@.remove(k@), self@.dom().finite()
        {
            let old_value = self.find(k);
            self.base_table.delete(k);
            old_value
        }

        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            self.base_table.domain()
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            let base = TableStEph::tabulate(f, keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStEph { base_table: base }
        }

        #[verifier::external_body]
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite()
        {
            let mut res = OrderedTableStEph::empty();
            let entries = self.collect();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                res.base_table
                    .insert(pair.0.clone(), new_value, |_old, new| new.clone());
            }
            res
        }

        #[verifier::external_body]
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite()
        {
            let mut res = OrderedTableStEph::empty();
            let entries = self.collect();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                if f(&pair.0, &pair.1) {
                    res.base_table
                        .insert(pair.0.clone(), pair.1.clone(), |_old, new| new.clone());
                }
            }
            res
        }

        #[verifier::external_body]
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (result: R)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let mut acc = init;
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                acc = f(acc, &pair.0, &pair.1);
            }
            acc
        }

        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            ensures self@.dom().finite()
        {
            self.base_table.intersection(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
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
        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite()
        {
            let array_seq = self.base_table.entries();
            let len = array_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(elements)
        }

        #[verifier::external_body]
        fn first_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            if entries.length() == 0 {
                None
            } else {
                Some(entries.nth(0).0.clone())
            }
        }

        #[verifier::external_body]
        fn last_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            if size == 0 {
                None
            } else {
                Some(entries.nth(size - 1).0.clone())
            }
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (result: Option<K>)
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
        fn next_key(&self, k: &K) -> (result: Option<K>)
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
        fn split_key(&mut self, k: &K) -> (result: (Self, Option<V>, Self))
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut left_entries = Vec::new();
            let mut right_entries = Vec::new();
            let mut found_value: Option<V> = None;

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

            (
                from_sorted_entries(left_seq),
                found_value,
                from_sorted_entries(right_seq),
            )
        }

        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite()
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        #[verifier::external_body]
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite()
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
        fn rank_key(&self, k: &K) -> (result: N)
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

        #[verifier::external_body]
        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            if i >= entries.length() {
                None
            } else {
                Some(entries.nth(i).0.clone())
            }
        }

        #[verifier::external_body]
        fn split_rank_key(&mut self, i: N) -> (result: (Self, Self))
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

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> (result: Self) {
            OrderedTableStEph {
                base_table: self.base_table.clone(),
            }
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (result: OrderedTableStEph<K, V>) {
        let len = entries.length();
        let mut elements = Vec::new();
        for i in 0..len {
            elements.push(entries.nth(i).clone());
        }
        OrderedTableStEph {
            base_table: crate::Chap42::TableStEph::TableStEph::from_sorted_entries(elements),
        }
    }

    } // verus!

    impl<K: StT + Ord, V: StT> PartialEq for OrderedTableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
        }
    }

    /// Macro for creating ephemeral ordered tables from sorted key-value pairs
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
