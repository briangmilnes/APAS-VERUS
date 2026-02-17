//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses open addressing for collision resolution.

pub mod FlatHashTable {

    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    /// Entry type for flat hash tables using open addressing.
    /// Represents the three states a slot can be in.
    #[derive(Clone, Debug, PartialEq)]
    pub enum FlatEntry<Key, Value> {
        /// Empty slot - never occupied
        Empty,
        /// Occupied slot with key-value pair
        Occupied(Key, Value),
        /// Deleted slot - previously occupied, now available for insertion
        Deleted,
    }

    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for FlatEntry<Key, Value> {
        fn new() -> Self { FlatEntry::Empty }
        fn insert(&mut self, key: Key, value: Value) { *self = FlatEntry::Occupied(key, value); }

        fn lookup(&self, key: &Key) -> Option<Value> {
            match self {
                | FlatEntry::Occupied(k, v) if k == key => Some(v.clone()),
                | _ => None,
            }
        }

        fn delete(&mut self, key: &Key) -> B {
            match self {
                | FlatEntry::Occupied(k, _) if k == key => {
                    *self = FlatEntry::Deleted;
                    true
                }
                | _ => false,
            }
        }
    }

    /// Flat Hash Table trait - extends ParaHashTableStEphTrait.
    /// Uses open addressing (linear probing, quadratic probing, double hashing).
    /// Entry type is parametric - can be FlatEntry or any type implementing EntryTrait.
    pub trait FlatHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default>:
        ParaHashTableStEphTrait<Key, Value, Entry, Metrics>
    {
        /// Probes for the next slot in the sequence.
        /// APAS: Work O(1), Span O(1).
        fn probe(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key, attempt: N) -> N;

        /// Finds the first available slot (Empty or Deleted) for insertion.
        /// APAS: Work O(1) expected, Span O(1).
        fn find_slot(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key) -> N;

        /// Inserts using linear probing as default.
        /// APAS: Work O(1) expected, Span O(1).
        fn insert_with_probe(table: &mut HashTable<Key, Value, Entry, Metrics>, key: Key, value: Value) {
            let slot = Self::find_slot(table, &key);
            if slot < table.table.len() {
                table.table[slot].insert(key, value);
            }
        }

        /// Looks up using probe sequence.
        /// APAS: Work O(1) expected, Span O(1).
        fn lookup_with_probe(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key) -> Option<Value> {
            for attempt in 0..table.current_size {
                let slot = Self::probe(table, key, attempt);
                if slot < table.table.len() {
                    if let Some(val) = table.table[slot].lookup(key) {
                        return Some(val);
                    }
                }
            }
            None
        }
    }
}
