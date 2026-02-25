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

    verus! {

    //		4. type definitions

    type Link<T> = Option<Box<Node<T>>>;

    struct Node<T: StT + Ord> {
        key: T,
        priority: u64,
        size: usize,
        left: Link<T>,
        right: Link<T>,
    }

    pub struct BSTTreapStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeTreap<T> = BSTTreapStEph<T>;


    //		6. spec fns

    #[allow(private_interfaces)]
    pub closed spec fn spec_size_link<T: StT + Ord>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    closed spec fn spec_contains_link<T: StT + Ord>(link: &Link<T>, target: T) -> bool
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

    closed spec fn spec_bst_link<T: StT + Ord>(link: &Link<T>) -> bool
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

    closed spec fn spec_size_wf_link<T: StT + Ord>(link: &Link<T>) -> bool
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

    closed spec fn spec_in_order_link<T: StT + Ord>(link: &Link<T>) -> Seq<T>
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

    closed spec fn spec_pre_order_link<T: StT + Ord>(link: &Link<T>) -> Seq<T>
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

    closed spec fn spec_height_link<T: StT + Ord>(link: &Link<T>) -> nat
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

    proof fn lemma_height_le_size<T: StT + Ord>(link: &Link<T>)
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

    proof fn lemma_size_wf_child_bounded<T: StT + Ord>(link: &Link<T>)
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

    proof fn lemma_wf_decompose<T: StT + Ord>(link: &Link<T>)
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

    proof fn lemma_wf_assemble_node<T: StT + Ord>(node: &Box<Node<T>>)
        requires
            node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
            spec_size_wf_link(&node.left),
            spec_size_wf_link(&node.right),
        ensures spec_size_wf_link(&Some(*node)),
    {
    }

    //		8. traits

    pub trait BSTTreapStEphTrait<T: StT + Ord> {
        spec fn spec_size(self) -> nat;
        spec fn spec_wf(self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                       -> Self
        where
            Self: Sized;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> usize;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> bool;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self)               -> usize
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_wf();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires old(self).spec_size() + 1 <= usize::MAX as nat, old(self).spec_wf();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, target: &T)     -> Option<&T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, target: &T) -> bool;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn minimum(&self)              -> Option<&T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn maximum(&self)              -> Option<&T>;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
    }


    //		9. impls

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn new_node<T: StT + Ord>(key: T, priority: u64) -> Node<T> {
        Node {
            key,
            priority,
            size: 1,
            left: None,
            right: None,
        }
    }

    /// - APAS: Work Θ(n), Span Θ(n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn clone_link<T: StT + Ord>(link: &Link<T>) -> (c: Link<T>)
        ensures spec_size_link(&c) == spec_size_link(link),
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => {
                let left = clone_link(&node.left);
                let right = clone_link(&node.right);
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

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn size_link<T: StT + Ord>(link: &Link<T>) -> (result: usize)
        ensures result as nat == spec_size_link(link),
    {
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    fn height_link<T: StT + Ord>(link: &Link<T>) -> (h: usize)
        requires
            spec_size_link(link) < usize::MAX as nat,
            spec_size_wf_link(link),
        ensures h as nat == spec_height_link(link),
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => {
                proof { lemma_size_wf_child_bounded(link); }
                let lh = height_link(&node.left);
                let rh = height_link(&node.right);
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

    /// - APAS: Work Θ(1), Span Θ(1)
    fn update_size<T: StT + Ord>(node: &mut Box<Node<T>>)
        requires 1 + spec_size_link(&old(node).left) + spec_size_link(&old(node).right) <= usize::MAX as nat,
        ensures
            node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
            node.key == old(node).key,
            node.left == old(node).left,
            node.right == old(node).right,
    {
        let l = size_link(&node.left);
        let r = size_link(&node.right);
        node.size = 1 + l + r;
    }

    fn rotate_left<T: StT + Ord>(mut x: Box<Node<T>>) -> (result: Box<Node<T>>)
        requires
            spec_size_wf_link(&Some(x)),
            spec_size_link(&Some(x)) <= usize::MAX as nat,
        ensures
            spec_size_wf_link(&Some(result)),
            spec_size_link(&Some(result)) == spec_size_link(&Some(x)),
    {
        assert(spec_size_wf_link(&x.left));
        assert(spec_size_wf_link(&x.right));
        if let Some(mut y) = x.right.take() {
            assert(spec_size_wf_link(&y.left));
            assert(spec_size_wf_link(&y.right));
            let ghost x_left_sz = spec_size_link(&x.left);
            let ghost y_left_sz = spec_size_link(&y.left);
            let ghost y_right_sz = spec_size_link(&y.right);
            x.right = y.left.take();
            assert(1 + x_left_sz + y_left_sz + 1 + y_right_sz <= usize::MAX as nat);
            update_size(&mut x);
            y.left = Some(x);
            update_size(&mut y);
            proof { lemma_wf_assemble_node(&y); }
            y
        } else {
            x
        }
    }

    fn rotate_right<T: StT + Ord>(mut x: Box<Node<T>>) -> (result: Box<Node<T>>)
        requires
            spec_size_wf_link(&Some(x)),
            spec_size_link(&Some(x)) <= usize::MAX as nat,
        ensures
            spec_size_wf_link(&Some(result)),
            spec_size_link(&Some(result)) == spec_size_link(&Some(x)),
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
            update_size(&mut x);
            y.right = Some(x);
            update_size(&mut y);
            proof { lemma_wf_assemble_node(&y); }
            y
        } else {
            x
        }
    }

    fn insert_link<T: StT + Ord>(link: Link<T>, value: T, priority: u64) -> (result: Link<T>)
        requires
            spec_size_link(&link) + 1 <= usize::MAX as nat,
            spec_size_wf_link(&link),
        ensures
            spec_size_wf_link(&result),
            spec_size_link(&result) <= spec_size_link(&link) + 1,
            spec_size_link(&result) >= spec_size_link(&link),
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
                    node.left = insert_link(node.left.take(), value, priority);
                    update_size(&mut node);
                    proof { lemma_wf_assemble_node(&node); }
                    let needs_rotate = match &node.left {
                        Some(l) => l.priority < node.priority,
                        None => false,
                    };
                    if needs_rotate { Some(rotate_right(node)) } else { Some(node) }
                } else if value > node.key {
                    node.right = insert_link(node.right.take(), value, priority);
                    update_size(&mut node);
                    proof { lemma_wf_assemble_node(&node); }
                    let needs_rotate = match &node.right {
                        Some(r) => r.priority < node.priority,
                        None => false,
                    };
                    if needs_rotate { Some(rotate_left(node)) } else { Some(node) }
                } else {
                    Some(node)
                }
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn find_link<'a, T: StT + Ord>(link: &'a Link<T>, target: &T) -> (result: Option<&'a T>)
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                if *target == node.key {
                    Some(&node.key)
                } else if *target < node.key {
                    find_link(&node.left, target)
                } else {
                    find_link(&node.right, target)
                }
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn min_link<T: StT + Ord>(link: &Link<T>) -> Option<&T>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => Some(&node.key),
                | Some(_) => min_link(&node.left),
            },
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn max_link<T: StT + Ord>(link: &Link<T>) -> Option<&T>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => Some(&node.key),
                | Some(_) => max_link(&node.right),
            },
        }
    }

    fn in_order_vec<T: StT + Ord>(link: &Link<T>) -> (result: Vec<T>)
        decreases *link,
    {
        match link {
            None => Vec::new(),
            Some(node) => {
                let mut left = in_order_vec(&node.left);
                left.push(node.key.clone());
                let right = in_order_vec(&node.right);
                let mut i: usize = 0;
                while i < right.len()
                    invariant i <= right.len(),
                    decreases right.len() - i,
                {
                    left.push(right[i].clone());
                    i = i + 1;
                }
                left
            }
        }
    }

    fn pre_order_vec<T: StT + Ord>(link: &Link<T>) -> (result: Vec<T>)
        decreases *link,
    {
        match link {
            None => Vec::new(),
            Some(node) => {
                let mut result = Vec::new();
                result.push(node.key.clone());
                let left = pre_order_vec(&node.left);
                let mut i: usize = 0;
                while i < left.len()
                    invariant i <= left.len(),
                    decreases left.len() - i,
                {
                    result.push(left[i].clone());
                    i = i + 1;
                }
                let right = pre_order_vec(&node.right);
                let mut j: usize = 0;
                while j < right.len()
                    invariant j <= right.len(),
                    decreases right.len() - j,
                {
                    result.push(right[j].clone());
                    j = j + 1;
                }
                result
            }
        }
    }

    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        closed spec fn spec_size(self) -> nat { spec_size_link(&self.root) }
        closed spec fn spec_wf(self) -> bool { spec_size_wf_link(&self.root) }

        fn new() -> Self { BSTTreapStEph { root: None } }

        fn size(&self) -> usize { size_link(&self.root) }

        fn is_empty(&self) -> bool { self.size() == 0 }

        fn height(&self) -> usize {
            height_link(&self.root)
        }

        fn insert(&mut self, value: T, priority: u64) {
            self.root = insert_link(self.root.take(), value, priority);
        }

        fn find(&self, target: &T) -> Option<&T> {
            find_link(&self.root, target)
        }

        fn contains(&self, target: &T) -> bool {
            self.find(target).is_some()
        }

        fn minimum(&self) -> Option<&T> { min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(in_order_vec(&self.root))
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(pre_order_vec(&self.root))
        }
    }

    impl<T: StT + Ord> Default for BSTreeTreap<T> {
        fn default() -> Self { Self::new() }
    }


    //		11. derive impls in verus!

    impl<T: StT + Ord> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures spec_size_link(&Some(Box::new(cloned))) == spec_size_link(&Some(Box::new(*self))),
        {
            Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            }
        }
    }

    impl<T: StT + Ord> Clone for BSTTreapStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures result.spec_size() == self.spec_size(),
        {
            BSTTreapStEph { root: clone_link(&self.root) }
        }
    }

    }


    //		13. derive impls outside verus!

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for Node<T> {
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

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for BSTTreapStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTTreapStEph").field("root", &self.root).finish()
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
}
