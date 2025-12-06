//! Lemmas about the relationship between Seq operations (take, skip, push) and to_set()
// Note: These are regular proof functions, not broadcast.
// Broadcast versions caused massive performance issues (30M+ rlimit).

use vstd::prelude::*;

verus! {

broadcast use {
    vstd::seq_lib::group_seq_properties,
    vstd::set::group_set_axioms,
};

/// If a sequence contains an element at index i, then that element is in the sequence's set view.
// Veracity: UNUSED pub proof fn lemma_seq_index_in_to_set<T>(seq: Seq<T>, i: int)
// Veracity: UNUSED     requires
// Veracity: UNUSED         0 <= i < seq.len(),
// Veracity: UNUSED     ensures
// Veracity: UNUSED         seq.to_set().contains(seq[i]),
// Veracity: UNUSED {
// Veracity: UNUSED     broadcast use vstd::seq_lib::group_seq_properties;
// Veracity: UNUSED     assert(seq.contains(seq[i]));
// Veracity: UNUSED }

/// If a sequence does not contain an element v, then pushing v onto the sequence
/// creates a subset of the original set with v inserted.
// Veracity: USED
pub proof fn lemma_push_not_contains_to_set_subset<T>(seq: Seq<T>, v: T)
    requires
        !seq.contains(v),
    ensures
        seq.push(v).to_set() <= seq.to_set().insert(v),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    assert forall |x: T| #[trigger] seq.push(v).to_set().contains(x) 
        implies seq.to_set().insert(v).contains(x) by {
        if seq.push(v).contains(x) {
            if x == v {
// Veracity: UNNEEDED assert                 assert(seq.to_set().insert(v).contains(v));
            } else {
// Veracity: UNNEEDED assert                 assert(seq.contains(x));
// Veracity: UNNEEDED assert                 assert(seq.to_set().contains(x));
// Veracity: UNNEEDED assert                 assert(seq.to_set().insert(v).contains(x));
            }
        }
    }
}

/// If a sequence does not contain an element v, then the original set with v inserted
/// is a subset of pushing v onto the sequence.
// Veracity: USED
pub proof fn lemma_push_not_contains_to_set_superset<T>(seq: Seq<T>, v: T)
    requires
        !seq.contains(v),
    ensures
        seq.to_set().insert(v) <= seq.push(v).to_set(),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    
    assert forall |x: T| #[trigger] seq.to_set().insert(v).contains(x) 
        implies seq.push(v).to_set().contains(x) by {
        if x == v {
// Veracity: UNNEEDED assert             assert(seq.push(v)[seq.len() as int] == v);
// Veracity: UNNEEDED assert             assert(seq.push(v).contains(v));
        } else if seq.to_set().contains(x) {
// Veracity: UNNEEDED assert             assert(seq.contains(x));
            let idx = seq.lemma_contains_to_index(x);
// Veracity: UNNEEDED assert             assert(seq.push(v)[idx] == x);
// Veracity: UNNEEDED assert             assert(seq.push(v).contains(x));
        }
    }
}

/// If a sequence does not contain an element v, then pushing v onto the sequence
/// creates a set equal to the original set with v inserted.
// Veracity: USED
pub proof fn lemma_push_not_contains_to_set<T>(seq: Seq<T>, v: T)
    requires
        !seq.contains(v),
    ensures
        seq.push(v).to_set() == seq.to_set().insert(v),
{
    lemma_push_not_contains_to_set_subset(seq, v);
    lemma_push_not_contains_to_set_superset(seq, v);
    broadcast use vstd::set::group_set_axioms;
}

/// Taking the full length of a sequence yields the original sequence.
// Veracity: UNUSED pub proof fn lemma_take_full<T>(seq: Seq<T>)
// Veracity: UNUSED     ensures
// Veracity: UNUSED         seq.take(seq.len() as int) == seq,
// Veracity: UNUSED {
// Veracity: UNUSED     broadcast use vstd::seq_lib::group_seq_properties;
// Veracity: UNUSED     assert(seq.take(seq.len() as int) =~= seq);
// Veracity: UNUSED }
// Veracity: USED

/// After taking n elements and inserting seq[n], the result is a subset of take(n+1).
pub proof fn lemma_take_extends_set_subset<T>(seq: Seq<T>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n).to_set().insert(seq[n]) <= seq.take(n+1).to_set(),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let prefix_n = seq.take(n);
    let prefix_n_plus_1 = seq.take(n + 1);
    
    // Key insight: take(n+1) = take(n).push(seq[n])
// Veracity: UNNEEDED assert     assert forall |i: int| 0 <= i < n implies #[trigger] prefix_n_plus_1[i] == prefix_n[i] by {}
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert     assert(prefix_n_plus_1[n] == seq[n]);
    
    assert forall |x: T| #[trigger] prefix_n.to_set().insert(seq[n]).contains(x) 
        implies prefix_n_plus_1.to_set().contains(x) by {
        if x == seq[n] {
// Veracity: UNNEEDED assert             assert(prefix_n_plus_1[n] == x);
// Veracity: UNNEEDED assert             assert(prefix_n_plus_1.contains(x));
        } else if prefix_n.to_set().contains(x) {
// Veracity: UNNEEDED assert             assert(prefix_n.contains(x));
            let idx = prefix_n.lemma_contains_to_index(x);
// Veracity: UNNEEDED assert             assert(0 <= idx < n);
// Veracity: UNNEEDED assert             assert(prefix_n_plus_1[idx] == x);
// Veracity: UNNEEDED assert             assert(prefix_n_plus_1.contains(x));
        }
    }
}

/// After taking n+1 elements, the result is a subset of take(n) with seq[n] inserted.
// Veracity: USED
pub proof fn lemma_take_extends_set_superset<T>(seq: Seq<T>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n+1).to_set() <= seq.take(n).to_set().insert(seq[n]),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    
    let prefix_n = seq.take(n);
    let prefix_n_plus_1 = seq.take(n + 1);
    
    assert forall |x: T| #[trigger] prefix_n_plus_1.to_set().contains(x)
        implies prefix_n.to_set().insert(seq[n]).contains(x) by {
// Veracity: UNNEEDED assert         assert(prefix_n_plus_1.contains(x));
        let idx = prefix_n_plus_1.lemma_contains_to_index(x);
        if idx < n {
// Veracity: UNNEEDED assert             assert(prefix_n[idx] == x);
// Veracity: UNNEEDED assert             assert(prefix_n.contains(x));
// Veracity: UNNEEDED assert             assert(prefix_n.to_set().contains(x));
        } else {
// Veracity: UNNEEDED assert             assert(idx == n);
// Veracity: UNNEEDED assert             assert(x == seq[n]);
        }
// Veracity: USED
    }
}

