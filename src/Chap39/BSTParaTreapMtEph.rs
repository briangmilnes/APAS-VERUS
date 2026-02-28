//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Parametric multi-threaded Treap (probabilistically balanced BST) with parallel operations.

//  Table of Contents
//	1. module
//	4. type definitions
//	6. spec fns
//	8. traits
//	9. impls
//	12. macros
//	13. derive impls outside verus!

//	1. module


pub mod BSTParaTreapMtEph {

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt::Write;
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    //		4. type definitions

    /// RwLock predicate for treap nodes. Children live behind separate locks,
    /// so we can only check one-level properties: a present node has size >= 1.
    pub struct TreapWf;

    #[verifier::reject_recursive_types(T)]
    pub enum Exposed<T: MtKey> {
        Leaf,
        Node(ParamTreap<T>, T, ParamTreap<T>),
    }

    #[verifier::reject_recursive_types(T)]
    pub struct NodeInner<T: MtKey> {
        pub key: T,
        pub priority: i64,
        pub size: usize,
        pub left: ParamTreap<T>,
        pub right: ParamTreap<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ParamTreap<T: MtKey> {
        pub root: Arc<RwLock<Option<Box<NodeInner<T>>>, TreapWf>>,
    }


    //		6. spec fns

    pub open spec fn spec_node_wf<T: MtKey>(v: &Option<Box<NodeInner<T>>>) -> bool {
        match v {
            None => true,
            Some(node) => node.size >= 1 && node.size < usize::MAX,
        }
    }


    //		9. impls

    impl<T: MtKey> RwLockPredicate<Option<Box<NodeInner<T>>>> for TreapWf {
        open spec fn inv(self, v: Option<Box<NodeInner<T>>>) -> bool {
            spec_node_wf(&v)
        }
    }

    #[verifier::external_body]
    fn new_treap_lock<T: MtKey>(val: Option<Box<NodeInner<T>>>) -> (lock: RwLock<Option<Box<NodeInner<T>>>, TreapWf>)
        requires spec_node_wf(&val),
    {
        RwLock::new(val, Ghost(TreapWf))
    }

    } // verus!

    //		13. derive impls outside verus!

    impl<T: MtKey> Clone for Exposed<T> {
        fn clone(&self) -> Self {
            match self {
                Exposed::Leaf => Exposed::Leaf,
                Exposed::Node(l, k, r) => Exposed::Node(l.clone(), k.clone(), r.clone()),
            }
        }
    }

    impl<T: MtKey> Clone for NodeInner<T> {
        fn clone(&self) -> Self {
            NodeInner {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    impl<T: MtKey> Clone for ParamTreap<T> {
        fn clone(&self) -> Self {
            ParamTreap { root: self.root.clone() }
        }
    }

    //		9. impls

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn priority_for<T: MtKey>(key: &T) -> i64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buf = String::new();
        let _ = write!(&mut buf, "{key:?}");
        Hash::hash(&buf, &mut hasher);
        hasher.finish() as i64
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn tree_priority<T: MtKey>(tree: &ParamTreap<T>) -> i64 {
        let handle = tree.root.acquire_read();
        let result = handle.borrow().as_ref().map_or(i64::MIN, |node| node.priority);
        handle.release_read();
        result
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn tree_size<T: MtKey>(tree: &ParamTreap<T>) -> usize {
        let handle = tree.root.acquire_read();
        let result = handle.borrow().as_ref().map_or(0, |node| node.size);
        handle.release_read();
        result
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn make_node<T: MtKey>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> ParamTreap<T> {
        let size = 1 + tree_size(&left) + tree_size(&right);
        ParamTreap {
            root: Arc::new(new_treap_lock(
                Some(Box::new(NodeInner { key, priority, size, left, right })),
            )),
        }
    }

    fn join_with_priority<T: MtKey + 'static>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        let left_priority = tree_priority(&left);
        let right_priority = tree_priority(&right);
        if priority > left_priority && priority > right_priority {
            return make_node(left, key, priority, right);
        }
        if left_priority > right_priority {
            if let Some((ll, lk, lp, lr)) = left.expose_with_priority() {
                let merged_right = join_with_priority(lr, key, priority, right);
                return make_node(ll, lk, lp, merged_right);
            }
            make_node(left, key, priority, right)
        } else {
            if let Some((rl, rk, rp, rr)) = right.expose_with_priority() {
                let merged_left = join_with_priority(left, key, priority, rl);
                return make_node(merged_left, rk, rp, rr);
            }
            make_node(left, key, priority, right)
        }
    }

    fn split_inner<T: MtKey + 'static>(tree: &ParamTreap<T>, key: &T) -> (ParamTreap<T>, bool, ParamTreap<T>)
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        match tree.expose_with_priority() {
            | None => (ParamTreap::new(), false, ParamTreap::new()),
            | Some((left, root_key, priority, right)) => match key.cmp(&root_key) {
                | Less => {
                    let (ll, found, lr) = split_inner(&left, key);
                    let rebuilt = join_with_priority(lr, root_key, priority, right);
                    (ll, found, rebuilt)
                }
                | Greater => {
                    let (rl, found, rr) = split_inner(&right, key);
                    let rebuilt = join_with_priority(left, root_key, priority, rl);
                    (rebuilt, found, rr)
                }
                | Equal => (left, true, right),
            },
        }
    }

    fn join_pair_inner<T: MtKey + 'static>(left: ParamTreap<T>, right: ParamTreap<T>) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        match right.expose_with_priority() {
            | None => left,
            | Some((r_left, r_key, r_priority, r_right)) => {
                let (split_left, _, split_right) = split_inner(&left, &r_key);
                let combined_left = join_pair_inner(split_left, r_left);
                let combined_right = join_pair_inner(split_right, r_right);
                join_with_priority(combined_left, r_key, r_priority, combined_right)
            }
        }
    }

