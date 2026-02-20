//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric single-threaded BST built around a joinMid interface.

pub mod BSTParaStEph {

    use std::cell::RefCell;
    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt;
    use std::rc::Rc;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Debug, Clone, Default)]
    pub enum Exposed<T: StT + Ord> {
        #[default]
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    #[derive(Debug, Clone)]
    struct NodeInner<T: StT + Ord> {
        key: T,
        size: N,
        left: ParamBST<T>,
        right: ParamBST<T>,
    }

    #[derive(Debug, Clone)]
    pub struct ParamBST<T: StT + Ord> {
        root: Rc<RefCell<Option<Box<NodeInner<T>>>>>,
    }

    pub trait ParamBSTTrait<T: StT + Ord>: Sized {
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                         -> Self;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose(&self)                 -> Exposed<T>;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn join_mid(exposed: Exposed<T>) -> Self;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                   -> N;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)               -> B;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn insert(&self, key: T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn delete(&self, key: &T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn find(&self, key: &T)          -> Option<T>;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn split(&self, key: &T)         -> (Self, B, Self);
        /// - APAS: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|))
        /// - Claude-Opus-4.6: Work Θ(log(|t1| + |t2|)), Span Θ(log(|t1| + |t2|))
        fn join_pair(&self, other: Self) -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential, no parallelism
        /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(m · lg(n/m)) — sequential variant
        fn union(&self, other: &Self)    -> Self;
        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self)               -> ArraySeqStPerS<T>;
    }

    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn expose_internal<T: StT + Ord>(tree: &ParamBST<T>) -> Exposed<T> {
        let guard = tree.root.borrow();
        match &*guard {
            | None => Exposed::Leaf,
            | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
        }
    }

    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn join_mid<T: StT + Ord>(exposed: Exposed<T>) -> ParamBST<T> {
        match exposed {
            | Exposed::Leaf => ParamBST {
                root: Rc::new(RefCell::new(None)),
            },
            | Exposed::Node(left, key, right) => {
                let size = 1 + left.size() + right.size();
                ParamBST {
                    root: Rc::new(RefCell::new(Some(Box::new(NodeInner { key, size, left, right })))),
                }
            }
        }
    }

    /// - APAS: Work O(lg |t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    fn split_inner<T: StT + Ord>(tree: &ParamBST<T>, key: &T) -> (ParamBST<T>, B, ParamBST<T>) {
        match expose_internal(tree) {
            | Exposed::Leaf => (
                ParamBST {
                    root: Rc::new(RefCell::new(None)),
                },
                false,
                ParamBST {
                    root: Rc::new(RefCell::new(None)),
                },
            ),
            | Exposed::Node(left, root_key, right) => match key.cmp(&root_key) {
                | Less => {
                    let (ll, found, lr) = split_inner(&left, key);
                    let rebuilt = join_mid(Exposed::Node(lr, root_key, right));
                    (ll, found, rebuilt)
                }
                | Greater => {
                    let (rl, found, rr) = split_inner(&right, key);
                    let rebuilt = join_mid(Exposed::Node(left, root_key, rl));
                    (rebuilt, found, rr)
                }
                | Equal => (left, true, right),
            },
        }
    }

    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn join_m<T: StT + Ord>(left: ParamBST<T>, key: T, right: ParamBST<T>) -> ParamBST<T> {
        join_mid(Exposed::Node(left, key, right))
    }

    /// - APAS: Work O(lg |t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    fn min_key<T: StT + Ord>(tree: &ParamBST<T>) -> Option<T> {
        match expose_internal(tree) {
            | Exposed::Leaf => None,
            | Exposed::Node(left, key, _) => match min_key(&left) {
                | Some(rec) => Some(rec),
                | None => Some(key),
            },
        }
    }

    /// - APAS: Work O(lg(|left| + |right|)), Span O(lg(|left| + |right|))
    /// - Claude-Opus-4.6: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|))
    fn join_pair_inner<T: StT + Ord>(left: ParamBST<T>, right: ParamBST<T>) -> ParamBST<T> {
        match expose_internal(&right) {
            | Exposed::Leaf => left,
            | Exposed::Node(_, key, _) => {
                let min_k = min_key(&right).unwrap_or(key);
                let (_, _, reduced_right) = split_inner(&right, &min_k);
                join_m(left, min_k, reduced_right)
            }
        }
    }

    /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential, no parallelism
    /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(m · lg(n/m)) — sequential variant
    fn union_inner<T: StT + Ord>(a: &ParamBST<T>, b: &ParamBST<T>) -> ParamBST<T> {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) => b.clone(),
            | (_, Exposed::Leaf) => a.clone(),
            | (Exposed::Node(al, ak, ar), _) => {
                let (bl, _, br) = split_inner(b, &ak);
                let left_union = union_inner(&al, &bl);
                let right_union = union_inner(&ar, &br);
                join_m(left_union, ak, right_union)
            }
        }
    }

    /// - APAS: Work O(|t|), Span O(|t|)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn collect_in_order<T: StT + Ord>(tree: &ParamBST<T>, out: &mut Vec<T>) {
        match expose_internal(tree) {
            | Exposed::Leaf => {}
            | Exposed::Node(left, key, right) => {
                collect_in_order(&left, out);
                out.push(key);
                collect_in_order(&right, out);
            }
        }
    }

    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {
        fn new() -> Self {
            ParamBST {
                root: Rc::new(RefCell::new(None)),
            }
        }

        fn expose(&self) -> Exposed<T> { expose_internal(self) }

        fn join_mid(exposed: Exposed<T>) -> Self { join_mid(exposed) }

        fn size(&self) -> N { self.root.borrow().as_ref().map_or(0, |node| node.size) }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn insert(&self, key: T) {
            let (left, _, right) = split_inner(self, &key);
            let rebuilt = join_m(left, key, right);
            let new_state = { rebuilt.root.borrow().clone() };
            *self.root.borrow_mut() = new_state;
        }

        fn delete(&self, key: &T) {
            let (left, _, right) = split_inner(self, key);
            let merged = join_pair_inner(left, right);
            let new_state = { merged.root.borrow().clone() };
            *self.root.borrow_mut() = new_state;
        }

        fn find(&self, key: &T) -> Option<T> {
            match expose_internal(self) {
                | Exposed::Leaf => None,
                | Exposed::Node(left, root_key, right) => match key.cmp(&root_key) {
                    | Less => ParamBSTTrait::find(&left, key),
                    | Greater => ParamBSTTrait::find(&right, key),
                    | Equal => Some(root_key),
                },
            }
        }

        fn split(&self, key: &T) -> (Self, B, Self) { split_inner(self, key) }

        fn join_pair(&self, other: Self) -> Self { join_pair_inner(self.clone(), other) }

        fn union(&self, other: &Self) -> Self { union_inner(self, other) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    #[macro_export]
    macro_rules! ParamBSTLit {
        () => {
            < $crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBST<_> as $crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBSTTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBST<_> as
                           $crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBSTTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
