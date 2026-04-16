// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Size-augmented BST with O(1) size queries and rank/select operations.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4a. type definitions
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 8b. traits
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 6c. spec fns
//	Section 7c. proof fns/broadcast groups
//	Section 8c. traits
//	Section 9c. impls
//	Section 10b. iterators — BSTSizeStEph
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!

//		Section 1. module

pub mod BSTSizeStEph {


    //		Section 2. imports

    use std::fmt;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! 
{


    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpecImpl;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdSpec;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    //		Section 4a. type definitions


    pub struct Node<T: StT + Ord> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    pub type Link<T> = Option<Box<Node<T>>>;

    //		Section 8a. traits


    pub trait NodeTrait<T: StT + Ord>: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_bstsizesteph_size_wf(&self) -> bool;

        spec fn spec_height(&self) -> nat;

        spec fn spec_content(&self) -> Set<T>;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new(key: T, priority: u64) -> (node: Self);
    }

    //		Section 9a. impls


    impl<T: StT + Ord> NodeTrait<T> for Node<T> {
        open spec fn spec_size(&self) -> nat {
            self.size as nat
        }

        open spec fn spec_bstsizesteph_size_wf(&self) -> bool
            decreases *self,
        {
            self.size as nat == 1 + Lnk::spec_size_link(&self.left) + Lnk::spec_size_link(&self.right)
            && Lnk::spec_link_size_wf(&self.left)
            && Lnk::spec_link_size_wf(&self.right)
        }

        open spec fn spec_height(&self) -> nat
            decreases *self,
        {
            let l = Lnk::spec_height_link(&self.left);
            let r = Lnk::spec_height_link(&self.right);
            1 + if l >= r { l } else { r }
        }

