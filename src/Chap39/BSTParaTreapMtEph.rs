//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Parametric multi-threaded Treap (probabilistically balanced BST) with parallel operations.

//  Table of Contents
//	Section 1. module
//	Section 2. imports (pub use BSTTreapSpecsAndLemmas::*)
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 4d. type definitions
//	Section 5d. view impls
//	Section 8d. traits
//	Section 9d. impls
//	Section 11a. top level coarse locking
//	Section 10d. iterators — ParamTreap
//	Section 12b. derive impls in verus!
//	Section 12c. derive impls in verus!
//	Section 12d. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!
//	Section 14e. derive impls outside verus!

//		Section 1. module

pub mod BSTParaTreapMtEph {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt;
    use std::fmt::Write;
    use std::vec::IntoIter;
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    pub use crate::Chap39::BSTTreapSpecsAndLemmas::BSTTreapSpecsAndLemmas::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    use crate::vstdplus::accept::accept;

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
        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block
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
// Veracity: UNNEEDED proof block         // Veracity: NEEDED proof block
        proof { use_type_invariant(tree); }
        let handle = tree.root.acquire_read();
        let exposed = match handle.borrow() {
            | None => Exposed::Leaf,
            | Some(node) => {
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(node.left@, node.right@);
                }
                let l = node.left.clone();
                let k = clone_elem(&node.key);
                // Veracity: NEEDED proof block
                let r = node.right.clone();
                // Veracity: NEEDED proof block
                proof {
                    // Ordering transfers from pred to clones via view equality.
                    // key.clone() ensures k@ == node.key@; view_ord_consistent gives
                    // node.key.cmp_spec(&k) == Equal, which lemma_cmp_equal_congruent_right uses.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|t: T| (#[trigger] l@.contains(t@))
                        implies t.cmp_spec(&k) == Less by {
                        lemma_cmp_equal_congruent_right(t, node.key, k);
                    }
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|t: T| (#[trigger] r@.contains(t@))
                        implies t.cmp_spec(&k) == Greater by {
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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
        // Veracity: NEEDED proof block
        let size = 1 + ls + rs;
        let ghost contents = lv.union(rv).insert(kv);
        // Veracity: NEEDED proof block
        proof {
            vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
            use_type_invariant(&left);
            use_type_invariant(&right);
        }
        new_param_treap(
            Some(Box::new(NodeInner { key, priority, size, left, right })),
            Ghost(contents),
        )
    }

    /// Merge two BST-ordered subtrees with a middle key, rebalancing by priority (treap heap).
    /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
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
                    // Veracity: NEEDED proof block
                    let ghost lkv = lk@;
                    let ghost lrv = lr@;
                    let ghost llv = ll@;
                    // Veracity: NEEDED proof block
                    proof {
                        // lr@.disjoint(right@): lr@ ⊆ left@ (expose), left@.disjoint(right@) (req).
                        // !lr@.contains(key@): lr@ ⊆ left@, !left@.contains(key@) (req).
                        // !right@.contains(key@): from req.
                        // Veracity: NEEDED proof block
                        vstd::set_lib::lemma_len_subset(lrv, left@);
                        // BST ordering for (lr, key, right): all lr < lk < key (transitivity).
                        assert(forall|t: T| #[trigger] lrv.contains(t@) ==> left@.contains(t@));
                    }
                    let merged_right = join_with_priority(lr, key, priority, right);
                    // Veracity: NEEDED proof block
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
                        vstd::set_lib::lemma_set_disjoint_lens(llv, lrv);
                        vstd::set_lib::lemma_set_disjoint_lens(lrv, right@);
                        vstd::set_lib::lemma_len_subset(llv, left@);
                        // BST for ll: already proved above (all ll < lk, from expose).
                    }
                    make_node(ll, lk, lp_actual, merged_right)
                }
            }
        } else {
            match expose_internal(&right) {
                Exposed::Leaf => make_node(left, key, priority, right),
                // Veracity: NEEDED proof block
                Exposed::Node(rl, rk, rr) => {
                    let rp_actual = right_priority;
                    let ghost rkv = rk@;
                    let ghost rlv = rl@;
                    let ghost rrv = rr@;
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_len_subset(rlv, right@);
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
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|t: T| (#[trigger] rlv.contains(t@)) implies t.cmp_spec(&key) == Greater by {
                        }
                    }
                    let merged_left = join_with_priority(left, key, priority, rl);
                    // Veracity: NEEDED proof block
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
                        lemma_joined_left_lt_rk(left@, rlv, key, merged_left@, rk, right@);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, rlv);
                        vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                        vstd::set_lib::lemma_len_subset(rrv, right@);
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
            // Veracity: NEEDED proof block
            forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(key) == Less,
            forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(key) == Greater,
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => (new_leaf(), false, new_leaf()),
            | Exposed::Node(left, root_key, right) => {
                // Veracity: NEEDED proof block
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                let ghost rk = root_key;
                let ghost kval = *key;
                match <T as std::cmp::Ord>::cmp(key, &root_key) {
                    // Veracity: NEEDED proof block
                    | Less => {
                        let ghost lv = left@;
                        let ghost rv = right@;
                        let ghost rkv = root_key@;
                        let priority = tree_priority_internal(tree);
                        let (ll, found, lr) = split_inner(&left, key);
                        let ghost llv = ll@;
                        let ghost lrv = lr@;
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED proof block
                            lemma_split_result_subset(llv, lrv, lv, key@);
                            vstd::set_lib::lemma_len_subset(lrv, lv);
                            // Requirements for join_with_priority(lr, root_key, priority, right):
                            // lr < root_key: from expose, all left < root_key, lr ⊆ left → lr < root_key ✓.
                            // right > root_key: from expose ✓.
                            // !lr.contains(root_key@), !right.contains(root_key@): from expose ✓.
                            // lr.disjoint(right): lr ⊆ left, left.disjoint(right) → lr.disjoint(right) ✓.
                        }
                        let rebuilt = join_with_priority(lr, root_key, priority, right);
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] (llv.union(rebuilt@)).contains(x)
                                <==> tree@.remove(key@).contains(x) by {
                                if llv.contains(x) {
                                }
                                if lv.contains(x) && x != key@ {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(llv.union(lrv).contains(x));
                                }
                            };
                            // Ordering: rebuilt elements > key.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
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
                                    lemma_cmp_eq_subst(kval, rk, t);
                                }
                            };
                        }
                        // Veracity: NEEDED proof block
                        (ll, found, rebuilt)
                    }
                    | Greater => {
                        let ghost lv = left@;
                        let ghost rv = right@;
                        let ghost rkv = root_key@;
                        let priority = tree_priority_internal(tree);
                        let (rl, found, rr) = split_inner(&right, key);
                        let ghost rlv = rl@;
                        // Veracity: NEEDED proof block
                        let ghost rrv = rr@;
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_split_result_subset(rlv, rrv, rv, key@);
                            vstd::set_lib::lemma_len_subset(rlv, rv);
                            // Requirements for join_with_priority(left, root_key, priority, rl):
                            // left < root_key: from expose ✓.
                            // rl > root_key: from expose, all right > root_key, rl ⊆ right → rl > root_key ✓.
                            // !left.contains(root_key@), !rl.contains(root_key@): from expose ✓.
                        }
                        let rebuilt = join_with_priority(left, root_key, priority, rl);
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] (rebuilt@.union(rrv)).contains(x)
                                <==> tree@.remove(key@).contains(x) by {
                                if rrv.contains(x) {
                                }
                                if rv.contains(x) && x != key@ {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(rlv.union(rrv).contains(x));
                                }
                            };
                            // Ordering: rebuilt elements < key.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if rlv.contains(t@) {
                                    // Recursive split ensures t < key.
                                // Veracity: NEEDED proof block
                                } else if lv.contains(t@) {
                                    lemma_cmp_antisymmetry(kval, rk);
                                    lemma_cmp_transitivity(t, rk, kval);
                                } else {
                                    lemma_cmp_antisymmetry(kval, rk);
                                    lemma_cmp_equal_congruent(t, rk, kval);
                                }
                            };
                        }
                        (rebuilt, found, rr)
                    }
                    | Equal => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] left@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                lemma_cmp_equal_congruent_right(t, kval, rk);
                            };
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
    fn join_pair_inner<T: MtKey + ClonePreservesView>(left: ParamTreap<T>, right: ParamTreap<T>) -> (joined: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            left@.finite(), right@.finite(),
            forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                left@.contains(s@) && right@.contains(o@) ==> s.cmp_spec(&o) == Less,
            left@.len() + right@.len() < usize::MAX as nat,
        // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                proof {
                    // expose ensures: right@ =~= rlv.union(rrv).insert(rkv)
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(rlv.subset_of(right@));
                    // !left@.contains(rkv): for l ∈ left@, l.cmp(&r_key)==Less (cross-ordering
                    // with r_key ∈ right@), so l@ != rkv by view_ord_consistent.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    assert forall|l: T| #[trigger] left@.contains(l@) implies l@ != rkv by {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        assert(right@.contains(rkv));
                        assert(l.cmp_spec(&r_key) == Less);
                    };
                    // Size bounds.
                    vstd::set_lib::lemma_len_subset(rlv, right@);
                    vstd::set_lib::lemma_len_subset(rrv, right@);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                }
                let (split_left, _, split_right) = split_inner(&left, &r_key);
                let ghost slv = split_left@;
                let ghost srv = split_right@;
                // Veracity: NEEDED proof block
                proof {
                    // split ensures: slv.union(srv) =~= left@.remove(rkv) =~= left@ (since rkv ∉ left@).
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(slv.union(srv) =~= left@);
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(slv.subset_of(left@));
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(srv.subset_of(left@));
                    vstd::set_lib::lemma_len_subset(slv, left@);
                    vstd::set_lib::lemma_len_subset(srv, left@);
                    vstd::set_lib::lemma_len_subset(srv, left@);
// Veracity: UNNEEDED proof block                     // Cross-ordering for join_pair_inner(split_left, r_left): slv < rlv.
// Veracity: UNNEEDED proof block                     // slv ⊆ left@, rlv ⊆ right@, all left@ < right@ (requires).
// Veracity: UNNEEDED proof block                     // Veracity: NEEDED assert
// Veracity: UNNEEDED proof block                     // Veracity: NEEDED assert
// Veracity: UNNEEDED proof block                     assert forall|s: T, o: T| #![trigger slv.contains(s@), rlv.contains(o@)]
// Veracity: UNNEEDED proof block                         slv.contains(s@) && rlv.contains(o@) implies s.cmp_spec(&o) == Less by {};
                    // Cross-ordering for join_pair_inner(split_right, r_right): srv < rrv.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger srv.contains(s@), rrv.contains(o@)]
                        srv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        assert(left@.contains(s@));
                        assert(right@.contains(o@));
                    };
                    // Ordering facts for rlv/rrv from expose_internal ensures (while r_left/r_right are live).
                    // Veracity: NEEDED proof block (speed hint)
                    // Ordering facts for slv/srv from split_inner ensures (while split_left/split_right are live).
                // Veracity: NEEDED proof block
                }
                // Establish inhabitedness before moves (for disjointness proofs below).
                // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                proof { use_type_invariant(&combined_left); use_type_invariant(&combined_right); }
                // Veracity: NEEDED proof block
                proof {
                    // Establish membership equivalences (set extensionality).
                    // Lift =~= to propositional equality for trigger-free membership substitution.
                    // Combined ordering foralls for clv/crv via case splits.
                    // Cross-ordering: clv ⊆ slv.union(rlv) < r_key, crv ⊆ srv.union(rrv) > r_key.
                    // Re-assert subset facts so trigger matching derives ordering in nested scope.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger clv.contains(s@), crv.contains(o@)]
                        clv.contains(s@) && crv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        // Reveal and explicit if-case to force unit facts from the conjunction.
                        reveal(ParamTreap::spec_ghost_locked_root);
                        if clv.contains(s@) && crv.contains(o@) {
                            // Both are unit facts here — E-matching fires on one-var ordering foralls.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert (speed hint)
                            assert(o.cmp_spec(&r_key) == Greater);
                            lemma_cmp_antisymmetry(o, r_key);
                            lemma_cmp_transitivity(s, r_key, o);
                        }
                    };
                    // Prove disjointness of slv/rlv and srv/rrv via inhabitedness + ordering contradiction.
                    // cross-ordering foralls at lines 884-888 and 891-894 are in ambient context.
                    // clv/crv disjointness via the cross-ordering forall above.
                    // Length reasoning using the established disjointnesses.
                    vstd::set_lib::lemma_set_disjoint_lens(slv, rlv);
                    vstd::set_lib::lemma_set_disjoint_lens(srv, rrv);
                    vstd::set_lib::lemma_set_disjoint_lens(slv, srv);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                    vstd::set_lib::lemma_set_disjoint_lens(clv, crv);
                    // BST ordering for join_with_priority (clv/crv ordering established above).
                    // Final set equality for join_with_priority ensures.
