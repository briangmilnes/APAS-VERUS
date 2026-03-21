//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: One trait, two impl types (recursive vs iterative).
//!
//! The trait defines the abstract interface once. Separate types (RecStack and
//! IterStack) each implement it. A caller picks the implementation by importing
//! the concrete type — the generic code is identical either way.
//!
//! Layout: trait + spec, then all RecStack code, then all IterStack code.
//! Caller files: trait_rec_caller.rs and trait_iter_caller.rs.
//!
//! RESULT: VERIFIES

pub mod trait_rec_vs_iter {

    use vstd::prelude::*;

    verus! {

    // Shared spec: sum of the first n elements of a sequence.
    pub open spec fn spec_sum(s: Seq<u64>, n: int) -> int
        decreases n,
    {
        if n <= 0 { 0 }
        else { spec_sum(s, n - 1) + s[n - 1] as int }
    }

    pub proof fn lemma_spec_sum_monotone(s: Seq<u64>, i: int, j: int)
        requires 0 <= i <= j, j <= s.len(),
        ensures spec_sum(s, i) <= spec_sum(s, j),
        decreases j - i,
    {
        if i < j { lemma_spec_sum_monotone(s, i, j - 1); }
    }

    // The abstract interface — one trait, one set of function names.
    pub trait StackTrait: Sized + View<V = Seq<u64>> {
        spec fn spec_stack_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s@ == Seq::<u64>::empty(), s.spec_stack_wf();

        fn push(&mut self, val: u64)
            requires old(self).spec_stack_wf(), old(self)@.len() < usize::MAX,
            ensures self@ == old(self)@.push(val), self.spec_stack_wf();

        fn size(&self) -> (n: usize)
            requires self.spec_stack_wf(),
            ensures n == self@.len();

        fn sum(&self) -> (total: u64)
            requires
                self.spec_stack_wf(),
                self@.len() <= u64::MAX,
                spec_sum(self@, self@.len() as int) <= u64::MAX,
            ensures total == spec_sum(self@, self@.len() as int);
    }

    // Recursive implementation

    pub struct RecStack {
        pub elements: Vec<u64>,
    }

    impl View for RecStack {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
    }

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

    // Iterative implementation

    pub struct IterStack {
        pub elements: Vec<u64>,
    }

    impl View for IterStack {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
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
                    spec_sum(self.elements@, i as int) <= u64::MAX,
                    spec_sum(self.elements@, self.elements@.len() as int) <= u64::MAX,
                    self.elements@.len() <= u64::MAX,
                decreases self.elements@.len() - i,
            {
                proof {
                    lemma_spec_sum_monotone(self.elements@, i + 1, self.elements@.len() as int);
                }
                total = total + self.elements[i];
                i = i + 1;
            }
            total
        }
    }

    } // verus!
} // pub mod
