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
        pub size: N,
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
        open spec fn inv(self, v: Link<T>) -> bool { true }
    }

    #[verifier::external_body]
    fn new_treap_link_lock<T: StTInMtT + Ord>(val: Link<T>) -> (lock: RwLock<Link<T>, TreapLinkWf>) {
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
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&self, value: T, priority: u64);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, target: &T)     -> Option<T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, target: &T) -> B;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N;
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


    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> N {
        match link {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn update<T: StTInMtT + Ord>(node: &mut Node<T>) {
        node.size = 1 + size_link(&node.left) + size_link(&node.right);
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<T: StTInMtT + Ord>(link: &mut Link<T>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.right.take() {
                x.right = y.left.take();
                update(&mut x);
                update(&mut y);
                y.left = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_right<T: StTInMtT + Ord>(link: &mut Link<T>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.left.take() {
                x.left = y.right.take();
                update(&mut x);
                update(&mut y);
                y.right = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T, priority: u64)
        decreases old(link),
    {
        if let Some(mut node) = link.take() {
            if value < node.key {
                insert_link(&mut node.left, value, priority);
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
            if let Some(mut n) = link.take() {
                update(&mut n);
                *link = Some(n);
            }
        } else {
            *link = Some(Box::new(Node::new(value, priority)));
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

    #[verifier::external_body]
    fn height_link<T: StTInMtT + Ord>(link: &Link<T>) -> N {
        match link {
            None => 0,
            Some(node) => 1 + height_link(&node.left).max(height_link(&node.right)),
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
        fn new() -> Self {
            BSTTreapMtEph {
                root: Arc::new(new_treap_link_lock(None)),
            }
        }

        fn insert(&self, value: T, priority: u64) {
            let (mut current, write_handle) = self.root.acquire_write();
            insert_link(&mut current, value, priority);
            write_handle.release_write(current);
        }

        fn find(&self, target: &T) -> Option<T> {
            let handle = self.root.acquire_read();
            let result = find_link(handle.borrow(), target).cloned();
            handle.release_read();
            result
        }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn size(&self) -> N {
            let handle = self.root.acquire_read();
            let result = size_link(handle.borrow());
            handle.release_read();
            result
        }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N {
            let handle = self.root.acquire_read();
            let result = height_link(handle.borrow());
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