/// After taking n elements and then taking n+1 elements (where n < len),
/// the additional element at index n is in the larger set.
pub proof fn lemma_take_one_more_extends_the_seq_set<T>(seq: Seq<T>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n).to_set().insert(seq[n]) == seq.take(n+1).to_set(),
// Veracity: USED
{
    lemma_take_extends_set_subset(seq, n);
    lemma_take_extends_set_superset(seq, n);
    broadcast use vstd::set::group_set_axioms;
}

pub proof fn lemma_set_contains_insert_idempotent<V>(s: Set<V>, v: V)
    requires
        s.contains(v),
    ensures
        s.insert(v) == s,
{
// Veracity: UNNEEDED assert     assert forall |x| s.insert(v).contains(x) implies #[trigger] s.contains(x) by {};
// Veracity: UNNEEDED assert     assert forall |x| #[trigger] s.contains(x) implies s.insert(v).contains(x) by {};
// Veracity: USED
    broadcast use vstd::set::group_set_axioms;
}

// View-aware lemmas for sequences with map operations

/// After taking n elements, mapping through view, and inserting seq[n]@,
/// the result equals take(n+1) mapped through view and converted to set.
pub proof fn lemma_take_one_more_extends_the_seq_set_with_view<T: View>(seq: Seq<T>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n).map(|i: int, k: T| k@).to_set().insert(seq[n]@) == seq.take(n+1).map(|i: int, k: T| k@).to_set(),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let mapped_n = seq.take(n).map(|i: int, k: T| k@);
    let mapped_n_plus_1 = seq.take(n+1).map(|i: int, k: T| k@);
    
    // Key: take(n+1) = take(n).push(seq[n])
    assert forall |i: int| 0 <= i < n implies #[trigger] mapped_n_plus_1[i] == mapped_n[i] by {
// Veracity: UNNEEDED assert         assert(seq.take(n+1)[i] == seq.take(n)[i]);
// Veracity: UNNEEDED assert         assert(mapped_n_plus_1[i] == seq.take(n+1)[i]@);
// Veracity: UNNEEDED assert         assert(mapped_n[i] == seq.take(n)[i]@);
    }
// Veracity: UNNEEDED assert     assert(mapped_n_plus_1[n] == seq[n]@);
    
    // Subset: mapped_n.to_set().insert(seq[n]@) <= mapped_n_plus_1.to_set()
    assert forall |x| #[trigger] mapped_n.to_set().insert(seq[n]@).contains(x) 
        implies mapped_n_plus_1.to_set().contains(x) by {
        if x == seq[n]@ {
            assert(mapped_n_plus_1[n] == x);
        } else if mapped_n.to_set().contains(x) {
            let idx = mapped_n.lemma_contains_to_index(x);
            assert(mapped_n_plus_1[idx] == x);
        }
    }
    
    // Superset: mapped_n_plus_1.to_set() <= mapped_n.to_set().insert(seq[n]@)
    assert forall |x| #[trigger] mapped_n_plus_1.to_set().contains(x)
        implies mapped_n.to_set().insert(seq[n]@).contains(x) by {
        let idx = mapped_n_plus_1.lemma_contains_to_index(x);
        if idx < n {
// Veracity: UNNEEDED assert             assert(mapped_n[idx] == x);
        } else {
// Veracity: UNNEEDED assert             assert(idx == n);
// Veracity: UNNEEDED assert             assert(x == seq[n]@);
        }
// Veracity: USED
    }
}

/// Taking the full length of a sequence, mapping through view, and converting to set
/// yields the same set as mapping the full sequence through view and converting to set.
pub proof fn lemma_take_full_to_set_with_view<T: View>(seq: Seq<T>)
// Veracity: USED
    ensures
        seq.take(seq.len() as int).map(|i: int, k: T| k@).to_set() == seq.map(|i: int, k: T| k@).to_set(),
{
    broadcast use vstd::seq_lib::group_seq_properties;
// Veracity: UNNEEDED assert     assert(seq.take(seq.len() as int) =~= seq);
}

/// Proves that a sequence mapped through view equals a target set when bidirectional containment holds.
/// Lemma: If i is a valid index, then seq.map(...)[i] is in seq.map(...).to_set()
pub proof fn lemma_seq_index_in_map_to_set<T: View>(seq: Seq<T>, i: int)
    requires
        0 <= i < seq.len(),
    ensures
        seq.map(|i: int, k: T| k@).to_set().contains(seq[i]@),
{
// Veracity: USED
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let mapped_seq = seq.map(|i: int, k: T| k@);
// Veracity: UNNEEDED assert     assert(mapped_seq[i] == seq[i]@);
    assert(mapped_seq.to_set().contains(mapped_seq[i]));
}

/// Lemma: If s is in seq.map(...).to_set(), then there exists an index i such that s == seq[i]@
pub proof fn lemma_map_to_set_contains_index<T: View>(seq: Seq<T>, s: T::V)
    requires
        seq.map(|i: int, k: T| k@).to_set().contains(s),
    ensures
        exists |i: int| #![trigger seq[i]] 0 <= i < seq.len() && s == seq[i]@,
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let mapped_seq = seq.map(|i: int, k: T| k@);
// Veracity: UNNEEDED assert     assert(mapped_seq.to_set().contains(s));
    let idx = mapped_seq.lemma_contains_to_index(s);
// Veracity: USED
// Veracity: UNNEEDED assert     assert(mapped_seq[idx] == s);
// Veracity: UNNEEDED assert     assert(seq[idx]@ == s);
// Veracity: UNNEEDED assert     assert(0 <= idx < seq.len());
}

