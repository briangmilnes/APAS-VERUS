//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses open addressing for collision resolution.

pub mod FlatHashTable {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!: FlatEntry)
    // 8. traits (inside verus!: FlatHashTable)
    // 9. impls (inside verus!: EntryTrait for FlatEntry)
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports
    use vstd::prelude::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    /// Entry type for flat hash tables using open addressing.
    /// Represents the three states a slot can be in.
    #[derive(PartialEq)]
    pub enum FlatEntry<Key, Value> {
        /// Empty slot - never occupied
        Empty,
        /// Occupied slot with key-value pair
        Occupied(Key, Value),
        /// Deleted slot - previously occupied, now available for insertion
        Deleted,
    }

    // 8. traits

    /// Flat Hash Table trait - extends ParaHashTableStEphTrait.
    /// Uses open addressing (linear probing, quadratic probing, double hashing).
    /// Entry type is parametric - can be FlatEntry or any type implementing EntryTrait.
    pub trait FlatHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>:
        ParaHashTableStEphTrait<Key, Value, Entry, Metrics, H>
    {
        /// Probes for the next slot in the sequence.
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on probing strategy.
        fn probe(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize)
            requires
                table.current_size > 0,
            ensures
                slot < table.current_size;

        /// Finds the first available slot (Empty or Deleted) for insertion.
        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on probing strategy.
        fn find_slot(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (slot: usize)
            requires
                table.current_size > 0,
                table.table@.len() == table.current_size as int,
            ensures
                slot < table.current_size;

        /// Inserts using linear probing as default.
        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — delegates to find_slot then O(1) write.
        fn insert_with_probe(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: Key, value: Value)
            requires
                old(table).current_size > 0,
                old(table).table@.len() == old(table).current_size as int,
            ensures
                table.table@.len() == table.current_size as int,
                table.current_size == old(table).current_size,
        {
            let slot = Self::find_slot(table, &key);
            let mut entry = Entry::new();
            EntryTrait::insert(&mut entry, key, value);
            table.table.set(slot, entry);
        }

        /// Looks up using probe sequence.
        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — iterates probe sequence until found or empty.
        fn lookup_with_probe(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> Option<Value>
            requires
                table.current_size > 0,
                table.table@.len() == table.current_size as int,
        {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.table@.len() == table.current_size as int,
                    table.current_size > 0,
                decreases table.current_size - attempt,
            {
                let slot = Self::probe(table, key, attempt);
                if let Some(val) = table.table[slot].lookup(key) {
                    return Some(val);
                }
                attempt = attempt + 1;
            }
            None
        }
    }

    // 9. impls

    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for FlatEntry<Key, Value> {
        open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
            match *self {
                FlatEntry::Empty => Map::empty(),
                FlatEntry::Deleted => Map::empty(),
                FlatEntry::Occupied(k, v) => Map::empty().insert(k, v),
            }
        }

        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — constant-time enum construction.
        fn new() -> (entry: Self)
            ensures entry is Empty,
        { FlatEntry::Empty }

        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — constant-time enum assignment.
        fn insert(&mut self, key: Key, value: Value)
            ensures *self == FlatEntry::<Key, Value>::Occupied(key, value),
        { *self = FlatEntry::Occupied(key, value); }

        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — single match + key comparison.
        fn lookup(&self, key: &Key) -> (found: Option<Value>)
            ensures
                self is Empty ==> found is None,
                self is Deleted ==> found is None,
        {
            match self {
                | FlatEntry::Occupied(k, v) => {
                    if *k == *key { Some(v.clone()) } else { None }
                }
                | _ => None,
            }
        }

        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — single match + enum assignment.
        fn delete(&mut self, key: &Key) -> (deleted: bool)
            ensures
                deleted ==> *self is Deleted,
                !deleted ==> *self == *old(self),
        {
            let is_match = if let FlatEntry::Occupied(k, _) = &*self {
                *k == *key
            } else {
                false
            };
            if is_match {
                *self = FlatEntry::Deleted;
                true
            } else {
                false
            }
        }

        /// Delegates to FlatEntry's Clone impl.
        fn clone_entry(&self) -> (cloned: Self) {
            self.clone()
        }
    }

    // 11. derive impls in verus!

    impl<Key: Clone, Value: Clone> Clone for FlatEntry<Key, Value> {
        fn clone(&self) -> (cloned: Self) {
            match self {
                FlatEntry::Empty => FlatEntry::Empty,
                FlatEntry::Occupied(k, v) => FlatEntry::Occupied(k.clone(), v.clone()),
                FlatEntry::Deleted => FlatEntry::Deleted,
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for FlatEntry<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FlatEntry::Empty => write!(f, "Empty"),
                FlatEntry::Occupied(k, v) => f.debug_tuple("Occupied").field(k).field(v).finish(),
                FlatEntry::Deleted => write!(f, "Deleted"),
            }
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for FlatEntry<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
