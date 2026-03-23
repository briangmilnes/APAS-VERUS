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
    // 6. spec fns (inside verus!: spec_is_power_of_two, spec_tri_probe, spec_quadprobflathashsteph_wf, spec_count_empties)
    // 7. proof fns (inside verus!: lemma_triangular_injective, lemma_empty_slot_reachable, lemma_*_empties)
    // 9. impls (inside verus!)
    // 13. derive impls outside verus!

    // 2. imports
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
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger};

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
            assert(pow(2, 1nat) == 2);
            vstd::arithmetic::div_mod::lemma_mul_mod_noop(a, b, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(b, 2);
            if b % 2 == 1 {
                assert(((a % 2) * (b % 2)) % 2 == (a * b) % 2);
                assert((1 * 1) % 2 == 0);
            }
        } else {
            let pn1 = pow(2, (n - 1) as nat);
            vstd::arithmetic::power::lemma_pow_positive(2, (n - 1) as nat);
            vstd::arithmetic::power::lemma_pow_adds(2, 1, (n - 1) as nat);
            assert(pow(2, n) == 2 * pn1);
            vstd::arithmetic::mul::lemma_mul_is_commutative(2int, pn1);
            assert(pn1 * 2 == pow(2, n));

            // (a*b) % pn1 == 0 via mod_mod.
            vstd::arithmetic::div_mod::lemma_mod_mod(a * b, pn1, 2);
            vstd::arithmetic::div_mod::lemma_small_mod(0nat, pn1 as nat);
            assert((a * b) % pn1 == 0);

            // IH: b % pn1 == 0.
            lemma_odd_factor_pow2(a, b, (n - 1) as nat);

            // b = pn1 * c.
            let c = b / pn1;
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(b, pn1);
            assert(b == pn1 * c);

            // a * b = pn1 * (a * c).
            assert(a * b == a * (pn1 * c));
            vstd::arithmetic::mul::lemma_mul_is_commutative(a, pn1);
            vstd::arithmetic::mul::lemma_mul_is_associative(pn1, a, c);
            assert(a * (pn1 * c) == pn1 * (a * c)) by (nonlinear_arith);
            let ac = a * c;

            // Decompose ac = 2*q + r.
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(ac, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(ac, 2);
            let r = ac % 2;
            let q = ac / 2;
            assert(ac == 2 * q + r);
            assert(0 <= r < 2);

            // pn1 * ac = (2*pn1)*q + pn1*r.
            let two_pn1 = 2 * pn1;
            assert(two_pn1 == pow(2, n));
            vstd::arithmetic::mul::lemma_mul_is_distributive_add(pn1, 2 * q, r);
            vstd::arithmetic::mul::lemma_mul_is_associative(pn1, 2int, q);
            vstd::arithmetic::mul::lemma_mul_is_commutative(pn1, 2int);
            assert(pn1 * (2 * q) == two_pn1 * q);
            assert(pn1 * ac == two_pn1 * q + pn1 * r);
            assert(a * b == two_pn1 * q + pn1 * r);

            vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(q, pn1 * r, two_pn1);
            assert((a * b) % two_pn1 == (pn1 * r) % two_pn1);
            assert((pn1 * r) % two_pn1 == 0);

            // r must be 0: if r == 1, pn1 % two_pn1 == pn1 > 0.
            if r == 1 {
                vstd::arithmetic::mul::lemma_mul_basics(pn1);
                vstd::arithmetic::div_mod::lemma_small_mod(pn1 as nat, two_pn1 as nat);
            }
            assert(r == 0);
            assert(ac % 2 == 0);

            // Since a odd and ac even, c even.
            vstd::arithmetic::div_mod::lemma_mul_mod_noop(a, c, 2);
            vstd::arithmetic::div_mod::lemma_mod_bound(c, 2);
            if c % 2 == 1 {
                assert(((a % 2) * (c % 2)) % 2 == ac % 2);
                assert((1 * 1) % 2 == 0);
            }
            assert(c % 2 == 0);

            // c = 2*d, b = pn1 * 2 * d = pow(2,n) * d.
            let d = c / 2;
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(c, 2);
            assert(c == 2 * d);
            vstd::arithmetic::mul::lemma_mul_is_associative(pn1, 2int, d);
            assert(b == two_pn1 * d);
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
            assert((j - i) * (j + i + 1) == j * (j + 1) - i * (i + 1)) by (nonlinear_arith);

            // Both are even (consecutive products).
            lemma_consecutive_even(i);
            lemma_consecutive_even(j);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i * (i + 1), 2);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j * (j + 1), 2);
            assert(i * (i + 1) == 2 * ti);
            assert(j * (j + 1) == 2 * tj);

            // prod = 2 * (tj - ti).
            let prod = (j - i) * (j + i + 1);
            assert(prod == 2 * (tj - ti)) by (nonlinear_arith)
                requires prod == j * (j + 1) - i * (i + 1),
                         i * (i + 1) == 2 * ti,
                         j * (j + 1) == 2 * tj;

            // tj - ti == m * q, so prod == 2 * m * q.
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(ti, m);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(tj, m);
            vstd::arithmetic::mul::lemma_mul_is_distributive_sub(m, tj / m, ti / m);
            assert(tj - ti == m * (tj / m - ti / m));
            let q = tj / m - ti / m;
            assert(prod == 2 * (m * q)) by (nonlinear_arith)
                requires prod == 2 * (tj - ti), tj - ti == m * q;

            // 2*m == pow(2, k+1).
            vstd::arithmetic::power::lemma_pow_adds(2, 1, k);
            vstd::arithmetic::power::lemma_pow1(2int);
            let two_m = pow(2, (k + 1) as nat);
            assert(two_m == 2 * m);

            // prod == two_m * q, so prod % two_m == 0.
            vstd::arithmetic::mul::lemma_mul_is_associative(2int, m, q);
            assert(prod == two_m * q);
            vstd::arithmetic::mul::lemma_mul_is_commutative(two_m, q);
            vstd::arithmetic::div_mod::lemma_mod_multiples_basic(q, two_m);
            assert(prod % two_m == 0);

            // Parity: (j-i) + (j+i+1) = 2j+1 is odd.
            let a = j - i;
            let b = j + i + 1;
            assert(a + b == 2 * j + 1) by (nonlinear_arith)
                requires a == j - i, b == j + i + 1;
            assert(a > 0);
            assert(b > 0);
            assert(a < m);
            assert(b < 2 * m);
            vstd::arithmetic::div_mod::lemma_add_mod_noop(a, b, 2);
            assert((a + b) % 2 == 1);
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
                assert(b % 2 != 0);
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

        assert(probes.no_duplicates()) by {
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
        assert(probes.to_set().subset_of(set_int_range(0, m))) by {
            assert forall |x: int| #[trigger] probes.to_set().contains(x)
                implies set_int_range(0, m).contains(x) by {
                let d = choose |d: int| 0 <= d < m && probes[d] == x;
                vstd::arithmetic::div_mod::lemma_mod_bound(hi + d * (d + 1) / 2, m);
            }
        };

        // set_int_range(0, m) has cardinality m.
        lemma_int_range(0, m);

        // Both finite, same cardinality, subset — so equal.
        lemma_subset_equality(probes.to_set(), set_int_range(0, m));

        // s ∈ set_int_range(0, m), so s ∈ probes.to_set().
        assert(set_int_range(0, m).contains(s));
        assert(probes.to_set().contains(s));

        // probes.contains(s) gives a witness d.
        assert(probes.contains(s));
        let d_s = choose |d: int| 0 <= d < m && probes[d] == s;
        assert(spec_tri_probe(hi, d_s, m) == s);

        // But the requires says table.table@[spec_tri_probe(h, d_s, m)] is not Empty.
        assert(!(table.table@[spec_tri_probe(hi, d_s, m)] is Empty));
        // Yet table.table@[s] is Empty. Contradiction.
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

    // 9. impls

    impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics, H>
        for QuadProbFlatHashTableStEph
    {
        open spec fn spec_impl_wf(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics, H>) -> bool {
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
                assert forall |d: int| #![trigger spec_tri_probe(h as int, d, table.current_size as int)]
                    0 <= d < table.current_size as int
                    implies !spec_flat_has_key(
                        #[trigger] table.table@[spec_tri_probe(h as int, d, table.current_size as int)], key) by {
                    assert(spec_tri_probe(h as int, d, m as int) == spec_tri_probe(h as int, d, table.current_size as int));
                }
                assert forall |d: int| 0 <= d < table.current_size as int
                    implies !(#[trigger] table.table@[spec_tri_probe(h as int, d, table.current_size as int)] is Empty) by {
                    assert(spec_tri_probe(h as int, d, m as int) == spec_tri_probe(h as int, d, table.current_size as int));
                }
                // spec_has_insert_capacity(old(table)) says an Empty slot exists.
                // table.table@ == old(table).table@ (invariant), so it still exists.
                let s_wit = choose |s: int| #![trigger old(table).table@[s]]
                    0 <= s < old(table).table@.len() && old(table).table@[s] is Empty;
                assert(table.table@[s_wit] is Empty);
                assert(0 <= s_wit && s_wit < table.current_size as int);
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
                    spec_hash_fn_valid::<Key, H>(table.spec_hash@),
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
                lemma_all_empties_count::<Key, Value>(new_table.table@);
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