/// This lemma bridges the gap between iterator specs and set equality.
pub proof fn lemma_seq_map_to_set_equality<T: View>(seq: Seq<T>, target: Set<T::V>)
    requires
        seq.no_duplicates(),
        forall|k: T| #![trigger seq.contains(k), target.contains(k@)] seq.contains(k) ==> target.contains(k@),
        forall|kv: T::V| #[trigger] target.contains(kv) ==> exists|k: T| #![trigger seq.contains(k)] seq.contains(k) && k@ == kv,
    ensures
        seq.map(|i: int, k: T| k@).to_set() == target,
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let mapped_set = seq.map(|i: int, k: T| k@).to_set();
    
    // Prove subset: mapped_set <= target
    assert forall |kv: T::V| #[trigger] mapped_set.contains(kv) implies target.contains(kv) by {
        if mapped_set.contains(kv) {
            let mapped_seq = seq.map(|i: int, k: T| k@);
            let idx = mapped_seq.lemma_contains_to_index(kv);
            assert(seq.contains(seq[idx]));
        }
    }
    
    // Prove superset: target <= mapped_set
    assert forall |kv: T::V| #[trigger] target.contains(kv) implies mapped_set.contains(kv) by {
        if target.contains(kv) {
            // From precondition: exists k such that seq.contains(k) && k@ == kv
            let k = choose|k: T| #![trigger seq.contains(k)] seq.contains(k) && k@ == kv;
            let idx = seq.lemma_contains_to_index(k);
// Veracity: UNNEEDED assert             assert(seq[idx] == k);
// Veracity: USED
            let mapped_seq = seq.map(|i: int, k: T| k@);
            assert(mapped_seq[idx] == seq[idx]@);
        }
    }
}

/// After taking n elements and mapping through view, intersecting with a set s2,
/// extending to n+1 either adds seq[n]@ (if in s2) or keeps the intersection unchanged.
pub proof fn lemma_take_one_more_intersect<T: View>(seq: Seq<T>, s2: Set<T::V>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n+1).map(|i: int, k: T| k@).to_set().intersect(s2) == 
            if s2.contains(seq[n]@) {
                seq.take(n).map(|i: int, k: T| k@).to_set().intersect(s2).insert(seq[n]@)
            } else {
                seq.take(n).map(|i: int, k: T| k@).to_set().intersect(s2)
            },
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let mapped_n = seq.take(n).map(|i: int, k: T| k@);
    let mapped_n_plus_1 = seq.take(n+1).map(|i: int, k: T| k@);
    let set_n = mapped_n.to_set();
    let set_n_plus_1 = mapped_n_plus_1.to_set();
    
    // From lemma_take_one_more_extends_the_seq_set_with_view:
    // set_n_plus_1 == set_n.insert(seq[n]@)
    lemma_take_one_more_extends_the_seq_set_with_view(seq, n);
// Veracity: UNNEEDED assert     assert(set_n_plus_1 == set_n.insert(seq[n]@));
    
    if s2.contains(seq[n]@) {
        // Case 1: seq[n]@ is in s2
        // (A ∪ {x}) ∩ B = (A ∩ B) ∪ {x} when x ∈ B
        assert forall |v: T::V| #[trigger] set_n_plus_1.intersect(s2).contains(v) 
            implies set_n.intersect(s2).insert(seq[n]@).contains(v) by {
// Veracity: UNNEEDED assert             assert(set_n_plus_1.contains(v) && s2.contains(v));
            if v == seq[n]@ {
// Veracity: UNNEEDED assert                 assert(set_n.intersect(s2).insert(seq[n]@).contains(v));
            } else {
// Veracity: UNNEEDED assert                 assert(set_n.contains(v));
// Veracity: UNNEEDED assert                 assert(set_n.intersect(s2).contains(v));
            }
        }
        
        assert forall |v: T::V| #[trigger] set_n.intersect(s2).insert(seq[n]@).contains(v)
            implies set_n_plus_1.intersect(s2).contains(v) by {
            if v == seq[n]@ {
// Veracity: UNNEEDED assert                 assert(set_n_plus_1.contains(v));
// Veracity: UNNEEDED assert                 assert(s2.contains(v));
            } else if set_n.intersect(s2).contains(v) {
// Veracity: UNNEEDED assert                 assert(set_n.contains(v));
// Veracity: UNNEEDED assert                 assert(set_n_plus_1.contains(v));
            }
        }
    } else {
        // Case 2: seq[n]@ is not in s2
        // (A ∪ {x}) ∩ B = A ∩ B when x ∉ B
        assert forall |v: T::V| #[trigger] set_n_plus_1.intersect(s2).contains(v)
            implies set_n.intersect(s2).contains(v) by {
// Veracity: UNNEEDED assert             assert(set_n_plus_1.contains(v) && s2.contains(v));
// Veracity: UNNEEDED assert             assert(v != seq[n]@); // because seq[n]@ is not in s2
// Veracity: UNNEEDED assert             assert(set_n.contains(v));
        }
        
        assert forall |v: T::V| #[trigger] set_n.intersect(s2).contains(v)
            implies set_n_plus_1.intersect(s2).contains(v) by {
// Veracity: UNNEEDED assert             assert(set_n.contains(v) && s2.contains(v));
// Veracity: UNNEEDED assert             assert(set_n_plus_1.contains(v));
        }
    }
}

// The problem for weighted sum iteration.
// Veracity: USED

pub open spec fn spec_nat_seq_sum(s: Seq<nat>) -> nat { s.fold_left(0nat, |acc: nat, v: nat| (acc + v) as nat) }
pub open spec fn spec_nat_set_sum(s: Set<nat>) -> nat { s.fold     (0nat, |acc: nat, v: nat| (acc + v) as nat) }

