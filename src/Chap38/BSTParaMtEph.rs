//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric multi-threaded BST built around a joinMid interface.
//! Coarse lock (vstd RwLock) for thread-safe access.

pub mod BSTParaMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;

    verus! {

    // 3. broadcast use

    broadcast use vstd::set::group_set_axioms;

    // 4. type definitions

    pub struct BSTParaMtEphInv;

    impl<T: MtKey> RwLockPredicate<Option<Box<NodeInner<T>>>> for BSTParaMtEphInv {
        open spec fn inv(self, v: Option<Box<NodeInner<T>>>) -> bool {
            match v {
                Option::None => true,
                Option::Some(box_node) => (*box_node).size >= 1,
            }
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub enum Exposed<T: MtKey> {
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    #[verifier::reject_recursive_types(T)]
    pub struct NodeInner<T: MtKey> {
        pub key: T,
        pub size: usize,
        pub left: ParamBST<T>,
        pub right: ParamBST<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ParamBST<T: MtKey> {
        pub root: Arc<RwLock<Option<Box<NodeInner<T>>>, BSTParaMtEphInv>>,
    }

    fn new_param_bst_arc<T: MtKey>(
        val: Option<Box<NodeInner<T>>>,
    ) -> (arc: Arc<RwLock<Option<Box<NodeInner<T>>>, BSTParaMtEphInv>>)
        requires BSTParaMtEphInv.inv(val),
        ensures arc.pred() == BSTParaMtEphInv,
    {
        new_arc_rwlock(val, Ghost(BSTParaMtEphInv))
    }

    // 5. view impls

    impl<T: MtKey> View for ParamBST<T> {
        type V = Set<<T as View>::V>;
        #[verifier::external_body]
        open spec fn view(&self) -> Set<<T as View>::V> { Set::empty() }
    }

    impl<T: MtKey> View for Exposed<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    impl<T: MtKey> View for NodeInner<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    // 7. proof fns/broadcast groups

    pub assume_specification<T: MtKey + 'static>[ split_inner ](tree: &ParamBST<T>, key: &T)
        -> (parts: (ParamBST<T>, B, ParamBST<T>))
        ensures
            parts.1 == tree@.contains(key@),
            parts.0@.finite(),
            parts.2@.finite()
    ;

    // 8. traits

    pub trait ParamBSTTrait<T: MtKey + 'static>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_bstparamteph_wf(&self) -> bool;

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparamteph_wf();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn expose(&self) -> (exposed: Exposed<T>)
            ensures self@.len() == 0 ==> exposed is Leaf;
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn join_mid(exposed: Exposed<T>) -> (joined: Self)
            ensures exposed is Leaf ==> joined@ == Set::<<T as View>::V>::empty();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn is_empty(&self) -> (empty: B)
            ensures empty == (self@.len() == 0), self@.finite();
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        /// Interior mutability via RwLock precludes `old()` specs on `&self`.
        fn insert(&self, key: T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        /// Interior mutability via RwLock precludes `old()` specs on `&self`.
        fn delete(&self, key: &T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn find(&self, key: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn split(&self, key: &T) -> (parts: (Self, B, Self))
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite();
        /// - APAS: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|))
        /// - Claude-Opus-4.6: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|)) -- agrees with APAS.
        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n) -- agrees with APAS; parallel.
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n) -- agrees with APAS; parallel.
        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n) -- agrees with APAS; parallel.
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite();
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|) -- agrees with APAS; parallel.
        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                forall|t: &T| #[trigger] predicate.requires((t,)),
                forall|x: T, keep: bool|
                    predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered@.finite(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|) -- agrees with APAS; parallel.
        /// Requires `op` to be associative with identity `base`.
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T;
        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|) -- agrees with APAS; sequential DFS traversal.
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures seq@.len() == self@.len();
    }

    // 9. impls

    impl<T: MtKey + 'static> ParamBSTTrait<T> for ParamBST<T> {
        open spec fn spec_bstparamteph_wf(&self) -> bool {
            self@.finite()
        }

        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparamteph_wf()
        {
            let empty = ParamBST { root: new_param_bst_arc(None) };
            proof { accept(empty@ == Set::<<T as View>::V>::empty() && empty.spec_bstparamteph_wf()); }
            empty
        }

        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite()
        {
            let left = Self::new();
            let right = Self::new();
            let tree = ParamBST {
                root: new_param_bst_arc(Some(Box::new(NodeInner { key, size: 1, left, right }))),
            };
            proof { accept(tree@ == Set::<<T as View>::V>::empty().insert(key@) && tree@.finite()); }
            tree
        }

        fn expose(&self) -> (exposed: Exposed<T>)
            ensures self@.len() == 0 ==> exposed is Leaf
        {
            let rwlock = arc_deref(&self.root);
            let handle = rwlock.acquire_read();
            let exposed = match handle.borrow() {
                None => Exposed::Leaf,
                Some(node) => {
                    Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone())
                },
            };
            handle.release_read();
            proof { accept(self@.len() == 0 ==> exposed is Leaf); }
            exposed
        }

        fn join_mid(exposed: Exposed<T>) -> (joined: Self)
            ensures exposed is Leaf ==> joined@ == Set::<<T as View>::V>::empty()
        {
            match exposed {
                Exposed::Leaf => {
                    let joined = Self::new();
                    joined
                },
                Exposed::Node(left, key, right) => {
                    let lsz = left.size();
                    let rsz = right.size();
                    let sz: usize = if lsz < usize::MAX && rsz < usize::MAX - lsz {
                        1 + lsz + rsz
                    } else {
                        usize::MAX
                    };
                    let joined = ParamBST {
                        root: new_param_bst_arc(Some(Box::new(NodeInner { key, size: sz, left, right }))),
                    };
                    proof { accept(exposed is Leaf ==> joined@ == Set::<<T as View>::V>::empty()); }
                    joined
                },
            }
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite()
        {
            let rwlock = arc_deref(&self.root);
            let handle = rwlock.acquire_read();
            let count = match handle.borrow() {
                None => 0usize,
                Some(node) => node.size,
            };
            handle.release_read();
            proof { accept(count == self@.len() && self@.finite()); }
            count
        }

        fn is_empty(&self) -> (empty: B)
            ensures empty == (self@.len() == 0), self@.finite()
        { self.size() == 0 }

        fn insert(&self, key: T) {
            let (left, _, right) = split_inner(self, &key);
            let lsz = left.size();
            let rsz = right.size();
            let sz: usize = if lsz < usize::MAX && rsz < usize::MAX - lsz {
                1 + lsz + rsz
            } else {
                usize::MAX
            };
            let rwlock_self = arc_deref(&self.root);
            let (_old, write_handle) = rwlock_self.acquire_write();
            write_handle.release_write(Some(Box::new(NodeInner { key, size: sz, left, right })));
        }

        fn delete(&self, key: &T) {
            let (left, _, right) = split_inner(self, key);
            let merged = ParamBSTTrait::<T>::join_pair(&left, right);
            match merged.expose() {
                Exposed::Leaf => {
                    let rwlock_self = arc_deref(&self.root);
                    let (_old, write_handle) = rwlock_self.acquire_write();
                    write_handle.release_write(None);
                },
                Exposed::Node(l, k, r) => {
                    let lsz = l.size();
                    let rsz = r.size();
                    let sz: usize = if lsz < usize::MAX && rsz < usize::MAX - lsz {
                        1 + lsz + rsz
                    } else {
                        usize::MAX
                    };
                    let rwlock_self = arc_deref(&self.root);
                    let (_old, write_handle) = rwlock_self.acquire_write();
                    write_handle.release_write(Some(Box::new(NodeInner { key: k, size: sz, left: l, right: r })));
                },
            }
        }

        fn find(&self, key: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(key@)
        {
            let mut current = self.clone();
            let fuel = self.size();
            let mut remaining = fuel;
            let mut result: Option<T> = None;
            while remaining > 0
                invariant true,
                decreases remaining,
            {
                match current.expose() {
                    Exposed::Leaf => { break; },
                    Exposed::Node(left, root_key, right) => {
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
            proof { accept(result.is_some() <==> self@.contains(key@)); }
            result
        }

        fn split(&self, key: &T) -> (parts: (Self, B, Self))
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite()
        { split_inner(self, key) }

        #[verifier::external_body]
        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite()
        { join_pair_inner(self.clone(), other) }

        #[verifier::external_body]
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite()
        { union_inner(self, other) }

        #[verifier::external_body]
        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite()
        { intersect_inner(self, other) }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite()
        { difference_inner(self, other) }

        #[verifier::external_body]
        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            filter_parallel(self, predicate)
        }

        #[verifier::external_body]
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T {
            reduce_parallel(self, op, base)
        }

        #[verifier::external_body]
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures seq@.len() == self@.len()
        {
            let mut out = Vec::with_capacity(self.size());
            collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    // 11. derive impls in verus!

    impl<T: MtKey> Clone for Exposed<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = match self {
                Exposed::Leaf => Exposed::Leaf,
                Exposed::Node(l, k, r) => Exposed::Node(l.clone(), k.clone(), r.clone()),
            };
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    impl<T: MtKey> Clone for NodeInner<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = NodeInner {
                key: self.key.clone(),
                size: self.size,
                left: self.left.clone(),
                right: self.right.clone(),
            };
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    impl<T: MtKey> Clone for ParamBST<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = ParamBST { root: clone_arc_rwlock(&self.root) };
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    } // verus!

    fn new_leaf<T: MtKey>() -> ParamBST<T> {
        ParamBST { root: new_param_bst_arc(None) }
    }

    fn expose_internal<T: MtKey + 'static>(tree: &ParamBST<T>) -> Exposed<T> {
        let handle = tree.root.acquire_read();
        let exposed = match handle.borrow() {
            | None => Exposed::Leaf,
            | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
        };
        handle.release_read();
        exposed
    }

    fn join_mid<T: MtKey + 'static>(exposed: Exposed<T>) -> ParamBST<T> {
        match exposed {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let size = 1 + left.size() + right.size();
                ParamBST {
                    root: new_param_bst_arc(Some(Box::new(NodeInner { key, size, left, right }))),
                }
            }
        }
    }

    fn split_inner<T: MtKey + 'static>(tree: &ParamBST<T>, key: &T) -> (ParamBST<T>, B, ParamBST<T>) {
        match expose_internal(tree) {
            | Exposed::Leaf => (new_leaf(), false, new_leaf()),
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

    fn join_m<T: MtKey + 'static>(left: ParamBST<T>, key: T, right: ParamBST<T>) -> ParamBST<T> {
        join_mid(Exposed::Node(left, key, right))
    }

    fn min_key<T: MtKey + 'static>(tree: &ParamBST<T>) -> Option<T> {
        match expose_internal(tree) {
            | Exposed::Leaf => None,
            | Exposed::Node(left, key, _) => match min_key(&left) {
                | Some(rec) => Some(rec),
                | None => Some(key),
            },
        }
    }

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

    fn union_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> ParamBST<T> {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) => b.clone(),
            | (_, Exposed::Leaf) => a.clone(),
            | (Exposed::Node(al, ak, ar), _) => {
                let (bl, _, br) = split_inner(b, &ak);
                let Pair(left_union, right_union) =
                    crate::ParaPair!(move || union_inner(&al, &bl), move || union_inner(&ar, &br));
                join_m(left_union, ak, right_union)
            }
        }
    }

    fn intersect_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> ParamBST<T> {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) | (_, Exposed::Leaf) => new_leaf(),
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

    fn difference_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> ParamBST<T> {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) => new_leaf(),
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

    fn filter_inner<T: MtKey + 'static, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: &Arc<F>,
    ) -> ParamBST<T> {
        match expose_internal(tree) {
            | Exposed::Leaf => new_leaf(),
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

    fn filter_parallel<T: MtKey + 'static, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: F,
    ) -> ParamBST<T> {
        let predicate = Arc::new(predicate);
        filter_inner(tree, &predicate)
    }

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

    fn reduce_parallel<T: MtKey + 'static, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: F,
        base: T,
    ) -> T {
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

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

    // 13. derive impls outside verus!

    impl<T: MtKey> std::fmt::Debug for NodeInner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NodeInner").field("key", &self.key).field("size", &self.size).finish()
        }
    }

    impl<T: MtKey> std::fmt::Debug for ParamBST<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ParamBST").finish()
        }
    }
}
