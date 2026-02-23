//! Experiment: Set len/empty equivalence proved both ways.
//!
//! vstd's `axiom_is_empty_len0` gives:
//!   s.is_empty() <==> (s.finite() && s.len() == 0)
//!
//! We derive and prove both implications explicitly:
//! 1. Forward:  s.len() == 0 && s.finite() ==> s.is_empty()
//! 2. Reverse:  s.len() > 0 && s.finite() ==> !s.is_empty()
//!
//! The reverse is hugely general — it lets you conclude non-emptiness from
//! positive length without a separate witness.

use vstd::prelude::*;

verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

/// Forward: length 0 implies empty.
pub proof fn lemma_len0_implies_empty<A>(s: Set<A>)
    requires s.finite(),
    ensures s.len() == 0 ==> s.is_empty(),
{
    vstd::set_lib::axiom_is_empty_len0(s);
}

/// Reverse: positive length implies not empty.
pub proof fn lemma_len_pos_implies_not_empty<A>(s: Set<A>)
    requires s.finite(),
    ensures s.len() > 0 ==> !s.is_empty(),
{
    vstd::set_lib::axiom_is_empty_len0(s);
}

/// Combined: both directions in one lemma.
pub proof fn lemma_set_len_empty_both_ways<A>(s: Set<A>)
    requires s.finite(),
    ensures
        s.len() == 0 ==> s.is_empty(),
        s.len() > 0 ==> !s.is_empty(),
{
    vstd::set_lib::axiom_is_empty_len0(s);
}

/// Exercise both directions on concrete sets.
pub proof fn test_both_directions()
{
    let empty: Set<int> = Set::empty();
    assert(empty.finite());
    assert(empty.len() == 0);
    lemma_len0_implies_empty(empty);
    assert(empty.is_empty());

    let s = empty.insert(7);
    assert(s.finite());
    assert(s.len() == 1);
    assert(s.len() > 0);
    lemma_len_pos_implies_not_empty(s);
    assert(!s.is_empty());
}

} // verus!
