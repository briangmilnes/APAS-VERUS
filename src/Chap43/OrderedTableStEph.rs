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

    pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
        base_table: TableStEph<K, V>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for OrderedTableStEph<K, V> {
        type V = Map<K::V, V::V>;

        #[verifier::external_body]
        open spec fn view(&self) -> Self::V;
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1 for keys) with ephemeral semantics
    pub trait OrderedTableStEphTrait<K: StT + Ord, V: StT> {
        fn size(&self) -> (result: N);
        fn empty() -> (result: Self);
        fn singleton(k: K, v: V) -> (result: Self);
        fn find(&self, k: &K) -> (result: Option<V>);
        fn lookup(&self, k: &K) -> (result: Option<V>);
        fn is_empty(&self) -> (result: B);
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F);
        fn delete(&mut self, k: &K) -> (result: Option<V>);
        fn domain(&self) -> (result: ArraySetStEph<K>);
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self);
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (result: Self);
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self);
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (result: R);
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F);
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F);
        fn difference(&mut self, other: &Self);
        fn restrict(&mut self, keys: &ArraySetStEph<K>);
        fn subtract(&mut self, keys: &ArraySetStEph<K>);
        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>);

        fn first_key(&self) -> (result: Option<K>);
        fn last_key(&self) -> (result: Option<K>);
        fn previous_key(&self, k: &K) -> (result: Option<K>);
        fn next_key(&self, k: &K) -> (result: Option<K>);
        fn split_key(&mut self, k: &K) -> (Self, Option<V>, Self)
        where
            Self: Sized;
        fn join_key(&mut self, other: Self);
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self);
        fn rank_key(&self, k: &K) -> (result: N);
        fn select_key(&self, i: N) -> (result: Option<K>);
        fn split_rank_key(&mut self, i: N) -> (Self, Self)
        where
            Self: Sized;
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        #[verifier::external_body]
        fn size(&self) -> (result: N) {
            self.base_table.size()
        }

        #[verifier::external_body]
        fn empty() -> (result: Self) {
            OrderedTableStEph {
                base_table: TableStEph::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V) -> (result: Self) {
            OrderedTableStEph {
                base_table: TableStEph::singleton(k, v),
            }
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (result: Option<V>) {
            self.base_table.find(k)
        }

        #[verifier::external_body]
        fn lookup(&self, k: &K) -> (result: Option<V>) {
            self.find(k)
        }

        #[verifier::external_body]
        fn is_empty(&self) -> (result: B) {
            self.size() == 0
        }

        #[verifier::external_body]
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F) {
            self.base_table.insert(k, v, combine);
        }

        #[verifier::external_body]
        fn delete(&mut self, k: &K) -> (result: Option<V>) {
            let old_value = self.find(k);
            self.base_table.delete(k);
            old_value
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>) {
            self.base_table.domain()
        }

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self) {
            OrderedTableStEph {
                base_table: TableStEph::tabulate(f, keys),
            }
        }

        #[verifier::external_body]
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (result: Self) {
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
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self) {
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
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (result: R) {
            let entries = self.collect();
            let mut acc = init;
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                acc = f(acc, &pair.0, &pair.1);
            }
            acc
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F) {
            self.base_table.intersection(&other.base_table, f);
        }

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F) {
            self.base_table.union(&other.base_table, f);
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self) {
            self.base_table.difference(&other.base_table);
        }

        #[verifier::external_body]
        fn restrict(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.restrict(keys);
        }

        #[verifier::external_body]
        fn subtract(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.subtract(keys);
        }

        #[verifier::external_body]
        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>) {
            let array_seq = self.base_table.entries();
            let len = array_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(elements)
        }

        #[verifier::external_body]
        fn first_key(&self) -> (result: Option<K>) {
            let entries = self.collect();
            if entries.length() == 0 {
                None
            } else {
                Some(entries.nth(0).0.clone())
            }
        }

        #[verifier::external_body]
        fn last_key(&self) -> (result: Option<K>) {
            let entries = self.collect();
            let size = entries.length();
            if size == 0 {
                None
            } else {
                Some(entries.nth(size - 1).0.clone())
            }
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (result: Option<K>) {
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
        fn next_key(&self, k: &K) -> (result: Option<K>) {
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
        fn split_key(&mut self, k: &K) -> (Self, Option<V>, Self) {
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

        #[verifier::external_body]
        fn join_key(&mut self, other: Self) {
            self.union(&other, |v1, _v2| v1.clone());
        }

        #[verifier::external_body]
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self) {
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
        fn rank_key(&self, k: &K) -> (result: N) {
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
        fn select_key(&self, i: N) -> (result: Option<K>) {
            let entries = self.collect();
            if i >= entries.length() {
                None
            } else {
                Some(entries.nth(i).0.clone())
            }
        }

        #[verifier::external_body]
        fn split_rank_key(&mut self, i: N) -> (Self, Self) {
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
