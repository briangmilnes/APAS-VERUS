//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Size-augmented BST with O(1) size queries and rank/select operations.

//  Table of Contents
//  1. module
//  2. imports
//  4. type definitions
//  5. view impls
//  7. proof fns
//  8. traits
//  9. impls
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!

// 1. module

pub mod BSTSizeStEph {

    use std::fmt;

    use vstd::prelude::*;

    verus! {

    // 2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    // 4. type definitions

    pub struct Node<T: StT + Ord> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    pub type Link<T> = Option<Box<Node<T>>>;

    pub struct BSTSizeStEph<T: StT + Ord> {
        pub root: Link<T>,
    }

    pub type BSTreeSize<T> = BSTSizeStEph<T>;

    pub struct Lnk;

    // 5. view impls

    impl<T: StT + Ord> View for BSTSizeStEph<T> {
        type V = Set<T>;
        open spec fn view(&self) -> Set<T> {
            Lnk::spec_content_link(&self.root)
        }
    }


    // 7. proof fns

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
                assert(node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right));
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


    // 8. traits

    pub trait LinkTrait<T: StT + Ord>: Sized {
        spec fn spec_size_link(link: &Link<T>) -> nat;
        spec fn spec_link_size_wf(link: &Link<T>) -> bool;
        spec fn spec_height_link(link: &Link<T>) -> nat;
        spec fn spec_content_link(link: &Link<T>) -> Set<T>;
    }

    pub trait NodeTrait<T: StT + Ord>: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_bstsizesteph_size_wf(&self) -> bool;

        spec fn spec_height(&self) -> nat;

        spec fn spec_content(&self) -> Set<T>;

