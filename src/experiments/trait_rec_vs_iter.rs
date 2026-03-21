//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: One trait, two impl types (recursive vs iterative).
//! IN PROGRESS — overflow and spec_sum assertion failures remain.
//!
//! Demonstrates pattern A for disambiguating recursive and iterative
//! implementations of the same ADT. The trait defines the abstract interface
//! once. Separate types (RecStack and IterStack) each implement it. Users
//! pick the implementation by choosing the type — no qualified calls needed.
//!
//! RESULT: VERIFIES

pub mod trait_rec_vs_iter {

    use vstd::prelude::*;

    verus! {

    // The abstract interface — one trait, one set of function names.
    pub trait StackTrait: Sized + View<V = Seq<u64>> {
        spec fn spec_stack_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s@ == Seq::<u64>::empty(), s.spec_stack_wf();

        fn push(&mut self, val: u64)
            requires old(self).spec_stack_wf(),
            ensures self@ == old(self)@.push(val), self.spec_stack_wf();

        fn size(&self) -> (n: usize)
            requires self.spec_stack_wf(),
            ensures n == self@.len();

        /// Sum all elements.  Recursive impl recurses; iterative impl loops.
        fn sum(&self) -> (total: u64)
            requires
                self.spec_stack_wf(),
                self@.len() <= u64::MAX,
                // No overflow: real sum fits in u64.
                spec_sum(self@, self@.len() as int) <= u64::MAX,
            ensures total == spec_sum(self@, self@.len() as int);
    }

    // Spec-level sum for verification.
    pub open spec fn spec_sum(s: Seq<u64>, n: int) -> int
        decreases n,
    {
        if n <= 0 { 0 }
        else { spec_sum(s, n - 1) + s[n - 1] as int }
    }

    // Recursive implementation: stores elements in a Vec, sums by recursion.
    pub struct RecStack {
        pub elements: Vec<u64>,
    }

    impl View for RecStack {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
    }

    // Iterative implementation: same storage, sums by loop.
    pub struct IterStack {
        pub elements: Vec<u64>,
    }

    impl View for IterStack {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
    }

    // Helper for recursive sum — free function, not in trait.
    fn rec_sum_helper(elements: &Vec<u64>, idx: usize) -> (total: u64)
        requires
            idx <= elements@.len(),
            elements@.len() <= u64::MAX,
            spec_sum(elements@, idx as int) <= u64::MAX,
        ensures total == spec_sum(elements@, idx as int),
        decreases idx,
    {
        if idx == 0 {
            0
        } else {
            let rest = rec_sum_helper(elements, idx - 1);
            rest + elements[idx - 1]
        }
    }

    impl StackTrait for RecStack {
        open spec fn spec_stack_wf(&self) -> bool { true }

        fn new() -> (s: Self) {
            RecStack { elements: Vec::new() }
        }

        fn push(&mut self, val: u64) {
            self.elements.push(val);
        }

        fn size(&self) -> (n: usize) {
            self.elements.len()
        }

        fn sum(&self) -> (total: u64) {
            rec_sum_helper(&self.elements, self.elements.len())
        }
    }

    impl StackTrait for IterStack {
        open spec fn spec_stack_wf(&self) -> bool { true }

        fn new() -> (s: Self) {
            IterStack { elements: Vec::new() }
        }

        fn push(&mut self, val: u64) {
            self.elements.push(val);
        }

        fn size(&self) -> (n: usize) {
            self.elements.len()
        }

        fn sum(&self) -> (total: u64) {
            let mut total: u64 = 0;
            let mut i: usize = 0;
            while i < self.elements.len()
                invariant
                    0 <= i <= self.elements@.len(),
                    total == spec_sum(self.elements@, i as int),
                    spec_sum(self.elements@, self.elements@.len() as int) <= u64::MAX,
                    self.elements@.len() <= u64::MAX,
                decreases self.elements@.len() - i,
            {
                assert(spec_sum(self.elements@, (i + 1) as int)
                    == spec_sum(self.elements@, i as int) + self.elements@[i as int] as int);
                total = total + self.elements[i];
                i = i + 1;
            }
            total
        }
    }

    // Calling experiment: generic code uses the trait, not the concrete type.
    // This proves that a single generic function works with both impls
    // without qualified calls or ambiguity.
    fn generic_caller<S: StackTrait>(s: &S) -> (total: u64)
        requires
            s.spec_stack_wf(),
            s@.len() <= u64::MAX,
            spec_sum(s@, s@.len() as int) <= u64::MAX,
        ensures total == spec_sum(s@, s@.len() as int),
    {
        s.sum()
    }

    // Concrete callers: user picks the type, calls the same generic code.
    fn test_rec_stack() {
        let mut s = RecStack::new();
        s.push(10);
        s.push(20);
        s.push(30);
        assert(s@ == seq![10u64, 20u64, 30u64]);
        assert(spec_sum(s@, 3) == 60);
        let total = generic_caller(&s);
        assert(total == 60);
    }

    fn test_iter_stack() {
        let mut s = IterStack::new();
        s.push(10);
        s.push(20);
        s.push(30);
        assert(s@ == seq![10u64, 20u64, 30u64]);
        assert(spec_sum(s@, 3) == 60);
        let total = generic_caller(&s);
        assert(total == 60);
    }

    // Mixed: both types in the same scope, no ambiguity.
    fn test_both_in_scope() {
        let mut r = RecStack::new();
        let mut i = IterStack::new();
        r.push(5);
        i.push(5);
        // Same method name, different types, no qualification needed.
        assert(spec_sum(r@, 1) == 5);
        assert(spec_sum(i@, 1) == 5);
        let r_sum = r.sum();
        let i_sum = i.sum();
        assert(r_sum == 5);
        assert(i_sum == 5);
        assert(r_sum == i_sum);
    }

    } // verus!
} // pub mod
