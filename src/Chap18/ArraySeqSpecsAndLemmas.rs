//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared spec functions and proof lemmas for the Chapter 18 ArraySeq and LinkedList modules.
//! All definitions are generic over element types and operate on `Seq<T>`.

//  Table of Contents
//	Section 1. module
//	Section 6. spec fns
//	Section 7. proof fns

//		Section 1. module

pub mod ArraySeqSpecsAndLemmas {

    use vstd::prelude::*;

    verus! {

broadcast use {
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    //		Section 6. spec fns


    /// Definition 18.7 (iterate). Left fold: spec_iterate(s, f, x) = f(...f(f(x, s[0]), s[1])..., s[n-1]).
    pub open spec fn spec_iterate<A, T>(s: Seq<T>, f: spec_fn(A, T) -> A, start_x: A) -> A {
        s.fold_left(start_x, f)
    }

    /// Definition 18.16 (inject). Apply position-value updates left to right; the first update
    /// to each position wins. Out-of-range positions are ignored.
    pub open spec fn spec_inject<T>(s: Seq<T>, updates: Seq<(usize, T)>) -> Seq<T>
        decreases updates.len()
    {
        if updates.len() == 0 {
            s
        } else {
            let rest = spec_inject(s, updates.drop_first());
            let pos = updates[0].0 as int;
            let val = updates[0].1;
            if 0 <= pos < s.len() { rest.update(pos, val) } else { rest }
        }
    }

    /// Definition 18.17 (ninject). Non-deterministic inject: each position in the result
    /// holds either the original value or a value from some update. The choice of which
    /// update "wins" is unspecified.
    pub open spec fn spec_ninject<T>(s: Seq<T>, updates: Seq<(usize, T)>, injected: Seq<T>) -> bool {
        injected.len() == s.len()
        && forall|i: int| #![trigger injected[i]] 0 <= i < s.len() ==> {
            injected[i] == s[i]
            || exists|j: int| #![trigger updates[j]] 0 <= j < updates.len()
                && updates[j].0 == i as usize && injected[i] == updates[j].1
        }
    }

    //		Section 7. proof fns


    /// The length of `spec_inject(s, u)` equals `s.len()`.
    pub(crate) proof fn lemma_spec_inject_len<T>(s: Seq<T>, u: Seq<(usize, T)>)
        ensures spec_inject(s, u).len() == s.len(),
        decreases u.len(),
    {
        reveal(spec_inject);
        if u.len() > 0 {
            lemma_spec_inject_len(s, u.drop_first());
        }
    }

    /// Each element of `spec_inject(s, u)` is either the original `s[i]` or some update value.
    pub(crate) proof fn lemma_spec_inject_element<T>(s: Seq<T>, u: Seq<(usize, T)>, i: int)
        requires 0 <= i < s.len(),
        ensures ({
            let r = spec_inject(s, u);
            r.len() == s.len()
            && (r[i] == s[i]
                || exists|j: int| #![trigger u[j]] 0 <= j < u.len()
                    && u[j].0 == i as usize && r[i] == u[j].1)
        }),
        decreases u.len(),
    {
        reveal(spec_inject);
        if u.len() > 0 {
            lemma_spec_inject_len(s, u.drop_first());
            lemma_spec_inject_element(s, u.drop_first(), i);
            let rest = spec_inject(s, u.drop_first());
            let pos = u[0].0 as int;
            let val = u[0].1;
            if 0 <= pos < s.len() {
                if i == pos {
                } else {
                    if rest[i] != s[i] {
                        let j = choose|j: int| #![trigger u.drop_first()[j]] 0 <= j < u.drop_first().len()
                            && u.drop_first()[j].0 == i as usize
                            && rest[i] == u.drop_first()[j].1;
                    }
                }
            } else {
                if rest[i] != s[i] {
                    let j = choose|j: int| #![trigger u.drop_first()[j]] 0 <= j < u.drop_first().len()
                        && u.drop_first()[j].0 == i as usize
                        && rest[i] == u.drop_first()[j].1;
                }
            }
        }
    }

    } // verus!
} // pub mod ArraySeqSpecsAndLemmas
