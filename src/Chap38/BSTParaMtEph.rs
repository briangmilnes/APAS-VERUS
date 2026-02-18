//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric multi-threaded BST built around a joinMid interface.

pub mod BSTParaMtEph {

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::sync::{Arc, RwLock};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub enum Exposed<T: MtKey> {
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    #[derive(Clone, Debug)]
    struct NodeInner<T: MtKey> {
        key: T,
        size: N,
        left: ParamBST<T>,
        right: ParamBST<T>,
    }

    #[derive(Debug, Clone)]
    pub struct ParamBST<T: MtKey> {
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,
    }

    pub trait ParamBSTTrait<T: MtKey + 'static>: Sized {
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                           -> Self;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose(&self)                   -> Exposed<T>;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn join_mid(exposed: Exposed<T>)   -> Self;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                     -> N;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                 -> B;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn insert(&self, key: T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn delete(&self, key: &T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn find(&self, key: &T)            -> Option<T>;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn split(&self, key: &T)           -> (Self, B, Self);
        /// - APAS: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|))
        /// - Claude-Opus-4.6: Work Θ(log(|t1| + |t2|)), Span Θ(log(|t1| + |t2|))
        fn join_pair(&self, other: Self)   -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
        fn union(&self, other: &Self)      -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
        fn intersect(&self, other: &Self)  -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
        fn difference(&self, other: &Self) -> Self;
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(&self, predicate: F) -> Self;
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T;
        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self)                 -> ArraySeqStPerS<T>;
    }

    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn expose_internal<T: MtKey + 'static>(tree: &ParamBST<T>) -> Exposed<T> {
        let guard = tree.root.read().unwrap();
        match &*guard {
            | None => Exposed::Leaf,
            | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
        }
    }

    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn join_mid<T: MtKey + 'static>(exposed: Exposed<T>) -> ParamBST<T> {
        match exposed {
            | Exposed::Leaf => ParamBST {
                root: Arc::new(RwLock::new(None)),
            },
            | Exposed::Node(left, key, right) => {
                let size = 1 + left.size() + right.size();
                ParamBST {
                    root: Arc::new(RwLock::new(Some(Box::new(NodeInner { key, size, left, right })))),
                }
            }
        }
    }

    /// - APAS: Work O(lg |t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    fn split_inner<T: MtKey + 'static>(tree: &ParamBST<T>, key: &T) -> (ParamBST<T>, B, ParamBST<T>) {
        match expose_internal(tree) {
            | Exposed::Leaf => (
                ParamBST {
                    root: Arc::new(RwLock::new(None)),
                },
                false,
                ParamBST {
                    root: Arc::new(RwLock::new(None)),
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
    fn join_m<T: MtKey + 'static>(left: ParamBST<T>, key: T, right: ParamBST<T>) -> ParamBST<T> {
        join_mid(Exposed::Node(left, key, right))
    }

    /// - APAS: Work O(lg |t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
    fn min_key<T: MtKey + 'static>(tree: &ParamBST<T>) -> Option<T> {
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
    fn join_pair_inner<T: MtKey + 'static>(left: ParamBST<T>, right: ParamBST<T>) -> ParamBST<T> {
        match expose_internal(&right) {
            | Exposed::Leaf => left,
            | Exposed::Node(_, key, _) => {
                let min_k = min_key(&right).unwrap_or(key);
                let (_, _, reduced_right) = split_inner(&right, &min_k);
                join_m(left, min_k, reduced_right)
            }
        }
    }

    /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
    /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
    fn union_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> ParamBST<T> {
        match expose_internal(a) {
            | Exposed::Leaf => b.clone(),
            | Exposed::Node(al, ak, ar) => {
                let (bl, _, br) = split_inner(b, &ak);
                let Pair(left_union, right_union) =
                    crate::ParaPair!(move || union_inner(&al, &bl), move || union_inner(&ar, &br));
                join_m(left_union, ak, right_union)
            }
        }
    }

    /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
    /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
    fn intersect_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> ParamBST<T> {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) | (_, Exposed::Leaf) => ParamBST {
                root: Arc::new(RwLock::new(None)),
            },
            | (Exposed::Node(al, ak, ar), _) => {
                let (bl, found, br) = split_inner(b, &ak);
                let Pair(left_res, right_res) =
                    crate::ParaPair!(move || intersect_inner(&al, &bl), move || { intersect_inner(&ar, &br) });
                if found {
                    join_m(left_res, ak, right_res)
                } else {
                    join_pair_inner(left_res, right_res)
                }
            }
        }
    }

    /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
    /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
    fn difference_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> ParamBST<T> {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) => ParamBST {
                root: Arc::new(RwLock::new(None)),
            },
            | (_, Exposed::Leaf) => a.clone(),
            | (Exposed::Node(al, ak, ar), _) => {
                let (bl, found, br) = split_inner(b, &ak);
                let Pair(left_res, right_res) = crate::ParaPair!(move || difference_inner(&al, &bl), move || {
                    difference_inner(&ar, &br)
                });
                if found {
                    join_pair_inner(left_res, right_res)
                } else {
                    join_m(left_res, ak, right_res)
                }
            }
        }
    }

    /// - APAS: Work O(|t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
    fn filter_inner<T: MtKey + 'static, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: &Arc<F>,
    ) -> ParamBST<T> {
        match expose_internal(tree) {
            | Exposed::Leaf => ParamBST {
                root: Arc::new(RwLock::new(None)),
            },
            | Exposed::Node(left, key, right) => {
                let pred_left = Arc::clone(predicate);
                let pred_right = Arc::clone(predicate);
                let Pair(left_filtered, right_filtered) =
                    crate::ParaPair!(move || filter_inner(&left, &pred_left), move || {
                        filter_inner(&right, &pred_right)
                    });
                if (**predicate)(&key) {
                    join_m(left_filtered, key, right_filtered)
                } else {
                    join_pair_inner(left_filtered, right_filtered)
                }
            }
        }
    }

    /// - APAS: Work O(|t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
    fn filter_parallel<T: MtKey + 'static, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: F,
    ) -> ParamBST<T> {
        let predicate = Arc::new(predicate);
        filter_inner(tree, &predicate)
    }

    /// - APAS: Work O(|t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
    fn reduce_inner<T: MtKey + 'static, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: &Arc<F>,
        identity: T,
    ) -> T {
        match expose_internal(tree) {
            | Exposed::Leaf => identity,
            | Exposed::Node(left, key, right) => {
                let op_left = Arc::clone(op);
                let op_right = Arc::clone(op);
                let left_base = identity.clone();
                let right_base = identity;
                let Pair(left_acc, right_acc) =
                    crate::ParaPair!(move || reduce_inner(&left, &op_left, left_base), move || {
                        reduce_inner(&right, &op_right, right_base)
                    });
                let op_ref = op.as_ref();
                let right_with_key = op_ref(key, right_acc);
                op_ref(left_acc, right_with_key)
            }
        }
    }

    /// - APAS: Work O(|t|), Span O(lg |t|)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
    fn reduce_parallel<T: MtKey + 'static, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: F,
        base: T,
    ) -> T {
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    /// - APAS: Work O(|t|), Span O(|t|)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn collect_in_order<T: MtKey + 'static>(tree: &ParamBST<T>, out: &mut Vec<T>) {
        match expose_internal(tree) {
            | Exposed::Leaf => {}
            | Exposed::Node(left, key, right) => {
                collect_in_order(&left, out);
                out.push(key);
                collect_in_order(&right, out);
            }
        }
    }

    impl<T: MtKey + 'static> ParamBSTTrait<T> for ParamBST<T> {
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new() -> Self {
            ParamBST {
                root: Arc::new(RwLock::new(None)),
            }
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose(&self) -> Exposed<T> { expose_internal(self) }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn join_mid(exposed: Exposed<T>) -> Self { join_mid(exposed) }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            guard.as_ref().map_or(0, |node| node.size)
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> B { self.size() == 0 }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn insert(&self, key: T) {
            let (left, _, right) = split_inner(self, &key);
            let rebuilt = join_m(left, key, right);
            let new_state = rebuilt.root.read().unwrap().clone();
            let mut guard = self.root.write().unwrap();
            *guard = new_state;
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn delete(&self, key: &T) {
            let (left, _, right) = split_inner(self, key);
            let merged = join_pair_inner(left, right);
            let new_state = merged.root.read().unwrap().clone();
            let mut guard = self.root.write().unwrap();
            *guard = new_state;
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
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

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn split(&self, key: &T) -> (Self, B, Self) { split_inner(self, key) }

        /// - APAS: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|))
        /// - Claude-Opus-4.6: Work Θ(log(|t1| + |t2|)), Span Θ(log(|t1| + |t2|))
        fn join_pair(&self, other: Self) -> Self { join_pair_inner(self.clone(), other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
        fn union(&self, other: &Self) -> Self { union_inner(self, other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
        fn intersect(&self, other: &Self) -> Self { intersect_inner(self, other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work Θ(m · lg(n/m)), Span Θ(lg n)
        fn difference(&self, other: &Self) -> Self { difference_inner(self, other) }

        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(&self, predicate: F) -> Self {
            filter_parallel(self, predicate)
        }

        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n)
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T {
            reduce_parallel(self, op, base)
        }

        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }
}