/// Sublemma: prove seq.fold_left == seq.to_set().fold for no-dup nat sequences
pub proof fn lemma_spec_nat_seq_fold_equals_spec_set_fold(seq: Seq<nat>)
    requires
        seq.no_duplicates(),
    ensures
        spec_nat_seq_sum(seq) == spec_nat_set_sum(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: nat, v: nat| (acc + v) as nat;
    
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<nat, nat>(0nat, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        
        assert(seq =~= prefix.push(last));
        
        assert(prefix.no_duplicates()) by {
            assert forall |i: int, j: int| 
                0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j
                implies prefix[i] != prefix[j] by {};
        };
        
        assert(!prefix.contains(last)) by {
            if prefix.contains(last) {
                let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last;
// Veracity: UNNEEDED assert                 assert(seq[i] != seq[n]);
            }
        };
        
        lemma_spec_nat_seq_fold_equals_spec_set_fold(prefix);
        lemma_nat_fold_left_step(seq, n);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0nat, f, last);
    }
// Veracity: USED
}

proof fn lemma_to_seq_no_duplicates<T>(s: Set<T>)
    requires
        s.finite(),
    ensures
        s.to_seq().no_duplicates(),
    decreases s.len(),
{
    broadcast use vstd::set_lib::group_set_lib_default;
    
    if s.len() == 0 {
// Veracity: UNNEEDED assert         assert(s.to_seq() =~= Seq::empty());
    } else {
        let x = s.choose();
        let rest = s.remove(x);
        lemma_to_seq_no_duplicates(rest);
// Veracity: UNNEEDED assert         assert(s.to_seq() =~= Seq::empty().push(x) + rest.to_seq());
        rest.lemma_to_seq_to_set_id();
        assert(!rest.to_seq().contains(x)) by {
            if rest.to_seq().contains(x) {
// Veracity: UNNEEDED assert                 assert(rest.to_seq().to_set().contains(x));
// Veracity: UNNEEDED assert                 assert(rest.contains(x));
            }
        };
        
        let prefix = Seq::empty().push(x);
        let suffix = rest.to_seq();
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates());
// Veracity: UNNEEDED assert         assert(suffix.no_duplicates());
        
        // No overlap between prefix and suffix
        assert forall |i: int, j: int| 
            0 <= i < prefix.len() && 0 <= j < suffix.len() 
            implies prefix[i] != suffix[j] by {
// Veracity: UNNEEDED assert             assert(prefix[i] == x);
// Veracity: UNNEEDED assert             assert(!suffix.contains(x));
        };
        
// Veracity: USED
        vstd::seq_lib::lemma_no_dup_in_concat(prefix, suffix);
    }
}

pub proof fn lemma_spec_nat_seq_sum_is_nat_set_sum(s: Set<nat>)
    requires
        s.finite(),
    ensures
        spec_nat_seq_sum(s.to_seq()) == spec_nat_set_sum(s),
{
    let seq = s.to_seq();
    lemma_to_seq_no_duplicates(s);
    s.lemma_to_seq_to_set_id();
    lemma_spec_nat_seq_fold_equals_spec_set_fold(seq);
}

// Veracity: USED

// Part 1: Nat sum monotonicity - if total sum fits, no intermediate overflow

// Veracity: USED
pub proof fn lemma_nat_partial_sum_monotonic(seq: Seq<nat>, i: int, j: int)
    requires
        0 <= i <= j <= seq.len(),
    ensures
        spec_nat_seq_sum(seq.take(i)) <= spec_nat_seq_sum(seq.take(j)),
    decreases j - i,
{
    if i == j {
    } else {
        lemma_nat_partial_sum_monotonic(seq, i, j - 1);
        lemma_nat_fold_left_step(seq, j - 1);
    }
}

