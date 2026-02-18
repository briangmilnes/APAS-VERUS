//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Treap (randomized heap-ordered BST) with interior locking for multi-threaded access.

pub mod BSTTreapMtEph {

    use std::sync::{Arc, RwLock};

    use rand::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        priority: u64,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StTInMtT + Ord> Node<T> {
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
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;

    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&self, value: T);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, target: &T)     -> Option<T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, target: &T) -> B;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn minimum(&self)              -> Option<T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn maximum(&self)              -> Option<T>;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
    }


    impl<T: StTInMtT + Ord> BSTTreapMtEph<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right); }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn rotate_left(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.right.take() {
                    x.right = y.left.take();
                    Self::update(&mut x);
                    Self::update(&mut y);
                    y.left = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.left.take() {
                    x.left = y.right.take();
                    Self::update(&mut x);
                    Self::update(&mut y);
                    y.right = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
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
                    Self::update(node);
                }
            } else {
                *link = Some(Box::new(Node::new(value, rng.random())));
            }
        }

        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
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

        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn min_link(link: &Link<T>) -> Option<&T> {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_link(&node.left),
                },
            }
        }

        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn max_link(link: &Link<T>) -> Option<&T> {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_link(&node.right),
                },
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                Self::in_order_collect(&node.left, out);
                out.push(node.key.clone());
                Self::in_order_collect(&node.right, out);
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                out.push(node.key.clone());
                Self::pre_order_collect(&node.left, out);
                Self::pre_order_collect(&node.right, out);
            }
        }
    }

    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {
        fn new() -> Self {
            BSTTreapMtEph {
                root: Arc::new(RwLock::new(None)),
            }
        }

        fn insert(&self, value: T) {
            let mut guard = self.root.write().unwrap();
            let mut rng = rng();
            Self::insert_link(&mut *guard, value, &mut rng);
        }

        fn find(&self, target: &T) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::find_link(&*guard, target).cloned()
        }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            Self::size_link(&*guard)
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
            Self::min_link(&*guard).cloned()
        }

        fn maximum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::max_link(&*guard).cloned()
        }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::in_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::pre_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTTreapMtEphLit {
        () => {
            < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
