//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral ordered table implementation extending TableMtEph.

pub mod OrderedTableMtEph {

    use std::sync::Arc;
    use std::thread;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableMtEph::TableMtEph::*;
    use crate::Types::Types::*;

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

    pub struct OrderedTableMtEph<K: MtKey, V: MtVal> {
        base_table: TableMtEph<K, V>,
    }

    pub type OrderedTableMt<K, V> = OrderedTableMtEph<K, V>;

    // 5. view impls

    impl<K: MtKey, V: MtVal> View for OrderedTableMtEph<K, V> {
        type V = Map<K::V, V::V>;

        #[verifier::external_body]
        open spec fn view(&self) -> Map<K::V, V::V> {
            Map::empty()
        }
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1 for keys) with multi-threaded ephemeral semantics
    pub trait OrderedTableMtEphTrait<K: MtKey, V: MtVal> {
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite();

        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();

        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite();

        fn find(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result.is_none();

        fn lookup(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result.is_none();

        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty();

        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: F)
            ensures self@.dom().finite();

        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@ == old(self)@.remove(k@), self@.dom().finite();

        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite();

        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();

        fn map<F: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite();

        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite();

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

        fn reduce<R: StTInMtT + 'static, F: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: F) -> (result: R)
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

        fn split_key(&mut self, k: &K) -> (Self, Option<V>, Self)
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

        fn split_rank_key(&mut self, i: N) -> (Self, Self)
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: MtKey, V: MtVal> OrderedTableMtEphTrait<K, V> for OrderedTableMtEph<K, V> {
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
            OrderedTableMtEph {
                base_table: TableMtEph::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite()
        {
            OrderedTableMtEph {
                base_table: TableMtEph::singleton(k, v),
            }
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result.is_none()
        {
            self.base_table.find(k)
        }

        #[verifier::external_body]
        fn lookup(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result.is_none()
        {
            self.find(k)
        }

        #[verifier::external_body]
        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty()
        {
            self.size() == 0
        }

        #[verifier::external_body]
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: F)
            ensures self@.dom().finite()
        {
            self.base_table.insert(k, v, combine);
        }

        #[verifier::external_body]
        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@ == old(self)@.remove(k@), self@.dom().finite()
        {
            let old_value = self.find(k);
            self.base_table.delete(k);
            old_value
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.domain()
        }

        #[verifier::external_body]
        fn tabulate<F>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            where F: Fn(&K) -> V + Send + Sync + 'static
            ensures result@.dom().finite()
        {
            OrderedTableMtEph {
                base_table: TableMtEph::tabulate(f, keys),
            }
        }

        #[verifier::external_body]
        fn map<F>(&self, f: F) -> (result: Self)
            where F: Fn(&K, &V) -> V + Send + Sync + 'static
            ensures result@.dom().finite()
        {
            let mut result = self.clone();
            let entries = self.collect();
            result.base_table = TableMtEph::empty();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                result
                    .base_table
                    .insert(pair.0.clone(), new_value, |_old, new| new.clone());
            }
            result
        }

        #[verifier::external_body]
        fn filter<F>(&self, f: F) -> (result: Self)
            where F: Fn(&K, &V) -> B + Send + Sync + 'static
            ensures result@.dom().finite()
        {
            let mut result = OrderedTableMtEph::empty();
            let entries = self.collect();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                if f(&pair.0, &pair.1) {
                    result
                        .base_table
                        .insert(pair.0.clone(), pair.1.clone(), |_old, new| new.clone());
                }
            }
            result
        }

        #[verifier::external_body]
        fn reduce<R, F>(&self, init: R, f: F) -> (result: R)
            where F: Fn(R, &K, &V) -> R + Send + Sync + 'static, R: Send + Sync + 'static
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let mut result = init;
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                result = f(result, &pair.0, &pair.1);
            }
            result
        }

        #[verifier::external_body]
        fn intersection<F>(&mut self, other: &Self, f: F)
            where F: Fn(&V, &V) -> V + Send + Sync + 'static
            ensures self@.dom().finite()
        {
            self.base_table.intersection(&other.base_table, f);
        }

        #[verifier::external_body]
        fn union<F>(&mut self, other: &Self, f: F)
            where F: Fn(&V, &V) -> V + Send + Sync + 'static
            ensures self@.dom().finite()
        {
            self.base_table.union(&other.base_table, f);
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            self.base_table.difference(&other.base_table);
        }

        #[verifier::external_body]
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.restrict(keys);
        }

        #[verifier::external_body]
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.subtract(keys);
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
        fn split_key(&mut self, k: &K) -> (Self, Option<V>, Self)
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

        #[verifier::external_body]
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
        fn split_rank_key(&mut self, i: N) -> (Self, Self)
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

    // 11. derive impls in verus!

    impl<K: MtKey, V: MtVal> Clone for OrderedTableMtEph<K, V> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            OrderedTableMtEph {
                base_table: self.base_table.clone(),
            }
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_entries<K: MtKey, V: MtVal>(entries: AVLTreeSeqStPerS<Pair<K, V>>) -> (result: OrderedTableMtEph<K, V>)
        ensures result@.dom().finite()
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

    impl<K: MtKey, V: MtVal> PartialEq for OrderedTableMtEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
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
