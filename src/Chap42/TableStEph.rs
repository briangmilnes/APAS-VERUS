//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 single-threaded ephemeral table implementation using ArraySeq as backing store.

pub mod TableStEph {

    use std::cmp::Ordering;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct TableStEph<K: StT + Ord, V: StT> {
        entries: ArraySeqStEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStEph<K, V>;

    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableStEphTrait<K: StT + Ord, V: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                     -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(key: K, value: V) -> Self;
        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn domain(&self)               -> ArraySetStEph<K>;
        /// APAS: Work Θ(|s| * W(f)), Span Θ(lg |s| + S(f))
        /// claude-4-sonet: Work Θ(|keys| × W(f)), Span Θ(|keys| × S(f)), Parallelism Θ(1)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self;
        /// APAS: Work Θ(Σ W(f(v))), Span Θ(lg |a| + max S(f(v)))
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(n × S(f)), Parallelism Θ(1)
        fn map<F: Fn(&V) -> V>(&mut self, f: F);
        /// APAS: Work Θ(Σ W(p(k,v))), Span Θ(lg |a| + max S(p(k,v)))
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(n × S(f)), Parallelism Θ(1)
        fn filter<F: Fn(&K, &V) -> B>(&mut self, f: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn difference(&mut self, other: &Self);
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, key: &K)        -> Option<V>;
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn delete(&mut self, key: &K);
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn restrict(&mut self, keys: &ArraySetStEph<K>);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn subtract(&mut self, keys: &ArraySetStEph<K>);

        /// Returns a flat sequence of (K, V) pairs in key order.
        /// Per APAS Algorithm 42.3, true collect groups values by key (Seq<(K, Seq<V>)>); this is entries.
        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn entries(&self)              -> ArraySeqStEphS<Pair<K, V>>;
    }

    impl<K: StT + Ord, V: StT> TableStEphTrait<K, V> for TableStEph<K, V> {
        /// Work: O(1), Span: O(1)
        fn size(&self) -> N { self.entries.length() }

        /// Work: O(1), Span: O(1)
        fn empty() -> Self {
            TableStEph {
                entries: ArraySeqStEphS::empty(),
            }
        }

        /// Work: O(1), Span: O(1)
        fn singleton(key: K, value: V) -> Self {
            TableStEph {
                entries: ArraySeqStEphS::singleton(Pair(key, value)),
            }
        }

        /// Work: O(n), Span: O(n)
        fn domain(&self) -> ArraySetStEph<K> {
            let mut keys = ArraySetStEph::empty();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                keys.insert(pair.0.clone());
            }
            keys
        }

