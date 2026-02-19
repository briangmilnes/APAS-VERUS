//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 multi-threaded ephemeral table implementation using ArraySeqMtEph as backing store.

pub mod TableMtEph {

    use std::cmp::Ordering;
    use std::sync::Arc;
    use std::thread;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
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

    pub struct TableMtEph<K: MtKey, V: MtVal> {
        entries: ArraySeqMtEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableMtEph<K, V>;

    // 5. view impls

    impl<K: MtKey, V: MtVal> View for TableMtEph<K, V> {
        type V = Map<K::V, V::V>;

        #[verifier::external_body]
        open spec fn view(&self) -> Map<K::V, V::V> {
            Map::empty()
        }
    }

    // 8. traits

    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableMtEphTrait<K: MtKey, V: MtVal>: Sized + View<V = Map<K::V, V::V>> {
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len();

        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();

        fn singleton(key: K, value: V) -> (result: Self)
            ensures result@.dom().finite(), result@.dom().len() == 1;

        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures result@.finite();

        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();

        fn map<F: Fn(&V) -> V + Send + Sync + 'static>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom();

        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&mut self, f: F)
            ensures self@.dom().subset_of(old(self)@.dom());

        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F)
            ensures self@.dom().finite();

        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F)
            ensures self@.dom().finite();

        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite();

        fn find(&self, key: &K) -> (result: Option<V>)
            ensures self@.dom().finite();

        fn delete(&mut self, key: &K)
            ensures self@.dom().finite();

        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, key: K, value: V, combine: F)
            ensures self@.dom().finite();

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();

        fn entries(&self) -> (result: ArraySeqMtEphS<Pair<K, V>>);
    }

    // 9. impls

    impl<K: MtKey, V: MtVal> TableMtEphTrait<K, V> for TableMtEph<K, V> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len()
        {
            self.entries.length()
        }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            TableMtEph {
                entries: ArraySeqMtEphS::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(key: K, value: V) -> (result: Self)
            ensures result@.dom().finite(), result@.dom().len() == 1
        {
            TableMtEph {
                entries: ArraySeqMtEphS::singleton(Pair(key, value)),
            }
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures result@.finite()
        {
            let mut keys = ArraySetStEph::empty();
            let len = self.entries.length();

            if len <= 1 {
                if len == 1 {
                    keys.insert(self.entries.nth(0).0.clone());
                }
                return keys;
            }

            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);

            let handle = thread::spawn(move || {
                ArraySeqMtEphS::tabulate(&|i| left_entries.nth(i).0.clone(), left_entries.length()).clone()
            });

            let right_keys = ArraySeqMtEphS::tabulate(&|i| right_entries.nth(i).0.clone(), right_entries.length()).clone();

            let left_keys = handle.join().unwrap();

            for i in 0..left_keys.length() {
                keys.insert(left_keys.nth(i).clone());
            }
            for i in 0..right_keys.length() {
                keys.insert(right_keys.nth(i).clone());
            }
            keys
        }

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
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

        #[verifier::external_body]
        fn map<F: Fn(&V) -> V + Send + Sync + 'static>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom()
        {
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

            let mut mapped_entries = Vec::with_capacity(len);
            for i in 0..left_mapped.length() {
                mapped_entries.push(left_mapped.nth(i).clone());
            }
            for i in 0..right_mapped.length() {
                mapped_entries.push(right_mapped.nth(i).clone());
            }

            self.entries = ArraySeqMtEphS::from_vec(mapped_entries);
        }

        #[verifier::external_body]
        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&mut self, f: F)
            ensures self@.dom().subset_of(old(self)@.dom())
        {
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

            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F)
            ensures self@.dom().finite()
        {
            let combine = Arc::new(combine);
            let mut intersection_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

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

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F)
            ensures self@.dom().finite()
        {
            let combine = Arc::new(combine);
            let mut union_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

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

            while i < self.entries.length() {
                union_entries.push(self.entries.nth(i).clone());
                i += 1;
            }

            while j < other.entries.length() {
                union_entries.push(other.entries.nth(j).clone());
                j += 1;
            }

            self.entries = ArraySeqMtEphS::from_vec(union_entries);
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            let mut difference_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

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

            while i < self.entries.length() {
                difference_entries.push(self.entries.nth(i).clone());
                i += 1;
            }

            self.entries = ArraySeqMtEphS::from_vec(difference_entries);
        }

        #[verifier::external_body]
        fn find(&self, key: &K) -> (result: Option<V>)
            ensures self@.dom().finite()
        {
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

        #[verifier::external_body]
        fn delete(&mut self, key: &K)
            ensures self@.dom().finite()
        {
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

            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        #[verifier::external_body]
        fn insert<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, key: K, value: V, combine: F)
            ensures self@.dom().finite()
        {
            if let Some(existing_value) = self.find(&key) {
                let combined_value = combine(&existing_value, &value);
                let len = self.entries.length();

                if len == 1 {
                    self.entries = ArraySeqMtEphS::singleton(Pair(key, combined_value));
                    return;
                }

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

                let mut updated_entries = Vec::with_capacity(len);
                for i in 0..left_updated.length() {
                    updated_entries.push(left_updated.nth(i).clone());
                }
                for i in 0..right_updated.length() {
                    updated_entries.push(right_updated.nth(i).clone());
                }

                self.entries = ArraySeqMtEphS::from_vec(updated_entries);
            } else {
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

        #[verifier::external_body]
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
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

            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        #[verifier::external_body]
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
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

            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        #[verifier::external_body]
        fn entries(&self) -> (result: ArraySeqMtEphS<Pair<K, V>>) {
            self.entries.clone()
        }
    }

    // 11. derive impls in verus!

    impl<K: MtKey, V: MtVal> Clone for TableMtEph<K, V> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            TableMtEph {
                entries: self.entries.clone(),
            }
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_entries<K: MtKey, V: MtVal>(entries: Vec<Pair<K, V>>) -> (result: TableMtEph<K, V>)
        ensures result@.dom().finite()
    {
        TableMtEph {
            entries: ArraySeqMtEphS::from_vec(entries),
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<K: MtKey, V: MtVal> PartialEq for TableMtEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.entries == other.entries
        }
    }

    impl<K: MtKey, V: MtVal> std::fmt::Debug for TableMtEph<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TableMtEph")
                .field("size", &self.entries.length())
                .finish()
        }
    }

    // 12. macros

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
