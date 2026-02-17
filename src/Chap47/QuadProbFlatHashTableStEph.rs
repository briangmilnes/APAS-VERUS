//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses quadratic probing for open addressing collision resolution.

pub mod QuadProbFlatHashTableStEph {

    use std::marker::PhantomData;

    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    /// Quadratic Probing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i²) mod m
    pub struct QuadProbFlatHashTableStEph;

    impl<Key: StT, Value: StT, Metrics: Default> ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics>
        for QuadProbFlatHashTableStEph
    {
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: Key, value: Value) {
            let slot = Self::find_slot(table, &key);
            match &table.table[slot] {
                | FlatEntry::Occupied(k, _) if k == &key => {
                    // Update existing
                    table.table[slot] = FlatEntry::Occupied(key, value);
                }
                | FlatEntry::Empty | FlatEntry::Deleted => {
                    // Insert new
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
                | _ => {
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
            }
        }

        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> Option<Value> {
            let mut attempt = 0;
            let max_attempts = table.current_size.div_ceil(2); // APAS Lemma 47.1: first ⌈m/2⌉ probes are distinct
            while attempt < max_attempts {
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

        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> B {
            let mut attempt = 0;
            let max_attempts = table.current_size.div_ceil(2); // APAS Lemma 47.1: first ⌈m/2⌉ probes are distinct
            while attempt < max_attempts {
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

        fn resize(
            table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>,
            new_size: N,
        ) -> HashTable<Key, Value, FlatEntry<Key, Value>, Metrics> {
            // Collect all key-value pairs from old table
            let mut pairs = Vec::new();
            for entry in &table.table {
                if let FlatEntry::Occupied(k, v) = entry {
                    pairs.push((k.clone(), v.clone()));
                }
            }

            // Create new table with new size using the stored generator
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

            // Reinsert all pairs into new table
            for (key, value) in pairs {
                Self::insert(&mut new_table, key, value);
            }

            new_table
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics>
        for QuadProbFlatHashTableStEph
    {
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key, attempt: N) -> N {
            let hash_val = (table.hash_fn)(key);

            // Quadratic probing: (hash(key) + i²) mod size
            (hash_val + (attempt * attempt)) % table.current_size
        }

        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> N {
            let mut attempt = 0;
            let max_attempts = table.current_size.div_ceil(2); // APAS Lemma 47.1: first ⌈m/2⌉ probes are distinct
            while attempt < max_attempts {
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
