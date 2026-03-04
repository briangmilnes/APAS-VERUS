//! Experiment: &mut parameters, &mut returns, and mut returns in Verus.
//!
//! Hypothesis: Verus can verify &mut self methods with old/fin in ensures,
//! and functions returning &mut T with proper specs.
//!
//! Result (pre new-mut-ref):
//! - &mut self with old() in requires/ensures: VERIFIES.
//! - fn returning &mut T: FAILS. "Verifier does not yet support &mut types."
//!
//! Result (post new-mut-ref, 2026-03-04):
//! - &mut self with old()/final() in ensures: VERIFIES.
//! - fn returning &mut T: VERIFIES. Uses *final(r) in ensures.
//! - *final(x) operator: VERIFIES. Replaces bare *x in ensures.
//! - All 10 functions verified, 0 errors.

use vstd::prelude::*;

verus! {

// 1. &mut self with ensures (old/fin)
//
// Hypothesis: We can specify mutation via old(x) = pre-state.
// Result: Verifies. In requires use old(self); in ensures self = final state.

pub struct Counter {
    pub val: u64,
}

impl Counter {
    pub fn new() -> (s: Self)
        ensures s.val == 0
    {
        Counter { val: 0 }
    }

    /// Increment by 1. Spec uses old/final to relate pre and post state.
    pub fn inc(self: &mut Counter)
        requires
            old(self).val < u64::MAX,
        ensures
            final(self).val == old(self).val + 1,
    {
        self.val = self.val + 1;
    }

    /// Add n. Proves useful fact: final value = old + n.
    pub fn add(self: &mut Counter, n: u64)
        requires
            old(self).val + n <= u64::MAX,
        ensures
            final(self).val == old(self).val + n,
    {
        let mut i: u64 = 0;
        while i < n
            invariant
                self.val == old(self).val + i,
                i <= n,
                self.val + (n - i) <= u64::MAX,
            decreases n - i
        {
            self.inc();
            i = i + 1;
        }
    }
}

// 2. fn returning &mut T (requires -V new-mut-ref)
//
// Hypothesis: Returning &mut from a function verifies with new-mut-ref.
// Result: VERIFIES. *r = entry value, *final(r) = updated value, final(c) = final container.

// 2a. Simple function returning &mut to a struct field.
pub fn get_val_mut(c: &mut Counter) -> (r: &mut u64)
    ensures
        *r == old(c).val,
        *final(r) == final(c).val,
{
    &mut c.val
}

// 2b. Caller uses returned &mut reference.
pub fn test_get_val_mut() {
    let mut c = Counter::new();
    assert(c.val == 0);
    let r = get_val_mut(&mut c);
    *r = 42;
    assert(c.val == 42);
}

// 3. *final() operator in ensures (requires -V new-mut-ref)
//
// Hypothesis: *final(x) replaces bare *x in ensures for updated value.
// Result: VERIFIES. *old(x) = entry, *final(x) = updated.

pub fn inc_via_ref(x: &mut u64)
    requires
        *old(x) < u64::MAX,
    ensures
        *final(x) == *old(x) + 1,
{
    *x = *x + 1;
}

pub fn test_inc_via_ref() {
    let mut v: u64 = 10;
    inc_via_ref(&mut v);
    assert(v == 11);
}

// 4. Useful proof: caller can reason about mutation
//
// Proves: after inc(), val increased by 1. After add(n), val increased by n.

pub fn proof_inc_spec() {
    let mut c = Counter::new();
    assert(c.val == 0);

    c.inc();
    assert(c.val == 1);

    c.inc();
    assert(c.val == 2);
}

pub fn proof_add_spec() {
    let mut c = Counter::new();
    c.add(5);
    assert(c.val == 5);

    c.add(3);
    assert(c.val == 8);
}

} // verus!
