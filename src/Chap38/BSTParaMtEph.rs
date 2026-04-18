// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Parametric multi-threaded BST built around a joinMid interface.
//! Coarse lock (vstd RwLock) for thread-safe access.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 4c. type definitions
//	Section 5c. view impls
//	Section 4d. type definitions
//	Section 5d. view impls
//	Section 6d. spec fns
//	Section 7d. proof fns/broadcast groups
//	Section 8d. traits
//	Section 9d. impls
//	Section 11a. top level coarse locking
//	Section 12b. derive impls in verus!
//	Section 12c. derive impls in verus!
//	Section 12d. derive impls in verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!


//		Section 1. module

pub mod BSTParaMtEph {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt::{Display, Formatter};
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
    use crate::Chap38::BSTParaSpecsAndLemmas::BSTParaSpecsAndLemmas::*;
    use crate::Types::Types::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;

    verus!
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::set::group_set_axioms,
        vstd::set_lib::group_set_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		Section 4a. type definitions


    pub struct BSTParaMtEphInv<T: MtKey> {
        pub ghost contents: Set<<T as View>::V>,
    }

    //		Section 9a. impls


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty BST.
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


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — clones a single element.
    /// Clone bridge for generic element: requires obeys_feq_clone so axiom_cloned_implies_eq fires.
    fn clone_elem<T: MtKey>(x: &T) -> (c: T)
        requires obeys_feq_clone::<T>(),
        ensures c == *x,
    {
        let c = x.clone();
        // Veracity: NEEDED assert
        // Veracity: NEEDED assert
        assert(cloned(*x, c));  // strictly_cloned(*x,c) from call_ensures; triggers axiom
        c
    }

    /// Expose ParamBST type_invariant across module boundaries:
    /// ghost_locked_root@.finite() is always true, so @.finite() holds.
    pub fn assert_parambst_view_finite<T: MtKey>(s: &ParamBST<T>)
        ensures s@.finite()
    {
        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block
        proof { use_type_invariant(s); }
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub enum Exposed<T: MtKey> {
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    //		Section 5b. view impls


    impl<T: MtKey> View for Exposed<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct NodeInner<T: MtKey> {
        pub key: T,
        pub size: usize,
        pub left: ParamBST<T>,
        pub right: ParamBST<T>,
    }

    //		Section 5c. view impls


    impl<T: MtKey> View for NodeInner<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    //		Section 4d. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct ParamBST<T: MtKey> {
        pub(crate) locked_root: RwLock<Option<Box<NodeInner<T>>>, BSTParaMtEphInv<T>>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    //		Section 5d. view impls


    impl<T: MtKey> View for ParamBST<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_ghost_locked_root() }
    }

    //		Section 6d. spec fns

    // view_ord_consistent defined in BSTParaSpecsAndLemmas, re-exported here.
    #[cfg(verus_keep_ghost)]
    pub use crate::Chap38::BSTParaSpecsAndLemmas::BSTParaSpecsAndLemmas::view_ord_consistent;
    use crate::vstdplus::accept::accept;

    //		Section 7d. proof fns/broadcast groups

    // cmp lemmas moved to BSTParaSpecsAndLemmas.

    //		Section 8d. traits


