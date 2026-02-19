//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 single-threaded ephemeral table implementation using ArraySeq as backing store.

pub mod TableStEph {

    use std::cmp::Ordering;
    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;

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

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableStEph<K: StT + Ord, V: StT> {
        entries: ArraySeqStEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStEph<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for TableStEph<K, V> {
        type V = Map<K::V, V::V>;
        #[verifier::external_body]
        open spec fn view(&self) -> Map<K::V, V::V> { Map::empty() }
    }

    // 8. traits

    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableStEphTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: N)
            ensures result == self@.len();
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();
        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(key: K, value: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(key@, value@);
        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures result@.finite();
        /// APAS: Work Θ(|s| * W(f)), Span Θ(lg |s| + S(f))
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
        /// APAS: Work Θ(Σ W(f(v))), Span Θ(lg |a| + max S(f(v)))
        fn map<F: Fn(&V) -> V>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom();
        /// APAS: Work Θ(Σ W(p(k,v))), Span Θ(lg |a| + max S(p(k,v)))
        fn filter<F: Fn(&K, &V) -> B>(&mut self, f: F)
            ensures self@.dom().subset_of(old(self)@.dom());
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures self@.dom().subset_of(old(self)@.dom().intersect(other@.dom()));
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures old(self)@.dom().union(other@.dom()).subset_of(self@.dom());
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn difference(&mut self, other: &Self)
            ensures self@.dom().subset_of(old(self)@.dom().difference(other@.dom()));
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn find(&self, key: &K) -> (result: Option<V>)
            ensures
                match result {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                };
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn delete(&mut self, key: &K)
            ensures !self@.contains_key(key@);
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            ensures self@.contains_key(key@);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().subset_of(old(self)@.dom());
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().subset_of(old(self)@.dom());

        /// Returns a flat sequence of (K, V) pairs in key order.
        fn entries(&self) -> (result: ArraySeqStEphS<Pair<K, V>>);
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> TableStEphTrait<K, V> for TableStEph<K, V> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.len()
        {
            self.entries.length()
        }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            TableStEph {
                entries: ArraySeqStEphS::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(key: K, value: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(key@, value@)
        {
            TableStEph {
                entries: ArraySeqStEphS::singleton(Pair(key, value)),
            }
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures result@.finite()
        {
            let mut keys = ArraySetStEph::empty();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                keys.insert(pair.0.clone());
            }
            keys
        }

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            let key_seq = keys.to_seq();
            let mut entries = Vec::with_capacity(key_seq.length());
            for i in 0..key_seq.length() {
                let key = key_seq.nth(i);
                let value = f(key);
                entries.push(Pair(key.clone(), value));
            }
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            TableStEph {
                entries: ArraySeqStEphS::from_vec(entries),
            }
        }

        #[verifier::external_body]
        fn map<F: Fn(&V) -> V>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom()
        {
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

        #[verifier::external_body]
        fn filter<F: Fn(&K, &V) -> B>(&mut self, f: F)
            ensures self@.dom().subset_of(old(self)@.dom())
        {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if f(&pair.0, &pair.1) {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures self@.dom().subset_of(old(self)@.dom().intersect(other@.dom()))
        {
            let mut intersection_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    Ordering::Less => i += 1,
                    Ordering::Greater => j += 1,
                    Ordering::Equal => {
                        let combined_value = combine(&pair1.1, &pair2.1);
                        intersection_entries.push(Pair(pair1.0.clone(), combined_value));
                        i += 1;
                        j += 1;
                    }
                }
            }

            self.entries = ArraySeqStEphS::from_vec(intersection_entries);
        }

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures old(self)@.dom().union(other@.dom()).subset_of(self@.dom())
        {
            let mut union_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    Ordering::Less => {
                        union_entries.push(pair1.clone());
                        i += 1;
                    }
                    Ordering::Greater => {
                        union_entries.push(pair2.clone());
                        j += 1;
                    }
                    Ordering::Equal => {
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

            self.entries = ArraySeqStEphS::from_vec(union_entries);
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self)
            ensures self@.dom().subset_of(old(self)@.dom().difference(other@.dom()))
        {
            let mut difference_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    Ordering::Less => {
                        difference_entries.push(pair1.clone());
                        i += 1;
                    }
                    Ordering::Greater => j += 1,
                    Ordering::Equal => {
                        i += 1;
                        j += 1;
                    }
                }
            }

            while i < self.entries.length() {
                difference_entries.push(self.entries.nth(i).clone());
                i += 1;
            }

            self.entries = ArraySeqStEphS::from_vec(difference_entries);
        }

        #[verifier::external_body]
        fn find(&self, key: &K) -> (result: Option<V>)
            ensures
                match result {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                }
        {
            let mut left = 0;
            let mut right = self.entries.length();

            while left < right {
                let mid = left + (right - left) / 2;
                let pair = self.entries.nth(mid);

                match key.cmp(&pair.0) {
                    Ordering::Less => right = mid,
                    Ordering::Greater => left = mid + 1,
                    Ordering::Equal => return Some(pair.1.clone()),
                }
            }

            None
        }

        #[verifier::external_body]
        fn delete(&mut self, key: &K)
            ensures !self@.contains_key(key@)
        {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if &pair.0 != key {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        #[verifier::external_body]
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            ensures self@.contains_key(key@)
        {
            if let Some(existing_value) = self.find(&key) {
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
                let mut new_entries = Vec::with_capacity(self.entries.length() + 1);
                for i in 0..self.entries.length() {
                    new_entries.push(self.entries.nth(i).clone());
                }
                new_entries.push(Pair(key, value));
                new_entries.sort_by(|a, b| a.0.cmp(&b.0));
                self.entries = ArraySeqStEphS::from_vec(new_entries);
            }
        }

        #[verifier::external_body]
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().subset_of(old(self)@.dom())
        {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if keys.find(&pair.0) {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        #[verifier::external_body]
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().subset_of(old(self)@.dom())
        {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if !keys.find(&pair.0) {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        #[verifier::external_body]
        fn entries(&self) -> (result: ArraySeqStEphS<Pair<K, V>>) {
            self.entries.clone()
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for TableStEph<K, V> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self) {
            TableStEph {
                entries: self.entries.clone(),
            }
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: Vec<Pair<K, V>>,
    ) -> (result: TableStEph<K, V>)
        ensures result@.dom().finite()
    {
        TableStEph {
            entries: ArraySeqStEphS::from_vec(entries),
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<K: StT + Ord, V: StT> Default for TableStEph<K, V> {
        fn default() -> Self {
            TableStEph::empty()
        }
    }

    impl<K: StT + Ord, V: StT> PartialEq for TableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.entries == other.entries
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for TableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEph({:?})", self.entries)
        }
    }
}
