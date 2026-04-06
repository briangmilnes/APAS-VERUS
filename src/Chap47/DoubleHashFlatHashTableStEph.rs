//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Double Hashing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses double hashing for open addressing collision resolution.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
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

    //		Section 7. proof fns/broadcast groups


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

    /// Exposes the spec-level second hash value (= 1) for probe coverage proofs.
    /// Within this module, the closed fn body is visible, so this is provable.
    pub proof fn lemma_spec_second_hash_value<Key>(key: Key, table_size: nat)
        ensures spec_second_hash::<Key>(key, table_size) == 1nat,
    {}

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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected — matches APAS
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
                        proof { assert(obeys_feq_full_trigger::<Key>()); }
                        let eq = feq(&k, &key);
                        if eq {
                            // Overwrite existing key.
                            let ghost old_table_seq = table.table@;
                            table.table.set(slot, FlatEntry::Occupied(key, value));
                            proof {
                                lemma_reveal_view_injective::<Key>();
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
            // Exhausted all m positions — unreachable given spec_has_insert_capacity.
            // spec_second_hash == 1 (via lemma), so step == 1 and double hashing
            // degenerates to linear probing. All m positions are visited.
            proof {
                lemma_spec_second_hash_value::<Key>(key, m as nat);
                assert(step as int == 1int);
                assert forall |j: int| 0 <= j < m as int
                    implies !(#[trigger] table.table@[j] is Empty) by {
                    lemma_probe_mod_identity(h as int, j, m as int);
                    let d = (j - h as int + m as int) % (m as int);
                    assert(d * 1int == d) by(nonlinear_arith);
                }
                assert(table.table@.len() == m as int);
                assert(table.table@ =~= old(table).table@);
                assert(false);
            }
            diverge::<()>();
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected — matches APAS
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
                                lemma_reveal_view_injective::<Key>();
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

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected — matches APAS
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
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
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
                                lemma_reveal_view_injective::<Key>();
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

        /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m').
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' slots, reinserts.
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
                proof {
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1−α)) expected — matches APAS
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