        fn new(key: T, priority: u64) -> (node: Self);
    }

    pub trait BSTSizeStEphTrait<T: StT + Ord>: Sized + View<V = Set<T>> {
        spec fn spec_size(&self) -> nat;
        spec fn spec_bstsizesteph_wf(&self) -> bool;
        spec fn spec_height(&self) -> nat;

        /// - APAS: Work Θ(1), Span Θ(1)
        fn new() -> (empty: Self)
            ensures
                empty.spec_size() == 0,
                empty.spec_bstsizesteph_wf(),
                empty@ == Set::<T>::empty();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures count as nat == self.spec_size();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == (self.spec_size() == 0);
        /// - APAS: Work Θ(n), Span Θ(n)
        fn height(&self) -> (height: usize)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_bstsizesteph_wf(),
            ensures
                height as nat == self.spec_height();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires
                old(self).spec_size() + 1 <= usize::MAX as nat,
                old(self).spec_bstsizesteph_wf(),
            ensures
                self@ == old(self)@.insert(value),
                self.spec_bstsizesteph_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size();
        /// - APAS: Work Θ(n), Span Θ(n)
        fn delete(&mut self, key: &T)
            requires old(self).spec_bstsizesteph_wf(),
            ensures
                self@ == old(self)@.remove(*key),
                self.spec_bstsizesteph_wf(),
                self.spec_size() <= old(self).spec_size();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires self.spec_bstsizesteph_wf(),
            ensures
                found is Some <==> self@.contains(*target),
                found is Some ==> *found.unwrap() == *target;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn contains(&self, target: &T) -> (contains: bool)
            requires self.spec_bstsizesteph_wf(),
            ensures contains == self@.contains(*target);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn minimum(&self) -> (minimum: Option<&T>)
            requires self.spec_bstsizesteph_wf(),
            ensures
                self.spec_size() == 0 ==> minimum is None,
                self.spec_size() > 0 ==> minimum is Some;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn maximum(&self) -> (maximum: Option<&T>)
            requires self.spec_bstsizesteph_wf(),
            ensures
                self.spec_size() == 0 ==> maximum is None,
                self.spec_size() > 0 ==> maximum is Some;
        /// - APAS: Work Θ(n), Span Θ(n)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            requires self.spec_bstsizesteph_wf(),
            ensures ordered.spec_len() == self.spec_size();
        /// - APAS: Work Θ(log n), Span Θ(log n) — Algorithm 40.1
        fn rank(&self, key: &T) -> (rank: usize)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_bstsizesteph_wf(),
            ensures
                rank as nat <= self.spec_size();
        /// - APAS: Work Θ(log n), Span Θ(log n) — Algorithm 40.1
        fn select(&self, rank: usize) -> (selected: Option<&T>)
            ensures (rank == 0 || rank as nat > self.spec_size()) ==> selected is None;
        /// - APAS: Work Θ(log n), Span Θ(log n) — Exercise 40.1
        fn split_rank(&self, rank: usize) -> (split: (BSTSizeStEph<T>, BSTSizeStEph<T>))
            requires self.spec_bstsizesteph_wf(),
            ensures
                Lnk::spec_link_size_wf(&split.0.root),
                Lnk::spec_link_size_wf(&split.1.root);

        // Internal associated functions.

        fn size_link(link: &Link<T>) -> (count: usize)
            ensures count as nat == Lnk::spec_size_link(link);
        fn update_size(node: &mut Node<T>)
            requires
                Lnk::spec_size_link(&old(node).left) + Lnk::spec_size_link(&old(node).right) + 1 <= usize::MAX as nat,
            ensures
                node.size as nat == Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right) + 1,
                node.key == old(node).key,
                node.priority == old(node).priority,
                node.left == old(node).left,
                node.right == old(node).right;
        fn make_node(key: T, priority: u64, left: Link<T>, right: Link<T>) -> (node: Link<T>)
            requires
                Lnk::spec_size_link(&left) + Lnk::spec_size_link(&right) + 1 <= usize::MAX as nat,
            ensures
                Lnk::spec_size_link(&node) == Lnk::spec_size_link(&left) + Lnk::spec_size_link(&right) + 1,
                Lnk::spec_link_size_wf(&node) <==> (Lnk::spec_link_size_wf(&left) && Lnk::spec_link_size_wf(&right));
        fn rotate_left(link: &mut Link<T>)
            requires
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
            ensures
                Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
                Lnk::spec_link_size_wf(link);
        fn rotate_right(link: &mut Link<T>)
            requires
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
            ensures
                Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
                Lnk::spec_link_size_wf(link);
        fn insert_link(link: &mut Link<T>, value: T, priority: u64)
            requires
                Lnk::spec_size_link(old(link)) + 1 <= usize::MAX as nat,
                Lnk::spec_link_size_wf(old(link)),
            ensures
                Lnk::spec_link_size_wf(link),
                Lnk::spec_size_link(link) <= Lnk::spec_size_link(old(link)) + 1,
                Lnk::spec_size_link(link) >= Lnk::spec_size_link(old(link)),
            decreases old(link);
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            ensures link.is_none() ==> found.is_none(),
            decreases *link;
        fn min_link(link: &Link<T>) -> (minimum: Option<&T>)
            ensures
                link.is_none() ==> minimum.is_none(),
                link.is_some() ==> minimum.is_some(),
            decreases *link;
        fn max_link(link: &Link<T>) -> (maximum: Option<&T>)
            ensures
                link.is_none() ==> maximum.is_none(),
                link.is_some() ==> maximum.is_some(),
            decreases *link;
        fn height_link(link: &Link<T>) -> (h: usize)
            requires
                Lnk::spec_size_link(link) < usize::MAX as nat,
                Lnk::spec_link_size_wf(link),
            ensures h as nat == Lnk::spec_height_link(link),
            decreases *link;
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>)
            requires Lnk::spec_link_size_wf(link),
            ensures out.len() == old(out).len() + Lnk::spec_size_link(link),
            decreases *link;
        fn in_order_collect_with_priority(link: &Link<T>, out: &mut Vec<(T, u64)>)
            requires Lnk::spec_link_size_wf(link),
            ensures out.len() == old(out).len() + Lnk::spec_size_link(link),
            decreases *link;
        fn find_min_priority_idx(items: &Vec<(T, u64)>, start: usize, end: usize) -> (min_idx: usize)
            requires start < end, end <= items.len(),
            ensures start <= min_idx && min_idx < end;
        fn build_treap_from_vec(items: &Vec<(T, u64)>, start: usize, end: usize) -> (treap: Link<T>)
            requires start <= end, end <= items.len(),
            ensures
                Lnk::spec_size_link(&treap) == (end - start) as nat,
                Lnk::spec_link_size_wf(&treap),
            decreases end - start;
        fn filter_by_key(items: &Vec<(T, u64)>, key: &T) -> (filtered: Vec<(T, u64)>)
            ensures filtered.len() <= items.len();
        fn rank_link(link: &Link<T>, key: &T) -> (rank: usize)
            requires
                Lnk::spec_size_link(link) < usize::MAX as nat,
                Lnk::spec_link_size_wf(link),
            ensures rank as nat <= Lnk::spec_size_link(link),
            decreases *link;
        fn select_link(link: &Link<T>, rank: usize) -> (selected: Option<&T>)
            ensures link.is_none() ==> selected.is_none(),
            decreases *link;
    }


    // 9. impls

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
    }

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

    fn compare_links<T: StT + Ord>(a: &Link<T>, b: &Link<T>) -> (equal: bool)
        requires true,
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

    impl<T: StT + Ord> BSTSizeStEphTrait<T> for BSTSizeStEph<T> {
        open spec fn spec_size(&self) -> nat { Lnk::spec_size_link(&self.root) }
        open spec fn spec_bstsizesteph_wf(&self) -> bool { Lnk::spec_link_size_wf(&self.root) }
        open spec fn spec_height(&self) -> nat { Lnk::spec_height_link(&self.root) }

        fn new() -> (empty: Self) { BSTSizeStEph { root: None } }

        fn size(&self) -> (count: usize) { Self::size_link(&self.root) }

        fn is_empty(&self) -> (is_empty: bool) { self.size() == 0 }

        fn height(&self) -> (height: usize) { Self::height_link(&self.root) }

        #[verifier::external_body]
        fn insert(&mut self, value: T, priority: u64) {
            Self::insert_link(&mut self.root, value, priority);
        }

        #[verifier::external_body]
        fn delete(&mut self, key: &T) {
            let mut items: Vec<(T, u64)> = Vec::new();
            Self::in_order_collect_with_priority(&self.root, &mut items);
            let filtered = Self::filter_by_key(&items, key);
            self.root = Self::build_treap_from_vec(&filtered, 0, filtered.len());
        }

        #[verifier::external_body]
        fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        #[verifier::external_body]
        fn contains(&self, target: &T) -> bool { self.find(target).is_some() }

        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out: Vec<T> = Vec::new();
            Self::in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn rank(&self, key: &T) -> usize { Self::rank_link(&self.root, key) }

        fn select(&self, rank: usize) -> Option<&T> {
            if rank == 0 || rank > self.size() {
                None
            } else {
                Self::select_link(&self.root, rank)
            }
        }

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

        fn size_link(link: &Link<T>) -> (count: usize) {
            match link {
                None => 0,
                Some(n) => n.size,
            }
        }

        fn update_size(node: &mut Node<T>) {
            let l = Self::size_link(&node.left);
            let r = Self::size_link(&node.right);
            node.size = 1 + l + r;
        }

        fn make_node(key: T, priority: u64, left: Link<T>, right: Link<T>) -> (node: Link<T>) {
            let mut node = Node::new(key, priority);
            node.left = left;
            node.right = right;
            Self::update_size(&mut node);
            Some(Box::new(node))
        }

        fn rotate_left(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                let ghost xl = Lnk::spec_size_link(&x.left);
                let ghost xr = Lnk::spec_size_link(&x.right);
                assert(x.size as nat == 1 + xl + xr);
                assert(Lnk::spec_link_size_wf(&x.left));
                assert(Lnk::spec_link_size_wf(&x.right));

                if let Some(mut y) = x.right.take() {
                    let ghost yl = Lnk::spec_size_link(&y.left);
                    let ghost yr = Lnk::spec_size_link(&y.right);
                    assert(y.size as nat == 1 + yl + yr);
                    assert(Lnk::spec_link_size_wf(&y.left));
                    assert(Lnk::spec_link_size_wf(&y.right));

                    x.right = y.left.take();
                    assert(Lnk::spec_link_size_wf(&x.right));
                    assert(Lnk::spec_link_size_wf(&x.left));
                    Self::update_size(&mut *x);

                    y.left = Some(x);
                    Self::update_size(&mut *y);
                    assert(Lnk::spec_link_size_wf(&y.right));
                    *link = Some(y);
                    proof { lemma_wf_assemble(link); }
                } else {
                    *link = Some(x);
                }
            }
        }

        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                let ghost xl = Lnk::spec_size_link(&x.left);
                let ghost xr = Lnk::spec_size_link(&x.right);
                assert(x.size as nat == 1 + xl + xr);
                assert(Lnk::spec_link_size_wf(&x.left));
                assert(Lnk::spec_link_size_wf(&x.right));

                if let Some(mut y) = x.left.take() {
                    let ghost yl = Lnk::spec_size_link(&y.left);
                    let ghost yr = Lnk::spec_size_link(&y.right);
                    assert(y.size as nat == 1 + yl + yr);
                    assert(Lnk::spec_link_size_wf(&y.left));
                    assert(Lnk::spec_link_size_wf(&y.right));

                    x.left = y.right.take();
                    assert(Lnk::spec_link_size_wf(&x.left));
                    assert(Lnk::spec_link_size_wf(&x.right));
                    Self::update_size(&mut *x);

                    y.right = Some(x);
                    Self::update_size(&mut *y);
                    assert(Lnk::spec_link_size_wf(&y.left));
                    *link = Some(y);
                    proof { lemma_wf_assemble(link); }
                } else {
                    *link = Some(x);
                }
            }
        }

        fn insert_link(link: &mut Link<T>, value: T, priority: u64)
            decreases old(link),
        {
            if let Some(mut node) = link.take() {
                let ghost old_left = Lnk::spec_size_link(&node.left);
                let ghost old_right = Lnk::spec_size_link(&node.right);
                assert(node.size as nat == 1 + old_left + old_right);
                assert(Lnk::spec_link_size_wf(&node.left));
                assert(Lnk::spec_link_size_wf(&node.right));

                if value < node.key {
                    Self::insert_link(&mut node.left, value, priority);
                    assert(Lnk::spec_link_size_wf(&node.right));
                    Self::update_size(&mut *node);
                    assert(Lnk::spec_link_size_wf(&node.right));
                    *link = Some(node);
                    proof { lemma_wf_assemble(link); }
                    let need_rotate = match link.as_ref().unwrap().left.as_ref() {
                        Some(left) => left.priority < link.as_ref().unwrap().priority,
                        None => false,
                    };
                    if need_rotate {
                        Self::rotate_right(link);
                    }
                } else if value > node.key {
                    Self::insert_link(&mut node.right, value, priority);
                    assert(Lnk::spec_link_size_wf(&node.left));
                    Self::update_size(&mut *node);
                    assert(Lnk::spec_link_size_wf(&node.left));
                    *link = Some(node);
                    proof { lemma_wf_assemble(link); }
                    let need_rotate = match link.as_ref().unwrap().right.as_ref() {
                        Some(right) => right.priority < link.as_ref().unwrap().priority,
                        None => false,
                    };
                    if need_rotate {
                        Self::rotate_left(link);
                    }
                } else {
                    *link = Some(node);
                    proof { lemma_wf_assemble(link); }
                }
            } else {
                *link = Some(Box::new(Node {
                    key: value,
                    priority,
                    size: 1,
                    left: None,
                    right: None,
                }));
                proof { lemma_wf_assemble(link); }
            }
        }

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => {
                    if *target == node.key {
                        Some(&node.key)
                    } else if *target < node.key {
                        Self::find_link(&node.left, target)
                    } else {
                        Self::find_link(&node.right, target)
                    }
                }
            }
        }

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

        fn height_link(link: &Link<T>) -> (h: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    proof { lemma_size_wf_child_bounded(link); }
                    let lh = Self::height_link(&node.left);
                    let rh = Self::height_link(&node.right);
                    let m = if lh >= rh { lh } else { rh };
                    proof {
                        lemma_height_le_size(&node.left);
                        lemma_height_le_size(&node.right);
                        assert(lh as nat == Lnk::spec_height_link(&node.left));
                        assert(rh as nat == Lnk::spec_height_link(&node.right));
                        assert(m as nat <= Lnk::spec_size_link(&node.left) || m as nat <= Lnk::spec_size_link(&node.right));
                        assert(m < usize::MAX);
                    }
                    1 + m
                }
            }
        }

        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::in_order_collect(&node.left, out);
                out.push(node.key.clone());
                Self::in_order_collect(&node.right, out);
            }
        }

        fn in_order_collect_with_priority(link: &Link<T>, out: &mut Vec<(T, u64)>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::in_order_collect_with_priority(&node.left, out);
                out.push((node.key.clone(), node.priority));
                Self::in_order_collect_with_priority(&node.right, out);
            }
        }

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

        fn rank_link(link: &Link<T>, key: &T) -> (rank: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
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

    // 11. derive impls in verus!

    fn clone_link<T: StT + Ord>(link: &Link<T>) -> (c: Link<T>)
        requires true,
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

    impl<T: StT + Ord> Default for BSTreeSize<T> {
        fn default() -> (default_val: Self)
            ensures default_val.spec_size() == 0, default_val.spec_bstsizesteph_wf(), default_val@ == Set::<T>::empty(),
        { Self::new() }
    }


    impl<T: StT + Ord> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self) {
            Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            }
        }
    }

    impl<T: StT + Ord> Clone for BSTSizeStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned@ == self@,
                Lnk::spec_size_link(&cloned.root) == Lnk::spec_size_link(&self.root),
                Lnk::spec_link_size_wf(&self.root) ==> Lnk::spec_link_size_wf(&cloned.root),
        {
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
            let equal = compare_links(&self.root, &other.root);
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

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

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for BSTSizeStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSizeStEph").field("root", &self.root).finish()
        }
    }

    impl<T: StT + Ord + fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})", self.key)
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
}