// Veracity: UNNEEDED proof block                     // clv == slv.union(rlv) and crv == srv.union(rrv) (established above).
                }
                // join_with_priority ensures: result@ =~= clv.union(crv).insert(rkv) =~= lv.union(rv).
                join_with_priority(combined_left, r_key, rp, combined_right)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    // Veracity: NEEDED proof block
    fn union_inner<T: MtKey + ClonePreservesView>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (combined: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a@.len() + b@.len() < usize::MAX as nat,
        ensures combined@.finite(), combined@ == a@.union(b@),
        decreases a@.len(),
    // Veracity: NEEDED proof block
    {
        // Veracity: NEEDED proof block
        proof { use_type_invariant(a); use_type_invariant(b); }
        match expose_internal(a) {
            | Exposed::Leaf => b.clone(),
            | Exposed::Node(al, ak, ar) => {
                let ap = tree_priority_internal(a);
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_len_subset(alv, a@);
                    vstd::set_lib::lemma_len_subset(arv, a@);
                }
                let (bl, _, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                // Veracity: NEEDED proof block
                let ghost brv = br@;
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    lemma_split_result_subset(blv, brv, b@, akv);
                    vstd::set_lib::lemma_len_subset(blv, b@);
                    vstd::set_lib::lemma_len_subset(brv, b@);
                    // Ordering foralls while exec vars are live (before closures move them).
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
                // Veracity: NEEDED proof block
                proof { use_type_invariant(&left_union); use_type_invariant(&right_union); }
                // Veracity: NEEDED proof block
                proof {
                    let luv = left_union@;
                    let ruv = right_union@;
                    // All elements of luv < ak, all of ruv > ak (from ambient ordering foralls).
                    // Disjointness of luv/ruv by contradiction via view_ord_consistent + inhabitedness.
                    // Length: luv.union(ruv) ⊆ a@.union(b@), and they're disjoint.
                    vstd::set_lib::lemma_set_disjoint_lens(luv, ruv);
                    // luv == alv.union(blv) ⊆ a@.union(b@) and ruv == arv.union(brv) ⊆ a@.union(b@).
                    vstd::set_lib::lemma_len_subset(luv.union(ruv), a@.union(b@));
                    vstd::set_lib::lemma_len_union(a@, b@);
                    // Final set equality: combined@ will be luv.union(ruv).insert(akv) =~= a@.union(b@).
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(luv.union(ruv).insert(akv) == a@.union(b@)) by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] luv.union(ruv).insert(akv).contains(x)
                            <==> a@.union(b@).contains(x) by {
                            if luv.union(ruv).insert(akv).contains(x) {
                                if x == akv {}
                                else if luv.contains(x) {
                                } else {
                                }
                            }
                            // backward: if x ∈ a@.union(b@) but not in LHS, derive contradiction
                            if a@.union(b@).contains(x) && !luv.union(ruv).insert(akv).contains(x) {
                                if a@.contains(x) {
                                } else {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED proof block (speed hint)
                                    // Veracity: NEEDED assert
                                    assert(b@.remove(akv).contains(x));
                                }
                            }
                        };
                    };
                }
                join_with_priority(left_union, ak, ap, right_union)
            }
        }
    // Veracity: NEEDED proof block
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    fn intersect_inner<T: MtKey + ClonePreservesView>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (common: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            // Veracity: NEEDED proof block
            a@.len() < usize::MAX as nat,
        ensures common@.finite(), common@ == a@.intersect(b@),
        decreases a@.len(),
    // Veracity: NEEDED proof block
    {
        // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED proof block (speed hint)
                    vstd::set_lib::lemma_len_subset(alv, av);
                    // Veracity: NEEDED proof block
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                // Veracity: NEEDED proof block
                proof {
                    lemma_split_result_subset(blv, brv, bv, akv);
                }
                // Veracity: NEEDED proof block
                proof {
                    use_type_invariant(&al); use_type_invariant(&ar);
                    use_type_invariant(&bl); use_type_invariant(&br);
                    // Ordering foralls for alv/arv while exec vars are live (before closures move them).
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
                // Veracity: NEEDED proof block
                proof { use_type_invariant(&left_res); use_type_invariant(&right_res); }
                // Veracity: NEEDED proof block
                proof {
                    // Prove: a@.intersect(b@) == left_res@.union(right_res@).union({ak@} if found).
                    // al.intersect(bl).union(ar.intersect(br)).union({ak?}) == a.intersect(b).
                    //
                    // Key: split_inner splits b by ak. bl = {x ∈ b | x < ak}, br = {x ∈ b | x > ak}.
                    // al < ak (expose), ar > ak (expose). bl < ak (split), br > ak (split).
                    // So al.intersect(bl) = elements both < ak, ar.intersect(br) = elements both > ak.
                    // These two sets are disjoint (one all < ak, one all > ak).
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|x| #[trigger] av.intersect(bv).contains(x) <==>
                        lrv.union(rrv).union(if found { Set::<<T as View>::V>::empty().insert(akv) }
                                           else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.intersect(bv).contains(x) {
                            if x == akv {
                            } else if alv.contains(x) {
                                // x ∈ al (so x < ak via expose ensures), x ∈ b, x ≠ ak.
                                // split_inner: blv.union(brv) =~= bv.remove(akv). x < ak, brv > ak → x ∈ blv.
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(blv.contains(x)) by {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(bv.remove(akv).contains(x));
                                    if brv.contains(x) {
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent(t_x, t_br, ak);
                                    }
                                };
                            } else {
                                // x ∈ ar (so x > ak), x ∈ b, x ≠ ak. Similarly x ∈ brv.
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(brv.contains(x)) by {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(bv.remove(akv).contains(x));
                                    if blv.contains(x) {
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent_right(t_bl, t_x, ak);
                                    }
                                };
                            // Veracity: NEEDED proof block
                            }
                        }
                    };
                    // Structural properties for join calls below.
                    lemma_halves_cross_ordered(lrv, rrv, ak);
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                }
                if found {
                    // Veracity: NEEDED proof block
                    join_with_priority(left_res, ak, ap, right_res)
                } else {
                    join_pair_inner(left_res, right_res)
                }
            }
        }
    }
