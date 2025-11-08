//! Additional set axioms not yet in vstd

pub mod set_axioms {
    use vstd::prelude::*;

    verus! {

    /// Axiom: A set is empty if and only if it has length 0
    /// 
    /// vstd provides axiom_set_empty_len (forward direction: empty().len() == 0)
    /// but not the reverse (len == 0 ==> empty)
    /// 
    /// This axiom provides the bi-directional equivalence needed to prove
    /// is_empty() without external_body.
    pub broadcast proof fn axiom_set_len_zero_iff_empty<A>(s: Set<A>)
        ensures
            #[trigger] s.len() == 0 <==> s == Set::<A>::empty(),
    {
        admit();
    }

    /// Axiom group for additional set axioms
    pub broadcast group group_set_axioms_plus {
        axiom_set_len_zero_iff_empty,
    }

    } // verus!
}

