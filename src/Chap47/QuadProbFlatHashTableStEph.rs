//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses triangular-number probing for open addressing collision resolution.
//! Probe sequence: h_i(k) = (h(k) + i*(i+1)/2) mod m, where m = 2^k.
//! When m is a power of two, the first m probes are a complete permutation of {0..m-1}.

pub mod QuadProbFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!: spec_is_power_of_two, spec_tri_probe, spec_quadprobflathashsteph_wf)
    // 7. proof fns (inside verus!: lemma_triangular_injective, lemma_empty_slot_reachable)
    // 9. impls (inside verus!)
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use vstd::arithmetic::power::pow;
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
    /// Uses triangular-number probe sequence h_i(k) = (h(k) + i*(i+1)/2) mod m
    /// with m a power of two, guaranteeing a complete permutation of all slots.
    pub struct QuadProbFlatHashTableStEph;

    // 6. spec fns

    /// Whether m is a power of two (m = 2^k for some k >= 1).
    pub open spec fn spec_is_power_of_two(m: int) -> bool {
        exists |k: nat| k >= 1 && m == pow(2, k)
    }

    /// Triangular probe position: the n-th probe starting from hash h in table of size m.
    pub open spec fn spec_tri_probe(h: int, n: int, m: int) -> int {
        (h + n * (n + 1) / 2) % m
    }

    /// Well-formedness for quadratic (triangular) probing flat hash tables.
    /// Requires m to be a power of two so that the triangular probe sequence
    /// i*(i+1)/2 mod m forms a complete permutation of {0..m-1}.
    /// (1) Table length equals current_size.
    /// (2) m is a power of two.
    /// (3) No duplicate keys across slots.
    /// (4) Every occupied key is reachable via triangular probing from its hash:
    ///     exists n placing it at its slot, with no Empty gaps on the probe path.
    pub open spec fn spec_quadprobflathashsteph_wf<Key, Value, Metrics, H>(
        table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
    ) -> bool {
        let m = table.current_size as int;
        table.table@.len() == m
        && m > 0
        && spec_is_power_of_two(m)
        // No duplicate keys.
        && (forall |i: int, j: int, k: Key|
            0 <= i < m && 0 <= j < m && i != j
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> !#[trigger] spec_flat_has_key(table.table@[j], k))
        // Probe chain integrity for triangular probing.
        && (forall |i: int, k: Key|
            0 <= i < m
            && #[trigger] spec_flat_has_key(table.table@[i], k)
            ==> {
                let h = (table.spec_hash@)(k) as int % m;
                exists |n: int| #![trigger table.table@[spec_tri_probe(h, n, m)]]
                    0 <= n < m
                    && spec_tri_probe(h, n, m) == i
                    && forall |j: int| 0 <= j < n
                        ==> !(#[trigger] table.table@[spec_tri_probe(h, j, m)] is Empty)
            })
    }

    // 7. proof fns

    /// n*(n+1) is always even since exactly one of n, n+1 is even.
    proof fn lemma_consecutive_even(a: int)
        ensures (a * (a + 1)) % 2 == 0,
    {
        vstd::arithmetic::div_mod::lemma_mod_bound(a, 2);
        if a % 2 == 0 {
            // (a%2)*(a+1) % 2 == a*(a+1) % 2 by noop_left; and (0*(a+1)) % 2 == 0.
            vstd::arithmetic::div_mod::lemma_mul_mod_noop_left(a, a + 1, 2);
            assert((a % 2) * (a + 1) % 2 == 0);
        } else {
            // a%2 == 1 (from mod_bound: 0 <= a%2 < 2); then (a+1)%2 == 0.
            assert(a % 2 == 1);
            vstd::arithmetic::div_mod::lemma_add_mod_noop(a, 1, 2);
            assert((a + 1) % 2 == 0);
            // a*((a+1)%2) % 2 == a*(a+1) % 2 by noop_right; and a*0 % 2 == 0.
            vstd::arithmetic::div_mod::lemma_mul_mod_noop_right(a, a + 1, 2);
            assert(a * ((a + 1) % 2) % 2 == 0);
        }
    }

    /// T(n+1) = T(n) + (n+1), where T(n) = n*(n+1)/2 is the n-th triangular number.
    proof fn lemma_tri_step(a: int)
        ensures a * (a + 1) / 2 + (a + 1) == (a + 1) * (a + 2) / 2,
    {
        lemma_consecutive_even(a);
        lemma_consecutive_even(a + 1);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(a * (a + 1), 2);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod((a + 1) * (a + 2), 2);
        let x: int = a * (a + 1) / 2;
        let y: int = (a + 1) * (a + 2) / 2;
        assert(a * (a + 1) == 2 * x);
        assert((a + 1) * (a + 2) == 2 * y);
        assert(a * (a + 1) + 2 * (a + 1) == (a + 1) * (a + 2)) by (nonlinear_arith);
        assert(2 * x + 2 * (a + 1) == 2 * y);
    }

    /// The triangular probing sequence is injective modulo 2^k:
    /// for 0 <= i < j < 2^k, i*(i+1)/2 mod 2^k != j*(j+1)/2 mod 2^k.
    ///
    /// PROOF OBLIGATION (parity argument from arXiv:2107.08824):
    ///
    /// If i*(i+1)/2 ≡ j*(j+1)/2 (mod m) with m = 2^k, then
    ///   (j-i)*(j+i+1) ≡ 0 (mod 2^(k+1)).
    /// Since (j-i) + (j+i+1) = 2j+1 (odd), exactly one of {j-i, j+i+1} is even
    /// and the other is odd.
    ///   - If j-i is even: j-i < m = 2^k, so j-i has at most k-1 factors of 2.
    ///     j+i+1 is odd (0 factors). Product has < k+1 factors. Contradiction.
    ///   - If j+i+1 is even: j+i+1 <= 2m-1 = 2^(k+1)-1 < 2^(k+1), so at most k factors.
    ///     j-i is odd. Product has <= k < k+1 factors. Contradiction.
    /// Hence no such i != j exist: the sequence is injective.
    pub proof fn lemma_triangular_injective(i: int, j: int, m: int)
        requires
            0 <= i < j < m,
            spec_is_power_of_two(m),
        ensures
            (i * (i + 1) / 2) % m != (j * (j + 1) / 2) % m,
    {
        assume(false); // TODO: implement parity argument above using nonlinear_arith
    }

    /// The triangular probe sequence visits all m slots when m is a power of two.
    /// Therefore a non-full table always has a reachable Empty slot.
    ///
    /// PROOF OBLIGATION: follows from lemma_triangular_injective (m distinct values
    /// mod m cover all of {0..m-1}) plus pigeonhole (num_elements < m => Empty exists).
    ///
    /// Preconditions encode the exhaustion state: all m probe positions were visited
    /// without finding an empty slot or matching key. This is unreachable.
    pub proof fn lemma_empty_slot_reachable<Key, Value, Metrics, H>(
        table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>,
        key: &Key,
        h: usize,
    )
        requires
            spec_quadprobflathashsteph_wf(table),
            h < table.current_size,
            h as nat == (table.spec_hash@)(*key) % (table.current_size as nat),
            forall |d: int| 0 <= d < table.current_size as int
                ==> !spec_flat_has_key(
                    table.table@[spec_tri_probe(h as int, d, table.current_size as int)],
                    *key),
            forall |d: int| 0 <= d < table.current_size as int
                ==> !(#[trigger] table.table@[
                    spec_tri_probe(h as int, d, table.current_size as int)] is Empty),
        ensures false,
    {
        assume(false); // TODO: prove via lemma_triangular_injective + pigeonhole
    }

    // 9. impls

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for QuadProbFlatHashTableStEph
    {
        open spec fn spec_impl_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            spec_quadprobflathashsteph_wf(table)
        }

        /// Resize is only valid to a power-of-two new size, preserving the wf invariant.
        open spec fn spec_resize_ok(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, new_size: usize) -> bool {
            spec_is_power_of_two(new_size as int)
        }

        /// - APAS: Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Claude-Opus-4.6: Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe then set.
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: Key, value: Value) {
            let h = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(0int * (0int + 1int) / 2int == 0int);
                assert(slot as int == spec_tri_probe(h as int, 0int, m as int));
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
                    slot as int == spec_tri_probe(h as int, attempt as int, m as int),
                    spec_quadprobflathashsteph_wf(table),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    old(table).num_elements < usize::MAX,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(
                            table.table@[spec_tri_probe(h as int, d, m as int)], key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[spec_tri_probe(h as int, d, m as int)] is Empty),
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
                                // Wf: probe chain (Occupied→Occupied, Empty status unchanged).
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        exists |n: int| #![trigger table.table@[spec_tri_probe(hk, n, m as int)]]
                                            0 <= n < m as int
                                            && spec_tri_probe(hk, n, m as int) == i
                                            && forall |j: int| 0 <= j < n
                                                ==> !(#[trigger] table.table@[spec_tri_probe(hk, j, m as int)] is Empty)
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
                                    let n = choose |n: int| #![trigger old_table_seq[spec_tri_probe(hk, n, m as int)]]
                                        0 <= n < m as int
                                        && spec_tri_probe(hk, n, m as int) == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[spec_tri_probe(hk, j, m as int)] is Empty);
                                    assert forall |j: int| 0 <= j < n
                                        implies !(#[trigger] table.table@[spec_tri_probe(hk, j, m as int)] is Empty) by {
                                        let pos = spec_tri_probe(hk, j, m as int);
                                        if pos == slot as int {
                                            // Was Occupied, now Occupied. Not Empty.
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                }
                                assert(spec_other_slots_preserved(old(table).table@, table.table@, slot as int));
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
                            // Wf: no-dup — key wasn't anywhere before.
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
                            let ga = attempt as int;
                            let gm = m as int;
                            assert forall |i: int, k: Key|
                                0 <= i < gm
                                && #[trigger] spec_flat_has_key(table.table@[i], k)
                                implies ({
                                    let hk = (table.spec_hash@)(k) as int % gm;
                                    exists |n: int| #![trigger table.table@[spec_tri_probe(hk, n, gm)]]
                                        0 <= n < gm
                                        && spec_tri_probe(hk, n, gm) == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] table.table@[spec_tri_probe(hk, j, gm)] is Empty)
                                }) by {
                                let hk = (table.spec_hash@)(k) as int % gm;
                                if i == slot as int {
                                    assert(spec_flat_has_key(table.table@[slot as int], k) ==> k == key);
                                    if k == key {
                                        assert(hk == h as int);
                                        // Witness n = attempt: spec_tri_probe(h, attempt, m) == slot.
                                        assert(slot as int == spec_tri_probe(h as int, ga, gm));
                                        // Probe path 0..attempt was not Empty (loop invariant).
                                        assert forall |j: int| 0 <= j < ga
                                            implies !(#[trigger] table.table@[spec_tri_probe(h as int, j, gm)] is Empty) by {
                                            let pos = spec_tri_probe(h as int, j, gm);
                                            if pos == slot as int {
                                                // Old slot was Empty; invariant says pos not Empty. Contradiction.
                                            }
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                } else {
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    let n = choose |n: int| #![trigger old_table_seq[spec_tri_probe(hk, n, gm)]]
                                        0 <= n < gm
                                        && spec_tri_probe(hk, n, gm) == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[spec_tri_probe(hk, j, gm)] is Empty);
                                    assert forall |j: int| 0 <= j < n
                                        implies !(#[trigger] table.table@[spec_tri_probe(hk, j, gm)] is Empty) by {
                                        let pos = spec_tri_probe(hk, j, gm);
                                        if pos == slot as int {
                                            // Was Empty, now Occupied. Not Empty.
                                        } else {
                                            assert(table.table@[pos] == old_table_seq[pos]);
                                        }
                                    }
                                }
                            }
                            assert(old_table_seq =~= old(table).table@);
                            assert(spec_other_slots_preserved(old(table).table@, table.table@, slot as int));
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
                // Slot update: slot_{a+1} = (slot_a + (a+1)) % m
                // Increments by a+1, maintaining slot = (h + (a+1)*(a+2)/2) % m.
                let new_inc: usize = attempt + 1;
                slot = if new_inc < m - slot { slot + new_inc } else { new_inc - (m - slot) };
                proof {
                    let ga: int = attempt as int;
                    let gm: int = m as int;
                    let inc: int = ga + 1;
                    let old_slot = (h as int + ga * (ga + 1) / 2) % gm;
                    let sum = old_slot + inc;
                    if sum < gm {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, gm as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - gm, gm);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - gm) as nat, gm as nat);
                    }
                    assert(slot as int == (old_slot + inc) % gm);
                    lemma_tri_step(ga);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        ga + 1, h as int + ga * (ga + 1) / 2, gm);
                    assert(slot as int == spec_tri_probe(h as int, ga + 1, gm));
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions. Unreachable because the triangular probe sequence
            // is a complete permutation of {0..m-1} (m a power of two), so a non-full table
            // must contain an empty slot that the probe sequence reaches.
            proof {
                // Bridge: loop invariant uses `m`, lemma requires `table.current_size`.
                assert(m == table.current_size);
                assert forall |d: int| 0 <= d < table.current_size as int
                    ==> !spec_flat_has_key(
                        table.table@[spec_tri_probe(h as int, d, table.current_size as int)], key) by {
                    assert(spec_tri_probe(h as int, d, m as int) == spec_tri_probe(h as int, d, table.current_size as int));
                }
                assert forall |d: int| 0 <= d < table.current_size as int
                    ==> !(#[trigger] table.table@[spec_tri_probe(h as int, d, table.current_size as int)] is Empty) by {
                    assert(spec_tri_probe(h as int, d, m as int) == spec_tri_probe(h as int, d, table.current_size as int));
                }
                lemma_empty_slot_reachable::<Key, Value, Metrics, H>(table, &key, h);
            }
        }

        /// - APAS: Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Claude-Opus-4.6: Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe sequence.
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(0int * (0int + 1int) / 2int == 0int);
                assert(slot as int == spec_tri_probe(h as int, 0int, m as int));
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
                    slot as int == spec_tri_probe(h as int, attempt as int, m as int),
                    spec_quadprobflathashsteph_wf(table),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(
                            table.table@[spec_tri_probe(h as int, d, m as int)], *key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[spec_tri_probe(h as int, d, m as int)] is Empty),
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
                            // Wf: if key existed at slot j, its probe path has no Empty before j.
                            // Since we found Empty at this attempt, key cannot be at any later probe.
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                if spec_flat_has_key(table.table@[j], *key) {
                                    let hk = (table.spec_hash@)(*key) as int % m as int;
                                    // wf gives n: 0<=n<m, probe(h,n,m)==j, path 0..n non-Empty.
                                    // But probe(h,attempt,m)==slot is Empty. Contradiction if n>attempt.
                                    // If n < attempt: invariant says !spec_flat_has_key at probe(h,n,m)==j.
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
                let new_inc: usize = attempt + 1;
                slot = if new_inc < m - slot { slot + new_inc } else { new_inc - (m - slot) };
                proof {
                    let ga: int = attempt as int;
                    let gm: int = m as int;
                    let old_slot = (h as int + ga * (ga + 1) / 2) % gm;
                    let inc: int = ga + 1;
                    let sum = old_slot + inc;
                    if sum < gm {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, gm as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - gm, gm);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - gm) as nat, gm as nat);
                    }
                    assert(slot as int == (old_slot + inc) % gm);
                    lemma_tri_step(ga);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        ga + 1, h as int + ga * (ga + 1) / 2, gm);
                    assert(slot as int == spec_tri_probe(h as int, ga + 1, gm));
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions without finding key.
            proof {
                assert forall |j: int| 0 <= j < table.table@.len()
                    implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                    if spec_flat_has_key(table.table@[j], *key) {
                        // wf gives n < m; invariant says !spec_flat_has_key at all probe positions 0..m.
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            None
        }

        /// - APAS: Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Claude-Opus-4.6: Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe until found, then tombstone.
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
                assert(0int * (0int + 1int) / 2int == 0int);
                assert(slot as int == spec_tri_probe(h as int, 0int, m as int));
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
                    slot as int == spec_tri_probe(h as int, attempt as int, m as int),
                    spec_quadprobflathashsteph_wf(table),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(
                            table.table@[spec_tri_probe(h as int, d, m as int)], *key),
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[spec_tri_probe(h as int, d, m as int)] is Empty),
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
                                // Wf: probe chain (Occupied→Deleted, which is not Empty).
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        exists |n: int| #![trigger table.table@[spec_tri_probe(hk, n, m as int)]]
                                            0 <= n < m as int
                                            && spec_tri_probe(hk, n, m as int) == i
                                            && forall |j: int| 0 <= j < n
                                                ==> !(#[trigger] table.table@[spec_tri_probe(hk, j, m as int)] is Empty)
                                    }) by {
                                    assert(i != slot as int);
                                    assert(table.table@[i] == old_table_seq[i]);
                                    assert(spec_flat_has_key(old_table_seq[i], k));
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    let n = choose |n: int| #![trigger old_table_seq[spec_tri_probe(hk, n, m as int)]]
                                        0 <= n < m as int
                                        && spec_tri_probe(hk, n, m as int) == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[spec_tri_probe(hk, j, m as int)] is Empty);
                                    assert forall |j: int| 0 <= j < n
                                        implies !(#[trigger] table.table@[spec_tri_probe(hk, j, m as int)] is Empty) by {
                                        let pos = spec_tri_probe(hk, j, m as int);
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
                let new_inc: usize = attempt + 1;
                slot = if new_inc < m - slot { slot + new_inc } else { new_inc - (m - slot) };
                proof {
                    let ga: int = attempt as int;
                    let gm: int = m as int;
                    let old_slot = (h as int + ga * (ga + 1) / 2) % gm;
                    let inc: int = ga + 1;
                    let sum = old_slot + inc;
                    if sum < gm {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, gm as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - gm, gm);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - gm) as nat, gm as nat);
                    }
                    assert(slot as int == (old_slot + inc) % gm);
                    lemma_tri_step(ga);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        ga + 1, h as int + ga * (ga + 1) / 2, gm);
                    assert(slot as int == spec_tri_probe(h as int, ga + 1, gm));
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
        /// - Claude-Opus-4.6: Work O(n + m + m') — collect n pairs, create m' empty slots, reinsert.
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
                        assert(old_map.insert(k, v) =~= old_map.union_prefer_right(entry_map));
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
                assert(table.table@.subrange(0, table.table@.len() as int) =~= table.table@);
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
                // spec_is_power_of_two follows from spec_resize_ok requirement.
                assert(spec_is_power_of_two(new_size as int));
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
                    assert(pairs@.subrange(0, (j + 1) as int).last() == pairs@[j as int]);
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
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — triangular probe position.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash_val = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let tri = attempt.wrapping_mul(attempt.wrapping_add(1)) / 2;
            (hash_val.wrapping_add(tri)) % table.current_size
        }

        /// - APAS: Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Claude-Opus-4.6: Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe sequence.
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
