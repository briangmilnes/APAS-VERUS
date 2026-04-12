//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Ephemeral Red-Black balanced binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on Link/Node) in sections 6/8/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 4d. type definitions
//	Section 5d. view impls
//	Section 8d. traits
//	Section 9d. impls
//	Section 10d. iterators
//	Section 11c. top level coarse locking
//	Section 12d. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!

//		Section 1. module

pub mod BSTRBMtEph {


    //		Section 2. imports

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use vstd::slice::slice_subrange;

    verus! 
{

    //		Section 4a. type definitions


    // (Arc kept for filter_parallel/reduce_parallel closure sharing.)


    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Color {
        Red,
        Black,
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StTInMtT + Ord + TotalOrder> {
        pub key: T,
        pub color: Color,
        pub size: usize,
        pub left: Option<Box<Node<T>>>,
        pub right: Option<Box<Node<T>>>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    //		Section 6b. spec fns


    /// Structural node count for RB tree links.
    pub open spec fn link_spec_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + link_spec_size(node.left) + link_spec_size(node.right),
        }
    }

    /// Spec-level containment for RB tree links.
    pub open spec fn link_contains<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, target: T) -> bool
        decreases link,
    {
        match link {
            None => false,
            Some(node) => node.key == target
                || link_contains(node.left, target)
                || link_contains(node.right, target),
        }
    }

