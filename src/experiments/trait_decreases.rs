// Copyright (c) 2025 Brian G. Milnes
//! Experiment: Recursive spec fns in trait impls — what patterns work?
//!
//! CONCLUSION: Verus cannot unfold recursive `open spec fn` through trait
//! dispatch, even with `decreases` on both trait declaration and impl. The
//! solver treats `<Tree<T> as Trait>::spec_size()` as opaque.
//!
//! Results:
//!   Test 1 (inherent spec fn, no trait):                   PASS — baseline
//!   Test 2 (spec fn in trait impl, decreases on both):     FAIL — opaque through dispatch
//!   Test 3 (trait spec fn delegates to inherent):           PASS — solver unfolds inherent
//!   Test 4 (no trait spec fn, ensures references inherent): PASS — trait is exec-only
//!
//! Working patterns for recursive spec fns on enum types:
//!   - Keep recursive spec fns as inherent methods on the concrete type.
//!   - Option A (Test 3): Trait declares a non-recursive spec fn that
//!     delegates to the inherent. The inherent owns the recursion.
//!   - Option B (Test 4): Trait has no spec fns. Exec methods put ensures
//!     clauses that reference the inherent spec fns directly.

pub mod trait_decreases {
    use vstd::prelude::*;

    verus! {

    pub enum Tree<T> {
        Leaf,
        Node(Box<TreeNode<T>>),
    }

    pub struct TreeNode<T> {
        pub left: Tree<T>,
        pub value: T,
        pub right: Tree<T>,
    }

    // Test 1: Inherent spec fn (baseline). PASS.
    // The solver can unfold inherent methods directly.
    impl<T> Tree<T> {
        pub open spec fn inherent_size(self) -> nat
            decreases self,
        {
            match self {
                Tree::Leaf => 0,
                Tree::Node(node) => 1 + node.left.inherent_size() + node.right.inherent_size(),
            }
        }
    }

    fn test1_inherent_works<T>(t: &Tree<T>) -> (b: bool)
        ensures b == (t.inherent_size() == 0)
    {
        match t {
            Tree::Leaf => true,
            Tree::Node(_) => false,
        }
    }

    // Test 2: Trait spec fn with decreases on BOTH trait and impl. FAIL.
    // The solver cannot unfold spec_size through trait dispatch.
    // Commented out — verification fails as documented above.
    //
    // pub trait Trait2<T>: Sized {
    //     spec fn trait_size(self) -> nat
    //         decreases self;
    //
    //     fn is_leaf(&self) -> (b: bool)
    //         ensures b == (self.trait_size() == 0);
    // }
    //
    // impl<T> Trait2<T> for Tree<T> {
    //     open spec fn trait_size(self) -> nat
    //         decreases self,
    //     {
    //         match self {
    //             Tree::Leaf => 0,
    //             Tree::Node(node) => 1 + node.left.trait_size() + node.right.trait_size(),
    //         }
    //     }
    //
    //     fn is_leaf(&self) -> (b: bool) {
    //         match self {
    //             Tree::Leaf => true,
    //             Tree::Node(_) => false,
    //         }
    //     }
    // }

    // Test 3: Trait spec fn delegates to inherent. PASS.
    // The trait provides an interface; the inherent owns the recursion.
    pub trait Trait3<T>: Sized {
        spec fn trait3_size(self) -> nat;

        fn is_leaf3(&self) -> (b: bool)
            ensures b == (self.trait3_size() == 0);
    }

    impl<T> Trait3<T> for Tree<T> {
        open spec fn trait3_size(self) -> nat {
            self.inherent_size()
        }

        fn is_leaf3(&self) -> (b: bool) {
            match self {
                Tree::Leaf => true,
                Tree::Node(_) => false,
            }
        }
    }

    // Test 4: No trait spec fn; ensures references inherent directly. PASS.
    // The trait is a pure exec interface. All spec reasoning goes through
    // the concrete type's inherent methods.
    pub trait Trait4<T>: Sized {
        fn is_leaf4(&self) -> (b: bool);
    }

    impl<T> Trait4<T> for Tree<T> {
        fn is_leaf4(&self) -> (b: bool)
            ensures b == (self.inherent_size() == 0)
        {
            match self {
                Tree::Leaf => true,
                Tree::Node(_) => false,
            }
        }
    }

    } // verus!
}
