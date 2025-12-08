//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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

    } // verus!
}