pub proof fn lemma_nat_fold_left_step(seq: Seq<nat>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        spec_nat_seq_sum(seq.take(n + 1)) == spec_nat_seq_sum(seq.take(n)) + seq[n],
{
    broadcast use vstd::seq_lib::group_seq_properties;
    
    let prefix = seq.take(n);
    let suffix = seq.subrange(n, n + 1);
    let f = |acc: nat, v: nat| acc + v;
    
    // Use lemma_fold_left_split: fold(take(n+1)) = fold(suffix, fold(prefix))
    seq.take(n + 1).lemma_fold_left_split(0nat, f, n);
    
    // take(n+1).subrange(0, n) == take(n)
    assert(seq.take(n + 1).subrange(0, n) =~= prefix);
    
    // take(n+1).subrange(n, n+1) == [seq[n]]
// Veracity: UNNEEDED assert     assert(seq.take(n + 1).subrange(n, n + 1) =~= Seq::empty().push(seq[n]));
    
    // fold([x], acc) = acc + x
// Veracity: UNNEEDED assert     assert(Seq::empty().push(seq[n]).fold_left(prefix.fold_left(0nat, f), f) 
// Veracity: UNNEEDED assert         == prefix.fold_left(0nat, f) + seq[n]);
}

/// The main theorem: If the total sum of nat values fits in MAX, then ALL partial sums fit.
/// This means no intermediate overflow regardless of ordering!
// Veracity: USED

pub proof fn lemma_spec_nat_seq_sum_no_intermediate_overflow(seq: Seq<nat>, max: nat)
    requires
        spec_nat_seq_sum(seq) <= max,
    ensures
        forall |i: int| 0 <= i <= seq.len() ==> spec_nat_seq_sum(#[trigger] seq.take(i)) <= max,
{
    assert forall |i: int| 0 <= i <= seq.len() implies spec_nat_seq_sum(#[trigger] seq.take(i)) <= max by {
// Veracity: USED
        lemma_nat_partial_sum_monotonic(seq, i, seq.len() as int);
// Veracity: UNNEEDED assert         assert(seq.take(seq.len() as int) =~= seq);
    }
}

/// Corollary: For any permutation/reordering of a nat sequence, 
/// if the total sum fits in max, all partial sums of ANY ordering fit.
/// 
/// This is the key result: for nat, if total fits, ANY fold order works!
pub proof fn lemma_nat_any_order_no_overflow(s1: Seq<nat>, s2: Seq<nat>, max: nat)
    requires
        s1.to_set() == s2.to_set(),
        s1.no_duplicates(),
        s2.no_duplicates(),
        spec_nat_seq_sum(s1) <= max,
    ensures
        spec_nat_seq_sum(s2) <= max,
        forall |i: int| 0 <= i <= s2.len() ==> spec_nat_seq_sum(#[trigger] s2.take(i)) <= max,
// Veracity: USED
{
    lemma_spec_nat_seq_sum_permutation_invariant(s1, s2);
    lemma_spec_nat_seq_sum_no_intermediate_overflow(s2, max);
}

pub proof fn lemma_no_dup_same_set_implies_same_multiset<T>(s1: Seq<T>, s2: Seq<T>)
    requires
        s1.to_set() == s2.to_set(),
        s1.no_duplicates(),
        s2.no_duplicates(),
    ensures
        s1.to_multiset() == s2.to_multiset(),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    // For no_duplicates sequences, to_multiset has count 0 or 1 for each element
    // If to_set is equal, the elements with count 1 are the same
    assert forall |x: T| s1.to_multiset().count(x) == s2.to_multiset().count(x) by {
        if s1.contains(x) {
// Veracity: UNNEEDED assert             assert(s1.to_set().contains(x));
            assert(s2.to_set().contains(x));
// Veracity: UNNEEDED assert             assert(s2.contains(x));
            // no_duplicates => count == 1
            s1.lemma_multiset_has_no_duplicates();
            s2.lemma_multiset_has_no_duplicates();
        } else {
            if s2.contains(x) {
// Veracity: UNNEEDED assert                 assert(s2.to_set().contains(x));
                assert(s1.to_set().contains(x));
// Veracity: UNNEEDED assert                 assert(s1.contains(x));
// Veracity: UNNEEDED assert                 assert(false);
            }
        }
    };
// Veracity: UNNEEDED assert     assert(s1.to_multiset() =~= s2.to_multiset());
}

// Veracity: USED
pub proof fn lemma_spec_nat_seq_sum_permutation_invariant(s1: Seq<nat>, s2: Seq<nat>)
    requires
        s1.to_set() == s2.to_set(),
        s1.no_duplicates(),
        s2.no_duplicates(),
    ensures
        spec_nat_seq_sum(s1) == spec_nat_seq_sum(s2),
{
    // no_duplicates + same to_set => same to_multiset
    lemma_no_dup_same_set_implies_same_multiset(s1, s2);
    
    // nat addition is commutative for fold_left
    let f = |acc: nat, v: nat| acc + v;
    assert(vstd::seq_lib::commutative_foldl(f)) by {
// Veracity: UNNEEDED assert         assert forall |x: nat, y: nat, v: nat| #[trigger] f(f(v, x), y) == f(f(v, y), x) by {};
    };
    
// Veracity: USED
    // Use vstd's permutation lemma
    vstd::seq_lib::lemma_fold_left_permutation(s1, s2, f, 0nat);
}

// Veracity: USED
// =============================================================================
// Lemmas for u32 view identity and set/seq membership equivalence
// =============================================================================

/// Lemma: for u32, seq.map(|_i, t| t@) =~= seq (view is identity)
pub proof fn lemma_u32_view_identity(seq: Seq<u32>)
    ensures seq.map(|_i: int, t: u32| t@) =~= seq,
{
    assert forall |i: int| 0 <= i < seq.len() implies 
        seq.map(|_i: int, t: u32| t@)[i] == #[trigger] seq[i] by {
// Veracity: UNNEEDED assert         assert(seq.map(|_i: int, t: u32| t@)[i] == seq[i]@);
    }
}

/// Lemma: connects to_seq postcondition to seq.to_set() == s for u32
pub proof fn lemma_to_seq_gives_same_set(s: Set<u32>, seq: Seq<u32>)
    requires 
        seq.no_duplicates(),
        forall |x: u32| s.contains(x) <==> seq.map(|_i: int, t: u32| t@).contains(x),
    ensures 
        seq.to_set() =~= s,
// Veracity: USED
{
    lemma_u32_view_identity(seq);
    let view_seq = seq.map(|_i: int, t: u32| t@);
    
    assert forall |x: u32| #![auto] s.contains(x) <==> seq.to_set().contains(x) by {
        if view_seq.contains(x) {
            let i = choose |i: int| 0 <= i < view_seq.len() && view_seq[i] == x;
// Veracity: UNNEEDED assert             assert(seq[i] == x);
        }
        if seq.contains(x) {
            let i = choose |i: int| 0 <= i < seq.len() && seq[i] == x;
// Veracity: UNNEEDED assert             assert(view_seq[i] == x);
        }
    }
}

/// Lemma: mapping then to_set equals to_set then map for u32 -> nat
pub proof fn lemma_seq_map_to_set_eq_set_map(seq: Seq<u32>, set: Set<u32>)
    requires 
        seq.no_duplicates(),
        seq.to_set() =~= set,
    ensures 
        seq.map(|_i: int, v: u32| v as nat).to_set() =~= set.map(|v: u32| v as nat),
{
    let mapped_seq = seq.map(|_i: int, v: u32| v as nat);
    let mapped_set = set.map(|v: u32| v as nat);
    
    assert forall |n: nat| #![auto] mapped_seq.to_set().contains(n) <==> mapped_set.contains(n) by {
        if mapped_seq.to_set().contains(n) {
            let i = choose |i: int| 0 <= i < seq.len() && (seq[i] as nat) == n;
            assert(set.contains(seq[i]));
        }
        if mapped_set.contains(n) {
            let v = choose |v: u32| set.contains(v) && (v as nat) == n;
// Veracity: UNNEEDED assert             assert(seq.to_set().contains(v));
            let i = choose |i: int| 0 <= i < seq.len() && seq[i] == v;
            assert(mapped_seq[i] == n);
        }
    }
}

// Veracity: USED
/// Lemma: for u32, s.to_seq() membership matches s membership via view map.
pub proof fn lemma_set_contains_iff_to_seq_map_contains(s: Set<u32>)
    requires 
        s.finite(),
    ensures
        forall |x: u32| s.contains(x) <==> s.to_seq().map(|_i: int, t: u32| t@).contains(x),
    decreases s.len(),
{
    lemma_u32_view_identity(s.to_seq());
    if s.len() == 0 {
// Veracity: UNNEEDED assert         assert(s =~= Set::empty());
// Veracity: UNNEEDED assert         assert(s.to_seq() =~= Seq::empty());
    } else {
        let x = s.choose();
        let smaller = s.remove(x);
        lemma_set_contains_iff_to_seq_map_contains(smaller);
        s.lemma_to_seq_to_set_id();
    }
}

