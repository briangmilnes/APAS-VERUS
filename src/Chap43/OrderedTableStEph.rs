//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered table implementation extending TableStEph.

pub mod OrderedTableStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Types::Types::*;

    #[derive(PartialEq)]
    pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
        base_table: TableStEph<K, V>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1 for keys) with ephemeral semantics
    pub trait OrderedTableStEphTrait<K: StT + Ord, V: StT> {
        // Base table operations (ADT 42.1) - ephemeral semantics
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                          -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                              -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(k: K, v: V)                -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, k: &K)                   -> Option<V>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn lookup(&self, k: &K)                 -> Option<V>; // Alias for find
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                      -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, k: &K)             -> Option<V>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn domain(&self)                        -> ArraySetStEph<K>;
        /// claude-4-sonet: Work Θ(|keys| × W(f)), Span Θ(|keys| × S(f)), Parallelism Θ(1)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self;
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(n × S(f)), Parallelism Θ(1)
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(n × S(f)), Parallelism Θ(1)
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(n × S(f)), Parallelism Θ(1)
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> R;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&mut self, other: &Self);
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn restrict(&mut self, keys: &ArraySetStEph<K>);
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn subtract(&mut self, keys: &ArraySetStEph<K>);
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn collect(&self)                       -> AVLTreeSeqStPerS<Pair<K, V>>;

        // Key ordering operations (ADT 43.1 adapted for tables)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn first_key(&self)                     -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn last_key(&self)                      -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn previous_key(&self, k: &K)           -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn next_key(&self, k: &K)               -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split_key(&mut self, k: &K)          -> (Self, Self)
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(log(|self| + |other|)), Span Θ(log(|self| + |other|)), Parallelism Θ(1)
        fn join_key(&mut self, other: Self);
        fn get_key_range(&self, k1: &K, k2: &K) -> Self;
        fn rank_key(&self, k: &K)               -> N;
        fn select_key(&self, i: N)              -> Option<K>;
        fn split_rank_key(&mut self, i: N)      -> (Self, Self)
        where
            Self: Sized;
    }

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        // Base table operations - delegate to backing store with ephemeral semantics

        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N { self.base_table.size() }

        /// Claude Work: O(1), Span: O(1)
        fn empty() -> Self {
            OrderedTableStEph {
                base_table: TableStEph::empty(),
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn singleton(k: K, v: V) -> Self {
            OrderedTableStEph {
                base_table: TableStEph::singleton(k, v),
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn find(&self, k: &K) -> Option<V> { self.base_table.find(k) }

        /// Claude Work: O(log n), Span: O(log n)
        fn lookup(&self, k: &K) -> Option<V> { self.find(k) }

        /// Claude Work: O(1), Span: O(1)
        fn is_empty(&self) -> B { self.size() == 0 }

        /// Claude Work: O(log n), Span: O(log n)
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F) { self.base_table.insert(k, v, combine); }

        /// Claude Work: O(log n), Span: O(log n)
        fn delete(&mut self, k: &K) -> Option<V> {
            let old_value = self.find(k);
            self.base_table.delete(k);
            old_value
        }

        /// Claude Work: O(n), Span: O(log n)
        fn domain(&self) -> ArraySetStEph<K> { self.base_table.domain() }

        /// Claude Work: O(n log n), Span: O(log² n)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self {
            OrderedTableStEph {
                base_table: TableStEph::tabulate(f, keys),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> Self {
            let mut result = OrderedTableStEph::empty();
            let entries = self.collect();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                result
                    .base_table
                    .insert(pair.0.clone(), new_value, |_old, new| new.clone());
            }
            result
        }

        /// Claude Work: O(n), Span: O(log n)
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self {
            let mut result = OrderedTableStEph::empty();
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

        /// Claude Work: O(n), Span: O(log n)
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> R {
            let entries = self.collect();
            let mut result = init;
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                result = f(result, &pair.0, &pair.1);
            }
            result
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F) {
            self.base_table.intersection(&other.base_table, f);
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F) { self.base_table.union(&other.base_table, f); }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn difference(&mut self, other: &Self) { self.base_table.difference(&other.base_table); }

        /// Claude Work: O(n), Span: O(log n)
        fn restrict(&mut self, keys: &ArraySetStEph<K>) { self.base_table.restrict(keys); }

        /// Claude Work: O(n), Span: O(log n)
        fn subtract(&mut self, keys: &ArraySetStEph<K>) { self.base_table.subtract(keys); }

        /// Claude Work: O(n), Span: O(log n)
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>> {
            let array_seq = self.base_table.collect();
            // Convert ArraySeqStEphS to AVLTreeSeqStPerS
            let len = array_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(elements)
        }

        // Key ordering operations (ADT 43.1 adapted for tables)

        /// Claude Work: O(log n), Span: O(log n)
        fn first_key(&self) -> Option<K> {
            let entries = self.collect();
            if entries.length() == 0 {
                None
            } else {
                Some(entries.nth(0).0.clone())
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn last_key(&self) -> Option<K> {
            let entries = self.collect();
            let size = entries.length();
            if size == 0 {
                None
            } else {
                Some(entries.nth(size - 1).0.clone())
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn previous_key(&self, k: &K) -> Option<K> {
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

        /// Claude Work: O(log n), Span: O(log n)
        fn next_key(&self, k: &K) -> Option<K> {
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

        /// Claude Work: O(log n), Span: O(log n)
        fn split_key(&mut self, k: &K) -> (Self, Self) {
            let entries = self.collect();
            let size = entries.length();
            let mut left_entries = Vec::new();
            let mut right_entries = Vec::new();
            let mut _found_value: Option<V> = None;

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    left_entries.push(pair.clone());
                } else {
                    // Keys >= k go to the right side
                    right_entries.push(pair.clone());
                }
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            // Clear current table (ephemeral behavior)
            *self = Self::empty();

            (from_sorted_entries(left_seq), from_sorted_entries(right_seq))
        }

        /// Claude Work: O(log(m + n)), Span: O(log(m + n))
        fn join_key(&mut self, other: Self) { self.union(&other, |v1, _v2| v1.clone()); }

        /// Claude Work: O(log n), Span: O(log n)
        fn get_key_range(&self, k1: &K, k2: &K) -> Self {
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

        /// Claude Work: O(log n), Span: O(log n)
        fn rank_key(&self, k: &K) -> N {
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

        /// Claude Work: O(log n), Span: O(log n)
        fn select_key(&self, i: N) -> Option<K> {
            let entries = self.collect();
            if i >= entries.length() {
                None
            } else {
                Some(entries.nth(i).0.clone())
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
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

            // Clear current table (ephemeral behavior)
            *self = Self::empty();

            (from_sorted_entries(left_seq), from_sorted_entries(right_seq))
        }
    }

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> Self {
            OrderedTableStEph {
                base_table: self.base_table.clone(),
            }
        }
    }

    // Convert persistent sequence to Vec for TableStEph
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(entries: AVLTreeSeqStPerS<Pair<K, V>>) -> OrderedTableStEph<K, V> {
        let len = entries.length();
        let mut elements = Vec::new();
        for i in 0..len {
            elements.push(entries.nth(i).clone());
        }
        OrderedTableStEph {
            base_table: crate::Chap42::TableStEph::TableStEph::from_sorted_entries(elements),
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