    pub trait ParamBSTTrait<T: MtKey>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_bstparamteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparamteph_wf();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite(),
                tree.spec_bstparamteph_wf();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — ACCEPTED DIFFERENCE: parametric design; concrete BST provides rebalancing
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn insert(&mut self, key: T)
            requires
                old(self).spec_bstparamteph_wf(),
                old(self)@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self.spec_bstparamteph_wf(),
                self@ =~= old(self)@.insert(key@);
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn delete(&mut self, key: &T)
            requires
                old(self).spec_bstparamteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self.spec_bstparamteph_wf(),
                self@ =~= old(self)@.remove(key@);
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn find(&self, key: &T) -> (found: Option<T>)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures found.is_some() <==> self@.contains(key@);
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
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
        /// - Alg Analysis: APAS (Ch38 Alg 38.4, CS 38.11): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)); delegates to join_pair_inner
        fn join_pair(&self, other: Self) -> (joined: Self)
            requires
                self@.finite(), other@.finite(),
                self@.disjoint(other@),
                self@.len() + other@.len() <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|s: T, o: T| #![trigger self@.contains(s@), other@.contains(o@)]
                    self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@.finite(), joined@ =~= self@.union(other@);
        /// Joins two disjoint BSTs where all elements of self are less than all elements of right.
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — sequential join by exposing right subtree
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(m * lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m * lg(n/m)), Span O(lg^2 n); parallel recursion via ParaPair!
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self@.len() + other@.len() <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures combined@ == self@.union(other@), combined@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(m * lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m * lg(n/m)), Span O(lg^2 n); parallel recursion via ParaPair!
        fn intersect(&self, other: &Self) -> (common: Self)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures common@ == self@.intersect(other@), common@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(m * lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m * lg(n/m)), Span O(lg^2 n); parallel recursion via ParaPair!
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>(),
            ensures remaining@ == self@.difference(other@), remaining@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(Σ W(f(x))), Span O(lg |t| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ W(f(x))), Span O(n + max S(f(x))) — ACCEPTED DIFFERENCE: Verus limitation; spec_fn not Send, blocks parallel filter
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(Σ W(f(x))), Span O(lg |t| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ W(f(x))), Span O(lg |t| + max S(f(x))); parallel recursion via ParaPair!
        /// Requires `op` to be associative with identity `base`.
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> (reduced: T)
            requires forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures self@.len() == 0 ==> reduced@ == base@;
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — agrees with APAS.
        fn min_key(&self) -> (minimum: Option<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@.len() == 0 <==> minimum.is_none(),
                minimum.is_some() ==> self@.contains(minimum.unwrap()@),
                minimum.is_some() ==> forall|t: T| (#[trigger] self@.contains(t@)) ==>
                    minimum.unwrap().cmp_spec(&t) == Less || minimum.unwrap()@ == t@;
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — ACCEPTED DIFFERENCE: parametric design; delegates to join_mid which is O(1)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(|t|) — helper for in_order.
        fn collect_in_order(&self, out: &mut Vec<T>)
            requires
                self@.finite(),
            ensures
                out@.len() == old(out)@.len() + self@.len(),
                forall|i: int| #![trigger out@[i]] 0 <= i < old(out)@.len() ==> out@[i] == old(out)@[i],
                forall|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() ==> self@.contains(out@[i]@),
                forall|v: T::V| self@.contains(v) ==>
                    exists|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() && out@[i]@ == v,
                forall|i: int, j: int| #![trigger out@[i], out@[j]] old(out)@.len() <= i < j < out@.len() ==> out@[i]@ != out@[j]@;
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(|t|), Span O(|t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(|t|) — agrees with APAS; sequential DFS traversal.
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures
                seq@.len() == self@.len(),
                forall|v: T::V| self@.contains(v) <==> seq@.contains(v),
                seq@.no_duplicates();
    }

    //		Section 9d. impls


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


    impl<T: MtKey> ParamBSTTrait<T> for ParamBST<T> {
        open spec fn spec_bstparamteph_wf(&self) -> bool {
            self@.finite()
            && obeys_feq_full::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparamteph_wf()
        {
                      // Veracity: NEEDED assert
                      // Veracity: NEEDED assert
                      assert(obeys_feq_full_trigger::<T>());
            new_param_bst(None, Ghost(Set::empty()))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite(),
                tree.spec_bstparamteph_wf()
        {
            let left = Self::new();
            let right = Self::new();
            let ghost kv = key@;
            new_param_bst(
                Some(Box::new(NodeInner { key, size: 1, left, right })),
                Ghost(Set::<<T as View>::V>::empty().insert(kv)),
            )
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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
                }
        {
            // Veracity: NEEDED proof block (speed hint)
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            expose_internal(self)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn join_mid(exposed: Exposed<T>) -> (joined: Self)
        {
            match exposed {
                | Exposed::Leaf => new_leaf(),
                | Exposed::Node(left, key, right) => {
                    let ghost kv = key@;
                    let ghost contents = left@.union(right@).insert(kv);
                    let lsz = left.size();
                    let rsz = right.size();
                    // Veracity: NEEDED proof block
                    let size: usize = 1 + lsz + rsz;
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        use_type_invariant(&left);
                        use_type_invariant(&right);
                    }
                    new_param_bst(
                        Some(Box::new(NodeInner { key, size, left, right })),
                        Ghost(contents),
                    )
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            // Veracity: NEEDED proof block (speed hint)
            ensures count == self@.len(), self@.finite()
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let handle = self.locked_root.acquire_read();
            // Veracity: NEEDED proof block
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
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite()
        { self.size() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert(&mut self, key: T)
            ensures
                self.spec_bstparamteph_wf(),
                self@ =~= old(self)@.insert(key@),
        // Veracity: NEEDED proof block
        {
            let ghost old_view = self@;
            let _sz = self.size();
            let (left, _found, right) = split_inner(self, &key);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
            }
            *self = Self::join_mid(Exposed::Node(left, key, right));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn delete(&mut self, key: &T)
            ensures
                self.spec_bstparamteph_wf(),
                // Veracity: NEEDED proof block
                self@ =~= old(self)@.remove(key@),
        {
            let ghost old_view = self@;
            let _sz = self.size();
            let (left, _found, right) = split_inner(self, key);
            // Veracity: NEEDED proof block
            proof {
                lemma_cmp_order_axioms::<T>();
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry(o, *key);
                    lemma_cmp_transitivity(s, *key, o);
                };
                // Capacity: _sz: usize == old_view.len(), so old_view.len() <= usize::MAX.
                // left@ ∪ right@ = old_view.remove(key@), whose len <= old_view.len().
            }
            *self = left.join_pair_inner(&right);
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn find(&self, key: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(key@)
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            find_recursive(self, key)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
            ensures
                parts.1 == self@.contains(key@),
                parts.0@.finite(),
                parts.2@.finite(),
// Veracity: UNNEEDED proof block                 parts.0@.union(parts.2@) =~= self@.remove(key@),
                parts.0@.disjoint(parts.2@),
                !parts.0@.contains(key@),
                // Veracity: NEEDED proof block
                !parts.2@.contains(key@),
                forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(&key) == Less,
                forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(&key) == Greater
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let r = split_inner(self, key);
            // Veracity: NEEDED proof block
            proof {
                // split_inner ensures: tree@ =~= parts.0@.union(parts.2@).union(if found { {key@} } else { {} })
                // So self@.remove(key@) =~= parts.0@.union(parts.2@).
                let (ref left, found, ref right) = r;
                let lv = left@;
                let rv = right@;
                let kv = key@;
                if found {
                } else {
                }
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)); delegates to join_pair_inner
        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite(), joined@ =~= self@.union(other@),
        {
            // Veracity: NEEDED proof block
            self.join_pair_inner(&other)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn join_pair_inner(&self, right: &Self) -> (joined: Self)
            ensures joined@.finite(), joined@ =~= self@.union(right@),
            decreases right@.len(),
        {
            // Veracity: NEEDED proof block
            proof {
                lemma_cmp_order_axioms::<T>();
                use_type_invariant(self);
                use_type_invariant(right);
            }
            let ghost lv = self@;
            // Veracity: NEEDED proof block
            let ghost rv = right@;
            match expose_internal(right) {
                | Exposed::Leaf => {
                    self.clone()
                }
                | Exposed::Node(rl, rk, rr) => {
                    let ghost rlv = rl@;
                    let ghost rrv = rr@;
                    let ghost rkv = rk@;
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                        // self ⊥ rl: rl ⊆ right, self ⊥ right.
// Veracity: UNNEEDED proof block                         // Ordering for recursive call: self < rl.
// Veracity: UNNEEDED proof block                         // Veracity: NEEDED assert
// Veracity: UNNEEDED proof block                         // Veracity: NEEDED assert
// Veracity: UNNEEDED proof block                         assert forall|s: T, o: T| #![trigger lv.contains(s@), rlv.contains(o@)]
// Veracity: UNNEEDED proof block                             lv.contains(s@) && rlv.contains(o@) implies s.cmp_spec(&o) == Less by {
// Veracity: UNNEEDED proof block                         };
// Veracity: UNNEEDED proof block                         // Ordering: self < rk.
                    }
                    let merged = self.join_pair_inner(&rl);
                    // Veracity: NEEDED proof block
                    let ghost mv = merged@;
                    // Veracity: NEEDED proof block
                    proof {
                        use_type_invariant(&merged);
                        // merged ⊥ rr.
                        // Ordering: merged < rk.
                        // Size bound.
                        vstd::set_lib::lemma_set_disjoint_lens(mv, rrv);
                    }
                    let result = Self::join_mid(Exposed::Node(merged, rk, rr));
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                    }
                    result
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n)
        // Veracity: NEEDED proof block
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite()
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); use_type_invariant(other); }
            union_inner(self, other)
        }

// Veracity: UNNEEDED proof block         /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n)
        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite()
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); use_type_invariant(other); }
            intersect_inner(self, other)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n)
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite()
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); use_type_invariant(other); }
            difference_inner(self, other)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn filter<F: Fn(&T) -> bool + Send + Sync + 'static>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            filter_parallel(self, predicate, Ghost(spec_pred))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n)
        fn reduce<F: Fn(T, T) -> T + Send + Sync + 'static>(&self, op: F, base: T) -> (reduced: T)
// Veracity: UNNEEDED proof block             ensures self@.len() == 0 ==> reduced@ == base@,
        {
            reduce_parallel(self, op, base)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn min_key(&self) -> (minimum: Option<T>)
            ensures
                self@.len() == 0 <==> minimum.is_none(),
                minimum.is_some() ==> self@.contains(minimum.unwrap()@),
                minimum.is_some() ==> forall|t: T| (#[trigger] self@.contains(t@)) ==>
                    minimum.unwrap().cmp_spec(&t) == Less || minimum.unwrap()@ == t@,
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            min_key_inner(self)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn join_m(left: Self, key: T, right: Self) -> (tree: Self)
        {
            // Veracity: NEEDED proof block
            Self::join_mid(Exposed::Node(left, key, right))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect_in_order(&self, out: &mut Vec<T>)
            ensures
                out@.len() == old(out)@.len() + self@.len(),
                forall|i: int| #![trigger out@[i]] 0 <= i < old(out)@.len() ==> out@[i] == old(out)@[i],
                forall|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() ==> self@.contains(out@[i]@),
                forall|v: T::V| self@.contains(v) ==>
                    exists|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() && out@[i]@ == v,
                forall|i: int, j: int| #![trigger out@[i], out@[j]] old(out)@.len() <= i < j < out@.len() ==> out@[i]@ != out@[j]@,
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            collect_in_order_inner(self, out)
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures
                seq@.len() == self@.len(),
                forall|v: T::V| self@.contains(v) <==> seq@.contains(v),
                seq@.no_duplicates(),
                seq@.to_set() =~= self@,
        {
            let count = self.size();
            let mut out = Vec::with_capacity(count);
            self.collect_in_order(&mut out);
            let result = ArraySeqStPerS::from_vec(out);
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| self@.contains(v) implies result@.contains(v) by {
                    let i = choose|i: int| #![trigger out@[i]] 0 <= i < out@.len() && out@[i]@ == v;
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(result@[i] == result.spec_index(i)@);
                };
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| result@.contains(v) implies self@.contains(v) by {
                    let i = choose|i: int| 0 <= i < result@.len() && result@[i] == v;
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(result.spec_index(i) == out@[i]);
                };
                // No duplicates: collect_in_order gives view-level no-dups, lift to result@.
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|i: int, j: int| 0 <= i < result@.len() && 0 <= j < result@.len() && i != j
                    implies result@[i] != result@[j] by {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(result.spec_index(i) == out@[i]);
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(result.spec_index(j) == out@[j]);
                    if i < j {
                    } else {
                    }
                };
            }
            result
        }
    }

    // Free functions (algorithmic helpers)

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn new_leaf<T: MtKey>() -> (tree: ParamBST<T>)
        ensures tree@ =~= Set::<<T as View>::V>::empty()
    {
        new_param_bst(None, Ghost(Set::empty()))
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn expose_internal<T: MtKey>(tree: &ParamBST<T>) -> (exposed: Exposed<T>)
        requires
            tree@.finite(),
        ensures
            tree@.finite(),
            exposed is Leaf ==> tree@.len() == 0,
            exposed matches Exposed::Node(left, key, right) ==> (
                // Veracity: NEEDED proof block
                tree@.contains(key@)
                && left@.finite()
// Veracity: UNNEEDED proof block                 && right@.finite()
                && left@.subset_of(tree@)
                && right@.subset_of(tree@)
                && tree@ =~= left@.union(right@).insert(key@)
                && !left@.contains(key@)
                && !right@.contains(key@)
                // Veracity: NEEDED proof block
                && left@.disjoint(right@)
                && left@.len() + right@.len() < usize::MAX as nat
                && (forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less)
                && (forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater)
            ),
        decreases tree@.len(), 0nat,
    {
        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block
        proof { use_type_invariant(tree); }
        // Veracity: NEEDED proof block
        // Veracity: NEEDED assert
        proof { assert(obeys_feq_full_trigger::<T>()); }
        let handle = tree.locked_root.acquire_read();
        let exposed = match handle.borrow() {
            | None => Exposed::Leaf,
            | Some(node) => {
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(node.left@, node.right@);
                    // Size bound from the lock predicate inv field.
                }
                let l = node.left.clone();
                let k = clone_elem(&node.key);
                let r = node.right.clone();
                // Veracity: NEEDED proof block
                proof {
                    // k == node.key from clone_elem ensures (value equality).
                    // l@ == node.left@ and r@ == node.right@ from ParamBST::clone ensures.
                    // Ordering properties transfer directly since k == node.key.
                }
                Exposed::Node(l, k, r)
            }
        };
        handle.release_read();
        exposed
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn split_inner<T: MtKey>(tree: &ParamBST<T>, key: &T) -> (parts: (ParamBST<T>, bool, ParamBST<T>))
        requires
            // Veracity: NEEDED proof block
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
        // Veracity: NEEDED proof block
        decreases tree@.len(),
    {
        let _sz = tree.size();
        // Veracity: NEEDED proof block
        proof {
            lemma_cmp_order_axioms::<T>();
            reveal(vstd::laws_cmp::obeys_cmp_ord);
        }
        // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                let ghost kref = *key;
                // Veracity: NEEDED proof block
                proof {
                    lv.lemma_subset_not_in_lt(tree@, rkv);
                    rv.lemma_subset_not_in_lt(tree@, rkv);
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                }
                match <T as std::cmp::Ord>::cmp(key, &root_key) {
                    | Less => {
                        let (ll, found, lr) = split_inner(&left, key);
                        // Veracity: NEEDED proof block
                        proof {
                            // lr ⊂ left, so lr is disjoint from right and ordered < root_key.
                            vstd::set_lib::lemma_len_subset(lr@, lv);
                            // Size bound: lr ⊂ left, so lr@.len() <= left@.len(), and
                            // left@.len() + right@.len() + 1 == tree@.len() <= usize::MAX.
                            // Ordering for join_mid: lr ⊂ left, left < root_key.
                            // Under new-mut-ref, Z3 needs explicit instantiation of the
                            // expose_internal ensures for the right subtree ordering.
                            assert(forall|t: T| #[trigger] right@.contains(t@)
                                ==> t.cmp_spec(&root_key) == Greater);
                        }
                        let rebuilt = ParamBST::<T>::join_mid(Exposed::Node(lr, root_key, right));
                        let ghost llv = ll@;
                        let ghost lrv = lr@;
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(&key) == Greater by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if lrv.contains(t@) {
                                // Veracity: NEEDED proof block
                                } else if rv.contains(t@) {
                                    lemma_cmp_antisymmetry(t, rk);
                                    lemma_cmp_transitivity(kref, rk, t);
                                } else {
                                    lemma_cmp_equal_congruent(t, rk, kref);
                                }
                            };
                            // Disjointness: ll < key < rebuilt.
                            use_type_invariant(&ll);
                        }
                        (ll, found, rebuilt)
                    }
                    | Greater => {
                        let (rl, found, rr) = split_inner(&right, key);
                        // Veracity: NEEDED proof block
                        proof {
                            vstd::set_lib::lemma_len_subset(rl@, rv);
                            // Size bound for join_mid.
                            // Ordering for join_mid: rl ⊂ right, right > root_key.
                            // Under new-mut-ref, Z3 needs explicit instantiation of the
                            // expose_internal ensures for the left subtree ordering.
                            assert(forall|t: T| #[trigger] left@.contains(t@)
                                ==> t.cmp_spec(&root_key) == Less);
                        }
                        let rebuilt = ParamBST::<T>::join_mid(Exposed::Node(left, root_key, rl));
                        let ghost rlv = rl@;
                        // Veracity: NEEDED proof block
                        let ghost rrv = rr@;
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(&key) == Less by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if rlv.contains(t@) {
                                } else if lv.contains(t@) {
                                    lemma_cmp_antisymmetry(kref, rk);
                                    lemma_cmp_transitivity(t, rk, kref);
                                } else {
                                    lemma_cmp_antisymmetry(kref, rk);
                                    lemma_cmp_equal_congruent(t, rk, kref);
                                }
                            };
                            // Disjointness: rebuilt < key < rr.
                            use_type_invariant(&rr);
                        }
                        (rebuilt, found, rr)
                    }
                    | Equal => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] lv.contains(t@)) implies
                                t.cmp_spec(&key) == Less by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                lemma_cmp_equal_congruent_right(t, kref, rk);
                            };
                            // Veracity: NEEDED proof block
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] rv.contains(t@)) implies
                                t.cmp_spec(&key) == Greater by {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                lemma_cmp_equal_congruent_right(t, kref, rk);
                            // Veracity: NEEDED proof block
                            };
                        }
                        (left, true, right)
                    }
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — BST search.
    fn find_recursive<T: MtKey>(tree: &ParamBST<T>, key: &T) -> (found: Option<T>)
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures found.is_some() <==> tree@.contains(key@),
        decreases tree@.len(),
    {
        // Veracity: NEEDED proof block
        proof {
            lemma_cmp_order_axioms::<T>();
            reveal(vstd::laws_cmp::obeys_cmp_ord);
        }
        match expose_internal(tree) {
            | Exposed::Leaf => None,
            | Exposed::Node(left, root_key, right) => {
                // Veracity: NEEDED proof block
                proof {
                    left@.lemma_subset_not_in_lt(tree@, root_key@);
                    // Veracity: NEEDED proof block
                    right@.lemma_subset_not_in_lt(tree@, root_key@);
                }
                match <T as std::cmp::Ord>::cmp(key, &root_key) {
                    | Equal => Some(root_key),
                    | Less => find_recursive(&left, key),
                    // Veracity: NEEDED proof block
                    | Greater => find_recursive(&right, key),
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn min_key_inner<T: MtKey>(tree: &ParamBST<T>) -> (min: Option<T>)
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures
            min.is_none() <==> tree@.len() == 0,
            min.is_some() ==> tree@.contains(min.unwrap()@),
            min.is_some() ==> forall|t: T| (#[trigger] tree@.contains(t@)) ==>
                min.unwrap().cmp_spec(&t) == Less || min.unwrap()@ == t@,
        // Veracity: NEEDED proof block
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => None,
            | Exposed::Node(left, key, right) => {
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                match min_key_inner(&left) {
                    | Some(rec) => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| #![trigger tree@.contains(t@)] tree@.contains(t@) implies
                                rec.cmp_spec(&t) == Less || rec@ == t@ by {
                                if left@.contains(t@) {
                                } else if right@.contains(t@) {
                                    lemma_cmp_antisymmetry(t, key);
                                    lemma_cmp_transitivity(rec, key, t);
                                } else {
                                    lemma_cmp_eq_subst(rec, key, t);
                                }
                            };
                        }
                        Some(rec)
                    }
                    | None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|t: T| #![trigger tree@.contains(t@)] tree@.contains(t@) implies
                                key.cmp_spec(&t) == Less || key@ == t@ by {
                                if right@.contains(t@) {
                                    lemma_cmp_antisymmetry(t, key);
                                }
                            };
                        // Veracity: NEEDED proof block
                        }
                        Some(key)
                    }
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n)
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
                    // Veracity: NEEDED proof block
                    a.clone()
                } else {
                    let (bl, _found, br) = split_inner(b, &ak);
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    let ghost akv = ak@;
                    // Veracity: NEEDED proof block
                    proof {
                        alv.lemma_subset_not_in_lt(a@, akv);
                        // Veracity: NEEDED proof block
                        use_type_invariant(&al);
                        use_type_invariant(&ar);
                        use_type_invariant(&bl);
                        use_type_invariant(&br);
                        // Bound the recursive call inputs: al ⊂ a, bl ⊆ b, ar ⊂ a, br ⊆ b.
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        vstd::set_lib::lemma_len_subset(blv, b@);
                        vstd::set_lib::lemma_len_subset(brv, b@);
                    }
                    let f1 = move || -> (merged: ParamBST<T>)
                        ensures merged@ == al@.union(bl@), merged@.finite()
                    {
                        union_inner(&al, &bl)
                    };
                    let f2 = move || -> (merged: ParamBST<T>)
                        ensures merged@ == ar@.union(br@), merged@.finite()
                    {
                        union_inner(&ar, &br)
                    };
                    let Pair(left_union, right_union) = crate::ParaPair!(f1, f2);
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_cmp_order_axioms::<T>();
                        // Ordering: left_union = al@ ∪ bl@ (all < ak), right_union = ar@ ∪ br@ (all > ak).
                        // Disjointness via existence witnesses.
                        // Size bound.
                        vstd::set_lib::lemma_set_disjoint_lens(left_union@, right_union@);
                        let ghost lu_ru = left_union@.union(right_union@);
                        vstd::set_lib::lemma_len_union(a@, b@);
                        vstd::set_lib::lemma_len_subset(lu_ru.insert(akv), a@.union(b@));
                    }
                    let result = ParamBST::<T>::join_mid(Exposed::Node(left_union, ak, right_union));
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|x| #[trigger] a@.union(b@).contains(x) <==>
                            result@.contains(x) by {
                            if blv.contains(x) || brv.contains(x) {
                            }
                            // Veracity: NEEDED proof block
                            if b@.contains(x) && x != akv {
                            }
                        };
                    }
                    result
                }
            }
        // Veracity: NEEDED proof block
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n)
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
                new_leaf()
            },
            | Exposed::Node(al, ak, ar) => {
                if b.is_empty() {
                    // Veracity: NEEDED proof block
                    new_leaf()
                } else {
                    let ghost sv = a@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    // Veracity: NEEDED proof block
                    let ghost akv = ak@;
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        lemma_cmp_order_axioms::<T>();
                    }
                    let (bl, found, br) = split_inner(b, &ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    // Veracity: NEEDED proof block
                    proof {
                        use_type_invariant(&al);
                        use_type_invariant(&ar);
                        use_type_invariant(&bl);
                        use_type_invariant(&br);
                    }
                    let f1 = move || -> (common: ParamBST<T>)
                        ensures common@ == al@.intersect(bl@), common@.finite()
                    {
                        intersect_inner(&al, &bl)
                    // Veracity: NEEDED proof block
                    };
                    let f2 = move || -> (common: ParamBST<T>)
                        ensures common@ == ar@.intersect(br@), common@.finite()
                    {
                        intersect_inner(&ar, &br)
                    };
                    let Pair(left_res, right_res) = crate::ParaPair!(f1, f2);
                    let ghost lrv = left_res@;
                    let ghost rrv = right_res@;
                    if found {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED proof block
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                        }
                        let result = ParamBST::<T>::join_mid(Exposed::Node(left_res, ak, right_res));
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] sv.intersect(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                }
                                if rrv.contains(x) {
                                }
                                if sv.contains(x) && b@.contains(x) && x != akv {
                                    if alv.contains(x) {
                                        let ghost t: T = choose|t: T| t@ == x;
                                    } else {
                                        let ghost t: T = choose|t: T| t@ == x;
                                    }
                                }
                            };
                        }
                        result
                    } else {
                        // Veracity: NEEDED proof block
                        proof {
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
                                lemma_cmp_antisymmetry(o, ak);
                                lemma_cmp_transitivity(s, ak, o);
                            };
                        }
                        let result = left_res.join_pair_inner(&right_res);
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] sv.intersect(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                }
                                if rrv.contains(x) {
                                }
                                // Veracity: NEEDED proof block
                                if sv.contains(x) && b@.contains(x) {
                                    if alv.contains(x) {
                                        let ghost t: T = choose|t: T| t@ == x;
                                    } else {
                                        let ghost t: T = choose|t: T| t@ == x;
                                    }
                                }
                            // Veracity: NEEDED proof block
                            };
                        }
                        result
                    }
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n)
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
        // Veracity: NEEDED proof block
        match expose_internal(a) {
            | Exposed::Leaf => {
                new_leaf()
            },
            | Exposed::Node(al, ak, ar) => {
                if b.is_empty() {
                    a.clone()
                } else {
                    let ghost sv = a@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                        lemma_cmp_order_axioms::<T>();
                    }
                    let (bl, found, br) = split_inner(b, &ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    // Veracity: NEEDED proof block
                    proof {
                        use_type_invariant(&al);
                        use_type_invariant(&ar);
                        use_type_invariant(&bl);
                        use_type_invariant(&br);
                    }
                    let f1 = move || -> (diff: ParamBST<T>)
                        ensures diff@ == al@.difference(bl@), diff@.finite()
                    {
                        difference_inner(&al, &bl)
                    };
                    // Veracity: NEEDED proof block
                    let f2 = move || -> (diff: ParamBST<T>)
                        ensures diff@ == ar@.difference(br@), diff@.finite()
                    {
                        difference_inner(&ar, &br)
                    };
                    // Veracity: NEEDED proof block
                    let Pair(left_res, right_res) = crate::ParaPair!(f1, f2);
                    let ghost lrv = left_res@;
                    let ghost rrv = right_res@;
                    if found {
                        // Veracity: NEEDED proof block
                        proof {
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
                                lemma_cmp_antisymmetry(o, ak);
                                lemma_cmp_transitivity(s, ak, o);
                            };
                        }
                        let result = left_res.join_pair_inner(&right_res);
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] sv.difference(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                if rrv.contains(x) {
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                if sv.contains(x) && !b@.contains(x) {
                                    if alv.contains(x) {
                                    } else if arv.contains(x) {
                                    }
                                }
                            };
                        }
                        result
                    } else {
                        // Veracity: NEEDED proof block
                        proof {
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                        }
                        let result = ParamBST::<T>::join_mid(Exposed::Node(left_res, ak, right_res));
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] sv.difference(b@).contains(x) <==>
                                result@.contains(x) by {
                                if lrv.contains(x) {
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                // Veracity: NEEDED proof block
                                if rrv.contains(x) {
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                if sv.contains(x) && !b@.contains(x) && x != akv {
                                    if alv.contains(x) {
                                    } else if arv.contains(x) {
                                    }
                                }
                            };
                        }
                        // Veracity: NEEDED proof block
                        result
                    }
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
    fn filter_inner<T: MtKey, F: Fn(&T) -> bool + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        predicate: &Arc<F>,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    // Veracity: NEEDED proof block
    ) -> (filtered: ParamBST<T>)
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            // Veracity: NEEDED proof block
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
            // Veracity: NEEDED proof block
            | Exposed::Node(left, key, right) => {
                let ghost lv = left@;
                let ghost rv = right@;
                let ghost kv = key@;
                // Veracity: NEEDED proof block
                proof {
                    // Expose ensures gives us the size bound for left/right subtrees.
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    use_type_invariant(&left);
                    use_type_invariant(&right);
                }
                // Sequential: borrow predicate for both recursive calls (spec_fn is not Send,
                // so parallel closures cannot capture it; sequential avoids the issue).
                let left_filtered = filter_inner(&left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner(&right, predicate, Ghost(spec_pred));
                // Veracity: NEEDED proof block
                proof {
                    lemma_cmp_order_axioms::<T>();
                    vstd::set_lib::lemma_len_subset(left_filtered@, lv);
                    vstd::set_lib::lemma_len_subset(right_filtered@, rv);
                    // Disjointness: subsets of disjoint sets.
                    // key is not in either filtered subtree (it's not in lv or rv).
                    // Ordering: filtered subsets inherit ordering from parent subtrees.
                }
                if (**predicate)(&key) {
                    let result = ParamBST::<T>::join_mid(
                        Exposed::Node(left_filtered, key, right_filtered),
                    );
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                        // result@ = left_filtered@ ∪ right_filtered@ ∪ {kv}.
                    }
                    result
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        // Ordering for join_pair_inner: all of left_filtered < all of right_filtered.
                        // s ∈ left_filtered ⊆ lv (all < key), o ∈ right_filtered ⊆ rv (all > key).
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|s: T, o: T|
                            #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                            left_filtered@.contains(s@) && right_filtered@.contains(o@)
                            implies s.cmp_spec(&o) == Less by {
                            lemma_cmp_antisymmetry(o, key);
                            lemma_cmp_transitivity(s, key, o);
                        };
                    }
                    let result = left_filtered.join_pair_inner(&right_filtered);
                    // Veracity: NEEDED proof block
                    proof {
                        // result@ = left_filtered@ ∪ right_filtered@.
                    }
                    // Veracity: NEEDED proof block
                    result
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
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
        // Veracity: NEEDED proof block
        proof { use_type_invariant(tree); }
        let predicate = Arc::new(predicate);
        filter_inner(tree, &predicate, Ghost(spec_pred))
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n)
    fn reduce_inner<T: MtKey, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: &Arc<F>,
        identity: T,
    ) -> (reduced: T)
        requires
            tree@.finite(),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> reduced@ == identity@,
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
                proof {
                    // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                let Pair(left_acc, right_acc) = crate::ParaPair!(f1, f2);
                let op_ref = arc_deref(op);
                let right_with_key = op_ref(key, right_acc);
                op_ref(left_acc, right_with_key)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n)
    fn reduce_parallel<T: MtKey, F: Fn(T, T) -> T + Send + Sync + 'static>(
        tree: &ParamBST<T>,
        op: F,
        base: T,
    ) -> (reduced: T)
        requires
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> reduced@ == base@,
    {
        let _ = tree.size();
        let op = Arc::new(op);
        reduce_inner(tree, &op, base)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn collect_in_order_inner<T: MtKey>(tree: &ParamBST<T>, out: &mut Vec<T>)
        requires
            tree@.finite(),
        ensures
            out@.len() == old(out)@.len() + tree@.len(),
            forall|i: int| #![trigger out@[i]] 0 <= i < old(out)@.len() ==> out@[i] == old(out)@[i],
            forall|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() ==> tree@.contains(out@[i]@),
            forall|v: T::V| tree@.contains(v) ==>
                exists|i: int| #![trigger out@[i]] old(out)@.len() <= i < out@.len() && out@[i]@ == v,
            forall|i: int, j: int| #![trigger out@[i], out@[j]] old(out)@.len() <= i < j < out@.len() ==> out@[i]@ != out@[j]@,
        decreases tree@.len(),
    {
        match expose_internal(tree) {
            | Exposed::Leaf => {}
            | Exposed::Node(left, key, right) => {
                let ghost g0 = out@.len();
                let ghost out_0 = out@;
                // Veracity: NEEDED proof block
                proof {
                    left@.lemma_subset_not_in_lt(tree@, key@);
                    right@.lemma_subset_not_in_lt(tree@, key@);
                }
                collect_in_order_inner(&left, out);
                let ghost g1 = out@.len();
                let ghost out_1 = out@;
                out.push(key);
                let ghost g2 = out@.len();
                let ghost out_2 = out@;
                collect_in_order_inner(&right, out);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|i: int| #![trigger out@[i]] g0 <= i < out@.len() implies
                        tree@.contains(out@[i]@) by {
                        if i < g1 as int {
                        } else if i == g1 as int {
                        } else if i < g2 as int {
                        } else {
                        }
                    };
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|v: T::V| tree@.contains(v) implies
                        exists|i: int| #![trigger out@[i]] g0 <= i < out@.len() && out@[i]@ == v by {
                        if left@.contains(v) {
                            let i_left = choose|i: int| #![trigger out_1[i]] g0 <= i < g1 as int && out_1[i]@ == v;
                            // Veracity: NEEDED proof block (speed hint)
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(out@[i_left] == out_2[i_left]);
                        } else if v == key@ {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(out@[g1 as int] == out_2[g1 as int]);
                        } else {
                        }
                    };
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|i: int| #![trigger out@[i]] 0 <= i < g0 implies out@[i] == out_0[i] by {
                    };
                    // No duplicate views in the new portion.
                }
            }
        }
    // Veracity: NEEDED proof block
    }

    //		Section 11a. top level coarse locking


    impl<T: MtKey> RwLockPredicate<Option<Box<NodeInner<T>>>> for BSTParaMtEphInv<T> {
        open spec fn inv(self, v: Option<Box<NodeInner<T>>>) -> bool {
            match v {
                Option::None => self.contents =~= Set::<<T as View>::V>::empty(),
                Option::Some(box_node) => {
                    self.contents =~= (*box_node).left@.union((*box_node).right@).insert((*box_node).key@)
                    && (*box_node).size >= 1
                    && (*box_node).left@.finite() && (*box_node).right@.finite()
                    && (*box_node).left@.disjoint((*box_node).right@)
                    // Veracity: NEEDED proof block
                    && !(*box_node).left@.contains((*box_node).key@)
                    && !(*box_node).right@.contains((*box_node).key@)
                    && (*box_node).left@.len() + (*box_node).right@.len() < usize::MAX as nat
                    && (*box_node).size as nat == (*box_node).left@.len() + (*box_node).right@.len() + 1
                    && (forall|t: T| (#[trigger] (*box_node).left@.contains(t@)) ==> t.cmp_spec(&(*box_node).key) == Less)
                    && (forall|t: T| (#[trigger] (*box_node).right@.contains(t@)) ==> t.cmp_spec(&(*box_node).key) == Greater)
                // Veracity: NEEDED proof block
                }
            }
        }
    }

    //		Section 12b. derive impls in verus!


    impl<T: MtKey> Clone for Exposed<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = match self {
                Exposed::Leaf => Exposed::Leaf,
                Exposed::Node(l, k, r) => Exposed::Node(l.clone(), k.clone(), r.clone()),
            };
            // Veracity: NEEDED proof block
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    //		Section 12c. derive impls in verus!


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
            // Veracity: NEEDED proof block
            proof { accept(cloned@ == self@); }  // assume_eq_clone_workaround
            cloned
        }
    }

    //		Section 12d. derive impls in verus!


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clones entire tree.
    impl<T: MtKey> Clone for ParamBST<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
            decreases self@.len(), 1nat,
        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let _sz = self.size();
            let exposed = expose_internal(self);
            match exposed {
                Exposed::Leaf => Self::new(),
                Exposed::Node(l, k, r) => {
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(l@, r@);
                    }
                    Self::join_mid(Exposed::Node(l, k, r))
                }
            }
        }
    }
    } // verus!

    //		Section 14a. derive impls outside verus!


    impl<T: MtKey> std::fmt::Debug for BSTParaMtEphInv<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "BSTParaMtEphInv")
        }
    }

    impl<T: MtKey> Display for BSTParaMtEphInv<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "BSTParaMtEphInv")
        }
    }

    //		Section 14b. derive impls outside verus!


    impl<T: MtKey> std::fmt::Debug for Exposed<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Exposed::Leaf => write!(f, "Leaf"),
                Exposed::Node(l, k, r) => write!(f, "Node({:?}, {:?}, {:?})", l, k, r),
            }
        }
    }

    impl<T: MtKey> Display for Exposed<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Exposed::Leaf => write!(f, "Leaf"),
                Exposed::Node(l, k, r) => write!(f, "Node({}, {}, {})", l, k, r),
            }
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<T: MtKey> std::fmt::Debug for NodeInner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NodeInner").field("key", &self.key).field("size", &self.size).finish()
        }
    }

    impl<T: MtKey> Display for NodeInner<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "NodeInner(key={}, size={})", self.key, self.size)
        }
    }

    //		Section 14d. derive impls outside verus!

    // Ghost<Set<T::V>> contains FnSpec (PhantomData at runtime), which lacks Send/Sync.
    // ParamBST is safe to send/share: the Ghost field is erased at runtime.
    unsafe impl<T: MtKey> Send for ParamBST<T> {}
    unsafe impl<T: MtKey> Sync for ParamBST<T> {}

    impl<T: MtKey> std::fmt::Debug for ParamBST<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ParamBST").finish()
        }
    }

    impl<T: MtKey> Display for ParamBST<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ParamBST")
        }
    }
}
