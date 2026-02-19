//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Treap (randomized heap-ordered BST) with interior locking for multi-threaded access.

pub mod BSTTreapMtEph {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        priority: u64,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StTInMtT + Ord> Node<T> {
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

    verus! {
        #[verifier::reject_recursive_types(T)]
        #[verifier::external_type_specification]
        struct ExNode<T: StTInMtT + Ord>(Node<T>);

        pub struct TreapLinkWf;

        impl<T: StTInMtT + Ord> RwLockPredicate<Link<T>> for TreapLinkWf {
            open spec fn inv(self, v: Link<T>) -> bool { true }
        }

        #[verifier::external_body]
        fn new_treap_link_lock<T: StTInMtT + Ord>(val: Link<T>) -> (lock: RwLock<Link<T>, TreapLinkWf>) {
            RwLock::new(val, Ghost(TreapLinkWf))
        }
    }

    #[derive(Clone)]
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>, TreapLinkWf>>,
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTTreapMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTTreapMtEph").finish()
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
    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn update<T: StTInMtT + Ord>(node: &mut Node<T>) { node.size = 1 + size_link(&node.left) + size_link(&node.right); }

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
    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T, priority: u64) {
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
            *link = Some(Box::new(Node::new(value, priority)));
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn find_link<'a, T: StTInMtT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
        match link {
            | None => None,
            | Some(node) => {
                if target == &node.key {
                    Some(&node.key)
                } else if target < &node.key {
                    find_link(&node.left, target)
                } else {
                    find_link(&node.right, target)
                }
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T> {
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
    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T> {
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
    fn in_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    /// - APAS: Work Θ(n), Span Θ(n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
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
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }

            let handle = self.root.acquire_read();
            let result = height_rec(handle.borrow());
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
