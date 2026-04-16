// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Linear Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses linear probing for open addressing collision resolution.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 9. impls
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod LinProbFlatHashTableStEph {

    //		Section 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Concurrency::diverge;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger, lemma_reveal_view_injective};

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4. type definitions


    /// Linear Probing Flat Hash Table implementation.
    pub struct LinProbFlatHashTableStEph;

    //		Section 6. spec fns


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


    //		Section 7. proof fns/broadcast groups


    //		Section 9. impls


    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for LinProbFlatHashTableStEph
    {
        open spec fn spec_parahashtablesteph_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            spec_linprobflathashsteph_wf(table)
            && spec_hash_fn_valid::<Key, H>(table.spec_hash@)
        }

        /// Flat tables require at least one Empty slot for insertion.
        open spec fn spec_has_insert_capacity(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            exists |j: int| #![trigger table.table@[j]]
                0 <= j < table.table@.len() && table.table@[j] is Empty
        }

        /// Flat tables require new_size > current_size for resize.
        open spec fn spec_resize_ok(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, new_size: usize) -> bool {
            new_size as int > table.current_size as int
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: Key, value: Value) {
            let h = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m == old(table).current_size,
                    m > 0,
                    h < m,
                    table.table@.len() == m as int,
                    h as nat == (table.spec_hash@)(key) % (m as nat),
                    spec_linprobflathashsteph_wf(table),
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    old(table).num_elements < usize::MAX,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d) % (m as int)], key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d) % (m as int)] is Empty),
                decreases m - attempt,
            {
                let slot: usize = if attempt < m - h { h + attempt } else { attempt - (m - h) };
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    let sum = h as int + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                }
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, _v) => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block (speed hint)
                        // Veracity: NEEDED assert
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, &key);
                        if eq {
                            // Overwrite existing key at this slot.
                            let ghost old_table_seq = table.table@;
                            // Veracity: NEEDED proof block
                            table.table.set(slot, FlatEntry::Occupied(key, value));
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_reveal_view_injective::<Key>();
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(spec_flat_has_key(old_table_seq[slot as int], key));
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |j: int| 0 <= j < old_table_seq.len() && j != slot as int
                                    implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(key) by {
                                    if spec_flat_has_key(old_table_seq[j], key) {}
                                }
                                let new_entry = FlatEntry::<Key, Value>::Occupied(key, value);
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(new_entry.spec_entry_to_map() =~=
                                    old_table_seq[slot as int].spec_entry_to_map().insert(key, value));
                                lemma_table_to_map_update_insert::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, new_entry, key, value);
                                // Wf: no-dup.
                                // Probe chain.
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                            ==> !(#[trigger] table.table@[(hk + d) % m as int] is Empty)
                                    }) by {
                                    if i == slot as int {
                                        if k == key {}
                                    }
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                        implies !(#[trigger] table.table@[(hk + d) % m as int] is Empty) by {
                                        let pos = (hk + d) % m as int;
                                        if pos == slot as int {
                                        } else {
                                        }
                                    }
                                }
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(spec_other_slots_preserved(
                                    old(table).table@, table.table@, slot as int));
                            }
                            // Veracity: NEEDED proof block
                            return;
                        }
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                    FlatEntry::Empty => {
                        // New key insertion at empty slot.
                        let ghost old_table_seq = table.table@;
                        table.table.set(slot, FlatEntry::Occupied(key, value));
                        // Veracity: NEEDED proof block
                        if table.num_elements < usize::MAX {
                            table.num_elements = table.num_elements + 1;
                        }
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |j: int| 0 <= j < old_table_seq.len()
                                implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(key) by {
                                if spec_flat_has_key(old_table_seq[j], key) {
                                    lemma_probe_mod_identity(h as int, j, m as int);
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(old_table_seq, key);
                            let new_entry = FlatEntry::<Key, Value>::Occupied(key, value);
                            lemma_table_to_map_update_insert::<Key, Value, FlatEntry<Key, Value>>(
                                old_table_seq, slot as int, new_entry, key, value);
                            // Wf: no-dup.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |i: int, j: int, k: Key|
                                0 <= i < m as int && 0 <= j < m as int && i != j
                                && #[trigger] spec_flat_has_key(table.table@[i], k)
                                implies !#[trigger] spec_flat_has_key(table.table@[j], k) by {
                                if i == slot as int {
                                    if k == key && j != slot as int {
                                        // Veracity: NEEDED assert
                                        // Veracity: NEEDED assert
                                        assert(!old_table_seq[j].spec_entry_to_map().dom().contains(key));
                                        if spec_flat_has_key(old_table_seq[j], key) {}
                                    }
                                } else {
                                    if j == slot as int {
                                        if k == key {
                                            // Veracity: NEEDED assert
                                            // Veracity: NEEDED assert
                                            assert(!old_table_seq[i].spec_entry_to_map().dom().contains(key));
                                            if spec_flat_has_key(old_table_seq[i], key) {}
                                        }
                                    } else {
                                    }
                                }
                            }
                            // Wf: probe chain.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |i: int, k: Key|
                                0 <= i < m as int
                                && #[trigger] spec_flat_has_key(table.table@[i], k)
                                implies ({
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                        ==> !(#[trigger] table.table@[(hk + d) % m as int] is Empty)
                                }) by {
                                let hk = (table.spec_hash@)(k) as int % m as int;
                                if i == slot as int {
                                    if k == key {
                                        // Veracity: NEEDED assert
                                        // Veracity: NEEDED assert
                                        assert forall |d: int| 0 <= d < (slot as int - h as int + m as int) % m as int
                                            implies !(#[trigger] table.table@[(h as int + d) % m as int] is Empty) by {
                                            let sum = h as int + attempt as int;
                                            if sum < m as int {
                                                vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                                                vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(attempt as int, m as int);
                                                vstd::arithmetic::div_mod::lemma_small_mod(attempt as nat, m as nat);
                                            } else {
                                                vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                                                vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                                                vstd::arithmetic::div_mod::lemma_small_mod(attempt as nat, m as nat);
                                            }
                                            let pos = (h as int + d) % m as int;
                                            if pos == slot as int {
                                                vstd::arithmetic::div_mod::lemma_small_mod(d as nat, m as nat);
                                                vstd::arithmetic::div_mod::lemma_small_mod(attempt as nat, m as nat);
                                            }
                                        }
                                    }
                                } else {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                        implies !(#[trigger] table.table@[(hk + d) % m as int] is Empty) by {
                                        let pos = (hk + d) % m as int;
                                        if pos == slot as int {
                                        } else {
                                        }
                                    }
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(spec_other_slots_preserved(
                                old(table).table@, table.table@, slot as int));
                        }
                        // Veracity: NEEDED proof block
                        return;
                    }
                    FlatEntry::Deleted => {
                        // Skip Deleted: continue probing to avoid duplicates.
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                }
                // Veracity: NEEDED proof block
                attempt = attempt + 1;
            }
            // Exhausted all m positions — unreachable given spec_has_insert_capacity.
            // Linear probing visits all m slots; loop invariant says none was Empty.
            // But the precondition guarantees an Empty slot exists. Contradiction.
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall |j: int| 0 <= j < m as int
                    implies !(#[trigger] table.table@[j] is Empty) by {
                    lemma_probe_mod_identity(h as int, j, m as int);
                }
            }
            diverge::<()>();
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected
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
                    spec_linprobflathashsteph_wf(table),
                    // Probe positions 0..attempt don't have the key.
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d) % (m as int)], *key),
                    // Probe positions 0..attempt are not Empty.
                    // Veracity: NEEDED proof block
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d) % (m as int)] is Empty),
                decreases m - attempt,
            {
                // Compute slot = (h + attempt) % m without overflow.
                let slot: usize = if attempt < m - h { h + attempt } else { attempt - (m - h) };
                // Veracity: NEEDED proof block
                proof {
                    let sum = h as int + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        // Veracity: NEEDED proof block
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                }
                // Veracity: NEEDED proof block
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, v) => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED assert
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, key);
                        if eq {
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_reveal_view_injective::<Key>();
                                // No other slot has this key: contradiction via wf no-dup multi-trigger.
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |j: int| 0 <= j < table.table@.len() && j != slot as int
                                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(table.table@[j], *key) {
                                        // Both triggers in E-graph: table[slot] and table[j].
                                        // Veracity: NEEDED assert
                                        // Veracity: NEEDED proof block
                                        // Veracity: NEEDED assert
                                        assert(spec_flat_has_key(table.table@[slot as int], *key));
                                    }
                                }
                                // Veracity: NEEDED proof block
                                lemma_table_to_map_unique_entry_value::<Key, Value, FlatEntry<Key, Value>>(
                                    table.table@, slot as int, *key);
                            }
                            return Some(v);
                        }
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                    FlatEntry::Empty => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                if spec_flat_has_key(table.table@[j], *key) {
                                    let dj = (j - h as int + m as int) % (m as int);
                                    lemma_probe_mod_identity(h as int, j, m as int);
                                    if dj > attempt as int {
                                        // wf: probe position attempt non-Empty, but slot is Empty.
                                    // Veracity: NEEDED proof block (speed hint)
                                    } else if dj < attempt as int {
                                        // Invariant: !spec_flat_has_key at (h+dj)%m == j.
                                    }
                                    // dj == attempt: j == slot which is Empty, can't have key.
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                        // Veracity: NEEDED proof block
                        }
                        return None;
                    }
                    FlatEntry::Deleted => {
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions without finding key.
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                        let dj = (j - h as int + m as int) % (m as int);
                        lemma_probe_mod_identity(h as int, j, m as int);
                        // dj < m == attempt: invariant gives !spec_flat_has_key at (h+dj)%m == j.
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            None
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m == old(table).current_size,
                    m > 0,
                    h < m,
                    table.table@.len() == m as int,
                    // Veracity: NEEDED proof block
                    h as nat == (table.spec_hash@)(*key) % (m as nat),
                    spec_linprobflathashsteph_wf(table),
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d) % (m as int)], *key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d) % (m as int)] is Empty),
                decreases m - attempt,
            {
                let slot: usize = if attempt < m - h { h + attempt } else { attempt - (m - h) };
                // Veracity: NEEDED proof block (speed hint)
                // Veracity: NEEDED proof block
                proof {
                    let sum = h as int + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    // Veracity: NEEDED proof block
                    }
                }
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, _v) => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED assert
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, key);
                        if eq {
                            let ghost old_table_seq = table.table@;
                            table.table.set(slot, FlatEntry::Deleted);
                            if table.num_elements > 0 {
                                table.num_elements = table.num_elements - 1;
                            }
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_reveal_view_injective::<Key>();
                                // No other slot has *key (old wf no-dup).
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |j: int| 0 <= j < old_table_seq.len() && j != slot as int
                                    implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(old_table_seq[j], *key) {
                                        // Veracity: NEEDED assert
                                        // Veracity: NEEDED assert
                                        assert(spec_flat_has_key(old_table_seq[slot as int], *key));
                                    }
                                }
                                // Map update: Deleted has empty map = old entry map with key removed.
                                let new_entry = FlatEntry::<Key, Value>::Deleted;
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(new_entry.spec_entry_to_map() =~=
                                    old_table_seq[slot as int].spec_entry_to_map().remove(*key));
                                lemma_table_to_map_update_remove::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, new_entry, *key);
                                // Old table contained *key.
                                lemma_table_to_map_unique_entry_value::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, *key);
                                // Wf preservation: no-dup.
                                // Wf preservation: probe chain integrity.
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                            ==> !(#[trigger] table.table@[(hk + d) % m as int] is Empty)
                                    // Veracity: NEEDED proof block
                                    }) by {
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED proof block
                                    assert forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                        implies !(#[trigger] table.table@[(hk + d) % m as int] is Empty) by {
                                        let pos = (hk + d) % m as int;
                                        if pos == slot as int {
                                            // Deleted is not Empty.
                                        } else {
                                        }
                                    }
                                }
                            }
                            return true;
                        }
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    // Veracity: NEEDED proof block
                    }
                    FlatEntry::Empty => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED proof block
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                if spec_flat_has_key(table.table@[j], *key) {
                                    let dj = (j - h as int + m as int) % (m as int);
                                    lemma_probe_mod_identity(h as int, j, m as int);
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                        }
                        return false;
                    }
                    FlatEntry::Deleted => {
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                }
                attempt = attempt + 1;
            }
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                        let dj = (j - h as int + m as int) % (m as int);
                        lemma_probe_mod_identity(h as int, j, m as int);
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            false
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m').
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m'), Span O(n + m + m') — collects n pairs from m slots, creates m' new slots, reinserts n pairs.
        fn resize(
            table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
            // Veracity: NEEDED proof block
            new_size: usize,
        ) -> (resized: HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) {
            // Phase 1: collect occupied pairs.
            let mut pairs: Vec<(Key, Value)> = Vec::new();
            let mut i: usize = 0;
            while i < table.table.len()
                invariant
                    i <= table.table@.len(),
                    table.table@.len() == table.current_size as int,
                    // Veracity: NEEDED proof block
                    pairs@.len() <= i as int,
                    spec_seq_pairs_to_map(pairs@) =~=
                        spec_table_to_map::<Key, Value, FlatEntry<Key, Value>>(
                            // Veracity: NEEDED proof block
                            table.table@.subrange(0, i as int)),
                decreases table.table.len() - i,
            {
                let ghost old_pairs = pairs@;
                let ghost old_map = spec_seq_pairs_to_map(old_pairs);
                let entry = table.table[i].clone();
                if let FlatEntry::Occupied(k, v) = entry {
                    pairs.push((k, v));
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                        // Help Z3 unfold spec_seq_pairs_to_map after push.
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(pairs@.drop_last() =~= old_pairs);
                        // Connect to spec_table_to_map via union_prefer_right.
                        let ghost entry_map = table.table@[i as int].spec_entry_to_map();
                    }
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                    }
                }
                // Veracity: NEEDED proof block
                proof {
                    let ghost sub_next = table.table@.subrange(0, (i + 1) as int);
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(sub_next.drop_last() =~= table.table@.subrange(0, i as int));
                }
                i = i + 1;
            // Veracity: NEEDED proof block
            }
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(table.table@.subrange(0, table.table@.len() as int)
                    =~= table.table@);
            }

            // Phase 2: create new table with empty entries.
            let mut new_table_vec: Vec<FlatEntry<Key, Value>> = Vec::new();
            let mut k: usize = 0;
            while k < new_size
                invariant
                    k <= new_size,
                    new_table_vec@.len() == k as int,
                    // Veracity: NEEDED proof block
                    forall |j: int| 0 <= j < new_table_vec@.len()
                        ==> (#[trigger] new_table_vec@[j]) is Empty,
                    spec_table_to_map::<Key, Value, FlatEntry<Key, Value>>(new_table_vec@)
                        == Map::<Key, Value>::empty(),
                decreases new_size - k,
            {
                let ghost old_vec = new_table_vec@;
                // Veracity: NEEDED proof block
                new_table_vec.push(FlatEntry::Empty);
                // Veracity: NEEDED proof block
                proof {
                    lemma_table_to_map_push_empty::<Key, Value, FlatEntry<Key, Value>>(
                        old_vec, FlatEntry::Empty);
                }
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
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(spec_linprobflathashsteph_wf(&new_table));
            // Veracity: NEEDED proof block
            }

            // Phase 3: reinsert all pairs.
            let mut j: usize = 0;
            // Veracity: NEEDED proof block
            proof {
                lemma_all_empties_count::<Key, Value>(new_table.table@);
            // Veracity: NEEDED proof block
            }
            while j < pairs.len()
                invariant
                    j <= pairs@.len(),
                    new_size > 0,
                    new_table.current_size == new_size,
                    new_table.table@.len() == new_table.current_size as int,
                    new_table.num_elements <= j,
                    Self::spec_parahashtablesteph_wf(&new_table),
                    new_table@ =~= spec_seq_pairs_to_map(pairs@.subrange(0, j as int)),
                    new_table.spec_hash == table.spec_hash,
                    pairs@.len() <= table.current_size as int,
                    new_size as int > table.current_size as int,
                    // Veracity: NEEDED proof block
                    spec_count_empties(new_table.table@) >= (new_size - j) as int,
                    obeys_feq_clone::<Key>(),
                    obeys_feq_clone::<Value>(),
                decreases pairs.len() - j,
            {
                let key = clone_elem(&pairs[j].0);
                let value = clone_elem(&pairs[j].1);
                // Veracity: NEEDED proof block
                proof {
                    // Prove spec_has_insert_capacity: empties > 0 implies exists Empty slot.
                    lemma_empties_positive_implies_exists_empty::<Key, Value>(
                        new_table.table@);
                }
                let ghost old_new_table_seq = new_table.table@;
                Self::insert(&mut new_table, key, value);
                // Veracity: NEEDED proof block
                proof {
                    // Use exists |s| from insert ensures to maintain empties invariant.
                    let s = choose |s: int| #[trigger] spec_other_slots_preserved(
                        old_new_table_seq, new_table.table@, s);
                    lemma_one_slot_change_empties::<Key, Value>(
                        old_new_table_seq, new_table.table@, s);
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(pairs@.subrange(0, (j + 1) as int).drop_last()
                        =~= pairs@.subrange(0, j as int));
                }
                j = j + 1;
            }
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(pairs@.subrange(0, pairs@.len() as int) =~= pairs@);
            }

            new_table
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for LinProbFlatHashTableStEph
    {
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — hash + addition + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash_val = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            (hash_val.wrapping_add(attempt)) % table.current_size
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected
        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (slot: usize) {
            let mut attempt: usize = 0;
            while attempt < table.current_size
                invariant
                    attempt <= table.current_size,
                    table.current_size > 0,
                    table.table@.len() == table.current_size as int,
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
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

    //		Section 14. derive impls outside verus!


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
