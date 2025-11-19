//! Lemmas about the relationship between Seq operations (take, skip, push) and to_set()
// Note: These are regular proof functions, not broadcast.
// Broadcast versions caused massive performance issues (30M+ rlimit).

use vstd::prelude::*;

verus! {

/// If a sequence contains an element at index i, then that element is in the sequence's set view.
pub proof fn lemma_seq_index_in_to_set<T>(seq: Seq<T>, i: int)
    requires
        0 <= i < seq.len(),
    ensures
        seq.to_set().contains(seq[i]),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    assert(seq.contains(seq[i]));
}

/// If a sequence does not contain an element v, then pushing v onto the sequence
/// creates a subset of the original set with v inserted.
pub proof fn lemma_push_not_contains_to_set_subset<T>(seq: Seq<T>, v: T)
    requires
        !seq.contains(v),
    ensures
        seq.push(v).to_set() <= seq.to_set().insert(v),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    assert forall |x: T| seq.push(v).to_set().contains(x) 
        implies seq.to_set().insert(v).contains(x) by {
        if seq.push(v).contains(x) {
            if x == v {
                assert(seq.to_set().insert(v).contains(v));
            } else {
                assert(seq.contains(x));
                assert(seq.to_set().contains(x));
                assert(seq.to_set().insert(v).contains(x));
            }
        }
    }
}

/// If a sequence does not contain an element v, then the original set with v inserted
/// is a subset of pushing v onto the sequence.
pub proof fn lemma_push_not_contains_to_set_superset<T>(seq: Seq<T>, v: T)
    requires
        !seq.contains(v),
    ensures
        seq.to_set().insert(v) <= seq.push(v).to_set(),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    
    assert forall |x: T| seq.to_set().insert(v).contains(x) 
        implies seq.push(v).to_set().contains(x) by {
        if x == v {
            assert(seq.push(v)[seq.len() as int] == v);
            assert(seq.push(v).contains(v));
        } else if seq.to_set().contains(x) {
            assert(seq.contains(x));
            let idx = seq.lemma_contains_to_index(x);
            assert(seq.push(v)[idx] == x);
            assert(seq.push(v).contains(x));
        }
    }
}

/// If a sequence does not contain an element v, then pushing v onto the sequence
/// creates a set equal to the original set with v inserted.
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
pub proof fn lemma_take_full<T>(seq: Seq<T>)
    ensures
        seq.take(seq.len() as int) == seq,
{
    broadcast use vstd::seq_lib::group_seq_properties;
    assert(seq.take(seq.len() as int) =~= seq);
}

/// Taking the full length of a sequence and converting to a set yields the same set.
pub proof fn lemma_take_full_to_set<T>(seq: Seq<T>)
    ensures
        seq.take(seq.len() as int).to_set() == seq.to_set(),
{
    lemma_take_full(seq);
}

/// If two sequences are equal, their set views are equal.
pub proof fn lemma_seq_equal_to_set_equal<T>(s1: Seq<T>, s2: Seq<T>)
    requires
        s1 == s2,
    ensures
        s1.to_set() == s2.to_set(),
{
}

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
    assert forall |i: int| 0 <= i < n implies #[trigger] prefix_n_plus_1[i] == prefix_n[i] by {}
    assert(prefix_n_plus_1[n] == seq[n]);
    
    assert forall |x: T| #[trigger] prefix_n.to_set().insert(seq[n]).contains(x) 
        implies prefix_n_plus_1.to_set().contains(x) by {
        if x == seq[n] {
            assert(prefix_n_plus_1[n] == x);
            assert(prefix_n_plus_1.contains(x));
        } else if prefix_n.to_set().contains(x) {
            assert(prefix_n.contains(x));
            let idx = prefix_n.lemma_contains_to_index(x);
            assert(0 <= idx < n);
            assert(prefix_n_plus_1[idx] == x);
            assert(prefix_n_plus_1.contains(x));
        }
    }
}

/// After taking n+1 elements, the result is a subset of take(n) with seq[n] inserted.
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
        assert(prefix_n_plus_1.contains(x));
        let idx = prefix_n_plus_1.lemma_contains_to_index(x);
        if idx < n {
            assert(prefix_n[idx] == x);
            assert(prefix_n.contains(x));
            assert(prefix_n.to_set().contains(x));
        } else {
            assert(idx == n);
            assert(x == seq[n]);
        }
    }
}

/// After taking n elements and then taking n+1 elements (where n < len),
/// the additional element at index n is in the larger set.
pub proof fn lemma_take_one_more_extends_the_seq_set<T>(seq: Seq<T>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n).to_set().insert(seq[n]) == seq.take(n+1).to_set(),
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
    assert forall |x| s.insert(v).contains(x) implies #[trigger] s.contains(x) by {};
    assert forall |x| #[trigger] s.contains(x) implies s.insert(v).contains(x) by {};
    broadcast use vstd::set::group_set_axioms;
}

} // verus!

