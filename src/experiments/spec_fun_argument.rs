//  Experiment: spec_fun_argument
//
//  HYPOTHESIS: A ghost spec_fn predicate passed alongside an exec closure,
//  connected via a biconditional bridge (<==>), enables the SMT solver to
//  reason about filter postconditions (predicate satisfaction, provenance,
//  completeness) that are otherwise opaque through f.ensures() alone.
//  The original vec_filter example from rust_verify_test/tests/lifetime.rs
//  uses a one-directional bridge (==>); we strengthen it to (<==>) and
//  add the three filter correctness postconditions.
//
//  RESULT: (pending validation)

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

#[verifier::external_body]
fn vec_filter_predicate_provenence_completeness_loop<V: Clone + Eq>(
    v: Vec<V>,
    f: impl Fn(&V) -> bool,
    Ghost(f_spec): Ghost<spec_fn(V) -> bool>,
) -> (r: Vec<V>)
    requires
        obeys_feq_clone::<V>(),
        forall|v: V| #[trigger] f.requires((&v,)),
        // Biconditional bridge: exec closure and spec predicate agree exactly.
        forall|v: V, ret: bool| f.ensures((&v,), ret) <==> f_spec(v) == ret,
    ensures
        r@.len() <= v@.len(),

        // Predicate satisfaction — every element in the result satisfies the predicate.
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
            obeys_feq_clone::<V>(),
            forall|v: V| #[trigger] f.requires((&v,)),
            forall|v: V, ret: bool| f.ensures((&v,), ret) <==> f_spec(v) == ret,

            // Predicate satisfaction — every element in r@ satisfies the spec predicate.
            forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==> f_spec(r@[k]),

            // Provenance: every element in r@ came from original[0..idx].
            forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==>
                exists|j: int| 0 <= j < idx && r@[k] == v@[j],
/*
            // Completeness: every element in original[0..idx] satisfying the predicate appears in r@.
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
                        // Predicate satisfaction over [0..idx+1].
                        assert(f_spec(r@[new_k]));
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==> f_spec(r@[k]));

                        // Provenance over [0..idx+1]: witness for new element is idx.
                        assert(0 <= idx < idx + 1 && r@[new_k] == v@[idx]);
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==>
                               exists|j: int| 0 <= j < idx + 1 && r@[k] == v@[j]);
/*
                        // Completeness over [0..idx+1]: witness for idx is new_k.
                        assert(0 <= new_k < r@.len() && r@[new_k] == original[idx]);
                        assume(forall|j: int| #![trigger original[j]] 0 <= j < idx + 1 && f_spec(original[j]) ==>
                                 exists|k: int| 0 <= k < r@.len() && r@[k] == original[j]);
*/
                        idx = idx + 1;
                    }
                } else {
                    proof {
                        assert(!f_spec(v@[idx]));
                        // Predicate satisfaction over [0..idx+1]: r@ unchanged.
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==> f_spec(r@[k]));

                        // Provenance over [0..idx+1]: r@ unchanged, idx+1 > idx so all old witnesses still valid.
                        assert(forall|k: int| #![trigger r@[k]] 0 <= k < r@.len() ==>
                            exists|j: int| 0 <= j < idx + 1 && r@[k] == v@[j]);
/*
                        // Completeness over [0..idx+1]: j == idx is vacuous (!f_spec), rest from prior.
                        assert(forall|j: int| #![trigger original[j]] 0 <= j < idx + 1 && f_spec(original[j]) ==>
                            exists|k: int| 0 <= k < r@.len() && r@[k] == original[j]);
*/
                        idx = idx + 1;
                    }
                }
            }
            None => {
                proof {
                    assert(idx == original.len());
                    // Predicate satisfaction — full.
                    assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==> f_spec(r@[i]));

                    // Provenance — full ensures form.
                    assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==>
                        exists|j: int| 0 <= j < v@.len() && r@[i] == v@[j]);
/*
                    // Completeness — full ensures form.
                    assert(forall|j: int| #![trigger original[j]] 0 <= j < original.len() && f_spec(original[j]) ==>
                        exists|i: int| 0 <= i < r@.len() && r@[i] == original[j]);
*/
                }
                break;
            }
        }
    }
    proof {
        assert(original =~= v@);

        // Predicate satisfaction — full ensures form.
        assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==> f_spec(r@[i]));

        // Provenance — full ensures form.
        assert(forall|i: int| #![trigger r@[i]] 0 <= i < r@.len() ==>
            exists|j: int| 0 <= j < v@.len() && r@[i] == v@[j]);
/*
        // Completeness — full ensures form.
        assert(forall|j: int| #![trigger v@[j]] 0 <= j < v@.len() && f_spec(v@[j]) ==>
            exists|i: int| 0 <= i < r@.len() && r@[i] == v@[j]);
*/
    }
    r
}

} // verus!