    fn union_inner<T: MtKey + 'static>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        match a.expose_with_priority() {
            | None => b.clone(),
            | Some((al, ak, ap, ar)) => {
                let (bl, _, br) = split_inner(b, &ak);
                let Pair(left_union, right_union) =
                    crate::ParaPair!(move || union_inner(&al, &bl), move || union_inner(&ar, &br));
                join_with_priority(left_union, ak, ap, right_union)
            }
        }
    }

    fn intersect_inner<T: MtKey + 'static>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        match a.expose_with_priority() {
            | None => ParamTreap::new(),
            | Some((al, ak, ap, ar)) => {
                let (bl, found, br) = split_inner(b, &ak);
                let Pair(left_res, right_res) =
                    crate::ParaPair!(move || intersect_inner(&al, &bl), move || intersect_inner(&ar, &br));
                if found {
                    join_with_priority(left_res, ak, ap, right_res)
                } else {
                    join_pair_inner(left_res, right_res)
                }
            }
        }
    }

    fn difference_inner<T: MtKey + 'static>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        match a.expose_with_priority() {
            | None => ParamTreap::new(),
            | Some((al, ak, ap, ar)) => {
                let (bl, found, br) = split_inner(b, &ak);
                let Pair(left_res, right_res) =
                    crate::ParaPair!(move || difference_inner(&al, &bl), move || difference_inner(&ar, &br));
                if found {
                    join_pair_inner(left_res, right_res)
                } else {
                    join_with_priority(left_res, ak, ap, right_res)
                }
            }
        }
    }

    fn filter_inner<T: MtKey + 'static, F: Pred<T>>(tree: &ParamTreap<T>, predicate: &Arc<F>) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        match tree.expose_with_priority() {
            | None => ParamTreap::new(),
            | Some((left, key, priority, right)) => {
                let pred_left = Arc::clone(predicate);
                let pred_right = Arc::clone(predicate);
                let Pair(left_filtered, right_filtered) =
                    crate::ParaPair!(move || filter_inner(&left, &pred_left), move || filter_inner(&right, &pred_right));
                if (**predicate)(&key) {
                    join_with_priority(left_filtered, key, priority, right_filtered)
                } else {
                    join_pair_inner(left_filtered, right_filtered)
                }
            }
        }
    }

    fn filter_parallel<T: MtKey + 'static, F: Pred<T>>(tree: &ParamTreap<T>, predicate: F) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        let predicate = Arc::new(predicate);
        filter_inner(tree, &predicate)
    }

    fn reduce_inner<T: MtKey + 'static, F>(tree: &ParamTreap<T>, op: &Arc<F>, identity: T) -> T
    where
        ParamTreap<T>: ParamTreapTrait<T>,
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
                    move || reduce_inner(&left, &op_left, left_base),
                    move || reduce_inner(&right, &op_right, right_base)
                );
                let op_ref = op.as_ref();
                let right_with_key = op_ref(key, right_acc);
                op_ref(left_acc, right_with_key)
            }
        }
    }

    fn reduce_parallel<T: MtKey + 'static, F>(tree: &ParamTreap<T>, op: F, base: T) -> T
    where
        ParamTreap<T>: ParamTreapTrait<T>,
        F: Fn(T, T) -> T + Send + Sync + 'static,
    {
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    fn collect_in_order<T: MtKey + 'static>(tree: &ParamTreap<T>, out: &mut Vec<T>)
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        match tree.expose_with_priority() {
            | None => {}
            | Some((left, key, _priority, right)) => {
                collect_in_order(&left, out);
                out.push(key);
                collect_in_order(&right, out);
            }
        }
    }

    //		8. traits

    pub trait ParamTreapTrait<T: MtKey + 'static>: Sized {
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                                   -> Self;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose(&self)                           -> Exposed<T>;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose_with_priority(&self)             -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)>;
        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        /// - Claude-Opus-4.6: Work O(log(|left| + |right|)), Span O(log(|left| + |right|)) — delegates to join_with_priority
        fn join_mid(exposed: Exposed<T>)           -> Self;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                             -> usize;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)                         -> bool;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn insert(&self, key: T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn delete(&self, key: &T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T)                    -> Option<T>;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn split(&self, key: &T)                   -> (Self, bool, Self);
        /// - APAS: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        /// - Claude-Opus-4.6: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        fn join_pair(&self, other: Self)           -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn union(&self, other: &Self)              -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn intersect(&self, other: &Self)          -> Self;
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn difference(&self, other: &Self)         -> Self;
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        fn filter<F: Pred<T>>(&self, predicate: F) -> Self;
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        fn reduce<F>(&self, op: F, base: T)        -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static;
        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|)
        fn in_order(&self)                         -> ArraySeqStPerS<T>;
    }

    impl<T: MtKey + 'static> ParamTreapTrait<T> for ParamTreap<T> {
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new() -> Self {
            ParamTreap {
                root: Arc::new(new_treap_lock(None)),
            }
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose(&self) -> Exposed<T> {
            match self.expose_with_priority() {
                | None => Exposed::Leaf,
                | Some((left, key, _, right)) => Exposed::Node(left, key, right),
            }
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose_with_priority(&self) -> Option<(ParamTreap<T>, T, i64, ParamTreap<T>)> {
            let handle = self.root.acquire_read();
            let result = handle.borrow()
                .as_ref()
                .map(|node| (node.left.clone(), node.key.clone(), node.priority, node.right.clone()));
            handle.release_read();
            result
        }

        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        /// - Claude-Opus-4.6: Work O(log(|left| + |right|)), Span O(log(|left| + |right|)) — delegates to join_with_priority
        fn join_mid(exposed: Exposed<T>) -> Self {
            match exposed {
                | Exposed::Leaf => ParamTreap::new(),
                | Exposed::Node(left, key, right) => {
                    let priority = priority_for(&key);
                    join_with_priority(left, key, priority, right)
                }
            }
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self) -> usize { tree_size(self) }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> bool { self.size() == 0 }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn insert(&self, key: T) {
            let (left, _, right) = split_inner(self, &key);
            let priority = priority_for(&key);
            let rebuilt = join_with_priority(left, key, priority, right);
            let read_handle = rebuilt.root.acquire_read();
            let new_state = read_handle.borrow().clone();
            read_handle.release_read();
            let (_old_val, write_handle) = self.root.acquire_write();
            write_handle.release_write(new_state);
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn delete(&self, key: &T) {
            let (left, _, right) = split_inner(self, key);
            let merged = join_pair_inner(left, right);
            let read_handle = merged.root.acquire_read();
            let new_state = read_handle.borrow().clone();
            read_handle.release_read();
            let (_old_val, write_handle) = self.root.acquire_write();
            write_handle.release_write(new_state);
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T) -> Option<T> {
            match self.expose_with_priority() {
                | None => None,
                | Some((left, root_key, _, right)) => match key.cmp(&root_key) {
                    | Less => ParamTreapTrait::find(&left, key),
                    | Greater => ParamTreapTrait::find(&right, key),
                    | Equal => Some(root_key),
                },
            }
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn split(&self, key: &T) -> (Self, bool, Self) { split_inner(self, key) }

        /// - APAS: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        /// - Claude-Opus-4.6: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        fn join_pair(&self, other: Self) -> Self { join_pair_inner(self.clone(), other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn union(&self, other: &Self) -> Self { union_inner(self, other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn intersect(&self, other: &Self) -> Self { intersect_inner(self, other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn difference(&self, other: &Self) -> Self { difference_inner(self, other) }

        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        fn filter<F: Pred<T>>(&self, predicate: F) -> Self { filter_parallel(self, predicate) }

        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        fn reduce<F>(&self, op: F, base: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            reduce_parallel(self, op, base)
        }

        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    //		12. macros

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
