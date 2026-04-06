// Copyright (c) 2025 Brian G. Milnes
//! REVIEWED: NO
//! monoid — Spec functions for monoid structure (associative + two-sided identity).
//! Extends `vstd::relations::associative` with identity laws.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 6. spec fns

//		Section 1. module

pub mod monoid {

    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{

    #[cfg(verus_keep_ghost)]
    use vstd::relations::associative;

    //		Section 6. spec fns


    /// The value id is a left identity for f: f(id, x) == x for all x.
    pub open spec fn spec_left_identity<T>(f: spec_fn(T, T) -> T, id: T) -> bool {
        forall|x: T| #[trigger] f(id, x) == x
    }

    /// The value id is a right identity for f: f(x, id) == x for all x.
    pub open spec fn spec_right_identity<T>(f: spec_fn(T, T) -> T, id: T) -> bool {
        forall|x: T| #[trigger] f(x, id) == x
    }

    /// The triple (T, f, id) forms a monoid: f is associative and id is a two-sided identity.
    pub open spec fn spec_monoid<T>(f: spec_fn(T, T) -> T, id: T) -> bool {
        associative(f) && spec_left_identity(f, id) && spec_right_identity(f, id)
    }

    } // verus!
}
