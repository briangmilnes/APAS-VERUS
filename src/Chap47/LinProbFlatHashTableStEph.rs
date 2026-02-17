//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Linear Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses linear probing for open addressing collision resolution.

pub mod LinProbFlatHashTableStEph {

    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;
    use std::marker::PhantomData;

    /// Linear Probing Flat Hash Table implementation.
    pub struct LinProbFlatHashTableStEph;

    impl<Key: StT, Value: StT, Metrics: Default> ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics>
        for LinProbFlatHashTableStEph
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
                    // This shouldn't happen if find_slot works correctly
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
            }
        }

        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> Option<Value> {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    | FlatEntry::Occupied(k, v) if k == key => return Some(v.clone()),
                    | FlatEntry::Empty => return None, // Stop at Empty - key not in table
                    | FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        // Continue probing past Deleted or non-matching Occupied
                        attempt += 1;
                    }
                }
            }
            None
        }

        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> B {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    | FlatEntry::Occupied(k, _) if k == key => {
                        // Mark as Deleted (tombstone) to maintain probe chain integrity
                        table.table[slot] = FlatEntry::Deleted;
                        table.num_elements -= 1;
                        return true;
                    }
                    | FlatEntry::Empty => return false, // Key not found, stop at Empty
                    | FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        // Continue probing
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
        for LinProbFlatHashTableStEph
    {
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key, attempt: N) -> N {
            let hash_val = (table.hash_fn)(key);

            // Linear probing: (hash(key) + attempt) mod size
            (hash_val + attempt) % table.current_size
        }

        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> N {
            // Find first Empty or Deleted slot (can reuse Deleted slots for insertion)
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    | FlatEntry::Empty | FlatEntry::Deleted => return slot,
                    | FlatEntry::Occupied(k, _) if k == key => return slot, // Update existing key
                    | _ => attempt += 1,
                }
            }
            // Table full - return first slot as fallback (shouldn't happen with proper load factor)
            Self::probe(table, key, 0)
        }
    }
}
