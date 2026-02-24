//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Key-Value BST (dictionary/table) with ephemeral treap structure.

pub mod BSTKeyValueStEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    type Link<K, V> = Option<Box<Node<K, V>>>;

    pub(crate) struct Node<K: StT + Ord, V: StT> {
        pub(crate) key: K,
        pub(crate) value: V,
        pub(crate) priority: u64,
        pub(crate) left: Link<K, V>,
        pub(crate) right: Link<K, V>,
    }

    fn clone_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> (result: Link<K, V>)
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => {
                let mut n = Node {
                    key: node.key.clone(),
                    value: node.value.clone(),
                    priority: node.priority,
                    left: clone_link(&node.left),
                    right: clone_link(&node.right),
                };
                Some(Box::new(n))
            }
        }
    }

    impl<K: StT + Ord, V: StT> Clone for Node<K, V> {
        fn clone(&self) -> Self {
            Node {
                key: self.key.clone(),
                value: self.value.clone(),
                priority: self.priority,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            }
        }
    }

    /// - APAS: N/A — internal node constructor.
    fn new_node<K: StT + Ord, V: StT>(key: K, value: V, priority: u64) -> Node<K, V> {
        Node {
            key,
            value,
            priority,
            left: None,
            right: None,
        }
    }

    pub struct BSTKeyValueStEph<K: StT + Ord, V: StT> {
        pub(crate) root: Link<K, V>,
        pub(crate) size: usize,
    }

    impl<K: StT + Ord, V: StT> Clone for BSTKeyValueStEph<K, V> {
        fn clone(&self) -> (result: Self)
            ensures true,
        {
            BSTKeyValueStEph {
                root: self.root.clone(),
                size: self.size,
            }
        }
    }

    pub type BSTreeKeyValue<K, V> = BSTKeyValueStEph<K, V>;

    pub trait BSTKeyValueStEphTrait<K: StT + Ord, V: StT> {
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                    -> Self
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)              -> usize;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)          -> bool;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)            -> usize
            requires self.spec_height() < usize::MAX as nat;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn insert(&mut self, key: K, value: V, priority: u64)
            requires old(self).spec_size() < usize::MAX;
        /// claude-4-sonet: Work Θ(n), Span Θ(n) — in-order filter + rebuild
        fn delete(&mut self, key: &K);
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn find(&self, key: &K)     -> Option<&V>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn contains(&self, key: &K) -> bool;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn get(&self, key: &K)      -> Option<&V>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn keys(&self)              -> ArraySeqStPerS<K>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn values(&self)            -> ArraySeqStPerS<V>;
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn minimum_key(&self)       -> Option<&K>;
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn maximum_key(&self)       -> Option<&K>;
    }

    fn height_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> (result: usize)
        requires spec_height_link(link) < usize::MAX as nat,
        ensures result == spec_height_link(link),
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => {
                let l = height_link(&node.left);
                let r = height_link(&node.right);
                1 + if l >= r { l } else { r }
            }
        }
    }

    pub(crate) open spec fn spec_height_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let l = spec_height_link(&node.left);
                let r = spec_height_link(&node.right);
                1 + if l >= r { l } else { r }
            }
        }
    }

    /// - APAS: N/A — internal treap rotation.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<K: StT + Ord, V: StT>(link: &mut Link<K, V>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.right.take() {
                x.right = y.left.take();
                y.left = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: N/A — internal treap rotation.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_right<K: StT + Ord, V: StT>(link: &mut Link<K, V>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.left.take() {
                x.left = y.right.take();
                y.right = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    fn insert_link<K: StT + Ord, V: StT>(link: &mut Link<K, V>, key: K, value: V, priority: u64) -> (inserted: bool)
        decreases old(link),
    {
        if let Some(mut node) = link.take() {
            if key < node.key {
                let inserted = insert_link(&mut node.left, key, value, priority);
                *link = Some(node);
                let need_rotate = match link.as_ref().unwrap().left.as_ref() {
                    Some(left) => left.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate {
                    rotate_right(link);
                }
                inserted
            } else if key > node.key {
                let inserted = insert_link(&mut node.right, key, value, priority);
                *link = Some(node);
                let need_rotate = match link.as_ref().unwrap().right.as_ref() {
                    Some(right) => right.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate {
                    rotate_left(link);
                }
                inserted
            } else {
                node.value = value;
                *link = Some(node);
                false
            }
        } else {
            *link = Some(Box::new(new_node(key, value, priority)));
            true
        }
    }

    fn find_link<'a, K: StT + Ord, V: StT>(link: &'a Link<K, V>, key: &K) -> Option<&'a V>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                if *key == node.key {
                    Some(&node.value)
                } else if *key < node.key {
                    find_link(&node.left, key)
                } else {
                    find_link(&node.right, key)
                }
            }
        }
    }

    /// - APAS: N/A — internal recursive min-key helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
    fn min_key_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> Option<&K>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => Some(&node.key),
                | Some(_) => min_key_link(&node.left),
            },
        }
    }

    /// - APAS: N/A — internal recursive max-key helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
    fn max_key_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> Option<&K>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => Some(&node.key),
                | Some(_) => max_key_link(&node.right),
            },
        }
    }

    /// - APAS: N/A — internal recursive key collection helper.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn collect_keys<K: StT + Ord, V: StT>(link: &Link<K, V>, out: &mut Vec<K>)
        decreases *link,
    {
        if let Some(node) = link {
            collect_keys(&node.left, out);
            out.push(node.key.clone());
            collect_keys(&node.right, out);
        }
    }

    /// - APAS: N/A — internal recursive value collection helper.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn collect_values<K: StT + Ord, V: StT>(link: &Link<K, V>, out: &mut Vec<V>)
        decreases *link,
    {
        if let Some(node) = link {
            collect_values(&node.left, out);
            out.push(node.value.clone());
            collect_values(&node.right, out);
        }
    }

    /// - APAS: N/A — in-order collect (key, value, priority) for rebuild.
    fn collect_in_order_kvp<K: StT + Ord, V: StT>(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>)
        decreases *link,
    {
        if let Some(node) = link {
            collect_in_order_kvp(&node.left, out);
            out.push((node.key.clone(), node.value.clone(), node.priority));
            collect_in_order_kvp(&node.right, out);
        }
    }

    fn find_min_priority_idx_kvp<K: StT + Ord, V: StT>(
        items: &Vec<(K, V, u64)>, start: usize, end: usize,
    ) -> (result: usize)
        requires start < end, end <= items.len(),
        ensures start <= result && result < end,
    {
        let mut min_idx = start;
        let mut i = start + 1;
        while i < end
            invariant
                start <= min_idx, min_idx < end, min_idx < i,
                i <= end, end <= items.len(),
            decreases end - i,
        {
            if items[i].2 < items[min_idx].2 {
                min_idx = i;
            }
            i = i + 1;
        }
        min_idx
    }

    fn build_treap_from_vec<K: StT + Ord, V: StT>(
        items: &Vec<(K, V, u64)>, start: usize, end: usize,
    ) -> (result: Link<K, V>)
        requires start <= end, end <= items.len(),
        decreases end - start,
    {
        if start >= end {
            return None;
        }
        let min_idx = find_min_priority_idx_kvp(items, start, end);
        let key = items[min_idx].0.clone();
        let value = items[min_idx].1.clone();
        let priority = items[min_idx].2;
        let left = build_treap_from_vec(items, start, min_idx);
        let right = build_treap_from_vec(items, min_idx + 1, end);
        let mut node = new_node(key, value, priority);
        node.left = left;
        node.right = right;
        Some(Box::new(node))
    }

    fn filter_by_key_kvp<K: StT + Ord, V: StT>(
        items: &Vec<(K, V, u64)>, key: &K,
    ) -> (result: Vec<(K, V, u64)>) {
        let mut filtered: Vec<(K, V, u64)> = Vec::new();
        let mut i: usize = 0;
        while i < items.len()
            invariant i <= items.len(),
            decreases items.len() - i,
        {
            if items[i].0 != *key {
                filtered.push((items[i].0.clone(), items[i].1.clone(), items[i].2));
            }
            i = i + 1;
        }
        filtered
    }

    impl<K: StT + Ord, V: StT> BSTKeyValueStEphTrait<K, V> for BSTKeyValueStEph<K, V> {
        closed spec fn spec_size(&self) -> nat { self.size as nat }
        closed spec fn spec_height(&self) -> nat { spec_height_link(&self.root) }

        fn new() -> Self { BSTKeyValueStEph { root: None, size: 0 } }

        fn size(&self) -> usize { self.size }

        fn is_empty(&self) -> bool { self.size == 0 }

        fn height(&self) -> usize { height_link(&self.root) }

        fn insert(&mut self, key: K, value: V, priority: u64) {
            let inserted = insert_link(&mut self.root, key, value, priority);
            if inserted {
                self.size = self.size + 1;
            }
        }

        fn delete(&mut self, key: &K) {
            let mut in_order: Vec<(K, V, u64)> = Vec::new();
            collect_in_order_kvp(&self.root, &mut in_order);
            let filtered = filter_by_key_kvp(&in_order, key);
            self.root = build_treap_from_vec(&filtered, 0, filtered.len());
            self.size = filtered.len();
        }

        fn find(&self, key: &K) -> Option<&V> { find_link(&self.root, key) }

        fn contains(&self, key: &K) -> bool { self.find(key).is_some() }

        fn get(&self, key: &K) -> Option<&V> { self.find(key) }

        fn keys(&self) -> ArraySeqStPerS<K> {
            let mut out = Vec::new();
            collect_keys(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn values(&self) -> ArraySeqStPerS<V> {
            let mut out = Vec::new();
            collect_values(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn minimum_key(&self) -> Option<&K> { min_key_link(&self.root) }

        fn maximum_key(&self) -> Option<&K> { max_key_link(&self.root) }
    }

    impl<K: StT + Ord, V: StT> Default for BSTreeKeyValue<K, V> {
        fn default() -> Self { Self::new() }
    }

    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for Node<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("value", &self.value)
                .field("priority", &self.priority)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for BSTKeyValueStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTKeyValueStEph")
                .field("root", &self.root)
                .field("size", &self.size)
                .finish()
        }
    }

    #[macro_export]
    macro_rules! BSTKeyValueStEphLit {
        () => {
            < $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEph<_, _> as $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEphTrait<_, _> >::new()
        };
        ( $( ($k:expr, $v:expr) ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEph<_, _> as $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEphTrait<_, _> >::new();
            $( {
                let __key = $k;
                let mut __h = ::std::collections::hash_map::DefaultHasher::new();
                __key.hash(&mut __h);
                __tree.insert(__key, $v, __h.finish());
            } )*
            __tree
        }};
    }
}
