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

    pub(crate) struct Node<K: StT + Ord, V: StT, R: StT> {
        pub(crate) key: K,
        pub(crate) value: V,
        pub(crate) priority: u64,
        pub(crate) size: usize,
        pub(crate) reduced_value: R,
        pub(crate) left: Link<K, V, R>,
        pub(crate) right: Link<K, V, R>,
    }

    fn clone_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> (result: Link<K, V, R>)
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => Some(Box::new(Node {
                key: node.key.clone(),
                value: node.value.clone(),
                priority: node.priority,
                size: node.size,
                reduced_value: node.reduced_value.clone(),
                left: clone_link(&node.left),
                right: clone_link(&node.right),
            })),
        }
    }

    impl<K: StT + Ord, V: StT, R: StT> Clone for Node<K, V, R> {
        fn clone(&self) -> Self {
            Node {
                key: self.key.clone(),
                value: self.value.clone(),
                priority: self.priority,
                size: self.size,
                reduced_value: self.reduced_value.clone(),
                left: clone_link(&self.left),
                right: clone_link(&self.right),
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
        #[verifier::external_body] // accept hole
        fn combine(a: T, b: T) -> T { a + b }
        fn lift(value: &T) -> T { *value }
    }

    /// Example: Count reduction (counts number of elements)
    pub struct CountOp<T>(PhantomData<T>);

    impl<T> Clone for CountOp<T> {
        fn clone(&self) -> Self { CountOp(PhantomData) }
    }

    impl<T: StT> ReduceOp<T, usize> for CountOp<T> {
        fn identity() -> usize { 0 }
        #[verifier::external_body] // accept hole
        fn combine(a: usize, b: usize) -> usize { a + b }
        fn lift(_value: &T) -> usize { 1 }
    }

    pub struct BSTReducedStEph<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> {
        pub(crate) root: Link<K, V, R>,
        pub(crate) _op: PhantomData<Op>,
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
        spec fn spec_size(&self) -> nat;
        spec fn spec_wf(&self) -> bool;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                                  -> Self
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                            -> usize;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                        -> bool;
        spec fn spec_height(&self) -> nat;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)                          -> usize
            requires self.spec_height() < usize::MAX as nat;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn insert(&mut self, key: K, value: V, priority: u64)
            requires old(self).spec_size() + 1 <= usize::MAX as nat, old(self).spec_wf();
        /// claude-4-sonet: Work Θ(n), Span Θ(n) — in-order filter + rebuild
        fn delete(&mut self, key: &K);
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn find(&self, key: &K)                   -> Option<&V>;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn contains(&self, key: &K)               -> bool;
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

    pub(crate) open spec fn spec_size_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> nat {
        match link {
            None => 0,
            Some(n) => n.size as nat,
        }
    }

    pub(crate) open spec fn spec_size_wf_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(n) => {
                n.size as nat == 1 + spec_size_link(&n.left) + spec_size_link(&n.right)
                && spec_size_wf_link(&n.left)
                && spec_size_wf_link(&n.right)
            }
        }
    }

    proof fn lemma_wf_assemble<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>)
        requires match link {
            None => true,
            Some(n) => {
                n.size as nat == 1 + spec_size_link(&n.left) + spec_size_link(&n.right)
                && spec_size_wf_link(&n.left)
                && spec_size_wf_link(&n.right)
            }
        },
        ensures spec_size_wf_link(link),
    {}

    fn size_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> (result: usize)
        ensures result as nat == spec_size_link(link),
    {
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1) — reads augmented reduced value.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn reduced_value_link<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(link: &Link<K, V, R>) -> R {
        match link.as_ref() {
            None => Op::identity(),
            Some(n) => n.reduced_value.clone(),
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1) — recomputes size and reduced value from children.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn update_node<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(node: &mut Node<K, V, R>)
        requires
            1 + spec_size_link(&old(node).left) + spec_size_link(&old(node).right) <= usize::MAX as nat,
            spec_size_wf_link(&old(node).left),
            spec_size_wf_link(&old(node).right),
        ensures
            node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
            spec_size_wf_link(&node.left),
            spec_size_wf_link(&node.right),
            node.key == old(node).key,
            node.left == old(node).left,
            node.right == old(node).right,
    {
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
    ) -> (result: Link<K, V, R>)
        requires
            1 + spec_size_link(&left) + spec_size_link(&right) <= usize::MAX as nat,
            spec_size_wf_link(&left),
            spec_size_wf_link(&right),
        ensures
            spec_size_link(&result) == 1 + spec_size_link(&left) + spec_size_link(&right),
            spec_size_wf_link(&result),
    {
        let ghost left_sz = spec_size_link(&left);
        let ghost right_sz = spec_size_link(&right);
        let node_reduced = Op::lift(&value);
        let mut node = Node::new(key, value, priority, node_reduced);
        node.left = left;
        node.right = right;
        update_node::<K, V, R, Op>(&mut node);
        let result = Some(Box::new(node));
        proof { lemma_wf_assemble(&result); }
        result
    }

    /// - APAS: N/A — internal treap rotation (updates sizes and reduced values).
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(link: &mut Link<K, V, R>)
        requires
            spec_size_link(old(link)) <= usize::MAX as nat,
            spec_size_wf_link(old(link)),
        ensures
            spec_size_link(link) == spec_size_link(old(link)),
            spec_size_wf_link(link),
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
                assert(spec_size_wf_link(&x.left));
                assert(spec_size_wf_link(&x.right));
                assert(1 + x_left_sz + y_left_sz + 1 + y_right_sz <= usize::MAX as nat);
                update_node::<K, V, R, Op>(&mut x);
                y.left = Some(x);
                update_node::<K, V, R, Op>(&mut y);
                *link = Some(y);
                proof { lemma_wf_assemble(link); }
            } else {
                *link = Some(x);
                proof { lemma_wf_assemble(link); }
            }
        }
    }

    /// - APAS: N/A — internal treap rotation (updates sizes and reduced values).
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_right<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(link: &mut Link<K, V, R>)
        requires
            spec_size_link(old(link)) <= usize::MAX as nat,
            spec_size_wf_link(old(link)),
        ensures
            spec_size_link(link) == spec_size_link(old(link)),
            spec_size_wf_link(link),
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
                assert(spec_size_wf_link(&x.left));
                assert(spec_size_wf_link(&x.right));
                assert(1 + y_right_sz + x_right_sz + 1 + y_left_sz <= usize::MAX as nat);
                update_node::<K, V, R, Op>(&mut x);
                y.right = Some(x);
                update_node::<K, V, R, Op>(&mut y);
                *link = Some(y);
                proof { lemma_wf_assemble(link); }
            } else {
                *link = Some(x);
                proof { lemma_wf_assemble(link); }
            }
        }
    }

    /// - APAS: N/A — internal recursive insert helper.
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
    fn insert_link<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(
        link: &mut Link<K, V, R>,
        key: K,
        value: V,
        priority: u64,
    )
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
            if key < node.key {
                insert_link::<K, V, R, Op>(&mut node.left, key, value, priority);
                update_node::<K, V, R, Op>(&mut *node);
                *link = Some(node);
                proof { lemma_wf_assemble(link); }
                let need_rotate = match link.as_ref().unwrap().left.as_ref() {
                    Some(left) => left.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate {
                    rotate_right::<K, V, R, Op>(link);
                }
            } else if key > node.key {
                insert_link::<K, V, R, Op>(&mut node.right, key, value, priority);
                update_node::<K, V, R, Op>(&mut *node);
                *link = Some(node);
                proof { lemma_wf_assemble(link); }
                let need_rotate = match link.as_ref().unwrap().right.as_ref() {
                    Some(right) => right.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate {
                    rotate_left::<K, V, R, Op>(link);
                }
            } else {
                node.value = value;
                update_node::<K, V, R, Op>(&mut *node);
                *link = Some(node);
                proof { lemma_wf_assemble(link); }
            }
        } else {
            let node_reduced = Op::lift(&value);
            *link = Some(Box::new(Node {
                key,
                value,
                priority,
                size: 1,
                reduced_value: node_reduced,
                left: None,
                right: None,
            }));
            proof { lemma_wf_assemble(link); }
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

    pub(crate) open spec fn spec_height_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(n) => {
                let l = spec_height_link(&n.left);
                let r = spec_height_link(&n.right);
                1 + if l >= r { l } else { r }
            }
        }
    }

    fn height_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> (result: usize)
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

    /// - APAS: N/A — build treap from sorted (key, value, priority) sequence.
    fn filter_by_key_kvp_r<K: StT + Ord, V: StT>(
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

    fn find_min_priority_idx_kvp_r<K: StT + Ord, V: StT>(
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

    fn build_treap_from_vec_r<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(
        items: &Vec<(K, V, u64)>, start: usize, end: usize,
    ) -> (result: Link<K, V, R>)
        requires start <= end, end <= items.len(),
        ensures
            spec_size_wf_link(&result),
            spec_size_link(&result) == (end - start) as nat,
        decreases end - start,
    {
        if start >= end {
            return None;
        }
        let min_idx = find_min_priority_idx_kvp_r(items, start, end);
        let key = items[min_idx].0.clone();
        let value = items[min_idx].1.clone();
        let priority = items[min_idx].2;
        let left = build_treap_from_vec_r::<K, V, R, Op>(items, start, min_idx);
        let right = build_treap_from_vec_r::<K, V, R, Op>(items, min_idx + 1, end);
        make_node::<K, V, R, Op>(key, value, priority, left, right)
    }

    /// - APAS: Work Θ(log n), Span Θ(log n) — range query on augmented BST.
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    fn range_reduce_link<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>(
        link: &Link<K, V, R>,
        low: &K,
        high: &K,
    ) -> R
        decreases *link,
    {
        match link {
            | None => Op::identity(),
            | Some(node) => {
                let mut result = Op::identity();

                // Include left subtree if it might contain keys >= low
                if node.key > *low {
                    result = Op::combine(result, range_reduce_link::<K, V, R, Op>(&node.left, low, high));
                }

                if !(node.key < *low) && !(node.key > *high) {
                    result = Op::combine(result, Op::lift(&node.value));
                }

                if node.key < *high {
                    result = Op::combine(result, range_reduce_link::<K, V, R, Op>(&node.right, low, high));
                }

                result
            }
        }
    }

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> BSTReducedStEphTrait<K, V, R, Op>
        for BSTReducedStEph<K, V, R, Op>
    {
        closed spec fn spec_size(&self) -> nat { spec_size_link(&self.root) }
        closed spec fn spec_wf(&self) -> bool { spec_size_wf_link(&self.root) }
        closed spec fn spec_height(&self) -> nat { spec_height_link(&self.root) }

        fn new() -> Self {
            BSTReducedStEph {
                root: None,
                _op: PhantomData,
            }
        }

        fn size(&self) -> usize { size_link(&self.root) }

        fn is_empty(&self) -> bool { self.size() == 0 }

        fn height(&self) -> usize { height_link(&self.root) }

        fn insert(&mut self, key: K, value: V, priority: u64) {
            insert_link::<K, V, R, Op>(&mut self.root, key, value, priority);
        }

        fn delete(&mut self, key: &K) {
            let mut in_order: Vec<(K, V, u64)> = Vec::new();
            collect_in_order_kvp(&self.root, &mut in_order);
            let filtered = filter_by_key_kvp_r(&in_order, key);
            self.root = build_treap_from_vec_r::<K, V, R, Op>(&filtered, 0, filtered.len());
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

        fn reduced_value(&self) -> R { reduced_value_link::<K, V, R, Op>(&self.root) }

        fn range_reduce(&self, low: &K, high: &K) -> R {
            range_reduce_link::<K, V, R, Op>(&self.root, low, high)
        }
    }

    // Type aliases for common reductions
    pub type BSTSumStEph<K, V> = BSTReducedStEph<K, V, V, SumOp<V>>;
    pub type BSTCountStEph<K, V> = BSTReducedStEph<K, V, usize, CountOp<V>>;

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
