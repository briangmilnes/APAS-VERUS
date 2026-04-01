//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses open addressing for collision resolution.

pub mod FlatHashTable {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!: FlatEntry)
    // 6. spec fns (inside verus!: spec_flat_has_key)
    // 8. traits (inside verus!: FlatHashTable — probe, find_slot abstract)
    // 9. impls (inside verus!: EntryTrait for FlatEntry)
    // 11. derive impls in verus!
    // 13. derive impls outside verus!
    //
    // Note: spec_count_empties and lemma_*_empties live in LinProbFlatHashTableStEph.rs
    // and DoubleHashFlatHashTableStEph.rs — the only two implementations that use them.

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

    // 6. spec fns

    /// Whether a flat entry slot contains the given key.
    /// Avoids EntryTrait bounds so wf specs can use it with unconstrained type params.
    pub open spec fn spec_flat_has_key<Key, Value>(entry: FlatEntry<Key, Value>, k: Key) -> bool {
        match entry {
            FlatEntry::Occupied(ek, _) => ek == k,
            _ => false,
        }
    }

    // 8. traits

    /// Flat Hash Table trait - extends ParaHashTableStEphTrait.
    /// Uses open addressing (linear probing, quadratic probing, double hashing).
    /// Entry type is parametric - can be FlatEntry or any type implementing EntryTrait.
    pub trait FlatHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>:
        ParaHashTableStEphTrait<Key, Value, Entry, Metrics, H>
    {
        /// Probes for the next slot in the sequence.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — abstract trait method; cost depends on probing strategy.
        fn probe(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize)
            requires
                table.current_size > 0,
                spec_hash_fn_valid::<Key, H>(table.spec_hash@),
            ensures
                slot < table.current_size;

        /// Finds the first available slot (Empty or Deleted) for insertion.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — abstract trait method; cost depends on probing strategy.
        fn find_slot(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (slot: usize)
            requires
                table.current_size > 0,
                table.table@.len() == table.current_size as int,
                spec_hash_fn_valid::<Key, H>(table.spec_hash@),
            ensures
                slot < table.current_size;

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

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant-time enum construction.
        fn new() -> (entry: Self)
            ensures entry is Empty,
        { FlatEntry::Empty }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant-time enum assignment.
        fn insert(&mut self, key: Key, value: Value)
            ensures
                *self == FlatEntry::<Key, Value>::Occupied(key, value),
                self.spec_entry_to_map()[key] == value,
        { *self = FlatEntry::Occupied(key, value); }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single match + key comparison.
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

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single match + enum assignment.
        fn delete(&mut self, key: &Key) -> (deleted: bool)
            ensures
                deleted ==> *self is Deleted,
                deleted ==> self.spec_entry_to_map() == Map::<Key, Value>::empty(),
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
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self,
        {
            let c = match self {
                FlatEntry::Empty => FlatEntry::Empty,
                FlatEntry::Occupied(k, v) => FlatEntry::Occupied(k.clone(), v.clone()),
                FlatEntry::Deleted => FlatEntry::Deleted,
            };
            proof { assume(c == *self); } // Clone bridge.
            c
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
