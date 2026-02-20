//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! BST with general reduced values augmentation using associative functions.

pub mod BSTReducedStEph {

    use std::fmt;
    use std::marker::PhantomData;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    type Link<K, V, R> = Option<Box<Node<K, V, R>>>;

    struct Node<K: StT + Ord, V: StT, R: StT> {
        key: K,
        value: V,
        priority: u64,
        size: N,
        reduced_value: R, // Reduced value of subtree rooted at this node
        left: Link<K, V, R>,
        right: Link<K, V, R>,
    }

    impl<K: StT + Ord, V: StT, R: StT> Clone for Node<K, V, R> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            Node {
                key: self.key.clone(),
                value: self.value.clone(),
                priority: self.priority,
                size: self.size,
                reduced_value: self.reduced_value.clone(),
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    trait NodeTrait<K: StT + Ord, V: StT, R: StT>: Sized {
        fn new(key: K, value: V, priority: u64, reduced_value: R) -> Self;
    }

    impl<K: StT + Ord, V: StT, R: StT> NodeTrait<K, V, R> for Node<K, V, R> {
        fn new(key: K, value: V, priority: u64, reduced_value: R) -> Self {
            Node {
                key,
                value,
                priority,
                size: 1,
                reduced_value,
                left: None,
                right: None,
            }
        }
    }

    /// Trait for associative reduction operations
    pub trait ReduceOp<V: StT, R: StT> {
        /// Identity element for the reduction operation
        fn identity()          -> R;
        /// Associative binary operation: f(a, b)
        fn combine(a: R, b: R) -> R;
        /// Convert value to reduced form
        fn lift(value: &V)     -> R;
    }

    /// Example: Sum reduction for numeric values
    pub struct SumOp<T>(PhantomData<T>);

    impl<T> Clone for SumOp<T> {
        fn clone(&self) -> Self { SumOp(PhantomData) }
    }

    impl<T: ArithmeticT> ReduceOp<T, T> for SumOp<T> {
        fn identity() -> T { T::default() }
        #[verifier::external_body]
        fn combine(a: T, b: T) -> T { a + b }
        fn lift(value: &T) -> T { *value }
    }

    /// Example: Count reduction (counts number of elements)
    pub struct CountOp<T>(PhantomData<T>);

    impl<T> Clone for CountOp<T> {
        fn clone(&self) -> Self { CountOp(PhantomData) }
    }

    impl<T: StT> ReduceOp<T, N> for CountOp<T> {
        fn identity() -> N { 0 }
        #[verifier::external_body]
        fn combine(a: N, b: N) -> N { a + b }
        fn lift(_value: &T) -> N { 1 }
    }

    pub struct BSTReducedStEph<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> {
        root: Link<K, V, R>,
        _op: PhantomData<Op>,
    }

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> Clone for BSTReducedStEph<K, V, R, Op> {
        fn clone(&self) -> Self {
            BSTReducedStEph {
                root: self.root.clone(),
                _op: PhantomData,
            }
        }
    }

    pub type BSTreeReduced<K, V, R, Op> = BSTReducedStEph<K, V, R, Op>;

