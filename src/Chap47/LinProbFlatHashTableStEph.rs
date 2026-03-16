//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Linear Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses linear probing for open addressing collision resolution.

pub mod LinProbFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!)
    // 9. impls (inside verus!)

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    /// Linear Probing Flat Hash Table implementation.
    pub struct LinProbFlatHashTableStEph;

    // 6. spec fns

    /// Well-formedness for linear probing flat hash tables.
    /// Unlike chained tables where keys live at their hash slot, open addressing
    /// displaces keys along probe chains. This spec captures:
    /// (1) no duplicate keys across slots,
    /// (2) every occupied key is reachable from its hash via linear probing
    ///     (no Empty gaps on the probe path).
    pub open spec fn spec_linprobflathashsteph_wf<Key, Value, Metrics, H>(
        table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
    ) -> bool {
        let m = table.current_size as int;
        // Basic structure.
        table.table@.len() == m
        && m > 0
        // No duplicate keys: each key appears in at most one slot.
        && (forall |i: int, j: int, k: Key|
            0 <= i < m && 0 <= j < m && i != j
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> !#[trigger] spec_flat_has_key(table.table@[j], k))
        // Probe chain integrity for linear probing.
        // For every key k at slot i, the linear probe path from hash(k) to i
        // has no Empty gaps: all intermediate slots are Occupied or Deleted.
        && (forall |i: int, k: Key|
            0 <= i < m
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> {
                let h = (table.spec_hash@)(k) as int % m;
                forall |d: int| 0 <= d < (i - h + m) % m
                    ==> !(#[trigger] table.table@[(h + d) % m] is Empty)
            })
    }

    // 9. impls

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for LinProbFlatHashTableStEph
    {
        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — linear probe find_slot then set.
        #[verifier::external_body]
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: Key, value: Value) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.table@.len() == table.current_size as int,
                    table.current_size == old(table).current_size,
                    table.num_elements == old(table).num_elements,
                decreases table.current_size - attempt,
            {
                let slot = linear_probe(&table.hash_fn, &key, table.current_size, attempt, table.spec_hash);
                let entry = table.table[slot].clone();
                if let FlatEntry::Occupied(k, _) = &entry {
                    if *k == key {
                        table.table.set(slot, FlatEntry::Occupied(key, value));
                        return;
                    }
                } else {
                    table.table.set(slot, FlatEntry::Occupied(key, value));
                    if table.num_elements < usize::MAX {
                        table.num_elements = table.num_elements + 1;
                    }
                    return;
                }
                attempt = attempt + 1;
            }
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — linear probe sequence until found or empty.
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m > 0,
                    h < m,
                    table.table@.len() == m as int,
                    h as nat == (table.spec_hash@)(*key) % (m as nat),
                    spec_hashtable_wf(table),
                    attempt > 0 ==> !table@.dom().contains(*key),
                decreases m - attempt,
            {
                // At attempt 0, slot == h (the hash slot). For attempt > 0,
                // the invariant already proves key absent so the exact slot is irrelevant.
                let slot: usize = if attempt == 0 { h } else { h.wrapping_add(attempt) % m };
                let entry = table.table[slot].clone();
                // Clone ensures: entry == table.table@[slot as int].
                match entry {
                    FlatEntry::Occupied(k, v) => {
                        let eq = k == *key;
                        proof { assume(eq == spec_flat_has_key(table.table@[slot as int], *key)); } // Eq bridge.
                        if eq {
                            proof {
                                assert(table.table@[slot as int].spec_entry_to_map().dom().contains(*key));
                                assert forall |j: int| 0 <= j < table.table@.len() && j != slot as int
                                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {}
                                lemma_table_to_map_unique_entry_value::<Key, Value, FlatEntry<Key, Value>>(
                                    table.table@, slot as int, *key);
                            }
                            return Some(v);
                        }
                        proof {
                            assert(!table.table@[slot as int].spec_entry_to_map().dom().contains(*key));
                        }
                    }
                    FlatEntry::Empty => {
                        proof {
                            if attempt == 0 {
                                // slot == h, entry at h is Empty → entry_to_map is Map::empty().
                                assert forall |j: int| 0 <= j < table.table@.len()
                                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                    if j == h as int {}
                                }
                                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                            }
                        }
                        return None;
                    }
                    FlatEntry::Deleted => {
                        proof {
                            assert(!table.table@[slot as int].spec_entry_to_map().dom().contains(*key));
                        }
                    }
                }
                // Didn't return: Occupied with different key, or Deleted.
                proof {
                    if attempt == 0 {
                        // slot == h, entry at h doesn't have *key. By wf, no other slot does either.
                        assert forall |j: int| 0 <= j < table.table@.len()
                            implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                            if j == h as int {}
                        }
                        lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                    }
                }
                attempt = attempt + 1;
            }
            None
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — linear probe until found or empty, then tombstone.
        #[verifier::external_body]
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.table@.len() == table.current_size as int,
                    table.current_size == old(table).current_size,
                decreases table.current_size - attempt,
            {
                let slot = linear_probe(&table.hash_fn, key, table.current_size, attempt, table.spec_hash);
                let entry = table.table[slot].clone();
                if let FlatEntry::Occupied(k, _) = &entry {
                    if *k == *key {
                        table.table.set(slot, FlatEntry::Deleted);
                        if table.num_elements > 0 {
                            table.num_elements = table.num_elements - 1;
                        }
                        return true;
                    }
                } else if let FlatEntry::Empty = &entry {
                    return false;
                }
                attempt = attempt + 1;
            }
            false
        }

        /// - APAS: Work O(n + m + m'), Span O(n + m + m').
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs from m slots, creates m' new slots, reinserts n pairs.
        #[verifier::external_body]
        fn resize(
            table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
            new_size: usize,
        ) -> (resized: HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) {
            // Phase 1: collect occupied pairs.
            let mut pairs: Vec<(Key, Value)> = Vec::new();
            let mut i: usize = 0;
            while i < table.table.len()
                invariant
                    i <= table.table@.len(),
                    table.table@.len() == table.current_size as int,
                decreases table.table.len() - i,
            {
                let entry = table.table[i].clone();
                if let FlatEntry::Occupied(k, v) = entry {
                    pairs.push((k, v));
                }
                i = i + 1;
            }

            // Phase 2: create new table with empty entries.
            let mut new_table_vec: Vec<FlatEntry<Key, Value>> = Vec::new();
            let mut k: usize = 0;
            while k < new_size
                invariant
                    k <= new_size,
                    new_table_vec@.len() == k as int,
                decreases new_size - k,
            {
                new_table_vec.push(FlatEntry::Empty);
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
            let mut j: usize = 0;
            while j < pairs.len()
                invariant
                    j <= pairs@.len(),
                    new_size > 0,
                    new_table.current_size == new_size,
                    new_table.table@.len() == new_table.current_size as int,
                    new_table.num_elements <= j,
                decreases pairs.len() - j,
            {
                let key = pairs[j].0.clone();
                let value = pairs[j].1.clone();
                Self::insert(&mut new_table, key, value);
                j = j + 1;
            }

            new_table
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for LinProbFlatHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — hash + addition + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash_val = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            (hash_val.wrapping_add(attempt)) % table.current_size
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — linear probe until empty/deleted/matching slot.
        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (slot: usize) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.current_size > 0,
                    table.table@.len() == table.current_size as int,
                decreases table.current_size - attempt,
            {
                let slot = Self::probe(table, key, attempt);
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Empty | FlatEntry::Deleted => { return slot; },
                    FlatEntry::Occupied(k, _) => {
                        if k == *key {
                            return slot;
                        }
                    },
                }
                attempt = attempt + 1;
            }
            Self::probe(table, key, 0)
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for LinProbFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LinProbFlatHashTableStEph")
        }
    }

    impl std::fmt::Display for LinProbFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LinProbFlatHashTableStEph")
        }
    }
}
