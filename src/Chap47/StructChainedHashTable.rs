//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Struct Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses custom linked list struct for separate chaining collision resolution.

pub mod StructChainedHashTable {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!: spec_chain_to_map)
    // 7. proof fns (inside verus!)
    // 8. traits (inside verus!: ChainListTrait)
    // 9. impls (inside verus!: ChainListTrait for ChainList, PartialEqSpecImpl, Default, chain helpers, EntryTrait for ChainList, ParaHashTableStEphTrait, ChainedHashTable)
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::vstdplus::accept::accept;
    use crate::Types::Types::*;

    verus! {

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

        /// Struct Chained Hash Table implementation.
        pub struct StructChainedHashTableStEph;

        // 6. spec fns

        /// Maps a chain (linked list of nodes) to its abstract Map representation.
        /// Head node's entry is inserted last, so it wins on duplicate keys.
        pub open spec fn spec_chain_to_map<Key, Value>(
            chain: Option<Box<Node<Key, Value>>>,
        ) -> Map<Key, Value>
            decreases chain,
        {
            match chain {
                None => Map::empty(),
                Some(node) => spec_chain_to_map(node.next).insert(node.key, node.value),
            }
        }

        // 7. proof fns

        proof fn _struct_chained_hash_table_verified() {}

        // 8. traits

        /// Spec trait for ChainList abstract state.
        pub trait ChainListTrait<Key, Value>: Sized {
            spec fn spec_to_map(&self) -> Map<Key, Value>;
        }

        // 9. impls

        impl<Key, Value> ChainListTrait<Key, Value> for ChainList<Key, Value> {
            open spec fn spec_to_map(&self) -> Map<Key, Value> {
                spec_chain_to_map(self.head)
            }
        }

        #[cfg(verus_keep_ghost)]
        impl<Key: PartialEq, Value: PartialEq> PartialEqSpecImpl for Node<Key, Value> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
        }

        #[cfg(verus_keep_ghost)]
        impl<Key: PartialEq, Value: PartialEq> PartialEqSpecImpl for ChainList<Key, Value> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
        }

        impl<Key, Value> Default for ChainList<Key, Value> {
            fn default() -> Self { ChainList { head: None } }
        }

        /// Inserts key-value into chain, updating if key exists, appending if not.
        fn chain_insert<Key: PartialEq, Value>(
            chain: Option<Box<Node<Key, Value>>>,
            key: Key,
            value: Value,
        ) -> (out: Option<Box<Node<Key, Value>>>)
            ensures
                out is Some,
                spec_chain_to_map(out).dom().contains(key),
            decreases chain,
        {
            match chain {
                None => {
                    let out = Some(Box::new(Node { key, value, next: None }));
                    proof { reveal_with_fuel(spec_chain_to_map, 2); }
                    out
                }
                Some(node) => {
                    let Node { key: nk, value: nv, next: nn } = *node;
                    if nk == key {
                        let out = Some(Box::new(Node { key, value, next: nn }));
                        proof { reveal_with_fuel(spec_chain_to_map, 2); }
                        out
                    } else {
                        let updated = chain_insert(nn, key, value);
                        let out = Some(Box::new(Node { key: nk, value: nv, next: updated }));
                        proof { reveal_with_fuel(spec_chain_to_map, 2); }
                        out
                    }
                }
            }
        }

        /// Looks up key in chain, returning value if found.
        fn chain_lookup<Key: PartialEq, Value: Clone>(
            chain: &Option<Box<Node<Key, Value>>>,
            key: &Key,
        ) -> (found: Option<Value>)
            ensures
                chain is None ==> found is None,
            decreases chain,
        {
            match chain {
                None => None,
                Some(node) => {
                    if node.key == *key {
                        Some(node.value.clone())
                    } else {
                        chain_lookup(&node.next, key)
                    }
                }
            }
        }

        /// Removes first node matching key, returns updated chain and whether found.
        fn chain_delete<Key: PartialEq, Value>(
            chain: Option<Box<Node<Key, Value>>>,
            key: &Key,
        ) -> (remaining_and_deleted: (Option<Box<Node<Key, Value>>>, B))
            ensures
                !remaining_and_deleted.1
                    ==> spec_chain_to_map(remaining_and_deleted.0) == spec_chain_to_map(chain),
            decreases chain,
        {
            match chain {
                None => {
                    proof { reveal_with_fuel(spec_chain_to_map, 1); }
                    (None, false)
                }
                Some(node) => {
                    let Node { key: nk, value: nv, next: nn } = *node;
                    if nk == *key {
                        (nn, true)
                    } else {
                        let (new_next, deleted) = chain_delete(nn, key);
                        let out = (Some(Box::new(Node { key: nk, value: nv, next: new_next })), deleted);
                        proof { reveal_with_fuel(spec_chain_to_map, 2); }
                        out
                    }
                }
            }
        }

        impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for ChainList<Key, Value> {
            fn new() -> (entry: Self) { ChainList { head: None } }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — recursive scan for duplicate key, n = chain length.
            fn insert(&mut self, key: Key, value: Value)
                ensures spec_chain_to_map(self.head).dom().contains(key),
            {
                self.head = chain_insert(self.head.take(), key, value);
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — recursive scan of chain, n = chain length.
            fn lookup(&self, key: &Key) -> (found: Option<Value>)
            {
                chain_lookup(&self.head, key)
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — recursive scan + rebuild, n = chain length.
            fn delete(&mut self, key: &Key) -> (deleted: B)
                ensures
                    !deleted ==> spec_chain_to_map(self.head) == spec_chain_to_map(old(self).head),
            {
                let (new_head, found) = chain_delete(self.head.take(), key);
                self.head = new_head;
                found
            }
        }

        // 9. impls (ParaHashTableStEphTrait, ChainedHashTable)

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, ChainList<Key, Value>, Metrics, H>
            for StructChainedHashTableStEph
        {
            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to insert_chained.
            #[verifier::external_body]
            fn insert(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: Key, value: Value) {
                Self::insert_chained(table, key, value);
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to lookup_chained.
            #[verifier::external_body]
            fn lookup(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> Option<Value> {
                Self::lookup_chained(table, key)
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — delegates to delete_chained.
            #[verifier::external_body]
            fn delete(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> B {
                Self::delete_chained(table, key)
            }

            /// - APAS: Work O(n + m + m'), Span O(n + m + m').
            /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — traverses all chains, creates m' lists, reinserts.
            #[verifier::external_body]
            fn resize(
                table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>,
                new_size: usize,
            ) -> HashTable<Key, Value, ChainList<Key, Value>, Metrics, H> {
                let mut pairs = Vec::new();
                for chain in &table.table {
                    let mut current = &chain.head;
                    while let Some(node) = current {
                        pairs.push((node.key.clone(), node.value.clone()));
                        current = &node.next;
                    }
                }

                let new_table_vec = (0..new_size).map(|_| ChainList::new()).collect();
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
            ChainedHashTable<Key, Value, ChainList<Key, Value>, Metrics, H>
            for StructChainedHashTableStEph
        {
            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: Work O(1), Span O(1) — delegates to stored hash function.
            #[verifier::external_body]
            fn hash_index(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> usize {
                (table.hash_fn)(key, table.current_size)
            }
        }

        // 11. derive impls in verus!

        impl<Key: Clone, Value: Clone> Clone for Node<Key, Value> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned == *self
                decreases self
            {
                let cloned = Node {
                    key: self.key.clone(),
                    value: self.value.clone(),
                    next: match &self.next {
                        None => None,
                        Some(b) => Some(Box::new((**b).clone())),
                    },
                };
                proof { accept(cloned == *self); }
                cloned
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::PartialEq for Node<Key, Value> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (*self == *other)
                decreases self, other
            {
                let equal = self.key == other.key
                    && self.value == other.value
                    && match (&self.next, &other.next) {
                        (None, None) => true,
                        (Some(a), Some(b)) => (**a) == (**b),
                        _ => false,
                    };
                proof { accept(equal == (*self == *other)); }
                equal
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::Eq for Node<Key, Value> {}

        impl<Key: Clone, Value: Clone> Clone for ChainList<Key, Value> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned == *self
            {
                let cloned = ChainList {
                    head: match &self.head {
                        None => None,
                        Some(b) => Some(Box::new((**b).clone())),
                    },
                };
                proof { accept(cloned == *self); }
                cloned
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::PartialEq for ChainList<Key, Value> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (*self == *other)
            {
                let equal = match (&self.head, &other.head) {
                    (None, None) => true,
                    (Some(a), Some(b)) => (**a) == (**b),
                    _ => false,
                };
                proof { accept(equal == (*self == *other)); }
                equal
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::Eq for ChainList<Key, Value> {}
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

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for Node<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for ChainList<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ChainList")
                .field("head", &self.head)
                .finish()
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for ChainList<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::fmt::Debug for StructChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StructChainedHashTableStEph")
        }
    }

    impl std::fmt::Display for StructChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StructChainedHashTableStEph")
        }
    }
}
