//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Treap (randomized heap-ordered BST) with interior locking for multi-threaded access.

pub mod BSTTreapMtEph {

    use std::fmt::{Debug, Display};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::Types::Types::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StTInMtT + Ord> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    trait NodeTrait<T: StTInMtT + Ord>: Sized {
        fn new(key: T, priority: u64) -> Self;
    }

    impl<T: StTInMtT + Ord> NodeTrait<T> for Node<T> {
        fn new(key: T, priority: u64) -> Self {
            Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    fn clone_link<T: StTInMtT + Ord + Clone>(link: &Link<T>) -> (c: Link<T>)
        decreases link,
    {
        match link {
            None => {
                let c = None;
                proof { accept(c == *link); }
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
                proof { accept(c == *link); }
                c
            }
        }
    }

    impl<T: StTInMtT + Ord + Clone> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            };
            proof { accept(cloned == *self); }
            cloned
        }
    }

    pub struct TreapLinkWf;

    impl<T: StTInMtT + Ord> RwLockPredicate<Link<T>> for TreapLinkWf {
        open spec fn inv(self, v: Link<T>) -> bool {
            spec_size_wf_link(&v) && spec_size_link(&v) < usize::MAX as nat
        }
    }

    #[verifier::external_body]
    fn new_treap_link_lock<T: StTInMtT + Ord>(val: Link<T>) -> (lock: RwLock<Link<T>, TreapLinkWf>)
        requires spec_size_wf_link(&val), spec_size_link(&val) < usize::MAX as nat,
    {
        RwLock::new(val, Ghost(TreapLinkWf))
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>, TreapLinkWf>>,
    }

    impl<T: StTInMtT + Ord> Clone for BSTTreapMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures true
        {
            let cloned = BSTTreapMtEph { root: self.root.clone() };
            cloned
        }
    }

    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;

    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;

        fn new()                       -> Self;
        fn insert(&self, value: T, priority: u64)
            requires self.spec_size() + 1 <= usize::MAX as nat;
        fn find(&self, target: &T)     -> Option<T>;
        fn contains(&self, target: &T) -> bool;
        fn size(&self)                 -> usize;
        fn is_empty(&self)             -> bool;
        fn height(&self)               -> usize
            requires self.spec_size() < usize::MAX as nat;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn minimum(&self)              -> Option<T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn maximum(&self)              -> Option<T>;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
    }


    pub open spec fn spec_size_link<T: StTInMtT + Ord>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_size_wf_link<T: StTInMtT + Ord>(link: &Link<T>) -> bool
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

    closed spec fn spec_height_link<T: StTInMtT + Ord>(link: &Link<T>) -> nat
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

    proof fn lemma_height_le_size<T: StTInMtT + Ord>(link: &Link<T>)
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
            }
        }
    }

    proof fn lemma_size_wf_child_bounded<T: StTInMtT + Ord>(link: &Link<T>)
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
            }
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> (result: usize)
        ensures result as nat == spec_size_link(link),
    {
        match link {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    proof fn lemma_wf_assemble_node<T: StTInMtT + Ord>(node: &Node<T>)
        requires
            node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
            spec_size_wf_link(&node.left),
            spec_size_wf_link(&node.right),
        ensures spec_size_wf_link(&Some(Box::new(*node))),
    {
    }

    fn update<T: StTInMtT + Ord>(node: &mut Node<T>)
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

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<T: StTInMtT + Ord>(link: &mut Link<T>)
        requires
            spec_size_wf_link(old(link)),
            spec_size_link(old(link)) <= usize::MAX as nat,
        ensures
            spec_size_wf_link(link),
            spec_size_link(link) == spec_size_link(old(link)),
    {
        if let Some(mut x) = link.take() {
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
                update(&mut x);
                y.left = Some(x);
                update(&mut y);
                proof { lemma_wf_assemble_node(&*y); }
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    fn rotate_right<T: StTInMtT + Ord>(link: &mut Link<T>)
        requires
            spec_size_wf_link(old(link)),
            spec_size_link(old(link)) <= usize::MAX as nat,
        ensures
            spec_size_wf_link(link),
            spec_size_link(link) == spec_size_link(old(link)),
    {
        if let Some(mut x) = link.take() {
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
                update(&mut x);
                y.right = Some(x);
                update(&mut y);
                proof { lemma_wf_assemble_node(&*y); }
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T, priority: u64)
        requires
            spec_size_link(old(link)) + 1 <= usize::MAX as nat,
            spec_size_wf_link(old(link)),
        ensures
            spec_size_wf_link(link),
            spec_size_link(link) <= spec_size_link(old(link)) + 1,
            spec_size_link(link) >= spec_size_link(old(link)),
        decreases old(link),
    {
        if let Some(mut node) = link.take() {
            assert(spec_size_wf_link(&node.left));
            assert(spec_size_wf_link(&node.right));
            if value < node.key {
                insert_link(&mut node.left, value, priority);
                update(&mut node);
                proof { lemma_wf_assemble_node(&*node); }
                *link = Some(node);
                let need_rotate_right = match link.as_ref().unwrap().left.as_ref() {
                    Some(left) => left.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate_right {
                    rotate_right(link);
                }
            } else if value > node.key {
                insert_link(&mut node.right, value, priority);
                update(&mut node);
                proof { lemma_wf_assemble_node(&*node); }
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
            proof { lemma_wf_assemble_node(&*n); }
            *link = Some(n);
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn find_link<'a, T: StTInMtT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                if (*target) == node.key {
                    Some(&node.key)
                } else if (*target) < node.key {
                    find_link(&node.left, target)
                } else {
                    find_link(&node.right, target)
                }
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T>
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
    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T>
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

    fn height_link<T: StTInMtT + Ord>(link: &Link<T>) -> (h: usize)
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

    /// - APAS: Work Θ(n), Span Θ(n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn in_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
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
    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {
        closed spec fn spec_size(&self) -> nat { 0 }
        closed spec fn spec_height(&self) -> nat { 0 }

        fn new() -> Self {
            BSTTreapMtEph {
                root: Arc::new(new_treap_link_lock(None)),
            }
        }

        fn insert(&self, value: T, priority: u64) {
            let (mut current, write_handle) = self.root.acquire_write();
            let sz = size_link(&current);
            if sz + 1 < usize::MAX {
                insert_link(&mut current, value, priority);
            }
            write_handle.release_write(current);
        }

        fn find(&self, target: &T) -> Option<T> {
            let handle = self.root.acquire_read();
            let result = find_link(handle.borrow(), target).cloned();
            handle.release_read();
            result
        }

        fn contains(&self, target: &T) -> bool { self.find(target).is_some() }

        fn size(&self) -> usize {
            let handle = self.root.acquire_read();
            let result = size_link(handle.borrow());
            handle.release_read();
            result
        }

        fn is_empty(&self) -> bool { self.size() == 0 }

        fn height(&self) -> usize {
            let handle = self.root.acquire_read();
            let link: &Link<T> = handle.borrow();
            let result = height_link(link);
            handle.release_read();
            result
        }

        fn minimum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let result = min_link(handle.borrow()).cloned();
            handle.release_read();
            result
        }

        fn maximum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let result = max_link(handle.borrow()).cloned();
            handle.release_read();
            result
        }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let mut out = Vec::with_capacity(size_link(handle.borrow()));
            in_order_collect(handle.borrow(), &mut out);
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let mut out = Vec::with_capacity(size_link(handle.borrow()));
            pre_order_collect(handle.borrow(), &mut out);
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }
    }

    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    } // verus!

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTTreapMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTTreapMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTTreapMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTTreapMtEph")
        }
    }

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

    #[macro_export]
    macro_rules! BSTTreapMtEphLit {
        () => {
            < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let __tree = < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new();
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
