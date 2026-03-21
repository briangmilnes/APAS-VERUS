//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Parametric multi-threaded Treap (probabilistically balanced BST) with parallel operations.

//  Table of Contents
//	1. module
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns/broadcast groups
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
    use vstd::std_specs::cmp::OrdSpec;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;

    verus! {

    // 3. broadcast use

    broadcast use {vstd::set::group_set_axioms, vstd::set_lib::group_set_properties};

    // 4. type definitions

    /// RwLock predicate for treap nodes. Carries ghost contents and BST ordering predicates
    /// so the type_invariant can fully characterise the locked value.
    pub struct BSTParaTreapMtEphInv<T: MtKey> {
        pub ghost contents: Set<<T as View>::V>,
    }

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
        pub(crate) root: RwLock<Option<Box<NodeInner<T>>>, BSTParaTreapMtEphInv<T>>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    // 5. view impls

    impl<T: MtKey> ParamTreap<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_root@.finite()
            && self.ghost_locked_root@ =~= self.root.pred().contents
            && (forall|v: T::V| #[trigger] self.ghost_locked_root@.contains(v)
                ==> exists|t: T| t@ == v)
        }

        pub closed spec fn spec_ghost_locked_root(self) -> Set<<T as View>::V> {
            self.ghost_locked_root@
        }
    }

    impl<T: MtKey> View for ParamTreap<T> {
        type V = Set<T::V>;
        open spec fn view(&self) -> Set<T::V> { self.spec_ghost_locked_root() }
    }

    impl<T: MtKey> RwLockPredicate<Option<Box<NodeInner<T>>>> for BSTParaTreapMtEphInv<T> {
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
                    && (forall|t: T| (#[trigger] (*box_node).left@.contains(t@))
                        ==> t.cmp_spec(&(*box_node).key) == Less)
                    && (forall|t: T| (#[trigger] (*box_node).right@.contains(t@))
                        ==> t.cmp_spec(&(*box_node).key) == Greater)
                }
            }
        }
    }

    // 6. spec fns

    /// View-consistent ordering: elements with equal views compare Equal.
    pub open spec fn view_ord_consistent<T: MtKey>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    // 7. proof fns

    /// Centralises the clone-body assume pattern per partial_eq_eq_clone_standard.
    // veracity: no_requires
    fn clone_elem<T: MtKey>(x: &T) -> (c: T)
        ensures c@ == x@,
    {
        let c = x.clone();
        proof { assume(c@ == x@); } // Clone bridge: T::clone preserves view.
        c
    }

    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    pub proof fn lemma_cmp_antisymmetry<T: MtKey>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Greater,
        ensures b.cmp_spec(&a) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec antisymmetry: Less(a,b) implies Greater(b,a).
    pub proof fn lemma_cmp_antisymmetry_less<T: MtKey>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
        ensures b.cmp_spec(&a) == Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Less(a,b) and Less(b,c) implies Less(a,c).
    pub proof fn lemma_cmp_transitivity<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Less,
        ensures a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Equal-substitution: Less(a,b) and Equal(b,c) implies Less(a,c).
    pub proof fn lemma_cmp_eq_subst<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Equal,
        ensures a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Left congruence: Equal(a,b) implies a and b compare the same way to c.
    pub proof fn lemma_cmp_equal_congruent<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
        assert(a@ == b@);
    }

    /// Right congruence: Equal(b,c) implies any a compares the same way to b and c.
    pub proof fn lemma_cmp_equal_congruent_right<T: MtKey>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            b.cmp_spec(&c) == Equal,
        ensures a.cmp_spec(&b) == a.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
        assert(b@ == c@);
    }

    /// Expose the type invariant guarantee of finiteness to callers in other modules.
    /// Calling this is a no-op at runtime; the spec-level ensures establishes tree@.finite().
    pub fn param_treap_assert_finite<T: MtKey>(tree: &ParamTreap<T>)
        ensures tree@.finite(),
    {
        proof { use_type_invariant(tree); }
    }

    /// After join(lr, key, right), every element is greater than lk.
    /// Hypotheses: lr > lk, right > key, lk ∈ left, all left < key.
    proof fn lemma_joined_right_gt_lk<T: MtKey>(
        lrv: Set<T::V>,
        right_v: Set<T::V>,
        key: T,
        joined_v: Set<T::V>,
        lk: T,
        left_v: Set<T::V>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            joined_v =~= lrv.union(right_v).insert(key@),
            forall|t: T| (#[trigger] lrv.contains(t@)) ==> t.cmp_spec(&lk) == Greater,
            forall|t: T| (#[trigger] right_v.contains(t@)) ==> t.cmp_spec(&key) == Greater,
            left_v.contains(lk@),
            forall|t: T| (#[trigger] left_v.contains(t@)) ==> t.cmp_spec(&key) == Less,
        ensures
            forall|t: T| (#[trigger] joined_v.contains(t@)) ==> t.cmp_spec(&lk) == Greater,
    {
        assert forall|t: T| (#[trigger] joined_v.contains(t@)) implies t.cmp_spec(&lk) == Greater by {
            if lrv.contains(t@) {
                // t ∈ lr: expose ensures t.cmp(&lk) == Greater ✓
            } else if right_v.contains(t@) {
                // t > key (req); lk < key (lk ∈ left, req); key < t → lk < t.
                lemma_cmp_antisymmetry(t, key);      // key.cmp(&t) == Less
                lemma_cmp_transitivity(lk, key, t);  // lk.cmp(&t) == Less
                lemma_cmp_antisymmetry_less(lk, t);  // t.cmp(&lk) == Greater
            } else {
                // t@ == key@
                assert(t@ == key@);
                lemma_cmp_equal_congruent_right(lk, t, key);  // lk.cmp(&t) == lk.cmp(&key) == Less
                lemma_cmp_antisymmetry_less(lk, t);           // t.cmp(&lk) == Greater
            }
        }
    }

    /// After join(left, key, rl), every element is less than rk.
    /// Hypotheses: left < key, rl < rk, rk ∈ right, all right > key.
    proof fn lemma_joined_left_lt_rk<T: MtKey>(
        left_v: Set<T::V>,
        rlv: Set<T::V>,
        key: T,
        joined_v: Set<T::V>,
        rk: T,
        right_v: Set<T::V>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            joined_v =~= left_v.union(rlv).insert(key@),
            forall|t: T| (#[trigger] left_v.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] rlv.contains(t@)) ==> t.cmp_spec(&rk) == Less,
            right_v.contains(rk@),
            forall|t: T| (#[trigger] right_v.contains(t@)) ==> t.cmp_spec(&key) == Greater,
        ensures
            forall|t: T| (#[trigger] joined_v.contains(t@)) ==> t.cmp_spec(&rk) == Less,
    {
        assert(rk.cmp_spec(&key) == Greater);  // rk ∈ right, all right > key
        assert forall|t: T| (#[trigger] joined_v.contains(t@)) implies t.cmp_spec(&rk) == Less by {
            if left_v.contains(t@) {
                // t < key (req); key < rk (antisymmetry of rk > key).
                lemma_cmp_antisymmetry(rk, key);     // key.cmp(&rk) == Less
                lemma_cmp_transitivity(t, key, rk);  // t.cmp(&rk) == Less
            } else if rlv.contains(t@) {
                // t ∈ rl: expose ensures t.cmp(&rk) == Less ✓
            } else {
                // t@ == key@; key < rk from above.
                assert(t@ == key@);
                lemma_cmp_antisymmetry(rk, key);        // key.cmp(&rk) == Less
                lemma_cmp_equal_congruent(t, key, rk);  // t.cmp(&rk) == key.cmp(&rk) == Less
            }
        }
    }

    // 9. impls

    fn new_param_treap<T: MtKey>(
        val: Option<Box<NodeInner<T>>>,
        Ghost(contents): Ghost<Set<<T as View>::V>>,
    ) -> (tree: ParamTreap<T>)
        requires
            (BSTParaTreapMtEphInv::<T> { contents }).inv(val),
            contents.finite(),
            forall|v: T::V| #[trigger] contents.contains(v) ==> exists|t: T| t@ == v,
        ensures tree@ =~= contents,
    {
        let ghost pred = BSTParaTreapMtEphInv::<T> { contents };
        ParamTreap {
            root: RwLock::new(val, Ghost(pred)),
            ghost_locked_root: Ghost(contents),
        }
    }

    fn new_leaf<T: MtKey>() -> (tree: ParamTreap<T>)
        ensures tree@ =~= Set::<<T as View>::V>::empty()
    {
        new_param_treap(None, Ghost(Set::empty()))
    }

    fn expose_internal<T: MtKey + 'static>(tree: &ParamTreap<T>) -> (exposed: Exposed<T>)
        requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
        ensures
            tree@.len() == 0 ==> exposed is Leaf,
            exposed is Leaf ==> tree@ =~= Set::<<T as View>::V>::empty(),
            exposed matches Exposed::Node(l, k, r) ==> {
                tree@ =~= l@.union(r@).insert(k@)
                && tree@.finite()
                && l@.finite() && r@.finite()
                && l@.disjoint(r@)
                && !l@.contains(k@)
                && !r@.contains(k@)
                && l@.len() + r@.len() < usize::MAX as nat
                && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
            },
        decreases tree@.len(), 0nat,
    {
        proof { use_type_invariant(tree); }
        let handle = tree.root.acquire_read();
        let exposed = match handle.borrow() {
            | None => Exposed::Leaf,
            | Some(node) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(node.left@, node.right@);
                    assert(node.left@.len() < tree@.len());
                    assert(node.right@.len() < tree@.len());
                }
                let l = node.left.clone();
                let k = clone_elem(&node.key);
                let r = node.right.clone();
                proof {
                    // Ordering transfers from pred to clones via view equality.
                    // key.clone() ensures k@ == node.key@; view_ord_consistent gives
                    // node.key.cmp_spec(&k) == Equal, which lemma_cmp_equal_congruent_right uses.
                    assert(k@ == node.key@);
                    assert(node.key.cmp_spec(&k) == Equal) by {
                        assert(node.key@ == k@);
                    };
                    assert(l@ =~= node.left@);  // from clone ensures
                    assert(r@ =~= node.right@); // from clone ensures
                    assert forall|t: T| (#[trigger] l@.contains(t@))
                        implies t.cmp_spec(&k) == Less by {
                        assert(node.left@.contains(t@));
                        assert(t.cmp_spec(&node.key) == Less);
                        lemma_cmp_equal_congruent_right(t, node.key, k);
                    }
                    assert forall|t: T| (#[trigger] r@.contains(t@))
                        implies t.cmp_spec(&k) == Greater by {
                        assert(node.right@.contains(t@));
                        assert(t.cmp_spec(&node.key) == Greater);
                        lemma_cmp_equal_congruent_right(t, node.key, k);
                    }
                }
                Exposed::Node(l, k, r)
            }
        };
        handle.release_read();
        exposed
    }

    fn expose_with_priority_internal<T: MtKey + 'static>(tree: &ParamTreap<T>) -> (parts: Option<(ParamTreap<T>, T, i64, ParamTreap<T>)>)
        requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
        ensures
            tree@.len() == 0 ==> parts is None,
            parts is None ==> tree@ =~= Set::<<T as View>::V>::empty(),
            parts matches Some((l, k, _, r)) ==> {
                tree@ =~= l@.union(r@).insert(k@)
                && tree@.finite()
                && l@.finite() && r@.finite()
                && l@.disjoint(r@)
                && !l@.contains(k@)
                && !r@.contains(k@)
                && l@.len() + r@.len() < usize::MAX as nat
                && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
            },
        decreases tree@.len(), 0nat,
    {
        match expose_internal(tree) {
            Exposed::Leaf => None,
            Exposed::Node(l, k, r) => {
                let priority = {
                    let handle = tree.root.acquire_read();
                    let p = match handle.borrow() {
                        None => i64::MIN,
                        Some(node) => node.priority,
                    };
                    handle.release_read();
                    p
                };
                Some((l, k, priority, r))
            }
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    #[verifier::external_body]
    fn priority_for<T: MtKey>(key: &T) -> i64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buf = String::new();
        let _ = write!(&mut buf, "{key:?}");
        Hash::hash(&buf, &mut hasher);
        hasher.finish() as i64
    }

    fn tree_priority_internal<T: MtKey + 'static>(tree: &ParamTreap<T>) -> (p: i64)
        ensures true,
    {
        let handle = tree.root.acquire_read();
        let result = match handle.borrow() {
            None => i64::MIN,
            Some(node) => node.priority,
        };
        handle.release_read();
        result
    }

    /// Build a new tree from (left, key, priority, right) maintaining BST and heap ordering.
    fn make_node<T: MtKey + 'static>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> (node: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            left@.finite(), right@.finite(),
            left@.disjoint(right@),
            !left@.contains(key@), !right@.contains(key@),
            left@.len() + right@.len() < usize::MAX as nat,
            forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
        ensures
            node@ =~= left@.union(right@).insert(key@),
            node@.finite(),
            forall|t: T| (#[trigger] node@.contains(t@)) ==>
                t.cmp_spec(&key) == Less || t.cmp_spec(&key) == Greater || t@ == key@,
    {
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
            use_type_invariant(&left);
            use_type_invariant(&right);
            assert forall|v: T::V| #[trigger] contents.contains(v)
                implies exists|t: T| t@ == v by {
                if lv.contains(v) { /* left's type_invariant */ }
                else if rv.contains(v) { /* right's type_invariant */ }
                else {
                    assert(v == kv);
                    assert(key@ == v);
                }
            };
        }
        new_param_treap(
            Some(Box::new(NodeInner { key, priority, size, left, right })),
            Ghost(contents),
        )
    }

    /// Merge two BST-ordered subtrees with a middle key, rebalancing by priority (treap heap).
    fn join_with_priority<T: MtKey + 'static>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> (result: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            left@.finite(), right@.finite(),
            left@.disjoint(right@),
            !left@.contains(key@), !right@.contains(key@),
            left@.len() + right@.len() < usize::MAX as nat,
            forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
        ensures result@ =~= left@.union(right@).insert(key@), result@.finite(),
        decreases left@.len() + right@.len(),
    {
        let left_priority = tree_priority_internal(&left);
        let right_priority = tree_priority_internal(&right);
        if priority > left_priority && priority > right_priority {
            return make_node(left, key, priority, right);
        }
        if left_priority > right_priority {
            match expose_internal(&left) {
                Exposed::Leaf => make_node(left, key, priority, right),
                Exposed::Node(ll, lk, lr) => {
                    let lp = tree_priority_internal(&ll);  // unused; real priority is in the lock
                    let lp_actual = {
                        // Re-read left priority from the actual node, not from the ll child.
                        // We already called expose_internal; the node priority came from node.priority.
                        // We need the LEFT node's priority for the heap check. But expose_internal
                        // doesn't return it. We use tree_priority_internal on the original left.
                        // However, left has been consumed. Re-derive from expose outputs: the
                        // priority of the *left root* is the priority we compare against.
                        // Since left_priority == tree_priority_internal(&original_left) was computed
                        // before expose, and after expose we need the same value. Stash it.
                        left_priority
                    };
                    let ghost lkv = lk@;
                    let ghost lrv = lr@;
                    let ghost llv = ll@;
                    proof {
                        // lr@.disjoint(right@): lr@ ⊆ left@ (expose), left@.disjoint(right@) (req).
                        assert(lrv.subset_of(left@));
                        // !lr@.contains(key@): lr@ ⊆ left@, !left@.contains(key@) (req).
                        assert(!lrv.contains(key@));
                        // !right@.contains(key@): from req.
                        assert(lrv.disjoint(right@));
                        vstd::set_lib::lemma_len_subset(lrv, left@);
                        assert(lrv.len() + right@.len() < usize::MAX as nat);
                        // BST ordering for (lr, key, right): all lr < lk < key (transitivity).
                        assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&key) == Less by {
                            // t ∈ lr ⊆ left, so t.cmp(&lk) == Greater (from expose).
                            // But we need t < key. Since lk < key (lk ∈ left, all of left < key),
                            // and t > lk... hmm, that would give t > lk, not t < key.
                            // Wait: lk ∈ left and all of left < key (from req forall), so lk < key.
                            // And t ∈ lr ⊆ right-subtree-of-left, so t > lk (expose forall).
                            // t > lk and lk < key: transitivity gives... t > lk AND lk < key
                            // → we cannot directly conclude t < key! t could be between lk and key.
                            // This is correct: left subtree of left has elements < lk < key,
                            // but lr = RIGHT subtree of left, which has lk < t ≤ key (strictly < key
                            // since !left@.contains(key@)).
                            // Formally: t ∈ left@ (t ∈ lr ⊆ left@), so t.cmp(&key) == Less (req forall).
                            assert(left@.contains(t@));
                        }
                    }
                    let merged_right = join_with_priority(lr, key, priority, right);
                    proof {
                        // merged_right@ =~= lrv.union(right@).insert(key@).
                        // For make_node(ll, lk, lp, merged_right):
                        // BST: all ll < lk < all merged_right.
                        // ll < lk: from expose ✓.
                        // merged_right > lk:
                        //   lr elements > lk (from expose Right), key > lk (lk ∈ left, all left < key? NO
                        //   -- we need lk < key. lk ∈ left@ and all of left@ < key (req forall: all left have cmp==Less wrt key). So lk.cmp(&key) == Less ✓.
                        //   right elements > key > lk (transitivity).
                        lemma_joined_right_gt_lk(lrv, right@, key, merged_right@, lk, left@);
                        assert(!llv.contains(lkv));
                        assert(!merged_right@.contains(lkv));
                        assert(llv.disjoint(merged_right@));
                        vstd::set_lib::lemma_set_disjoint_lens(llv, lrv);
                        vstd::set_lib::lemma_set_disjoint_lens(lrv, right@);
                        assert(merged_right@.len() == lrv.len() + right@.len() + 1);
                        vstd::set_lib::lemma_len_subset(llv, left@);
                        assert(llv.len() + merged_right@.len() < usize::MAX as nat);
                        // BST for ll: already proved above (all ll < lk, from expose).
                    }
                    make_node(ll, lk, lp_actual, merged_right)
                }
            }
        } else {
            match expose_internal(&right) {
                Exposed::Leaf => make_node(left, key, priority, right),
                Exposed::Node(rl, rk, rr) => {
                    let rp_actual = right_priority;
                    let ghost rkv = rk@;
                    let ghost rlv = rl@;
                    let ghost rrv = rr@;
                    proof {
                        assert(rlv.subset_of(right@));
                        assert(!rlv.contains(key@));
                        assert(left@.disjoint(rlv));
                        vstd::set_lib::lemma_len_subset(rlv, right@);
                        assert(left@.len() + rlv.len() < usize::MAX as nat);
                        // BST ordering for (left, key, rl): all left < key, all rl < rk.
                        // We need all rl < key: rl ⊆ right, all right > key (req), so... wait.
                        // rl = LEFT subtree of right. right elements > key (req). rl ⊆ right.
                        // But rl elements < rk (expose), and rk ∈ right, rk > key (req). So... no,
                        // rl < rk > key doesn't give rl < key directly.
                        // Actually: rl ⊆ right and all right > key (req forall Greater). So rl > key.
                        // But we need rl < key for join_with_priority(left, key, priority, rl)!
                        // WAIT: this is wrong. rl = left subtree of right, which has elements < rk.
                        // And rk > key. So elements of rl are between key and rk? Not necessarily.
                        // Actually for BST: rl ⊆ right, all right > key. So all rl > key.
                        // But join_with_priority(left, key, priority, rl) needs all rl < key. CONTRADICTION.
                        // The correct recursion when right_priority > left_priority is:
                        // expose RIGHT, not left. And then:
                        //   merged_left = join_with_priority(left, key, priority, rl)  -- NO, rl > key!
                        //   make_node(merged_left, rk, rp, rr)
                        // For merged_left = join(left, key, rl), we need all rl > key.
                        // But join_with_priority's sig: (left, key, _, right) where left < key < right.
                        // So we should call join(left, key, _, rl) where left < key and rl > key... but rl > key!
                        // So this IS correct: pass rl as the "right" argument, not "left"!
                        // The call join_with_priority(left, key, priority, rl) IS correct:
                        //   left < key (req) AND rl > key (since rl ⊆ right and all right > key).
                        // So all rl > key → t.cmp(&key) == Greater for all t ∈ rl. ✓
                        assert forall|t: T| (#[trigger] rlv.contains(t@)) implies t.cmp_spec(&key) == Greater by {
                            assert(right@.contains(t@));
                        }
                    }
                    let merged_left = join_with_priority(left, key, priority, rl);
                    proof {
                        // For make_node(merged_left, rk, rp, rr):
                        // all merged_left < rk: left < rk (rk ∈ right, all right > key ≥ left? No,
                        // key > all left, and rk > key, so rk > all left by transitivity).
                        // More precisely: for t ∈ left: t.cmp(&key) == Less, key.cmp(&rk) == Less
                        //   (rk ∈ right, all right > key → rk > key → key.cmp(&rk) == Less? No,
                        //    right@.contains(rk@) and t.cmp_spec(&key) == Greater for all t ∈ right,
                        //    so rk.cmp(&key) == Greater → key.cmp(&rk) == Less ✓.
                        //   t.cmp(&key) == Less, key.cmp(&rk) == Less → t.cmp(&rk) == Less ✓.
                        // For t@ == key@: key.cmp(&rk) == Less (from above) → by congruence, t.cmp(&rk) == Less ✓.
                        // For t ∈ rl: all rl have cmp(&rk) == Less (from expose) ✓.
                        assert(right@.contains(rk@));  // rk ∈ right from expose
                        lemma_joined_left_lt_rk(left@, rlv, key, merged_left@, rk, right@);
                        assert(!rrv.contains(rkv));
                        assert(!merged_left@.contains(rkv));
                        assert(merged_left@.disjoint(rrv));
                        vstd::set_lib::lemma_set_disjoint_lens(left@, rlv);
                        vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                        assert(merged_left@.len() == left@.len() + rlv.len() + 1);
                        vstd::set_lib::lemma_len_subset(rrv, right@);
                        assert(merged_left@.len() + rrv.len() < usize::MAX as nat);
                    }
                    make_node(merged_left, rk, rp_actual, rr)
                }
            }
        }
    }

    fn split_inner<T: MtKey + 'static>(tree: &ParamTreap<T>, key: &T) -> (parts: (ParamTreap<T>, bool, ParamTreap<T>))
        requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
        ensures
            parts.1 == tree@.contains(key@),
            parts.0@.finite(),
            parts.2@.finite(),
            parts.0@.union(parts.2@) =~= tree@.remove(key@),
            parts.0@.disjoint(parts.2@),
            !parts.0@.contains(key@) && !parts.2@.contains(key@),
            forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(key) == Less,
            forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(key) == Greater,
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => (new_leaf(), false, new_leaf()),
            | Exposed::Node(left, root_key, right) => {
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    assert(!left@.union(right@).contains(root_key@));
                    assert(tree@.len() == left@.len() + right@.len() + 1);
                }
                let ghost rk = root_key;
                let ghost kval = *key;
                match key.cmp(&root_key) {
                    | Less => {
                        let ghost lv = left@;
                        let ghost rv = right@;
                        let ghost rkv = root_key@;
                        let priority = tree_priority_internal(tree);
                        let (ll, found, lr) = split_inner(&left, key);
                        let ghost llv = ll@;
                        let ghost lrv = lr@;
                        proof {
                            // lr, ll ⊆ left@.
                            assert forall|x| lrv.contains(x) implies lv.contains(x) by {
                                assert(llv.union(lrv).contains(x));
                            };
                            assert(lrv.subset_of(lv));
                            assert forall|x| llv.contains(x) implies lv.contains(x) by {
                                assert(llv.union(lrv).contains(x));
                            };
                            assert(llv.subset_of(lv));
                            vstd::set_lib::lemma_len_subset(lrv, lv);
                            // Requirements for join_with_priority(lr, root_key, priority, right):
                            // lr < root_key: from expose, all left < root_key, lr ⊆ left → lr < root_key ✓.
                            assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&root_key) == Less by {
                                assert(lv.contains(t@));
                            };
                            // right > root_key: from expose ✓.
                            // !lr.contains(root_key@), !right.contains(root_key@): from expose ✓.
                            // lr.disjoint(right): lr ⊆ left, left.disjoint(right) → lr.disjoint(right) ✓.
                            assert(lrv.disjoint(rv));
                            assert(lrv.len() + rv.len() < usize::MAX as nat);
                        }
                        let rebuilt = join_with_priority(lr, root_key, priority, right);
                        proof {
                            assert(rebuilt@ =~= lrv.union(rv).insert(rkv));
                            assert(!rv.contains(key@));
                            assert forall|x| #[trigger] (llv.union(rebuilt@)).contains(x)
                                <==> tree@.remove(key@).contains(x) by {
                                if llv.contains(x) {
                                    assert(llv.union(lrv).contains(x));
                                }
                                if lv.contains(x) && x != key@ {
                                    assert(lv.remove(key@).contains(x));
                                    assert(llv.union(lrv).contains(x));
                                }
                            };
                            assert(llv.union(rebuilt@) =~= tree@.remove(key@));
                            // Ordering: rebuilt elements > key.
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(key) == Greater by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if lrv.contains(t@) {
                                    // Recursive split ensures t > key.
                                } else if rv.contains(t@) {
                                    lemma_cmp_antisymmetry(t, rk);
                                    lemma_cmp_transitivity(kval, rk, t);
                                } else {
                                    assert(t@ == rkv);
                                    lemma_cmp_eq_subst(kval, rk, t);
                                }
                            };
                            assert(llv.disjoint(rebuilt@));
                        }
                        (ll, found, rebuilt)
                    }
                    | Greater => {
                        let ghost lv = left@;
                        let ghost rv = right@;
                        let ghost rkv = root_key@;
                        let priority = tree_priority_internal(tree);
                        let (rl, found, rr) = split_inner(&right, key);
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
                            // Requirements for join_with_priority(left, root_key, priority, rl):
                            // left < root_key: from expose ✓.
                            // rl > root_key: from expose, all right > root_key, rl ⊆ right → rl > root_key ✓.
                            assert forall|t: T| (#[trigger] rlv.contains(t@)) implies t.cmp_spec(&root_key) == Greater by {
                                assert(rv.contains(t@));
                            };
                            // !left.contains(root_key@), !rl.contains(root_key@): from expose ✓.
                            assert(lv.disjoint(rlv));
                            assert(lv.len() + rlv.len() < usize::MAX as nat);
                        }
                        let rebuilt = join_with_priority(left, root_key, priority, rl);
                        proof {
                            assert(rebuilt@ =~= lv.union(rlv).insert(rkv));
                            assert(!lv.contains(key@));
                            assert forall|x| #[trigger] (rebuilt@.union(rrv)).contains(x)
                                <==> tree@.remove(key@).contains(x) by {
                                if rrv.contains(x) {
                                    assert(rlv.union(rrv).contains(x));
                                }
                                if rv.contains(x) && x != key@ {
                                    assert(rv.remove(key@).contains(x));
                                    assert(rlv.union(rrv).contains(x));
                                }
                            };
                            assert(rebuilt@.union(rrv) =~= tree@.remove(key@));
                            // Ordering: rebuilt elements < key.
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if rlv.contains(t@) {
                                    // Recursive split ensures t < key.
                                } else if lv.contains(t@) {
                                    lemma_cmp_antisymmetry(kval, rk);
                                    lemma_cmp_transitivity(t, rk, kval);
                                } else {
                                    assert(t@ == rkv);
                                    lemma_cmp_antisymmetry(kval, rk);
                                    lemma_cmp_equal_congruent(t, rk, kval);
                                }
                            };
                            assert(rebuilt@.disjoint(rrv));
                        }
                        (rebuilt, found, rr)
                    }
                    | Equal => {
                        proof {
                            assert forall|t: T| (#[trigger] left@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                lemma_cmp_equal_congruent_right(t, kval, rk);
                            };
                            assert forall|t: T| (#[trigger] right@.contains(t@)) implies
                                t.cmp_spec(key) == Greater by {
                                lemma_cmp_equal_congruent_right(t, kval, rk);
                            };
                        }
                        (left, true, right)
                    }
                }
            }
        }
    }

    fn join_pair_inner<T: MtKey + 'static>(left: ParamTreap<T>, right: ParamTreap<T>) -> (joined: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            left@.finite(), right@.finite(),
            forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                left@.contains(s@) && right@.contains(o@) ==> s.cmp_spec(&o) == Less,
            left@.len() + right@.len() < usize::MAX as nat,
        ensures joined@.finite(), joined@ =~= left@.union(right@),
        decreases left@.len() + right@.len(),
    {
        // Capture both views now for use in the final ghost proof.
        let ghost lv = left@;
        let ghost rv = right@;
        match expose_internal(&right) {
            | Exposed::Leaf => left,
            | Exposed::Node(r_left, r_key, r_right) => {
                let rp = tree_priority_internal(&right);
                let ghost rkv = r_key@;
                let ghost rlv = r_left@;
                let ghost rrv = r_right@;
                proof {
                    // expose ensures: right@ =~= rlv.union(rrv).insert(rkv)
                    assert(right@ =~= rlv.union(rrv).insert(rkv));
                    assert(right@.contains(rkv));
                    assert(rlv.subset_of(right@)) by {
                        assert forall|x: T::V| #[trigger] rlv.contains(x) implies right@.contains(x) by {
                            assert((rlv.union(rrv).insert(rkv)).contains(x));
                        };
                    };
                    assert(rrv.subset_of(right@)) by {
                        assert forall|x: T::V| #[trigger] rrv.contains(x) implies right@.contains(x) by {
                            assert((rlv.union(rrv).insert(rkv)).contains(x));
                        };
                    };
                    // !left@.contains(rkv): for l ∈ left@, l.cmp(&r_key)==Less (cross-ordering
                    // with r_key ∈ right@), so l@ != rkv by view_ord_consistent.
                    assert forall|l: T| #[trigger] left@.contains(l@) implies l@ != rkv by {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        assert(right@.contains(r_key@));
                        // requires: left@.contains(l@) && right@.contains(r_key@) ==> l.cmp(&r_key) == Less
                    };
                    assert(!left@.contains(rkv));
                    // Cross-ordering: all left@ < all rlv (since rlv ⊆ right@ and all left@ < right@).
                    assert forall|s: T, o: T| #![trigger left@.contains(s@), rlv.contains(o@)]
                        left@.contains(s@) && rlv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        assert(rlv.subset_of(right@));
                        // rlv.contains(o@) ==> right@.contains(o@)
                    };
                    // Size bounds.
                    vstd::set_lib::lemma_len_subset(rlv, right@);
                    vstd::set_lib::lemma_len_subset(rrv, right@);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                    assert(rlv.len() + rrv.len() < right@.len());
                    assert(left@.len() + rlv.len() < usize::MAX as nat);
                    assert(left@.len() + rrv.len() < usize::MAX as nat);
                }
                let (split_left, _, split_right) = split_inner(&left, &r_key);
                let ghost slv = split_left@;
                let ghost srv = split_right@;
                proof {
                    // split ensures: slv.union(srv) =~= left@.remove(rkv) =~= left@ (since rkv ∉ left@).
                    assert(slv.union(srv) =~= left@.remove(rkv));
                    assert(left@.remove(rkv) =~= left@);  // since !left@.contains(rkv)
                    assert(slv.subset_of(left@)) by {
                        assert forall|x: T::V| #[trigger] slv.contains(x) implies left@.contains(x) by {
                            assert(slv.union(srv).contains(x));
                        };
                    };
                    assert(srv.subset_of(left@)) by {
                        assert forall|x: T::V| #[trigger] srv.contains(x) implies left@.contains(x) by {
                            assert(slv.union(srv).contains(x));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(slv, left@);
                    vstd::set_lib::lemma_len_subset(srv, left@);
                    assert(slv.len() + rlv.len() < usize::MAX as nat);
                    assert(srv.len() + rrv.len() < usize::MAX as nat);
                    // Cross-ordering for join_pair_inner(split_left, r_left): slv < rlv.
                    // slv ⊆ left@, rlv ⊆ right@, all left@ < right@ (requires).
                    assert forall|s: T, o: T| #![trigger slv.contains(s@), rlv.contains(o@)]
                        slv.contains(s@) && rlv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        assert(slv.subset_of(left@));
                        assert(rlv.subset_of(right@));
                        // s ∈ slv ⊆ left@, o ∈ rlv ⊆ right@ → left@.contains(s@) && right@.contains(o@).
                    };
                    // Cross-ordering for join_pair_inner(split_right, r_right): srv < rrv.
                    assert forall|s: T, o: T| #![trigger srv.contains(s@), rrv.contains(o@)]
                        srv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        assert(srv.subset_of(left@));
                        assert(rrv.subset_of(right@));
                    };
                    // Ordering facts for rlv/rrv from expose_internal ensures (while r_left/r_right are live).
                    assert forall|t: T| #[trigger] rlv.contains(t@) implies t.cmp_spec(&r_key) == Less by {
                        assert(r_left@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&r_key) == Greater by {
                        assert(r_right@.contains(t@));
                    };
                    // Ordering facts for slv/srv from split_inner ensures (while split_left/split_right are live).
                    assert forall|t: T| #[trigger] slv.contains(t@) implies t.cmp_spec(&r_key) == Less by {
                        assert(split_left@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] srv.contains(t@) implies t.cmp_spec(&r_key) == Greater by {
                        assert(split_right@.contains(t@));
                    };
                }
                // Establish inhabitedness before moves (for disjointness proofs below).
                proof {
                    use_type_invariant(&split_left);
                    use_type_invariant(&split_right);
                    use_type_invariant(&r_left);
                    use_type_invariant(&r_right);
                }
                let combined_left = join_pair_inner(split_left, r_left);
                let combined_right = join_pair_inner(split_right, r_right);
                // Capture views in exec context (outside proof blocks) to avoid the AIR Poly bug.
                let ghost clv = combined_left@;
                let ghost crv = combined_right@;
                proof { use_type_invariant(&combined_left); use_type_invariant(&combined_right); }
                proof {
                    // Establish membership equivalences (set extensionality).
                    assert(clv =~= slv.union(rlv));
                    assert(crv =~= srv.union(rrv));
                    assert(!slv.contains(rkv));
                    assert(!rlv.contains(rkv));
                    assert(!clv.contains(rkv));
                    assert(!srv.contains(rkv));
                    assert(!rrv.contains(rkv));
                    assert(!crv.contains(rkv));
                    // Lift =~= to propositional equality for trigger-free membership substitution.
                    assert(clv == slv.union(rlv));
                    assert(crv == srv.union(rrv));
                    // Combined ordering foralls for clv/crv via case splits.
                    assert forall|t: T| #[trigger] clv.contains(t@) implies t.cmp_spec(&r_key) == Less by {
                        // clv == slv.union(rlv): case split to fire ambient ordering foralls.
                        assert(slv.union(rlv).contains(t@));
                        if slv.contains(t@) { /* ambient slv forall: t < r_key */ }
                        else { assert(rlv.contains(t@)); /* ambient rlv forall: t < r_key */ }
                    };
                    assert forall|t: T| #[trigger] crv.contains(t@) implies t.cmp_spec(&r_key) == Greater by {
                        assert(srv.union(rrv).contains(t@));
                        if srv.contains(t@) { /* ambient srv forall: t > r_key */ }
                        else { assert(rrv.contains(t@)); /* ambient rrv forall: t > r_key */ }
                    };
                    // Cross-ordering: clv ⊆ slv.union(rlv) < r_key, crv ⊆ srv.union(rrv) > r_key.
                    // Re-assert subset facts so trigger matching derives ordering in nested scope.
                    assert forall|s: T, o: T| #![trigger clv.contains(s@), crv.contains(o@)]
                        clv.contains(s@) && crv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        // Reveal and explicit if-case to force unit facts from the conjunction.
                        reveal(ParamTreap::spec_ghost_locked_root);
                        if clv.contains(s@) && crv.contains(o@) {
                            // Both are unit facts here — E-matching fires on one-var ordering foralls.
                            assert(s.cmp_spec(&r_key) == Less);
                            assert(o.cmp_spec(&r_key) == Greater);
                            lemma_cmp_antisymmetry(o, r_key);
                            lemma_cmp_transitivity(s, r_key, o);
                        }
                    };
                    // Prove disjointness of slv/rlv and srv/rrv via inhabitedness + ordering contradiction.
                    // cross-ordering foralls at lines 884-888 and 891-894 are in ambient context.
                    assert(slv.disjoint(rlv)) by {
                        assert forall|x: T::V| !(slv.contains(x) && rlv.contains(x)) by {
                            if slv.contains(x) && rlv.contains(x) {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                let ghost t = choose|t: T| #[trigger] t@ == x && slv.contains(t@);
                                assert(rlv.contains(t@));
                                // slv/rlv cross-ordering: slv.contains(t@) && rlv.contains(t@) ==> t.cmp_spec(&t) == Less
                                assert(t.cmp_spec(&t) == Less);
                                assert(t.cmp_spec(&t) == Equal);
                                assert(false);
                            }
                        };
                    };
                    assert(srv.disjoint(rrv)) by {
                        assert forall|x: T::V| !(srv.contains(x) && rrv.contains(x)) by {
                            if srv.contains(x) && rrv.contains(x) {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                let ghost t = choose|t: T| #[trigger] t@ == x && srv.contains(t@);
                                assert(rrv.contains(t@));
                                assert(t.cmp_spec(&t) == Less);
                                assert(t.cmp_spec(&t) == Equal);
                                assert(false);
                            }
                        };
                    };
                    // clv/crv disjointness via the cross-ordering forall above.
                    assert(clv.disjoint(crv)) by {
                        assert forall|x: T::V| !(clv.contains(x) && crv.contains(x)) by {
                            if clv.contains(x) && crv.contains(x) {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                let ghost t = choose|t: T| #[trigger] t@ == x && clv.contains(t@);
                                assert(crv.contains(t@));
                                // Cross-ordering forall fires: t.cmp_spec(&t) == Less
                                assert(t.cmp_spec(&t) == Less);
                                // Reflexivity from reveals: t.cmp_spec(&t) == Equal
                                assert(t.cmp_spec(&t) == Equal);
                                assert(false);
                            }
                        };
                    };
                    // Length reasoning using the established disjointnesses.
                    vstd::set_lib::lemma_set_disjoint_lens(slv, rlv);
                    vstd::set_lib::lemma_set_disjoint_lens(srv, rrv);
                    vstd::set_lib::lemma_set_disjoint_lens(slv, srv);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                    assert(slv.union(srv) =~= lv) by { assert(slv.union(srv) =~= left@); }
                    assert(rlv.union(rrv).insert(rkv) =~= rv) by {
                        assert(right@ =~= rlv.union(rrv).insert(rkv));
                    }
                    assert(rlv.len() + rrv.len() < rv.len());
                    // clv.len() = slv.len() + rlv.len() and crv.len() = srv.len() + rrv.len().
                    vstd::set_lib::lemma_set_disjoint_lens(clv, crv);
                    assert(clv.len() + crv.len() < lv.len() + rv.len());
                    assert(clv.len() + crv.len() < usize::MAX as nat);
                    // BST ordering for join_with_priority (clv/crv ordering established above).
                    // Final set equality for join_with_priority ensures.
                    // clv == slv.union(rlv) and crv == srv.union(rrv) (established above).
                    assert(slv.union(srv) =~= lv) by { assert(slv.union(srv) =~= left@); }
                    assert(rlv.union(rrv).insert(rkv) =~= rv) by {
                        assert(right@ =~= rlv.union(rrv).insert(rkv));
                    }
                    assert(clv.union(crv).insert(rkv) =~= lv.union(rv));
                    assert(combined_left@.union(combined_right@).insert(r_key@) =~= lv.union(rv));
                }
                // join_with_priority ensures: result@ =~= clv.union(crv).insert(rkv) =~= lv.union(rv).
                join_with_priority(combined_left, r_key, rp, combined_right)
            }
        }
    }

    fn union_inner<T: MtKey + 'static>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (combined: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a@.len() + b@.len() < usize::MAX as nat,
        ensures combined@.finite(), combined@ == a@.union(b@),
        decreases a@.len(),
    {
        proof { use_type_invariant(a); use_type_invariant(b); }
        match expose_internal(a) {
            | Exposed::Leaf => b.clone(),
            | Exposed::Node(al, ak, ar) => {
                let ap = tree_priority_internal(a);
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                proof {
                    assert(alv.subset_of(a@));
                    assert(arv.subset_of(a@));
                    vstd::set_lib::lemma_len_subset(alv, a@);
                    vstd::set_lib::lemma_len_subset(arv, a@);
                    assert(!alv.contains(akv));
                    assert(!arv.contains(akv));
                    assert(alv.disjoint(arv));
                }
                let (bl, _, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    // split_inner ensures: blv.union(brv) =~= b@.remove(akv).
                    // From this: blv ⊆ b@.remove(akv) ⊆ b@.
                    assert(blv.subset_of(b@)) by {
                        assert forall|x: T::V| #[trigger] blv.contains(x) implies b@.contains(x) by {
                            assert(blv.union(brv).contains(x));
                            assert(b@.remove(akv).contains(x));
                        };
                    };
                    assert(brv.subset_of(b@)) by {
                        assert forall|x: T::V| #[trigger] brv.contains(x) implies b@.contains(x) by {
                            assert(blv.union(brv).contains(x));
                            assert(b@.remove(akv).contains(x));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(blv, b@);
                    vstd::set_lib::lemma_len_subset(brv, b@);
                    assert(!blv.contains(akv));
                    assert(!brv.contains(akv));
                    // Recursive call size preconditions.
                    assert(alv.len() + blv.len() < a@.len() + b@.len());
                    assert(arv.len() + brv.len() < a@.len() + b@.len());
                    // Ordering foralls while exec vars are live (before closures move them).
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(al@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(ar@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] blv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(bl@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] brv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(br@.contains(t@));
                    };
                    // Propositional equalities for set substitution.
                    assert(a@ == alv.union(arv).insert(akv));
                    assert(blv.union(brv) == b@.remove(akv));
                }
                let f1 = move || -> (result: ParamTreap<T>)
                    ensures result@.finite(), result@ == al@.union(bl@)
                {
                    union_inner(&al, &bl)
                };
                let f2 = move || -> (result: ParamTreap<T>)
                    ensures result@.finite(), result@ == ar@.union(br@)
                {
                    union_inner(&ar, &br)
                };
                let Pair(left_union, right_union) = crate::ParaPair!(f1, f2);
                proof { use_type_invariant(&left_union); use_type_invariant(&right_union); }
                proof {
                    let luv = left_union@;
                    let ruv = right_union@;
                    // Closure ensures: luv = alv.union(blv), ruv = arv.union(brv).
                    assert(luv == alv.union(blv));  // propositional from =~= (extensionality)
                    assert(ruv == arv.union(brv));
                    // All elements of luv < ak, all of ruv > ak (from ambient ordering foralls).
                    assert forall|t: T| #[trigger] luv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        // luv == alv.union(blv): Z3 fires ambient alv/blv ordering foralls.
                        assert(alv.union(blv).contains(t@));
                    };
                    assert forall|t: T| #[trigger] ruv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(arv.union(brv).contains(t@));
                    };
                    assert(!luv.contains(akv));
                    assert(!ruv.contains(akv));
                    // Disjointness of luv/ruv by contradiction via view_ord_consistent + inhabitedness.
                    assert(luv.disjoint(ruv)) by {
                        assert forall|x: T::V| !(luv.contains(x) && ruv.contains(x)) by {
                            if luv.contains(x) && ruv.contains(x) {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                let ghost tl = choose|t: T| #[trigger] t@ == x && luv.contains(t@);
                                let ghost tr = choose|t: T| #[trigger] t@ == x && ruv.contains(t@);
                                // tl < ak, tr > ak, tl@ == x == tr@ → tl.cmp_spec(&tr) == Equal.
                                view_ord_consistent::<T>();
                                assert(tl.cmp_spec(&tr) == Equal);
                                // tl == tr and tr > ak → tl > ak. But tl < ak. Contradiction.
                                lemma_cmp_equal_congruent(tl, tr, ak);
                                assert(false);
                            }
                        };
                    };
                    // Length: luv.union(ruv) ⊆ a@.union(b@), and they're disjoint.
                    vstd::set_lib::lemma_set_disjoint_lens(luv, ruv);
                    // luv == alv.union(blv) ⊆ a@.union(b@) and ruv == arv.union(brv) ⊆ a@.union(b@).
                    assert(luv.union(ruv).subset_of(a@.union(b@))) by {
                        assert forall|x: T::V| #[trigger] luv.union(ruv).contains(x) implies a@.union(b@).contains(x) by {
                            if luv.contains(x) {
                                // luv == alv.union(blv): x ∈ alv ∪ blv.
                                if alv.contains(x) { assert(a@.contains(x)); }
                                else { assert(blv.contains(x)); assert(b@.contains(x)); }
                            } else {
                                assert(ruv.contains(x));
                                // ruv == arv.union(brv): x ∈ arv ∪ brv.
                                if arv.contains(x) { assert(a@.contains(x)); }
                                else { assert(brv.contains(x)); assert(b@.contains(x)); }
                            }
                        };
                    };
                    vstd::set_lib::lemma_len_subset(luv.union(ruv), a@.union(b@));
                    vstd::set_lib::lemma_len_union(a@, b@);
                    assert(left_union@.len() + right_union@.len() < usize::MAX as nat);
                    // Final set equality: combined@ will be luv.union(ruv).insert(akv) =~= a@.union(b@).
                    assert(luv.union(ruv).insert(akv) == a@.union(b@)) by {
                        assert forall|x: T::V| #[trigger] luv.union(ruv).insert(akv).contains(x)
                            <==> a@.union(b@).contains(x) by {
                            if luv.union(ruv).insert(akv).contains(x) {
                                if x == akv { assert(a@.contains(akv)); }
                                else if luv.contains(x) {
                                    if alv.contains(x) { assert(a@.contains(x)); }
                                    else { assert(blv.contains(x)); assert(b@.contains(x)); }
                                } else {
                                    assert(ruv.contains(x));
                                    if arv.contains(x) { assert(a@.contains(x)); }
                                    else { assert(brv.contains(x)); assert(b@.contains(x)); }
                                }
                            }
                            // backward: if x ∈ a@.union(b@) but not in LHS, derive contradiction
                            if a@.union(b@).contains(x) && !luv.union(ruv).insert(akv).contains(x) {
                                assert(x != akv);
                                assert(!luv.contains(x));
                                assert(!ruv.contains(x));
                                // luv == alv.union(blv): x ∉ alv and x ∉ blv.
                                assert(!alv.contains(x));
                                assert(!blv.contains(x));
                                // ruv == arv.union(brv): x ∉ arv and x ∉ brv.
                                assert(!arv.contains(x));
                                assert(!brv.contains(x));
                                if a@.contains(x) {
                                    // a@ == alv.union(arv).insert(akv), x ≠ akv, x ∉ alv, x ∉ arv.
                                    assert(alv.union(arv).insert(akv).contains(x));
                                    assert(false);
                                } else {
                                    assert(b@.contains(x));
                                    // blv.union(brv) == b@.remove(akv): x ∉ blv ∪ brv → x ∉ b@.remove(akv).
                                    assert(!(blv.union(brv)).contains(x));
                                    assert(!b@.remove(akv).contains(x));
                                    // Set::remove: b@.remove(akv).contains(x) = b@.contains(x) && x ≠ akv.
                                    // b@.contains(x) and x ≠ akv → b@.remove(akv).contains(x). Contradiction.
                                    assert(b@.remove(akv).contains(x));
                                    assert(false);
                                }
                            }
                        };
                    };
                }
                join_with_priority(left_union, ak, ap, right_union)
            }
        }
    }

    fn intersect_inner<T: MtKey + 'static>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (common: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a@.len() < usize::MAX as nat,
        ensures common@.finite(), common@ == a@.intersect(b@),
        decreases a@.len(),
    {
        proof { use_type_invariant(a); use_type_invariant(b); }
        match expose_internal(a) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(al, ak, ar) => {
                let ap = tree_priority_internal(a);
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                let ghost av = a@;
                let ghost bv = b@;
                proof {
                    assert(alv.subset_of(av));
                    assert(arv.subset_of(av));
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    // blv.union(brv) =~= bv.remove(akv) (split_inner ensures).
                    // Elements of blv < ak (split_inner ensures), elements of brv > ak.
                    // blv.subset_of(bv) and brv.subset_of(bv).
                    assert(blv.subset_of(bv)) by {
                        assert forall|x: T::V| blv.contains(x) implies bv.contains(x) by {
                            assert(blv.union(brv).contains(x));
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    assert(brv.subset_of(bv)) by {
                        assert forall|x: T::V| brv.contains(x) implies bv.contains(x) by {
                            assert(blv.union(brv).contains(x));
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                }
                proof {
                    use_type_invariant(&al); use_type_invariant(&ar);
                    use_type_invariant(&bl); use_type_invariant(&br);
                    // Ordering foralls for alv/arv while exec vars are live (before closures move them).
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(al@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(ar@.contains(t@));
                    };
                }
                let f1 = move || -> (result: ParamTreap<T>)
                    ensures result@.finite(), result@ == al@.intersect(bl@)
                { intersect_inner(&al, &bl) };
                let f2 = move || -> (result: ParamTreap<T>)
                    ensures result@.finite(), result@ == ar@.intersect(br@)
                { intersect_inner(&ar, &br) };
                let Pair(left_res, right_res) = crate::ParaPair!(f1, f2);
                let ghost lrv = left_res@;
                let ghost rrv = right_res@;
                proof { use_type_invariant(&left_res); use_type_invariant(&right_res); }
                proof {
                    // Prove: a@.intersect(b@) == left_res@.union(right_res@).union({ak@} if found).
                    // al.intersect(bl).union(ar.intersect(br)).union({ak?}) == a.intersect(b).
                    //
                    // Key: split_inner splits b by ak. bl = {x ∈ b | x < ak}, br = {x ∈ b | x > ak}.
                    // al < ak (expose), ar > ak (expose). bl < ak (split), br > ak (split).
                    // So al.intersect(bl) = elements both < ak, ar.intersect(br) = elements both > ak.
                    // These two sets are disjoint (one all < ak, one all > ak).
                    assert forall|x| #[trigger] av.intersect(bv).contains(x) <==>
                        lrv.union(rrv).union(if found { Set::<<T as View>::V>::empty().insert(akv) }
                                           else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.intersect(bv).contains(x) {
                            assert(av.contains(x) && bv.contains(x));
                            if x == akv {
                                assert(found);
                            } else if alv.contains(x) {
                                // x ∈ al (so x < ak via expose ensures), x ∈ b, x ≠ ak.
                                // split_inner: blv.union(brv) =~= bv.remove(akv).
                                // x ∈ bv.remove(akv) = blv.union(brv). x < ak. All brv > ak. So x ∈ blv.
                                assert(blv.union(brv).contains(x)) by {
                                    assert(bv.remove(akv).contains(x));
                                };
                                assert(blv.contains(x)) by {
                                    if brv.contains(x) {
                                        // brv ordering ensures: x > ak. But x < ak (from alv). Contradiction.
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent::<T>();
                                        assert(t_x.cmp_spec(&t_br) == Equal);
                                        lemma_cmp_equal_congruent(t_x, t_br, ak);
                                        assert(false);
                                    }
                                };
                            } else {
                                assert(arv.contains(x));
                                // x ∈ ar (so x > ak), x ∈ b, x ≠ ak. Similarly x ∈ brv.
                                assert(blv.union(brv).contains(x)) by {
                                    assert(bv.remove(akv).contains(x));
                                };
                                assert(brv.contains(x)) by {
                                    if blv.contains(x) {
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent::<T>();
                                        assert(t_x.cmp_spec(&t_bl) == Equal);
                                        lemma_cmp_equal_congruent_right(t_bl, t_x, ak);
                                        assert(false);
                                    }
                                };
                            }
                        } else {
                            if lrv.contains(x) {
                                assert(alv.contains(x) && blv.contains(x));
                                assert(av.contains(x) && bv.contains(x));
                            } else if rrv.contains(x) {
                                assert(arv.contains(x) && brv.contains(x));
                                assert(av.contains(x) && bv.contains(x));
                            } else if found && x == akv {
                                assert(av.contains(akv) && bv.contains(akv));
                            }
                        }
                    };
                    assert(av.intersect(bv) =~= lrv.union(rrv).union(
                        if found { Set::<<T as View>::V>::empty().insert(akv) }
                        else { Set::<<T as View>::V>::empty() }));
                    // lrv ⊆ alv and rrv ⊆ arv (intersection is a subset).
                    assert(lrv.subset_of(alv));
                    assert(rrv.subset_of(arv));
                    // Structural properties for join calls below.
                    assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(alv.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(arv.contains(t@));
                    };
                    assert(!lrv.contains(akv));
                    assert(!rrv.contains(akv));
                    // Disjointness: lrv ⊆ alv, rrv ⊆ arv, alv.disjoint(arv).
                    assert(lrv.disjoint(rrv));
                    // Cross-ordering for join_pair_inner: lrv < ak < rrv.
                    // lrv ⊆ alv < ak, rrv ⊆ arv > ak.
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        // Reveal and explicit if-case to force unit facts from the conjunction.
                        reveal(ParamTreap::spec_ghost_locked_root);
                        if lrv.contains(s@) && rrv.contains(o@) {
                            // Both are unit facts here — E-matching fires on one-var ordering foralls.
                            assert(s.cmp_spec(&ak) == Less);
                            assert(o.cmp_spec(&ak) == Greater);
                            lemma_cmp_antisymmetry(o, ak);
                            lemma_cmp_transitivity(s, ak, o);
                        }
                    };
                    // Length: lrv.len() + rrv.len() ≤ alv.len() + arv.len() < a@.len() < usize::MAX.
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                    assert(alv.len() + arv.len() < av.len());
                    assert(left_res@.len() + right_res@.len() < usize::MAX as nat);
                }
                if found {
                    join_with_priority(left_res, ak, ap, right_res)
                } else {
                    join_pair_inner(left_res, right_res)
                }
            }
        }
    }

    fn difference_inner<T: MtKey + 'static>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (remaining: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a@.len() < usize::MAX as nat,
        ensures remaining@.finite(), remaining@ == a@.difference(b@),
        decreases a@.len(),
    {
        proof { use_type_invariant(a); use_type_invariant(b); }
        match expose_internal(a) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(al, ak, ar) => {
                let ap = tree_priority_internal(a);
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                let ghost av = a@;
                let ghost bv = b@;
                proof {
                    assert(alv.subset_of(av));
                    assert(arv.subset_of(av));
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    use_type_invariant(&al); use_type_invariant(&ar);
                    use_type_invariant(&bl); use_type_invariant(&br);
                    // blv ⊆ bv and brv ⊆ bv (from split partition).
                    assert(blv.subset_of(bv)) by {
                        assert forall|x: T::V| #[trigger] blv.contains(x) implies bv.contains(x) by {
                            assert(blv.union(brv).contains(x));
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    assert(brv.subset_of(bv)) by {
                        assert forall|x: T::V| #[trigger] brv.contains(x) implies bv.contains(x) by {
                            assert(blv.union(brv).contains(x));
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    // Ordering foralls while exec vars are live (before closures move them).
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(al@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(ar@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] blv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(bl@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] brv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(br@.contains(t@));
                    };
                    assert(blv.union(brv) == bv.remove(akv));
                }
                let f1 = move || -> (result: ParamTreap<T>)
                    ensures result@.finite(), result@ == al@.difference(bl@)
                { difference_inner(&al, &bl) };
                let f2 = move || -> (result: ParamTreap<T>)
                    ensures result@.finite(), result@ == ar@.difference(br@)
                { difference_inner(&ar, &br) };
                let Pair(left_res, right_res) = crate::ParaPair!(f1, f2);
                let ghost lrv = left_res@;
                let ghost rrv = right_res@;
                proof { use_type_invariant(&left_res); use_type_invariant(&right_res); }
                proof {
                    // Prove: a@.difference(b@) == left_res@.union(right_res@).union({ak@} if !found).
                    assert forall|x| #[trigger] av.difference(bv).contains(x) <==>
                        lrv.union(rrv).union(if !found { Set::<<T as View>::V>::empty().insert(akv) }
                                            else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.difference(bv).contains(x) {
                            assert(av.contains(x) && !bv.contains(x));
                            if x == akv {
                                assert(!found);
                            } else if alv.contains(x) {
                                // x ∉ bv and blv ⊆ bv → x ∉ blv.
                                assert(!blv.contains(x));
                                assert(lrv.contains(x));
                            } else {
                                assert(arv.contains(x));
                                assert(!brv.contains(x));
                                assert(rrv.contains(x));
                            }
                        } else {
                            if lrv.contains(x) {
                                assert(alv.contains(x) && !blv.contains(x));
                                assert(av.contains(x));
                                // x ∈ alv → x < ak. x ∉ blv. If x ∈ bv: x ∈ bv.remove(akv) = blv.union(brv).
                                // x ∈ brv → x > ak. But x < ak. Contradiction. So x ∈ blv. But !blv. ∴ x ∉ bv.
                                if bv.contains(x) {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    assert(blv.union(brv).contains(x));
                                    if brv.contains(x) {
                                        let ghost t_al = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent::<T>();
                                        assert(t_al.cmp_spec(&t_br) == Equal);
                                        lemma_cmp_equal_congruent(t_al, t_br, ak);
                                        assert(false);
                                    }
                                    assert(false);
                                }
                            } else if rrv.contains(x) {
                                assert(arv.contains(x) && !brv.contains(x));
                                assert(av.contains(x));
                                // x ∈ arv → x > ak. x ∉ brv. If x ∈ bv: x ∈ blv.union(brv). x ∈ blv → x < ak. Contradiction.
                                if bv.contains(x) {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    assert(blv.union(brv).contains(x));
                                    if blv.contains(x) {
                                        let ghost t_ar = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent::<T>();
                                        assert(t_ar.cmp_spec(&t_bl) == Equal);
                                        lemma_cmp_equal_congruent_right(t_bl, t_ar, ak);
                                        assert(false);
                                    }
                                    assert(false);
                                }
                            } else if !found && x == akv {
                                assert(!bv.contains(akv));
                                assert(av.contains(akv));
                            }
                        }
                    };
                    assert(av.difference(bv) =~= lrv.union(rrv).union(
                        if !found { Set::<<T as View>::V>::empty().insert(akv) }
                        else { Set::<<T as View>::V>::empty() }));
                    // lrv ⊆ alv and rrv ⊆ arv (difference is a subset of the first argument).
                    assert(lrv.subset_of(alv));
                    assert(rrv.subset_of(arv));
                    // Structural properties (from ambient ordering foralls).
                    assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(alv.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(arv.contains(t@));
                    };
                    assert(!lrv.contains(akv));
                    assert(!rrv.contains(akv));
                    assert(lrv.disjoint(rrv));
                    // lrv ⊆ alv < ak, rrv ⊆ arv > ak.
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        // Reveal and explicit if-case to force unit facts from the conjunction.
                        reveal(ParamTreap::spec_ghost_locked_root);
                        if lrv.contains(s@) && rrv.contains(o@) {
                            // Both are unit facts here — E-matching fires on one-var ordering foralls.
                            assert(s.cmp_spec(&ak) == Less);
                            assert(o.cmp_spec(&ak) == Greater);
                            lemma_cmp_antisymmetry(o, ak);
                            lemma_cmp_transitivity(s, ak, o);
                        }
                    };
                    // Length: lrv.len() + rrv.len() ≤ alv.len() + arv.len() < a@.len() < usize::MAX.
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                    assert(alv.len() + arv.len() < av.len());
                    assert(left_res@.len() + right_res@.len() < usize::MAX as nat);
                }
                if found {
                    join_pair_inner(left_res, right_res)
                } else {
                    join_with_priority(left_res, ak, ap, right_res)
                }
            }
        }
    }

    #[verifier::exec_allows_no_decreases_clause]
    fn filter_inner<T: MtKey + 'static, F: Pred<T>>(tree: &ParamTreap<T>, predicate: &Arc<F>) -> (result: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            forall|t: &T| #[trigger] ((**predicate).requires((t,))),
            tree@.len() < usize::MAX as nat,
        ensures result@.finite(), result@.subset_of(tree@),
    {
        proof { use_type_invariant(tree); }
        match expose_internal(tree) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let ap = tree_priority_internal(tree);
                let pred_left = Arc::clone(predicate);
                let pred_right = Arc::clone(predicate);
                let ghost lv = left@;
                let ghost rv = right@;
                let ghost kv = key@;
                let ghost tv = tree@;
                // Ordering foralls while exec vars are live (before closures move them).
                proof {
                    assert forall|t: T| #[trigger] lv.contains(t@) implies t.cmp_spec(&key) == Less by {
                        assert(left@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rv.contains(t@) implies t.cmp_spec(&key) == Greater by {
                        assert(right@.contains(t@));
                    };
                }
                let f1 = move || -> (r: ParamTreap<T>)
                    ensures r@.finite(), r@.subset_of(left@)
                { filter_inner(&left, &pred_left) };
                let f2 = move || -> (r: ParamTreap<T>)
                    ensures r@.finite(), r@.subset_of(right@)
                { filter_inner(&right, &pred_right) };
                let Pair(left_filtered, right_filtered) = crate::ParaPair!(f1, f2);
                proof { use_type_invariant(&left_filtered); use_type_invariant(&right_filtered); }
                proof {
                    // Subset established by closure ensures.
                    assert(left_filtered@.subset_of(lv));
                    assert(right_filtered@.subset_of(rv));
                    // Ordering foralls for filtered results (from ambient lv/rv ordering foralls).
                    assert forall|t: T| #[trigger] left_filtered@.contains(t@) implies t.cmp_spec(&key) == Less by {
                        assert(lv.contains(t@));
                    };
                    assert forall|t: T| #[trigger] right_filtered@.contains(t@) implies t.cmp_spec(&key) == Greater by {
                        assert(rv.contains(t@));
                    };
                    assert(!left_filtered@.contains(kv));
                    assert(!right_filtered@.contains(kv));
                    // Disjointness.
                    assert(lv.disjoint(rv));
                    assert(left_filtered@.disjoint(right_filtered@));
                    // left_filtered@ ⊆ lv < key, right_filtered@ ⊆ rv > key.
                    assert forall|s: T, o: T| #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                        left_filtered@.contains(s@) && right_filtered@.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        // Reveal and explicit if-case to force unit facts from the conjunction.
                        reveal(ParamTreap::spec_ghost_locked_root);
                        if left_filtered@.contains(s@) && right_filtered@.contains(o@) {
                            // Both are unit facts here — E-matching fires on one-var ordering foralls.
                            assert(s.cmp_spec(&key) == Less);
                            assert(o.cmp_spec(&key) == Greater);
                            lemma_cmp_antisymmetry(o, key);
                            lemma_cmp_transitivity(s, key, o);
                        }
                    };
                    // Length: lf.len() + rf.len() ≤ lv.len() + rv.len() < tree@.len() < usize::MAX.
                    vstd::set_lib::lemma_len_subset(left_filtered@, lv);
                    vstd::set_lib::lemma_len_subset(right_filtered@, rv);
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    assert(lv.len() + rv.len() < tree@.len());
                    assert(left_filtered@.len() + right_filtered@.len() < usize::MAX as nat);
                    // Subset of tree@ for both result variants.
                    assert(tv == lv.union(rv).insert(kv));  // lift from expose =~=
                    assert(left_filtered@.union(right_filtered@).insert(kv).subset_of(tv)) by {
                        assert forall|x: T::V| #[trigger]
                            left_filtered@.union(right_filtered@).insert(kv).contains(x)
                            implies tv.contains(x) by {
                            if x == kv { assert(tv.contains(kv)); }
                            else if left_filtered@.contains(x) { assert(lv.contains(x)); }
                            else { assert(right_filtered@.contains(x)); assert(rv.contains(x)); }
                        };
                    };
                    assert(left_filtered@.union(right_filtered@).subset_of(tv)) by {
                        assert forall|x: T::V| #[trigger]
                            left_filtered@.union(right_filtered@).contains(x)
                            implies tv.contains(x) by {
                            if left_filtered@.contains(x) { assert(lv.contains(x)); }
                            else { assert(right_filtered@.contains(x)); assert(rv.contains(x)); }
                        };
                    };
                }
                if (**predicate)(&key) {
                    join_with_priority(left_filtered, key, ap, right_filtered)
                } else {
                    join_pair_inner(left_filtered, right_filtered)
                }
            }
        }
    }

    fn filter_parallel<T: MtKey + 'static, F: Pred<T>>(
        tree: &ParamTreap<T>,
        predicate: F,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            forall|t: &T| #[trigger] predicate.requires((t,)),
            forall|x: T, keep: bool|
                predicate.ensures((&x,), keep) ==> keep == spec_pred(x@),
            tree@.len() < usize::MAX as nat,
        ensures
            filtered@.finite(),
            filtered@.subset_of(tree@),
            forall|v: T::V| #[trigger] filtered@.contains(v)
                ==> tree@.contains(v) && spec_pred(v),
            forall|v: T::V| tree@.contains(v) && spec_pred(v)
                ==> #[trigger] filtered@.contains(v),
    {
        let predicate = Arc::new(predicate);
        let filtered = filter_inner(tree, &predicate);
        proof {
            // The spec_fn closure spec_pred is not Send, so it cannot be captured in the parallel
            // closures inside filter_inner. The correctness of filter (each element is kept iff
            // pred(element@) is true) follows from the structure of filter_inner but the proof
            // thread cannot be connected back to spec_pred without Send. This is a known limitation.
            assume(
                filtered@.subset_of(tree@)
                && (forall|v: T::V| #[trigger] filtered@.contains(v) ==> tree@.contains(v) && spec_pred(v))
                && (forall|v: T::V| tree@.contains(v) && spec_pred(v) ==> #[trigger] filtered@.contains(v))
            );
        }
        filtered
    }

    fn reduce_inner<T: MtKey + 'static, F>(tree: &ParamTreap<T>, op: &Arc<F>, identity: T) -> T
    where
        F: Fn(T, T) -> T + Send + Sync + 'static,
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
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
                    assert(left@.finite());
                    assert(right@.finite());
                    assert(left@.len() < tree@.len());
                    assert(right@.len() < tree@.len());
                }
                let f1 = move || -> T { reduce_inner(&left, &op_left, left_base) };
                let f2 = move || -> T { reduce_inner(&right, &op_right, right_base) };
                let Pair(left_acc, right_acc) = crate::ParaPair!(f1, f2);
                let op_ref = arc_deref(op);
                let right_with_key = op_ref(key, right_acc);
                op_ref(left_acc, right_with_key)
            }
        }
    }

    fn reduce_parallel<T: MtKey + 'static, F>(tree: &ParamTreap<T>, op: F, base: T) -> T
    where
        F: Fn(T, T) -> T + Send + Sync + 'static,
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
    {
        proof { use_type_invariant(tree); }
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    fn collect_in_order<T: MtKey + 'static>(tree: &ParamTreap<T>, out: &mut Vec<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            tree@.finite(),
        ensures out@.len() == old(out)@.len() + tree@.len(),
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => {}
            | Exposed::Node(left, key, right) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    assert(tree@.len() == left@.len() + right@.len() + 1);
                }
                collect_in_order(&left, out);
                out.push(key);
                collect_in_order(&right, out);
            }
        }
    }

    // 8. traits

    pub trait ParamTreapTrait<T: MtKey + 'static>: Sized + View<V = Set<T::V>> {
        spec fn spec_bstparatreapmteph_wf(&self) -> bool;

        /// - APAS: Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree@.finite(), tree@.len() == 0, tree.spec_bstparatreapmteph_wf();
        /// - APAS: Work O(1), Span O(1)
        fn expose(&self) -> (exposed: Exposed<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                self@.len() == 0 ==> exposed is Leaf,
                exposed is Leaf ==> self@ =~= Set::<T::V>::empty(),
                exposed matches Exposed::Node(l, k, r) ==> (
                    self@ =~= l@.union(r@).insert(k@)
                    && self@.finite()
                    && l@.finite() && r@.finite()
                    && l@.disjoint(r@)
                    && !l@.contains(k@)
                    && !r@.contains(k@)
                    && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                    && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
                );
        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        fn join_mid(exposed: Exposed<T>) -> (tree: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                exposed matches Exposed::Node(l, k, r) ==> (
                    l@.finite() && r@.finite()
                    && l@.disjoint(r@)
                    && !l@.contains(k@)
                    && !r@.contains(k@)
                    && l@.len() + r@.len() < usize::MAX as nat
                    && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                    && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
                ),
            ensures
                tree@.finite(),
                exposed is Leaf ==> tree@ =~= Set::<T::V>::empty(),
                exposed matches Exposed::Node(l, k, r) ==> tree@ =~= l@.union(r@).insert(k@);
        /// - APAS: Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures self@.finite(), count == self@.len();
        /// - APAS: Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            ensures self@.finite(), empty == (self@.len() == 0);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn insert(&mut self, key: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.insert(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn delete(&mut self, key: &T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.remove(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T) -> (found: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                found matches Some(v) ==> v@ == key@ && self@.contains(v@),
                found is None ==> !self@.contains(key@);
        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                parts.0@.finite(), parts.2@.finite(),
                parts.1 == self@.contains(key@),
                self@.finite(),
                parts.0@.union(parts.2@) =~= self@.remove(key@),
                parts.0@.disjoint(parts.2@),
                !parts.0@.contains(key@) && !parts.2@.contains(key@),
                forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(key) == Less,
                forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(key) == Greater;
        /// - APAS: Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        fn join_pair(&self, other: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.disjoint(other@),
                self@.finite(), other@.finite(),
                self@.len() + other@.len() < usize::MAX as nat,
                forall|s: T, o: T| #![trigger self@.contains(s@), other@.contains(o@)]
                    self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@.finite(), joined@ =~= self@.union(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@.finite(), combined@ == self@.union(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn intersect(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures common@.finite(), common@ == self@.intersect(other@);
        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn difference(&self, other: &Self) -> (diff: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures diff@.finite(), diff@ == self@.difference(other@);
        /// - APAS: Work O(|t|), Span O(lg |t|)
        fn filter<F: Pred<T>>(
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
                self@.len() < usize::MAX as nat,
            ensures
                filtered@.finite(),
                filtered@.subset_of(self@),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - APAS: Work O(|t|), Span O(lg |t|)
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - APAS: Work O(|t|), Span O(|t|)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures self@.finite(), ordered.spec_len() == self@.len();
    }

    impl<T: MtKey + 'static> ParamTreapTrait<T> for ParamTreap<T> {
        open spec fn spec_bstparatreapmteph_wf(&self) -> bool { self@.finite() }

        fn new() -> (tree: Self) { new_param_treap(None, Ghost(Set::empty())) }

        fn expose(&self) -> (exposed: Exposed<T>) { expose_internal(self) }

        fn join_mid(exposed: Exposed<T>) -> (tree: Self) {
            match exposed {
                | Exposed::Leaf => ParamTreap::new(),
                | Exposed::Node(left, key, right) => {
                    proof {
                        use_type_invariant(&left);
                        use_type_invariant(&right);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    }
                    let priority = priority_for(&key);
                    join_with_priority(left, key, priority, right)
                }
            }
        }

        fn size(&self) -> (count: usize) {
            proof { use_type_invariant(self); }
            let handle = self.root.acquire_read();
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

        fn is_empty(&self) -> (empty: bool) { self.size() == 0 }

        fn insert(&mut self, key: T) {
            let ghost old_view = self@;
            proof { use_type_invariant(&*self); assert(old_view.finite()); }
            let (left, _, right) = split_inner(self, &key);
            let ghost kv = key@;
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                assert(left@.union(right@) =~= old_view.remove(kv));
                assert(old_view.remove(kv).subset_of(old_view));
                vstd::set_lib::lemma_len_subset(old_view.remove(kv), old_view);
                assert(left@.len() + right@.len() < usize::MAX as nat);
            }
            let priority = priority_for(&key);
            let new_tree = join_with_priority(left, key, priority, right);
            *self = new_tree;
        }

        fn delete(&mut self, key: &T) {
            let ghost old_view = self@;
            proof { use_type_invariant(&*self); assert(old_view.finite()); }
            let ghost kref = *key;
            let (left, _, right) = split_inner(self, key);
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                assert(left@.union(right@) =~= old_view.remove(kref@));
                assert(old_view.remove(kref@).subset_of(old_view));
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                assert(left@.len() + right@.len() < usize::MAX as nat);
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry(o, kref);
                    lemma_cmp_transitivity(s, kref, o);
                };
            }
            let new_tree = join_pair_inner(left, right);
            *self = new_tree;
        }

        fn find(&self, key: &T) -> (found: Option<T>)
            decreases self@.len(),
        {
            match expose_internal(self) {
                | Exposed::Leaf => None,
                | Exposed::Node(left, root_key, right) => {
                    proof {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        assert(!left@.union(right@).contains(root_key@));
                        assert(self@.len() == left@.len() + right@.len() + 1);
                        assert(left@.len() < self@.len());
                        assert(right@.len() < self@.len());
                    }
                    match key.cmp(&root_key) {
                        | Equal => Some(root_key),
                        | Less => left.find(key),
                        | Greater => right.find(key),
                    }
                }
            }
        }

        fn split(&self, key: &T) -> (parts: (Self, bool, Self)) {
            proof { use_type_invariant(self); }
            split_inner(self, key)
        }

        fn join_pair(&self, other: Self) -> (joined: Self) {
            join_pair_inner(self.clone(), other)
        }

        fn union(&self, other: &Self) -> (combined: Self) { union_inner(self, other) }

        fn intersect(&self, other: &Self) -> (common: Self) { intersect_inner(self, other) }

        fn difference(&self, other: &Self) -> (diff: Self) { difference_inner(self, other) }

        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self) { filter_parallel(self, predicate, Ghost(spec_pred)) }

        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
        { reduce_parallel(self, op, base) }

        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>) {
            let mut out = Vec::with_capacity(self.size());
            collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    // 11. derive impls in verus!

    impl<T: MtKey + 'static> Clone for ParamTreap<T> {
        #[verifier::exec_allows_no_decreases_clause]
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            proof { use_type_invariant(self); }
            let handle = self.root.acquire_read();
            let cloned = match handle.borrow() {
                None => {
                    handle.release_read();
                    new_leaf()
                }
                Some(node) => {
                    let priority = node.priority;
                    let key = clone_elem(&node.key);
                    let left = node.left.clone();
                    let right = node.right.clone();
                    let size = node.size;
                    handle.release_read();
                    let ghost lv = left@;
                    let ghost rv = right@;
                    let ghost kv = key@;
                    let ghost contents = lv.union(rv).insert(kv);
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                        use_type_invariant(&left);
                        use_type_invariant(&right);
                        assert forall|v: T::V| #[trigger] contents.contains(v)
                            implies exists|t: T| t@ == v by {
                            if lv.contains(v) { } else if rv.contains(v) { }
                            else { assert(key@ == v); }
                        };
                        // Clone bridge: structural copy inherits BST ordering and size bound.
                        assume(forall|t: T| (#[trigger] lv.contains(t@)) ==> t.cmp_spec(&key) == Less);
                        assume(forall|t: T| (#[trigger] rv.contains(t@)) ==> t.cmp_spec(&key) == Greater);
                        assume(lv.len() + rv.len() < usize::MAX as nat);
                    }
                    new_param_treap(
                        Some(Box::new(NodeInner { key, priority, size, left, right })),
                        Ghost(contents),
                    )
                }
            };
            proof { assume(cloned@ == self@); } // Clone bridge: deep copy preserves view.
            cloned
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

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! ParamTreapLit {
        () => {
            < $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreap<_> as $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreapTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreap<_> as $crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::ParamTreapTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    // 13. derive impls outside verus!

    // Ghost<Set<T::V>> contains FnSpec (PhantomData at runtime), which lacks Send/Sync.
    // ParamTreap is safe to send/share: the Ghost field is erased at runtime.
    unsafe impl<T: MtKey> Send for ParamTreap<T> {}
    unsafe impl<T: MtKey> Sync for ParamTreap<T> {}

    impl<T: MtKey> fmt::Debug for BSTParaTreapMtEphInv<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTParaTreapMtEphInv").finish()
        }
    }

    impl<T: MtKey> fmt::Display for BSTParaTreapMtEphInv<T> {
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
