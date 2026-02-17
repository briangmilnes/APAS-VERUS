//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric Nested Hash Table - Sequential Ephemeral (Chapter 47, Section 1.1).
//! A parametric implementation of hash tables using nested tables.
//! Work: insert O(1), lookup O(1), delete O(1) expected with constant load factor.
//! Span: O(1) (sequential).

pub mod ParaHashTableStEph {

    use std::fmt::Display;
    use std::marker::PhantomData;
    use std::rc::Rc;

    use crate::Types::Types::*;

    /// Hash function generator: takes table size, returns hash function for that size.
    /// This allows the hash function to adapt to different table sizes (e.g., hash(key) mod size).
    /// Uses Rc for clonability during resize operations.
    pub type HashFunGen<K> = Rc<dyn Fn(N) -> Box<dyn Fn(&K) -> N>>;

    /// Hash function: takes a key, returns hash code.
    pub type HashFun<K> = Box<dyn Fn(&K) -> N>;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct LoadAndSize {
        pub load: f64,
        pub size: N,
    }

    /// Parametric nested hash table structure.
    pub struct HashTable<Key, Value, Entry, Metrics> {
        pub table: Vec<Entry>,
        pub hash_fn_gen: HashFunGen<Key>,
        pub hash_fn: HashFun<Key>,
        pub initial_size: N,
        pub current_size: N,
        pub num_elements: N,
        pub metrics: Metrics,
        pub _phantom: PhantomData<(Key, Value)>,
    }

    /// Trait for parametric nested hash tables.
    /// Entry type must implement this trait to define how Key and Value are stored.
    pub trait EntryTrait<Key, Value> {
        fn new()                        -> Self;
        fn insert(&mut self, key: Key, value: Value);
        fn lookup(&self, key: &Key)     -> Option<Value>;
        fn delete(&mut self, key: &Key) -> B;
    }

    /// Trait for parametric nested hash tables.
    pub trait ParaHashTableStEphTrait<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default> {
        /// Creates an empty hash table with the given initial size.
        /// Takes a hash function generator that produces hash functions for different table sizes.
        /// APAS: Work O(m), Span O(m) where m is initial size.
        fn createTable(hash_fn_gen: HashFunGen<Key>, initial_size: N)           -> HashTable<Key, Value, Entry, Metrics> {
            let table = (0..initial_size).map(|_| Entry::new()).collect();
            let hash_fn = hash_fn_gen(initial_size);
            HashTable {
                table,
                hash_fn_gen,
                hash_fn,
                initial_size,
                current_size: initial_size,
                num_elements: 0,
                metrics: Metrics::default(),
                _phantom: PhantomData,
            }
        }

        /// Inserts a key-value pair into the hash table.
        /// APAS: Work O(1) expected, Span O(1).
        fn insert(table: &mut HashTable<Key, Value, Entry, Metrics>, key: Key, value: Value);

        /// Looks up a key in the hash table, returning its value if found.
        /// APAS: Work O(1) expected, Span O(1).
        fn lookup(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key)     -> Option<Value>;

        /// Deletes a key from the hash table if it exists.
        /// APAS: Work O(1) expected, Span O(1).
        fn delete(table: &mut HashTable<Key, Value, Entry, Metrics>, key: &Key) -> B;

        /// Accessor for metrics field.
        fn metrics(table: &HashTable<Key, Value, Entry, Metrics>)               -> &Metrics { &table.metrics }

        /// Returns the load (number of entries) and size (table capacity).
        /// APAS: Work O(1), Span O(1).
        /// Load factor α = load/size = num_elements/size
        fn loadAndSize(table: &HashTable<Key, Value, Entry, Metrics>)           -> LoadAndSize {
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
        /// Uses the stored hash function generator to create a new hash function for the new size.
        /// APAS: Work O(n + m + m'), Span O(n + m + m') where n is number of elements,
        /// m is old size, m' is new size.
        fn resize(table: &HashTable<Key, Value, Entry, Metrics>, new_size: N)   -> HashTable<Key, Value, Entry, Metrics>;
    }
}
