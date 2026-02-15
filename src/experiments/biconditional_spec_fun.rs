//! HYPOTHESIS: We should be able to prove biconditionals against spec functions.
//!
//! Given a concrete exec closure with `ensures keep == f(x)` and a ghost
//! `spec_fn` `spec_pred` defined as `|x| f(x)`, can Verus prove:
//!
//!   forall|v: T, keep: bool| pred.ensures((&v,), keep) <==> spec_pred(v) == keep
//!
//! at the call site, without `assume`?
//! RESULT: Alas not in this Verus. 

pub mod biconditional_spec_fun {

use vstd::prelude::*;

verus! {

// A simple spec function to use as the predicate.
pub open spec fn spec_is_even(n: int) -> bool {
    n % 2 == 0
}

// A simple exec function matching the spec.
pub fn is_even(n: &usize) -> (keep: bool)
    ensures keep == spec_is_even(*n as int),
{
    *n % 2 == 0
}

// ─── Test 1: Forward direction only (==>). Expected: PASS ───

pub fn test_forward_direction()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);

    // Forward: if the closure returns keep, then spec_pred agrees.
    assert(forall|v: usize, keep: bool|
        pred.ensures((&v,), keep) ==> spec_pred(v) == keep);
}

// ─── Test 2: Backward direction only (<==). Expected: ??? ───

pub fn test_backward_direction()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);

    // Backward: if spec_pred says keep, then pred.ensures holds.
    assert(forall|v: usize, keep: bool|
        spec_pred(v) == keep ==> pred.ensures((&v,), keep));
}

// ─── Test 3: Full biconditional (<==>). Expected: ??? ───

pub fn test_biconditional()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);

    // Biconditional: pred.ensures <==> spec_pred match.
    assert(forall|v: usize, keep: bool|
        pred.ensures((&v,), keep) <==> spec_pred(v) == keep);
}

// ─── Test 4: Identity closure (simplest possible). Expected: ??? ───

pub fn test_identity_biconditional()
{
    let pred = |x: &bool| -> (keep: bool)
        ensures keep == *x,
    { *x };

    let ghost spec_pred: spec_fn(bool) -> bool = |v: bool| v;

    assert(forall|v: bool, keep: bool|
        pred.ensures((&v,), keep) <==> spec_pred(v) == keep);
}

// ─── Test 5: Biconditional without intermediate spec_fn. Expected: ??? ───

pub fn test_biconditional_inline()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    // No ghost spec_fn — just use the spec function directly.
    assert(forall|v: usize, keep: bool|
        pred.ensures((&v,), keep) <==> spec_is_even(v as int) == keep);
}

// ─── Test 6: assert-forall with by block. Expected: ??? ───

pub fn test_biconditional_with_by()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);

    assert forall|v: usize, keep: bool|
        pred.ensures((&v,), keep) <==> spec_pred(v) == keep
    by {};
}

// ─── Test 7: Forward direction with assert-forall + by. ───

pub fn test_forward_with_by()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);

    assert forall|v: usize, keep: bool|
        pred.ensures((&v,), keep) implies spec_pred(v) == keep
    by {};
}

// ─── Test 8: Call the closure and check ensures for a specific value. ───

pub fn test_specific_call()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);

    let v: usize = 4;
    let keep = pred(&v);

    // After calling, Verus knows pred.ensures((&v,), keep).
    // Can it connect to spec_pred?
    assert(spec_pred(v) == keep);
    assert(pred.ensures((&v,), keep));
}

// ─── Test 9: Can we get backward from call_ensures? ───

pub fn test_call_ensures()
{
    let pred = |x: &usize| -> (keep: bool)
        ensures keep == spec_is_even(*x as int),
    { is_even(x) };

    let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);

    // Verus has call_ensures: if requires hold, then exists ret s.t. ensures holds.
    // pred.requires((&4usize,)) should hold (no requires on our closure).
    // So exists|keep: bool| pred.ensures((&4usize,), keep) && keep == spec_is_even(4).
    proof {
        // Try asserting call_ensures gives us the backward direction for a specific input.
        assert(pred.requires((&4usize,)));
    }
}

} // verus!

} // mod
