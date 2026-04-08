//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared spec functions and proof lemmas for the graph modules in Chap52.
//! Contains spec_count_true, spec_sum_of, spec_sum_adj_sizes, spec_sum_entry_sizes,
//! and all associated proof lemmas used across AdjSeq, AdjMatrix, and AdjTable variants.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 7. proof fns

//		Section 1. module

pub mod AdjTableGraphSpecsAndLemmas {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap42::TableStEph::TableStEph::{
        spec_entries_to_map, spec_keys_no_dups,
        lemma_entries_to_map_finite, lemma_entries_to_map_no_key,
        lemma_entries_to_map_get, lemma_entries_to_map_contains_key,
    };

    verus! {

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    //		Section 6. spec fns


    /// Count how many of f(0), f(1), ..., f(n-1) are true.
    pub open spec fn spec_count_true(f: spec_fn(int) -> bool, n: int) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else if f(n - 1) { 1 + spec_count_true(f, n - 1) }
        else { spec_count_true(f, n - 1) }
    }

    /// Sum of f(0) + f(1) + ... + f(n-1).
    pub open spec fn spec_sum_of(n: int, f: spec_fn(int) -> nat) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { f(n - 1) + spec_sum_of(n - 1, f) }
    }

    /// Sum of neighbor-set sizes over map domain (recursive over dom).
    pub open spec fn spec_sum_adj_sizes<VV>(m: Map<VV, Set<VV>>) -> nat
        decreases m.dom().len()
        when m.dom().finite()
    {
        if m.dom().is_empty() {
            0
        } else {
            let k = m.dom().choose();
            m[k].len() + spec_sum_adj_sizes(m.remove(k))
        }
    }

    /// Sequential sum of entry value-set sizes: entries[0].1.len() + ... + entries[n-1].1.len().
    pub open spec fn spec_sum_entry_sizes<VV>(entries: Seq<(VV, Set<VV>)>, n: int) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { entries[n - 1].1.len() + spec_sum_entry_sizes(entries, n - 1) }
    }

    //		Section 7. proof fns/broadcast groups


    // spec_count_true lemmas

    pub proof fn lemma_count_true_monotone(f: spec_fn(int) -> bool, i: int, n: int)
        requires 0 <= i <= n
        ensures spec_count_true(f, i) <= spec_count_true(f, n)
        decreases n - i
    {
        if i < n {
            lemma_count_true_monotone(f, i, n - 1);
        }
    }

    pub proof fn lemma_count_true_bound(f: spec_fn(int) -> bool, n: int)
        requires n >= 0
        ensures spec_count_true(f, n) <= n as nat
        decreases n
    {
        if n > 0 {
            lemma_count_true_bound(f, n - 1);
        }
    }

    /// Count of an all-false predicate is zero.
    pub proof fn lemma_count_true_all_false(f: spec_fn(int) -> bool, n: int)
        requires forall|i: int| 0 <= i < n ==> !#[trigger] f(i)
        ensures spec_count_true(f, n) == 0
        decreases n
    {
        if n > 0 {
            assert(!f(n - 1));
            lemma_count_true_all_false(f, n - 1);
        }
    }

    /// Extensionality for spec_count_true: identical predicates yield identical counts.
    pub proof fn lemma_count_true_ext(f: spec_fn(int) -> bool, g: spec_fn(int) -> bool, n: int)
        requires forall|i: int| 0 <= i < n ==> #[trigger] f(i) == g(i)
        ensures spec_count_true(f, n) == spec_count_true(g, n)
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) == g(n - 1));
            lemma_count_true_ext(f, g, n - 1);
        }
    }

    /// Flipping one position from false to true increases count by 1.
    pub proof fn lemma_count_true_set_true(f: spec_fn(int) -> bool, g: spec_fn(int) -> bool, k: int, n: int)
        requires
            0 <= k < n,
            !f(k) && g(k),
            forall|i: int| 0 <= i < n && i != k ==> #[trigger] f(i) == g(i),
        ensures spec_count_true(g, n) == spec_count_true(f, n) + 1
        decreases n
    {
        if n > 0 {
            if k == n - 1 {
                lemma_count_true_ext(f, g, n - 1);
            } else {
                assert(f(n - 1) == g(n - 1));
                lemma_count_true_set_true(f, g, k, n - 1);
            }
        }
    }

    /// Flipping one position from true to false decreases count by 1.
    pub proof fn lemma_count_true_set_false(f: spec_fn(int) -> bool, g: spec_fn(int) -> bool, k: int, n: int)
        requires
            0 <= k < n,
            f(k) && !g(k),
            forall|i: int| 0 <= i < n && i != k ==> #[trigger] f(i) == g(i),
        ensures spec_count_true(f, n) == spec_count_true(g, n) + 1
        decreases n
    {
        if n > 0 {
            if k == n - 1 {
                lemma_count_true_ext(f, g, n - 1);
            } else {
                assert(f(n - 1) == g(n - 1));
                lemma_count_true_set_false(f, g, k, n - 1);
            }
        }
    }

    /// Count is at least 1 if the predicate holds at some position.
    pub proof fn lemma_count_true_at_least_one(f: spec_fn(int) -> bool, k: int, n: int)
        requires 0 <= k < n, f(k)
        ensures spec_count_true(f, n) >= 1
        decreases n
    {
        if k == n - 1 {
        } else {
            lemma_count_true_at_least_one(f, k, n - 1);
        }
    }

    // spec_sum_of lemmas

    pub proof fn lemma_sum_of_monotone(i: int, n: int, f: spec_fn(int) -> nat)
        requires 0 <= i <= n
        ensures spec_sum_of(i, f) <= spec_sum_of(n, f)
        decreases n - i
    {
        if i < n {
            lemma_sum_of_monotone(i, n - 1, f);
        }
    }

    /// Unfolding one step: spec_sum_of(i+1, f) == f(i) + spec_sum_of(i, f).
    pub proof fn lemma_sum_of_unfold(i: int, f: spec_fn(int) -> nat)
        requires i >= 0
        ensures spec_sum_of(i + 1, f) == f(i) + spec_sum_of(i, f)
    {
    }

    /// Sum of all-zero function is zero.
    pub proof fn lemma_sum_of_all_zero(f: spec_fn(int) -> nat, n: int)
        requires forall|i: int| 0 <= i < n ==> #[trigger] f(i) == 0nat
        ensures spec_sum_of(n, f) == 0
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) == 0nat);
            lemma_sum_of_all_zero(f, n - 1);
        }
    }

    /// Extensionality for spec_sum_of: identical functions yield identical sums.
    pub proof fn lemma_sum_of_ext(f: spec_fn(int) -> nat, g: spec_fn(int) -> nat, n: int)
        requires forall|i: int| 0 <= i < n ==> #[trigger] f(i) == g(i)
        ensures spec_sum_of(n, f) == spec_sum_of(n, g)
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) == g(n - 1));
            lemma_sum_of_ext(f, g, n - 1);
        }
    }

    /// Changing one term in the sum: the total changes by the difference.
    pub proof fn lemma_sum_of_change_one(n: int, old_f: spec_fn(int) -> nat, new_f: spec_fn(int) -> nat, k: int)
        requires
            0 <= k < n,
            forall|i: int| 0 <= i < n && i != k ==> #[trigger] old_f(i) == new_f(i),
        ensures
            spec_sum_of(n, new_f) + old_f(k) == spec_sum_of(n, old_f) + new_f(k),
        decreases n,
    {
        if n > 0 {
            if k == n - 1 {
                lemma_sum_of_ext(old_f, new_f, n - 1);
            } else {
                assert(old_f(n - 1) == new_f(n - 1));
                lemma_sum_of_change_one(n - 1, old_f, new_f, k);
            }
        }
    }

    /// Lower bound: the sum of nats is at least any single term.
    pub proof fn lemma_sum_of_lower_bound(n: int, f: spec_fn(int) -> nat, k: int)
        requires 0 <= k < n
        ensures spec_sum_of(n, f) >= f(k)
        decreases n
    {
        if k == n - 1 {
        } else {
            lemma_sum_of_lower_bound(n - 1, f, k);
        }
    }

    /// Upper bound: if each f(i) ≤ bound, then sum ≤ n * bound.
    pub proof fn lemma_sum_of_bounded(n: int, f: spec_fn(int) -> nat, bound: nat)
        requires
            n >= 0,
            forall|i: int| 0 <= i < n ==> #[trigger] f(i) <= bound,
        ensures spec_sum_of(n, f) <= (n as nat) * bound
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) <= bound);
            lemma_sum_of_bounded(n - 1, f, bound);
            assert(spec_sum_of(n, f) <= bound + ((n - 1) as nat) * bound);
            assert((n as nat) * bound == bound + ((n - 1) as nat) * bound) by(nonlinear_arith)
                requires n >= 1;
        }
    }

    // spec_sum_adj_sizes lemmas

    /// Extract any key from the recursive sum: decompose at k regardless of choose() order.
    pub proof fn lemma_sum_adj_remove<VV>(m: Map<VV, Set<VV>>, k: VV)
        requires m.dom().finite(), m.dom().contains(k)
        ensures spec_sum_adj_sizes(m) == m[k].len() + spec_sum_adj_sizes(m.remove(k))
        decreases m.dom().len()
    {
        let chosen = m.dom().choose();
        if chosen == k {
            // Definition picks k directly.
        } else {
            // Definition picks chosen != k.
            lemma_sum_adj_remove(m.remove(chosen), k);
            lemma_sum_adj_remove(m.remove(k), chosen);
            assert(m.remove(chosen).remove(k) =~= m.remove(k).remove(chosen));
        }
    }

    /// If every value set in m1 is no larger than the corresponding set in m2
    /// (same domain), then the sum of sizes is no larger.
    pub proof fn lemma_sum_adj_sizes_monotone<VV>(m1: Map<VV, Set<VV>>, m2: Map<VV, Set<VV>>)
        requires
            m1.dom().finite(),
            m1.dom() =~= m2.dom(),
            forall|k: VV| #[trigger] m1.dom().contains(k) ==> m1[k].len() <= m2[k].len(),
        ensures
            spec_sum_adj_sizes(m1) <= spec_sum_adj_sizes(m2)
        decreases m1.dom().len()
    {
        if m1.dom().is_empty() {
        } else {
            let k = m1.dom().choose();
            lemma_sum_adj_remove(m1, k);
            lemma_sum_adj_remove(m2, k);
            assert(m1.remove(k).dom() =~= m2.remove(k).dom());
            assert forall|j: VV| #[trigger] m1.remove(k).dom().contains(j)
                implies m1.remove(k)[j].len() <= m2.remove(k)[j].len()
            by {
                assert(m1.dom().contains(j));
            };
            lemma_sum_adj_sizes_monotone(m1.remove(k), m2.remove(k));
        }
    }

    // spec_sum_entry_sizes lemmas

    /// Connect sequential entry sum to recursive map sum.
    pub proof fn lemma_sum_entry_sizes_eq<VV>(entries: Seq<(VV, Set<VV>)>, n: int)
        requires
            0 <= n <= entries.len(),
            spec_keys_no_dups(entries),
        ensures
            spec_sum_entry_sizes(entries, n) == spec_sum_adj_sizes(
                spec_entries_to_map(entries.subrange(0, n)))
        decreases n
    {
        lemma_entries_to_map_finite::<VV, Set<VV>>(entries.subrange(0, n));
        if n == 0 {
            assert(entries.subrange(0, 0) =~= Seq::<(VV, Set<VV>)>::empty());
        } else {
            let sub_n = entries.subrange(0, n);
            let sub_prev = entries.subrange(0, n - 1);
            assert(spec_keys_no_dups::<VV, Set<VV>>(sub_n)) by {
                assert forall|i: int, j: int|
                    0 <= i < j < sub_n.len()
                    implies (#[trigger] sub_n[i]).0 != (#[trigger] sub_n[j]).0
                by {
                    assert(entries[i].0 != entries[j].0);
                };
            };
            assert(spec_keys_no_dups::<VV, Set<VV>>(sub_prev)) by {
                assert forall|i: int, j: int|
                    0 <= i < j < sub_prev.len()
                    implies (#[trigger] sub_prev[i]).0 != (#[trigger] sub_prev[j]).0
                by {
                    assert(entries[i].0 != entries[j].0);
                };
            };
            lemma_sum_entry_sizes_eq(entries, n - 1);
            let prefix_map = spec_entries_to_map(sub_prev);
            let full_map = spec_entries_to_map(sub_n);
            let key = entries[n - 1].0;
            let val = entries[n - 1].1;
            assert(sub_n =~= sub_prev.push((key, val)));
            assert forall|idx: int| 0 <= idx < sub_prev.len()
                implies (#[trigger] sub_prev[idx]).0 != key
            by {};
            lemma_entries_to_map_no_key::<VV, Set<VV>>(sub_prev, key);
            assert(sub_n.drop_last() =~= sub_prev);
            assert(sub_n.last() == (key, val));
            lemma_entries_to_map_finite::<VV, Set<VV>>(sub_n);
            lemma_entries_to_map_contains_key::<VV, Set<VV>>(sub_n, n - 1);
            lemma_sum_adj_remove(full_map, key);
            lemma_entries_to_map_get::<VV, Set<VV>>(sub_n, n - 1);
            lemma_entries_to_map_finite::<VV, Set<VV>>(sub_prev);
            assert(full_map =~= prefix_map.insert(key, val));
            assert(!prefix_map.dom().contains(key));
            assert(prefix_map.insert(key, val).remove(key) =~= prefix_map);
        }
    }

    /// Partial sums are monotonically non-decreasing.
    pub proof fn lemma_sum_entry_sizes_monotone<VV>(entries: Seq<(VV, Set<VV>)>, i: int, j: int)
        requires 0 <= i <= j <= entries.len()
        ensures spec_sum_entry_sizes(entries, i) <= spec_sum_entry_sizes(entries, j)
        decreases j - i
    {
        if i < j {
            lemma_sum_entry_sizes_monotone(entries, i, j - 1);
        }
    }

    } // verus!
} // pub mod AdjTableGraphSpecsAndLemmas
