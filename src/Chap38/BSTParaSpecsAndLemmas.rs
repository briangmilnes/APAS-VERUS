// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Shared spec functions and proof lemmas for the parametric BST modules in Chap38.
//! Both BSTParaStEph and BSTParaMtEph import from here to avoid duplicating
//! comparison ordering lemmas.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 6. spec fns
//	Section 7. proof fns

//		Section 1. module

pub mod BSTParaSpecsAndLemmas {

    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;

    use crate::Types::Types::*;

    verus! {

    //		Section 6. spec fns


    /// View-consistent ordering: elements with the same view compare Equal.
    pub open spec fn view_ord_consistent<T: StT + Ord>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    //		Section 7. proof fns


    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    pub proof fn lemma_cmp_antisymmetry<T: StT + Ord>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Greater,
        ensures
            b.cmp_spec(&a) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Less(a,b) and Less(b,c) implies Less(a,c).
    pub proof fn lemma_cmp_transitivity<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Less,
        ensures
            a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Equal-substitution: Less(a,b) and Equal(b,c) implies Less(a,c).
    pub proof fn lemma_cmp_eq_subst<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Equal,
        ensures
            a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Left congruence: Equal(a,b) implies a and b compare the same way to c.
    pub proof fn lemma_cmp_equal_congruent<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures
            a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Right congruence: Equal(b,c) implies any a compares the same way to b and c.
    pub proof fn lemma_cmp_equal_congruent_right<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            b.cmp_spec(&c) == Equal,
        ensures
            a.cmp_spec(&b) == a.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Ordering axioms: restates obeys_cmp_spec and view_ord_consistent as postconditions.
    pub proof fn lemma_cmp_order_axioms<T: StT + Ord>()
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
    {}

    } // verus!
} // mod BSTParaSpecsAndLemmas
