//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Ephemeral Treap (randomized heap-ordered BST) with full parametric BST interface.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 8b. traits
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 6c. spec fns
//	Section 7c. proof fns/broadcast groups
//	Section 9c. impls
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module


pub mod BSTTreapStEph {


    //		Section 2. imports

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
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::IsLtTransitive;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger};

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::set::group_set_axioms,
        vstd::set_lib::group_set_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		Section 4. type definitions


    type Link<T> = Option<Box<Node<T>>>;

    //		Section 4a. type definitions


    pub struct Node<T: StT + Ord + IsLtTransitive> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    //		Section 4b. type definitions


    pub struct BSTTreapStEph<T: StT + Ord + IsLtTransitive> {
        pub root: Link<T>,
    }

    pub type BSTreeTreap<T> = BSTTreapStEph<T>;

    //		Section 5b. view impls


    impl<T: StT + Ord + IsLtTransitive> View for BSTTreapStEph<T> {
        type V = Set<T::V>;
        open spec fn view(&self) -> Set<T::V> { spec_set_view_link(&self.root) }
    }

    //		Section 8b. traits


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

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new()                       -> (empty_tree: Self)
        where
            Self: Sized,
            ensures
                empty_tree.spec_size() == 0,
                empty_tree.spec_bsttreapsteph_wf(),
                empty_tree.spec_bst();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self)                 -> (sz: usize)
            ensures sz as nat == self.spec_size();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self)             -> (empty: bool)
            ensures empty == (self.spec_size() == 0);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self)               -> (h: usize)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_bsttreapsteph_wf(),
            ensures h as nat == self.spec_height();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
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
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn delete(&mut self, target: &T)
            requires
                old(self).spec_bsttreapsteph_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                self.spec_bsttreapsteph_wf(),
                self.spec_size() <= old(self).spec_size(),
                forall|k: T| self.spec_contains(k) ==> old(self).spec_contains(k),
                old(self).spec_bst() ==> self.spec_bst();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn find(&self, target: &T)     -> (found: Option<&T>)
            requires
                self.spec_bsttreapsteph_wf(),
                self.spec_bst(),
                T::obeys_partial_cmp_spec(),
            ensures
                found.is_some() <==> self.spec_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bsttreapsteph_wf(),
                self.spec_bst(),
                T::obeys_partial_cmp_spec(),
            ensures found == self.spec_contains(*target);
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn minimum(&self)              -> (min_val: Option<&T>)
            ensures match (min_val, self.spec_min()) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg n) expected, Span O(lg n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn maximum(&self)              -> (max_val: Option<&T>)
            ensures match (max_val, self.spec_max()) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self)             -> (ordered: ArraySeqStPerS<T>)
            ensures ordered.spec_len() == self.spec_in_order().len();
        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self)            -> (preordered: ArraySeqStPerS<T>)
            ensures preordered.spec_len() == self.spec_pre_order().len();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new_node(key: T, priority: u64) -> (n: Node<T>)
            ensures
                Self::spec_link_size_wf(&Some(Box::new(n))),
                n.size == 1;

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(link: &Link<T>) -> (sz: usize)
            ensures sz as nat == Self::spec_size_link(link);

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn update_size(node: &mut Box<Node<T>>)
            requires 1 + Self::spec_size_link(&old(node).left) + Self::spec_size_link(&old(node).right) <= usize::MAX as nat,
            ensures
                node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right),
                node.key == old(node).key,
                node.left == old(node).left,
                node.right == old(node).right;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(x: Box<Node<T>>) -> (rotated: Box<Node<T>>)
            requires
                Self::spec_link_size_wf(&Some(x)),
                Self::spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                Self::spec_link_size_wf(&Some(rotated)),
                Self::spec_size_link(&Some(rotated)) == Self::spec_size_link(&Some(x)),
                Self::spec_bst_link(&Some(x)) ==> Self::spec_bst_link(&Some(rotated)),
                forall|k: T| spec_contains_link(&Some(rotated), k) <==> spec_contains_link(&Some(x), k);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(x: Box<Node<T>>) -> (rotated: Box<Node<T>>)
            requires
                Self::spec_link_size_wf(&Some(x)),
                Self::spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                Self::spec_link_size_wf(&Some(rotated)),
                Self::spec_size_link(&Some(rotated)) == Self::spec_size_link(&Some(x)),
                Self::spec_bst_link(&Some(x)) ==> Self::spec_bst_link(&Some(rotated)),
                forall|k: T| spec_contains_link(&Some(rotated), k) <==> spec_contains_link(&Some(x), k);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn clone_link(link: &Link<T>) -> (c: Link<T>)
            ensures
                Self::spec_size_link(&c) == Self::spec_size_link(link),
                Self::spec_link_size_wf(link) ==> Self::spec_link_size_wf(&c);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_link(link: &Link<T>) -> (h: usize)
            requires
                Self::spec_size_link(link) < usize::MAX as nat,
                Self::spec_link_size_wf(link),
            ensures h as nat == Self::spec_height_link(link);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn delete_link(link: Link<T>, target: &T) -> (deleted: Link<T>)
            requires
                Self::spec_link_size_wf(&link),
                T::obeys_partial_cmp_spec(),
            ensures
                Self::spec_link_size_wf(&deleted),
                Self::spec_size_link(&deleted) <= Self::spec_size_link(&link),
                forall|k: T| spec_contains_link(&deleted, k) ==> spec_contains_link(&link, k),
                Self::spec_bst_link(&link) ==> Self::spec_bst_link(&deleted);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            requires
                Self::spec_bst_link(link),
                T::obeys_partial_cmp_spec(),
            ensures
                found.is_some() <==> spec_contains_link(link, *target),
                found.is_some() ==> *found.unwrap() == *target;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn min_link(link: &Link<T>) -> (min_val: Option<&T>)
            ensures match (min_val, Self::spec_min_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn max_link(link: &Link<T>) -> (max_val: Option<&T>)
            ensures match (max_val, Self::spec_max_link(link)) {
                (Some(rv), Some(sv)) => *rv == sv,
                (None, None) => true,
                _ => false,
            };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            ensures ordered@.len() == Self::spec_in_order_link(link).len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order_vec(link: &Link<T>) -> (ordered: Vec<T>)
            ensures ordered@.len() == Self::spec_pre_order_link(link).len();

    }

    // 8b. parametric trait

    pub trait ParamBSTTreapStEphTrait<T: StT + Ord + IsLtTransitive>:
        Sized + View<V = Set<<T as View>::V>>
    {
        spec fn spec_parambsttreapsteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn param_new() -> (tree: Self)
            ensures tree@.finite(), tree@.len() == 0, tree.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: T) -> (tree: Self)
            requires vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent_st::<T>(),
            ensures
                tree@.finite(),
                tree@ =~= Set::<<T as View>::V>::empty().insert(key@),
                tree.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn expose(&self) -> (exposed: ExposedTreap<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
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
                    && spec_param_wf_link(&l.root)
                    && spec_param_wf_link(&r.root)
                );

        /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
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
                    && spec_param_wf_link(&l.root)
                    && spec_param_wf_link(&r.root)
                ),
            ensures
                tree@.finite(),
                tree.spec_parambsttreapsteph_wf(),
                exposed is Leaf ==> tree@ =~= Set::<T::V>::empty(),
                exposed matches ExposedTreap::Node(l, k, r) ==> tree@ =~= l@.union(r@).insert(k@);

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn param_size(&self) -> (count: usize)
            requires self.spec_parambsttreapsteph_wf(),
            ensures self@.finite(), count == self@.len();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn param_is_empty(&self) -> (empty: bool)
            requires self.spec_parambsttreapsteph_wf(),
            ensures self@.finite(), empty == (self@.len() == 0);

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn param_insert(&mut self, key: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent_st::<T>(),
                old(self).spec_parambsttreapsteph_wf(),
                old(self)@.len() < usize::MAX as nat,
            ensures
                self@.finite(),
                self@ =~= old(self)@.insert(key@),
                self.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn param_delete(&mut self, key: &T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                old(self).spec_parambsttreapsteph_wf(),
                old(self)@.len() < usize::MAX as nat,
            ensures
                self@.finite(),
                self@ =~= old(self)@.remove(key@),
                self.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn param_find(&self, key: &T) -> (found: Option<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
            ensures
                found matches Some(v) ==> v@ == key@ && self@.contains(v@),
                found is None ==> !self@.contains(key@);

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg |t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg |t|), Span O(lg |t|)
        fn param_split(&self, key: &T) -> (parts: (Self, bool, Self))
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
            ensures
                parts.0@.finite(), parts.2@.finite(),
                parts.1 == self@.contains(key@),
                self@.finite(),
                parts.0@.union(parts.2@) =~= self@.remove(key@),
                parts.0@.disjoint(parts.2@),
                !parts.0@.contains(key@) && !parts.2@.contains(key@),
                forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(key) == Less,
                forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(key) == Greater,
                parts.0.spec_parambsttreapsteph_wf(),
                parts.2.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t_1| + |t_2|)), Span O(lg(|t_1| + |t_2|))
        fn param_join_pair(&self, other: Self) -> (joined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
                other.spec_parambsttreapsteph_wf(),
                self@.disjoint(other@),
                self@.finite(), other@.finite(),
                self@.len() + other@.len() < usize::MAX as nat,
                forall|s: T, o: T| #![trigger self@.contains(s@), other@.contains(o@)]
                    self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less,
            ensures
                joined@.finite(),
                joined@ =~= self@.union(other@),
                joined.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)) — DIFFERS: St sequential, APAS parallel
        fn param_union(&self, other: &Self) -> (combined: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
                other.spec_parambsttreapsteph_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@.finite(),
                combined@ == self@.union(other@),
                combined.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)) — DIFFERS: St sequential, APAS parallel
        fn param_intersect(&self, other: &Self) -> (common: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
                other.spec_parambsttreapsteph_wf(),
                self@.len() < usize::MAX as nat,
            ensures
                common@.finite(),
                common@ == self@.intersect(other@),
                common.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(m · lg(n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m · lg(n/m)) — DIFFERS: St sequential, APAS parallel
        fn param_difference(&self, other: &Self) -> (diff: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
                other.spec_parambsttreapsteph_wf(),
                self@.len() < usize::MAX as nat,
            ensures
                diff@.finite(),
                diff@ == self@.difference(other@),
                diff.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|) — DIFFERS: St sequential, APAS parallel
        fn param_filter<F: Fn(&T) -> bool>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
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
                    ==> #[trigger] filtered@.contains(v),
                filtered.spec_parambsttreapsteph_wf();

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(lg |t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|) — DIFFERS: St sequential, APAS parallel
        fn param_reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (reduced: T)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                obeys_feq_clone::<T>(),
                self.spec_parambsttreapsteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(|t|), Span O(|t|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|t|), Span O(|t|)
        fn param_in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent_st::<T>(),
                self.spec_parambsttreapsteph_wf(),
            ensures self@.finite(), ordered.spec_len() == self@.len();
    }

    //		Section 9b. impls


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
                }
            }
        }

        proof fn lemma_size_wf_child_bounded(link: &Link<T>)
            decreases *link,
        {
            match link {
                None => {},
                Some(node) => {
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> Self { BSTTreapStEph { root: None } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> usize { Self::size_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> bool { self.size() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> usize {
            Self::height_link(&self.root)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn insert(&mut self, value: T, priority: u64) {
            self.root = Self::insert_link(self.root.take(), value, priority);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn delete(&mut self, target: &T) {
            self.root = Self::delete_link(self.root.take(), target);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn find(&self, target: &T) -> Option<&T> {
            Self::find_link(&self.root, target)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn contains(&self, target: &T) -> bool {
            self.find(target).is_some()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(Self::in_order_vec(&self.root))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::from_vec(Self::pre_order_vec(&self.root))
        }

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new_node(key: T, priority: u64) -> (n: Node<T>) {
            let n = Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            };
            // Veracity: NEEDED assert
            assert(Self::spec_link_size_wf(&n.left));
            // Veracity: NEEDED assert
            assert(Self::spec_link_size_wf(&n.right));
            n
        }

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(link: &Link<T>) -> (sz: usize) {
            match link.as_ref() {
                None => 0,
                Some(n) => n.size,
            }
        }

        /// - Alg Analysis: APAS (Ch39 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn update_size(node: &mut Box<Node<T>>) {
            let l = Self::size_link(&node.left);
            let r = Self::size_link(&node.right);
            node.size = 1 + l + r;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(mut x: Box<Node<T>>) -> (rotated: Box<Node<T>>) {
            let ghost bst_input = Self::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_right = x.right;
            // Veracity: NEEDED assert
            assert(Self::spec_link_size_wf(&x.left));
            // Veracity: NEEDED assert
            assert(Self::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.right.take() {
                let ghost yk = y.key;
                let ghost b  = y.left;
                let ghost c  = y.right;

                // Veracity: NEEDED assert
                assert(Self::spec_link_size_wf(&y.left));
                // Veracity: NEEDED assert
                assert(Self::spec_link_size_wf(&y.right));
                let ghost x_left_sz = Self::spec_size_link(&x.left);
                let ghost y_left_sz = Self::spec_size_link(&y.left);
                let ghost y_right_sz = Self::spec_size_link(&y.right);

                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_right);
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies xk.is_lt(&k) by {
                            Self::lemma_contains_left(&y, k);
                        };
                    }
                }

                x.right = y.left.take();
                Self::update_size(&mut x);

                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        // Veracity: NEEDED assert
                        assert(Self::spec_bst_link(&Some(x)));
                    }
                }

                y.left = Some(x);
                Self::update_size(&mut y);
                // Veracity: NEEDED proof block
                proof {
                    Self::lemma_wf_assemble_node(&y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_right);
                        Self::lemma_contains_root(&y);
                        Self::lemma_contains_root(&y);
                        // Veracity: NEEDED assert
                        assert(spec_contains_link(&orig_right, yk));
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&y.left, k) implies k.is_lt(&yk) by {
                            if spec_contains_link(&x.left, k) {
                                T::is_lt_transitive(k, xk, yk);
                            }
                            if spec_contains_link(&x.right, k) {
                            }
                        };
                    }
                }
                y
            } else {
                x
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(mut x: Box<Node<T>>) -> (rotated: Box<Node<T>>) {
            let ghost bst_input = Self::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_left = x.left;
            // Veracity: NEEDED assert
            assert(Self::spec_link_size_wf(&x.left));
            // Veracity: NEEDED assert
            assert(Self::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.left.take() {
                let ghost yk = y.key;
                let ghost b  = y.right;
                let ghost a  = y.left;

                // Veracity: NEEDED assert
                assert(Self::spec_link_size_wf(&y.left));
                // Veracity: NEEDED assert
                assert(Self::spec_link_size_wf(&y.right));
                let ghost x_right_sz = Self::spec_size_link(&x.right);
                let ghost y_left_sz = Self::spec_size_link(&y.left);
                let ghost y_right_sz = Self::spec_size_link(&y.right);

                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_left);
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies k.is_lt(&xk) by {
                            Self::lemma_contains_right(&y, k);
                        };
                    }
                }

                x.left = y.right.take();
                Self::update_size(&mut x);

                // Veracity: NEEDED proof block
                proof {
                    if bst_input {
                        // Veracity: NEEDED assert
                        assert(Self::spec_bst_link(&Some(x)));
                    }
                }

                y.right = Some(x);
                Self::update_size(&mut y);
                // Veracity: NEEDED proof block
                proof {
                    Self::lemma_wf_assemble_node(&y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        Self::lemma_bst_decompose(&orig_left);
                        Self::lemma_contains_root(&y);
                        // Veracity: NEEDED assert
                        assert(spec_contains_link(&orig_left, yk));
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] spec_contains_link(&y.right, k) implies yk.is_lt(&k) by {
                            if spec_contains_link(&x.right, k) {
                                T::is_lt_transitive(yk, xk, k);
                            }
                            if spec_contains_link(&x.left, k) {
                            }
                        };
                    }
                }
                y
            } else {
                x
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_link(link: &Link<T>) -> (h: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    // Veracity: NEEDED proof block
                    proof { Self::lemma_size_wf_child_bounded(link); }
                    let lh = Self::height_link(&node.left);
                    let rh = Self::height_link(&node.right);
                    let m = if lh >= rh { lh } else { rh };
                    // Veracity: NEEDED proof block
                    proof {
                        Self::lemma_height_le_size(&node.left);
                        Self::lemma_height_le_size(&node.right);
                    }
                    1 + m
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn insert_link(link: Link<T>, value: T, priority: u64) -> (inserted: Link<T>)
            decreases link,
        {
            // Veracity: NEEDED proof block
            proof { reveal_with_fuel(spec_contains_link, 3); }
            match link {
                None => {
                    let n = Box::new(Node { key: value, priority, size: 1, left: None, right: None });
                    // Veracity: NEEDED proof block
                    proof { Self::lemma_wf_assemble_node(&n); }
                    Some(n)
                },
                Some(mut node) => {
                    let ghost orig_key = node.key;
                    let ghost orig_left = node.left;
                    let ghost orig_right = node.right;
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert forall |k: T|
                            #[trigger] spec_contains_link(&link, k) <==>
                            (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                            by {};
                    }
                    // Veracity: NEEDED assert
                    assert(Self::spec_link_size_wf(&node.left));
                    // Veracity: NEEDED assert
                    assert(Self::spec_link_size_wf(&node.right));
                    if value < node.key {
                        node.left = Self::insert_link(node.left.take(), value, priority);
                        Self::update_size(&mut node);
                        // Veracity: NEEDED proof block
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            // Inserted value is in the left subtree, hence in the tree.
                            Self::lemma_contains_left(&node, value);
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
                            }
                        }
                        let needs_rotate = match &node.left {
                            Some(l) => l.priority < node.priority,
                            None => false,
                        };
                        if needs_rotate {
                            let ghost pre_rotate = node;
                            let rotated = Self::rotate_right(node);
                            // Veracity: NEEDED proof block
                            proof {
                                // Rotation preserves containment; value was in pre_rotate.
                            }
                            Some(rotated)
                        } else { Some(node) }
                    } else if node.key < value {
                        node.right = Self::insert_link(node.right.take(), value, priority);
                        Self::update_size(&mut node);
                        // Veracity: NEEDED proof block
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            // Inserted value is in the right subtree, hence in the tree.
                            Self::lemma_contains_right(&node, value);
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
                            }
                        }
                        let needs_rotate = match &node.right {
                            Some(r) => r.priority < node.priority,
                            None => false,
                        };
                        if needs_rotate {
                            let ghost pre_rotate = node;
                            let rotated = Self::rotate_left(node);
                            // Veracity: NEEDED proof block
                            proof {
                            }
                            Some(rotated)
                        } else { Some(node) }
                    } else {
                        // !(value < node.key) && !(node.key < value) => structurally equal.
                        // Veracity: NEEDED proof block
                        proof {
                            T::is_lt_antisymmetric(value, node.key);
                            Self::lemma_contains_root(&node);
                        }
                        Some(node)
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn delete_link(link: Link<T>, target: &T) -> (deleted: Link<T>)
            decreases Self::spec_size_link(&link),
        {
            // Veracity: NEEDED proof block
            proof { reveal_with_fuel(spec_contains_link, 3); }
            match link {
                None => None,
                Some(mut node) => {
                    let ghost orig_key = node.key;
                    let ghost orig_left = node.left;
                    let ghost orig_right = node.right;
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert forall |k: T|
                            #[trigger] spec_contains_link(&link, k) <==>
                            (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                            by {};
                    }
                    // Veracity: NEEDED assert
                    assert(Self::spec_link_size_wf(&node.left));
                    // Veracity: NEEDED assert
                    assert(Self::spec_link_size_wf(&node.right));
                    if *target < node.key {
                        // Target in left subtree.
                        node.left = Self::delete_link(node.left.take(), target);
                        Self::update_size(&mut node);
                        // Veracity: NEEDED proof block
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
                            }
                        }
                        Some(node)
                    } else if node.key < *target {
                        // Target in right subtree.
                        node.right = Self::delete_link(node.right.take(), target);
                        Self::update_size(&mut node);
                        // Veracity: NEEDED proof block
                        proof {
                            Self::lemma_wf_assemble_node(&node);
                            if Self::spec_bst_link(&link) {
                                Self::lemma_bst_decompose(&link);
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
                            // Veracity: NEEDED proof block
                            proof {
                                if Self::spec_bst_link(&link) {
                                    Self::lemma_bst_decompose(&link);
                                }
                            }
                            node.left.take()
                        } else if node.left.is_none() {
                            // Only right child.
                            // Veracity: NEEDED proof block
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
                                // Veracity: NEEDED proof block
                                proof {
                                    // Veracity: NEEDED assert
                                    assert forall |k: T|
                                        #[trigger] spec_contains_link(&link, k) <==>
                                        (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                        by {};
                                    if Self::spec_bst_link(&link) {
                                        Self::lemma_bst_decompose(&Some(rotated));
                                    }
                                }
                                // Veracity: NEEDED assert
                                assert(Self::spec_link_size_wf(&Some(rotated)));
                                // Veracity: NEEDED assert
                                assert(Self::spec_link_size_wf(&rotated.right));
                                rotated.right = Self::delete_link(rotated.right.take(), target);
                                Self::update_size(&mut rotated);
                                // Veracity: NEEDED proof block
                                proof {
                                    Self::lemma_wf_assemble_node(&rotated);
                                    if Self::spec_bst_link(&link) {
                                    }
                                }
                                Some(rotated)
                            } else {
                                let mut rotated = Self::rotate_left(node);
                                let ghost rot_key = rotated.key;
                                let ghost rot_left = rotated.left;
                                let ghost rot_right = rotated.right;
                                // Veracity: NEEDED proof block
                                proof {
                                    // Veracity: NEEDED assert
                                    assert forall |k: T|
                                        #[trigger] spec_contains_link(&link, k) <==>
                                        (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                        by {};
                                    if Self::spec_bst_link(&link) {
                                        Self::lemma_bst_decompose(&Some(rotated));
                                    }
                                }
                                // Veracity: NEEDED assert
                                assert(Self::spec_link_size_wf(&Some(rotated)));
                                // Veracity: NEEDED assert
                                assert(Self::spec_link_size_wf(&rotated.left));
                                rotated.left = Self::delete_link(rotated.left.take(), target);
                                Self::update_size(&mut rotated);
                                // Veracity: NEEDED proof block
                                proof {
                                    Self::lemma_wf_assemble_node(&rotated);
                                    if Self::spec_bst_link(&link) {
                                    }
                                }
                                Some(rotated)
                            }
                        }
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            decreases *link,
        {
            // Veracity: NEEDED proof block
            proof { reveal_with_fuel(spec_contains_link, 2); }
            match link {
                | None => None,
                | Some(node) => {
                    // Veracity: NEEDED proof block
                    proof { Self::lemma_bst_decompose(link); }
                    if *target < node.key {
                        let r = Self::find_link(&node.left, target);
                        // Veracity: NEEDED proof block
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
                            }
                        }
                        r
                    } else if node.key < *target {
                        let r = Self::find_link(&node.right, target);
                        // Veracity: NEEDED proof block
                        proof {
                            // Forward: found in subtree → in whole tree.
                            if r.is_some() {
                                Self::lemma_contains_right(node, *target);
                            }
                            // Reverse: in whole tree → must be in right subtree → found.
                            T::is_lt_irreflexive(*target);
                            if spec_contains_link(link, *target) {
                                // Veracity: NEEDED assert
                                assert(!spec_contains_link(&node.left, *target));
                            }
                        }
                        r
                    } else {
                        // Neither target < node.key nor node.key < target.
                        // Veracity: NEEDED proof block
                        proof {
                            T::is_lt_antisymmetric(*target, node.key);
                            Self::lemma_contains_root(node);
                        }
                        Some(&node.key)
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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

    // 9c. parametric trait impl

    impl<T: StT + Ord + IsLtTransitive> ParamBSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        open spec fn spec_parambsttreapsteph_wf(&self) -> bool {
            spec_param_wf_link(&self.root)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn param_new() -> (tree: Self) {
            BSTTreapStEph { root: None }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: T) -> (tree: Self) {
            let priority = priority_for_st(&key);
            make_node_treap_st(
                BSTTreapStEph { root: None },
                key,
                priority,
                BSTTreapStEph { root: None },
            )
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn expose(&self) -> (exposed: ExposedTreap<T>) {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let cloned = clone_with_view(self);
            match expose_to_parts_st(cloned) {
                None => ExposedTreap::Leaf,
                Some((l, k, _, r)) => ExposedTreap::Node(l, k, r),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn join_mid(exposed: ExposedTreap<T>) -> (tree: Self) {
            match exposed {
                ExposedTreap::Leaf => BSTTreapStEph { root: None },
                ExposedTreap::Node(left, key, right) => {
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_param_wf_implies_size_wf::<T>(&left.root);
                        lemma_param_wf_implies_size_wf::<T>(&right.root);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    }
                    let priority = priority_for_st(&key);
                    join_with_priority_st(left, key, priority, right)
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn param_size(&self) -> (count: usize) {
            // Veracity: NEEDED proof block
            proof {
                lemma_wf_implies_finite(&self.root);
                lemma_wf_size_eq_view_len(&self.root);
            }
            BSTTreapStEph::<T>::size_link(&self.root)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn param_is_empty(&self) -> (empty: bool) {
            self.param_size() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn param_insert(&mut self, key: T) {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let ghost old_view = self@;
            let cloned = clone_with_view(&*self);
            let (left, _, right) = split_inner_st(cloned, &key);
            let ghost kv = key@;
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(old_view.remove(kv), old_view);
            }
            let priority = priority_for_st(&key);
            let new_tree = join_with_priority_st(left, key, priority, right);
            *self = new_tree;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn param_delete(&mut self, key: &T) {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let ghost old_view = self@;
            let ghost kref = *key;
            let cloned = clone_with_view(&*self);
            let (left, _, right) = split_inner_st(cloned, key);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                vstd::set_lib::lemma_len_subset(old_view.remove(kref@), old_view);
                // Veracity: NEEDED assert
                assert forall|s: T, o: T| #![trigger left@.contains(s@), right@.contains(o@)]
                    left@.contains(s@) && right@.contains(o@) implies s.cmp_spec(&o) == Less by {
                    lemma_cmp_antisymmetry_st(o, kref);
                    lemma_cmp_transitivity_st(s, kref, o);
                };
            }
            let new_tree = join_pair_inner_st(left, right);
            *self = new_tree;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn param_find(&self, key: &T) -> (found: Option<T>)
            decreases self@.len(),
        {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let cloned = clone_with_view(self);
            match expose_to_parts_st(cloned) {
                | None => None,
                | Some((left, root_key, _, right)) => {
                    // Veracity: NEEDED proof block
                    proof {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    }
                    match key.cmp(&root_key) {
                        | Equal => {
                            // Veracity: NEEDED proof block
                            proof {
                            }
                            Some(root_key)
                        }
                        | Less => {
                            let result = left.param_find(key);
                            // Veracity: NEEDED proof block
                            proof {
                                match &result {
                                    Some(v) => {
                                    }
                                    None => {
                                        // key < root_key, so key@ ≠ root_key@.
                                        // If key@ were in right@, then root_key < key (from ordering),
                                        // contradicting key < root_key.
                                        // Veracity: NEEDED assert
                                        assert forall|t: T| #[trigger] right@.contains(t@) implies
                                            t.cmp_spec(&root_key) == Greater by {};
                                        // key@ ∉ right@ (if it were, key > root_key, contradiction).
                                        if right@.contains(key@) {
                                            let ghost tk = choose|t: T| #[trigger] t@ == key@ && right@.contains(t@);
                                            lemma_cmp_equal_congruent_st(*key, tk, root_key);
                                        }
                                    }
                                }
                            }
                            result
                        }
                        | Greater => {
                            let result = right.param_find(key);
                            // Veracity: NEEDED proof block
                            proof {
                                match &result {
                                    Some(v) => {
                                    }
                                    None => {
                                        if left@.contains(key@) {
                                            let ghost tk = choose|t: T| #[trigger] t@ == key@ && left@.contains(t@);
                                            lemma_cmp_equal_congruent_st(*key, tk, root_key);
                                        }
                                    }
                                }
                            }
                            result
                        }
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn param_split(&self, key: &T) -> (parts: (Self, bool, Self)) {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let cloned = clone_with_view(self);
            split_inner_st(cloned, key)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
        fn param_join_pair(&self, other: Self) -> (joined: Self) {
            // Veracity: NEEDED proof block
            proof {
                lemma_param_wf_implies_size_wf::<T>(&self.root);
                lemma_param_wf_implies_size_wf::<T>(&other.root);
            }
            let cloned = clone_with_view(self);
            join_pair_inner_st(cloned, other)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
        fn param_union(&self, other: &Self) -> (combined: Self) {
            // Veracity: NEEDED proof block
            proof {
                lemma_param_wf_implies_size_wf::<T>(&self.root);
                lemma_param_wf_implies_size_wf::<T>(&other.root);
            }
            let a = clone_with_view(self);
            let b = clone_with_view(other);
            union_inner_st(a, b)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
        fn param_intersect(&self, other: &Self) -> (common: Self) {
            // Veracity: NEEDED proof block
            proof {
                lemma_param_wf_implies_size_wf::<T>(&self.root);
                lemma_param_wf_implies_size_wf::<T>(&other.root);
            }
            let a = clone_with_view(self);
            let b = clone_with_view(other);
            intersect_inner_st(a, b)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
        fn param_difference(&self, other: &Self) -> (diff: Self) {
            // Veracity: NEEDED proof block
            proof {
                lemma_param_wf_implies_size_wf::<T>(&self.root);
                lemma_param_wf_implies_size_wf::<T>(&other.root);
            }
            let a = clone_with_view(self);
            let b = clone_with_view(other);
            difference_inner_st(a, b)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
        fn param_filter<F: Fn(&T) -> bool>(
            &self,
            predicate: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self) {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let cloned = clone_with_view(self);
            filter_inner_st(cloned, &predicate, Ghost(spec_pred))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn param_reduce<F: Fn(T, T) -> T>(&self, op: F, base: T) -> (reduced: T) {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let cloned = clone_with_view(self);
            // Veracity: NEEDED proof block
            proof { lemma_wf_implies_finite(&cloned.root); }
            reduce_inner_st(cloned, &op, base)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn param_in_order(&self) -> (ordered: ArraySeqStPerS<T>) {
            // Veracity: NEEDED proof block
            proof { lemma_param_wf_implies_size_wf::<T>(&self.root); }
            let cloned = clone_with_view(self);
            // Veracity: NEEDED proof block
            proof {
                lemma_wf_implies_finite(&cloned.root);
                lemma_wf_implies_finite(&self.root);
            }
            let mut out = Vec::new();
            collect_in_order_st(cloned, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    pub enum ExposedTreap<T: StT + Ord + IsLtTransitive> {
        Leaf,
        Node(BSTTreapStEph<T>, T, BSTTreapStEph<T>),
    }

    //		Section 6c. spec fns


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

    //		Section 7c. proof fns/broadcast groups


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
        // Veracity: NEEDED assert
        assert forall|t: T| (#[trigger] joined_v.contains(t@)) implies t.cmp_spec(&lk) == Greater by {
            if lrv.contains(t@) {
            } else if right_v.contains(t@) {
                lemma_cmp_antisymmetry_st(t, key);
                lemma_cmp_transitivity_st(lk, key, t);
                lemma_cmp_antisymmetry_less_st(lk, t);
            } else {
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
        // Veracity: NEEDED assert
        assert forall|t: T| (#[trigger] joined_v.contains(t@)) implies t.cmp_spec(&rk) == Less by {
            if left_v.contains(t@) {
                lemma_cmp_antisymmetry_st(rk, key);
                lemma_cmp_transitivity_st(t, key, rk);
            } else if rlv.contains(t@) {
            } else {
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
                } else if lv.contains(x) {
                    lemma_wf_view_inhabited_st::<T>(&node.left, x);
                } else {
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
        // Veracity: NEEDED assert
        assert forall|x: <T as View>::V| #[trigger] spec_set_view_link(link).contains(x)
            implies (exists|t: T| t@ == x) by {
            lemma_wf_view_inhabited_st::<T>(link, x);
        };
    }

    /// Well-formed link implies the set view is finite.
    pub proof fn lemma_wf_implies_finite<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
        requires spec_param_wf_link(link),
        ensures spec_set_view_link(link).finite(),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_wf_implies_finite(&node.left);
                lemma_wf_implies_finite(&node.right);
            }
        }
    }

    /// Well-formed link implies size_link == view.len().
    pub proof fn lemma_wf_size_eq_view_len<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
        requires spec_param_wf_link(link),
        ensures BSTTreapStEph::<T>::spec_size_link(link) == spec_set_view_link(link).len(),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_wf_size_eq_view_len(&node.left);
                lemma_wf_size_eq_view_len(&node.right);
                let lv = spec_set_view_link(&node.left);
                let rv = spec_set_view_link(&node.right);
                let kv = node.key@;
                vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
            }
        }
    }

    /// spec_param_wf_link implies spec_link_size_wf (the module-level wf predicate).
    pub proof fn lemma_param_wf_implies_size_wf<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
        requires spec_param_wf_link(link),
        ensures BSTTreapStEph::<T>::spec_link_size_wf(link),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_param_wf_implies_size_wf(&node.left);
                lemma_param_wf_implies_size_wf(&node.right);
                lemma_wf_size_eq_view_len(&node.left);
                lemma_wf_size_eq_view_len(&node.right);
            }
        }
    }

    //		Section 9c. impls


    /// Clone a StT element with a cmp_spec-preserving postcondition.
    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn clone_elem_st<T: StT + Ord + IsLtTransitive>(x: &T) -> (c: T)
        ensures c@ == x@,
    {
        let c = x.clone();
        // Veracity: NEEDED proof block
        proof { accept(c@ == x@); } // eq/clone workaround: structural copy preserves view.
        c
    }

    /// Clone a BSTTreapStEph with view preservation.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn clone_with_view<T: StT + Ord + IsLtTransitive>(tree: &BSTTreapStEph<T>) -> (cloned: BSTTreapStEph<T>)
        requires spec_param_wf_link(&tree.root), tree.spec_bsttreapsteph_wf(),
        ensures cloned@ =~= tree@, spec_param_wf_link(&cloned.root), cloned.spec_bsttreapsteph_wf(),
    {
        let cloned = tree.clone();
        // Veracity: NEEDED proof block
        proof {
            accept(cloned@ =~= tree@ && spec_param_wf_link(&cloned.root)); // eq/clone workaround.
            lemma_param_wf_implies_size_wf::<T>(&cloned.root);
        }
        cloned
    }

    /// Hash-based priority for treap heap ordering.
    #[verifier::external_body]
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn priority_for_st<T: StT + Ord + IsLtTransitive>(key: &T) -> (p: u64) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buf = String::new();
        let _ = std::fmt::Write::write_fmt(&mut buf, format_args!("{key:?}"));
        Hash::hash(&buf, &mut hasher);
        hasher.finish()
    }

    // 9b. parametric internal functions

    /// Build a new tree from (left, key, priority, right) maintaining BST ordering.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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
            left.spec_bsttreapsteph_wf(), right.spec_bsttreapsteph_wf(),
        ensures
            node@ =~= left@.union(right@).insert(key@),
            node@.finite(),
            spec_param_wf_link(&node.root),
            node.spec_bsttreapsteph_wf(),
    {
        let ls = BSTTreapStEph::<T>::size_link(&left.root);
        let rs = BSTTreapStEph::<T>::size_link(&right.root);
        // Veracity: NEEDED proof block
        proof {
            lemma_wf_size_eq_view_len(&left.root);
            lemma_wf_size_eq_view_len(&right.root);
        }
        let size = 1 + ls + rs;
        // Veracity: NEEDED proof block
        proof {
            vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn tree_priority_st<T: StT + Ord + IsLtTransitive>(tree: &BSTTreapStEph<T>) -> (p: u64)
        requires tree.spec_bsttreapsteph_wf(),
        ensures true,
    {
        match &tree.root {
            None => 0u64,
            Some(node) => node.priority,
        }
    }

    /// Expose a tree into its constituent parts. Consumes the tree.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn expose_to_parts_st<T: StT + Ord + IsLtTransitive>(tree: BSTTreapStEph<T>) -> (parts: Option<(BSTTreapStEph<T>, T, u64, BSTTreapStEph<T>)>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            spec_param_wf_link(&tree.root),
            tree.spec_bsttreapsteph_wf(),
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
                && l.spec_bsttreapsteph_wf()
                && r.spec_bsttreapsteph_wf()
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
    /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
    fn join_with_priority_st<T: StT + Ord + IsLtTransitive>(
        left: BSTTreapStEph<T>, key: T, priority: u64, right: BSTTreapStEph<T>,
    ) -> (joined: BSTTreapStEph<T>)
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
            left.spec_bsttreapsteph_wf(), right.spec_bsttreapsteph_wf(),
        ensures joined@ =~= left@.union(right@).insert(key@), joined@.finite(),
            spec_param_wf_link(&joined.root),
            joined.spec_bsttreapsteph_wf(),
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
                    // Veracity: NEEDED proof block
                    proof {
                        // lr ⊆ left, so lr < key and lr.disjoint(right).
                        vstd::set_lib::lemma_len_subset(lrv, ll@.union(lr@).insert(lkv));
                    }
                    let merged_right = join_with_priority_st(lr, key, priority, right);
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_joined_right_gt_lk_st(lrv, right@, key, merged_right@, lk, ll@.union(lrv).insert(lkv));
                        vstd::set_lib::lemma_set_disjoint_lens(llv, lrv);
                        vstd::set_lib::lemma_set_disjoint_lens(lrv, right@);
                        vstd::set_lib::lemma_len_subset(llv, ll@.union(lr@).insert(lkv));
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
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::set_lib::lemma_len_subset(rlv, rl@.union(rr@).insert(rkv));
                    }
                    let merged_left = join_with_priority_st(left, key, priority, rl);
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_joined_left_lt_rk_st(left@, rlv, key, merged_left@, rk, rl@.union(rr@).insert(rkv));
                        vstd::set_lib::lemma_set_disjoint_lens(left@, rlv);
                        vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                        vstd::set_lib::lemma_len_subset(rrv, rl@.union(rr@).insert(rkv));
                    }
                    make_node_treap_st(merged_left, rk, rp, rr)
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst case
    fn split_inner_st<T: StT + Ord + IsLtTransitive>(
        tree: BSTTreapStEph<T>, key: &T,
    ) -> (parts: (BSTTreapStEph<T>, bool, BSTTreapStEph<T>))
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            spec_param_wf_link(&tree.root),
            tree.spec_bsttreapsteph_wf(),
        ensures
            parts.1 == tree@.contains(key@),
            parts.0@.finite(), parts.2@.finite(),
            parts.0@.union(parts.2@) =~= tree@.remove(key@),
            parts.0@.disjoint(parts.2@),
            !parts.0@.contains(key@) && !parts.2@.contains(key@),
            forall|t: T| (#[trigger] parts.0@.contains(t@)) ==> t.cmp_spec(key) == Less,
            forall|t: T| (#[trigger] parts.2@.contains(t@)) ==> t.cmp_spec(key) == Greater,
            spec_param_wf_link(&parts.0.root), spec_param_wf_link(&parts.2.root),
            parts.0.spec_bsttreapsteph_wf(), parts.2.spec_bsttreapsteph_wf(),
        decreases tree@.len(),
    {
        let empty = BSTTreapStEph::<T> { root: None };
        match expose_to_parts_st(tree) {
            | None => (BSTTreapStEph { root: None }, false, BSTTreapStEph { root: None }),
            | Some((left, root_key, root_pri, right)) => {
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
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
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall|x| lrv.contains(x) implies lv.contains(x) by {
                                // Veracity: NEEDED assert
                                assert(llv.union(lrv).contains(x));
                            };
                            // Veracity: NEEDED assert
                            assert forall|x| llv.contains(x) implies lv.contains(x) by {
                                // Veracity: NEEDED assert
                                assert(llv.union(lrv).contains(x));
                            };
                            vstd::set_lib::lemma_len_subset(lrv, lv);
                        }
                        let rebuilt = join_with_priority_st(lr, root_key, root_pri, right);
                        // Veracity: NEEDED proof block
                        proof {
                            reveal(vstd::laws_cmp::obeys_cmp_ord);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            // key < root_key, all rv elements > root_key, so key ∉ rv.
                            if rv.contains(key@) {
                                let ghost t = choose|t: T| t@ == key@ && #[trigger] rv.contains(t@);
                                lemma_cmp_equal_congruent_st(kval, t, rk);
                            }
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] (llv.union(rebuilt@)).contains(x)
                                <==> tree@.remove(key@).contains(x) by {
                                if llv.contains(x) {
                                }
                                if lv.contains(x) && x != key@ {
                                    // Veracity: NEEDED assert
                                    assert(llv.union(lrv).contains(x));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(key) == Greater by {
                                if lrv.contains(t@) {
                                    // Recursive split ensures t > key.
                                } else if rv.contains(t@) {
                                    lemma_cmp_antisymmetry_st(t, rk);
                                    lemma_cmp_transitivity_st(kval, rk, t);
                                } else {
                                    lemma_cmp_eq_subst_st(kval, rk, t);
                                }
                            };
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
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall|x| rlv.contains(x) implies rv.contains(x) by {
                                // Veracity: NEEDED assert
                                assert(rlv.union(rrv).contains(x));
                            };
                            // Veracity: NEEDED assert
                            assert forall|x| rrv.contains(x) implies rv.contains(x) by {
                                // Veracity: NEEDED assert
                                assert(rlv.union(rrv).contains(x));
                            };
                            vstd::set_lib::lemma_len_subset(rlv, rv);
                        }
                        let rebuilt = join_with_priority_st(left, root_key, root_pri, rl);
                        // Veracity: NEEDED proof block
                        proof {
                            reveal(vstd::laws_cmp::obeys_cmp_ord);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            // key > root_key, all lv elements < root_key, so key ∉ lv.
                            if lv.contains(key@) {
                                let ghost t = choose|t: T| t@ == key@ && #[trigger] lv.contains(t@);
                                lemma_cmp_equal_congruent_st(kval, t, rk);
                            }
                            // Veracity: NEEDED assert
                            assert forall|x| #[trigger] (rebuilt@.union(rrv)).contains(x)
                                <==> tree@.remove(key@).contains(x) by {
                                if rrv.contains(x) {
                                }
                                if rv.contains(x) && x != key@ {
                                    // Veracity: NEEDED assert
                                    assert(rlv.union(rrv).contains(x));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] rebuilt@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                if rlv.contains(t@) {
                                    // Recursive split ensures t < key.
                                } else if lv.contains(t@) {
                                    lemma_cmp_antisymmetry_st(kval, rk);
                                    lemma_cmp_transitivity_st(t, rk, kval);
                                } else {
                                    lemma_cmp_antisymmetry_st(kval, rk);
                                    lemma_cmp_equal_congruent_st(t, rk, kval);
                                }
                            };
                        }
                        (rebuilt, found, rr)
                    }
                    | Equal => {
                        // Veracity: NEEDED proof block
                        proof {
                            reveal(vstd::laws_cmp::obeys_cmp_ord);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            // key.cmp_spec(&root_key) == Equal, so kval@ == rk@.
                            // Veracity: NEEDED assert
                            assert forall|t: T| (#[trigger] left@.contains(t@)) implies
                                t.cmp_spec(key) == Less by {
                                lemma_cmp_equal_congruent_right_st(t, kval, rk);
                            };
                            // Veracity: NEEDED assert
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

    /// - Alg Analysis: APAS (Ch39 DS 39.3): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg(|t1|+|t2|)), Span O(lg(|t1|+|t2|))
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
            left.spec_bsttreapsteph_wf(), right.spec_bsttreapsteph_wf(),
        ensures joined@.finite(), joined@ =~= left@.union(right@),
            spec_param_wf_link(&joined.root),
            joined.spec_bsttreapsteph_wf(),
        decreases left@.len() + right@.len(),
    {
        let ghost lv = left@;
        let ghost rv = right@;
        // Veracity: NEEDED proof block
        proof {
            // St analog of type_invariant witness accessibility.
            lemma_wf_view_all_inhabited_st::<T>(&left.root);
            // Derive lv.disjoint(rv) from strict ordering + inhabitedness.
        }
        match expose_to_parts_st(right) {
            | None => left,
            | Some((r_left, r_key, rp, r_right)) => {
                let ghost rkv = r_key@;
                let ghost rlv = r_left@;
                let ghost rrv = r_right@;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|l: T| #[trigger] lv.contains(l@) implies l@ != rkv by {
                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        // Veracity: NEEDED assert
                        assert(rv.contains(r_key@));
                    };
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger lv.contains(s@), rlv.contains(o@)]
                        lv.contains(s@) && rlv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        // Veracity: NEEDED assert
                        assert(rlv.subset_of(rv));
                    };
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger lv.contains(s@), rrv.contains(o@)]
                        lv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        // Veracity: NEEDED assert
                        assert(rrv.subset_of(rv));
                    };
                    vstd::set_lib::lemma_len_subset(rlv, rv);
                    vstd::set_lib::lemma_len_subset(rrv, rv);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                    // Ordering facts while exec vars are live.
                }
                let (split_left, _, split_right) = split_inner_st(left, &r_key);
                let ghost slv = split_left@;
                let ghost srv = split_right@;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(slv.subset_of(lv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] slv.contains(x) implies lv.contains(x) by {
                            // Veracity: NEEDED assert
                            assert(slv.union(srv).contains(x));
                        };
                    };
                    // Veracity: NEEDED assert
                    assert(srv.subset_of(lv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] srv.contains(x) implies lv.contains(x) by {
                            // Veracity: NEEDED assert
                            assert(slv.union(srv).contains(x));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(slv, lv);
                    vstd::set_lib::lemma_len_subset(srv, lv);
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger slv.contains(s@), rlv.contains(o@)]
                        slv.contains(s@) && rlv.contains(o@) implies s.cmp_spec(&o) == Less by {
                    };
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger srv.contains(s@), rrv.contains(o@)]
                        srv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {
                    };
                }
                let combined_left = join_pair_inner_st(split_left, r_left);
                let combined_right = join_pair_inner_st(split_right, r_right);
                let ghost clv = combined_left@;
                let ghost crv = combined_right@;
                // Veracity: NEEDED proof block
                proof {
                    // slv ⊆ lv, rlv ⊆ rv, lv.disjoint(rv) → slv.disjoint(rlv).
                    // srv ⊆ lv, rrv ⊆ rv, lv.disjoint(rv) → srv.disjoint(rrv).
                    // clv < r_key < crv → clv.disjoint(crv).
                    vstd::set_lib::lemma_set_disjoint_lens(slv, rlv);
                    vstd::set_lib::lemma_set_disjoint_lens(srv, rrv);
                    vstd::set_lib::lemma_set_disjoint_lens(slv, srv);
                    vstd::set_lib::lemma_set_disjoint_lens(rlv, rrv);
                    // Veracity: NEEDED assert
                    assert(slv.union(srv) =~= lv);
                    vstd::set_lib::lemma_set_disjoint_lens(clv, crv);
                }
                join_with_priority_st(combined_left, r_key, rp, combined_right)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
    fn union_inner_st<T: StT + Ord + IsLtTransitive>(
        a: BSTTreapStEph<T>, b: BSTTreapStEph<T>,
    ) -> (combined: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a@.len() + b@.len() < usize::MAX as nat,
            spec_param_wf_link(&a.root), spec_param_wf_link(&b.root),
            a.spec_bsttreapsteph_wf(), b.spec_bsttreapsteph_wf(),
        ensures combined@.finite(), combined@ == a@.union(b@),
            spec_param_wf_link(&combined.root),
            combined.spec_bsttreapsteph_wf(),
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
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, _, br) = split_inner_st(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(blv.subset_of(bv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] blv.contains(x) implies bv.contains(x) by {
                            // Veracity: NEEDED assert
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    // Veracity: NEEDED assert
                    assert(brv.subset_of(bv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] brv.contains(x) implies bv.contains(x) by {
                            // Veracity: NEEDED assert
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(blv, bv);
                    vstd::set_lib::lemma_len_subset(brv, bv);
                }
                let left_union = union_inner_st(al, bl);
                let right_union = union_inner_st(ar, br);
                // Veracity: NEEDED proof block
                proof {
                    let luv = left_union@;
                    let ruv = right_union@;
                    // Veracity: NEEDED assert
                    assert(luv.disjoint(ruv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| !(luv.contains(x) && ruv.contains(x)) by {
                            if luv.contains(x) && ruv.contains(x) {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                lemma_wf_view_inhabited_st::<T>(&left_union.root, x);
                                let ghost tl = choose|t: T| #[trigger] t@ == x && luv.contains(t@);
                            }
                        };
                    };
                    vstd::set_lib::lemma_set_disjoint_lens(luv, ruv);
                    vstd::set_lib::lemma_len_subset(luv.union(ruv), av.union(bv));
                    vstd::set_lib::lemma_len_union(av, bv);
                    // Veracity: NEEDED assert
                    assert(luv.union(ruv).insert(akv) == av.union(bv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] luv.union(ruv).insert(akv).contains(x)
                            <==> av.union(bv).contains(x) by {
                            if luv.union(ruv).insert(akv).contains(x) {
                                if x == akv { assert(av.contains(akv)); }
                                else if luv.contains(x) {
                                    if alv.contains(x) { assert(av.contains(x)); }
                                } else {
                                    if arv.contains(x) { assert(av.contains(x)); }
                                }
                            }
                            if av.union(bv).contains(x) && !luv.union(ruv).insert(akv).contains(x) {
                                if av.contains(x) {
                                } else {
                                    // Veracity: NEEDED assert
                                    assert(bv.remove(akv).contains(x));
                                }
                            }
                        };
                    };
                }
                join_with_priority_st(left_union, ak, ap, right_union)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
    fn intersect_inner_st<T: StT + Ord + IsLtTransitive>(
        a: BSTTreapStEph<T>, b: BSTTreapStEph<T>,
    ) -> (common: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a@.len() < usize::MAX as nat,
            spec_param_wf_link(&a.root), spec_param_wf_link(&b.root),
            a.spec_bsttreapsteph_wf(), b.spec_bsttreapsteph_wf(),
        ensures common@.finite(), common@ == a@.intersect(b@),
            spec_param_wf_link(&common.root),
            common.spec_bsttreapsteph_wf(),
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
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner_st(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                // Veracity: NEEDED proof block
                proof {
                    // St analog of type_invariant: establish witness accessibility
                    // before trees are consumed by recursive calls.
                    lemma_wf_view_all_inhabited_st::<T>(&al.root);
                    lemma_wf_view_all_inhabited_st::<T>(&ar.root);
                    lemma_wf_view_all_inhabited_st::<T>(&bl.root);
                    lemma_wf_view_all_inhabited_st::<T>(&br.root);
                }
                let left_res = intersect_inner_st(al, bl);
                let right_res = intersect_inner_st(ar, br);
                let ghost lrv = left_res@;
                let ghost rrv = right_res@;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|x| #[trigger] av.intersect(bv).contains(x) <==>
                        lrv.union(rrv).union(if found { Set::<<T as View>::V>::empty().insert(akv) }
                                           else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.intersect(bv).contains(x) {
                            if x == akv {
                            } else if alv.contains(x) {
                                // Veracity: NEEDED assert
                                assert(blv.union(brv).contains(x)) by {
                                    // Veracity: NEEDED assert
                                    assert(bv.remove(akv).contains(x));
                                };
                            } else {
                                // Veracity: NEEDED assert
                                assert(brv.contains(x)) by {
                                    if blv.contains(x) {
                                        reveal(vstd::laws_cmp::obeys_cmp_ord);
                                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                        let ghost t_x = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent_st::<T>();
                                        lemma_cmp_equal_congruent_right_st(t_bl, t_x, ak);
                                    }
                                };
                            }
                        } else {
                            if lrv.contains(x) {
                            } else if rrv.contains(x) {
                            } else if found && x == akv {
                            }
                        }
                    };
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        if lrv.contains(s@) && rrv.contains(o@) {
// Veracity: TESTING assert                             assert(o.cmp_spec(&ak) == Greater);
                            lemma_cmp_antisymmetry_st(o, ak);
                            lemma_cmp_transitivity_st(s, ak, o);
                        }
                    };
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                    // Veracity: NEEDED assert
                    assert(alv.len() + arv.len() < av.len());
                    // Veracity: NEEDED assert
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
    fn difference_inner_st<T: StT + Ord + IsLtTransitive>(
        a: BSTTreapStEph<T>, b: BSTTreapStEph<T>,
    ) -> (remaining: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            a@.len() < usize::MAX as nat,
            spec_param_wf_link(&a.root), spec_param_wf_link(&b.root),
            a.spec_bsttreapsteph_wf(), b.spec_bsttreapsteph_wf(),
        ensures remaining@.finite(), remaining@ == a@.difference(b@),
            spec_param_wf_link(&remaining.root),
            remaining.spec_bsttreapsteph_wf(),
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
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(alv.subset_of(av));
                    // Veracity: NEEDED assert
                    assert(arv.subset_of(av));
                    vstd::set_lib::lemma_len_subset(alv, av);
                    vstd::set_lib::lemma_len_subset(arv, av);
                }
                let (bl, found, br) = split_inner_st(b, &ak);
                let ghost blv = bl@;
                let ghost brv = br@;
                // Veracity: NEEDED proof block
                proof {
                    // St analog of type_invariant: establish witness accessibility
                    // before trees are consumed by recursive calls.
                    lemma_wf_view_all_inhabited_st::<T>(&al.root);
                    lemma_wf_view_all_inhabited_st::<T>(&ar.root);
                    lemma_wf_view_all_inhabited_st::<T>(&bl.root);
                    lemma_wf_view_all_inhabited_st::<T>(&br.root);
                    // Veracity: NEEDED assert
                    assert(blv.subset_of(bv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] blv.contains(x) implies bv.contains(x) by {
                            // Veracity: NEEDED assert
                            assert(blv.union(brv).contains(x));
                            // Veracity: NEEDED assert
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    // Veracity: NEEDED assert
                    assert(brv.subset_of(bv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger] brv.contains(x) implies bv.contains(x) by {
                            // Veracity: NEEDED assert
                            assert(blv.union(brv).contains(x));
                            // Veracity: NEEDED assert
                            assert(bv.remove(akv).contains(x));
                        };
                    };
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] alv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        // Veracity: NEEDED assert
                        assert(al@.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] arv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        // Veracity: NEEDED assert
                        assert(ar@.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] blv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        // Veracity: NEEDED assert
                        assert(bl@.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] brv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        // Veracity: NEEDED assert
                        assert(br@.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert(blv.union(brv) == bv.remove(akv));
                }
                let left_res = difference_inner_st(al, bl);
                let right_res = difference_inner_st(ar, br);
                let ghost lrv = left_res@;
                let ghost rrv = right_res@;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|x| #[trigger] av.difference(bv).contains(x) <==>
                        lrv.union(rrv).union(if !found { Set::<<T as View>::V>::empty().insert(akv) }
                                            else { Set::<<T as View>::V>::empty() }).contains(x) by {
                        if av.difference(bv).contains(x) {
                            // Veracity: NEEDED assert
                            assert(av.contains(x) && !bv.contains(x));
                            if x == akv {
                                // Veracity: NEEDED assert
                                assert(!found);
                            } else if alv.contains(x) {
                                // Veracity: NEEDED assert
                                assert(!blv.contains(x));
                                // Veracity: NEEDED assert
                                assert(lrv.contains(x));
                            } else {
                                // Veracity: NEEDED assert
                                assert(arv.contains(x));
                                // Veracity: NEEDED assert
                                assert(!brv.contains(x));
                                // Veracity: NEEDED assert
                                assert(rrv.contains(x));
                            }
                        } else {
                            if lrv.contains(x) {
                                // Veracity: NEEDED assert
                                assert(alv.contains(x) && !blv.contains(x));
                                // Veracity: NEEDED assert
                                assert(av.contains(x));
                                if bv.contains(x) {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    // Veracity: NEEDED assert
                                    assert(blv.union(brv).contains(x));
                                    if brv.contains(x) {
                                        let ghost t_al = choose|t: T| #[trigger] t@ == x && alv.contains(t@);
                                        let ghost t_br = choose|t: T| #[trigger] t@ == x && brv.contains(t@);
                                        view_ord_consistent_st::<T>();
                                        // Veracity: NEEDED assert
                                        assert(t_al.cmp_spec(&t_br) == Equal);
                                        lemma_cmp_equal_congruent_st(t_al, t_br, ak);
                                        // Veracity: NEEDED assert
                                        assert(false);
                                    }
                                    // Veracity: NEEDED assert
                                    assert(false);
                                }
                            } else if rrv.contains(x) {
                                // Veracity: NEEDED assert
                                assert(arv.contains(x) && !brv.contains(x));
                                // Veracity: NEEDED assert
                                assert(av.contains(x));
                                if bv.contains(x) {
                                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                    // Veracity: NEEDED assert
                                    assert(blv.union(brv).contains(x));
                                    if blv.contains(x) {
                                        let ghost t_ar = choose|t: T| #[trigger] t@ == x && arv.contains(t@);
                                        let ghost t_bl = choose|t: T| #[trigger] t@ == x && blv.contains(t@);
                                        view_ord_consistent_st::<T>();
                                        // Veracity: NEEDED assert
                                        assert(t_ar.cmp_spec(&t_bl) == Equal);
                                        lemma_cmp_equal_congruent_right_st(t_bl, t_ar, ak);
                                        // Veracity: NEEDED assert
                                        assert(false);
                                    }
                                    // Veracity: NEEDED assert
                                    assert(false);
                                }
                            } else if !found && x == akv {
                                // Veracity: NEEDED assert
                                assert(!bv.contains(akv));
                                // Veracity: NEEDED assert
                                assert(av.contains(akv));
                            }
                        }
                    };
                    // Veracity: NEEDED assert
                    assert(av.difference(bv) =~= lrv.union(rrv).union(
                        if !found { Set::<<T as View>::V>::empty().insert(akv) }
                        else { Set::<<T as View>::V>::empty() }));
                    // Veracity: NEEDED assert
                    assert(lrv.subset_of(alv));
                    // Veracity: NEEDED assert
                    assert(rrv.subset_of(arv));
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&ak) == Less by {
                        // Veracity: NEEDED assert
                        assert(alv.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] rrv.contains(t@) implies t.cmp_spec(&ak) == Greater by {
                        // Veracity: NEEDED assert
                        assert(arv.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert(!lrv.contains(akv));
                    // Veracity: NEEDED assert
                    assert(!rrv.contains(akv));
                    // Veracity: NEEDED assert
                    assert(lrv.disjoint(rrv));
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
                        lrv.contains(s@) && rrv.contains(o@) implies s.cmp_spec(&o) == Less by {
                        if lrv.contains(s@) && rrv.contains(o@) {
                            // Veracity: NEEDED assert
                            assert(s.cmp_spec(&ak) == Less);
                            // Veracity: NEEDED assert
                            assert(o.cmp_spec(&ak) == Greater);
                            lemma_cmp_antisymmetry_st(o, ak);
                            lemma_cmp_transitivity_st(s, ak, o);
                        }
                    };
                    vstd::set_lib::lemma_len_subset(lrv, alv);
                    vstd::set_lib::lemma_len_subset(rrv, arv);
                    vstd::set_lib::lemma_set_disjoint_lens(alv, arv);
                    // Veracity: NEEDED assert
                    assert(alv.len() + arv.len() < av.len());
                    // Veracity: NEEDED assert
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected — DIFFERS: St sequential, APAS parallel
    fn filter_inner_st<T: StT + Ord + IsLtTransitive, F: Fn(&T) -> bool>(
        tree: BSTTreapStEph<T>,
        predicate: &F,
        Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
    ) -> (filtered: BSTTreapStEph<T>)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            forall|t: &T| #[trigger] predicate.requires((t,)),
            forall|x: T, keep: bool| #[trigger] predicate.ensures((&x,), keep)
                ==> keep == spec_pred(x@),
            tree@.len() < usize::MAX as nat,
            spec_param_wf_link(&tree.root),
            tree.spec_bsttreapsteph_wf(),
        ensures
            filtered@.finite(),
            filtered@.subset_of(tree@),
            forall|v: T::V| #[trigger] filtered@.contains(v)
                ==> tree@.contains(v) && spec_pred(v),
            forall|v: T::V| #[trigger] tree@.contains(v) && spec_pred(v)
                ==> filtered@.contains(v),
            spec_param_wf_link(&filtered.root),
            filtered.spec_bsttreapsteph_wf(),
        decreases tree@.len(),
    {
        match expose_to_parts_st(tree) {
            | None => BSTTreapStEph { root: None },
            | Some((left, key, ap, right)) => {
                let ghost lv = left@;
                let ghost rv = right@;
                let ghost kv = key@;
                let ghost tv = lv.union(rv).insert(kv);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] lv.contains(t@) implies t.cmp_spec(&key) == Less by {
                        // Veracity: NEEDED assert
                        assert(left@.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] rv.contains(t@) implies t.cmp_spec(&key) == Greater by {
                        // Veracity: NEEDED assert
                        assert(right@.contains(t@));
                    };
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    // Veracity: NEEDED assert
                    assert(lv.len() + rv.len() < tv.len());
                }
                let left_filtered = filter_inner_st(left, predicate, Ghost(spec_pred));
                let right_filtered = filter_inner_st(right, predicate, Ghost(spec_pred));
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(left_filtered@.subset_of(lv));
                    // Veracity: NEEDED assert
                    assert(right_filtered@.subset_of(rv));
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] left_filtered@.contains(t@) implies t.cmp_spec(&key) == Less by {
                        // Veracity: NEEDED assert
                        assert(lv.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] right_filtered@.contains(t@) implies t.cmp_spec(&key) == Greater by {
                        // Veracity: NEEDED assert
                        assert(rv.contains(t@));
                    };
                    // Veracity: NEEDED assert
                    assert(!left_filtered@.contains(kv));
                    // Veracity: NEEDED assert
                    assert(!right_filtered@.contains(kv));
                    // Veracity: NEEDED assert
                    assert(lv.disjoint(rv));
                    // Veracity: NEEDED assert
                    assert(left_filtered@.disjoint(right_filtered@));
                    // Veracity: NEEDED assert
                    assert forall|s: T, o: T| #![trigger left_filtered@.contains(s@), right_filtered@.contains(o@)]
                        left_filtered@.contains(s@) && right_filtered@.contains(o@) implies s.cmp_spec(&o) == Less by {
                        if left_filtered@.contains(s@) && right_filtered@.contains(o@) {
                            // Veracity: NEEDED assert
                            assert(s.cmp_spec(&key) == Less);
                            // Veracity: NEEDED assert
                            assert(o.cmp_spec(&key) == Greater);
                            lemma_cmp_antisymmetry_st(o, key);
                            lemma_cmp_transitivity_st(s, key, o);
                        }
                    };
                    vstd::set_lib::lemma_len_subset(left_filtered@, lv);
                    vstd::set_lib::lemma_len_subset(right_filtered@, rv);
                    vstd::set_lib::lemma_set_disjoint_lens(lv, rv);
                    // Veracity: NEEDED assert
                    assert(lv.len() + rv.len() < tv.len());
                    // Veracity: NEEDED assert
                    assert(left_filtered@.len() + right_filtered@.len() < usize::MAX as nat);
                    // Veracity: NEEDED assert
                    assert(tv == lv.union(rv).insert(kv));
                    // Veracity: NEEDED assert
                    assert(left_filtered@.union(right_filtered@).subset_of(tv)) by {
                        // Veracity: NEEDED assert
                        assert forall|x: T::V| #[trigger]
                            left_filtered@.union(right_filtered@).contains(x)
                            implies tv.contains(x) by {
                            if left_filtered@.contains(x) { assert(lv.contains(x)); }
                            else { assert(right_filtered@.contains(x)); assert(rv.contains(x)); }
                        };
                    };
                }
                let keep = (*predicate)(&key);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert((*predicate).ensures((&key,), keep));
                    // Veracity: NEEDED assert
                    assert(keep == spec_pred(kv));
                }
                if keep {
                    // Veracity: NEEDED proof block
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        // Veracity: NEEDED assert
                        assert forall|v: T::V| #[trigger]
                            lf.union(rf).insert(kv).contains(v) implies spec_pred(v) by {
                            if v == kv { assert(spec_pred(kv)); }
                            else if lf.contains(v) { assert(left_filtered@.contains(v)); }
                            else { assert(rf.contains(v)); assert(right_filtered@.contains(v)); }
                        };
                        // Veracity: NEEDED assert
                        assert forall|v: T::V| #[trigger]
                            tv.contains(v) && spec_pred(v)
                            implies lf.union(rf).insert(kv).contains(v) by {
                            if v == kv { }
                            else {
                                // Veracity: NEEDED assert
                                assert(lv.union(rv).contains(v));
                                if lv.contains(v) { assert(left_filtered@.contains(v)); assert(lf.contains(v)); }
                                else { assert(rv.contains(v)); assert(right_filtered@.contains(v)); assert(rf.contains(v)); }
                            }
                        };
                    }
                    join_with_priority_st(left_filtered, key, ap, right_filtered)
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        let lf = left_filtered@;
                        let rf = right_filtered@;
                        // Veracity: NEEDED assert
                        assert forall|v: T::V| #[trigger]
                            lf.union(rf).contains(v) implies spec_pred(v) by {
                            if lf.contains(v) { assert(left_filtered@.contains(v)); }
                            else { assert(rf.contains(v)); assert(right_filtered@.contains(v)); }
                        };
                        // Veracity: NEEDED assert
                        assert forall|v: T::V| #[trigger]
                            tv.contains(v) && spec_pred(v)
                            implies lf.union(rf).contains(v) by {
                            if v == kv { assert(!spec_pred(kv)); }
                            else {
                                // Veracity: NEEDED assert
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn reduce_inner_st<T: StT + Ord + IsLtTransitive, F: Fn(T, T) -> T>(
        tree: BSTTreapStEph<T>, op: &F, identity: T,
    ) -> (reduced: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            obeys_feq_clone::<T>(),
            tree@.finite(),
            spec_param_wf_link(&tree.root),
            tree.spec_bsttreapsteph_wf(),
            forall|a: T, b: T| #[trigger] op.requires((a, b)),
        ensures tree@.len() == 0 ==> reduced == identity,
        decreases tree@.len(),
    {
        match expose_to_parts_st(tree) {
            | None => identity,
            | Some((left, key, _, right)) => {
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(left@.finite());
                    // Veracity: NEEDED assert
                    assert(right@.finite());
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                let left_base = identity.clone_plus();
                let right_base = identity;
                let left_acc = reduce_inner_st(left, op, left_base);
                let right_acc = reduce_inner_st(right, op, right_base);
                let right_with_key = (*op)(key, right_acc);
                (*op)(left_acc, right_with_key)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn collect_in_order_st<T: StT + Ord + IsLtTransitive>(
        tree: BSTTreapStEph<T>, out: &mut Vec<T>,
    )
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
            tree@.finite(),
            spec_param_wf_link(&tree.root),
            tree.spec_bsttreapsteph_wf(),
        ensures out@.len() == old(out)@.len() + tree@.len(),
        decreases tree@.len(),
    {
        match expose_to_parts_st(tree) {
            | None => {}
            | Some((left, key, _, right)) => {
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                collect_in_order_st(left, out);
                out.push(key);
                collect_in_order_st(right, out);
            }
        }
    }

    //		Section 12a. derive impls in verus!


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

    impl<T: StT + Ord + IsLtTransitive> Default for BSTreeTreap<T> {
        fn default() -> (d: Self)
            ensures d.spec_size() == 0, d.spec_bsttreapsteph_wf(), d.spec_bst(),
        { Self::new() }
    }

    //		Section 12b. derive impls in verus!


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

    //		Section 13. macros


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

    //		Section 14a. derive impls outside verus!

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

    impl<T: StT + Ord + IsLtTransitive + fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Node(key={}, priority={}, size={})", self.key, self.priority, self.size)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: StT + Ord + IsLtTransitive + fmt::Debug> fmt::Debug for BSTTreapStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTTreapStEph").field("root", &self.root).finish()
        }
    }

    impl<T: StT + Ord + IsLtTransitive> fmt::Display for BSTTreapStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTTreapStEph(size: {})", self.size())
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<T: StT + Ord + IsLtTransitive + fmt::Debug> fmt::Debug for ExposedTreap<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ExposedTreap::Leaf => write!(f, "Leaf"),
                ExposedTreap::Node(l, k, r) => write!(f, "Node({:?}, {:?}, {:?})", l, k, r),
            }
        }
    }

    impl<T: StT + Ord + IsLtTransitive + fmt::Display> fmt::Display for ExposedTreap<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ExposedTreap::Leaf => write!(f, "Leaf"),
                ExposedTreap::Node(l, k, r) => write!(f, "Node({}, {}, {})", l, k, r),
            }
        }
    }
}
