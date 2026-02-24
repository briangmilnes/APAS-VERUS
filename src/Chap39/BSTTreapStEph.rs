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
        size: N,
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
                    && (forall|k: T| spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                    && (forall|k: T| spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
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
        fn size(&self)                 -> N;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_wf();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&mut self, value: T, priority: u64);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, target: &T)     -> Option<&T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, target: &T) -> B;
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
    fn size_link<T: StT + Ord>(link: &Link<T>) -> (result: N)
        ensures result as nat == spec_size_link(link),
    {
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    fn height_link<T: StT + Ord>(link: &Link<T>) -> (h: N)
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
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn update<T: StT + Ord>(node: &mut Node<T>)
        ensures
            node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
    {
        node.size = 1 + size_link(&node.left) + size_link(&node.right);
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<T: StT + Ord>(link: &mut Link<T>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.right.take() {
                x.right = y.left.take();
                update(&mut x);
                y.left = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_right<T: StT + Ord>(link: &mut Link<T>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.left.take() {
                x.left = y.right.take();
                update(&mut x);
                y.right = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    #[verifier::external_body]
    fn insert_link<T: StT + Ord>(link: &mut Link<T>, value: T, priority: u64)
        decreases old(link),
    {
        if let Some(node) = link.as_mut() {
            if value < node.key {
                insert_link(&mut node.left, value, priority);
                if node.left.as_ref().is_some_and(|left| left.priority < node.priority) {
                    rotate_right(link);
                }
            } else if value > node.key {
                insert_link(&mut node.right, value, priority);
                if node.right.as_ref().is_some_and(|right| right.priority < node.priority) {
                    rotate_left(link);
                }
            }
            if let Some(node) = link.as_mut() {
                update(node);
            }
        } else {
            *link = Some(Box::new(new_node(value, priority)));
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    #[verifier::external_body]
    fn find_link<'a, T: StT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T>
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

    /// - APAS: Work Θ(n), Span Θ(n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn in_order_collect<T: StT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    /// - APAS: Work Θ(n), Span Θ(n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn pre_order_collect<T: StT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        closed spec fn spec_size(self) -> nat { spec_size_link(&self.root) }
        closed spec fn spec_wf(self) -> bool { spec_size_wf_link(&self.root) }

        fn new() -> Self { BSTTreapStEph { root: None } }

        fn size(&self) -> N { size_link(&self.root) }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N {
            height_link(&self.root)
        }

        fn insert(&mut self, value: T, priority: u64) {
            insert_link(&mut self.root, value, priority);
        }

        fn find(&self, target: &T) -> Option<&T> {
            find_link(&self.root, target)
        }

        fn contains(&self, target: &T) -> B {
            self.find(target).is_some()
        }

        fn minimum(&self) -> Option<&T> { min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { max_link(&self.root) }

        #[verifier::external_body]
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        #[verifier::external_body]
        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            pre_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
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
