//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Parametric multi-threaded Treap (probabilistically balanced BST) with parallel operations.

//  Table of Contents
//	1. module
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//	1. module


pub mod BSTParaTreapMtEph {

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt;
    use std::fmt::Write;
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;

    verus! {

    //		4. type definitions

    /// RwLock predicate for treap nodes. Children live behind separate locks,
    /// so we can only check one-level properties: a present node has size >= 1.
    pub struct BSTParaTreapMtEphInv;

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
        pub root: Arc<RwLock<Option<Box<NodeInner<T>>>, BSTParaTreapMtEphInv>>,
    }

    // 5. view impls

    impl<T: MtKey> View for ParamTreap<T> {
        type V = Set<T::V>;

        #[verifier::external_body]
        open spec fn view(&self) -> Set<T::V> {
            Set::empty()
        }
    }


    //		9. impls

    impl<T: MtKey> RwLockPredicate<Option<Box<NodeInner<T>>>> for BSTParaTreapMtEphInv {
        open spec fn inv(self, v: Option<Box<NodeInner<T>>>) -> bool {
            match &v {
                None => true,
                Some(node) => node.size >= 1 && node.size < usize::MAX,
            }
        }
    }

    fn new_param_treap_arc<T: MtKey>(
        val: Option<Box<NodeInner<T>>>,
    ) -> (arc: Arc<RwLock<Option<Box<NodeInner<T>>>, BSTParaTreapMtEphInv>>)
        requires BSTParaTreapMtEphInv.inv(val),
        ensures arc.pred() == BSTParaTreapMtEphInv,
    {
        new_arc_rwlock(val, Ghost(BSTParaTreapMtEphInv))
    }

    //		11. derive impls in verus!

    impl<T: MtKey> Clone for ParamTreap<T> {
        fn clone(&self) -> (cloned: Self)
            ensures true
        {
            ParamTreap { root: self.root.clone() }
        }
    }

