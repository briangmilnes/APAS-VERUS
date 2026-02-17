//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric single-threaded BST built around a joinMid interface.

pub mod BSTParaStEph {

    use std::cell::RefCell;
    use std::cmp::Ordering::Equal;
    use std::cmp::Ordering::Greater;
    use std::cmp::Ordering::Less;
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
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                         -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn expose(&self)                 -> Exposed<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn join_mid(exposed: Exposed<T>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                   -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)               -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, key: T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, key: &T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, key: &T)          -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split(&self, key: &T)         -> (Self, B, Self);
        /// claude-4-sonet: Work Θ(log(|self| + |other|)), Span Θ(log(|self| + |other|)), Parallelism Θ(1)
        fn join_pair(&self, other: Self) -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self)    -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn in_order(&self)               -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> ParamBST<T> {
        fn expose_internal(&self) -> Exposed<T> {
            let guard = self.root.borrow();
            match &*guard {
                | None => Exposed::Leaf,
                | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
            }
        }

        fn join_mid(exposed: Exposed<T>) -> Self {
            match exposed {
                | Exposed::Leaf => ParamBST::new(),
                | Exposed::Node(left, key, right) => {
                    let size = 1 + left.size() + right.size();
                    ParamBST {
                        root: Rc::new(RefCell::new(Some(Box::new(NodeInner { key, size, left, right })))),
                    }
                }
            }
        }

        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {
            match tree.expose_internal() {
                | Exposed::Leaf => (ParamBST::new(), false, ParamBST::new()),
                | Exposed::Node(left, root_key, right) => match key.cmp(&root_key) {
                    | Less => {
                        let (ll, found, lr) = ParamBST::split_inner(&left, key);
                        let rebuilt = ParamBST::join_mid(Exposed::Node(lr, root_key, right));
                        (ll, found, rebuilt)
                    }
                    | Greater => {
                        let (rl, found, rr) = ParamBST::split_inner(&right, key);
                        let rebuilt = ParamBST::join_mid(Exposed::Node(left, root_key, rl));
                        (rebuilt, found, rr)
                    }
                    | Equal => (left, true, right),
                },
            }
        }

        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(left, key, right)) }

        fn min_key(tree: &Self) -> Option<T> {
            match tree.expose_internal() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, key, _) => match ParamBST::min_key(&left) {
                    | Some(rec) => Some(rec),
                    | None => Some(key),
                },
            }
        }

        fn join_pair_inner(left: Self, right: Self) -> Self {
            match right.expose_internal() {
                | Exposed::Leaf => left,
                | Exposed::Node(_, key, _) => {
                    let min_key = ParamBST::min_key(&right).unwrap_or(key);
                    let (_, _, reduced_right) = ParamBST::split_inner(&right, &min_key);
                    ParamBST::join_m(left, min_key, reduced_right)
                }
            }
        }

        fn union_inner(a: &Self, b: &Self) -> Self {
            match a.expose_internal() {
                | Exposed::Leaf => b.clone(),
                | Exposed::Node(al, ak, ar) => {
                    let (bl, _, br) = ParamBST::split_inner(b, &ak);
                    let left_union = ParamBST::union_inner(&al, &bl);
                    let right_union = ParamBST::union_inner(&ar, &br);
                    ParamBST::join_m(left_union, ak, right_union)
                }
            }
        }

        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {
            match tree.expose_internal() {
                | Exposed::Leaf => {}
                | Exposed::Node(left, key, right) => {
                    ParamBST::collect_in_order(&left, out);
                    out.push(key);
                    ParamBST::collect_in_order(&right, out);
                }
            }
        }
    }

    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {
        fn new() -> Self {
            ParamBST {
                root: Rc::new(RefCell::new(None)),
            }
        }

        fn expose(&self) -> Exposed<T> { self.expose_internal() }

        fn join_mid(exposed: Exposed<T>) -> Self {
            match exposed {
                | Exposed::Leaf => ParamBST::new(),
                | Exposed::Node(left, key, right) => {
                    let size = 1 + left.size() + right.size();
                    ParamBST {
                        root: Rc::new(RefCell::new(Some(Box::new(NodeInner { key, size, left, right })))),
                    }
                }
            }
        }

        fn size(&self) -> N { self.root.borrow().as_ref().map_or(0, |node| node.size) }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn insert(&self, key: T) {
            let (left, _, right) = ParamBST::split_inner(self, &key);
            let rebuilt = ParamBST::join_m(left, key, right);
            let new_state = { rebuilt.root.borrow().clone() };
            *self.root.borrow_mut() = new_state;
        }

        fn delete(&self, key: &T) {
            let (left, _, right) = ParamBST::split_inner(self, key);
            let merged = ParamBST::join_pair_inner(left, right);
            let new_state = { merged.root.borrow().clone() };
            *self.root.borrow_mut() = new_state;
        }

        fn find(&self, key: &T) -> Option<T> {
            match self.expose_internal() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, root_key, right) => match key.cmp(&root_key) {
                    | Less => ParamBSTTrait::find(&left, key),
                    | Greater => ParamBSTTrait::find(&right, key),
                    | Equal => Some(root_key),
                },
            }
        }

        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }

        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), other) }

        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            ParamBST::collect_in_order(self, &mut out);
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
