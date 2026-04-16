// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses triangular-number probing for open addressing collision resolution.
//! Probe sequence: h_i(k) = (h(k) + i*(i+1)/2) mod m, where m = 2^k.
//! When m is a power of two, the first m probes are a complete permutation of {0..m-1}.

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

pub mod QuadProbFlatHashTableStEph {

    //		Section 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power::pow;
    #[cfg(verus_keep_ghost)]
    use vstd::set_lib::{set_int_range, lemma_int_range, lemma_subset_equality};
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger, lemma_reveal_view_injective};

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4. type definitions


    /// Quadratic Probing Flat Hash Table implementation.
    /// Uses triangular-number probe sequence h_i(k) = (h(k) + i*(i+1)/2) mod m
    /// with m a power of two, guaranteeing a complete permutation of all slots.
    pub struct QuadProbFlatHashTableStEph;

    //		Section 6. spec fns


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

    //		Section 7. proof fns/broadcast groups


    /// n*(n+1) is always even since exactly one of n, n+1 is even.
    proof fn lemma_consecutive_even(a: int)
        ensures (a * (a + 1)) % 2 == 0,
    {
        vstd::arithmetic::div_mod::lemma_mod_bound(a, 2);
        if a % 2 == 0 {
            // (a%2)*(a+1) % 2 == a*(a+1) % 2 by noop_left; and (0*(a+1)) % 2 == 0.
            vstd::arithmetic::div_mod::lemma_mul_mod_noop_left(a, a + 1, 2);
        } else {
            // a%2 == 1 (from mod_bound: 0 <= a%2 < 2); then (a+1)%2 == 0.
            vstd::arithmetic::div_mod::lemma_add_mod_noop(a, 1, 2);
            // a*((a+1)%2) % 2 == a*(a+1) % 2 by noop_right; and a*0 % 2 == 0.
            vstd::arithmetic::div_mod::lemma_mul_mod_noop_right(a, a + 1, 2);
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
        // Veracity: NEEDED assert
        // Veracity: NEEDED assert
        assert(a * (a + 1) + 2 * (a + 1) == (a + 1) * (a + 2)) by (nonlinear_arith);
    }

    /// If a is odd and 2^n divides a*b, then 2^n divides b.
    proof fn lemma_odd_factor_pow2(a: int, b: int, n: nat)
        requires
            a % 2 != 0,
            n >= 1,
            (a * b) % pow(2, n) == 0,
        ensures
            b % pow(2, n) == 0,
        decreases n,
    {
        vstd::arithmetic::power::lemma_pow1(2int);
        if n == 1 {
            vstd::arithmetic::div_mod::lemma_mul_mod_noop(a, b, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(b, 2);
            if b % 2 == 1 {
            }
        } else {
            let pn1 = pow(2, (n - 1) as nat);
            vstd::arithmetic::power::lemma_pow_positive(2, (n - 1) as nat);
            vstd::arithmetic::power::lemma_pow_adds(2, 1, (n - 1) as nat);
            vstd::arithmetic::mul::lemma_mul_is_commutative(2int, pn1);

            // (a*b) % pn1 == 0 via mod_mod.
            vstd::arithmetic::div_mod::lemma_mod_mod(a * b, pn1, 2);
            vstd::arithmetic::div_mod::lemma_small_mod(0nat, pn1 as nat);

            // IH: b % pn1 == 0.
            lemma_odd_factor_pow2(a, b, (n - 1) as nat);

            // b = pn1 * c.
            let c = b / pn1;
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(b, pn1);

            // a * b = pn1 * (a * c).
            vstd::arithmetic::mul::lemma_mul_is_commutative(a, pn1);
            vstd::arithmetic::mul::lemma_mul_is_associative(pn1, a, c);
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(a * (pn1 * c) == pn1 * (a * c)) by (nonlinear_arith);
            let ac = a * c;

            // Decompose ac = 2*q + r.
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(ac, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(ac, 2);
            let r = ac % 2;
            let q = ac / 2;

            // pn1 * ac = (2*pn1)*q + pn1*r.
            let two_pn1 = 2 * pn1;
            vstd::arithmetic::mul::lemma_mul_is_distributive_add(pn1, 2 * q, r);
            vstd::arithmetic::mul::lemma_mul_is_associative(pn1, 2int, q);
            vstd::arithmetic::mul::lemma_mul_is_commutative(pn1, 2int);

            vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(q, pn1 * r, two_pn1);

            // r must be 0: if r == 1, pn1 % two_pn1 == pn1 > 0.
            if r == 1 {
                vstd::arithmetic::mul::lemma_mul_basics(pn1);
                vstd::arithmetic::div_mod::lemma_small_mod(pn1 as nat, two_pn1 as nat);
            }

            // Since a odd and ac even, c even.
            vstd::arithmetic::div_mod::lemma_mul_mod_noop(a, c, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(c, 2);
            if c % 2 == 1 {
            }

            // c = 2*d, b = pn1 * 2 * d = pow(2,n) * d.
            let d = c / 2;
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(c, 2);
            vstd::arithmetic::mul::lemma_mul_is_associative(pn1, 2int, d);
            vstd::arithmetic::mul::lemma_mul_is_commutative(two_pn1, d);
            vstd::arithmetic::div_mod::lemma_mod_multiples_basic(d, two_pn1);
        }
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
        let k: nat = choose |k: nat| k >= 1 && m == pow(2, k);

        if (i * (i + 1) / 2) % m == (j * (j + 1) / 2) % m {
            let ti = i * (i + 1) / 2;
            let tj = j * (j + 1) / 2;

            // (j-i)*(j+i+1) == j*(j+1) - i*(i+1).
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert((j - i) * (j + i + 1) == j * (j + 1) - i * (i + 1)) by (nonlinear_arith);

            // Both are even (consecutive products).
            lemma_consecutive_even(i);
            lemma_consecutive_even(j);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i * (i + 1), 2);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j * (j + 1), 2);

            // prod = 2 * (tj - ti).
            let prod = (j - i) * (j + i + 1);
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(prod == 2 * (tj - ti)) by (nonlinear_arith)
                requires prod == j * (j + 1) - i * (i + 1),
                         i * (i + 1) == 2 * ti,
                         j * (j + 1) == 2 * tj;

            // tj - ti == m * q, so prod == 2 * m * q.
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(ti, m);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(tj, m);
            vstd::arithmetic::mul::lemma_mul_is_distributive_sub(m, tj / m, ti / m);
            let q = tj / m - ti / m;
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(prod == 2 * (m * q)) by (nonlinear_arith)
                requires prod == 2 * (tj - ti), tj - ti == m * q;

            // 2*m == pow(2, k+1).
            vstd::arithmetic::power::lemma_pow_adds(2, 1, k);
            vstd::arithmetic::power::lemma_pow1(2int);
            let two_m = pow(2, (k + 1) as nat);

            // prod == two_m * q, so prod % two_m == 0.
            vstd::arithmetic::mul::lemma_mul_is_associative(2int, m, q);
            vstd::arithmetic::mul::lemma_mul_is_commutative(two_m, q);
            vstd::arithmetic::div_mod::lemma_mod_multiples_basic(q, two_m);

            // Parity: (j-i) + (j+i+1) = 2j+1 is odd.
            let a = j - i;
            let b = j + i + 1;
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(a + b == 2 * j + 1) by (nonlinear_arith)
                requires a == j - i, b == j + i + 1;
            vstd::arithmetic::div_mod::lemma_add_mod_noop(a, b, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(a, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(b, 2);

            // prod = a * b.
            if a % 2 != 0 {
                // a odd => apply lemma_odd_factor_pow2: two_m | b.
                lemma_odd_factor_pow2(a, b, (k + 1) as nat);
                // b < two_m and b > 0, so b % two_m == b != 0.
                vstd::arithmetic::div_mod::lemma_small_mod(b as nat, two_m as nat);
            } else {
                // b odd => two_m | a.
                vstd::arithmetic::mul::lemma_mul_is_commutative(a, b);
                lemma_odd_factor_pow2(b, a, (k + 1) as nat);
                vstd::arithmetic::power::lemma_pow_positive(2, k);
                vstd::arithmetic::div_mod::lemma_small_mod(a as nat, two_m as nat);
            }
        }
    }

    /// Adding a constant preserves inequality modulo m.
    /// Contrapositive: if (a+x) % m == (a+y) % m with 0 <= a,x,y < m and x != y, false.
    proof fn lemma_mod_add_cancel(a: int, x: int, y: int, m: int)
        requires
            m > 0,
            0 <= a < m,
            0 <= x < m,
            0 <= y < m,
            x != y,
            (a + x) % m == (a + y) % m,
        ensures false,
    {
        if a + x < m && a + y < m {
            vstd::arithmetic::div_mod::lemma_small_mod((a + x) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_small_mod((a + y) as nat, m as nat);
        } else if a + x >= m && a + y >= m {
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(a + x - m, m);
            vstd::arithmetic::div_mod::lemma_small_mod((a + x - m) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(a + y - m, m);
            vstd::arithmetic::div_mod::lemma_small_mod((a + y - m) as nat, m as nat);
        } else if a + x < m {
            vstd::arithmetic::div_mod::lemma_small_mod((a + x) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(a + y - m, m);
            vstd::arithmetic::div_mod::lemma_small_mod((a + y - m) as nat, m as nat);
        } else {
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(a + x - m, m);
            vstd::arithmetic::div_mod::lemma_small_mod((a + x - m) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_small_mod((a + y) as nat, m as nat);
        }
    }

    /// The triangular probe sequence visits all m slots when m is a power of two.
    /// Therefore a table with an Empty slot always has a reachable Empty slot.
    ///
    /// Follows from lemma_triangular_injective (m distinct values mod m cover all of
    /// {0..m-1}) plus the exists-Empty precondition: some slot is Empty, every probe
    /// position is non-Empty, but probes cover all slots — contradiction.
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
            exists |s: int| #![trigger table.table@[s]]
                0 <= s < table.current_size as int && table.table@[s] is Empty,
            forall |d: int| #![trigger spec_tri_probe(h as int, d, table.current_size as int)]
                0 <= d < table.current_size as int
                ==> !spec_flat_has_key(
                    #[trigger] table.table@[spec_tri_probe(h as int, d, table.current_size as int)],
                    *key),
            forall |d: int| 0 <= d < table.current_size as int
                ==> !(#[trigger] table.table@[
                    spec_tri_probe(h as int, d, table.current_size as int)] is Empty),
        ensures false,
    {
        let m = table.current_size as int;
        let hi = h as int;

        // Witness: an Empty slot exists.
        let s = choose |s: int| #![trigger table.table@[s]]
            0 <= s < m && table.table@[s] is Empty;

        // Build the probe sequence and show no_duplicates via injectivity.
        let probes = Seq::new(m as nat, |d: int| spec_tri_probe(hi, d, m));

        // Veracity: NEEDED assert
        // Veracity: NEEDED assert
        assert(probes.no_duplicates()) by {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall |i: int, j: int|
                0 <= i < m && 0 <= j < m && i != j
                implies probes[i] != probes[j] by {
                let ti = i * (i + 1) / 2;
                let tj = j * (j + 1) / 2;
                if i < j {
                    lemma_triangular_injective(i, j, m);
                } else {
                    lemma_triangular_injective(j, i, m);
                }
                // T(i) % m != T(j) % m. Show (h + T(i)) % m != (h + T(j)) % m.
                vstd::arithmetic::div_mod::lemma_add_mod_noop(hi, ti, m);
                vstd::arithmetic::div_mod::lemma_add_mod_noop(hi, tj, m);
                vstd::arithmetic::div_mod::lemma_mod_bound(hi, m);
                vstd::arithmetic::div_mod::lemma_mod_bound(ti, m);
                vstd::arithmetic::div_mod::lemma_mod_bound(tj, m);
                if (hi + ti) % m == (hi + tj) % m {
                    lemma_mod_add_cancel(hi % m, ti % m, tj % m, m);
                }
            }
        };

        // probes.to_set() has m elements.
        probes.unique_seq_to_set();

        // probes.to_set() ⊆ set_int_range(0, m) — all probe values in [0, m).

        // set_int_range(0, m) has cardinality m.
        lemma_int_range(0, m);

        // Both finite, same cardinality, subset — so equal.
        lemma_subset_equality(probes.to_set(), set_int_range(0, m));

        // s ∈ set_int_range(0, m), so s ∈ probes.to_set().
        // Veracity: NEEDED assert
        // Veracity: NEEDED assert
        assert(probes.to_set().contains(s));

        // probes.contains(s) gives a witness d.
        let d_s = choose |d: int| 0 <= d < m && probes[d] == s;

        // But the requires says table.table@[spec_tri_probe(h, d_s, m)] is not Empty.
        // Yet table.table@[s] is Empty. Contradiction.
    }

    /// An all-Empty sequence has empties count equal to its length.
    //		Section 9. impls


    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for QuadProbFlatHashTableStEph
    {
        open spec fn spec_parahashtablesteph_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            spec_quadprobflathashsteph_wf(table)
            && spec_hash_fn_valid::<Key, H>(table.spec_hash@)
        }

        /// Flat tables require at least one Empty slot for insertion.
        open spec fn spec_has_insert_capacity(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
            exists |j: int| #![trigger table.table@[j]]
                0 <= j < table.table@.len() && table.table@[j] is Empty
        }

        /// Resize is only valid to a larger power-of-two new size.
        open spec fn spec_resize_ok(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, new_size: usize) -> bool {
            spec_is_power_of_two(new_size as int)
            && new_size as int > table.current_size as int
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe then set.
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: Key, value: Value) {
            let h = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
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
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
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
                                // Wf: probe chain (Occupied→Occupied, Empty status unchanged).
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
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
                                        if k == key {}
                                    }
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    let n = choose |n: int| #![trigger old_table_seq[spec_tri_probe(hk, n, m as int)]]
                                        0 <= n < m as int
                                        && spec_tri_probe(hk, n, m as int) == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[spec_tri_probe(hk, j, m as int)] is Empty);
                                }
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(spec_other_slots_preserved(old(table).table@, table.table@, slot as int));
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
                            // Wf: no-dup — key wasn't anywhere before.
                            // Wf: probe chain — new key at slot with witness n = attempt.
                            let ga = attempt as int;
                            let gm = m as int;
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
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
                                    if k == key {
                                    }
                                } else {
                                    let n = choose |n: int| #![trigger old_table_seq[spec_tri_probe(hk, n, gm)]]
                                        0 <= n < gm
                                        && spec_tri_probe(hk, n, gm) == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[spec_tri_probe(hk, j, gm)] is Empty);
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(spec_other_slots_preserved(old(table).table@, table.table@, slot as int));
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
                // Slot update: slot_{a+1} = (slot_a + (a+1)) % m
                // Increments by a+1, maintaining slot = (h + (a+1)*(a+2)/2) % m.
                let new_inc: usize = attempt + 1;
                slot = if new_inc < m - slot { slot + new_inc } else { new_inc - (m - slot) };
                // Veracity: NEEDED proof block
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
                    lemma_tri_step(ga);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        ga + 1, h as int + ga * (ga + 1) / 2, gm);
                // Veracity: NEEDED proof block
                }
                attempt = attempt + 1;
            }
            // Exhausted all m positions. Unreachable because the triangular probe sequence
            // is a complete permutation of {0..m-1} (m a power of two), so a non-full table
            // must contain an empty slot that the probe sequence reaches.
            // Veracity: NEEDED proof block
            proof {
                // Bridge: loop invariant uses `m`, lemma requires `table.current_size`.
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall |d: int| #![trigger spec_tri_probe(h as int, d, table.current_size as int)]
                    0 <= d < table.current_size as int
                    implies !spec_flat_has_key(
                        #[trigger] table.table@[spec_tri_probe(h as int, d, table.current_size as int)], key) by {
                }
                // spec_has_insert_capacity(old(table)) says an Empty slot exists.
                // table.table@ == old(table).table@ (invariant), so it still exists.
                let s_wit = choose |s: int| #![trigger old(table).table@[s]]
                    0 <= s < old(table).table@.len() && old(table).table@[s] is Empty;
                lemma_empty_slot_reachable::<Key, Value, Metrics, H>(table, &key, h);
            }
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe sequence.
        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            // Veracity: NEEDED proof block
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
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
                    // Veracity: NEEDED proof block
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[spec_tri_probe(h as int, d, m as int)] is Empty),
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
                            // Wf: if key existed at slot j, its probe path has no Empty before j.
                            // Since we found Empty at this attempt, key cannot be at any later probe.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                                // Veracity: NEEDED proof block
                                if spec_flat_has_key(table.table@[j], *key) {
                                    let hk = (table.spec_hash@)(*key) as int % m as int;
                                    // wf gives n: 0<=n<m, probe(h,n,m)==j, path 0..n non-Empty.
                                    // But probe(h,attempt,m)==slot is Empty. Contradiction if n>attempt.
                                    // If n < attempt: invariant says !spec_flat_has_key at probe(h,n,m)==j.
                                }
                            // Veracity: NEEDED proof block
                            }
                            lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
                        }
                        return None;
                    }
                    FlatEntry::Deleted => {
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                }
                let new_inc: usize = attempt + 1;
                slot = if new_inc < m - slot { slot + new_inc } else { new_inc - (m - slot) };
                // Veracity: NEEDED proof block
                proof {
                    let ga: int = attempt as int;
                    let gm: int = m as int;
                    let old_slot = (h as int + ga * (ga + 1) / 2) % gm;
                    let inc: int = ga + 1;
                    // Veracity: NEEDED proof block
                    let sum = old_slot + inc;
                    if sum < gm {
                        vstd::arithmetic::div_mod::lemma_small_mod(sum as nat, gm as nat);
                    } else {
                        vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(sum - gm, gm);
                        vstd::arithmetic::div_mod::lemma_small_mod((sum - gm) as nat, gm as nat);
                    }
                    lemma_tri_step(ga);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        ga + 1, h as int + ga * (ga + 1) / 2, gm);
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
                        // Veracity: NEEDED proof block
                        // wf gives n < m; invariant says !spec_flat_has_key at all probe positions 0..m.
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            None
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe until found, then tombstone.
        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
            let h = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let m = table.current_size;
            let mut attempt: usize = 0;
            let mut slot: usize = h;
            // Veracity: NEEDED proof block
            proof {
                vstd::arithmetic::div_mod::lemma_small_mod(h as nat, m as nat);
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
                    // Veracity: NEEDED proof block
                    spec_quadprobflathashsteph_wf(table),
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
                    table.table@ == old(table).table@,
                    table.spec_hash == old(table).spec_hash,
                    table.num_elements == old(table).num_elements,
                    forall |d: int| 0 <= d < attempt as int
                        ==> !#[trigger] spec_flat_has_key(
                            table.table@[spec_tri_probe(h as int, d, m as int)], *key),
                    // Veracity: NEEDED proof block
                    forall |d: int| 0 <= d < attempt as int
                        ==> !(#[trigger] table.table@[spec_tri_probe(h as int, d, m as int)] is Empty),
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
                                // Wf: probe chain (Occupied→Deleted, which is not Empty).
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall |i: int, k: Key|
                                    0 <= i < m as int
                                    && #[trigger] spec_flat_has_key(table.table@[i], k)
                                    // Veracity: NEEDED proof block
                                    implies ({
                                        let hk = (table.spec_hash@)(k) as int % m as int;
                                        exists |n: int| #![trigger table.table@[spec_tri_probe(hk, n, m as int)]]
                                            0 <= n < m as int
                                            // Veracity: NEEDED proof block
                                            && spec_tri_probe(hk, n, m as int) == i
                                            && forall |j: int| 0 <= j < n
                                                ==> !(#[trigger] table.table@[spec_tri_probe(hk, j, m as int)] is Empty)
                                    }) by {
                                    let hk = (table.spec_hash@)(k) as int % m as int;
                                    let n = choose |n: int| #![trigger old_table_seq[spec_tri_probe(hk, n, m as int)]]
                                        0 <= n < m as int
                                        && spec_tri_probe(hk, n, m as int) == i
                                        && forall |j: int| 0 <= j < n
                                            ==> !(#[trigger] old_table_seq[spec_tri_probe(hk, j, m as int)] is Empty);
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
                        return false;
                    }
                    FlatEntry::Deleted => {
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                }
                // Veracity: NEEDED proof block
                let new_inc: usize = attempt + 1;
                slot = if new_inc < m - slot { slot + new_inc } else { new_inc - (m - slot) };
                // Veracity: NEEDED proof block
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
                    lemma_tri_step(ga);
                    vstd::arithmetic::div_mod::lemma_add_mod_noop_right(
                        ga + 1, h as int + ga * (ga + 1) / 2, gm);
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
                    }
                }
                lemma_table_to_map_not_contains::<Key, Value, FlatEntry<Key, Value>>(table.table@, *key);
            }
            false
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m').
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m') — collect n pairs, create m' empty slots, reinsert.
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
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    assert(sub_next.drop_last() =~= table.table@.subrange(0, i as int));
                }
                i = i + 1;
            }
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(table.table@.subrange(0, table.table@.len() as int) =~= table.table@);
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
            // Veracity: NEEDED proof block
            proof {
                // spec_is_power_of_two follows from spec_resize_ok requirement.
                // Veracity: NEEDED assert
                assert(spec_quadprobflathashsteph_wf(&new_table));
                lemma_all_empties_count::<Key, Value>(new_table.table@);
            }
// Veracity: NEEDED proof block

            // Phase 3: reinsert all pairs.
            let mut j: usize = 0;
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
                    // Veracity: NEEDED proof block
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
        for QuadProbFlatHashTableStEph
    {
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — triangular probe position.
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize) {
            let hash_val = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
            let tri = attempt.wrapping_mul(attempt.wrapping_add(1)) / 2;
            (hash_val.wrapping_add(tri)) % table.current_size
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1-α)) expected, Span O(1/(1-α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1/(1-α)), Span O(1/(1-α)) — triangular probe sequence.
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
