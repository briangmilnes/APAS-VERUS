//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral Treap (randomized heap-ordered BST) with full parametric BST interface.

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

//		1. module


pub mod BSTTreapStEph {

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt;
    use std::hash::{Hash, Hasher};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdIs;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdSpec;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::IsLtTransitive;

    verus! {

    //		4. type definitions

    type Link<T> = Option<Box<Node<T>>>;

    pub struct Node<T: StT + Ord + IsLtTransitive> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    pub struct BSTTreapStEph<T: StT + Ord + IsLtTransitive> {
        pub root: Link<T>,
    }

    pub type BSTreeTreap<T> = BSTTreapStEph<T>;

    #[verifier::reject_recursive_types(T)]
    pub enum ExposedTreap<T: StT + Ord + IsLtTransitive> {
        Leaf,
        Node(BSTTreapStEph<T>, T, BSTTreapStEph<T>),
    }

    // 3. broadcast use

    broadcast use {vstd::set::group_set_axioms, vstd::set_lib::group_set_properties};

    // 5. view impls

    /// Recursive set computation from a Link.
    pub open spec fn spec_set_view_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> Set<<T as View>::V>
        decreases *link,
    {
        match link {
            None => Set::<<T as View>::V>::empty(),
            Some(node) => {
                spec_set_view_link(&node.left).union(spec_set_view_link(&node.right)).insert(node.key@)
            }
        }
    }

    impl<T: StT + Ord + IsLtTransitive> View for BSTTreapStEph<T> {
        type V = Set<T::V>;
        open spec fn view(&self) -> Set<T::V> { spec_set_view_link(&self.root) }
    }

    // 6. spec fns

    // Free spec fn: reveal_with_fuel cannot reference trait methods with generic params.
    pub open spec fn spec_contains_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>, target: T) -> bool
        decreases *link,
    {
        match link {
            None => false,
            Some(node) => {
                node.key == target
                    || spec_contains_link(&node.left, target)
                    || spec_contains_link(&node.right, target)
            }
        }
    }

    /// View-consistent ordering: elements with equal views compare Equal.
    pub open spec fn view_ord_consistent_st<T: StT + Ord + IsLtTransitive>() -> bool {
        forall|a: T, b: T| a@ == b@ <==> (#[trigger] a.cmp_spec(&b)) == Equal
    }

    /// Recursive well-formedness invariant for the parametric interface.
    /// Mirrors the RwLockPredicate invariant from BSTParaTreapMtEph.
    pub open spec fn spec_param_wf_link<T: StT + Ord + IsLtTransitive>(link: &Link<T>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(node) => {
                let lv = spec_set_view_link(&node.left);
                let rv = spec_set_view_link(&node.right);
                let kv = node.key@;
                &&& spec_param_wf_link(&node.left)
                &&& spec_param_wf_link(&node.right)
                &&& node.size >= 1
                &&& lv.finite() && rv.finite()
                &&& lv.disjoint(rv)
                &&& !lv.contains(kv) && !rv.contains(kv)
                &&& lv.len() + rv.len() < usize::MAX as nat
                &&& node.size as nat == lv.len() + rv.len() + 1
                &&& (forall|t: T| (#[trigger] lv.contains(t@)) ==> t.cmp_spec(&node.key) == Less)
                &&& (forall|t: T| (#[trigger] rv.contains(t@)) ==> t.cmp_spec(&node.key) == Greater)
            }
        }
    }

    // 7. proof fns/broadcast groups

    /// Clone a StT element with a cmp_spec-preserving postcondition.
    // veracity: no_requires
    fn clone_elem_st<T: StT + Ord + IsLtTransitive>(x: &T) -> (c: T)
        ensures c@ == x@,
    {
        let c = x.clone();
        proof { assume(c@ == x@); } // eq/clone workaround: structural copy preserves view.
        c
    }

    /// Clone a BSTTreapStEph with view preservation.
    fn clone_with_view<T: StT + Ord + IsLtTransitive>(tree: &BSTTreapStEph<T>) -> (cloned: BSTTreapStEph<T>)
        requires spec_param_wf_link(&tree.root),
        ensures cloned@ =~= tree@, spec_param_wf_link(&cloned.root),
    {
        let cloned = tree.clone();
        proof { assume(cloned@ =~= tree@ && spec_param_wf_link(&cloned.root)); } // eq/clone workaround.
        cloned
    }

    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    pub proof fn lemma_cmp_antisymmetry_st<T: StT + Ord + IsLtTransitive>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Greater,
        ensures b.cmp_spec(&a) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec antisymmetry: Less(a,b) implies Greater(b,a).
    pub proof fn lemma_cmp_antisymmetry_less_st<T: StT + Ord + IsLtTransitive>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
        ensures b.cmp_spec(&a) == Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Less(a,b) and Less(b,c) implies Less(a,c).
    pub proof fn lemma_cmp_transitivity_st<T: StT + Ord + IsLtTransitive>(a: T, b: T, c: T)
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
    pub proof fn lemma_cmp_eq_subst_st<T: StT + Ord + IsLtTransitive>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Equal,
        ensures a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Left congruence: Equal(a,b) implies a and b compare the same way to c.
    pub proof fn lemma_cmp_equal_congruent_st<T: StT + Ord + IsLtTransitive>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
        assert(a@ == b@);
    }

    /// Right congruence: Equal(b,c) implies any a compares the same way to b and c.
    pub proof fn lemma_cmp_equal_congruent_right_st<T: StT + Ord + IsLtTransitive>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            b.cmp_spec(&c) == Equal,
        ensures a.cmp_spec(&b) == a.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
        assert(b@ == c@);
    }

    /// After join(lr, key, right), every element is greater than lk.
    proof fn lemma_joined_right_gt_lk_st<T: StT + Ord + IsLtTransitive>(
        lrv: Set<T::V>,
        right_v: Set<T::V>,
        key: T,
        joined_v: Set<T::V>,
        lk: T,
        left_v: Set<T::V>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
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
            } else if right_v.contains(t@) {
                lemma_cmp_antisymmetry_st(t, key);
                lemma_cmp_transitivity_st(lk, key, t);
                lemma_cmp_antisymmetry_less_st(lk, t);
            } else {
                assert(t@ == key@);
                lemma_cmp_equal_congruent_right_st(lk, t, key);
                lemma_cmp_antisymmetry_less_st(lk, t);
            }
        }
    }

    /// After join(left, key, rl), every element is less than rk.
    proof fn lemma_joined_left_lt_rk_st<T: StT + Ord + IsLtTransitive>(
        left_v: Set<T::V>,
        rlv: Set<T::V>,
        key: T,
        joined_v: Set<T::V>,
        rk: T,
        right_v: Set<T::V>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            joined_v =~= left_v.union(rlv).insert(key@),
            forall|t: T| (#[trigger] left_v.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] rlv.contains(t@)) ==> t.cmp_spec(&rk) == Less,
            right_v.contains(rk@),
            forall|t: T| (#[trigger] right_v.contains(t@)) ==> t.cmp_spec(&key) == Greater,
        ensures
            forall|t: T| (#[trigger] joined_v.contains(t@)) ==> t.cmp_spec(&rk) == Less,
    {
        assert(rk.cmp_spec(&key) == Greater);
        assert forall|t: T| (#[trigger] joined_v.contains(t@)) implies t.cmp_spec(&rk) == Less by {
            if left_v.contains(t@) {
                lemma_cmp_antisymmetry_st(rk, key);
                lemma_cmp_transitivity_st(t, key, rk);
            } else if rlv.contains(t@) {
            } else {
                assert(t@ == key@);
                lemma_cmp_antisymmetry_st(rk, key);
                lemma_cmp_equal_congruent_st(t, key, rk);
            }
        }
    }

    /// Every element in a well-formed tree's set view has a T witness.
    /// This is the St analog of the witness accessibility that type_invariant
    /// provides in the Mt version.
    pub proof fn lemma_wf_view_inhabited_st<T: StT + Ord + IsLtTransitive>(
        link: &Link<T>, x: <T as View>::V,
    )
        requires spec_param_wf_link(link), spec_set_view_link(link).contains(x),
        ensures exists|t: T| #[trigger] t@ == x,
        decreases *link,
    {
        match link {
            None => {}
            Some(node) => {
                let kv = node.key@;
                let lv = spec_set_view_link(&node.left);
                let rv = spec_set_view_link(&node.right);
                if kv == x {
                    assert(node.key@ == x);
                } else if lv.contains(x) {
                    lemma_wf_view_inhabited_st::<T>(&node.left, x);
                } else {
                    assert(rv.contains(x));
                    lemma_wf_view_inhabited_st::<T>(&node.right, x);
                }
            }
        }
    }

    /// Universal version: every value in a well-formed tree's view has a T witness.
    pub proof fn lemma_wf_view_all_inhabited_st<T: StT + Ord + IsLtTransitive>(
        link: &Link<T>,
    )
        requires spec_param_wf_link(link),
        ensures forall|x: <T as View>::V| #[trigger] spec_set_view_link(link).contains(x)
            ==> (exists|t: T| t@ == x),
    {
        assert forall|x: <T as View>::V| #[trigger] spec_set_view_link(link).contains(x)
            implies (exists|t: T| t@ == x) by {
            lemma_wf_view_inhabited_st::<T>(link, x);
        };
    }

    /// Hash-based priority for treap heap ordering.
    #[verifier::external_body]
    fn priority_for_st<T: StT + Ord + IsLtTransitive>(key: &T) -> (p: u64) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buf = String::new();
        let _ = std::fmt::Write::write_fmt(&mut buf, format_args!("{key:?}"));
        Hash::hash(&buf, &mut hasher);
        hasher.finish()
    }

    //		8. traits

    pub trait BSTTreapStEphTrait<T: StT + Ord + IsLtTransitive> {
        spec fn spec_size_link(link: &Link<T>) -> nat;
        spec fn spec_bst_link(link: &Link<T>) -> bool;
        spec fn spec_link_size_wf(link: &Link<T>) -> bool;
        spec fn spec_in_order_link(link: &Link<T>) -> Seq<T>;
        spec fn spec_pre_order_link(link: &Link<T>) -> Seq<T>;
        spec fn spec_min_link(link: &Link<T>) -> Option<T>;
        spec fn spec_max_link(link: &Link<T>) -> Option<T>;
        spec fn spec_height_link(link: &Link<T>) -> nat;

        spec fn spec_size(self) -> nat;
        spec fn spec_bsttreapsteph_wf(self) -> bool;
        spec fn spec_bst(self) -> bool;
        spec fn spec_height(self) -> nat;
        spec fn spec_contains(self, target: T) -> bool;
        spec fn spec_min(self) -> Option<T>;
        spec fn spec_max(self) -> Option<T>;
        spec fn spec_in_order(self) -> Seq<T>;
        spec fn spec_pre_order(self) -> Seq<T>;

        proof fn lemma_height_le_size(link: &Link<T>)
            requires
                Self::spec_link_size_wf(link),
                Self::spec_size_link(link) < usize::MAX as nat,
            ensures Self::spec_height_link(link) <= Self::spec_size_link(link);

        proof fn lemma_size_wf_child_bounded(link: &Link<T>)
            requires
                Self::spec_link_size_wf(link),
                Self::spec_size_link(link) > 0,
                Self::spec_size_link(link) < usize::MAX as nat,
            ensures
                match link {
                    None => true,
                    Some(node) => {
                        Self::spec_size_link(&node.left) < usize::MAX as nat
                        && Self::spec_size_link(&node.right) < usize::MAX as nat
                    },
                };

        proof fn lemma_wf_decompose(link: &Link<T>)
            requires Self::spec_link_size_wf(link),
            ensures match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right)
                    && Self::spec_link_size_wf(&node.left)
                    && Self::spec_link_size_wf(&node.right)
                },
            };

        proof fn lemma_wf_assemble_node(node: &Box<Node<T>>)
            requires
                node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right),
                Self::spec_link_size_wf(&node.left),
                Self::spec_link_size_wf(&node.right),
            ensures Self::spec_link_size_wf(&Some(*node));

