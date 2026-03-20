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
    // 6. spec fns
    // 7. proof fns/broadcast groups
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::{OrdSpec, PartialEqSpec, PartialOrdSpec};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        vstd::set_lib::group_set_properties,
    };

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
        pub(crate) root: Arc<RwLock<Option<Box<NodeInner<T>>>, BSTParaMtEphInv>>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    fn new_param_bst<T: MtKey>(
        val: Option<Box<NodeInner<T>>>,
        Ghost(contents): Ghost<Set<<T as View>::V>>,
    ) -> (tree: ParamBST<T>)
        requires BSTParaMtEphInv.inv(val),
        ensures tree@ =~= contents,
    {
        ParamBST {
            root: new_arc_rwlock(val, Ghost(BSTParaMtEphInv)),
            ghost_locked_root: Ghost(contents),
        }
    }

    // 5. view impls

    impl<T: MtKey> ParamBST<T> {
        pub closed spec fn spec_ghost_locked_root(self) -> Set<<T as View>::V> {
            self.ghost_locked_root@
        }
    }

    impl<T: MtKey> View for ParamBST<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_ghost_locked_root() }
    }

    impl<T: MtKey> View for Exposed<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    impl<T: MtKey> View for NodeInner<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    // 6. spec fns

    /// View-consistent ordering: elements with the same view compare Equal.
    pub open spec fn view_ord_consistent<T: MtKey>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    // 7. proof fns/broadcast groups

    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    proof fn lemma_cmp_antisymmetry<T: MtKey>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Greater,
        ensures
            b.cmp_spec(&a) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Less(a,b) and Less(b,c) implies Less(a,c).
    proof fn lemma_cmp_transitivity<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Less,
        ensures
            a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Equal-substitution: Less(a,b) and Equal(b,c) implies Less(a,c).
    proof fn lemma_cmp_eq_subst<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Equal,
        ensures
            a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Left congruence: Equal(a,b) implies a and b compare the same way to c.
    proof fn lemma_cmp_equal_congruent<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures
            a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Right congruence: Equal(b,c) implies any a compares the same way to b and c.
    proof fn lemma_cmp_equal_congruent_right<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            b.cmp_spec(&c) == Equal,
        ensures
            a.cmp_spec(&b) == a.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Ordering axioms for generic MtKey: obeys_cmp_spec and view_ord_consistent.
    /// Consolidates the two standard ordering assumes into one proof function.
    /// Callers get both properties from a single call.
    proof fn lemma_cmp_order_axioms<T: MtKey>()
        ensures
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
    {
        assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
        assume(view_ord_consistent::<T>());
    }

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
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T
            requires forall|a: T, b: T| #[trigger] op.requires((a, b));
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
            new_param_bst(None, Ghost(Set::empty()))
        }

        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite()
        {
            let left = Self::new();
            let right = Self::new();
            let ghost kv = key@;
            new_param_bst(
                Some(Box::new(NodeInner { key, size: 1, left, right })),
                Ghost(Set::<<T as View>::V>::empty().insert(kv)),
            )
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
            proof { assume(self@.len() == 0 ==> exposed is Leaf); }
            exposed
        }

        fn join_mid(exposed: Exposed<T>) -> (joined: Self)
            ensures exposed is Leaf ==> joined@ == Set::<<T as View>::V>::empty()
        {
            match exposed {
                Exposed::Leaf => {
                    Self::new()
                },
                Exposed::Node(left, key, right) => {
                    let lsz = left.size();
                    let rsz = right.size();
                    let sz: usize = if lsz < usize::MAX && rsz < usize::MAX - lsz {
                        1 + lsz + rsz
                    } else {
                        usize::MAX
                    };
                    let ghost kv = key@;
                    let ghost contents = left@.union(right@).insert(kv);
                    new_param_bst(
                        Some(Box::new(NodeInner { key, size: sz, left, right })),
                        Ghost(contents),
                    )
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
            proof { assume(count == self@.len() && self@.finite()); }
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
            find_recursive(self, key)
        }

        fn split(&self, key: &T) -> (parts: (Self, B, Self))
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite()
        { split_inner(self, key) }

        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite()
        { join_pair_inner(self.clone(), other) }

        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite()
        { union_inner(self, other) }

        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite()
        { intersect_inner(self, other) }

        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite()
        { difference_inner(self, other) }

        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            filter_parallel(self, predicate, Ghost(spec_pred))
        }

        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> T {
            reduce_parallel(self, op, base)
        }

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
            proof { assume(cloned@ == self@); }
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
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    impl<T: MtKey> Clone for ParamBST<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            ParamBST {
                root: clone_arc_rwlock(&self.root),
                ghost_locked_root: Ghost(self.ghost_locked_root@),
            }
        }
    }

    // 12. outside-verus algorithmic helpers (external_body, pending full proofs)

    fn new_leaf<T: MtKey>() -> (tree: ParamBST<T>)
        ensures tree@ =~= Set::<<T as View>::V>::empty()
    {
        new_param_bst(None, Ghost(Set::empty()))
    }

    #[verifier::external_body]
    fn expose_internal<T: MtKey + 'static>(tree: &ParamBST<T>) -> (exposed: Exposed<T>)
        ensures
            tree@.finite(),
            exposed is Leaf ==> tree@.len() == 0,
            exposed matches Exposed::Node(left, key, right) ==> (
                tree@.contains(key@)
                && left@.finite()
                && right@.finite()
                && left@.subset_of(tree@)
                && right@.subset_of(tree@)
                && tree@ =~= left@.union(right@).insert(key@)
                && !left@.contains(key@)
                && !right@.contains(key@)
                && left@.disjoint(right@)
                && (forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less)
                && (forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater)
            ),
    {
        let handle = tree.root.acquire_read();
        let exposed = match handle.borrow() {
            | None => Exposed::Leaf,
            | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
        };
        handle.release_read();
        exposed
    }

    fn join_mid<T: MtKey + 'static>(exposed: Exposed<T>) -> (result: ParamBST<T>)
        ensures
            exposed is Leaf ==> result@ =~= Set::<<T as View>::V>::empty(),
            exposed matches Exposed::Node(l, k, r) ==> result@ =~= l@.union(r@).insert(k@),
    {
        match exposed {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let lsz = left.size();
                let rsz = right.size();
                let size: usize = if lsz < usize::MAX && rsz < usize::MAX - lsz {
                    1 + lsz + rsz
                } else {
                    usize::MAX
                };
                let ghost kv = key@;
                let ghost contents = left@.union(right@).insert(kv);
                new_param_bst(
                    Some(Box::new(NodeInner { key, size, left, right })),
                    Ghost(contents),
                )
            }
        }
    }

    fn split_inner<T: MtKey + 'static>(tree: &ParamBST<T>, key: &T) -> (parts: (ParamBST<T>, B, ParamBST<T>))
        ensures
            parts.0@.finite(),
            parts.2@.finite(),
            parts.1 == tree@.contains(key@),
            tree@.finite(),
            !parts.0@.contains(key@) && !parts.2@.contains(key@),
            tree@ =~= parts.0@.union(parts.2@).union(
                if parts.1 { Set::<<T as View>::V>::empty().insert(key@) } else { Set::<<T as View>::V>::empty() }
            ),
            forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
        decreases tree@.len(),
    {
        proof {
            lemma_cmp_order_axioms::<T>();
            reveal(vstd::laws_cmp::obeys_cmp_ord);
        }
        match expose_internal(tree) {
            | Exposed::Leaf => {
                (new_leaf(), false, new_leaf())
            },
            | Exposed::Node(left, root_key, right) => {
                let ghost lv = left@;
                let ghost rv = right@;
                let ghost rkv = root_key@;
                let ghost kv = key@;
                let ghost rk = root_key;
                let ghost kref = *key;
                proof {
                    lv.lemma_subset_not_in_lt(tree@, rkv);
                    rv.lemma_subset_not_in_lt(tree@, rkv);
                }
                match key.cmp(&root_key) {
                    | Less => {
                        let (ll, found, lr) = split_inner(&left, key);
                        let rebuilt = join_mid(Exposed::Node(lr, root_key, right));
                        let ghost llv = ll@;
                        let ghost lrv = lr@;
                        proof {
                            assert(rebuilt@ =~= lrv.union(rv).insert(rkv));
                            assert(!rv.contains(kv));
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(&key) == Greater by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if lrv.contains(t@) {
                                    // From recursive split ensures.
                                } else if rv.contains(t@) {
                                    // t > rk (expose). Antisymmetry: rk < t.
                                    // kref < rk (cmp). Transitivity: kref < t.
                                    // Reveals handle: kref < t → t > kref.
                                    lemma_cmp_antisymmetry(t, rk);
                                    lemma_cmp_transitivity(kref, rk, t);
                                } else {
                                    // t@ == rkv. Congruence: t compares like rk.
                                    // rk > kref (antisymmetry of kref < rk).
                                    // So t > kref.
                                    assert(t@ == rkv);
                                    assert(t.cmp_spec(&rk) == Equal);
                                    lemma_cmp_equal_congruent(t, rk, kref);
                                }
                            };
                        }
                        (ll, found, rebuilt)
                    }
                    | Greater => {
                        let (rl, found, rr) = split_inner(&right, key);
                        let rebuilt = join_mid(Exposed::Node(left, root_key, rl));
                        let ghost rlv = rl@;
                        let ghost rrv = rr@;
                        proof {
                            assert(rebuilt@ =~= lv.union(rlv).insert(rkv));
                            assert(!lv.contains(kv));
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(&key) == Less by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if rlv.contains(t@) {
                                    // From recursive split ensures.
                                } else if lv.contains(t@) {
                                    // kref > rk (cmp). Antisymmetry: rk < kref.
                                    // t < rk (expose). Transitivity: t < kref.
                                    lemma_cmp_antisymmetry(kref, rk);
                                    lemma_cmp_transitivity(t, rk, kref);
                                } else {
                                    // t@ == rkv. Congruence: t compares like rk.
                                    // rk < kref (antisymmetry of kref > rk).
                                    // So t < kref.
                                    assert(t@ == rkv);
                                    assert(t.cmp_spec(&rk) == Equal);
                                    lemma_cmp_antisymmetry(kref, rk);
                                    lemma_cmp_equal_congruent(t, rk, kref);
                                }
                            };
                        }
                        (rebuilt, found, rr)
                    }
                    | Equal => {
                        proof {
                            assert forall|t: T| (#[trigger] lv.contains(t@)) implies
                                t.cmp_spec(&key) == Less by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                lemma_cmp_equal_congruent_right(t, kref, rk);
                            };
                            assert forall|t: T| (#[trigger] rv.contains(t@)) implies
                                t.cmp_spec(&key) == Greater by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                lemma_cmp_equal_congruent_right(t, kref, rk);
                            };
                        }
                        (left, true, right)
                    }
                }
            }
        }
    }

    fn join_m<T: MtKey + 'static>(left: ParamBST<T>, key: T, right: ParamBST<T>) -> (result: ParamBST<T>)
        ensures result@ =~= left@.union(right@).insert(key@)
    {
        join_mid(Exposed::Node(left, key, right))
    }

    fn find_recursive<T: MtKey + 'static>(tree: &ParamBST<T>, key: &T) -> (found: Option<T>)
        ensures found.is_some() <==> tree@.contains(key@),
        decreases tree@.len(),
    {
        proof {
            lemma_cmp_order_axioms::<T>();
            reveal(vstd::laws_cmp::obeys_cmp_ord);
        }
        match expose_internal(tree) {
            | Exposed::Leaf => None,
            | Exposed::Node(left, root_key, right) => {
                proof {
                    left@.lemma_subset_not_in_lt(tree@, root_key@);
                    right@.lemma_subset_not_in_lt(tree@, root_key@);
                }
                match key.cmp(&root_key) {
                    | Equal => Some(root_key),
                    | Less => find_recursive(&left, key),
                    | Greater => find_recursive(&right, key),
                }
            }
        }
    }

    fn min_key<T: MtKey + 'static>(tree: &ParamBST<T>) -> (result: Option<T>)
        requires tree@.finite(),
        ensures
            result.is_none() <==> tree@.len() == 0,
            result.is_some() ==> tree@.contains(result.unwrap()@),
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => None,
            | Exposed::Node(left, key, _) => {
                proof { left@.lemma_subset_not_in_lt(tree@, key@); }
                match min_key(&left) {
                    | Some(rec) => Some(rec),
                    | None => Some(key),
                }
            },
        }
    }

    fn join_pair_inner<T: MtKey + 'static>(left: ParamBST<T>, right: ParamBST<T>) -> (joined: ParamBST<T>)
        ensures joined@.finite()
    {
        let _ = left.size();
        match expose_internal(&right) {
            | Exposed::Leaf => left,
            | Exposed::Node(_, key, _) => {
                let min_k = min_key(&right).unwrap_or(key);
                let (_, _, reduced_right) = split_inner(&right, &min_k);
                join_m(left, min_k, reduced_right)
            }
        }
    }

    fn union_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> (combined: ParamBST<T>)
        ensures combined@ == a@.union(b@), combined@.finite(),
        decreases a@.len(),
    {
        let _ = b.size();
        match expose_internal(a) {
            | Exposed::Leaf => b.clone(),
            | Exposed::Node(al, ak, ar) => {
                if b.is_empty() {
                    a.clone()
                } else {
                    let (bl, _found, br) = split_inner(b, &ak);
                    proof { al@.lemma_subset_not_in_lt(a@, ak@); }
                    let f1 = move || -> (result: ParamBST<T>)
                        ensures result@ == al@.union(bl@), result@.finite()
                    {
                        union_inner(&al, &bl)
                    };
                    let f2 = move || -> (result: ParamBST<T>)
                        ensures result@ == ar@.union(br@), result@.finite()
                    {
                        union_inner(&ar, &br)
                    };
                    let Pair(left_union, right_union) = crate::ParaPair!(f1, f2);
                    join_m(left_union, ak, right_union)
                }
            }
        }
    }

    #[verifier::external_body]
    fn intersect_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> (common: ParamBST<T>)
        ensures common@ == a@.intersect(b@), common@.finite()
    {
        let _ = b.size();
        match expose_internal(a) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(al, ak, ar) => {
                if b.is_empty() {
                    new_leaf()
                } else {
                    let (bl, found, br) = split_inner(b, &ak);
                    let f1 = move || -> (result: ParamBST<T>)
                        ensures result@ == al@.intersect(bl@), result@.finite()
                    {
                        intersect_inner(&al, &bl)
                    };
                    let f2 = move || -> (result: ParamBST<T>)
                        ensures result@ == ar@.intersect(br@), result@.finite()
                    {
                        intersect_inner(&ar, &br)
                    };
                    let Pair(left_res, right_res) = crate::ParaPair!(f1, f2);
                    if found {
                        join_m(left_res, ak, right_res)
                    } else {
                        join_pair_inner(left_res, right_res)
                    }
                }
            }
        }
    }

    #[verifier::external_body]
    fn difference_inner<T: MtKey + 'static>(a: &ParamBST<T>, b: &ParamBST<T>) -> (remaining: ParamBST<T>)
        ensures remaining@ == a@.difference(b@), remaining@.finite()
    {
        let _ = b.size();
        match expose_internal(a) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(al, ak, ar) => {
                if b.is_empty() {
                    a.clone()
                } else {
                    let (bl, found, br) = split_inner(b, &ak);
                    let f1 = move || -> (result: ParamBST<T>)
                        ensures result@ == al@.difference(bl@), result@.finite()
                    {
                        difference_inner(&al, &bl)
                    };
                    let f2 = move || -> (result: ParamBST<T>)
                        ensures result@ == ar@.difference(br@), result@.finite()
                    {
                        difference_inner(&ar, &br)
                    };
                    let Pair(left_res, right_res) = crate::ParaPair!(f1, f2);
                    if found {
                        join_pair_inner(left_res, right_res)
                    } else {
                        join_m(left_res, ak, right_res)
                    }
                }
            }
        }
    }

    #[verifier::external_body]
    fn filter_inner<T: MtKey + 'static, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: &Arc<F>,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamBST<T>)
        requires
            forall|t: &T| #[trigger] predicate.requires((t,)),
            forall|x: T, keep: bool|
                predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
        ensures
            filtered@.subset_of(tree@),
            filtered@.finite(),
            forall|v: T::V| #[trigger] filtered@.contains(v)
                ==> tree@.contains(v) && spec_pred(v),
            forall|v: T::V| tree@.contains(v) && spec_pred(v)
                ==> #[trigger] filtered@.contains(v),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let pred_left = Arc::clone(predicate);
                let pred_right = Arc::clone(predicate);
                let Pair(left_filtered, right_filtered) =
                    crate::ParaPair!(
                        move || filter_inner(&left, &pred_left, Ghost::assume_new()),
                        move || filter_inner(&right, &pred_right, Ghost::assume_new())
                    );
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
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamBST<T>)
        requires
            forall|t: &T| #[trigger] predicate.requires((t,)),
            forall|x: T, keep: bool|
                predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
        ensures
            filtered@.subset_of(tree@),
            filtered@.finite(),
            forall|v: T::V| #[trigger] filtered@.contains(v)
                ==> tree@.contains(v) && spec_pred(v),
            forall|v: T::V| tree@.contains(v) && spec_pred(v)
                ==> #[trigger] filtered@.contains(v),
    {
        let predicate = Arc::new(predicate);
        filter_inner(tree, &predicate, Ghost(spec_pred))
    }

    fn reduce_inner<T: MtKey + 'static, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: &Arc<F>,
        identity: T,
    ) -> T
        requires
            tree@.finite(),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => identity,
            | Exposed::Node(left, key, right) => {
                let op_left = Arc::clone(op);
                let op_right = Arc::clone(op);
                let left_base = identity.clone();
                let right_base = identity;
                proof {
                    left@.lemma_subset_not_in_lt(tree@, key@);
                    right@.lemma_subset_not_in_lt(tree@, key@);
                }
                let f1 = move || -> T
                {
                    reduce_inner(&left, &op_left, left_base)
                };
                let f2 = move || -> T
                {
                    reduce_inner(&right, &op_right, right_base)
                };
                let Pair(left_acc, right_acc) = crate::ParaPair!(f1, f2);
                let op_ref = arc_deref(op);
                let right_with_key = op_ref(key, right_acc);
                op_ref(left_acc, right_with_key)
            }
        }
    }

    fn reduce_parallel<T: MtKey + 'static, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: F,
        base: T,
    ) -> T
        requires forall|a: T, b: T| #[trigger] op.requires((a, b)),
    {
        let _ = tree.size();
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    fn collect_in_order<T: MtKey + 'static>(tree: &ParamBST<T>, out: &mut Vec<T>)
        requires tree@.finite(),
        ensures out@.len() == old(out)@.len() + tree@.len(),
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => {}
            | Exposed::Node(left, key, right) => {
                proof {
                    left@.lemma_subset_not_in_lt(tree@, key@);
                    right@.lemma_subset_not_in_lt(tree@, key@);
                    assert(!left@.union(right@).contains(key@));
                    assert(tree@.len() == left@.len() + right@.len() + 1);
                }
                collect_in_order(&left, out);
                out.push(key);
                collect_in_order(&right, out);
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    // Ghost<Set<T::V>> contains FnSpec (PhantomData at runtime), which lacks Send/Sync.
    // ParamBST is safe to send/share: the Ghost field is erased at runtime.
    unsafe impl<T: MtKey> Send for ParamBST<T> {}
    unsafe impl<T: MtKey> Sync for ParamBST<T> {}

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