        open spec fn spec_content(&self) -> Set<T>
            decreases *self,
        {
            Lnk::spec_content_link(&self.left)
                .union(Lnk::spec_content_link(&self.right))
                .insert(self.key)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new(key: T, priority: u64) -> (node: Self)
            ensures
                node.key == key,
                node.priority == priority,
                node.size == 1,
                node.left is None,
                node.right is None,
        {
            Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    //		Section 4b. type definitions


    pub struct BSTSizeStEph<T: StT + Ord> {
        pub root: Link<T>,
    }

    pub type BSTreeSize<T> = BSTSizeStEph<T>;

    //		Section 5b. view impls


    impl<T: StT + Ord> View for BSTSizeStEph<T> {
        type V = Set<T>;
        open spec fn view(&self) -> Set<T> {
            Lnk::spec_content_link(&self.root)
        }
    }

    //		Section 8b. traits


    pub trait BSTSizeStEphTrait<T: StT + Ord>: Sized + View<V = Set<T>> {
        spec fn spec_size(&self) -> nat;
        spec fn spec_bstsizesteph_wf(&self) -> bool;
        spec fn spec_height(&self) -> nat;

        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
            ensures
                empty.spec_size() == 0,
                empty.spec_bstsizesteph_wf(),
                empty@ == Set::<T>::empty();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1) — reads augmented size field
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures count as nat == self.spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == (self.spec_size() == 0);
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (height: usize)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_bstsizesteph_wf(),
            ensures
                height as nat == self.spec_height();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires
                old(self).spec_size() + 1 <= usize::MAX as nat,
                old(self).spec_bstsizesteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                self@ == old(self)@.insert(value),
                self.spec_bstsizesteph_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn delete(&mut self, key: &T)
            requires
                old(self).spec_bstsizesteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                self@ == old(self)@.remove(*key),
                self.spec_bstsizesteph_wf(),
                self.spec_size() <= old(self).spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires
                self.spec_bstsizesteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                found is Some <==> self@.contains(*target),
                found is Some ==> *found.unwrap() == *target;
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected
        fn contains(&self, target: &T) -> (contains: bool)
            requires
                self.spec_bstsizesteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures contains == self@.contains(*target);
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected
        fn minimum(&self) -> (minimum: Option<&T>)
            requires self.spec_bstsizesteph_wf(),
            ensures
                self.spec_size() == 0 ==> minimum is None,
                self.spec_size() > 0 ==> minimum is Some;
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected
        fn maximum(&self) -> (maximum: Option<&T>)
            requires self.spec_bstsizesteph_wf(),
            ensures
                self.spec_size() == 0 ==> maximum is None,
                self.spec_size() > 0 ==> maximum is Some;
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires self.spec_bstsizesteph_wf(),
            ensures ordered.spec_len() == self.spec_size();
        /// - Alg Analysis: APAS (Ch40 Alg 40.1): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn rank(&self, key: &T) -> (rank: usize)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_bstsizesteph_wf(),
            ensures
                rank as nat <= self.spec_size();
        /// - Alg Analysis: APAS (Ch40 Alg 40.1): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn select(&self, rank: usize) -> (selected: Option<&T>)
            ensures (rank == 0 || rank as nat > self.spec_size()) ==> selected is None;
        /// - Alg Analysis: APAS (Ch40 Ex 40.1): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn split_rank(&self, rank: usize) -> (split: (BSTSizeStEph<T>, BSTSizeStEph<T>))
            requires self.spec_bstsizesteph_wf(),
            ensures
                Lnk::spec_link_size_wf(&split.0.root),
                Lnk::spec_link_size_wf(&split.1.root);

        // Internal associated functions.

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(link: &Link<T>) -> (count: usize)
            ensures count as nat == Lnk::spec_size_link(link);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn update_size(node: &mut Node<T>)
            requires
                Lnk::spec_size_link(&old(node).left) + Lnk::spec_size_link(&old(node).right) + 1 <= usize::MAX as nat,
            ensures
                node.size as nat == Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right) + 1,
                node.key == old(node).key,
                node.priority == old(node).priority,
                node.left == old(node).left,
                node.right == old(node).right;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn make_node(key: T, priority: u64, left: Link<T>, right: Link<T>) -> (node: Link<T>)
            requires
                Lnk::spec_size_link(&left) + Lnk::spec_size_link(&right) + 1 <= usize::MAX as nat,
            ensures
                Lnk::spec_size_link(&node) == Lnk::spec_size_link(&left) + Lnk::spec_size_link(&right) + 1,
                Lnk::spec_link_size_wf(&node) <==> (Lnk::spec_link_size_wf(&left) && Lnk::spec_link_size_wf(&right)),
                Lnk::spec_content_link(&node) == Lnk::spec_content_link(&left).union(Lnk::spec_content_link(&right)).insert(key);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(link: &mut Link<T>)
            requires
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
                Lnk::spec_ordered_link(old(link)),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
            ensures
                Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
                Lnk::spec_link_size_wf(link),
                Lnk::spec_content_link(link) == Lnk::spec_content_link(old(link)),
                Lnk::spec_ordered_link(link),
                // After non-trivial rotation, root key comes from right subtree.
                spec_has_right_child_link(old(link)) ==> (
                    spec_root_key_link(link) != spec_root_key_link(old(link))
                    && spec_root_key_link(link).cmp_spec(&spec_root_key_link(old(link)))
                        == std::cmp::Ordering::Greater
                );
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(link: &mut Link<T>)
            requires
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
                Lnk::spec_ordered_link(old(link)),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
            ensures
                Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
                Lnk::spec_link_size_wf(link),
                Lnk::spec_content_link(link) == Lnk::spec_content_link(old(link)),
                Lnk::spec_ordered_link(link),
                // After non-trivial rotation, root key comes from left subtree.
                spec_has_left_child_link(old(link)) ==> (
                    spec_root_key_link(link) != spec_root_key_link(old(link))
                    && spec_root_key_link(link).cmp_spec(&spec_root_key_link(old(link)))
                        == std::cmp::Ordering::Less
                );
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn insert_link(link: &mut Link<T>, value: T, priority: u64)
            requires
                Lnk::spec_size_link(old(link)) + 1 <= usize::MAX as nat,
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_ordered_link(old(link)),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                Lnk::spec_link_size_wf(link),
                Lnk::spec_size_link(link) <= Lnk::spec_size_link(old(link)) + 1,
                Lnk::spec_size_link(link) >= Lnk::spec_size_link(old(link)),
                Lnk::spec_content_link(link) == Lnk::spec_content_link(old(link)).insert(value),
                Lnk::spec_ordered_link(link),
            decreases old(link);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn delete_link(link: &mut Link<T>, key: &T) -> (deleted: bool)
            requires
                Lnk::spec_ordered_link(old(link)),
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                Lnk::spec_content_link(link) == Lnk::spec_content_link(old(link)).remove(*key),
                Lnk::spec_ordered_link(link),
                Lnk::spec_link_size_wf(link),
                Lnk::spec_size_link(link) + if deleted { 1nat } else { 0nat } == Lnk::spec_size_link(old(link)),
            decreases Lnk::spec_size_link(old(link));
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
                Lnk::spec_ordered_link(link),
            ensures
                link.is_none() ==> found.is_none(),
                found is Some ==> Lnk::spec_content_link(link).contains(*found.unwrap()),
                found is Some ==> *found.unwrap() == *target,
                Lnk::spec_content_link(link).contains(*target) ==> found is Some,
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn min_link(link: &Link<T>) -> (minimum: Option<&T>)
            ensures
                link.is_none() ==> minimum.is_none(),
                link.is_some() ==> minimum.is_some(),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn max_link(link: &Link<T>) -> (maximum: Option<&T>)
            ensures
                link.is_none() ==> maximum.is_none(),
                link.is_some() ==> maximum.is_some(),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_link(link: &Link<T>) -> (h: usize)
            requires
                Lnk::spec_size_link(link) < usize::MAX as nat,
                Lnk::spec_link_size_wf(link),
            ensures h as nat == Lnk::spec_height_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>)
            requires Lnk::spec_link_size_wf(link),
            ensures out.len() == old(out).len() + Lnk::spec_size_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_collect_with_priority(link: &Link<T>, out: &mut Vec<(T, u64)>)
            requires Lnk::spec_link_size_wf(link),
            ensures out.len() == old(out).len() + Lnk::spec_size_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn find_min_priority_idx(items: &Vec<(T, u64)>, start: usize, end: usize) -> (min_idx: usize)
            requires start < end, end <= items.len(),
            ensures start <= min_idx && min_idx < end;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, O(n^2) worst
        fn build_treap_from_vec(items: &Vec<(T, u64)>, start: usize, end: usize) -> (treap: Link<T>)
            requires start <= end, end <= items.len(),
            ensures
                Lnk::spec_size_link(&treap) == (end - start) as nat,
                Lnk::spec_link_size_wf(&treap),
            decreases end - start;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_by_key(items: &Vec<(T, u64)>, key: &T) -> (filtered: Vec<(T, u64)>)
            ensures filtered.len() <= items.len();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn rank_link(link: &Link<T>, key: &T) -> (rank: usize)
            requires
                Lnk::spec_size_link(link) < usize::MAX as nat,
                Lnk::spec_link_size_wf(link),
            ensures rank as nat <= Lnk::spec_size_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn select_link(link: &Link<T>, rank: usize) -> (selected: Option<&T>)
            ensures link.is_none() ==> selected.is_none(),
            decreases *link;
    }

