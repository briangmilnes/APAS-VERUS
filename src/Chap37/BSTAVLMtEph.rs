//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral AVL-balanced binary search tree with interior locking for multi-threaded access.

pub mod BSTAVLMtEph {

    use std::sync::{Arc, RwLock};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        height: i32,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn new_node<T: StTInMtT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            height: 1,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn height_link<T: StTInMtT + Ord>(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }

    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    fn update<T: StTInMtT + Ord>(node: &mut Node<T>) {
        node.height = 1 + height_link(&node.left).max(height_link(&node.right));
        node.size = 1 + size_link(&node.left) + size_link(&node.right);
    }

    fn rotate_right<T: StTInMtT + Ord>(link: &mut Link<T>) {
        if let Some(mut y) = link.take() {
            if let Some(mut x) = y.left.take() {
                y.left = x.right.take();
                update(&mut y);
                update(&mut x);
                x.right = Some(y);
                *link = Some(x);
            } else {
                *link = Some(y);
            }
        }
    }

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

    fn rebalance<T: StTInMtT + Ord>(link: &mut Link<T>) {
        if let Some(node) = link.as_mut() {
            update(node);
            let bf = height_link(&node.left) - height_link(&node.right);
            if bf > 1 {
                if let Some(left) = node.left.as_mut() {
                    if height_link(&left.right) > height_link(&left.left) {
                        rotate_left(&mut node.left);
                    }
                }
                rotate_right(link);
            } else if bf < -1 {
                if let Some(right) = node.right.as_mut() {
                    if height_link(&right.left) > height_link(&right.right) {
                        rotate_right(&mut node.right);
                    }
                }
                rotate_left(link);
            }
        }
        if let Some(node) = link.as_mut() {
            update(node);
        }
    }

    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T) {
        match link {
            | Some(node) => {
                if value < node.key {
                    insert_link(&mut node.left, value);
                } else if value > node.key {
                    insert_link(&mut node.right, value);
                } else {
                    return;
                }
            }
            | None => {
                *link = Some(Box::new(new_node(value)));
                return;
            }
        }
        rebalance(link);
    }

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

    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T> {
        match link {
            | None => None,
            | Some(node) => match &node.left {
                | None => Some(&node.key),
                | Some(_) => min_link(&node.left),
            },
        }
    }

    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T> {
        match link {
            | None => None,
            | Some(node) => match &node.right {
                | None => Some(&node.key),
                | Some(_) => max_link(&node.right),
            },
        }
    }

    fn in_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    // Parallel traversals

    fn in_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> Vec<T> {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                use crate::Types::Types::Pair;
                let Pair(left_vec, right_vec) = crate::ParaPair!(
                    move || in_order_parallel(&node.left),
                    move || in_order_parallel(&node.right)
                );
                let mut result = left_vec;
                result.push(node.key.clone());
                result.extend(right_vec);
                result
            }
        }
    }

    fn pre_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> Vec<T> {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                use crate::Types::Types::Pair;
                let Pair(left_vec, right_vec) = crate::ParaPair!(
                    move || pre_order_parallel(&node.left),
                    move || pre_order_parallel(&node.right)
                );
                let mut result = vec![node.key.clone()];
                result.extend(left_vec);
                result.extend(right_vec);
                result
            }
        }
    }

    // Parallel construction from sorted slice
    fn build_balanced<T: StTInMtT + Ord>(values: &[T]) -> Link<T> {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        
        // Parallel construction of left and right subtrees
        use crate::Types::Types::Pair;
        let Pair(left, right) = crate::ParaPair!(
            move || build_balanced(&values[..mid]),
            move || build_balanced(&values[mid + 1..])
        );
        
        let mut node = Box::new(new_node(values[mid].clone()));
        node.left = left;
        node.right = right;
        update(&mut node);
        Some(node)
    }

    // Parallel filter
    fn filter_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, predicate: &std::sync::Arc<F>) -> Vec<T>
    where
        F: Fn(&T) -> bool + Send + Sync,
    {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                let pred_left = std::sync::Arc::clone(predicate);
                let pred_right = std::sync::Arc::clone(predicate);
                
                use crate::Types::Types::Pair;
                let Pair(left_vals, right_vals) = crate::ParaPair!(
                    move || filter_parallel(&node.left, &pred_left),
                    move || filter_parallel(&node.right, &pred_right)
                );
                
                let mut result = left_vals;
                if predicate(&node.key) {
                    result.push(node.key.clone());
                }
                result.extend(right_vals);
                result
            }
        }
    }

    // Parallel reduce
    fn reduce_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, op: &std::sync::Arc<F>, identity: T) -> T
    where
        F: Fn(T, T) -> T + Send + Sync,
    {
        match link {
            | None => identity,
            | Some(node) => {
                let op_left = std::sync::Arc::clone(op);
                let op_right = std::sync::Arc::clone(op);
                let id_left = identity.clone();
                
                use crate::Types::Types::Pair;
                let Pair(left_acc, right_acc) = crate::ParaPair!(
                    move || reduce_parallel(&node.left, &op_left, id_left),
                    move || reduce_parallel(&node.right, &op_right, identity)
                );
                
                let with_key = op(left_acc, node.key.clone());
                op(with_key, right_acc)
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;

    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel construction from sorted slice
        fn from_sorted_slice(values: &[T]) -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn insert(&self, value: T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn find(&self, target: &T)     -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn contains(&self, target: &T) -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn minimum(&self)              -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn maximum(&self)              -> Option<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel traversal
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel traversal
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel filter
        fn filter<F>(&self, predicate: F) -> ArraySeqStPerS<T>
        where
            F: Fn(&T) -> bool + Send + Sync;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel reduce
        fn reduce<F>(&self, op: F, identity: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync;
    }

    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {
        fn new() -> Self {
            BSTAVLMtEph {
                root: Arc::new(RwLock::new(None)),
            }
        }

        fn from_sorted_slice(values: &[T]) -> Self {
            BSTAVLMtEph {
                root: Arc::new(RwLock::new(build_balanced(values))),
            }
        }

        fn insert(&self, value: T) {
            let mut guard = self.root.write().unwrap();
            insert_link(&mut *guard, value);
        }

        fn find(&self, target: &T) -> Option<T> {
            let guard = self.root.read().unwrap();
            find_link(&*guard, target).cloned()
        }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            size_link(&*guard)
        }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N {
            let guard = self.root.read().unwrap();
            height_link(&*guard) as N
        }

        fn minimum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            min_link(&*guard).cloned()
        }

        fn maximum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            max_link(&*guard).cloned()
        }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let out = in_order_parallel(&*guard);
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let out = pre_order_parallel(&*guard);
            ArraySeqStPerS::from_vec(out)
        }

        fn filter<F>(&self, predicate: F) -> ArraySeqStPerS<T>
        where
            F: Fn(&T) -> bool + Send + Sync,
        {
            let guard = self.root.read().unwrap();
            let predicate = std::sync::Arc::new(predicate);
            let out = filter_parallel(&*guard, &predicate);
            ArraySeqStPerS::from_vec(out)
        }

        fn reduce<F>(&self, op: F, identity: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync,
        {
            let guard = self.root.read().unwrap();
            let op = std::sync::Arc::new(op);
            reduce_parallel(&*guard, &op, identity)
        }
    }

    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTAVLMtEphLit {
        () => {
            < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> as $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> as $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
