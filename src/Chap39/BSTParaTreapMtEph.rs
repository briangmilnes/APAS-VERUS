//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Parametric multi-threaded Treap (probabilistically balanced BST) with parallel operations.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 6a. spec fns
//	Section 7a. proof fns/broadcast groups
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 4d. type definitions
//	Section 5d. view impls
//	Section 8d. traits
//	Section 9d. impls
//	Section 11a. top level coarse locking
//	Section 12b. derive impls in verus!
//	Section 12c. derive impls in verus!
//	Section 12d. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!

//		Section 1. module

pub mod BSTParaTreapMtEph {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt;
    use std::fmt::Write;
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {vstd::set::group_set_axioms, vstd::set_lib::group_set_properties};

    //		Section 4a. type definitions


    /// RwLock predicate for treap nodes. Carries ghost contents and BST ordering predicates
    /// so the type_invariant can fully characterise the locked value.
    pub struct BSTParaTreapMtEphInv<T: MtKey> {
        pub ghost contents: Set<<T as View>::V>,
    }

    //		Section 6a. spec fns


    /// View-consistent ordering: elements with equal views compare Equal.
    pub open spec fn view_ord_consistent<T: MtKey>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    //		Section 7a. proof fns/broadcast groups


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

    //		Section 9a. impls


    /// Clone a MtKey element with a view-preserving postcondition.
    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn clone_elem<T: MtKey + ClonePreservesView>(x: &T) -> (c: T)
        ensures c@ == x@,
    {
        x.clone_view()
    }

