//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Double Hashing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses double hashing for open addressing collision resolution.

pub mod DoubleHashFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 9. impls (outside verus! — reference HashTable which contains dyn Fn types)

    // 2. imports
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

    // 9. impls (inside verus! — second_hash does not reference dyn Fn types)

    impl DoubleHashFlatHashTableStEph {
        /// Compute second hash value for double hashing.
        /// APAS: hh(k) must be relatively prime to m.
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(sizeof(Key)), Span O(sizeof(Key)) — iterates over key bytes with FNV-1a.
        /// Strategy: Always return an odd number (works for power-of-2 sizes),
        /// and for prime sizes, ensure < m and non-zero.
        #[verifier::external_body]
        pub fn second_hash<Key: StT>(key: &Key, table_size: N) -> N {
            if table_size <= 2 {
                return 1;
            }

            let key_ptr = key as *const Key as *const u8;
            let key_size = std::mem::size_of::<Key>();
            let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis

            unsafe {
                for i in 0..key_size {
                    let byte = *key_ptr.add(i);
                    hash ^= byte as u64;
                    hash = hash.wrapping_mul(0x100000001b3); // FNV prime
                }
            }

            let base = (table_size - 1) as u64;
            let mut step = ((hash % base) + 1) as N;

            if step % 2 == 0 && step < table_size - 1 {
                step += 1;
            }

            step
        }
    }

    } // verus!

    // 9. impls (outside verus! — these reference HashTable which contains dyn Fn types)

    impl<Key: StT, Value: StT, Metrics: Default> ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics>
        for DoubleHashFlatHashTableStEph
    {
        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash find_slot then O(1) write.
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: Key, value: Value) {
            let slot = Self::find_slot(table, &key);
            match &table.table[slot] {
                | FlatEntry::Occupied(k, _) if k == &key => {
                    table.table[slot] = FlatEntry::Occupied(key, value);
                }
                | FlatEntry::Empty | FlatEntry::Deleted => {
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
                | _ => {
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
            }
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until found or empty.
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> Option<Value> {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    | FlatEntry::Occupied(k, v) if k == key => return Some(v.clone()),
                    | FlatEntry::Empty => return None,
                    | FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        attempt += 1;
                    }
                }
            }
            None
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until found or empty, then tombstone.
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> B {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    | FlatEntry::Occupied(k, _) if k == key => {
                        table.table[slot] = FlatEntry::Deleted;
                        table.num_elements -= 1;
                        return true;
                    }
                    | FlatEntry::Empty => return false,
                    | FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        attempt += 1;
                    }
                }
            }
            false
        }

        /// - APAS: Work O(n + m + m'), Span O(n + m + m').
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' slots, reinserts.
        fn resize(
            table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>,
            new_size: N,
        ) -> HashTable<Key, Value, FlatEntry<Key, Value>, Metrics> {
            let mut pairs = Vec::new();
            for entry in &table.table {
                if let FlatEntry::Occupied(k, v) = entry {
                    pairs.push((k.clone(), v.clone()));
                }
            }

            let new_table_vec = (0..new_size).map(|_| FlatEntry::new()).collect();
            let new_hash_fn = (table.hash_fn_gen)(new_size);
            let mut new_table = HashTable {
                table: new_table_vec,
                hash_fn_gen: table.hash_fn_gen.clone(),
                hash_fn: new_hash_fn,
                initial_size: table.initial_size,
                current_size: new_size,
                num_elements: 0,
                metrics: Metrics::default(),
                _phantom: PhantomData,
            };

            for (key, value) in pairs {
                Self::insert(&mut new_table, key, value);
            }

            new_table
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics>
        for DoubleHashFlatHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — two hash values + arithmetic + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key, attempt: N) -> N {
            let hash1 = (table.hash_fn)(key);
            let step = Self::second_hash(key, table.current_size);
            (hash1 + (attempt * step)) % table.current_size
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until empty/deleted/matching.
        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> N {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    | FlatEntry::Empty | FlatEntry::Deleted => return slot,
                    | FlatEntry::Occupied(k, _) if k == key => return slot,
                    | _ => attempt += 1,
                }
            }
            Self::probe(table, key, 0)
        }
    }
}
