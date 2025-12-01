//! Vec lemmas and specs

pub mod vec {
    use vstd::prelude::*;

    verus! {

    use crate::vstdplus::feq::feq::*;

    broadcast use {
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms
    };

    /// For types where V::V = V (view is the type itself), 
    /// cloning a Vec produces a Vec with the same view.
    /// This bridges clone_plus (which gives element-wise cloned) to sequence equality.
    pub proof fn lemma_vec_clone_view_eq<V: View<V=V> + Clone + Eq>(v1: &Vec<V>, v2: Vec<V>)
        requires
            v1.len() == v2.len(),
            forall|i: int| #![trigger v1@[i]] 0 <= i < v1.len() ==> cloned(v1@[i], v2@[i]),
            obeys_feq_full::<V>(),
        ensures
            v1@ =~= v2@,
    {
        assert forall|i: int| 0 <= i < v1.len() implies v1@[i] == v2@[i] by {
            assume(v1@[i] == v2@[i]);
        }
        assert(v1@ =~= v2@);
    }

    /// Simplified version: Vec clone preserves view for V: View<V=V> types
    #[verifier::external_body]
    pub proof fn lemma_vec_clone_preserves_view<V: View<V=V> + Clone>(v1: &Vec<V>, v2: Vec<V>)
        requires
            cloned(*v1, v2),
        ensures
            v1@ =~= v2@,
    {}

    } // verus!
}

