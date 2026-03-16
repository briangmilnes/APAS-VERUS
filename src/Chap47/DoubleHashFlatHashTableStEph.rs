//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Double Hashing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses double hashing for open addressing collision resolution.

pub mod DoubleHashFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!)
    // 9. impls (inside verus!)

    // 2. imports
    use std::hash::Hash;
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    /// Double Hashing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i·hh(k)) mod m
    /// Uses two hash functions to avoid both primary and secondary clustering.
    pub struct DoubleHashFlatHashTableStEph;

    // 6. spec fns

    /// Well-formedness for double hashing flat hash tables.
    /// Probe sequence: slot (h + j * s) % m for attempt j = 0, 1, 2, ...
    /// where h = hash(k) % m and s = second_hash(k, m) >= 1.
    /// Since the second hash is opaque (external_body), the spec uses an
    /// existential: there exists some step s >= 1 placing the key at its slot.
    pub open spec fn spec_doublehashflathashsteph_wf<Key, Value, Metrics, H>(
        table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
    ) -> bool {
        let m = table.current_size as int;
        table.table@.len() == m
        && m > 0
        // No duplicate keys.
        && (forall |i: int, j: int, k: Key|
            0 <= i < m && 0 <= j < m && i != j
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> !#[trigger] spec_flat_has_key(table.table@[j], k))
        // Probe chain integrity for double hashing.
        && (forall |i: int, k: Key|
            0 <= i < m
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> {
                let h = (table.spec_hash@)(k) as int % m;
                exists |s: int, n: int| #![trigger table.table@[(h + n * s) % m]] s >= 1 && 0 <= n < m
                    && (h + n * s) % m == i
                    && forall |j: int| 0 <= j < n
                        ==> !(#[trigger] table.table@[(h + j * s) % m] is Empty)
            })
    }

    // 9. impls

    impl DoubleHashFlatHashTableStEph {
        /// Compute second hash value for double hashing.
        /// APAS: hh(k) must be relatively prime to m.
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(sizeof(Key)), Span O(sizeof(Key)) — hashes key with SipHash.
        /// Strategy: Always return an odd number (works for power-of-2 sizes),
        /// and for prime sizes, ensure < m and non-zero.
        pub fn second_hash<Key: StT + Hash>(key: &Key, table_size: usize) -> (step: usize)
            requires table_size > 0,
            ensures step >= 1,
        {
            compute_second_hash(key, table_size)
        }
    }

    impl<Key: StT + Hash, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for DoubleHashFlatHashTableStEph
    {
        open spec fn spec_impl_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            spec_doublehashflathashsteph_wf(table)
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe find_slot then set.
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
                let slot = double_hash_probe(&table.hash_fn, &key, table.current_size, attempt, table.spec_hash);
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
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until found or empty.
        #[verifier::external_body]
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let step = Self::second_hash(key, m);
            let mut attempt: usize = 0;
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m > 0,
                    h < m,
                    step >= 1,
                    table.table@.len() == m as int,
                decreases m - attempt,
            {
                let slot = double_hash_probe(&table.hash_fn, key, table.current_size, attempt, table.spec_hash);
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
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until found or empty, then tombstone.
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
                let slot = double_hash_probe(&table.hash_fn, key, table.current_size, attempt, table.spec_hash);
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
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' slots, reinserts.
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
                spec_hash: table.spec_hash,
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

    impl<Key: StT + Hash, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for DoubleHashFlatHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — two hash values + arithmetic + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash1 = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let step = Self::second_hash(key, table.current_size);
            (hash1.wrapping_add(attempt.wrapping_mul(step))) % table.current_size
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until empty/deleted/matching.
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

    impl std::fmt::Debug for DoubleHashFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DoubleHashFlatHashTableStEph")
        }
    }

    impl std::fmt::Display for DoubleHashFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DoubleHashFlatHashTableStEph")
        }
    }
}
