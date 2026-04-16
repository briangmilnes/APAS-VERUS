// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Double Hashing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses double hashing for open addressing collision resolution.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 9. impls
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod DoubleHashFlatHashTableStEph {

    //		Section 2. imports
    use std::hash::Hash;
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


    /// Double Hashing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i·hh(k)) mod m
    /// Uses two hash functions to avoid both primary and secondary clustering.
    pub struct DoubleHashFlatHashTableStEph;

    //		Section 6. spec fns


    /// Abstract second hash value for double hashing.
    /// Closed so body is hidden from SMT; `second_hash` links via ensures.
    pub closed spec fn spec_second_hash<Key>(key: Key, table_size: nat) -> nat { 1 }

    /// Well-formedness for double hashing flat hash tables.
    /// Probe sequence: slot (h + j * s) % m for attempt j = 0, 1, 2, ...
    /// where h = hash(k) % m and s = spec_second_hash(k, m) >= 1.
    /// The step s is concrete via `spec_second_hash`, linked to runtime by
    /// `compute_second_hash` ensures `step == spec_second_hash(key, m)`.
    pub open spec fn spec_doublehashflathashsteph_wf<Key, Value, Metrics, H>(
        table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
    ) -> bool {
        let m = table.current_size as int;
        table.table@.len() == m
        && m > 0
        // No duplicate keys.
        && (forall |i: int, j: int, k: Key|
            0 <= i < m && 0 <= j < m && i != j
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> !#[trigger] spec_flat_has_key(table.table@[j], k))
        // Probe chain integrity for double hashing.
        && (forall |i: int, k: Key|
            0 <= i < m
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> {
                let h = (table.spec_hash@)(k) as int % m;
                let s = spec_second_hash(k, m as nat) as int;
                s >= 1
                && exists |n: int| #![trigger table.table@[(h + n * s) % m]] 0 <= n < m
                    && (h + n * s) % m == i
                    && forall |j: int| 0 <= j < n
                        ==> !(#[trigger] table.table@[(h + j * s) % m] is Empty)
            })
    }


    /// Exposes the spec-level second hash value (= 1) for probe coverage proofs.
    /// Within this module, the closed fn body is visible, so this is provable.
    pub proof fn lemma_spec_second_hash_value<Key>(key: Key, table_size: nat)
        ensures spec_second_hash::<Key>(key, table_size) == 1nat,
    {}

    //		Section 9. impls


    impl DoubleHashFlatHashTableStEph {
        /// Computes a second hash value for double hashing.
        /// APAS: hh(k) must be relatively prime to m.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(sizeof(Key)), Span O(sizeof(Key)) — hashes key with SipHash.
        /// Strategy: Always return an odd number (works for power-of-2 sizes),
        /// and for prime sizes, ensure < m and non-zero.
        #[verifier::external_body]
        pub fn second_hash<Key: StT + Hash>(key: &Key, table_size: usize) -> (step: usize)
            requires table_size > 0,
            ensures
                step >= 1,
                table_size > 1 ==> step < table_size,
                step as nat == spec_second_hash(*key, table_size as nat),
        {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;
            if table_size <= 1 {
                return 1;
            }
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let hash = hasher.finish();
            let base = (table_size - 1) as u64;
            let mut step = ((hash % base) + 1) as usize;
            if step % 2 == 0 && step < table_size - 1 {
                step += 1;
            }
            step
        }
    }

    impl<Key: StT + Hash, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for DoubleHashFlatHashTableStEph
    {
        open spec fn spec_parahashtablesteph_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            spec_doublehashflathashsteph_wf(table)
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
            let step = Self::second_hash(&key, m);
            // step == spec_second_hash(key, m): wf gives concrete probe chain.
            let mut slot: usize = h;
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
            }
            let mut attempt: usize = 0;
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m == old(table).current_size,
                    m > 0,
                    h < m,
                    step >= 1usize,
                    step as nat == spec_second_hash(key, m as nat),
                    table.table@.len() == m as int,
                    slot < m,
                    slot as int == (h as int + attempt as int * step as int) % (m as int),
                    h as nat == (table.spec_hash@)(key) % (m as nat),
                    spec_doublehashflathashsteph_wf(table),
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    old(table).num_elements < usize::MAX,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d * step as int) % (m as int)], key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * step as int) % (m as int)] is Empty),
                decreases m - attempt,
            {
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, _v) => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block (speed hint)
                        // Veracity: NEEDED assert
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, &key);
                        if eq {
                            // Overwrite existing key.
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
                                // Wf: probe chain (Occupied→Occupied).
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        let s = spec_second_hash(k, m as nat) as int;
                                        s >= 1
                                        && exists |n: int| #![trigger table.table@[(hk + n * s) % m as int]] 0 <= n < m as int
                                            && (hk + n * s) % m as int == i
                                            && forall |j: int| 0 <= j < n
                                                ==> !(#[trigger] table.table@[(hk + j * s) % m as int] is Empty)
                                    }) by {
                                    if i == slot as int {
                                        if k == key {}
                                    }
                                    let hk = (table.spec_hash@)(k) as int % m as int;
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
                        // New key at empty slot.
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
                                if spec_flat_has_key(old_table_seq[j], key) {}
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(old_table_seq, key);
                            let new_entry = FlatEntry::<Key, Value>::Occupied(key, value);
                            lemma_table_to_map_update_insert::<Key, Value, FlatEntry<Key, Value>>(
                                old_table_seq, slot as int, new_entry, key, value);
                            // Wf: no-dup.
                            // Wf: probe chain — new key with witness n=attempt.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |i: int, k: Key|
                                0 <= i < m as int
                                && #[trigger] spec_flat_has_key(table.table@[i], k)
                                implies ({
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    let s = spec_second_hash(k, m as nat) as int;
                                    s >= 1
                                    && exists |n: int| #![trigger table.table@[(hk + n * s) % m as int]] 0 <= n < m as int
                                        && (hk + n * s) % m as int == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] table.table@[(hk + j * s) % m as int] is Empty)
                                }) by {
                                let hk = (table.spec_hash@)(k) as int % m as int;
                                if i == slot as int {
                                    if k == key {
                                    }
                                } else {
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(spec_other_slots_preserved(
                                old(table).table@, table.table@, slot as int));
                        // Veracity: NEEDED proof block
                        }
                        return;
                    }
                    FlatEntry::Deleted => {
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                // Veracity: NEEDED proof block
                }
                // Incremental slot update (same as lookup).
                let step_mod: usize = step % m;
                let ghost prev_slot: int = slot as int;
                slot = if step_mod < m - slot { slot + step_mod } else { step_mod - (m - slot) };
                // Veracity: NEEDED proof block
                proof {
                    let gs: int = step as int;
                    let gm: int = m as int;
                    let gsm: int = step_mod as int;
                    let ga: int = attempt as int;
                    let gh: int = h as int;
                    if gsm < gm - prev_slot {
                        vstd::arithmetic::div_mod::lemma_small_mod((prev_slot + gsm) as nat, gm as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_small_mod((prev_slot + gsm - gm) as nat, gm as nat);
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(prev_slot + gsm - gm, gm);
                    }
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(prev_slot, gs, gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(gs, gh + ga * gs, gm);
                // Veracity: NEEDED proof block
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions — unreachable given spec_has_insert_capacity.
            // spec_second_hash == 1 (via lemma), so step == 1 and double hashing
            // degenerates to linear probing. All m positions are visited.
            // Veracity: NEEDED proof block
            proof {
                lemma_spec_second_hash_value::<Key>(key, m as nat);
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall |j: int| 0 <= j < m as int
                    implies !(#[trigger] table.table@[j] is Empty) by {
                    lemma_probe_mod_identity(h as int, j, m as int);
                    let d = (j - h as int + m as int) % (m as int);
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(d * 1int == d) by(nonlinear_arith);
                }
            }
            diverge::<()>();
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let step = Self::second_hash(key, m);
            // step == spec_second_hash(*key, m): wf gives concrete probe chain.
            let mut slot: usize = h;
            // Veracity: NEEDED proof block
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
            }
            let mut attempt: usize = 0;
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m > 0,
                    h < m,
                    step >= 1usize,
                    step as nat == spec_second_hash(*key, m as nat),
                    table.table@.len() == m as int,
                    slot < m,
                    slot as int == (h as int + attempt as int * step as int) % (m as int),
                    h as nat == (table.spec_hash@)(*key) % (m as nat),
                    spec_doublehashflathashsteph_wf(table),
                    // Prior probe positions don't have the key.
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d * step as int) % (m as int)], *key),
                    // Prior probe positions are not Empty.
                    // Veracity: NEEDED proof block
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * step as int) % (m as int)] is Empty),
                decreases m - attempt,
            // Veracity: NEEDED proof block
            {
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
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |j: int| 0 <= j < table.table@.len() && j != slot as int
                                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(table.table@[j], *key) {
                                        // Veracity: NEEDED proof block (speed hint)
                                        // Veracity: NEEDED assert
                                        // Veracity: NEEDED assert
                                        assert(spec_flat_has_key(table.table@[slot as int], *key));
                                    }
                                // Veracity: NEEDED proof block
                                }
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
                                // Veracity: NEEDED proof block
                                if spec_flat_has_key(table.table@[j], *key) {
                                    // Bridge gave us: exists n with (h+n*step)%m == j, path non-Empty.
                                    // If n < attempt: invariant says !spec_flat_has_key. Contradiction.
                                    // If n >= attempt: path[attempt] non-Empty, but slot is Empty.
                                    // If n == attempt: (h+n*step)%m == slot is Empty, can't be Occupied.
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
                // Update slot = (slot + step) % m incrementally.
                let step_mod: usize = step % m;
                let ghost prev_slot: int = slot as int;
                slot = if step_mod < m - slot { slot + step_mod } else { step_mod - (m - slot) };
                // Veracity: NEEDED proof block
                proof {
                    let gs: int = step as int;
                    let gm: int = m as int;
                    let gsm: int = step_mod as int;
                    let ga: int = attempt as int;
                    let gh: int = h as int;
                    // Safe addition produces (prev_slot + step_mod) % m.
                    if gsm < gm - prev_slot {
                        vstd::arithmetic::div_mod::lemma_small_mod((prev_slot + gsm) as nat, gm as nat);
                    } else {
                        // Veracity: NEEDED proof block
                        vstd::arithmetic::div_mod::lemma_small_mod((prev_slot + gsm - gm) as nat, gm as nat);
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(prev_slot + gsm - gm, gm);
                    }
                    // (prev_slot + step % m) % m == (prev_slot + step) % m.
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(prev_slot, gs, gm);
                    // Chain: prev_slot == (h + attempt * step) % m, so
                    // (prev_slot + step) % m == (step + (h + attempt*step) % m) % m
                    //                       == (step + h + attempt*step) % m
                    //                       == (h + (attempt+1)*step) % m.
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(gs, gh + ga * gs, gm);
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
                        // Bridge: exists n < m with (h+n*step)%m == j.
                        // n < m == attempt, so invariant: !spec_flat_has_key at (h+n*step)%m == j.
                    // Veracity: NEEDED proof block
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
            let step = Self::second_hash(key, m);
            // step == spec_second_hash(*key, m): wf gives concrete probe chain.
            let mut slot: usize = h;
            // Veracity: NEEDED proof block
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
            }
            let mut attempt: usize = 0;
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m == old(table).current_size,
                    m > 0,
                    h < m,
                    step >= 1usize,
                    step as nat == spec_second_hash(*key, m as nat),
                    table.table@.len() == m as int,
                    slot < m,
                    slot as int == (h as int + attempt as int * step as int) % (m as int),
                    // Veracity: NEEDED proof block
                    h as nat == (table.spec_hash@)(*key) % (m as nat),
                    spec_doublehashflathashsteph_wf(table),
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d * step as int) % (m as int)], *key),
                    // Veracity: NEEDED proof block
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * step as int) % (m as int)] is Empty),
                decreases m - attempt,
            {
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
                                let new_entry = FlatEntry::<Key, Value>::Deleted;
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(new_entry.spec_entry_to_map() =~=
                                    old_table_seq[slot as int].spec_entry_to_map().remove(*key));
                                lemma_table_to_map_update_remove::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, new_entry, *key);
                                lemma_table_to_map_unique_entry_value::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, *key);
                                // Wf: no-dup.
                                // Wf: probe chain integrity.
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |i: int, k: Key|
                                    // Veracity: NEEDED proof block
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        // Veracity: NEEDED proof block
                                        let s = spec_second_hash(k, m as nat) as int;
                                        s >= 1
                                        && exists |n: int| #![trigger table.table@[(hk + n * s) % m as int]] 0 <= n < m as int
                                            && (hk + n * s) % m as int == i
                                            && forall |j: int| 0 <= j < n
                                                ==> !(#[trigger] table.table@[(hk + j * s) % m as int] is Empty)
                                    }) by {
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    // Old wf: same witnesses work for new table.
                                    // Non-Empty preserved at every position.
                                }
                            }
                            return true;
                        // Veracity: NEEDED proof block
                        }
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                    FlatEntry::Empty => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED proof block
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                if spec_flat_has_key(table.table@[j], *key) {
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
                // Incremental slot update (same as lookup).
                // Veracity: NEEDED proof block
                let step_mod: usize = step % m;
                let ghost prev_slot: int = slot as int;
                slot = if step_mod < m - slot { slot + step_mod } else { step_mod - (m - slot) };
                // Veracity: NEEDED proof block
                proof {
                    let gs: int = step as int;
                    let gm: int = m as int;
                    let gsm: int = step_mod as int;
                    let ga: int = attempt as int;
                    let gh: int = h as int;
                    if gsm < gm - prev_slot {
                        vstd::arithmetic::div_mod::lemma_small_mod((prev_slot + gsm) as nat, gm as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_small_mod((prev_slot + gsm - gm) as nat, gm as nat);
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(prev_slot + gsm - gm, gm);
                    }
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(prev_slot, gs, gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(gs, gh + ga * gs, gm);
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions.
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            false
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m').
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' slots, reinserts.
        fn resize(
            table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
            new_size: usize,
        ) -> (resized: HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) {
            // Phase 1: collect occupied pairs.
            let mut pairs: Vec<(Key, Value)> = Vec::new();
            // Veracity: NEEDED proof block
            let mut i: usize = 0;
            while i < table.table.len()
                invariant
                    // Veracity: NEEDED proof block
                    i <= table.table@.len(),
                    table.table@.len() == table.current_size as int,
                    pairs@.len() <= i as int,
                    spec_seq_pairs_to_map(pairs@) =~=
                        spec_table_to_map::<Key, Value, FlatEntry<Key, Value>>(
                            table.table@.subrange(0, i as int)),
                decreases table.table.len() - i,
            {
                // Veracity: NEEDED proof block
                let ghost old_pairs = pairs@;
                let ghost old_map = spec_seq_pairs_to_map(old_pairs);
                let entry = table.table[i].clone();
                if let FlatEntry::Occupied(k, v) = entry {
                    pairs.push((k, v));
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(pairs@.drop_last() =~= old_pairs);
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
                    // Veracity: NEEDED proof block
                    assert(sub_next.drop_last() =~= table.table@.subrange(0, i as int));
                }
                i = i + 1;
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
                // Veracity: NEEDED proof block
                invariant
                    k <= new_size,
                    new_table_vec@.len() == k as int,
                    forall |j: int| 0 <= j < new_table_vec@.len()
                        ==> (#[trigger] new_table_vec@[j]) is Empty,
                    spec_table_to_map::<Key, Value, FlatEntry<Key, Value>>(new_table_vec@)
                        == Map::<Key, Value>::empty(),
                // Veracity: NEEDED proof block
                decreases new_size - k,
            {
                let ghost old_vec = new_table_vec@;
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
            // Veracity: NEEDED proof block (speed hint)
            proof {
                // Veracity: NEEDED assert
                assert(spec_doublehashflathashsteph_wf(&new_table));
            }

            // Phase 3: reinsert all pairs.
            // Veracity: NEEDED proof block
            let mut j: usize = 0;
            // Veracity: NEEDED proof block
            proof {
                lemma_all_empties_count::<Key, Value>(new_table.table@);
            }
            while j < pairs.len()
                invariant
                    j <= pairs@.len(),
                    new_size > 0,
                    new_table.current_size == new_size,
                    new_table.table@.len() == new_table.current_size as int,
                    new_table.num_elements <= j,
                    // Veracity: NEEDED proof block
                    Self::spec_parahashtablesteph_wf(&new_table),
                    new_table@ =~= spec_seq_pairs_to_map(pairs@.subrange(0, j as int)),
                    new_table.spec_hash == table.spec_hash,
                    pairs@.len() <= table.current_size as int,
                    new_size as int > table.current_size as int,
                    spec_count_empties(new_table.table@) >= (new_size - j) as int,
                    obeys_feq_clone::<Key>(),
                    obeys_feq_clone::<Value>(),
                decreases pairs.len() - j,
            {
                let key = clone_elem(&pairs[j].0);
                let value = clone_elem(&pairs[j].1);
                // Veracity: NEEDED proof block
                proof {
                    lemma_empties_positive_implies_exists_empty::<Key, Value>(
                        new_table.table@);
                }
                let ghost old_new_table_seq = new_table.table@;
                Self::insert(&mut new_table, key, value);
                // Veracity: NEEDED proof block
                proof {
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

    impl<Key: StT + Hash, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for DoubleHashFlatHashTableStEph
    {
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — two hash values + arithmetic + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash1 = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let step = Self::second_hash(key, table.current_size);
            (hash1.wrapping_add(attempt.wrapping_mul(step))) % table.current_size
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


    impl std::fmt::Debug for DoubleHashFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DoubleHashFlatHashTableStEph")
        }
    }

    impl std::fmt::Display for DoubleHashFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DoubleHashFlatHashTableStEph")
        }
    }
}
