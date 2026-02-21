// Copyright (c) 2025 Brian G. Milnes
//! Hypothesis: Can Verus express clone preserving closure specs?

use vstd::prelude::*;

verus! {

// Try 1: Generic over closure with single arg
pub fn clone_fn1<T, U, F: Fn(&T) -> U + Clone>(f: F) -> (res: F)
    requires
        forall|x: &T| #[trigger] f.requires((x,)),
    ensures
        forall|x: &T| #[trigger] res.requires((x,)),
{
    let res = f.clone();
    assume(forall|x: &T| #[trigger] res.requires((x,)));
    res
}

// Try 2: With ensures preservation too
pub fn clone_fn2<T, U, F: Fn(&T) -> U + Clone>(f: F) -> (res: F)
    requires
        forall|x: &T| #[trigger] f.requires((x,)),
    ensures
        forall|x: &T| f.requires((x,)) == res.requires((x,)),
        forall|x: &T, r: U| f.ensures((x,), r) == res.ensures((x,), r),
{
    let res = f.clone();
    assume(forall|x: &T| f.requires((x,)) == res.requires((x,)));
    assume(forall|x: &T, r: U| f.ensures((x,), r) == res.ensures((x,), r));
    res
}

// Try 3: Binary function
pub fn clone_fn_binary<T, F: Fn(&T, &T) -> T + Clone>(f: F) -> (res: F)
    requires
        forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
    ensures
        forall|x: &T, y: &T| f.requires((x, y)) == res.requires((x, y)),
        forall|x: &T, y: &T, r: T| f.ensures((x, y), r) == res.ensures((x, y), r),
{
    let res = f.clone();
    assume(forall|x: &T, y: &T| f.requires((x, y)) == res.requires((x, y)));
    assume(forall|x: &T, y: &T, r: T| f.ensures((x, y), r) == res.ensures((x, y), r));
    res
}

// Try 4: Predicate (returns bool)
pub fn clone_predicate<T, F: Fn(&T) -> bool + Clone>(f: F) -> (res: F)
    requires
        forall|x: &T| #[trigger] f.requires((x,)),
    ensures
        forall|x: &T| f.requires((x,)) == res.requires((x,)),
        forall|x: &T, r: bool| f.ensures((x,), r) == res.ensures((x,), r),
{
    let res = f.clone();
    assume(forall|x: &T| f.requires((x,)) == res.requires((x,)));
    assume(forall|x: &T, r: bool| f.ensures((x,), r) == res.ensures((x,), r));
    res
}

// Try 5: Can we make it external_body with the ensures and no assume?
#[verifier::external_body]
pub fn clone_fn_axiom<T, U, F: Fn(&T) -> U + Clone>(f: F) -> (res: F)
    ensures
        forall|x: &T| f.requires((x,)) == res.requires((x,)),
        forall|x: &T, r: U| f.ensures((x,), r) == res.ensures((x,), r),
{
    f.clone()
}

} // verus!
