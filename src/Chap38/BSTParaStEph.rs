//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Parametric single-threaded BST built around a joinMid interface.
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
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!


//		Section 1. module

pub mod BSTParaStEph {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt::{Debug, Display, Formatter};

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
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		Section 4a. type definitions


    pub struct BSTParaStEphInv<T: StT + Ord> {
        pub ghost contents: Set<<T as View>::V>,
    }

    //		Section 9a. impls


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty BST.
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


    /// Exposes the BST type invariant: every view in the set has a backing element.
    /// Enables callers to instantiate forall-over-T quantifiers from view-level containment.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — proof-only.
    pub fn reveal_param_bst_backings<T: StT + Ord>(tree: &ParamBST<T>)
        ensures forall|v: <T as View>::V| tree@.contains(v) ==> exists|t: T| #[trigger] tree@.contains(t@) && t@ == v
    {
        proof { use_type_invariant(tree); }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — clones a single element.
    /// Clone bridge for generic element: requires obeys_feq_clone so axiom_cloned_implies_eq fires.
    fn clone_elem<T: StT>(x: &T) -> (c: T)
        requires obeys_feq_clone::<T>(),
        ensures c == *x,
    {
        let c = x.clone();
        assert(cloned(*x, c));  // strictly_cloned(*x,c) from call_ensures; triggers axiom
        c
    }

    // 10. free fns

    /// Algorithm 38.9 — sequential filter recursive helper (takes &F for recursion).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
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
                }
                let left_filtered = filter_inner(&left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner(&right, predicate, Ghost(spec_pred));
                if predicate(&key) {
                    proof {
                        vstd::set_lib::lemma_len_subset(left_filtered@, left@);
                        vstd::set_lib::lemma_len_subset(right_filtered@, right@);
                        // Disjointness: subsets of disjoint sets.
                        // Ordering: left_filtered ⊆ left (< key), right_filtered ⊆ right (> key).
                    }
                    ParamBST::join_m(left_filtered, key, right_filtered)
                } else {
                    proof {
                        vstd::set_lib::lemma_len_subset(left_filtered@, left@);
                        vstd::set_lib::lemma_len_subset(right_filtered@, right@);
                        // Ordering: left_filtered ⊆ left < key < right ⊇ right_filtered.
                        assert forall|s: T, o: T| #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                            left_filtered@.contains(s@) && right_filtered@.contains(o@) implies
                            s.cmp_spec(&o) == Less by {
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn reduce_inner<T: StT + Ord, F: Fn(T, T) -> T>(
        tree: &ParamBST<T>,
        op: &F,
        identity: T,
    ) -> (reduced: T)
        requires
            tree@.finite(),
            forall|a: T, b: T| op.requires((a, b)),
        ensures tree@.len() == 0 ==> reduced@ == identity@,
        decreases tree@.len(),
    {
        match tree.expose() {
            | Exposed::Leaf => identity,
            | Exposed::Node(left, key, right) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                let left_acc = reduce_inner(&left, op, identity.clone());
                let right_acc = reduce_inner(&right, op, identity);
                let right_with_key = op(key, right_acc);
                op(left_acc, right_with_key)
            }
        }
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    #[derive(Debug, Default)]
    pub enum Exposed<T: StT + Ord> {
        #[default]
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    //		Section 5b. view impls


    impl<T: StT + Ord> View for Exposed<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    #[derive(Debug)]
    pub struct NodeInner<T: StT + Ord> {
        pub key: T,
        pub size: usize,
        pub left: ParamBST<T>,
        pub right: ParamBST<T>,
    }

    //		Section 5c. view impls


    impl<T: StT + Ord> View for NodeInner<T> {
        type V = ();
        open spec fn view(&self) -> () { () }
    }

    //		Section 4d. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct ParamBST<T: StT + Ord> {
        pub(crate) locked_root: RwLock<Option<Box<NodeInner<T>>>, BSTParaStEphInv<T>>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    //		Section 5d. view impls


    impl<T: StT + Ord> View for ParamBST<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_ghost_locked_root() }
    }

    //		Section 6d. spec fns


    /// View-consistent ordering: elements with the same view compare Equal.
    pub open spec fn view_ord_consistent<T: StT + Ord>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    //		Section 7d. proof fns/broadcast groups


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
    }

    //		Section 8d. traits


    pub trait ParamBSTTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_bstparasteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparasteph_wf();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn singleton(key: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(key@),
                tree@.finite(),
                tree.spec_bstparasteph_wf();
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — DIFFERS: parametric impl wraps node without rebalancing
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — DIFFERS: delegates to join_mid which is O(1) in parametric impl
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn insert(&mut self, key: T)
            requires
                old(self).spec_bstparasteph_wf(),
                old(self)@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self.spec_bstparasteph_wf(),
                self@ =~= old(self)@.insert(key@);
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn delete(&mut self, key: &T)
            requires
                old(self).spec_bstparasteph_wf(),
                old(self)@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self.spec_bstparasteph_wf(),
                self@ =~= old(self)@.remove(key@);
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn find(&self, key: &T) -> (found: Option<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found.is_some() <==> self@.contains(key@);
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — matches APAS
        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|) — agrees with APAS.
        fn max_key(&self) -> (maximum: Option<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@.len() == 0 <==> maximum.is_none(),
                maximum.is_some() ==> self@.contains(maximum.unwrap()@),
                maximum.is_some() ==> forall|t: T| (#[trigger] self@.contains(t@)) ==>
                    t.cmp_spec(&maximum.unwrap()) == Less || maximum.unwrap()@ == t@;
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|)) — matches APAS
        fn join_pair(&self, other: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.disjoint(other@),
                self@.finite(), other@.finite(),
                self@.len() + other@.len() < usize::MAX as nat,
                forall|s: T, o: T| #![trigger self@.contains(s@), other@.contains(o@)] self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@.finite(), joined@ =~= self@.union(other@);
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(m * lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m * lg(n/m)), Span O(m * lg(n/m)) — DIFFERS: sequential recursion, no parallel split
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() <= usize::MAX as nat,
            ensures combined@ == self@.union(other@), combined@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(m * lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m * lg(n/m)), Span O(m * lg(n/m)) — DIFFERS: sequential recursion, no parallel split
        fn intersect(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures common@ == self@.intersect(other@), common@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(m * lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m * lg(n/m)), Span O(m * lg(n/m)) — DIFFERS: sequential recursion, no parallel split
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures remaining@ == self@.difference(other@), remaining@.finite();
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(Σ W(f(x))), Span O(lg |t| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ W(f(x))), Span O(n + max S(f(x))) — DIFFERS: sequential tree traversal
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
        /// - Alg Analysis: APAS (Ch38 CS 38.11): Work O(Σ W(f(x))), Span O(lg |t| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ W(f(x))), Span O(n + max S(f(x))) — DIFFERS: sequential tree traversal
        /// Requires `op` to be associative with identity `base`.
        fn reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (reduced: T)
            requires
                self@.finite(),
                forall|a: T, b: T| op.requires((a, b)),
            ensures self@.len() == 0 ==> reduced@ == base@;
        /// - Alg Analysis: APAS: N/A -- Verus-specific scaffolding.
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(|t|) — agrees with APAS.
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures
                seq@.len() == self@.len(),
                forall|v: T::V| self@.contains(v) <==> seq@.contains(v),
                seq@.no_duplicates();
    }

    //		Section 9d. impls


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


    impl<T: StT + Ord> ParamBSTTrait<T> for ParamBST<T> {
        open spec fn spec_bstparasteph_wf(&self) -> bool {
            self@.finite()
            && obeys_feq_full::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_bstparasteph_wf()
        {
        assert(obeys_feq_full_trigger::<T>());
         new_param_bst(None, Ghost(Set::empty())) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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
                },
            decreases self@.len(), 0nat,
        {
            proof { use_type_invariant(self); }
            proof { assert(obeys_feq_full_trigger::<T>()); }
            let handle = self.locked_root.acquire_read();
            let exposed = match handle.borrow() {
                | None => {
                    Exposed::Leaf
                }
                | Some(node) => {
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(node.left@, node.right@);
                    }
                    let l = node.left.clone();
                    let k = clone_elem(&node.key);
                    let r = node.right.clone();
                    proof {
                        // k == node.key from clone_elem ensures.
                        // l@ == node.left@ and r@ == node.right@ from ParamBST::clone ensures.
                        // Ordering transfers: spec fn on equal args yields equal results.
                    }
                    Exposed::Node(l, k, r)
                }
            };
            handle.release_read();
            exposed
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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
                        // Witness property: every view in contents has a T witness.
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
        fn join_m(left: Self, key: T, right: Self) -> (tree: Self)
        {
            Self::join_mid(Exposed::Node(left, key, right))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite()
        { self.size() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert(&mut self, key: T) {
            let ghost old_view = self@;
            let (left, _, right) = self.split(&key);
            let ghost kv = key@;
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                // left@ ∪ right@ =~= old_view.remove(kv), disjoint.
                // disjoint_lens: left@.len() + right@.len() == old_view.remove(kv).len().
                // old_view.remove(kv).len() ≤ old_view.len() < usize::MAX (from requires).
                vstd::set_lib::lemma_len_subset(old_view.remove(kv), old_view);
            }
            *self = Self::join_m(left, key, right);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn delete(&mut self, key: &T) {
            let ghost old_view = self@;
            let ghost kref = *key;
            let (left, _, right) = self.split(key);
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies
                    s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry(o, kref);
                    lemma_cmp_transitivity(s, kref, o);
                };
            }
            *self = left.join_pair(right);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split(&self, key: &T) -> (parts: (Self, bool, Self))
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
                                assert forall|x| lrv.contains(x) implies lv.contains(x) by {
                                    assert(llv.union(lrv).contains(x));
                                };
                                assert forall|x| llv.contains(x) implies lv.contains(x) by {
                                    assert(llv.union(lrv).contains(x));
                                };
                                vstd::set_lib::lemma_len_subset(lrv, lv);
                            }
                            let rebuilt = Self::join_mid(Exposed::Node(lr, root_key, right));
                            proof {
                                assert forall|x| #[trigger] (llv.union(rebuilt@)).contains(x) <==> self@.remove(key@).contains(x) by {
                                    if llv.contains(x) {
                                    }
                                    if lv.contains(x) && x != key@ {
                                        assert(llv.union(lrv).contains(x));
                                    }
                                };
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
                                assert forall|x| rrv.contains(x) implies rv.contains(x) by {
                                    assert(rlv.union(rrv).contains(x));
                                };
                                vstd::set_lib::lemma_len_subset(rlv, rv);
                            }
                            let rebuilt = Self::join_mid(Exposed::Node(left, root_key, rl));
                            proof {
                                assert forall|x| #[trigger] (rebuilt@.union(rrv)).contains(x) <==> self@.remove(key@).contains(x) by {
                                    if rrv.contains(x) {
                                    }
                                    if rv.contains(x) && x != key@ {
                                        assert(rlv.union(rrv).contains(x));
                                    }
                                };
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
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

        /// APAS: "last need only to traverse right branches."
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn max_key(&self) -> (maximum: Option<T>)
            ensures
                self@.len() == 0 <==> maximum.is_none(),
                maximum.is_some() ==> self@.contains(maximum.unwrap()@),
                maximum.is_some() ==> forall|t: T| (#[trigger] self@.contains(t@)) ==>
                    t.cmp_spec(&maximum.unwrap()) == Less || maximum.unwrap()@ == t@,
            decreases self@.len(),
        {
            match self.expose() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, key, right) => {
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    }
                    match right.max_key() {
                        | Some(rec) => {
                            proof {
                                assert forall|t: T| #![trigger self@.contains(t@)] self@.contains(t@) implies
                                    t.cmp_spec(&rec) == Less || rec@ == t@ by {
                                    if right@.contains(t@) {
                                        // IH covers this.
                                    } else if left@.contains(t@) {
                                        // expose: t.cmp_spec(&key) == Less.
                                        // rec ∈ right, expose: rec.cmp_spec(&key) == Greater,
                                        // so key.cmp_spec(&rec) == Less (antisymmetry).
                                        lemma_cmp_antisymmetry(rec, key);
                                        // t < key < rec (transitivity).
                                        lemma_cmp_transitivity(t, key, rec);
                                    } else {
                                        // t@ == key@, so t.cmp_spec(&key) == Equal.
                                        // key.cmp_spec(&rec) == Less (from above).
                                        // t cmp rec == key cmp rec == Less (congruence).
                                        lemma_cmp_antisymmetry(rec, key);
                                        lemma_cmp_equal_congruent(t, key, rec);
                                    }
                                };
                            }
                            Some(rec)
                        }
                        | None => {
                            proof {
                                assert forall|t: T| #![trigger self@.contains(t@)] self@.contains(t@) implies
                                    t.cmp_spec(&key) == Less || key@ == t@ by {
                                    if left@.contains(t@) {
                                        // expose: t.cmp_spec(&key) == Less. Done.
                                    }
                                    // Otherwise t@ ∈ right@ (empty) or t@ == key@.
                                };
                            }
                            Some(key)
                        }
                    }
                }
            }
        }

        /// Algorithm 38.4 — join two trees via recursive decomposition.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn join_pair(&self, other: Self) -> (joined: Self)
            ensures joined@.finite(), joined@ =~= self@.union(other@),
            decreases other@.len(),
        {
            match other.expose() {
                | Exposed::Leaf => {
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
                        // self ⊥ left: left ⊆ other and self ⊥ other.
                        assert forall|x| lv.contains(x) implies ov.contains(x) by {};
                    }
                    let merged = self.join_pair(left);
                    proof {
                        let ghost mv = merged@;
                        // merged ⊥ right.
                        // key ∉ merged.
                        // Size bound.
                        vstd::set_lib::lemma_set_disjoint_lens(sv, lv);
                    }
                    Self::join_m(merged, key, right)
                }
            }
        }

        /// Algorithm 38.6 — sequential union via divide-and-conquer on split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite(),
            decreases self@.len(),
        {
            match (self.expose(), other.expose()) {
                | (Exposed::Leaf, _) => {
                    other.clone()
                }
                | (_, Exposed::Leaf) => {
                    self.clone()
                }
                | (Exposed::Node(al, ak, ar), _) => {
                    let ghost sv = self@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                    }
                    let (bl, _, br) = other.split(&ak);
                    let ghost blv = bl@;
                    let ghost brv = br@;
                    proof {
                        // Recursive call bounds: al@.len() + bl@.len() <= usize::MAX.
                        // bl ⊂ other (from split), so bl.len() ≤ other.len().
                        assert(other@.remove(akv).subset_of(other@));
                        vstd::set_lib::lemma_len_subset(blv, other@);
                        vstd::set_lib::lemma_len_subset(brv, other@);
                    }
                    let left_union = al.union(&bl);
                    let right_union = ar.union(&br);
                    let ghost luv = left_union@;
                    let ghost ruv = right_union@;
                    proof {
                        // Ordering: luv = alv ∪ blv (all < ak), ruv = arv ∪ brv (all > ak).
                        // Disjointness via witness property: every view in al@/bl@ has a T witness.
                        use_type_invariant(&al);
                        use_type_invariant(&bl);
                        // Size: luv ∪ ruv ∪ {akv} ⊆ self@ ∪ other@, with akv ∉ luv ∪ ruv.
                        // So luv.len() + ruv.len() + 1 ≤ (self@ ∪ other@).len() ≤ usize::MAX.
                        vstd::set_lib::lemma_set_disjoint_lens(luv, ruv);
                        let ghost luv_ruv = luv.union(ruv);
                        vstd::set_lib::lemma_len_union(sv, other@);
                        vstd::set_lib::lemma_len_subset(luv_ruv.insert(akv), sv.union(other@));
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
                            }
                            if other@.contains(x) && x != akv {
                                assert(blv.union(brv).contains(x));
                            }
                        };
                    }
                    result
                }
            }
        }

        /// Algorithm 38.7 — sequential intersect. Keeps keys present in both trees.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn intersect(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite(),
            decreases self@.len(),
        {
            match (self.expose(), other.expose()) {
                | (Exposed::Leaf, _) | (_, Exposed::Leaf) => {
                    Self::new()
                }
                | (Exposed::Node(al, ak, ar), _) => {
                    let ghost sv = self@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
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
                            // Disjoint: subsets of disjoint sets.
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: lrv ⊂ al@ (< ak), rrv ⊂ ar@ (> ak).
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
                                }
                                if rrv.contains(x) {
                                }
                                if sv.contains(x) && other@.contains(x) && x != akv {
                                    // Route x to matching child via ordering witness.
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
                        proof {
                            // Disjoint: subsets of disjoint sets.
                            // Finite: from intersect ensures.
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: lrv ⊂ al@ (< ak), rrv ⊂ ar@ (> ak). s < ak < o ⟹ s < o.
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
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
                                    assert(other@.remove(akv).contains(x));
                                }
                                if rrv.contains(x) {
                                }
                                if sv.contains(x) && other@.contains(x) {
                                    assert(blv.union(brv).contains(x));
                                    // Route x to matching child via ordering witness.
                                    if alv.contains(x) {
                                        let ghost t: T = choose|t: T| t@ == x;
                                    } else {
                                        assert(exists|t: T| t@ == x);
                                        let ghost t: T = choose|t: T| t@ == x;
                                        assert(t.cmp_spec(&ak) == Greater);
                                        assert(rrv.contains(x));
                                    }
                                }
                            };
                        }
                        result
                    }
                }
            }
        }

        /// Algorithm 38.8 — sequential difference. Keeps keys in `self` not in `other`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite(),
            decreases self@.len(),
        {
            match (self.expose(), other.expose()) {
                | (Exposed::Leaf, _) => {
                    Self::new()
                }
                | (_, Exposed::Leaf) => {
                    self.clone()
                }
                | (Exposed::Node(al, ak, ar), _) => {
                    let ghost sv = self@;
                    let ghost alv = al@;
                    let ghost arv = ar@;
                    let ghost akv = ak@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
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
                            // Disjoint: subsets of disjoint sets are disjoint.
                            // Finite: from difference ensures.
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: elements of lrv ⊂ al@ are < ak, elements of rrv ⊂ ar@ are > ak.
                            assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)] lrv.contains(s@) && rrv.contains(o@)
                                implies s.cmp_spec(&o) == Less by {
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
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                if rrv.contains(x) {
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                if sv.contains(x) && !other@.contains(x) {
                                    if alv.contains(x) {
                                    } else if arv.contains(x) {
                                    }
                                }
                            };
                        }
                        result
                    } else {
                        proof {
                            // Disjoint: subsets of disjoint sets.
                            // Size: subset lens bounded by parent lens.
                            vstd::set_lib::lemma_len_subset(lrv, alv);
                            vstd::set_lib::lemma_len_subset(rrv, arv);
                            // Ordering: lrv ⊂ al@ (< ak), rrv ⊂ ar@ (> ak).
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
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                if rrv.contains(x) {
                                    let ghost t: T = choose|t: T| t@ == x;
                                }
                                if sv.contains(x) && !other@.contains(x) && x != akv {
                                    if alv.contains(x) {
                                    } else if arv.contains(x) {
                                    }
                                }
                            };
                        }
                        result
                    }
                }
            }
        }

        /// Algorithm 38.9 — sequential filter. Keeps keys satisfying `predicate`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn filter<F: Fn(&T) -> bool>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            filter_inner(self, &predicate, Ghost(spec_pred))
        }

        /// Algorithm 38.10 — sequential reduce. Folds `op(L', op(k, R'))`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (reduced: T) {
            reduce_inner(self, &op, base)
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
            decreases self@.len(),
        {
            match self.expose() {
                | Exposed::Leaf => {}
                | Exposed::Node(left, key, right) => {
                    let ghost g0 = out@.len();
                    let ghost out_0 = out@;
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
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
                            } else if i == g1 as int {
                            } else {
                            }
                        };
                        // Completeness: all elements of self@ appear.
                        assert forall|v: T::V| self@.contains(v) implies
                            exists|i: int| #![trigger out@[i]] g0 <= i < out@.len() && out@[i]@ == v by {
                            if left@.contains(v) {
                                let i_left = choose|i: int| #![trigger out_1[i]] g0 <= i < g1 as int && out_1[i]@ == v;
                                assert(out@[i_left]@ == v);
                            } else if v == key@ {
                                assert(out@[g1 as int]@ == key@);
                            } else {
                            }
                        };
                        // Preservation: old elements unchanged.
                        assert forall|i: int| #![trigger out@[i]] 0 <= i < g0 implies out@[i] == out_0[i] by {
                        };
                        // No duplicate views in the new portion.
                        // Left portion has views in left@, key has view key@, right portion has views in right@.
                        // These three regions are pairwise disjoint by BST invariant.
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
        {
            let count = self.size();
            let mut out = Vec::with_capacity(count);
            self.collect_in_order(&mut out);
            let result = ArraySeqStPerS::from_vec(out);
            proof {
                // Containment: self@.contains(v) <==> seq@.contains(v).
                assert forall|v: T::V| self@.contains(v) implies result@.contains(v) by {
                    let i = choose|i: int| #![trigger out@[i]] 0 <= i < out@.len() && out@[i]@ == v;
                    assert(result@[i] == result.spec_index(i)@);
                };
                assert forall|v: T::V| result@.contains(v) implies self@.contains(v) by {
                    let i = choose|i: int| 0 <= i < result@.len() && result@[i] == v;
                    assert(result.spec_index(i) == out@[i]);
                };
                // No duplicates: collect_in_order gives view-level no-dups, lift to result@.
                assert forall|i: int, j: int| 0 <= i < result@.len() && 0 <= j < result@.len() && i != j
                    implies result@[i] != result@[j] by {
                    assert(result.spec_index(i) == out@[i]);
                    assert(result.spec_index(j) == out@[j]);
                };
            }
            result
        }
    }

    //		Section 11a. top level coarse locking


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

    //		Section 12b. derive impls in verus!


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

    //		Section 12c. derive impls in verus!


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

    //		Section 12d. derive impls in verus!


    impl<T: StT + Ord> Clone for ParamBST<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
            decreases self@.len(), 1nat,
        {
            proof { use_type_invariant(self); }
            let exposed = self.expose();
            match exposed {
                Exposed::Leaf => {
                    Self::new()
                }
                Exposed::Node(l, k, r) => {
                    proof {
                        vstd::set_lib::lemma_set_disjoint_lens(l@, r@);
                    }
                    Self::join_mid(Exposed::Node(l, k, r))
                }
            }
        }
    }

    } // verus!

    //		Section 13. macros


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

    //		Section 14a. derive impls outside verus!

    impl<T: StT + Ord> Debug for BSTParaStEphInv<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "BSTParaStEphInv")
        }
    }

    impl<T: StT + Ord> Display for BSTParaStEphInv<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "BSTParaStEphInv")
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: StT + Ord + Display> Display for Exposed<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Exposed::Leaf => write!(f, "Leaf"),
                Exposed::Node(l, k, r) => write!(f, "Node({}, {}, {})", l, k, r),
            }
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<T: StT + Ord + Display> Display for NodeInner<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "NodeInner(key={}, size={})", self.key, self.size)
        }
    }

    //		Section 14d. derive impls outside verus!

    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for ParamBST<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ParamBST").finish()
        }
    }

    impl<T: StT + Ord + Display> Display for ParamBST<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ParamBST")
        }
    }
}
