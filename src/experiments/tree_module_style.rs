// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Experiment: Separate structs per node kind with per-type traits, mutable
//! keys via set_key, Tree as a wrapper (not in the enum), no inherent impl
//! blocks, no free spec fns — all specs live in trait impls only.
//!
//! Hypothesis: Trait impl spec fns can reference other trait impl spec fns
//! on children (via NodeTrait::spec_size on Box<Node>) without free fns.
//!
//! Result: YES. No free spec fns needed. Trait impl specs call
//! NodeTrait::spec_size(&*n) on children directly. Verus resolves the
//! single impl and unfolds through the recursive trait dispatch.

pub mod tree_module_style {

    use vstd::prelude::*;

    verus! {

    // 4. type definitions

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

    pub trait LeafTrait: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;
        spec fn spec_contains(&self, needle: u64) -> bool;

        fn new(key: u64) -> (t: Self)
            ensures
                t.spec_size() == 1,
                t.spec_height() == 1,
                t.spec_contains(key);

        fn set_key(&mut self, key: u64)
            ensures
                self.spec_size() == old(self).spec_size(),
                self.spec_height() == old(self).spec_height(),
                self.spec_contains(key);
    }

    pub trait InteriorTrait: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;
        spec fn spec_contains(&self, needle: u64) -> bool;

        fn new(key: u64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> (t: Self)
            ensures t.spec_contains(key);

        fn set_key(&mut self, key: u64)
            ensures
                self.spec_size() == old(self).spec_size(),
                self.spec_height() == old(self).spec_height(),
                self.spec_contains(key);
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
                t.spec_height() == 0;
    }

    // 9. impls

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
            1 + if l >= r { l } else { r }
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
                    1 + if l >= r { l } else { r }
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
}