    //		Section 9b. impls


    impl<T: StT + Ord> BSTSizeStEphTrait<T> for BSTSizeStEph<T> {
        open spec fn spec_size(&self) -> nat { Lnk::spec_size_link(&self.root) }
        open spec fn spec_bstsizesteph_wf(&self) -> bool { Lnk::spec_link_size_wf(&self.root) && Lnk::spec_ordered_link(&self.root) }
        open spec fn spec_height(&self) -> nat { Lnk::spec_height_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self) { BSTSizeStEph { root: None } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize) { Self::size_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool) { self.size() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (height: usize) { Self::height_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn insert(&mut self, value: T, priority: u64) {
            Self::insert_link(&mut self.root, value, priority);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn delete(&mut self, key: &T) {
            Self::delete_link(&mut self.root, key);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn contains(&self, target: &T) -> bool { self.find(target).is_some() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out: Vec<T> = Vec::new();
            Self::in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn rank(&self, key: &T) -> usize { Self::rank_link(&self.root, key) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn select(&self, rank: usize) -> Option<&T> {
            if rank == 0 || rank > self.size() {
                None
            } else {
                Self::select_link(&self.root, rank)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, O(n^2) worst
        fn split_rank(&self, rank: usize) -> (BSTSizeStEph<T>, BSTSizeStEph<T>) {
            if rank == 0 {
                (Self::new(), self.clone())
            } else if rank >= self.size() {
                (self.clone(), Self::new())
            } else {
                let mut items: Vec<(T, u64)> = Vec::new();
                Self::in_order_collect_with_priority(&self.root, &mut items);
                let r = if rank < items.len() { rank } else { items.len() };
                let left_root = Self::build_treap_from_vec(&items, 0, r);
                let right_root = Self::build_treap_from_vec(&items, r, items.len());
                (
                    BSTSizeStEph { root: left_root },
                    BSTSizeStEph { root: right_root },
                )
            }
        }

        // Internal associated functions.

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(link: &Link<T>) -> (count: usize) {
            match link {
                None => 0,
                Some(n) => n.size,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn update_size(node: &mut Node<T>) {
            let l = Self::size_link(&node.left);
            let r = Self::size_link(&node.right);
            node.size = 1 + l + r;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn make_node(key: T, priority: u64, left: Link<T>, right: Link<T>) -> (node: Link<T>) {
            let mut node = Node::new(key, priority);
            node.left = left;
            node.right = right;
            Self::update_size(&mut node);
            Some(Box::new(node))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(link: &mut Link<T>) {
            let ghost old_content = Lnk::spec_content_link(link);
            if let Some(mut x) = link.take() {
                let ghost xl = Lnk::spec_size_link(&x.left);
                let ghost xr = Lnk::spec_size_link(&x.right);
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&x.left));
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&x.right));
                let ghost x_left_content = Lnk::spec_content_link(&x.left);
                let ghost x_right_content = Lnk::spec_content_link(&x.right);
                let ghost x_key = x.key;
                // Ordering facts from pre-rotation.
                // Veracity: NEEDED assert
                assert(Lnk::spec_ordered_link(&x.right));

                if let Some(mut y) = x.right.take() {
                    let ghost yl = Lnk::spec_size_link(&y.left);
                    let ghost yr = Lnk::spec_size_link(&y.right);
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&y.left));
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&y.right));
                    let ghost y_left_content = Lnk::spec_content_link(&y.left);
                    let ghost y_right_content = Lnk::spec_content_link(&y.right);
                    let ghost y_key = y.key;

                    x.right = y.left.take();
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&x.right));
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&x.left));
                    // Ordering of new x: left=A, right=B (y_left_content).
                    // B was part of old x.right content, so B > x_key.
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(forall |k: T| #[trigger] y_left_content.contains(k)
                            ==> x_right_content.contains(k));
                    }
                    Self::update_size(&mut *x);

                    // Prove ordering for new x before it moves into y.left.
                    // Veracity: NEEDED proof block
                    proof {
                    }

                    y.left = Some(x);
                    Self::update_size(&mut *y);
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&y.right));
                    // Capture content before y is moved.
                    let ghost y_left_new_content = Lnk::spec_content_link(&y.left);
                    let ghost pre_move_content = y_left_new_content.union(y_right_content).insert(y_key);

                    // Ordering of new y: left=Some(new_x), right=C.
                    // Veracity: NEEDED proof block
                    proof {
                        // y_key was in old x.right, so y_key > x_key.
                        // Veracity: NEEDED assert
                        assert(x_right_content.contains(y_key));
                        // By antisymmetry: x_key < y_key.
                        lemma_cmp_antisymmetry(y_key, x_key);
                        // All k in A: k < x_key < y_key by transitivity.
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] x_left_content.contains(k)
                            implies k.cmp_spec(&y_key) == std::cmp::Ordering::Less by {
                            if x_left_content.contains(k) {
                                lemma_cmp_transitivity_lt(k, x_key, y_key);
                            }
                        };
                        // All k in B: k < y_key (pre-rotation ordering of y).
                        // Combine: all k in y_left_new_content < y_key.
                        // content(C) > y_key (pre-rotation ordering of y).
                    }

