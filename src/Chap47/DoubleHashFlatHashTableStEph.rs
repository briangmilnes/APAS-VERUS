//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Double Hashing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses double hashing for open addressing collision resolution.

pub mod DoubleHashFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!)
    // 7. proof fns (inside verus!)
    // 9. impls (inside verus!)

    // 2. imports
    use std::hash::Hash;
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Concurrency::diverge;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    verus! {

    // 3. broadcast use
    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions

    /// Double Hashing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i·hh(k)) mod m
    /// Uses two hash functions to avoid both primary and secondary clustering.
    pub struct DoubleHashFlatHashTableStEph;

    // 6. spec fns

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

    // 7. proof fns

    // 9. impls

    impl DoubleHashFlatHashTableStEph {
        /// Compute second hash value for double hashing.
        /// APAS: hh(k) must be relatively prime to m.
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(sizeof(Key)), Span O(sizeof(Key)) — hashes key with SipHash.
        /// Strategy: Always return an odd number (works for power-of-2 sizes),
        /// and for prime sizes, ensure < m and non-zero.
        pub fn second_hash<Key: StT + Hash>(key: &Key, table_size: usize) -> (step: usize)
            requires table_size > 0,
            ensures
                step >= 1,
                table_size > 1 ==> step < table_size,
                step as nat == spec_second_hash(*key, table_size as nat),
        {
            compute_second_hash(key, table_size)
        }
    }

    impl<Key: StT + Hash, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for DoubleHashFlatHashTableStEph
    {
        open spec fn spec_impl_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            spec_doublehashflathashsteph_wf(table)
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe find_slot then set.
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: Key, value: Value) {
            let h = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let step = Self::second_hash(&key, m);
            // step == spec_second_hash(key, m): wf gives concrete probe chain.
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(slot as int == (h as int + 0int * step as int) % (m as int));
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
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, &key);
                        if eq {
                            // Overwrite existing key.
                            let ghost old_table_seq = table.table@;
                            table.table.set(slot, FlatEntry::Occupied(key, value));
                            proof {
                                assert(spec_flat_has_key(old_table_seq[slot as int], key));
                                assert forall |j: int| 0 <= j < old_table_seq.len() && j != slot as int
                                    implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(key) by {
                                    if spec_flat_has_key(old_table_seq[j], key) {
                                        assert(spec_flat_has_key(old_table_seq[slot as int], key));
                                    }
                                }
                                let new_entry = FlatEntry::<Key, Value>::Occupied(key, value);
                                assert(new_entry.spec_entry_to_map() =~= Map::<Key, Value>::empty().insert(key, value));
                                assert(new_entry.spec_entry_to_map() =~=
                                    old_table_seq[slot as int].spec_entry_to_map().insert(key, value));
                                lemma_table_to_map_update_insert::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, new_entry, key, value);
                                // Wf: no-dup.
                                assert forall |i: int, j: int, k: Key|
                                    0 <= i < m as int && 0 <= j < m as int && i != j
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies !#[trigger] spec_flat_has_key(table.table@[j], k) by {
                                    if i == slot as int {
                                        assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                        if k == key && j != slot as int {
                                            assert(table.table@[j] == old_table_seq[j]);
                                            assert(!spec_flat_has_key(old_table_seq[j], key));
                                        }
                                    } else {
                                        assert(table.table@[i] == old_table_seq[i]);
                                        assert(spec_flat_has_key(old_table_seq[i], k));
                                        if j != slot as int {
                                            assert(table.table@[j] == old_table_seq[j]);
                                        } else {
                                            assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                            if k == key {
                                                assert(spec_flat_has_key(old_table_seq[slot as int], key));
                                                assert(i != slot as int);
                                            }
                                        }
                                    }
                                }
                                // Wf: probe chain (Occupied→Occupied).
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
                                        assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                        if k == key {
                                            assert(spec_flat_has_key(old_table_seq[slot as int], key));
                                        }
                                    } else {
                                        assert(table.table@[i] == old_table_seq[i]);
                                        assert(spec_flat_has_key(old_table_seq[i], k));
                                    }
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    // Non-Empty preserved at every position.
                                    assert forall |pos: int| 0 <= pos < m as int
                                        && !(old_table_seq[pos] is Empty)
                                        implies !(#[trigger] table.table@[pos] is Empty) by {
                                        if pos == slot as int {
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                }
                                // One-slot modification witness for trait ensures.
                                assert(spec_other_slots_preserved(
                                    old(table).table@, table.table@, slot as int));
                            }
                            return;
                        }
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                    FlatEntry::Empty => {
                        // New key at empty slot.
                        let ghost old_table_seq = table.table@;
                        table.table.set(slot, FlatEntry::Occupied(key, value));
                        if table.num_elements < usize::MAX {
                            table.num_elements = table.num_elements + 1;
                        }
                        proof {
                            assert(old_table_seq[slot as int] is Empty);
                            assert forall |j: int| 0 <= j < old_table_seq.len()
                                implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(key) by {
                                if spec_flat_has_key(old_table_seq[j], key) {
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(old_table_seq, key);
                            let new_entry = FlatEntry::<Key, Value>::Occupied(key, value);
                            assert(new_entry.spec_entry_to_map() =~= Map::<Key, Value>::empty().insert(key, value));
                            assert(old_table_seq[slot as int].spec_entry_to_map() =~= Map::<Key, Value>::empty());
                            assert(new_entry.spec_entry_to_map() =~=
                                old_table_seq[slot as int].spec_entry_to_map().insert(key, value));
                            lemma_table_to_map_update_insert::<Key, Value, FlatEntry<Key, Value>>(
                                old_table_seq, slot as int, new_entry, key, value);
                            // Wf: no-dup.
                            assert forall |i: int, j: int, k: Key|
                                0 <= i < m as int && 0 <= j < m as int && i != j
                                && #[trigger] spec_flat_has_key(table.table@[i], k)
                                implies !#[trigger] spec_flat_has_key(table.table@[j], k) by {
                                if i == slot as int {
                                    assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                    if k == key && j != slot as int {
                                        assert(table.table@[j] == old_table_seq[j]);
                                        assert(!old_table_seq[j].spec_entry_to_map().dom().contains(key));
                                        if spec_flat_has_key(old_table_seq[j], key) {
                                        }
                                    }
                                } else {
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    if j == slot as int {
                                        assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                        if k == key {
                                            assert(!old_table_seq[i].spec_entry_to_map().dom().contains(key));
                                            if spec_flat_has_key(old_table_seq[i], key) {
                                            }
                                        }
                                    } else {
                                        assert(table.table@[j] == old_table_seq[j]);
                                    }
                                }
                            }
                            // Wf: probe chain — new key with witness n=attempt.
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
                                    assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                    if k == key {
                                        assert(hk == h as int);
                                        // step == spec_second_hash(key, m): witness n = attempt.
                                        assert(spec_second_hash(key, m as nat) as int == step as int);
                                        assert forall |j: int| 0 <= j < attempt as int
                                            implies !(#[trigger] table.table@[(h as int + j * step as int) % m as int] is Empty) by {
                                            let pos = (h as int + j * step as int) % m as int;
                                            if pos == slot as int {
                                            }
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                } else {
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    // Non-Empty preserved at every position.
                                    assert forall |pos: int| 0 <= pos < m as int
                                        && !(old_table_seq[pos] is Empty)
                                        implies !(#[trigger] table.table@[pos] is Empty) by {
                                        if pos == slot as int {
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                }
                            }
                            // One-slot modification witness for trait ensures.
                            assert(old_table_seq =~= old(table).table@);
                            assert(spec_other_slots_preserved(
                                old(table).table@, table.table@, slot as int));
                        }
                        return;
                    }
                    FlatEntry::Deleted => {
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                }
                // Incremental slot update (same as lookup).
                let step_mod: usize = step % m;
                let ghost prev_slot: int = slot as int;
                slot = if step_mod < m - slot { slot + step_mod } else { step_mod - (m - slot) };
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
                    assert(slot as int == (prev_slot + gsm) % gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(prev_slot, gs, gm);
                    assert(slot as int == (prev_slot + gs) % gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(gs, gh + ga * gs, gm);
                    assert(gs + gh + ga * gs == gh + (ga + 1) * gs) by(nonlinear_arith);
                    assert(slot as int == (gh + (ga + 1) * gs) % gm);
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions.
            proof {
                assume(false); // Table full: unreachable with load factor < 1.
            }
            diverge::<()>();
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until found or empty.
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let step = Self::second_hash(key, m);
            // step == spec_second_hash(*key, m): wf gives concrete probe chain.
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(slot as int == (h as int + 0int * step as int) % (m as int));
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
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * step as int) % (m as int)] is Empty),
                decreases m - attempt,
            {
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, v) => {
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, key);
                        if eq {
                            proof {
                                assert(spec_flat_has_key(table.table@[slot as int], *key));
                                assert forall |j: int| 0 <= j < table.table@.len() && j != slot as int
                                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(table.table@[j], *key) {
                                        assert(spec_flat_has_key(table.table@[slot as int], *key));
                                    }
                                }
                                lemma_table_to_map_unique_entry_value::<Key, Value, FlatEntry<Key, Value>>(
                                    table.table@, slot as int, *key);
                            }
                            return Some(v);
                        }
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], *key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                    FlatEntry::Empty => {
                        proof {
                            assert(table.table@[slot as int] is Empty);
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                if spec_flat_has_key(table.table@[j], *key) {
                                    // Bridge gave us: exists n with (h+n*step)%m == j, path non-Empty.
                                    // If n < attempt: invariant says !spec_flat_has_key. Contradiction.
                                    // If n >= attempt: path[attempt] non-Empty, but slot is Empty.
                                    // If n == attempt: (h+n*step)%m == slot is Empty, can't be Occupied.
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                        }
                        return None;
                    }
                    FlatEntry::Deleted => {
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], *key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                }
                // Update slot = (slot + step) % m incrementally.
                let step_mod: usize = step % m;
                let ghost prev_slot: int = slot as int;
                slot = if step_mod < m - slot { slot + step_mod } else { step_mod - (m - slot) };
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
                        vstd::arithmetic::div_mod::lemma_small_mod((prev_slot + gsm - gm) as nat, gm as nat);
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(prev_slot + gsm - gm, gm);
                    }
                    assert(slot as int == (prev_slot + gsm) % gm);
                    // (prev_slot + step % m) % m == (prev_slot + step) % m.
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(prev_slot, gs, gm);
                    assert(slot as int == (prev_slot + gs) % gm);
                    // Chain: prev_slot == (h + attempt * step) % m, so
                    // (prev_slot + step) % m == (step + (h + attempt*step) % m) % m
                    //                       == (step + h + attempt*step) % m
                    //                       == (h + (attempt+1)*step) % m.
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(gs, gh + ga * gs, gm);
                    assert(gs + gh + ga * gs == gh + (ga + 1) * gs) by(nonlinear_arith);
                    assert(slot as int == (gh + (ga + 1) * gs) % gm);
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions without finding key.
            proof {
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                        // Bridge: exists n < m with (h+n*step)%m == j.
                        // n < m == attempt, so invariant: !spec_flat_has_key at (h+n*step)%m == j.
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            None
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until found or empty, then tombstone.
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let step = Self::second_hash(key, m);
            // step == spec_second_hash(*key, m): wf gives concrete probe chain.
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(slot as int == (h as int + 0int * step as int) % (m as int));
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
                    h as nat == (table.spec_hash@)(*key) % (m as nat),
                    spec_doublehashflathashsteph_wf(table),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d * step as int) % (m as int)], *key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * step as int) % (m as int)] is Empty),
                decreases m - attempt,
            {
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, _v) => {
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, key);
                        if eq {
                            let ghost old_table_seq = table.table@;
                            table.table.set(slot, FlatEntry::Deleted);
                            if table.num_elements > 0 {
                                table.num_elements = table.num_elements - 1;
                            }
                            proof {
                                assert(spec_flat_has_key(old_table_seq[slot as int], *key));
                                assert forall |j: int| 0 <= j < old_table_seq.len() && j != slot as int
                                    implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(old_table_seq[j], *key) {
                                        assert(spec_flat_has_key(old_table_seq[slot as int], *key));
                                    }
                                }
                                let new_entry = FlatEntry::<Key, Value>::Deleted;
                                assert(new_entry.spec_entry_to_map() =~= Map::<Key, Value>::empty());
                                assert(old_table_seq[slot as int].spec_entry_to_map().dom().contains(*key));
                                assert(new_entry.spec_entry_to_map() =~=
                                    old_table_seq[slot as int].spec_entry_to_map().remove(*key));
                                lemma_table_to_map_update_remove::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, new_entry, *key);
                                lemma_table_to_map_unique_entry_value::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, *key);
                                // Wf: no-dup.
                                assert forall |i: int, j: int, k: Key|
                                    0 <= i < m as int && 0 <= j < m as int && i != j
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies !#[trigger] spec_flat_has_key(table.table@[j], k) by {
                                    if i == slot as int {
                                    } else {
                                        assert(table.table@[i] == old_table_seq[i]);
                                        assert(spec_flat_has_key(old_table_seq[i], k));
                                        if j != slot as int {
                                            assert(table.table@[j] == old_table_seq[j]);
                                        }
                                    }
                                }
                                // Wf: probe chain integrity.
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
                                    assert(i != slot as int);
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    // Old wf: same witnesses work for new table.
                                    // Non-Empty preserved at every position.
                                    assert forall |pos: int| 0 <= pos < m as int
                                        && !(old_table_seq[pos] is Empty)
                                        implies !(#[trigger] table.table@[pos] is Empty) by {
                                        if pos == slot as int {
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                }
                            }
                            return true;
                        }
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], *key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                    FlatEntry::Empty => {
                        proof {
                            assert(table.table@[slot as int] is Empty);
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
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], *key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                }
                // Incremental slot update (same as lookup).
                let step_mod: usize = step % m;
                let ghost prev_slot: int = slot as int;
                slot = if step_mod < m - slot { slot + step_mod } else { step_mod - (m - slot) };
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
                    assert(slot as int == (prev_slot + gsm) % gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(prev_slot, gs, gm);
                    assert(slot as int == (prev_slot + gs) % gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(gs, gh + ga * gs, gm);
                    assert(gs + gh + ga * gs == gh + (ga + 1) * gs) by(nonlinear_arith);
                    assert(slot as int == (gh + (ga + 1) * gs) % gm);
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions.
            proof {
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            false
        }

        /// - APAS: Work O(n + m + m'), Span O(n + m + m').
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' slots, reinserts.
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
                    spec_seq_pairs_to_map(pairs@) =~=
                        spec_table_to_map::<Key, Value, FlatEntry<Key, Value>>(
                            table.table@.subrange(0, i as int)),
                decreases table.table.len() - i,
            {
                let ghost old_pairs = pairs@;
                let ghost old_map = spec_seq_pairs_to_map(old_pairs);
                let entry = table.table[i].clone();
                if let FlatEntry::Occupied(k, v) = entry {
                    pairs.push((k, v));
                    proof {
                        assert(pairs@.drop_last() =~= old_pairs);
                        assert(pairs@.last() == (k, v));
                        assert(spec_seq_pairs_to_map(pairs@)
                            =~= spec_seq_pairs_to_map(old_pairs).insert(k, v));
                        assert(spec_seq_pairs_to_map(pairs@) =~= old_map.insert(k, v));
                        let ghost entry_map = table.table@[i as int].spec_entry_to_map();
                        assert(entry_map =~= Map::<Key, Value>::empty().insert(k, v));
                        assert(old_map.insert(k, v)
                            =~= old_map.union_prefer_right(entry_map));
                    }
                } else {
                    proof {
                        assert(table.table@[i as int].spec_entry_to_map()
                            =~= Map::<Key, Value>::empty());
                    }
                }
                proof {
                    let ghost sub_next = table.table@.subrange(0, (i + 1) as int);
                    assert(sub_next.drop_last() =~= table.table@.subrange(0, i as int));
                    assert(sub_next.last() == table.table@[i as int]);
                }
                i = i + 1;
            }
            proof {
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
                    forall |j: int| 0 <= j < new_table_vec@.len()
                        ==> (#[trigger] new_table_vec@[j]) is Empty,
                    spec_table_to_map::<Key, Value, FlatEntry<Key, Value>>(new_table_vec@)
                        == Map::<Key, Value>::empty(),
                decreases new_size - k,
            {
                let ghost old_vec = new_table_vec@;
                new_table_vec.push(FlatEntry::Empty);
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
            proof {
                assert forall |j: int| 0 <= j < new_table.table@.len()
                    implies (#[trigger] new_table.table@[j]) is Empty by {}
                assert(spec_doublehashflathashsteph_wf(&new_table));
            }

            // Phase 3: reinsert all pairs.
            let mut j: usize = 0;
            while j < pairs.len()
                invariant
                    j <= pairs@.len(),
                    new_size > 0,
                    new_table.current_size == new_size,
                    new_table.table@.len() == new_table.current_size as int,
                    new_table.num_elements <= j,
                    Self::spec_impl_wf(&new_table),
                    new_table@ =~= spec_seq_pairs_to_map(pairs@.subrange(0, j as int)),
                    new_table.spec_hash == table.spec_hash,
                decreases pairs.len() - j,
            {
                let key = clone_elem(&pairs[j].0);
                let value = clone_elem(&pairs[j].1);
                Self::insert(&mut new_table, key, value);
                proof {
                    assert(pairs@.subrange(0, (j + 1) as int).drop_last()
                        =~= pairs@.subrange(0, j as int));
                    assert(pairs@.subrange(0, (j + 1) as int).last()
                        == pairs@[j as int]);
                }
                j = j + 1;
            }
            proof {
                assert(pairs@.subrange(0, pairs@.len() as int) =~= pairs@);
            }

            new_table
        }
    }

    impl<Key: StT + Hash, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for DoubleHashFlatHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — two hash values + arithmetic + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash1 = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let step = Self::second_hash(key, table.current_size);
            (hash1.wrapping_add(attempt.wrapping_mul(step))) % table.current_size
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — double hash probe until empty/deleted/matching.
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
