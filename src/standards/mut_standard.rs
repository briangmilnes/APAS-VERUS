// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0
//! Mut Standard: all verified &mut patterns in APAS-VERUS.
//!
//! Seven stable patterns (no flags needed) plus two experimental patterns
//! requiring `-V new-mut-ref`. Stable patterns use `old(self)` for pre-state
//! and bare `self` for final state in ensures. New-mut-ref replaces bare
//! `self` with `final(self)` and enables returning `&mut T`.
//!
//! Pattern 1: &mut self method with old()/ensures.
//! Pattern 2: &mut self with loop invariant referencing old(self).
//! Pattern 3: &mut T non-self parameter.
//! Pattern 4: Caller reasoning — sequential mutations compose.
//! Pattern 5: Struct field mutation via &mut self.
//! Pattern 6: &mut Vec<T> parameter — collection building.
//! Pattern 7: Option::take() + reassign — owned-to-borrowed bridge.
//! Pattern 8: (experimental) final() operator (-V new-mut-ref).
//! Pattern 9: (experimental) Returning &mut T (-V new-mut-ref).
//!
//! Reference: src/experiments/mut_refs_and_mut_returns.rs
// 1. module
pub mod mut_standard {

    use vstd::prelude::*;

    verus! {

    // 4. type definitions

    pub struct Counter {
        pub val: u64,
    }

    pub struct Pair {
        pub fst: u64,
        pub snd: u64,
    }

    // 5. view impls

    impl View for Counter {
        type V = u64;

        open spec fn view(&self) -> u64 {
            self.val
        }
    }

    // 8. traits

    pub trait CounterTrait: Sized {
        spec fn spec_val(&self) -> u64;

        fn new() -> (s: Self)
            ensures
                s.spec_val() == 0;

        /// Pattern 1: &mut self with old()/ensures.
        /// In requires: old(self) = pre-state.
        /// In ensures: bare self = final state.
        fn inc(&mut self)
            requires
                old(self).spec_val() < u64::MAX,
            ensures
                self.spec_val() == old(self).spec_val() + 1;

        /// Pattern 2: &mut self with loop invariant referencing old(self).
        fn add(&mut self, n: u64)
            requires
                old(self).spec_val() + n <= u64::MAX,
            ensures
                self.spec_val() == old(self).spec_val() + n;
    }

    pub trait PairTrait: Sized {
        spec fn spec_fst(&self) -> u64;

        spec fn spec_snd(&self) -> u64;

        fn new(fst: u64, snd: u64) -> (s: Self)
            ensures
                s.spec_fst() == fst,
                s.spec_snd() == snd;

        /// Pattern 5: Struct field mutation via &mut self.
        fn set_fst(&mut self, val: u64)
            ensures
                self.spec_fst() == val,
                self.spec_snd() == old(self).spec_snd();

        fn set_snd(&mut self, val: u64)
            ensures
                self.spec_fst() == old(self).spec_fst(),
                self.spec_snd() == val;

        /// Pattern 5 continued: swap via &mut self.
        fn swap(&mut self)
            ensures
                self.spec_fst() == old(self).spec_snd(),
                self.spec_snd() == old(self).spec_fst();
    }

    // 9. impls

    impl CounterTrait for Counter {
        open spec fn spec_val(&self) -> u64 {
            self.val
        }

        fn new() -> (s: Self) {
            Counter { val: 0 }
        }

        // Pattern 1: &mut self.
        // In body, self is the live mutable reference.
        // old(self) in requires = pre-call state.
        // self in ensures = post-call state.
        fn inc(&mut self) {
            self.val = self.val + 1;
        }

        // Pattern 2: Loop invariant references old(self) for the pre-loop snapshot.
        // self.val inside the loop = current live state, updated each iteration.
        fn add(&mut self, n: u64) {
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

    impl PairTrait for Pair {
        open spec fn spec_fst(&self) -> u64 {
            self.fst
        }

        open spec fn spec_snd(&self) -> u64 {
            self.snd
        }

        fn new(fst: u64, snd: u64) -> (s: Self) {
            Pair { fst, snd }
        }

        // Pattern 5: Direct field assignment preserves other fields.
        fn set_fst(&mut self, val: u64) {
            self.fst = val;
        }

        fn set_snd(&mut self, val: u64) {
            self.snd = val;
        }

        // Pattern 5 continued: swap reads both then writes both.
        fn swap(&mut self) {
            let tmp = self.fst;
            self.fst = self.snd;
            self.snd = tmp;
        }
    }

    // Pattern 3: &mut T non-self parameter.
    // Same rules as &mut self — old(x) in requires, bare *x in ensures.
    pub fn increment_ref(x: &mut u64)
        requires
            *old(x) < u64::MAX,
        ensures
            *x == *old(x) + 1,
    {
        *x = *x + 1;
    }

    // Pattern 6: &mut Vec<T> parameter — collection building.
    // Caller passes &mut Vec, callee appends. Vec::push specs propagate.
    pub fn collect_range(out: &mut Vec<u64>, lo: u64, hi: u64)
        requires
            lo <= hi,
            hi <= u64::MAX,
        ensures
            out@.len() == old(out)@.len() + (hi - lo),
    {
        let mut i = lo;
        while i < hi
            invariant
                lo <= i <= hi,
                out@.len() == old(out)@.len() + (i - lo),
            decreases hi - i
        {
            out.push(i);
            i = i + 1;
        }
    }

    // Pattern 7: Option::take() + reassign.
    // Extract an owned value from an Option field, transform it, put it back.
    // Bridges owned-value recursion with &mut self container interface.
    pub struct Container {
        pub item: Option<Box<u64>>,
    }

    pub fn replace_item(c: &mut Container, new_val: u64)
        ensures
            c.item == Some(Box::new(new_val)),
    {
        let _old = c.item.take();
        c.item = Some(Box::new(new_val));
    }

    // Pattern 4: Caller reasoning — sequential mutations compose.
    // After each &mut call, the caller sees the updated state.
    pub fn proof_sequential_mutations() {
        let mut c = Counter::new();
        assert(c.val == 0);

        c.inc();
        assert(c.val == 1);

        c.inc();
        assert(c.val == 2);

        c.add(5);
        assert(c.val == 7);
    }

    pub fn proof_field_mutation() {
        let mut p = Pair::new(10, 20);
        assert(p.fst == 10 && p.snd == 20);

        p.set_fst(99);
        assert(p.fst == 99 && p.snd == 20);

        p.swap();
        assert(p.fst == 20 && p.snd == 99);
    }

    pub fn proof_ref_parameter() {
        let mut v: u64 = 100;
        increment_ref(&mut v);
        assert(v == 101);

        increment_ref(&mut v);
        assert(v == 102);
    }

    pub fn proof_collect_range() {
        let mut out: Vec<u64> = Vec::new();
        collect_range(&mut out, 0, 5);
        assert(out@.len() == 5);

        collect_range(&mut out, 10, 13);
        assert(out@.len() == 8);
    }

    // Patterns 8-9: Experimental (-V new-mut-ref)
    //
    // Pattern 8: final() operator.
    // In ensures, *final(x) = updated value, *old(x) = entry value.
    // Bare *x in ensures is REJECTED — must disambiguate.
    //
    //   pub fn inc_ref(x: &mut u64)
    //       requires *old(x) < u64::MAX,
    //       ensures *final(x) == *old(x) + 1,
    //   { *x = *x + 1; }
    //
    // Pattern 9: Returning &mut T.
    // Previously blocked: "Verifier does not yet support &mut types."
    // With new-mut-ref, functions can return &mut T with specs.
    //
    //   pub fn get_val_mut(c: &mut Counter) -> (r: &mut u64)
    //       ensures
    //           *r == old(c).val,
    //           *final(r) == final(c).val,
    //   { &mut c.val }
    //
    // See src/experiments/mut_refs_and_mut_returns.rs for verified examples.

    } // verus!
} // pub mod mut_standard
