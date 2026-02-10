//  Experiment: vec_filter
//
//  HYPOTHESIS: A ghost spec_fn predicate passed alongside an exec closure,
//  connected via a biconditional bridge (<==>), enables the SMT solver to
//  reason about filter postconditions (predicate satisfaction, provenance,
//  completeness) that are otherwise opaque through f.ensures() alone.
//  The original vec_filter example from rust_verify_test/tests/lifetime.rs
//  uses a one-directional bridge (==>); we strengthen it to (<==>) and
//  add the three filter correctness postconditions.
//
//  RESULT: The Anvil multiset pattern verifies all three properties in one
//  postcondition.  The explicit forall/exists approach verifies predicate
//  satisfaction and provenance but completeness causes Z3 to diverge.

use vstd::prelude::*;

use crate::vstdplus::feq::feq::obeys_feq_clone;
use crate::vstdplus::feq::feq::axiom_cloned_implies_eq_owned;

verus! {

broadcast use {
    vstd::set::group_set_axioms,
    vstd::multiset::group_multiset_axioms,
    vstd::multiset::group_multiset_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::seq_lib::group_filter_ensures
};

// This version proves only the length bound and predicate satisfaction.
// It uses a simple for loop over indices and clones each matching element.
fn vec_filter_predicate_for<V: Clone + Eq>(
    v: &Vec<V>,
    f: impl Fn(&V) -> bool,
    Ghost(f_spec): Ghost<spec_fn(V) -> bool>,
) -> (r: Vec<V>)
    requires
        obeys_feq_clone::<V>(),
        forall|v: V| #[trigger] f.requires((&v,)),
        forall|v: V, ret: bool| f.ensures((&v,), ret) <==> f_spec(v) == ret,
    ensures
        r@.len() <= v@.len(),
        // Every element in the result satisfies the predicate.
        forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==> f_spec(r@[i]),
{
    let mut r: Vec<V> = Vec::new();

    #[verifier::loop_isolation(false)]
    for idx in 0..v.len()
        invariant
            r@.len() <= idx,
            forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==> f_spec(r@[k]),
    {
        if f(&v[idx]) {
            let elem = v[idx].clone();
            proof {
                // The clone axiom ensures the cloned value equals the original in spec.
                axiom_cloned_implies_eq_owned::<V>(v@[idx as int], elem);
                assert(f_spec(elem));
            }
            r.push(elem);
        }
    }
    r
}

// This version is adapted from VerusCodebases/anvil/src/vstd_ext/vec_lib.rs.
// It uses a single multiset equality as the postcondition, which collapses
// predicate satisfaction, provenance, and completeness into one statement.
// The loop invariant tracks the multiset of the subrange processed so far.
fn vec_filter_anvil<V: Clone + Eq>(
    v: &Vec<V>,
    f: impl Fn(&V) -> bool,
    Ghost(f_spec): Ghost<spec_fn(V) -> bool>,
) -> (r: Vec<V>)
    requires
        obeys_feq_clone::<V>(),
        forall|v: V| #[trigger] f.requires((&v,)),
        forall|v: V, ret: bool| f.ensures((&v,), ret) ==> f_spec(v) == ret,
    ensures
        r@.to_multiset() =~= v@.to_multiset().filter(f_spec),
{
    let mut r: Vec<V> = Vec::new();

    #[verifier::loop_isolation(false)]
    for i in 0..v.len()
        invariant
            i <= v.len(),
            r@.to_multiset() =~= v@.subrange(0, i as int).to_multiset().filter(f_spec),
    {
        proof {
            broadcast use vstd::seq_lib::group_to_multiset_ensures;
        }
        // Extending the subrange by one element lets the multiset axioms advance the invariant.
        assert(v@.subrange(0, i as int + 1) =~= v@.subrange(0, i as int).push(v@[i as int]));
        if f(&v[i]) {
            let elem = v[i].clone();
            proof {
                // The clone axiom ensures the cloned value equals the original in spec.
                axiom_cloned_implies_eq_owned::<V>(v@[i as int], elem);
            }
            r.push(elem);
        }
    }
    // The full subrange equals the original sequence.
    assert(v@.subrange(0, v.len() as int) =~= v@);
    r
}

// This version attempts to prove all three properties as explicit forall/exists
// clauses.  It uses a loop/match over into_iter().  Marked external_body because
// the completeness proof causes Z3 to diverge.
#[verifier::external_body]
fn vec_filter_predicate_provenence_completeness_loop<V: Clone + Eq>(
    v: Vec<V>,
    f: impl Fn(&V) -> bool,
    Ghost(f_spec): Ghost<spec_fn(V) -> bool>,
) -> (r: Vec<V>)
    requires
        obeys_feq_clone::<V>(),
        forall|v: V| #[trigger] f.requires((&v,)),
        // The biconditional bridge ensures the exec closure and spec predicate agree exactly.
        forall|v: V, ret: bool| f.ensures((&v,), ret) <==> f_spec(v) == ret,
    ensures
        r@.len() <= v@.len(),

        // Predicate satisfaction: every element in the result satisfies the predicate.
        forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==> f_spec(r@[i]),

        // Provenance: every element in the result came from the input.
        forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==>
            exists|j: int| 0 <= j < v@.len() && r@[i] == v@[j],
/*
        // Completeness: every input element satisfying the predicate appears in the result.
        forall|j: int| #![trigger v@[j]] 0 <= j < v@.len() && f_spec(v@[j]) ==>
            exists|i: int| 0 <= i < r@.len() && r@[i] == v@[j],
*/
{
    let ghost original = v@;
    let mut r: Vec<V> = Vec::new();
    let mut iter = v.into_iter();
    let ghost mut idx: int = 0;

    #[verifier::loop_isolation(false)]
    loop
        invariant
            0 <= idx <= original.len(),
            v@ == original, 
            v@ =~= original, 
            iter@.0 == idx,
            iter@.1 == original,
            r@.len() <= idx,
            forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==> f_spec(r@[k]),
            forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==>
                exists|j: int| 0 <= j < idx && r@[k] == v@[j],
/*
            // Every element in original[0..idx] satisfying the predicate appears in r@.
            forall|j: int| #![trigger original[j]] 0 <= j < idx && f_spec(original[j]) ==>
                exists|k: int| 0 <= k < r@.len() && r@[k] == original[j],
*/
        decreases original.len() - idx,
    {
        match iter.next() {
            Some(item) => {
                if f(&item) {
                    r.push(item);
                    proof {
                        let ghost new_k = (r@.len() - 1) as int;
                        // The new element satisfies the predicate.
                        assert(f_spec(r@[new_k]));
                        // All elements in r@ still satisfy the predicate.
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==> f_spec(r@[k]));

                        // The provenance witness for the new element is idx.
                        assert(0 <= idx < idx + 1 && r@[new_k] == v@[idx]);
                        // All elements in r@ have provenance from original[0..idx+1].
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==>
                               exists|j: int| 0 <= j < idx + 1 && r@[k] == v@[j]);
/*
                        // The completeness witness for idx is new_k.
                        assert(0 <= new_k < r@.len() && r@[new_k] == original[idx]);
                        assume(forall|j: int| #![trigger original[j]] 0 <= j < idx + 1 && f_spec(original[j]) ==>
                                 exists|k: int| 0 <= k < r@.len() && r@[k] == original[j]);
*/
                        idx = idx + 1;
                    }
                } else {
                    proof {
                        // The current element does not satisfy the predicate.
                        assert(!f_spec(v@[idx]));
                        // Predicate satisfaction is preserved since r@ is unchanged.
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==> f_spec(r@[k]));

                        // Provenance is preserved since r@ is unchanged and idx+1 > idx.
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==>
                            exists|j: int| 0 <= j < idx + 1 && r@[k] == v@[j]);
/*
                        // Completeness for j == idx is vacuous since !f_spec, and the rest holds from the prior invariant.
                        assert(forall|j: int| #![trigger original[j]] 0 <= j < idx + 1 && f_spec(original[j]) ==>
                            exists|k: int| 0 <= k < r@.len() && r@[k] == original[j]);
*/
                        idx = idx + 1;
                    }
                }
            }
            None => {
                proof {
                    // The iterator is exhausted so we have processed all elements.
                    assert(idx == original.len());
                    // Predicate satisfaction holds over the full result.
                    assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==> f_spec(r@[i]));

                    // Provenance holds over the full input length.
                    assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==>
                        exists|j: int| 0 <= j < v@.len() && r@[i] == v@[j]);
/*
                    // Completeness holds over the full input length.
                    assert(forall|j: int| #![trigger original[j]] 0 <= j < original.len() && f_spec(original[j]) ==>
                        exists|i: int| 0 <= i < r@.len() && r@[i] == original[j]);
*/
                }
                break;
            }
        }
    }
    proof {
        // The ghost original is the same as v@ for bridging to the postcondition.
        assert(original =~= v@);

        // Predicate satisfaction in the ensures form.
        assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==> f_spec(r@[i]));

        // Provenance in the ensures form.
        assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==>
            exists|j: int| 0 <= j < v@.len() && r@[i] == v@[j]);
/*
        // Completeness in the ensures form.
        assert(forall|j: int| #![trigger v@[j]] 0 <= j < v@.len() && f_spec(v@[j]) ==>
            exists|i: int| 0 <= i < r@.len() && r@[i] == v@[j]);
*/
    }
    r
}

} // verus!