                    *link = Some(y);
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_wf_assemble(link);
                        lemma_ordered_assemble(link);
                        // Root key postcondition: new root is y_key, old was x_key, y_key > x_key.
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                    }
                } else {
                    *link = Some(x);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(link: &mut Link<T>) {
            let ghost old_content = Lnk::spec_content_link(link);
            if let Some(mut x) = link.take() {
                let ghost xl = Lnk::spec_size_link(&x.left);
                let ghost xr = Lnk::spec_size_link(&x.right);
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&x.left));
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&x.right));
                let ghost x_left_content = Lnk::spec_content_link(&x.left);
                let ghost x_right_content = Lnk::spec_content_link(&x.right);
                let ghost x_key = x.key;
                // Veracity: NEEDED assert
                assert(Lnk::spec_ordered_link(&x.left));

                if let Some(mut y) = x.left.take() {
                    let ghost yl = Lnk::spec_size_link(&y.left);
                    let ghost yr = Lnk::spec_size_link(&y.right);
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&y.left));
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&y.right));
                    let ghost y_left_content = Lnk::spec_content_link(&y.left);
                    let ghost y_right_content = Lnk::spec_content_link(&y.right);
                    let ghost y_key = y.key;

                    x.left = y.right.take();
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&x.left));
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&x.right));
                    // Ordering of new x: left=B (y_right_content), right=C (x_right_content).
                    // B was part of old x.left content, so B < x_key.
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(forall |k: T| #[trigger] y_right_content.contains(k)
                            ==> x_left_content.contains(k));
                    }
                    Self::update_size(&mut *x);

                    // Veracity: NEEDED proof block
                    proof {
                    }

                    y.right = Some(x);
                    Self::update_size(&mut *y);
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&y.left));
                    // Capture content before y is moved.
                    let ghost y_right_new_content = Lnk::spec_content_link(&y.right);
                    let ghost pre_move_content = y_left_content.union(y_right_new_content).insert(y_key);

                    // Ordering of new y: left=A (y_left_content), right=Some(new_x).
                    // Veracity: NEEDED proof block
                    proof {
                        // y_key was in old x.left, so y_key < x_key.
                        // Veracity: NEEDED assert
                        assert(x_left_content.contains(y_key));
                        // By antisymmetry: x_key > y_key.
                        lemma_cmp_antisymmetry_lt(y_key, x_key);
                        // All k in C: k > x_key > y_key by transitivity.
                        // Veracity: NEEDED assert
                        assert forall |k: T| #[trigger] x_right_content.contains(k)
                            implies k.cmp_spec(&y_key) == std::cmp::Ordering::Greater by {
                            if x_right_content.contains(k) {
                                lemma_cmp_transitivity_gt(k, x_key, y_key);
                            }
                        };
                        // All k in B: k > y_key (pre-rotation ordering of y).
                        // Combine: all k in y_right_new_content > y_key.
                        // content(A) < y_key (pre-rotation ordering of y).
                    }

                    *link = Some(y);
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_wf_assemble(link);
                        lemma_ordered_assemble(link);
                        // Root key postcondition: new root is y_key, old was x_key, y_key < x_key.
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                    }
                } else {
                    *link = Some(x);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn insert_link(link: &mut Link<T>, value: T, priority: u64)
            decreases old(link),
        {
            // Veracity: NEEDED proof block
            proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
            let ghost old_content = Lnk::spec_content_link(link);
            if let Some(mut node) = link.take() {
                let ghost old_left = Lnk::spec_size_link(&node.left);
                let ghost old_right = Lnk::spec_size_link(&node.right);
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&node.left));
                // Veracity: NEEDED assert
                assert(Lnk::spec_link_size_wf(&node.right));
                let ghost old_left_content = Lnk::spec_content_link(&node.left);
                let ghost old_right_content = Lnk::spec_content_link(&node.right);
                let ghost node_key = node.key;

                match value.cmp(&node.key) {
                    std::cmp::Ordering::Less => {
                        Self::insert_link(&mut node.left, value, priority);
                        // Veracity: NEEDED assert
                        assert(Lnk::spec_link_size_wf(&node.right));
                        // New left content = old_left_content ∪ {value}. All < node_key:
                        // old left elements < node_key (pre-ordering), value < node_key (from cmp).
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        Self::update_size(&mut *node);
                        // Veracity: NEEDED assert
                        assert(Lnk::spec_link_size_wf(&node.right));
                        *link = Some(node);
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble(link);
                        }
                        let need_rotate = match link.as_ref().unwrap().left.as_ref() {
                            Some(left) => left.priority < link.as_ref().unwrap().priority,
                            None => false,
                        };
                        if need_rotate {
                            Self::rotate_right(link);
                        }
                    },
                    std::cmp::Ordering::Greater => {
                        Self::insert_link(&mut node.right, value, priority);
                        // Veracity: NEEDED assert
                        assert(Lnk::spec_link_size_wf(&node.left));
                        // New right content = old_right_content ∪ {value}. All > node_key:
                        // old right elements > node_key (pre-ordering), value > node_key (from cmp).
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        Self::update_size(&mut *node);
                        // Veracity: NEEDED assert
                        assert(Lnk::spec_link_size_wf(&node.left));
                        *link = Some(node);
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble(link);
                        }
                        let need_rotate = match link.as_ref().unwrap().right.as_ref() {
                            Some(right) => right.priority < link.as_ref().unwrap().priority,
                            None => false,
                        };
                        if need_rotate {
                            Self::rotate_left(link);
                        }
                    },
                    std::cmp::Ordering::Equal => {
                        *link = Some(node);
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble(link);
                        }
                    },
                }
            } else {
                *link = Some(Box::new(Node {
                    key: value,
                    priority,
                    size: 1,
                    left: None,
                    right: None,
                }));
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_assemble(link);
                    lemma_ordered_assemble(link);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn delete_link(link: &mut Link<T>, key: &T) -> (deleted: bool)
            decreases Lnk::spec_size_link(old(link)),
        {
            // Veracity: NEEDED proof block
            proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
            let ghost old_content = Lnk::spec_content_link(link);
            let ghost old_size = Lnk::spec_size_link(link);
            if let Some(mut node) = link.take() {
                let ghost old_left_content = Lnk::spec_content_link(&node.left);
                let ghost old_right_content = Lnk::spec_content_link(&node.right);
                let ghost node_key = node.key;
                let ghost old_left_size = Lnk::spec_size_link(&node.left);
                let ghost old_right_size = Lnk::spec_size_link(&node.right);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&node.left));
                    // Veracity: NEEDED assert
                    assert(Lnk::spec_link_size_wf(&node.right));
                }

                match key.cmp(&node.key) {
                    std::cmp::Ordering::Less => {
                        // Veracity: NEEDED proof block
                        proof {
                            if old_right_content.contains(*key) {
                            }
                        }
                        let deleted = Self::delete_link(&mut node.left, key);
                        Self::update_size(&mut *node);
                        *link = Some(node);
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble(link);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        }
                        deleted
                    }
                    std::cmp::Ordering::Greater => {
                        // Veracity: NEEDED proof block
                        proof {
                            if old_left_content.contains(*key) {
                            }
                        }
                        let deleted = Self::delete_link(&mut node.right, key);
                        Self::update_size(&mut *node);
                        *link = Some(node);
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble(link);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        }
                        deleted
                    }
                    std::cmp::Ordering::Equal => {
                        if node.left.is_none() && node.right.is_none() {
                            // Veracity: NEEDED proof block
                            proof {
                            }
                            true
                        } else {
                            let rotate_right = if node.right.is_none() {
                                true
                            } else if node.left.is_none() {
                                false
                            } else {
                                node.left.as_ref().unwrap().priority <= node.right.as_ref().unwrap().priority
                            };
                            *link = Some(node);
                            if rotate_right {
                                Self::rotate_right(link);
                                let ghost new_root_key = spec_root_key_link(link);
                                // Veracity: NEEDED proof block
                                proof {
                                    // From rotate_right ensures: new root < old root.
                                }
                                let mut rot = link.take().unwrap();
                                let ghost rot_left_content = Lnk::spec_content_link(&rot.left);
                                let ghost rot_right_content = Lnk::spec_content_link(&rot.right);
                                // Veracity: NEEDED proof block
                                proof {
                                }
                                let deleted = Self::delete_link(&mut rot.right, key);
                                Self::update_size(&mut *rot);
                                *link = Some(rot);
                                // Veracity: NEEDED proof block
                                proof {
                                    // rot.key == new_root_key != *key.
                                    // rot.key < *key by antisymmetry.
                                    lemma_cmp_antisymmetry_lt(rot.key, *key);
                                    // If *key in left: key.cmp_spec(&rot.key) == Less, contradicts Greater.
                                    if rot_left_content.contains(*key) {
                                    }
                                    lemma_wf_assemble(link);
                                    lemma_ordered_assemble(link);
                                }
                                deleted
                            } else {
                                Self::rotate_left(link);
                                let ghost new_root_key = spec_root_key_link(link);
                                // Veracity: NEEDED proof block
                                proof {
                                    // From rotate_left ensures: new root > old root.
                                }
                                let mut rot = link.take().unwrap();
                                let ghost rot_left_content = Lnk::spec_content_link(&rot.left);
                                let ghost rot_right_content = Lnk::spec_content_link(&rot.right);
                                // Veracity: NEEDED proof block
                                proof {
                                }
                                let deleted = Self::delete_link(&mut rot.left, key);
                                Self::update_size(&mut *rot);
                                *link = Some(rot);
                                // Veracity: NEEDED proof block
                                proof {
                                    // rot.key == new_root_key != *key.
                                    // rot.key > *key by antisymmetry.
                                    lemma_cmp_antisymmetry(rot.key, *key);
                                    // If *key in right: key.cmp_spec(&rot.key) == Greater, contradicts Less.
                                    if rot_right_content.contains(*key) {
                                    }
                                    lemma_wf_assemble(link);
                                    lemma_ordered_assemble(link);
                                }
                                deleted
                            }
                        }
                    }
                }
            } else {
                false
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            decreases *link,
        {
            // Veracity: NEEDED proof block
            proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
            match link {
                | None => None,
                | Some(node) => {
                    match target.cmp(&node.key) {
                        std::cmp::Ordering::Equal => {
                            // Veracity: NEEDED assert
                            assert(Lnk::spec_content_link(link) =~=
                                Lnk::spec_content_link(&node.left)
                                    .union(Lnk::spec_content_link(&node.right))
                                    .insert(node.key));
                            Some(&node.key)
                        },
                        std::cmp::Ordering::Less => {
                            let found = Self::find_link(&node.left, target);
                            // Veracity: NEEDED proof block
                            proof {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                // Veracity: NEEDED assert
                                assert(Lnk::spec_content_link(link) =~=
                                    Lnk::spec_content_link(&node.left)
                                        .union(Lnk::spec_content_link(&node.right))
                                        .insert(node.key));
                                if found.is_some() {
                                }
                                // Completeness: target not in right (right > key, target < key).
                                // target != key (Less != Equal via reflexivity).
                                if Lnk::spec_content_link(link).contains(*target) {
                                }
                            }
                            found
                        },
                        std::cmp::Ordering::Greater => {
                            let found = Self::find_link(&node.right, target);
                            // Veracity: NEEDED proof block
                            proof {
                                reveal(vstd::laws_cmp::obeys_cmp_ord);
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                if found.is_some() {
                                }
                                // Completeness: target not in left (left < key, target > key).
                                // target != key (Greater != Equal via reflexivity).
                                if Lnk::spec_content_link(link).contains(*target) {
                                }
                            }
                            found
                        },
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn min_link(link: &Link<T>) -> (minimum: Option<&T>)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn max_link(link: &Link<T>) -> (maximum: Option<&T>)
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
        fn height_link(link: &Link<T>) -> (h: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    // Veracity: NEEDED proof block
                    proof { lemma_size_wf_child_bounded(link); }
                    let lh = Self::height_link(&node.left);
                    let rh = Self::height_link(&node.right);
                    let m = if lh >= rh { lh } else { rh };
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_height_le_size(&node.left);
                        lemma_height_le_size(&node.right);
                    }
                    1 + m
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::in_order_collect(&node.left, out);
                out.push(node.key.clone());
                Self::in_order_collect(&node.right, out);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_collect_with_priority(link: &Link<T>, out: &mut Vec<(T, u64)>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::in_order_collect_with_priority(&node.left, out);
                out.push((node.key.clone(), node.priority));
                Self::in_order_collect_with_priority(&node.right, out);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn find_min_priority_idx(items: &Vec<(T, u64)>, start: usize, end: usize) -> (min_idx: usize) {
            let mut min_idx = start;
            let mut i = start + 1;
            while i < end
                invariant
                    start <= min_idx,
                    min_idx < end,
                    min_idx < i,
                    i <= end,
                    end <= items.len(),
                decreases end - i,
            {
                if items[i].1 < items[min_idx].1 {
                    min_idx = i;
                }
                i = i + 1;
            }
            min_idx
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, O(n^2) worst
        fn build_treap_from_vec(items: &Vec<(T, u64)>, start: usize, end: usize) -> (treap: Link<T>)
            decreases end - start,
        {
            if start >= end {
                return None;
            }
            let min_idx = Self::find_min_priority_idx(items, start, end);
            let key = items[min_idx].0.clone();
            let priority = items[min_idx].1;
            let left = Self::build_treap_from_vec(items, start, min_idx);
            let right = Self::build_treap_from_vec(items, min_idx + 1, end);
            Self::make_node(key, priority, left, right)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_by_key(items: &Vec<(T, u64)>, key: &T) -> (filtered: Vec<(T, u64)>) {
            let mut filtered: Vec<(T, u64)> = Vec::new();
            let mut i: usize = 0;
            while i < items.len()
                invariant
                    i <= items.len(),
                    filtered.len() <= i,
                decreases items.len() - i,
            {
                if items[i].0 != *key {
                    filtered.push((items[i].0.clone(), items[i].1));
                }
                i = i + 1;
            }
            filtered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn rank_link(link: &Link<T>, key: &T) -> (rank: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    // Veracity: NEEDED proof block
                    proof { lemma_size_wf_child_bounded(link); }
                    let left_size = Self::size_link(&node.left);
                    if *key < node.key {
                        Self::rank_link(&node.left, key)
                    } else if *key == node.key {
                        left_size + 1
                    } else {
                        let r = Self::rank_link(&node.right, key);
                        left_size + 1 + r
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn select_link(link: &Link<T>, rank: usize) -> Option<&T>
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => {
                    let left_size = Self::size_link(&node.left);
                    if rank <= left_size {
                        Self::select_link(&node.left, rank)
                    } else if rank == left_size + 1 {
                        Some(&node.key)
                    } else {
                        Self::select_link(&node.right, rank - left_size - 1)
                    }
                }
            }
        }
    }

    //		Section 4c. type definitions


    pub struct Lnk;

    //		Section 6c. spec fns


    // Root key of a link (arbitrary if None).
    pub open spec fn spec_root_key_link<T: StT + Ord>(link: &Link<T>) -> T {
        match link {
            Some(node) => node.key,
            None => arbitrary(),
        }
    }

    // Whether a link's root has a left child.
    pub open spec fn spec_has_left_child_link<T: StT + Ord>(link: &Link<T>) -> bool {
        match link {
            Some(node) => node.left.is_some(),
            None => false,
        }
    }

    // Whether a link's root has a right child.
    pub open spec fn spec_has_right_child_link<T: StT + Ord>(link: &Link<T>) -> bool {
        match link {
            Some(node) => node.right.is_some(),
            None => false,
        }
    }

    //		Section 7c. proof fns/broadcast groups


    proof fn lemma_height_le_size<T: StT + Ord>(link: &Link<T>)
        requires
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) < usize::MAX as nat,
        ensures Lnk::spec_height_link(link) <= Lnk::spec_size_link(link),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_size_wf_child_bounded(link);
                lemma_height_le_size(&node.left);
                lemma_height_le_size(&node.right);
            }
        }
    }

    proof fn lemma_size_wf_child_bounded<T: StT + Ord>(link: &Link<T>)
        requires
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) > 0,
            Lnk::spec_size_link(link) < usize::MAX as nat,
        ensures
            match link {
                None => true,
                Some(node) => {
                    Lnk::spec_size_link(&node.left) < usize::MAX as nat
                    && Lnk::spec_size_link(&node.right) < usize::MAX as nat
                },
            },
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
            }
        }
    }

    proof fn lemma_wf_assemble<T: StT + Ord>(link: &Link<T>)
        requires
            match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right)
                    && Lnk::spec_link_size_wf(&node.left)
                    && Lnk::spec_link_size_wf(&node.right)
                }
            }
        ensures Lnk::spec_link_size_wf(link),
    {}

    proof fn lemma_ordered_assemble<T: StT + Ord>(link: &Link<T>)
        requires
            match link {
                None => true,
                Some(node) => {
                    Lnk::spec_ordered_link(&node.left)
                    && Lnk::spec_ordered_link(&node.right)
                    && (forall |k: T| #[trigger] Lnk::spec_content_link(&node.left).contains(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Less)
                    && (forall |k: T| #[trigger] Lnk::spec_content_link(&node.right).contains(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Greater)
                }
            }
        ensures Lnk::spec_ordered_link(link),
    {}

    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    proof fn lemma_cmp_antisymmetry<T: StT + Ord>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Greater,
        ensures
            b.cmp_spec(&a) == std::cmp::Ordering::Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec antisymmetry: Less(a,b) implies Greater(b,a).
    proof fn lemma_cmp_antisymmetry_lt<T: StT + Ord>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Less,
        ensures
            b.cmp_spec(&a) == std::cmp::Ordering::Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity for Less: Less(a,b) and Less(b,c) implies Less(a,c).
    proof fn lemma_cmp_transitivity_lt<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Less,
            b.cmp_spec(&c) == std::cmp::Ordering::Less,
        ensures
            a.cmp_spec(&c) == std::cmp::Ordering::Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity for Greater: Greater(a,b) and Greater(b,c) implies Greater(a,c).
    proof fn lemma_cmp_transitivity_gt<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Greater,
            b.cmp_spec(&c) == std::cmp::Ordering::Greater,
        ensures
            a.cmp_spec(&c) == std::cmp::Ordering::Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    //		Section 8c. traits


    pub trait LinkTrait<T: StT + Ord>: Sized {
        spec fn spec_size_link(link: &Link<T>) -> nat;
        spec fn spec_link_size_wf(link: &Link<T>) -> bool;
        spec fn spec_height_link(link: &Link<T>) -> nat;
        spec fn spec_content_link(link: &Link<T>) -> Set<T>;
        spec fn spec_ordered_link(link: &Link<T>) -> bool;
    }

    //		Section 9c. impls


    impl<T: StT + Ord> LinkTrait<T> for Lnk {
        open spec fn spec_size_link(link: &Link<T>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(node) => node.size as nat,
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

        open spec fn spec_height_link(link: &Link<T>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(node) => {
                    let lh = Self::spec_height_link(&node.left);
                    let rh = Self::spec_height_link(&node.right);
                    1 + if lh >= rh { lh } else { rh }
                }
            }
        }

        open spec fn spec_content_link(link: &Link<T>) -> Set<T>
            decreases *link,
        {
            match link {
                None => Set::empty(),
                Some(node) =>
                    Self::spec_content_link(&node.left)
                        .union(Self::spec_content_link(&node.right))
                        .insert(node.key),
            }
        }

        open spec fn spec_ordered_link(link: &Link<T>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(node) => {
                    Self::spec_ordered_link(&node.left)
                    && Self::spec_ordered_link(&node.right)
                    && (forall |k: T| #[trigger] Self::spec_content_link(&node.left).contains(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Less)
                    && (forall |k: T| #[trigger] Self::spec_content_link(&node.right).contains(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Greater)
                }
            }
        }
    }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn compare_links<T: StT + Ord>(a: &Link<T>, b: &Link<T>) -> (equal: bool)
        requires Lnk::spec_ordered_link(a), Lnk::spec_ordered_link(b),
        ensures
            (a is None && b is None) ==> equal,
            (a is Some && b is None) ==> !equal,
            (a is None && b is Some) ==> !equal,
        decreases *a,
    {
        match (a, b) {
            (None, None) => true,
            (Some(an), Some(bn)) => {
                if an.key != bn.key {
                    false
                } else {
                    compare_links(&an.left, &bn.left) && compare_links(&an.right, &bn.right)
                }
            }
            _ => false,
        }
    }


        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn clone_link<T: StT + Ord>(link: &Link<T>) -> (c: Link<T>)
        requires Lnk::spec_ordered_link(link),
        ensures
            Lnk::spec_content_link(&c) == Lnk::spec_content_link(link),
            Lnk::spec_size_link(&c) == Lnk::spec_size_link(link),
            Lnk::spec_link_size_wf(link) ==> Lnk::spec_link_size_wf(&c),
        decreases link,
    {
        match link {
            None => None,
            Some(node) => {
                let k = node.key.clone();
                // Veracity: NEEDED proof block
                proof { accept(k == node.key); } // accept hole: Clone bridge
                Some(Box::new(Node {
                    key: k,
                    priority: node.priority,
                    size: node.size,
                    left: clone_link(&node.left),
                    right: clone_link(&node.right),
                }))
            }
        }
    }

    //		Section 10b. iterators — BSTSizeStEph

    /// Snapshot iterator over BSTSizeStEph — collects elements via in_order traversal,
    /// then yields owned T values from the captured Vec.
    #[verifier::reject_recursive_types(T)]
    pub struct BSTSizeStEphIter<T: StT + Ord> {
        pub inner: IntoIter<T>,
    }

    impl<T: StT + Ord> View for BSTSizeStEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant_bstsizesteph<T: StT + Ord>(it: &BSTSizeStEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: StT + Ord> std::iter::Iterator for BSTSizeStEphIter<T> {
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

    /// Ghost iterator for for-loop support over BSTSizeStEphIter.
    #[verifier::reject_recursive_types(T)]
    pub struct BSTSizeStEphGhostIterator<T: StT + Ord> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: StT + Ord> View for BSTSizeStEphGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for BSTSizeStEphIter<T> {
        type GhostIter = BSTSizeStEphGhostIterator<T>;
        open spec fn ghost_iter(&self) -> BSTSizeStEphGhostIterator<T> {
            BSTSizeStEphGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StT + Ord> vstd::pervasive::ForLoopGhostIterator for BSTSizeStEphGhostIterator<T> {
        type ExecIter = BSTSizeStEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSizeStEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &BSTSizeStEphIter<T>) -> BSTSizeStEphGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT + Ord> std::iter::IntoIterator for &'a BSTSizeStEph<T> {
        type Item = T;
        type IntoIter = BSTSizeStEphIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_bstsizesteph_wf()
            ensures
                it@.0 == 0,
                it@.1.len() == self.spec_size(),
                iter_invariant_bstsizesteph(&it),
        {
            let in_ord = self.in_order();
            BSTSizeStEphIter { inner: in_ord.seq.into_iter() }
        }
    }

    //		Section 12a. derive impls in verus!


    impl<T: StT + Ord> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self) {
            // Veracity: NEEDED proof block
            proof { accept(Lnk::spec_ordered_link(&self.left)); accept(Lnk::spec_ordered_link(&self.right)); } // Clone body: ordering bridge
            Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            }
        }
    }

    impl<T: StT + Ord> Default for BSTreeSize<T> {
        fn default() -> (default_val: Self)
            ensures default_val.spec_size() == 0, default_val.spec_bstsizesteph_wf(), default_val@ == Set::<T>::empty(),
        { Self::new() }
    }

    //		Section 12b. derive impls in verus!


    impl<T: StT + Ord> Clone for BSTSizeStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned@ == self@,
                Lnk::spec_size_link(&cloned.root) == Lnk::spec_size_link(&self.root),
                Lnk::spec_link_size_wf(&self.root) ==> Lnk::spec_link_size_wf(&cloned.root),
        {
            // Veracity: NEEDED proof block
            proof { accept(Lnk::spec_ordered_link(&self.root)); } // Clone body: ordering bridge
            BSTSizeStEph { root: clone_link(&self.root) }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord> PartialEqSpecImpl for BSTSizeStEph<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord> Eq for BSTSizeStEph<T> {}

    impl<T: StT + Ord> PartialEq for BSTSizeStEph<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            // Veracity: NEEDED proof block
            proof { accept(Lnk::spec_ordered_link(&self.root)); accept(Lnk::spec_ordered_link(&other.root)); } // PartialEq body: ordering bridge
            let equal = compare_links(&self.root, &other.root);
            // Veracity: NEEDED proof block
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTSizeStEphLit {
        () => {
            < $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEph<_> as $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEph<_> as $crate::Chap40::BSTSizeStEph::BSTSizeStEph::BSTSizeStEphTrait<_> >::new();
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

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for Node<T> {
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

    impl<T: StT + Ord + fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})", self.key)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for BSTSizeStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSizeStEph").field("root", &self.root).finish()
        }
    }

    impl<T: StT + Ord + fmt::Display> fmt::Display for BSTSizeStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.root {
                None => write!(f, "BSTSizeStEph(empty)"),
                Some(_) => write!(f, "BSTSizeStEph(non-empty)"),
            }
        }
    }

    //		Section 14c. derive impls outside verus!

    impl fmt::Debug for Lnk {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Lnk")
        }
    }

    impl fmt::Display for Lnk {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Lnk")
        }
    }

    //		Section 14d. derive impls outside verus!

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for BSTSizeStEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSizeStEphIter")
        }
    }

    impl<T: StT + Ord + fmt::Display> fmt::Display for BSTSizeStEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSizeStEphIter")
        }
    }

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for BSTSizeStEphGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSizeStEphGhostIterator")
        }
    }

    impl<T: StT + Ord + fmt::Display> fmt::Display for BSTSizeStEphGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSizeStEphGhostIterator")
        }
    }
}