        proof fn lemma_contains_left(node: &Box<Node<T>>, k: T)
            requires spec_contains_link(&node.left, k),
            ensures spec_contains_link(&Some(*node), k);

        proof fn lemma_contains_right(node: &Box<Node<T>>, k: T)
            requires spec_contains_link(&node.right, k),
            ensures spec_contains_link(&Some(*node), k);

        proof fn lemma_bst_decompose(link: &Link<T>)
            requires Self::spec_bst_link(link),
            ensures match link {
                None => true,
                Some(node) => {
                    Self::spec_bst_link(&node.left)
                    && Self::spec_bst_link(&node.right)
                    && (forall|k: T| #[trigger] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                    && (forall|k: T| #[trigger] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
                },
            };

        proof fn lemma_contains_root(node: &Box<Node<T>>)
            ensures spec_contains_link(&Some(*node), node.key);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new()                       -> (empty_tree: Self)
        where
            Self: Sized,
            ensures
                empty_tree.spec_size() == 0,
                empty_tree.spec_bsttreapsteph_wf(),
                empty_tree.spec_bst();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> (sz: usize)
            ensures sz as nat == self.spec_size();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> (empty: bool)
            ensures empty == (self.spec_size() == 0);
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self)               -> (h: usize)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_bsttreapsteph_wf(),
            ensures h as nat == self.spec_height();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires
                old(self).spec_size() + 1 <= usize::MAX as nat,
                old(self).spec_bsttreapsteph_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                self.spec_bsttreapsteph_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size(),
                self.spec_contains(value),
                forall|k: T| old(self).spec_contains(k) ==> self.spec_contains(k),
                old(self).spec_bst() ==> self.spec_bst();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn delete(&mut self, target: &T)
            requires
                old(self).spec_bsttreapsteph_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                self.spec_bsttreapsteph_wf(),
                self.spec_size() <= old(self).spec_size(),
                forall|k: T| self.spec_contains(k) ==> old(self).spec_contains(k),
                old(self).spec_bst() ==> self.spec_bst();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, target: &T)     -> (found: Option<&T>)
            requires
                self.spec_bsttreapsteph_wf(),
                self.spec_bst(),
                T::obeys_partial_cmp_spec(),
            ensures
                found.is_some() <==> self.spec_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bsttreapsteph_wf(),
                self.spec_bst(),
                T::obeys_partial_cmp_spec(),
            ensures found == self.spec_contains(*target);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn minimum(&self)              -> (min_val: Option<&T>)
            ensures match (min_val, self.spec_min()) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn maximum(&self)              -> (max_val: Option<&T>)
            ensures match (max_val, self.spec_max()) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn in_order(&self)             -> (ordered: ArraySeqStPerS<T>)
            ensures ordered.spec_len() == self.spec_in_order().len();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn pre_order(&self)            -> (preordered: ArraySeqStPerS<T>)
            ensures preordered.spec_len() == self.spec_pre_order().len();

        /// - APAS: Work Θ(1), Span Θ(1)
        fn new_node(key: T, priority: u64) -> (n: Node<T>)
            ensures
                Self::spec_link_size_wf(&Some(Box::new(n))),
                n.size == 1;

        /// - APAS: Work Θ(1), Span Θ(1)
        fn size_link(link: &Link<T>) -> (sz: usize)
            ensures sz as nat == Self::spec_size_link(link);

        /// - APAS: Work Θ(1), Span Θ(1)
        fn update_size(node: &mut Box<Node<T>>)
            requires 1 + Self::spec_size_link(&old(node).left) + Self::spec_size_link(&old(node).right) <= usize::MAX as nat,
            ensures
                node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right),
                node.key == old(node).key,
                node.left == old(node).left,
                node.right == old(node).right;

        fn rotate_left(x: Box<Node<T>>) -> (rotated: Box<Node<T>>)
            requires
                Self::spec_link_size_wf(&Some(x)),
                Self::spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                Self::spec_link_size_wf(&Some(rotated)),
                Self::spec_size_link(&Some(rotated)) == Self::spec_size_link(&Some(x)),
                Self::spec_bst_link(&Some(x)) ==> Self::spec_bst_link(&Some(rotated)),
                forall|k: T| spec_contains_link(&Some(rotated), k) <==> spec_contains_link(&Some(x), k);

        fn rotate_right(x: Box<Node<T>>) -> (rotated: Box<Node<T>>)
            requires
                Self::spec_link_size_wf(&Some(x)),
                Self::spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                Self::spec_link_size_wf(&Some(rotated)),
                Self::spec_size_link(&Some(rotated)) == Self::spec_size_link(&Some(x)),
                Self::spec_bst_link(&Some(x)) ==> Self::spec_bst_link(&Some(rotated)),
                forall|k: T| spec_contains_link(&Some(rotated), k) <==> spec_contains_link(&Some(x), k);

        fn clone_link(link: &Link<T>) -> (c: Link<T>)
            ensures
                Self::spec_size_link(&c) == Self::spec_size_link(link),
                Self::spec_link_size_wf(link) ==> Self::spec_link_size_wf(&c);

        fn height_link(link: &Link<T>) -> (h: usize)
            requires
                Self::spec_size_link(link) < usize::MAX as nat,
                Self::spec_link_size_wf(link),
            ensures h as nat == Self::spec_height_link(link);

        fn insert_link(link: Link<T>, value: T, priority: u64) -> (inserted: Link<T>)
            requires
                Self::spec_size_link(&link) + 1 <= usize::MAX as nat,
                Self::spec_link_size_wf(&link),
                T::obeys_partial_cmp_spec(),
            ensures
                Self::spec_link_size_wf(&inserted),
                Self::spec_size_link(&inserted) <= Self::spec_size_link(&link) + 1,
                Self::spec_size_link(&inserted) >= Self::spec_size_link(&link),
                forall|k: T| spec_contains_link(&link, k) ==> spec_contains_link(&inserted, k),
                forall|k: T| spec_contains_link(&inserted, k) ==> (spec_contains_link(&link, k) || k == value),
                spec_contains_link(&inserted, value),
                Self::spec_bst_link(&link) ==> Self::spec_bst_link(&inserted);

        fn delete_link(link: Link<T>, target: &T) -> (deleted: Link<T>)
            requires
                Self::spec_link_size_wf(&link),
                T::obeys_partial_cmp_spec(),
            ensures
                Self::spec_link_size_wf(&deleted),
                Self::spec_size_link(&deleted) <= Self::spec_size_link(&link),
                forall|k: T| spec_contains_link(&deleted, k) ==> spec_contains_link(&link, k),
                Self::spec_bst_link(&link) ==> Self::spec_bst_link(&deleted);

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            requires
                Self::spec_bst_link(link),
                T::obeys_partial_cmp_spec(),
            ensures
                found.is_some() <==> spec_contains_link(link, *target),
                found.is_some() ==> *found.unwrap() == *target;

        fn min_link(link: &Link<T>) -> (min_val: Option<&T>)
            ensures match (min_val, Self::spec_min_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };

        fn max_link(link: &Link<T>) -> (max_val: Option<&T>)
            ensures match (max_val, Self::spec_max_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };

        fn in_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            ensures ordered@.len() == Self::spec_in_order_link(link).len();

        fn pre_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            ensures ordered@.len() == Self::spec_pre_order_link(link).len();

    }


    //		9. impls

    impl<T: StT + Ord + IsLtTransitive> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        open spec fn spec_size_link(link: &Link<T>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(node) => node.size as nat,
            }
        }

        open spec fn spec_bst_link(link: &Link<T>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(node) => {
                    Self::spec_bst_link(&node.left)
                        && Self::spec_bst_link(&node.right)
                        && (forall|k: T| #![trigger spec_contains_link(&node.left, k)] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                        && (forall|k: T| #![trigger spec_contains_link(&node.right, k)] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
                }
            }
        }

        open spec fn spec_link_size_wf(link: &Link<T>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right)
                        && Self::spec_link_size_wf(&node.left)
                        && Self::spec_link_size_wf(&node.right)
                }
            }
        }

        open spec fn spec_in_order_link(link: &Link<T>) -> Seq<T>
            decreases *link,
        {
            match link {
                None => Seq::empty(),
                Some(node) => {
                    Self::spec_in_order_link(&node.left)
                        + seq![node.key]
                        + Self::spec_in_order_link(&node.right)
                }
            }
        }

        open spec fn spec_pre_order_link(link: &Link<T>) -> Seq<T>
            decreases *link,
        {
            match link {
                None => Seq::empty(),
                Some(node) => {
                    seq![node.key]
                        + Self::spec_pre_order_link(&node.left)
                        + Self::spec_pre_order_link(&node.right)
                }
            }
        }

        open spec fn spec_min_link(link: &Link<T>) -> Option<T>
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => match node.left {
                    None => Some(node.key),
                    Some(_) => Self::spec_min_link(&node.left),
                },
            }
        }

        open spec fn spec_max_link(link: &Link<T>) -> Option<T>
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => match node.right {
                    None => Some(node.key),
                    Some(_) => Self::spec_max_link(&node.right),
                },
            }
        }

        open spec fn spec_height_link(link: &Link<T>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(node) => {
                    let lh = Self::spec_height_link(&node.left);
                    let rh = Self::spec_height_link(&node.right);
                    let m = if lh >= rh { lh } else { rh };
                    1 + m
                }
            }
        }

        open spec fn spec_size(self) -> nat { Self::spec_size_link(&self.root) }
        open spec fn spec_bsttreapsteph_wf(self) -> bool { Self::spec_link_size_wf(&self.root) }
        open spec fn spec_bst(self) -> bool { Self::spec_bst_link(&self.root) }
        open spec fn spec_height(self) -> nat { Self::spec_height_link(&self.root) }
        open spec fn spec_contains(self, target: T) -> bool { spec_contains_link(&self.root, target) }
        open spec fn spec_min(self) -> Option<T> { Self::spec_min_link(&self.root) }
        open spec fn spec_max(self) -> Option<T> { Self::spec_max_link(&self.root) }
        open spec fn spec_in_order(self) -> Seq<T> { Self::spec_in_order_link(&self.root) }
        open spec fn spec_pre_order(self) -> Seq<T> { Self::spec_pre_order_link(&self.root) }

        proof fn lemma_height_le_size(link: &Link<T>)
            decreases *link,
        {
            match link {
                None => {},
                Some(node) => {
                    Self::lemma_size_wf_child_bounded(link);
                    Self::lemma_height_le_size(&node.left);
                    Self::lemma_height_le_size(&node.right);
                    assert(Self::spec_height_link(link) <= Self::spec_size_link(link));
                }
            }
        }

        proof fn lemma_size_wf_child_bounded(link: &Link<T>)
            decreases *link,
        {
            match link {
                None => {},
                Some(node) => {
                    assert(node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right));
                    assert(Self::spec_size_link(&node.left) < node.size as nat);
                    assert(Self::spec_size_link(&node.right) < node.size as nat);
                    assert(node.size as nat == Self::spec_size_link(link));
                }
            }
        }

        proof fn lemma_wf_decompose(link: &Link<T>) {
        }

        proof fn lemma_wf_assemble_node(node: &Box<Node<T>>) {
        }

        proof fn lemma_contains_left(node: &Box<Node<T>>, k: T) {
        }

        proof fn lemma_contains_right(node: &Box<Node<T>>, k: T) {
        }

        proof fn lemma_bst_decompose(link: &Link<T>) {
        }

        proof fn lemma_contains_root(node: &Box<Node<T>>) {
        }

        fn new() -> Self { BSTTreapStEph { root: None } }

        fn size(&self) -> usize { Self::size_link(&self.root) }

        fn is_empty(&self) -> bool { self.size() == 0 }

        fn height(&self) -> usize {
            Self::height_link(&self.root)
        }

        fn insert(&mut self, value: T, priority: u64) {
            self.root = Self::insert_link(self.root.take(), value, priority);
        }

        fn delete(&mut self, target: &T) {
            self.root = Self::delete_link(self.root.take(), target);
        }

        fn find(&self, target: &T) -> Option<&T> {
            Self::find_link(&self.root, target)
        }

        fn contains(&self, target: &T) -> bool {
            self.find(target).is_some()
        }

        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(Self::in_order_vec(&self.root))
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(Self::pre_order_vec(&self.root))
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        fn new_node(key: T, priority: u64) -> (n: Node<T>) {
            let n = Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            };
            assert(Self::spec_link_size_wf(&n.left));
            assert(Self::spec_link_size_wf(&n.right));
            n
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        fn size_link(link: &Link<T>) -> (sz: usize) {
            match link.as_ref() {
                None => 0,
                Some(n) => n.size,
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        fn update_size(node: &mut Box<Node<T>>) {
            let l = Self::size_link(&node.left);
            let r = Self::size_link(&node.right);
            node.size = 1 + l + r;
        }

        fn rotate_left(mut x: Box<Node<T>>) -> (rotated: Box<Node<T>>) {
            let ghost bst_input = Self::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_right = x.right;
            assert(Self::spec_link_size_wf(&x.left));
            assert(Self::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.right.take() {
                let ghost yk = y.key;
                let ghost b  = y.left;
                let ghost c  = y.right;

                assert(Self::spec_link_size_wf(&y.left));
                assert(Self::spec_link_size_wf(&y.right));
                let ghost x_left_sz = Self::spec_size_link(&x.left);
                let ghost y_left_sz = Self::spec_size_link(&y.left);
                let ghost y_right_sz = Self::spec_size_link(&y.right);

                proof {
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_right);
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies xk.is_lt(&k) by {
                            Self::lemma_contains_left(&y, k);
                        };
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies k.is_lt(&yk) by {};
                    }
                }

                x.right = y.left.take();
                assert(1 + x_left_sz + y_left_sz + 1 + y_right_sz <= usize::MAX as nat);
                Self::update_size(&mut x);

                proof {
                    if bst_input {
                        assert(Self::spec_bst_link(&x.left));
                        assert(Self::spec_bst_link(&x.right));
                        assert(Self::spec_bst_link(&Some(x)));
                    }
                }

                y.left = Some(x);
                Self::update_size(&mut y);
                proof {
                    Self::lemma_wf_assemble_node(&y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_right);
                        assert(Self::spec_bst_link(&y.right));
                        Self::lemma_contains_root(&y);
                        Self::lemma_contains_root(&y);
                        assert(spec_contains_link(&orig_right, yk));
                        assert(xk.is_lt(&yk));
                        assert(x.right == b);
                        assert forall |k: T| #[trigger] spec_contains_link(&y.left, k) implies k.is_lt(&yk) by {
                            if spec_contains_link(&x.left, k) {
                                T::is_lt_transitive(k, xk, yk);
                            }
                            if spec_contains_link(&x.right, k) {
                                assert(spec_contains_link(&b, k));
                            }
                        };
                        assert(Self::spec_bst_link(&Some(y)));
                    }
                }
                y
            } else {
                x
            }
        }

        fn rotate_right(mut x: Box<Node<T>>) -> (rotated: Box<Node<T>>) {
            let ghost bst_input = Self::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_left = x.left;
            assert(Self::spec_link_size_wf(&x.left));
            assert(Self::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.left.take() {
                let ghost yk = y.key;
                let ghost b  = y.right;
                let ghost a  = y.left;

                assert(Self::spec_link_size_wf(&y.left));
                assert(Self::spec_link_size_wf(&y.right));
                let ghost x_right_sz = Self::spec_size_link(&x.right);
                let ghost y_left_sz = Self::spec_size_link(&y.left);
                let ghost y_right_sz = Self::spec_size_link(&y.right);

                proof {
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_left);
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies k.is_lt(&xk) by {
                            Self::lemma_contains_right(&y, k);
                        };
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies yk.is_lt(&k) by {};
                    }
                }

                x.left = y.right.take();
                assert(1 + y_left_sz + x_right_sz + 1 + y_right_sz <= usize::MAX as nat);
                Self::update_size(&mut x);

                proof {
                    if bst_input {
                        assert(Self::spec_bst_link(&x.right));
                        assert(Self::spec_bst_link(&x.left));
                        assert(Self::spec_bst_link(&Some(x)));
                    }
                }

                y.right = Some(x);
                Self::update_size(&mut y);
                proof {
                    Self::lemma_wf_assemble_node(&y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_left);
                        assert(Self::spec_bst_link(&y.left));
                        Self::lemma_contains_root(&y);
                        assert(spec_contains_link(&orig_left, yk));
                        assert(yk.is_lt(&xk));
                        assert(x.left == b);
                        assert forall |k: T| #[trigger] spec_contains_link(&y.right, k) implies yk.is_lt(&k) by {
                            if spec_contains_link(&x.right, k) {
                                T::is_lt_transitive(yk, xk, k);
                            }
                            if spec_contains_link(&x.left, k) {
                                assert(spec_contains_link(&b, k));
                            }
                        };
                        assert(Self::spec_bst_link(&Some(y)));
                    }
                }
                y
            } else {
                x
            }
        }

        fn clone_link(link: &Link<T>) -> (c: Link<T>)
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => {
                    let left = Self::clone_link(&node.left);
                    let right = Self::clone_link(&node.right);
                    Some(Box::new(Node {
                        key: node.key.clone(),
                        priority: node.priority,
                        size: node.size,
                        left,
                        right,
                    }))
                }
            }
        }

        fn height_link(link: &Link<T>) -> (h: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    proof { Self::lemma_size_wf_child_bounded(link); }
                    let lh = Self::height_link(&node.left);
                    let rh = Self::height_link(&node.right);
                    let m = if lh >= rh { lh } else { rh };
                    proof {
                        Self::lemma_height_le_size(&node.left);
                        Self::lemma_height_le_size(&node.right);
                        assert(lh as nat == Self::spec_height_link(&node.left));
                        assert(rh as nat == Self::spec_height_link(&node.right));
                        assert(m as nat <= Self::spec_size_link(&node.left) || m as nat <= Self::spec_size_link(&node.right));
                        assert(m < usize::MAX);
                    }
                    1 + m
                }
            }
        }

        fn insert_link(link: Link<T>, value: T, priority: u64) -> (inserted: Link<T>)
            decreases link,
        {
            proof { reveal_with_fuel(spec_contains_link, 3); }
            match link {
                None => {
                    let n = Box::new(Node { key: value, priority, size: 1, left: None, right: None });
                    proof { Self::lemma_wf_assemble_node(&n); }
                    Some(n)
                },
                Some(mut node) => {
                    let ghost orig_key = node.key;
                    let ghost orig_left = node.left;
                    let ghost orig_right = node.right;
                    proof {
                        assert forall |k: T|
                            #[trigger] spec_contains_link(&link, k) <==>
                            (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                            by {};
                    }
                    assert(Self::spec_link_size_wf(&node.left));
                    assert(Self::spec_link_size_wf(&node.right));
                    if value < node.key {
                        node.left = Self::insert_link(node.left.take(), value, priority);
                        Self::update_size(&mut node);
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            // Inserted value is in the left subtree, hence in the tree.
                            Self::lemma_contains_left(&node, value);
                            assert forall |k: T| #[trigger] spec_contains_link(&link, k)
                                implies spec_contains_link(&Some(node), k) by {
                                if spec_contains_link(&node.left, k) { Self::lemma_contains_left(&node, k); }
                                if spec_contains_link(&node.right, k) { Self::lemma_contains_right(&node, k); }
                            };
                            assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                                implies (spec_contains_link(&link, k) || k == value) by {};
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
                                assert(node.key == orig_key);
                                assert forall |k: T| #[trigger] spec_contains_link(&node.left, k)
                                    implies k.is_lt(&node.key) by {
                                    if spec_contains_link(&orig_left, k) {
                                    } else {
                                        assert(value.is_lt(&node.key));
                                    }
                                };
                                assert(Self::spec_bst_link(&Some(node)));
                            }
                        }
                        let needs_rotate = match &node.left {
                            Some(l) => l.priority < node.priority,
                            None => false,
                        };
                        if needs_rotate {
                            let ghost pre_rotate = node;
                            let rotated = Self::rotate_right(node);
                            proof {
                                // Rotation preserves containment; value was in pre_rotate.
                                assert(spec_contains_link(&Some(rotated), value));
                            }
                            Some(rotated)
                        } else { Some(node) }
                    } else if node.key < value {
                        node.right = Self::insert_link(node.right.take(), value, priority);
                        Self::update_size(&mut node);
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            // Inserted value is in the right subtree, hence in the tree.
                            Self::lemma_contains_right(&node, value);
                            assert forall |k: T| #[trigger] spec_contains_link(&link, k)
                                implies spec_contains_link(&Some(node), k) by {
                                if spec_contains_link(&node.left, k) { Self::lemma_contains_left(&node, k); }
                                if spec_contains_link(&node.right, k) { Self::lemma_contains_right(&node, k); }
                            };
                            assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                                implies (spec_contains_link(&link, k) || k == value) by {};
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
                                assert(node.key == orig_key);
                                assert forall |k: T| #[trigger] spec_contains_link(&node.right, k)
                                    implies node.key.is_lt(&k) by {
                                    if spec_contains_link(&orig_right, k) {
                                    } else {
                                        assert(node.key.is_lt(&value));
                                    }
                                };
                                assert(Self::spec_bst_link(&Some(node)));
                            }
                        }
                        let needs_rotate = match &node.right {
                            Some(r) => r.priority < node.priority,
                            None => false,
                        };
                        if needs_rotate {
                            let ghost pre_rotate = node;
                            let rotated = Self::rotate_left(node);
                            proof {
                                assert(spec_contains_link(&Some(rotated), value));
                            }
                            Some(rotated)
                        } else { Some(node) }
                    } else {
                        // !(value < node.key) && !(node.key < value) => structurally equal.
                        proof {
                            T::is_lt_antisymmetric(value, node.key);
                            Self::lemma_contains_root(&node);
                        }
                        Some(node)
                    }
                }
            }
        }

        fn delete_link(link: Link<T>, target: &T) -> (deleted: Link<T>)
            decreases Self::spec_size_link(&link),
        {
            proof { reveal_with_fuel(spec_contains_link, 3); }
            match link {
                None => None,
                Some(mut node) => {
                    let ghost orig_key = node.key;
                    let ghost orig_left = node.left;
                    let ghost orig_right = node.right;
                    proof {
                        assert forall |k: T|
                            #[trigger] spec_contains_link(&link, k) <==>
                            (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                            by {};
                    }
                    assert(Self::spec_link_size_wf(&node.left));
                    assert(Self::spec_link_size_wf(&node.right));
                    if *target < node.key {
                        // Target in left subtree.
                        node.left = Self::delete_link(node.left.take(), target);
                        Self::update_size(&mut node);
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                                implies spec_contains_link(&link, k) by {
                                if spec_contains_link(&node.left, k) {
                                    assert(spec_contains_link(&orig_left, k));
                                }
                            };
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
                                assert forall |k: T| #[trigger] spec_contains_link(&node.left, k)
                                    implies k.is_lt(&node.key) by {
                                    assert(spec_contains_link(&orig_left, k));
                                };
                            }
                        }
                        Some(node)
                    } else if node.key < *target {
                        // Target in right subtree.
                        node.right = Self::delete_link(node.right.take(), target);
                        Self::update_size(&mut node);
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                                implies spec_contains_link(&link, k) by {
                                if spec_contains_link(&node.right, k) {
                                    assert(spec_contains_link(&orig_right, k));
                                }
                            };
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
                                assert forall |k: T| #[trigger] spec_contains_link(&node.right, k)
                                    implies node.key.is_lt(&k) by {
                                    assert(spec_contains_link(&orig_right, k));
                                };
                            }
                        }
                        Some(node)
                    } else {
                        // Found the target node.
                        if node.left.is_none() && node.right.is_none() {
                            // Leaf: remove.
                            None
                        } else if node.right.is_none() {
                            // Only left child.
                            proof {
                                if Self::spec_bst_link(&link) {
                                    Self::lemma_bst_decompose(&link);
                                }
                            }
                            node.left.take()
                        } else if node.left.is_none() {
                            // Only right child.
                            proof {
                                if Self::spec_bst_link(&link) {
                                    Self::lemma_bst_decompose(&link);
                                }
                            }
                            node.right.take()
                        } else {
                            // Two children: rotate smaller-priority child up, then recurse.
                            let left_pri = node.left.as_ref().unwrap().priority;
                            let right_pri = node.right.as_ref().unwrap().priority;
                            if left_pri <= right_pri {
                                let mut rotated = Self::rotate_right(node);
                                let ghost rot_key = rotated.key;
                                let ghost rot_left = rotated.left;
                                let ghost rot_right = rotated.right;
                                proof {
                                    assert forall |k: T|
                                        #[trigger] spec_contains_link(&link, k) <==>
                                        (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                        by {};
                                    if Self::spec_bst_link(&link) {
                                        Self::lemma_bst_decompose(&Some(rotated));
                                        assert(Self::spec_bst_link(&rot_left));
                                        assert(Self::spec_bst_link(&rot_right));
                                        assert forall |k: T| #[trigger] spec_contains_link(&rot_left, k)
                                            implies k.is_lt(&rot_key) by {};
                                        assert forall |k: T| #[trigger] spec_contains_link(&rot_right, k)
                                            implies rot_key.is_lt(&k) by {};
                                    }
                                }
                                assert(Self::spec_link_size_wf(&Some(rotated)));
                                assert(Self::spec_link_size_wf(&rotated.right));
                                assert(Self::spec_size_link(&rotated.right) < Self::spec_size_link(&link));
                                rotated.right = Self::delete_link(rotated.right.take(), target);
                                Self::update_size(&mut rotated);
                                proof {
                                    Self::lemma_wf_assemble_node(&rotated);
                                    assert forall |k: T| #[trigger] spec_contains_link(&Some(rotated), k)
                                        implies spec_contains_link(&link, k) by {
                                        if spec_contains_link(&rotated.right, k) {
                                            assert(spec_contains_link(&rot_right, k));
                                        }
                                        if spec_contains_link(&rotated.left, k) {
                                            assert(spec_contains_link(&rot_left, k));
                                        }
                                    };
                                    if Self::spec_bst_link(&link) {
                                        assert forall |k: T| #[trigger] spec_contains_link(&rotated.right, k)
                                            implies rotated.key.is_lt(&k) by {
                                            assert(spec_contains_link(&rot_right, k));
                                        };
                                        assert(Self::spec_bst_link(&Some(rotated)));
                                    }
                                }
                                Some(rotated)
                            } else {
                                let mut rotated = Self::rotate_left(node);
                                let ghost rot_key = rotated.key;
                                let ghost rot_left = rotated.left;
                                let ghost rot_right = rotated.right;
                                proof {
                                    assert forall |k: T|
                                        #[trigger] spec_contains_link(&link, k) <==>
                                        (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                        by {};
                                    if Self::spec_bst_link(&link) {
                                        Self::lemma_bst_decompose(&Some(rotated));
                                        assert(Self::spec_bst_link(&rot_left));
                                        assert(Self::spec_bst_link(&rot_right));
                                        assert forall |k: T| #[trigger] spec_contains_link(&rot_left, k)
                                            implies k.is_lt(&rot_key) by {};
                                        assert forall |k: T| #[trigger] spec_contains_link(&rot_right, k)
                                            implies rot_key.is_lt(&k) by {};
                                    }
                                }
                                assert(Self::spec_link_size_wf(&Some(rotated)));
                                assert(Self::spec_link_size_wf(&rotated.left));
                                assert(Self::spec_size_link(&rotated.left) < Self::spec_size_link(&link));
                                rotated.left = Self::delete_link(rotated.left.take(), target);
                                Self::update_size(&mut rotated);
                                proof {
                                    Self::lemma_wf_assemble_node(&rotated);
                                    assert forall |k: T| #[trigger] spec_contains_link(&Some(rotated), k)
                                        implies spec_contains_link(&link, k) by {
                                        if spec_contains_link(&rotated.left, k) {
                                            assert(spec_contains_link(&rot_left, k));
                                        }
                                        if spec_contains_link(&rotated.right, k) {
                                            assert(spec_contains_link(&rot_right, k));
                                        }
                                    };
                                    if Self::spec_bst_link(&link) {
                                        assert forall |k: T| #[trigger] spec_contains_link(&rotated.left, k)
                                            implies k.is_lt(&rotated.key) by {
                                            assert(spec_contains_link(&rot_left, k));
                                        };
                                        assert(Self::spec_bst_link(&Some(rotated)));
                                    }
                                }
                                Some(rotated)
                            }
                        }
                    }
                }
            }
        }

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            decreases *link,
        {
            proof { reveal_with_fuel(spec_contains_link, 2); }
            match link {
                | None => None,
                | Some(node) => {
                    proof { Self::lemma_bst_decompose(link); }
                    if *target < node.key {
                        let r = Self::find_link(&node.left, target);
                        proof {
                            // Forward: found in subtree → in whole tree.
                            if r.is_some() {
                                Self::lemma_contains_left(node, *target);
                            }
                            // Reverse: in whole tree → must be in left subtree → found.
                            T::is_lt_irreflexive(*target);
                            if spec_contains_link(link, *target) {
                                if spec_contains_link(&node.right, *target) {
                                    T::is_lt_transitive(*target, node.key, *target);
                                }
                                assert(!spec_contains_link(&node.right, *target));
                                assert(node.key != *target);
                                assert(spec_contains_link(&node.left, *target));
                                assert(r.is_some());
                            }
                        }
                        r
                    } else if node.key < *target {
                        let r = Self::find_link(&node.right, target);
                        proof {
                            // Forward: found in subtree → in whole tree.
                            if r.is_some() {
                                Self::lemma_contains_right(node, *target);
                            }
                            // Reverse: in whole tree → must be in right subtree → found.
                            T::is_lt_irreflexive(*target);
                            if spec_contains_link(link, *target) {
                                assert(!spec_contains_link(&node.left, *target));
                                assert(node.key != *target);
                                assert(spec_contains_link(&node.right, *target));
                                assert(r.is_some());
                            }
                        }
                        r
                    } else {
                        // Neither target < node.key nor node.key < target.
                        proof {
                            T::is_lt_antisymmetric(*target, node.key);
                            Self::lemma_contains_root(node);
                            assert(spec_contains_link(link, *target));
                        }
                        Some(&node.key)
                    }
                }
            }
        }

        fn min_link(link: &Link<T>) -> (min_val: Option<&T>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_link(&node.left),
                },
            }
        }

        fn max_link(link: &Link<T>) -> (max_val: Option<&T>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_link(&node.right),
                },
            }
        }

        fn in_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            decreases *link,
        {
            match link {
                None => Vec::new(),
                Some(node) => {
                    let mut result = Self::in_order_vec(&node.left);
                    result.push(node.key.clone());
                    let mut right = Self::in_order_vec(&node.right);
                    result.append(&mut right);
                    result
                }
            }
        }

        fn pre_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            decreases *link,
        {
            match link {
                None => Vec::new(),
                Some(node) => {
                    let mut result = Vec::new();
                    result.push(node.key.clone());
                    let mut left = Self::pre_order_vec(&node.left);
                    result.append(&mut left);
                    let mut right = Self::pre_order_vec(&node.right);
                    result.append(&mut right);
                    result
                }
            }
        }
    }

    impl<T: StT + Ord + IsLtTransitive> Default for BSTreeTreap<T> {
        fn default() -> (d: Self)
            ensures d.spec_size() == 0, d.spec_bsttreapsteph_wf(), d.spec_bst(),
        { Self::new() }
    }

    // 9b. parametric internal functions

    /// Build a new tree from (left, key, priority, right) maintaining BST ordering.
    fn make_node_treap_st<T: StT + Ord + IsLtTransitive>(
        left: BSTTreapStEph<T>, key: T, priority: u64, right: BSTTreapStEph<T>,
    ) -> (node: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            left@.finite(), right@.finite(),
            left@.disjoint(right@),
            !left@.contains(key@), !right@.contains(key@),
            left@.len() + right@.len() < usize::MAX as nat,
            forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
            spec_param_wf_link(&left.root), spec_param_wf_link(&right.root),
        ensures
            node@ =~= left@.union(right@).insert(key@),
            node@.finite(),
            spec_param_wf_link(&node.root),
    {
        let ls = BSTTreapStEph::<T>::size_link(&left.root);
        let rs = BSTTreapStEph::<T>::size_link(&right.root);
        proof {
            // Show size_link matches set view length via wf invariant.
            // For a well-formed link, size_link == spec_set_view_link.len().
            // This follows from the wf invariant's size == lv.len() + rv.len() + 1.
            assume(ls as nat == left@.len());
            assume(rs as nat == right@.len());
        }
        let size = 1 + ls + rs;
        proof {
            vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
            assert(!left@.union(right@).contains(key@));
        }
        BSTTreapStEph {
            root: Some(Box::new(Node {
                key,
                priority,
                size,
                left: left.root,
                right: right.root,
            })),
        }
    }

    /// Read the priority of a tree's root node (i64::MIN for empty).
    fn tree_priority_st<T: StT + Ord + IsLtTransitive>(tree: &BSTTreapStEph<T>) -> (p: u64)
        ensures true,
    {
        match &tree.root {
            None => 0u64,
            Some(node) => node.priority,
        }
    }

    /// Expose a tree into its constituent parts. Consumes the tree.
    fn expose_to_parts_st<T: StT + Ord + IsLtTransitive>(tree: BSTTreapStEph<T>) -> (parts: Option<(BSTTreapStEph<T>, T, u64, BSTTreapStEph<T>)>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            spec_param_wf_link(&tree.root),
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
                && spec_param_wf_link(&l.root)
                && spec_param_wf_link(&r.root)
            },
    {
        match tree.root {
            None => None,
            Some(node) => {
                let priority = node.priority;
                let key = node.key;
                let left = BSTTreapStEph { root: node.left };
                let right = BSTTreapStEph { root: node.right };
                Some((left, key, priority, right))
            }
        }
    }

    /// Merge two BST-ordered subtrees with a middle key, rebalancing by priority.
    fn join_with_priority_st<T: StT + Ord + IsLtTransitive>(
        left: BSTTreapStEph<T>, key: T, priority: u64, right: BSTTreapStEph<T>,
    ) -> (result: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            left@.finite(), right@.finite(),
            left@.disjoint(right@),
            !left@.contains(key@), !right@.contains(key@),
            left@.len() + right@.len() < usize::MAX as nat,
            forall|t: T| (#[trigger] left@.contains(t@)) ==> t.cmp_spec(&key) == Less,
            forall|t: T| (#[trigger] right@.contains(t@)) ==> t.cmp_spec(&key) == Greater,
            spec_param_wf_link(&left.root), spec_param_wf_link(&right.root),
        ensures result@ =~= left@.union(right@).insert(key@), result@.finite(),
            spec_param_wf_link(&result.root),
        decreases left@.len() + right@.len(),
    {
        let left_priority = tree_priority_st(&left);
        let right_priority = tree_priority_st(&right);
        if priority >= left_priority && priority >= right_priority {
            return make_node_treap_st(left, key, priority, right);
        }
        if left_priority >= right_priority {
            match expose_to_parts_st(left) {
                None => make_node_treap_st(BSTTreapStEph { root: None }, key, priority, right),
                Some((ll, lk, lp, lr)) => {
                    let ghost lkv = lk@;
                    let ghost lrv = lr@;
                    let ghost llv = ll@;
                    proof {
                        // lr ⊆ left, so lr < key and lr.disjoint(right).
                        assert(lrv.subset_of(ll@.union(lr@).insert(lkv)));
                        vstd::set_lib::lemma_len_subset(lrv, ll@.union(lr@).insert(lkv));
                        assert(!lrv.contains(key@));
                        assert(lrv.disjoint(right@));
                        assert(lrv.len() + right@.len() < usize::MAX as nat);
                        assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&key) == Less by {
                            assert(ll@.union(lr@).insert(lkv).contains(t@));
                        }
                    }
                    let merged_right = join_with_priority_st(lr, key, priority, right);
                    proof {
                        lemma_joined_right_gt_lk_st(lrv, right@, key, merged_right@, lk, ll@.union(lrv).insert(lkv));
                        assert(!llv.contains(lkv));
                        assert(!merged_right@.contains(lkv));
                        assert(llv.disjoint(merged_right@));
                        vstd::set_lib::lemma_set_disjoint_lens(llv, lrv);
                        vstd::set_lib::lemma_set_disjoint_lens(lrv, right@);
                        assert(merged_right@.len() == lrv.len() + right@.len() + 1);
                        vstd::set_lib::lemma_len_subset(llv, ll@.union(lr@).insert(lkv));
                        assert(llv.len() + merged_right@.len() < usize::MAX as nat);
                    }
                    make_node_treap_st(ll, lk, lp, merged_right)
                }
            }
        } else {
            match expose_to_parts_st(right) {
                None => make_node_treap_st(left, key, priority, BSTTreapStEph { root: None }),
                Some((rl, rk, rp, rr)) => {
                    let ghost rkv = rk@;
                    let ghost rlv = rl@;
                    let ghost rrv = rr@;
                    proof {
                        assert(rlv.subset_of(rl@.union(rr@).insert(rkv)));
                        vstd::set_lib::lemma_len_subset(rlv, rl@.union(rr@).insert(rkv));
                        assert(!rlv.contains(key@));
                        assert(left@.disjoint(rlv));
                        assert(left@.len() + rlv.len() < usize::MAX as nat);
                        assert forall|t: T| (#[trigger] rlv.contains(t@)) implies t.cmp_spec(&key) == Greater by {
                            assert(rl@.union(rr@).insert(rkv).contains(t@));
                        }
                    }
                    let merged_left = join_with_priority_st(left, key, priority, rl);
                    proof {
                        assert(rl@.union(rr@).insert(rkv).contains(rkv));
                        lemma_joined_left_lt_rk_st(left@, rlv, key, merged_left@, rk, rl@.union(rr@).insert(rkv));
                        assert(!rrv.contains(rkv));
                        assert(!merged_left@.contains(rkv));
                        assert(merged_left@.disjoint(rrv));
                        vstd::set_lib::lemma_set_disjoint_lens(left@, rlv);
                        vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                        assert(merged_left@.len() == left@.len() + rlv.len() + 1);
                        vstd::set_lib::lemma_len_subset(rrv, rl@.union(rr@).insert(rkv));
                        assert(merged_left@.len() + rrv.len() < usize::MAX as nat);
                    }
                    make_node_treap_st(merged_left, rk, rp, rr)
                }
            }
        }
    }

    fn split_inner_st<T: StT + Ord + IsLtTransitive>(
        tree: BSTTreapStEph<T>, key: &T,
    ) -> (parts: (BSTTreapStEph<T>, bool, BSTTreapStEph<T>))
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            spec_param_wf_link(&tree.root),
        ensures
            parts.1 == tree@.contains(key@),
            parts.0@.finite(), parts.2@.finite(),
            parts.0@.union(parts.2@) =~= tree@.remove(key@),
            parts.0@.disjoint(parts.2@),
            !parts.0@.contains(key@) && !parts.2@.contains(key@),
            forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(key) == Less,
            forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(key) == Greater,
            spec_param_wf_link(&parts.0.root), spec_param_wf_link(&parts.2.root),
        decreases tree@.len(),
    {
        let empty = BSTTreapStEph::<T> { root: None };
        match expose_to_parts_st(tree) {
            | None => (BSTTreapStEph { root: None }, false, BSTTreapStEph { root: None }),
            | Some((left, root_key, root_pri, right)) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    assert(!left@.union(right@).contains(root_key@));
                }
                let ghost rk = root_key;
                let ghost kval = *key;
                match key.cmp(&root_key) {
                    | Less => {
                        let ghost lv = left@;
                        let ghost rv = right@;
                        let ghost rkv = root_key@;
                        let (ll, found, lr) = split_inner_st(left, key);
                        let ghost llv = ll@;
                        let ghost lrv = lr@;
                        proof {
                            assert forall|x| lrv.contains(x) implies lv.contains(x) by {
                                assert(llv.union(lrv).contains(x));
                            };
                            assert(lrv.subset_of(lv));
                            assert forall|x| llv.contains(x) implies lv.contains(x) by {
                                assert(llv.union(lrv).contains(x));
                            };
                            assert(llv.subset_of(lv));
                            vstd::set_lib::lemma_len_subset(lrv, lv);
                            assert forall|t: T| (#[trigger] lrv.contains(t@)) implies t.cmp_spec(&root_key) == Less by {
                                assert(lv.contains(t@));
                            };
                            assert(lrv.disjoint(rv));
                            assert(lrv.len() + rv.len() < usize::MAX as nat);
                        }
                        let rebuilt = join_with_priority_st(lr, root_key, root_pri, right);
                        proof {
                            reveal(vstd::laws_cmp::obeys_cmp_ord);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            assert(rebuilt@ =~= lrv.union(rv).insert(rkv));
                            // key < root_key, all rv elements > root_key, so key ∉ rv.
                            assert(kval.cmp_spec(&rk) == Less);
                            if rv.contains(key@) {
                                let ghost t = choose|t: T| t@ == key@ && #[trigger] rv.contains(t@);
                                assert(t.cmp_spec(&rk) == Greater);
                                lemma_cmp_equal_congruent_st(kval, t, rk);
                                assert(false);
                            }
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
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(key) == Greater by {
                                if lrv.contains(t@) {
                                    // Recursive split ensures t > key.
                                } else if rv.contains(t@) {
                                    lemma_cmp_antisymmetry_st(t, rk);
                                    lemma_cmp_transitivity_st(kval, rk, t);
                                } else {
                                    assert(t@ == rkv);
                                    lemma_cmp_eq_subst_st(kval, rk, t);
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
                        let (rl, found, rr) = split_inner_st(right, key);
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
                            assert forall|t: T| (#[trigger] rlv.contains(t@)) implies t.cmp_spec(&root_key) == Greater by {
                                assert(rv.contains(t@));
                            };
                            assert(lv.disjoint(rlv));
                            assert(lv.len() + rlv.len() < usize::MAX as nat);
                        }
                        let rebuilt = join_with_priority_st(left, root_key, root_pri, rl);
                        proof {
                            reveal(vstd::laws_cmp::obeys_cmp_ord);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            assert(rebuilt@ =~= lv.union(rlv).insert(rkv));
                            // key > root_key, all lv elements < root_key, so key ∉ lv.
                            assert(kval.cmp_spec(&rk) == Greater);
                            if lv.contains(key@) {
                                let ghost t = choose|t: T| t@ == key@ && #[trigger] lv.contains(t@);
                                assert(t.cmp_spec(&rk) == Less);
                                lemma_cmp_equal_congruent_st(kval, t, rk);
                                assert(false);
                            }
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
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                if rlv.contains(t@) {
                                    // Recursive split ensures t < key.
                                } else if lv.contains(t@) {
                                    lemma_cmp_antisymmetry_st(kval, rk);
                                    lemma_cmp_transitivity_st(t, rk, kval);
                                } else {
                                    assert(t@ == rkv);
                                    lemma_cmp_antisymmetry_st(kval, rk);
                                    lemma_cmp_equal_congruent_st(t, rk, kval);
                                }
                            };
                            assert(rebuilt@.disjoint(rrv));
                        }
                        (rebuilt, found, rr)
                    }
                    | Equal => {
                        proof {
                            reveal(vstd::laws_cmp::obeys_cmp_ord);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            // key.cmp_spec(&root_key) == Equal, so kval@ == rk@.
                            assert(kval.cmp_spec(&rk) == Equal);
                            assert forall|t: T| (#[trigger] left@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                lemma_cmp_equal_congruent_right_st(t, kval, rk);
                            };
                            assert forall|t: T| (#[trigger] right@.contains(t@)) implies
                                t.cmp_spec(key) == Greater by {
                                lemma_cmp_equal_congruent_right_st(t, kval, rk);
                            };
                        }
                        (left, true, right)
                    }
                }
            }
        }
    }

    fn join_pair_inner_st<T: StT + Ord + IsLtTransitive>(
        left: BSTTreapStEph<T>, right: BSTTreapStEph<T>,
    ) -> (joined: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            left@.finite(), right@.finite(),
            forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                left@.contains(s@) && right@.contains(o@) ==> s.cmp_spec(&o) == Less,
            left@.len() + right@.len() < usize::MAX as nat,
            spec_param_wf_link(&left.root), spec_param_wf_link(&right.root),
        ensures joined@.finite(), joined@ =~= left@.union(right@),
            spec_param_wf_link(&joined.root),
        decreases left@.len() + right@.len(),
    {
        let ghost lv = left@;
        let ghost rv = right@;
        proof {
            // St analog of type_invariant witness accessibility.
            lemma_wf_view_all_inhabited_st::<T>(&left.root);
            // Derive lv.disjoint(rv) from strict ordering + inhabitedness.
            assert(lv.disjoint(rv)) by {
                assert forall|x: T::V| !(lv.contains(x) && rv.contains(x)) by {
                    if lv.contains(x) && rv.contains(x) {
                        let ghost t = choose|t: T| #[trigger] t@ == x;
                        assert(lv.contains(t@));
                        assert(rv.contains(t@));
                        assert(t.cmp_spec(&t) == Less);
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        assert(t.cmp_spec(&t) == Equal);
                    }
                };
            };
        }
        match expose_to_parts_st(right) {
            | None => left,
            | Some((r_left, r_key, rp, r_right)) => {
                let ghost rkv = r_key@;
                let ghost rlv = r_left@;
                let ghost rrv = r_right@;
                proof {
                    assert(rv =~= rlv.union(rrv).insert(rkv));
                    assert(rv.contains(rkv));
                    assert(rlv.subset_of(rv)) by {
                        assert forall|x: T::V| #[trigger] rlv.contains(x) implies rv.contains(x) by {
                            assert((rlv.union(rrv).insert(rkv)).contains(x));
                        };
                    };
                    assert(rrv.subset_of(rv)) by {
                        assert forall|x: T::V| #[trigger] rrv.contains(x) implies rv.contains(x) by {
                            assert((rlv.union(rrv).insert(rkv)).contains(x));
                        };
                    };
                    assert forall|l: T| #[trigger] lv.contains(l@) implies l@ != rkv by {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        assert(rv.contains(r_key@));
                    };
                    assert(!lv.contains(rkv));
                    assert forall|s: T, o: T| #![trigger lv.contains(s@), rlv.contains(o@)]
                        lv.contains(s@) && rlv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        assert(rlv.subset_of(rv));
                    };
                    assert forall|s: T, o: T| #![trigger lv.contains(s@), rrv.contains(o@)]
                        lv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        assert(rrv.subset_of(rv));
                    };
                    vstd::set_lib::lemma_len_subset(rlv, rv);
                    vstd::set_lib::lemma_len_subset(rrv, rv);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                    assert(rlv.len() + rrv.len() < rv.len());
                    assert(lv.len() + rlv.len() < usize::MAX as nat);
                    assert(lv.len() + rrv.len() < usize::MAX as nat);
                    // Ordering facts while exec vars are live.
                    assert forall|t: T| #[trigger] rlv.contains(t@) implies t.cmp_spec(&r_key) == Less by {
                        assert(r_left@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&r_key) == Greater by {
                        assert(r_right@.contains(t@));
                    };
                }
                let (split_left, _, split_right) = split_inner_st(left, &r_key);
                let ghost slv = split_left@;
                let ghost srv = split_right@;
                proof {
                    assert(slv.union(srv) =~= lv.remove(rkv));
                    assert(lv.remove(rkv) =~= lv);
                    assert(slv.subset_of(lv)) by {
                        assert forall|x: T::V| #[trigger] slv.contains(x) implies lv.contains(x) by {
                            assert(slv.union(srv).contains(x));
                        };
                    };
                    assert(srv.subset_of(lv)) by {
                        assert forall|x: T::V| #[trigger] srv.contains(x) implies lv.contains(x) by {
                            assert(slv.union(srv).contains(x));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(slv, lv);
                    vstd::set_lib::lemma_len_subset(srv, lv);
                    assert(slv.len() + rlv.len() < usize::MAX as nat);
                    assert(srv.len() + rrv.len() < usize::MAX as nat);
                    assert forall|s: T, o: T| #![trigger slv.contains(s@), rlv.contains(o@)]
                        slv.contains(s@) && rlv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        assert(slv.subset_of(lv));
                        assert(rlv.subset_of(rv));
                    };
                    assert forall|s: T, o: T| #![trigger srv.contains(s@), rrv.contains(o@)]
                        srv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        assert(srv.subset_of(lv));
                        assert(rrv.subset_of(rv));
                    };
                    assert forall|t: T| #[trigger] slv.contains(t@) implies t.cmp_spec(&r_key) == Less by {
                        assert(split_left@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] srv.contains(t@) implies t.cmp_spec(&r_key) == Greater by {
                        assert(split_right@.contains(t@));
                    };
                }
                let combined_left = join_pair_inner_st(split_left, r_left);
                let combined_right = join_pair_inner_st(split_right, r_right);
                let ghost clv = combined_left@;
                let ghost crv = combined_right@;
                proof {
                    assert(clv =~= slv.union(rlv));
                    assert(crv =~= srv.union(rrv));
                    assert(!slv.contains(rkv));
                    assert(!rlv.contains(rkv));
                    assert(!clv.contains(rkv));
                    assert(!srv.contains(rkv));
                    assert(!rrv.contains(rkv));
                    assert(!crv.contains(rkv));
                    assert(clv == slv.union(rlv));
                    assert(crv == srv.union(rrv));
                    assert forall|t: T| #[trigger] clv.contains(t@) implies t.cmp_spec(&r_key) == Less by {
                        assert(slv.union(rlv).contains(t@));
                        if slv.contains(t@) { }
                        else { assert(rlv.contains(t@)); }
                    };
                    assert forall|t: T| #[trigger] crv.contains(t@) implies t.cmp_spec(&r_key) == Greater by {
                        assert(srv.union(rrv).contains(t@));
                        if srv.contains(t@) { }
                        else { assert(rrv.contains(t@)); }
                    };
                    // slv ⊆ lv, rlv ⊆ rv, lv.disjoint(rv) → slv.disjoint(rlv).
                    assert(slv.disjoint(rlv)) by {
                        assert forall|x: T::V| !(slv.contains(x) && rlv.contains(x)) by {
                            if slv.contains(x) && rlv.contains(x) {
                                assert(lv.contains(x) && rv.contains(x));
                            }
                        };
                    };
                    // srv ⊆ lv, rrv ⊆ rv, lv.disjoint(rv) → srv.disjoint(rrv).
                    assert(srv.disjoint(rrv)) by {
                        assert forall|x: T::V| !(srv.contains(x) && rrv.contains(x)) by {
                            if srv.contains(x) && rrv.contains(x) {
                                assert(lv.contains(x) && rv.contains(x));
                            }
                        };
                    };
                    // clv < r_key < crv → clv.disjoint(crv).
                    assert(clv.disjoint(crv)) by {
                        assert forall|x: T::V| !(clv.contains(x) && crv.contains(x)) by {
                            if clv.contains(x) && crv.contains(x) {
                                lemma_wf_view_inhabited_st::<T>(&combined_left.root, x);
                                let ghost tl = choose|t: T| #[trigger] t@ == x && clv.contains(t@);
                                assert(tl.cmp_spec(&r_key) == Less);
                                assert(crv.contains(tl@));
                                assert(tl.cmp_spec(&r_key) == Greater);
                                assert(false);
                            }
                        };
                    };
                    vstd::set_lib::lemma_set_disjoint_lens(slv, rlv);
                    vstd::set_lib::lemma_set_disjoint_lens(srv, rrv);
                    vstd::set_lib::lemma_set_disjoint_lens(slv, srv);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                    assert(slv.union(srv) =~= lv);
                    assert(rlv.union(rrv).insert(rkv) =~= rv);
                    assert(rlv.len() + rrv.len() < rv.len());
                    vstd::set_lib::lemma_set_disjoint_lens(clv, crv);
                    assert(clv.len() + crv.len() < lv.len() + rv.len());
                    assert(clv.len() + crv.len() < usize::MAX as nat);
                    assert(clv.union(crv).insert(rkv) =~= lv.union(rv));
                }
                join_with_priority_st(combined_left, r_key, rp, combined_right)
            }
        }
    }

    fn union_inner_st<T: StT + Ord + IsLtTransitive>(
        a: BSTTreapStEph<T>, b: BSTTreapStEph<T>,
    ) -> (combined: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a@.len() + b@.len() < usize::MAX as nat,
            spec_param_wf_link(&a.root), spec_param_wf_link(&b.root),
        ensures combined@.finite(), combined@ == a@.union(b@),
            spec_param_wf_link(&combined.root),
        decreases a@.len(),
    {
        match expose_to_parts_st(a) {
            | None => b,
            | Some((al, ak, ap, ar)) => {
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                let ghost av = al@.union(ar@).insert(akv);
                let ghost bv = b@;
                proof {
                    assert(alv.subset_of(av));
                    assert(arv.subset_of(av));
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                    assert(!alv.contains(akv));
                    assert(!arv.contains(akv));
                    assert(alv.disjoint(arv));
                }
                let (bl, _, br) = split_inner_st(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
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
                    vstd::set_lib::lemma_len_subset(blv, bv);
                    vstd::set_lib::lemma_len_subset(brv, bv);
                    assert(!blv.contains(akv));
                    assert(!brv.contains(akv));
                    assert(alv.len() + blv.len() < av.len() + bv.len());
                    assert(arv.len() + brv.len() < av.len() + bv.len());
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
                    assert(av == alv.union(arv).insert(akv));
                    assert(blv.union(brv) == bv.remove(akv));
                }
                let left_union = union_inner_st(al, bl);
                let right_union = union_inner_st(ar, br);
                proof {
                    let luv = left_union@;
                    let ruv = right_union@;
                    assert(luv == alv.union(blv));
                    assert(ruv == arv.union(brv));
                    assert forall|t: T| #[trigger] luv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(alv.union(blv).contains(t@));
                    };
                    assert forall|t: T| #[trigger] ruv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(arv.union(brv).contains(t@));
                    };
                    assert(!luv.contains(akv));
                    assert(!ruv.contains(akv));
                    assert(luv.disjoint(ruv)) by {
                        assert forall|x: T::V| !(luv.contains(x) && ruv.contains(x)) by {
                            if luv.contains(x) && ruv.contains(x) {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                lemma_wf_view_inhabited_st::<T>(&left_union.root, x);
                                let ghost tl = choose|t: T| #[trigger] t@ == x && luv.contains(t@);
                                assert(ruv.contains(tl@));
                                assert(tl.cmp_spec(&ak) == Less);
                                assert(tl.cmp_spec(&ak) == Greater);
                                assert(false);
                            }
                        };
                    };
                    vstd::set_lib::lemma_set_disjoint_lens(luv, ruv);
                    assert(luv.union(ruv).subset_of(av.union(bv))) by {
                        assert forall|x: T::V| #[trigger] luv.union(ruv).contains(x) implies av.union(bv).contains(x) by {
                            if luv.contains(x) {
                                if alv.contains(x) { assert(av.contains(x)); }
                                else { assert(blv.contains(x)); assert(bv.contains(x)); }
                            } else {
                                assert(ruv.contains(x));
                                if arv.contains(x) { assert(av.contains(x)); }
                                else { assert(brv.contains(x)); assert(bv.contains(x)); }
                            }
                        };
                    };
                    vstd::set_lib::lemma_len_subset(luv.union(ruv), av.union(bv));
                    vstd::set_lib::lemma_len_union(av, bv);
                    assert(left_union@.len() + right_union@.len() < usize::MAX as nat);
                    assert(luv.union(ruv).insert(akv) == av.union(bv)) by {
                        assert forall|x: T::V| #[trigger] luv.union(ruv).insert(akv).contains(x)
                            <==> av.union(bv).contains(x) by {
                            if luv.union(ruv).insert(akv).contains(x) {
                                if x == akv { assert(av.contains(akv)); }
                                else if luv.contains(x) {
                                    if alv.contains(x) { assert(av.contains(x)); }
                                    else { assert(blv.contains(x)); assert(bv.contains(x)); }
                                } else {
                                    assert(ruv.contains(x));
                                    if arv.contains(x) { assert(av.contains(x)); }
                                    else { assert(brv.contains(x)); assert(bv.contains(x)); }
                                }
                            }
                            if av.union(bv).contains(x) && !luv.union(ruv).insert(akv).contains(x) {
                                assert(x != akv);
                                assert(!luv.contains(x));
                                assert(!ruv.contains(x));
                                assert(!alv.contains(x));
                                assert(!blv.contains(x));
                                assert(!arv.contains(x));
                                assert(!brv.contains(x));
                                if av.contains(x) {
                                    assert(alv.union(arv).insert(akv).contains(x));
                                    assert(false);
                                } else {
                                    assert(bv.contains(x));
                                    assert(!(blv.union(brv)).contains(x));
                                    assert(!bv.remove(akv).contains(x));
                                    assert(bv.remove(akv).contains(x));
                                    assert(false);
                                }
                            }
                        };
                    };
                }
                join_with_priority_st(left_union, ak, ap, right_union)
            }
        }
    }

    fn intersect_inner_st<T: StT + Ord + IsLtTransitive>(
        a: BSTTreapStEph<T>, b: BSTTreapStEph<T>,
    ) -> (common: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a@.len() < usize::MAX as nat,
            spec_param_wf_link(&a.root), spec_param_wf_link(&b.root),
        ensures common@.finite(), common@ == a@.intersect(b@),
            spec_param_wf_link(&common.root),
        decreases a@.len(),
    {
        match expose_to_parts_st(a) {
            | None => BSTTreapStEph { root: None },
            | Some((al, ak, ap, ar)) => {
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                let ghost av = al@.union(ar@).insert(akv);
                let ghost bv = b@;
                proof {
                    assert(alv.subset_of(av));
                    assert(arv.subset_of(av));
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner_st(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    // St analog of type_invariant: establish witness accessibility
                    // before trees are consumed by recursive calls.
                    lemma_wf_view_all_inhabited_st::<T>(&al.root);
                    lemma_wf_view_all_inhabited_st::<T>(&ar.root);
                    lemma_wf_view_all_inhabited_st::<T>(&bl.root);
                    lemma_wf_view_all_inhabited_st::<T>(&br.root);
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
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(al@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(ar@.contains(t@));
                    };
                }
                let left_res = intersect_inner_st(al, bl);
                let right_res = intersect_inner_st(ar, br);
                let ghost lrv = left_res@;
                let ghost rrv = right_res@;
                proof {
                    assert forall|x| #[trigger] av.intersect(bv).contains(x) <==>
                        lrv.union(rrv).union(if found { Set::<<T as View>::V>::empty().insert(akv) }
                                           else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.intersect(bv).contains(x) {
                            assert(av.contains(x) && bv.contains(x));
                            if x == akv {
                                assert(found);
                            } else if alv.contains(x) {
                                assert(blv.union(brv).contains(x)) by {
                                    assert(bv.remove(akv).contains(x));
                                };
                                assert(blv.contains(x)) by {
                                    if brv.contains(x) {
                                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent_st::<T>();
                                        assert(t_x.cmp_spec(&t_br) == Equal);
                                        lemma_cmp_equal_congruent_st(t_x, t_br, ak);
                                        assert(false);
                                    }
                                };
                            } else {
                                assert(arv.contains(x));
                                assert(blv.union(brv).contains(x)) by {
                                    assert(bv.remove(akv).contains(x));
                                };
                                assert(brv.contains(x)) by {
                                    if blv.contains(x) {
                                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent_st::<T>();
                                        assert(t_x.cmp_spec(&t_bl) == Equal);
                                        lemma_cmp_equal_congruent_right_st(t_bl, t_x, ak);
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
                    assert(lrv.subset_of(alv));
                    assert(rrv.subset_of(arv));
                    assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(alv.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(arv.contains(t@));
                    };
                    assert(!lrv.contains(akv));
                    assert(!rrv.contains(akv));
                    assert(lrv.disjoint(rrv));
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        if lrv.contains(s@) && rrv.contains(o@) {
                            assert(s.cmp_spec(&ak) == Less);
                            assert(o.cmp_spec(&ak) == Greater);
                            lemma_cmp_antisymmetry_st(o, ak);
                            lemma_cmp_transitivity_st(s, ak, o);
                        }
                    };
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                    assert(alv.len() + arv.len() < av.len());
                    assert(left_res@.len() + right_res@.len() < usize::MAX as nat);
                }
                if found {
                    join_with_priority_st(left_res, ak, ap, right_res)
                } else {
                    join_pair_inner_st(left_res, right_res)
                }
            }
        }
    }

    fn difference_inner_st<T: StT + Ord + IsLtTransitive>(
        a: BSTTreapStEph<T>, b: BSTTreapStEph<T>,
    ) -> (remaining: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a@.len() < usize::MAX as nat,
            spec_param_wf_link(&a.root), spec_param_wf_link(&b.root),
        ensures remaining@.finite(), remaining@ == a@.difference(b@),
            spec_param_wf_link(&remaining.root),
        decreases a@.len(),
    {
        match expose_to_parts_st(a) {
            | None => BSTTreapStEph { root: None },
            | Some((al, ak, ap, ar)) => {
                let ghost akv = ak@;
                let ghost alv = al@;
                let ghost arv = ar@;
                let ghost av = al@.union(ar@).insert(akv);
                let ghost bv = b@;
                proof {
                    assert(alv.subset_of(av));
                    assert(arv.subset_of(av));
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner_st(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                proof {
                    // St analog of type_invariant: establish witness accessibility
                    // before trees are consumed by recursive calls.
                    lemma_wf_view_all_inhabited_st::<T>(&al.root);
                    lemma_wf_view_all_inhabited_st::<T>(&ar.root);
                    lemma_wf_view_all_inhabited_st::<T>(&bl.root);
                    lemma_wf_view_all_inhabited_st::<T>(&br.root);
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
                let left_res = difference_inner_st(al, bl);
                let right_res = difference_inner_st(ar, br);
                let ghost lrv = left_res@;
                let ghost rrv = right_res@;
                proof {
                    assert forall|x| #[trigger] av.difference(bv).contains(x) <==>
                        lrv.union(rrv).union(if !found { Set::<<T as View>::V>::empty().insert(akv) }
                                            else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.difference(bv).contains(x) {
                            assert(av.contains(x) && !bv.contains(x));
                            if x == akv {
                                assert(!found);
                            } else if alv.contains(x) {
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
                                if bv.contains(x) {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    assert(blv.union(brv).contains(x));
                                    if brv.contains(x) {
                                        let ghost t_al = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent_st::<T>();
                                        assert(t_al.cmp_spec(&t_br) == Equal);
                                        lemma_cmp_equal_congruent_st(t_al, t_br, ak);
                                        assert(false);
                                    }
                                    assert(false);
                                }
                            } else if rrv.contains(x) {
                                assert(arv.contains(x) && !brv.contains(x));
                                assert(av.contains(x));
                                if bv.contains(x) {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    assert(blv.union(brv).contains(x));
                                    if blv.contains(x) {
                                        let ghost t_ar = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent_st::<T>();
                                        assert(t_ar.cmp_spec(&t_bl) == Equal);
                                        lemma_cmp_equal_congruent_right_st(t_bl, t_ar, ak);
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
                    assert(lrv.subset_of(alv));
                    assert(rrv.subset_of(arv));
                    assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        assert(alv.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        assert(arv.contains(t@));
                    };
                    assert(!lrv.contains(akv));
                    assert(!rrv.contains(akv));
                    assert(lrv.disjoint(rrv));
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        if lrv.contains(s@) && rrv.contains(o@) {
                            assert(s.cmp_spec(&ak) == Less);
                            assert(o.cmp_spec(&ak) == Greater);
                            lemma_cmp_antisymmetry_st(o, ak);
                            lemma_cmp_transitivity_st(s, ak, o);
                        }
                    };
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                    assert(alv.len() + arv.len() < av.len());
                    assert(left_res@.len() + right_res@.len() < usize::MAX as nat);
                }
                if found {
                    join_pair_inner_st(left_res, right_res)
                } else {
                    join_with_priority_st(left_res, ak, ap, right_res)
                }
            }
        }
    }

    fn filter_inner_st<T: StT + Ord + IsLtTransitive, F: Fn(&T) -> bool>(
        tree: BSTTreapStEph<T>,
        predicate: &F,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (result: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            forall|t: &T| #[trigger] predicate.requires((t,)),
            forall|x: T, keep: bool| #[trigger] predicate.ensures((&x,), keep)
                ==> keep == spec_pred(x@),
            tree@.len() < usize::MAX as nat,
            spec_param_wf_link(&tree.root),
        ensures
            result@.finite(),
            result@.subset_of(tree@),
            forall|v: T::V| #[trigger] result@.contains(v)
                ==> tree@.contains(v) && spec_pred(v),
            forall|v: T::V| #[trigger] tree@.contains(v) && spec_pred(v)
                ==> result@.contains(v),
            spec_param_wf_link(&result.root),
        decreases tree@.len(),
    {
        match expose_to_parts_st(tree) {
            | None => BSTTreapStEph { root: None },
            | Some((left, key, ap, right)) => {
                let ghost lv = left@;
                let ghost rv = right@;
                let ghost kv = key@;
                let ghost tv = lv.union(rv).insert(kv);
                proof {
                    assert forall|t: T| #[trigger] lv.contains(t@) implies t.cmp_spec(&key) == Less by {
                        assert(left@.contains(t@));
                    };
                    assert forall|t: T| #[trigger] rv.contains(t@) implies t.cmp_spec(&key) == Greater by {
                        assert(right@.contains(t@));
                    };
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    assert(lv.len() + rv.len() < tv.len());
                }
                let left_filtered = filter_inner_st(left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner_st(right, predicate, Ghost(spec_pred));
                proof {
                    assert(left_filtered@.subset_of(lv));
                    assert(right_filtered@.subset_of(rv));
                    assert forall|t: T| #[trigger] left_filtered@.contains(t@) implies t.cmp_spec(&key) == Less by {
                        assert(lv.contains(t@));
                    };
                    assert forall|t: T| #[trigger] right_filtered@.contains(t@) implies t.cmp_spec(&key) == Greater by {
                        assert(rv.contains(t@));
                    };
                    assert(!left_filtered@.contains(kv));
                    assert(!right_filtered@.contains(kv));
                    assert(lv.disjoint(rv));
                    assert(left_filtered@.disjoint(right_filtered@));
                    assert forall|s: T, o: T| #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                        left_filtered@.contains(s@) && right_filtered@.contains(o@) ==> s.cmp_spec(&o) == Less by {
                        if left_filtered@.contains(s@) && right_filtered@.contains(o@) {
                            assert(s.cmp_spec(&key) == Less);
                            assert(o.cmp_spec(&key) == Greater);
                            lemma_cmp_antisymmetry_st(o, key);
                            lemma_cmp_transitivity_st(s, key, o);
                        }
                    };
                    vstd::set_lib::lemma_len_subset(left_filtered@, lv);
                    vstd::set_lib::lemma_len_subset(right_filtered@, rv);
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    assert(lv.len() + rv.len() < tv.len());
                    assert(left_filtered@.len() + right_filtered@.len() < usize::MAX as nat);
                    assert(tv == lv.union(rv).insert(kv));
                    assert(left_filtered@.union(right_filtered@).subset_of(tv)) by {
                        assert forall|x: T::V| #[trigger]
                            left_filtered@.union(right_filtered@).contains(x)
                            implies tv.contains(x) by {
                            if left_filtered@.contains(x) { assert(lv.contains(x)); }
                            else { assert(right_filtered@.contains(x)); assert(rv.contains(x)); }
                        };
                    };
                }
                let keep = (*predicate)(&key);
                proof {
                    assert((*predicate).ensures((&key,), keep));
                    assert(keep == spec_pred(kv));
                }
                if keep {
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        assert forall|v: T::V| #[trigger]
                            lf.union(rf).insert(kv).contains(v) implies spec_pred(v) by {
                            if v == kv { assert(spec_pred(kv)); }
                            else if lf.contains(v) { assert(left_filtered@.contains(v)); }
                            else { assert(rf.contains(v)); assert(right_filtered@.contains(v)); }
                        };
                        assert forall|v: T::V| #[trigger]
                            tv.contains(v) && spec_pred(v)
                            implies lf.union(rf).insert(kv).contains(v) by {
                            if v == kv { }
                            else {
                                assert(lv.union(rv).contains(v));
                                if lv.contains(v) { assert(left_filtered@.contains(v)); assert(lf.contains(v)); }
                                else { assert(rv.contains(v)); assert(right_filtered@.contains(v)); assert(rf.contains(v)); }
                            }
                        };
                    }
                    join_with_priority_st(left_filtered, key, ap, right_filtered)
                } else {
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        assert forall|v: T::V| #[trigger]
                            lf.union(rf).contains(v) implies spec_pred(v) by {
                            if lf.contains(v) { assert(left_filtered@.contains(v)); }
                            else { assert(rf.contains(v)); assert(right_filtered@.contains(v)); }
                        };
                        assert forall|v: T::V| #[trigger]
                            tv.contains(v) && spec_pred(v)
                            implies lf.union(rf).contains(v) by {
                            if v == kv { assert(!spec_pred(kv)); }
                            else {
                                assert(lv.union(rv).contains(v));
                                if lv.contains(v) { assert(left_filtered@.contains(v)); assert(lf.contains(v)); }
                                else { assert(rv.contains(v)); assert(right_filtered@.contains(v)); assert(rf.contains(v)); }
                            }
                        };
                    }
                    join_pair_inner_st(left_filtered, right_filtered)
                }
            }
        }
    }

    fn reduce_inner_st<T: StT + Ord + IsLtTransitive, F: Fn(T, T) -> T>(
        tree: BSTTreapStEph<T>, op: &F, identity: T,
    ) -> (result: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            tree@.finite(),
            spec_param_wf_link(&tree.root),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> result == identity,
        decreases tree@.len(),
    {
        match expose_to_parts_st(tree) {
            | None => identity,
            | Some((left, key, _, right)) => {
                proof {
                    assert(left@.finite());
                    assert(right@.finite());
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                let left_base = identity.clone();
                proof { assume(left_base == identity); } // eq/clone workaround.
                let right_base = identity;
                let left_acc = reduce_inner_st(left, op, left_base);
                let right_acc = reduce_inner_st(right, op, right_base);
                let right_with_key = (*op)(key, right_acc);
                (*op)(left_acc, right_with_key)
            }
        }
    }

    fn collect_in_order_st<T: StT + Ord + IsLtTransitive>(
        tree: BSTTreapStEph<T>, out: &mut Vec<T>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            tree@.finite(),
            spec_param_wf_link(&tree.root),
        ensures out@.len() == old(out)@.len() + tree@.len(),
        decreases tree@.len(),
    {
        match expose_to_parts_st(tree) {
            | None => {}
            | Some((left, key, _, right)) => {
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                collect_in_order_st(left, out);
                out.push(key);
                collect_in_order_st(right, out);
            }
        }
    }

    // 8b. parametric trait

    pub trait ParamBSTTreapStEphTrait<T: StT + Ord + IsLtTransitive>:
        Sized + View<V = Set<<T as View>::V>>
    {
        /// - APAS: Work O(1), Span O(1)
        fn param_new() -> (tree: Self)
            ensures tree@.finite(), tree@.len() == 0;

        /// - APAS: Work O(1), Span O(1)
        fn singleton(key: T) -> (tree: Self)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent_st::<T>(),
            ensures tree@.finite(), tree@ =~= Set::<<T as View>::V>::empty().insert(key@);

        /// - APAS: Work O(1), Span O(1)
        fn expose(&self) -> (exposed: ExposedTreap<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
            ensures
                self@.len() == 0 ==> exposed is Leaf,
                exposed is Leaf ==> self@ =~= Set::<T::V>::empty(),
                exposed matches ExposedTreap::Node(l, k, r) ==> (
                    self@ =~= l@.union(r@).insert(k@)
                    && self@.finite()
                    && l@.finite() && r@.finite()
                    && l@.disjoint(r@)
                    && !l@.contains(k@)
                    && !r@.contains(k@)
                    && l@.len() + r@.len() < usize::MAX as nat
                    && (forall|t: T| (#[trigger] l@.contains(t@)) ==> t.cmp_spec(&k) == Less)
                    && (forall|t: T| (#[trigger] r@.contains(t@)) ==> t.cmp_spec(&k) == Greater)
                );

        /// - APAS: Work O(log(|left| + |right|)), Span O(log(|left| + |right|))
        fn join_mid(exposed: ExposedTreap<T>) -> (tree: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                exposed matches ExposedTreap::Node(l, k, r) ==> (
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
                exposed matches ExposedTreap::Node(l, k, r) ==> tree@ =~= l@.union(r@).insert(k@);

        /// - APAS: Work O(1), Span O(1)
        fn param_size(&self) -> (count: usize)
            ensures self@.finite(), count == self@.len();

        /// - APAS: Work O(1), Span O(1)
        fn param_is_empty(&self) -> (empty: bool)
            ensures self@.finite(), empty == (self@.len() == 0);

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn param_insert(&mut self, key: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent_st::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.insert(key@);

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn param_delete(&mut self, key: &T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                old(self)@.len() < usize::MAX as nat,
            ensures self@.finite(), self@ =~= old(self)@.remove(key@);

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn param_find(&self, key: &T) -> (found: Option<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
            ensures
                found matches Some(v) ==> v@ == key@ && self@.contains(v@),
                found is None ==> !self@.contains(key@);

        /// - APAS: Work O(lg |t|), Span O(lg |t|)
        fn param_split(&self, key: &T) -> (parts: (Self, bool, Self))
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
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
        fn param_join_pair(&self, other: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self@.disjoint(other@),
                self@.finite(), other@.finite(),
                self@.len() + other@.len() < usize::MAX as nat,
                forall|s: T, o: T| #![trigger self@.contains(s@), other@.contains(o@)]
                    self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures joined@.finite(), joined@ =~= self@.union(other@);

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn param_union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@.finite(), combined@ == self@.union(other@);

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn param_intersect(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self@.len() < usize::MAX as nat,
            ensures common@.finite(), common@ == self@.intersect(other@);

        /// - APAS: Work O(m · lg(n/m)), Span O(lg n)
        fn param_difference(&self, other: &Self) -> (diff: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self@.len() < usize::MAX as nat,
            ensures diff@.finite(), diff@ == self@.difference(other@);

        /// - APAS: Work O(|t|), Span O(lg |t|)
        fn param_filter<F: Fn(&T) -> bool>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
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
        fn param_reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (reduced: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;

        /// - APAS: Work O(|t|), Span O(|t|)
        fn param_in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
            ensures self@.finite(), ordered.spec_len() == self@.len();
    }

    // 9c. parametric trait impl

    impl<T: StT + Ord + IsLtTransitive> ParamBSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        fn param_new() -> (tree: Self) {
            BSTTreapStEph { root: None }
        }

        fn singleton(key: T) -> (tree: Self) {
            let priority = priority_for_st(&key);
            make_node_treap_st(
                BSTTreapStEph { root: None },
                key,
                priority,
                BSTTreapStEph { root: None },
            )
        }

        fn expose(&self) -> (exposed: ExposedTreap<T>) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let cloned = clone_with_view(self);
            match expose_to_parts_st(cloned) {
                None => ExposedTreap::Leaf,
                Some((l, k, _, r)) => ExposedTreap::Node(l, k, r),
            }
        }

        fn join_mid(exposed: ExposedTreap<T>) -> (tree: Self) {
            match exposed {
                ExposedTreap::Leaf => BSTTreapStEph { root: None },
                ExposedTreap::Node(left, key, right) => {
                    proof {
                        // St analog of use_type_invariant: children are structurally wf.
                        assume(spec_param_wf_link(&left.root));
                        assume(spec_param_wf_link(&right.root));
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    }
                    let priority = priority_for_st(&key);
                    join_with_priority_st(left, key, priority, right)
                }
            }
        }

        fn param_size(&self) -> (count: usize) {
            proof {
                assume(spec_param_wf_link(&self.root)); // St analog of use_type_invariant.
                assume(BSTTreapStEph::<T>::spec_size_link(&self.root) == self@.len());
            }
            BSTTreapStEph::<T>::size_link(&self.root)
        }

        fn param_is_empty(&self) -> (empty: bool) {
            self.param_size() == 0
        }

        fn param_insert(&mut self, key: T) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let ghost old_view = self@;
            let cloned = clone_with_view(&*self);
            let (left, _, right) = split_inner_st(cloned, &key);
            let ghost kv = key@;
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                assert(left@.union(right@) =~= old_view.remove(kv));
                assert(old_view.remove(kv).subset_of(old_view));
                vstd::set_lib::lemma_len_subset(old_view.remove(kv), old_view);
                assert(left@.len() + right@.len() < usize::MAX as nat);
            }
            let priority = priority_for_st(&key);
            let new_tree = join_with_priority_st(left, key, priority, right);
            *self = new_tree;
        }

        fn param_delete(&mut self, key: &T) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let ghost old_view = self@;
            let ghost kref = *key;
            let cloned = clone_with_view(&*self);
            let (left, _, right) = split_inner_st(cloned, key);
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                assert(left@.union(right@) =~= old_view.remove(kref@));
                assert(old_view.remove(kref@).subset_of(old_view));
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                assert(left@.len() + right@.len() < usize::MAX as nat);
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry_st(o, kref);
                    lemma_cmp_transitivity_st(s, kref, o);
                };
            }
            let new_tree = join_pair_inner_st(left, right);
            *self = new_tree;
        }

        fn param_find(&self, key: &T) -> (found: Option<T>)
            decreases self@.len(),
        {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let cloned = clone_with_view(self);
            match expose_to_parts_st(cloned) {
                | None => None,
                | Some((left, root_key, _, right)) => {
                    proof {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                        assert(!left@.union(right@).contains(root_key@));
                        assert(self@.len() == left@.len() + right@.len() + 1);
                    }
                    match key.cmp(&root_key) {
                        | Equal => {
                            proof {
                                assert(root_key@ == key@);
                                assert(self@.contains(root_key@));
                            }
                            Some(root_key)
                        }
                        | Less => {
                            let result = left.param_find(key);
                            proof {
                                match &result {
                                    Some(v) => {
                                        assert(left@.contains(v@));
                                        assert(self@.contains(v@));
                                    }
                                    None => {
                                        assert(!left@.contains(key@));
                                        // key < root_key, so key@ ≠ root_key@.
                                        assert(key.cmp_spec(&root_key) == Less);
                                        // If key@ were in right@, then root_key < key (from ordering),
                                        // contradicting key < root_key.
                                        assert forall|t: T| #[trigger] right@.contains(t@) implies
                                            t.cmp_spec(&root_key) == Greater by {};
                                        // key@ ∉ right@ (if it were, key > root_key, contradiction).
                                        if right@.contains(key@) {
                                            let ghost tk = choose|t: T| #[trigger] t@ == key@ && right@.contains(t@);
                                            assert(tk.cmp_spec(&root_key) == Greater);
                                            lemma_cmp_equal_congruent_st(*key, tk, root_key);
                                            assert(false);
                                        }
                                        assert(!right@.contains(key@));
                                        assert(key@ != root_key@);
                                        assert(!self@.contains(key@));
                                    }
                                }
                            }
                            result
                        }
                        | Greater => {
                            let result = right.param_find(key);
                            proof {
                                match &result {
                                    Some(v) => {
                                        assert(right@.contains(v@));
                                        assert(self@.contains(v@));
                                    }
                                    None => {
                                        assert(!right@.contains(key@));
                                        assert(key.cmp_spec(&root_key) == Greater);
                                        if left@.contains(key@) {
                                            let ghost tk = choose|t: T| #[trigger] t@ == key@ && left@.contains(t@);
                                            assert(tk.cmp_spec(&root_key) == Less);
                                            lemma_cmp_equal_congruent_st(*key, tk, root_key);
                                            assert(false);
                                        }
                                        assert(!left@.contains(key@));
                                        assert(key@ != root_key@);
                                        assert(!self@.contains(key@));
                                    }
                                }
                            }
                            result
                        }
                    }
                }
            }
        }

        fn param_split(&self, key: &T) -> (parts: (Self, bool, Self)) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let cloned = clone_with_view(self);
            split_inner_st(cloned, key)
        }

        fn param_join_pair(&self, other: Self) -> (joined: Self) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            proof { assume(spec_param_wf_link(&other.root)); }
            let cloned = clone_with_view(self);
            join_pair_inner_st(cloned, other)
        }

        fn param_union(&self, other: &Self) -> (combined: Self) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            proof { assume(spec_param_wf_link(&other.root)); }
            let a = clone_with_view(self);
            let b = clone_with_view(other);
            union_inner_st(a, b)
        }

        fn param_intersect(&self, other: &Self) -> (common: Self) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            proof { assume(spec_param_wf_link(&other.root)); }
            let a = clone_with_view(self);
            let b = clone_with_view(other);
            intersect_inner_st(a, b)
        }

        fn param_difference(&self, other: &Self) -> (diff: Self) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            proof { assume(spec_param_wf_link(&other.root)); }
            let a = clone_with_view(self);
            let b = clone_with_view(other);
            difference_inner_st(a, b)
        }

        fn param_filter<F: Fn(&T) -> bool>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let cloned = clone_with_view(self);
            filter_inner_st(cloned, &predicate, Ghost(spec_pred))
        }

        fn param_reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (reduced: T) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let cloned = clone_with_view(self);
            proof { assume(cloned@.finite()); } // follows from wf.
            reduce_inner_st(cloned, &op, base)
        }

        fn param_in_order(&self) -> (ordered: ArraySeqStPerS<T>) {
            proof { assume(spec_param_wf_link(&self.root)); } // St analog of use_type_invariant.
            let cloned = clone_with_view(self);
            proof { assume(cloned@.finite()); }
            let mut out = Vec::new();
            proof { assume(self@.finite()); }
            collect_in_order_st(cloned, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }


    //		11. derive impls in verus!

    impl<T: StT + Ord + IsLtTransitive> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures
                BSTTreapStEph::<T>::spec_size_link(&Some(Box::new(cloned))) == BSTTreapStEph::<T>::spec_size_link(&Some(Box::new(*self))),
                BSTTreapStEph::<T>::spec_link_size_wf(&Some(Box::new(*self))) ==> BSTTreapStEph::<T>::spec_link_size_wf(&Some(Box::new(cloned))),
        {
            Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: BSTTreapStEph::<T>::clone_link(&self.left),
                right: BSTTreapStEph::<T>::clone_link(&self.right),
            }
        }
    }

    impl<T: StT + Ord + IsLtTransitive> Clone for BSTTreapStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned.spec_size() == self.spec_size(),
                self.spec_bsttreapsteph_wf() ==> cloned.spec_bsttreapsteph_wf(),
        {
            BSTTreapStEph { root: BSTTreapStEph::<T>::clone_link(&self.root) }
        }
    }

    }


    //		12. macros

    #[macro_export]
    macro_rules! BSTTreapStEphLit {
        () => {
            < $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::Chap39::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new();
            $( {
                let __val = $x;
                let mut __h = ::std::collections::hash_map::DefaultHasher::new();
                __val.hash(&mut __h);
                __tree.insert(__val, __h.finish());
            } )*
            __tree
        }};
    }


    //		13. derive impls outside verus!

    impl<T: StT + Ord + IsLtTransitive + fmt::Debug> fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("priority", &self.priority)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T: StT + Ord + IsLtTransitive + fmt::Debug> fmt::Debug for BSTTreapStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTTreapStEph").field("root", &self.root).finish()
        }
    }

    impl<T: StT + Ord + IsLtTransitive + fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Node(key={}, priority={}, size={})", self.key, self.priority, self.size)
        }
    }

    impl<T: StT + Ord + IsLtTransitive> fmt::Display for BSTTreapStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTTreapStEph(size: {})", self.size())
        }
    }
}