    /// Expose the type invariant guarantee of finiteness to callers in other modules.
    /// Calling this is a no-op at runtime; the spec-level ensures establishes tree@.finite().
    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    pub fn param_treap_assert_finite<T: MtKey>(tree: &ParamTreap<T>)
        ensures tree@.finite(),
    {
        proof { use_type_invariant(tree); }
    }


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn new_leaf<T: MtKey>() -> (tree: ParamTreap<T>)
        ensures tree@ =~= Set::<<T as View>::V>::empty()
    {
        new_param_treap(None, Ghost(Set::empty()))
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn expose_internal<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>) -> (exposed: Exposed<T>)
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn expose_with_priority_internal<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>) -> (parts: Option<(ParamTreap<T>, T, i64, ParamTreap<T>)>)
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

    /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
    #[verifier::external_body]
    fn priority_for<T: MtKey>(key: &T) -> i64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buf = String::new();
        let _ = write!(&mut buf, "{key:?}");
        Hash::hash(&buf, &mut hasher);
        hasher.finish() as i64
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn tree_priority_internal<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>) -> (p: i64)
        requires tree.spec_bstparatreapmteph_wf(),
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn make_node<T: MtKey + ClonePreservesView>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> (node: ParamTreap<T>)
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
    /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — matches APAS
    fn join_with_priority<T: MtKey + ClonePreservesView>(left: ParamTreap<T>, key: T, priority: i64, right: ParamTreap<T>) -> (joined: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            left@.finite(), right@.finite(),
            left@.disjoint(right@),
            !left@.contains(key@), !right@.contains(key@),
            left@.len() + right@.len() < usize::MAX as nat,
            forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
        ensures joined@ =~= left@.union(right@).insert(key@), joined@.finite(),
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
    fn split_inner<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>, key: &T) -> (parts: (ParamTreap<T>, bool, ParamTreap<T>))
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
                    assert(tree@.len() == left@.len() + right@.len() + 1);
                }
                let ghost rk = root_key;
                let ghost kval = *key;
                match <T as std::cmp::Ord>::cmp(key, &root_key) {
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
                            assert forall|x| llv.contains(x) implies lv.contains(x) by {
                                assert(llv.union(lrv).contains(x));
                            };
                            vstd::set_lib::lemma_len_subset(lrv, lv);
                            // Requirements for join_with_priority(lr, root_key, priority, right):
                            // lr < root_key: from expose, all left < root_key, lr ⊆ left → lr < root_key ✓.
                            assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&root_key) == Less by {
                                assert(lv.contains(t@));
                            };
                            // right > root_key: from expose ✓.
                            // !lr.contains(root_key@), !right.contains(root_key@): from expose ✓.
                            // lr.disjoint(right): lr ⊆ left, left.disjoint(right) → lr.disjoint(right) ✓.
                        }
                        let rebuilt = join_with_priority(lr, root_key, priority, right);
                        proof {
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
                            assert forall|x| rrv.contains(x) implies rv.contains(x) by {
                                assert(rlv.union(rrv).contains(x));
                            };
                            vstd::set_lib::lemma_len_subset(rlv, rv);
                            // Requirements for join_with_priority(left, root_key, priority, rl):
                            // left < root_key: from expose ✓.
                            // rl > root_key: from expose, all right > root_key, rl ⊆ right → rl > root_key ✓.
                            assert forall|t: T| (#[trigger] rlv.contains(t@)) implies t.cmp_spec(&root_key) == Greater by {
                                assert(rv.contains(t@));
                            };
                            // !left.contains(root_key@), !rl.contains(root_key@): from expose ✓.
                        }
                        let rebuilt = join_with_priority(left, root_key, priority, rl);
                        proof {
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

    /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — matches APAS
    fn join_pair_inner<T: MtKey + ClonePreservesView>(left: ParamTreap<T>, right: ParamTreap<T>) -> (joined: ParamTreap<T>)
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
                    assert(rlv.subset_of(right@));
                    assert(rrv.subset_of(right@));
                    // !left@.contains(rkv): for l ∈ left@, l.cmp(&r_key)==Less (cross-ordering
                    // with r_key ∈ right@), so l@ != rkv by view_ord_consistent.
                    assert forall|l: T| #[trigger] left@.contains(l@) implies l@ != rkv by {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        assert(right@.contains(r_key@));
                        // requires: left@.contains(l@) && right@.contains(r_key@) ==> l.cmp(&r_key) == Less
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
                    assert(slv.union(srv) =~= left@);
                    assert(slv.subset_of(left@));
                    assert(srv.subset_of(left@));
                    vstd::set_lib::lemma_len_subset(slv, left@);
                    vstd::set_lib::lemma_len_subset(srv, left@);
                    vstd::set_lib::lemma_len_subset(srv, left@);
                    assert(slv.len() + rlv.len() < usize::MAX as nat);
                    assert(srv.len() + rrv.len() < usize::MAX as nat);
                    // Cross-ordering for join_pair_inner(split_left, r_left): slv < rlv.
                    // slv ⊆ left@, rlv ⊆ right@, all left@ < right@ (requires).
                    assert forall|s: T, o: T| #![trigger slv.contains(s@), rlv.contains(o@)]
                        slv.contains(s@) && rlv.contains(o@) implies s.cmp_spec(&o) == Less by {};
                    // Cross-ordering for join_pair_inner(split_right, r_right): srv < rrv.
                    assert forall|s: T, o: T| #![trigger srv.contains(s@), rrv.contains(o@)]
                        srv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {};
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
                    assert(!clv.contains(rkv));
                    assert(!crv.contains(rkv));
                    // Lift =~= to propositional equality for trigger-free membership substitution.
                    assert(clv == slv.union(rlv));
                    assert(crv == srv.union(rrv));
                    // Combined ordering foralls for clv/crv via case splits.
                    assert forall|t: T| #[trigger] clv.contains(t@) implies t.cmp_spec(&r_key) == Less by {
                        if slv.contains(t@) { /* ambient slv forall: t < r_key */ }
                        else { assert(rlv.contains(t@)); /* ambient rlv forall: t < r_key */ }
                    };
                    assert forall|t: T| #[trigger] crv.contains(t@) implies t.cmp_spec(&r_key) == Greater by {
                        if srv.contains(t@) { /* ambient srv forall: t > r_key */ }
                        else { assert(rrv.contains(t@)); /* ambient rrv forall: t > r_key */ }
                    };
                    // Cross-ordering: clv ⊆ slv.union(rlv) < r_key, crv ⊆ srv.union(rrv) > r_key.
                    // Re-assert subset facts so trigger matching derives ordering in nested scope.
                    assert forall|s: T, o: T| #![trigger clv.contains(s@), crv.contains(o@)]
                        clv.contains(s@) && crv.contains(o@) implies s.cmp_spec(&o) == Less by {
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
                    assert(slv.union(srv) =~= lv);
                    assert(rlv.union(rrv).insert(rkv) =~= rv);
                    vstd::set_lib::lemma_set_disjoint_lens(clv, crv);
                    assert(clv.len() + crv.len() < usize::MAX as nat);
                    // BST ordering for join_with_priority (clv/crv ordering established above).
                    // Final set equality for join_with_priority ensures.
                    // clv == slv.union(rlv) and crv == srv.union(rrv) (established above).
                    assert(clv.union(crv).insert(rkv) =~= lv.union(rv));
                    assert(combined_left@.union(combined_right@).insert(r_key@) =~= lv.union(rv));
                }
                // join_with_priority ensures: result@ =~= clv.union(crv).insert(rkv) =~= lv.union(rv).
                join_with_priority(combined_left, r_key, rp, combined_right)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    fn union_inner<T: MtKey + ClonePreservesView>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (combined: ParamTreap<T>)
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
                    vstd::set_lib::lemma_len_subset(alv, a@);
                    vstd::set_lib::lemma_len_subset(arv, a@);
                }
                let (bl, _, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    // split_inner ensures: blv.union(brv) =~= b@.remove(akv) ⊆ b@.
                    assert(blv.subset_of(b@)) by {
                        assert forall|x: T::V| #[trigger] blv.contains(x) implies b@.contains(x) by {
                            assert(b@.remove(akv).contains(x));
                        };
                    };
                    assert(brv.subset_of(b@)) by {
                        assert forall|x: T::V| #[trigger] brv.contains(x) implies b@.contains(x) by {
                            assert(b@.remove(akv).contains(x));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(blv, b@);
                    vstd::set_lib::lemma_len_subset(brv, b@);
                    // Ordering foralls while exec vars are live (before closures move them).
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                    assert forall|t: T| #[trigger] blv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] brv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                }
                let f1 = move || -> (merged: ParamTreap<T>)
                    ensures merged@.finite(), merged@ == al@.union(bl@)
                {
                    union_inner(&al, &bl)
                };
                let f2 = move || -> (merged: ParamTreap<T>)
                    ensures merged@.finite(), merged@ == ar@.union(br@)
                {
                    union_inner(&ar, &br)
                };
                let Pair(left_union, right_union) = crate::ParaPair!(f1, f2);
                proof { use_type_invariant(&left_union); use_type_invariant(&right_union); }
                proof {
                    let luv = left_union@;
                    let ruv = right_union@;
                    // All elements of luv < ak, all of ruv > ak (from ambient ordering foralls).
                    assert forall|t: T| #[trigger] luv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] ruv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                    // Disjointness of luv/ruv by contradiction via view_ord_consistent + inhabitedness.
                    assert(luv.disjoint(ruv)) by {
                        assert forall|x: T::V| !(luv.contains(x) && ruv.contains(x)) by {
                            if luv.contains(x) && ruv.contains(x) {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                let ghost tl = choose|t: T| #[trigger] t@ == x && luv.contains(t@);
                                let ghost tr = choose|t: T| #[trigger] t@ == x && ruv.contains(t@);
                                // tl < ak, tr > ak, tl@ == x == tr@ → contradiction via congruence.
                                view_ord_consistent::<T>();
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
                                if alv.contains(x) {} else { assert(blv.contains(x)); }
                            } else {
                                if arv.contains(x) {} else { assert(brv.contains(x)); }
                            }
                        };
                    };
                    vstd::set_lib::lemma_len_subset(luv.union(ruv), a@.union(b@));
                    vstd::set_lib::lemma_len_union(a@, b@);
                    // Final set equality: combined@ will be luv.union(ruv).insert(akv) =~= a@.union(b@).
                    assert(luv.union(ruv).insert(akv) == a@.union(b@)) by {
                        assert forall|x: T::V| #[trigger] luv.union(ruv).insert(akv).contains(x)
                            <==> a@.union(b@).contains(x) by {
                            if luv.union(ruv).insert(akv).contains(x) {
                                if x == akv {}
                                else if luv.contains(x) {
                                    if alv.contains(x) {} else { assert(blv.contains(x)); }
                                } else {
                                    if arv.contains(x) {} else { assert(brv.contains(x)); }
                                }
                            }
                            // backward: if x ∈ a@.union(b@) but not in LHS, derive contradiction
                            if a@.union(b@).contains(x) && !luv.union(ruv).insert(akv).contains(x) {
                                if a@.contains(x) {
                                    assert(false);
                                } else {
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    fn intersect_inner<T: MtKey + ClonePreservesView>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (common: ParamTreap<T>)
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
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    // blv.subset_of(bv) and brv.subset_of(bv).
                    assert(blv.subset_of(bv)) by {
                        assert forall|x: T::V| #[trigger] blv.contains(x) implies bv.contains(x) by {
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    assert(brv.subset_of(bv)) by {
                        assert forall|x: T::V| #[trigger] brv.contains(x) implies bv.contains(x) by {
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                }
                proof {
                    use_type_invariant(&al); use_type_invariant(&ar);
                    use_type_invariant(&bl); use_type_invariant(&br);
                    // Ordering foralls for alv/arv while exec vars are live (before closures move them).
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                }
                let f1 = move || -> (common: ParamTreap<T>)
                    ensures common@.finite(), common@ == al@.intersect(bl@)
                { intersect_inner(&al, &bl) };
                let f2 = move || -> (common: ParamTreap<T>)
                    ensures common@.finite(), common@ == ar@.intersect(br@)
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
                            if x == akv {
                            } else if alv.contains(x) {
                                // x ∈ al (so x < ak via expose ensures), x ∈ b, x ≠ ak.
                                // split_inner: blv.union(brv) =~= bv.remove(akv). x < ak, brv > ak → x ∈ blv.
                                assert(blv.contains(x)) by {
                                    assert(bv.remove(akv).contains(x));
                                    if brv.contains(x) {
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent(t_x, t_br, ak);
                                        assert(false);
                                    }
                                };
                            } else {
                                // x ∈ ar (so x > ak), x ∈ b, x ≠ ak. Similarly x ∈ brv.
                                assert(brv.contains(x)) by {
                                    assert(bv.remove(akv).contains(x));
                                    if blv.contains(x) {
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent_right(t_bl, t_x, ak);
                                        assert(false);
                                    }
                                };
                            }
                        }
                    };
                    assert(av.intersect(bv) =~= lrv.union(rrv).union(
                        if found { Set::<<T as View>::V>::empty().insert(akv) }
                        else { Set::<<T as View>::V>::empty() }));
                    // Structural properties for join calls below.
                    assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                    // Cross-ordering for join_pair_inner: lrv < ak < rrv.
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        if lrv.contains(s@) && rrv.contains(o@) {
                            lemma_cmp_antisymmetry(o, ak);
                            lemma_cmp_transitivity(s, ak, o);
                        }
                    };
                    // Length: lrv.len() + rrv.len() ≤ alv.len() + arv.len() < a@.len() < usize::MAX.
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                }
                if found {
                    join_with_priority(left_res, ak, ap, right_res)
                } else {
                    join_pair_inner(left_res, right_res)
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    fn difference_inner<T: MtKey + ClonePreservesView>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (remaining: ParamTreap<T>)
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
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    // blv ⊆ bv and brv ⊆ bv (from split partition).
                    assert(blv.subset_of(bv)) by {
                        assert forall|x: T::V| #[trigger] blv.contains(x) implies bv.contains(x) by {
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    assert(brv.subset_of(bv)) by {
                        assert forall|x: T::V| #[trigger] brv.contains(x) implies bv.contains(x) by {
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    // Ordering foralls while exec vars are live (before closures move them).
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                    assert forall|t: T| #[trigger] blv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] brv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                }
                let f1 = move || -> (diff: ParamTreap<T>)
                    ensures diff@.finite(), diff@ == al@.difference(bl@)
                { difference_inner(&al, &bl) };
                let f2 = move || -> (diff: ParamTreap<T>)
                    ensures diff@.finite(), diff@ == ar@.difference(br@)
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
                            if x == akv {
                                assert(!found);
                            } else if alv.contains(x) {
                                // x ∉ bv and blv ⊆ bv → x ∉ blv.
                                assert(!blv.contains(x));
                                assert(lrv.contains(x));
                            } else {
                                assert(!brv.contains(x));
                                assert(rrv.contains(x));
                            }
                        } else {
                            if lrv.contains(x) {
                                assert(alv.contains(x) && !blv.contains(x));
                                // x ∈ alv → x < ak. x ∉ blv. If x ∈ bv: x ∈ bv.remove(akv) = blv.union(brv).
                                // x ∈ brv → x > ak. But x < ak. Contradiction. So x ∈ blv. But !blv. ∴ x ∉ bv.
                                if bv.contains(x) {
                                    assert(blv.union(brv).contains(x));
                                    if brv.contains(x) {
                                        let ghost t_al = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent(t_al, t_br, ak);
                                        assert(false);
                                    }
                                    assert(false);
                                }
                            } else if rrv.contains(x) {
                                assert(arv.contains(x) && !brv.contains(x));
                                // x ∈ arv → x > ak. x ∉ brv. If x ∈ bv: x ∈ blv.union(brv). x ∈ blv → x < ak. Contradiction.
                                if bv.contains(x) {
                                    assert(blv.union(brv).contains(x));
                                    if blv.contains(x) {
                                        let ghost t_ar = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent_right(t_bl, t_ar, ak);
                                        assert(false);
                                    }
                                    assert(false);
                                }
                            } else if !found && x == akv {
                                assert(av.contains(akv));
                            }
                        }
                    };
                    assert(av.difference(bv) =~= lrv.union(rrv).union(
                        if !found { Set::<<T as View>::V>::empty().insert(akv) }
                        else { Set::<<T as View>::V>::empty() }));
                    // Structural properties (from ambient ordering foralls).
                    assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&ak) == Less by {};
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&ak) == Greater by {};
                    // lrv ⊆ alv < ak, rrv ⊆ arv > ak.
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        if lrv.contains(s@) && rrv.contains(o@) {
                            lemma_cmp_antisymmetry(o, ak);
                            lemma_cmp_transitivity(s, ak, o);
                        }
                    };
                    // Length: lrv.len() + rrv.len() ≤ alv.len() + arv.len() < a@.len() < usize::MAX.
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                }
                if found {
                    join_pair_inner(left_res, right_res)
                } else {
                    join_with_priority(left_res, ak, ap, right_res)
                }
            }
        }
    }

    // Sequential (non-parallel) filter: carries Ghost(spec_pred) through recursion to prove
    // full predicate semantics. Parallel closures cannot capture Ghost<spec_fn(T::V) -> bool>
    // because T::V is a ghost type that does not implement Send. Sequential recursion avoids
    // the Send constraint entirely while preserving correctness.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    fn filter_inner<T: MtKey + ClonePreservesView, F: Pred<T>>(
        tree: &ParamTreap<T>,
        predicate: &Arc<F>,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            forall|t: &T| #[trigger] ((**predicate).requires((t,))),
            forall|x: T, keep: bool| #[trigger] (**predicate).ensures((&x,), keep)
                ==> keep == spec_pred(x@),
            tree@.len() < usize::MAX as nat,
        ensures
            filtered@.finite(),
            filtered@.subset_of(tree@),
            forall|v: T::V| #[trigger] filtered@.contains(v)
                ==> tree@.contains(v) && spec_pred(v),
            forall|v: T::V| #[trigger] tree@.contains(v) && spec_pred(v)
                ==> filtered@.contains(v),
        decreases tree@.len(),
    {
        proof { use_type_invariant(tree); }
        match expose_internal(tree) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let ap = tree_priority_internal(tree);
                let ghost lv = left@;
                let ghost rv = right@;
                let ghost kv = key@;
                let ghost tv = tree@;
                // Termination: subtrees are strictly smaller than tree.
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    assert(lv.len() + rv.len() < tv.len());
                }
                // Sequential recursive calls.
                let left_filtered = filter_inner(&left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner(&right, predicate, Ghost(spec_pred));
                proof { use_type_invariant(&left_filtered); use_type_invariant(&right_filtered); }
                proof {
                    // BST ordering across the two filtered partitions.
                    assert forall|s: T, o: T| #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                        left_filtered@.contains(s@) && right_filtered@.contains(o@) implies s.cmp_spec(&o) == Less by {
                        lemma_cmp_antisymmetry(o, key);
                        lemma_cmp_transitivity(s, key, o);
                    };
                    // Length bounds for join preconditions.
                    vstd::set_lib::lemma_len_subset(left_filtered@, lv);
                    vstd::set_lib::lemma_len_subset(right_filtered@, rv);
                    assert(left_filtered@.len() + right_filtered@.len() < usize::MAX as nat);
                }
                let keep = (**predicate)(&key);
                if keep {
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        // Forward: lf.union(rf).insert(kv).contains(v) ==> spec_pred(v).
                        assert forall|v: T::V| #[trigger]
                            lf.union(rf).insert(kv).contains(v) implies spec_pred(v) by {
                            if v == kv {
                            } else if lf.contains(v) {
                                assert(left_filtered@.contains(v));
                            } else {
                                assert(right_filtered@.contains(v));
                            }
                        };
                        // Backward: tv.contains(v) && spec_pred(v) ==> lf.union(rf).insert(kv).contains(v).
                        assert forall|v: T::V| #[trigger]
                            tv.contains(v) && spec_pred(v)
                            implies lf.union(rf).insert(kv).contains(v) by {
                            if v == kv {
                                // kv is in the insert.
                            } else {
                                if lv.contains(v) {
                                    assert(left_filtered@.contains(v));
                                } else {
                                    assert(right_filtered@.contains(v));
                                }
                            }
                        };
                    }
                    join_with_priority(left_filtered, key, ap, right_filtered)
                } else {
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        // Forward: lf.union(rf).contains(v) ==> spec_pred(v).
                        assert forall|v: T::V| #[trigger]
                            lf.union(rf).contains(v) implies spec_pred(v) by {
                            if lf.contains(v) {
                                assert(left_filtered@.contains(v));
                            } else {
                                assert(right_filtered@.contains(v));
                            }
                        };
                        // Backward: tv.contains(v) && spec_pred(v) ==> lf.union(rf).contains(v).
                        // kv is excluded; spec_pred(kv) = keep = false vacuously handles v == kv.
                        assert forall|v: T::V| #[trigger]
                            tv.contains(v) && spec_pred(v)
                            implies lf.union(rf).contains(v) by {
                            if v == kv {
                            } else {
                                if lv.contains(v) {
                                    assert(left_filtered@.contains(v));
                                } else {
                                    assert(right_filtered@.contains(v));
                                }
                            }
                        };
                    }
                    join_pair_inner(left_filtered, right_filtered)
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(log^2 n)
    fn filter_parallel<T: MtKey + ClonePreservesView, F: Pred<T>>(
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
        filter_inner(tree, &predicate, Ghost(spec_pred))
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn reduce_inner<T: MtKey + ClonePreservesView, F>(tree: &ParamTreap<T>, op: &Arc<F>, identity: T) -> (reduced: T)
    where
        F: Fn(T, T) -> T + Send + Sync + 'static,
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            tree@.finite(),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> reduced == identity,
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
    fn reduce_parallel<T: MtKey + ClonePreservesView, F>(tree: &ParamTreap<T>, op: F, base: T) -> (reduced: T)
    where
        F: Fn(T, T) -> T + Send + Sync + 'static,
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> reduced == base,
    {
        proof { use_type_invariant(tree); }
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn collect_in_order<T: MtKey + ClonePreservesView>(tree: &ParamTreap<T>, out: &mut Vec<T>)
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

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub enum Exposed<T: MtKey> {
        Leaf,
        Node(ParamTreap<T>, T, ParamTreap<T>),
    }

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct NodeInner<T: MtKey> {
        pub key: T,
        pub priority: i64,
        pub size: usize,
        pub left: ParamTreap<T>,
        pub right: ParamTreap<T>,
    }

    //		Section 4d. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct ParamTreap<T: MtKey> {
        pub(crate) root: RwLock<Option<Box<NodeInner<T>>>, BSTParaTreapMtEphInv<T>>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    //		Section 5d. view impls


    impl<T: MtKey> View for ParamTreap<T> {
        type V = Set<T::V>;
        open spec fn view(&self) -> Set<T::V> { self.spec_ghost_locked_root() }
    }

    //		Section 8d. traits


    pub trait ParamTreapTrait<T: MtKey + ClonePreservesView>: Sized + View<V = Set<T::V>> {
        spec fn spec_bstparatreapmteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn new() -> (tree: Self)
            ensures tree@.finite(), tree@.len() == 0, tree.spec_bstparatreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
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
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — matches APAS
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
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn size(&self) -> (count: usize)
            ensures self@.finite(), count == self@.len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn is_empty(&self) -> (empty: bool)
            ensures self@.finite(), empty == (self@.len() == 0);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn insert(&mut self, key: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.insert(key@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn delete(&mut self, key: &T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.remove(key@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn find(&self, key: &T) -> (found: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                found matches Some(v) ==> v@ == key@ && self@.contains(v@),
                found is None ==> !self@.contains(key@);
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
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
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — matches APAS
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
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n) — matches APAS
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@.finite(), combined@ == self@.union(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n) — matches APAS
        fn intersect(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures common@.finite(), common@ == self@.intersect(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n) — matches APAS
        fn difference(&self, other: &Self) -> (diff: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures diff@.finite(), diff@ == self@.difference(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(lg |t|) — matches APAS
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
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(lg |t|) — matches APAS
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(|t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(|t|) — matches APAS
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures self@.finite(), ordered.spec_len() == self@.len();
    }

    //		Section 9d. impls


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

    impl<T: MtKey + ClonePreservesView> ParamTreapTrait<T> for ParamTreap<T> {
        open spec fn spec_bstparatreapmteph_wf(&self) -> bool { self@.finite() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self) { new_param_treap(None, Ghost(Set::empty())) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn expose(&self) -> (exposed: Exposed<T>) { expose_internal(self) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool) { self.size() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
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
                    match <T as std::cmp::Ord>::cmp(key, &root_key) {
                        | Equal => Some(root_key),
                        | Less => left.find(key),
                        | Greater => right.find(key),
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn split(&self, key: &T) -> (parts: (Self, bool, Self)) {
            proof { use_type_invariant(self); }
            split_inner(self, key)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn join_pair(&self, other: Self) -> (joined: Self) {
            join_pair_inner(self.clone(), other)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn union(&self, other: &Self) -> (combined: Self) { union_inner(self, other) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn intersect(&self, other: &Self) -> (common: Self) { intersect_inner(self, other) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn difference(&self, other: &Self) -> (diff: Self) { difference_inner(self, other) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
        fn filter<F: Pred<T>>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self) { filter_parallel(self, predicate, Ghost(spec_pred)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
        { reduce_parallel(self, op, base) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>) {
            let mut out = Vec::with_capacity(self.size());
            collect_in_order(self, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    //		Section 11a. top level coarse locking


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

    //		Section 12b. derive impls in verus!


    impl<T: MtKey + ClonePreservesView> Clone for Exposed<T> {
        fn clone(&self) -> (cloned: Self)
            ensures true
        {
            match self {
                Exposed::Leaf => Exposed::Leaf,
                Exposed::Node(l, k, r) => Exposed::Node(l.clone(), k.clone(), r.clone()),
            }
        }
    }

    //		Section 12c. derive impls in verus!


    impl<T: MtKey + ClonePreservesView> Clone for NodeInner<T> {
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

    //		Section 12d. derive impls in verus!


    impl<T: MtKey + ClonePreservesView> Clone for ParamTreap<T> {
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
    } // verus!

    //		Section 13. macros


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

    //		Section 14a. derive impls outside verus!

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

    //		Section 14b. derive impls outside verus!

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

    //		Section 14c. derive impls outside verus!

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

    //		Section 14d. derive impls outside verus!

    // Ghost<Set<T::V>> contains FnSpec (PhantomData at runtime), which lacks Send/Sync.
    // ParamTreap is safe to send/share: the Ghost field is erased at runtime.
    unsafe impl<T: MtKey> Send for ParamTreap<T> {}
    unsafe impl<T: MtKey> Sync for ParamTreap<T> {}

    impl<T: MtKey + ClonePreservesView> fmt::Debug for ParamTreap<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreap(size: {})", self.size())
        }
    }

    impl<T: MtKey + ClonePreservesView> fmt::Display for ParamTreap<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreap(size: {})", self.size())
        }
    }
}
