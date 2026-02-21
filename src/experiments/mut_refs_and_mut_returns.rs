//! Experiment: &mut parameters, &mut returns, and mut returns in Verus.
//!
//! Hypothesis: Verus can verify &mut self methods with old/fin in ensures,
//! and functions returning &mut T with proper specs.
//!
//! Result:
//! - &mut self with old() in requires/ensures: VERIFIES. Use old(self) not self
//!   in requires; ensures can use self for final state.
//! - fn returning &mut T: FAILS. "Verifier does not yet support &mut types
//!   except in special cases." Use new-mut-ref feature (experimental) for that.
//! - mut return (owned T): N/A; returning owned values works normally.
//!
//! Useful proof: mutation specs let callers prove state changes (inc +1, add +n).

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

    /// Increment by 1. Spec uses old/fin to relate pre and post state.
    pub fn inc(self: &mut Counter)
        requires
            old(self).val < u64::MAX,
        ensures
            self.val == old(self).val + 1,
    {
        self.val = self.val + 1;
    }

    /// Add n. Proves useful fact: final value = old + n.
    pub fn add(self: &mut Counter, n: u64)
        requires
            old(self).val + n <= u64::MAX,
        ensures
            self.val == old(self).val + n,
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

// 2. fn returning &mut T
//
// Hypothesis: Returning &mut from a function verifies with ensures on *y.
// Result: FAILS. "The verifier does not yet support &mut types, except in special cases."
// Verus blocks functions that return &mut T. Use new-mut-ref feature (experimental) for that.

// 3. Useful proof: caller can reason about mutation
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
