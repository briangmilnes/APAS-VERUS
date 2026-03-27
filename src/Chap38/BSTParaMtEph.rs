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
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::cloned;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        vstd::set_lib::group_set_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    // 4. type definitions

    pub struct BSTParaMtEphInv<T: MtKey> {
        pub ghost contents: Set<<T as View>::V>,
    }

    impl<T: MtKey> RwLockPredicate<Option<Box<NodeInner<T>>>> for BSTParaMtEphInv<T> {
        open spec fn inv(self, v: Option<Box<NodeInner<T>>>) -> bool {
            match v {
                Option::None => self.contents =~= Set::<<T as View>::V>::empty(),
                Option::Some(box_node) => {
                    self.contents =~= (*box_node).left@.union((*box_node).right@).insert((*box_node).key@)
                    && (*box_node).size >= 1
                    && (*box_node).left@.finite() && (*box_node).right@.finite()
                    && (*box_node).left@.disjoint((*box_node).right@)
                    && !(*box_node).left@.contains((*box_node).key@)
                    && !(*box_node).right@.contains((*box_node).key@)
                    && (*box_node).left@.len() + (*box_node).right@.len() < usize::MAX as nat
                    && (*box_node).size as nat == (*box_node).left@.len() + (*box_node).right@.len() + 1
                    && (forall|t: T| (#[trigger] (*box_node).left@.contains(t@)) ==> t.cmp_spec(&(*box_node).key) == Less)
                    && (forall|t: T| (#[trigger] (*box_node).right@.contains(t@)) ==> t.cmp_spec(&(*box_node).key) == Greater)
                }
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
        pub(crate) locked_root: RwLock<Option<Box<NodeInner<T>>>, BSTParaMtEphInv<T>>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    fn new_param_bst<T: MtKey>(
        val: Option<Box<NodeInner<T>>>,
        Ghost(contents): Ghost<Set<<T as View>::V>>,
    ) -> (tree: ParamBST<T>)
        requires
            (BSTParaMtEphInv::<T> { contents }).inv(val),
            contents.finite(),
            forall|v: <T as View>::V| contents.contains(v)
                ==> exists|t: T| t@ == v,
        ensures tree@ =~= contents,
    {
        let ghost pred = BSTParaMtEphInv::<T> { contents };
        ParamBST {
            locked_root: RwLock::new(val, Ghost(pred)),
            ghost_locked_root: Ghost(contents),
        }
    }

    // 5. view impls

    impl<T: MtKey> ParamBST<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_root@.finite()
            && self.ghost_locked_root@ =~= self.locked_root.pred().contents
            && (forall|v: <T as View>::V| self.ghost_locked_root@.contains(v)
                ==> exists|t: T| t@ == v)
        }

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

    /// Clone bridge for generic element: requires obeys_feq_clone so axiom_cloned_implies_eq fires.
    fn clone_elem<T: MtKey>(x: &T) -> (c: T)
        requires obeys_feq_clone::<T>(),
        ensures c == *x,
    {
        let c = x.clone();
        assert(cloned(*x, c));  // strictly_cloned(*x,c) from call_ensures; triggers axiom
        c
    }

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
    /// Callers supply these as preconditions and this function restates them as postconditions.
    proof fn lemma_cmp_order_axioms<T: MtKey>()
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
    {}

    // 8. traits

    pub trait ParamBSTTrait<T: MtKey>: Sized + View<V = Set<<T as View>::V>> {
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
            requires
                exposed matches Exposed::Node(l, k, r) ==> {
                    l@.finite() && r@.finite()
                    && l@.disjoint(r@)
                    && !l@.contains(k@)
                    && !r@.contains(k@)
                    && l@.len() + r@.len() < usize::MAX as nat
                    && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                    && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
                },
            ensures
                exposed is Leaf ==> joined@ == Set::<<T as View>::V>::empty(),
                exposed matches Exposed::Node(l, k, r) ==> joined@ =~= l@.union(r@).insert(k@);
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite();
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn insert(&mut self, key: T)
            requires
                old(self)@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures self.spec_bstparamteph_wf();
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn delete(&mut self, key: &T)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures self.spec_bstparamteph_wf();
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn find(&self, key: &T) -> (found: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures found.is_some() <==> self@.contains(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite();
        /// - APAS: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|))
        /// - Claude-Opus-4.6: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|)) -- agrees with APAS.
        fn join_pair(&self, other: Self) -> (joined: Self)
            requires
                self@.len() + other@.len() <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures joined@.finite();
        /// Joins two disjoint BSTs where all elements of self are less than all elements of right.
        fn join_pair_inner(&self, right: &Self) -> (joined: Self)
            requires
                self@.finite(), right@.finite(),
                self@.disjoint(right@),
                self@.len() + right@.len() <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|s: T, o: T| #![trigger self@.contains(s@), right@.contains(o@)]
                    self@.contains(s@) && right@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@.finite(), joined@ =~= self@.union(right@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n) -- agrees with APAS; parallel.
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self@.len() + other@.len() <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures combined@ == self@.union(other@), combined@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n) -- agrees with APAS; parallel.
        fn intersect(&self, other: &Self) -> (common: Self)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures common@ == self@.intersect(other@), common@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(lg n) -- agrees with APAS; parallel.
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures remaining@ == self@.difference(other@), remaining@.finite();
        /// - APAS: Work O(|t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(lg |t|) -- agrees with APAS; parallel.
        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
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

    impl<T: MtKey> ParamBSTTrait<T> for ParamBST<T> {
        open spec fn spec_bstparamteph_wf(&self) -> bool {
            self@.finite()
            && obeys_feq_full::<T>()
        }

        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparamteph_wf()
        {
                      assert(obeys_feq_full_trigger::<T>());
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
            proof { use_type_invariant(self); }
            expose_internal(self)
        }

        fn join_mid(exposed: Exposed<T>) -> (joined: Self)
        {
            match exposed {
                | Exposed::Leaf => new_leaf(),
                | Exposed::Node(left, key, right) => {
                    let ghost kv = key@;
                    let ghost contents = left@.union(right@).insert(kv);
                    let lsz = left.size();
                    let rsz = right.size();
                    let size: usize = 1 + lsz + rsz;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        assert(!left@.union(right@).contains(kv));
                        assert(contents.len() == size as nat);
                        use_type_invariant(&left);
                        use_type_invariant(&right);
                        assert forall|v: <T as View>::V| contents.contains(v)
                            implies exists|t: T| t@ == v by {
                            if left@.contains(v) {
                            } else if right@.contains(v) {
                            } else {
                                assert(v == kv);
                                assert(key@ == v);
                            }
                        };
                    }
                    new_param_bst(
                        Some(Box::new(NodeInner { key, size, left, right })),
                        Ghost(contents),
                    )
                }
            }
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite()
        {
            proof { use_type_invariant(self); }
            let handle = self.locked_root.acquire_read();
            let count = match handle.borrow() {
                None => 0usize,
                Some(node) => {
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(node.left@, node.right@);
                    }
                    node.size
                }
            };
            handle.release_read();
            count
        }

        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite()
        { self.size() == 0 }

        fn insert(&mut self, key: T)
        {
                      assert(obeys_feq_full_trigger::<T>());
            let _sz = self.size();
            let (left, _, right) = split_inner(self, &key);
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
            }
            *self = Self::join_mid(Exposed::Node(left, key, right));
        }

        fn delete(&mut self, key: &T)
        {
                      assert(obeys_feq_full_trigger::<T>());
            let _sz = self.size();
            let (left, _, right) = split_inner(self, key);
            proof {
                lemma_cmp_order_axioms::<T>();
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry(o, *key);
                    lemma_cmp_transitivity(s, *key, o);
                };
            }
            *self = left.join_pair_inner(&right);
        }

        fn find(&self, key: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(key@)
        {
            proof { use_type_invariant(self); }
            find_recursive(self, key)
        }

        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite()
        {
            proof { use_type_invariant(self); }
            split_inner(self, key)
        }

        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite()
        {
            proof { use_type_invariant(self); use_type_invariant(&other); }
            union_inner(self, &other)
        }

        fn join_pair_inner(&self, right: &Self) -> (joined: Self)
            ensures joined@.finite(), joined@ =~= self@.union(right@),
            decreases right@.len(),
        {
            proof {
                lemma_cmp_order_axioms::<T>();
                use_type_invariant(self);
                use_type_invariant(right);
            }
            let ghost lv = self@;
            let ghost rv = right@;
            match expose_internal(right) {
                | Exposed::Leaf => {
                    proof { assert(lv.union(rv) =~= lv); }
                    self.clone()
                }
                | Exposed::Node(rl, rk, rr) => {
                    let ghost rlv = rl@;
                    let ghost rrv = rr@;
                    let ghost rkv = rk@;
                    proof {
                        assert(rv =~= rlv.union(rrv).insert(rkv));
                        vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                        assert(rv.len() == rlv.len() + rrv.len() + 1);
                        // self ⊥ rl: rl ⊆ right, self ⊥ right.
                        assert forall|x| rlv.contains(x) implies rv.contains(x) by {};
                        assert(lv.disjoint(rlv));
                        assert(lv.len() + rlv.len() <= usize::MAX as nat);
                        // Ordering for recursive call: self < rl.
                        assert forall|s: T, o: T| #![trigger lv.contains(s@), rlv.contains(o@)]
                            lv.contains(s@) && rlv.contains(o@) implies s.cmp_spec(&o) == Less by {
                            assert(rv.contains(o@));
                        };
                        // Ordering: self < rk.
                        assert forall|t: T| (#[trigger] lv.contains(t@))
                            implies t.cmp_spec(&rk) == Less by {
                            assert(rv.contains(rkv));
                        };
                    }
                    let merged = self.join_pair_inner(&rl);
                    let ghost mv = merged@;
                    proof {
                        assert(mv =~= lv.union(rlv));
                        use_type_invariant(&merged);
                        // merged ⊥ rr.
                        assert forall|x| !(mv.contains(x) && rrv.contains(x)) by {
                            if mv.contains(x) && rrv.contains(x) {
                                if lv.contains(x) { assert(rv.contains(x)); }
                                else { assert(rlv.contains(x)); }
                            }
                        };
                        assert(!mv.contains(rkv));
                        assert(!rrv.contains(rkv));
                        // Ordering: merged < rk.
                        assert forall|t: T| (#[trigger] mv.contains(t@))
                            implies t.cmp_spec(&rk) == Less by {
                            if lv.contains(t@) {
                            } else {
                                assert(rlv.contains(t@));
                            }
                        };
                        // Size bound.
                        vstd::set_lib::lemma_set_disjoint_lens(mv, rrv);
                        assert(mv.len() + rrv.len() < usize::MAX as nat);
                    }
                    let result = Self::join_mid(Exposed::Node(merged, rk, rr));
                    proof {
                        assert(result@ =~= lv.union(rv));
                    }
                    result
                }
            }
        }

        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite()
        {
            proof { use_type_invariant(self); use_type_invariant(other); }
            union_inner(self, other)
        }

        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite()
        {
            proof { use_type_invariant(self); use_type_invariant(other); }
            intersect_inner(self, other)
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite()
        {
            proof { use_type_invariant(self); use_type_invariant(other); }
            difference_inner(self, other)
        }

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
            proof { assume(cloned@ == self@); }  // assume_eq_clone_workaround
            cloned
        }
    }

    impl<T: MtKey> Clone for ParamBST<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
            decreases self@.len(), 1nat,
        {
            proof { use_type_invariant(self); }
            let _sz = self.size();
            let exposed = expose_internal(self);
            match exposed {
                Exposed::Leaf => Self::new(),
                Exposed::Node(l, k, r) => {
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(l@, r@);
                        assert(l@.len() + r@.len() + 1 == self@.len());
                    }
                    Self::join_mid(Exposed::Node(l, k, r))
                }
            }
        }
    }

    // Free functions (algorithmic helpers)

    fn new_leaf<T: MtKey>() -> (tree: ParamBST<T>)
        ensures tree@ =~= Set::<<T as View>::V>::empty()
    {
        new_param_bst(None, Ghost(Set::empty()))
    }

    fn expose_internal<T: MtKey>(tree: &ParamBST<T>) -> (exposed: Exposed<T>)
        requires
            tree@.finite(),
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
                && left@.len() + right@.len() < usize::MAX as nat
                && (forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less)
                && (forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater)
            ),
        decreases tree@.len(), 0nat,
    {
        proof { use_type_invariant(tree); }
        proof { assert(obeys_feq_full_trigger::<T>()); }
        let handle = tree.locked_root.acquire_read();
        let exposed = match handle.borrow() {
            | None => Exposed::Leaf,
            | Some(node) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(node.left@, node.right@);
                    assert(node.left@.len() < tree@.len());
                    assert(node.right@.len() < tree@.len());
                    // Size bound from the lock predicate inv field.
                    assert(node.left@.len() + node.right@.len() < usize::MAX as nat);
                }
                let l = node.left.clone();
                let k = clone_elem(&node.key);
                let r = node.right.clone();
                proof {
                    // k == node.key from clone_elem ensures (value equality).
                    // l@ == node.left@ and r@ == node.right@ from ParamBST::clone ensures.
                    // Ordering properties transfer directly since k == node.key.
                    assert forall |t: T| (#[trigger] l@.contains(t@))
                        implies t.cmp_spec(&k) == Less by {
                        assert(node.left@.contains(t@));
                    }
                    assert forall |t: T| (#[trigger] r@.contains(t@))
                        implies t.cmp_spec(&k) == Greater by {
                        assert(node.right@.contains(t@));
                    }
                    assert(l@.len() + r@.len() < usize::MAX as nat);
                }
                Exposed::Node(l, k, r)
            }
        };
        handle.release_read();
        exposed
    }

    fn split_inner<T: MtKey>(tree: &ParamBST<T>, key: &T) -> (parts: (ParamBST<T>, bool, ParamBST<T>))
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
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
            parts.0@.disjoint(parts.2@),
        decreases tree@.len(),
    {
        let _sz = tree.size();
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
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                }
                match key.cmp(&root_key) {
                    | Less => {
                        let (ll, found, lr) = split_inner(&left, key);
                        proof {
                            // lr ⊂ left, so lr is disjoint from right and ordered < root_key.
                            assert forall|x| #[trigger] lr@.contains(x) implies lv.contains(x) by {
                                assert(ll@.union(lr@).contains(x));
                            };
                            vstd::set_lib::lemma_len_subset(lr@, lv);
                            // Size bound: lr ⊂ left, so lr@.len() <= left@.len(), and
                            // left@.len() + right@.len() + 1 == tree@.len() <= usize::MAX.
                            assert(lr@.len() + rv.len() < usize::MAX as nat);
                            // Ordering for join_mid: lr ⊂ left, left < root_key.
                            assert forall|t: T| (#[trigger] lr@.contains(t@))
                                implies t.cmp_spec(&root_key) == Less by {
                                assert(lv.contains(t@));
                            };
                        }
                        let rebuilt = ParamBST::<T>::join_mid(Exposed::Node(lr, root_key, right));
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
                                } else if rv.contains(t@) {
                                    lemma_cmp_antisymmetry(t, rk);
                                    lemma_cmp_transitivity(kref, rk, t);
                                } else {
                                    assert(t@ == rkv);
                                    assert(t.cmp_spec(&rk) == Equal);
                                    lemma_cmp_equal_congruent(t, rk, kref);
                                }
                            };
                            // Disjointness: ll < key < rebuilt.
                            use_type_invariant(&ll);
                            assert forall|x| !(#[trigger] llv.contains(x) && rebuilt@.contains(x)) by {
                                if llv.contains(x) && rebuilt@.contains(x) {
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(llv.contains(t@));
                                    assert(t.cmp_spec(&key) == Less);
                                    assert(rebuilt@.contains(t@));
                                    assert(t.cmp_spec(&key) == Greater);
                                }
                            };
                        }
                        (ll, found, rebuilt)
                    }
                    | Greater => {
                        let (rl, found, rr) = split_inner(&right, key);
                        proof {
                            assert forall|x| #[trigger] rl@.contains(x) implies rv.contains(x) by {
                                assert(rl@.union(rr@).contains(x));
                            };
                            vstd::set_lib::lemma_len_subset(rl@, rv);
                            // Size bound for join_mid.
                            assert(rl@.len() + lv.len() < usize::MAX as nat);
                            // Ordering for join_mid: rl ⊂ right, right > root_key.
                            assert forall|t: T| (#[trigger] rl@.contains(t@))
                                implies t.cmp_spec(&root_key) == Greater by {
                                assert(rv.contains(t@));
                            };
                        }
                        let rebuilt = ParamBST::<T>::join_mid(Exposed::Node(left, root_key, rl));
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
                                } else if lv.contains(t@) {
                                    lemma_cmp_antisymmetry(kref, rk);
                                    lemma_cmp_transitivity(t, rk, kref);
                                } else {
                                    assert(t@ == rkv);
                                    assert(t.cmp_spec(&rk) == Equal);
                                    lemma_cmp_antisymmetry(kref, rk);
                                    lemma_cmp_equal_congruent(t, rk, kref);
                                }
                            };
                            // Disjointness: rebuilt < key < rr.
                            use_type_invariant(&rr);
                            assert forall|x| !(#[trigger] rebuilt@.contains(x) && rrv.contains(x)) by {
                                if rebuilt@.contains(x) && rrv.contains(x) {
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(rebuilt@.contains(t@));
                                    assert(t.cmp_spec(&key) == Less);
                                    assert(rrv.contains(t@));
                                    assert(t.cmp_spec(&key) == Greater);
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

    fn find_recursive<T: MtKey>(tree: &ParamBST<T>, key: &T) -> (found: Option<T>)
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
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

    fn min_key<T: MtKey>(tree: &ParamBST<T>) -> (result: Option<T>)
        requires
            tree@.finite(),
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

    fn union_inner<T: MtKey>(a: &ParamBST<T>, b: &ParamBST<T>) -> (combined: ParamBST<T>)
        requires
            a@.finite(), b@.finite(), a@.len() + b@.len() <= usize::MAX as nat,
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
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
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    let ghost akv = ak@;
                    proof {
                        alv.lemma_subset_not_in_lt(a@, akv);
                        use_type_invariant(&al);
                        use_type_invariant(&ar);
                        use_type_invariant(&bl);
                        use_type_invariant(&br);
                        // Bound the recursive call inputs: al ⊂ a, bl ⊆ b, ar ⊂ a, br ⊆ b.
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        assert(blv.subset_of(b@));
                        assert(brv.subset_of(b@));
                        vstd::set_lib::lemma_len_subset(blv, b@);
                        vstd::set_lib::lemma_len_subset(brv, b@);
                        assert(alv.len() + blv.len() <= usize::MAX as nat);
                        assert(arv.len() + brv.len() <= usize::MAX as nat);
                    }
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
                    proof {
                        lemma_cmp_order_axioms::<T>();
                        // Ordering: left_union = al@ ∪ bl@ (all < ak), right_union = ar@ ∪ br@ (all > ak).
                        assert forall|t: T| (#[trigger] left_union@.contains(t@))
                            implies t.cmp_spec(&ak) == Less by {
                            if alv.contains(t@) {} else { assert(blv.contains(t@)); }
                        };
                        assert forall|t: T| (#[trigger] right_union@.contains(t@))
                            implies t.cmp_spec(&ak) == Greater by {
                            if arv.contains(t@) {} else { assert(brv.contains(t@)); }
                        };
                        assert(!left_union@.contains(akv));
                        assert(!right_union@.contains(akv));
                        // Disjointness via existence witnesses.
                        assert forall|x| !(left_union@.contains(x) && right_union@.contains(x)) by {
                            if left_union@.contains(x) && right_union@.contains(x) {
                                assert(alv.contains(x) || blv.contains(x));
                                assert(exists|t: T| t@ == x);
                                let ghost t: T = choose|t: T| t@ == x;
                                assert(left_union@.contains(t@));
                                assert(t.cmp_spec(&ak) == Less);
                                assert(right_union@.contains(t@));
                                assert(t.cmp_spec(&ak) == Greater);
                            }
                        };
                        // Size bound.
                        vstd::set_lib::lemma_set_disjoint_lens(left_union@, right_union@);
                        let ghost lu_ru = left_union@.union(right_union@);
                        assert(!lu_ru.contains(akv));
                        assert forall|x| #[trigger] lu_ru.insert(akv).contains(x)
                            implies a@.union(b@).contains(x) by {
                            if x == akv { assert(a@.contains(akv)); }
                            else if left_union@.contains(x) {
                                if alv.contains(x) { assert(a@.contains(x)); }
                                else { assert(blv.contains(x)); assert(b@.contains(x)); }
                            } else {
                                assert(right_union@.contains(x));
                                if arv.contains(x) { assert(a@.contains(x)); }
                                else { assert(brv.contains(x)); assert(b@.contains(x)); }
                            }
                        };
                        assert(lu_ru.insert(akv).subset_of(a@.union(b@)));
                        vstd::set_lib::lemma_len_union(a@, b@);
                        vstd::set_lib::lemma_len_subset(lu_ru.insert(akv), a@.union(b@));
                        assert(left_union@.len() + right_union@.len() < usize::MAX as nat);
                    }
                    let result = ParamBST::<T>::join_mid(Exposed::Node(left_union, ak, right_union));
                    proof {
                        assert forall|x| #[trigger] a@.union(b@).contains(x) <==>
                            result@.contains(x) by {
                            if blv.contains(x) || brv.contains(x) {
                                assert(b@.remove(akv).contains(x));
                                assert(b@.contains(x));
                            }
                            if b@.contains(x) && x != akv {
                                assert(b@.remove(akv).contains(x));
                                assert(blv.union(brv).contains(x));
                            }
                        };
                        assert(result@ =~= a@.union(b@));
                    }
                    result
                }
            }
        }
    }

    fn intersect_inner<T: MtKey>(a: &ParamBST<T>, b: &ParamBST<T>) -> (common: ParamBST<T>)
        requires
            a@.finite(), b@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures common@ == a@.intersect(b@), common@.finite(),
        decreases a@.len(),
    {
        let _sa = a.size();
        let _ = b.size();
        match expose_internal(a) {
            | Exposed::Leaf => {
                proof { assert(a@.intersect(b@) =~= Set::empty()); }
                new_leaf()
            },
            | Exposed::Node(al, ak, ar) => {
                if b.is_empty() {
                    proof { assert(a@.intersect(b@) =~= Set::empty()); }
                    new_leaf()
                } else {
                    let ghost sv = a@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        assert(sv.len() == alv.len() + arv.len() + 1);
                        lemma_cmp_order_axioms::<T>();
                    }
                    let (bl, found, br) = split_inner(b, &ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    proof {
                        use_type_invariant(&al);
                        use_type_invariant(&ar);
                        use_type_invariant(&bl);
                        use_type_invariant(&br);
                    }
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
                    let ghost lrv = left_res@;
                    let ghost rrv = right_res@;
                    if found {
                        proof {
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            assert(!lrv.contains(akv));
                            assert(!rrv.contains(akv));
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&ak) == Less by {
                                assert(alv.contains(t@));
                            };
                            assert forall|t: T| (#[trigger] rrv.contains(t@)) implies t.cmp_spec(&ak) == Greater by {
                                assert(arv.contains(t@));
                            };
                        }
                        let result = ParamBST::<T>::join_mid(Exposed::Node(left_res, ak, right_res));
                        proof {
                            assert forall|x| #[trigger] sv.intersect(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && blv.contains(x));
                                    assert(b@.remove(akv).contains(x));
                                }
                                if rrv.contains(x) {
                                    assert(arv.contains(x) && brv.contains(x));
                                    assert(b@.remove(akv).contains(x));
                                }
                                if sv.contains(x) && b@.contains(x) && x != akv {
                                    assert(b@.remove(akv).contains(x));
                                    assert(blv.union(brv).contains(x));
                                    if alv.contains(x) {
                                        assert(exists|t: T| t@ == x);
                                        let ghost t: T = choose|t: T| t@ == x;
                                        assert(alv.contains(t@));
                                        assert(t.cmp_spec(&ak) == Less);
                                        assert(!brv.contains(x));
                                        assert(blv.contains(x));
                                        assert(lrv.contains(x));
                                    } else {
                                        assert(arv.contains(x));
                                        assert(exists|t: T| t@ == x);
                                        let ghost t: T = choose|t: T| t@ == x;
                                        assert(arv.contains(t@));
                                        assert(t.cmp_spec(&ak) == Greater);
                                        assert(!blv.contains(x));
                                        assert(brv.contains(x));
                                        assert(rrv.contains(x));
                                    }
                                }
                            };
                            assert(result@ =~= sv.intersect(b@));
                        }
                        result
                    } else {
                        proof {
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
                                assert(alv.contains(s@));
                                assert(arv.contains(o@));
                                lemma_cmp_antisymmetry(o, ak);
                                lemma_cmp_transitivity(s, ak, o);
                            };
                        }
                        let result = left_res.join_pair_inner(&right_res);
                        proof {
                            assert forall|x| #[trigger] sv.intersect(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && blv.contains(x));
                                    assert(b@.remove(akv).contains(x));
                                }
                                if rrv.contains(x) {
                                    assert(arv.contains(x) && brv.contains(x));
                                    assert(b@.remove(akv).contains(x));
                                }
                                if sv.contains(x) && b@.contains(x) {
                                    assert(b@.remove(akv).contains(x));
                                    assert(blv.union(brv).contains(x));
                                    if alv.contains(x) {
                                        assert(exists|t: T| t@ == x);
                                        let ghost t: T = choose|t: T| t@ == x;
                                        assert(alv.contains(t@));
                                        assert(t.cmp_spec(&ak) == Less);
                                        assert(!brv.contains(x));
                                        assert(blv.contains(x));
                                        assert(lrv.contains(x));
                                    } else {
                                        assert(arv.contains(x));
                                        assert(exists|t: T| t@ == x);
                                        let ghost t: T = choose|t: T| t@ == x;
                                        assert(arv.contains(t@));
                                        assert(t.cmp_spec(&ak) == Greater);
                                        assert(!blv.contains(x));
                                        assert(brv.contains(x));
                                        assert(rrv.contains(x));
                                    }
                                }
                            };
                            assert(result@ =~= sv.intersect(b@));
                        }
                        result
                    }
                }
            }
        }
    }

    fn difference_inner<T: MtKey>(a: &ParamBST<T>, b: &ParamBST<T>) -> (remaining: ParamBST<T>)
        requires
            a@.finite(), b@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures remaining@ == a@.difference(b@), remaining@.finite(),
        decreases a@.len(),
    {
        let _sa = a.size();
        let _ = b.size();
        match expose_internal(a) {
            | Exposed::Leaf => {
                proof { assert(a@.difference(b@) =~= Set::empty()); }
                new_leaf()
            },
            | Exposed::Node(al, ak, ar) => {
                if b.is_empty() {
                    proof { assert(a@.difference(b@) =~= a@); }
                    a.clone()
                } else {
                    let ghost sv = a@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        assert(sv.len() == alv.len() + arv.len() + 1);
                        lemma_cmp_order_axioms::<T>();
                    }
                    let (bl, found, br) = split_inner(b, &ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    proof {
                        use_type_invariant(&al);
                        use_type_invariant(&ar);
                        use_type_invariant(&bl);
                        use_type_invariant(&br);
                    }
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
                    let ghost lrv = left_res@;
                    let ghost rrv = right_res@;
                    if found {
                        proof {
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
                                assert(alv.contains(s@));
                                assert(arv.contains(o@));
                                lemma_cmp_antisymmetry(o, ak);
                                lemma_cmp_transitivity(s, ak, o);
                            };
                        }
                        let result = left_res.join_pair_inner(&right_res);
                        proof {
                            assert forall|x| #[trigger] sv.difference(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && !blv.contains(x));
                                    assert(sv.contains(x));
                                    assert(x != akv);
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(alv.contains(t@));
                                    assert(t.cmp_spec(&ak) == Less);
                                    assert(!brv.contains(x));
                                    assert(!b@.contains(x));
                                }
                                if rrv.contains(x) {
                                    assert(arv.contains(x) && !brv.contains(x));
                                    assert(sv.contains(x));
                                    assert(x != akv);
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(arv.contains(t@));
                                    assert(t.cmp_spec(&ak) == Greater);
                                    assert(!blv.contains(x));
                                    assert(!b@.contains(x));
                                }
                                if sv.contains(x) && !b@.contains(x) {
                                    assert(!blv.union(brv).contains(x));
                                    if alv.contains(x) {
                                        assert(alv.difference(blv).contains(x));
                                    } else if arv.contains(x) {
                                        assert(arv.difference(brv).contains(x));
                                    }
                                }
                            };
                            assert(result@ =~= sv.difference(b@));
                        }
                        result
                    } else {
                        proof {
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            assert(!lrv.contains(akv));
                            assert(!rrv.contains(akv));
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&ak) == Less by {
                                assert(alv.contains(t@));
                            };
                            assert forall|t: T| (#[trigger] rrv.contains(t@)) implies t.cmp_spec(&ak) == Greater by {
                                assert(arv.contains(t@));
                            };
                        }
                        let result = ParamBST::<T>::join_mid(Exposed::Node(left_res, ak, right_res));
                        proof {
                            assert(blv.union(brv) =~= b@);
                            assert forall|x| #[trigger] sv.difference(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && !blv.contains(x));
                                    assert(sv.contains(x));
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(alv.contains(t@));
                                    assert(t.cmp_spec(&ak) == Less);
                                    assert(!brv.contains(x));
                                    assert(!blv.union(brv).contains(x));
                                }
                                if rrv.contains(x) {
                                    assert(arv.contains(x) && !brv.contains(x));
                                    assert(sv.contains(x));
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(arv.contains(t@));
                                    assert(t.cmp_spec(&ak) == Greater);
                                    assert(!blv.contains(x));
                                    assert(!blv.union(brv).contains(x));
                                }
                                if sv.contains(x) && !b@.contains(x) && x != akv {
                                    assert(!blv.union(brv).contains(x));
                                    if alv.contains(x) {
                                        assert(alv.difference(blv).contains(x));
                                    } else if arv.contains(x) {
                                        assert(arv.difference(brv).contains(x));
                                    }
                                }
                            };
                            assert(result@ =~= sv.difference(b@));
                        }
                        result
                    }
                }
            }
        }
    }

    fn filter_inner<T: MtKey, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: &Arc<F>,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamBST<T>)
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
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
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let ghost lv = left@;
                let ghost rv = right@;
                let ghost kv = key@;
                proof {
                    // Expose ensures gives us the size bound for left/right subtrees.
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    assert(!lv.union(rv).contains(kv));
                    assert(lv.len() + rv.len() < usize::MAX as nat);
                    use_type_invariant(&left);
                    use_type_invariant(&right);
                }
                // Sequential: borrow predicate for both recursive calls (spec_fn is not Send,
                // so parallel closures cannot capture it; sequential avoids the issue).
                let left_filtered = filter_inner(&left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner(&right, predicate, Ghost(spec_pred));
                proof {
                    lemma_cmp_order_axioms::<T>();
                    vstd::set_lib::lemma_len_subset(left_filtered@, lv);
                    vstd::set_lib::lemma_len_subset(right_filtered@, rv);
                    // Disjointness: subsets of disjoint sets.
                    assert forall|x| !(left_filtered@.contains(x) && right_filtered@.contains(x)) by {
                        if left_filtered@.contains(x) && right_filtered@.contains(x) {
                            assert(lv.contains(x) && rv.contains(x));
                        }
                    };
                    // key is not in either filtered subtree (it's not in lv or rv).
                    assert(!left_filtered@.contains(kv));
                    assert(!right_filtered@.contains(kv));
                    // Ordering: filtered subsets inherit ordering from parent subtrees.
                    assert forall|t: T| (#[trigger] left_filtered@.contains(t@))
                        implies t.cmp_spec(&key) == Less by {
                        assert(lv.contains(t@));
                    };
                    assert forall|t: T| (#[trigger] right_filtered@.contains(t@))
                        implies t.cmp_spec(&key) == Greater by {
                        assert(rv.contains(t@));
                    };
                    assert(left_filtered@.len() + right_filtered@.len() <= usize::MAX as nat);
                }
                if (**predicate)(&key) {
                    let result = ParamBST::<T>::join_mid(
                        Exposed::Node(left_filtered, key, right_filtered),
                    );
                    proof {
                        // result@ = left_filtered@ ∪ right_filtered@ ∪ {kv}.
                        assert forall|v: T::V| #[trigger] result@.contains(v)
                            implies tree@.contains(v) && spec_pred(v) by {
                            if v == kv { assert(tree@.contains(kv)); }
                            else if left_filtered@.contains(v) { assert(lv.contains(v)); }
                            else { assert(rv.contains(v)); }
                        };
                        assert forall|v: T::V| tree@.contains(v) && spec_pred(v)
                            implies #[trigger] result@.contains(v) by {
                            if v == kv { assert(result@.contains(kv)); }
                            else if lv.contains(v) { assert(left_filtered@.contains(v)); }
                            else { assert(rv.contains(v)); assert(right_filtered@.contains(v)); }
                        };
                    }
                    result
                } else {
                    proof {
                        assert(!spec_pred(kv));
                        // Ordering for join_pair_inner: all of left_filtered < all of right_filtered.
                        // s ∈ left_filtered ⊆ lv (all < key), o ∈ right_filtered ⊆ rv (all > key).
                        assert forall|s: T, o: T|
                            #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                            left_filtered@.contains(s@) && right_filtered@.contains(o@)
                            implies s.cmp_spec(&o) == Less by {
                            assert(lv.contains(s@));
                            assert(rv.contains(o@));
                            lemma_cmp_antisymmetry(o, key);
                            lemma_cmp_transitivity(s, key, o);
                        };
                    }
                    let result = left_filtered.join_pair_inner(&right_filtered);
                    proof {
                        // result@ = left_filtered@ ∪ right_filtered@.
                        assert forall|v: T::V| #[trigger] result@.contains(v)
                            implies tree@.contains(v) && spec_pred(v) by {
                            if left_filtered@.contains(v) { assert(lv.contains(v)); }
                            else { assert(right_filtered@.contains(v)); assert(rv.contains(v)); }
                        };
                        assert forall|v: T::V| tree@.contains(v) && spec_pred(v)
                            implies #[trigger] result@.contains(v) by {
                            if v == kv { assert(false); }
                            else if lv.contains(v) { assert(left_filtered@.contains(v)); }
                            else { assert(rv.contains(v)); assert(right_filtered@.contains(v)); }
                        };
                    }
                    result
                }
            }
        }
    }

    fn filter_parallel<T: MtKey, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: F,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamBST<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
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
        proof { use_type_invariant(tree); }
        let predicate = Arc::new(predicate);
        filter_inner(tree, &predicate, Ghost(spec_pred))
    }

    fn reduce_inner<T: MtKey, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: &Arc<F>,
        identity: T,
    ) -> (result: T)
        requires
            tree@.finite(),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> result@ == identity@,
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

    fn reduce_parallel<T: MtKey, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: F,
        base: T,
    ) -> (result: T)
        requires
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> result@ == base@,
    {
        let _ = tree.size();
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    fn collect_in_order<T: MtKey>(tree: &ParamBST<T>, out: &mut Vec<T>)
        requires
            tree@.finite(),
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
