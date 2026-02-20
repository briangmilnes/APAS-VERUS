//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered table implementation extending TableStPer.

pub mod OrderedTableStPer {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStEph::TableStEph::spec_entries_to_map;
    use crate::Chap42::TableStPer::TableStPer::*;
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
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite();
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();
        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite();
        fn find(&self, k: &K) -> (result: Option<V>);
        fn insert(&self, k: K, v: V) -> (result: Self)
            ensures result@.dom().finite();
        fn delete(&self, k: &K) -> (result: Self)
            ensures result@ == self@.remove(k@), result@.dom().finite();
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite();
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite();
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (result: Self)
            ensures result@.dom().finite();
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (result: Self)
            ensures result@.dom().finite();
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@.dom().finite();
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
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
        fn split_key(&self, k: &K) -> (result: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();
        fn join_key(left: &Self, right: &Self) -> (result: Self)
            ensures result@.dom().finite();
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite();
        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite();
        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn split_rank_key(&self, i: N) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite()
        {
            self.base_table.size()
        }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            OrderedTableStPer {
                base_table: TableStPer::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: TableStPer::singleton(k, v),
            }
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (result: Option<V>) {
            self.base_table.find(k)
        }

        #[verifier::external_body]
        fn insert(&self, k: K, v: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.insert(k, v, |_old, new| new.clone()),
            }
        }

        #[verifier::external_body]
        fn delete(&self, k: &K) -> (result: Self)
            ensures result@ == self@.remove(k@), result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.delete(k),
            }
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.domain()
        }

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: TableStPer::tabulate(f, keys),
            }
        }

        #[verifier::external_body]
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.map(f),
            }
        }

        #[verifier::external_body]
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.filter(f),
            }
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.intersection(&other.base_table, f),
            }
        }

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.union(&other.base_table, f),
            }
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.difference(&other.base_table),
            }
        }

        #[verifier::external_body]
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.restrict(keys),
            }
        }

        #[verifier::external_body]
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableStPer {
                base_table: self.base_table.subtract(keys),
            }
        }

        #[verifier::external_body]
        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite()
        {
            let array_seq = self.base_table.collect();
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
        fn split_key(&self, k: &K) -> (result: (Self, Option<V>, Self))
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

            (
                from_sorted_entries(left_seq),
                found_value,
                from_sorted_entries(right_seq),
            )
        }

        #[verifier::external_body]
        fn join_key(left: &Self, right: &Self) -> (result: Self)
            ensures result@.dom().finite()
        {
            left.union(right, |v1, _v2| v1.clone())
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
        fn split_rank_key(&self, i: N) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();

            if i >= size {
                return (self.clone(), Self::empty());
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

            (from_sorted_entries(left_seq), from_sorted_entries(right_seq))
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.clone(),
            }
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> OrderedTableStPer<K, V> {
        let len = entries.length();
        let mut elements = Vec::new();
        for i in 0..len {
            elements.push(entries.nth(i).clone());
        }
        OrderedTableStPer {
            base_table: crate::Chap42::TableStPer::TableStPer::from_sorted_entries(elements),
        }
    }

    } // verus!

    impl<K: StT + Ord, V: StT> PartialEq for OrderedTableStPer<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
        }
    }

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
}
