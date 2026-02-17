//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric multi-threaded Treap (probabilistically balanced BST) with parallel operations.

pub mod BSTParaTreapMtEph {

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt::Write;
    use std::hash::{Hash, Hasher};
    use std::sync::{Arc, RwLock};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub enum Exposed<T: MtKey> {
        Leaf,
        Node(ParamTreap<T>, T, ParamTreap<T>),
    }

    #[derive(Clone)]
    struct NodeInner<T: MtKey> {
        key: T,
        priority: i64,
        size: N,
        left: ParamTreap<T>,
        right: ParamTreap<T>,
    }

    #[derive(Clone)]
    pub struct ParamTreap<T: MtKey> {
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,
    }

    fn priority_for<T: MtKey>(key: &T) -> i64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buf = String::new();
        let _ = write!(&mut buf, "{key:?}");
        Hash::hash(&buf, &mut hasher);
        hasher.finish() as i64
    }

    fn tree_priority<T: MtKey>(tree: &ParamTreap<T>) -> i64 {
        let guard = tree.root.read().unwrap();
        guard.as_ref().map_or(i64::MIN, |node| node.priority)
    }

    fn tree_size<T: MtKey>(tree: &ParamTreap<T>) -> N {
        let guard = tree.root.read().unwrap();
        guard.as_ref().map_or(0, |node| node.size)
    }

    fn make_node<T: MtKey>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> ParamTreap<T> {
        let size = 1 + tree_size(&left) + tree_size(&right);
        ParamTreap {
            root: Arc::new(RwLock::new(Some(Box::new(NodeInner {
                key,
                priority,
                size,
                left,
                right,
            })))),
        }
    }

    pub trait ParamTreapTrait<T: MtKey + 'static>: Sized {
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                                   -> Self;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn expose(&self)                           -> Exposed<T>;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn join_mid(exposed: Exposed<T>)           -> Self;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                             -> N;
        /// APAS: Work O(1), Span O(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                         -> B;
        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn insert(&self, key: T);
        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn delete(&self, key: &T);
        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn find(&self, key: &T)                    -> Option<T>;
        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn split(&self, key: &T)                   -> (Self, B, Self);
        // APAS - work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        // gpt-5-codex-medium: work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        fn join_pair(&self, other: Self)           -> Self;
        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn union(&self, other: &Self)              -> Self;
        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn intersect(&self, other: &Self)          -> Self;
        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn difference(&self, other: &Self)         -> Self;
        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter<F: Pred<T>>(&self, predicate: F) -> Self;
        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce<F>(&self, op: F, base: T)        -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static;
        // APAS - work O(|t|), span O(|t|)
        // gpt-5-codex-medium: work O(|t|), span O(|t|)
        fn in_order(&self)                         -> ArraySeqStPerS<T>;
    }

    impl<T: MtKey + 'static> ParamTreap<T> {
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
        pub fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {
            let guard = self.root.read().unwrap();
            guard
                .as_ref()
                .map(|node| (node.left.clone(), node.key.clone(), node.priority, node.right.clone()))
        }

        // APAS - work O(lg (|left| + |right|)), span O(lg (|left| + |right|))
        // gpt-5-codex-medium: work O(lg (|left| + |right|)), span O(lg (|left| + |right|))
        fn join_with_priority(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> ParamTreap<T> {
            let left_priority = tree_priority(&left);
            let right_priority = tree_priority(&right);
            if priority > left_priority && priority > right_priority {
                return make_node(left, key, priority, right);
            }
            if left_priority > right_priority {
                if let Some((ll, lk, lp, lr)) = left.expose_with_priority() {
                    let merged_right = ParamTreap::join_with_priority(lr, key, priority, right);
                    return make_node(ll, lk, lp, merged_right);
                }
                make_node(left, key, priority, right)
            } else {
                if let Some((rl, rk, rp, rr)) = right.expose_with_priority() {
                    let merged_left = ParamTreap::join_with_priority(left, key, priority, rl);
                    return make_node(merged_left, rk, rp, rr);
                }
                make_node(left, key, priority, right)
            }
        }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {
            match tree.expose_with_priority() {
                | None => (ParamTreap::new(), false, ParamTreap::new()),
                | Some((left, root_key, priority, right)) => match key.cmp(&root_key) {
                    | Less => {
                        let (ll, found, lr) = ParamTreap::split_inner(&left, key);
                        let rebuilt = ParamTreap::join_with_priority(lr, root_key, priority, right);
                        (ll, found, rebuilt)
                    }
                    | Greater => {
                        let (rl, found, rr) = ParamTreap::split_inner(&right, key);
                        let rebuilt = ParamTreap::join_with_priority(left, root_key, priority, rl);
                        (rebuilt, found, rr)
                    }
                    | Equal => (left, true, right),
                },
            }
        }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn join_pair_inner(left: Self, right: Self) -> Self {
            match right.expose_with_priority() {
                | None => left,
                | Some((r_left, r_key, r_priority, r_right)) => {
                    let (split_left, _, split_right) = ParamTreap::split_inner(&left, &r_key);
                    let combined_left = ParamTreap::join_pair_inner(split_left, r_left);
                    let combined_right = ParamTreap::join_pair_inner(split_right, r_right);
                    ParamTreap::join_with_priority(combined_left, r_key, r_priority, combined_right)
                }
            }
        }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn union_inner(a: &Self, b: &Self) -> Self {
            match a.expose_with_priority() {
                | None => b.clone(),
                | Some((al, ak, ap, ar)) => {
                    let (bl, _, br) = ParamTreap::split_inner(b, &ak);
                    let Pair(left_union, right_union) =
                        crate::ParaPair!(move || ParamTreap::union_inner(&al, &bl), move || {
                            ParamTreap::union_inner(&ar, &br)
                        });
                    ParamTreap::join_with_priority(left_union, ak, ap, right_union)
                }
            }
        }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn intersect_inner(a: &Self, b: &Self) -> Self {
            match a.expose_with_priority() {
                | None => ParamTreap::new(),
                | Some((al, ak, ap, ar)) => {
                    let (bl, found, br) = ParamTreap::split_inner(b, &ak);
                    let Pair(left_res, right_res) =
                        crate::ParaPair!(move || ParamTreap::intersect_inner(&al, &bl), move || {
                            ParamTreap::intersect_inner(&ar, &br)
                        });
                    if found {
                        ParamTreap::join_with_priority(left_res, ak, ap, right_res)
                    } else {
                        ParamTreap::join_pair_inner(left_res, right_res)
                    }
                }
            }
        }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn difference_inner(a: &Self, b: &Self) -> Self {
            match a.expose_with_priority() {
                | None => ParamTreap::new(),
                | Some((al, ak, ap, ar)) => {
                    let (bl, found, br) = ParamTreap::split_inner(b, &ak);
                    let Pair(left_res, right_res) =
                        crate::ParaPair!(move || ParamTreap::difference_inner(&al, &bl), move || {
                            ParamTreap::difference_inner(&ar, &br)
                        });
                    if found {
                        ParamTreap::join_pair_inner(left_res, right_res)
                    } else {
                        ParamTreap::join_with_priority(left_res, ak, ap, right_res)
                    }
                }
            }
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter_inner<F: Pred<T>>(tree: &Self, predicate: &Arc<F>) -> Self {
            match tree.expose_with_priority() {
                | None => ParamTreap::new(),
                | Some((left, key, priority, right)) => {
                    let pred_left = Arc::clone(predicate);
                    let pred_right = Arc::clone(predicate);
                    let Pair(left_filtered, right_filtered) =
                        crate::ParaPair!(move || ParamTreap::filter_inner(&left, &pred_left), move || {
                            ParamTreap::filter_inner(&right, &pred_right)
                        });
                    if (**predicate)(&key) {
                        ParamTreap::join_with_priority(left_filtered, key, priority, right_filtered)
                    } else {
                        ParamTreap::join_pair_inner(left_filtered, right_filtered)
                    }
                }
            }
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter_parallel<F: Pred<T>>(tree: &Self, predicate: F) -> Self {
            let predicate = Arc::new(predicate);
            ParamTreap::filter_inner(tree, &predicate)
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce_inner<F>(tree: &Self, op: &Arc<F>, identity: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            match tree.expose_with_priority() {
                | None => identity,
                | Some((left, key, _priority, right)) => {
                    let op_left = Arc::clone(op);
                    let op_right = Arc::clone(op);
                    let left_base = identity.clone();
                    let right_base = identity;
                    let Pair(left_acc, right_acc) = crate::ParaPair!(
                        move || ParamTreap::reduce_inner(&left, &op_left, left_base),
                        move || ParamTreap::reduce_inner(&right, &op_right, right_base)
                    );
                    let op_ref = op.as_ref();
                    let right_with_key = op_ref(key, right_acc);
                    op_ref(left_acc, right_with_key)
                }
            }
        }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce_parallel<F>(tree: &Self, op: F, base: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            let op = Arc::new(op);
            ParamTreap::reduce_inner(tree, &op, base)
        }

        // APAS - work O(|t|), span O(|t|)
        // gpt-5-codex-medium: work O(|t|), span O(|t|)
        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {
            match tree.expose_internal() {
                | Exposed::Leaf => {}
                | Exposed::Node(left, key, right) => {
                    ParamTreap::collect_in_order(&left, out);
                    out.push(key);
                    ParamTreap::collect_in_order(&right, out);
                }
            }
        }
    }

    impl<T: MtKey + 'static> ParamTreapTrait<T> for ParamTreap<T> {
        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn new() -> Self {
            ParamTreap {
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
                | Exposed::Leaf => ParamTreap::new(),
                | Exposed::Node(left, key, right) => {
                    let priority = priority_for(&key);
                    ParamTreap::join_with_priority(left, key, priority, right)
                }
            }
        }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn size(&self) -> N { tree_size(self) }

        // APAS - work O(1), span O(1)
        // gpt-5-codex-medium: work O(1), span O(1)
        fn is_empty(&self) -> B { self.size() == 0 }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn insert(&self, key: T) {
            let (left, _, right) = ParamTreap::split_inner(self, &key);
            let priority = priority_for(&key);
            let rebuilt = ParamTreap::join_with_priority(left, key, priority, right);
            let new_state = rebuilt.root.read().unwrap().clone();
            let mut guard = self.root.write().unwrap();
            *guard = new_state;
        }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn delete(&self, key: &T) {
            let (left, _, right) = ParamTreap::split_inner(self, key);
            let merged = ParamTreap::join_pair_inner(left, right);
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
                    | Less => ParamTreapTrait::find(&left, key),
                    | Greater => ParamTreapTrait::find(&right, key),
                    | Equal => Some(root_key),
                },
            }
        }

        // APAS - work O(lg |t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(lg |t|), span O(lg |t|)
        fn split(&self, key: &T) -> (Self, B, Self) { ParamTreap::split_inner(self, key) }

        // APAS - work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        // gpt-5-codex-medium: work O(lg (|t_1| + |t_2|)), span O(lg (|t_1| + |t_2|))
        fn join_pair(&self, other: Self) -> Self { ParamTreap::join_pair_inner(self.clone(), other) }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn union(&self, other: &Self) -> Self { ParamTreap::union_inner(self, other) }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn intersect(&self, other: &Self) -> Self { ParamTreap::intersect_inner(self, other) }

        // APAS - work O(m · lg (n / m)), span O(lg n)
        // gpt-5-codex-medium: work O(m · lg (n / m)), span O(lg n)
        fn difference(&self, other: &Self) -> Self { ParamTreap::difference_inner(self, other) }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn filter<F: Pred<T>>(&self, predicate: F) -> Self { ParamTreap::filter_parallel(self, predicate) }

        // APAS - work O(|t|), span O(lg |t|)
        // gpt-5-codex-medium: work O(|t|), span O(lg |t|)
        fn reduce<F>(&self, op: F, base: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            ParamTreap::reduce_parallel(self, op, base)
        }

        // APAS - work O(|t|), span O(|t|)
        // gpt-5-codex-medium: work O(|t|), span O(|t|)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            ParamTreap::collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    #[macro_export]
    macro_rules! ParamTreapLit {
        () => {
            < $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreap<_> as $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreapTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreap<_> as $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreapTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
