//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Key-Value BST (dictionary/table) with ephemeral treap structure.

pub mod BSTKeyValueStEph {

    use rand::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<K, V> = Option<Box<Node<K, V>>>;

    #[derive(Clone, Debug)]
    struct Node<K: StT + Ord, V: StT> {
        key: K,
        value: V,
        priority: u64,
        left: Link<K, V>,
        right: Link<K, V>,
    }

    /// - APAS: N/A — internal node constructor.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn new_node<K: StT + Ord, V: StT>(key: K, value: V, priority: u64) -> Node<K, V> {
        Node {
            key,
            value,
            priority,
            left: None,
            right: None,
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTKeyValueStEph<K: StT + Ord, V: StT> {
        root: Link<K, V>,
        size: N,
    }

    pub type BSTreeKeyValue<K, V> = BSTKeyValueStEph<K, V>;

    pub trait BSTKeyValueStEphTrait<K: StT + Ord, V: StT> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                    -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)              -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)          -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)            -> N;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn insert(&mut self, key: K, value: V);
        /// claude-4-sonet: Work Θ(n), Span Θ(n) — in-order filter + rebuild
        fn delete(&mut self, key: &K);
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn find(&self, key: &K)     -> Option<&V>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn contains(&self, key: &K) -> B;
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

    /// - APAS: N/A — internal recursive insert helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
    fn insert_link<K: StT + Ord, V: StT>(link: &mut Link<K, V>, key: K, value: V, rng: &mut impl Rng) -> bool {
        if let Some(node) = link.as_mut() {
            if key < node.key {
                let inserted = insert_link(&mut node.left, key, value, rng);
                if node.left.as_ref().is_some_and(|left| left.priority < node.priority) {
                    rotate_right(link);
                }
                inserted
            } else if key > node.key {
                let inserted = insert_link(&mut node.right, key, value, rng);
                if node.right.as_ref().is_some_and(|right| right.priority < node.priority) {
                    rotate_left(link);
                }
                inserted
            } else {
                // Key exists, update value
                node.value = value;
                false // No new insertion
            }
        } else {
            *link = Some(Box::new(new_node(key, value, rng.random())));
            true // New insertion
        }
    }

    /// - APAS: N/A — internal recursive find helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
    fn find_link<'a, K: StT + Ord, V: StT>(link: &'a Link<K, V>, key: &K) -> Option<&'a V> {
        match link {
            | None => None,
            | Some(node) => {
                if key == &node.key {
                    Some(&node.value)
                } else if key < &node.key {
                    find_link(&node.left, key)
                } else {
                    find_link(&node.right, key)
                }
            }
        }
    }

    /// - APAS: N/A — internal recursive min-key helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
    fn min_key_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> Option<&K> {
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
    fn max_key_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> Option<&K> {
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
    fn collect_keys<K: StT + Ord, V: StT>(link: &Link<K, V>, out: &mut Vec<K>) {
        if let Some(node) = link {
            collect_keys(&node.left, out);
            out.push(node.key.clone());
            collect_keys(&node.right, out);
        }
    }

    /// - APAS: N/A — internal recursive value collection helper.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn collect_values<K: StT + Ord, V: StT>(link: &Link<K, V>, out: &mut Vec<V>) {
        if let Some(node) = link {
            collect_values(&node.left, out);
            out.push(node.value.clone());
            collect_values(&node.right, out);
        }
    }

    /// - APAS: N/A — in-order collect (key, value, priority) for rebuild.
    fn collect_in_order_kvp<K: StT + Ord, V: StT>(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>) {
        if let Some(node) = link {
            collect_in_order_kvp(&node.left, out);
            out.push((node.key.clone(), node.value.clone(), node.priority));
            collect_in_order_kvp(&node.right, out);
        }
    }

    /// - APAS: N/A — build treap from sorted (key, value, priority) sequence.
    fn build_treap_from_sorted<K: StT + Ord, V: StT>(
        seq: &[(K, V, u64)],
    ) -> Link<K, V> {
        if seq.is_empty() {
            return None;
        }
        let min_idx = seq
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.2.cmp(&b.2))
            .map(|(i, _)| i)
            .unwrap();
        let (key, value, priority) = seq[min_idx].clone();
        let left_seq = &seq[..min_idx];
        let right_seq = &seq[min_idx + 1..];
        let left = build_treap_from_sorted(left_seq);
        let right = build_treap_from_sorted(right_seq);
        let mut node = new_node(key, value, priority);
        node.left = left;
        node.right = right;
        Some(Box::new(node))
    }

    impl<K: StT + Ord, V: StT> BSTKeyValueStEphTrait<K, V> for BSTKeyValueStEph<K, V> {
        fn new() -> Self { BSTKeyValueStEph { root: None, size: 0 } }

        fn size(&self) -> N { self.size }

        fn is_empty(&self) -> B { self.size == 0 }

        fn height(&self) -> N {
            fn height_rec<K: StT + Ord, V: StT>(link: &Link<K, V>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }
            height_rec(&self.root)
        }

        fn insert(&mut self, key: K, value: V) {
            let mut r = rng();
            let inserted = insert_link(&mut self.root, key, value, &mut r);
            if inserted {
                self.size += 1;
            }
        }

        fn delete(&mut self, key: &K) {
            let mut in_order: Vec<(K, V, u64)> = Vec::new();
            collect_in_order_kvp(&self.root, &mut in_order);
            let filtered: Vec<(K, V, u64)> = in_order.into_iter().filter(|(k, _, _)| k != key).collect();
            self.root = build_treap_from_sorted(&filtered);
            self.size = filtered.len();
        }

        fn find(&self, key: &K) -> Option<&V> { find_link(&self.root, key) }

        fn contains(&self, key: &K) -> B { self.find(key).is_some() }

        fn get(&self, key: &K) -> Option<&V> { self.find(key) }

        fn keys(&self) -> ArraySeqStPerS<K> {
            let mut out = Vec::with_capacity(self.size);
            collect_keys(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn values(&self) -> ArraySeqStPerS<V> {
            let mut out = Vec::with_capacity(self.size);
            collect_values(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn minimum_key(&self) -> Option<&K> { min_key_link(&self.root) }

        fn maximum_key(&self) -> Option<&K> { max_key_link(&self.root) }
    }

    impl<K: StT + Ord, V: StT> Default for BSTreeKeyValue<K, V> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTKeyValueStEphLit {
        () => {
            < $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEph<_, _> as $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEphTrait<_, _> >::new()
        };
        ( $( ($k:expr, $v:expr) ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEph<_, _> as $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEphTrait<_, _> >::new();
            $( __tree.insert($k, $v); )*
            __tree
        }};
    }
}
