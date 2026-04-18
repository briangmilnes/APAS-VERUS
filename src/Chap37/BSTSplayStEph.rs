// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Ephemeral Splay Tree (standard BST semantics) with public methods.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 10b. iterators — BSTSplayStEph
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module


pub mod BSTSplayStEph {


    //		Section 2. imports

    use std::fmt;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! 
{


    use crate::vstdplus::total_order::total_order::TotalOrder;

    //		Section 4. type definitions


    type Link<T> = Option<Box<Node<T>>>;

    //		Section 4a. type definitions


    pub struct Node<T: TotalOrder + Clone> {
        pub key: T,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    //		Section 9a. impls


    impl<T: TotalOrder + Clone> BSTSplayNodeFns<T> for Node<T> {
        open spec fn spec_key(self) -> T { self.key }
        open spec fn spec_left(self) -> Link<T> { self.left }
        open spec fn spec_right(self) -> Link<T> { self.right }
        open spec fn spec_node_size(self) -> nat { self.size as nat }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant-time allocation.
    // veracity: no_requires
    fn new_node(key: T) -> (node: Node<T>)
    {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    // veracity: no_requires
    fn update(&mut self)
    {
// Veracity: UNNEEDED proof block         proof { reveal(spec_size_link); }
        let ls = <Link<T> as BSTSplayLinkFns<T>>::size_link(&self.left);
        let rs = <Link<T> as BSTSplayLinkFns<T>>::size_link(&self.right);
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            self.size = 1 + ls + rs;
        }
    }

    // Bottom-up splay: bring target (or nearest key) toward the root using
    // zig, zig-zig, and zig-zag rotations (Sleator & Tarjan).
    /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n) amortized, Span O(lg n) amortized
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) amortized, Span O(lg n) amortized — agrees with APAS.
    fn splay(root: Box<Node<T>>, target: &T) -> (splayed: Box<Node<T>>)
        decreases root,
    {
        let ghost orig = root;
        let mut root = root;
        // Veracity: NEEDED proof block
        proof {
            reveal_with_fuel(spec_is_bst_link, 4);
            reveal_with_fuel(spec_contains_link, 4);
        }
        match TotalOrder::cmp(target,&root.key) {
            // Veracity: NEEDED proof block
            core::cmp::Ordering::Equal => {
                proof { reveal_with_fuel(spec_contains_link, 2); }
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
                    assert forall|x: T| spec_contains_link(&orig_root_left, x) implies
                        (T::le(x, root_key) && x != root_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_root_right, x) implies
                        (T::le(root_key, x) && x != root_key) by {};
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
                    assert forall|x: T| spec_contains_link(&orig_left_left, x) implies
                        (T::le(x, left_key) && x != left_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_left_right, x) implies
                        (T::le(left_key, x) && x != left_key) by {};
                    // left_key ∈ orig_root_left, so left_key < root_key.
                    // Elements in orig_left_right are in orig_root_left, so < root_key.
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_left_right, x) implies
                        (T::le(x, root_key) && x != root_key) by {
                    };
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_left_left, x) implies
                        (T::le(x, root_key) && x != root_key) by {
                    };
                }
                match TotalOrder::cmp(target,&left.key) {
                    core::cmp::Ordering::Equal => {
                        // Zig: right rotation
                        // Veracity: NEEDED proof block
                        root.left = left.right.take();
                        root.update();
                        left.right = Some(root);
                        left.update();
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 3);
                            reveal_with_fuel(spec_contains_link, 4);
                            // BST ordering: elements in left.right (= Some(root)) > left.key.
                            // Veracity: NEEDED assert (speed hint)
                            assert(spec_contains_link(&orig_root_left, left_key)) by {
                                reveal_with_fuel(spec_contains_link, 2);
                            };
                            lemma_zig_child_ordering(&left.right, left_key, root_key, &orig_left_right, &orig_root_right);
                            // Element preservation.
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                spec_contains_link(&Some(left), x)
                            by {
                                reveal_with_fuel(spec_contains_link, 3);
                                if x == root_key {
                                } else if spec_contains_link(&orig_root_right, x) {
                                } else if spec_contains_link(&orig_root_left, x) {
                                    reveal_with_fuel(spec_contains_link, 2);
                                    if x == left_key {
                                    } else if spec_contains_link(&orig_left_left, x) {
                                    } else {
                                    }
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(&Some(left), x) implies
                                spec_contains_link(&Some(orig), x)
                            by {
                                reveal_with_fuel(spec_contains_link, 3);
                                if x == left_key {
                                } else if spec_contains_link(&left.left, x) {
                                } else {
                                    reveal_with_fuel(spec_contains_link, 2);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_left_right, x) {
                                    } else {
                                    }
                                }
                            };
                        }
                        left
                    }
                    core::cmp::Ordering::Less => {
                        // Zig-zig: recurse into left.left, then two right rotations.
                        if let Some(ll) = left.left.take() {
                            left.left = Some(Self::splay(ll, target));
                        }
                        root.left = left.right.take();
                        root.update();
                        left.right = Some(root);
                        left.update();
                        if let Some(mut ll) = left.left.take() {
                            let ghost ll_key = ll.key;
                            let ghost ll_left = ll.left;
                            // Veracity: NEEDED proof block
                            let ghost ll_right = ll.right;
                            left.left = ll.right.take();
                            left.update();
                            ll.right = Some(left);
                            ll.update();
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(spec_contains_link, 5);
                                // ll_key ∈ splay result ∈ orig_left_left, so < left_key.
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&orig_left_left, ll_key));
                                // BST: left.right > left_key, then chain ll.right > ll_key.
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&orig_root_left, left_key)) by {
                                reveal_with_fuel(spec_contains_link, 2);
                            };
                            lemma_zig_child_ordering(&left.right, left_key, root_key, &orig_left_right, &orig_root_right);
                                lemma_zig_child_ordering(&ll.right, ll_key, left_key, &ll_right, &left.right);
                                // BST: left.left (= ll_right) elements < left_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] spec_contains_link(&left.left, x) implies
                                    (T::le(x, left_key) && x != left_key)
                                by {
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(ll), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == left_key {
                                        } else if spec_contains_link(&orig_left_left, x) {
                                        } else {
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(ll), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == ll_key {
                                    } else if spec_contains_link(&ll_left, x) {
                                        // Veracity: NEEDED assert
                                        assert(spec_contains_link(&orig_left_left, x));
                                    } else {
                                        reveal_with_fuel(spec_contains_link, 3);
                                        if x == left_key {
                                        } else if spec_contains_link(&ll_right, x) {
                                            // Veracity: NEEDED assert
                                            assert(spec_contains_link(&orig_left_left, x));
                                        } else {
                                            reveal_with_fuel(spec_contains_link, 2);
                                            if x == root_key {
                                            } else if spec_contains_link(&orig_left_right, x) {
                                            } else {
                                            }
                                        }
                                    // Veracity: NEEDED proof block
                                    }
                                };
                            }
                            ll
                        } else {
                            // orig_left_left was None. Single Zig rotation.
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(spec_contains_link, 4);
                                // Veracity: NEEDED assert (speed hint)
                                assert(spec_contains_link(&orig_root_left, left_key)) by {
                                reveal_with_fuel(spec_contains_link, 2);
                            };
                            lemma_zig_child_ordering(&left.right, left_key, root_key, &orig_left_right, &orig_root_right);
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(left), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(left), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == left_key {
                                    } else {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == root_key {
                                        } else if spec_contains_link(&orig_left_right, x) {
                                        } else {
                                        }
                                    }
                                };
                            }
                            left
                        }
                    }
                    core::cmp::Ordering::Greater => {
                        // Zig-zag: recurse into left.right, left-rotate left, right-rotate root.
                        if let Some(lr) = left.right.take() {
                            left.right = Some(Self::splay(lr, target));
                        // Veracity: NEEDED proof block
                        }
                        if left.right.is_some() {
                            let mut lr = left.right.take().unwrap();
                            let ghost lr_key = lr.key;
                            let ghost lr_left = lr.left;
                            let ghost lr_right = lr.right;
                            // lr is splay of orig_left_right. BST, same elements.
                            proof {
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&orig_left_right, lr_key));
                                // Capture splay BST ordering while lr is intact.
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&lr_left, x) implies
                                    (T::le(x, lr_key) && x != lr_key) by {};
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&lr_right, x) implies
                                    (T::le(lr_key, x) && x != lr_key) by {};
                            }
                            // Veracity: NEEDED proof block
                            left.right = lr.left.take();
                            left.update();
                            lr.left = Some(left);
                            lr.update();
                            root.left = lr.right.take();
                            root.update();
                            lr.right = Some(root);
                            lr.update();
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(spec_contains_link, 5);
                                // BST: lr.left (= Some(left)) elements < lr_key.
                                lemma_zag_child_ordering(&lr.left, left_key, lr_key, &lr_left, &orig_left_left);
                                // BST: lr.right (= Some(root)) elements > lr_key.
                                lemma_zig_child_ordering(&lr.right, lr_key, root_key, &lr_right, &orig_root_right);
                                // BST: left.right (= lr_left) elements > left_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] spec_contains_link(&left.right, x) implies
                                    (T::le(left_key, x) && x != left_key)
                                by {
                                };
                                // BST: root.left (= lr_right) elements < root_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] spec_contains_link(&root.left, x) implies
                                    (T::le(x, root_key) && x != root_key)
                                by {
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(spec_contains_link(&orig_left_right, x));
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(lr), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == left_key {
                                        } else if spec_contains_link(&orig_left_left, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(spec_contains_link(&orig_left_right, x));
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(lr), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == lr_key {
                                    } else if spec_contains_link(&lr.left, x) {
                                        reveal_with_fuel(spec_contains_link, 3);
                                        if x == left_key {
                                        } else if spec_contains_link(&orig_left_left, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(spec_contains_link(&orig_left_right, x));
                                        }
                                    } else {
                                        reveal_with_fuel(spec_contains_link, 3);
                                        if x == root_key {
                                        } else if spec_contains_link(&lr_right, x) {
                                            // Veracity: NEEDED assert (speed hint)
                                            assert(spec_contains_link(&orig_left_right, x));
                                        } else {
                                        }
                                    }
                                // Veracity: NEEDED proof block
                                };
                            }
                            lr
                        } else {
                            // orig_left_right was None. Single Zig rotation.
                            root.left = left.right.take();
                            root.update();
                            left.right = Some(root);
                            left.update();
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(spec_contains_link, 4);
                                // Veracity: NEEDED assert (speed hint)
                                assert(spec_contains_link(&orig_root_left, left_key)) by {
                                reveal_with_fuel(spec_contains_link, 2);
                            };
                            lemma_zig_child_ordering(&left.right, left_key, root_key, &orig_left_right, &orig_root_right);
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(left), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(left), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == left_key {
                                    } else if spec_contains_link(&left.left, x) {
                                    } else {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == root_key {
                                        } else {
                                        }
                                    }
                                };
                            }
                            // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_root_left, x) implies
                        (T::le(x, root_key) && x != root_key) by {};
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_root_right, x) implies
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
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_right_left, x) implies
                        (T::le(x, right_key) && x != right_key) by {};
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_right_right, x) implies
                        (T::le(right_key, x) && x != right_key) by {};
                    // right_key ∈ orig_root_right, so right_key > root_key.
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_contains_link(&orig_root_right, right_key));
                    // Elements in orig_right_left are in orig_root_right, so > root_key.
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_right_left, x) implies
                        (T::le(root_key, x) && x != root_key) by {
                    };
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    assert forall|x: T| spec_contains_link(&orig_right_right, x) implies
                        (T::le(root_key, x) && x != root_key) by {
                    };
                }
                match TotalOrder::cmp(target,&right.key) {
                    core::cmp::Ordering::Equal => {
                        // Zag: left rotation
                        root.right = right.left.take();
                        root.update();
                        right.left = Some(root);
                        right.update();
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 3);
                            reveal_with_fuel(spec_contains_link, 4);
                            // BST ordering: elements in right.left (= Some(root)) < right.key.
                            lemma_zag_child_ordering(&right.left, root_key, right_key, &orig_right_left, &orig_root_left);
                            // BST ordering: elements in right.right > right.key (unchanged).
                            // Element preservation.
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                spec_contains_link(&Some(right), x)
                            by {
                                reveal_with_fuel(spec_contains_link, 3);
                                if x == root_key {
                                } else if spec_contains_link(&orig_root_left, x) {
                                } else if spec_contains_link(&orig_root_right, x) {
                                    reveal_with_fuel(spec_contains_link, 2);
                                    if x == right_key {
                                    } else if spec_contains_link(&orig_right_left, x) {
                                    } else {
                                    }
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(&Some(right), x) implies
                                spec_contains_link(&Some(orig), x)
                            by {
                                reveal_with_fuel(spec_contains_link, 3);
                                if x == right_key {
                                } else if spec_contains_link(&right.right, x) {
                                } else {
                                    // x in right.left = Some(root with left=orig_root_left, right=orig_right_left)
                                    reveal_with_fuel(spec_contains_link, 2);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                    } else {
                                    }
                                }
                            };
                        }
                        right
                    }
                    core::cmp::Ordering::Greater => {
                        // Zag-zag: recurse into right.right, then two left rotations.
                        if let Some(rr) = right.right.take() {
                            right.right = Some(Self::splay(rr, target));
                        // Veracity: NEEDED proof block
                        }
                        root.right = right.left.take();
                        root.update();
                        right.left = Some(root);
                        right.update();
                        if let Some(mut rr) = right.right.take() {
                            let ghost rr_key = rr.key;
                            let ghost rr_left = rr.left;
                            let ghost rr_right = rr.right;
                            right.right = rr.left.take();
                            right.update();
                            rr.left = Some(right);
                            rr.update();
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(spec_contains_link, 5);
                                // rr_key ∈ splay result ∈ orig_right_right, so > right_key.
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&orig_right_right, rr_key));
                                // BST: right.left < right_key, then chain rr.left < rr_key.
                                lemma_zag_child_ordering(&right.left, root_key, right_key, &orig_right_left, &orig_root_left);
                                lemma_zag_child_ordering(&rr.left, right_key, rr_key, &rr_left, &right.left);
                                // BST: right.right (= rr_left) elements > right_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] spec_contains_link(&right.right, x) implies
                                    (T::le(right_key, x) && x != right_key)
                                by {
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(rr), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == right_key {
                                        } else if spec_contains_link(&orig_right_right, x) {
                                        } else {
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(rr), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == rr_key {
                                    } else if spec_contains_link(&rr_right, x) {
                                        // Veracity: NEEDED assert
                                        assert(spec_contains_link(&orig_right_right, x));
                                    } else {
                                        reveal_with_fuel(spec_contains_link, 3);
                                        if x == right_key {
                                        } else if spec_contains_link(&rr_left, x) {
                                            // Veracity: NEEDED assert (speed hint)
                                            // Veracity: NEEDED proof block
                                            assert(spec_contains_link(&orig_right_right, x));
                                        } else {
                                            reveal_with_fuel(spec_contains_link, 2);
                                            if x == root_key {
                                            } else if spec_contains_link(&orig_right_left, x) {
                                            } else {
                                            }
                                        }
                                    }
                                };
                            }
                            rr
                        } else {
                            // orig_right_right was None. Single Zag rotation.
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(spec_contains_link, 4);
                                lemma_zag_child_ordering(&right.left, root_key, right_key, &orig_right_left, &orig_root_left);
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(right), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(right), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == right_key {
                                    } else {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == root_key {
                                        } else if spec_contains_link(&orig_right_left, x) {
                                        } else {
                                        }
                                    }
                                };
                            // Veracity: NEEDED proof block
                            }
                            right
                        }
                    }
                    core::cmp::Ordering::Less => {
                        // Zag-zig: recurse into right.left, right-rotate right, left-rotate root.
                        if let Some(rl) = right.left.take() {
                            right.left = Some(Self::splay(rl, target));
                        }
                        if right.left.is_some() {
                            let mut rl = right.left.take().unwrap();
                            let ghost rl_key = rl.key;
                            let ghost rl_left = rl.left;
                            let ghost rl_right = rl.right;
                            // rl is splay of orig_right_left. BST, same elements.
                            proof {
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&orig_right_left, rl_key));
                                // Veracity: NEEDED proof block
                                // Capture splay BST ordering while rl is intact.
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&rl_left, x) implies
                                    (T::le(x, rl_key) && x != rl_key) by {};
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&rl_right, x) implies
                                    (T::le(rl_key, x) && x != rl_key) by {};
                            }
                            right.left = rl.right.take();
                            right.update();
                            rl.right = Some(right);
                            rl.update();
                            root.right = rl.left.take();
                            root.update();
                            rl.left = Some(root);
                            rl.update();
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 4);
                                reveal_with_fuel(spec_contains_link, 5);
                                // BST: rl.right (= Some(right)) elements > rl_key.
                                lemma_zig_child_ordering(&rl.right, rl_key, right_key, &rl_right, &orig_right_right);
                                // BST: rl.left (= Some(root)) elements < rl_key.
                                lemma_zag_child_ordering(&rl.left, root_key, rl_key, &rl_left, &orig_root_left);
                                // BST: right.left (= rl_right) elements < right_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] spec_contains_link(&right.left, x) implies
                                    (T::le(x, right_key) && x != right_key)
                                by {
                                };
                                // BST: root.right (= rl_left) elements > root_key.
                                // Veracity: NEEDED assert
                                assert forall|x: T| #[trigger] spec_contains_link(&root.right, x) implies
                                    (T::le(root_key, x) && x != root_key)
                                by {
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(spec_contains_link(&orig_right_left, x));
                                };
                                // Element preservation.
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(rl), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == right_key {
                                        } else if spec_contains_link(&orig_right_right, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(spec_contains_link(&orig_right_left, x));
                                        }
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(rl), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 5);
                                    if x == rl_key {
                                    } else if spec_contains_link(&rl.right, x) {
                                        reveal_with_fuel(spec_contains_link, 3);
                                        if x == right_key {
                                        } else if spec_contains_link(&orig_right_right, x) {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(spec_contains_link(&orig_right_left, x));
                                        }
                                    } else {
                                        // Veracity: NEEDED proof block
                                        reveal_with_fuel(spec_contains_link, 3);
                                        if x == root_key {
                                        } else if spec_contains_link(&rl_left, x) {
                                            // Veracity: NEEDED assert
                                            assert(spec_contains_link(&orig_right_left, x));
                                        } else {
                                        }
                                    }
                                };
                            }
                            rl
                        } else {
                            // orig_right_left was None. Single Zag rotation.
                            root.right = right.left.take();
                            root.update();
                            right.left = Some(root);
                            right.update();
                            proof {
                                reveal_with_fuel(spec_is_bst_link, 3);
                                reveal_with_fuel(spec_contains_link, 4);
                                lemma_zag_child_ordering(&right.left, root_key, right_key, &orig_right_left, &orig_root_left);
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(orig), x) implies
                                    spec_contains_link(&Some(right), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == root_key {
                                    } else if spec_contains_link(&orig_root_left, x) {
                                    } else if spec_contains_link(&orig_root_right, x) {
                                        reveal_with_fuel(spec_contains_link, 2);
                                    }
                                };
                                // Veracity: NEEDED assert
                                assert forall|x: T| spec_contains_link(&Some(right), x) implies
                                    spec_contains_link(&Some(orig), x)
                                by {
                                    reveal_with_fuel(spec_contains_link, 3);
                                    if x == right_key {
                                    } else if spec_contains_link(&right.right, x) {
                                    } else {
                                        reveal_with_fuel(spec_contains_link, 2);
                                        if x == root_key {
                                        } else {
                                        }
                                    }
                                };
                            }
                            right
                        }
                    }
                }
            }
        }
    }

    } // impl BSTSplayNodeFns for Node

    //		Section 4b. type definitions


    pub struct BSTSplayStEph<T: TotalOrder + Clone> {
        pub root: Link<T>,
    }

    pub type BSTreeSplay<T> = BSTSplayStEph<T>;

    //		Section 6b. spec fns


    pub open spec fn spec_size_link<T: TotalOrder + Clone>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_height_link<T: TotalOrder + Clone>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let lh = spec_height_link(&node.left);
                let rh = spec_height_link(&node.right);
                1 + if lh >= rh { lh } else { rh }
            }
        }
    }

    /// Recursive membership predicate for a splay tree link.
    pub open spec fn spec_contains_link<T: TotalOrder + Clone>(link: &Link<T>, value: T) -> bool
        decreases *link,
    {
        match link {
            None => false,
            Some(node) =>
                node.key == value
                || spec_contains_link(&node.left, value)
                || spec_contains_link(&node.right, value),
        }
    }

    /// In-order traversal of a splay tree link as a spec-level sequence.
    pub open spec fn spec_in_order_link<T: TotalOrder + Clone>(link: &Link<T>) -> Seq<T>
        decreases *link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => {
                spec_in_order_link(&node.left).push(node.key).add(spec_in_order_link(&node.right))
            }
        }
    }

    /// Pre-order traversal of a splay tree link as a spec-level sequence.
    pub open spec fn spec_pre_order_link<T: TotalOrder + Clone>(link: &Link<T>) -> Seq<T>
        decreases *link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => {
                Seq::empty().push(node.key).add(spec_pre_order_link(&node.left)).add(spec_pre_order_link(&node.right))
            }
        }
    }

    /// BST ordering invariant for a splay tree link.
    pub open spec fn spec_is_bst_link<T: TotalOrder + Clone>(link: &Link<T>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_is_bst_link(&node.left)
                && spec_is_bst_link(&node.right)
                && (forall|x: T| (#[trigger] spec_contains_link(&node.left, x)) ==>
                    T::le(x, node.key) && x != node.key)
                && (forall|x: T| (#[trigger] spec_contains_link(&node.right, x)) ==>
                    T::le(node.key, x) && x != node.key)
            }
        }
    }

    //		Section 7b. proof fns/broadcast groups


    proof fn lemma_bst_deep_link<T: TotalOrder + Clone>(link: &Link<T>)
        requires spec_is_bst_link(link),
        ensures
            match link {
                None => true,
                Some(node) =>
                    spec_is_bst_link(&node.left)
                    && spec_is_bst_link(&node.right)
                    && (forall|x: T| (#[trigger] spec_contains_link(&node.left, x)) ==>
                        T::le(x, node.key) && x != node.key)
                    && (forall|x: T| (#[trigger] spec_contains_link(&node.right, x)) ==>
                        T::le(node.key, x) && x != node.key)
                    && match &node.left {
                        None => true,
                        Some(lnode) =>
                            spec_is_bst_link(&lnode.left)
                            && spec_is_bst_link(&lnode.right)
                            && (forall|x: T| (#[trigger] spec_contains_link(&lnode.left, x)) ==>
                                T::le(x, lnode.key) && x != lnode.key)
                            && (forall|x: T| (#[trigger] spec_contains_link(&lnode.right, x)) ==>
                                T::le(lnode.key, x) && x != lnode.key)
                    }
                    && match &node.right {
                        None => true,
                        Some(rnode) =>
                            spec_is_bst_link(&rnode.left)
                            && spec_is_bst_link(&rnode.right)
                            && (forall|x: T| (#[trigger] spec_contains_link(&rnode.left, x)) ==>
                                T::le(x, rnode.key) && x != rnode.key)
                            && (forall|x: T| (#[trigger] spec_contains_link(&rnode.right, x)) ==>
                                T::le(rnode.key, x) && x != rnode.key)
                    }
            },
    {
        reveal_with_fuel(spec_is_bst_link, 3);
        match link {
            None => {},
            Some(node) => {
                match &node.left {
                    None => {},
                    Some(_) => {},
                }
                match &node.right {
                    None => {},
                    Some(_) => {},
                }
            }
        }
    }

    /// After a zig (right) rotation, all elements in the rotated child subtree
    /// are strictly greater than the pivot key lo. The child decomposes into
    /// key hi plus two subtrees; all three parts are > lo by BST ordering or
    /// transitivity through hi.
    proof fn lemma_zig_child_ordering<T: TotalOrder + Clone>(
        child: &Link<T>,
        lo: T, hi: T,
        sub_lo: &Link<T>,
        sub_hi: &Link<T>,
    )
        requires
            T::le(lo, hi), lo != hi,
            forall|y: T| spec_contains_link(sub_lo, y) ==> (T::le(lo, y) && y != lo),
            forall|y: T| spec_contains_link(sub_hi, y) ==> (T::le(hi, y) && y != hi),
            forall|y: T| spec_contains_link(child, y) ==>
                (y == hi || spec_contains_link(sub_lo, y) || spec_contains_link(sub_hi, y)),
        ensures
            forall|x: T| #[trigger] spec_contains_link(child, x) ==> (T::le(lo, x) && x != lo),
    {
        // Veracity: NEEDED assert
        assert forall|x: T| #[trigger] spec_contains_link(child, x) implies
            (T::le(lo, x) && x != lo)
        by {
            if x == hi {
            } else if spec_contains_link(sub_lo, x) {
            } else {
                T::transitive(lo, hi, x);
                if x == lo { T::antisymmetric(lo, hi); }
            }
        };
    }

    /// Mirror of lemma_zig_child_ordering for zag (left) rotation: all elements
    /// in the rotated child subtree are strictly less than the pivot key hi.
    proof fn lemma_zag_child_ordering<T: TotalOrder + Clone>(
        child: &Link<T>,
        lo: T, hi: T,
        sub_hi: &Link<T>,
        sub_lo: &Link<T>,
    )
        requires
            T::le(lo, hi), lo != hi,
            forall|y: T| spec_contains_link(sub_hi, y) ==> (T::le(y, hi) && y != hi),
            forall|y: T| spec_contains_link(sub_lo, y) ==> (T::le(y, lo) && y != lo),
            forall|y: T| spec_contains_link(child, y) ==>
                (y == lo || spec_contains_link(sub_lo, y) || spec_contains_link(sub_hi, y)),
        ensures
            forall|x: T| #[trigger] spec_contains_link(child, x) ==> (T::le(x, hi) && x != hi),
    {
        // Veracity: NEEDED assert
        assert forall|x: T| #[trigger] spec_contains_link(child, x) implies
            (T::le(x, hi) && x != hi)
        by {
            if x == lo {
            } else if spec_contains_link(sub_hi, x) {
            } else {
                T::transitive(x, lo, hi);
                if x == hi { T::antisymmetric(lo, hi); }
            }
        };
    }

    //		Section 8b. traits


    pub trait BSTSplayNodeFns<T: TotalOrder + Clone>: Sized {
        spec fn spec_key(self) -> T;
        spec fn spec_left(self) -> Link<T>;
        spec fn spec_right(self) -> Link<T>;
        spec fn spec_node_size(self) -> nat;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant-time allocation.
        // veracity: no_requires
        fn new_node(key: T) -> (node: Node<T>)
            ensures
                node.key == key,
                node.size == 1,
                node.left is None,
                node.right is None;

        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        // veracity: no_requires
        fn update(&mut self)
            ensures
                self.spec_key() == old(self).spec_key(),
                self.spec_left() == old(self).spec_left(),
                self.spec_right() == old(self).spec_right();

        fn splay(root: Box<Node<T>>, target: &T) -> (splayed: Box<Node<T>>)
            requires spec_is_bst_link(&Some(root)),
            ensures
                spec_is_bst_link(&Some(splayed)),
                forall|x: T| spec_contains_link(&Some(splayed), x) <==> spec_contains_link(&Some(root), x);
    }


    pub trait BSTSplayLinkFns<T: TotalOrder + Clone> {
        spec fn link_is_bst(&self) -> bool;
        spec fn link_contains(&self, value: T) -> bool;
        spec fn link_size(&self) -> nat;
        spec fn link_height(&self) -> nat;
        spec fn link_in_order(&self) -> Seq<T>;
        spec fn link_pre_order(&self) -> Seq<T>;
        spec fn link_is_some(&self) -> bool;

        // veracity: no_requires
        fn size_link(&self) -> (size: usize)
            ensures size as nat == self.link_size();
        fn height_link(&self) -> (height: usize)
            requires self.link_height() < usize::MAX as nat,
            ensures height as nat == self.link_height();
        fn bst_insert(link: &mut Link<T>, value: T) -> (inserted: bool)
            requires spec_is_bst_link(old(link)),
            ensures
                spec_is_bst_link(link),
                spec_contains_link(link, value),
                forall|x: T| spec_contains_link(old(link), x) ==> spec_contains_link(link, x),
                forall|x: T| spec_contains_link(link, x) ==> (spec_contains_link(old(link), x) || x == value);
        fn insert_link(link: &mut Link<T>, value: T) -> (inserted: bool)
            requires spec_is_bst_link(old(link)),
            ensures
                spec_is_bst_link(link),
                spec_contains_link(link, value),
                forall|x: T| spec_contains_link(old(link), x) ==> spec_contains_link(link, x),
                forall|x: T| spec_contains_link(link, x) ==> (spec_contains_link(old(link), x) || x == value);
        fn find_link<'a>(&'a self, target: &T) -> (found: Option<&'a T>)
            requires self.link_is_bst(),
            ensures
                found.is_some() <==> self.link_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        fn min_link(&self) -> (min: Option<&T>)
            requires self.link_is_bst(),
            ensures
                self.link_is_some() ==> min.is_some(),
                min.is_some() ==> self.link_contains(*min.unwrap()),
                min.is_some() ==> forall|x: T| #[trigger] self.link_contains(x) ==> T::le(*min.unwrap(), x);
        fn max_link(&self) -> (max: Option<&T>)
            requires self.link_is_bst(),
            ensures
                self.link_is_some() ==> max.is_some(),
                max.is_some() ==> self.link_contains(*max.unwrap()),
                max.is_some() ==> forall|x: T| #[trigger] self.link_contains(x) ==> T::le(x, *max.unwrap());
        // veracity: no_requires
        fn in_order_collect(&self, out: &mut Vec<T>)
            requires self.link_is_bst(),
            ensures out@.len() == old(out)@.len() + self.link_in_order().len();
        // veracity: no_requires
        fn pre_order_collect(&self, out: &mut Vec<T>)
            requires self.link_is_bst(),
            ensures out@.len() == old(out)@.len() + self.link_pre_order().len();
    }


    pub trait BSTSplayStEphTrait<T: TotalOrder + Clone> {
        spec fn spec_size(self) -> nat;
        spec fn spec_height(self) -> nat;
        spec fn spec_contains(self, value: T) -> bool;
        spec fn spec_bstsplaysteph_wf(&self) -> bool;
        spec fn spec_in_order(self) -> Seq<T>;
        spec fn spec_pre_order(self) -> Seq<T>;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
        where
            Self: Sized,
            ensures
                tree.spec_bstsplaysteph_wf(),
                tree.spec_size() == 0,
                forall|x: T| !tree.spec_contains(x);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (n: usize)
            requires self.spec_bstsplaysteph_wf(),
            ensures n as nat == self.spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstsplaysteph_wf(),
            ensures b == (self.spec_size() == 0);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires
                self.spec_bstsplaysteph_wf(),
                self.spec_height() < usize::MAX as nat,
            ensures h as nat == self.spec_height();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn insert(&mut self, value: T)
            requires old(self).spec_bstsplaysteph_wf(),
            ensures
                self.spec_bstsplaysteph_wf(),
                self.spec_contains(value),
                forall|x: T| old(self).spec_contains(x) ==> self.spec_contains(x);
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures
                found.is_some() <==> self.spec_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstsplaysteph_wf(),
            ensures found == self.spec_contains(*target);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<&T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures
                self.spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> self.spec_contains(*min.unwrap()),
                min.is_some() ==> forall|x: T| self.spec_contains(x) ==> T::le(*min.unwrap(), x);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<&T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures
                self.spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> self.spec_contains(*max.unwrap()),
                max.is_some() ==> forall|x: T| self.spec_contains(x) ==> T::le(x, *max.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures seq.spec_len() == self.spec_in_order().len();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures seq.spec_len() == self.spec_pre_order().len();
    // Veracity: NEEDED proof block
    }

    //		Section 9b. impls


    impl<T: TotalOrder + Clone> BSTSplayLinkFns<T> for Link<T> {
        open spec fn link_is_bst(&self) -> bool { spec_is_bst_link(self) }
        open spec fn link_contains(&self, value: T) -> bool { spec_contains_link(self, value) }
        open spec fn link_size(&self) -> nat { spec_size_link(self) }
        open spec fn link_height(&self) -> nat { spec_height_link(self) }
        // Veracity: NEEDED proof block
        open spec fn link_in_order(&self) -> Seq<T> { spec_in_order_link(self) }
        open spec fn link_pre_order(&self) -> Seq<T> { spec_pre_order_link(self) }
        open spec fn link_is_some(&self) -> bool { self.is_some() }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — cached size field.
    // veracity: no_requires
    fn size_link(&self) -> (size: usize)
    {
        proof { reveal(spec_size_link); }
        match self.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive tree traversal.
    fn height_link(&self) -> (height: usize)
        decreases *self,
    {
        proof { reveal_with_fuel(spec_height_link, 2); }
        // Veracity: NEEDED proof block
        match self {
            | None => 0,
            | Some(node) => {
                let lh = node.left.height_link();
                let rh = node.right.height_link();
                let m = if lh >= rh { lh } else { rh };
                1 + m
            }
        }
    }

    /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — standard BST insert path.
    fn bst_insert(link: &mut Link<T>, value: T) -> (inserted: bool)
        // Veracity: NEEDED proof block
        decreases *old(link),
    {
        let cur = link.take();
        match cur {
            | None => {
                *link = Some(Box::new(Node::<T>::new_node(value)));
                proof {
                    reveal_with_fuel(spec_is_bst_link, 2);
                    reveal_with_fuel(spec_contains_link, 2);
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
                        node.update();
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(spec_contains_link, 2);
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(&node.left, x) implies
                                (T::le(x, node.key) && x != node.key)
                            by {
                                if spec_contains_link(&old_left, x) {
                                } else {
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(old(link), x) implies
                                spec_contains_link(link, x)
                            // Veracity: NEEDED proof block
                            by {
                                reveal_with_fuel(spec_contains_link, 2);
                                if spec_contains_link(&old_left, x) {
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(link, x) implies
                                (spec_contains_link(old(link), x) || x == value)
                            by {
                                reveal_with_fuel(spec_contains_link, 2);
                                if spec_contains_link(&node.left, x) {
                                    if !spec_contains_link(&old_left, x) {
                                    }
                                }
                            };
                        }
                        true
                    }
                    core::cmp::Ordering::Greater => {
                        Self::bst_insert(&mut node.right, value);
                        node.update();
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(spec_contains_link, 2);
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(&node.right, x) implies
                                (T::le(node.key, x) && x != node.key)
                            by {
                                if spec_contains_link(&old_right, x) {
                                } else {
                                }
                            };
                            // Veracity: NEEDED proof block
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(old(link), x) implies
                                spec_contains_link(link, x)
                            by {
                                reveal_with_fuel(spec_contains_link, 2);
                                if spec_contains_link(&old_right, x) {
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| spec_contains_link(link, x) implies
                                (spec_contains_link(old(link), x) || x == value)
                            by {
                                reveal_with_fuel(spec_contains_link, 2);
                                if spec_contains_link(&node.right, x) {
                                    if !spec_contains_link(&old_right, x) {
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
                            reveal_with_fuel(spec_contains_link, 2);
                        }
                        false
                    }
                }
            }
        }
    }

    /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n) amortized, Span O(lg n) amortized
    // Veracity: NEEDED proof block
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) amortized, Span O(lg n) amortized — bst_insert + splay.
    fn insert_link(link: &mut Link<T>, value: T) -> (inserted: bool)
    {
        let v = value.clone();
        let inserted = Self::bst_insert(link, value);
        if inserted {
            if let Some(root) = link.take() {
                *link = Some(Node::<T>::splay(root, &v));
            }
        }
        // Veracity: NEEDED proof block
        inserted
    }

    /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — standard BST search.
    fn find_link<'a>(&'a self, target: &T) -> (found: Option<&'a T>)
        decreases *self,
    {
        match self {
            | None => None,
            | Some(node) => {
                match TotalOrder::cmp(target, &node.key) {
                    core::cmp::Ordering::Equal => Some(&node.key),
                    core::cmp::Ordering::Less => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!spec_contains_link(&node.right, *target)) by {
                                if spec_contains_link(&node.right, *target) {
                                    T::antisymmetric(*target, node.key);
                                }
                            };
                        }
                        node.left.find_link(target)
                    // Veracity: NEEDED proof block
                    }
                    core::cmp::Ordering::Greater => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!spec_contains_link(&node.left, *target)) by {
                                if spec_contains_link(&node.left, *target) {
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
// Veracity: NEEDED proof block

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends leftmost path.
    fn min_link(&self) -> (min: Option<&T>)
        decreases *self,
    {
        let ghost link = self;
        match self {
            | None => None,
            | Some(node) => match node.left {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] spec_contains_link(self, x) implies T::le(node.key, x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(spec_contains_link, 2);
                            if x == node.key {
                                T::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&node.right, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let min = node.left.min_link();
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(spec_contains_link, 2);
                        // Bridge from trait spec fn to free spec fn.
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] spec_contains_link(self, x) implies T::le(*min.unwrap(), x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            // Veracity: NEEDED proof block
                            reveal_with_fuel(spec_contains_link, 2);
                            // Bridge: recursive postcondition is in link_contains, connect to spec_contains_link.
                            // Veracity: NEEDED assert
                            assert(node.left.link_contains(x) == spec_contains_link(&node.left, x));
                            if spec_contains_link(&node.left, x) {
                            } else if x == node.key {
                            } else {
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&node.right, x));
                                T::transitive(*min.unwrap(), node.key, x);
                            }
                        };
                    }
                    min
                }
            },
        // Veracity: NEEDED proof block
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends rightmost path.
    fn max_link(&self) -> (max: Option<&T>)
        decreases *self,
    {
        let ghost link = self;
        match self {
            | None => None,
            | Some(node) => match node.right {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] spec_contains_link(self, x) implies T::le(x, node.key) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(spec_contains_link, 2);
                            if x == node.key {
                                T::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&node.left, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let max = node.right.max_link();
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(spec_contains_link, 2);
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] spec_contains_link(self, x) implies T::le(x, *max.unwrap()) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(spec_contains_link, 2);
                            // Veracity: NEEDED assert
                            assert(node.right.link_contains(x) == spec_contains_link(&node.right, x));
                            if spec_contains_link(&node.right, x) {
                            } else if x == node.key {
                            } else {
                                // Veracity: NEEDED assert
                                assert(spec_contains_link(&node.left, x));
                                T::transitive(x, node.key, *max.unwrap());
                            }
                        };
                    }
                    max
                }
            },
        }
    }

    /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — visits every node.
    // veracity: no_requires
    fn in_order_collect(&self, out: &mut Vec<T>)
        decreases *self,
    {
        if let Some(node) = self {
            node.left.in_order_collect(out);
            out.push(node.key.clone());
            node.right.in_order_collect(out);
        }
    }

    /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — visits every node.
    // veracity: no_requires
    fn pre_order_collect(&self, out: &mut Vec<T>)
        decreases *self,
    {
        if let Some(node) = self {
            out.push(node.key.clone());
            node.left.pre_order_collect(out);
            node.right.pre_order_collect(out);
        }
    }

    } // impl BSTSplayLinkFns for Link


    impl<T: TotalOrder + Clone> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {
        open spec fn spec_size(self) -> nat { spec_size_link(&self.root) }
        open spec fn spec_height(self) -> nat { spec_height_link(&self.root) }
        open spec fn spec_contains(self, value: T) -> bool { spec_contains_link(&self.root, value) }
        open spec fn spec_bstsplaysteph_wf(&self) -> bool { spec_is_bst_link(&self.root) }
        open spec fn spec_in_order(self) -> Seq<T> { spec_in_order_link(&self.root) }
        open spec fn spec_pre_order(self) -> Seq<T> { spec_pre_order_link(&self.root) }

// Veracity: UNNEEDED proof block         /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        // Veracity: NEEDED proof block
        fn new() -> (tree: Self) { BSTSplayStEph { root: None } }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — cached size field.
        fn size(&self) -> (n: usize) { self.root.size_link() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — compares cached size.
        fn is_empty(&self) -> (b: bool) { self.size() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive tree traversal.
        fn height(&self) -> (h: usize) {
            self.root.height_link()
        }

// Veracity: UNNEEDED proof block         /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n) amortized, Span O(lg n) amortized
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) amortized, Span O(lg n) amortized — agrees with APAS.
        // Veracity: NEEDED proof block
        fn insert(&mut self, value: T) { <Link<T> as BSTSplayLinkFns<T>>::insert_link(&mut self.root, value); }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn find(&self, target: &T) -> (found: Option<&T>) { self.root.find_link(target) }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — delegates to find.
        fn contains(&self, target: &T) -> (found: bool) { self.find(target).is_some() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends leftmost path.
        fn minimum(&self) -> (min: Option<&T>) {
            proof { reveal(spec_size_link); }
            let min = self.root.min_link();
            proof {
                // Bridge: min_link ensures uses link_contains; trait ensures uses spec_contains.
                if min.is_some() {
                    // Veracity: NEEDED assert
                    assert forall|x: T| self.spec_contains(x) implies T::le(*min.unwrap(), x) by {
                        // Veracity: NEEDED assert
                        assert(self.root.link_contains(x) == spec_contains_link(&self.root, x));
                    };
                }
            }
            min
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends rightmost path.
        fn maximum(&self) -> (max: Option<&T>) {
            proof { reveal(spec_size_link); }
            let max = self.root.max_link();
            proof {
                if max.is_some() {
                    // Veracity: NEEDED assert
                    assert forall|x: T| self.spec_contains(x) implies T::le(x, *max.unwrap()) by {
                        // Veracity: NEEDED assert
                        assert(self.root.link_contains(x) == spec_contains_link(&self.root, x));
                    };
                }
            }
            max
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — in-order traversal.
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            self.root.in_order_collect(&mut out);
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — pre-order traversal.
        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            self.root.pre_order_collect(&mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    //		Section 10b. iterators — BSTSplayStEph

    /// Snapshot iterator over BSTSplayStEph — collects elements via in_order traversal,
    /// then yields owned T values from the captured Vec.
    #[verifier::reject_recursive_types(T)]
    pub struct BSTSplayStEphIter<T: TotalOrder + Clone> {
        pub inner: IntoIter<T>,
    }

    impl<T: TotalOrder + Clone> View for BSTSplayStEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant_bstsplaysteph<T: TotalOrder + Clone>(it: &BSTSplayStEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: TotalOrder + Clone> std::iter::Iterator for BSTSplayStEphIter<T> {
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
            self.inner.next()
        }
    }

    /// Ghost iterator for for-loop support over BSTSplayStEphIter.
    #[verifier::reject_recursive_types(T)]
    pub struct BSTSplayStEphGhostIterator<T: TotalOrder + Clone> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: TotalOrder + Clone> View for BSTSplayStEphGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: TotalOrder + Clone> vstd::pervasive::ForLoopGhostIteratorNew for BSTSplayStEphIter<T> {
        type GhostIter = BSTSplayStEphGhostIterator<T>;
        open spec fn ghost_iter(&self) -> BSTSplayStEphGhostIterator<T> {
            BSTSplayStEphGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: TotalOrder + Clone> vstd::pervasive::ForLoopGhostIterator for BSTSplayStEphGhostIterator<T> {
        type ExecIter = BSTSplayStEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSplayStEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &BSTSplayStEphIter<T>) -> BSTSplayStEphGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: TotalOrder + Clone> std::iter::IntoIterator for &'a BSTSplayStEph<T> {
        type Item = T;
        type IntoIter = BSTSplayStEphIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_bstsplaysteph_wf()
            ensures
                it@.0 == 0,
                it@.1.len() == self.spec_in_order().len(),
                iter_invariant_bstsplaysteph(&it),
        {
            let in_ord = self.in_order();
            BSTSplayStEphIter { inner: in_ord.seq.into_iter() }
        }
    }

    //		Section 12a. derive impls in verus!


    impl<T: TotalOrder + Clone> Clone for Node<T> {
        fn clone(&self) -> (copy: Self)
            ensures true,
            decreases *self,
        {
            let left = match &self.left {
                None => None,
                Some(boxed) => Some(Box::new((&**boxed).clone())),
            };
            let right = match &self.right {
                None => None,
                Some(boxed) => Some(Box::new((&**boxed).clone())),
            };
            Node {
                key: self.key.clone(),
                size: self.size,
                left,
                right,
            }
        }
    }

    //		Section 12b. derive impls in verus!


    impl<T: TotalOrder + Clone> Default for BSTSplayStEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: TotalOrder + Clone> Clone for BSTSplayStEph<T> {
        fn clone(&self) -> (copy: Self)
            ensures true,
        {
            BSTSplayStEph { root: self.root.clone() }
        }
    }

    }

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTSplayStEphLit {
        () => {
            < $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEph<_> as $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEph<_> as $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    //		Section 14a. derive impls outside verus!

    impl<T: TotalOrder + Clone + fmt::Debug> fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T: TotalOrder + Clone + fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: TotalOrder + Clone + fmt::Debug> fmt::Debug for BSTSplayStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSplayStEph").field("root", &self.root).finish()
        }
    }

    impl<T: TotalOrder + Clone> fmt::Display for BSTSplayStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSplayStEph(size={})", self.size())
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<T: TotalOrder + Clone + fmt::Debug> fmt::Debug for BSTSplayStEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSplayStEphIter")
        }
    }

    impl<T: TotalOrder + Clone + fmt::Display> fmt::Display for BSTSplayStEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSplayStEphIter")
        }
    }

    impl<T: TotalOrder + Clone + fmt::Debug> fmt::Debug for BSTSplayStEphGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSplayStEphGhostIterator")
        }
    }

    impl<T: TotalOrder + Clone + fmt::Display> fmt::Display for BSTSplayStEphGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSplayStEphGhostIterator")
        }
    }
}
