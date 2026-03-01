//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral Treap (randomized heap-ordered BST) with `find` support.

//  Table of Contents
//	1. module
//	4. type definitions
//	6. spec fns
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod BSTTreapStEph {

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdIs;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::IsLtTransitive;

    verus! {

    //		4. type definitions

    type Link<T> = Option<Box<Node<T>>>;

    pub struct Node<T: StT + Ord + IsLtTransitive> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    pub struct BSTTreapStEph<T: StT + Ord + IsLtTransitive> {
        pub root: Link<T>,
    }

    pub type BSTreeTreap<T> = BSTTreapStEph<T>;


    //		6. spec fns

    pub open spec fn spec_size_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_contains_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>, target: T) -> bool
        decreases *link,
    {
        match link {
            None => false,
            Some(node) => {
                node.key == target
                    || spec_contains_link(&node.left, target)
                    || spec_contains_link(&node.right, target)
            }
        }
    }

    pub open spec fn spec_bst_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_bst_link(&node.left)
                    && spec_bst_link(&node.right)
                    && (forall|k: T| #![trigger spec_contains_link(&node.left, k)] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                    && (forall|k: T| #![trigger spec_contains_link(&node.right, k)] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
            }
        }
    }

    pub open spec fn spec_size_wf_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(node) => {
                node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right)
                    && spec_size_wf_link(&node.left)
                    && spec_size_wf_link(&node.right)
            }
        }
    }

    pub open spec fn spec_in_order_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> Seq<T>
        decreases *link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => {
                spec_in_order_link(&node.left)
                    + seq![node.key]
                    + spec_in_order_link(&node.right)
            }
        }
    }

    pub open spec fn spec_pre_order_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> Seq<T>
        decreases *link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => {
                seq![node.key]
                    + spec_pre_order_link(&node.left)
                    + spec_pre_order_link(&node.right)
            }
        }
    }

    pub open spec fn spec_min_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> Option<T>
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => match node.left {
                None => Some(node.key),
                Some(_) => spec_min_link(&node.left),
            },
        }
    }

    pub open spec fn spec_max_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> Option<T>
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => match node.right {
                None => Some(node.key),
                Some(_) => spec_max_link(&node.right),
            },
        }
    }

    pub open spec fn spec_height_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let lh = spec_height_link(&node.left);
                let rh = spec_height_link(&node.right);
                let m = if lh >= rh { lh } else { rh };
                1 + m
            }
        }
    }

    proof fn lemma_height_le_size<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
        requires
            spec_size_wf_link(link),
            spec_size_link(link) < usize::MAX as nat,
        ensures spec_height_link(link) <= spec_size_link(link),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_size_wf_child_bounded(link);
                lemma_height_le_size(&node.left);
                lemma_height_le_size(&node.right);
                assert(spec_height_link(link) <= spec_size_link(link));
            }
        }
    }

    proof fn lemma_size_wf_child_bounded<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
        requires
            spec_size_wf_link(link),
            spec_size_link(link) > 0,
            spec_size_link(link) < usize::MAX as nat,
        ensures
            match link {
                None => true,
                Some(node) => {
                    spec_size_link(&node.left) < usize::MAX as nat
                    && spec_size_link(&node.right) < usize::MAX as nat
                },
            },
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                assert(node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right));
                assert(spec_size_link(&node.left) < node.size as nat);
                assert(spec_size_link(&node.right) < node.size as nat);
                assert(node.size as nat == spec_size_link(link));
            }
        }
    }

    proof fn lemma_wf_decompose<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
        requires spec_size_wf_link(link),
        ensures match link {
            None => true,
            Some(node) => {
                node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right)
                && spec_size_wf_link(&node.left)
                && spec_size_wf_link(&node.right)
            },
        },
    {
    }

    proof fn lemma_wf_assemble_node<T: StT + Ord + IsLtTransitive>(node: &Box<Node<T>>)
        requires
            node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
            spec_size_wf_link(&node.left),
            spec_size_wf_link(&node.right),
        ensures spec_size_wf_link(&Some(*node)),
    {
    }

    proof fn lemma_contains_left<T: StT + Ord + IsLtTransitive>(node: &Box<Node<T>>, k: T)
        requires spec_contains_link(&node.left, k),
        ensures spec_contains_link(&Some(*node), k),
    {
    }

    proof fn lemma_contains_right<T: StT + Ord + IsLtTransitive>(node: &Box<Node<T>>, k: T)
        requires spec_contains_link(&node.right, k),
        ensures spec_contains_link(&Some(*node), k),
    {
    }

    proof fn lemma_bst_decompose<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
        requires spec_bst_link(link),
        ensures match link {
            None => true,
            Some(node) => {
                spec_bst_link(&node.left)
                && spec_bst_link(&node.right)
                && (forall|k: T| #[trigger] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                && (forall|k: T| #[trigger] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
            },
        },
    {
    }

    proof fn lemma_contains_root<T: StT + Ord + IsLtTransitive>(node: &Box<Node<T>>)
        ensures spec_contains_link(&Some(*node), node.key),
    {
    }

    //		8. traits

    pub trait BSTTreapStEphTrait<T: StT + Ord + IsLtTransitive> {
        spec fn spec_size(self) -> nat;
        spec fn spec_wf(self) -> bool;
        spec fn spec_bst(self) -> bool;
        spec fn spec_height(self) -> nat;
        spec fn spec_contains(self, target: T) -> bool;
        spec fn spec_min(self) -> Option<T>;
        spec fn spec_max(self) -> Option<T>;
        spec fn spec_in_order(self) -> Seq<T>;
        spec fn spec_pre_order(self) -> Seq<T>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                       -> (empty_tree: Self)
        where
            Self: Sized,
            ensures
                empty_tree.spec_size() == 0,
                empty_tree.spec_wf();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> (sz: usize)
            ensures sz as nat == self.spec_size();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> (empty: bool)
            ensures empty == (self.spec_size() == 0);
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self)               -> (h: usize)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_wf(),
            ensures h as nat == self.spec_height();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires old(self).spec_size() + 1 <= usize::MAX as nat, old(self).spec_wf(),
            ensures
                self.spec_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, target: &T)     -> (found: Option<&T>)
            ensures found.is_some() ==> self.spec_contains(*found.unwrap());
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, target: &T) -> bool;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn minimum(&self)              -> (r: Option<&T>)
            ensures match (r, self.spec_min()) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn maximum(&self)              -> (r: Option<&T>)
            ensures match (r, self.spec_max()) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self)             -> (result: ArraySeqStPerS<T>)
            ensures result.spec_len() == self.spec_in_order().len();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn pre_order(&self)            -> (result: ArraySeqStPerS<T>)
            ensures result.spec_len() == self.spec_pre_order().len();

        /// - APAS: Work Θ(1), Span Θ(1)
        fn new_node(key: T, priority: u64) -> (n: Node<T>)
            ensures
                spec_size_wf_link(&Some(Box::new(n))),
                n.size == 1,
        {
            let n = Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            };
            assert(spec_size_wf_link(&n.left));
            assert(spec_size_wf_link(&n.right));
            n
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        fn size_link(link: &Link<T>) -> (sz: usize)
            ensures sz as nat == spec_size_link(link),
        {
            match link.as_ref() {
                None => 0,
                Some(n) => n.size,
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        fn update_size(node: &mut Box<Node<T>>)
            requires 1 + spec_size_link(&old(node).left) + spec_size_link(&old(node).right) <= usize::MAX as nat,
            ensures
                node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
                node.key == old(node).key,
                node.left == old(node).left,
                node.right == old(node).right,
        {
            let l = Self::size_link(&node.left);
            let r = Self::size_link(&node.right);
            node.size = 1 + l + r;
        }

        fn rotate_left(mut x: Box<Node<T>>) -> (rotated: Box<Node<T>>)
            requires
                spec_size_wf_link(&Some(x)),
                spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                spec_size_wf_link(&Some(rotated)),
                spec_size_link(&Some(rotated)) == spec_size_link(&Some(x)),
                spec_bst_link(&Some(x)) ==> spec_bst_link(&Some(rotated)),
        {
            let ghost bst_input = spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_right = x.right;
            assert(spec_size_wf_link(&x.left));
            assert(spec_size_wf_link(&x.right));
            if let Some(mut y) = x.right.take() {
                let ghost yk = y.key;
                let ghost b  = y.left;
                let ghost c  = y.right;

                assert(spec_size_wf_link(&y.left));
                assert(spec_size_wf_link(&y.right));
                let ghost x_left_sz = spec_size_link(&x.left);
                let ghost y_left_sz = spec_size_link(&y.left);
                let ghost y_right_sz = spec_size_link(&y.right);

                proof {
                    if bst_input {
                        lemma_bst_decompose(&orig_right);
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies xk.is_lt(&k) by {
                            lemma_contains_left(&y, k);
                        };
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies k.is_lt(&yk) by {};
                    }
                }

                x.right = y.left.take();
                assert(1 + x_left_sz + y_left_sz + 1 + y_right_sz <= usize::MAX as nat);
                Self::update_size(&mut x);

                proof {
                    if bst_input {
                        assert(spec_bst_link(&x.left));
                        assert(spec_bst_link(&x.right));
                        assert(spec_bst_link(&Some(x)));
                    }
                }

                y.left = Some(x);
                Self::update_size(&mut y);
                proof {
                    lemma_wf_assemble_node(&y);
                    if bst_input {
                        lemma_bst_decompose(&orig_right);
                        assert(spec_bst_link(&y.right));
                        lemma_contains_root(&y);
                        lemma_contains_root(&y);
                        assert(spec_contains_link(&orig_right, yk));
                        assert(xk.is_lt(&yk));
                        assert(x.right == b);
                        assert forall |k: T| #[trigger] spec_contains_link(&y.left, k) implies k.is_lt(&yk) by {
                            if spec_contains_link(&x.left, k) {
                                T::is_lt_transitive(k, xk, yk);
                            }
                            if spec_contains_link(&x.right, k) {
                                assert(spec_contains_link(&b, k));
                            }
                        };
                        assert(spec_bst_link(&Some(y)));
                    }
                }
                y
            } else {
                x
            }
        }

        fn rotate_right(mut x: Box<Node<T>>) -> (rotated: Box<Node<T>>)
            requires
                spec_size_wf_link(&Some(x)),
                spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                spec_size_wf_link(&Some(rotated)),
                spec_size_link(&Some(rotated)) == spec_size_link(&Some(x)),
        {
            assert(spec_size_wf_link(&x.left));
            assert(spec_size_wf_link(&x.right));
            if let Some(mut y) = x.left.take() {
                assert(spec_size_wf_link(&y.left));
                assert(spec_size_wf_link(&y.right));
                let ghost x_right_sz = spec_size_link(&x.right);
                let ghost y_left_sz = spec_size_link(&y.left);
                let ghost y_right_sz = spec_size_link(&y.right);
                x.left = y.right.take();
                assert(1 + y_left_sz + x_right_sz + 1 + y_right_sz <= usize::MAX as nat);
                Self::update_size(&mut x);
                y.right = Some(x);
                Self::update_size(&mut y);
                proof { lemma_wf_assemble_node(&y); }
                y
            } else {
                x
            }
        }

        fn clone_link(link: &Link<T>) -> (c: Link<T>)
            ensures spec_size_link(&c) == spec_size_link(link);

        fn height_link(link: &Link<T>) -> (h: usize)
            requires
                spec_size_link(link) < usize::MAX as nat,
                spec_size_wf_link(link),
            ensures h as nat == spec_height_link(link);

        fn insert_link(link: Link<T>, value: T, priority: u64) -> (inserted: Link<T>)
            requires
                spec_size_link(&link) + 1 <= usize::MAX as nat,
                spec_size_wf_link(&link),
            ensures
                spec_size_wf_link(&inserted),
                spec_size_link(&inserted) <= spec_size_link(&link) + 1,
                spec_size_link(&inserted) >= spec_size_link(&link);

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            ensures found.is_some() ==> spec_contains_link(link, *found.unwrap());

        fn min_link(link: &Link<T>) -> (r: Option<&T>)
            ensures match (r, spec_min_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };

        fn max_link(link: &Link<T>) -> (r: Option<&T>)
            ensures match (r, spec_max_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };

        fn in_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            ensures ordered@.len() == spec_in_order_link(link).len();

        fn pre_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            ensures ordered@.len() == spec_pre_order_link(link).len();

    }


    //		9. impls

    impl<T: StT + Ord + IsLtTransitive> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        open spec fn spec_size(self) -> nat { spec_size_link(&self.root) }
        open spec fn spec_wf(self) -> bool { spec_size_wf_link(&self.root) }
        open spec fn spec_bst(self) -> bool { spec_bst_link(&self.root) }
        open spec fn spec_height(self) -> nat { spec_height_link(&self.root) }
        open spec fn spec_contains(self, target: T) -> bool { spec_contains_link(&self.root, target) }
        open spec fn spec_min(self) -> Option<T> { spec_min_link(&self.root) }
        open spec fn spec_max(self) -> Option<T> { spec_max_link(&self.root) }
        open spec fn spec_in_order(self) -> Seq<T> { spec_in_order_link(&self.root) }
        open spec fn spec_pre_order(self) -> Seq<T> { spec_pre_order_link(&self.root) }

        fn new() -> Self { BSTTreapStEph { root: None } }

        fn size(&self) -> usize { Self::size_link(&self.root) }

        fn is_empty(&self) -> bool { self.size() == 0 }

        fn height(&self) -> usize {
            Self::height_link(&self.root)
        }

        fn insert(&mut self, value: T, priority: u64) {
            self.root = Self::insert_link(self.root.take(), value, priority);
        }

        fn find(&self, target: &T) -> Option<&T> {
            Self::find_link(&self.root, target)
        }

        fn contains(&self, target: &T) -> bool {
            self.find(target).is_some()
        }

        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(Self::in_order_vec(&self.root))
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(Self::pre_order_vec(&self.root))
        }

        fn clone_link(link: &Link<T>) -> (c: Link<T>)
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => {
                    let left = Self::clone_link(&node.left);
                    let right = Self::clone_link(&node.right);
                    Some(Box::new(Node {
                        key: node.key.clone(),
                        priority: node.priority,
                        size: node.size,
                        left,
                        right,
                    }))
                }
            }
        }

        fn height_link(link: &Link<T>) -> (h: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    proof { lemma_size_wf_child_bounded(link); }
                    let lh = Self::height_link(&node.left);
                    let rh = Self::height_link(&node.right);
                    let m = if lh >= rh { lh } else { rh };
                    proof {
                        lemma_height_le_size(&node.left);
                        lemma_height_le_size(&node.right);
                        assert(lh as nat == spec_height_link(&node.left));
                        assert(rh as nat == spec_height_link(&node.right));
                        assert(m as nat <= spec_size_link(&node.left) || m as nat <= spec_size_link(&node.right));
                        assert(m < usize::MAX);
                    }
                    1 + m
                }
            }
        }

        fn insert_link(link: Link<T>, value: T, priority: u64) -> (inserted: Link<T>)
            decreases link,
        {
            match link {
                None => {
                    let n = Box::new(Node { key: value, priority, size: 1, left: None, right: None });
                    proof { lemma_wf_assemble_node(&n); }
                    Some(n)
                },
                Some(mut node) => {
                    assert(spec_size_wf_link(&node.left));
                    assert(spec_size_wf_link(&node.right));
                    if value < node.key {
                        node.left = Self::insert_link(node.left.take(), value, priority);
                        Self::update_size(&mut node);
                        proof { lemma_wf_assemble_node(&node); }
                        let needs_rotate = match &node.left {
                            Some(l) => l.priority < node.priority,
                            None => false,
                        };
                        if needs_rotate { Some(Self::rotate_right(node)) } else { Some(node) }
                    } else if value > node.key {
                        node.right = Self::insert_link(node.right.take(), value, priority);
                        Self::update_size(&mut node);
                        proof { lemma_wf_assemble_node(&node); }
                        let needs_rotate = match &node.right {
                            Some(r) => r.priority < node.priority,
                            None => false,
                        };
                        if needs_rotate { Some(Self::rotate_left(node)) } else { Some(node) }
                    } else {
                        Some(node)
                    }
                }
            }
        }

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => {
                    if *target == node.key {
                        proof { lemma_contains_root(node); }
                        Some(&node.key)
                    } else if *target < node.key {
                        let r = Self::find_link(&node.left, target);
                        proof {
                            if r.is_some() { lemma_contains_left(node, *r.unwrap()); }
                        }
                        r
                    } else {
                        let r = Self::find_link(&node.right, target);
                        proof {
                            if r.is_some() { lemma_contains_right(node, *r.unwrap()); }
                        }
                        r
                    }
                }
            }
        }

        fn min_link(link: &Link<T>) -> (r: Option<&T>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_link(&node.left),
                },
            }
        }

        fn max_link(link: &Link<T>) -> (r: Option<&T>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_link(&node.right),
                },
            }
        }

        fn in_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            decreases *link,
        {
            match link {
                None => Vec::new(),
                Some(node) => {
                    let mut result = Self::in_order_vec(&node.left);
                    result.push(node.key.clone());
                    let mut right = Self::in_order_vec(&node.right);
                    result.append(&mut right);
                    result
                }
            }
        }

        fn pre_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            decreases *link,
        {
            match link {
                None => Vec::new(),
                Some(node) => {
                    let mut result = Vec::new();
                    result.push(node.key.clone());
                    let mut left = Self::pre_order_vec(&node.left);
                    result.append(&mut left);
                    let mut right = Self::pre_order_vec(&node.right);
                    result.append(&mut right);
                    result
                }
            }
        }
    }

    impl<T: StT + Ord + IsLtTransitive> Default for BSTreeTreap<T> {
        fn default() -> (d: Self)
            ensures d.spec_size() == 0, d.spec_wf(),
        { Self::new() }
    }


    //		11. derive impls in verus!

    impl<T: StT + Ord + IsLtTransitive> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures spec_size_link(&Some(Box::new(cloned))) == spec_size_link(&Some(Box::new(*self))),
        {
            Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: BSTTreapStEph::<T>::clone_link(&self.left),
                right: BSTTreapStEph::<T>::clone_link(&self.right),
            }
        }
    }

    impl<T: StT + Ord + IsLtTransitive> Clone for BSTTreapStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned.spec_size() == self.spec_size(),
        {
            BSTTreapStEph { root: BSTTreapStEph::<T>::clone_link(&self.root) }
        }
    }

    }


    //		12. macros

    #[macro_export]
    macro_rules! BSTTreapStEphLit {
        () => {
            < $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new();
            $( {
                let __val = $x;
                let mut __h = ::std::collections::hash_map::DefaultHasher::new();
                __val.hash(&mut __h);
                __tree.insert(__val, __h.finish());
            } )*
            __tree
        }};
    }


    //		13. derive impls outside verus!

    impl<T: StT + Ord + IsLtTransitive + fmt::Debug> fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("priority", &self.priority)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T: StT + Ord + IsLtTransitive + fmt::Debug> fmt::Debug for BSTTreapStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTTreapStEph").field("root", &self.root).finish()
        }
    }
}
