//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral weight-balanced (BB[α]) binary search tree with interior locking for multi-threaded access.

pub mod BSTBBAlphaMtEph {

    use std::sync::{Arc, RwLock};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    const ALPHA: f64 = 0.75;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone, Debug)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn new_node<T: StTInMtT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    fn update<T: StTInMtT + Ord>(node: &mut Node<T>) { node.size = 1 + size_link(&node.left) + size_link(&node.right); }

    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T) -> bool {
        match link {
            | Some(node) => {
                let inserted = if value < node.key {
                    insert_link(&mut node.left, value)
                } else if value > node.key {
                    insert_link(&mut node.right, value)
                } else {
                    false
                };
                if inserted {
                    update(node);
                }
                inserted
            }
            | None => {
                *link = Some(Box::new(new_node(value)));
                true
            }
        }
    }

    fn needs_rebuild<T: StTInMtT + Ord>(node: &Node<T>) -> bool {
        let total = node.size as f64;
        let left = size_link(&node.left) as f64;
        let right = size_link(&node.right) as f64;
        left > ALPHA * total || right > ALPHA * total
    }

    fn rebalance_if_needed<T: StTInMtT + Ord>(link: &mut Link<T>, total_size: N) {
        if let Some(node) = link.as_ref() {
            if needs_rebuild(node) {
                let mut values = Vec::with_capacity(total_size);
                collect_values(&Some(node.clone()), &mut values);
                *link = build_balanced(&values);
            }
        }
    }

    fn collect_values<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            collect_values(&node.left, out);
            out.push(node.key.clone());
            collect_values(&node.right, out);
        }
    }

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
            | Some(node) => match node.left {
                | None => Some(&node.key),
                | Some(_) => min_link(&node.left),
            },
        }
    }

    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T> {
        match link {
            | None => None,
            | Some(node) => match node.right {
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

    #[derive(Debug, Clone)]
    pub struct BSTBBAlphaMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;

    pub trait BSTBBAlphaMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
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
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
    }

    impl<T: StTInMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {
        fn new() -> Self {
            BSTBBAlphaMtEph {
                root: Arc::new(RwLock::new(None)),
            }
        }

        fn insert(&self, value: T) {
            let mut guard = self.root.write().unwrap();
            let inserted = insert_link(&mut *guard, value);
            if inserted {
                let total = size_link(&*guard);
                rebalance_if_needed(&mut *guard, total);
            }
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
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }

            let guard = self.root.read().unwrap();
            height_rec(&*guard)
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
            let mut out = Vec::with_capacity(size_link(&*guard));
            in_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(size_link(&*guard));
            pre_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    impl<T: StTInMtT + Ord> Default for BSTBBAlphaMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTBBAlphaMtEphLit {
        () => {
            < $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEph<_> as $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEph<_> as $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
