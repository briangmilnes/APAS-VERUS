//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 multi-threaded ephemeral table implementation using ArraySeqMtEph as backing store.

pub mod TableMtEph {

    use std::cmp::Ordering;
    use std::sync::Arc;
    use std::thread;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;

    #[derive(Clone, PartialEq)]
    pub struct TableMtEph<K: MtKey, V: MtVal> {
        entries: ArraySeqMtEphS<Pair<K, V>>,
    }

    impl<K: MtKey, V: MtVal> std::fmt::Debug for TableMtEph<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TableMtEph")
                .field("size", &self.entries.length())
                .finish()
        }
    }

    pub type TableS<K, V> = TableMtEph<K, V>;

    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableMtEphTrait<K: MtKey, V: MtVal> {
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
        /// claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn domain(&self)               -> ArraySetStEph<K>;
        /// APAS: Work Θ(|s| * W(f)), Span Θ(lg |s| + S(f))
        /// claude-4-sonet: Work Θ(|keys| × W(f)), Span Θ(log |keys| + S(f)), Parallelism Θ(|keys|/(log |keys| + S(f)))
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> Self;
        /// APAS: Work Θ(Σ W(f(v))), Span Θ(lg |a| + max S(f(v)))
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(log n + S(f)), Parallelism Θ(n/(log n + S(f)))
        fn map<F: Fn(&V) -> V + Send + Sync + 'static>(&mut self, f: F);
        /// APAS: Work Θ(Σ W(p(k,v))), Span Θ(lg |a| + max S(p(k,v)))
        /// claude-4-sonet: Work Θ(n × W(f)), Span Θ(log n + S(f)), Parallelism Θ(n/(log n + S(f)))
        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&mut self, f: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&mut self, other: &Self);
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, key: &K)        -> Option<V>;
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn delete(&mut self, key: &K);
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, key: K, value: V, combine: F);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn restrict(&mut self, keys: &ArraySetStEph<K>);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn subtract(&mut self, keys: &ArraySetStEph<K>);

        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn collect(&self)              -> ArraySeqMtEphS<Pair<K, V>>;
    }

    impl<K: MtKey, V: MtVal> TableMtEphTrait<K, V> for TableMtEph<K, V> {
        /// Work: O(1), Span: O(1)
        fn size(&self) -> N { self.entries.length() }

        /// Work: O(1), Span: O(1)
        fn empty() -> Self {
            TableMtEph {
                entries: ArraySeqMtEphS::empty(),
            }
        }

        /// Work: O(1), Span: O(1)
        fn singleton(key: K, value: V) -> Self {
            TableMtEph {
                entries: ArraySeqMtEphS::singleton(Pair(key, value)),
            }
        }

        /// Work: O(n), Span: O(log n) - parallel domain extraction
        fn domain(&self) -> ArraySetStEph<K> {
            let mut keys = ArraySetStEph::empty();
            let len = self.entries.length();

            if len <= 1 {
                // Base case: extract key directly
                if len == 1 {
                    keys.insert(self.entries.nth(0).0.clone());
                }
                return keys;
            }

            // Parallel extraction using spawn/join
            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);

            let handle = thread::spawn(move || {
                ArraySeqMtEphS::tabulate(&|i| left_entries.nth(i).0.clone(), left_entries.length()).clone()
            });

            let right_keys = ArraySeqMtEphS::tabulate(&|i| right_entries.nth(i).0.clone(), right_entries.length()).clone();

            let left_keys = handle.join().unwrap();

            // Insert all keys sequentially (ArraySetStEph is single-threaded)
            for i in 0..left_keys.length() {
                keys.insert(left_keys.nth(i).clone());
            }
            for i in 0..right_keys.length() {
                keys.insert(right_keys.nth(i).clone());
            }
            keys
        }

        /// Work: O(n), Span: O(log n) - parallel tabulation
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> Self {
            let key_seq = keys.to_seq();
            let f = Arc::new(f);
            let len = key_seq.length();

            if len == 0 {
                return TableMtEph::empty();
            }

            if len == 1 {
                let key = key_seq.nth(0);
                let value = f(key);
                return TableMtEph::singleton(key.clone(), value);
            }

            // Parallel tabulation using spawn/join
            let mid = len / 2;
            let left_seq = key_seq.subseq_copy(0, mid);
            let right_seq = key_seq.subseq_copy(mid, len - mid);
            let f_clone = f.clone();

            let handle = thread::spawn(move || {
                ArraySeqMtEphS::tabulate(
                    &|i| {
                        let key = left_seq.nth(i);
                        let value = f_clone(key);
                        Pair(key.clone(), value)
                    },
                    left_seq.length(),
                )
            });

            let right_entries = ArraySeqMtEphS::tabulate(
                &|i| {
                    let key = right_seq.nth(i);
                    let value = f(key);
                    Pair(key.clone(), value)
                },
                right_seq.length(),
            );

            let left_entries = handle.join().unwrap();

            // Merge and sort entries - combine both sequences
            let total_len = left_entries.length() + right_entries.length();
            let mut entries = Vec::with_capacity(total_len);
            for i in 0..left_entries.length() {
                entries.push(left_entries.nth(i).clone());
            }
            for i in 0..right_entries.length() {
                entries.push(right_entries.nth(i).clone());
            }
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            TableMtEph {
                entries: ArraySeqMtEphS::from_vec(entries),
            }
        }

        /// Work: O(n), Span: O(log n) - parallel map
        fn map<F: Fn(&V) -> V + Send + Sync + 'static>(&mut self, f: F) {
            let f = Arc::new(f);
            let len = self.entries.length();

            if len <= 1 {
                if len == 1 {
                    let pair = self.entries.nth(0).clone();
                    let new_value = f(&pair.1);
                    self.entries = ArraySeqMtEphS::singleton(Pair(pair.0, new_value));
                }
                return;
            }

            // Parallel map using spawn/join
            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);
            let f_clone = f.clone();

            let handle = thread::spawn(move || {
                ArraySeqMtEphS::tabulate(
                    &|i| {
                        let pair = left_entries.nth(i).clone();
                        let new_value = f_clone(&pair.1);
                        Pair(pair.0, new_value)
                    },
                    left_entries.length(),
                )
            });

            let right_mapped = ArraySeqMtEphS::tabulate(
                &|i| {
                    let pair = right_entries.nth(i).clone();
                    let new_value = f(&pair.1);
                    Pair(pair.0, new_value)
                },
                right_entries.length(),
            );

            let left_mapped = handle.join().unwrap();

            // Merge results - combine both sequences
            let mut mapped_entries = Vec::with_capacity(len);
            for i in 0..left_mapped.length() {
                mapped_entries.push(left_mapped.nth(i).clone());
            }
            for i in 0..right_mapped.length() {
                mapped_entries.push(right_mapped.nth(i).clone());
            }

            self.entries = ArraySeqMtEphS::from_vec(mapped_entries);
        }

        /// Work: O(n), Span: O(log n) - parallel filter
        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&mut self, f: F) {
            let f = Arc::new(f);
            let len = self.entries.length();

            if len == 0 {
                return;
            }

            if len == 1 {
                let pair = self.entries.nth(0).clone();
                if !f(&pair.0, &pair.1) {
                    self.entries = ArraySeqMtEphS::empty();
                }
                return;
            }

            // Parallel filter using spawn/join
            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);
            let f_clone = f.clone();

            let handle = thread::spawn(move || {
                let mut left_filtered = Vec::new();
                for i in 0..left_entries.length() {
                    let pair = left_entries.nth(i).clone();
                    if f_clone(&pair.0, &pair.1) {
                        left_filtered.push(pair);
                    }
                }
                left_filtered
            });

            let mut right_filtered = Vec::new();
            for i in 0..right_entries.length() {
                let pair = right_entries.nth(i).clone();
                if f(&pair.0, &pair.1) {
                    right_filtered.push(pair);
                }
            }

            let left_filtered = handle.join().unwrap();

            // Merge results - combine both filtered sequences
            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        /// Work: O(n + m), Span: O(log(n + m)) - parallel intersection
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F) {
            let combine = Arc::new(combine);
            let mut intersection_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            // Sequential merge (sorted sequences)
            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i).clone();
                let pair2 = other.entries.nth(j).clone();

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

            self.entries = ArraySeqMtEphS::from_vec(intersection_entries);
        }

        /// Work: O(n + m), Span: O(log(n + m)) - parallel union
        fn union<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F) {
            let combine = Arc::new(combine);
            let mut union_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            // Sequential merge (sorted sequences)
            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i).clone();
                let pair2 = other.entries.nth(j).clone();

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

            self.entries = ArraySeqMtEphS::from_vec(union_entries);
        }

        /// Work: O(n + m), Span: O(log(n + m)) - parallel difference
        fn difference(&mut self, other: &Self) {
            let mut difference_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            // Sequential merge (sorted sequences)
            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i).clone();
                let pair2 = other.entries.nth(j).clone();

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

            self.entries = ArraySeqMtEphS::from_vec(difference_entries);
        }

        /// Work: O(log n), Span: O(log n) - binary search
        fn find(&self, key: &K) -> Option<V> {
            // Binary search since entries are sorted by key
            let mut left = 0;
            let mut right = self.entries.length();

            while left < right {
                let mid = left + (right - left) / 2;
                let pair = self.entries.nth(mid).clone();

                match key.cmp(&pair.0) {
                    | Ordering::Less => right = mid,
                    | Ordering::Greater => left = mid + 1,
                    | Ordering::Equal => return Some(pair.1.clone()),
                }
            }

            None
        }

        /// Work: O(n), Span: O(log n) - parallel filter
        fn delete(&mut self, key: &K) {
            let len = self.entries.length();

            if len == 0 {
                return;
            }

            if len == 1 {
                let pair = self.entries.nth(0).clone();
                if pair.0 == *key {
                    self.entries = ArraySeqMtEphS::empty();
                }
                return;
            }

            // Parallel delete using spawn/join
            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);
            let key_clone = key.clone();

            let handle = thread::spawn(move || {
                let mut left_filtered = Vec::new();
                for i in 0..left_entries.length() {
                    let pair = left_entries.nth(i).clone();
                    if pair.0 != key_clone {
                        left_filtered.push(pair);
                    }
                }
                left_filtered
            });

            let key_clone2 = key.clone();
            let mut right_filtered = Vec::new();
            for i in 0..right_entries.length() {
                let pair = right_entries.nth(i).clone();
                if pair.0 != key_clone2 {
                    right_filtered.push(pair);
                }
            }

            let left_filtered = handle.join().unwrap();

            // Merge results - combine both filtered sequences
            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        /// Work: O(n), Span: O(log n) - parallel insert with combine
        fn insert<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, key: K, value: V, combine: F) {
            // Check if key already exists
            if let Some(existing_value) = self.find(&key) {
                // Key exists, combine values and replace
                let combined_value = combine(&existing_value, &value);
                let len = self.entries.length();

                if len == 1 {
                    self.entries = ArraySeqMtEphS::singleton(Pair(key, combined_value));
                    return;
                }

                // Parallel update using spawn/join
                let mid = len / 2;
                let left_entries = self.entries.subseq_copy(0, mid);
                let right_entries = self.entries.subseq_copy(mid, len - mid);
                let key_clone = key.clone();
                let combined_clone = combined_value.clone();

                let handle = thread::spawn(move || {
                    ArraySeqMtEphS::tabulate(
                        &|i| {
                            let pair = left_entries.nth(i).clone();
                            if pair.0 == key_clone {
                                Pair(key_clone.clone(), combined_clone.clone())
                            } else {
                                pair
                            }
                        },
                        left_entries.length(),
                    )
                });

                let right_updated = ArraySeqMtEphS::tabulate(
                    &|i| {
                        let pair = right_entries.nth(i).clone();
                        if pair.0 == key {
                            Pair(key.clone(), combined_value.clone())
                        } else {
                            pair
                        }
                    },
                    right_entries.length(),
                );

                let left_updated = handle.join().unwrap();

                // Merge results - combine both sequences
                let mut updated_entries = Vec::with_capacity(len);
                for i in 0..left_updated.length() {
                    updated_entries.push(left_updated.nth(i).clone());
                }
                for i in 0..right_updated.length() {
                    updated_entries.push(right_updated.nth(i).clone());
                }

                self.entries = ArraySeqMtEphS::from_vec(updated_entries);
            } else {
                // Key doesn't exist, add new entry
                let new_pair = Pair(key, value);
                let new_entries = ArraySeqMtEphS::tabulate(
                    &|i| {
                        if i < self.entries.length() {
                            self.entries.nth(i).clone()
                        } else {
                            new_pair.clone()
                        }
                    },
                    self.entries.length() + 1,
                );
                let mut entries_vec = Vec::with_capacity(new_entries.length());
                for i in 0..new_entries.length() {
                    entries_vec.push(new_entries.nth(i).clone());
                }
                entries_vec.sort_by(|a, b| a.0.cmp(&b.0));
                self.entries = ArraySeqMtEphS::from_vec(entries_vec);
            }
        }

        /// Work: O(n + m), Span: O(log n) - parallel restrict
        fn restrict(&mut self, keys: &ArraySetStEph<K>) {
            let len = self.entries.length();

            if len == 0 {
                return;
            }

            if len == 1 {
                let pair = self.entries.nth(0).clone();
                if !keys.find(&pair.0) {
                    self.entries = ArraySeqMtEphS::empty();
                }
                return;
            }

            // Parallel restrict using spawn/join
            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);
            let keys_clone = keys.clone();

            let handle = thread::spawn(move || {
                let mut left_filtered = Vec::new();
                for i in 0..left_entries.length() {
                    let pair = left_entries.nth(i).clone();
                    if keys_clone.find(&pair.0) {
                        left_filtered.push(pair);
                    }
                }
                left_filtered
            });

            let keys_clone2 = keys.clone();
            let mut right_filtered = Vec::new();
            for i in 0..right_entries.length() {
                let pair = right_entries.nth(i).clone();
                if keys_clone2.find(&pair.0) {
                    right_filtered.push(pair);
                }
            }

            let left_filtered = handle.join().unwrap();

            // Merge results - combine both filtered sequences
            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        /// Work: O(n + m), Span: O(log n) - parallel subtract
        fn subtract(&mut self, keys: &ArraySetStEph<K>) {
            let len = self.entries.length();

            if len == 0 {
                return;
            }

            if len == 1 {
                let pair = self.entries.nth(0).clone();
                if keys.find(&pair.0) {
                    self.entries = ArraySeqMtEphS::empty();
                }
                return;
            }

            // Parallel subtract using spawn/join
            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);
            let keys_clone = keys.clone();

            let handle = thread::spawn(move || {
                let mut left_filtered = Vec::new();
                for i in 0..left_entries.length() {
                    let pair = left_entries.nth(i).clone();
                    if !keys_clone.find(&pair.0) {
                        left_filtered.push(pair);
                    }
                }
                left_filtered
            });

            let keys_clone2 = keys.clone();
            let mut right_filtered = Vec::new();
            for i in 0..right_entries.length() {
                let pair = right_entries.nth(i).clone();
                if !keys_clone2.find(&pair.0) {
                    right_filtered.push(pair);
                }
            }

            let left_filtered = handle.join().unwrap();

            // Merge results - combine both filtered sequences
            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        fn collect(&self) -> ArraySeqMtEphS<Pair<K, V>> { self.entries.clone() }
    }

    /// Create tables from sorted entries
    pub fn from_sorted_entries<K: MtKey, V: MtVal>(entries: Vec<Pair<K, V>>) -> TableMtEph<K, V> {
        TableMtEph {
            entries: ArraySeqMtEphS::from_vec(entries),
        }
    }

    /// Macro for creating multi-threaded ephemeral table literals
    #[macro_export]
    macro_rules! TableMtEphLit {
        () => {
            $crate::Chap42::TableMtEph::TableMtEph::TableMtEph::empty()
        };
        ($($key:expr => $value:expr),+ $(,)?) => {{
            let mut entries = vec![$($crate::Types::Types::Pair($key, $value)),+];
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            $crate::Chap42::TableMtEph::TableMtEph::from_sorted_entries(entries)
        }};
    }
}
