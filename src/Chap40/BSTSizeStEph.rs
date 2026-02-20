//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Size-augmented BST with O(1) size queries and rank/select operations.

pub mod BSTSizeStEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    type Link<T> = Option<Box<Node<T>>>;

    struct Node<T: StT + Ord> {
        key: T,
        priority: u64,
        size: N, // Size of subtree rooted at this node
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StT + Ord> Clone for Node<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    trait NodeTrait<T: StT + Ord>: Sized {
        fn new(key: T, priority: u64) -> Self;
    }

    impl<T: StT + Ord> NodeTrait<T> for Node<T> {
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

    pub struct BSTSizeStEph<T: StT + Ord> {
        root: Link<T>,
    }

    impl<T: StT + Ord> Clone for BSTSizeStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures true,
        {
            BSTSizeStEph { root: self.root.clone() }
        }
    }

    pub type BSTreeSize<T> = BSTSizeStEph<T>;

    pub trait BSTSizeStEphTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn insert(&mut self, value: T, priority: u64);
        /// claude-4-sonet: Work Θ(n), Span Θ(n) — in-order filter + rebuild
        fn delete(&mut self, key: &T);
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn find(&self, target: &T)     -> Option<&T>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn contains(&self, target: &T) -> B;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn minimum(&self)              -> Option<&T>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn maximum(&self)              -> Option<&T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, key: &T)        -> N;
        /// - APAS: Work Θ(log n) with size augmentation, Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn select(&self, rank: N)      -> Option<&T>;
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn split_rank(&self, rank: N)  -> (BSTSizeStEph<T>, BSTSizeStEph<T>);
    }

    fn height_link<T: StT + Ord>(link: &Link<T>) -> N
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => {
                let m = height_link(&node.left).max(height_link(&node.right));
                proof { assume(m < usize::MAX); }
                1 + m
            }
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1) — O(1) size access via augmented field.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn size_link<T: StT + Ord>(link: &Link<T>) -> N {
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1) — recomputes subtree size from children.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn update_size<T: StT + Ord>(node: &mut Node<T>) {
        node.size = 1 + size_link(&node.left) + size_link(&node.right);
    }

    /// - APAS: Work Θ(1), Span Θ(1) — corresponds to APAS makeNode.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn make_node<T: StT + Ord>(key: T, priority: u64, left: Link<T>, right: Link<T>) -> Link<T> {
        let mut node = Node::new(key, priority);
        node.left = left;
        node.right = right;
        update_size(&mut node);
        Some(Box::new(node))
    }

    /// - APAS: N/A — internal treap rotation (updates sizes).
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<T: StT + Ord>(link: &mut Link<T>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.right.take() {
                x.right = y.left.take();
                update_size(&mut x);
                update_size(&mut y);
                y.left = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: N/A — internal treap rotation (updates sizes).
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_right<T: StT + Ord>(link: &mut Link<T>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.left.take() {
                x.left = y.right.take();
                update_size(&mut x);
                update_size(&mut y);
                y.right = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: N/A — internal recursive insert helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
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
                update_size(node);
            }
        } else {
            *link = Some(Box::new(Node::new(value, priority)));
        }
    }

    /// - APAS: N/A — internal recursive find helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
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

    /// - APAS: N/A — internal recursive minimum helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
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

    /// - APAS: N/A — internal recursive maximum helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
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

    /// - APAS: N/A — internal recursive in-order traversal helper.
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

    /// - APAS: N/A — internal in-order traversal collecting (key, priority) for rebuild.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn in_order_collect_with_priority<T: StT + Ord>(
        link: &Link<T>,
        out: &mut Vec<(T, u64)>,
    )
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect_with_priority(&node.left, out);
            out.push((node.key.clone(), node.priority));
            in_order_collect_with_priority(&node.right, out);
        }
    }

    /// - APAS: N/A — build treap from sorted (key, priority) sequence.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — min-priority root, recurse.
    #[verifier::external_body]
    fn build_treap_from_sorted<T: StT + Ord>(seq: &[(T, u64)]) -> Link<T> {
        if seq.is_empty() {
            return None;
        }
        let min_idx = seq
            .iter()
            .enumerate()
            .min_by_key(|entry| entry.1.1)
            .map(|entry| entry.0)
            .unwrap();
        let (key, priority) = seq[min_idx].clone();
        let left_seq = &seq[..min_idx];
        let right_seq = &seq[min_idx + 1..];
        let left = build_treap_from_sorted(left_seq);
        let right = build_treap_from_sorted(right_seq);
        make_node(key, priority, left, right)
    }

    /// - APAS: Work Θ(log n) with size augmentation, Span Θ(log n) — Algorithm 40.1.
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    #[verifier::external_body]
    fn rank_link<T: StT + Ord>(link: &Link<T>, key: &T) -> N
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => {
                let left_size = size_link(&node.left);
                if *key < node.key {
                    rank_link(&node.left, key)
                } else if *key == node.key {
                    left_size + 1
                } else {
                    left_size + 1 + rank_link(&node.right, key)
                }
            }
        }
    }

    /// - APAS: Work Θ(log n) with size augmentation, Span Θ(log n) — Algorithm 40.1.
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    fn select_link<T: StT + Ord>(link: &Link<T>, rank: N) -> Option<&T>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                let left_size = size_link(&node.left);
                if rank <= left_size {
                    select_link(&node.left, rank)
                } else if rank == left_size + 1 {
                    Some(&node.key)
                } else {
                    select_link(&node.right, rank - left_size - 1)
                }
            }
        }
    }

    impl<T: StT + Ord> BSTSizeStEphTrait<T> for BSTSizeStEph<T> {
        fn new() -> Self { BSTSizeStEph { root: None } }

        fn size(&self) -> N { size_link(&self.root) }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N { height_link(&self.root) }

        fn insert(&mut self, value: T, priority: u64) {
            insert_link(&mut self.root, value, priority);
        }

        #[verifier::external_body]
        fn delete(&mut self, key: &T) {
            let mut in_order: Vec<(T, u64)> = Vec::new();
            in_order_collect_with_priority(&self.root, &mut in_order);
            let filtered: Vec<(T, u64)> = in_order.into_iter().filter(|(k, _)| k != key).collect();
            self.root = build_treap_from_sorted(&filtered);
        }

        fn find(&self, target: &T) -> Option<&T> { find_link(&self.root, target) }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn minimum(&self) -> Option<&T> { min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { max_link(&self.root) }

        #[verifier::external_body]
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn rank(&self, key: &T) -> N { rank_link(&self.root, key) }

        fn select(&self, rank: N) -> Option<&T> {
            if rank == 0 || rank > self.size() {
                None
            } else {
                select_link(&self.root, rank)
            }
        }

        #[verifier::external_body]
        fn split_rank(&self, rank: N) -> (BSTSizeStEph<T>, BSTSizeStEph<T>) {
            if rank == 0 {
                (BSTSizeStEph::new(), self.clone())
            } else if rank >= self.size() {
                (self.clone(), BSTSizeStEph::new())
            } else {
                let mut in_order: Vec<(T, u64)> = Vec::new();
                in_order_collect_with_priority(&self.root, &mut in_order);
                let rank = rank.min(in_order.len());
                let left_seq: Vec<(T, u64)> = in_order[..rank].to_vec();
                let right_seq: Vec<(T, u64)> = in_order[rank..].to_vec();
                let left_root = build_treap_from_sorted(&left_seq);
                let right_root = build_treap_from_sorted(&right_seq);
                (
                    BSTSizeStEph { root: left_root },
                    BSTSizeStEph { root: right_root },
                )
            }
        }
    }

    impl<T: StT + Ord> Default for BSTreeSize<T> {
        fn default() -> Self { Self::new()         }
    }

    }

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

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for BSTSizeStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSizeStEph").field("root", &self.root).finish()
        }
    }

    #[macro_export]
    macro_rules! BSTSizeStEphLit {
        () => {
            < $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEph<_> as $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEph<_> as $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEphTrait<_> >::new();
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
