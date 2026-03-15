//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! LinkedList Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses LinkedListStEphS (Chap18) for separate chaining collision resolution.

pub mod LinkedListChainedHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 7. proof fns (inside verus!)
    // 9. impls (inside verus!: EntryTrait for LinkedListStEphS, ParaHashTableStEphTrait, ChainedHashTable)

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap18::LinkedListStEph::LinkedListStEph::*;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

        // 4. type definitions

        /// LinkedList Chained Hash Table implementation.
        pub struct LinkedListChainedHashTableStEph;

        // 7. proof fns

        proof fn _linked_list_chained_hash_table_verified() {}

        // 9. impls

        impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for LinkedListStEphS<(Key, Value)> {
            open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
                spec_seq_pairs_to_map(self.seq@)
            }

            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: Work O(1), Span O(1) — empty LinkedListStEphS construction.
            fn new() -> (entry: Self) { LinkedListStEphS { seq: Vec::new() } }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan for duplicate key, n = chain length.
            fn insert(&mut self, key: Key, value: Value)
                ensures
                    self.seq@.len() >= 1,
                    old(self).seq@.len() <= self.seq@.len(),
                    self.seq@.len() <= old(self).seq@.len() + 1,
            {
                let mut i: usize = 0;
                while i < self.seq.len()
                    invariant
                        i <= self.seq@.len(),
                        self.seq@ == old(self).seq@,
                    decreases self.seq.len() - i,
                {
                    if self.seq[i].0 == key {
                        self.seq.remove(i);
                        self.seq.push((key, value));
                        return;
                    }
                    i += 1;
                }
                self.seq.push((key, value));
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan of chain, n = chain length.
            fn lookup(&self, key: &Key) -> (found: Option<Value>) {
                let mut i: usize = 0;
                while i < self.seq.len()
                    decreases self.seq.len() - i,
                {
                    if self.seq[i].0 == *key {
                        return Some(self.seq[i].1.clone());
                    }
                    i += 1;
                }
                None
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan + Vec::remove, n = chain length.
            fn delete(&mut self, key: &Key) -> (deleted: bool)
                ensures !deleted ==> self.seq@ == old(self).seq@,
            {
                let mut i: usize = 0;
                while i < self.seq.len()
                    invariant
                        i <= self.seq@.len(),
                        self.seq@ == old(self).seq@,
                    decreases self.seq.len() - i,
                {
                    if self.seq[i].0 == *key {
                        self.seq.remove(i);
                        return true;
                    }
                    i += 1;
                }
                false
            }

            /// Element-wise clone avoiding Verus tuple-Clone limitation.
            fn clone_entry(&self) -> (cloned: Self) {
                let mut new_seq: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < self.seq.len()
                    invariant
                        i <= self.seq@.len(),
                    decreases self.seq.len() - i,
                {
                    new_seq.push((self.seq[i].0.clone(), self.seq[i].1.clone()));
                    i += 1;
                }
                LinkedListStEphS { seq: new_seq }
            }
        }

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>
            for LinkedListChainedHashTableStEph
        {
            /// - APAS: Work O(n) worst, Span O(n).
            /// - Claude-Opus-4.6: Work O(n) worst, Span O(n) — hash, copy bucket entries, insert, set back.
            #[verifier::external_body]
            fn insert(table: &mut HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: Key, value: Value) {
                let index = call_hash_fn(&table.hash_fn, &key, table.current_size);
                let bucket_len = table.table[index].seq.len();
                let mut new_seq: Vec<(Key, Value)> = Vec::new();
                let mut existed = false;
                let mut i: usize = 0;
                while i < bucket_len
                    invariant
                        i <= bucket_len,
                        index < table.table.len(),
                        bucket_len == table.table@[index as int].seq@.len(),
                        table.table@.len() == table.current_size as int,
                        table.current_size == old(table).current_size,
                        table.num_elements == old(table).num_elements,
                    decreases bucket_len - i,
                {
                    if table.table[index].seq[i].0 == key {
                        existed = true;
                    } else {
                        new_seq.push((table.table[index].seq[i].0.clone(), table.table[index].seq[i].1.clone()));
                    }
                    i = i + 1;
                }
                new_seq.push((key, value));
                table.table.set(index, LinkedListStEphS { seq: new_seq });
                if !existed {
                    table.num_elements = table.num_elements + 1;
                }
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — hash, index bucket, scan chain.
            #[verifier::external_body]
            fn lookup(table: &HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size);
                EntryTrait::lookup(&table.table[index], key)
            }

            /// - APAS: Work O(n) worst, Span O(n).
            /// - Claude-Opus-4.6: Work O(n) worst, Span O(n) — hash, copy bucket entries, delete, set back.
            #[verifier::external_body]
            fn delete(table: &mut HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: &Key) -> (deleted: bool) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size);
                let bucket_len = table.table[index].seq.len();
                let mut new_seq: Vec<(Key, Value)> = Vec::new();
                let mut deleted = false;
                let mut i: usize = 0;
                while i < bucket_len
                    invariant
                        i <= bucket_len,
                        index < table.table.len(),
                        bucket_len == table.table@[index as int].seq@.len(),
                        table.table@.len() == table.current_size as int,
                        table.current_size == old(table).current_size,
                    decreases bucket_len - i,
                {
                    if table.table[index].seq[i].0 == *key && !deleted {
                        deleted = true;
                    } else {
                        new_seq.push((table.table[index].seq[i].0.clone(), table.table[index].seq[i].1.clone()));
                    }
                    i = i + 1;
                }
                table.table.set(index, LinkedListStEphS { seq: new_seq });
                if deleted && table.num_elements > 0 {
                    table.num_elements = table.num_elements - 1;
                }
                deleted
            }

            /// - APAS: Work O(n + m + m'), Span O(n + m + m').
            /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' lists, reinserts.
            #[verifier::external_body]
            fn resize(
                table: &HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>,
                new_size: usize,
            ) -> (resized: HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>) {
                // Phase 1: collect all pairs from all chains.
                let mut pairs: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < table.table.len()
                    invariant
                        i <= table.table@.len(),
                        table.table@.len() == table.current_size as int,
                    decreases table.table.len() - i,
                {
                    let ghost table_len = table.table@.len();
                    let chain_len = table.table[i].seq.len();
                    let mut j: usize = 0;
                    while j < chain_len
                        invariant
                            i < table_len,
                            table.table@.len() == table_len,
                            j <= chain_len,
                            chain_len == table.table@[i as int].seq@.len(),
                        decreases chain_len - j,
                    {
                        pairs.push((table.table[i].seq[j].0.clone(), table.table[i].seq[j].1.clone()));
                        j = j + 1;
                    }
                    i = i + 1;
                }

                // Phase 2: create new table.
                let mut new_table_vec: Vec<LinkedListStEphS<(Key, Value)>> = Vec::new();
                let mut k: usize = 0;
                while k < new_size
                    invariant
                        k <= new_size,
                        new_table_vec@.len() == k as int,
                    decreases new_size - k,
                {
                    new_table_vec.push(LinkedListStEphS { seq: Vec::new() });
                    k = k + 1;
                }
                let mut new_table = HashTable {
                    table: new_table_vec,
                    hash_fn: table.hash_fn.clone(),
                    initial_size: table.initial_size,
                    current_size: new_size,
                    num_elements: 0,
                    metrics: Metrics::default(),
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
            ChainedHashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>
            for LinkedListChainedHashTableStEph
        {
            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: Work O(1), Span O(1) — delegates to stored hash function.
            fn hash_index(table: &HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: &Key) -> (index: usize) {
                call_hash_fn(&table.hash_fn, key, table.current_size)
            }
        }
    }

    // 13. derive impls outside verus!

    impl std::fmt::Debug for LinkedListChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LinkedListChainedHashTableStEph")
        }
    }

    impl std::fmt::Display for LinkedListChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LinkedListChainedHashTableStEph")
        }
    }
}
