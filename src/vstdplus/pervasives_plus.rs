//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Additional pervasive utilities beyond vstd::pervasive.

pub mod pervasives_plus {
    use vstd::prelude::*;

    verus! {

    /// A spec function for documenting claims in code without proving them.
    /// Always returns true regardless of the claim - use for documentation only.
    /// 
    /// Example:
    /// ```
    /// assert(comment("x should be positive here", x > 0));
    /// ```
    /// 
    /// This type-checks and documents intent, but doesn't prove x > 0.
    pub open spec fn comment(description: &str, claim: bool) -> bool {
        true  // Always returns true, ignoring the claim
    }

    /// Swap two elements in a Vec without cloning.
    /// Wraps std Vec::swap, preserving T-level element identity.
    /// Required because vstd specs Vec::set and set_and_swap but not Vec::swap.
    #[verifier::external_body]
    pub fn vec_swap<T>(v: &mut Vec<T>, i: usize, j: usize)
        requires
            i < old(v)@.len(),
            j < old(v)@.len(),
        ensures
            v@.len() == old(v)@.len(),
            v@[i as int] == old(v)@[j as int],
            v@[j as int] == old(v)@[i as int],
            forall|k: int| 0 <= k < v@.len() && k != i as int && k != j as int ==>
                #[trigger] v@[k] == old(v)@[k],
    {
        v.swap(i, j);
    }

    } // verus!
}