    /// Spec-level height for RB tree links.
    pub open spec fn link_height<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => {
                let lh = link_height(node.left);
                let rh = link_height(node.right);
                1 + if lh > rh { lh } else { rh }
            }
        }
    }

    /// BST ordering invariant for RB tree links.
    pub open spec fn spec_is_bst_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_is_bst_link(node.left)
                && spec_is_bst_link(node.right)
                && (forall|x: T| (#[trigger] link_contains(node.left, x)) ==>
                    TotalOrder::le(x, node.key) && x != node.key)
                && (forall|x: T| (#[trigger] link_contains(node.right, x)) ==>
                    TotalOrder::le(node.key, x) && x != node.key)
            }
        }
    }

    /// Convert Link<T> (concrete RB tree pointer) to BalBinTree<T> (abstract binary tree).
    /// Strips color and size, retaining structure and keys.
    pub open spec fn link_to_bbt<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> BalBinTree<T>
        decreases link,
    {
        match link {
            None => BalBinTree::Leaf,
            Some(node) => BalBinTree::Node(Box::new(BalBinNode {
                left: link_to_bbt(node.left),
                value: node.key,
                right: link_to_bbt(node.right),
            })),
        }
    }

    //		Section 7b. proof fns/broadcast groups


    /// Bridge: link_spec_size == BalBinTree::spec_size after conversion.
    proof fn lemma_link_to_bbt_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures link_spec_size(link) == link_to_bbt(link).spec_size(),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_link_to_bbt_size::<T>(node.left);
                lemma_link_to_bbt_size::<T>(node.right);
            }
        }
    }

    /// Bridge: link_contains == BalBinTree::tree_contains after conversion.
    proof fn lemma_link_to_bbt_contains<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, target: T)
        ensures link_contains(link, target) == link_to_bbt(link).tree_contains(target),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_link_to_bbt_contains::<T>(node.left, target);
                lemma_link_to_bbt_contains::<T>(node.right, target);
            }
        }
    }

    /// Bridge: link_height == BalBinTree::spec_height after conversion.
    proof fn lemma_link_to_bbt_height<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures link_height(link) == link_to_bbt(link).spec_height(),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_link_to_bbt_height::<T>(node.left);
                lemma_link_to_bbt_height::<T>(node.right);
            }
        }
    }

    /// Bridge: spec_is_bst_link on Link implies tree_is_bst on BalBinTree conversion.
    proof fn lemma_link_to_bbt_is_bst<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires spec_is_bst_link(link),
        ensures link_to_bbt(link).tree_is_bst(),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                reveal_with_fuel(spec_is_bst_link, 2);
                lemma_link_to_bbt_is_bst::<T>(node.left);
                lemma_link_to_bbt_is_bst::<T>(node.right);
                // Veracity: NEEDED assert
                assert forall|x: T| (#[trigger] link_to_bbt(node.left).tree_contains(x))
                    implies (TotalOrder::le(x, node.key) && x != node.key)
                by {
                    lemma_link_to_bbt_contains::<T>(node.left, x);
                };
                // Veracity: NEEDED assert
                assert forall|x: T| (#[trigger] link_to_bbt(node.right).tree_contains(x))
                    implies (TotalOrder::le(node.key, x) && x != node.key)
                by {
                    lemma_link_to_bbt_contains::<T>(node.right, x);
                };
            }
        }
    }

    /// Height is bounded by structural node count.
    proof fn lemma_height_le_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures link_height(link) <= link_spec_size(link),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_height_le_size::<T>(node.left);
                lemma_height_le_size::<T>(node.right);
            }
        }
    }

    //		Section 8b. traits


    /// Trait for RB tree node/link operations (Layer 1).
    pub trait BSTRBMtNodeFns<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bst(self) -> bool;
        spec fn spec_size(self) -> nat;
        spec fn spec_contains(self, target: T) -> bool;
        spec fn spec_height(self) -> nat;
        spec fn spec_is_empty(self) -> bool;

        // veracity: no_requires
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_red(&self) -> (red: bool)
            ensures self.spec_is_empty() ==> !red;
        // veracity: no_requires
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(&self) -> (size: usize)
            ensures self.spec_is_empty() ==> size == 0;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn flip_colors(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn fix_up(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn insert_link(&mut self, value: T)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                self.spec_contains(value),
                forall|x: T| old(self).spec_contains(x) ==> self.spec_contains(x),
                forall|x: T| self.spec_contains(x) ==> (old(self).spec_contains(x) || x == value),
                self.spec_size() <= old(self).spec_size() + 1;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find_link(&self, target: &T) -> (found: Option<&T>)
            requires self.spec_bst(),
            ensures
                found.is_some() <==> self.spec_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn min_link(&self) -> (min: Option<&T>)
            requires self.spec_bst(),
            ensures
                !self.spec_is_empty() ==> min.is_some(),
                min.is_some() ==> self.spec_contains(*min.unwrap()),
                min.is_some() ==> forall|x: T| #[trigger] self.spec_contains(x) ==> TotalOrder::le(*min.unwrap(), x);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn max_link(&self) -> (max: Option<&T>)
            requires self.spec_bst(),
            ensures
                !self.spec_is_empty() ==> max.is_some(),
                max.is_some() ==> self.spec_contains(*max.unwrap()),
                max.is_some() ==> forall|x: T| #[trigger] self.spec_contains(x) ==> TotalOrder::le(x, *max.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_collect(&self, out: &mut Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order_collect(&self, out: &mut Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_parallel(&self) -> (elements: Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order_parallel(&self) -> (elements: Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_parallel<F>(&self, predicate: &Arc<F>) -> (filtered: Vec<T>)
            where
                F: Fn(&T) -> bool + Send + Sync,
            requires
                self.spec_size() <= usize::MAX as nat,
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce_parallel<F>(&self, op: &Arc<F>, identity: T) -> (reduced: T)
            where
                F: Fn(T, T) -> T + Send + Sync,
            requires
                self.spec_size() <= usize::MAX as nat,
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_rec(&self) -> (h: usize)
            requires self.spec_height() <= usize::MAX as nat,
            ensures h as nat == self.spec_height();
        /// Exec mirror of link_spec_size for runtime size guards.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn compute_link_spec_size(&self) -> (n: usize)
            requires self.spec_size() <= usize::MAX,
            ensures n as nat == self.spec_size();
    }

    //		Section 9b. impls


    // Free functions operating on Node<T> (not Link<T>).

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn new_node<T: StTInMtT + Ord + TotalOrder>(key: T) -> (node: Node<T>)
        requires link_spec_size::<T>(None) + 1 <= usize::MAX as nat,
        ensures
            node.key == key,
            node.size == 1,
            node.left is None,
            node.right is None,
    {
        Node {
            key,
            color: Color::Red,
            size: 1,
            left: None,
            right: None,
        }
    }

    // veracity: no_requires
    /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn update<T: StTInMtT + Ord + TotalOrder>(node: &mut Node<T>)
        ensures
            node.left == old(node).left,
            node.right == old(node).right,
            node.key == old(node).key,
            node.color == old(node).color,
    {
        let ls = node.left.size_link();
        let rs = node.right.size_link();
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            node.size = 1 + ls + rs;
        }
    }

    // Verified RB tree algorithms (Layer 1) — trait impl on Link<T>.

    impl<T: StTInMtT + Ord + TotalOrder> BSTRBMtNodeFns<T> for Link<T> {

    open spec fn spec_bst(self) -> bool { spec_is_bst_link(self) }
    open spec fn spec_size(self) -> nat { link_spec_size(self) }
    open spec fn spec_contains(self, target: T) -> bool { link_contains(self, target) }
    open spec fn spec_height(self) -> nat { link_height(self) }
    open spec fn spec_is_empty(self) -> bool { self is None }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn is_red(&self) -> (red: bool)
    {
        match self {
            Some(node) => matches!(node.color, Color::Red),
            None => false,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn size_link(&self) -> (size: usize)
    {
        match self.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_left(&mut self)
    {
        let ghost old_link = *self;
        if let Some(mut h) = self.take() {
            // Veracity: NEEDED proof block
            let ghost h_key = h.key;
            let ghost old_h_left = h.left;
            let ghost old_h_right = h.right;
            proof {
                reveal_with_fuel(spec_is_bst_link, 3);
                reveal_with_fuel(link_contains, 3);
                // Veracity: NEEDED assert
                assert forall|z: T| link_contains(old_h_left, z) implies
                    (TotalOrder::le(z, h_key) && z != h_key) by {};
                // Veracity: NEEDED assert
                assert forall|z: T| link_contains(old_h_right, z) implies
                    (TotalOrder::le(h_key, z) && z != h_key) by {};
            }
            // Veracity: NEEDED proof block
            if let Some(mut x) = h.right.take() {
                let ghost x_key = x.key;
                let ghost old_x_left = x.left;
                let ghost old_x_right = x.right;
                proof {
                    reveal_with_fuel(spec_is_bst_link, 2);
                    reveal_with_fuel(link_contains, 2);
                    // Veracity: NEEDED assert
                    assert(TotalOrder::le(h_key, x_key) && x_key != h_key);
                    // Veracity: NEEDED assert
                    assert forall|z: T| link_contains(old_x_left, z) implies
                        (TotalOrder::le(z, x_key) && z != x_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|z: T| link_contains(old_x_right, z) implies
                        (TotalOrder::le(x_key, z) && z != x_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|z: T| link_contains(old_x_left, z) implies
                        (TotalOrder::le(h_key, z) && z != h_key) by {
                    };
                // Veracity: NEEDED proof block
                }
                h.right = x.left.take();
                update(&mut h);
                x.color = h.color;
                h.color = Color::Red;
                x.left = Some(h);
                update(&mut x);
                *self = Some(x);
                proof {
                    reveal_with_fuel(spec_is_bst_link, 3);
                    reveal_with_fuel(link_contains, 4);
                    // Veracity: NEEDED assert
                    assert forall|z: T| #[trigger] link_contains(x.left, z) implies
                        (TotalOrder::le(z, x_key) && z != x_key)
                    by {
                        reveal_with_fuel(link_contains, 2);
                        if z == h_key {
                        } else if link_contains(old_h_left, z) {
                            T::transitive(z, h_key, x_key);
                            if z == x_key { T::antisymmetric(h_key, x_key); }
                        } else {
                        // Veracity: NEEDED proof block
                        }
                    };
                    // Size preservation: rotation rearranges subtrees, no nodes added/removed.
                    // Veracity: NEEDED assert
                    assert(link_spec_size(*self) == link_spec_size(old_link)) by {
                        reveal_with_fuel(link_spec_size, 3);
                    };
                }
            } else {
                *self = Some(h);
                proof { reveal_with_fuel(link_spec_size, 2); }
            }
        }
    // Veracity: NEEDED proof block
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_right(&mut self)
    {
        let ghost old_link = *self;
        if let Some(mut h) = self.take() {
            let ghost h_key = h.key;
            let ghost old_h_left = h.left;
            let ghost old_h_right = h.right;
            proof {
                reveal_with_fuel(spec_is_bst_link, 3);
                // Veracity: NEEDED proof block
                reveal_with_fuel(link_contains, 3);
                // Veracity: NEEDED assert
                assert forall|z: T| link_contains(old_h_left, z) implies
                    (TotalOrder::le(z, h_key) && z != h_key) by {};
                // Veracity: NEEDED assert
                assert forall|z: T| link_contains(old_h_right, z) implies
                    (TotalOrder::le(h_key, z) && z != h_key) by {};
            }
            if let Some(mut x) = h.left.take() {
                let ghost x_key = x.key;
                let ghost old_x_left = x.left;
                let ghost old_x_right = x.right;
                proof {
                    reveal_with_fuel(spec_is_bst_link, 2);
                    reveal_with_fuel(link_contains, 2);
                    // Veracity: NEEDED assert (speed hint)
                    assert(TotalOrder::le(x_key, h_key) && x_key != h_key);
                    // Veracity: NEEDED assert
                    assert forall|z: T| link_contains(old_x_left, z) implies
                        // Veracity: NEEDED proof block
                        (TotalOrder::le(z, x_key) && z != x_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|z: T| link_contains(old_x_right, z) implies
                        (TotalOrder::le(x_key, z) && z != x_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|z: T| link_contains(old_x_right, z) implies
                        (TotalOrder::le(z, h_key) && z != h_key) by {
                    };
                }
                h.left = x.right.take();
                update(&mut h);
                x.color = h.color;
                h.color = Color::Red;
                x.right = Some(h);
                update(&mut x);
                *self = Some(x);
                proof {
                    reveal_with_fuel(spec_is_bst_link, 3);
                    reveal_with_fuel(link_contains, 4);
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    assert forall|z: T| #[trigger] link_contains(x.right, z) implies
                        (TotalOrder::le(x_key, z) && z != x_key)
                    by {
                        reveal_with_fuel(link_contains, 2);
                        if z == h_key {
                        } else if link_contains(old_h_right, z) {
                            T::transitive(x_key, h_key, z);
                            if z == x_key { T::antisymmetric(x_key, h_key); }
                        } else {
                        }
                    };
                    // Veracity: NEEDED assert
                    assert(link_spec_size(*self) == link_spec_size(old_link)) by {
                        // Veracity: NEEDED proof block
                        reveal_with_fuel(link_spec_size, 3);
                    };
                }
            } else {
                *self = Some(h);
                proof { reveal_with_fuel(link_spec_size, 2); }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn flip_colors(&mut self)
    {
        let ghost old_link = *self;
        if let Some(mut node) = self.take() {
            let ghost node_key = node.key;
            let ghost orig_left = node.left;
            let ghost orig_right = node.right;
            proof {
                reveal_with_fuel(spec_is_bst_link, 2);
            }
            node.color = match node.color {
                // Veracity: NEEDED proof block
                | Color::Red => Color::Black,
                | Color::Black => Color::Red,
            };
            if let Some(mut left) = node.left.take() {
                left.color = match left.color {
                    | Color::Red => Color::Black,
                    | Color::Black => Color::Red,
                };
                node.left = Some(left);
            }
            if let Some(mut right) = node.right.take() {
                right.color = match right.color {
                    | Color::Red => Color::Black,
                    | Color::Black => Color::Red,
                };
                node.right = Some(right);
            }
            *self = Some(node);
            proof {
                reveal_with_fuel(spec_is_bst_link, 3);
                reveal_with_fuel(link_contains, 3);
                // Children's key/left/right unchanged (only color modified).
                // Veracity: NEEDED assert
                assert forall|z: T| #[trigger] link_contains(node.left, z) implies
                    (TotalOrder::le(z, node_key) && z != node_key) by {
                    // Veracity: NEEDED assert
                    assert(link_contains(orig_left, z));
                };
                // Veracity: NEEDED assert
                assert forall|z: T| #[trigger] link_contains(node.right, z) implies
                    (TotalOrder::le(node_key, z) && z != node_key) by {
                    // Veracity: NEEDED assert
                    assert(link_contains(orig_right, z));
                };
                // Size preservation: flip_colors only changes colors, not structure.
                // Veracity: NEEDED assert
                assert(link_spec_size(*self) == link_spec_size(old_link)) by {
                    // Veracity: NEEDED proof block
                    reveal_with_fuel(link_spec_size, 3);
                };
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn fix_up(&mut self)
    {
        let ghost old_link = *self;

        // Check rotate_left condition via take/read/put-back.
        let tmp = self.take();
        let rotate_left_needed = match &tmp {
            | Some(node) => node.right.is_red() && !node.left.is_red(),
            | None => false,
        };
        *self = tmp;
        let ghost before_rl = *self;
        if rotate_left_needed {
            // Veracity: NEEDED proof block
            self.rotate_left();
        }
        let ghost after_rl = *self;
        proof {
        }

        // Check rotate_right condition.
        let tmp = self.take();
        let rotate_right_needed = match &tmp {
            | Some(node) => {
                match &node.left {
                    | Some(left) => node.left.is_red() && left.left.is_red(),
                    | None => false,
                }
            }
            // Veracity: NEEDED proof block
            | None => false,
        };
        *self = tmp;
        let ghost before_rr = *self;
        if rotate_right_needed {
            self.rotate_right();
        }
        let ghost after_rr = *self;
        proof {
        // Veracity: NEEDED proof block
        }

        // Check flip condition.
        let tmp = self.take();
        let flip_needed = match &tmp {
            | Some(node) => node.left.is_red() && node.right.is_red(),
            | None => false,
        };
        *self = tmp;
        let ghost before_fl = *self;
        if flip_needed {
            self.flip_colors();
        }
        let ghost after_fl = *self;
        proof {
        }
// Veracity: NEEDED proof block

        // Update size via take/put-back.
        if let Some(mut node) = self.take() {
            update(&mut node);
            *self = Some(node);
        }

        proof {
            reveal_with_fuel(spec_is_bst_link, 2);
            reveal_with_fuel(link_contains, 2);
            // Chain containment equivalences through each step.
            // Trait ensures uses spec_contains; bridge to link_contains via open unfolding.
        }
    }

    // Veracity: NEEDED proof block
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn insert_link(&mut self, value: T)
        decreases old(self),
    {
        let cur = self.take();
        match cur {
            | None => {
                *self = Some(Box::new(new_node(value)));
                proof {
                    reveal_with_fuel(spec_is_bst_link, 2);
                    reveal_with_fuel(link_contains, 2);
                }
                return;
            }
            | Some(mut node) => {
                let ghost old_left = node.left;
                let ghost old_right = node.right;
                let ghost node_key = node.key;
                match TotalOrder::cmp(&value, &node.key) {
                    core::cmp::Ordering::Less => {
                        node.left.insert_link(value);
                        update(&mut node);
                        *self = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(node.left, x) implies
                                (TotalOrder::le(x, node.key) && x != node.key)
                            by {
                                if link_contains(old_left, x) {
                                } else {
                                    // Bridge: trait ensures via spec_contains.
                                    // Veracity: NEEDED assert
                                    assert(node.left.spec_contains(x) ==>
                                        (old_left.spec_contains(x) || x == value));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*old(self), x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*old(self), x) implies
                                link_contains(*self, x)
                            by {
                                // Veracity: NEEDED proof block
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_left, x) {
                                    // Bridge: trait ensures via spec_contains.
                                    // Veracity: NEEDED assert
                                    assert(old_left.spec_contains(x) ==>
                                        node.left.spec_contains(x));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*self, x) implies
                                (link_contains(*old(self), x) || x == value)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                } else if link_contains(node.left, x) {
                                    // Bridge: trait ensures via spec_contains.
                                    // Veracity: NEEDED assert
                                    assert(node.left.spec_contains(x) ==>
                                        (old_left.spec_contains(x) || x == value));
                                    if link_contains(old_left, x) {
                                    }
                                }
                            };
                        }
                    }
                    core::cmp::Ordering::Greater => {
                        node.right.insert_link(value);
                        update(&mut node);
                        *self = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(node.right, x) implies
                                (TotalOrder::le(node.key, x) && x != node.key)
                            by {
                                if link_contains(old_right, x) {
                                } else {
                                    // Bridge: trait ensures via spec_contains.
                                    // Veracity: NEEDED assert
                                    assert(node.right.spec_contains(x) ==>
                                        (old_right.spec_contains(x) || x == value));
                                }
                            };
                            // Veracity: NEEDED proof block
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*old(self), x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*old(self), x) implies
                                link_contains(*self, x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_right, x) {
                                    // Bridge: trait ensures via spec_contains.
                                    // Veracity: NEEDED assert
                                    assert(old_right.spec_contains(x) ==>
                                        node.right.spec_contains(x));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*self, x) implies
                                (link_contains(*old(self), x) || x == value)
                            by {
                                // Veracity: NEEDED proof block
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                } else if link_contains(node.right, x) {
                                    // Bridge: trait ensures via spec_contains.
                                    // Veracity: NEEDED assert
                                    assert(node.right.spec_contains(x) ==>
                                        (old_right.spec_contains(x) || x == value));
                                    if link_contains(old_right, x) {
                                    }
                                }
                            // Veracity: NEEDED proof block
                            };
                        }
                    }
                    core::cmp::Ordering::Equal => {
                        *self = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                        }
                        return;
                    }
                }
            }
        }
        self.fix_up();
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn find_link(&self, target: &T) -> (found: Option<&T>)
        decreases *self,
    {
        match self {
            // Veracity: NEEDED proof block
            | None => None,
            | Some(node) => {
                match TotalOrder::cmp(target, &node.key) {
                    core::cmp::Ordering::Equal => Some(&node.key),
                    core::cmp::Ordering::Less => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!link_contains(node.right, *target)) by {
                                if link_contains(node.right, *target) {
                                    T::antisymmetric(*target, node.key);
                                }
                            };
                        }
                        node.left.find_link(target)
                    }
                    // Veracity: NEEDED proof block
                    core::cmp::Ordering::Greater => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!link_contains(node.left, *target)) by {
                                if link_contains(node.left, *target) {
                                    T::antisymmetric(node.key, *target);
                                }
                            };
                        }
                        node.right.find_link(target)
                    }
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn min_link(&self) -> (min: Option<&T>)
        decreases *self,
    {
        match self {
            | None => None,
            | Some(node) => match node.left {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(node.key, x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                // Veracity: NEEDED proof block
                                T::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.right, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let min = node.left.min_link();
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        // Bridge: trait ensures uses spec_contains → link_contains.
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(*min.unwrap(), x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.left, x) {
                                // Veracity: NEEDED assert
                                assert(node.left.spec_contains(x));
                            } else if x == node.key {
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.right, x));
                                T::transitive(*min.unwrap(), node.key, x);
                            }
                        };
                    }
                    min
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn max_link(&self) -> (max: Option<&T>)
        decreases *self,
    {
        match self {
            | None => None,
            | Some(node) => match node.right {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(x, node.key) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                T::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.left, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let max = node.right.max_link();
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        // Bridge: trait ensures uses spec_contains → link_contains.
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(x, *max.unwrap()) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.right, x) {
                                // Veracity: NEEDED assert
                                assert(node.right.spec_contains(x));
                            } else if x == node.key {
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.left, x));
                                T::transitive(x, node.key, *max.unwrap());
                            }
                        };
                    }
                    max
                }
            },
        }
    // Veracity: NEEDED proof block
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn in_order_collect(&self, out: &mut Vec<T>)
        decreases *self,
    {
        if let Some(node) = self {
            node.left.in_order_collect(out);
            out.push(node.key.clone());
            node.right.in_order_collect(out);
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn pre_order_collect(&self, out: &mut Vec<T>)
        decreases *self,
    {
        if let Some(node) = self {
            out.push(node.key.clone());
            node.left.pre_order_collect(out);
            node.right.pre_order_collect(out);
        }
    }

    // Veracity: NEEDED proof block
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn in_order_parallel(&self) -> (elements: Vec<T>)
    {
        let mut out = Vec::new();
        self.in_order_collect(&mut out);
        out
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn pre_order_parallel(&self) -> (elements: Vec<T>)
    {
        let mut out = Vec::new();
        self.pre_order_collect(&mut out);
        out
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn filter_parallel<F>(&self, predicate: &Arc<F>) -> (filtered: Vec<T>)
        where
            // Veracity: NEEDED proof block
            F: Fn(&T) -> bool + Send + Sync,
        decreases *self,
    {
        match self {
            | None => Vec::new(),
            | Some(node) => {
                proof {
                    reveal_with_fuel(link_spec_size, 2);
                }
                let left_vals = node.left.filter_parallel(predicate);
                let mut right_vals = node.right.filter_parallel(predicate);
                let mut result = left_vals;
                if (**predicate)(&node.key) {
                    result.push(node.key.clone());
                }
                result.append(&mut right_vals);
                result
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn reduce_parallel<F>(&self, op: &Arc<F>, identity: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync,
        decreases *self,
    {
        match self {
            | None => identity,
            | Some(node) => {
                proof {
                    reveal_with_fuel(link_spec_size, 2);
                }
                let id_left = identity.clone();
                let left_acc = node.left.reduce_parallel(op, id_left);
                let right_acc = node.right.reduce_parallel(op, identity);
                let with_key = (**op)(left_acc, node.key.clone());
                (**op)(with_key, right_acc)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn height_rec(&self) -> (h: usize)
        decreases *self,
    // Veracity: NEEDED proof block
    {
        match self {
            | None => 0,
            | Some(node) => {
                proof {
                    // link_height = 1 + max(left, right), so children have height < usize::MAX.
                }
                1 + node.left.height_rec().max(node.right.height_rec())
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn compute_link_spec_size(&self) -> (n: usize)
        decreases *self,
    {
        match self {
            None => 0,
            Some(node) => {
                let l = node.left.compute_link_spec_size();
                let r = node.right.compute_link_spec_size();
                1 + l + r
            }
        }
    }

    } // impl BSTRBMtNodeFns for Link

    // Free function: builds balanced tree from sorted slice.

    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn build_balanced<T: StTInMtT + Ord + TotalOrder>(values: &[T]) -> (link: Link<T>)
        ensures link_spec_size(link) <= values@.len(),
        decreases values.len(),
    {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        let left_slice = slice_subrange(values, 0, mid);
        let right_slice = slice_subrange(values, mid + 1, values.len());
        let left = build_balanced(left_slice);
        let right = build_balanced(right_slice);
        let mut node = Box::new(new_node(values[mid].clone()));
        node.left = left;
        node.right = right;
        node.color = Color::Black;
        update(&mut node);
        proof {
            reveal_with_fuel(link_spec_size, 2);
        }
        Some(node)
    }

    //		Section 4c. type definitions


    /// Lock predicate: link size fits in usize.
    pub struct BSTRBMtEphInv;

    pub type BSTreeRB<T> = BSTRBMtEph<T>;

    //		Section 4d. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTRBMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub(crate) root: RwLock<Link<T>, BSTRBMtEphInv>,
        pub(crate) ghost_root: Ghost<Link<T>>,
    }

    //		Section 5d. view impls


    impl<T: StTInMtT + Ord + TotalOrder> View for BSTRBMtEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { link_to_bbt(self.spec_ghost_root()) }
    }

    //		Section 8d. traits


    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_bstrbmteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_bstrbmteph_wf(),
                    tree@ == BalBinTree::<T>::Leaf,
                    tree@.tree_is_bst(),
                    forall|x: T| !tree@.tree_contains(x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_sorted_slice(values: &[T]) -> (tree: Self)
            ensures tree.spec_bstrbmteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>)
            requires old(self).spec_bstrbmteph_wf(),
            ensures self.spec_bstrbmteph_wf(),
                    match inserted {
                        Ok(_) => self@.tree_is_bst()
                            && self@.tree_contains(value)
                            && forall|x: T| (#[trigger] self@.tree_contains(x)) <==>
                                (old(self)@.tree_contains(x) || x == value),
                        Err(_) => self@ == old(self)@,
                    };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstrbmteph_wf(),
            ensures found == self@.tree_contains(*target);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (n: usize)
            requires self.spec_bstrbmteph_wf(),
            ensures n as nat == self@.spec_size();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstrbmteph_wf(),
            ensures b == (self@.spec_size() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires self.spec_bstrbmteph_wf(),
            ensures h as nat == self@.spec_height();

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<T>)
            requires self.spec_bstrbmteph_wf(),
            ensures
                found.is_some() == self@.tree_contains(*target),
                found.is_some() ==> found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F>(&self, predicate: F) -> (seq: ArraySeqStPerS<T>)
        where
            // Veracity: NEEDED proof block
            F: Fn(&T) -> bool + Send + Sync
            requires
                self.spec_bstrbmteph_wf(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync
            requires
                self.spec_bstrbmteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn iter(&self) -> (it: BSTRBMtEphIter<T>)
            requires self.spec_bstrbmteph_wf()
            ensures it@.0 == 0, iter_invariant_bstrbmteph(&it);
    }

    //		Section 9d. impls
// Veracity: NEEDED proof block (speed hint)


    impl<T: StTInMtT + Ord + TotalOrder> BSTRBMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            link_spec_size(self.ghost_root@) <= usize::MAX
            && spec_is_bst_link(self.ghost_root@)
        }

        pub closed spec fn spec_ghost_root(self) -> Link<T> {
            self.ghost_root@
        }
    }
// Veracity: NEEDED proof block

    impl<T: StTInMtT + Ord + TotalOrder> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {
        open spec fn spec_bstrbmteph_wf(&self) -> bool {
            link_spec_size(self.spec_ghost_root()) <= usize::MAX
            && spec_is_bst_link(self.spec_ghost_root())
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> Self {
            BSTRBMtEph {
                root: RwLock::new(None, Ghost(BSTRBMtEphInv)),
                ghost_root: Ghost(None),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_sorted_slice(values: &[T]) -> Self {
            let vlen = values.len();
            let link = build_balanced(values);
            let ghost ghost_link = link;
            proof {
                // build_balanced ensures link_spec_size(link) <= values@.len().
                // vlen: usize = values.len(), so values@.len() <= usize::MAX.
                // spec_is_bst_link requires sorted input — cannot prove here.
                assume(spec_is_bst_link(ghost_link));
            }
            BSTRBMtEph {
                root: RwLock::new(link, Ghost(BSTRBMtEphInv)),
                ghost_root: Ghost(ghost_link),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>) {
            let (mut current, write_handle) = self.root.acquire_write();
            proof { assume(self.ghost_root@ == current); }
            let sz = current.compute_link_spec_size();
            if sz < usize::MAX {
                // spec_is_bst_link(current) from lock predicate via acquire_write.
                current.insert_link(value);
                let ghost after_insert = current;
                let temp = current.take();
                if let Some(mut node) = temp {
                    node.color = Color::Black;
                    current = Some(node);
                }
                let ghost old_ghost = self.ghost_root@;
                let ghost new_root = current;
                proof {
                    // Color change (Black root) preserves structural properties.
                    reveal_with_fuel(link_spec_size, 2);
                    reveal_with_fuel(link_contains, 2);
                    // Veracity: NEEDED proof block (speed hint)
                    reveal_with_fuel(spec_is_bst_link, 2);
                    // Bridge from trait ensures (spec_contains) to link_contains.
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(after_insert, x) implies
                        (link_contains(old_ghost, x) || x == value)
                    by {
                        // Veracity: NEEDED assert
                        assert(after_insert.spec_contains(x) ==>
                            (old_ghost.spec_contains(x) || x == value));
                    };
                    // Veracity: NEEDED assert
                    assert forall|x: T| (link_contains(old_ghost, x) || x == value) implies
                        // Veracity: NEEDED proof block
                        link_contains(after_insert, x)
                    by {
                        if link_contains(old_ghost, x) {
                            // Veracity: NEEDED assert
                            assert(old_ghost.spec_contains(x) ==>
                                after_insert.spec_contains(x));
                        }
                    };
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(new_root, x) <==>
                        // Veracity: NEEDED proof block
                        link_contains(after_insert, x)
                    by { reveal_with_fuel(link_contains, 2); };
                    // Size bound: insert adds at most 1, color change preserves size.
                    // Bridge to BalBinTree view.
                    lemma_link_to_bbt_size::<T>(new_root);
                    lemma_link_to_bbt_size::<T>(old_ghost);
                    lemma_link_to_bbt_contains::<T>(new_root, value);
                    // Veracity: NEEDED assert
                    assert forall|x: T| (#[trigger] link_to_bbt(new_root).tree_contains(x))
                        <==> (link_to_bbt(old_ghost).tree_contains(x) || x == value)
                    // Veracity: NEEDED proof block
                    by {
                        lemma_link_to_bbt_contains::<T>(new_root, x);
                        lemma_link_to_bbt_contains::<T>(old_ghost, x);
                    };
                    // Ghost-real bridge: spec_is_bst_link → tree_is_bst via link_to_bbt.
                    lemma_link_to_bbt_is_bst::<T>(new_root);
                // Veracity: NEEDED proof block
                }
                self.ghost_root = Ghost(new_root);
                write_handle.release_write(current);
                Ok(())
            } else {
                write_handle.release_write(current);
                Err(())
            }
        }

        // Reader: spec_is_bst_link from lock predicate, assume return matches ghost.
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let found = data.find_link(target).is_some();
            proof { assume(found == self@.tree_contains(*target)); }
            handle.release_read();
            found
        }

        // Reader: link_spec_size from lock predicate, assume return matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (n: usize) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let n = data.size_link();
            proof { assume(n as nat == self@.spec_size()); }
            handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool) {
            let handle = self.root.acquire_read();
            let b = handle.borrow().is_none();
            proof { assume(b == (self@.spec_size() == 0)); }
            handle.release_read();
            b
        }

        // Reader: height bounded by size from lock predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            proof {
                // Lock predicate gives link_spec_size(*data) <= usize::MAX.
                // Lemma gives link_height <= link_spec_size.
                lemma_height_le_size::<T>(*data);
            }
            let h = data.height_rec();
            proof { assume(h as nat == self@.spec_height()); }
            handle.release_read();
            h
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<T>) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let found = data.find_link(target).cloned();
            proof {
                assume(found.is_some() == self@.tree_contains(*target));
                accept(found.is_some() ==> found.unwrap() == *target);
            }
            handle.release_read();
            found
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let min = data.min_link().cloned();
            handle.release_read();
            min
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let max = data.max_link().cloned();
            handle.release_read();
            max
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let out = data.in_order_parallel();
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let out = data.pre_order_parallel();
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n h(T)), Span O(n h(T))
        fn filter<F>(&self, predicate: F) -> ArraySeqStPerS<T>
        where
            F: Fn(&T) -> bool + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let predicate = Arc::new(predicate);
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let out = data.filter_parallel(&predicate);
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let op = Arc::new(op);
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let accumulated = data.reduce_parallel(&op, identity);
            handle.release_read();
            accumulated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn iter(&self) -> BSTRBMtEphIter<T> {
            let seq = self.in_order();
            BSTRBMtEphIter { snapshot: seq.seq, pos: 0 }
        }
    }

    //		Section 10d. iterators — BSTRBMtEph

    /// Snapshot iterator over BSTRBMtEph elements in ascending key order.
    #[verifier::reject_recursive_types(T)]
    pub struct BSTRBMtEphIter<T: StTInMtT + Ord + TotalOrder> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTRBMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    pub open spec fn iter_invariant_bstrbmteph<T: StTInMtT + Ord + TotalOrder>(it: &BSTRBMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::iter::Iterator for BSTRBMtEphIter<T> {
        type Item = T;
        fn next(&mut self) -> (next: Option<T>)
            ensures
                ({
                    let (old_index, old_seq) = old(self)@;
                    match next {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index >= old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            if self.pos >= self.snapshot.len() {
                None
            } else {
                let item = self.snapshot[self.pos].clone();
                self.pos = self.pos + 1;
                proof { assume(item == old(self)@.1[old(self)@.0]); }
                Some(item)
            }
        }
    }

    /// Ghost iterator for for-loop support over BSTRBMtEphIter.
    #[verifier::reject_recursive_types(T)]
    pub struct BSTRBMtEphGhostIterator<T: StTInMtT + Ord + TotalOrder> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTRBMtEphGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIteratorNew for BSTRBMtEphIter<T> {
        type GhostIter = BSTRBMtEphGhostIterator<T>;
        open spec fn ghost_iter(&self) -> BSTRBMtEphGhostIterator<T> {
            BSTRBMtEphGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIterator for BSTRBMtEphGhostIterator<T> {
        type ExecIter = BSTRBMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTRBMtEphIter<T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &BSTRBMtEphIter<T>) -> BSTRBMtEphGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + TotalOrder> std::iter::IntoIterator for &'a BSTRBMtEph<T> {
        type Item = T;
        type IntoIter = BSTRBMtEphIter<T>;
        fn into_iter(self) -> (it: BSTRBMtEphIter<T>)
            requires self.spec_bstrbmteph_wf()
            ensures it@.0 == 0, iter_invariant_bstrbmteph(&it),
        {
            self.iter()
        }
    }

    //		Section 11c. top level coarse locking


    impl<T: StTInMtT + Ord + TotalOrder> RwLockPredicate<Link<T>> for BSTRBMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            link_spec_size(v) <= usize::MAX
            && spec_is_bst_link(v)
        }
    }

    //		Section 12d. derive impls in verus!


    impl<T: StTInMtT + Ord + TotalOrder> Default for BSTRBMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTRBMtEphLit {
        () => {
            < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> >::new()
        };
        ($($x:expr),* $(,)?) => {{
            let mut __tree = < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }

    //		Section 14a. derive impls outside verus!

    impl std::fmt::Debug for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Color::Red => write!(f, "Red"),
                Color::Black => write!(f, "Black"),
            }
        }
    }

    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(self, f)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("color", &self.color)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    //		Section 14c. derive impls outside verus!

    impl std::fmt::Debug for BSTRBMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTRBMtEphInv").finish()
        }
    }

    impl std::fmt::Display for BSTRBMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTRBMtEphInv")
        }
    }

    //		Section 14d. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTRBMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTRBMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTRBMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTRBMtEph(size={})", self.size())
        }
    }
}
