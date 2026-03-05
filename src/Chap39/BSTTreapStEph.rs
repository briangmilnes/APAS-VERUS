//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral Treap (randomized heap-ordered BST) with `find` support.

//  Table of Contents
//	1. module
//	4. type definitions
//	6. spec fns
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod BSTTreapStEph {

    use std::fmt;

    use vstd::prelude::*;
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

    //		8. traits

    pub trait BSTTreapStEphTrait<T: StT + Ord + IsLtTransitive> {
        spec fn spec_size_link(link: &Link<T>) -> nat;
        spec fn spec_bst_link(link: &Link<T>) -> bool;
        spec fn spec_size_wf_link(link: &Link<T>) -> bool;
        spec fn spec_in_order_link(link: &Link<T>) -> Seq<T>;
        spec fn spec_pre_order_link(link: &Link<T>) -> Seq<T>;
        spec fn spec_min_link(link: &Link<T>) -> Option<T>;
        spec fn spec_max_link(link: &Link<T>) -> Option<T>;
        spec fn spec_height_link(link: &Link<T>) -> nat;

        spec fn spec_size(self) -> nat;
        spec fn spec_wf(self) -> bool;
        spec fn spec_bst(self) -> bool;
        spec fn spec_height(self) -> nat;
        spec fn spec_contains(self, target: T) -> bool;
        spec fn spec_min(self) -> Option<T>;
        spec fn spec_max(self) -> Option<T>;
        spec fn spec_in_order(self) -> Seq<T>;
        spec fn spec_pre_order(self) -> Seq<T>;

        proof fn lemma_height_le_size(link: &Link<T>)
            requires
                Self::spec_size_wf_link(link),
                Self::spec_size_link(link) < usize::MAX as nat,
            ensures Self::spec_height_link(link) <= Self::spec_size_link(link);

        proof fn lemma_size_wf_child_bounded(link: &Link<T>)
            requires
                Self::spec_size_wf_link(link),
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
            requires Self::spec_size_wf_link(link),
            ensures match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right)
                    && Self::spec_size_wf_link(&node.left)
                    && Self::spec_size_wf_link(&node.right)
                },
            };

        proof fn lemma_wf_assemble_node(node: &Box<Node<T>>)
            requires
                node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right),
                Self::spec_size_wf_link(&node.left),
                Self::spec_size_wf_link(&node.right),
            ensures Self::spec_size_wf_link(&Some(*node));

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
                empty_tree.spec_wf(),
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
                self.spec_wf(),
            ensures h as nat == self.spec_height();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires
                old(self).spec_size() + 1 <= usize::MAX as nat,
                old(self).spec_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                self.spec_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size(),
                forall|k: T| old(self).spec_contains(k) ==> self.spec_contains(k),
                old(self).spec_bst() ==> self.spec_bst();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn delete(&mut self, target: &T)
            requires
                old(self).spec_wf(),
                T::obeys_partial_cmp_spec(),
            ensures
                self.spec_wf(),
                self.spec_size() <= old(self).spec_size(),
                forall|k: T| self.spec_contains(k) ==> old(self).spec_contains(k),
                old(self).spec_bst() ==> self.spec_bst();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn find(&self, target: &T)     -> (found: Option<&T>)
            ensures found.is_some() ==> self.spec_contains(*found.unwrap());
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
        fn contains(&self, target: &T) -> (found: bool)
            ensures found ==> exists|v: T| self.spec_contains(v);
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
                Self::spec_size_wf_link(&Some(Box::new(n))),
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
                Self::spec_size_wf_link(&Some(x)),
                Self::spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                Self::spec_size_wf_link(&Some(rotated)),
                Self::spec_size_link(&Some(rotated)) == Self::spec_size_link(&Some(x)),
                Self::spec_bst_link(&Some(x)) ==> Self::spec_bst_link(&Some(rotated)),
                forall|k: T| spec_contains_link(&Some(rotated), k) <==> spec_contains_link(&Some(x), k);

        fn rotate_right(x: Box<Node<T>>) -> (rotated: Box<Node<T>>)
            requires
                Self::spec_size_wf_link(&Some(x)),
                Self::spec_size_link(&Some(x)) <= usize::MAX as nat,
            ensures
                Self::spec_size_wf_link(&Some(rotated)),
                Self::spec_size_link(&Some(rotated)) == Self::spec_size_link(&Some(x)),
                Self::spec_bst_link(&Some(x)) ==> Self::spec_bst_link(&Some(rotated)),
                forall|k: T| spec_contains_link(&Some(rotated), k) <==> spec_contains_link(&Some(x), k);

        fn clone_link(link: &Link<T>) -> (c: Link<T>)
            ensures
                Self::spec_size_link(&c) == Self::spec_size_link(link),
                Self::spec_size_wf_link(link) ==> Self::spec_size_wf_link(&c);

        fn height_link(link: &Link<T>) -> (h: usize)
            requires
                Self::spec_size_link(link) < usize::MAX as nat,
                Self::spec_size_wf_link(link),
            ensures h as nat == Self::spec_height_link(link);

        fn insert_link(link: Link<T>, value: T, priority: u64) -> (inserted: Link<T>)
            requires
                Self::spec_size_link(&link) + 1 <= usize::MAX as nat,
                Self::spec_size_wf_link(&link),
                T::obeys_partial_cmp_spec(),
            ensures
                Self::spec_size_wf_link(&inserted),
                Self::spec_size_link(&inserted) <= Self::spec_size_link(&link) + 1,
                Self::spec_size_link(&inserted) >= Self::spec_size_link(&link),
                forall|k: T| spec_contains_link(&link, k) ==> spec_contains_link(&inserted, k),
                forall|k: T| spec_contains_link(&inserted, k) ==> (spec_contains_link(&link, k) || k == value),
                Self::spec_bst_link(&link) ==> Self::spec_bst_link(&inserted);

        fn delete_link(link: Link<T>, target: &T) -> (deleted: Link<T>)
            requires
                Self::spec_size_wf_link(&link),
                T::obeys_partial_cmp_spec(),
            ensures
                Self::spec_size_wf_link(&deleted),
                Self::spec_size_link(&deleted) <= Self::spec_size_link(&link),
                forall|k: T| spec_contains_link(&deleted, k) ==> spec_contains_link(&link, k),
                Self::spec_bst_link(&link) ==> Self::spec_bst_link(&deleted);

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
            ensures found.is_some() ==> spec_contains_link(link, *found.unwrap());

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

        open spec fn spec_size_wf_link(link: &Link<T>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(node) => {
                    node.size as nat == 1 + Self::spec_size_link(&node.left) + Self::spec_size_link(&node.right)
                        && Self::spec_size_wf_link(&node.left)
                        && Self::spec_size_wf_link(&node.right)
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
        open spec fn spec_wf(self) -> bool { Self::spec_size_wf_link(&self.root) }
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
            assert(Self::spec_size_wf_link(&n.left));
            assert(Self::spec_size_wf_link(&n.right));
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
            assert(Self::spec_size_wf_link(&x.left));
            assert(Self::spec_size_wf_link(&x.right));
            if let Some(mut y) = x.right.take() {
                let ghost yk = y.key;
                let ghost b  = y.left;
                let ghost c  = y.right;

                assert(Self::spec_size_wf_link(&y.left));
                assert(Self::spec_size_wf_link(&y.right));
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
            assert(Self::spec_size_wf_link(&x.left));
            assert(Self::spec_size_wf_link(&x.right));
            if let Some(mut y) = x.left.take() {
                let ghost yk = y.key;
                let ghost b  = y.right;
                let ghost a  = y.left;

                assert(Self::spec_size_wf_link(&y.left));
                assert(Self::spec_size_wf_link(&y.right));
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
                    assert(Self::spec_size_wf_link(&node.left));
                    assert(Self::spec_size_wf_link(&node.right));
                    if value < node.key {
                        node.left = Self::insert_link(node.left.take(), value, priority);
                        Self::update_size(&mut node);
                        proof {
                            Self::lemma_wf_assemble_node(&node);
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
                        if needs_rotate { Some(Self::rotate_right(node)) } else { Some(node) }
                    } else if node.key < value {
                        node.right = Self::insert_link(node.right.take(), value, priority);
                        Self::update_size(&mut node);
                        proof {
                            Self::lemma_wf_assemble_node(&node);
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
                        if needs_rotate { Some(Self::rotate_left(node)) } else { Some(node) }
                    } else {
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
                    assert(Self::spec_size_wf_link(&node.left));
                    assert(Self::spec_size_wf_link(&node.right));
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
                                assert(Self::spec_size_wf_link(&Some(rotated)));
                                assert(Self::spec_size_wf_link(&rotated.right));
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
                                assert(Self::spec_size_wf_link(&Some(rotated)));
                                assert(Self::spec_size_wf_link(&rotated.left));
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
            match link {
                | None => None,
                | Some(node) => {
                    if *target == node.key {
                        proof { Self::lemma_contains_root(node); }
                        Some(&node.key)
                    } else if *target < node.key {
                        let r = Self::find_link(&node.left, target);
                        proof {
                            if r.is_some() { Self::lemma_contains_left(node, *r.unwrap()); }
                        }
                        r
                    } else {
                        let r = Self::find_link(&node.right, target);
                        proof {
                            if r.is_some() { Self::lemma_contains_right(node, *r.unwrap()); }
                        }
                        r
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
            ensures d.spec_size() == 0, d.spec_wf(), d.spec_bst(),
        { Self::new() }
    }


    //		11. derive impls in verus!

    impl<T: StT + Ord + IsLtTransitive> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures
                BSTTreapStEph::<T>::spec_size_link(&Some(Box::new(cloned))) == BSTTreapStEph::<T>::spec_size_link(&Some(Box::new(*self))),
                BSTTreapStEph::<T>::spec_size_wf_link(&Some(Box::new(*self))) ==> BSTTreapStEph::<T>::spec_size_wf_link(&Some(Box::new(cloned))),
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
                self.spec_wf() ==> cloned.spec_wf(),
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