    pub trait BSTReducedStEphTrait<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                                  -> Self
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                        -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)                          -> N;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn insert(&mut self, key: K, value: V, priority: u64);
        /// claude-4-sonet: Work Θ(n), Span Θ(n) — in-order filter + rebuild
        fn delete(&mut self, key: &K);
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn find(&self, key: &K)                   -> Option<&V>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn contains(&self, key: &K)               -> B;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn get(&self, key: &K)                    -> Option<&V>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn keys(&self)                            -> ArraySeqStPerS<K>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn values(&self)                          -> ArraySeqStPerS<V>;
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn minimum_key(&self)                     -> Option<&K>;
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn maximum_key(&self)                     -> Option<&K>;
        /// - APAS: Work Θ(1), Span Θ(1) — reads augmented field at root.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn reduced_value(&self)                   -> R;
        /// - APAS: Work Θ(log n), Span Θ(log n) — range query on augmented BST.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn range_reduce(&self, low: &K, high: &K) -> R;
    }

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> Default for BSTreeReduced<K, V, R, Op> {
        fn default() -> Self { Self::new() }
    }

    /// - APAS: Work Θ(1), Span Θ(1) — O(1) via augmented size field.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn size_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> N {
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1) — reads augmented reduced value.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn reduced_value_link<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(link: &Link<K, V, R>) -> R {
        match link.as_ref() {
            None => Op::identity(),
            Some(n) => n.reduced_value.clone(),
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1) — recomputes size and reduced value from children.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn update_node<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(node: &mut Node<K, V, R>) {
        node.size = 1 + size_link(&node.left) + size_link(&node.right);

        // Compute reduced value: f(left_reduced, f(node_value, right_reduced))
        let left_reduced = reduced_value_link::<K, V, R, Op>(&node.left);
        let right_reduced = reduced_value_link::<K, V, R, Op>(&node.right);
        let node_reduced = Op::lift(&node.value);

        node.reduced_value = Op::combine(left_reduced, Op::combine(node_reduced, right_reduced));
    }

    /// - APAS: Work Θ(1), Span Θ(1) — corresponds to APAS makeNode with reduced values.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn make_node<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(
        key: K,
        value: V,
        priority: u64,
        left: Link<K, V, R>,
        right: Link<K, V, R>,
    ) -> Link<K, V, R> {
        let node_reduced = Op::lift(&value);
        let mut node = Node::new(key, value, priority, node_reduced);
        node.left = left;
        node.right = right;
        update_node::<K, V, R, Op>(&mut node);
        Some(Box::new(node))
    }

    /// - APAS: N/A — internal treap rotation (updates sizes and reduced values).
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(link: &mut Link<K, V, R>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.right.take() {
                x.right = y.left.take();
                update_node::<K, V, R, Op>(&mut x);
                update_node::<K, V, R, Op>(&mut y);
                y.left = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: N/A — internal treap rotation (updates sizes and reduced values).
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_right<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(link: &mut Link<K, V, R>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.left.take() {
                x.left = y.right.take();
                update_node::<K, V, R, Op>(&mut x);
                update_node::<K, V, R, Op>(&mut y);
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
    fn insert_link<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(
        link: &mut Link<K, V, R>,
        key: K,
        value: V,
        priority: u64,
    ) {
        if let Some(node) = link.as_mut() {
            if key < node.key {
                insert_link::<K, V, R, Op>(&mut node.left, key, value, priority);
                if node.left.as_ref().is_some_and(|left| left.priority < node.priority) {
                    rotate_right::<K, V, R, Op>(link);
                }
            } else if key > node.key {
                insert_link::<K, V, R, Op>(&mut node.right, key, value, priority);
                if node.right.as_ref().is_some_and(|right| right.priority < node.priority) {
                    rotate_left::<K, V, R, Op>(link);
                }
            } else {
                node.value = value;
            }
            if let Some(node) = link.as_mut() {
                update_node::<K, V, R, Op>(node);
            }
        } else {
            let node_reduced = Op::lift(&value);
            *link = Some(Box::new(Node::new(key, value, priority, node_reduced)));
        }
    }

    /// - APAS: N/A — internal recursive find helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
    fn find_link<'a, K: StT + Ord, V: StT, R: StT>(
        link: &'a Link<K, V, R>,
        key: &K,
    ) -> Option<&'a V>
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
    fn min_key_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> Option<&K>
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
    fn max_key_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> Option<&K>
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
    fn collect_keys<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>, out: &mut Vec<K>)
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
    fn collect_values<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>, out: &mut Vec<V>)
        decreases *link,
    {
        if let Some(node) = link {
            collect_values(&node.left, out);
            out.push(node.value.clone());
            collect_values(&node.right, out);
        }
    }

    /// - APAS: N/A — in-order collect (key, value, priority) for rebuild.
    fn collect_in_order_kvp<K: StT + Ord, V: StT, R: StT>(
        link: &Link<K, V, R>,
        out: &mut Vec<(K, V, u64)>,
    )
        decreases *link,
    {
        if let Some(node) = link {
            collect_in_order_kvp(&node.left, out);
            out.push((node.key.clone(), node.value.clone(), node.priority));
            collect_in_order_kvp(&node.right, out);
        }
    }

    fn height_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> N
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

    /// - APAS: N/A — build treap from sorted (key, value, priority) sequence.
    #[verifier::external_body]
    fn build_treap_from_sorted<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(
        seq: &[(K, V, u64)],
    ) -> Link<K, V, R> {
        if seq.is_empty() {
            return None;
        }
        let min_idx = seq
            .iter()
            .enumerate()
            .min_by_key(|entry| entry.1.2)
            .map(|entry| entry.0)
            .unwrap();
        let (key, value, priority) = seq[min_idx].clone();
        let left_seq = &seq[..min_idx];
        let right_seq = &seq[min_idx + 1..];
        let left = build_treap_from_sorted::<K, V, R, Op>(left_seq);
        let right = build_treap_from_sorted::<K, V, R, Op>(right_seq);
        make_node::<K, V, R, Op>(key, value, priority, left, right)
    }

    /// - APAS: Work Θ(log n), Span Θ(log n) — range query on augmented BST.
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    #[verifier::external_body]
    fn range_reduce_link<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(
        link: &Link<K, V, R>,
        low: &K,
        high: &K,
    ) -> R {
        match link {
            | None => Op::identity(),
            | Some(node) => {
                let mut result = Op::identity();

                // Include left subtree if it might contain keys >= low
                if &node.key > low {
                    result = Op::combine(result, range_reduce_link::<K, V, R, Op>(&node.left, low, high));
                }

                // Include current node if it's in range
                if &node.key >= low && &node.key <= high {
                    result = Op::combine(result, Op::lift(&node.value));
                }

                // Include right subtree if it might contain keys <= high
                if &node.key < high {
                    result = Op::combine(result, range_reduce_link::<K, V, R, Op>(&node.right, low, high));
                }

                result
            }
        }
    }

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> BSTReducedStEphTrait<K, V, R, Op>
        for BSTReducedStEph<K, V, R, Op>
    {
        fn new() -> Self {
            BSTReducedStEph {
                root: None,
                _op: PhantomData,
            }
        }

        fn size(&self) -> N { size_link(&self.root) }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N { height_link(&self.root) }

        fn insert(&mut self, key: K, value: V, priority: u64) {
            insert_link::<K, V, R, Op>(&mut self.root, key, value, priority);
        }

        #[verifier::external_body]
        fn delete(&mut self, key: &K) {
            let mut in_order: Vec<(K, V, u64)> = Vec::new();
            collect_in_order_kvp(&self.root, &mut in_order);
            let filtered: Vec<(K, V, u64)> = in_order.into_iter().filter(|x| x.0 != *key).collect();
            self.root = build_treap_from_sorted::<K, V, R, Op>(&filtered);
        }

        fn find(&self, key: &K) -> Option<&V> { find_link(&self.root, key) }

        fn contains(&self, key: &K) -> B { self.find(key).is_some() }

        fn get(&self, key: &K) -> Option<&V> { self.find(key) }

        #[verifier::external_body]
        fn keys(&self) -> ArraySeqStPerS<K> {
            let mut out = Vec::with_capacity(self.size());
            collect_keys(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        #[verifier::external_body]
        fn values(&self) -> ArraySeqStPerS<V> {
            let mut out = Vec::with_capacity(self.size());
            collect_values(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn minimum_key(&self) -> Option<&K> { min_key_link(&self.root) }

        fn maximum_key(&self) -> Option<&K> { max_key_link(&self.root) }

        fn reduced_value(&self) -> R { reduced_value_link::<K, V, R, Op>(&self.root) }

        fn range_reduce(&self, low: &K, high: &K) -> R {
            range_reduce_link::<K, V, R, Op>(&self.root, low, high)
        }
    }

    // Type aliases for common reductions
    pub type BSTSumStEph<K, V> = BSTReducedStEph<K, V, V, SumOp<V>>;
    pub type BSTCountStEph<K, V> = BSTReducedStEph<K, V, N, CountOp<V>>;

    }

    impl<T: fmt::Debug> fmt::Debug for SumOp<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("SumOp").field(&self.0).finish()
        }
    }

    impl<T: fmt::Debug> fmt::Debug for CountOp<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("CountOp").field(&self.0).finish()
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug, R: StT + fmt::Debug, Op: ReduceOp<V, R> + fmt::Debug> fmt::Debug
        for BSTReducedStEph<K, V, R, Op>
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTReducedStEph")
                .field("root", &self.root)
                .field("_op", &self._op)
                .finish()
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug, R: StT + fmt::Debug> fmt::Debug for Node<K, V, R> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("value", &self.value)
                .field("priority", &self.priority)
                .field("size", &self.size)
                .field("reduced_value", &self.reduced_value)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    #[macro_export]
    macro_rules! BSTReducedStEphLit {
        () => {
            < $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEph<_, _, _, _> as $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEphTrait<_, _, _, _> >::new()
        };
        ( $( ($k:expr, $v:expr) ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEph<_, _, _, _> as $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEphTrait<_, _, _, _> >::new();
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