        /// Work: O(n), Span: O(n)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> Self {
            let key_seq = keys.to_seq();
            let mut entries = Vec::with_capacity(key_seq.length());
            for i in 0..key_seq.length() {
                let key = key_seq.nth(i);
                let value = f(key);
                entries.push(Pair(key.clone(), value));
            }
            // Sort entries by key
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            TableStEph {
                entries: ArraySeqStEphS::from_vec(entries),
            }
        }

        /// Work: O(n), Span: O(n)
        fn map<F: Fn(&V) -> V>(&mut self, f: F) {
            let mapped_entries = ArraySeqStEphS::tabulate(
                &|i| {
                    let pair = self.entries.nth(i);
                    let new_value = f(&pair.1);
                    Pair(pair.0.clone(), new_value)
                },
                self.entries.length(),
            );
            self.entries = mapped_entries;
        }

        /// Work: O(n), Span: O(n)
        fn filter<F: Fn(&K, &V) -> B>(&mut self, f: F) {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if f(&pair.0, &pair.1) {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        /// Work: O(n + m), Span: O(n + m)
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F) {
            let mut intersection_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    | Ordering::Less => i += 1,
                    | Ordering::Greater => j += 1,
                    | Ordering::Equal => {
                        let combined_value = combine(&pair1.1, &pair2.1);
                        intersection_entries.push(Pair(pair1.0.clone(), combined_value));
                        i += 1;
                        j += 1;
                    }
                }
            }

            self.entries = ArraySeqStEphS::from_vec(intersection_entries);
        }

        /// Work: O(n + m), Span: O(n + m)
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F) {
            let mut union_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    | Ordering::Less => {
                        union_entries.push(pair1.clone());
                        i += 1;
                    }
                    | Ordering::Greater => {
                        union_entries.push(pair2.clone());
                        j += 1;
                    }
                    | Ordering::Equal => {
                        let combined_value = combine(&pair1.1, &pair2.1);
                        union_entries.push(Pair(pair1.0.clone(), combined_value));
                        i += 1;
                        j += 1;
                    }
                }
            }

            // Add remaining entries from self
            while i < self.entries.length() {
                union_entries.push(self.entries.nth(i).clone());
                i += 1;
            }

            // Add remaining entries from other
            while j < other.entries.length() {
                union_entries.push(other.entries.nth(j).clone());
                j += 1;
            }

            self.entries = ArraySeqStEphS::from_vec(union_entries);
        }

        /// Work: O(n + m), Span: O(n + m)
        fn difference(&mut self, other: &Self) {
            let mut difference_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    | Ordering::Less => {
                        difference_entries.push(pair1.clone());
                        i += 1;
                    }
                    | Ordering::Greater => {
                        j += 1;
                    }
                    | Ordering::Equal => {
                        i += 1;
                        j += 1;
                    }
                }
            }

            // Add remaining entries from self
            while i < self.entries.length() {
                difference_entries.push(self.entries.nth(i).clone());
                i += 1;
            }

            self.entries = ArraySeqStEphS::from_vec(difference_entries);
        }

        /// Work: O(log n), Span: O(log n)
        fn find(&self, key: &K) -> Option<V> {
            // Binary search since entries are sorted by key
            let mut left = 0;
            let mut right = self.entries.length();

            while left < right {
                let mid = left + (right - left) / 2;
                let pair = self.entries.nth(mid);

                match key.cmp(&pair.0) {
                    | Ordering::Less => right = mid,
                    | Ordering::Greater => left = mid + 1,
                    | Ordering::Equal => return Some(pair.1.clone()),
                }
            }

            None
        }

        /// Work: O(n), Span: O(n)
        fn delete(&mut self, key: &K) {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if &pair.0 != key {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        /// Work: O(n), Span: O(n)
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F) {
            // Check if key already exists
            if let Some(existing_value) = self.find(&key) {
                // Key exists, combine values and replace
                let combined_value = combine(&existing_value, &value);
                let mut updated_entries = Vec::new();
                for i in 0..self.entries.length() {
                    let pair = self.entries.nth(i);
                    if pair.0 != key {
                        updated_entries.push(pair.clone());
                    }
                }
                updated_entries.push(Pair(key, combined_value));
                updated_entries.sort_by(|a, b| a.0.cmp(&b.0));
                self.entries = ArraySeqStEphS::from_vec(updated_entries);
            } else {
                // Key doesn't exist, add new entry
                let mut new_entries = Vec::with_capacity(self.entries.length() + 1);
                for i in 0..self.entries.length() {
                    new_entries.push(self.entries.nth(i).clone());
                }
                new_entries.push(Pair(key, value));
                new_entries.sort_by(|a, b| a.0.cmp(&b.0));
                self.entries = ArraySeqStEphS::from_vec(new_entries);
            }
        }

        /// Work: O(n + m), Span: O(n + m)
        fn restrict(&mut self, keys: &ArraySetStEph<K>) {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if keys.find(&pair.0) {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        /// Work: O(n + m), Span: O(n + m)
        fn subtract(&mut self, keys: &ArraySetStEph<K>) {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if !keys.find(&pair.0) {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        fn entries(&self) -> ArraySeqStEphS<Pair<K, V>> { self.entries.clone() }
    }

    /// Create tables from sorted entries
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(entries: Vec<Pair<K, V>>) -> TableStEph<K, V> {
        TableStEph {
            entries: ArraySeqStEphS::from_vec(entries),
        }
    }

    /// Macro for creating ephemeral table literals
    #[macro_export]
    macro_rules! TableStEphLit {
        () => {
            $crate::Chap42::TableStEph::TableStEph::TableStEph::empty()
        };
        ($($key:expr => $value:expr),+ $(,)?) => {{
            let mut entries = vec![$($crate::Types::Types::Pair($key, $value)),+];
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            $crate::Chap42::TableStEph::TableStEph::from_sorted_entries(entries)
        }};
    }
}
