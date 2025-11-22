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
    
    assert forall |x: T| #[trigger] seq.push(v).to_set().contains(x) 
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
    
    assert forall |x: T| #[trigger] seq.to_set().insert(v).contains(x) 
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
        assert(seq.take(n+1)[i] == seq.take(n)[i]);
        assert(mapped_n_plus_1[i] == seq.take(n+1)[i]@);
        assert(mapped_n[i] == seq.take(n)[i]@);
    }
    assert(mapped_n_plus_1[n] == seq[n]@);
    
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
            assert(mapped_n[idx] == x);
        } else {
            assert(idx == n);
            assert(x == seq[n]@);
        }
    }
}

/// Taking the full length of a sequence, mapping through view, and converting to set
/// yields the same set as mapping the full sequence through view and converting to set.
pub proof fn lemma_take_full_to_set_with_view<T: View>(seq: Seq<T>)
    ensures
        seq.take(seq.len() as int).map(|i: int, k: T| k@).to_set() == seq.map(|i: int, k: T| k@).to_set(),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    assert(seq.take(seq.len() as int) =~= seq);
}

/// Proves that a sequence mapped through view equals a target set when bidirectional containment holds.
/// Lemma: If i is a valid index, then seq.map(...)[i] is in seq.map(...).to_set()
pub proof fn lemma_seq_index_in_map_to_set<T: View>(seq: Seq<T>, i: int)
    requires
        0 <= i < seq.len(),
    ensures
        seq.map(|i: int, k: T| k@).to_set().contains(seq[i]@),
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let mapped_seq = seq.map(|i: int, k: T| k@);
    assert(mapped_seq[i] == seq[i]@);
    assert(mapped_seq.to_set().contains(mapped_seq[i]));
}

/// Lemma: If s is in seq.map(...).to_set(), then there exists an index i such that s == seq[i]@
pub proof fn lemma_map_to_set_contains_index<T: View>(seq: Seq<T>, s: T::V)
    requires
        seq.map(|i: int, k: T| k@).to_set().contains(s),
    ensures
        exists |i: int| #![auto] 0 <= i < seq.len() && s == seq[i]@,
{
    broadcast use vstd::seq_lib::group_seq_properties;
    broadcast use vstd::set::group_set_axioms;
    
    let mapped_seq = seq.map(|i: int, k: T| k@);
    assert(mapped_seq.to_set().contains(s));
    let idx = mapped_seq.lemma_contains_to_index(s);
    assert(mapped_seq[idx] == s);
    assert(seq[idx]@ == s);
    assert(0 <= idx < seq.len());
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
            assert(seq[idx] == k);
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
    assert(set_n_plus_1 == set_n.insert(seq[n]@));
    
    if s2.contains(seq[n]@) {
        // Case 1: seq[n]@ is in s2
        // (A ∪ {x}) ∩ B = (A ∩ B) ∪ {x} when x ∈ B
        assert forall |v: T::V| #[trigger] set_n_plus_1.intersect(s2).contains(v) 
            implies set_n.intersect(s2).insert(seq[n]@).contains(v) by {
            assert(set_n_plus_1.contains(v) && s2.contains(v));
            if v == seq[n]@ {
                assert(set_n.intersect(s2).insert(seq[n]@).contains(v));
            } else {
                assert(set_n.contains(v));
                assert(set_n.intersect(s2).contains(v));
            }
        }
        
        assert forall |v: T::V| #[trigger] set_n.intersect(s2).insert(seq[n]@).contains(v)
            implies set_n_plus_1.intersect(s2).contains(v) by {
            if v == seq[n]@ {
                assert(set_n_plus_1.contains(v));
                assert(s2.contains(v));
            } else if set_n.intersect(s2).contains(v) {
                assert(set_n.contains(v));
                assert(set_n_plus_1.contains(v));
            }
        }
    } else {
        // Case 2: seq[n]@ is not in s2
        // (A ∪ {x}) ∩ B = A ∩ B when x ∉ B
        assert forall |v: T::V| #[trigger] set_n_plus_1.intersect(s2).contains(v)
            implies set_n.intersect(s2).contains(v) by {
            assert(set_n_plus_1.contains(v) && s2.contains(v));
            assert(v != seq[n]@); // because seq[n]@ is not in s2
            assert(set_n.contains(v));
        }
        
        assert forall |v: T::V| #[trigger] set_n.intersect(s2).contains(v)
            implies set_n_plus_1.intersect(s2).contains(v) by {
            assert(set_n.contains(v) && s2.contains(v));
            assert(set_n_plus_1.contains(v));
        }
    }
}

} // verus!

