//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Ephemeral Treap (randomized heap-ordered BST) with interior locking for multi-threaded access.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 5c. view impls
//	Section 6c. spec fns
//	Section 7c. proof fns/broadcast groups
//	Section 8c. traits
//	Section 9c. impls
//	Section 4d. type definitions
//	Section 9d. impls
//	Section 11b. top level coarse locking
//	Section 12a. derive impls in verus!
//	Section 12c. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!

//		Section 1. module


pub mod BSTTreapMtEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdIs;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdSpec;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::IsLtTransitive;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use vstd::set::group_set_axioms;

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StTInMtT + Ord> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    //		Section 4b. type definitions


    pub struct BSTTreapMtEphInv;

    //		Section 9b. impls


    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> (sz: usize)
        requires Lnk::spec_link_size_wf(link),
        ensures sz as nat == Lnk::spec_size_link(link),
    {
        match link {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn update<T: StTInMtT + Ord>(node: &mut Node<T>)
        requires
            Lnk::spec_link_size_wf(&old(node).left),
            Lnk::spec_link_size_wf(&old(node).right),
            1 + Lnk::spec_size_link(&old(node).left) + Lnk::spec_size_link(&old(node).right) <= usize::MAX as nat,
        ensures
            node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right),
            node.key == old(node).key,
            node.left == old(node).left,
            node.right == old(node).right,
    {
        let l = size_link(&node.left);
        let r = size_link(&node.right);
        node.size = 1 + l + r;
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_left<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>)
        requires
            Lnk::spec_link_size_wf(old(link)),
            Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(link, k) <==> spec_contains_link(old(link), k),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
    {
        if let Some(mut x) = link.take() {
            let ghost bst_input = Lnk::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_right = x.right;
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&x.left));
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.right.take() {
                let ghost yk = y.key;
                let ghost b  = y.left;
                let ghost a  = y.right;
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&y.left));
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&y.right));
                let ghost x_left_sz = Lnk::spec_size_link(&x.left);
                let ghost y_left_sz = Lnk::spec_size_link(&y.left);
                let ghost y_right_sz = Lnk::spec_size_link(&y.right);
                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        lemma_bst_decompose(&orig_right);
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies xk.is_lt(&k) by {
                            lemma_contains_left(&y, k);
                        };
                    }
                }
                x.right = y.left.take();
                update(&mut x);
                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        // Veracity: NEEDED assert
                        assert(Lnk::spec_bst_link(&Some(x)));
                    }
                }
                y.left = Some(x);
                update(&mut y);
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_assemble_node(&*y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        lemma_bst_decompose(&orig_right);
                        lemma_contains_root(&y);
                        // Veracity: NEEDED assert
                        assert(spec_contains_link(&orig_right, yk));
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&y.left, k) implies k.is_lt(&yk) by {
                            if spec_contains_link(&x.left, k) {
                                T::is_lt_transitive(k, xk, yk);
                            }
                            if spec_contains_link(&x.right, k) {
                            }
                        };
                    }
                }
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_right<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>)
        requires
            Lnk::spec_link_size_wf(old(link)),
            Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(link, k) <==> spec_contains_link(old(link), k),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
    {
        if let Some(mut x) = link.take() {
            let ghost bst_input = Lnk::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_left = x.left;
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&x.left));
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.left.take() {
                let ghost yk = y.key;
                let ghost b  = y.right;
                let ghost a  = y.left;
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&y.left));
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&y.right));
                let ghost x_right_sz = Lnk::spec_size_link(&x.right);
                let ghost y_left_sz = Lnk::spec_size_link(&y.left);
                let ghost y_right_sz = Lnk::spec_size_link(&y.right);
                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        lemma_bst_decompose(&orig_left);
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies k.is_lt(&xk) by {
                            lemma_contains_right(&y, k);
                        };
                    }
                }
                x.left = y.right.take();
                update(&mut x);
                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        // Veracity: NEEDED assert
                        assert(Lnk::spec_bst_link(&Some(x)));
                    }
                }
                y.right = Some(x);
                update(&mut y);
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_assemble_node(&*y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        lemma_bst_decompose(&orig_left);
                        lemma_contains_root(&y);
                        // Veracity: NEEDED assert
                        assert(spec_contains_link(&orig_left, yk));
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&y.right, k) implies yk.is_lt(&k) by {
                            if spec_contains_link(&x.right, k) {
                                T::is_lt_transitive(yk, xk, k);
                            }
                            if spec_contains_link(&x.left, k) {
                            }
                        };
                    }
                }
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected
    fn insert_link<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>, value: T, priority: u64)
        requires
            Lnk::spec_size_link(old(link)) + 1 <= usize::MAX as nat,
            Lnk::spec_link_size_wf(old(link)),
            T::obeys_partial_cmp_spec(),
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) <= Lnk::spec_size_link(old(link)) + 1,
            Lnk::spec_size_link(link) >= Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(old(link), k) ==> spec_contains_link(link, k),
            forall|k: T| spec_contains_link(link, k) ==> (spec_contains_link(old(link), k) || k == value),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
        decreases old(link),
    {
        // Veracity: NEEDED proof block
        proof { reveal_with_fuel(spec_contains_link, 3); }
        if let Some(mut node) = link.take() {
            let ghost orig_key = node.key;
            let ghost orig_left = node.left;
            let ghost orig_right = node.right;
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall |k: T|
                    #[trigger] spec_contains_link(old(link), k) <==>
                    (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                    by {};
            }
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&node.left));
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&node.right));
            if value < node.key {
                insert_link(&mut node.left, value, priority);
                update(&mut node);
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_assemble_node(&*node);
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                    }
                }
                *link = Some(node);
                let need_rotate_right = match link.as_ref().unwrap().left.as_ref() {
                    Some(left) => left.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate_right {
                    rotate_right(link);
                }
            } else if node.key < value {
                insert_link(&mut node.right, value, priority);
                update(&mut node);
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_assemble_node(&*node);
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                    }
                }
                *link = Some(node);
                let need_rotate_left = match link.as_ref().unwrap().right.as_ref() {
                    Some(right) => right.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate_left {
                    rotate_left(link);
                }
            } else {
                *link = Some(node);
            }
        } else {
            let n = Box::new(Node { key: value, priority, size: 1, left: None, right: None });
            // Veracity: NEEDED proof block
            proof { lemma_wf_assemble_node(&*n); }
            *link = Some(n);
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected
    fn delete_link<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>, target: &T)
        requires
            Lnk::spec_link_size_wf(old(link)),
            Lnk::spec_size_link(old(link)) < usize::MAX as nat,
            T::obeys_partial_cmp_spec(),
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) <= Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(link, k) ==> spec_contains_link(old(link), k),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
        decreases Lnk::spec_size_link(old(link)),
    {
        // Veracity: NEEDED proof block
        proof { reveal_with_fuel(spec_contains_link, 3); }
        if let Some(mut node) = link.take() {
            let ghost orig_key = node.key;
            let ghost orig_left = node.left;
            let ghost orig_right = node.right;
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall |k: T|
                    #[trigger] spec_contains_link(old(link), k) <==>
                    (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                    by {};
            }
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&node.left));
            // Veracity: NEEDED assert
            assert(Lnk::spec_link_size_wf(&node.right));
            if *target < node.key {
                delete_link(&mut node.left, target);
                update(&mut node);
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_assemble_node(&*node);
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                    }
                }
                *link = Some(node);
            } else if node.key < *target {
                delete_link(&mut node.right, target);
                update(&mut node);
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_assemble_node(&*node);
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                    }
                }
                *link = Some(node);
            } else {
                // Found the target.
                if node.left.is_none() && node.right.is_none() {
                    // Leaf: remove (link stays None from take).
                } else if node.right.is_none() {
                    // Veracity: NEEDED proof block
                    proof {
                        if Lnk::spec_bst_link(old(link)) {
                            lemma_bst_decompose(old(link));
                        }
                    }
                    *link = node.left.take();
                } else if node.left.is_none() {
                    // Veracity: NEEDED proof block
                    proof {
                        if Lnk::spec_bst_link(old(link)) {
                            lemma_bst_decompose(old(link));
                        }
                    }
                    *link = node.right.take();
                } else {
                    // Two children: rotate smaller-priority child up, then recurse.
                    let left_pri = node.left.as_ref().unwrap().priority;
                    let right_pri = node.right.as_ref().unwrap().priority;
                    *link = Some(node);
                    if left_pri <= right_pri {
                        rotate_right(link);
                        if let Some(mut rotated) = link.take() {
                            let ghost rot_key = rotated.key;
                            let ghost rot_left = rotated.left;
                            let ghost rot_right = rotated.right;
                            // Veracity: NEEDED proof block
                            proof {
                                // Veracity: NEEDED assert
                                assert forall |k: T|
                                    #[trigger] spec_contains_link(old(link), k) <==>
                                    (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                    by {};
                                if Lnk::spec_bst_link(old(link)) {
                                    lemma_bst_decompose(&Some(rotated));
                                }
                            }
                            delete_link(&mut rotated.right, target);
                            update(&mut rotated);
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_wf_assemble_node(&*rotated);
                                if Lnk::spec_bst_link(old(link)) {
                                }
                            }
                            *link = Some(rotated);
                        }
                    } else {
                        rotate_left(link);
                        if let Some(mut rotated) = link.take() {
                            let ghost rot_key = rotated.key;
                            let ghost rot_left = rotated.left;
                            let ghost rot_right = rotated.right;
                            // Veracity: NEEDED proof block
                            proof {
                                // Veracity: NEEDED assert
                                assert forall |k: T|
                                    #[trigger] spec_contains_link(old(link), k) <==>
                                    (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                    by {};
                                if Lnk::spec_bst_link(old(link)) {
                                    lemma_bst_decompose(&Some(rotated));
                                }
                            }
                            delete_link(&mut rotated.left, target);
                            update(&mut rotated);
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_wf_assemble_node(&*rotated);
                                if Lnk::spec_bst_link(old(link)) {
                                }
                            }
                            *link = Some(rotated);
                        }
                    }
                }
            }
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected
    fn find_link<'a, T: StTInMtT + Ord + IsLtTransitive>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
        requires
            Lnk::spec_bst_link(link),
            T::obeys_partial_cmp_spec(),
        ensures
            found.is_some() <==> spec_contains_link(link, *target),
            found.is_some() ==> *found.unwrap() == *target,
        decreases *link,
    {
        // Veracity: NEEDED proof block
        proof { reveal_with_fuel(spec_contains_link, 2); }
        match link {
            | None => None,
            | Some(node) => {
                // Veracity: NEEDED proof block
                proof { lemma_bst_decompose(link); }
                if *target < node.key {
                    let r = find_link(&node.left, target);
                    // Veracity: NEEDED proof block
                    proof {
                        if r.is_some() {
                            lemma_contains_left(node, *target);
                        }
                        T::is_lt_irreflexive(*target);
                        if spec_contains_link(link, *target) {
                            if spec_contains_link(&node.right, *target) {
                                T::is_lt_transitive(*target, node.key, *target);
                            }
                        }
                    }
                    r
                } else if node.key < *target {
                    let r = find_link(&node.right, target);
                    // Veracity: NEEDED proof block
                    proof {
                        if r.is_some() {
                            lemma_contains_right(node, *target);
                        }
                        T::is_lt_irreflexive(*target);
                        if spec_contains_link(link, *target) {
                            if spec_contains_link(&node.left, *target) {
                                T::is_lt_transitive(*target, node.key, *target);
                            }
                        }
                    }
                    r
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        T::is_lt_antisymmetric(*target, node.key);
                        lemma_contains_root(node);
                    }
                    Some(&node.key)
                }
            }
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected
    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> (min_val: Option<&T>)
        requires Lnk::spec_bst_link(link),
        ensures
            min_val.is_some() ==> spec_contains_link(link, *min_val.unwrap()),
            match (min_val, Lnk::spec_min_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            },
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => {
                    // Veracity: NEEDED proof block
                    proof { lemma_contains_root(node); }
                    Some(&node.key)
                },
                | Some(_) => {
                    let r = min_link(&node.left);
                    // Veracity: NEEDED proof block
                    proof {
                        if r.is_some() { lemma_contains_left(node, *r.unwrap()); }
                    }
                    r
                },
            },
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected
    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> (max_val: Option<&T>)
        requires Lnk::spec_bst_link(link),
        ensures
            max_val.is_some() ==> spec_contains_link(link, *max_val.unwrap()),
            match (max_val, Lnk::spec_max_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            },
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => {
                    // Veracity: NEEDED proof block
                    proof { lemma_contains_root(node); }
                    Some(&node.key)
                },
                | Some(_) => {
                    let r = max_link(&node.right);
                    // Veracity: NEEDED proof block
                    proof {
                        if r.is_some() { lemma_contains_right(node, *r.unwrap()); }
                    }
                    r
                },
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn height_link<T: StTInMtT + Ord>(link: &Link<T>) -> (h: usize)
        requires
            Lnk::spec_size_link(link) < usize::MAX as nat,
            Lnk::spec_link_size_wf(link),
        ensures h as nat == Lnk::spec_height_link(link),
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => {
                // Veracity: NEEDED proof block
                proof { lemma_size_wf_child_bounded(link); }
                let lh = height_link(&node.left);
                let rh = height_link(&node.right);
                let m = if lh >= rh { lh } else { rh };
                // Veracity: NEEDED proof block
                proof {
                    lemma_height_le_size(&node.left);
                    lemma_height_le_size(&node.right);
                }
                1 + m
            }
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn in_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        requires Lnk::spec_link_size_wf(link),
        ensures out@.len() == old(out)@.len() + Lnk::spec_size_link(link) as int,
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        requires Lnk::spec_link_size_wf(link),
        ensures out@.len() == old(out)@.len() + Lnk::spec_size_link(link) as int,
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTTreapMtEph<T: StTInMtT + Ord + IsLtTransitive> {
        pub(crate) locked_root: RwLock<Link<T>, BSTTreapMtEphInv>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;

    //		Section 5c. view impls


    impl<T: StTInMtT + Ord + IsLtTransitive> View for BSTTreapMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_ghost_locked_root() }
    }

    //		Section 6c. spec fns


    pub open spec fn spec_bsttreapmteph_link_wf<T: StTInMtT + Ord + IsLtTransitive>(link: &Link<T>) -> bool {
        Lnk::spec_link_size_wf(link)
        && Lnk::spec_size_link(link) < usize::MAX as nat
        && Lnk::spec_bst_link(link)
    }

    // Free spec fn: reveal_with_fuel cannot reference trait methods with generic params.
    pub open spec fn spec_contains_link<T: StTInMtT + Ord>(link: &Link<T>, val: T) -> bool
        decreases *link,
    {
        match link {
            None => false,
            Some(node) => {
                node.key == val
                    || spec_contains_link(&node.left, val)
                    || spec_contains_link(&node.right, val)
            }
        }
    }

    /// Maps a Link<T> to the Set of views it contains.
    pub open spec fn spec_set_of_link<T: StTInMtT + Ord>(link: &Link<T>) -> Set<<T as View>::V>
        decreases *link,
    {
        match link {
            None => Set::empty(),
            Some(node) => {
                spec_set_of_link(&node.left)
                    .union(spec_set_of_link(&node.right))
                    .insert(node.key@)
            }
        }
    }

    //		Section 7c. proof fns/broadcast groups


    proof fn lemma_bst_decompose<T: StTInMtT + Ord>(link: &Link<T>)
        requires Lnk::spec_bst_link(link),
        ensures match link {
            None => true,
            Some(node) => {
                Lnk::spec_bst_link(&node.left)
                && Lnk::spec_bst_link(&node.right)
                && (forall|k: T| #[trigger] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                && (forall|k: T| #[trigger] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
            },
        },
    {
    }

    proof fn lemma_contains_left<T: StTInMtT + Ord>(node: &Box<Node<T>>, k: T)
        requires spec_contains_link(&node.left, k),
        ensures spec_contains_link(&Some(*node), k),
    {
    }

    proof fn lemma_contains_right<T: StTInMtT + Ord>(node: &Box<Node<T>>, k: T)
        requires spec_contains_link(&node.right, k),
        ensures spec_contains_link(&Some(*node), k),
    {
    }

    proof fn lemma_contains_root<T: StTInMtT + Ord>(node: &Box<Node<T>>)
        ensures spec_contains_link(&Some(*node), node.key),
    {
    }

    /// Forward direction: structural containment implies set membership.
    proof fn lemma_contains_implies_in_set<T: StTInMtT + Ord>(link: &Link<T>, val: T)
        requires spec_contains_link(link, val),
        ensures spec_set_of_link(link).contains(val@),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                if node.key == val {
                    // val@ == node.key@, and node.key@ is inserted into the set.
                } else if spec_contains_link(&node.left, val) {
                    lemma_contains_implies_in_set(&node.left, val);
                } else {
                    lemma_contains_implies_in_set(&node.right, val);
                }
            }
        }
    }

    proof fn lemma_set_of_link_finite<T: StTInMtT + Ord>(link: &Link<T>)
        ensures spec_set_of_link(link).finite(),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_set_of_link_finite(&node.left);
                lemma_set_of_link_finite(&node.right);
            }
        }
    }

    proof fn lemma_height_le_size<T: StTInMtT + Ord>(link: &Link<T>)
        requires
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) < usize::MAX as nat,
        ensures Lnk::spec_height_link(link) <= Lnk::spec_size_link(link),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_size_wf_child_bounded(link);
                lemma_height_le_size(&node.left);
                lemma_height_le_size(&node.right);
            }
        }
    }

    proof fn lemma_size_wf_child_bounded<T: StTInMtT + Ord>(link: &Link<T>)
        requires
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) > 0,
            Lnk::spec_size_link(link) < usize::MAX as nat,
        ensures
            match link {
                None => true,
                Some(node) => {
                    Lnk::spec_size_link(&node.left) < usize::MAX as nat
                    && Lnk::spec_size_link(&node.right) < usize::MAX as nat
                },
            },
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
            }
        }
    }

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    proof fn lemma_wf_assemble_node<T: StTInMtT + Ord>(node: &Node<T>)
        requires
            node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right),
            Lnk::spec_link_size_wf(&node.left),
            Lnk::spec_link_size_wf(&node.right),
        ensures Lnk::spec_link_size_wf(&Some(Box::new(*node))),
    {
    }

    //		Section 8c. traits


    pub trait LinkTrait<T: StTInMtT + Ord>: Sized {
        spec fn spec_size_link(link: &Link<T>) -> nat;
        spec fn spec_link_size_wf(link: &Link<T>) -> bool;
        spec fn spec_height_link(link: &Link<T>) -> nat;
        spec fn spec_bst_link(link: &Link<T>) -> bool;
        spec fn spec_in_order_link(link: &Link<T>) -> Seq<T>;
        spec fn spec_pre_order_link(link: &Link<T>) -> Seq<T>;
        spec fn spec_min_link(link: &Link<T>) -> Option<T>;
        spec fn spec_max_link(link: &Link<T>) -> Option<T>;
    }

    /// Treap trait for multi-threaded ephemeral access.
    ///
    /// The RwLock invariant (`BSTTreapMtEphInv`) enforces `spec_bsttreapmteph_link_wf` on the link
    /// (size well-formedness, size < MAX, and BST ordering) on every acquire/release.
    /// Read-only methods have Set-based specs via external_body View.
    /// Interior mutability via RwLock precludes `old()` specs on insert/delete.
    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord + IsLtTransitive>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_bsttreapmteph_wf(&self) -> bool;
        spec fn spec_size(self) -> nat;
        spec fn spec_contains(self, target: T) -> bool;

        proof fn lemma_bst_decompose(link: &Link<T>)
            requires Lnk::spec_bst_link(link),
            ensures match link {
                None => true,
                Some(node) => {
                    Lnk::spec_bst_link(&node.left)
                    && Lnk::spec_bst_link(&node.right)
                    && (forall|k: T| #[trigger] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                    && (forall|k: T| #[trigger] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
                },
            };

        proof fn lemma_contains_left(node: &Box<Node<T>>, k: T)
            requires spec_contains_link(&node.left, k),
            ensures spec_contains_link(&Some(*node), k);

        proof fn lemma_contains_right(node: &Box<Node<T>>, k: T)
            requires spec_contains_link(&node.right, k),
            ensures spec_contains_link(&Some(*node), k);

        proof fn lemma_contains_root(node: &Box<Node<T>>)
            ensures spec_contains_link(&Some(*node), node.key);

        proof fn lemma_height_le_size(link: &Link<T>)
            requires
                Lnk::spec_link_size_wf(link),
                Lnk::spec_size_link(link) < usize::MAX as nat,
            ensures Lnk::spec_height_link(link) <= Lnk::spec_size_link(link);

        proof fn lemma_size_wf_child_bounded(link: &Link<T>)
            requires
                Lnk::spec_link_size_wf(link),
                Lnk::spec_size_link(link) > 0,
                Lnk::spec_size_link(link) < usize::MAX as nat,
            ensures
                match link {
                    None => true,
                    Some(node) => {
                        Lnk::spec_size_link(&node.left) < usize::MAX as nat
                        && Lnk::spec_size_link(&node.right) < usize::MAX as nat
                    },
                };

        proof fn lemma_wf_decompose(link: &Link<T>)
            requires Lnk::spec_link_size_wf(link),
            ensures match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right)
                    && Lnk::spec_link_size_wf(&node.left)
                    && Lnk::spec_link_size_wf(&node.right)
                },
            };

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        proof fn lemma_wf_assemble_node(node: &Node<T>)
            requires
                node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right),
                Lnk::spec_link_size_wf(&node.left),
                Lnk::spec_link_size_wf(&node.right),
            ensures Lnk::spec_link_size_wf(&Some(Box::new(*node)));

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(link: &Link<T>) -> (sz: usize)
            requires Lnk::spec_link_size_wf(link),
            ensures sz as nat == Lnk::spec_size_link(link);

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            requires
                Lnk::spec_bst_link(link),
                T::obeys_partial_cmp_spec(),
            ensures
                found.is_some() <==> spec_contains_link(link, *target),
                found.is_some() ==> *found.unwrap() == *target;

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn min_link(link: &Link<T>) -> (min_val: Option<&T>)
            requires Lnk::spec_bst_link(link),
            ensures
                min_val.is_some() ==> spec_contains_link(link, *min_val.unwrap()),
                match (min_val, Lnk::spec_min_link(link)) {
                    (Some(rv), Some(sv)) => *rv == sv,
                    (None, None) => true,
                    _ => false,
                };

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn max_link(link: &Link<T>) -> (max_val: Option<&T>)
            requires Lnk::spec_bst_link(link),
            ensures
                max_val.is_some() ==> spec_contains_link(link, *max_val.unwrap()),
                match (max_val, Lnk::spec_max_link(link)) {
                    (Some(rv), Some(sv)) => *rv == sv,
                    (None, None) => true,
                    _ => false,
                };

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_link(link: &Link<T>) -> (h: usize)
            requires
                Lnk::spec_size_link(link) < usize::MAX as nat,
                Lnk::spec_link_size_wf(link),
            ensures h as nat == Lnk::spec_height_link(link);

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty_tree: Self)
            ensures empty_tree@ == Set::<<T as View>::V>::empty(), empty_tree.spec_bsttreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires
                old(self).spec_bsttreapmteph_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                self@ =~= old(self)@.insert(value@),
                self.spec_bsttreapmteph_wf(),
                self.spec_contains(value),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn delete(&mut self, target: &T)
            requires
                old(self).spec_bsttreapmteph_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                self@ =~= old(self)@.remove(target@),
                self.spec_bsttreapmteph_wf(),
                self.spec_size() <= old(self).spec_size(),
                forall|k: T| self@.contains(k@) ==> #[trigger] old(self)@.contains(k@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn find(&self, target: &T) -> (found: Option<T>)
            requires
                self.spec_bsttreapmteph_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                found.is_some() <==> self@.contains(target@),
                found.is_some() ==> found.unwrap()@ == target@;
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bsttreapmteph_wf(),
                T::obeys_partial_cmp_spec(),
            ensures found <==> self@.contains(target@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires self.spec_bsttreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn minimum(&self) -> (min_val: Option<T>)
            ensures min_val.is_some() ==> self@.contains(min_val.unwrap()@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n) expected, Span O(lg n) expected
        fn maximum(&self) -> (max_val: Option<T>)
            ensures max_val.is_some() ==> self@.contains(max_val.unwrap()@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            ensures ordered@.len() == self@.len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (preordered: ArraySeqStPerS<T>)
            ensures preordered@.len() == self@.len();
    }

    //		Section 9c. impls


    impl<T: StTInMtT + Ord + IsLtTransitive> BSTTreapMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_root@.finite()
        }

        pub closed spec fn spec_ghost_locked_root(self) -> Set<<T as View>::V> {
            self.ghost_locked_root@
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {
        open spec fn spec_bsttreapmteph_wf(&self) -> bool {
            self@.finite()
        }

        open spec fn spec_size(self) -> nat {
            self@.len()
        }

        open spec fn spec_contains(self, target: T) -> bool {
            self@.contains(target@)
        }

        proof fn lemma_bst_decompose(link: &Link<T>) {
        }

        proof fn lemma_contains_left(node: &Box<Node<T>>, k: T) {
        }

        proof fn lemma_contains_right(node: &Box<Node<T>>, k: T) {
        }

        proof fn lemma_contains_root(node: &Box<Node<T>>) {
        }

        proof fn lemma_height_le_size(link: &Link<T>)
            decreases *link,
        {
            match link {
                None => {},
                Some(node) => {
                    lemma_size_wf_child_bounded(link);
                    lemma_height_le_size(&node.left);
                    lemma_height_le_size(&node.right);
                }
            }
        }

        proof fn lemma_size_wf_child_bounded(link: &Link<T>)
            decreases *link,
        {
            match link {
                None => {},
                Some(node) => {
                }
            }
        }

        proof fn lemma_wf_decompose(link: &Link<T>) {
        }

        proof fn lemma_wf_assemble_node(node: &Node<T>) {
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size_link(link: &Link<T>) -> (sz: usize) {
            size_link(link)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            decreases *link,
        {
            find_link(link, target)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn min_link(link: &Link<T>) -> (min_val: Option<&T>)
            decreases *link,
        {
            min_link(link)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn max_link(link: &Link<T>) -> (max_val: Option<&T>)
            decreases *link,
        {
            max_link(link)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_link(link: &Link<T>) -> (h: usize)
            decreases *link,
        {
            height_link(link)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty_tree: Self)
            ensures empty_tree@ == Set::<<T as View>::V>::empty(), empty_tree.spec_bsttreapmteph_wf()
        {
            BSTTreapMtEph {
                locked_root: RwLock::new(None, Ghost(BSTTreapMtEphInv)),
                ghost_locked_root: Ghost(Set::<<T as View>::V>::empty()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn insert(&mut self, value: T, priority: u64)
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(&*self); }
            let ghost value_view = value@;
            let ghost old_set = self.ghost_locked_root@;
            let (mut current, write_handle) = self.locked_root.acquire_write();
            let sz = size_link(&current);
            if sz + 1 < usize::MAX {
                insert_link(&mut current, value, priority);
            }
            write_handle.release_write(current);
            self.ghost_locked_root = Ghost(old_set.insert(value_view));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn delete(&mut self, target: &T)
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(&*self); }
            let ghost target_view = target@;
            let ghost old_set = self.ghost_locked_root@;
            let (mut current, write_handle) = self.locked_root.acquire_write();
            delete_link(&mut current, target);
            write_handle.release_write(current);
            self.ghost_locked_root = Ghost(old_set.remove(target_view));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn find(&self, target: &T) -> (found: Option<T>)
        {
            let handle = self.locked_root.acquire_read();
            let result = find_link(handle.borrow(), target).cloned();
            handle.release_read();
            // Veracity: NEEDED proof block
            proof {
                assume(result.is_some() <==> self@.contains(target@));
                accept(result.is_some() ==> result.unwrap()@ == target@);
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn contains(&self, target: &T) -> (found: bool)
        {
            self.find(target).is_some()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite()
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(&*self); }
            let handle = self.locked_root.acquire_read();
            let result = size_link(handle.borrow());
            handle.release_read();
            // Veracity: NEEDED proof block
            proof { assume(result as nat == self@.len()); }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite()
        {
            self.size() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
        {
            let handle = self.locked_root.acquire_read();
            let link: &Link<T> = handle.borrow();
            let result = height_link(link);
            handle.release_read();
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn minimum(&self) -> (min_val: Option<T>)
            ensures min_val.is_some() ==> self@.contains(min_val.unwrap()@)
        {
            let handle = self.locked_root.acquire_read();
            let result = min_link(handle.borrow()).cloned();
            handle.release_read();
            // Veracity: NEEDED proof block
            proof { assume(result.is_some() ==> self@.contains(result.unwrap()@)); }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn maximum(&self) -> (max_val: Option<T>)
            ensures max_val.is_some() ==> self@.contains(max_val.unwrap()@)
        {
            let handle = self.locked_root.acquire_read();
            let result = max_link(handle.borrow()).cloned();
            handle.release_read();
            // Veracity: NEEDED proof block
            proof { assume(result.is_some() ==> self@.contains(result.unwrap()@)); }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            ensures ordered@.len() == self@.len()
        {
            let handle = self.locked_root.acquire_read();
            let mut out = Vec::with_capacity(size_link(handle.borrow()));
            in_order_collect(handle.borrow(), &mut out);
            handle.release_read();
            let ordered = ArraySeqStPerS::from_vec(out);
            // Veracity: NEEDED proof block
            proof { assume(ordered@.len() == self@.len()); }
            ordered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (preordered: ArraySeqStPerS<T>)
            ensures preordered@.len() == self@.len()
        {
            let handle = self.locked_root.acquire_read();
            let mut out = Vec::with_capacity(size_link(handle.borrow()));
            pre_order_collect(handle.borrow(), &mut out);
            handle.release_read();
            let preordered = ArraySeqStPerS::from_vec(out);
            // Veracity: NEEDED proof block
            proof { assume(preordered@.len() == self@.len()); }
            preordered
        }
    }

    //		Section 4d. type definitions


    pub struct Lnk;

    //		Section 9d. impls


    impl<T: StTInMtT + Ord> LinkTrait<T> for Lnk {
        open spec fn spec_size_link(link: &Link<T>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(node) => node.size as nat,
            }
        }

        open spec fn spec_link_size_wf(link: &Link<T>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right)
                        && Self::spec_link_size_wf(&node.left)
                        && Self::spec_link_size_wf(&node.right)
                }
            }
        }

        open spec fn spec_height_link(link: &Link<T>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(node) => {
                    let lh = Self::spec_height_link(&node.left);
                    let rh = Self::spec_height_link(&node.right);
                    1 + if lh >= rh { lh } else { rh }
                }
            }
        }

        open spec fn spec_bst_link(link: &Link<T>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(node) => {
                    Self::spec_bst_link(&node.left)
                        && Self::spec_bst_link(&node.right)
                        && (forall|k: T| #![trigger spec_contains_link(&node.left, k)] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                        && (forall|k: T| #![trigger spec_contains_link(&node.right, k)] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
                }
            }
        }

        open spec fn spec_in_order_link(link: &Link<T>) -> Seq<T>
            decreases *link,
        {
            match link {
                None => Seq::empty(),
                Some(node) => {
                    Self::spec_in_order_link(&node.left)
                        + seq![node.key]
                        + Self::spec_in_order_link(&node.right)
                }
            }
        }

        open spec fn spec_pre_order_link(link: &Link<T>) -> Seq<T>
            decreases *link,
        {
            match link {
                None => Seq::empty(),
                Some(node) => {
                    seq![node.key]
                        + Self::spec_pre_order_link(&node.left)
                        + Self::spec_pre_order_link(&node.right)
                }
            }
        }

        open spec fn spec_min_link(link: &Link<T>) -> Option<T>
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => match node.left {
                    None => Some(node.key),
                    Some(_) => Self::spec_min_link(&node.left),
                },
            }
        }

        open spec fn spec_max_link(link: &Link<T>) -> Option<T>
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => match node.right {
                    None => Some(node.key),
                    Some(_) => Self::spec_max_link(&node.right),
                },
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn clone_link<T: StTInMtT + Ord + Clone>(link: &Link<T>) -> (c: Link<T>)
        requires Lnk::spec_link_size_wf(link),
        ensures
            Lnk::spec_size_link(&c) == Lnk::spec_size_link(link),
            Lnk::spec_link_size_wf(&c),
        decreases link,
    {
        match link {
            None => {
                let c = None;
                // Veracity: NEEDED proof block
                proof { assume(c == *link); }
                c
            }
            Some(node) => {
                let left = clone_link(&node.left);
                let right = clone_link(&node.right);
                let c = Some(Box::new(Node {
                    key: node.key.clone(),
                    priority: node.priority,
                    size: node.size,
                    left,
                    right,
                }));
                // Veracity: NEEDED proof block
                proof { assume(c == *link); }
                c
            }
        }
    }

    //		Section 11b. top level coarse locking


    impl<T: StTInMtT + Ord + IsLtTransitive> RwLockPredicate<Link<T>> for BSTTreapMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            spec_bsttreapmteph_link_wf(&v)
        }
    }

    //		Section 12a. derive impls in verus!


    impl<T: StTInMtT + Ord + Clone> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            // Veracity: NEEDED proof block
            proof {
                assume(Lnk::spec_link_size_wf(&self.left));
                assume(Lnk::spec_link_size_wf(&self.right));
            }
            let cloned = Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            };
            // Veracity: NEEDED proof block
            proof { assume(cloned == *self); }
            cloned
        }
    }

    //		Section 12c. derive impls in verus!


    impl<T: StTInMtT + Ord + IsLtTransitive> Default for BSTTreapMtEph<T> {
        fn default() -> (d: Self) { Self::new() }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> Clone for BSTTreapMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let handle = self.locked_root.acquire_read();
            let inner_clone = clone_link(handle.borrow());
            handle.release_read();
            // Veracity: NEEDED proof block
            proof {
                assume(spec_bsttreapmteph_link_wf(&inner_clone));
                assume(self.ghost_locked_root@.finite());
            }
            let cloned = BSTTreapMtEph {
                locked_root: RwLock::new(inner_clone, Ghost(BSTTreapMtEphInv)),
                ghost_locked_root: Ghost(self.ghost_locked_root@),
            };
            // Veracity: NEEDED proof block
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTTreapMtEphLit {
        () => {
            < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new();
            $( {
                let __val = $x;
                let mut __h = ::std::collections::hash_map::DefaultHasher::new();
                __val.hash(&mut __h);
                __tree.insert(__val, __h.finish());
            } )*
            __tree
        }};
    }

    //		Section 14a. derive impls outside verus!

    impl<T: StTInMtT + Ord + std::fmt::Debug> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("priority", &self.priority)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T: StTInMtT + Ord + std::fmt::Display> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Node(key={}, priority={}, size={})", self.key, self.priority, self.size)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl std::fmt::Debug for BSTTreapMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTTreapMtEphInv").finish()
        }
    }

    impl std::fmt::Display for BSTTreapMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTTreapMtEphInv")
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<T: StTInMtT + Ord + IsLtTransitive> std::fmt::Debug for BSTTreapMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTTreapMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> std::fmt::Display for BSTTreapMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTTreapMtEph")
        }
    }

    //		Section 14d. derive impls outside verus!

    impl std::fmt::Debug for Lnk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Lnk").finish()
        }
    }

    impl std::fmt::Display for Lnk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Lnk")
        }
    }
}
