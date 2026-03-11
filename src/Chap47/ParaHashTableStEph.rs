//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric Nested Hash Table - Sequential Ephemeral (Chapter 47, Section 1.1).
//! A parametric implementation of hash tables using nested tables.
//! Work: insert O(1), lookup O(1), delete O(1) expected with constant load factor.
//! Span: O(1) (sequential).

pub mod ParaHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!: LoadAndSize, HashTable)
    // 6. spec fns
    // 8. traits (inside verus!: EntryTrait, ParaHashTableStEphTrait)
    // 13. derive impls outside verus!

    // 2. imports
    use std::fmt::Display;
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    #[derive(Clone, Copy, PartialEq)]
    pub struct LoadAndSize {
        pub load: f64,
        pub size: usize,
    }

    /// Parametric nested hash table structure.
    /// Generic `H` is the hash function type: takes (&Key, usize) and returns an index.
    pub struct HashTable<Key, Value, Entry, Metrics, H> {
        pub table: Vec<Entry>,
        pub hash_fn: H,
        pub initial_size: usize,
        pub current_size: usize,
        pub num_elements: usize,
        pub metrics: Metrics,
        pub _phantom: PhantomData<(Key, Value)>,
    }

    // 6. spec fns

    pub open spec fn spec_hashtable_wf<Key, Value, Entry, Metrics, H>(table: &HashTable<Key, Value, Entry, Metrics, H>) -> bool {
        table.table@.len() == table.current_size as int
    }

    // 8. traits

    /// Trait for parametric nested hash tables.
    /// Entry type must implement this trait to define how Key and Value are stored.
    pub trait EntryTrait<Key, Value> : Sized {
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn new() -> (entry: Self);
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn insert(&mut self, key: Key, value: Value);
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn lookup(&self, key: &Key) -> (found: Option<Value>);
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn delete(&mut self, key: &Key) -> (deleted: bool);
    }

    /// Trait for parametric nested hash tables.
    pub trait ParaHashTableStEphTrait<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone> {
        /// Creates an empty hash table with the given initial size.
        /// Takes a hash function that maps (&Key, table_size) to a bucket index.
        /// - APAS: Work O(m), Span O(m) where m is initial size.
        /// - Claude-Opus-4.6: Work O(m), Span O(m) — agrees with APAS; iterates m times to create entries.
        fn createTable(hash_fn: H, initial_size: usize) -> (table: HashTable<Key, Value, Entry, Metrics, H>)
            ensures
                table.initial_size == initial_size,
                table.current_size == initial_size,
                table.num_elements == 0,
                table.table@.len() == initial_size as int,
                spec_hashtable_wf(&table),
        {
            let mut table_vec: Vec<Entry> = Vec::new();
            let mut i: usize = 0;
            while i < initial_size
                invariant
                    i <= initial_size,
                    table_vec@.len() == i as int,
                decreases initial_size - i,
            {
                table_vec.push(Entry::new());
                i += 1;
            }
            HashTable {
                table: table_vec,
                hash_fn,
                initial_size,
                current_size: initial_size,
                num_elements: 0,
                metrics: Metrics::default(),
                _phantom: PhantomData,
            }
        }

        /// Inserts a key-value pair into the hash table.
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn insert(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: Key, value: Value)
            requires
                old(table).current_size > 0,
                old(table).table@.len() == old(table).current_size as int,
                old(table).num_elements < usize::MAX,
            ensures
                table.table@.len() == table.current_size as int;

        /// Looks up a key in the hash table, returning its value if found.
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn lookup(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> Option<Value>
            requires
                table.current_size > 0,
                table.table@.len() == table.current_size as int;

        /// Deletes a key from the hash table if it exists.
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn delete(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (deleted: bool)
            requires
                old(table).current_size > 0,
                old(table).table@.len() == old(table).current_size as int,
            ensures
                table.table@.len() == table.current_size as int;

        /// Accessor for metrics field.
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — field access.
        fn metrics(table: &HashTable<Key, Value, Entry, Metrics, H>) -> (m: &Metrics)
            ensures m == &table.metrics,
        { &table.metrics }

        /// Returns the load (number of entries) and size (table capacity).
        /// Load factor α = load/size = num_elements/size
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — agrees with APAS; field reads and one division.
        #[verifier::external_body]
        fn loadAndSize(table: &HashTable<Key, Value, Entry, Metrics, H>) -> (load_and_size: LoadAndSize)
            ensures load_and_size.size == table.current_size,
        {
            let load_factor = if table.current_size == 0 {
                0.0
            } else {
                table.num_elements as f64 / table.current_size as f64
            };
            LoadAndSize {
                load: load_factor,
                size: table.current_size,
            }
        }

        /// Resizes the hash table to a new size and rehashes all entries.
        /// Clones the stored hash function for the new table.
        /// - APAS: Work O(n + m + m'), Span O(n + m + m') where n is number of elements,
        ///   m is old size, m' is new size.
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn resize(table: &HashTable<Key, Value, Entry, Metrics, H>, new_size: usize) -> (resized: HashTable<Key, Value, Entry, Metrics, H>)
            requires
                new_size > 0,
            ensures
                resized.current_size == new_size,
                resized.table@.len() == new_size as int;
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for LoadAndSize {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("LoadAndSize")
                .field("load", &self.load)
                .field("size", &self.size)
                .finish()
        }
    }

    impl std::fmt::Display for LoadAndSize {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LoadAndSize(load={}, size={})", self.load, self.size)
        }
    }

    impl<Key, Value, Entry: std::fmt::Debug, Metrics: std::fmt::Debug, H> std::fmt::Debug
        for HashTable<Key, Value, Entry, Metrics, H>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("HashTable")
                .field("current_size", &self.current_size)
                .field("num_elements", &self.num_elements)
                .field("initial_size", &self.initial_size)
                .finish()
        }
    }

    impl<Key, Value, Entry: std::fmt::Debug, Metrics: std::fmt::Debug, H> std::fmt::Display
        for HashTable<Key, Value, Entry, Metrics, H>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HashTable(size={}, elements={})", self.current_size, self.num_elements)
        }
    }
}
