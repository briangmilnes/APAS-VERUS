//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric single-threaded BST built around a joinMid interface.
//! Coarse lock (vstd RwLock) for thread-safe access.

pub mod BSTParaStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 10. free fns
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

    // 3. broadcast use

    broadcast use vstd::set::group_set_axioms;

    // 4. type definitions

    pub struct BstParaWf;

    impl<T: StT + Ord> RwLockPredicate<Option<Box<NodeInner<T>>>> for BstParaWf {
        closed spec fn inv(self, v: Option<Box<NodeInner<T>>>) -> bool {
            match v {
                Option::None => true,
                Option::Some(box_node) => (*box_node).size >= 1,
            }
        }
    }

    #[verifier::reject_recursive_types(T)]
    #[derive(Debug, Default)]
    pub enum Exposed<T: StT + Ord> {
        #[default]
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    #[verifier::reject_recursive_types(T)]
    #[derive(Debug)]
    struct NodeInner<T: StT + Ord> {
        key: T,
        size: usize,
        left: ParamBST<T>,
        right: ParamBST<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ParamBST<T: StT + Ord> {
        root: Arc<RwLock<Option<Box<NodeInner<T>>>, BstParaWf>>,
    }

    #[verifier::external_body]
    fn new_bst_para_lock<T: StT + Ord>(val: Option<Box<NodeInner<T>>>) -> (lock: RwLock<Option<Box<NodeInner<T>>>, BstParaWf>) {
        RwLock::new(val, Ghost(BstParaWf))
    }

    // 5. view impls

    impl<T: StT + Ord> ParamBST<T> {
        #[verifier::external_body]
        pub closed spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            Set::empty()
        }
    }

    impl<T: StT + Ord> View for ParamBST<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }

    impl<T: StT + Ord> View for Exposed<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    impl<T: StT + Ord> View for NodeInner<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    // 8. traits

    pub trait ParamBSTTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        /// - APAS: Work O(1), Span O(1)
        fn new() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty();
        /// - APAS: Work O(1), Span O(1)
        fn singleton(key: T) -> (result: Self)
            ensures
                result@ == Set::<<T as View>::V>::empty().insert(key@),
                result@.finite();
        /// - APAS: Work O(1), Span O(1)
        fn expose(&self) -> (exposed: Exposed<T>)
            ensures self@.len() == 0 ==> exposed is Leaf;
        /// - APAS: Work O(1), Span O(1)
        fn join_mid(exposed: Exposed<T>) -> (result: Self)
            ensures exposed is Leaf ==> result@ == Set::<<T as View>::V>::empty();
        /// - APAS: Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - APAS: Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: B)
            ensures empty == (self@.len() == 0), self@.finite();
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// Interior mutability via RwLock precludes `old()` specs on `&self`.
        fn insert(&self, key: T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// Interior mutability via RwLock precludes `old()` specs on `&self`.
        fn delete(&self, key: &T);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn split(&self, key: &T) -> (parts: (Self, B, Self))
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite();
        /// - APAS: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|))
        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential
        fn intersect(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite();
        /// - APAS: Work O(|t|), Span O(|t|) — sequential
        fn filter<F: Fn(&T) -> bool>(&self, predicate: F) -> (filtered: Self)
            ensures filtered@.subset_of(self@), filtered@.finite();
        /// - APAS: Work O(|t|), Span O(|t|) — sequential
        /// Requires `op` to be associative with identity `base`.
        fn reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> T;
        /// - APAS: Work O(|t|), Span O(|t|)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures seq@.len() == self@.len();
    }

    // 9. impls

    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {
        #[verifier::external_body]
        fn new() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty()
        { new_leaf() }

        #[verifier::external_body]
        fn singleton(key: T) -> (result: Self)
            ensures
                result@ == Set::<<T as View>::V>::empty().insert(key@),
                result@.finite()
        {
            join_mid(Exposed::Node(new_leaf(), key, new_leaf()))
        }

        #[verifier::external_body]
        fn expose(&self) -> (exposed: Exposed<T>)
            ensures self@.len() == 0 ==> exposed is Leaf
        { expose_internal(self) }

        #[verifier::external_body]
        fn join_mid(exposed: Exposed<T>) -> (result: Self)
            ensures exposed is Leaf ==> result@ == Set::<<T as View>::V>::empty()
        { join_mid(exposed) }

        #[verifier::external_body]
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite()
        {
            let handle = self.root.acquire_read();
            let n = handle.borrow().as_ref().map_or(0, |node| node.size);
            handle.release_read();
            n
        }

        #[verifier::external_body]
        fn is_empty(&self) -> (empty: B)
            ensures empty == (self@.len() == 0), self@.finite()
        { self.size() == 0 }

        #[verifier::external_body]
        fn insert(&self, key: T) {
            let (left, _, right) = split_inner(self, &key);
            let rebuilt = join_m(left, key, right);
            let read_h = rebuilt.root.acquire_read();
            let new_val = read_h.borrow().clone();
            read_h.release_read();
            let (_, write_h) = self.root.acquire_write();
            write_h.release_write(new_val);
        }

        #[verifier::external_body]
        fn delete(&self, key: &T) {
            let (left, _, right) = split_inner(self, key);
            let merged = join_pair_inner(left, right);
            let read_h = merged.root.acquire_read();
            let new_val = read_h.borrow().clone();
            read_h.release_read();
            let (_, write_h) = self.root.acquire_write();
            write_h.release_write(new_val);
        }

        #[verifier::external_body]
        fn find(&self, key: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(key@)
        {
            match expose_internal(self) {
                | Exposed::Leaf => None,
                | Exposed::Node(left, root_key, right) => match key.cmp(&root_key) {
                    | Less => ParamBSTTrait::find(&left, key),
                    | Greater => ParamBSTTrait::find(&right, key),
                    | Equal => Some(root_key),
                },
            }
        }

        #[verifier::external_body]
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
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite()
        { union_inner(self, other) }

        #[verifier::external_body]
        fn intersect(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite()
        { intersect_inner(self, other) }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite()
        { difference_inner(self, other) }

        #[verifier::external_body]
        fn filter<F: Fn(&T) -> bool>(&self, predicate: F) -> (filtered: Self)
            ensures filtered@.subset_of(self@), filtered@.finite()
        {
            filter_inner(self, &predicate)
        }

        #[verifier::external_body]
        fn reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> T {
            reduce_inner(self, &op, base)
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

    // 10. free fns

    #[verifier::external_body]
    fn new_leaf<T: StT + Ord>() -> (tree: ParamBST<T>) {
        ParamBST { root: Arc::new(new_bst_para_lock(None)) }
    }

    #[verifier::external_body]
    fn expose_internal<T: StT + Ord>(tree: &ParamBST<T>) -> (exposed: Exposed<T>) {
        let handle = tree.root.acquire_read();
        let result = match handle.borrow() {
            | None => Exposed::Leaf,
            | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
        };
        handle.release_read();
        result
    }

    #[verifier::external_body]
    fn join_mid<T: StT + Ord>(exposed: Exposed<T>) -> (tree: ParamBST<T>) {
        match exposed {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let size = 1 + left.size() + right.size();
                ParamBST {
                    root: Arc::new(new_bst_para_lock(Some(Box::new(NodeInner { key, size, left, right })))),
                }
            }
        }
    }

    #[verifier::external_body]
    fn split_inner<T: StT + Ord>(tree: &ParamBST<T>, key: &T) -> (parts: (ParamBST<T>, B, ParamBST<T>)) {
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

    #[verifier::external_body]
    fn join_m<T: StT + Ord>(left: ParamBST<T>, key: T, right: ParamBST<T>) -> (tree: ParamBST<T>) {
        join_mid(Exposed::Node(left, key, right))
    }

    #[verifier::external_body]
    fn min_key<T: StT + Ord>(tree: &ParamBST<T>) -> (result: Option<T>) {
        match expose_internal(tree) {
            | Exposed::Leaf => None,
            | Exposed::Node(left, key, _) => match min_key(&left) {
                | Some(rec) => Some(rec),
                | None => Some(key),
            },
        }
    }

    #[verifier::external_body]
    fn join_pair_inner<T: StT + Ord>(left: ParamBST<T>, right: ParamBST<T>) -> (tree: ParamBST<T>) {
        match expose_internal(&right) {
            | Exposed::Leaf => left,
            | Exposed::Node(_, key, _) => {
                let min_k = min_key(&right).unwrap_or(key);
                let (_, _, reduced_right) = split_inner(&right, &min_k);
                join_m(left, min_k, reduced_right)
            }
        }
    }

    /// Algorithm 38.6 — sequential union via divide-and-conquer on split.
    #[verifier::external_body]
    fn union_inner<T: StT + Ord>(a: &ParamBST<T>, b: &ParamBST<T>) -> (tree: ParamBST<T>) {
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

    /// Algorithm 38.7 — sequential intersect. Keeps keys present in both trees.
    #[verifier::external_body]
    fn intersect_inner<T: StT + Ord>(a: &ParamBST<T>, b: &ParamBST<T>) -> (tree: ParamBST<T>) {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) | (_, Exposed::Leaf) => new_leaf(),
            | (Exposed::Node(al, ak, ar), _) => {
                let (bl, found, br) = split_inner(b, &ak);
                let left_res = intersect_inner(&al, &bl);
                let right_res = intersect_inner(&ar, &br);
                if found {
                    join_m(left_res, ak, right_res)
                } else {
                    join_pair_inner(left_res, right_res)
                }
            }
        }
    }

    /// Algorithm 38.8 — sequential difference. Keeps keys in `a` not in `b`.
    #[verifier::external_body]
    fn difference_inner<T: StT + Ord>(a: &ParamBST<T>, b: &ParamBST<T>) -> (tree: ParamBST<T>) {
        match (expose_internal(a), expose_internal(b)) {
            | (Exposed::Leaf, _) => new_leaf(),
            | (_, Exposed::Leaf) => a.clone(),
            | (Exposed::Node(al, ak, ar), _) => {
                let (bl, found, br) = split_inner(b, &ak);
                let left_res = difference_inner(&al, &bl);
                let right_res = difference_inner(&ar, &br);
                if found {
                    join_pair_inner(left_res, right_res)
                } else {
                    join_m(left_res, ak, right_res)
                }
            }
        }
    }

    /// Algorithm 38.9 — sequential filter. Keeps keys satisfying `predicate`.
    #[verifier::external_body]
    fn filter_inner<T: StT + Ord, F: Fn(&T) -> bool>(
        tree: &ParamBST<T>,
        predicate: &F,
    ) -> (filtered: ParamBST<T>) {
        match expose_internal(tree) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let left_filtered = filter_inner(&left, predicate);
                let right_filtered = filter_inner(&right, predicate);
                if predicate(&key) {
                    join_m(left_filtered, key, right_filtered)
                } else {
                    join_pair_inner(left_filtered, right_filtered)
                }
            }
        }
    }

    /// Algorithm 38.10 — sequential reduce. Folds `op(L', op(k, R'))`.
    #[verifier::external_body]
    fn reduce_inner<T: StT + Ord, F: Fn(T, T) -> T>(
        tree: &ParamBST<T>,
        op: &F,
        identity: T,
    ) -> T {
        match expose_internal(tree) {
            | Exposed::Leaf => identity,
            | Exposed::Node(left, key, right) => {
                let left_acc = reduce_inner(&left, op, identity.clone());
                let right_acc = reduce_inner(&right, op, identity);
                let right_with_key = op(key, right_acc);
                op(left_acc, right_with_key)
            }
        }
    }

    #[verifier::external_body]
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

    // 11. derive impls in verus!

    impl<T: StT + Ord + Clone> Clone for Exposed<T> {
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

    impl<T: StT + Ord + Clone> Clone for NodeInner<T> {
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

    impl<T: StT + Ord> Clone for ParamBST<T> {
        #[verifier::external_body]
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            ParamBST { root: Arc::clone(&self.root) }
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for ParamBST<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ParamBST").finish()
        }
    }
}
