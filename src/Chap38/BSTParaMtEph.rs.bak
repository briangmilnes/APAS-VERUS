//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric multi-threaded BST built around a joinMid interface.

pub mod BSTParaMtEph {

    use std::cmp::Ordering::Equal;
    use std::cmp::Ordering::Greater;
    use std::cmp::Ordering::Less;
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
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                           -> Self;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn expose(&self)                   -> Exposed<T>;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn join_mid(exposed: Exposed<T>)   -> Self;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                     -> N;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                 -> B;
        /// APAS: Work O(lg |t|), Span O(lg |t|)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn insert(&self, key: T);
        /// APAS: Work O(lg |t|), Span O(lg |t|)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn delete(&self, key: &T);
        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn find(&self, key: &T)            -> Option<T>;
        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn split(&self, key: &T)           -> (Self, B, Self);
        // APAS - work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        // gpt-5-codex-medium: work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        fn join_pair(&self, other: Self)   -> Self;
        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn union(&self, other: &Self)      -> Self;
        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn intersect(&self, other: &Self)  -> Self;
        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn difference(&self, other: &Self) -> Self;
        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter<F: Fn(&T)                -> bool + Send + Sync + 'static>(&self, predicate: F) -> Self;
        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce<F: Fn(T, T)              -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T;
        // APAS - work O(|t|), span O(|t|)
        // gpt-5-codex-medium: work O(|t|), span O(|t|)
        fn in_order(&self)                 -> ArraySeqStPerS<T>;
    }

    impl<T: MtKey + 'static> ParamBST<T> {
        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn expose_internal(&self) -> Exposed<T> {
            let guard = self.root.read().unwrap();
            match &*guard {
                | None => Exposed::Leaf,
                | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
            }
        }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn join_mid(exposed: Exposed<T>) -> Self {
            match exposed {
                | Exposed::Leaf => ParamBST::new(),
                | Exposed::Node(left, key, right) => {
                    let size = 1 + left.size() + right.size();
                    ParamBST {
                        root: Arc::new(RwLock::new(Some(Box::new(NodeInner { key, size, left, right })))),
                    }
                }
            }
        }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
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

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(left, key, right)) }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn min_key(tree: &Self) -> Option<T> {
            match tree.expose_internal() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, key, _) => match ParamBST::min_key(&left) {
                    | Some(rec) => Some(rec),
                    | None => Some(key),
                },
            }
        }

        // APAS - work O(lg (|left| + |right|)), span O(lg (|left| + |right|))
        // gpt-5-codex-medium: work O(lg (|left| + |right|)), span O(lg (|left| + |right|))
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

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn union_inner(a: &Self, b: &Self) -> Self {
            match a.expose_internal() {
                | Exposed::Leaf => b.clone(),
                | Exposed::Node(al, ak, ar) => {
                    let (bl, _, br) = ParamBST::split_inner(b, &ak);
                    let Pair(left_union, right_union) = crate::ParaPair!(
                        move || ParamBST::union_inner(&al, &bl),
                        move || ParamBST::union_inner(&ar, &br)
                    );
                    ParamBST::join_m(left_union, ak, right_union)
                }
            }
        }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn intersect_inner(a: &Self, b: &Self) -> Self {
            match (a.expose_internal(), b.expose_internal()) {
                | (Exposed::Leaf, _) | (_, Exposed::Leaf) => ParamBST::new(),
                | (Exposed::Node(al, ak, ar), _) => {
                    let (bl, found, br) = ParamBST::split_inner(b, &ak);
                    let Pair(left_res, right_res) =
                        crate::ParaPair!(move || ParamBST::intersect_inner(&al, &bl), move || {
                            ParamBST::intersect_inner(&ar, &br)
                        });
                    if found {
                        ParamBST::join_m(left_res, ak, right_res)
                    } else {
                        ParamBST::join_pair_inner(left_res, right_res)
                    }
                }
            }
        }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn difference_inner(a: &Self, b: &Self) -> Self {
            match (a.expose_internal(), b.expose_internal()) {
                | (Exposed::Leaf, _) => ParamBST::new(),
                | (_, Exposed::Leaf) => a.clone(),
                | (Exposed::Node(al, ak, ar), _) => {
                    let (bl, found, br) = ParamBST::split_inner(b, &ak);
                    let Pair(left_res, right_res) =
                        crate::ParaPair!(move || ParamBST::difference_inner(&al, &bl), move || {
                            ParamBST::difference_inner(&ar, &br)
                        });
                    if found {
                        ParamBST::join_pair_inner(left_res, right_res)
                    } else {
                        ParamBST::join_m(left_res, ak, right_res)
                    }
                }
            }
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter_inner<F: Fn(&T) -> bool + Send + Sync + 'static>(tree: &Self, predicate: &Arc<F>) -> Self {
            match tree.expose_internal() {
                | Exposed::Leaf => ParamBST::new(),
                | Exposed::Node(left, key, right) => {
                    let pred_left = Arc::clone(predicate);
                    let pred_right = Arc::clone(predicate);
                    let Pair(left_filtered, right_filtered) =
                        crate::ParaPair!(move || ParamBST::filter_inner(&left, &pred_left), move || {
                            ParamBST::filter_inner(&right, &pred_right)
                        });
                    if (**predicate)(&key) {
                        ParamBST::join_m(left_filtered, key, right_filtered)
                    } else {
                        ParamBST::join_pair_inner(left_filtered, right_filtered)
                    }
                }
            }
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter_parallel<F: Fn(&T) -> bool + Send + Sync + 'static>(tree: &Self, predicate: F) -> Self {
            let predicate = Arc::new(predicate);
            ParamBST::filter_inner(tree, &predicate)
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce_inner<F: Fn(T, T) -> T + Send + Sync + 'static>(tree: &Self, op: &Arc<F>, identity: T) -> T {
            match tree.expose_internal() {
                | Exposed::Leaf => identity,
                | Exposed::Node(left, key, right) => {
                    let op_left = Arc::clone(op);
                    let op_right = Arc::clone(op);
                    let left_base = identity.clone();
                    let right_base = identity;
                    let Pair(left_acc, right_acc) =
                        crate::ParaPair!(move || ParamBST::reduce_inner(&left, &op_left, left_base), move || {
                            ParamBST::reduce_inner(&right, &op_right, right_base)
                        });
                    let op_ref = op.as_ref();
                    let right_with_key = op_ref(key, right_acc);
                    op_ref(left_acc, right_with_key)
                }
            }
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce_parallel<F: Fn(T, T) -> T + Send + Sync + 'static>(tree: &Self, op: F, base: T) -> T {
            let op = Arc::new(op);
            ParamBST::reduce_inner(tree, &op, base)
        }

        // APAS - work O(|t|), span O(|t|)
        // gpt-5-codex-medium: work O(|t|), span O(|t|)
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

    impl<T: MtKey + 'static> ParamBSTTrait<T> for ParamBST<T> {
        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn new() -> Self {
            ParamBST {
                root: Arc::new(RwLock::new(None)),
            }
        }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn expose(&self) -> Exposed<T> { self.expose_internal() }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn join_mid(exposed: Exposed<T>) -> Self {
            match exposed {
                | Exposed::Leaf => ParamBST::new(),
                | Exposed::Node(left, key, right) => {
                    let size = 1 + left.size() + right.size();
                    ParamBST {
                        root: Arc::new(RwLock::new(Some(Box::new(NodeInner { key, size, left, right })))),
                    }
                }
            }
        }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            guard.as_ref().map_or(0, |node| node.size)
        }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn is_empty(&self) -> B { self.size() == 0 }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn insert(&self, key: T) {
            let (left, _, right) = ParamBST::split_inner(self, &key);
            let rebuilt = ParamBST::join_m(left, key, right);
            let new_state = rebuilt.root.read().unwrap().clone();
            let mut guard = self.root.write().unwrap();
            *guard = new_state;
        }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn delete(&self, key: &T) {
            let (left, _, right) = ParamBST::split_inner(self, key);
            let merged = ParamBST::join_pair_inner(left, right);
            let new_state = merged.root.read().unwrap().clone();
            let mut guard = self.root.write().unwrap();
            *guard = new_state;
        }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
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

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }

        // APAS - work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        // gpt-5-codex-medium: work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), other) }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn intersect(&self, other: &Self) -> Self { ParamBST::intersect_inner(self, other) }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn difference(&self, other: &Self) -> Self { ParamBST::difference_inner(self, other) }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(&self, predicate: F) -> Self {
            ParamBST::filter_parallel(self, predicate)
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T {
            ParamBST::reduce_parallel(self, op, base)
        }

        // APAS - work O(|t|), span O(|t|)
        // gpt-5-codex-medium: work O(|t|), span O(|t|)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            ParamBST::collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }
}
