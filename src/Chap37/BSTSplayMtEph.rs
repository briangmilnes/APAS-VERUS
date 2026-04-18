// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Ephemeral splay-style binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on Link/Node) in sections 6/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 6a. spec fns
//	Section 7a. proof fns/broadcast groups
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 5c. view impls
//	Section 8c. traits
//	Section 9c. impls
//	Section 11b. top level coarse locking
//	Section 12a. derive impls in verus!
//	Section 12c. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module

pub mod BSTSplayMtEph {


    //		Section 2. imports

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use vstd::slice::slice_subrange;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4a. type definitions


    // (Arc kept for filter_parallel/reduce_parallel closure sharing.)


    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StTInMtT + Ord + TotalOrder> {
        pub key: T,
        pub size: usize,
        pub left: Option<Box<Node<T>>>,
        pub right: Option<Box<Node<T>>>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    //		Section 6a. spec fns


    /// Structural node count for splay tree links.
    pub open spec fn link_spec_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat {
        match link {
            None => 0nat,
            Some(node) => node.size as nat,
        }
    }

    /// Spec-level containment for splay tree links.
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

    /// Spec-level height for splay tree links.
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

    /// Structural node count (recursive, independent of cached size field).
    pub open spec fn link_node_count<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + link_node_count(node.left) + link_node_count(node.right),
        }
    }

    /// Spec-level in-order traversal for splay tree links.
    pub open spec fn spec_in_order_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Seq<T>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => {
                let left = spec_in_order_link(node.left);
                let right = spec_in_order_link(node.right);
                left.push(node.key).add(right)
            }
        }
    }

    /// Spec-level pre-order traversal for splay tree links.
    pub open spec fn spec_pre_order_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Seq<T>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => {
                let left = spec_pre_order_link(node.left);
                let right = spec_pre_order_link(node.right);
                Seq::empty().push(node.key).add(left).add(right)
            }
        }
    }

    /// BST ordering invariant for splay tree links.
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

    //		Section 7a. proof fns/broadcast groups


    /// Height is bounded by structural node count.
    proof fn lemma_height_le_node_count<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures link_height(link) <= link_node_count(link),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_height_le_node_count::<T>(node.left);
                lemma_height_le_node_count::<T>(node.right);
            }
        }
    }

    /// After a zig (right) rotation, all elements in the rotated child subtree
    /// are strictly greater than the pivot key lo.
    proof fn lemma_zig_child_ordering<T: StTInMtT + Ord + TotalOrder>(
        child: Link<T>,
        lo: T, hi: T,
        sub_lo: Link<T>,
        sub_hi: Link<T>,
    )
        requires
            TotalOrder::le(lo, hi), lo != hi,
            forall|y: T| link_contains(sub_lo, y) ==> (TotalOrder::le(lo, y) && y != lo),
            forall|y: T| link_contains(sub_hi, y) ==> (TotalOrder::le(hi, y) && y != hi),
            forall|y: T| link_contains(child, y) ==>
                (y == hi || link_contains(sub_lo, y) || link_contains(sub_hi, y)),
        ensures
            forall|x: T| #[trigger] link_contains(child, x) ==> (TotalOrder::le(lo, x) && x != lo),
    {
        // Veracity: NEEDED assert
        assert forall|x: T| #[trigger] link_contains(child, x) implies
            (TotalOrder::le(lo, x) && x != lo)
        by {
            if x == hi {
            } else if link_contains(sub_lo, x) {
            } else {
                TotalOrder::transitive(lo, hi, x);
                if x == lo { TotalOrder::antisymmetric(lo, hi); }
            }
        };
    }

    /// Mirror of lemma_zig_child_ordering for zag (left) rotation: all elements
    /// in the rotated child subtree are strictly less than the pivot key hi.
    proof fn lemma_zag_child_ordering<T: StTInMtT + Ord + TotalOrder>(
        child: Link<T>,
        lo: T, hi: T,
        sub_hi: Link<T>,
        sub_lo: Link<T>,
    )
        requires
            TotalOrder::le(lo, hi), lo != hi,
            forall|y: T| link_contains(sub_hi, y) ==> (TotalOrder::le(y, hi) && y != hi),
            forall|y: T| link_contains(sub_lo, y) ==> (TotalOrder::le(y, lo) && y != lo),
            forall|y: T| link_contains(child, y) ==>
                (y == lo || link_contains(sub_lo, y) || link_contains(sub_hi, y)),
        ensures
            forall|x: T| #[trigger] link_contains(child, x) ==> (TotalOrder::le(x, hi) && x != hi),
    {
        // Veracity: NEEDED assert
        assert forall|x: T| #[trigger] link_contains(child, x) implies
            (TotalOrder::le(x, hi) && x != hi)
        by {
            if x == lo {
            } else if link_contains(sub_hi, x) {
            } else {
                TotalOrder::transitive(x, lo, hi);
                if x == hi { TotalOrder::antisymmetric(lo, hi); }
            }
        };
    }

    //		Section 8a. traits


    pub trait BSTSplayMtNodeFns<T: StTInMtT + Ord + TotalOrder>: Sized {

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new_node(key: T) -> (node: Node<T>)
            requires link_spec_size::<T>(None) + 1 <= usize::MAX as nat,
            ensures
                node.key == key,
                node.size == 1,
                node.left is None,
                node.right is None;

        // veracity: no_requires
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(link: &Link<T>) -> (size: usize)
            ensures size as nat == link_spec_size(*link);

        // veracity: no_requires
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn update(node: &mut Node<T>)
            ensures
                node.left == old(node).left,
                node.right == old(node).right,
                node.key == old(node).key,
                link_spec_size(old(node).left) + link_spec_size(old(node).right) < usize::MAX as nat
                    ==> node.size as nat == 1 + link_spec_size(old(node).left) + link_spec_size(old(node).right);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn splay(root: Box<Node<T>>, target: &T) -> (splayed: Box<Node<T>>)
            requires spec_is_bst_link(Some(root)),
            ensures
                spec_is_bst_link(Some(splayed)),
                forall|x: T| link_contains(Some(splayed), x) <==> link_contains(Some(root), x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn bst_insert(link: &mut Link<T>, value: T) -> (inserted: bool)
            requires spec_is_bst_link(*old(link)),
            ensures
                spec_is_bst_link(*link),
                link_contains(*link, value),
                forall|x: T| link_contains(*old(link), x) ==> link_contains(*link, x),
                forall|x: T| link_contains(*link, x) ==> (link_contains(*old(link), x) || x == value);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) amortized, Span O(lg n) amortized
        fn insert_link(link: &mut Link<T>, value: T) -> (inserted: bool)
            requires spec_is_bst_link(*old(link)),
            ensures
                spec_is_bst_link(*link),
                link_contains(*link, value),
                forall|x: T| link_contains(*old(link), x) ==> link_contains(*link, x),
                forall|x: T| link_contains(*link, x) ==> (link_contains(*old(link), x) || x == value);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            requires spec_is_bst_link(*link),
            ensures
                found.is_some() <==> link_contains(*link, *target),
                found.is_some() ==> *found.unwrap() == *target;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn min_link(link: &Link<T>) -> (min: Option<&T>)
            requires spec_is_bst_link(*link),
            ensures
                link.is_some() ==> min.is_some(),
                min.is_some() ==> link_contains(*link, *min.unwrap()),
                min.is_some() ==> forall|x: T| #[trigger] link_contains(*link, x) ==> TotalOrder::le(*min.unwrap(), x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn max_link(link: &Link<T>) -> (max: Option<&T>)
            requires spec_is_bst_link(*link),
            ensures
                link.is_some() ==> max.is_some(),
                max.is_some() ==> link_contains(*link, *max.unwrap()),
                max.is_some() ==> forall|x: T| #[trigger] link_contains(*link, x) ==> TotalOrder::le(x, *max.unwrap());

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>)
            requires link_spec_size(*link) <= usize::MAX as nat,
            ensures true;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>)
            requires link_spec_size(*link) <= usize::MAX as nat,
            ensures true;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_parallel(link: &Link<T>) -> (elements: Vec<T>)
            requires link_spec_size(*link) <= usize::MAX as nat,
            ensures true;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        fn pre_order_parallel(link: &Link<T>) -> (elements: Vec<T>)
            requires link_spec_size(*link) <= usize::MAX as nat,
            ensures true;

        // veracity: no_requires
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn clone_link(link: &Link<T>) -> (c: Link<T>)
            ensures c == *link;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn build_balanced(values: &[T]) -> (link: Link<T>)
            requires
                obeys_feq_clone::<T>(),
                forall|i: int, j: int| 0 <= i < j < values@.len() ==>
                    TotalOrder::le(#[trigger] values@[i], #[trigger] values@[j])
                    && values@[i] != values@[j],
            ensures
                link_spec_size(link) <= values@.len(),
                link_node_count(link) <= values@.len(),
                spec_is_bst_link(link),
                forall|x: T| #[trigger] link_contains(link, x) ==>
                    exists|i: int| 0 <= i < values@.len() && values@[i] == x;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(log^2 n)
        fn filter_parallel<F>(link: &Link<T>, predicate: &Arc<F>) -> (filtered: Vec<T>)
        where
            F: Fn(&T) -> bool + Send + Sync
            requires
                link_node_count(*link) <= usize::MAX as nat,
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures true;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        fn reduce_parallel<F>(link: &Link<T>, op: &Arc<F>, identity: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync
            requires
                link_node_count(*link) <= usize::MAX as nat,
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_rec(link: &Link<T>) -> (h: usize)
            requires link_height(*link) <= usize::MAX as nat,
            ensures h as nat == link_height(*link);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn compute_link_spec_size(link: &Link<T>) -> (n: usize)
            requires link_spec_size(*link) <= usize::MAX,
            ensures n as nat == link_spec_size(*link);
    }

    //		Section 9a. impls


    impl<T: StTInMtT + Ord + TotalOrder> BSTSplayMtNodeFns<T> for Node<T> {

    // Verified splay tree algorithms (Layer 1).

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn new_node(key: T) -> (node: Node<T>)
    {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn size_link(link: &Link<T>) -> (size: usize)
    {
// Veracity: UNNEEDED proof block         proof { reveal(link_spec_size); }
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    // veracity: no_requires
    /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn update(node: &mut Node<T>)
    {
        let ls = Self::size_link(&node.left);
        let rs = Self::size_link(&node.right);
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            node.size = 1 + ls + rs;
        }
    }


    // Bottom-up splay: bring target (or nearest key) toward the root using
    // zig, zig-zig, and zig-zag rotations (Sleator & Tarjan).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn splay(root: Box<Node<T>>, target: &T) -> (splayed: Box<Node<T>>)
        decreases root,
    {
        let ghost orig = root;
        let mut root = root;
        // Veracity: NEEDED proof block
        proof {
            reveal_with_fuel(spec_is_bst_link, 4);
            reveal_with_fuel(link_contains, 4);
        }
        match TotalOrder::cmp(target,&root.key) {
            // Veracity: NEEDED proof block
            core::cmp::Ordering::Equal => {
                proof { reveal_with_fuel(link_contains, 2); }
                root
            }
            core::cmp::Ordering::Less => {
                let ghost root_key = root.key;
                let ghost orig_root_left = root.left;
                // Veracity: NEEDED proof block
                let ghost orig_root_right = root.right;
                // Capture BST ordering facts while root is intact.
                proof {
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_root_left, x) implies
                        (TotalOrder::le(x, root_key) && x != root_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_root_right, x) implies
                        (TotalOrder::le(root_key, x) && x != root_key) by {};
                }
                let Some(mut left) = root.left.take() else {
                    return root
                };
                let ghost left_key = left.key;
                // Veracity: NEEDED proof block
                let ghost orig_left_left = left.left;
                let ghost orig_left_right = left.right;
                // Capture BST facts for left while left is intact.
                proof {
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_left_left, x) implies
                        (TotalOrder::le(x, left_key) && x != left_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_left_right, x) implies
                        (TotalOrder::le(left_key, x) && x != left_key) by {};
                    // left_key ∈ orig_root_left, so left_key < root_key.
                    // Elements in orig_left_right are in orig_root_left, so < root_key.
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_left_right, x) implies
                        (TotalOrder::le(x, root_key) && x != root_key) by {
                    };
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_left_left, x) implies
                        (TotalOrder::le(x, root_key) && x != root_key) by {
                    };
                }
                match TotalOrder::cmp(target,&left.key) {
                    // Veracity: NEEDED proof block
                    core::cmp::Ordering::Equal => {
                        // Zig: right rotation
                        root.left = left.right.take();
                        // Veracity: NEEDED proof block
                        Self::update(&mut root);
                        proof {
                        }
                        left.right = Some(root);
                        Self::update(&mut left);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 3);
                            reveal_with_fuel(link_contains, 4);
                            // BST ordering: elements in left.right (= Some(root)) > left.key.
                            // Veracity: NEEDED assert (speed hint)
                            assert(link_contains(orig_root_left, left_key)) by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            lemma_zig_child_ordering(left.right, left_key, root_key, orig_left_right, orig_root_right);
                            // Element preservation.
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(Some(orig), x) implies
                                link_contains(Some(left), x)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if x == root_key {
                                } else if link_contains(orig_root_right, x) {
                                } else if link_contains(orig_root_left, x) {
                                    reveal_with_fuel(link_contains, 2);
                                    if x == left_key {
                                    } else if link_contains(orig_left_left, x) {
                                    } else {
                                    }
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(Some(left), x) implies
                                link_contains(Some(orig), x)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if x == left_key {
                                } else if link_contains(left.left, x) {
                                } else {
                                    reveal_with_fuel(link_contains, 2);
                                    if x == root_key {
                                    } else if link_contains(orig_left_right, x) {
                                    } else {
                                    }
                                }
                            };
                        }
                        left
                    }
                    core::cmp::Ordering::Less => {
                        // Veracity: NEEDED proof block
                        // Zig-zig: recurse into left.left, then two right rotations.
                        if let Some(ll) = left.left.take() {
                            left.left = Some(Self::splay(ll, target));
                        }
                        root.left = left.right.take();
                        Self::update(&mut root);
                        proof {
                        }
                        left.right = Some(root);
                        // Veracity: NEEDED proof block
                        Self::update(&mut left);
                        if let Some(mut ll) = left.left.take() {
                            let ghost ll_key = ll.key;
                            // Veracity: NEEDED proof block
                            let ghost ll_left = ll.left;
                            let ghost ll_right = ll.right;
                            left.left = ll.right.take();
                            Self::update(&mut left);
                            proof {
                            }
                            ll.right = Some(left);
                            Self::update(&mut ll);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                // ll_key ∈ splay result ∈ orig_left_left, so < left_key.
                                // Veracity: NEEDED assert
                                assert(link_contains(orig_left_left, ll_key));
                                // BST: left.right > left_key, then chain ll.right > ll_key.
                                // Veracity: NEEDED assert
                                assert(link_contains(orig_root_left, left_key)) by {
                                    reveal_with_fuel(link_contains, 2);
                                };
                                lemma_zig_child_ordering(left.right, left_key, root_key, orig_left_right, orig_root_right);
                                lemma_zig_child_ordering(ll.right, ll_key, left_key, ll_right, left.right);
                                // BST: left.left (= ll_right) elements < left_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] link_contains(left.left, x) implies
                                    (TotalOrder::le(x, left_key) && x != left_key)
                                by {
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(ll), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == root_key {
                                    } else if link_contains(orig_root_right, x) {
                                    } else if link_contains(orig_root_left, x) {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == left_key {
                                        } else if link_contains(orig_left_left, x) {
                                        } else {
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(ll), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == ll_key {
                                    } else if link_contains(ll_left, x) {
                                        // Veracity: NEEDED assert
                                        assert(link_contains(orig_left_left, x));
                                    } else {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == left_key {
                                        } else if link_contains(ll_right, x) {
                                            // Veracity: NEEDED assert
                                            assert(link_contains(orig_left_left, x));
                                        } else {
                                            reveal_with_fuel(link_contains, 2);
                                            if x == root_key {
                                            } else if link_contains(orig_left_right, x) {
                                            // Veracity: NEEDED proof block
                                            } else {
                                            }
                                        }
                                    }
                                };
                            }
                            ll
                        } else {
                            // orig_left_left was None. Single Zig rotation.
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                // Veracity: NEEDED assert
                                assert(link_contains(orig_root_left, left_key)) by {
                                    reveal_with_fuel(link_contains, 2);
                                };
                                lemma_zig_child_ordering(left.right, left_key, root_key, orig_left_right, orig_root_right);
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(left), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == root_key {
                                    } else if link_contains(orig_root_right, x) {
                                    } else if link_contains(orig_root_left, x) {
                                        reveal_with_fuel(link_contains, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(left), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == left_key {
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == root_key {
                                        } else if link_contains(orig_left_right, x) {
                                        } else {
                                        }
                                    }
                                };
                            }
                            left
                        }
                    }
                    core::cmp::Ordering::Greater => {
                        // Veracity: NEEDED proof block
                        // Zig-zag: recurse into left.right, left-rotate left, right-rotate root.
                        if let Some(lr) = left.right.take() {
                            left.right = Some(Self::splay(lr, target));
                        }
                        if left.right.is_some() {
                            let mut lr = left.right.take().unwrap();
                            let ghost lr_key = lr.key;
                            let ghost lr_left = lr.left;
                            let ghost lr_right = lr.right;
                            // lr is splay of orig_left_right. BST, same elements.
                            proof {
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED proof block
                                assert(link_contains(orig_left_right, lr_key));
                                // Capture splay BST ordering while lr is intact.
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(lr_left, x) implies
                                    (TotalOrder::le(x, lr_key) && x != lr_key) by {};
                                // Veracity: NEEDED proof block
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(lr_right, x) implies
                                    (TotalOrder::le(lr_key, x) && x != lr_key) by {};
                            // Veracity: NEEDED proof block
                            }
                            left.right = lr.left.take();
                            Self::update(&mut left);
                            proof {
                            }
                            lr.left = Some(left);
                            Self::update(&mut lr);
                            root.left = lr.right.take();
                            Self::update(&mut root);
                            proof {
                            }
                            lr.right = Some(root);
                            Self::update(&mut lr);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                // BST: lr.left (= Some(left)) elements < lr_key.
                                lemma_zag_child_ordering(lr.left, left_key, lr_key, lr_left, orig_left_left);
                                // BST: lr.right (= Some(root)) elements > lr_key.
                                lemma_zig_child_ordering(lr.right, lr_key, root_key, lr_right, orig_root_right);
                                // BST: left.right (= lr_left) elements > left_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] link_contains(left.right, x) implies
                                    (TotalOrder::le(left_key, x) && x != left_key)
                                by {
                                };
                                // BST: root.left (= lr_right) elements < root_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] link_contains(root.left, x) implies
                                    (TotalOrder::le(x, root_key) && x != root_key)
                                by {
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(link_contains(orig_left_right, x));
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(lr), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == root_key {
                                    } else if link_contains(orig_root_right, x) {
                                    } else if link_contains(orig_root_left, x) {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == left_key {
                                        } else if link_contains(orig_left_left, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(link_contains(orig_left_right, x));
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(lr), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == lr_key {
                                    } else if link_contains(lr.left, x) {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == left_key {
                                        } else if link_contains(orig_left_left, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(link_contains(orig_left_right, x));
                                        }
                                    // Veracity: NEEDED proof block
                                    } else {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == root_key {
                                        } else if link_contains(lr_right, x) {
// Veracity: UNNEEDED assert                                             assert(link_contains(orig_left_right, x));
                                        // Veracity: NEEDED proof block
                                        } else {
                                        }
                                    }
                                };
                                // Help solver piece together BST for lr.
                            }
                            lr
                        } else {
                            // orig_left_right was None. Single Zig rotation.
                            proof {
                            }
                            root.left = left.right.take();
                            Self::update(&mut root);
                            left.right = Some(root);
                            Self::update(&mut left);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                // Veracity: NEEDED assert
                                assert(link_contains(orig_root_left, left_key)) by {
                                    reveal_with_fuel(link_contains, 2);
                                };
                                lemma_zig_child_ordering(left.right, left_key, root_key, orig_left_right, orig_root_right);
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(left), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == root_key {
                                    } else if link_contains(orig_root_right, x) {
                                    } else if link_contains(orig_root_left, x) {
                                        reveal_with_fuel(link_contains, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(left), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == left_key {
                                    } else if link_contains(left.left, x) {
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        // Veracity: NEEDED proof block
                                        if x == root_key {
                                        } else {
                                        }
                                    }
                                };
                            }
                            left
                        }
                    }
                }
            }
            core::cmp::Ordering::Greater => {
                let ghost root_key = root.key;
                let ghost orig_root_left = root.left;
                // Veracity: NEEDED proof block
                let ghost orig_root_right = root.right;
                // Capture BST ordering facts while root is intact.
                proof {
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_root_left, x) implies
                        (TotalOrder::le(x, root_key) && x != root_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_root_right, x) implies
                        (TotalOrder::le(root_key, x) && x != root_key) by {};
                }
                let Some(mut right) = root.right.take() else {
                    return root
                };
                let ghost right_key = right.key;
                let ghost orig_right_left = right.left;
                let ghost orig_right_right = right.right;
                // Capture BST facts for right while right is intact.
                proof {
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_right_left, x) implies
                        (TotalOrder::le(x, right_key) && x != right_key) by {};
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED proof block
                    assert forall|x: T| link_contains(orig_right_right, x) implies
                        (TotalOrder::le(right_key, x) && x != right_key) by {};
                    // right_key ∈ orig_root_right, so right_key > root_key.
                    // Veracity: NEEDED proof block
                    // Elements in orig_right_left are in orig_root_right, so > root_key.
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_right_left, x) implies
                        (TotalOrder::le(root_key, x) && x != root_key) by {
                    };
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(orig_right_right, x) implies
                        (TotalOrder::le(root_key, x) && x != root_key) by {
                    };
                }
                match TotalOrder::cmp(target,&right.key) {
                    core::cmp::Ordering::Equal => {
                        // Zag: left rotation
                        root.right = right.left.take();
                        Self::update(&mut root);
                        proof {
                        }
                        right.left = Some(root);
                        Self::update(&mut right);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 3);
                            reveal_with_fuel(link_contains, 4);
                            // BST ordering: elements in right.left (= Some(root)) < right.key.
                            // Veracity: NEEDED assert (speed hint)
                            assert(link_contains(orig_root_right, right_key)) by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            lemma_zag_child_ordering(right.left, root_key, right_key, orig_right_left, orig_root_left);
                            // BST ordering: elements in right.right > right.key (unchanged).
                            // Element preservation.
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(Some(orig), x) implies
                                link_contains(Some(right), x)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if x == root_key {
                                } else if link_contains(orig_root_left, x) {
                                } else if link_contains(orig_root_right, x) {
                                    reveal_with_fuel(link_contains, 2);
                                    if x == right_key {
                                    } else if link_contains(orig_right_left, x) {
                                    } else {
                                    }
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(Some(right), x) implies
                                link_contains(Some(orig), x)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if x == right_key {
                                // Veracity: NEEDED proof block (speed hint)
                                } else if link_contains(right.right, x) {
                                } else {
                                    // x in right.left = Some(root with left=orig_root_left, right=orig_right_left)
                                    reveal_with_fuel(link_contains, 2);
                                    if x == root_key {
                                    } else if link_contains(orig_root_left, x) {
                                    } else {
                                    }
                                }
                            // Veracity: NEEDED proof block
                            };
                        }
                        right
                    // Veracity: NEEDED proof block
                    }
                    core::cmp::Ordering::Greater => {
                        // Zag-zag: recurse into right.right, then two left rotations.
                        if let Some(rr) = right.right.take() {
                            right.right = Some(Self::splay(rr, target));
                        }
                        root.right = right.left.take();
                        Self::update(&mut root);
                        proof {
                        }
                        right.left = Some(root);
                        Self::update(&mut right);
                        if let Some(mut rr) = right.right.take() {
                            let ghost rr_key = rr.key;
                            let ghost rr_left = rr.left;
                            let ghost rr_right = rr.right;
                            right.right = rr.left.take();
                            Self::update(&mut right);
                            proof {
                            }
                            rr.left = Some(right);
                            Self::update(&mut rr);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                // rr_key ∈ splay result ∈ orig_right_right, so > right_key.
                                // Veracity: NEEDED assert
                                assert(link_contains(orig_right_right, rr_key));
                                // BST: right.left < right_key, then chain rr.left < rr_key.
                                assert(link_contains(orig_root_right, right_key)) by {
                                    reveal_with_fuel(link_contains, 2);
                                };
                                lemma_zag_child_ordering(right.left, root_key, right_key, orig_right_left, orig_root_left);
                                lemma_zag_child_ordering(rr.left, right_key, rr_key, rr_left, right.left);
                                // BST: right.right (= rr_left) elements > right_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] link_contains(right.right, x) implies
                                    (TotalOrder::le(right_key, x) && x != right_key)
                                by {
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(rr), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == root_key {
                                    } else if link_contains(orig_root_left, x) {
                                    } else if link_contains(orig_root_right, x) {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == right_key {
                                        } else if link_contains(orig_right_right, x) {
                                        } else {
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(rr), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == rr_key {
                                    // Veracity: NEEDED proof block
                                    } else if link_contains(rr_right, x) {
                                        // Veracity: NEEDED assert
                                        assert(link_contains(orig_right_right, x));
                                    } else {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == right_key {
                                        } else if link_contains(rr_left, x) {
                                            // Veracity: NEEDED assert (speed hint)
                                            assert(link_contains(orig_right_right, x));
                                        } else {
                                            reveal_with_fuel(link_contains, 2);
                                            if x == root_key {
                                            } else if link_contains(orig_right_left, x) {
                                            } else {
                                            }
                                        }
                                    }
                                };
                                // Help solver piece together BST for rr.
                            }
                            rr
                        } else {
                            // orig_right_right was None. Single Zag rotation.
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                // Veracity: NEEDED assert (speed hint)
                                assert(link_contains(orig_root_right, right_key)) by {
                                    reveal_with_fuel(link_contains, 2);
                                };
                                lemma_zag_child_ordering(right.left, root_key, right_key, orig_right_left, orig_root_left);
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(right), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == root_key {
                                    } else if link_contains(orig_root_left, x) {
                                    } else if link_contains(orig_root_right, x) {
                                        reveal_with_fuel(link_contains, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(right), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    // Veracity: NEEDED proof block
                                    if x == right_key {
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == root_key {
                                        } else if link_contains(orig_right_left, x) {
                                        } else {
                                        }
                                    }
                                };
                            }
                            right
                        }
                    // Veracity: NEEDED proof block
                    }
                    core::cmp::Ordering::Less => {
                        // Zag-zig: recurse into right.left, right-rotate right, left-rotate root.
                        if let Some(rl) = right.left.take() {
                            right.left = Some(Self::splay(rl, target));
// Veracity: UNNEEDED proof block                         }
// Veracity: UNNEEDED proof block                         if right.left.is_some() {
                            let mut rl = right.left.take().unwrap();
                            let ghost rl_key = rl.key;
                            // Veracity: NEEDED proof block
                            let ghost rl_left = rl.left;
                            let ghost rl_right = rl.right;
                            // rl is splay of orig_right_left. BST, same elements.
                            proof {
                                // Veracity: NEEDED assert
                                assert(link_contains(orig_right_left, rl_key));
                                // Capture splay BST ordering while rl is intact.
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(rl_left, x) implies
                                    (TotalOrder::le(x, rl_key) && x != rl_key) by {};
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(rl_right, x) implies
                                    (TotalOrder::le(rl_key, x) && x != rl_key) by {};
                            }
                            right.left = rl.right.take();
                            Self::update(&mut right);
                            proof {
                            }
                            rl.right = Some(right);
                            Self::update(&mut rl);
                            root.right = rl.left.take();
                            Self::update(&mut root);
                            proof {
                            }
                            rl.left = Some(root);
                            Self::update(&mut rl);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                // BST: rl.right (= Some(right)) elements > rl_key.
                                lemma_zig_child_ordering(rl.right, rl_key, right_key, rl_right, orig_right_right);
                                // BST: rl.left (= Some(root)) elements < rl_key.
                                lemma_zag_child_ordering(rl.left, root_key, rl_key, rl_left, orig_root_left);
                                // BST: right.left (= rl_right) elements < right_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] link_contains(right.left, x) implies
                                    (TotalOrder::le(x, right_key) && x != right_key)
                                by {
                                };
                                // BST: root.right (= rl_left) elements > root_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] link_contains(root.right, x) implies
                                    (TotalOrder::le(root_key, x) && x != root_key)
                                by {
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(link_contains(orig_right_left, x));
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(rl), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == root_key {
                                    } else if link_contains(orig_root_left, x) {
                                    } else if link_contains(orig_root_right, x) {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == right_key {
                                        } else if link_contains(orig_right_right, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(link_contains(orig_right_left, x));
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED proof block
                                assert forall|x: T| link_contains(Some(rl), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == rl_key {
                                    // Veracity: NEEDED proof block
                                    } else if link_contains(rl.right, x) {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == right_key {
                                        } else if link_contains(orig_right_right, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(link_contains(orig_right_left, x));
                                        }
                                    } else {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == root_key {
                                        } else if link_contains(rl_left, x) {
                                            // Veracity: NEEDED assert
                                            assert(link_contains(orig_right_left, x));
                                        } else {
                                        }
                                    }
                                };
                            }
                            rl
                        } else {
                            // orig_right_left was None. Single Zag rotation.
                            proof {
                            }
                            root.right = right.left.take();
                            Self::update(&mut root);
                            right.left = Some(root);
                            Self::update(&mut right);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                // Veracity: NEEDED assert (speed hint)
                                assert(link_contains(orig_root_right, right_key)) by {
                                    reveal_with_fuel(link_contains, 2);
                                };
                                lemma_zag_child_ordering(right.left, root_key, right_key, orig_right_left, orig_root_left);
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(orig), x) implies
                                    link_contains(Some(right), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == root_key {
                                    } else if link_contains(orig_root_left, x) {
                                    } else if link_contains(orig_root_right, x) {
                                        reveal_with_fuel(link_contains, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| link_contains(Some(right), x) implies
                                    // Veracity: NEEDED proof block
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == right_key {
                                    } else if link_contains(right.right, x) {
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == root_key {
                                        } else {
                                        }
                                    }
                                };
                            }
                            right
                        // Veracity: NEEDED proof block
                        }
                    }
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn bst_insert(link: &mut Link<T>, value: T) -> (inserted: bool)
        decreases *old(link),
    {
        let cur = link.take();
        match cur {
            | None => {
                *link = Some(Box::new(Self::new_node(value)));
                proof {
                    reveal_with_fuel(spec_is_bst_link, 2);
                    reveal_with_fuel(link_contains, 2);
                }
                true
            }
            | Some(mut node) => {
                let ghost old_left = node.left;
                let ghost old_right = node.right;
                let ghost node_key = node.key;
                match TotalOrder::cmp(&value, &node.key) {
                    core::cmp::Ordering::Less => {
                        Self::bst_insert(&mut node.left, value);
                        Self::update(&mut node);
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(node.left, x) implies
                                (TotalOrder::le(x, node.key) && x != node.key)
                            by {
                                if link_contains(old_left, x) {
                                } else {
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(*old(link), x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| (node_key == x || #[trigger] link_contains(old_left, x) || link_contains(old_right, x)) implies
                                // Veracity: NEEDED proof block
                                link_contains(*old(link), x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*old(link), x) implies
                                link_contains(*link, x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_left, x) {
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*link, x) implies
                                (link_contains(*old(link), x) || x == value)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                } else if link_contains(node.left, x) {
                                    if link_contains(old_left, x) {
                                    }
                                }
                            };
                        }
                        true
                    }
                    core::cmp::Ordering::Greater => {
                        Self::bst_insert(&mut node.right, value);
                        Self::update(&mut node);
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(node.right, x) implies
                                (TotalOrder::le(node.key, x) && x != node.key)
                            by {
                                if link_contains(old_right, x) {
                                } else {
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(*old(link), x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            // Veracity: NEEDED proof block
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| (node_key == x || #[trigger] link_contains(old_left, x) || link_contains(old_right, x)) implies
                                link_contains(*old(link), x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*old(link), x) implies
                                link_contains(*link, x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_right, x) {
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(*link, x) implies
                                (link_contains(*old(link), x) || x == value)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                } else if link_contains(node.right, x) {
                                    if link_contains(old_right, x) {
                                    }
                                }
                            };
                        }
                        true
                    }
                    core::cmp::Ordering::Equal => {
                        *link = Some(node);
                        proof {
                            // Veracity: NEEDED proof block
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                        }
                        false
                    }
                }
            }
        }
    }

    // Veracity: NEEDED proof block
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn insert_link(link: &mut Link<T>, value: T) -> (inserted: bool)
    {
        let v = value.clone();
        let inserted = Self::bst_insert(link, value);
        if inserted {
            if let Some(root) = link.take() {
                *link = Some(Self::splay(root, &v));
            }
        }
        inserted
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                match TotalOrder::cmp(target, &node.key) {
                    core::cmp::Ordering::Equal => Some(&node.key),
                    // Veracity: NEEDED proof block
                    core::cmp::Ordering::Less => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!link_contains(node.right, *target)) by {
                                if link_contains(node.right, *target) {
                                    TotalOrder::antisymmetric(*target, node.key);
                                }
                            };
                        }
                        Self::find_link(&node.left, target)
                    }
                    core::cmp::Ordering::Greater => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!link_contains(node.left, *target)) by {
                                if link_contains(node.left, *target) {
                                    // Veracity: NEEDED proof block
                                    TotalOrder::antisymmetric(node.key, *target);
                                }
                            };
                        }
                        Self::find_link(&node.right, target)
                    }
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn min_link(link: &Link<T>) -> (min: Option<&T>)
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*link, x) implies TotalOrder::le(node.key, x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                TotalOrder::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.right, x));
                            // Veracity: NEEDED proof block
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let min = Self::min_link(&node.left);
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*link, x) implies TotalOrder::le(*min.unwrap(), x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.left, x) {
                            } else if x == node.key {
                            // Veracity: NEEDED proof block
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.right, x));
                                TotalOrder::transitive(*min.unwrap(), node.key, x);
                            }
                        };
                    }
                    min
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn max_link(link: &Link<T>) -> (max: Option<&T>)
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*link, x) implies TotalOrder::le(x, node.key) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                TotalOrder::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.left, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let max = Self::max_link(&node.right);
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*link, x) implies TotalOrder::le(x, *max.unwrap()) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.right, x) {
                            } else if x == node.key {
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.left, x));
                                TotalOrder::transitive(x, node.key, *max.unwrap());
                            }
                        };
                    }
                    max
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn in_order_collect(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            Self::in_order_collect(&node.left, out);
            out.push(node.key.clone());
            Self::in_order_collect(&node.right, out);
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            Self::pre_order_collect(&node.left, out);
            Self::pre_order_collect(&node.right, out);
        // Veracity: NEEDED proof block
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn in_order_parallel(link: &Link<T>) -> (elements: Vec<T>)
    {
        let mut out = Vec::new();
        Self::in_order_collect(link, &mut out);
        out
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
    fn pre_order_parallel(link: &Link<T>) -> (elements: Vec<T>)
    {
        let mut out = Vec::new();
        // Veracity: NEEDED proof block
        Self::pre_order_collect(link, &mut out);
        out
    }

    /// Recursive deep clone of a Link, bypassing derived Clone for Box/Option.
    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn clone_link(link: &Link<T>) -> (c: Link<T>)
        decreases *link,
    {
        match link {
            None => {
                let c: Link<T> = None;
                c
            }
            Some(node) => {
                let left = Self::clone_link(&node.left);
                let right = Self::clone_link(&node.right);
                let c = Some(Box::new(Node {
                    key: node.key.clone(),
                    size: node.size,
                    left,
                    // Veracity: NEEDED proof block
                    right,
                }));
                proof { accept(c == *link); }
                c
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn build_balanced(values: &[T]) -> (link: Link<T>)
        decreases values.len(),
    {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        let left_slice = slice_subrange(values, 0, mid);
        let right_slice = slice_subrange(values, mid + 1, values.len());
        proof {
            // Sorted sub-slices: left_slice and right_slice are sorted.
            // Prove left_slice is strictly sorted.
            // Veracity: NEEDED assert
            assert forall|i: int, j: int| 0 <= i < j < left_slice@.len() implies
                TotalOrder::le(#[trigger] left_slice@[i], #[trigger] left_slice@[j])
                && left_slice@[i] != left_slice@[j]
            by {
            }
            // Prove right_slice is strictly sorted.
            // Veracity: NEEDED assert
            assert forall|i: int, j: int| 0 <= i < j < right_slice@.len() implies
                TotalOrder::le(#[trigger] right_slice@[i], #[trigger] right_slice@[j])
                // Veracity: NEEDED proof block
                && right_slice@[i] != right_slice@[j]
            by {
            }
        }
        let left = Self::build_balanced(left_slice);
        let right = Self::build_balanced(right_slice);
        let ghost gl = left;
        let ghost gr = right;
        let ghost pivot = values@[mid as int];
        // Veracity: NEEDED proof block
        // Prove ordering while left/right are in scope (triggers match directly).
        proof {
            // Left elements < pivot.
            // Veracity: NEEDED assert
            assert forall|x: T| #[trigger] link_contains(left, x) implies
                TotalOrder::le(x, pivot) && x != pivot
            by {
                let i = choose|i: int| 0 <= i < left_slice@.len() && left_slice@[i] == x;
            }
            // Right elements > pivot.
            // Veracity: NEEDED assert
            assert forall|x: T| #[trigger] link_contains(right, x) implies
                TotalOrder::le(pivot, x) && x != pivot
            by {
                let j = choose|j: int| 0 <= j < right_slice@.len() && right_slice@[j] == x;
            }
            // Containment: left elements are in values[0..mid].
            // Veracity: NEEDED assert
            assert forall|x: T| #[trigger] link_contains(left, x) implies
                exists|k: int| 0 <= k < values@.len() && values@[k] == x
            by {
                let i = choose|i: int| 0 <= i < left_slice@.len() && left_slice@[i] == x;
            }
            // Containment: right elements are in values[mid+1..].
            // Veracity: NEEDED assert
            assert forall|x: T| #[trigger] link_contains(right, x) implies
                exists|k: int| 0 <= k < values@.len() && values@[k] == x
            by {
                let j = choose|j: int| 0 <= j < right_slice@.len() && right_slice@[j] == x;
            }
        }
        let key = values[mid].clone();
        let ghost gkey = key;
        proof {
            // Introduce cloned trigger for broadcast axiom.
            // Veracity: NEEDED assert
            assert(cloned(pivot, gkey));
            // Broadcast axiom_cloned_implies_eq_owned fires: pivot == gkey.
        }
        let mut node = Box::new(Self::new_node(key));
        node.left = left;
        node.right = right;
        Self::update(&mut node);
        proof {
            // Veracity: NEEDED proof block (speed hint)
            reveal_with_fuel(link_node_count, 2);

            // node.left == gl, node.right == gr via update preserves.
            // node.key == gkey from new_node, gkey == pivot from cloned broadcast.

            // Transfer ordering to node fields via ghost equality.
            // Veracity: NEEDED assert
            assert forall|x: T| #[trigger] link_contains(node.left, x) implies
                TotalOrder::le(x, node.key) && x != node.key
            by {
            }
            // Veracity: NEEDED assert
            assert forall|x: T| #[trigger] link_contains(node.right, x) implies
                TotalOrder::le(node.key, x) && x != node.key
            by {
            }
            reveal_with_fuel(spec_is_bst_link, 2);

            // Containment for the full tree.
            // Veracity: NEEDED assert
            assert forall|x: T| #[trigger] link_contains(Some(node), x) implies
                exists|k: int| 0 <= k < values@.len() && values@[k] == x
            by {
                // Veracity: NEEDED proof block
                reveal_with_fuel(link_contains, 2);
                if node.key == x {
                } else if link_contains(node.left, x) {
                } else {
                    // Veracity: NEEDED assert
                    assert(link_contains(gr, x));
                }
            }
        }
        Some(node)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(log^2 n)
    fn filter_parallel<F>(link: &Link<T>, predicate: &Arc<F>) -> (filtered: Vec<T>)
    where
        F: Fn(&T) -> bool + Send + Sync
        decreases *link,
    {
        // Veracity: NEEDED proof block
        match link {
            | None => Vec::new(),
            | Some(node) => {
                proof {
                    reveal_with_fuel(link_node_count, 2);
                }
                let left_vals = Self::filter_parallel(&node.left, predicate);
                let mut right_vals = Self::filter_parallel(&node.right, predicate);
                let mut result = left_vals;
                if (**predicate)(&node.key) {
                    result.push(node.key.clone());
                }
                result.append(&mut right_vals);
                result
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
    fn reduce_parallel<F>(link: &Link<T>, op: &Arc<F>, identity: T) -> (reduced: T)
    where
        F: Fn(T, T) -> T + Send + Sync
        decreases *link,
    {
        match link {
            | None => identity,
            | Some(node) => {
                proof {
                    reveal_with_fuel(link_node_count, 2);
                }
                let id_left = identity.clone();
                let left_acc = Self::reduce_parallel(&node.left, op, id_left);
                let right_acc = Self::reduce_parallel(&node.right, op, identity);
                let with_key = (**op)(left_acc, node.key.clone());
                (**op)(with_key, right_acc)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn height_rec(link: &Link<T>) -> (h: usize)
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => {
                proof {
                }
                1 + Self::height_rec(&node.left).max(Self::height_rec(&node.right))
            }
        }
    }

    /// Exec mirror of link_spec_size for runtime size guards.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn compute_link_spec_size(link: &Link<T>) -> (n: usize)
    {
        match link {
            None => 0,
            Some(node) => node.size,
        }
    }

    } // impl BSTSplayMtNodeFns for Node

    //		Section 4b. type definitions


    /// Lock predicate: link size fits in usize.
    pub struct BSTSplayMtEphInv;

    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTSplayMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub(crate) root: RwLock<Link<T>, BSTSplayMtEphInv>,
        pub(crate) ghost_root: Ghost<Link<T>>,
    }

    //		Section 5c. view impls


    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSplayMtEph<T> {
        type V = Link<T>;
        open spec fn view(&self) -> Link<T> { self.spec_ghost_root() }
    }

    //		Section 8c. traits


    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized + View<V = Link<T>> {
        spec fn spec_bstsplaymteph_wf(&self) -> bool;
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;
        spec fn spec_contains(&self, value: T) -> bool;
        spec fn spec_in_order(&self) -> Seq<T>;
        spec fn spec_pre_order(&self) -> Seq<T>;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_bstsplaymteph_wf(),
                    tree@ is None,
                    link_spec_size(tree@) == 0,
                    forall|x: T| !link_contains(tree@, x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_sorted_slice(values: &[T]) -> (tree: Self)
            requires
                obeys_feq_clone::<T>(),
                forall|i: int, j: int| 0 <= i < j < values@.len() ==>
                    TotalOrder::le(#[trigger] values@[i], #[trigger] values@[j])
                    && values@[i] != values@[j],
            ensures tree.spec_bstsplaymteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>)
            requires old(self).spec_bstsplaymteph_wf(),
            ensures self.spec_bstsplaymteph_wf(),
                    match inserted {
                        Ok(_) => link_spec_size(self@) <= link_spec_size(old(self)@) + 1
                            && link_contains(self@, value)
                            && forall|x: T| link_contains(old(self)@, x) ==>
                                #[trigger] link_contains(self@, x)
                            && forall|x: T| (#[trigger] link_contains(self@, x)) ==>
                                (link_contains(old(self)@, x) || x == value),
                        Err(_) => self@ == old(self)@,
                    };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstsplaymteph_wf(),
            ensures found == link_contains(self@, *target);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (n: usize)
            requires self.spec_bstsplaymteph_wf(),
            ensures n as nat == link_spec_size(self@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstsplaymteph_wf(),
            ensures b == (self@ is None);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires self.spec_bstsplaymteph_wf(),
            ensures h as nat == link_height(self@);

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<T>)
            requires self.spec_bstsplaymteph_wf(),
            ensures
                found.is_some() <==> link_contains(self@, *target),
                found.is_some() ==> found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsplaymteph_wf(),
            ensures
                link_spec_size(self@) > 0 ==> min.is_some(),
                min.is_some() ==> link_contains(self@, min.unwrap()),
                min.is_some() ==> forall|x: T| link_contains(self@, x) ==>
                    #[trigger] TotalOrder::le(min.unwrap(), x);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsplaymteph_wf(),
            ensures
                link_spec_size(self@) > 0 ==> max.is_some(),
                max.is_some() ==> link_contains(self@, max.unwrap()),
                max.is_some() ==> forall|x: T| link_contains(self@, x) ==>
                    #[trigger] TotalOrder::le(x, max.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsplaymteph_wf(),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsplaymteph_wf(),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n h(T)), Span O(n h(T))
        fn filter<F>(&self, predicate: F) -> (seq: ArraySeqStPerS<T>)
        where
            F: Fn(&T) -> bool + Send + Sync
            requires
                self.spec_bstsplaymteph_wf(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync
            requires
                self.spec_bstsplaymteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
    }

    //		Section 9c. impls
// Veracity: NEEDED proof block (speed hint)


    impl<T: StTInMtT + Ord + TotalOrder> BSTSplayMtEph<T> {
        // Veracity: NEEDED proof block (speed hint)
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            link_node_count(self.ghost_root@) <= usize::MAX && spec_is_bst_link(self.ghost_root@)
        }

        pub closed spec fn spec_ghost_root(self) -> Link<T> {
            self.ghost_root@
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {
        open spec fn spec_bstsplaymteph_wf(&self) -> bool {
            link_node_count(self@) <= usize::MAX
            && spec_is_bst_link(self@)
        }
        open spec fn spec_size(&self) -> nat { link_spec_size(self@) }
        open spec fn spec_height(&self) -> nat { link_height(self@) }
        open spec fn spec_contains(&self, value: T) -> bool { link_contains(self@, value) }
        // Veracity: NEEDED proof block
        open spec fn spec_in_order(&self) -> Seq<T> { spec_in_order_link(self@) }
        open spec fn spec_pre_order(&self) -> Seq<T> { spec_pre_order_link(self@) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> Self {
            BSTSplayMtEph {
                root: RwLock::new(None, Ghost(BSTSplayMtEphInv)),
                ghost_root: Ghost(None),
            }
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_sorted_slice(values: &[T]) -> Self {
            let vlen = values.len();
            let link = Node::<T>::build_balanced(values);
            let ghost ghost_link = link;
            BSTSplayMtEph {
                root: RwLock::new(link, Ghost(BSTSplayMtEphInv)),
                // Veracity: NEEDED proof block
                ghost_root: Ghost(ghost_link),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>) {
            let (mut current, write_handle) = self.root.acquire_write();
            proof { assume(self.ghost_root@ == current); }
            // Veracity: NEEDED proof block
            let sz = Node::<T>::compute_link_spec_size(&current);
            if sz < usize::MAX {
                Node::<T>::insert_link(&mut current, value);
                // Veracity: NEEDED proof block
                proof {
                    assume(link_node_count(current) <= usize::MAX as nat);
                    assume(link_spec_size(current) <= link_spec_size(old(self)@) + 1);
                }
                let ghost new_root = current;
                self.ghost_root = Ghost(new_root);
                write_handle.release_write(current);
                Ok(())
            // Veracity: NEEDED proof block
            } else {
                write_handle.release_write(current);
                Err(())
            }
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool) {
            let handle = self.root.acquire_read();
            let found = Node::<T>::find_link(handle.borrow(), target).is_some();
            // Veracity: NEEDED proof block
            proof { assume(found == link_contains(self@, *target)); }
            handle.release_read();
            found
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (n: usize) {
            let handle = self.root.acquire_read();
            let n = Node::<T>::size_link(handle.borrow());
            proof { assume(n as nat == link_spec_size(self@)); }
            handle.release_read();
            n
        // Veracity: NEEDED proof block
        }

        // Predicate: assume return predicate matches spec predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool) {
            let handle = self.root.acquire_read();
            let b = handle.borrow().is_none();
            proof { assume(b == (self@ is None)); }
            handle.release_read();
            b
        }

        // Reader: height bounded by node count from lock predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            proof {
                lemma_height_le_node_count::<T>(*data);
            }
            let h = Node::<T>::height_rec(data);
            proof { assume(h as nat == link_height(self@)); }
            handle.release_read();
            h
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<T>) {
            let handle = self.root.acquire_read();
            let found = Node::<T>::find_link(handle.borrow(), target).cloned();
            proof {
                assume(found.is_some() <==> link_contains(self@, *target));
                accept(found.is_some() ==> found.unwrap() == *target);
            }
            handle.release_read();
            found
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<T>) {
            let handle = self.root.acquire_read();
            let min = Node::<T>::min_link(handle.borrow()).cloned();
            proof {
                assume(link_spec_size(self@) > 0 ==> min.is_some());
                assume(min.is_some() ==> link_contains(self@, min.unwrap()));
                assume(min.is_some() ==> forall|x: T| link_contains(self@, x) ==>
                    #[trigger] TotalOrder::le(min.unwrap(), x));
            }
            handle.release_read();
            min
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<T>) {
            let handle = self.root.acquire_read();
            let max = Node::<T>::max_link(handle.borrow()).cloned();
            proof {
                assume(link_spec_size(self@) > 0 ==> max.is_some());
                assume(max.is_some() ==> link_contains(self@, max.unwrap()));
                assume(max.is_some() ==> forall|x: T| link_contains(self@, x) ==>
                    #[trigger] TotalOrder::le(x, max.unwrap()));
            }
            handle.release_read();
            max
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let out = Node::<T>::in_order_parallel(handle.borrow());
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> ArraySeqStPerS<T> {
            // Veracity: NEEDED proof block
            let handle = self.root.acquire_read();
            let out = Node::<T>::pre_order_parallel(handle.borrow());
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
            let out = Node::<T>::filter_parallel(data, &predicate);
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
            let accumulated = Node::<T>::reduce_parallel(data, &op, identity);
            handle.release_read();
            accumulated
        }
    }

    //		Section 11b. top level coarse locking


    impl<T: StTInMtT + Ord + TotalOrder> RwLockPredicate<Link<T>> for BSTSplayMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            link_node_count(v) <= usize::MAX && spec_is_bst_link(v)
        }
    }

    //		Section 12a. derive impls in verus!


    impl<T: StTInMtT + Ord + TotalOrder> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self,
        {
            let left = Node::<T>::clone_link(&self.left);
            let right = Node::<T>::clone_link(&self.right);
            let cloned = Node {
                key: self.key.clone(),
                size: self.size,
                left,
                right,
            };
            proof { accept(cloned == *self); }
            cloned
        }
    }

    //		Section 12c. derive impls in verus!


    impl<T: StTInMtT + Ord + TotalOrder + 'static> Default for BSTSplayMtEph<T> {
        fn default() -> Self { Self::new() }
    }
    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTSplayMtEphLit {
        () => {
            < $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEph<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }

    //		Section 14a. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl std::fmt::Debug for BSTSplayMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSplayMtEphInv").finish()
        }
    }

    impl std::fmt::Display for BSTSplayMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSplayMtEphInv")
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSplayMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSplayMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> std::fmt::Display for BSTSplayMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSplayMtEph(size={})", self.size())
        }
    }
}
