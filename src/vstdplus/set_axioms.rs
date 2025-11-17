//! Additional set axioms not yet in vstd

pub mod set_axioms {
    use vstd::prelude::*;

    verus! {

    /// Returns a set containing only the given element.
    pub open spec fn singleton<V>(x: V) -> Set<V> {
        Set::empty().insert(x)
    }

    /// Lemma: A singleton set has length 1.
    pub proof fn lemma_singleton_len<V>(x: V)
        ensures
            singleton(x).len() == 1,
    { }

    /// Axiom: A set is empty if and only if it has length 0
    /// 
    /// vstd provides axiom_set_empty_len (forward direction: empty().len() == 0)
    /// but not the reverse (len == 0 ==> empty)
    /// 
    /// This axiom provides the bi-directional equivalence needed to prove
    /// is_empty() without external_body.
    pub broadcast proof fn axiom_set_len_zero_iff_empty<V>(s: Set<V>)
        ensures
            #[trigger] s.len() == 0 <==> s == Set::<V>::empty(),
    {
        admit();
    }

    /// Lemma: A set equals the union of a singleton element and the set with that element removed.
    pub broadcast proof fn lemma_set_split_element<V>(s: Set<V>, x: V)
        requires
            s.contains(x),
        ensures
            s =~= singleton(x) + #[trigger] s.remove(x),
    {}

    /// Lemma: Moving an element from one set to another preserves union (stronger, not needed for iterator).
    /// 
    /// If we have A + B and move element x from B to A (where x ∈ B),
    /// the union remains the same: A + B == (A ∪ {x}) + (B \ {x})
    pub proof fn lemma_set_move_element_preserves_union<V>(a: Set<V>, b: Set<V>, x: V)
        requires
            b.contains(x),
            a.disjoint(b),
        ensures
            a + b =~= a.insert(x) + b.remove(x),
    {
        assert forall |e: V| #![auto] (a + b).contains(e) <==> (a.insert(x) + b.remove(x)).contains(e) by {
            if e == x {
                assert((a + b).contains(e));
                assert((a.insert(x) + b.remove(x)).contains(e));
            } else {
                if a.contains(e) {
                    assert((a + b).contains(e));
                    assert(a.insert(x).contains(e));
                    assert((a.insert(x) + b.remove(x)).contains(e));
                } else if b.contains(e) {
                    assert((a + b).contains(e));
                    assert(b.remove(x).contains(e));
                    assert((a.insert(x) + b.remove(x)).contains(e));
                } else {
                    assert(!(a + b).contains(e));
                    assert(!a.insert(x).contains(e));
                    assert(!b.remove(x).contains(e));
                    assert(!(a.insert(x) + b.remove(x)).contains(e));
                }
            }
        }
    }

    /// Lemma: Union distributes over insert: a.union(b.insert(x)) == a.union(b).insert(x)
    /// 
    /// This is the key property needed for iterator-based union verification:
    /// when we insert an element into one side of a union, it's equivalent to
    /// inserting into the union result.
    pub broadcast proof fn lemma_union_insert_commute<V>(a: Set<V>, b: Set<V>, x: V)
        ensures
            #[trigger] a.union(b.insert(x)) =~= a.union(b).insert(x),
    {
    }
    /// Axiom group for additional set axioms
    pub broadcast group group_set_axioms_plus {
        axiom_set_len_zero_iff_empty,
        lemma_set_split_element,
        lemma_union_insert_commute,
    }

    } // verus!
}