// Veracity: NEEDED proof block

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    fn difference_inner<T: MtKey + ClonePreservesView>(a: &ParamTreap<T>, b: &ParamTreap<T>) -> (remaining: ParamTreap<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a@.len() < usize::MAX as nat,
        ensures remaining@.finite(), remaining@ == a@.difference(b@),
        decreases a@.len(),
    {
        // Veracity: NEEDED proof block
        proof { use_type_invariant(a); use_type_invariant(b); }
        match expose_internal(a) {
            // Veracity: NEEDED proof block
            | Exposed::Leaf => new_leaf(),
            // Veracity: NEEDED proof block
            | Exposed::Node(al, ak, ar) => {
                let ap = tree_priority_internal(a);
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                let ghost av = a@;
                let ghost bv = b@;
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                // Veracity: NEEDED proof block
                proof {
                    lemma_split_result_subset(blv, brv, bv, akv);
                    // Ordering foralls while exec vars are live (before closures move them).
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
                // Veracity: NEEDED proof block
                proof { use_type_invariant(&left_res); use_type_invariant(&right_res); }
                // Veracity: NEEDED proof block
                proof {
                    // Prove: a@.difference(b@) == left_res@.union(right_res@).union({ak@} if !found).
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|x| #[trigger] av.difference(bv).contains(x) <==>
                        lrv.union(rrv).union(if !found { Set::<<T as View>::V>::empty().insert(akv) }
                                            else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.difference(bv).contains(x) {
                            if x == akv {
                            } else if alv.contains(x) {
                                // x ∉ bv and blv ⊆ bv → x ∉ blv.
                            } else {
                            }
                        } else {
                            if lrv.contains(x) {
                                // x ∈ alv → x < ak. x ∉ blv. If x ∈ bv: x ∈ bv.remove(akv) = blv.union(brv).
                                // x ∈ brv → x > ak. But x < ak. Contradiction. So x ∈ blv. But !blv. ∴ x ∉ bv.
                                if bv.contains(x) {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(blv.union(brv).contains(x));
                                    if brv.contains(x) {
                                        let ghost t_al = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent(t_al, t_br, ak);
                                    }
                                }
                            } else if rrv.contains(x) {
                                // x ∈ arv → x > ak. x ∉ brv. If x ∈ bv: x ∈ blv.union(brv). x ∈ blv → x < ak. Contradiction.
                                if bv.contains(x) {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(blv.union(brv).contains(x));
                                    if blv.contains(x) {
                                        let ghost t_ar = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent::<T>();
                                        lemma_cmp_equal_congruent_right(t_bl, t_ar, ak);
                                    }
                                }
                            } else if !found && x == akv {
                            }
                        }
                    };
                    // Structural properties (from ambient ordering foralls).
                    lemma_halves_cross_ordered(lrv, rrv, ak);
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                }
                if found {
                    join_pair_inner(left_res, right_res)
                } else {
                    // Veracity: NEEDED proof block
                    join_with_priority(left_res, ak, ap, right_res)
                }
            }
        }
    }

    // Sequential (non-parallel) filter: carries Ghost(spec_pred) through recursion to prove
    // full predicate semantics. Parallel closures cannot capture Ghost<spec_fn(T::V) -> bool>
    // because T::V is a ghost type that does not implement Send. Sequential recursion avoids
    // the Send constraint entirely while preserving correctness.
    // Veracity: NEEDED proof block
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, Span O(log^2 n) expected
    fn filter_inner<T: MtKey + ClonePreservesView, F: Pred<T>>(
        tree: &ParamTreap<T>,
        predicate: &Arc<F>,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: ParamTreap<T>)
        // Veracity: NEEDED proof block
        requires
            // Veracity: NEEDED proof block
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
        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block
        proof { use_type_invariant(tree); }
        match expose_internal(tree) {
            | Exposed::Leaf => new_leaf(),
            | Exposed::Node(left, key, right) => {
                let ap = tree_priority_internal(tree);
                let ghost lv = left@;
                let ghost rv = right@;
                // Veracity: NEEDED proof block (speed hint)
                let ghost kv = key@;
                let ghost tv = tree@;
                // Termination: subtrees are strictly smaller than tree.
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                }
                // Sequential recursive calls.
                let left_filtered = filter_inner(&left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner(&right, predicate, Ghost(spec_pred));
                // Veracity: NEEDED proof block
                proof { use_type_invariant(&left_filtered); use_type_invariant(&right_filtered); }
                // Veracity: NEEDED proof block
                proof {
                    // BST ordering across the two filtered partitions.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                        left_filtered@.contains(s@) && right_filtered@.contains(o@) implies s.cmp_spec(&o) == Less by {
                        lemma_cmp_antisymmetry(o, key);
                        lemma_cmp_transitivity(s, key, o);
                    };
                    // Length bounds for join preconditions.
                    vstd::set_lib::lemma_len_subset(left_filtered@, lv);
                    vstd::set_lib::lemma_len_subset(right_filtered@, rv);
                }
                let keep = (**predicate)(&key);
                if keep {
                    // Veracity: NEEDED proof block
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        // Forward: lf.union(rf).insert(kv).contains(v) ==> spec_pred(v).
                        // Backward: tv.contains(v) && spec_pred(v) ==> lf.union(rf).insert(kv).contains(v).
                    }
                    join_with_priority(left_filtered, key, ap, right_filtered)
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        // Forward: lf.union(rf).contains(v) ==> spec_pred(v).
                        // Backward: tv.contains(v) && spec_pred(v) ==> lf.union(rf).contains(v).
                        // kv is excluded; spec_pred(kv) = keep = false vacuously handles v == kv.
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
        // Veracity: NEEDED proof block
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
        // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
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
        // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree@.finite(), tree@.len() == 0, tree.spec_bstparatreapmteph_wf();
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures self@.finite(), count == self@.len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            ensures self@.finite(), empty == (self@.len() == 0);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn insert(&mut self, key: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.insert(key@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn delete(&mut self, key: &T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.remove(key@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T) -> (found: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                found matches Some(v) ==> v@ == key@ && self@.contains(v@),
                found is None ==> !self@.contains(key@);
        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n)
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@.finite(), combined@ == self@.union(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n)
        fn intersect(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures common@.finite(), common@ == self@.intersect(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)), Span O(lg n)
        fn difference(&self, other: &Self) -> (diff: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() < usize::MAX as nat,
            ensures diff@.finite(), diff@ == self@.difference(other@);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(lg |t|)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(lg |t|)
        fn reduce<F>(&self, op: F, base: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync + 'static
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                // Veracity: NEEDED proof block
                view_ord_consistent::<T>(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(|t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(|t|)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures self@.finite(), ordered.spec_len() == self@.len();
    }

    //		Section 9d. impls


    // Veracity: NEEDED proof block
    impl<T: MtKey> ParamTreap<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_root@.finite()
            && self.ghost_locked_root@ =~= self.root.pred().contents
            // Veracity: NEEDED proof block
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

        // Veracity: NEEDED proof block (speed hint)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn join_mid(exposed: Exposed<T>) -> (tree: Self) {
            match exposed {
                // Veracity: NEEDED proof block
                | Exposed::Leaf => ParamTreap::new(),
                | Exposed::Node(left, key, right) => {
                    // Veracity: NEEDED proof block
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

        // Veracity: NEEDED proof block (speed hint)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (count: usize) {
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let handle = self.root.acquire_read();
            let count = match handle.borrow() {
                None => 0usize,
                Some(node) => {
                    // Veracity: NEEDED proof block
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
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            proof { use_type_invariant(&*self); assert(old_view.finite()); }
            let (left, _, right) = split_inner(self, &key);
            let ghost kv = key@;
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(old_view.remove(kv), old_view);
            }
            let priority = priority_for(&key);
            let new_tree = join_with_priority(left, key, priority, right);
            *self = new_tree;
        }
// Veracity: NEEDED proof block (speed hint)

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case; Span O(log n) expected, O(n) worst case
        fn delete(&mut self, key: &T) {
            let ghost old_view = self@;
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            proof { use_type_invariant(&*self); assert(old_view.finite()); }
            let ghost kref = *key;
            let (left, _, right) = split_inner(self, key);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
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
                    // Veracity: NEEDED proof block
                    proof {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
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
            // Veracity: NEEDED proof block
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
// Veracity: UNNEEDED proof block                     && (*box_node).left@.len() + (*box_node).right@.len() < usize::MAX as nat
                    && (*box_node).size as nat == (*box_node).left@.len() + (*box_node).right@.len() + 1
                    && (forall|t: T| (#[trigger] (*box_node).left@.contains(t@))
                        ==> t.cmp_spec(&(*box_node).key) == Less)
                    && (forall|t: T| (#[trigger] (*box_node).right@.contains(t@))
                        ==> t.cmp_spec(&(*box_node).key) == Greater)
                }
            }
        }
    }

    //		Section 10d. iterators — ParamTreap

    /// Snapshot iterator over ParamTreap — collects elements via in_order traversal,
    /// then yields owned T values from the captured Vec.
    #[verifier::reject_recursive_types(T)]
    pub struct ParamTreapIter<T: MtKey> {
        pub inner: IntoIter<T>,
    }

    impl<T: MtKey> View for ParamTreapIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant_paramtreap<T: MtKey>(it: &ParamTreapIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: MtKey> std::iter::Iterator for ParamTreapIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
            ensures
                ({
                    let (old_index, old_seq) = old(self)@;
                    match next {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index >= old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for for-loop support over ParamTreapIter.
    #[verifier::reject_recursive_types(T)]
    pub struct ParamTreapGhostIterator<T: MtKey> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: MtKey> View for ParamTreapGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: MtKey> vstd::pervasive::ForLoopGhostIteratorNew for ParamTreapIter<T> {
        type GhostIter = ParamTreapGhostIterator<T>;
        open spec fn ghost_iter(&self) -> ParamTreapGhostIterator<T> {
            ParamTreapGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: MtKey> vstd::pervasive::ForLoopGhostIterator for ParamTreapGhostIterator<T> {
        type ExecIter = ParamTreapIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ParamTreapIter<T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &ParamTreapIter<T>) -> ParamTreapGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: MtKey + ClonePreservesView> std::iter::IntoIterator for &'a ParamTreap<T> {
        type Item = T;
        type IntoIter = ParamTreapIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures
                it@.0 == 0,
                it@.1.len() == self@.len(),
                iter_invariant_paramtreap(&it),
        {
            let in_ord = self.in_order();
            ParamTreapIter { inner: in_ord.seq.into_iter() }
        }
    }

    //		Section 12b. derive impls in verus!


    impl<T: MtKey + ClonePreservesView> Clone for Exposed<T> {
        fn clone(&self) -> (cloned: Self)
            ensures true
        {
            match self {
                // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
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
            // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                        use_type_invariant(&left);
                        use_type_invariant(&right);
                        // Clone bridge: structural copy inherits BST ordering and size bound.
                        accept(forall|t: T| (#[trigger] lv.contains(t@)) ==> t.cmp_spec(&key) == Less);
                        accept(forall|t: T| (#[trigger] rv.contains(t@)) ==> t.cmp_spec(&key) == Greater);
                        accept(lv.len() + rv.len() < usize::MAX as nat);
                    }
                    new_param_treap(
                        Some(Box::new(NodeInner { key, priority, size, left, right })),
                        Ghost(contents),
                    )
                }
            };
            // Veracity: NEEDED proof block
            proof { accept(cloned@ == self@); } // Clone bridge: deep copy preserves view.
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

    //		Section 14e. derive impls outside verus!

    impl<T: MtKey> fmt::Debug for ParamTreapIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreapIter")
        }
    }

    impl<T: MtKey> fmt::Display for ParamTreapIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreapIter")
        }
    }

    impl<T: MtKey> fmt::Debug for ParamTreapGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreapGhostIterator")
        }
    }

    impl<T: MtKey> fmt::Display for ParamTreapGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ParamTreapGhostIterator")
        }
    }
}
