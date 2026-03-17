//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral splay-style binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on Link/Node) in sections 6/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//  1. module
//  2. imports
//  4. type definitions
//  6. spec fns
//  9. impls
//  11. top level coarse locking
//  13. macros
//  14. derive impls outside verus!

// 1. module

pub mod BSTSplayMtEph {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus! {

    // 2. imports

    use crate::vstdplus::accept::accept;

    // (Arc kept for filter_parallel/reduce_parallel closure sharing.)

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    #[derive(Clone)]
    struct Node<T: StTInMtT + Ord + TotalOrder> {
        key: T,
        size: N,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    // 6. spec fns

    /// Structural node count for splay tree links.
    pub open spec fn link_spec_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + link_spec_size(node.left) + link_spec_size(node.right),
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
                    T::le(x, node.key) && x != node.key)
                && (forall|x: T| (#[trigger] link_contains(node.right, x)) ==>
                    T::le(node.key, x) && x != node.key)
            }
        }
    }

    // 9. impls

    // Verified splay tree algorithms (Layer 1).

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
            size: 1,
            left: None,
            right: None,
        }
    }

    fn size_link<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (size: N)
        requires link_spec_size(*link) <= usize::MAX as nat,
        ensures
            (link is None) ==> size == 0,
    {
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    fn update<T: StTInMtT + Ord + TotalOrder>(node: &mut Node<T>)
        requires 1 + link_spec_size(old(node).left) + link_spec_size(old(node).right) <= usize::MAX as nat,
        ensures
            node.left == old(node).left,
            node.right == old(node).right,
            node.key == old(node).key,
    {
        let ls = size_link(&node.left);
        let rs = size_link(&node.right);
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            node.size = 1 + ls + rs;
        }
    }


    // Bottom-up splay: bring target (or nearest key) toward the root using
    // zig, zig-zig, and zig-zag rotations (Sleator & Tarjan).
    fn splay<T: StTInMtT + Ord + TotalOrder>(root: Box<Node<T>>, target: &T) -> (result: Box<Node<T>>)
        requires spec_is_bst_link(Some(root)),
        ensures
            spec_is_bst_link(Some(result)),
            forall|x: T| link_contains(Some(result), x) <==> link_contains(Some(root), x),
        decreases root,
    {
        let ghost orig = root;
        let mut root = root;
        proof {
            reveal_with_fuel(spec_is_bst_link, 4);
            reveal_with_fuel(link_contains, 4);
        }
        match TotalOrder::cmp(target,&root.key) {
            core::cmp::Ordering::Equal => {
                proof { reveal_with_fuel(link_contains, 2); }
                root
            }
            core::cmp::Ordering::Less => {
                let ghost root_key = root.key;
                let ghost orig_root_left = root.left;
                let ghost orig_root_right = root.right;
                // Capture BST ordering facts while root is intact.
                proof {
                    assert forall|x: T| link_contains(orig_root_left, x) implies
                        (T::le(x, root_key) && x != root_key) by {};
                    assert forall|x: T| link_contains(orig_root_right, x) implies
                        (T::le(root_key, x) && x != root_key) by {};
                }
                let Some(mut left) = root.left.take() else {
                    return root
                };
                let ghost left_key = left.key;
                let ghost orig_left_left = left.left;
                let ghost orig_left_right = left.right;
                // Capture BST facts for left while left is intact.
                proof {
                    assert forall|x: T| link_contains(orig_left_left, x) implies
                        (T::le(x, left_key) && x != left_key) by {};
                    assert forall|x: T| link_contains(orig_left_right, x) implies
                        (T::le(left_key, x) && x != left_key) by {};
                    // left_key ∈ orig_root_left, so left_key < root_key.
                    assert(link_contains(orig_root_left, left_key));
                    // Elements in orig_left_right are in orig_root_left, so < root_key.
                    assert forall|x: T| link_contains(orig_left_right, x) implies
                        (T::le(x, root_key) && x != root_key) by {
                        assert(link_contains(orig_root_left, x));
                    };
                    assert forall|x: T| link_contains(orig_left_left, x) implies
                        (T::le(x, root_key) && x != root_key) by {
                        assert(link_contains(orig_root_left, x));
                    };
                }
                match TotalOrder::cmp(target,&left.key) {
                    core::cmp::Ordering::Equal => {
                        // Zig: right rotation
                        root.left = left.right.take();
                        update(&mut root);
                        proof {
                            assert(root.key == root_key);
                            assert(root.left == orig_left_right);
                            assert(root.right == orig_root_right);
                        }
                        left.right = Some(root);
                        update(&mut left);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 3);
                            reveal_with_fuel(link_contains, 4);
                            assert(left.key == left_key);
                            // BST ordering: elements in left.right (= Some(root)) > left.key.
                            assert forall|x: T| link_contains(left.right, x) implies
                                (T::le(left_key, x) && x != left_key)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if link_contains(orig_left_right, x) {
                                } else if link_contains(orig_root_right, x) {
                                    T::transitive(left_key, root_key, x);
                                    if x == left_key { T::antisymmetric(left_key, root_key); }
                                } else if x == root_key {
                                }
                            };
                            // Element preservation.
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
                                        assert(link_contains(left.left, x));
                                    } else {
                                        assert(link_contains(orig_left_right, x));
                                    }
                                }
                            };
                            assert forall|x: T| link_contains(Some(left), x) implies
                                link_contains(Some(orig), x)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if x == left_key {
                                    assert(link_contains(orig_root_left, left_key));
                                } else if link_contains(left.left, x) {
                                    assert(link_contains(orig_left_left, x));
                                    assert(link_contains(orig_root_left, x));
                                } else {
                                    assert(link_contains(left.right, x));
                                    reveal_with_fuel(link_contains, 2);
                                    if x == root_key {
                                    } else if link_contains(orig_left_right, x) {
                                        assert(link_contains(orig_root_left, x));
                                    } else {
                                        assert(link_contains(orig_root_right, x));
                                    }
                                }
                            };
                        }
                        left
                    }
                    core::cmp::Ordering::Less => {
                        // Zig-zig: recurse into left.left, then two right rotations.
                        if let Some(ll) = left.left.take() {
                            left.left = Some(splay(ll, target));
                        }
                        root.left = left.right.take();
                        update(&mut root);
                        proof {
                            assert(root.key == root_key);
                            assert(root.left == orig_left_right);
                            assert(root.right == orig_root_right);
                        }
                        left.right = Some(root);
                        update(&mut left);
                        if let Some(mut ll) = left.left.take() {
                            let ghost ll_key = ll.key;
                            let ghost ll_left = ll.left;
                            let ghost ll_right = ll.right;
                            left.left = ll.right.take();
                            update(&mut left);
                            proof {
                                assert(left.key == left_key);
                                assert(left.left == ll_right);
                            }
                            ll.right = Some(left);
                            update(&mut ll);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                assert(ll.key == ll_key);
                                assert(ll.left == ll_left);
                                // ll_key ∈ splay result ∈ orig_left_left, so < left_key.
                                assert(link_contains(orig_left_left, ll_key));
                                // BST: ll.right elements > ll_key.
                                assert forall|x: T| link_contains(ll.right, x) implies
                                    (T::le(ll_key, x) && x != ll_key)
                                by {
                                    reveal_with_fuel(link_contains, 4);
                                    if x == left_key {
                                    } else if link_contains(ll_right, x) {
                                    } else if link_contains(left.right, x) {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == root_key {
                                            T::transitive(ll_key, left_key, root_key);
                                            if x == ll_key { T::antisymmetric(ll_key, root_key); }
                                        } else if link_contains(orig_left_right, x) {
                                            T::transitive(ll_key, left_key, x);
                                            if x == ll_key { T::antisymmetric(ll_key, left_key); }
                                        } else {
                                            assert(link_contains(orig_root_right, x));
                                            T::transitive(ll_key, root_key, x);
                                            if x == ll_key { T::antisymmetric(ll_key, root_key); }
                                        }
                                    }
                                };
                                // BST: left.left (= ll_right) elements < left_key.
                                assert forall|x: T| link_contains(left.left, x) implies
                                    (T::le(x, left_key) && x != left_key)
                                by {
                                    assert(link_contains(orig_left_left, x));
                                };
                                // BST: left.right elements > left_key.
                                assert forall|x: T| link_contains(left.right, x) implies
                                    (T::le(left_key, x) && x != left_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if link_contains(orig_left_right, x) {
                                    } else if link_contains(orig_root_right, x) {
                                        T::transitive(left_key, root_key, x);
                                        if x == left_key { T::antisymmetric(left_key, root_key); }
                                    } else if x == root_key {
                                    }
                                };
                                // Element preservation.
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
                                            assert(link_contains(orig_left_right, x));
                                        }
                                    }
                                };
                                assert forall|x: T| link_contains(Some(ll), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == ll_key {
                                        assert(link_contains(orig_left_left, ll_key));
                                        assert(link_contains(orig_root_left, ll_key));
                                    } else if link_contains(ll_left, x) {
                                        assert(link_contains(orig_left_left, x));
                                        assert(link_contains(orig_root_left, x));
                                    } else {
                                        assert(link_contains(ll.right, x));
                                        reveal_with_fuel(link_contains, 3);
                                        if x == left_key {
                                            assert(link_contains(orig_root_left, left_key));
                                        } else if link_contains(ll_right, x) {
                                            assert(link_contains(orig_left_left, x));
                                            assert(link_contains(orig_root_left, x));
                                        } else {
                                            reveal_with_fuel(link_contains, 2);
                                            if x == root_key {
                                            } else if link_contains(orig_left_right, x) {
                                                assert(link_contains(orig_root_left, x));
                                            } else {
                                                assert(link_contains(orig_root_right, x));
                                            }
                                        }
                                    }
                                };
                                assert(spec_is_bst_link(Some(ll))) by {
                                    reveal_with_fuel(spec_is_bst_link, 4);
                                };
                            }
                            ll
                        } else {
                            // orig_left_left was None. Single Zig rotation.
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                assert(left.key == left_key);
                                assert forall|x: T| link_contains(left.right, x) implies
                                    (T::le(left_key, x) && x != left_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if link_contains(orig_left_right, x) {
                                    } else if link_contains(orig_root_right, x) {
                                        T::transitive(left_key, root_key, x);
                                        if x == left_key { T::antisymmetric(left_key, root_key); }
                                    } else if x == root_key {
                                    }
                                };
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
                                assert forall|x: T| link_contains(Some(left), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == left_key {
                                        assert(link_contains(orig_root_left, left_key));
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == root_key {
                                        } else if link_contains(orig_left_right, x) {
                                            assert(link_contains(orig_root_left, x));
                                        } else {
                                            assert(link_contains(orig_root_right, x));
                                        }
                                    }
                                };
                                assert(spec_is_bst_link(Some(left))) by {
                                    reveal_with_fuel(spec_is_bst_link, 4);
                                };
                            }
                            left
                        }
                    }
                    core::cmp::Ordering::Greater => {
                        // Zig-zag: recurse into left.right, left-rotate left, right-rotate root.
                        if let Some(lr) = left.right.take() {
                            left.right = Some(splay(lr, target));
                        }
                        if left.right.is_some() {
                            let mut lr = left.right.take().unwrap();
                            let ghost lr_key = lr.key;
                            let ghost lr_left = lr.left;
                            let ghost lr_right = lr.right;
                            // lr is splay of orig_left_right. BST, same elements.
                            proof {
                                assert(link_contains(orig_left_right, lr_key));
                                assert(link_contains(orig_root_left, lr_key));
                                // Capture splay BST ordering while lr is intact.
                                assert forall|x: T| link_contains(lr_left, x) implies
                                    (T::le(x, lr_key) && x != lr_key) by {};
                                assert forall|x: T| link_contains(lr_right, x) implies
                                    (T::le(lr_key, x) && x != lr_key) by {};
                            }
                            left.right = lr.left.take();
                            update(&mut left);
                            proof {
                                assert(left.key == left_key);
                                assert(left.left == orig_left_left);
                                assert(left.right == lr_left);
                            }
                            lr.left = Some(left);
                            update(&mut lr);
                            root.left = lr.right.take();
                            update(&mut root);
                            proof {
                                assert(root.key == root_key);
                                assert(root.left == lr_right);
                                assert(root.right == orig_root_right);
                            }
                            lr.right = Some(root);
                            update(&mut lr);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                assert(lr.key == lr_key);
                                // BST: lr.left (= Some(left)) elements < lr_key.
                                assert forall|x: T| link_contains(lr.left, x) implies
                                    (T::le(x, lr_key) && x != lr_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == left_key {
                                    } else if link_contains(orig_left_left, x) {
                                        T::transitive(x, left_key, lr_key);
                                        if x == lr_key { T::antisymmetric(left_key, lr_key); }
                                    } else {
                                        // x ∈ lr_left ⊂ orig_left_right > left_key, < lr_key from splay BST.
                                    }
                                };
                                // BST: lr.right (= Some(root)) elements > lr_key.
                                assert forall|x: T| link_contains(lr.right, x) implies
                                    (T::le(lr_key, x) && x != lr_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == root_key {
                                    } else if link_contains(lr_right, x) {
                                        // lr_right > lr_key from splay BST.
                                    } else {
                                        assert(link_contains(orig_root_right, x));
                                        T::transitive(lr_key, root_key, x);
                                        if x == lr_key { T::antisymmetric(lr_key, root_key); }
                                    }
                                };
                                // BST: left.right (= lr_left) elements > left_key.
                                assert forall|x: T| link_contains(left.right, x) implies
                                    (T::le(left_key, x) && x != left_key)
                                by {
                                    assert(link_contains(orig_left_right, x));
                                };
                                // BST: root.left (= lr_right) elements < root_key.
                                assert forall|x: T| link_contains(root.left, x) implies
                                    (T::le(x, root_key) && x != root_key)
                                by {
                                    assert(link_contains(orig_left_right, x));
                                    assert(link_contains(orig_root_left, x));
                                };
                                // Element preservation.
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
                                            assert(link_contains(orig_left_right, x));
                                        }
                                    }
                                };
                                assert forall|x: T| link_contains(Some(lr), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == lr_key {
                                        assert(link_contains(orig_root_left, lr_key));
                                    } else if link_contains(lr.left, x) {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == left_key {
                                            assert(link_contains(orig_root_left, left_key));
                                        } else if link_contains(orig_left_left, x) {
                                            assert(link_contains(orig_root_left, x));
                                        } else {
                                            assert(link_contains(orig_left_right, x));
                                            assert(link_contains(orig_root_left, x));
                                        }
                                    } else {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == root_key {
                                        } else if link_contains(lr_right, x) {
                                            assert(link_contains(orig_left_right, x));
                                            assert(link_contains(orig_root_left, x));
                                        } else {
                                            assert(link_contains(orig_root_right, x));
                                        }
                                    }
                                };
                                // Help solver piece together BST for lr.
                                assert(spec_is_bst_link(lr_left));
                                assert(spec_is_bst_link(lr_right));
                                assert(spec_is_bst_link(orig_left_left));
                                assert(spec_is_bst_link(orig_root_right));
                                assert(spec_is_bst_link(Some(left))) by {
                                    reveal_with_fuel(spec_is_bst_link, 2);
                                };
                                assert(spec_is_bst_link(Some(root))) by {
                                    reveal_with_fuel(spec_is_bst_link, 2);
                                };
                                assert(spec_is_bst_link(Some(lr))) by {
                                    reveal_with_fuel(spec_is_bst_link, 2);
                                };
                            }
                            lr
                        } else {
                            // orig_left_right was None. Single Zig rotation.
                            proof {
                                assert(root.key == root_key);
                                assert(root.right == orig_root_right);
                            }
                            root.left = left.right.take();
                            update(&mut root);
                            left.right = Some(root);
                            update(&mut left);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                assert(left.key == left_key);
                                assert forall|x: T| link_contains(left.right, x) implies
                                    (T::le(left_key, x) && x != left_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if link_contains(orig_root_right, x) {
                                        T::transitive(left_key, root_key, x);
                                        if x == left_key { T::antisymmetric(left_key, root_key); }
                                    } else if x == root_key {
                                    }
                                };
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
                                assert forall|x: T| link_contains(Some(left), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == left_key {
                                        assert(link_contains(orig_root_left, left_key));
                                    } else if link_contains(left.left, x) {
                                        assert(link_contains(orig_left_left, x));
                                        assert(link_contains(orig_root_left, x));
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == root_key {
                                        } else {
                                            assert(link_contains(orig_root_right, x));
                                        }
                                    }
                                };
                                assert(spec_is_bst_link(Some(left))) by {
                                    reveal_with_fuel(spec_is_bst_link, 4);
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
                let ghost orig_root_right = root.right;
                // Capture BST ordering facts while root is intact.
                proof {
                    assert forall|x: T| link_contains(orig_root_left, x) implies
                        (T::le(x, root_key) && x != root_key) by {};
                    assert forall|x: T| link_contains(orig_root_right, x) implies
                        (T::le(root_key, x) && x != root_key) by {};
                }
                let Some(mut right) = root.right.take() else {
                    return root
                };
                let ghost right_key = right.key;
                let ghost orig_right_left = right.left;
                let ghost orig_right_right = right.right;
                // Capture BST facts for right while right is intact.
                proof {
                    assert forall|x: T| link_contains(orig_right_left, x) implies
                        (T::le(x, right_key) && x != right_key) by {};
                    assert forall|x: T| link_contains(orig_right_right, x) implies
                        (T::le(right_key, x) && x != right_key) by {};
                    // right_key ∈ orig_root_right, so right_key > root_key.
                    assert(link_contains(orig_root_right, right_key));
                    // Elements in orig_right_left are in orig_root_right, so > root_key.
                    assert forall|x: T| link_contains(orig_right_left, x) implies
                        (T::le(root_key, x) && x != root_key) by {
                        assert(link_contains(orig_root_right, x));
                    };
                    assert forall|x: T| link_contains(orig_right_right, x) implies
                        (T::le(root_key, x) && x != root_key) by {
                        assert(link_contains(orig_root_right, x));
                    };
                }
                match TotalOrder::cmp(target,&right.key) {
                    core::cmp::Ordering::Equal => {
                        // Zag: left rotation
                        root.right = right.left.take();
                        update(&mut root);
                        proof {
                            assert(root.key == root_key);
                            assert(root.left == orig_root_left);
                            assert(root.right == orig_right_left);
                        }
                        right.left = Some(root);
                        update(&mut right);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 3);
                            reveal_with_fuel(link_contains, 4);
                            assert(right.key == right_key);
                            // BST ordering: elements in right.left (= Some(root)) < right.key.
                            assert forall|x: T| link_contains(right.left, x) implies
                                (T::le(x, right_key) && x != right_key)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if link_contains(orig_right_left, x) {
                                } else if link_contains(orig_root_left, x) {
                                    T::transitive(x, root_key, right_key);
                                    if x == right_key { T::antisymmetric(root_key, right_key); }
                                } else if x == root_key {
                                }
                            };
                            // BST ordering: elements in right.right > right.key (unchanged).
                            // Element preservation.
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
                                        assert(link_contains(orig_right_right, x));
                                        assert(link_contains(right.right, x));
                                    }
                                }
                            };
                            assert forall|x: T| link_contains(Some(right), x) implies
                                link_contains(Some(orig), x)
                            by {
                                reveal_with_fuel(link_contains, 3);
                                if x == right_key {
                                    assert(link_contains(orig_root_right, right_key));
                                } else if link_contains(right.right, x) {
                                    assert(link_contains(orig_right_right, x));
                                    assert(link_contains(orig_root_right, x));
                                } else {
                                    // x in right.left = Some(root with left=orig_root_left, right=orig_right_left)
                                    assert(link_contains(right.left, x));
                                    reveal_with_fuel(link_contains, 2);
                                    if x == root_key {
                                    } else if link_contains(orig_root_left, x) {
                                    } else {
                                        assert(link_contains(orig_right_left, x));
                                        assert(link_contains(orig_root_right, x));
                                    }
                                }
                            };
                        }
                        right
                    }
                    core::cmp::Ordering::Greater => {
                        // Zag-zag: recurse into right.right, then two left rotations.
                        if let Some(rr) = right.right.take() {
                            right.right = Some(splay(rr, target));
                        }
                        root.right = right.left.take();
                        update(&mut root);
                        proof {
                            assert(root.key == root_key);
                            assert(root.left == orig_root_left);
                            assert(root.right == orig_right_left);
                        }
                        right.left = Some(root);
                        update(&mut right);
                        if let Some(mut rr) = right.right.take() {
                            let ghost rr_key = rr.key;
                            let ghost rr_left = rr.left;
                            let ghost rr_right = rr.right;
                            right.right = rr.left.take();
                            update(&mut right);
                            proof {
                                assert(right.key == right_key);
                                assert(right.right == rr_left);
                            }
                            rr.left = Some(right);
                            update(&mut rr);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                assert(rr.key == rr_key);
                                assert(rr.right == rr_right);
                                // rr_key ∈ splay result ∈ orig_right_right, so > right_key.
                                assert(link_contains(orig_right_right, rr_key));
                                // BST: rr.left (= Some(right)) elements < rr_key.
                                assert forall|x: T| link_contains(rr.left, x) implies
                                    (T::le(x, rr_key) && x != rr_key)
                                by {
                                    reveal_with_fuel(link_contains, 4);
                                    if x == right_key {
                                    } else if link_contains(rr_left, x) {
                                    } else if link_contains(right.left, x) {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == root_key {
                                            T::transitive(root_key, right_key, rr_key);
                                            if x == rr_key { T::antisymmetric(root_key, rr_key); }
                                        } else if link_contains(orig_right_left, x) {
                                            T::transitive(x, right_key, rr_key);
                                            if x == rr_key { T::antisymmetric(right_key, rr_key); }
                                        } else {
                                            assert(link_contains(orig_root_left, x));
                                            T::transitive(x, root_key, rr_key);
                                            if x == rr_key { T::antisymmetric(root_key, rr_key); }
                                        }
                                    }
                                };
                                // BST: right.right (= rr_left) elements > right_key.
                                assert forall|x: T| link_contains(right.right, x) implies
                                    (T::le(right_key, x) && x != right_key)
                                by {
                                    assert(link_contains(orig_right_right, x));
                                };
                                // BST: right.left elements < right_key.
                                assert forall|x: T| link_contains(right.left, x) implies
                                    (T::le(x, right_key) && x != right_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if link_contains(orig_right_left, x) {
                                    } else if link_contains(orig_root_left, x) {
                                        T::transitive(x, root_key, right_key);
                                        if x == right_key { T::antisymmetric(root_key, right_key); }
                                    } else if x == root_key {
                                    }
                                };
                                // Element preservation.
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
                                            assert(link_contains(orig_right_left, x));
                                        }
                                    }
                                };
                                assert forall|x: T| link_contains(Some(rr), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == rr_key {
                                        assert(link_contains(orig_right_right, rr_key));
                                        assert(link_contains(orig_root_right, rr_key));
                                    } else if link_contains(rr_right, x) {
                                        assert(link_contains(orig_right_right, x));
                                        assert(link_contains(orig_root_right, x));
                                    } else {
                                        assert(link_contains(rr.left, x));
                                        reveal_with_fuel(link_contains, 3);
                                        if x == right_key {
                                            assert(link_contains(orig_root_right, right_key));
                                        } else if link_contains(rr_left, x) {
                                            assert(link_contains(orig_right_right, x));
                                            assert(link_contains(orig_root_right, x));
                                        } else {
                                            reveal_with_fuel(link_contains, 2);
                                            if x == root_key {
                                            } else if link_contains(orig_right_left, x) {
                                                assert(link_contains(orig_root_right, x));
                                            } else {
                                                assert(link_contains(orig_root_left, x));
                                            }
                                        }
                                    }
                                };
                                // Help solver piece together BST for rr.
                                assert(spec_is_bst_link(rr_left));
                                assert(spec_is_bst_link(rr_right));
                                assert(spec_is_bst_link(orig_root_left));
                                assert(spec_is_bst_link(orig_right_right));
                                assert(spec_is_bst_link(Some(right))) by {
                                    reveal_with_fuel(spec_is_bst_link, 2);
                                };
                                assert(spec_is_bst_link(Some(root))) by {
                                    reveal_with_fuel(spec_is_bst_link, 2);
                                };
                                assert(spec_is_bst_link(Some(rr))) by {
                                    reveal_with_fuel(spec_is_bst_link, 2);
                                };
                            }
                            rr
                        } else {
                            // orig_right_right was None. Single Zag rotation.
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                assert(right.key == right_key);
                                assert forall|x: T| link_contains(right.left, x) implies
                                    (T::le(x, right_key) && x != right_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if link_contains(orig_right_left, x) {
                                    } else if link_contains(orig_root_left, x) {
                                        T::transitive(x, root_key, right_key);
                                        if x == right_key { T::antisymmetric(root_key, right_key); }
                                    } else if x == root_key {
                                    }
                                };
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
                                assert forall|x: T| link_contains(Some(right), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == right_key {
                                        assert(link_contains(orig_root_right, right_key));
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == root_key {
                                        } else if link_contains(orig_right_left, x) {
                                            assert(link_contains(orig_root_right, x));
                                        } else {
                                            assert(link_contains(orig_root_left, x));
                                        }
                                    }
                                };
                                assert(spec_is_bst_link(Some(right))) by {
                                    reveal_with_fuel(spec_is_bst_link, 4);
                                };
                            }
                            right
                        }
                    }
                    core::cmp::Ordering::Less => {
                        // Zag-zig: recurse into right.left, right-rotate right, left-rotate root.
                        if let Some(rl) = right.left.take() {
                            right.left = Some(splay(rl, target));
                        }
                        if right.left.is_some() {
                            let mut rl = right.left.take().unwrap();
                            let ghost rl_key = rl.key;
                            let ghost rl_left = rl.left;
                            let ghost rl_right = rl.right;
                            // rl is splay of orig_right_left. BST, same elements.
                            proof {
                                assert(link_contains(orig_right_left, rl_key));
                                assert(link_contains(orig_root_right, rl_key));
                                // Capture splay BST ordering while rl is intact.
                                assert forall|x: T| link_contains(rl_left, x) implies
                                    (T::le(x, rl_key) && x != rl_key) by {};
                                assert forall|x: T| link_contains(rl_right, x) implies
                                    (T::le(rl_key, x) && x != rl_key) by {};
                            }
                            right.left = rl.right.take();
                            update(&mut right);
                            proof {
                                assert(right.key == right_key);
                                assert(right.left == rl_right);
                                assert(right.right == orig_right_right);
                            }
                            rl.right = Some(right);
                            update(&mut rl);
                            root.right = rl.left.take();
                            update(&mut root);
                            proof {
                                assert(root.key == root_key);
                                assert(root.left == orig_root_left);
                                assert(root.right == rl_left);
                            }
                            rl.left = Some(root);
                            update(&mut rl);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(link_contains, 5);
                                assert(rl.key == rl_key);
                                // BST: rl.right (= Some(right)) elements > rl_key.
                                assert forall|x: T| link_contains(rl.right, x) implies
                                    (T::le(rl_key, x) && x != rl_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == right_key {
                                    } else if link_contains(orig_right_right, x) {
                                        T::transitive(rl_key, right_key, x);
                                        if x == rl_key { T::antisymmetric(rl_key, right_key); }
                                    } else {
                                        // x ∈ rl_right ⊂ orig_right_left < right_key, > rl_key from splay BST.
                                    }
                                };
                                // BST: rl.left (= Some(root)) elements < rl_key.
                                assert forall|x: T| link_contains(rl.left, x) implies
                                    (T::le(x, rl_key) && x != rl_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == root_key {
                                    } else if link_contains(rl_left, x) {
                                        // rl_left < rl_key from splay BST.
                                    } else {
                                        assert(link_contains(orig_root_left, x));
                                        T::transitive(x, root_key, rl_key);
                                        if x == rl_key { T::antisymmetric(root_key, rl_key); }
                                    }
                                };
                                // BST: right.left (= rl_right) elements < right_key.
                                assert forall|x: T| link_contains(right.left, x) implies
                                    (T::le(x, right_key) && x != right_key)
                                by {
                                    assert(link_contains(orig_right_left, x));
                                };
                                // BST: root.right (= rl_left) elements > root_key.
                                assert forall|x: T| link_contains(root.right, x) implies
                                    (T::le(root_key, x) && x != root_key)
                                by {
                                    assert(link_contains(orig_right_left, x));
                                    assert(link_contains(orig_root_right, x));
                                };
                                // Element preservation.
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
                                            assert(link_contains(orig_right_left, x));
                                        }
                                    }
                                };
                                assert forall|x: T| link_contains(Some(rl), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 5);
                                    if x == rl_key {
                                        assert(link_contains(orig_root_right, rl_key));
                                    } else if link_contains(rl.right, x) {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == right_key {
                                            assert(link_contains(orig_root_right, right_key));
                                        } else if link_contains(orig_right_right, x) {
                                            assert(link_contains(orig_root_right, x));
                                        } else {
                                            assert(link_contains(orig_right_left, x));
                                            assert(link_contains(orig_root_right, x));
                                        }
                                    } else {
                                        reveal_with_fuel(link_contains, 3);
                                        if x == root_key {
                                        } else if link_contains(rl_left, x) {
                                            assert(link_contains(orig_right_left, x));
                                            assert(link_contains(orig_root_right, x));
                                        } else {
                                            assert(link_contains(orig_root_left, x));
                                        }
                                    }
                                };
                                assert(spec_is_bst_link(Some(rl))) by {
                                    reveal_with_fuel(spec_is_bst_link, 4);
                                };
                            }
                            rl
                        } else {
                            // orig_right_left was None. Single Zag rotation.
                            proof {
                                assert(root.key == root_key);
                                assert(root.left == orig_root_left);
                            }
                            root.right = right.left.take();
                            update(&mut root);
                            right.left = Some(root);
                            update(&mut right);
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(link_contains, 4);
                                assert(right.key == right_key);
                                assert forall|x: T| link_contains(right.left, x) implies
                                    (T::le(x, right_key) && x != right_key)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if link_contains(orig_root_left, x) {
                                        T::transitive(x, root_key, right_key);
                                        if x == right_key { T::antisymmetric(root_key, right_key); }
                                    } else if x == root_key {
                                    }
                                };
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
                                assert forall|x: T| link_contains(Some(right), x) implies
                                    link_contains(Some(orig), x)
                                by {
                                    reveal_with_fuel(link_contains, 3);
                                    if x == right_key {
                                        assert(link_contains(orig_root_right, right_key));
                                    } else if link_contains(right.right, x) {
                                        assert(link_contains(orig_right_right, x));
                                        assert(link_contains(orig_root_right, x));
                                    } else {
                                        reveal_with_fuel(link_contains, 2);
                                        if x == root_key {
                                        } else {
                                            assert(link_contains(orig_root_left, x));
                                        }
                                    }
                                };
                                assert(spec_is_bst_link(Some(right))) by {
                                    reveal_with_fuel(spec_is_bst_link, 4);
                                };
                            }
                            right
                        }
                    }
                }
            }
        }
    }

    fn bst_insert<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>, value: T) -> (inserted: bool)
        requires spec_is_bst_link(*old(link)),
        ensures
            spec_is_bst_link(*link),
            link_contains(*link, value),
            forall|x: T| link_contains(*old(link), x) ==> link_contains(*link, x),
            forall|x: T| link_contains(*link, x) ==> (link_contains(*old(link), x) || x == value),
        decreases old(link),
    {
        let cur = link.take();
        match cur {
            | None => {
                *link = Some(Box::new(new_node(value)));
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
                        bst_insert(&mut node.left, value);
                        update(&mut node);
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            assert forall|x: T| link_contains(node.left, x) implies
                                (T::le(x, node.key) && x != node.key)
                            by {
                                if link_contains(old_left, x) {
                                } else {
                                    assert(x == value);
                                }
                            };
                            assert forall|x: T| link_contains(*old(link), x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            assert forall|x: T| (node_key == x || link_contains(old_left, x) || link_contains(old_right, x)) implies
                                link_contains(*old(link), x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            assert forall|x: T| link_contains(*old(link), x) implies
                                link_contains(*link, x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_left, x) {
                                    assert(link_contains(node.left, x));
                                }
                            };
                            assert forall|x: T| link_contains(*link, x) implies
                                (link_contains(*old(link), x) || x == value)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                    assert(node_key == x);
                                } else if link_contains(node.left, x) {
                                    if link_contains(old_left, x) {
                                    }
                                }
                            };
                        }
                        true
                    }
                    core::cmp::Ordering::Greater => {
                        bst_insert(&mut node.right, value);
                        update(&mut node);
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            assert forall|x: T| link_contains(node.right, x) implies
                                (T::le(node.key, x) && x != node.key)
                            by {
                                if link_contains(old_right, x) {
                                } else {
                                    assert(x == value);
                                }
                            };
                            assert forall|x: T| link_contains(*old(link), x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            assert forall|x: T| (node_key == x || link_contains(old_left, x) || link_contains(old_right, x)) implies
                                link_contains(*old(link), x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            assert forall|x: T| link_contains(*old(link), x) implies
                                link_contains(*link, x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_right, x) {
                                    assert(link_contains(node.right, x));
                                }
                            };
                            assert forall|x: T| link_contains(*link, x) implies
                                (link_contains(*old(link), x) || x == value)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                    assert(node_key == x);
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
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                        }
                        false
                    }
                }
            }
        }
    }

    fn insert_link<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>, value: T) -> (inserted: bool)
        requires spec_is_bst_link(*old(link)),
        ensures
            spec_is_bst_link(*link),
            link_contains(*link, value),
            forall|x: T| link_contains(*old(link), x) ==> link_contains(*link, x),
            forall|x: T| link_contains(*link, x) ==> (link_contains(*old(link), x) || x == value),
    {
        let v = value.clone();
        let inserted = bst_insert(link, value);
        if inserted {
            if let Some(root) = link.take() {
                *link = Some(splay(root, &v));
            }
        }
        inserted
    }

    fn find_link<'a, T: StTInMtT + Ord + TotalOrder>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
        requires spec_is_bst_link(*link),
        ensures
            found.is_some() <==> link_contains(*link, *target),
            found.is_some() ==> *found.unwrap() == *target,
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                match TotalOrder::cmp(target, &node.key) {
                    core::cmp::Ordering::Equal => Some(&node.key),
                    core::cmp::Ordering::Less => {
                        proof {
                            assert(!link_contains(node.right, *target)) by {
                                if link_contains(node.right, *target) {
                                    T::antisymmetric(*target, node.key);
                                }
                            };
                        }
                        find_link(&node.left, target)
                    }
                    core::cmp::Ordering::Greater => {
                        proof {
                            assert(!link_contains(node.left, *target)) by {
                                if link_contains(node.left, *target) {
                                    T::antisymmetric(node.key, *target);
                                }
                            };
                        }
                        find_link(&node.right, target)
                    }
                }
            }
        }
    }

    fn min_link<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (min: Option<&T>)
        requires spec_is_bst_link(*link),
        ensures
            link.is_some() ==> min.is_some(),
            min.is_some() ==> link_contains(*link, *min.unwrap()),
            min.is_some() ==> forall|x: T| link_contains(*link, x) ==> T::le(*min.unwrap(), x),
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => {
                    proof {
                        assert forall|x: T| link_contains(*link, x) implies T::le(node.key, x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                T::reflexive(x);
                            } else {
                                assert(link_contains(node.right, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let min = min_link(&node.left);
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        assert(link_contains(node.left, *min.unwrap()));
                        assert forall|x: T| link_contains(*link, x) implies T::le(*min.unwrap(), x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.left, x) {
                            } else if x == node.key {
                            } else {
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

    fn max_link<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (max: Option<&T>)
        requires spec_is_bst_link(*link),
        ensures
            link.is_some() ==> max.is_some(),
            max.is_some() ==> link_contains(*link, *max.unwrap()),
            max.is_some() ==> forall|x: T| link_contains(*link, x) ==> T::le(x, *max.unwrap()),
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => {
                    proof {
                        assert forall|x: T| link_contains(*link, x) implies T::le(x, node.key) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                T::reflexive(x);
                            } else {
                                assert(link_contains(node.left, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let max = max_link(&node.right);
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        assert(link_contains(node.right, *max.unwrap()));
                        assert forall|x: T| link_contains(*link, x) implies T::le(x, *max.unwrap()) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.right, x) {
                            } else if x == node.key {
                            } else {
                                assert(link_contains(node.left, x));
                                T::transitive(x, node.key, *max.unwrap());
                            }
                        };
                    }
                    max
                }
            },
        }
    }

    fn in_order_collect<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>, out: &mut Vec<T>)
        requires link_spec_size(*link) <= usize::MAX as nat,
        ensures true,
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>, out: &mut Vec<T>)
        requires link_spec_size(*link) <= usize::MAX as nat,
        ensures true,
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    fn in_order_parallel<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (result: Vec<T>)
        requires link_spec_size(*link) <= usize::MAX as nat,
        ensures true,
        decreases *link,
    {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                use crate::Types::Types::Pair;
                let Pair(left_vec, right_vec) = crate::ParaPair!(
                    move || in_order_parallel(&node.left),
                    move || in_order_parallel(&node.right)
                );
                let mut result = left_vec;
                result.push(node.key.clone());
                result.extend(right_vec);
                result
            }
        }
    }

    fn pre_order_parallel<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (result: Vec<T>)
        requires link_spec_size(*link) <= usize::MAX as nat,
        ensures true,
        decreases *link,
    {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                use crate::Types::Types::Pair;
                let Pair(left_vec, right_vec) = crate::ParaPair!(
                    move || pre_order_parallel(&node.left),
                    move || pre_order_parallel(&node.right)
                );
                let mut result = vec![node.key.clone()];
                result.extend(left_vec);
                result.extend(right_vec);
                result
            }
        }
    }

    fn build_balanced<T: StTInMtT + Ord + TotalOrder>(values: &[T]) -> (link: Link<T>)
        ensures link_spec_size(link) <= values@.len(),
        decreases values.len(),
    {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        let left_slice = &values[..mid];
        let right_slice = &values[mid + 1..];

        use crate::Types::Types::Pair;
        let f1 = move || -> (l: Link<T>)
            ensures link_spec_size(l) <= left_slice@.len()
        { build_balanced(left_slice) };
        let f2 = move || -> (r: Link<T>)
            ensures link_spec_size(r) <= right_slice@.len()
        { build_balanced(right_slice) };
        let Pair(left, right) = crate::ParaPair!(f1, f2);

        let mut node = Box::new(new_node(values[mid].clone()));
        node.left = left;
        node.right = right;
        update(&mut node);
        Some(node)
    }

    fn filter_parallel<T: StTInMtT + Ord + TotalOrder, F>(link: &Link<T>, predicate: &Arc<F>) -> (result: Vec<T>)
        where
            F: Fn(&T) -> bool + Send + Sync,
        requires link_spec_size(*link) <= usize::MAX as nat,
        ensures true,
        decreases *link,
    {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                let pred_left = Arc::clone(predicate);
                let pred_right = Arc::clone(predicate);

                use crate::Types::Types::Pair;
                let Pair(left_vals, right_vals) = crate::ParaPair!(
                    move || filter_parallel(&node.left, &pred_left),
                    move || filter_parallel(&node.right, &pred_right)
                );

                let mut result = left_vals;
                if predicate(&node.key) {
                    result.push(node.key.clone());
                }
                result.extend(right_vals);
                result
            }
        }
    }

    fn reduce_parallel<T: StTInMtT + Ord + TotalOrder, F>(link: &Link<T>, op: &Arc<F>, identity: T) -> (result: T)
        where
            F: Fn(T, T) -> T + Send + Sync,
        requires link_spec_size(*link) <= usize::MAX as nat,
        ensures true,
        decreases *link,
    {
        match link {
            | None => identity,
            | Some(node) => {
                let op_left = Arc::clone(op);
                let op_right = Arc::clone(op);
                let id_left = identity.clone();

                use crate::Types::Types::Pair;
                let Pair(left_acc, right_acc) = crate::ParaPair!(
                    move || reduce_parallel(&node.left, &op_left, id_left),
                    move || reduce_parallel(&node.right, &op_right, identity)
                );

                let with_key = op(left_acc, node.key.clone());
                op(with_key, right_acc)
            }
        }
    }

    fn height_rec<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (h: N)
        requires link_height(*link) < usize::MAX as nat,
        ensures h as nat == link_height(*link),
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
        }
    }

    /// Exec mirror of link_spec_size for runtime size guards.
    fn compute_link_spec_size<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (n: usize)
        requires link_spec_size(*link) <= usize::MAX,
        ensures n as nat == link_spec_size(*link),
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let l = compute_link_spec_size(&node.left);
                let r = compute_link_spec_size(&node.right);
                1 + l + r
            }
        }
    }

    // 11. top level coarse locking

    /// Lock predicate: link size fits in usize.
    pub struct BSTSplayMtEphInv;

    impl<T: StTInMtT + Ord + TotalOrder> RwLockPredicate<Link<T>> for BSTSplayMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            link_spec_size(v) <= usize::MAX
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSplayMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub(crate) root: RwLock<Link<T>, BSTSplayMtEphInv>,
        pub(crate) ghost_root: Ghost<Link<T>>,
    }

    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;

    impl<T: StTInMtT + Ord + TotalOrder> BSTSplayMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            link_spec_size(self.ghost_root@) <= usize::MAX
        }

        pub closed spec fn spec_ghost_root(self) -> Link<T> {
            self.ghost_root@
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSplayMtEph<T> {
        type V = Link<T>;
        open spec fn view(&self) -> Link<T> { self.spec_ghost_root() }
    }

    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized + View<V = Link<T>> {
        spec fn spec_bstsplaymteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
            ensures tree.spec_bstsplaymteph_wf(),
                    tree@ is None;

        fn from_sorted_slice(values: &[T]) -> (tree: Self)
            ensures tree.spec_bstsplaymteph_wf();

        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsplaymteph_wf(),
            ensures self.spec_bstsplaymteph_wf(),
                    match r {
                        Ok(_) => link_spec_size(self@) <= link_spec_size(old(self)@) + 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn contains(&self, target: &T) -> (found: B)
            requires self.spec_bstsplaymteph_wf(),
            ensures found == link_contains(self@, *target);

        fn size(&self) -> (n: N)
            requires self.spec_bstsplaymteph_wf(),
            ensures n as nat == link_spec_size(self@);

        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsplaymteph_wf(),
            ensures b == (self@ is None);

        fn height(&self) -> (h: N)
            requires self.spec_bstsplaymteph_wf(),
            ensures h as nat == link_height(self@);

        fn find(&self, target: &T) -> (found: Option<T>)
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            ensures true;
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        fn filter<F>(&self, predicate: F) -> (seq: ArraySeqStPerS<T>)
        where
            F: Fn(&T) -> bool + Send + Sync
            ensures true;
        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync
            ensures true;
    }

    impl<T: StTInMtT + Ord + TotalOrder> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {
        open spec fn spec_bstsplaymteph_wf(&self) -> bool {
            link_spec_size(self@) <= usize::MAX
            && spec_is_bst_link(self@)
        }

        fn new() -> Self {
            BSTSplayMtEph {
                root: RwLock::new(None, Ghost(BSTSplayMtEphInv)),
                ghost_root: Ghost(None),
            }
        }

        fn from_sorted_slice(values: &[T]) -> Self {
            let link = build_balanced(values);
            let ghost ghost_link = link;
            BSTSplayMtEph {
                root: RwLock::new(link, Ghost(BSTSplayMtEphInv)),
                ghost_root: Ghost(ghost_link),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        fn insert(&mut self, value: T) -> (r: Result<(), ()>) {
            let (mut current, write_handle) = self.root.acquire_write();
            proof { accept(self.ghost_root@ == current); }
            let sz = compute_link_spec_size(&current);
            if sz < usize::MAX {
                insert_link(&mut current, value);
                let ghost new_root = current;
                self.ghost_root = Ghost(new_root);
                write_handle.release_write(current);
                Ok(())
            } else {
                write_handle.release_write(current);
                Err(())
            }
        }

        // Reader: assume return value matches ghost.
        fn contains(&self, target: &T) -> (found: B) {
            let handle = self.root.acquire_read();
            let found = find_link(handle.borrow(), target).is_some();
            proof { accept(found == link_contains(self@, *target)); }
            handle.release_read();
            found
        }

        // Reader: assume return value matches ghost.
        fn size(&self) -> (n: N) {
            let handle = self.root.acquire_read();
            let n = size_link(handle.borrow());
            proof { accept(n as nat == link_spec_size(self@)); }
            handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        fn is_empty(&self) -> (b: B) {
            let handle = self.root.acquire_read();
            let b = handle.borrow().is_none();
            proof { accept(b == (self@ is None)); }
            handle.release_read();
            b
        }

        // Reader: assume return value matches ghost.
        fn height(&self) -> (h: N) {
            let handle = self.root.acquire_read();
            let h = height_rec(handle.borrow());
            proof { accept(h as nat == link_height(self@)); }
            handle.release_read();
            h
        }

        fn find(&self, target: &T) -> Option<T> {
            let handle = self.root.acquire_read();
            let found = find_link(handle.borrow(), target).cloned();
            handle.release_read();
            found
        }

        fn minimum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let min = min_link(handle.borrow()).cloned();
            handle.release_read();
            min
        }

        fn maximum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let max = max_link(handle.borrow()).cloned();
            handle.release_read();
            max
        }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let out = in_order_parallel(handle.borrow());
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let out = pre_order_parallel(handle.borrow());
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn filter<F>(&self, predicate: F) -> ArraySeqStPerS<T>
        where
            F: Fn(&T) -> bool + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let predicate = Arc::new(predicate);
            let out = filter_parallel(handle.borrow(), &predicate);
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let op = Arc::new(op);
            let accumulated = reduce_parallel(handle.borrow(), &op, identity);
            handle.release_read();
            accumulated
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> Default for BSTSplayMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    } // verus!

    // 13. macros

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

    // 14. derive impls outside verus!

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

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSplayMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSplayMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTSplayMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSplayMtEph(size={})", self.size())
        }
    }
}
