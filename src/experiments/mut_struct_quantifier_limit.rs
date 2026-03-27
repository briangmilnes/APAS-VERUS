//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Hypothesis: Z3 can't verify an exec function that modifies a 4-field struct
//! with quantified Map predicates in both requires and ensures, even when the
//! proof is trivial. This isolates the &mut encoding issue blocking UnionFind
//! union_merge/union/kruskal_process_edge.
//!
//! The experiment creates a minimal reproduction:
//! - Struct with 4 fields (2 Maps, 1 Vec, 1 Ghost Map) — mirrors UnionFindStEph
//! - Closed wf predicate with quantified conjuncts over the maps
//! - A proof lemma that verifies the wf transition
//! - An exec wrapper that calls the mutations + the proof lemma
//!
//! If the exec wrapper fails with rlimit/OOM but the proof lemma verifies,
//! this confirms the Z3 &mut encoding limitation.
//!
//! If both verify, the issue is specific to UnionFind's complexity, not the
//! encoding pattern, and we should look harder at the real code.

pub mod mut_struct_quantifier_limit {

    use vstd::prelude::*;

    use std::hash::Hash;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    verus! {

    // A struct with 4 fields, 2 of which are Maps — mirrors UnionFindStEph.
    #[verifier::reject_recursive_types(V)]
    pub struct FourFieldStruct<V: View + Eq + Hash + Clone> {
        pub map_a: HashMapWithViewPlus<V, V>,
        pub map_b: HashMapWithViewPlus<V, usize>,
        pub elements: Vec<V>,
        pub ghost_map: Ghost<Map<<V as View>::V, <V as View>::V>>,
    }

    // Closed wf with quantified conjuncts — mirrors spec_uf_wf.
    pub closed spec fn spec_four_field_wf<V: View + Eq + Hash + Clone>(s: &FourFieldStruct<V>) -> bool {
        &&& obeys_key_model::<V>()
        &&& s.map_a@.dom() =~= s.map_b@.dom()
        &&& s.ghost_map@.dom() =~= s.map_a@.dom()
        &&& forall|v: <V as View>::V| #[trigger] s.map_a@.contains_key(v) ==>
            s.map_a@.contains_key(s.map_a@[v]@)
        &&& forall|v: <V as View>::V| #[trigger] s.ghost_map@.contains_key(v) ==> {
            &&& s.ghost_map@.contains_key(s.ghost_map@[v])
            &&& s.ghost_map@[s.ghost_map@[v]] == s.ghost_map@[v]
        }
        &&& forall|i: int| 0 <= i < s.elements@.len() as int ==>
            s.map_a@.contains_key(#[trigger] s.elements@[i]@)
        &&& forall|v: <V as View>::V| #[trigger] s.map_a@.contains_key(v) ==>
            exists|i: int| 0 <= i < s.elements@.len() as int && #[trigger] s.elements@[i]@ == v
        &&& forall|v: <V as View>::V| s.map_a@.contains_key(v)
            && s.map_a@[v]@ != v ==>
            s.map_b@[v] < #[trigger] s.map_b@[s.map_a@[v]@]
    }

    // Proof lemma: given pre-state wf + specific mutation facts, post-state is wf.
    // This should verify (it's the analogue of lemma_union_merge_wf).
    proof fn lemma_mutation_preserves_wf<V: View + Eq + Hash + Clone>(
        old_s: &FourFieldStruct<V>,
        new_s: &FourFieldStruct<V>,
        key: <V as View>::V,
        val: V,
    )
        requires
            spec_four_field_wf(old_s),
            new_s.map_a@ =~= old_s.map_a@.insert(key, val),
            new_s.map_b@ =~= old_s.map_b@,
            new_s.elements@ =~= old_s.elements@,
            new_s.ghost_map@ =~= old_s.ghost_map@,
            old_s.map_a@.contains_key(key),
            old_s.map_a@.contains_key(val@),
        ensures
            spec_four_field_wf(new_s),
    {
        reveal(spec_four_field_wf);
    }

    // Test 1: Can we call the proof lemma from an exec function with &mut self?
    // This is the pattern that fails in UnionFind.
    #[verifier::rlimit(50)]
    fn mutate_and_prove<V: View + Eq + Hash + Clone>(
        s: &mut FourFieldStruct<V>,
        key: &V,
        val: V,
    )
        requires
            spec_four_field_wf(old(s)),
            old(s).map_a@.contains_key(key@),
            old(s).map_a@.contains_key(val@),
        ensures
            spec_four_field_wf(s),
    {
        let val_clone = val.clone();
        s.map_a.insert(key.clone(), val);
        proof {
            lemma_mutation_preserves_wf(old(s), s, key@, val_clone);
        }
    }

    // Test 2: Same but with TWO mutations (map_a + ghost_map) — closer to union_merge.
    #[verifier::rlimit(80)]
    fn mutate_two_fields<V: View + Eq + Hash + Clone>(
        s: &mut FourFieldStruct<V>,
        key: &V,
        val: V,
        ghost_key: Ghost<<V as View>::V>,
        ghost_val: Ghost<<V as View>::V>,
    )
        requires
            spec_four_field_wf(old(s)),
            old(s).map_a@.contains_key(key@),
            old(s).map_a@.contains_key(val@),
            old(s).ghost_map@.contains_key(ghost_key@),
        ensures
            s.map_a@ =~= old(s).map_a@.insert(key@, val),
            s.ghost_map@ =~= old(s).ghost_map@.insert(ghost_key@, ghost_val@),
            s.map_b@ =~= old(s).map_b@,
            s.elements@ =~= old(s).elements@,
    {
        s.map_a.insert(key.clone(), val);
        s.ghost_map = Ghost(s.ghost_map@.insert(ghost_key@, ghost_val@));
    }

    } // verus!
}
