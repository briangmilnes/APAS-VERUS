//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Capacity Bounds Standard: integer max bounds in requires, not assumes.
//!
//! Rule: when an operation grows a collection (insert, push, append, resize),
//! the capacity bound belongs in the operation's REQUIRES clause, not as an
//! assume in the body. Callers prove they have room; the implementation uses
//! the bound to prove the result fits.
//!
//! Why: Vec, array, and tree-backed structures store lengths as usize. After
//! insert, `new_len == old_len + 1` must fit in usize. Verus cannot infer
//! this — it needs an explicit bound. The choice is where to state it:
//!
//! - CORRECT: `requires self@.len() < usize::MAX` on insert. The caller
//!   proves room exists. The implementation uses it. Zero holes.
//!
//! - WRONG: `assume(new_vec@.len() < usize::MAX)` inside the body. This is
//!   a proof hole. It says "trust me, the collection won't overflow." Every
//!   insert adds a hole. Twenty inserts, twenty holes — all saying the same
//!   thing that one requires clause would have said.
//!
//! Patterns by operation:
//!
//! 1. Single-element insert/push:
//!    `requires self@.len() < usize::MAX` (or `< usize::MAX - 1` if the
//!    operation may grow by more than 1, e.g., rebalancing that splits nodes).
//!
//! 2. Bulk insert/append/union:
//!    `requires self@.len() + other@.len() <= usize::MAX` (sum must fit).
//!
//! 3. Constructors from data (from_vec, from_seq, collect):
//!    `requires data@.len() <= usize::MAX` — trivially true for Vec inputs
//!    but needed for Seq or Set inputs that could be spec-infinite.
//!
//! 4. Filter/map/tabulate producing new collections:
//!    Output length <= input length, so `requires input@.len() <= usize::MAX`
//!    suffices. No separate bound on the output.
//!
//! Where NOT to put the bound:
//!
//! - NOT in spec_wf. Capacity is not a structural invariant of the type. A
//!   collection with `len() == usize::MAX` is well-formed — it just cannot
//!   accept another insert. The spec_wf_standard explicitly says: do not
//!   invent synthetic invariants like `self@.len() <= usize::MAX` in wf.
//!
//! - NOT as ensures on the output. `ensures self@.len() <= usize::MAX` is
//!   always true for Vec-backed types (Vec lengths are usize), so it adds
//!   no information. State the functional postcondition instead:
//!   `ensures self@ == old(self)@.insert(key)`.
//!
//! How callers propagate the bound:
//!
//! Most callers operate on collections well below usize::MAX. The bound
//! propagates naturally: if the caller's collection came from a Vec or was
//! built by repeated insert from a bounded source, the length is bounded.
//! For recursive algorithms that split and merge, the sum of parts equals
//! the original, so the bound carries through.
//!
//! Integer types and their MAX values:
//!
//!   | Type  | MAX                    | Typical use               |
//!   |-------|------------------------|---------------------------|
//!   | usize | platform-dependent     | Lengths, indices, sizes   |
//!   | u64   | 18446744073709551615   | Element values, keys      |
//!   | u32   | 4294967295             | Hash values, small counts |
//!   | i64   | 9223372036854775807    | Signed arithmetic         |
//!
//! For non-usize integer arithmetic (e.g., summing u64 values), the same
//! principle applies: overflow bounds go in requires, not as assumes.
//! `requires a <= u64::MAX - b` before `a + b`.
//!
//! References:
//! - src/standards/spec_wf_standard.rs (do not put capacity in wf).
//! - src/standards/finite_sets_standard.rs (finiteness IS in wf; capacity is not).
//! - src/Chap37/AVLTreeSeqStEph.rs (correct: `values@.len() < usize::MAX` in requires).
//! - src/Chap41/AVLTreeSetStEph.rs (WRONG: `assume(new_vec@.len() < usize::MAX)` in body).

pub mod capacity_bounds_standard {

    use vstd::prelude::*;

