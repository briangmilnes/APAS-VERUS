//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! LinkedList Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses LinkedList for separate chaining collision resolution.

pub mod LinkedListChainedHashTableStEph {

    use std::collections::LinkedList;
    use std::marker::PhantomData;

    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for LinkedList<(Key, Value)> {
        fn new() -> Self { LinkedList::new() }

        fn insert(&mut self, key: Key, value: Value) {
            // Update if key exists, otherwise append
            for (k, v) in self.iter_mut() {
                if k == &key {
                    *v = value;
                    return;
                }
            }
            self.push_back((key, value));
        }

        fn lookup(&self, key: &Key) -> Option<Value> {
            for (k, v) in self.iter() {
                if k == key {
                    return Some(v.clone());
                }
            }
            None
        }

        fn delete(&mut self, key: &Key) -> B {
            let mut found_idx = None;
            for (idx, (k, _)) in self.iter().enumerate() {
                if k == key {
                    found_idx = Some(idx);
                    break;
                }
            }
            if let Some(idx) = found_idx {
                let mut split_off = self.split_off(idx);
                split_off.pop_front();
                self.append(&mut split_off);
                true
            } else {
                false
            }
        }
    }

    /// LinkedList Chained Hash Table implementation.
    pub struct LinkedListChainedHashTableStEph;

    impl<Key: StT, Value: StT, Metrics: Default> ParaHashTableStEphTrait<Key, Value, LinkedList<(Key, Value)>, Metrics>
        for LinkedListChainedHashTableStEph
    {
        fn insert(table: &mut HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, key: Key, value: Value) {
            Self::insert_chained(table, key, value);
        }

        fn lookup(table: &HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, key: &Key) -> Option<Value> {
            Self::lookup_chained(table, key)
        }

        fn delete(table: &mut HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, key: &Key) -> B {
            Self::delete_chained(table, key)
        }

        fn resize(
            table: &HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>,
            new_size: N,
        ) -> HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics> {
            // Collect all key-value pairs from all chains
            let mut pairs = Vec::new();
            for chain in &table.table {
                for (k, v) in chain.iter() {
                    pairs.push((k.clone(), v.clone()));
                }
            }

            // Create new table with new size using the stored generator
            let new_table_vec = (0..new_size).map(|_| LinkedList::new()).collect();
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

    impl<Key: StT, Value: StT, Metrics: Default> ChainedHashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>
        for LinkedListChainedHashTableStEph
    {
        fn hash_index(table: &HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, _key: &Key) -> N {
            // Simple modulo hash - implementers can provide better hash function
            let hash_val = 0; // Placeholder: would use actual hash function
            hash_val % table.current_size
        }
    }
}
