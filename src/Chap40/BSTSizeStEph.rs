//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Size-augmented BST with O(1) size queries and rank/select operations.

pub mod BSTSizeStEph {

    use rand::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone, Debug)]
    struct Node<T: StT + Ord> {
        key: T,
        priority: u64,
        size: N, // Size of subtree rooted at this node
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StT + Ord> Node<T> {
        fn new(key: T, priority: u64) -> Self {
            Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTSizeStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeSize<T> = BSTSizeStEph<T>;

    pub trait BSTSizeStEphTrait<T: StT + Ord> {
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
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, key: &T)        -> N;
        fn select(&self, rank: N)      -> Option<&T>;
        fn split_rank(&self, rank: N)  -> (BSTSizeStEph<T>, BSTSizeStEph<T>);
    }

    impl<T: StT + Ord> BSTSizeStEph<T> {
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        fn update_size(node: &mut Node<T>) {
            node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right);
        }

        fn make_node(key: T, priority: u64, left: Link<T>, right: Link<T>) -> Link<T> {
            let mut node = Node::new(key, priority);
            node.left = left;
            node.right = right;
            Self::update_size(&mut node);
            Some(Box::new(node))
        }

        fn rotate_left(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.right.take() {
                    x.right = y.left.take();
                    Self::update_size(&mut x);
                    Self::update_size(&mut y);
                    y.left = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.left.take() {
                    x.left = y.right.take();
                    Self::update_size(&mut x);
                    Self::update_size(&mut y);
                    y.right = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {
            if let Some(node) = link.as_mut() {
                if value < node.key {
                    Self::insert_link(&mut node.left, value, rng);
                    if node.left.as_ref().is_some_and(|left| left.priority < node.priority) {
                        Self::rotate_right(link);
                    }
                } else if value > node.key {
                    Self::insert_link(&mut node.right, value, rng);
                    if node.right.as_ref().is_some_and(|right| right.priority < node.priority) {
                        Self::rotate_left(link);
                    }
                }
                if let Some(node) = link.as_mut() {
                    Self::update_size(node);
                }
            } else {
                *link = Some(Box::new(Node::new(value, rng.random())));
            }
        }

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => {
                    if target == &node.key {
                        Some(&node.key)
                    } else if target < &node.key {
                        Self::find_link(&node.left, target)
                    } else {
                        Self::find_link(&node.right, target)
                    }
                }
            }
        }

        fn min_link(link: &Link<T>) -> Option<&T> {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_link(&node.left),
                },
            }
        }

        fn max_link(link: &Link<T>) -> Option<&T> {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_link(&node.right),
                },
            }
        }

        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                Self::in_order_collect(&node.left, out);
                out.push(node.key.clone());
                Self::in_order_collect(&node.right, out);
            }
        }

        fn rank_link(link: &Link<T>, key: &T) -> N {
            match link {
                | None => 0,
                | Some(node) => {
                    let left_size = Self::size_link(&node.left);
                    if key < &node.key {
                        Self::rank_link(&node.left, key)
                    } else if key == &node.key {
                        left_size + 1
                    } else {
                        left_size + 1 + Self::rank_link(&node.right, key)
                    }
                }
            }
        }

        fn select_link(link: &Link<T>, rank: N) -> Option<&T> {
            match link {
                | None => None,
                | Some(node) => {
                    let left_size = Self::size_link(&node.left);
                    if rank <= left_size {
                        Self::select_link(&node.left, rank)
                    } else if rank == left_size + 1 {
                        Some(&node.key)
                    } else {
                        Self::select_link(&node.right, rank - left_size - 1)
                    }
                }
            }
        }

        fn split_rank_link(link: &Link<T>, rank: N) -> (Link<T>, Link<T>) {
            match link {
                | None => (None, None),
                | Some(node) => {
                    let left_size = Self::size_link(&node.left);
                    if rank <= left_size {
                        let (ll, lr) = Self::split_rank_link(&node.left, rank);
                        let right_tree = Self::make_node(node.key.clone(), node.priority, lr, node.right.clone());
                        (ll, right_tree)
                    } else {
                        let (rl, rr) = Self::split_rank_link(&node.right, rank - left_size - 1);
                        let left_tree = Self::make_node(node.key.clone(), node.priority, node.left.clone(), rl);
                        (left_tree, rr)
                    }
                }
            }
        }
    }

    impl<T: StT + Ord> BSTSizeStEphTrait<T> for BSTSizeStEph<T> {
        fn new() -> Self { BSTSizeStEph { root: None } }

        fn size(&self) -> N { Self::size_link(&self.root) }

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
            Self::insert_link(&mut self.root, value, &mut r);
        }

        fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn rank(&self, key: &T) -> N { Self::rank_link(&self.root, key) }

        fn select(&self, rank: N) -> Option<&T> {
            if rank == 0 || rank > self.size() {
                None
            } else {
                Self::select_link(&self.root, rank)
            }
        }

        fn split_rank(&self, rank: N) -> (BSTSizeStEph<T>, BSTSizeStEph<T>) {
            if rank == 0 {
                (BSTSizeStEph::new(), self.clone())
            } else if rank >= self.size() {
                (self.clone(), BSTSizeStEph::new())
            } else {
                let (left_root, right_root) = Self::split_rank_link(&self.root, rank);
                (BSTSizeStEph { root: left_root }, BSTSizeStEph { root: right_root })
            }
        }
    }

    impl<T: StT + Ord> Default for BSTreeSize<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTSizeStEphLit {
        () => {
            < $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEph<_> as $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEph<_> as $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