// =============================================================================
// Weighted tuple fold lemmas - for summing weights in edge sequences/sets
// =============================================================================

/// Spec function: sum of third component over a sequence of triples
pub open spec fn spec_weighted_seq_sum<A, B>(seq: Seq<(A, B, u32)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u32)| acc + t.2 as nat)
}
// Veracity: USED

/// Spec function: sum of third component over a set of triples  
pub open spec fn spec_weighted_set_sum<A, B>(s: Set<(A, B, u32)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u32)| acc + t.2 as nat)
}
// Veracity: USED

/// Lemma: fold_left step for weighted tuple sequences
proof fn lemma_weighted_fold_left_step<A, B>(seq: Seq<(A, B, u32)>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        spec_weighted_seq_sum(seq.take(n + 1)) == spec_weighted_seq_sum(seq.take(n)) + seq[n].2 as nat,
{
    let prefix = seq.take(n);
    let f = |acc: nat, t: (A, B, u32)| acc + t.2 as nat;
    
    seq.take(n + 1).lemma_fold_left_split(0nat, f, n);
    assert(seq.take(n + 1).subrange(0, n) =~= prefix);
// Veracity: UNNEEDED assert     assert(seq.take(n + 1).subrange(n, n + 1) =~= Seq::empty().push(seq[n]));
}

/// Lemma: for no-dup sequences representing a set, seq fold equals set fold for weighted tuples
pub proof fn lemma_weighted_seq_fold_equals_set_fold<A, B>(seq: Seq<(A, B, u32)>)
    requires
        seq.no_duplicates(),
    ensures
        spec_weighted_seq_sum(seq) == spec_weighted_set_sum(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: nat, t: (A, B, u32)| (acc + t.2 as nat) as nat;
    
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, u32), nat>(0nat, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        
        assert(seq =~= prefix.push(last));
        
        // prefix has no duplicates (inherited from seq)
        assert(prefix.no_duplicates()) by {
            assert forall |i: int, j: int| 
                0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j
                implies prefix[i] != prefix[j] by {};
        };
        
        // last is not in prefix (from no_duplicates on seq)
        assert(!prefix.contains(last)) by {
            if prefix.contains(last) {
                let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last;
// Veracity: UNNEEDED assert                 assert(seq[i] != seq[n]);
            }
        };
        
        lemma_weighted_seq_fold_equals_set_fold(prefix);
        lemma_weighted_fold_left_step(seq, n);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
// Veracity: USED
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0nat, f, last);
    }
}

/// Lemma: weighted seq sum equals weighted set sum for a finite set
pub proof fn lemma_weighted_seq_sum_is_set_sum<A, B>(s: Set<(A, B, u32)>)
    requires
        s.finite(),
    ensures
        spec_weighted_seq_sum(s.to_seq()) == spec_weighted_set_sum(s),
// Veracity: USED
{
    let seq = s.to_seq();
    lemma_to_seq_no_duplicates(s);
    s.lemma_to_seq_to_set_id();
    lemma_weighted_seq_fold_equals_set_fold(seq);
}

/// Lemma: int fold equals nat fold as int for weighted sums
/// Since we're adding non-negative values, the int and nat accumulators stay in sync.
pub proof fn lemma_int_fold_equals_nat_fold_weighted<T: View<V = (A, B, u32)>, A, B>(seq: Seq<T>)
    ensures
        seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat)
            == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() == 0 {
// Veracity: USED
    } else {
        let prefix = seq.take((seq.len() - 1) as int);
        lemma_int_fold_equals_nat_fold_weighted::<T, A, B>(prefix);
    }
}

/// Lemma: fold_left adding e@.2 equals spec_weighted_seq_sum of the mapped sequence
/// This shows that folding over elements extracting weight via View 
/// equals the weighted sum of the view-mapped sequence.
pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum<T: View<V = (A, B, u32)>, A, B>(seq: Seq<T>)
    ensures
        seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) 
            == spec_weighted_seq_sum(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let f_orig = |acc: nat, e: T| acc + e@.2 as nat;
    let f_mapped = |acc: nat, t: (A, B, u32)| acc + t.2 as nat;
    let view_seq = seq.map(|_i: int, e: T| e@);
    
    if seq.len() == 0 {
// Veracity: UNNEEDED assert         assert(seq =~= Seq::empty());
// Veracity: UNNEEDED assert         assert(view_seq =~= Seq::empty());
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        
// Veracity: UNNEEDED assert         assert(seq =~= prefix.push(last));
        
        // Inductive hypothesis on prefix
        lemma_seq_fold_left_plus_is_weighted_seq_sum::<T, A, B>(prefix);
        
        // prefix.map(view) == view_seq.take(n)
// Veracity: USED
        assert(prefix.map(|_i: int, e: T| e@) =~= view_seq.take(n));
        
        // Show the fold equality extends with the last element
        // seq.fold_left(f_orig) = prefix.fold_left(f_orig) + last@.2
        // view_seq.fold_left(f_mapped) = view_seq.take(n).fold_left(f_mapped) + view_seq[n].2
        // And last@.2 == view_seq[n].2 since view_seq[n] = last@
// Veracity: UNNEEDED assert         assert(view_seq[n] == last@);
    }
}

/// Lemma: fold_left with int accumulator equals fold_left with nat accumulator cast to int
/// For functions that only add non-negative values, the results are equal.
pub proof fn lemma_fold_left_int_equals_nat_as_int<T: View<V = (A, B, u32)>, A, B>(seq: Seq<T>)
    ensures
        seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat)
            == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() == 0 {
// Veracity: UNNEEDED assert         assert(seq =~= Seq::empty());
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        
// Veracity: UNNEEDED assert         assert(seq =~= prefix.push(last));
        
        // Inductive hypothesis
        lemma_fold_left_int_equals_nat_as_int::<T, A, B>(prefix);
        
        // Both folds add the same value (last@.2 as nat) to their previous result
        // int version: prefix_result_int + last@.2 as nat
        // nat version: (prefix_result_nat + last@.2 as nat) as int
// Veracity: USED
        // By IH: prefix_result_int == prefix_result_nat as int
        // So both add the same value and produce equal results
    }
}

