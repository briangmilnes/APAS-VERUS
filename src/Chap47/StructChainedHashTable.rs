//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Struct Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses custom linked list struct for separate chaining collision resolution.

pub mod StructChainedHashTable {

    use std::marker::PhantomData;

    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    /// Custom linked list node.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Node<Key, Value> {
        pub key: Key,
        pub value: Value,
        pub next: Option<Box<Node<Key, Value>>>,
    }

    /// Custom linked list for chained hash table.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ChainList<Key, Value> {
        pub head: Option<Box<Node<Key, Value>>>,
    }

    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for ChainList<Key, Value> {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — empty list construction.
        fn new() -> Self { ChainList { head: None } }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan for duplicate key, then head insert, n = chain length.
        fn insert(&mut self, key: Key, value: Value) {
            // Search for existing key and update
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.key == key {
                    node.value = value;
                    return;
                }
                current = &mut node.next;
            }
            // Key not found, insert at head
            let new_node = Box::new(Node {
                key,
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan of linked list, n = chain length.
        fn lookup(&self, key: &Key) -> Option<Value> {
            let mut current = &self.head;
            while let Some(node) = current {
                if &node.key == key {
                    return Some(node.value.clone());
                }
                current = &node.next;
            }
            None
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan + pointer surgery, n = chain length.
        fn delete(&mut self, key: &Key) -> B {
            let mut current = &mut self.head;
            loop {
                match current {
                    | None => return false,
                    | Some(node) if &node.key == key => {
                        *current = node.next.take();
                        return true;
                    }
                    | Some(node) => {
                        current = &mut node.next;
                    }
                }
            }
        }
    }

    impl<Key, Value> Default for ChainList<Key, Value> {
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — empty list construction.
        fn default() -> Self { ChainList { head: None } }
    }

    /// Struct Chained Hash Table implementation.
    pub struct StructChainedHashTableStEph;

    impl<Key: StT, Value: StT, Metrics: Default> ParaHashTableStEphTrait<Key, Value, ChainList<Key, Value>, Metrics>
        for StructChainedHashTableStEph
    {
        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to insert_chained.
        fn insert(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics>, key: Key, value: Value) {
            Self::insert_chained(table, key, value);
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to lookup_chained.
        fn lookup(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics>, key: &Key) -> Option<Value> {
            Self::lookup_chained(table, key)
        }

        /// - APAS: Work O(1+α) expected, Span O(1+α).
        /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to delete_chained.
        fn delete(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics>, key: &Key) -> B {
            Self::delete_chained(table, key)
        }

        /// - APAS: Work O(n + m + m'), Span O(n + m + m').
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — traverses all chains, creates m' lists, reinserts.
        fn resize(
            table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics>,
            new_size: N,
        ) -> HashTable<Key, Value, ChainList<Key, Value>, Metrics> {
            // Collect all key-value pairs from all chains
            let mut pairs = Vec::new();
            for chain in &table.table {
                let mut current = &chain.head;
                while let Some(node) = current {
                    pairs.push((node.key.clone(), node.value.clone()));
                    current = &node.next;
                }
            }

            // Create new table with new size using the stored generator
            let new_table_vec = (0..new_size).map(|_| ChainList::new()).collect();
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

            // Reinsert all pairs into new table
            for (key, value) in pairs {
                Self::insert(&mut new_table, key, value);
            }

            new_table
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> ChainedHashTable<Key, Value, ChainList<Key, Value>, Metrics>
        for StructChainedHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — placeholder always returns 0; should use actual hash function.
        fn hash_index(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics>, _key: &Key) -> N {
            // Simple modulo hash - implementers can provide better hash function
            let hash_val = 0; // Placeholder: would use actual hash function
            hash_val % table.current_size
        }
    }
}
