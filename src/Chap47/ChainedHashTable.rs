//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses separate chaining for collision resolution.

pub mod ChainedHashTable {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 8. traits (outside verus! — references HashTable which contains dyn Fn types)
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
        proof fn _chained_hash_table_verified() {}

        /// Parametric entry type for chained hash tables.
        /// Container type is abstract - can be Vec, LinkedList, Seq, etc.
        #[verifier::reject_recursive_types(Key)]
        #[verifier::reject_recursive_types(Value)]
        #[verifier::reject_recursive_types(Container)]
        pub struct ChainEntry<Key, Value, Container> {
            pub chain: Container,
            pub _phantom: PhantomData<(Key, Value)>,
        }

        #[cfg(verus_keep_ghost)]
        impl<Key, Value, Container: core::cmp::PartialEq> PartialEqSpecImpl for ChainEntry<Key, Value, Container> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self.chain == other.chain }
        }

        impl<Key, Value, Container: Clone> Clone for ChainEntry<Key, Value, Container> {
            #[verifier::external_body]
            fn clone(&self) -> (result: Self)
                ensures result == *self
            {
                ChainEntry { chain: self.chain.clone(), _phantom: PhantomData }
            }
        }

        impl<Key, Value, Container: core::cmp::PartialEq> core::cmp::PartialEq for ChainEntry<Key, Value, Container> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (self.chain == other.chain)
            {
                let r = self.chain == other.chain;
                proof { accept(r == (self.chain == other.chain)); }
                r
            }
        }

        impl<Key, Value, Container: core::cmp::Eq> core::cmp::Eq for ChainEntry<Key, Value, Container> {}
    }

    // 8. traits (outside verus! — references HashTable which contains dyn Fn types)

    /// Chained Hash Table trait - extends ParaHashTableStEphTrait.
    /// Uses separate chaining (linked lists or sequences) for collision resolution.
    /// Entry type is parametric - can be ChainEntry, LinkedList, or any type implementing EntryTrait.
    pub trait ChainedHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default>:
        ParaHashTableStEphTrait<Key, Value, Entry, Metrics>
    {
        /// Computes the hash index for a key.
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on hash function.
        fn hash_index(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key) -> usize;

        /// Inserts into the chain at the hashed bucket, updating num_elements on new keys.
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — lookup to check existence, then chain insert.
        fn insert_chained(table: &mut HashTable<Key, Value, Entry, Metrics>, key: Key, value: Value) {
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
        fn lookup_chained(table: &HashTable<Key, Value, Entry, Metrics>, key: &Key) -> Option<Value> {
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
        fn delete_chained(table: &mut HashTable<Key, Value, Entry, Metrics>, key: &Key) -> B {
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
}
