//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! LinkedList Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses LinkedList for separate chaining collision resolution.

pub mod LinkedListChainedHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 9. impls (inside verus!: EntryTrait for LinkedList; outside verus!: ParaHashTableStEphTrait, ChainedHashTable)

    // 2. imports
    use std::collections::LinkedList;
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {
        proof fn _linked_list_chained_hash_table_verified() {}

        /// LinkedList Chained Hash Table implementation.
        pub struct LinkedListChainedHashTableStEph;
    }

    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for LinkedList<(Key, Value)> {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — empty LinkedList construction.
        fn new() -> Self { LinkedList::new() }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan for duplicate key, n = chain length.
        fn insert(&mut self, key: Key, value: Value) {
            for (k, v) in self.iter_mut() {
                if k == &key {
                    *v = value;
                    return;
                }
            }
            self.push_back((key, value));
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan of chain, n = chain length.
        fn lookup(&self, key: &Key) -> Option<Value> {
            for (k, v) in self.iter() {
                if k == key {
                    return Some(v.clone());
                }
            }
            None
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan + split_off/append, n = chain length.
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

    // 9. impls (outside verus! — these reference HashTable which contains dyn Fn types)

    impl<Key: StT, Value: StT, Metrics: Default> ParaHashTableStEphTrait<Key, Value, LinkedList<(Key, Value)>, Metrics>
        for LinkedListChainedHashTableStEph
    {
        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to insert_chained.
        fn insert(table: &mut HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, key: Key, value: Value) {
            Self::insert_chained(table, key, value);
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to lookup_chained.
        fn lookup(table: &HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, key: &Key) -> Option<Value> {
            Self::lookup_chained(table, key)
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to delete_chained.
        fn delete(table: &mut HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, key: &Key) -> B {
            Self::delete_chained(table, key)
        }

        /// - APAS: Work O(n + m + m'), Span O(n + m + m').
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' lists, reinserts.
        fn resize(
            table: &HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>,
            new_size: usize,
        ) -> HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics> {
            let mut pairs = Vec::new();
            for chain in &table.table {
                for (k, v) in chain.iter() {
                    pairs.push((k.clone(), v.clone()));
                }
            }

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

            for (key, value) in pairs {
                Self::insert(&mut new_table, key, value);
            }

            new_table
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> ChainedHashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>
        for LinkedListChainedHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — delegates to stored hash function.
        fn hash_index(table: &HashTable<Key, Value, LinkedList<(Key, Value)>, Metrics>, key: &Key) -> usize {
            (table.hash_fn)(key) % table.current_size
        }
    }
}