    impl<T: MtKey> Clone for NodeInner<T> {
        fn clone(&self) -> (cloned: Self)
            ensures true
        {
            NodeInner {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    impl<T: MtKey> Clone for Exposed<T> {
        fn clone(&self) -> (cloned: Self)
            ensures true
        {
            match self {
                Exposed::Leaf => Exposed::Leaf,
                Exposed::Node(l, k, r) => Exposed::Node(l.clone(), k.clone(), r.clone()),
            }
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn priority_for<T: MtKey>(key: &T) -> i64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buf = String::new();
        let _ = write!(&mut buf, "{key:?}");
        Hash::hash(&buf, &mut hasher);
        hasher.finish() as i64
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn tree_priority<T: MtKey>(tree: &ParamTreap<T>) -> i64 {
        let handle = tree.root.acquire_read();
        let result = handle.borrow().as_ref().map_or(i64::MIN, |node| node.priority);
        handle.release_read();
        result
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn tree_size<T: MtKey>(tree: &ParamTreap<T>) -> usize {
        let handle = tree.root.acquire_read();
        let result = handle.borrow().as_ref().map_or(0, |node| node.size);
        handle.release_read();
        result
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn make_node<T: MtKey>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> ParamTreap<T> {
        let size = 1 + tree_size(&left) + tree_size(&right);
        ParamTreap {
            root: new_param_treap_arc(
                Some(Box::new(NodeInner { key, priority, size, left, right })),
            ),
        }
    }

    #[verifier::external_body]
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

    #[verifier::external_body]
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

    #[verifier::external_body]
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

    #[verifier::external_body]
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

    #[verifier::external_body]
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

    #[verifier::external_body]
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

    #[verifier::external_body]
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

    #[verifier::external_body]
    fn filter_parallel<T: MtKey + 'static, F: Pred<T>>(tree: &ParamTreap<T>, predicate: F) -> ParamTreap<T>
    where
        ParamTreap<T>: ParamTreapTrait<T>,
    {
        let predicate = Arc::new(predicate);
        filter_inner(tree, &predicate)
    }

    #[verifier::external_body]
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

    #[verifier::external_body]
    fn reduce_parallel<T: MtKey + 'static, F>(tree: &ParamTreap<T>, op: F, base: T) -> T
    where
        ParamTreap<T>: ParamTreapTrait<T>,
        F: Fn(T, T) -> T + Send + Sync + 'static,
    {
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    #[verifier::external_body]
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

    pub trait ParamTreapTrait<T: MtKey + 'static>: Sized + View<V = Set<T::V>> {
        spec fn spec_bstparatreapmteph_wf(&self) -> bool;

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new() -> (tree: Self)
            ensures tree@.finite(), tree@.len() == 0, tree.spec_bstparatreapmteph_wf();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose(&self) -> (exposed: Exposed<T>)
            ensures
                self@.finite(),
                exposed is Leaf ==> self@.len() == 0,
                exposed matches Exposed::Node(left, key, right) ==> (
                    self@.contains(key@)
                    && left@.finite()
                    && right@.finite()
                    && left@.subset_of(self@)
                    && right@.subset_of(self@)
                    && self@ =~= left@.union(right@).insert(key@)
                );
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose_with_priority(&self) -> (parts: Option<(ParamTreap<T>, T, i64, ParamTreap<T>)>)
            ensures
                self@.finite(),
                parts is None ==> self@.len() == 0,
                parts matches Some((left, key, _, right)) ==> (
                    self@.contains(key@)
                    && left@.finite()
                    && right@.finite()
                    && left@.subset_of(self@)
                    && right@.subset_of(self@)
                    && self@ =~= left@.union(right@).insert(key@)
                );
        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        /// - Claude-Opus-4.6: Work O(log(|left| + |right|)), Span O(log(|left| + |right|)) — delegates to join_with_priority
        fn join_mid(exposed: Exposed<T>) -> (tree: Self)
            ensures
                tree@.finite(),
                exposed is Leaf ==> tree@ =~= Set::<T::V>::empty(),
                exposed matches Exposed::Node(l, k, r) ==> tree@ =~= l@.union(r@).insert(k@);
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures self@.finite(), count == self@.len();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (empty: bool)
            ensures self@.finite(), empty == (self@.len() == 0);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn insert(&self, key: T)
            ensures true;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn delete(&self, key: &T)
            ensures true;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T) -> (found: Option<T>)
            ensures
                found matches Some(v) ==> v@ == key@ && self@.contains(v@),
                found is None ==> !self@.contains(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
            ensures
                parts.0@.finite(), parts.2@.finite(),
                parts.1 == self@.contains(key@),
                self@.finite(),
                !parts.0@.contains(key@) && !parts.2@.contains(key@),
                self@ =~= parts.0@.union(parts.2@).union(
                    if parts.1 { Set::<T::V>::empty().insert(key@) } else { Set::<T::V>::empty() }
                );
        /// - APAS: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        /// - Claude-Opus-4.6: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite(), joined@ =~= self@.union(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@.finite(), combined@ == self@.union(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@.finite(), common@ == self@.intersect(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        fn difference(&self, other: &Self) -> (diff: Self)
            ensures diff@.finite(), diff@ == self@.difference(other@);
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                forall|t: &T| #[trigger] predicate.requires((t,)),
                forall|x: T, keep: bool|
                    predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.finite(),
                filtered@.subset_of(self@),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
            ensures true;
        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            ensures self@.finite(), ordered.spec_len() == self@.len();
    }

    impl<T: MtKey + 'static> ParamTreapTrait<T> for ParamTreap<T> {
        open spec fn spec_bstparatreapmteph_wf(&self) -> bool {
            self@.finite()
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new() -> (tree: Self)
            ensures tree.spec_bstparatreapmteph_wf()
        {
            let tree = ParamTreap {
                root: new_param_treap_arc(None),
            };
            proof { assume(tree@.finite() && tree@.len() == 0); }
            tree
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose(&self) -> (exposed: Exposed<T>) {
            match self.expose_with_priority() {
                | None => Exposed::Leaf,
                | Some((left, key, _, right)) => Exposed::Node(left, key, right),
            }
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn expose_with_priority(&self) -> (parts: Option<(ParamTreap<T>, T, i64, ParamTreap<T>)>) {
            let rwlock = arc_deref(&self.root);
            let handle = rwlock.acquire_read();
            let result = match handle.borrow() {
                None => None,
                Some(node) => {
                    let left = node.left.clone();
                    let key = node.key.clone();
                    let priority = node.priority;
                    let right = node.right.clone();
                    Some((left, key, priority, right))
                },
            };
            handle.release_read();
            proof {
                assume(
                    self@.finite()
                    && (result is None ==> self@.len() == 0)
                    && (result matches Some((left, key, _, right)) ==> (
                        self@.contains(key@)
                        && left@.finite()
                        && right@.finite()
                        && left@.subset_of(self@)
                        && right@.subset_of(self@)
                        && self@ =~= left@.union(right@).insert(key@)
                    ))
                );
            }
            result
        }

        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        /// - Claude-Opus-4.6: Work O(log(|left| + |right|)), Span O(log(|left| + |right|)) — delegates to join_with_priority
        #[verifier::external_body]
        fn join_mid(exposed: Exposed<T>) -> (tree: Self) {
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
        fn size(&self) -> (count: usize) {
            let rwlock = arc_deref(&self.root);
            let handle = rwlock.acquire_read();
            let count = match handle.borrow() {
                None => 0usize,
                Some(node) => node.size,
            };
            handle.release_read();
            proof { assume(self@.finite() && count == self@.len()); }
            count
        }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (empty: bool) { self.size() == 0 }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn insert(&self, key: T) {
            let (left, _, right) = self.split(&key);
            let rebuilt = Self::join_mid(Exposed::Node(left, key, right));
            match rebuilt.expose_with_priority() {
                None => {
                    let rwlock_self = arc_deref(&self.root);
                    let (_old, write_handle) = rwlock_self.acquire_write();
                    write_handle.release_write(None);
                },
                Some((l, k, p, r)) => {
                    let lsz = l.size();
                    let rsz = r.size();
                    let sz: usize = if rsz < usize::MAX - 1 && lsz < usize::MAX - 1 - rsz {
                        1 + lsz + rsz
                    } else {
                        usize::MAX - 1
                    };
                    let rwlock_self = arc_deref(&self.root);
                    let (_old, write_handle) = rwlock_self.acquire_write();
                    write_handle.release_write(Some(Box::new(NodeInner { key: k, priority: p, size: sz, left: l, right: r })));
                },
            }
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn delete(&self, key: &T) {
            let (left, _, right) = self.split(key);
            let merged = ParamTreapTrait::<T>::join_pair(&left, right);
            match merged.expose_with_priority() {
                None => {
                    let rwlock_self = arc_deref(&self.root);
                    let (_old, write_handle) = rwlock_self.acquire_write();
                    write_handle.release_write(None);
                },
                Some((l, k, p, r)) => {
                    let lsz = l.size();
                    let rsz = r.size();
                    let sz: usize = if rsz < usize::MAX - 1 && lsz < usize::MAX - 1 - rsz {
                        1 + lsz + rsz
                    } else {
                        usize::MAX - 1
                    };
                    let rwlock_self = arc_deref(&self.root);
                    let (_old, write_handle) = rwlock_self.acquire_write();
                    write_handle.release_write(Some(Box::new(NodeInner { key: k, priority: p, size: sz, left: l, right: r })));
                },
            }
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T) -> (found: Option<T>) {
            let mut current = self.clone();
            let fuel = self.size();
            let mut remaining = fuel;
            let mut result: Option<T> = None;
            while remaining > 0
                invariant true,
                decreases remaining,
            {
                match current.expose_with_priority() {
                    None => { break; },
                    Some((left, root_key, _, right)) => {
                        if (*key) == root_key {
                            result = Some(root_key);
                            break;
                        } else if (*key) < root_key {
                            current = left;
                        } else {
                            current = right;
                        }
                    }
                }
                remaining = remaining - 1;
            }
            proof {
                assume(
                    (result matches Some(v) ==> v@ == key@ && self@.contains(v@))
                    && (result is None ==> !self@.contains(key@))
                );
            }
            result
        }

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|)
        #[verifier::external_body]
        fn split(&self, key: &T) -> (parts: (Self, bool, Self)) { split_inner(self, key) }

        /// - APAS: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        /// - Claude-Opus-4.6: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        #[verifier::external_body]
        fn join_pair(&self, other: Self) -> (joined: Self) { join_pair_inner(self.clone(), other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        #[verifier::external_body]
        fn union(&self, other: &Self) -> (combined: Self) { union_inner(self, other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        #[verifier::external_body]
        fn intersect(&self, other: &Self) -> (common: Self) { intersect_inner(self, other) }

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n)
        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (diff: Self) { difference_inner(self, other) }

        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        #[verifier::external_body]
        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self) { filter_parallel(self, predicate) }

        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|)
        #[verifier::external_body]
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static,
        {
            reduce_parallel(self, op, base)
        }

        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|)
        #[verifier::external_body]
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>) {
            let mut out = Vec::with_capacity(self.size());
            collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    } // verus!

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

    //		13. derive impls outside verus!

    impl fmt::Debug for BSTParaTreapMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTParaTreapMtEphInv").finish()
        }
    }

    impl fmt::Display for BSTParaTreapMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTParaTreapMtEphInv")
        }
    }

    impl<T: MtKey> fmt::Debug for Exposed<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Exposed::Leaf => write!(f, "Exposed::Leaf"),
                Exposed::Node(_, _, _) => write!(f, "Exposed::Node(...)"),
            }
        }
    }

    impl<T: MtKey> fmt::Display for Exposed<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Exposed::Leaf => write!(f, "Leaf"),
                Exposed::Node(_, _, _) => write!(f, "Node(...)"),
            }
        }
    }

    impl<T: MtKey> fmt::Debug for NodeInner<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("NodeInner")
                .field("priority", &self.priority)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<T: MtKey> fmt::Display for NodeInner<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "NodeInner(priority={}, size={})", self.priority, self.size)
        }
    }

    impl<T: MtKey> fmt::Debug for ParamTreap<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreap(size: {})", self.size())
        }
    }

    impl<T: MtKey> fmt::Display for ParamTreap<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreap(size: {})", self.size())
        }
    }
}
