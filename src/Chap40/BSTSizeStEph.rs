//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Size-augmented BST with O(1) size queries and rank/select operations.

pub mod BSTSizeStEph {

    use std::fmt;

    use vstd::prelude::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    // 2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    // 4. type definitions

    pub struct Node<T: StT + Ord> {
        pub key: T,
        pub priority: u64,
        pub size: N,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    pub type Link<T> = Option<Box<Node<T>>>;

    pub struct BSTSizeStEph<T: StT + Ord> {
        pub root: Link<T>,
    }

    pub type BSTreeSize<T> = BSTSizeStEph<T>;

    // 6. spec fns

    pub open spec fn spec_size_link<T: StT + Ord>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_size_wf_link<T: StT + Ord>(link: &Link<T>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(node) => {
                node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right)
                    && spec_size_wf_link(&node.left)
                    && spec_size_wf_link(&node.right)
            }
        }
    }

    pub open spec fn spec_height_link<T: StT + Ord>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let lh = spec_height_link(&node.left);
                let rh = spec_height_link(&node.right);
                1 + if lh >= rh { lh } else { rh }
            }
        }
    }

    // 7. proof fns

    proof fn lemma_height_le_size<T: StT + Ord>(link: &Link<T>)
        requires
            spec_size_wf_link(link),
            spec_size_link(link) < usize::MAX as nat,
        ensures spec_height_link(link) <= spec_size_link(link),
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
            spec_size_wf_link(link),
            spec_size_link(link) > 0,
            spec_size_link(link) < usize::MAX as nat,
        ensures
            match link {
                None => true,
                Some(node) => {
                    spec_size_link(&node.left) < usize::MAX as nat
                    && spec_size_link(&node.right) < usize::MAX as nat
                },
            },
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                assert(node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right));
            }
        }
    }

    proof fn lemma_wf_assemble<T: StT + Ord>(link: &Link<T>)
        requires
            match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right)
                    && spec_size_wf_link(&node.left)
                    && spec_size_wf_link(&node.right)
                }
            }
        ensures spec_size_wf_link(link),
    {}

    // 8. traits

    trait NodeTrait<T: StT + Ord>: Sized {
        fn new(key: T, priority: u64) -> Self;
    }

    pub trait BSTSizeStEphTrait<T: StT + Ord>: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_wf(&self) -> bool;
        spec fn spec_height(&self) -> nat;

        /// - APAS: Work Θ(1), Span Θ(1)
        fn new() -> (result: Self)
            ensures
                result.spec_size() == 0,
                result.spec_wf();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: N)
            ensures result as nat == self.spec_size();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (result: B)
            ensures result == (self.spec_size() == 0);
        /// - APAS: Work Θ(n), Span Θ(n)
        fn height(&self) -> (result: N)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_wf(),
            ensures
                result as nat == self.spec_height();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires
                old(self).spec_size() + 1 <= usize::MAX as nat,
                old(self).spec_wf(),
            ensures
                self.spec_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size();
        /// - APAS: Work Θ(n), Span Θ(n)
        fn delete(&mut self, key: &T)
            ensures self.spec_wf();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn find(&self, target: &T) -> Option<&T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn contains(&self, target: &T) -> B;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn minimum(&self) -> Option<&T>;
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn maximum(&self) -> Option<&T>;
        /// - APAS: Work Θ(n), Span Θ(n)
        fn in_order(&self) -> ArraySeqStPerS<T>;
        /// - APAS: Work Θ(log n), Span Θ(log n) — Algorithm 40.1
        fn rank(&self, key: &T) -> (result: N)
            requires
                self.spec_size() < usize::MAX as nat,
                self.spec_wf(),
            ensures
                result as nat <= self.spec_size();
        /// - APAS: Work Θ(log n), Span Θ(log n) — Algorithm 40.1
        fn select(&self, rank: N) -> Option<&T>;
        /// - APAS: Work Θ(log n), Span Θ(log n) — Exercise 40.1
        fn split_rank(&self, rank: N) -> (BSTSizeStEph<T>, BSTSizeStEph<T>);

        // Internal associated functions.

        fn size_link(link: &Link<T>) -> (result: N)
            ensures result as nat == spec_size_link(link);
        fn update_size(node: &mut Node<T>)
            requires
                spec_size_link(&old(node).left) + spec_size_link(&old(node).right) + 1 <= usize::MAX as nat,
            ensures
                node.size as nat == spec_size_link(&node.left) + spec_size_link(&node.right) + 1,
                node.key == old(node).key,
                node.priority == old(node).priority,
                node.left == old(node).left,
                node.right == old(node).right;
        fn make_node(key: T, priority: u64, left: Link<T>, right: Link<T>) -> (result: Link<T>)
            requires
                spec_size_link(&left) + spec_size_link(&right) + 1 <= usize::MAX as nat,
            ensures
                spec_size_link(&result) == spec_size_link(&left) + spec_size_link(&right) + 1,
                spec_size_wf_link(&result) <==> (spec_size_wf_link(&left) && spec_size_wf_link(&right));
        fn rotate_left(link: &mut Link<T>)
            requires
                spec_size_wf_link(old(link)),
                spec_size_link(old(link)) <= usize::MAX as nat,
            ensures
                spec_size_link(link) == spec_size_link(old(link)),
                spec_size_wf_link(link);
        fn rotate_right(link: &mut Link<T>)
            requires
                spec_size_wf_link(old(link)),
                spec_size_link(old(link)) <= usize::MAX as nat,
            ensures
                spec_size_link(link) == spec_size_link(old(link)),
                spec_size_wf_link(link);
        fn insert_link(link: &mut Link<T>, value: T, priority: u64)
            requires
                spec_size_link(old(link)) + 1 <= usize::MAX as nat,
                spec_size_wf_link(old(link)),
            ensures
                spec_size_wf_link(link),
                spec_size_link(link) <= spec_size_link(old(link)) + 1,
                spec_size_link(link) >= spec_size_link(old(link)),
            decreases old(link);
        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T>
            decreases *link;
        fn min_link(link: &Link<T>) -> Option<&T>
            decreases *link;
        fn max_link(link: &Link<T>) -> Option<&T>
            decreases *link;
        fn height_link(link: &Link<T>) -> (h: N)
            requires
                spec_size_link(link) < usize::MAX as nat,
                spec_size_wf_link(link),
            ensures h as nat == spec_height_link(link),
            decreases *link;
        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>)
            decreases *link;
        fn in_order_collect_with_priority(link: &Link<T>, out: &mut Vec<(T, u64)>)
            decreases *link;
        fn find_min_priority_idx(items: &Vec<(T, u64)>, start: usize, end: usize) -> (result: usize)
            requires start < end, end <= items.len(),
            ensures start <= result && result < end;
        fn build_treap_from_vec(items: &Vec<(T, u64)>, start: usize, end: usize) -> (result: Link<T>)
            requires start <= end, end <= items.len(),
            ensures
                spec_size_link(&result) == (end - start) as nat,
                spec_size_wf_link(&result),
            decreases end - start;
        fn filter_by_key(items: &Vec<(T, u64)>, key: &T) -> (result: Vec<(T, u64)>);
        fn rank_link(link: &Link<T>, key: &T) -> (result: N)
            requires
                spec_size_link(link) < usize::MAX as nat,
                spec_size_wf_link(link),
            ensures result as nat <= spec_size_link(link),
            decreases *link;
        fn select_link(link: &Link<T>, rank: N) -> Option<&T>
            decreases *link;
    }

    // 9. impls

    impl<T: StT + Ord> NodeTrait<T> for Node<T> {
        fn new(key: T, priority: u64) -> Self {
            Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    impl<T: StT + Ord> BSTSizeStEphTrait<T> for BSTSizeStEph<T> {
        open spec fn spec_size(&self) -> nat { spec_size_link(&self.root) }
        open spec fn spec_wf(&self) -> bool { spec_size_wf_link(&self.root) }
        open spec fn spec_height(&self) -> nat { spec_height_link(&self.root) }

        fn new() -> (result: Self) { BSTSizeStEph { root: None } }

        fn size(&self) -> (result: N) { Self::size_link(&self.root) }

        fn is_empty(&self) -> (result: B) { self.size() == 0 }

        fn height(&self) -> (result: N) { Self::height_link(&self.root) }

        fn insert(&mut self, value: T, priority: u64) {
            Self::insert_link(&mut self.root, value, priority);
        }

        fn delete(&mut self, key: &T) {
            let mut items: Vec<(T, u64)> = Vec::new();
            Self::in_order_collect_with_priority(&self.root, &mut items);
            let filtered = Self::filter_by_key(&items, key);
            self.root = Self::build_treap_from_vec(&filtered, 0, filtered.len());
        }

        fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out: Vec<T> = Vec::new();
            Self::in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn rank(&self, key: &T) -> N { Self::rank_link(&self.root, key) }

        fn select(&self, rank: N) -> Option<&T> {
            if rank == 0 || rank > self.size() {
                None
            } else {
                Self::select_link(&self.root, rank)
            }
        }

        fn split_rank(&self, rank: N) -> (BSTSizeStEph<T>, BSTSizeStEph<T>) {
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

        fn size_link(link: &Link<T>) -> (result: N) {
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

        fn make_node(key: T, priority: u64, left: Link<T>, right: Link<T>) -> (result: Link<T>) {
            let mut node = Node::new(key, priority);
            node.left = left;
            node.right = right;
            Self::update_size(&mut node);
            Some(Box::new(node))
        }

        fn rotate_left(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                let ghost xl = spec_size_link(&x.left);
                let ghost xr = spec_size_link(&x.right);
                assert(x.size as nat == 1 + xl + xr);
                assert(spec_size_wf_link(&x.left));
                assert(spec_size_wf_link(&x.right));

                if let Some(mut y) = x.right.take() {
                    let ghost yl = spec_size_link(&y.left);
                    let ghost yr = spec_size_link(&y.right);
                    assert(y.size as nat == 1 + yl + yr);
                    assert(spec_size_wf_link(&y.left));
                    assert(spec_size_wf_link(&y.right));

                    x.right = y.left.take();
                    assert(spec_size_wf_link(&x.right));
                    assert(spec_size_wf_link(&x.left));
                    Self::update_size(&mut *x);

                    y.left = Some(x);
                    Self::update_size(&mut *y);
                    assert(spec_size_wf_link(&y.right));
                    *link = Some(y);
                    proof { lemma_wf_assemble(link); }
                } else {
                    *link = Some(x);
                }
            }
        }

        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                let ghost xl = spec_size_link(&x.left);
                let ghost xr = spec_size_link(&x.right);
                assert(x.size as nat == 1 + xl + xr);
                assert(spec_size_wf_link(&x.left));
                assert(spec_size_wf_link(&x.right));

                if let Some(mut y) = x.left.take() {
                    let ghost yl = spec_size_link(&y.left);
                    let ghost yr = spec_size_link(&y.right);
                    assert(y.size as nat == 1 + yl + yr);
                    assert(spec_size_wf_link(&y.left));
                    assert(spec_size_wf_link(&y.right));

                    x.left = y.right.take();
                    assert(spec_size_wf_link(&x.left));
                    assert(spec_size_wf_link(&x.right));
                    Self::update_size(&mut *x);

                    y.right = Some(x);
                    Self::update_size(&mut *y);
                    assert(spec_size_wf_link(&y.left));
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
                let ghost old_left = spec_size_link(&node.left);
                let ghost old_right = spec_size_link(&node.right);
                assert(node.size as nat == 1 + old_left + old_right);
                assert(spec_size_wf_link(&node.left));
                assert(spec_size_wf_link(&node.right));

                if value < node.key {
                    Self::insert_link(&mut node.left, value, priority);
                    assert(spec_size_wf_link(&node.right));
                    Self::update_size(&mut *node);
                    assert(spec_size_wf_link(&node.right));
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
                    assert(spec_size_wf_link(&node.left));
                    Self::update_size(&mut *node);
                    assert(spec_size_wf_link(&node.left));
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

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T>
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

        fn min_link(link: &Link<T>) -> Option<&T>
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

        fn max_link(link: &Link<T>) -> Option<&T>
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

        fn height_link(link: &Link<T>) -> (h: N)
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
                        assert(lh as nat == spec_height_link(&node.left));
                        assert(rh as nat == spec_height_link(&node.right));
                        assert(m as nat <= spec_size_link(&node.left) || m as nat <= spec_size_link(&node.right));
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

        fn find_min_priority_idx(items: &Vec<(T, u64)>, start: usize, end: usize) -> (result: usize) {
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

        fn build_treap_from_vec(items: &Vec<(T, u64)>, start: usize, end: usize) -> (result: Link<T>)
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

        fn filter_by_key(items: &Vec<(T, u64)>, key: &T) -> (result: Vec<(T, u64)>) {
            let mut filtered: Vec<(T, u64)> = Vec::new();
            let mut i: usize = 0;
            while i < items.len()
                invariant i <= items.len(),
                decreases items.len() - i,
            {
                if items[i].0 != *key {
                    filtered.push((items[i].0.clone(), items[i].1));
                }
                i = i + 1;
            }
            filtered
        }

        fn rank_link(link: &Link<T>, key: &T) -> (result: N)
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

        fn select_link(link: &Link<T>, rank: N) -> Option<&T>
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
        decreases link,
    {
        match link {
            None => None,
            Some(node) => Some(Box::new(Node {
                key: node.key.clone(),
                priority: node.priority,
                size: node.size,
                left: clone_link(&node.left),
                right: clone_link(&node.right),
            })),
        }
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
        fn clone(&self) -> (result: Self)
            ensures true,
        {
            BSTSizeStEph { root: clone_link(&self.root) }
        }
    }

    impl<T: StT + Ord> Default for BSTreeSize<T> {
        fn default() -> Self { Self::new() }
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
}
