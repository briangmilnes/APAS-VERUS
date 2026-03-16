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
        /// Returns (new_chain, existed) where existed is true if key was already present.
        fn chain_insert<Key: PartialEq, Value>(
            chain: Option<Box<Node<Key, Value>>>,
            key: Key,
            value: Value,
        ) -> (result: (Option<Box<Node<Key, Value>>>, bool))
            ensures
                result.0 is Some,
                spec_chain_to_map(result.0) == spec_chain_to_map(chain).insert(key, value),
                result.1 == spec_chain_to_map(chain).dom().contains(key),
            decreases chain,
        {
            match chain {
                None => {
                    let out = Some(Box::new(Node { key, value, next: None }));
                    proof { reveal_with_fuel(spec_chain_to_map, 2); }
                    (out, false)
                }
                Some(node) => {
                    let Node { key: nk, value: nv, next: nn } = *node;
                    let eq = nk == key;
                    proof { assume(eq == (nk == key)); } // Eq bridge.
                    if eq {
                        let out = Some(Box::new(Node { key, value, next: nn }));
                        proof { reveal_with_fuel(spec_chain_to_map, 2); }
                        (out, true)
                    } else {
                        let (updated, existed) = chain_insert(nn, key, value);
                        let out = Some(Box::new(Node { key: nk, value: nv, next: updated }));
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            assert(spec_chain_to_map(nn).insert(key, value).insert(nk, nv)
                                =~= spec_chain_to_map(nn).insert(nk, nv).insert(key, value));
                        }
                        (out, existed)
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
                spec_chain_to_map(*chain).dom().contains(*key)
                    ==> found == Some(spec_chain_to_map(*chain)[*key]),
                !spec_chain_to_map(*chain).dom().contains(*key)
                    ==> found is None,
            decreases chain,
        {
            match chain {
                None => {
                    proof { reveal_with_fuel(spec_chain_to_map, 1); }
                    None
                }
                Some(node) => {
                    let eq = node.key == *key;
                    proof { assume(eq == (node.key == *key)); } // Eq bridge.
                    if eq {
                        let v = node.value.clone();
                        proof {
                            assume(v == node.value); // Clone bridge.
                            reveal_with_fuel(spec_chain_to_map, 2);
                        }
                        Some(v)
                    } else {
                        let result = chain_lookup(&node.next, key);
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            // spec_chain_to_map(*chain) = spec_chain_to_map(node.next).insert(node.key, node.value)
                            // node.key != *key, so value at *key comes from node.next's map.
                        }
                        result
                    }
                }
            }
        }

        /// Removes all nodes matching key, returns updated chain and whether any found.
        fn chain_delete<Key: PartialEq, Value>(
            chain: Option<Box<Node<Key, Value>>>,
            key: &Key,
        ) -> (remaining_and_deleted: (Option<Box<Node<Key, Value>>>, bool))
            ensures
                spec_chain_to_map(remaining_and_deleted.0)
                    == spec_chain_to_map(chain).remove(*key),
                remaining_and_deleted.1
                    == spec_chain_to_map(chain).dom().contains(*key),
            decreases chain,
        {
            match chain {
                None => {
                    proof { reveal_with_fuel(spec_chain_to_map, 1); }
                    (None, false)
                }
                Some(node) => {
                    let Node { key: nk, value: nv, next: nn } = *node;
                    let eq = nk == *key;
                    proof { assume(eq == (nk == *key)); } // Eq bridge.
                    let (new_next, tail_deleted) = chain_delete(nn, key);
                    if eq {
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            assert(spec_chain_to_map(nn).insert(*key, nv).remove(*key)
                                =~= spec_chain_to_map(nn).remove(*key));
                        }
                        (new_next, true)
                    } else {
                        let out = Some(Box::new(Node { key: nk, value: nv, next: new_next }));
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            assert(spec_chain_to_map(nn).insert(nk, nv).remove(*key)
                                =~= spec_chain_to_map(nn).remove(*key).insert(nk, nv));
                        }
                        (out, tail_deleted)
                    }
                }
            }
        }

        impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for ChainList<Key, Value> {
            open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
                spec_chain_to_map(self.head)
            }

            fn new() -> (entry: Self) { ChainList { head: None } }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — recursive scan for duplicate key, n = chain length.
            fn insert(&mut self, key: Key, value: Value)
                ensures spec_chain_to_map(self.head).dom().contains(key),
            {
                let (new_head, _existed) = chain_insert(self.head.take(), key, value);
                self.head = new_head;
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — recursive scan of chain, n = chain length.
            fn lookup(&self, key: &Key) -> (found: Option<Value>)
            {
                chain_lookup(&self.head, key)
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — recursive scan + rebuild, n = chain length.
            fn delete(&mut self, key: &Key) -> (deleted: bool)
                ensures
                    !deleted ==> spec_chain_to_map(self.head) == spec_chain_to_map(old(self).head),
            {
                let (new_head, found) = chain_delete(self.head.take(), key);
                self.head = new_head;
                found
            }

            /// Delegates to ChainList's Clone impl.
            fn clone_entry(&self) -> (cloned: Self) {
                self.clone()
            }
        }

        // 9. impls (ParaHashTableStEphTrait, ChainedHashTable)

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, ChainList<Key, Value>, Metrics, H>
            for StructChainedHashTableStEph
        {
            /// - APAS: Work O(n) worst, Span O(n).
            /// - Claude-Opus-4.6: Work O(n) worst, Span O(n) — hash, clone chain, insert into clone, set back.
            fn insert(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: Key, value: Value) {
                let index = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let mut entry = table.table[index].clone();
                let ghost old_entry_map = entry.spec_entry_to_map();

                let chain = entry.head.take();
                let (new_head, existed) = chain_insert(chain, key, value);
                entry.head = new_head;

                table.table.set(index, entry);

                proof {
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().insert(key, value));

                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(key) by {}

                    lemma_table_to_map_update_insert::<Key, Value, ChainList<Key, Value>>(
                        old_table, index as int, table.table@[index as int], key, value);

                    assert(table.table@.len() == table.current_size as int);
                    assert(table.current_size > 0);
                    assert forall |j: int, k: Key| 0 <= j < table.table@.len()
                        && j != (table.spec_hash@)(k) as int % table.current_size as int
                        implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k) by {
                        if j != index as int {
                            assert(table.table@[j] == old_table[j]);
                        }
                    }
                }

                if !existed {
                    table.num_elements = table.num_elements + 1;
                }
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — hash, index bucket, scan chain.
            fn lookup(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let result = chain_lookup(&table.table[index].head, key);
                proof {
                    // chain_lookup ensures correctness against spec_chain_to_map(head).
                    // spec_entry_to_map for ChainList == spec_chain_to_map(self.head).
                    if spec_chain_to_map(table.table@[index as int].head).dom().contains(*key) {
                        // Key in this bucket. By wf, not in any other bucket.
                        assert forall |j: int| 0 <= j < table.table@.len() && j != index as int
                            implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {}
                        lemma_table_to_map_unique_entry_value::<Key, Value, ChainList<Key, Value>>(
                            table.table@, index as int, *key);
                    } else {
                        // Key not in this bucket. By wf, not in any other bucket either.
                        assert forall |j: int| 0 <= j < table.table@.len()
                            implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                            if j == index as int {
                            } else {
                            }
                        }
                        lemma_table_to_map_not_contains::<Key, Value, ChainList<Key, Value>>(
                            table.table@, *key);
                    }
                }
                result
            }

            /// - APAS: Work O(n) worst, Span O(n).
            /// - Claude-Opus-4.6: Work O(n) worst, Span O(n) — hash, clone chain, delete from clone, set back.
            fn delete(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let mut entry = table.table[index].clone();
                let ghost old_entry_map = entry.spec_entry_to_map();

                let chain = entry.head.take();
                let (new_head, found) = chain_delete(chain, key);
                entry.head = new_head;

                table.table.set(index, entry);

                proof {
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().remove(*key));

                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(*key) by {}

                    lemma_table_to_map_update_remove::<Key, Value, ChainList<Key, Value>>(
                        old_table, index as int, table.table@[index as int], *key);

                    assert(table.table@.len() == table.current_size as int);
                    assert(table.current_size > 0);
                    assert forall |j: int, k: Key| 0 <= j < table.table@.len()
                        && j != (table.spec_hash@)(k) as int % table.current_size as int
                        implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k) by {
                        if j == index as int {
                            assert(!old_table[j].spec_entry_to_map().dom().contains(k));
                        } else {
                            assert(table.table@[j] == old_table[j]);
                        }
                    }

                    // Prove found == old(table)@.dom().contains(*key).
                    if found {
                        lemma_table_to_map_update_contains::<Key, Value, ChainList<Key, Value>>(
                            old_table, index as int, old_table[index as int], *key);
                        assert(old_table.update(index as int, old_table[index as int]) =~= old_table);
                    } else {
                        lemma_table_to_map_not_contains::<Key, Value, ChainList<Key, Value>>(old_table, *key);
                    }
                }

                if found && table.num_elements > 0 {
                    table.num_elements = table.num_elements - 1;
                }
                found
            }

            /// - APAS: Work O(n + m + m'), Span O(n + m + m').
            /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — traverses all chains, creates m' lists, reinserts.
            #[verifier::external_body]
            fn resize(
                table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>,
                new_size: usize,
            ) -> (resized: HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>) {
                // Phase 1: collect all pairs from all chains.
                let mut pairs: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < table.table.len()
                    invariant
                        i <= table.table@.len(),
                        table.table@.len() == table.current_size as int,
                    decreases table.table.len() - i,
                {
                    // Traverse the linked list for this bucket using to_seq on the chain.
                    let chain_clone = table.table[i].clone();
                    let mut current = chain_clone.head;
                    while current.is_some()
                        decreases current,
                    {
                        let node = current.unwrap();
                        pairs.push((node.key.clone(), node.value.clone()));
                        current = node.next;
                    }
                    i = i + 1;
                }

                // Phase 2: create new table.
                let mut new_table_vec: Vec<ChainList<Key, Value>> = Vec::new();
                let mut k: usize = 0;
                while k < new_size
                    invariant
                        k <= new_size,
                        new_table_vec@.len() == k as int,
                    decreases new_size - k,
                {
                    new_table_vec.push(ChainList { head: None });
                    k = k + 1;
                }
                let mut new_table = HashTable {
                    table: new_table_vec,
                    hash_fn: table.hash_fn.clone(),
                    initial_size: table.initial_size,
                    current_size: new_size,
                    num_elements: 0,
                    metrics: Metrics::default(),
                    spec_hash: table.spec_hash,
                    _phantom: PhantomData,
                };

                // Phase 3: reinsert all pairs.
                let mut m: usize = 0;
                while m < pairs.len()
                    invariant
                        m <= pairs@.len(),
                        new_size > 0,
                        new_table.current_size == new_size,
                        new_table.table@.len() == new_table.current_size as int,
                        new_table.num_elements <= m,
                    decreases pairs.len() - m,
                {
                    let key = pairs[m].0.clone();
                    let value = pairs[m].1.clone();
                    Self::insert(&mut new_table, key, value);
                    m = m + 1;
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
            fn hash_index(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> (index: usize) {
                call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash)
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
