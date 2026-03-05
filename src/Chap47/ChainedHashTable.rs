//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses separate chaining for collision resolution.

pub mod ChainedHashTable {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!: ChainEntry)
    // 7. proof fns (inside verus!)
    // 8. traits (inside verus!: ChainedHashTable)
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::vstdplus::accept::accept;
    use crate::Types::Types::*;

    verus! {

        // 4. type definitions

        /// Parametric entry type for chained hash tables.
        /// Container type is abstract - can be Vec, LinkedList, Seq, etc.
        #[verifier::reject_recursive_types(Key)]
        #[verifier::reject_recursive_types(Value)]
        #[verifier::reject_recursive_types(Container)]
        pub struct ChainEntry<Key, Value, Container> {
            pub chain: Container,
            pub _phantom: PhantomData<(Key, Value)>,
        }

        // 7. proof fns

        proof fn _chained_hash_table_verified() {}

        // 8. traits

        /// Chained Hash Table trait - extends ParaHashTableStEphTrait.
        /// Uses separate chaining (linked lists or sequences) for collision resolution.
        /// Entry type is parametric - can be ChainEntry, LinkedList, or any type implementing EntryTrait.
        pub trait ChainedHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>:
            ParaHashTableStEphTrait<Key, Value, Entry, Metrics, H>
        {
            /// Computes the hash index for a key.
            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on hash function.
            fn hash_index(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (index: usize)
                requires
                    table.current_size > 0,
                ensures
                    index < table.current_size;

            /// Inserts into the chain at the hashed bucket, updating num_elements on new keys.
            /// - APAS: Work O(1) expected, Span O(1).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — lookup to check existence, then chain insert.
            #[verifier::external_body]
            fn insert_chained(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: Key, value: Value)
                requires
                    old(table).current_size > 0,
                    old(table).table@.len() == old(table).current_size as int,
                    old(table).num_elements < usize::MAX,
                ensures
                    table.table@.len() == table.current_size as int,
            {
                let index = Self::hash_index(table, &key);
                if index < table.table.len() {
                    let existed = table.table[index].lookup(&key).is_some();
                    table.table[index].insert(key, value);
                    if !existed {
                        table.num_elements += 1;
                    }
                }
            }

            /// Looks up in the chain at the hashed bucket.
            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — agrees with APAS; hashes then linear scan of chain.
            fn lookup_chained(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> Option<Value>
                requires
                    table.current_size > 0,
                    table.table@.len() == table.current_size as int,
            {
                let index = Self::hash_index(table, key);
                if index < table.table.len() {
                    table.table[index].lookup(key)
                } else {
                    None
                }
            }

            /// Deletes from the chain at the hashed bucket, updating num_elements on removal.
            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — agrees with APAS; hashes then linear scan of chain.
            #[verifier::external_body]
            fn delete_chained(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (deleted: bool)
                requires
                    old(table).current_size > 0,
                    old(table).table@.len() == old(table).current_size as int,
                ensures
                    table.table@.len() == table.current_size as int,
            {
                let index = Self::hash_index(table, key);
                if index < table.table.len() {
                    let deleted = table.table[index].delete(key);
                    if deleted {
                        table.num_elements -= 1;
                    }
                    deleted
                } else {
                    false
                }
            }
        }

        // 11. derive impls in verus!

        #[cfg(verus_keep_ghost)]
        impl<Key, Value, Container: core::cmp::PartialEq> PartialEqSpecImpl for ChainEntry<Key, Value, Container> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self.chain == other.chain }
        }

        impl<Key, Value, Container: Clone> Clone for ChainEntry<Key, Value, Container> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned == *self
            {
                let cloned = ChainEntry { chain: self.chain.clone(), _phantom: PhantomData };
                proof { assume(cloned == *self); }
                cloned
            }
        }

        impl<Key, Value, Container: core::cmp::PartialEq> core::cmp::PartialEq for ChainEntry<Key, Value, Container> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self.chain == other.chain)
            {
                let equal = self.chain == other.chain;
                proof { accept(equal == (self.chain == other.chain)); }
                equal
            }
        }

        impl<Key, Value, Container: core::cmp::Eq> core::cmp::Eq for ChainEntry<Key, Value, Container> {}
    }

    // 13. derive impls outside verus!

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug, Container: std::fmt::Debug> std::fmt::Debug
        for ChainEntry<Key, Value, Container>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ChainEntry")
                .field("chain", &self.chain)
                .finish()
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug, Container: std::fmt::Debug> std::fmt::Display
        for ChainEntry<Key, Value, Container>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
