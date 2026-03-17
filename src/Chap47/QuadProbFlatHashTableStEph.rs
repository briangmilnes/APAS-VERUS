//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses quadratic probing for open addressing collision resolution.

pub mod QuadProbFlatHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 6. spec fns (inside verus!)
    // 9. impls (inside verus!)
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

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
                let slot = quadratic_probe(&table.hash_fn, &key, table.current_size, attempt, table.spec_hash);
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
                        let eq = k == *key;
                        proof { accept(eq == spec_flat_has_key(table.table@[slot as int], *key)); } // Eq bridge.
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
                let slot = quadratic_probe(&table.hash_fn, key, table.current_size, attempt, table.spec_hash);
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
