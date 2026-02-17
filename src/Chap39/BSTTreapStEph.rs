//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Treap (randomized heap-ordered BST) with `find` support.

pub mod BSTTreapStEph {

    use rand::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone, Debug)]
    struct Node<T: StT + Ord> {
        key: T,
        priority: u64,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn new_node<T: StT + Ord>(key: T, priority: u64) -> Node<T> {
        Node {
            key,
            priority,
            size: 1,
            left: None,
            right: None,
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTTreapStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeTreap<T> = BSTTreapStEph<T>;

    pub trait BSTTreapStEphTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N;
        /// claude-4-sonet: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected, Parallelism Θ(1)
        fn insert(&mut self, value: T);
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
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
    }

    fn size_link<T: StT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    fn update<T: StT + Ord>(node: &mut Node<T>) { node.size = 1 + size_link(&node.left) + size_link(&node.right); }

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

    fn rotate_right<T: StT + Ord>(link: &mut Link<T>) {
        if let Some(mut x) = link.take() {
            if let Some(mut y) = x.left.take() {
                x.left = y.right.take();
                update(&mut x);
                update(&mut y);
                y.right = Some(x);
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    fn insert_link<T: StT + Ord>(link: &mut Link<T>, value: T, rng: &mut impl Rng) {
        if let Some(node) = link.as_mut() {
            if value < node.key {
                insert_link(&mut node.left, value, rng);
                if node.left.as_ref().is_some_and(|left| left.priority < node.priority) {
                    rotate_right(link);
                }
            } else if value > node.key {
                insert_link(&mut node.right, value, rng);
                if node.right.as_ref().is_some_and(|right| right.priority < node.priority) {
                    rotate_left(link);
                }
            }
            if let Some(node) = link.as_mut() {
                update(node);
            }
        } else {
            *link = Some(Box::new(new_node(value, rng.random())));
        }
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

    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        fn new() -> Self { BSTTreapStEph { root: None } }

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
            let mut r = rng();
            insert_link(&mut self.root, value, &mut r);
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

    impl<T: StT + Ord> Default for BSTreeTreap<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTTreapStEphLit {
        () => {
            < $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
