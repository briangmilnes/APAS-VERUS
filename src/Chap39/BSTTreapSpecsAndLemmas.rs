//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared spec functions and proof lemmas for the BST Treap modules (StEph, MtEph, ParaMtEph).
//! All definitions are generic over `T: View + Ord` and operate on ordering via `cmp_spec`.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 7. proof fns

//		Section 1. module

pub mod BSTTreapSpecsAndLemmas {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;


    verus! {

    //		Section 3. broadcast use


    broadcast use {vstd::set::group_set_axioms, vstd::set_lib::group_set_properties};

    //		Section 6. spec fns


    /// View-consistent ordering: elements with equal views compare Equal.
    pub open spec fn view_ord_consistent<T: View + Ord>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    //		Section 7. proof fns


    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    pub proof fn lemma_cmp_antisymmetry<T: View + Ord>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Greater,
        ensures b.cmp_spec(&a) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec antisymmetry: Less(a,b) implies Greater(b,a).
    pub proof fn lemma_cmp_antisymmetry_less<T: View + Ord>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
        ensures b.cmp_spec(&a) == Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Less(a,b) and Less(b,c) implies Less(a,c).
    pub proof fn lemma_cmp_transitivity<T: View + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Less,
        ensures a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Equal-substitution: Less(a,b) and Equal(b,c) implies Less(a,c).
    pub proof fn lemma_cmp_eq_subst<T: View + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Equal,
        ensures a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Left congruence: Equal(a,b) implies a and b compare the same way to c.
    pub proof fn lemma_cmp_equal_congruent<T: View + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Right congruence: Equal(b,c) implies any a compares the same way to b and c.
    pub proof fn lemma_cmp_equal_congruent_right<T: View + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            b.cmp_spec(&c) == Equal,
        ensures a.cmp_spec(&b) == a.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// After join(lr, key, right), every element is greater than lk.
    /// Hypotheses: lr > lk, right > key, lk ∈ left, all left < key.
    pub proof fn lemma_joined_right_gt_lk<T: View + Ord>(
        lrv: Set<T::V>,
        right_v: Set<T::V>,
        key: T,
        joined_v: Set<T::V>,
        lk: T,
        left_v: Set<T::V>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            joined_v =~= lrv.union(right_v).insert(key@),
            forall|t: T| (#[trigger] lrv.contains(t@)) ==> t.cmp_spec(&lk) == Greater,
            forall|t: T| (#[trigger] right_v.contains(t@)) ==> t.cmp_spec(&key) == Greater,
            left_v.contains(lk@),
            forall|t: T| (#[trigger] left_v.contains(t@)) ==> t.cmp_spec(&key) == Less,
        ensures
            forall|t: T| (#[trigger] joined_v.contains(t@)) ==> t.cmp_spec(&lk) == Greater,
    {
        // Veracity: NEEDED assert
        assert forall|t: T| (#[trigger] joined_v.contains(t@)) implies t.cmp_spec(&lk) == Greater by {
            if lrv.contains(t@) {
            } else if right_v.contains(t@) {
                lemma_cmp_antisymmetry(t, key);
                lemma_cmp_transitivity(lk, key, t);
                lemma_cmp_antisymmetry_less(lk, t);
            } else {
                lemma_cmp_equal_congruent_right(lk, t, key);
                lemma_cmp_antisymmetry_less(lk, t);
            }
        }
    }

    /// After join(left, key, rl), every element is less than rk.
    /// Hypotheses: left < key, rl < rk, rk ∈ right, all right > key.
    pub proof fn lemma_joined_left_lt_rk<T: View + Ord>(
        left_v: Set<T::V>,
        rlv: Set<T::V>,
        key: T,
        joined_v: Set<T::V>,
        rk: T,
        right_v: Set<T::V>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            joined_v =~= left_v.union(rlv).insert(key@),
            forall|t: T| (#[trigger] left_v.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] rlv.contains(t@)) ==> t.cmp_spec(&rk) == Less,
            right_v.contains(rk@),
            forall|t: T| (#[trigger] right_v.contains(t@)) ==> t.cmp_spec(&key) == Greater,
        ensures
            forall|t: T| (#[trigger] joined_v.contains(t@)) ==> t.cmp_spec(&rk) == Less,
    {
        // Veracity: NEEDED assert
        assert forall|t: T| (#[trigger] joined_v.contains(t@)) implies t.cmp_spec(&rk) == Less by {
            if left_v.contains(t@) {
                lemma_cmp_antisymmetry(rk, key);
                lemma_cmp_transitivity(t, key, rk);
            } else if rlv.contains(t@) {
            } else {
                lemma_cmp_antisymmetry(rk, key);
                lemma_cmp_equal_congruent(t, key, rk);
            }
        }
    }

    } // verus!
} // pub mod BSTTreapSpecsAndLemmas
