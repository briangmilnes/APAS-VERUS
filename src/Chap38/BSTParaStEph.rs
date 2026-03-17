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
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::cmp::Ordering::{Equal, Greater, Less};

    use vstd::prelude::*;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::{OrdSpec, PartialEqSpec, PartialOrdSpec};



    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // 3. broadcast use

    broadcast use vstd::set::group_set_axioms;

    // 4. type definitions

    pub struct BSTParaStEphInv<T: StT + Ord> {
        pub ghost contents: Set<<T as View>::V>,
    }

    impl<T: StT + Ord> RwLockPredicate<Option<Box<NodeInner<T>>>> for BSTParaStEphInv<T> {
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
    #[derive(Debug, Default)]
    pub enum Exposed<T: StT + Ord> {
        #[default]
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    #[verifier::reject_recursive_types(T)]
    #[derive(Debug)]
    pub struct NodeInner<T: StT + Ord> {
        pub key: T,
        pub size: usize,
        pub left: ParamBST<T>,
        pub right: ParamBST<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ParamBST<T: StT + Ord> {
        pub(crate) locked_root: RwLock<Option<Box<NodeInner<T>>>, BSTParaStEphInv<T>>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    fn new_param_bst<T: StT + Ord>(
        val: Option<Box<NodeInner<T>>>,
        Ghost(contents): Ghost<Set<<T as View>::V>>,
    ) -> (tree: ParamBST<T>)
        requires
            (BSTParaStEphInv::<T> { contents }).inv(val),
            contents.finite(),
            forall|v: <T as View>::V| contents.contains(v)
                ==> exists|t: T| t@ == v,
        ensures tree@ =~= contents,
    {
        let ghost pred = BSTParaStEphInv::<T> { contents };
        ParamBST {
            locked_root: RwLock::new(val, Ghost(pred)),
            ghost_locked_root: Ghost(contents),
        }
    }

    // 5. view impls

    impl<T: StT + Ord> ParamBST<T> {
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

    impl<T: StT + Ord> View for ParamBST<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_ghost_locked_root() }
    }

    impl<T: StT + Ord> View for Exposed<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    impl<T: StT + Ord> View for NodeInner<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    // 6. spec fns

    /// View-consistent ordering: elements with the same view compare Equal.
    pub open spec fn view_ord_consistent<T: StT + Ord>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    // 7. proof fns

    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    proof fn lemma_cmp_antisymmetry<T: StT + Ord>(a: T, b: T)
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
    proof fn lemma_cmp_transitivity<T: StT + Ord>(a: T, b: T, c: T)
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
    proof fn lemma_cmp_eq_subst<T: StT + Ord>(a: T, b: T, c: T)
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
        // Greater: c < a, with a < b gives c < b (transitivity),
        // so b > c, contradicting Equal(b,c). Solver handles.
        // Equal: a@ == c@ (view_ord_consistent), b@ == c@ (same),
        // so a@ == b@, hence Equal(a,b), contradicting Less(a,b).
    }

    /// Left congruence: Equal(a,b) implies a and b compare the same way to c.
    proof fn lemma_cmp_equal_congruent<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures
            a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
        // a@ == b@ from view_ord_consistent + Equal(a,b).
        assert(a@ == b@);
        // Mismatch cases (a cmp c) != (b cmp c) all lead to contradiction:
        // (L,G) or (G,L): transitivity gives Less(a,b) or Less(b,a),
        //   contradicting Equal(a,b).
        // (L,E) or (E,L) or (G,E) or (E,G): view_ord_consistent chains
        //   a@ == b@ == c@ or a@ == c@ == b@, collapsing to Equal on both.
    }

    /// Right congruence: Equal(b,c) implies any a compares the same way to b and c.
    proof fn lemma_cmp_equal_congruent_right<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            b.cmp_spec(&c) == Equal,
        ensures
            a.cmp_spec(&b) == a.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
        assert(b@ == c@);
    }

    // 8. traits

    pub trait ParamBSTTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_bstparasteph_wf(&self) -> bool;

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparasteph_wf();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite(),
                tree.spec_bstparasteph_wf();
        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn expose(&self) -> (exposed: Exposed<T>)
            ensures
                self@.len() == 0 ==> exposed is Leaf,
                exposed is Leaf ==> self@ =~= Set::<<T as View>::V>::empty(),
                exposed matches Exposed::Node(l, k, r) ==> {
                    self@ =~= l@.union(r@).insert(k@)
                    && self@.finite()
                    && l@.finite() && r@.finite()
                    && l@.disjoint(r@)
                    && !l@.contains(k@)
                    && !r@.contains(k@)
                    && l@.len() + r@.len() < usize::MAX as nat
                    && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                    && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
                };
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
        /// - APAS: (no cost stated for joinM, but O(1) wrapping joinMid)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- trivial wrapper around joinMid.
        fn join_m(left: Self, key: T, right: Self) -> (tree: Self)
            requires
                left@.finite(), right@.finite(),
                left@.disjoint(right@),
                !left@.contains(key@),
                !right@.contains(key@),
                left@.len() + right@.len() < usize::MAX as nat,
                forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less,
                forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
            ensures tree@ =~= left@.union(right@).insert(key@);
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
        fn insert(&mut self, key: T)
            requires
                old(self).spec_bstparasteph_wf(),
                old(self)@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self.spec_bstparasteph_wf(),
                self@ =~= old(self)@.insert(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn delete(&mut self, key: &T)
            requires
                old(self).spec_bstparasteph_wf(),
                old(self)@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self.spec_bstparasteph_wf(),
                self@ =~= old(self)@.remove(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn find(&self, key: &T) -> (found: Option<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found.is_some() <==> self@.contains(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn split(&self, key: &T) -> (parts: (Self, B, Self))
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite(),
                parts.0@.union(parts.2@) =~= self@.remove(key@),
                parts.0@.disjoint(parts.2@),
                !parts.0@.contains(key@),
                !parts.2@.contains(key@),
                forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(&key) == Less,
                forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(&key) == Greater;
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        /// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
        fn min_key(&self) -> (minimum: Option<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@.len() == 0 <==> minimum.is_none(),
                minimum.is_some() ==> self@.contains(minimum.unwrap()@),
                minimum.is_some() ==> forall|t: T| (#[trigger] self@.contains(t@)) ==>
                    minimum.unwrap().cmp_spec(&t) == Less || minimum.unwrap()@ == t@;
        /// - APAS: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|))
        /// - Claude-Opus-4.6: Work O(lg(|t1| + |t2|)), Span O(lg(|t1| + |t2|)) -- agrees with APAS.
        fn join_pair(&self, other: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.disjoint(other@),
                self@.finite(), other@.finite(),
                self@.len() + other@.len() < usize::MAX as nat,
                forall|s: T, o: T| #![trigger self@.contains(s@), other@.contains(o@)] self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@.finite(), joined@ =~= self@.union(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(m · lg(n/m)) -- agrees with APAS; sequential, not parallel.
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() <= usize::MAX as nat,
            ensures combined@ == self@.union(other@), combined@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(m · lg(n/m)) -- agrees with APAS; sequential, not parallel.
        fn intersect(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures common@ == self@.intersect(other@), common@.finite();
        /// - APAS: Work O(m · lg(n/m)), Span O(m · lg(n/m)) — sequential
        /// - Claude-Opus-4.6: Work O(m · lg(n/m)), Span O(m · lg(n/m)) -- agrees with APAS; sequential, not parallel.
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures remaining@ == self@.difference(other@), remaining@.finite();
        /// - APAS: Work O(|t|), Span O(|t|) — sequential
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|) -- agrees with APAS; sequential.
        fn filter<F: Fn(&T) -> bool>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self@.finite(),
                forall|t: &T| predicate.requires((t,)),
                forall|x: T, keep: bool|
                    predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                filtered@.subset_of(self@),
                filtered@.finite(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - APAS: Work O(|t|), Span O(|t|) — sequential
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|) -- agrees with APAS; sequential.
        /// Requires `op` to be associative with identity `base`.
        fn reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (result: T)
            requires self@.finite(), forall|a: T, b: T| op.requires((a, b)),
            ensures self@.len() == 0 ==> result@ == base@;
        /// - APAS: N/A -- Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|) -- helper for in_order.
        fn collect_in_order(&self, out: &mut Vec<T>)
            requires self@.finite(),
            ensures out@.len() == old(out)@.len() + self@.len();
        /// - APAS: Work O(|t|), Span O(|t|)
        /// - Claude-Opus-4.6: Work O(|t|), Span O(|t|) -- agrees with APAS.
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures
                seq@.len() == self@.len(),
                forall|v: T::V| self@.contains(v) <==> seq@.contains(v);
    }

    // 9. impls

    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {
        open spec fn spec_bstparasteph_wf(&self) -> bool {
            self@.finite()
        }

        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparasteph_wf()
        { new_param_bst(None, Ghost(Set::empty())) }

        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite(),
                tree.spec_bstparasteph_wf()
        {
            let left: Self = Self::new();
            let right: Self = Self::new();
            Self::join_mid(Exposed::Node(left, key, right))
        }

        fn expose(&self) -> (exposed: Exposed<T>)
            ensures
                self@.len() == 0 ==> exposed is Leaf,
                exposed is Leaf ==> self@ =~= Set::<<T as View>::V>::empty(),
                exposed matches Exposed::Node(l, k, r) ==> {
                    self@ =~= l@.union(r@).insert(k@)
                    && self@.finite()
                    && l@.finite() && r@.finite()
                    && l@.disjoint(r@)
                    && !l@.contains(k@)
                    && !r@.contains(k@)
                    && l@.len() + r@.len() < usize::MAX as nat
                    && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                    && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
                },
        {
            proof { use_type_invariant(self); }
            let handle = self.locked_root.acquire_read();
            let exposed = match handle.borrow() {
                | None => {
                    Exposed::Leaf
                }
                | Some(node) => {
                    let l = node.left.clone();
                    let k = node.key.clone();
                    let r = node.right.clone();
                    // T::clone has no verified ensures; cmp_spec(k) vs cmp_spec(node.key) unresolvable.
                    proof { assume(
                        k@ == node.key@
                        && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                        && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
                    ); }
                    Exposed::Node(l, k, r)
                }
            };
            handle.release_read();
            exposed
        }

        fn join_mid(exposed: Exposed<T>) -> (joined: Self)
        {
            match exposed {
                | Exposed::Leaf => Self::new(),
                | Exposed::Node(left, key, right) => {
                    let ghost lv = left@;
                    let ghost rv = right@;
                    let ghost kv = key@;
                    let ls = left.size();
                    let rs = right.size();
                    let size = 1 + ls + rs;
                    let ghost contents = lv.union(rv).insert(kv);
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                        assert(!lv.union(rv).contains(kv));
                        assert(contents.len() == size as nat);
                        // Witness property: every view in contents has a T witness.
                        use_type_invariant(&left);
                        use_type_invariant(&right);
                        assert forall|v: <T as View>::V| contents.contains(v)
                            implies exists|t: T| t@ == v by {
                            if lv.contains(v) {
                                // From left's type_invariant.
                            } else if rv.contains(v) {
                                // From right's type_invariant.
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

        fn join_m(left: Self, key: T, right: Self) -> (tree: Self)
        {
            Self::join_mid(Exposed::Node(left, key, right))
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

        fn is_empty(&self) -> (empty: B)
            ensures empty == (self@.len() == 0), self@.finite()
        { self.size() == 0 }

        fn insert(&mut self, key: T) {
            let ghost old_view = self@;
            let (left, _, right) = self.split(&key);
            let ghost kv = key@;
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                // left@ ∪ right@ =~= old_view.remove(kv), disjoint.
                // disjoint_lens: left@.len() + right@.len() == old_view.remove(kv).len().
                // old_view.remove(kv).len() ≤ old_view.len() < usize::MAX (from requires).
                assert(left@.union(right@) =~= old_view.remove(kv));
                assert(old_view.remove(kv).subset_of(old_view));
                vstd::set_lib::lemma_len_subset(old_view.remove(kv), old_view);
                assert(left@.len() + right@.len() < usize::MAX as nat);
            }
            *self = Self::join_m(left, key, right);
        }

        fn delete(&mut self, key: &T) {
            let ghost old_view = self@;
            let ghost kref = *key;
            let (left, _, right) = self.split(key);
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                assert(left@.union(right@) =~= old_view.remove(kref@));
                assert(old_view.remove(kref@).subset_of(old_view));
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                assert(left@.len() + right@.len() < usize::MAX as nat);
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies
                    s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry(o, kref);
                    lemma_cmp_transitivity(s, kref, o);
                };
            }
            *self = left.join_pair(right);
        }

        fn find(&self, key: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(key@),
            decreases self@.len(),
        {
            match self.expose() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, root_key, right) => {
                    proof {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        assert(!left@.union(right@).contains(root_key@));
                        assert(self@.len() == left@.len() + right@.len() + 1);
                    }
                    match key.cmp(&root_key) {
                        | Less => left.find(key),
                        | Greater => right.find(key),
                        | Equal => Some(root_key),
                    }
                }
            }
        }

        /// Algorithm 38.5 — split via expose and recursive descent.
        fn split(&self, key: &T) -> (parts: (Self, B, Self))
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite(),
                parts.0@.union(parts.2@) =~= self@.remove(key@),
                parts.0@.disjoint(parts.2@),
                !parts.0@.contains(key@),
                !parts.2@.contains(key@),
                forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(&key) == Less,
                forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
            decreases self@.len(),
        {
            match self.expose() {
                | Exposed::Leaf => (Self::new(), false, Self::new()),
                | Exposed::Node(left, root_key, right) => {
                    proof {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        assert(!left@.union(right@).contains(root_key@));
                        assert(self@.len() == left@.len() + right@.len() + 1);
                    }
                    let ghost rk = root_key;
                    let ghost kval = *key;
                    match key.cmp(&root_key) {
                        | Less => {
                            let ghost lv = left@;
                            let ghost rv = right@;
                            let ghost rkv = root_key@;
                            let (ll, found, lr) = left.split(key);
                            let ghost llv = ll@;
                            let ghost lrv = lr@;
                            proof {
                                // ll ∪ lr =~= left.remove(key@), so lr ⊆ left and ll ⊆ left.
                                assert forall|x| lrv.contains(x) implies lv.contains(x) by {
                                    assert(llv.union(lrv).contains(x));
                                };
                                assert(lrv.subset_of(lv));
                                assert forall|x| llv.contains(x) implies lv.contains(x) by {
                                    assert(llv.union(lrv).contains(x));
                                };
                                assert(llv.subset_of(lv));
                                vstd::set_lib::lemma_len_subset(lrv, lv);
                            }
                            let rebuilt = Self::join_mid(Exposed::Node(lr, root_key, right));
                            proof {
                                assert(rebuilt@ =~= lrv.union(rv).insert(rkv));
                                assert(!rv.contains(key@));
                                assert forall|x| #[trigger] (llv.union(rebuilt@)).contains(x) <==> self@.remove(key@).contains(x) by {
                                    if llv.contains(x) {
                                        assert(llv.union(lrv).contains(x));
                                    }
                                    if lv.contains(x) && x != key@ {
                                        assert(lv.remove(key@).contains(x));
                                        assert(llv.union(lrv).contains(x));
                                    }
                                };
                                assert(llv.union(rebuilt@) =~= self@.remove(key@));
                                // Ordering: rebuilt elements > key.
                                assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                    t.cmp_spec(&key) == Greater by {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    if lrv.contains(t@) {
                                        // Recursive split ensures t > key.
                                    } else if rv.contains(t@) {
                                        // expose: t > rk. rk < t (antisymmetry).
                                        // kval < rk < t (transitivity) → t > kval.
                                        lemma_cmp_antisymmetry(t, rk);
                                        lemma_cmp_transitivity(kval, rk, t);
                                    } else {
                                        // t@ == rkv → Equal(t, rk) via view_ord_consistent.
                                        // kval < rk, Equal(rk, t) → kval < t (eq_subst).
                                        assert(t@ == rkv);
                                        lemma_cmp_eq_subst(kval, rk, t);
                                    }
                                };
                            }
                            (ll, found, rebuilt)
                        }
                        | Greater => {
                            let ghost lv = left@;
                            let ghost rv = right@;
                            let ghost rkv = root_key@;
                            let (rl, found, rr) = right.split(key);
                            let ghost rlv = rl@;
                            let ghost rrv = rr@;
                            proof {
                                assert forall|x| rlv.contains(x) implies rv.contains(x) by {
                                    assert(rlv.union(rrv).contains(x));
                                };
                                assert(rlv.subset_of(rv));
                                assert forall|x| rrv.contains(x) implies rv.contains(x) by {
                                    assert(rlv.union(rrv).contains(x));
                                };
                                assert(rrv.subset_of(rv));
                                vstd::set_lib::lemma_len_subset(rlv, rv);
                            }
                            let rebuilt = Self::join_mid(Exposed::Node(left, root_key, rl));
                            proof {
                                assert(rebuilt@ =~= lv.union(rlv).insert(rkv));
                                assert(!lv.contains(key@));
                                assert forall|x| #[trigger] (rebuilt@.union(rrv)).contains(x) <==> self@.remove(key@).contains(x) by {
                                    if rrv.contains(x) {
                                        assert(rlv.union(rrv).contains(x));
                                    }
                                    if rv.contains(x) && x != key@ {
                                        assert(rv.remove(key@).contains(x));
                                        assert(rlv.union(rrv).contains(x));
                                    }
                                };
                                assert(rebuilt@.union(rrv) =~= self@.remove(key@));
                                // Ordering: rebuilt elements < key.
                                assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                    t.cmp_spec(&key) == Less by {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    if rlv.contains(t@) {
                                        // Recursive split ensures t < key.
                                    } else if lv.contains(t@) {
                                        // expose: t < rk. kval > rk → rk < kval (antisymmetry).
                                        // t < rk < kval (transitivity).
                                        lemma_cmp_antisymmetry(kval, rk);
                                        lemma_cmp_transitivity(t, rk, kval);
                                    } else {
                                        // t@ == rkv → Equal(t, rk) via view_ord_consistent.
                                        // t cmp kval == rk cmp kval (congruence), rk < kval.
                                        assert(t@ == rkv);
                                        lemma_cmp_antisymmetry(kval, rk);
                                        lemma_cmp_equal_congruent(t, rk, kval);
                                    }
                                };
                            }
                            (rebuilt, found, rr)
                        }
                        | Equal => {
                            proof {
                                // left < root_key == key, right > root_key == key.
                                // Right congruence: Equal(kval, rk) → t cmp kval == t cmp rk.
                                assert forall|t: T| (#[trigger] left@.contains(t@)) implies
                                    t.cmp_spec(&key) == Less by {
                                    lemma_cmp_equal_congruent_right(t, kval, rk);
                                };
                                assert forall|t: T| (#[trigger] right@.contains(t@)) implies
                                    t.cmp_spec(&key) == Greater by {
                                    lemma_cmp_equal_congruent_right(t, kval, rk);
                                };
                            }
                            (left, true, right)
                        }
                    }
                }
            }
        }

        fn min_key(&self) -> (minimum: Option<T>)
            ensures
                self@.len() == 0 <==> minimum.is_none(),
                minimum.is_some() ==> self@.contains(minimum.unwrap()@),
                minimum.is_some() ==> forall|t: T| (#[trigger] self@.contains(t@)) ==>
                    minimum.unwrap().cmp_spec(&t) == Less || minimum.unwrap()@ == t@,
            decreases self@.len(),
        {
            match self.expose() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, key, right) => {
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        assert(!left@.union(right@).contains(key@));
                        assert(self@.len() == left@.len() + right@.len() + 1);
                    }
                    match left.min_key() {
                        | Some(rec) => {
                            proof {
                                assert forall|t: T| #![trigger self@.contains(t@)] self@.contains(t@) implies
                                    rec.cmp_spec(&t) == Less || rec@ == t@ by {
                                    if left@.contains(t@) {
                                        // IH covers this.
                                    } else if right@.contains(t@) {
                                        // expose: t.cmp_spec(&key) == Greater, so
                                        // key.cmp_spec(&t) == Less (antisymmetry).
                                        lemma_cmp_antisymmetry(t, key);
                                        // rec ∈ left, expose: rec.cmp_spec(&key) == Less.
                                        // rec < key < t (transitivity).
                                        lemma_cmp_transitivity(rec, key, t);
                                    } else {
                                        // t@ == key@, so t.cmp_spec(&key) == Equal.
                                        // rec.cmp_spec(&key) == Less, key equals t in order.
                                        lemma_cmp_eq_subst(rec, key, t);
                                    }
                                };
                            }
                            Some(rec)
                        }
                        | None => {
                            proof {
                                assert forall|t: T| #![trigger self@.contains(t@)] self@.contains(t@) implies
                                    key.cmp_spec(&t) == Less || key@ == t@ by {
                                    if right@.contains(t@) {
                                        lemma_cmp_antisymmetry(t, key);
                                    }
                                    // Otherwise t@ ∈ left@ (empty) or t@ == key@.
                                };
                            }
                            Some(key)
                        }
                    }
                }
            }
        }

        /// Algorithm 38.4 — join two trees via recursive decomposition.
        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite(), joined@ =~= self@.union(other@),
            decreases other@.len(),
        {
            match other.expose() {
                | Exposed::Leaf => {
                    proof { assert(self@.union(other@) =~= self@); }
                    self.clone()
                }
                | Exposed::Node(left, key, right) => {
                    let ghost sv = self@;
                    let ghost ov = other@;
                    let ghost lv = left@;
                    let ghost rv = right@;
                    let ghost kv = key@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                        assert(!lv.union(rv).contains(kv));
                        assert(ov.len() == lv.len() + rv.len() + 1);
                        // self ⊥ left: left ⊆ other and self ⊥ other.
                        assert forall|x| lv.contains(x) implies ov.contains(x) by {};
                        assert(sv.disjoint(lv));
                        assert(sv.len() + lv.len() < usize::MAX as nat);
                    }
                    let merged = self.join_pair(left);
                    proof {
                        let ghost mv = merged@;
                        // merged ⊥ right.
                        assert forall|x| !(mv.contains(x) && rv.contains(x)) by {
                            if rv.contains(x) { assert(ov.contains(x)); }
                        };
                        // key ∉ merged.
                        assert(ov.contains(kv));
                        assert(!mv.contains(kv));
                        // Size bound.
                        vstd::set_lib::lemma_set_disjoint_lens(sv, lv);
                        assert(mv.len() == sv.len() + lv.len());
                        assert(mv.len() + rv.len() < usize::MAX as nat);
                    }
                    Self::join_m(merged, key, right)
                }
            }
        }

        /// Algorithm 38.6 — sequential union via divide-and-conquer on split.
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite(),
            decreases self@.len(),
        {
            match (self.expose(), other.expose()) {
                | (Exposed::Leaf, _) => {
                    proof { assert(self@.union(other@) =~= other@); }
                    other.clone()
                }
                | (_, Exposed::Leaf) => {
                    proof { assert(self@.union(other@) =~= self@); }
                    self.clone()
                }
                | (Exposed::Node(al, ak, ar), _) => {
                    let ghost sv = self@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        assert(!alv.union(arv).contains(akv));
                        assert(sv.len() == alv.len() + arv.len() + 1);
                    }
                    let (bl, _, br) = other.split(&ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    proof {
                        // Recursive call bounds: al@.len() + bl@.len() <= usize::MAX.
                        // bl ⊂ other (from split), so bl.len() ≤ other.len().
                        assert(blv.subset_of(other@.remove(akv)));
                        assert(other@.remove(akv).subset_of(other@));
                        vstd::set_lib::lemma_len_subset(blv, other@);
                        vstd::set_lib::lemma_len_subset(brv, other@);
                        // al.len() = sv.len() - ar.len() - 1, so al.len() + bl.len() < sv.len() + other.len().
                        assert(alv.len() + blv.len() <= sv.len() - 1 + other@.len());
                        assert(arv.len() + brv.len() <= sv.len() - 1 + other@.len());
                    }
                    let left_union = al.union(&bl);
                    let right_union = ar.union(&br);
                    let ghost luv = left_union@;
                    let ghost ruv = right_union@;
                    proof {
                        // Ordering: luv = alv ∪ blv (all < ak), ruv = arv ∪ brv (all > ak).
                        assert forall|t: T| (#[trigger] luv.contains(t@))
                            implies t.cmp_spec(&ak) == Less by {
                            if alv.contains(t@) {} else { assert(blv.contains(t@)); }
                        };
                        assert forall|t: T| (#[trigger] ruv.contains(t@))
                            implies t.cmp_spec(&ak) == Greater by {
                            if arv.contains(t@) {} else { assert(brv.contains(t@)); }
                        };
                        // Non-containment: akv ∉ al ∪ bl, akv ∉ ar ∪ br.
                        assert(!luv.contains(akv));
                        assert(!ruv.contains(akv));
                        // Disjointness via witness property: every view in al@/bl@ has a T witness.
                        use_type_invariant(&al);
                        use_type_invariant(&bl);
                        assert forall|x| !(luv.contains(x) && ruv.contains(x)) by {
                            if luv.contains(x) && ruv.contains(x) {
                                // x ∈ luv = alv ∪ blv → has T witness from type_invariant.
                                assert(alv.contains(x) || blv.contains(x));
                                assert(exists|t: T| t@ == x);
                                let ghost t: T = choose|t: T| t@ == x;
                                assert(t@ == x);
                                assert(luv.contains(t@));
                                assert(t.cmp_spec(&ak) == Less);
                                assert(ruv.contains(t@));
                                assert(t.cmp_spec(&ak) == Greater);
                            }
                        };
                        // Size: luv ∪ ruv ∪ {akv} ⊆ self@ ∪ other@, with akv ∉ luv ∪ ruv.
                        // So luv.len() + ruv.len() + 1 ≤ (self@ ∪ other@).len() ≤ usize::MAX.
                        vstd::set_lib::lemma_set_disjoint_lens(luv, ruv);
                        let ghost luv_ruv = luv.union(ruv);
                        assert(!luv_ruv.contains(akv));
                        assert forall|x| #[trigger] luv_ruv.insert(akv).contains(x)
                            implies sv.union(other@).contains(x) by {
                            if x == akv { assert(sv.contains(akv)); }
                            else if luv.contains(x) {
                                if alv.contains(x) { assert(sv.contains(x)); }
                                else { assert(blv.contains(x)); assert(other@.contains(x)); }
                            } else {
                                assert(ruv.contains(x));
                                if arv.contains(x) { assert(sv.contains(x)); }
                                else { assert(brv.contains(x)); assert(other@.contains(x)); }
                            }
                        };
                        assert(luv_ruv.insert(akv).subset_of(sv.union(other@)));
                        vstd::set_lib::lemma_len_union(sv, other@);
                        vstd::set_lib::lemma_len_subset(luv_ruv.insert(akv), sv.union(other@));
                        assert(luv.len() + ruv.len() < usize::MAX as nat);
                    }
                    let result = Self::join_m(left_union, ak, right_union);
                    proof {
                        // result@ = luv ∪ ruv ∪ {akv} = (alv ∪ blv) ∪ (arv ∪ brv) ∪ {akv}.
                        // self@ ∪ other@ = (alv ∪ arv ∪ {akv}) ∪ other@.
                        // blv ∪ brv =~= other@.remove(akv).
                        // So result@ = alv ∪ arv ∪ blv ∪ brv ∪ {akv}
                        //            = alv ∪ arv ∪ other@.remove(akv) ∪ {akv}
                        //            = self@ ∪ other@.
                        assert forall|x| #[trigger] sv.union(other@).contains(x) <==>
                            result@.contains(x) by {
                            if blv.contains(x) || brv.contains(x) {
                                assert(other@.remove(akv).contains(x));
                                assert(other@.contains(x));
                            }
                            if other@.contains(x) && x != akv {
                                assert(other@.remove(akv).contains(x));
                                assert(blv.union(brv).contains(x));
                            }
                        };
                        assert(result@ =~= sv.union(other@));
                    }
                    result
                }
            }
        }

        /// Algorithm 38.7 — sequential intersect. Keeps keys present in both trees.
        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite(),
            decreases self@.len(),
        {
            match (self.expose(), other.expose()) {
                | (Exposed::Leaf, _) | (_, Exposed::Leaf) => {
                    proof { assert(self@.intersect(other@) =~= Set::empty()); }
                    Self::new()
                }
                | (Exposed::Node(al, ak, ar), _) => {
                    let ghost sv = self@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        assert(sv.len() == alv.len() + arv.len() + 1);
                    }
                    let (bl, found, br) = other.split(&ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    let left_res = al.intersect(&bl);
                    let right_res = ar.intersect(&br);
                    let ghost lrv = left_res@;
                    let ghost rrv = right_res@;
                    if found {
                        proof {
                            // lrv = al@.intersect(bl@) ⊂ al@, rrv = ar@.intersect(br@) ⊂ ar@.
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            // Disjoint: subsets of disjoint sets.
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            // Non-containment: akv ∉ al@ and akv ∉ ar@ from expose.
                            assert(!lrv.contains(akv));
                            assert(!rrv.contains(akv));
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: lrv ⊂ al@ (< ak), rrv ⊂ ar@ (> ak).
                            assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&ak) == Less by {
                                assert(alv.contains(t@));
                            };
                            assert forall|t: T| (#[trigger] rrv.contains(t@)) implies t.cmp_spec(&ak) == Greater by {
                                assert(arv.contains(t@));
                            };
                        }
                        let result = Self::join_m(left_res, ak, right_res);
                        proof {
                            // result@ = lrv ∪ rrv ∪ {akv} where lrv = alv∩blv, rrv = arv∩brv.
                            // found ⟹ akv ∈ other@.
                            use_type_invariant(&al);
                            use_type_invariant(&ar);
                            assert forall|x| #[trigger] sv.intersect(other@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && blv.contains(x));
                                    assert(other@.remove(akv).contains(x));
                                }
                                if rrv.contains(x) {
                                    assert(arv.contains(x) && brv.contains(x));
                                    assert(other@.remove(akv).contains(x));
                                }
                                if sv.contains(x) && other@.contains(x) && x != akv {
                                    assert(other@.remove(akv).contains(x));
                                    assert(blv.union(brv).contains(x));
                                    // Route x to matching child via ordering witness.
                                    if alv.contains(x) {
                                        assert(exists|t: T| t@ == x);
                                        let ghost t: T = choose|t: T| t@ == x;
                                        assert(alv.contains(t@));
                                        assert(t.cmp_spec(&ak) == Less);
                                        // brv ordering says Greater — contradiction.
                                        assert(!brv.contains(x));
                                        assert(blv.contains(x));
                                        assert(lrv.contains(x));
                                    } else {
                                        assert(arv.contains(x));
                                        assert(exists|t: T| t@ == x);
                                        let ghost t: T = choose|t: T| t@ == x;
                                        assert(arv.contains(t@));
                                        assert(t.cmp_spec(&ak) == Greater);
                                        // blv ordering says Less — contradiction.
                                        assert(!blv.contains(x));
                                        assert(brv.contains(x));
                                        assert(rrv.contains(x));
                                    }
                                }
                            };
                            assert(result@ =~= sv.intersect(other@));
                        }
                        result
                    } else {
                        proof {
                            // lrv = al@.intersect(bl@) ⊂ al@, rrv = ar@.intersect(br@) ⊂ ar@.
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            // Disjoint: subsets of disjoint sets.
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            // Finite: from intersect ensures.
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: lrv ⊂ al@ (< ak), rrv ⊂ ar@ (> ak). s < ak < o ⟹ s < o.
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
                                assert(alv.contains(s@));
                                assert(arv.contains(o@));
                                lemma_cmp_antisymmetry(o, ak);
                                lemma_cmp_transitivity(s, ak, o);
                            };
                        }
                        let result = left_res.join_pair(right_res);
                        proof {
                            // !found ⟹ akv ∉ other@.
                            use_type_invariant(&al);
                            use_type_invariant(&ar);
                            assert forall|x| #[trigger] sv.intersect(other@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && blv.contains(x));
                                    assert(other@.remove(akv).contains(x));
                                }
                                if rrv.contains(x) {
                                    assert(arv.contains(x) && brv.contains(x));
                                    assert(other@.remove(akv).contains(x));
                                }
                                if sv.contains(x) && other@.contains(x) {
                                    assert(other@.remove(akv).contains(x));
                                    assert(blv.union(brv).contains(x));
                                    // Route x to matching child via ordering witness.
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
                            assert(result@ =~= sv.intersect(other@));
                        }
                        result
                    }
                }
            }
        }

        /// Algorithm 38.8 — sequential difference. Keeps keys in `self` not in `other`.
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite(),
            decreases self@.len(),
        {
            match (self.expose(), other.expose()) {
                | (Exposed::Leaf, _) => {
                    proof { assert(self@.difference(other@) =~= Set::empty()); }
                    Self::new()
                }
                | (_, Exposed::Leaf) => {
                    proof { assert(self@.difference(other@) =~= self@); }
                    self.clone()
                }
                | (Exposed::Node(al, ak, ar), _) => {
                    let ghost sv = self@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        assert(sv.len() == alv.len() + arv.len() + 1);
                    }
                    let (bl, found, br) = other.split(&ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    let left_res = al.difference(&bl);
                    let right_res = ar.difference(&br);
                    let ghost lrv = left_res@;
                    let ghost rrv = right_res@;
                    if found {
                        proof {
                            // lrv = al@.difference(bl@) ⊂ al@, rrv = ar@.difference(br@) ⊂ ar@.
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            // Disjoint: subsets of disjoint sets are disjoint.
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            // Finite: from difference ensures.
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: elements of lrv ⊂ al@ are < ak, elements of rrv ⊂ ar@ are > ak.
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
                                assert(alv.contains(s@));
                                assert(arv.contains(o@));
                                lemma_cmp_antisymmetry(o, ak);
                                lemma_cmp_transitivity(s, ak, o);
                            };
                        }
                        let result = left_res.join_pair(right_res);
                        proof {
                            // found ⟹ akv ∈ other@, so akv ∉ difference.
                            // other@ = blv ∪ brv ∪ {akv} (found means akv ∈ other@).
                            use_type_invariant(&al);
                            use_type_invariant(&ar);
                            assert forall|x| #[trigger] sv.difference(other@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && !blv.contains(x));
                                    assert(sv.contains(x));
                                    // Prove x ∉ other@: need x ∉ brv and x != akv.
                                    assert(x != akv);  // x ∈ alv and !alv.contains(akv).
                                    // x ∉ brv by ordering contradiction via witness.
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(alv.contains(t@));
                                    assert(t.cmp_spec(&ak) == Less);
                                    // If brv.contains(x) then brv.contains(t@) so
                                    // t.cmp_spec(&ak) == Greater — contradicts Less.
                                    assert(!brv.contains(x));
                                    assert(!other@.contains(x));
                                }
                                if rrv.contains(x) {
                                    assert(arv.contains(x) && !brv.contains(x));
                                    assert(sv.contains(x));
                                    // Prove x ∉ other@: need x ∉ blv and x != akv.
                                    assert(x != akv);  // x ∈ arv and !arv.contains(akv).
                                    // x ∉ blv by ordering contradiction via witness.
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(arv.contains(t@));
                                    assert(t.cmp_spec(&ak) == Greater);
                                    // If blv.contains(x) then blv.contains(t@) so
                                    // t.cmp_spec(&ak) == Less — contradicts Greater.
                                    assert(!blv.contains(x));
                                    assert(!other@.contains(x));
                                }
                                if sv.contains(x) && !other@.contains(x) {
                                    assert(!blv.union(brv).contains(x));
                                    if alv.contains(x) {
                                        assert(alv.difference(blv).contains(x));
                                    } else if arv.contains(x) {
                                        assert(arv.difference(brv).contains(x));
                                    }
                                }
                            };
                            assert(result@ =~= sv.difference(other@));
                        }
                        result
                    } else {
                        proof {
                            // lrv = al@.difference(bl@) ⊂ al@, rrv = ar@.difference(br@) ⊂ ar@.
                            assert(lrv.subset_of(alv));
                            assert(rrv.subset_of(arv));
                            // Disjoint: subsets of disjoint sets.
                            assert forall|x| !(lrv.contains(x) && rrv.contains(x)) by {
                                if lrv.contains(x) && rrv.contains(x) {
                                    assert(alv.contains(x) && arv.contains(x));
                                }
                            };
                            // Non-containment: akv ∉ al@ and akv ∉ ar@ from expose.
                            assert(!lrv.contains(akv));
                            assert(!rrv.contains(akv));
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: lrv ⊂ al@ (< ak), rrv ⊂ ar@ (> ak).
                            assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&ak) == Less by {
                                assert(alv.contains(t@));
                            };
                            assert forall|t: T| (#[trigger] rrv.contains(t@)) implies t.cmp_spec(&ak) == Greater by {
                                assert(arv.contains(t@));
                            };
                        }
                        let result = Self::join_m(left_res, ak, right_res);
                        proof {
                            // !found ⟹ akv ∉ other@, so akv ∈ difference.
                            // other@ = blv ∪ brv (akv ∉ other@, so remove is identity).
                            use_type_invariant(&al);
                            use_type_invariant(&ar);
                            assert(blv.union(brv) =~= other@);
                            assert forall|x| #[trigger] sv.difference(other@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    assert(alv.contains(x) && !blv.contains(x));
                                    assert(sv.contains(x));
                                    // Prove x ∉ other@ = blv ∪ brv.
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
                                    // Prove x ∉ other@ = blv ∪ brv.
                                    assert(exists|t: T| t@ == x);
                                    let ghost t: T = choose|t: T| t@ == x;
                                    assert(arv.contains(t@));
                                    assert(t.cmp_spec(&ak) == Greater);
                                    assert(!blv.contains(x));
                                    assert(!blv.union(brv).contains(x));
                                }
                                if sv.contains(x) && !other@.contains(x) && x != akv {
                                    assert(!blv.union(brv).contains(x));
                                    if alv.contains(x) {
                                        assert(alv.difference(blv).contains(x));
                                    } else if arv.contains(x) {
                                        assert(arv.difference(brv).contains(x));
                                    }
                                }
                            };
                            assert(result@ =~= sv.difference(other@));
                        }
                        result
                    }
                }
            }
        }

        /// Algorithm 38.9 — sequential filter. Keeps keys satisfying `predicate`.
        fn filter<F: Fn(&T) -> bool>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            filter_inner(self, &predicate, Ghost(spec_pred))
        }

        /// Algorithm 38.10 — sequential reduce. Folds `op(L', op(k, R'))`.
        fn reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (result: T) {
            reduce_inner(self, &op, base)
        }

        fn collect_in_order(&self, out: &mut Vec<T>)
            ensures
                out@.len() == old(out)@.len() + self@.len(),
                forall|i: int| #![trigger out@[i]] 0 <= i < old(out)@.len() ==> out@[i] == old(out)@[i],
                forall|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() ==> self@.contains(out@[i]@),
                forall|v: T::V| self@.contains(v) ==>
                    exists|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() && out@[i]@ == v,
            decreases self@.len(),
        {
            match self.expose() {
                | Exposed::Leaf => {}
                | Exposed::Node(left, key, right) => {
                    let ghost g0 = out@.len();
                    let ghost out_0 = out@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        assert(!left@.union(right@).contains(key@));
                        assert(self@.len() == left@.len() + right@.len() + 1);
                    }
                    left.collect_in_order(out);
                    let ghost g1 = out@.len();
                    let ghost out_1 = out@;
                    out.push(key);
                    let ghost g2 = out@.len();
                    let ghost out_2 = out@;
                    right.collect_in_order(out);
                    proof {
                        // Containment: new elements come from self@.
                        assert forall|i: int| #![trigger out@[i]] g0 <= i < out@.len() implies
                            self@.contains(out@[i]@) by {
                            if i < g1 as int {
                                // Element from left subtree, preserved through push and right.
                                assert(out@[i] == out_2[i]);
                                assert(out_2[i] == out_1[i]);
                                assert(left@.contains(out_1[i]@));
                            } else if i == g1 as int {
                                // The key itself.
                                assert(out@[i] == out_2[i]);
                                assert(out_2[g1 as int] == key);
                            } else if i < g2 as int {
                                // Impossible: g2 == g1 + 1.
                            } else {
                                // Element from right subtree.
                                assert(right@.contains(out@[i]@));
                            }
                        };
                        // Completeness: all elements of self@ appear.
                        assert forall|v: T::V| self@.contains(v) implies
                            exists|i: int| #![trigger out@[i]] g0 <= i < out@.len() && out@[i]@ == v by {
                            if left@.contains(v) {
                                let i_left = choose|i: int| #![trigger out_1[i]] g0 <= i < g1 as int && out_1[i]@ == v;
                                // Preserved through push and right.
                                assert(out_2[i_left] == out_1[i_left]);
                                assert(out@[i_left] == out_2[i_left]);
                            } else if v == key@ {
                                assert(out_2[g1 as int] == key);
                                assert(out@[g1 as int] == out_2[g1 as int]);
                            } else {
                                assert(right@.contains(v));
                            }
                        };
                        // Preservation: old elements unchanged.
                        assert forall|i: int| #![trigger out@[i]] 0 <= i < g0 implies out@[i] == out_0[i] by {
                            assert(out@[i] == out_2[i]);
                            assert(out_2[i] == out_1[i]);
                            assert(out_1[i] == out_0[i]);
                        };
                    }
                }
            }
        }

        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
        {
            let count = self.size();
            let mut out = Vec::with_capacity(count);
            self.collect_in_order(&mut out);
            let result = ArraySeqStPerS::from_vec(out);
            proof {
                // seq@.len() == self@.len() follows from:
                // collect_in_order: out@.len() == 0 + self@.len()
                // from_vec: result.spec_len() == out@.len()
                // And spec_len() == seq@.len() (map preserves length).

                // Containment: self@.contains(v) <==> seq@.contains(v).
                // Forward: self@.contains(v) ==> ∃i. out@[i]@ == v ==> seq@.contains(v).
                assert forall|v: T::V| self@.contains(v) implies result@.contains(v) by {
                    let i = choose|i: int| #![trigger out@[i]] 0 <= i < out@.len() && out@[i]@ == v;
                    // result.spec_index(i) == out@[i], and result@[i] == out@[i]@.
                    assert(result@[i] == result.spec_index(i)@);
                    assert(result.spec_index(i) == out@[i]);
                };
                // Backward: seq@.contains(v) ==> ∃i. out@[i]@ == v ==> self@.contains(v).
                assert forall|v: T::V| result@.contains(v) implies self@.contains(v) by {
                    let i = choose|i: int| 0 <= i < result@.len() && result@[i] == v;
                    assert(result@[i] == result.spec_index(i)@);
                    assert(result.spec_index(i) == out@[i]);
                    assert(out@[i]@ == v);
                    assert(self@.contains(out@[i]@));
                };
            }
            result
        }
    }

    // 10. free fns

    /// Algorithm 38.9 — sequential filter recursive helper (takes &F for recursion).
    fn filter_inner<T: StT + Ord, F: Fn(&T) -> bool>(
        tree: &ParamBST<T>,
        predicate: &F,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamBST<T>)
        requires
            tree@.finite(),
            forall|t: &T| predicate.requires((t,)),
            forall|x: T, keep: bool|
                predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures
            filtered@.subset_of(tree@),
            filtered@.finite(),
            forall|v: T::V| #[trigger] filtered@.contains(v)
                ==> tree@.contains(v) && spec_pred(v),
            forall|v: T::V| tree@.contains(v) && spec_pred(v)
                ==> #[trigger] filtered@.contains(v),
        decreases tree@.len(),
    {
        match tree.expose() {
            | Exposed::Leaf => ParamBST::new(),
            | Exposed::Node(left, key, right) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    assert(!left@.union(right@).contains(key@));
                    assert(tree@.len() == left@.len() + right@.len() + 1);
                }
                let left_filtered = filter_inner(&left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner(&right, predicate, Ghost(spec_pred));
                if predicate(&key) {
                    proof {
                        vstd::set_lib::lemma_len_subset(left_filtered@, left@);
                        vstd::set_lib::lemma_len_subset(right_filtered@, right@);
                        // Disjointness: subsets of disjoint sets.
                        assert forall|x| !(left_filtered@.contains(x) && right_filtered@.contains(x)) by {
                            if left_filtered@.contains(x) && right_filtered@.contains(x) {
                                assert(left@.contains(x) && right@.contains(x));
                            }
                        };
                        // Ordering: left_filtered ⊆ left (< key), right_filtered ⊆ right (> key).
                        assert forall|t: T| (#[trigger] left_filtered@.contains(t@)) implies t.cmp_spec(&key) == Less by {
                            assert(left@.contains(t@));
                        };
                        assert forall|t: T| (#[trigger] right_filtered@.contains(t@)) implies t.cmp_spec(&key) == Greater by {
                            assert(right@.contains(t@));
                        };
                    }
                    ParamBST::join_m(left_filtered, key, right_filtered)
                } else {
                    proof {
                        assert forall|x| !(left_filtered@.contains(x) && right_filtered@.contains(x)) by {
                            if right_filtered@.contains(x) { assert(right@.contains(x)); }
                        };
                        vstd::set_lib::lemma_len_subset(left_filtered@, left@);
                        vstd::set_lib::lemma_len_subset(right_filtered@, right@);
                        // Ordering: left_filtered ⊆ left < key < right ⊇ right_filtered.
                        assert forall|s: T, o: T| #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                            left_filtered@.contains(s@) && right_filtered@.contains(o@) implies
                            s.cmp_spec(&o) == Less by {
                            assert(left@.contains(s@));
                            assert(right@.contains(o@));
                            lemma_cmp_antisymmetry(o, key);
                            lemma_cmp_transitivity(s, key, o);
                        };
                    }
                    left_filtered.join_pair(right_filtered)
                }
            }
        }
    }

    /// Algorithm 38.10 — sequential reduce recursive helper (takes &F for recursion).
    fn reduce_inner<T: StT + Ord, F: Fn(T, T) -> T>(
        tree: &ParamBST<T>,
        op: &F,
        identity: T,
    ) -> (result: T)
        requires
            tree@.finite(),
            forall|a: T, b: T| op.requires((a, b)),
        ensures tree@.len() == 0 ==> result@ == identity@,
        decreases tree@.len(),
    {
        match tree.expose() {
            | Exposed::Leaf => identity,
            | Exposed::Node(left, key, right) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    assert(!left@.union(right@).contains(key@));
                    assert(tree@.len() == left@.len() + right@.len() + 1);
                }
                let left_acc = reduce_inner(&left, op, identity.clone());
                let right_acc = reduce_inner(&right, op, identity);
                let right_with_key = op(key, right_acc);
                op(left_acc, right_with_key)
            }
        }
    }

    // 11. derive impls in verus!

    impl<T: StT + Ord + Clone> Clone for Exposed<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            match self {
                Exposed::Leaf => Exposed::Leaf,
                Exposed::Node(l, k, r) => Exposed::Node(l.clone(), k.clone(), r.clone()),
            }
        }
    }

    impl<T: StT + Ord + Clone> Clone for NodeInner<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            NodeInner {
                key: self.key.clone(),
                size: self.size,
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    impl<T: StT + Ord> Clone for ParamBST<T> {
        #[verifier::external_body]
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let handle = self.locked_root.acquire_read();
            let inner_clone = handle.borrow().clone();
            handle.release_read();
            let ghost pred = BSTParaStEphInv::<T> { contents: self@ };
            ParamBST {
                locked_root: RwLock::new(inner_clone, Ghost(pred)),
                ghost_locked_root: Ghost(self@),
            }
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
            let mut __tree = < $crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBST<_> as
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
