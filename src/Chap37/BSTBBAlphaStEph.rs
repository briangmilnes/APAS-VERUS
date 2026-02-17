//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral weight-balanced (BB[α]) binary search tree with `find` support.

pub mod BSTBBAlphaStEph {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    const ALPHA: f64 = 0.75;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StT + Ord> {
        key: T,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn new_node<T: StT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn size_link<T: StT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    fn update<T: StT + Ord>(node: &mut Node<T>) { node.size = 1 + size_link(&node.left) + size_link(&node.right); }

    fn insert_link<T: StT + Ord>(link: &mut Link<T>, value: T) -> bool {
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

    fn needs_rebuild<T: StT + Ord>(node: &Node<T>) -> bool {
        let total = node.size as f64;
        let left = size_link(&node.left) as f64;
        let right = size_link(&node.right) as f64;
        left > ALPHA * total || right > ALPHA * total
    }

    fn rebalance_if_needed<T: StT + Ord>(link: &mut Link<T>, total_size: N) {
        if let Some(node) = link.as_ref() {
            if needs_rebuild(node) {
                let mut values = Vec::with_capacity(total_size);
                collect_values(&Some(node.clone()), &mut values);
                *link = build_balanced(&values);
            }
        }
    }

    fn collect_values<T: StT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            collect_values(&node.left, out);
            out.push(node.key.clone());
            collect_values(&node.right, out);
        }
    }

    fn build_balanced<T: StT + Ord>(values: &[T]) -> Link<T> {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        let mut node = Box::new(new_node(values[mid].clone()));
        node.left = build_balanced(&values[..mid]);
        node.right = build_balanced(&values[mid + 1..]);
        update(&mut node);
        Some(node)
    }

    fn find_link<'a, T: StT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
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

    fn min_link<T: StT + Ord>(link: &Link<T>) -> Option<&T> {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => Some(&node.key),
                | Some(_) => min_link(&node.left),
            },
        }
    }

    fn max_link<T: StT + Ord>(link: &Link<T>) -> Option<&T> {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => Some(&node.key),
                | Some(_) => max_link(&node.right),
            },
        }
    }

    fn in_order_collect<T: StT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: StT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTBBAlphaStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;

    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, value: T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, target: &T)     -> Option<&T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn contains(&self, target: &T) -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn minimum(&self)              -> Option<&T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn maximum(&self)              -> Option<&T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {
        fn new() -> Self { BSTBBAlphaStEph { root: None } }

        fn size(&self) -> N { size_link(&self.root) }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N {
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }
            height_rec(&self.root)
        }

        fn insert(&mut self, value: T) {
            let inserted = insert_link(&mut self.root, value);
            if inserted {
                let total = size_link(&self.root);
                rebalance_if_needed(&mut self.root, total);
            }
        }

        fn find(&self, target: &T) -> Option<&T> { find_link(&self.root, target) }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn minimum(&self) -> Option<&T> { min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            pre_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTBBAlphaStEphLit {
        () => {
            < $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEph<_> as $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEph<_> as $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
