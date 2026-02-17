//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral AVL-balanced binary search tree with `find` support and public traversal.

pub mod BSTAVLStEph {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone, Debug)]
    struct Node<T: StT + Ord> {
        key: T,
        height: i32,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn new_node<T: StT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            height: 1,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn height_link<T: StT + Ord>(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }

    fn size_link<T: StT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    fn update<T: StT + Ord>(node: &mut Node<T>) {
        node.height = 1 + height_link(&node.left).max(height_link(&node.right));
        node.size = 1 + size_link(&node.left) + size_link(&node.right);
    }

    fn rotate_right<T: StT + Ord>(link: &mut Link<T>) {
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

    fn rotate_left<T: StT + Ord>(link: &mut Link<T>) {
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

    fn rebalance<T: StT + Ord>(link: &mut Link<T>) {
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

    fn insert_link<T: StT + Ord>(link: &mut Link<T>, value: T) {
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
    pub struct BSTAVLStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeAVL<T> = BSTAVLStEph<T>;

    pub trait BSTAVLStEphTrait<T: StT + Ord> {
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

    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {
        fn new() -> Self { BSTAVLStEph { root: None } }

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

        fn insert(&mut self, value: T) { insert_link(&mut self.root, value); }

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

    impl<T: StT + Ord> Default for BSTAVLStEph<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTAVLStEphLit {
        () => {
            < $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEph<_> as $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEph<_> as $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