    verus! {

    // CORRECT: capacity bound in insert requires.

    pub struct BoundedStack {
        pub items: Vec<u64>,
    }

    impl View for BoundedStack {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> {
            self.items@
        }
    }

    pub trait BoundedStackTrait: Sized + View<V = Seq<u64>> {
        spec fn spec_boundedstack_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s.spec_boundedstack_wf(), s@.len() == 0;

        /// Push requires room for one more element.
        fn push(&mut self, x: u64)
            requires
                old(self).spec_boundedstack_wf(),
                old(self)@.len() < usize::MAX,  // Capacity bound: caller proves room.
            ensures
                self.spec_boundedstack_wf(),
                self@ == old(self)@.push(x);

        fn pop(&mut self) -> (x: u64)
            requires old(self).spec_boundedstack_wf(), old(self)@.len() > 0,
            ensures self.spec_boundedstack_wf(), x == old(self)@.last(), self@ == old(self)@.drop_last();

        fn len(&self) -> (count: usize)
            requires self.spec_boundedstack_wf(),
            ensures count == self@.len();
    }

    impl BoundedStackTrait for BoundedStack {
        // wf is trivially true for a Vec wrapper (per spec_wf_standard).
        // Capacity is NOT in wf — a full stack is still well-formed.
        open spec fn spec_boundedstack_wf(&self) -> bool { true }

        fn new() -> (s: Self) {
            BoundedStack { items: Vec::new() }
        }

        fn push(&mut self, x: u64) {
            // The requires gives us: old(self)@.len() < usize::MAX.
            // Vec::push needs the same bound (implicitly). Verus verifies
            // that items.len() + 1 <= usize::MAX from the requires.
            // Zero assumes. Zero holes.
            self.items.push(x);
        }

        fn pop(&mut self) -> (x: u64) {
            let len = self.items.len();
            let x = self.items[len - 1];
            self.items.truncate(len - 1);
            x
        }

        fn len(&self) -> (count: usize) {
            self.items.len()
        }
    }

    // Demonstrate that callers can satisfy the bound naturally.
    fn push_three_items() {
        let mut stack = BoundedStack::new();
        // Each push needs len < usize::MAX. After new(), len == 0.
        // 0 < usize::MAX: trivially true.
        stack.push(10);
        // 1 < usize::MAX: trivially true.
        stack.push(20);
        // 2 < usize::MAX: trivially true.
        stack.push(30);
        assert(stack@.len() == 3);
        assert(stack@[0] == 10);
        assert(stack@[1] == 20);
        assert(stack@[2] == 30);
    }

    // Demonstrate the bound propagating through a loop.
    fn fill_from_source(source: &Vec<u64>)
        requires source@.len() < usize::MAX,
    {
        let mut stack = BoundedStack::new();
        let mut i: usize = 0;
        while i < source.len()
            invariant
                stack.spec_boundedstack_wf(),
                0 <= i <= source@.len(),
                stack@.len() == i,
                // The bound propagates: i <= source.len() < usize::MAX.
                source@.len() < usize::MAX,
            decreases source@.len() - i,
        {
            // stack@.len() == i < source@.len() < usize::MAX. Room for one more.
            stack.push(source[i]);
            i += 1;
        }
        assert(stack@.len() == source@.len());
    }

    // CORRECT: bulk merge bounds the sum.
    //
    //   fn append(&mut self, other: &Vec<u64>)
    //       requires old(self)@.len() + other@.len() <= usize::MAX;
    //
    // The sum of both lengths must fit in usize.

    // ANTIPATTERN: assume inside the body instead of requires on the trait.
    //
    // fn insert(&mut self, x: u64)
    //     requires old(self).spec_X_wf(),
    //     // Missing: old(self)@.len() < usize::MAX
    // {
    //     self.items.push(x);
    //     proof { assume(self.items@.len() < usize::MAX); }  // HOLE
    //     // This assume is unnecessary if the requires states the bound.
    //     // The assume is saying "trust me" instead of "prove it."
    // }

    // ANTIPATTERN: capacity in wf.
    //
    // open spec fn spec_bad_wf(&self) -> bool {
    //     self@.len() < usize::MAX  // WRONG: a full collection is still well-formed.
    // }
    //
    // This breaks after insert fills the collection to MAX: the output
    // violates wf even though the collection is perfectly valid. The wf
    // predicate should describe structural validity, not operational headroom.

    // Integer arithmetic overflow: same principle.

    fn safe_add(a: u64, b: u64) -> (sum: u64)
        requires a <= u64::MAX - b,  // Caller proves no overflow.
        ensures sum == a + b,
    {
        a + b
    }

    fn safe_increment(x: u64) -> (y: u64)
        requires x < u64::MAX,  // Room for +1.
        ensures y == x + 1,
    {
        x + 1
    }

    // Demonstrate chained arithmetic bounds.
    fn sum_three(a: u64, b: u64, c: u64) -> (total: u64)
        requires (a as nat) + (b as nat) + (c as nat) <= u64::MAX as nat,
        ensures total == a + b + c,
    {
        let ab = a + b;
        ab + c
    }

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for BoundedStack {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "BoundedStack(len={})", self.items.len())
        }
    }
    impl std::fmt::Display for BoundedStack {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "BoundedStack(len={})", self.items.len())
        }
    }
}
