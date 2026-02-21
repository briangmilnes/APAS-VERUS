//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Struct Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses custom linked list struct for separate chaining collision resolution.

pub mod StructChainedHashTable {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 9. impls (inside verus!: EntryTrait for ChainList; outside verus!: ParaHashTableStEphTrait, ChainedHashTable)
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {
        proof fn _struct_chained_hash_table_verified() {}

        // 4. type definitions

        /// Custom linked list node.
        #[verifier::reject_recursive_types(Key)]
        #[verifier::reject_recursive_types(Value)]
        pub struct Node<Key, Value> {
            pub key: Key,
            pub value: Value,
            pub next: Option<Box<Node<Key, Value>>>,
        }

        /// Custom linked list for chained hash table.
        #[verifier::reject_recursive_types(Key)]
        #[verifier::reject_recursive_types(Value)]
        pub struct ChainList<Key, Value> {
            pub head: Option<Box<Node<Key, Value>>>,
        }

        #[cfg(verus_keep_ghost)]
        impl<Key: PartialEq, Value: PartialEq> PartialEqSpecImpl for Node<Key, Value> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
        }

        impl<Key: Clone, Value: Clone> Clone for Node<Key, Value> {
            fn clone(&self) -> (result: Self)
                ensures result == *self
                decreases self
            {
                let result = Node {
                    key: self.key.clone(),
                    value: self.value.clone(),
                    next: match &self.next {
                        None => None,
                        Some(b) => Some(Box::new((**b).clone())),
                    },
                };
                proof { assume(result == *self); }
                result
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::PartialEq for Node<Key, Value> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (*self == *other)
                decreases self, other
            {
                let r = self.key == other.key
                    && self.value == other.value
                    && match (&self.next, &other.next) {
                        (None, None) => true,
                        (Some(a), Some(b)) => (**a) == (**b),
                        _ => false,
                    };
                proof { assume(r == (*self == *other)); }
                r
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::Eq for Node<Key, Value> {}

        #[cfg(verus_keep_ghost)]
        impl<Key: PartialEq, Value: PartialEq> PartialEqSpecImpl for ChainList<Key, Value> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
        }

        impl<Key: Clone, Value: Clone> Clone for ChainList<Key, Value> {
            fn clone(&self) -> (result: Self)
                ensures result == *self
            {
                let result = ChainList {
                    head: match &self.head {
                        None => None,
                        Some(b) => Some(Box::new((**b).clone())),
                    },
                };
                proof { assume(result == *self); }
                result
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::PartialEq for ChainList<Key, Value> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (*self == *other)
            {
                let r = match (&self.head, &other.head) {
                    (None, None) => true,
                    (Some(a), Some(b)) => (**a) == (**b),
                    _ => false,
                };
                proof { assume(r == (*self == *other)); }
                r
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::Eq for ChainList<Key, Value> {}

        impl<Key, Value> Default for ChainList<Key, Value> {
            fn default() -> Self { ChainList { head: None } }
        }

        /// Struct Chained Hash Table implementation.
        pub struct StructChainedHashTableStEph;
    }

    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for ChainList<Key, Value> {
        fn new() -> Self { ChainList { head: None } }

        fn insert(&mut self, key: Key, value: Value) {
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.key == key {
                    node.value = value;
                    return;
                }
                current = &mut node.next;
            }
            let new_node = Box::new(Node {
                key,
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

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

    // 9. impls (outside verus! — these reference HashTable which contains dyn Fn types)

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
            new_size: usize,
        ) -> HashTable<Key, Value, ChainList<Key, Value>, Metrics> {
            let mut pairs = Vec::new();
            for chain in &table.table {
                let mut current = &chain.head;
                while let Some(node) = current {
                    pairs.push((node.key.clone(), node.value.clone()));
                    current = &node.next;
                }
            }

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
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — delegates to stored hash function.
        fn hash_index(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics>, key: &Key) -> usize {
            (table.hash_fn)(key) % table.current_size
        }
    }

    // 13. derive impls outside verus!

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for Node<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("value", &self.value)
                .field("next", &self.next)
                .finish()
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for ChainList<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ChainList")
                .field("head", &self.head)
                .finish()
        }
    }
}