// ============================================================================
// Unsigned integer weighted sum lemmas (for u8, u16, u64, u128, usize)
// ============================================================================
// Veracity: USED

// u8
pub open spec fn spec_weighted_seq_sum_u8<A, B>(seq: Seq<(A, B, u8)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u8)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_u8<A, B>(s: Set<(A, B, u8)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u8)| acc + t.2 as nat)
}
// Veracity: USED
pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u8<T: View<V = (A, B, u8)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u8(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_u8::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
// USED: called from WeightedDirGraphStEphU8
pub proof fn lemma_fold_left_int_equals_nat_as_int_u8<T: View<V = (A, B, u8)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u8::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}
pub proof fn lemma_weighted_seq_fold_equals_set_fold_u8<A, B>(seq: Seq<(A, B, u8)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u8(seq) == spec_weighted_set_sum_u8(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: nat, t: (A, B, u8)| acc + t.2 as nat;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, u8), nat>(0nat, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_weighted_seq_fold_equals_set_fold_u8::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0nat, f, last);
    }
}

// u16
// Veracity: USED
// Veracity: USED
pub open spec fn spec_weighted_seq_sum_u16<A, B>(seq: Seq<(A, B, u16)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u16)| acc + t.2 as nat)
// Veracity: USED
}
pub open spec fn spec_weighted_set_sum_u16<A, B>(s: Set<(A, B, u16)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u16)| acc + t.2 as nat)
}
pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u16<T: View<V = (A, B, u16)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u16(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_u16::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
// USED: called from WeightedDirGraphStEphU16
pub proof fn lemma_fold_left_int_equals_nat_as_int_u16<T: View<V = (A, B, u16)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u16::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}
pub proof fn lemma_weighted_seq_fold_equals_set_fold_u16<A, B>(seq: Seq<(A, B, u16)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u16(seq) == spec_weighted_set_sum_u16(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: nat, t: (A, B, u16)| acc + t.2 as nat;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, u16), nat>(0nat, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_weighted_seq_fold_equals_set_fold_u16::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0nat, f, last);
    }
// Veracity: USED
}

// u64
pub open spec fn spec_weighted_seq_sum_u64<A, B>(seq: Seq<(A, B, u64)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u64)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_u64<A, B>(s: Set<(A, B, u64)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u64)| acc + t.2 as nat)
}
// Veracity: UNUSED pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u64<T: View<V = (A, B, u64)>, A, B>(seq: Seq<T>)
// Veracity: UNUSED     ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u64(seq.map(|_i: int, e: T| e@)),
// Veracity: UNUSED     decreases seq.len(),
// Veracity: UNUSED {
// Veracity: UNUSED     let view_seq = seq.map(|_i: int, e: T| e@);
// Veracity: UNUSED // Veracity: UNUSED     if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
// Veracity: UNUSED // Veracity: UNUSED     else {
// Veracity: UNUSED // Veracity: UNUSED         let n = (seq.len() - 1) as int;
// Veracity: UNUSED // Veracity: UNUSED         lemma_seq_fold_left_plus_is_weighted_seq_sum_u64::<T, A, B>(seq.take(n));
// Veracity: UNUSED // Veracity: UNUSED         assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNUSED // Veracity: UNUSED         assert(view_seq[n] == seq[n]@);
// Veracity: UNUSED     }
// Veracity: UNUSED }
// USED: called from WeightedDirGraphStEphU64
pub proof fn lemma_fold_left_int_equals_nat_as_int_u64<T: View<V = (A, B, u64)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u64::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}
pub proof fn lemma_weighted_seq_fold_equals_set_fold_u64<A, B>(seq: Seq<(A, B, u64)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u64(seq) == spec_weighted_set_sum_u64(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: nat, t: (A, B, u64)| acc + t.2 as nat;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, u64), nat>(0nat, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_weighted_seq_fold_equals_set_fold_u64::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0nat, f, last);
    }
// Veracity: USED
}
// Veracity: USED

// u128
pub open spec fn spec_weighted_seq_sum_u128<A, B>(seq: Seq<(A, B, u128)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u128)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_u128<A, B>(s: Set<(A, B, u128)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u128)| acc + t.2 as nat)
}
pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u128<T: View<V = (A, B, u128)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u128(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
// Veracity: USED
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_u128::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
// USED: called from WeightedDirGraphStEphU128
pub proof fn lemma_fold_left_int_equals_nat_as_int_u128<T: View<V = (A, B, u128)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u128::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}
pub proof fn lemma_weighted_seq_fold_equals_set_fold_u128<A, B>(seq: Seq<(A, B, u128)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u128(seq) == spec_weighted_set_sum_u128(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: nat, t: (A, B, u128)| acc + t.2 as nat;
    if seq.len() == 0 {
// Veracity: USED
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, u128), nat>(0nat, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_weighted_seq_fold_equals_set_fold_u128::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: USED
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0nat, f, last);
    }
}

// usize
pub open spec fn spec_weighted_seq_sum_usize<A, B>(seq: Seq<(A, B, usize)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, usize)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_usize<A, B>(s: Set<(A, B, usize)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, usize)| acc + t.2 as nat)
}
pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_usize<T: View<V = (A, B, usize)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_usize(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_usize::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
// Veracity: USED
}
// USED: called from WeightedDirGraphStEphUsize
pub proof fn lemma_fold_left_int_equals_nat_as_int_usize<T: View<V = (A, B, usize)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_usize::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}
pub proof fn lemma_weighted_seq_fold_equals_set_fold_usize<A, B>(seq: Seq<(A, B, usize)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_usize(seq) == spec_weighted_set_sum_usize(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: nat, t: (A, B, usize)| acc + t.2 as nat;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, usize), nat>(0nat, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_weighted_seq_fold_equals_set_fold_usize::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
// Veracity: USED
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0nat, f, last);
// Veracity: USED
    }
}

// ============================================================================
// Signed integer weighted sum lemmas (for each signed integer type)
// ============================================================================

