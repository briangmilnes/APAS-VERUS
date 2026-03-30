//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Linear Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses linear probing for open addressing collision resolution.

pub mod LinProbFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!: spec_linprobflathashsteph_wf)
    // 7. proof fns (inside verus!: spec_count_empties, lemma_*_empties, lemma_probe_mod_identity)
    // 9. impls (inside verus!)

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Concurrency::diverge;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger, lemma_reveal_view_injective};

    verus! {

    // 3. broadcast use
    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

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

    // 7. proof fns

    /// Counts the number of Empty entries in a flat hash table sequence.
    pub open spec fn spec_count_empties<Key, Value>(
        table: Seq<FlatEntry<Key, Value>>,
    ) -> int
        decreases table.len(),
    {
        if table.len() == 0 { 0 }
        else if table.last() is Empty { spec_count_empties(table.drop_last()) + 1 }
        else { spec_count_empties(table.drop_last()) }
    }

    /// An all-Empty sequence has empties count equal to its length.
    pub proof fn lemma_all_empties_count<Key, Value>(table: Seq<FlatEntry<Key, Value>>)
        requires forall |j: int| 0 <= j < table.len() ==> (#[trigger] table[j]) is Empty,
        ensures spec_count_empties(table) == table.len(),
        decreases table.len(),
    {
        if table.len() > 0 {
            assert(table.last() == table[table.len() - 1]);
            assert forall |j: int| 0 <= j < table.drop_last().len()
                implies (#[trigger] table.drop_last()[j]) is Empty by {
                assert(table.drop_last()[j] == table[j]);
            }
            lemma_all_empties_count::<Key, Value>(table.drop_last());
        }
    }

    /// If empties count > 0, there exists an Empty slot.
    pub proof fn lemma_empties_positive_implies_exists_empty<Key, Value>(
        table: Seq<FlatEntry<Key, Value>>,
    )
        requires spec_count_empties(table) > 0,
        ensures exists |j: int| 0 <= j < table.len() && (#[trigger] table[j]) is Empty,
        decreases table.len(),
    {
        if table.last() is Empty {
            assert(table[table.len() - 1] is Empty);
        } else {
            lemma_empties_positive_implies_exists_empty::<Key, Value>(table.drop_last());
            let j = choose |j: int| 0 <= j < table.drop_last().len()
                && (#[trigger] table.drop_last()[j]) is Empty;
            assert(table[j] == table.drop_last()[j]);
        }
    }

    /// Changing one slot decreases empties by at most 1.
    pub proof fn lemma_one_slot_change_empties<Key, Value>(
        old_table: Seq<FlatEntry<Key, Value>>,
        new_table: Seq<FlatEntry<Key, Value>>,
        s: int,
    )
        requires
            old_table.len() == new_table.len(),
            0 <= s < old_table.len(),
            forall |j: int| 0 <= j < old_table.len() && j != s
                ==> #[trigger] new_table[j] == old_table[j],
        ensures
            spec_count_empties(new_table) >= spec_count_empties(old_table) - 1,
        decreases old_table.len(),
    {
        if old_table.len() == 1 {
            assert(old_table.drop_last().len() == 0);
            assert(spec_count_empties::<Key, Value>(old_table.drop_last()) == 0);
            assert(new_table.drop_last().len() == 0);
            assert(spec_count_empties::<Key, Value>(new_table.drop_last()) == 0);
        } else if s == old_table.len() - 1 {
            assert forall |j: int| 0 <= j < old_table.drop_last().len()
                implies #[trigger] new_table.drop_last()[j] == old_table.drop_last()[j] by {
                assert(new_table.drop_last()[j] == new_table[j]);
                assert(old_table.drop_last()[j] == old_table[j]);
            }
            assert(new_table.drop_last() =~= old_table.drop_last());
        } else {
            assert(new_table.last() == old_table.last());
            assert(s < old_table.drop_last().len());
            assert forall |j: int| 0 <= j < old_table.drop_last().len() && j != s
                implies #[trigger] new_table.drop_last()[j] == old_table.drop_last()[j] by {
                assert(new_table.drop_last()[j] == new_table[j]);
                assert(old_table.drop_last()[j] == old_table[j]);
            }
            lemma_one_slot_change_empties::<Key, Value>(
                old_table.drop_last(), new_table.drop_last(), s);
        }
    }

    /// Modular probe identity: (h + (j - h + m) % m) % m == j for 0 <= h, j < m.
    proof fn lemma_probe_mod_identity(h: int, j: int, m: int)
        requires 0 <= h < m, 0 <= j < m, m > 0,
        ensures (h + (j - h + m) % m) % m == j,
    {
        if j >= h {
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(j - h, m);
            vstd::arithmetic::div_mod::lemma_small_mod((j - h) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_small_mod(j as nat, m as nat);
        } else {
            vstd::arithmetic::div_mod::lemma_small_mod((j - h + m) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(j, m);
            vstd::arithmetic::div_mod::lemma_small_mod(j as nat, m as nat);
        }
    }

    // 9. impls

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for LinProbFlatHashTableStEph
    {
        open spec fn spec_impl_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
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

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — linear probe find_slot then set.
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
                proof {
                    let sum = h as int + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                    assert(slot as int == (h as int + attempt as int) % (m as int));
                }
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, _v) => {
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, &key);
                        if eq {
                            // Overwrite existing key at this slot.
                            let ghost old_table_seq = table.table@;
                            table.table.set(slot, FlatEntry::Occupied(key, value));
                            proof {
                                lemma_reveal_view_injective::<Key>();
                                assert(spec_flat_has_key(old_table_seq[slot as int], key));
                                // No other slot has key (old wf no-dup).
                                assert forall |j: int| 0 <= j < old_table_seq.len() && j != slot as int
                                    implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(key) by {
                                    if spec_flat_has_key(old_table_seq[j], key) {
                                        assert(spec_flat_has_key(old_table_seq[slot as int], key));
                                    }
                                }
                                // Map update: new entry = {key→value}, old entry = {key→old_v}.
                                let new_entry = FlatEntry::<Key, Value>::Occupied(key, value);
                                assert(new_entry.spec_entry_to_map() =~= Map::<Key, Value>::empty().insert(key, value));
                                assert(new_entry.spec_entry_to_map() =~=
                                    old_table_seq[slot as int].spec_entry_to_map().insert(key, value));
                                lemma_table_to_map_update_insert::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, new_entry, key, value);
                                // Wf preservation: same key at same slot, Occupied→Occupied.
                                assert forall |i: int, j: int, k: Key|
                                    0 <= i < m as int && 0 <= j < m as int && i != j
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies !#[trigger] spec_flat_has_key(table.table@[j], k) by {
                                    if i == slot as int {
                                        // table[slot] has key (only). For j != slot, old has no key at j.
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
                                // Probe chain: Occupied→Occupied at same slot, no change to Empty/non-Empty.
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                            ==> !(#[trigger] table.table@[(hk + d) % m as int] is Empty)
                                    }) by {
                                    // table[i] == old[i] or i == slot (same key).
                                    // Either way, key existed at i in old table too.
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
                                    assert forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                        implies !(#[trigger] table.table@[(hk + d) % m as int] is Empty) by {
                                        let pos = (hk + d) % m as int;
                                        if pos == slot as int {
                                            // Was Occupied, now Occupied. Not Empty.
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
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
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                    FlatEntry::Empty => {
                        // New key insertion at empty slot.
                        let ghost old_table_seq = table.table@;
                        table.table.set(slot, FlatEntry::Occupied(key, value));
                        if table.num_elements < usize::MAX {
                            table.num_elements = table.num_elements + 1;
                        }
                        proof {
                            // Key was not in the table (same proof as lookup not-found).
                            assert(old_table_seq[slot as int] is Empty);
                            assert forall |j: int| 0 <= j < old_table_seq.len()
                                implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(key) by {
                                if spec_flat_has_key(old_table_seq[j], key) {
                                    let dj = (j - h as int + m as int) % (m as int);
                                    lemma_probe_mod_identity(h as int, j, m as int);
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(old_table_seq, key);
                            // Map update: empty entry → {key→value}.
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
                                        // Key wasn't anywhere in old table (proven via entry_to_map).
                                        assert(!old_table_seq[j].spec_entry_to_map().dom().contains(key));
                                        if spec_flat_has_key(old_table_seq[j], key) {
                                            // Occupied(key,_) entry_to_map contains key. Contradiction.
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
                                                // Occupied(key,_) entry_to_map contains key. Contradiction.
                                            }
                                        }
                                    } else {
                                        assert(table.table@[j] == old_table_seq[j]);
                                    }
                                }
                            }
                            // Wf: probe chain — key at slot via attempt steps, slot was Empty→Occupied.
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
                                    // New key at slot. hk == h. Probe path 0..attempt was not Empty.
                                    assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                    if k == key {
                                        assert(hk == h as int);
                                        assert forall |d: int| 0 <= d < (slot as int - h as int + m as int) % m as int
                                            implies !(#[trigger] table.table@[(h as int + d) % m as int] is Empty) by {
                                            // Prove (slot - h + m) % m == attempt via case split.
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
                                            assert((slot as int - h as int + m as int) % m as int == attempt as int);
                                            // Position (h+d)%m: d < attempt, so this is a prior probe position.
                                            // Prior positions were not Empty (invariant).
                                            let pos = (h as int + d) % m as int;
                                            // table.table@[pos] was not changed (pos != slot since d < attempt).
                                            if pos == slot as int {
                                                // pos == slot means (h+d)%m == (h+attempt)%m, so d == attempt mod m.
                                                // But 0 <= d < attempt and 0 <= attempt < m, so d != attempt. Contradiction.
                                                vstd::arithmetic::div_mod::lemma_small_mod(d as nat, m as nat);
                                                vstd::arithmetic::div_mod::lemma_small_mod(attempt as nat, m as nat);
                                            }
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                } else {
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    // Old probe chain for k: no Empty on path from hk to i.
                                    assert forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                        implies !(#[trigger] table.table@[(hk + d) % m as int] is Empty) by {
                                        let pos = (hk + d) % m as int;
                                        if pos == slot as int {
                                            // Was Empty, now Occupied. Not Empty.
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
                        // Skip Deleted: continue probing to avoid duplicates.
                        proof {
                            assert(!spec_flat_has_key(table.table@[slot as int], key));
                            assert(!(table.table@[slot as int] is Empty));
                        }
                    }
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions — unreachable given spec_has_insert_capacity.
            // Linear probing visits all m slots; loop invariant says none was Empty.
            // But the precondition guarantees an Empty slot exists. Contradiction.
            proof {
                assert forall |j: int| 0 <= j < m as int
                    implies !(#[trigger] table.table@[j] is Empty) by {
                    lemma_probe_mod_identity(h as int, j, m as int);
                }
                assert(table.table@.len() == m as int);
                assert(table.table@ =~= old(table).table@);
                assert(false);
            }
            diverge::<()>();
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
                    spec_linprobflathashsteph_wf(table),
                    // Probe positions 0..attempt don't have the key.
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(table.table@[(h as int + d) % (m as int)], *key),
                    // Probe positions 0..attempt are not Empty.
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[(h as int + d) % (m as int)] is Empty),
                decreases m - attempt,
            {
                // Compute slot = (h + attempt) % m without overflow.
                let slot: usize = if attempt < m - h { h + attempt } else { attempt - (m - h) };
                proof {
                    let sum = h as int + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                    assert(slot as int == (h as int + attempt as int) % (m as int));
                }
                let entry = table.table[slot].clone();
                match entry {
                    FlatEntry::Occupied(k, v) => {
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, key);
                        if eq {
                            proof {
                                lemma_reveal_view_injective::<Key>();
                                assert(spec_flat_has_key(table.table@[slot as int], *key));
                                // No other slot has this key: contradiction via wf no-dup multi-trigger.
                                assert forall |j: int| 0 <= j < table.table@.len() && j != slot as int
                                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(table.table@[j], *key) {
                                        // Both triggers in E-graph: table[slot] and table[j].
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
                                    let dj = (j - h as int + m as int) % (m as int);
                                    lemma_probe_mod_identity(h as int, j, m as int);
                                    if dj > attempt as int {
                                        // wf: probe position attempt non-Empty, but slot is Empty.
                                    } else if dj < attempt as int {
                                        // Invariant: !spec_flat_has_key at (h+dj)%m == j.
                                    }
                                    // dj == attempt: j == slot which is Empty, can't have key.
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
                attempt = attempt + 1;
            }
            // Exhausted all m positions without finding key.
            proof {
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

        /// - APAS: Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Claude-Opus-4.6: Work O(1/(1−α)) expected, Span O(1/(1−α)) — linear probe until found or empty, then tombstone.
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
                proof {
                    let sum = h as int + attempt as int;
                    if sum < m as int {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, m as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - m as int, m as int);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - m as int) as nat, m as nat);
                    }
                    assert(slot as int == (h as int + attempt as int) % (m as int));
                }
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
                                lemma_reveal_view_injective::<Key>();
                                assert(spec_flat_has_key(old_table_seq[slot as int], *key));
                                // No other slot has *key (old wf no-dup).
                                assert forall |j: int| 0 <= j < old_table_seq.len() && j != slot as int
                                    implies !#[trigger] old_table_seq[j].spec_entry_to_map().dom().contains(*key) by {
                                    if spec_flat_has_key(old_table_seq[j], *key) {
                                        assert(spec_flat_has_key(old_table_seq[slot as int], *key));
                                    }
                                }
                                // Map update: Deleted has empty map = old entry map with key removed.
                                let new_entry = FlatEntry::<Key, Value>::Deleted;
                                assert(new_entry.spec_entry_to_map() =~= Map::<Key, Value>::empty());
                                assert(old_table_seq[slot as int].spec_entry_to_map().dom().contains(*key));
                                assert(new_entry.spec_entry_to_map() =~=
                                    old_table_seq[slot as int].spec_entry_to_map().remove(*key));
                                lemma_table_to_map_update_remove::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, new_entry, *key);
                                // Old table contained *key.
                                lemma_table_to_map_unique_entry_value::<Key, Value, FlatEntry<Key, Value>>(
                                    old_table_seq, slot as int, *key);
                                // Wf preservation: no-dup.
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
                                // Wf preservation: probe chain integrity.
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                            ==> !(#[trigger] table.table@[(hk + d) % m as int] is Empty)
                                    }) by {
                                    assert(i != slot as int);
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    assert forall |d: int| 0 <= d < (i - hk + m as int) % m as int
                                        implies !(#[trigger] table.table@[(hk + d) % m as int] is Empty) by {
                                        let pos = (hk + d) % m as int;
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
                                    let dj = (j - h as int + m as int) % (m as int);
                                    lemma_probe_mod_identity(h as int, j, m as int);
                                }
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                            assert(table@ =~= old(table)@.remove(*key));
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
                attempt = attempt + 1;
            }
            proof {
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                        let dj = (j - h as int + m as int) % (m as int);
                        lemma_probe_mod_identity(h as int, j, m as int);
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                assert(table@ =~= old(table)@.remove(*key));
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
                    pairs@.len() <= i as int,
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
                        // Help Z3 unfold spec_seq_pairs_to_map after push.
                        assert(pairs@.drop_last() =~= old_pairs);
                        assert(pairs@.last() == (k, v));
                        assert(spec_seq_pairs_to_map(pairs@)
                            =~= spec_seq_pairs_to_map(old_pairs).insert(k, v));
                        assert(spec_seq_pairs_to_map(pairs@) =~= old_map.insert(k, v));
                        // Connect to spec_table_to_map via union_prefer_right.
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
                assert(spec_linprobflathashsteph_wf(&new_table));
            }

            // Phase 3: reinsert all pairs.
            let mut j: usize = 0;
            proof {
                assert forall |idx: int| 0 <= idx < new_table.table@.len()
                    implies (#[trigger] new_table.table@[idx]) is Empty by {}
                lemma_all_empties_count::<Key, Value>(new_table.table@);
            }
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
                    pairs@.len() <= table.current_size as int,
                    new_size as int > table.current_size as int,
                    spec_count_empties(new_table.table@) >= (new_size - j) as int,
                    obeys_feq_clone::<Key>(),
                    obeys_feq_clone::<Value>(),
                decreases pairs.len() - j,
            {
                let key = clone_elem(&pairs[j].0);
                let value = clone_elem(&pairs[j].1);
                proof {
                    // Prove spec_has_insert_capacity: empties > 0 implies exists Empty slot.
                    assert((new_size - j) as int > 0int) by {
                        assert(j as int <= pairs@.len());
                        assert(pairs@.len() <= table.current_size as int);
                        assert(new_size as int > table.current_size as int);
                    }
                    lemma_empties_positive_implies_exists_empty::<Key, Value>(
                        new_table.table@);
                }
                let ghost old_new_table_seq = new_table.table@;
                Self::insert(&mut new_table, key, value);
                proof {
                    // Use exists |s| from insert ensures to maintain empties invariant.
                    let s = choose |s: int| #[trigger] spec_other_slots_preserved(
                        old_new_table_seq, new_table.table@, s);
                    lemma_one_slot_change_empties::<Key, Value>(
                        old_new_table_seq, new_table.table@, s);
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
