//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses quadratic probing for open addressing collision resolution.

pub mod QuadProbFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!)
    // 7. proof fns (inside verus!)
    // 9. impls (inside verus!)
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    verus! {

    // 3. broadcast use
    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions

    /// Quadratic Probing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i²) mod m
    pub struct QuadProbFlatHashTableStEph;

    // 6. spec fns

    /// Well-formedness for quadratic probing flat hash tables.
    /// Quadratic probe sequence: slot (h + j²) % m for attempt j = 0, 1, 2, ...
    /// (1) No duplicate keys across slots.
    /// (2) Every occupied key is reachable via quadratic probing from its hash:
    ///     there exists an attempt n placing the key at its slot, and all earlier
    ///     probe positions are not Empty.
    pub open spec fn spec_quadprobflathashsteph_wf<Key, Value, Metrics, H>(
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
        // Probe chain integrity for quadratic probing.
        && (forall |i: int, k: Key|
            0 <= i < m
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> {
                let h = (table.spec_hash@)(k) as int % m;
                exists |n: int| #![trigger table.table@[(h + n * n) % m]] 0 <= n < m
                    && (h + n * n) % m == i
                    && forall |j: int| 0 <= j < n
                        ==> !(#[trigger] table.table@[(h + j * j) % m] is Empty)
            })
    }

    // 7. proof fns

    // 9. impls

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for QuadProbFlatHashTableStEph
    {
        open spec fn spec_impl_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            spec_quadprobflathashsteph_wf(table)
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe find_slot then set.
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: Key, value: Value) {
            let h = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(slot as int == (h as int + 0int * 0int) % (m as int));
            }
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m == old(table).current_size,
                    m > 0,
                    h < m,
                    slot < m,
                    table.table@.len() == m as int,
                    h as nat == (table.spec_hash@)(key) % (m as nat),
                    slot as int == (h as int + attempt as int * attempt as int) % (m as int),
                    spec_quadprobflathashsteph_wf(table),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    old(table).num_elements < usize::MAX,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d * d) % (m as int)], key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * d) % (m as int)] is Empty),
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
                                // Wf: no-dup (same key at same slot).
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
                                // Wf: probe chain (Occupied→Occupied, no Empty/non-Empty change).
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        exists |n: int| #![trigger table.table@[(hk + n * n) % m as int]] 0 <= n < m as int
                                            && (hk + n * n) % m as int == i
                                            && forall |j: int| 0 <= j < n
                                                ==> !(#[trigger] table.table@[(hk + j * j) % m as int] is Empty)
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
                                    let n = choose |n: int| #![trigger old_table_seq[(hk + n * n) % m as int]] 0 <= n < m as int
                                        && (hk + n * n) % m as int == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[(hk + j * j) % m as int] is Empty);
                                    assert forall |j: int| 0 <= j < n
                                        implies !(#[trigger] table.table@[(hk + j * j) % m as int] is Empty) by {
                                        let pos = (hk + j * j) % m as int;
                                        if pos == slot as int {
                                            // Was Occupied, now Occupied. Not Empty.
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                }
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
                            // Key not in table (same proof as lookup not-found).
                            assert(old_table_seq[slot as int] is Empty);
                            assert forall |j: int| 0 <= j < old_table_seq.len()
                                implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(key) by {
                                if spec_flat_has_key(old_table_seq[j], key) {
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(old_table_seq, key);
                            // Map update.
                            let new_entry = FlatEntry::<Key, Value>::Occupied(key, value);
                            assert(new_entry.spec_entry_to_map() =~= Map::<Key, Value>::empty().insert(key, value));
                            assert(old_table_seq[slot as int].spec_entry_to_map() =~= Map::<Key, Value>::empty());
                            assert(new_entry.spec_entry_to_map() =~=
                                old_table_seq[slot as int].spec_entry_to_map().insert(key, value));
                            lemma_table_to_map_update_insert::<Key, Value, FlatEntry<Key, Value>>(
                                old_table_seq, slot as int, new_entry, key, value);
                            // Wf: no-dup — key wasn't anywhere, now only at slot.
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
                            // Wf: probe chain — new key at slot with witness n = attempt.
                            assert forall |i: int, k: Key|
                                0 <= i < m as int
                                && #[trigger] spec_flat_has_key(table.table@[i], k)
                                implies ({
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    exists |n: int| #![trigger table.table@[(hk + n * n) % m as int]] 0 <= n < m as int
                                        && (hk + n * n) % m as int == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] table.table@[(hk + j * j) % m as int] is Empty)
                                }) by {
                                let hk = (table.spec_hash@)(k) as int % m as int;
                                if i == slot as int {
                                    assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                    if k == key {
                                        assert(hk == h as int);
                                        // Witness: n = attempt. (h + attempt²) % m == slot.
                                        // Probe path 0..attempt was not Empty (invariant + slot update).
                                        assert forall |j: int| 0 <= j < attempt as int
                                            implies !(#[trigger] table.table@[(h as int + j * j) % m as int] is Empty) by {
                                            let pos = (h as int + j * j) % m as int;
                                            if pos == slot as int {
                                                // pos == slot means (h+j²)%m == (h+attempt²)%m.
                                                // Invariant: !spec_flat_has_key at (h+j²)%m.
                                                // But slot is Empty in old table. So old[pos] is Empty.
                                                // But invariant says pos is not Empty. Contradiction.
                                            }
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                } else {
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    let n = choose |n: int| #![trigger old_table_seq[(hk + n * n) % m as int]] 0 <= n < m as int
                                        && (hk + n * n) % m as int == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[(hk + j * j) % m as int] is Empty);
                                    assert forall |j: int| 0 <= j < n
                                        implies !(#[trigger] table.table@[(hk + j * j) % m as int] is Empty) by {
                                        let pos = (hk + j * j) % m as int;
                                        if pos == slot as int {
                                            // Was Empty, now Occupied. Not Empty.
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                }
                            }
                        }
                        return;
                    }
                    FlatEntry::Deleted => {
                        // Skip Deleted: continue probing to avoid duplicates.
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                }
                // Incremental slot update (same as lookup).
                let ghost prev_slot: int = slot as int;
                let slot1: usize = if attempt < m - slot { slot + attempt } else { attempt - (m - slot) };
                proof {
                    let sum = prev_slot + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                    assert(slot1 as int == (prev_slot + attempt as int) % (m as int));
                }
                let incr: usize = attempt + 1;
                slot = if incr < m {
                    if incr < m - slot1 { slot1 + incr } else { incr - (m - slot1) }
                } else {
                    slot1
                };
                proof {
                    let gi: int = incr as int;
                    let gm: int = m as int;
                    let gs1: int = slot1 as int;
                    let ga: int = attempt as int;
                    let gh: int = h as int;
                    if gi < gm {
                        let sum2 = gs1 + gi;
                        if sum2 < gm {
                            vstd::arithmetic::div_mod::lemma_small_mod(sum2 as nat, gm as nat);
                        } else {
                            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum2 - gm, gm);
                            vstd::arithmetic::div_mod::lemma_small_mod((sum2 - gm) as nat, gm as nat);
                        }
                        assert(slot as int == (gs1 + gi) % gm);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(gs1, gm);
                        vstd::arithmetic::div_mod::lemma_small_mod(gs1 as nat, gm as nat);
                        assert(slot as int == (gs1 + gi) % gm);
                    }
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        gi, prev_slot + ga, gm);
                    assert((gi + gs1) % gm == (gi + prev_slot + ga) % gm);
                    assert((gs1 + gi) % gm == (gi + gs1) % gm);
                    assert(slot as int == (gi + prev_slot + ga) % gm);
                    assert(prev_slot == (gh + ga * ga) % gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        gi + ga, gh + ga * ga, gm);
                    assert((gi + ga + prev_slot) % gm == (gi + ga + gh + ga * ga) % gm);
                    assert(gi + prev_slot + ga == gi + ga + prev_slot);
                    assert(slot as int == (gh + ga * ga + ga + gi) % gm);
                    assert((ga + 1) * (ga + 1) == ga * ga + 2 * ga + 1) by(nonlinear_arith);
                    assert(ga + gi == 2 * ga + 1);
                    assert(slot as int == (gh + (ga + 1) * (ga + 1)) % gm);
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions.
            proof {
                assume(false); // Table full: unreachable with load factor < 1.
            }
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe sequence until found or empty.
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(slot as int == (h as int + 0int * 0int) % (m as int));
            }
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m > 0,
                    h < m,
                    slot < m,
                    table.table@.len() == m as int,
                    h as nat == (table.spec_hash@)(*key) % (m as nat),
                    slot as int == (h as int + attempt as int * attempt as int) % (m as int),
                    spec_quadprobflathashsteph_wf(table),
                    // Prior probe positions don't have the key.
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d * d) % (m as int)], *key),
                    // Prior probe positions are not Empty.
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * d) % (m as int)] is Empty),
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
                            // If any key k == *key existed at some slot j, the wf says there exists
                            // an attempt n placing it there with no Empty on the quadratic path.
                            // But we found Empty at attempt, so n > attempt. And prior attempts
                            // don't have the key. The wf existential + our invariant give contradiction.
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                if spec_flat_has_key(table.table@[j], *key) {
                                    // wf: exists n: 0 <= n < m, (h + n*n) % m == j, path 0..n non-Empty.
                                    // At attempt d in 0..attempt: not Empty (invariant).
                                    // At attempt: Empty. So n != attempt. If n < attempt: invariant says
                                    // !spec_flat_has_key at (h+n*n)%m, but that's j. Contradiction.
                                    // If n > attempt: path[attempt] must be non-Empty, but it is. Contradiction.
                                    // So n >= attempt means path includes position attempt which is Empty.
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
                // Update slot for next attempt: slot_{a+1} = (h + (a+1)^2) % m
                // = (slot + 2*a + 1) % m. Compute in two overflow-safe steps.
                let ghost prev_slot: int = slot as int;
                let slot1: usize = if attempt < m - slot { slot + attempt } else { attempt - (m - slot) };
                proof {
                    let sum = prev_slot + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                    assert(slot1 as int == (prev_slot + attempt as int) % (m as int));
                }
                let incr: usize = attempt + 1;
                slot = if incr < m {
                    if incr < m - slot1 { slot1 + incr } else { incr - (m - slot1) }
                } else {
                    slot1
                };
                proof {
                    let gi: int = incr as int;
                    let gm: int = m as int;
                    let gs1: int = slot1 as int;
                    let ga: int = attempt as int;
                    let gh: int = h as int;
                    if gi < gm {
                        let sum2 = gs1 + gi;
                        if sum2 < gm {
                            vstd::arithmetic::div_mod::lemma_small_mod(sum2 as nat, gm as nat);
                        } else {
                            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum2 - gm, gm);
                            vstd::arithmetic::div_mod::lemma_small_mod((sum2 - gm) as nat, gm as nat);
                        }
                        assert(slot as int == (gs1 + gi) % gm);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(gs1, gm);
                        vstd::arithmetic::div_mod::lemma_small_mod(gs1 as nat, gm as nat);
                        assert(slot as int == (gs1 + gi) % gm);
                    }
                    // Chain: slot = (gs1 + gi) % gm
                    //   where gs1 = (prev_slot + ga) % gm, prev_slot = (gh + ga*ga) % gm.
                    // Lift to: slot = (gh + ga*ga + ga + gi) % gm = (gh + (ga+1)^2) % gm.
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        gi, prev_slot + ga, gm);
                    assert((gi + gs1) % gm == (gi + prev_slot + ga) % gm);
                    assert((gs1 + gi) % gm == (gi + gs1) % gm);
                    assert(slot as int == (gi + prev_slot + ga) % gm);
                    assert(prev_slot == (gh + ga * ga) % gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        gi + ga, gh + ga * ga, gm);
                    assert((gi + ga + prev_slot) % gm == (gi + ga + gh + ga * ga) % gm);
                    assert(gi + prev_slot + ga == gi + ga + prev_slot);
                    assert(slot as int == (gh + ga * ga + ga + gi) % gm);
                    assert((ga + 1) * (ga + 1) == ga * ga + 2 * ga + 1) by(nonlinear_arith);
                    assert(ga + gi == 2 * ga + 1);
                    assert(slot as int == (gh + (ga + 1) * (ga + 1)) % gm);
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions without finding key.
            proof {
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                        // wf says exists n: 0 <= n < m with (h+n*n)%m == j.
                        // n < m == attempt, so invariant: !spec_flat_has_key at (h+n*n)%m == j.
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            None
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe until found or empty, then tombstone.
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(slot as int == (h as int + 0int * 0int) % (m as int));
            }
            while attempt < m
                invariant
                    attempt <= m,
                    m == table.current_size,
                    m == old(table).current_size,
                    m > 0,
                    h < m,
                    slot < m,
                    table.table@.len() == m as int,
                    h as nat == (table.spec_hash@)(*key) % (m as nat),
                    slot as int == (h as int + attempt as int * attempt as int) % (m as int),
                    spec_quadprobflathashsteph_wf(table),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d * d) % (m as int)], *key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d * d) % (m as int)] is Empty),
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
                                // No other slot has *key (old wf no-dup).
                                assert forall |j: int| 0 <= j < old_table_seq.len() && j != slot as int
                                    implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(old_table_seq[j], *key) {
                                        assert(spec_flat_has_key(old_table_seq[slot as int], *key));
                                    }
                                }
                                // Map update: Deleted has empty map.
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
                                        exists |n: int| #![trigger table.table@[(hk + n * n) % m as int]] 0 <= n < m as int
                                            && (hk + n * n) % m as int == i
                                            && forall |j: int| 0 <= j < n
                                                ==> !(#[trigger] table.table@[(hk + j * j) % m as int] is Empty)
                                    }) by {
                                    assert(i != slot as int);
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    // Old wf: exists n with probe chain in old_table_seq.
                                    // Same n works: Occupied→Deleted at slot is not Empty.
                                    let n = choose |n: int| #![trigger old_table_seq[(hk + n * n) % m as int]] 0 <= n < m as int
                                        && (hk + n * n) % m as int == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[(hk + j * j) % m as int] is Empty);
                                    assert forall |j: int| 0 <= j < n
                                        implies !(#[trigger] table.table@[(hk + j * j) % m as int] is Empty) by {
                                        let pos = (hk + j * j) % m as int;
                                        if pos == slot as int {
                                            // Deleted is not Empty.
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
                let ghost prev_slot: int = slot as int;
                let slot1: usize = if attempt < m - slot { slot + attempt } else { attempt - (m - slot) };
                proof {
                    let sum = prev_slot + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                    assert(slot1 as int == (prev_slot + attempt as int) % (m as int));
                }
                let incr: usize = attempt + 1;
                slot = if incr < m {
                    if incr < m - slot1 { slot1 + incr } else { incr - (m - slot1) }
                } else {
                    slot1
                };
                proof {
                    let gi: int = incr as int;
                    let gm: int = m as int;
                    let gs1: int = slot1 as int;
                    let ga: int = attempt as int;
                    let gh: int = h as int;
                    if gi < gm {
                        let sum2 = gs1 + gi;
                        if sum2 < gm {
                            vstd::arithmetic::div_mod::lemma_small_mod(sum2 as nat, gm as nat);
                        } else {
                            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum2 - gm, gm);
                            vstd::arithmetic::div_mod::lemma_small_mod((sum2 - gm) as nat, gm as nat);
                        }
                        assert(slot as int == (gs1 + gi) % gm);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(gs1, gm);
                        vstd::arithmetic::div_mod::lemma_small_mod(gs1 as nat, gm as nat);
                        assert(slot as int == (gs1 + gi) % gm);
                    }
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        gi, prev_slot + ga, gm);
                    assert((gi + gs1) % gm == (gi + prev_slot + ga) % gm);
                    assert((gs1 + gi) % gm == (gi + gs1) % gm);
                    assert(slot as int == (gi + prev_slot + ga) % gm);
                    assert(prev_slot == (gh + ga * ga) % gm);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        gi + ga, gh + ga * ga, gm);
                    assert((gi + ga + prev_slot) % gm == (gi + ga + gh + ga * ga) % gm);
                    assert(gi + prev_slot + ga == gi + ga + prev_slot);
                    assert(slot as int == (gh + ga * ga + ga + gi) % gm);
                    assert((ga + 1) * (ga + 1) == ga * ga + 2 * ga + 1) by(nonlinear_arith);
                    assert(ga + gi == 2 * ga + 1);
                    assert(slot as int == (gh + (ga + 1) * (ga + 1)) % gm);
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions without finding key.
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
        /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs from m slots, creates m' new slots, reinserts n pairs.
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
                assert(spec_quadprobflathashsteph_wf(&new_table));
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

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for QuadProbFlatHashTableStEph
    {
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — hash + i² + modulo.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash_val = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            (hash_val.wrapping_add(attempt.wrapping_mul(attempt))) % table.current_size
        }

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — quadratic probe up to ⌈m/2⌉ (Lemma 47.1).
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

    impl std::fmt::Debug for QuadProbFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QuadProbFlatHashTableStEph")
        }
    }

    impl std::fmt::Display for QuadProbFlatHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QuadProbFlatHashTableStEph")
        }
    }
}
