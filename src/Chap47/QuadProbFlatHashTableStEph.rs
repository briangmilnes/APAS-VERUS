//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses quadratic probing for open addressing collision resolution.

pub mod QuadProbFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 9. impls (inside verus!)
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    /// Quadratic Probing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i²) mod m
    pub struct QuadProbFlatHashTableStEph;

    // 9. impls

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for QuadProbFlatHashTableStEph
    {
        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe find_slot then set.
        #[verifier::external_body]
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: Key, value: Value) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.table@.len() == table.current_size as int,
                    table.current_size == old(table).current_size,
                    table.num_elements == old(table).num_elements,
                decreases table.current_size - attempt,
            {
                let slot = quadratic_probe(&table.hash_fn, &key, table.current_size, attempt);
                let entry = table.table[slot].clone();
                if let FlatEntry::Occupied(k, _) = &entry {
                    if *k == key {
                        table.table.set(slot, FlatEntry::Occupied(key, value));
                        return;
                    }
                } else {
                    table.table.set(slot, FlatEntry::Occupied(key, value));
                    if table.num_elements < usize::MAX {
                        table.num_elements = table.num_elements + 1;
                    }
                    return;
                }
                attempt = attempt + 1;
            }
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe sequence until found or empty.
        #[verifier::external_body]
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.table@.len() == table.current_size as int,
                decreases table.current_size - attempt,
            {
                let slot = quadratic_probe(&table.hash_fn, key, table.current_size, attempt);
                let entry = table.table[slot].clone();
                if let FlatEntry::Occupied(k, v) = entry {
                    if k == *key {
                        return Some(v);
                    }
                } else if let FlatEntry::Empty = entry {
                    return None;
                }
                attempt = attempt + 1;
            }
            None
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe until found or empty, then tombstone.
        #[verifier::external_body]
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.table@.len() == table.current_size as int,
                    table.current_size == old(table).current_size,
                decreases table.current_size - attempt,
            {
                let slot = quadratic_probe(&table.hash_fn, key, table.current_size, attempt);
                let entry = table.table[slot].clone();
                if let FlatEntry::Occupied(k, _) = &entry {
                    if *k == *key {
                        table.table.set(slot, FlatEntry::Deleted);
                        if table.num_elements > 0 {
                            table.num_elements = table.num_elements - 1;
                        }
                        return true;
                    }
                } else if let FlatEntry::Empty = &entry {
                    return false;
                }
                attempt = attempt + 1;
            }
            false
        }

        /// - APAS: Work O(n + m + m'), Span O(n + m + m').
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs from m slots, creates m' new slots, reinserts n pairs.
        #[verifier::external_body]
        fn resize(
            table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
            new_size: usize,
        ) -> (resized: HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) {
            let mut pairs: Vec<(Key, Value)> = Vec::new();
            let mut i: usize = 0;
            while i < table.table.len()
                invariant
                    i <= table.table@.len(),
                    table.table@.len() == table.current_size as int,
                decreases table.table.len() - i,
            {
                let entry = table.table[i].clone();
                if let FlatEntry::Occupied(k, v) = entry {
                    pairs.push((k, v));
                }
                i = i + 1;
            }

            let mut new_table_vec: Vec<FlatEntry<Key, Value>> = Vec::new();
            let mut k: usize = 0;
            while k < new_size
                invariant
                    k <= new_size,
                    new_table_vec@.len() == k as int,
                decreases new_size - k,
            {
                new_table_vec.push(FlatEntry::Empty);
                k = k + 1;
            }
            let mut new_table = HashTable {
                table: new_table_vec,
                hash_fn: table.hash_fn.clone(),
                initial_size: table.initial_size,
                current_size: new_size,
                num_elements: 0,
                metrics: Metrics::default(),
                _phantom: PhantomData,
            };

            let mut j: usize = 0;
            while j < pairs.len()
                invariant
                    j <= pairs@.len(),
                    new_size > 0,
                    new_table.current_size == new_size,
                    new_table.table@.len() == new_table.current_size as int,
                    new_table.num_elements <= j,
                decreases pairs.len() - j,
            {
                let key = pairs[j].0.clone();
                let value = pairs[j].1.clone();
                Self::insert(&mut new_table, key, value);
                j = j + 1;
            }

            new_table
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for QuadProbFlatHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — hash + i² + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash_val = call_hash_fn(&table.hash_fn, key, table.current_size);
            (hash_val.wrapping_add(attempt.wrapping_mul(attempt))) % table.current_size
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe up to ⌈m/2⌉ (Lemma 47.1).
        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (slot: usize) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.current_size > 0,
                    table.table@.len() == table.current_size as int,
                decreases table.current_size - attempt,
            {
                let slot = Self::probe(table, key, attempt);
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Empty | FlatEntry::Deleted => { return slot; },
                    FlatEntry::Occupied(k, _) => {
                        if k == *key {
                            return slot;
                        }
                    },
                }
                attempt = attempt + 1;
            }
            Self::probe(table, key, 0)
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for QuadProbFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QuadProbFlatHashTableStEph")
        }
    }

    impl std::fmt::Display for QuadProbFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QuadProbFlatHashTableStEph")
        }
    }
}
