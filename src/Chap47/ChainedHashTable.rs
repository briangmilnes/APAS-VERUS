//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses separate chaining for collision resolution.

pub mod ChainedHashTable {

    use std::marker::PhantomData;

    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    /// Parametric entry type for chained hash tables.
    /// Container type is abstract - can be Vec, LinkedList, Seq, etc.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ChainEntry<Key, Value, Container> {
        pub chain: Container,
        pub _phantom: PhantomData<(Key, Value)>,
    }

    /// Chained Hash Table trait - extends ParaHashTableStEphTrait.
    /// Uses separate chaining (linked lists or sequences) for collision resolution.
    /// Entry type is parametric - can be ChainEntry, LinkedList, or any type implementing EntryTrait.
    pub trait ChainedHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default>:
        ParaHashTableStEphTrait<Key, Value, Entry, Metrics>
    {
        /// Computes the hash index for a key.
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on hash function.
        fn hash_index(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key) -> N;

        /// Inserts into the chain at the hashed bucket.
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: Work O(1) expected, Span O(1) — agrees with APAS; hashes then delegates to EntryTrait::insert.
        fn insert_chained(table: &mut HashTable<Key, Value, Entry, Metrics>, key: Key, value: Value) {
            let index = Self::hash_index(table, &key);
            if index < table.table.len() {
                table.table[index].insert(key, value);
            }
        }

        /// Looks up in the chain at the hashed bucket.
        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — agrees with APAS; hashes then linear scan of chain.
        fn lookup_chained(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key) -> Option<Value> {
            let index = Self::hash_index(table, key);
            if index < table.table.len() {
                table.table[index].lookup(key)
            } else {
                None
            }
        }

        /// Deletes from the chain at the hashed bucket.
        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — agrees with APAS; hashes then linear scan of chain.
        fn delete_chained(table: &mut HashTable<Key, Value, Entry, Metrics>, key: &Key) -> B {
            let index = Self::hash_index(table, key);
            if index < table.table.len() {
                table.table[index].delete(key)
            } else {
                false
            }
        }
    }
}
