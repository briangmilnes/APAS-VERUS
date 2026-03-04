//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Vec Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses Vec for separate chaining collision resolution.

pub mod VecChainedHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 7. proof fns (inside verus!)
    // 9. impls (inside verus!: EntryTrait for Vec, ParaHashTableStEphTrait, ChainedHashTable)

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

        // 4. type definitions

        /// Vec Chained Hash Table implementation.
        pub struct VecChainedHashTableStEph;

        // 7. proof fns

        proof fn _vec_chained_hash_table_verified() {}

        // 9. impls

        impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for Vec<(Key, Value)> {
            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: Work O(1), Span O(1) — empty Vec construction.
            fn new() -> (entry: Self) { Vec::new() }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n) worst case, Span O(n) — linear scan for duplicate key, n = chain length.
            fn insert(&mut self, key: Key, value: Value) {
                let mut i: usize = 0;
                while i < self.len()
                    decreases self.len() - i,
                {
                    if self[i].0 == key {
                        self.remove(i);
                        self.push((key, value));
                        return;
                    }
                    i += 1;
                }
                self.push((key, value));
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan of chain, n = chain length.
            fn lookup(&self, key: &Key) -> (found: Option<Value>) {
                let mut i: usize = 0;
                while i < self.len()
                    decreases self.len() - i,
                {
                    if self[i].0 == *key {
                        return Some(self[i].1.clone());
                    }
                    i += 1;
                }
                None
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan + Vec::remove (shifts elements), n = chain length.
            fn delete(&mut self, key: &Key) -> (deleted: B) {
                let mut i: usize = 0;
                while i < self.len()
                    decreases self.len() - i,
                {
                    if self[i].0 == *key {
                        self.remove(i);
                        return true;
                    }
                    i += 1;
                }
                false
            }
        }

        // 9. impls (ParaHashTableStEphTrait, ChainedHashTable)

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, Vec<(Key, Value)>, Metrics, H>
            for VecChainedHashTableStEph
        {
            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to insert_chained.
            #[verifier::external_body]
            fn insert(table: &mut HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: Key, value: Value) {
                Self::insert_chained(table, key, value);
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to lookup_chained.
            #[verifier::external_body]
            fn lookup(table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> Option<Value> {
                Self::lookup_chained(table, key)
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to delete_chained.
            #[verifier::external_body]
            fn delete(table: &mut HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> B {
                Self::delete_chained(table, key)
            }

            /// - APAS: Work O(n + m + m'), Span O(n + m + m').
            /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' chains, reinserts.
            #[verifier::external_body]
            fn resize(
                table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>,
                new_size: usize,
            ) -> HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H> {
                let mut pairs = Vec::new();
                for chain in &table.table {
                    for (k, v) in chain.iter() {
                        pairs.push((k.clone(), v.clone()));
                    }
                }

                let new_table_vec = (0..new_size).map(|_| Vec::new()).collect();
                let mut new_table = HashTable {
                    table: new_table_vec,
                    hash_fn: table.hash_fn.clone(),
                    initial_size: table.initial_size,
                    current_size: new_size,
                    num_elements: 0,
                    metrics: Metrics::default(),
                    _phantom: PhantomData,
                };

                for (key, value) in pairs {
                    Self::insert(&mut new_table, key, value);
                }

                new_table
            }
        }

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ChainedHashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>
            for VecChainedHashTableStEph
        {
            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: Work O(1), Span O(1) — delegates to stored hash function.
            #[verifier::external_body]
            fn hash_index(table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> usize {
                (table.hash_fn)(key, table.current_size)
            }
        }
    }
}
