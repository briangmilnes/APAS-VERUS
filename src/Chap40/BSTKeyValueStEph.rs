//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Key-Value BST (dictionary/table) with ephemeral treap structure.

pub mod BSTKeyValueStEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 4. type definitions

    pub type Link<K, V> = Option<Box<Node<K, V>>>;

    pub struct Node<K: StT + Ord, V: StT> {
        pub key: K,
        pub value: V,
        pub priority: u64,
        pub left: Link<K, V>,
        pub right: Link<K, V>,
    }

    fn clone_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> (result: Link<K, V>)
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => {
                Some(Box::new(Node {
                    key: node.key.clone(),
                    value: node.value.clone(),
                    priority: node.priority,
                    left: clone_link(&node.left),
                    right: clone_link(&node.right),
                }))
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

    pub struct BSTKeyValueStEph<K: StT + Ord, V: StT> {
        pub root: Link<K, V>,
        pub size: usize,
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

    // 6. spec fns

    pub open spec fn spec_height_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> nat
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

    // 8. traits

    pub trait BSTKeyValueStEphTrait<K: StT + Ord, V: StT>: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;

        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new() -> (result: Self)
            ensures result.spec_size() == 0;
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: usize)
            ensures result as nat == self.spec_size();
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (result: bool)
            ensures result == (self.spec_size() == 0);
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self) -> (result: usize)
            requires self.spec_height() < usize::MAX as nat,
            ensures result as nat == self.spec_height();
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn insert(&mut self, key: K, value: V, priority: u64)
            requires old(self).spec_size() < usize::MAX;
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — filter + rebuild
        fn delete(&mut self, key: &K);
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn find(&self, key: &K) -> Option<&V>;
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn contains(&self, key: &K) -> bool;
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn get(&self, key: &K) -> Option<&V>;
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn keys(&self) -> ArraySeqStPerS<K>;
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn values(&self) -> ArraySeqStPerS<V>;
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn minimum_key(&self) -> Option<&K>;
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn maximum_key(&self) -> Option<&K>;

        // Internal associated functions.

        fn new_node(key: K, value: V, priority: u64) -> Node<K, V>;
        fn height_link(link: &Link<K, V>) -> (result: usize)
            requires spec_height_link(link) < usize::MAX as nat,
            ensures result == spec_height_link(link),
            decreases *link;
        fn rotate_left(link: &mut Link<K, V>);
        fn rotate_right(link: &mut Link<K, V>);
        fn insert_link(link: &mut Link<K, V>, key: K, value: V, priority: u64) -> (inserted: bool)
            decreases old(link);
        fn find_link<'a>(link: &'a Link<K, V>, key: &K) -> Option<&'a V>
            decreases *link;
        fn min_key_link(link: &Link<K, V>) -> Option<&K>
            decreases *link;
        fn max_key_link(link: &Link<K, V>) -> Option<&K>
            decreases *link;
        fn collect_keys(link: &Link<K, V>, out: &mut Vec<K>)
            decreases *link;
        fn collect_values(link: &Link<K, V>, out: &mut Vec<V>)
            decreases *link;
        fn collect_in_order_kvp(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>)
            decreases *link;
        fn find_min_priority_idx_kvp(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (result: usize)
            requires start < end, end <= items.len(),
            ensures start <= result && result < end;
        fn build_treap_from_vec(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (result: Link<K, V>)
            requires start <= end, end <= items.len(),
            decreases end - start;
        fn filter_by_key_kvp(
            items: &Vec<(K, V, u64)>, key: &K,
        ) -> (result: Vec<(K, V, u64)>);
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> BSTKeyValueStEphTrait<K, V> for BSTKeyValueStEph<K, V> {
        open spec fn spec_size(&self) -> nat { self.size as nat }
        open spec fn spec_height(&self) -> nat { spec_height_link(&self.root) }

        fn new() -> (result: Self) { BSTKeyValueStEph { root: None, size: 0 } }

        fn size(&self) -> (result: usize) { self.size }

        fn is_empty(&self) -> (result: bool) { self.size == 0 }

        fn height(&self) -> (result: usize) { Self::height_link(&self.root) }

        fn insert(&mut self, key: K, value: V, priority: u64) {
            let inserted = Self::insert_link(&mut self.root, key, value, priority);
            if inserted {
                self.size = self.size + 1;
            }
        }

        fn delete(&mut self, key: &K) {
            let mut in_order: Vec<(K, V, u64)> = Vec::new();
            Self::collect_in_order_kvp(&self.root, &mut in_order);
            let filtered = Self::filter_by_key_kvp(&in_order, key);
            self.root = Self::build_treap_from_vec(&filtered, 0, filtered.len());
            self.size = filtered.len();
        }

        fn find(&self, key: &K) -> Option<&V> { Self::find_link(&self.root, key) }

        fn contains(&self, key: &K) -> bool { self.find(key).is_some() }

        fn get(&self, key: &K) -> Option<&V> { self.find(key) }

        fn keys(&self) -> ArraySeqStPerS<K> {
            let mut out = Vec::new();
            Self::collect_keys(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn values(&self) -> ArraySeqStPerS<V> {
            let mut out = Vec::new();
            Self::collect_values(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn minimum_key(&self) -> Option<&K> { Self::min_key_link(&self.root) }

        fn maximum_key(&self) -> Option<&K> { Self::max_key_link(&self.root) }

        // Internal associated functions.

        fn new_node(key: K, value: V, priority: u64) -> Node<K, V> {
            Node { key, value, priority, left: None, right: None }
        }

        fn height_link(link: &Link<K, V>) -> (result: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    let l = Self::height_link(&node.left);
                    let r = Self::height_link(&node.right);
                    1 + if l >= r { l } else { r }
                }
            }
        }

        fn rotate_left(link: &mut Link<K, V>) {
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

        fn rotate_right(link: &mut Link<K, V>) {
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

        fn insert_link(link: &mut Link<K, V>, key: K, value: V, priority: u64) -> (inserted: bool)
            decreases old(link),
        {
            if let Some(mut node) = link.take() {
                if key < node.key {
                    let inserted = Self::insert_link(&mut node.left, key, value, priority);
                    *link = Some(node);
                    let need_rotate = match link.as_ref().unwrap().left.as_ref() {
                        Some(left) => left.priority < link.as_ref().unwrap().priority,
                        None => false,
                    };
                    if need_rotate {
                        Self::rotate_right(link);
                    }
                    inserted
                } else if key > node.key {
                    let inserted = Self::insert_link(&mut node.right, key, value, priority);
                    *link = Some(node);
                    let need_rotate = match link.as_ref().unwrap().right.as_ref() {
                        Some(right) => right.priority < link.as_ref().unwrap().priority,
                        None => false,
                    };
                    if need_rotate {
                        Self::rotate_left(link);
                    }
                    inserted
                } else {
                    node.value = value;
                    *link = Some(node);
                    false
                }
            } else {
                *link = Some(Box::new(Self::new_node(key, value, priority)));
                true
            }
        }

        fn find_link<'a>(link: &'a Link<K, V>, key: &K) -> Option<&'a V>
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => {
                    if *key == node.key {
                        Some(&node.value)
                    } else if *key < node.key {
                        Self::find_link(&node.left, key)
                    } else {
                        Self::find_link(&node.right, key)
                    }
                }
            }
        }

        fn min_key_link(link: &Link<K, V>) -> Option<&K>
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_key_link(&node.left),
                },
            }
        }

        fn max_key_link(link: &Link<K, V>) -> Option<&K>
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_key_link(&node.right),
                },
            }
        }

        fn collect_keys(link: &Link<K, V>, out: &mut Vec<K>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_keys(&node.left, out);
                out.push(node.key.clone());
                Self::collect_keys(&node.right, out);
            }
        }

        fn collect_values(link: &Link<K, V>, out: &mut Vec<V>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_values(&node.left, out);
                out.push(node.value.clone());
                Self::collect_values(&node.right, out);
            }
        }

        fn collect_in_order_kvp(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_in_order_kvp(&node.left, out);
                out.push((node.key.clone(), node.value.clone(), node.priority));
                Self::collect_in_order_kvp(&node.right, out);
            }
        }

        fn find_min_priority_idx_kvp(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (result: usize) {
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

        fn build_treap_from_vec(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (result: Link<K, V>)
            decreases end - start,
        {
            if start >= end {
                return None;
            }
            let min_idx = Self::find_min_priority_idx_kvp(items, start, end);
            let key = items[min_idx].0.clone();
            let value = items[min_idx].1.clone();
            let priority = items[min_idx].2;
            let left = Self::build_treap_from_vec(items, start, min_idx);
            let right = Self::build_treap_from_vec(items, min_idx + 1, end);
            let mut node = Self::new_node(key, value, priority);
            node.left = left;
            node.right = right;
            Some(Box::new(node))
        }

        fn filter_by_key_kvp(
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
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Default for BSTreeKeyValue<K, V> {
        fn default() -> Self { Self::new() }
    }

    }

    // 13. derive impls outside verus!

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

    // 12. macros

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