// i8
pub open spec fn spec_signed_weighted_seq_sum_i8<A, B>(seq: Seq<(A, B, i8)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i8)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i8<A, B>(s: Set<(A, B, i8)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i8)| acc + t.2 as int)
}
pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i8<T: View<V = (A, B, i8)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i8(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i8::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i8<A, B>(seq: Seq<(A, B, i8)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_i8(seq) == spec_signed_weighted_set_sum_i8(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: int, t: (A, B, i8)| acc + t.2 as int;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, i8), int>(0int, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
// Veracity: USED
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_signed_weighted_seq_fold_equals_set_fold_i8::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0int, f, last);
    }
}

// i16
pub open spec fn spec_signed_weighted_seq_sum_i16<A, B>(seq: Seq<(A, B, i16)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i16)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i16<A, B>(s: Set<(A, B, i16)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i16)| acc + t.2 as int)
}
// Veracity: UNUSED pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i16<T: View<V = (A, B, i16)>, A, B>(seq: Seq<T>)
// Veracity: UNUSED     ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i16(seq.map(|_i: int, e: T| e@)),
// Veracity: UNUSED     decreases seq.len(),
// Veracity: UNUSED {
// Veracity: UNUSED     let view_seq = seq.map(|_i: int, e: T| e@);
// Veracity: UNUSED     if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
// Veracity: UNUSED     else {
// Veracity: UNUSED         let n = (seq.len() - 1) as int;
// Veracity: UNUSED         lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i16::<T, A, B>(seq.take(n));
// Veracity: UNUSED         assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNUSED         assert(view_seq[n] == seq[n]@);
// Veracity: UNUSED     }
// Veracity: UNUSED }
pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i16<A, B>(seq: Seq<(A, B, i16)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_i16(seq) == spec_signed_weighted_set_sum_i16(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: int, t: (A, B, i16)| acc + t.2 as int;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, i16), int>(0int, f);
// Veracity: USED
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
// Veracity: USED
        lemma_signed_weighted_seq_fold_equals_set_fold_i16::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0int, f, last);
    }
}

// i32 (original names for backwards compatibility)
pub open spec fn spec_signed_weighted_seq_sum<A, B>(seq: Seq<(A, B, i32)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i32)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum<A, B>(s: Set<(A, B, i32)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i32)| acc + t.2 as int)
}
pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum<T: View<V = (A, B, i32)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold<A, B>(seq: Seq<(A, B, i32)>)
// Veracity: USED
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum(seq) == spec_signed_weighted_set_sum(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: int, t: (A, B, i32)| acc + t.2 as int;
    if seq.len() == 0 {
// Veracity: USED
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, i32), int>(0int, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_signed_weighted_seq_fold_equals_set_fold::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0int, f, last);
    }
}

// i64
pub open spec fn spec_signed_weighted_seq_sum_i64<A, B>(seq: Seq<(A, B, i64)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i64)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i64<A, B>(s: Set<(A, B, i64)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i64)| acc + t.2 as int)
}
pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i64<T: View<V = (A, B, i64)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i64(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i64::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i64<A, B>(seq: Seq<(A, B, i64)>)
    requires seq.no_duplicates(),
// Veracity: USED
    ensures spec_signed_weighted_seq_sum_i64(seq) == spec_signed_weighted_set_sum_i64(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: int, t: (A, B, i64)| acc + t.2 as int;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, i64), int>(0int, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
// Veracity: USED
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_signed_weighted_seq_fold_equals_set_fold_i64::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0int, f, last);
    }
}

// i128
pub open spec fn spec_signed_weighted_seq_sum_i128<A, B>(seq: Seq<(A, B, i128)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i128)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i128<A, B>(s: Set<(A, B, i128)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i128)| acc + t.2 as int)
}
pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i128<T: View<V = (A, B, i128)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i128(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i128::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i128<A, B>(seq: Seq<(A, B, i128)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_i128(seq) == spec_signed_weighted_set_sum_i128(seq.to_set()),
// Veracity: USED
    decreases seq.len(),
{
    let f = |acc: int, t: (A, B, i128)| acc + t.2 as int;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, i128), int>(0int, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: USED
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_signed_weighted_seq_fold_equals_set_fold_i128::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0int, f, last);
    }
}

// isize
pub open spec fn spec_signed_weighted_seq_sum_isize<A, B>(seq: Seq<(A, B, isize)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, isize)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_isize<A, B>(s: Set<(A, B, isize)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, isize)| acc + t.2 as int)
}
pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_isize<T: View<V = (A, B, isize)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_isize(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_isize::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
// Veracity: UNNEEDED assert         assert(view_seq[n] == seq[n]@);
    }
}
pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_isize<A, B>(seq: Seq<(A, B, isize)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_isize(seq) == spec_signed_weighted_set_sum_isize(seq.to_set()),
    decreases seq.len(),
{
    let f = |acc: int, t: (A, B, isize)| acc + t.2 as int;
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::empty());
        vstd::set::fold::lemma_fold_empty::<(A, B, isize), int>(0int, f);
    } else {
        let n = (seq.len() - 1) as int;
        let prefix = seq.take(n);
        let last = seq[n];
        assert(seq =~= prefix.push(last));
// Veracity: UNNEEDED assert         assert(prefix.no_duplicates()) by { assert forall |i: int, j: int| 0 <= i < prefix.len() && 0 <= j < prefix.len() && i != j implies prefix[i] != prefix[j] by {}; };
// Veracity: UNNEEDED assert         assert(!prefix.contains(last)) by { if prefix.contains(last) { let i = choose |i: int| 0 <= i < prefix.len() && prefix[i] == last; assert(seq[i] != seq[n]); } };
        lemma_signed_weighted_seq_fold_equals_set_fold_isize::<A, B>(prefix);
        lemma_push_not_contains_to_set(prefix, last);
// Veracity: UNNEEDED assert         assert(vstd::set::fold::is_fun_commutative(f)) by {};
// Veracity: UNNEEDED assert         assert(!prefix.to_set().contains(last)) by {};
        vstd::seq_lib::seq_to_set_is_finite(prefix);
        vstd::set::fold::lemma_fold_insert(prefix.to_set(), 0int, f, last);
    }
}

} // verus!
