// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Multi-Struct Standard: how to structure modules with multiple data types.
//!
//! Tree-like types with multiple node kinds use separate structs composed into
//! a discriminated enum, each with its own trait. This pattern applies to BSTs,
//! AVL trees, B-trees, and any recursive ADT with variant-specific behavior.
//!
//! Key rules:
//! - Separate structs per variant (Leaf, Interior), composed into enum (Node).
//! - Wrapper struct (Tree) outside the enum.
//! - Per-type traits (LeafTrait, InteriorTrait, NodeTrait, TreeTrait).
//! - Bottom-up ordering: structs, traits, impls.
//! - Impl member order matches trait declaration order.
//! - Recursive spec fns go directly in trait impls with `decreases *self`.
//! - Child traversal uses qualified trait calls: `NodeTrait::spec_size(&*n)`.
//! - No inherent impl blocks, no free spec fns, no stub delegation.
//!
//! Reference: src/experiments/tree_module_style.rs
// 1. module
pub mod multi_struct_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    // 4. type definitions
    //
    // Bottom-up: leaf first, then interior, then enum, then wrapper.
    pub struct Leaf {
        pub key: u64,
    }

    pub struct Interior {
        pub key: u64,
        pub left: Option<Box<Node>>,
        pub right: Option<Box<Node>>,
    }

    pub enum Node {
        LeafNode(Leaf),
        InteriorNode(Interior),
    }

    pub struct Tree {
        pub child: Option<Box<Node>>,
    }

    // 8. traits
    //
    // Per-type traits. Each type gets its own trait with abstract specs and
    // exec methods. Trait order matches struct order (bottom-up).
    pub trait LeafTrait: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_height(&self) -> nat;

        spec fn spec_contains(&self, needle: u64) -> bool;

        fn new(key: u64) -> (t: Self)
            ensures
                t.spec_size() == 1,
                t.spec_height() == 1,
                t.spec_contains(key),
        ;

        fn set_key(&mut self, key: u64)
            ensures
                self.spec_size() == old(self).spec_size(),
                self.spec_height() == old(self).spec_height(),
                self.spec_contains(key),
        ;
    }

    pub trait InteriorTrait: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_height(&self) -> nat;

        spec fn spec_contains(&self, needle: u64) -> bool;

        fn new(key: u64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> (t: Self)
            ensures
                t.spec_contains(key),
        ;

        fn set_key(&mut self, key: u64)
            ensures
                self.spec_size() == old(self).spec_size(),
                self.spec_height() == old(self).spec_height(),
                self.spec_contains(key),
        ;
    }

    pub trait NodeTrait: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_height(&self) -> nat;

        spec fn spec_contains(&self, needle: u64) -> bool;
    }

    pub trait TreeTrait: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_height(&self) -> nat;

        fn new() -> (t: Self)
            ensures
                t.spec_size() == 0,
                t.spec_height() == 0,
        ;
    }

    // 9. impls
    //
    // Impl order matches trait order (bottom-up).
    // Non-recursive types reference children via qualified trait calls.
    impl LeafTrait for Leaf {
        open spec fn spec_size(&self) -> nat {
            1
        }

        open spec fn spec_height(&self) -> nat {
            1
        }

        open spec fn spec_contains(&self, needle: u64) -> bool {
            needle == self.key
        }

        fn new(key: u64) -> (t: Self) {
            Leaf { key }
        }

        fn set_key(&mut self, key: u64) {
            self.key = key;
        }
    }

    // Non-recursive: Interior references children via NodeTrait::spec_size(&*n).
    impl InteriorTrait for Interior {
        open spec fn spec_size(&self) -> nat {
            let l = match self.left {
                None => 0nat,
                Some(n) => NodeTrait::spec_size(&*n),
            };
            let r = match self.right {
                None => 0nat,
                Some(n) => NodeTrait::spec_size(&*n),
            };
            1 + l + r
        }

        open spec fn spec_height(&self) -> nat {
            let l = match self.left {
                None => 0nat,
                Some(n) => NodeTrait::spec_height(&*n),
            };
            let r = match self.right {
                None => 0nat,
                Some(n) => NodeTrait::spec_height(&*n),
            };
            1 + if l >= r {
                l
            } else {
                r
            }
        }

        open spec fn spec_contains(&self, needle: u64) -> bool {
            if needle == self.key {
                true
            } else if needle < self.key {
                match self.left {
                    None => false,
                    Some(n) => NodeTrait::spec_contains(&*n, needle),
                }
            } else {
                match self.right {
                    None => false,
                    Some(n) => NodeTrait::spec_contains(&*n, needle),
                }
            }
        }

        fn new(key: u64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> (t: Self) {
            Interior { key, left, right }
        }

        fn set_key(&mut self, key: u64) {
            self.key = key;
        }
    }

    // Recursive: Node impl uses `decreases *self` on each spec fn.
    // Verus resolves the single impl and unfolds through trait dispatch.
    impl NodeTrait for Node {
        open spec fn spec_size(&self) -> nat
            decreases *self,
        {
            match *self {
                Node::LeafNode(_) => 1,
                Node::InteriorNode(i) => {
                    let l = match i.left {
                        None => 0nat,
                        Some(n) => NodeTrait::spec_size(&*n),
                    };
                    let r = match i.right {
                        None => 0nat,
                        Some(n) => NodeTrait::spec_size(&*n),
                    };
                    1 + l + r
                },
            }
        }

        open spec fn spec_height(&self) -> nat
            decreases *self,
        {
            match *self {
                Node::LeafNode(_) => 1,
                Node::InteriorNode(i) => {
                    let l = match i.left {
                        None => 0nat,
                        Some(n) => NodeTrait::spec_height(&*n),
                    };
                    let r = match i.right {
                        None => 0nat,
                        Some(n) => NodeTrait::spec_height(&*n),
                    };
                    1 + if l >= r {
                        l
                    } else {
                        r
                    }
                },
            }
        }

        open spec fn spec_contains(&self, needle: u64) -> bool
            decreases *self,
        {
            match *self {
                Node::LeafNode(l) => needle == l.key,
                Node::InteriorNode(i) => {
                    if needle == i.key {
                        true
                    } else if needle < i.key {
                        match i.left {
                            None => false,
                            Some(n) => NodeTrait::spec_contains(&*n, needle),
                        }
                    } else {
                        match i.right {
                            None => false,
                            Some(n) => NodeTrait::spec_contains(&*n, needle),
                        }
                    }
                },
            }
        }
    }

    // Wrapper struct delegates to NodeTrait on its child.
    impl TreeTrait for Tree {
        open spec fn spec_size(&self) -> nat {
            match self.child {
                None => 0,
                Some(n) => NodeTrait::spec_size(&*n),
            }
        }

        open spec fn spec_height(&self) -> nat {
            match self.child {
                None => 0,
                Some(n) => NodeTrait::spec_height(&*n),
            }
        }

        fn new() -> (t: Self) {
            Tree { child: None }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl Debug for Leaf {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Leaf({})", self.key)
        }
    }

    impl Display for Leaf {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    impl Debug for Interior {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Interior({}, {:?}, {:?})", self.key, self.left, self.right)
        }
    }

    impl Display for Interior {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Interior({})", self.key)
        }
    }

    impl Debug for Node {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Node::LeafNode(l) => write!(f, "{:?}", l),
                Node::InteriorNode(i) => write!(f, "{:?}", i),
            }
        }
    }

    impl Display for Node {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Node::LeafNode(l) => write!(f, "{}", l),
                Node::InteriorNode(i) => write!(f, "{}", i),
            }
        }
    }

    impl Debug for Tree {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Tree({:?})", self.child)
        }
    }

    impl Display for Tree {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match &self.child {
                None => write!(f, "Tree(empty)"),
                Some(n) => write!(f, "Tree({})", n),
            }
        }
    }
} // pub mod multi_struct_standard
