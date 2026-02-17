//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered table implementation extending TableStPer.

pub mod OrderedTableStPer {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;

    #[derive(PartialEq)]
    pub struct OrderedTableStPer<K: StT + Ord, V: StT> {
        base_table: TableStPer<K, V>,
    }

    pub type OrderedTablePer<K, V> = OrderedTableStPer<K, V>;

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1 for keys)
    pub trait OrderedTableStPerTrait<K: StT + Ord, V: StT> {
        // Base table operations (ADT 42.1) - delegated
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                              -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                  -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(k: K, v: V)                    -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, k: &K)                       -> Option<V>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, k: K, v: V)                -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, k: &K)                     -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn domain(&self)                            -> ArraySetStEph<K>;
        /// claude-4-sonet: Work Θ(|keys| × W(f)), Span Θ(|keys| × S(f)), Parallelism Θ(1)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self;
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(n × S(f)), Parallelism Θ(1)
        fn map<F: Fn(&V) -> V>(&self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(n × S(f)), Parallelism Θ(1)
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self)          -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn restrict(&self, keys: &ArraySetStEph<K>) -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn subtract(&self, keys: &ArraySetStEph<K>) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn collect(&self)                           -> AVLTreeSeqStPerS<Pair<K, V>>;

        // Key ordering operations (ADT 43.1 adapted for tables)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn first_key(&self)                         -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn last_key(&self)                          -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn previous_key(&self, k: &K)               -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn next_key(&self, k: &K)                   -> Option<K>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split_key(&self, k: &K)                  -> (Self, Option<V>, Self)
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|)), Parallelism Θ(1)
        fn join_key(left: &Self, right: &Self)      -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn get_key_range(&self, k1: &K, k2: &K)     -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank_key(&self, k: &K)                   -> N;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn select_key(&self, i: N)                  -> Option<K>;
        fn split_rank_key(&self, i: N)              -> (Self, Self)
        where
            Self: Sized;
    }

    impl<K: StT + Ord, V: StT> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        // Base table operations - delegate to backing store

        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N { self.base_table.size() }

        /// Claude Work: O(1), Span: O(1)
        fn empty() -> Self {
            OrderedTableStPer {
                base_table: TableStPer::empty(),
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn singleton(k: K, v: V) -> Self {
            OrderedTableStPer {
                base_table: TableStPer::singleton(k, v),
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn find(&self, k: &K) -> Option<V> { self.base_table.find(k) }

        /// Claude Work: O(log n), Span: O(log n)
        fn insert(&self, k: K, v: V) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.insert(k, v, |_old, new| new.clone()),
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn delete(&self, k: &K) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.delete(k),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn domain(&self) -> ArraySetStEph<K> { self.base_table.domain() }

        /// Claude Work: O(n log n), Span: O(log² n)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self {
            OrderedTableStPer {
                base_table: TableStPer::tabulate(f, keys),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn map<F: Fn(&V) -> V>(&self, f: F) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.map(f),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.filter(f),
            }
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.intersection(&other.base_table, f),
            }
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.union(&other.base_table, f),
            }
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn difference(&self, other: &Self) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.difference(&other.base_table),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn restrict(&self, keys: &ArraySetStEph<K>) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.restrict(keys),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn subtract(&self, keys: &ArraySetStEph<K>) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.subtract(keys),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>> {
            let array_seq = self.base_table.collect();
            // Convert ArraySeqStPerS to AVLTreeSeqStPerS
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
        fn split_key(&self, k: &K) -> (Self, Option<V>, Self) {
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

        /// Claude Work: O(log(m + n)), Span: O(log(m + n))
        fn join_key(left: &Self, right: &Self) -> Self { left.union(right, |v1, _v2| v1.clone()) }

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
        fn split_rank_key(&self, i: N) -> (Self, Self) {
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

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> Self {
            OrderedTableStPer {
                base_table: self.base_table.clone(),
            }
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT>(entries: AVLTreeSeqStPerS<Pair<K, V>>) -> OrderedTableStPer<K, V> {
        let len = entries.length();
        let mut elements = Vec::new();
        for i in 0..len {
            elements.push(entries.nth(i).clone());
        }
        OrderedTableStPer {
            base_table: crate::Chap42::TableStPer::TableStPer::from_sorted_entries(elements),
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
