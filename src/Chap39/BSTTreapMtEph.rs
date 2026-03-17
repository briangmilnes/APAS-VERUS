//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral Treap (randomized heap-ordered BST) with interior locking for multi-threaded access.

//  Table of Contents
//	1. module
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//	1. module


pub mod BSTTreapMtEph {

    use vstd::prelude::*;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdIs;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdSpec;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::total_order::total_order::IsLtTransitive;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

    //		3. broadcast use

    broadcast use vstd::set::group_set_axioms;

    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StTInMtT + Ord> {
        pub key: T,
        pub priority: u64,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    pub struct BSTTreapMtEphInv;

    #[verifier::reject_recursive_types(T)]
    pub struct BSTTreapMtEph<T: StTInMtT + Ord + IsLtTransitive> {
        pub(crate) locked_root: RwLock<Link<T>, BSTTreapMtEphInv>,
        pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
    }

    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;

    pub struct Lnk;

    //		5. view impls

    impl<T: StTInMtT + Ord + IsLtTransitive> BSTTreapMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_root@.finite()
        }

        pub closed spec fn spec_ghost_locked_root(self) -> Set<<T as View>::V> {
            self.ghost_locked_root@
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> View for BSTTreapMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_ghost_locked_root() }
    }

    //		6. spec fns

    pub open spec fn spec_bsttreapmteph_link_wf<T: StTInMtT + Ord + IsLtTransitive>(link: &Link<T>) -> bool {
        Lnk::spec_link_size_wf(link)
        && Lnk::spec_size_link(link) < usize::MAX as nat
        && Lnk::spec_bst_link(link)
    }

    // Free spec fn: reveal_with_fuel cannot reference trait methods with generic params.
    pub open spec fn spec_contains_link<T: StTInMtT + Ord>(link: &Link<T>, val: T) -> bool
        decreases *link,
    {
        match link {
            None => false,
            Some(node) => {
                node.key == val
                    || spec_contains_link(&node.left, val)
                    || spec_contains_link(&node.right, val)
            }
        }
    }


    //		7. proof fns/broadcast groups

    proof fn lemma_bst_decompose<T: StTInMtT + Ord>(link: &Link<T>)
        requires Lnk::spec_bst_link(link),
        ensures match link {
            None => true,
            Some(node) => {
                Lnk::spec_bst_link(&node.left)
                && Lnk::spec_bst_link(&node.right)
                && (forall|k: T| #[trigger] spec_contains_link(&node.left, k) ==> k.is_lt(&node.key))
                && (forall|k: T| #[trigger] spec_contains_link(&node.right, k) ==> node.key.is_lt(&k))
            },
        },
    {
    }

    proof fn lemma_contains_left<T: StTInMtT + Ord>(node: &Box<Node<T>>, k: T)
        requires spec_contains_link(&node.left, k),
        ensures spec_contains_link(&Some(*node), k),
    {
    }

    proof fn lemma_contains_right<T: StTInMtT + Ord>(node: &Box<Node<T>>, k: T)
        requires spec_contains_link(&node.right, k),
        ensures spec_contains_link(&Some(*node), k),
    {
    }

    proof fn lemma_contains_root<T: StTInMtT + Ord>(node: &Box<Node<T>>)
        ensures spec_contains_link(&Some(*node), node.key),
    {
    }

    proof fn lemma_height_le_size<T: StTInMtT + Ord>(link: &Link<T>)
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

    proof fn lemma_size_wf_child_bounded<T: StTInMtT + Ord>(link: &Link<T>)
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

    /// - APAS: Work Θ(1), Span Θ(1)
    proof fn lemma_wf_assemble_node<T: StTInMtT + Ord>(node: &Node<T>)
        requires
            node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right),
            Lnk::spec_link_size_wf(&node.left),
            Lnk::spec_link_size_wf(&node.right),
        ensures Lnk::spec_link_size_wf(&Some(Box::new(*node))),
    {
    }


    //		8. traits

    pub trait LinkTrait<T: StTInMtT + Ord>: Sized {
        spec fn spec_size_link(link: &Link<T>) -> nat;
        spec fn spec_link_size_wf(link: &Link<T>) -> bool;
        spec fn spec_height_link(link: &Link<T>) -> nat;
        spec fn spec_bst_link(link: &Link<T>) -> bool;
    }

    /// Treap trait for multi-threaded ephemeral access.
    ///
    /// The RwLock invariant (`BSTTreapMtEphInv`) enforces `spec_bsttreapmteph_link_wf` on the link
    /// (size well-formedness, size < MAX, and BST ordering) on every acquire/release.
    /// Read-only methods have Set-based specs via external_body View.
    /// Interior mutability via RwLock precludes `old()` specs on insert/delete.
    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord + IsLtTransitive>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_bsttreapmteph_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        fn new() -> (empty_tree: Self)
            ensures empty_tree@ == Set::<<T as View>::V>::empty(), empty_tree.spec_bsttreapmteph_wf();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn insert(&mut self, value: T, priority: u64)
            requires T::obeys_partial_cmp_spec(),
            ensures self@.contains(value@), self.spec_bsttreapmteph_wf();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn delete(&mut self, target: &T)
            requires T::obeys_partial_cmp_spec(),
            ensures !self@.contains(target@), self.spec_bsttreapmteph_wf();
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn find(&self, target: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(target@);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn contains(&self, target: &T) -> (found: bool)
            ensures found <==> self@.contains(target@);
        /// - APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - APAS: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        fn height(&self) -> (h: usize);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn minimum(&self) -> (min_val: Option<T>)
            ensures min_val.is_some() ==> self@.contains(min_val.unwrap()@);
        /// - APAS: Work O(log n) expected, Span O(log n) expected
        fn maximum(&self) -> (max_val: Option<T>)
            ensures max_val.is_some() ==> self@.contains(max_val.unwrap()@);
        /// - APAS: Work Θ(n), Span Θ(n)
        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            ensures ordered@.len() == self@.len();
        /// - APAS: Work Θ(n), Span Θ(n)
        fn pre_order(&self) -> (preordered: ArraySeqStPerS<T>)
            ensures preordered@.len() == self@.len();
    }


    //		9. impls

    impl<T: StTInMtT + Ord> LinkTrait<T> for Lnk {
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
    }

    fn clone_link<T: StTInMtT + Ord + Clone>(link: &Link<T>) -> (c: Link<T>)
        requires true,
        ensures
            Lnk::spec_size_link(&c) == Lnk::spec_size_link(link),
            Lnk::spec_link_size_wf(link) ==> Lnk::spec_link_size_wf(&c),
        decreases link,
    {
        match link {
            None => {
                let c = None;
                proof { accept(c == *link); }
                c
            }
            Some(node) => {
                let left = clone_link(&node.left);
                let right = clone_link(&node.right);
                let c = Some(Box::new(Node {
                    key: node.key.clone(),
                    priority: node.priority,
                    size: node.size,
                    left,
                    right,
                }));
                proof { accept(c == *link); }
                c
            }
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> RwLockPredicate<Link<T>> for BSTTreapMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            spec_bsttreapmteph_link_wf(&v)
        }
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> (sz: usize)
        requires true,
        ensures sz as nat == Lnk::spec_size_link(link),
    {
        match link {
            None => 0,
            Some(n) => n.size,
        }
    }

    fn update<T: StTInMtT + Ord>(node: &mut Node<T>)
        requires 1 + Lnk::spec_size_link(&old(node).left) + Lnk::spec_size_link(&old(node).right) <= usize::MAX as nat,
        ensures
            node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right),
            node.key == old(node).key,
            node.left == old(node).left,
            node.right == old(node).right,
    {
        let l = size_link(&node.left);
        let r = size_link(&node.right);
        node.size = 1 + l + r;
    }

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn rotate_left<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>)
        requires
            Lnk::spec_link_size_wf(old(link)),
            Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(link, k) <==> spec_contains_link(old(link), k),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
    {
        if let Some(mut x) = link.take() {
            let ghost bst_input = Lnk::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_right = x.right;
            assert(Lnk::spec_link_size_wf(&x.left));
            assert(Lnk::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.right.take() {
                let ghost yk = y.key;
                let ghost b  = y.left;
                let ghost a  = y.right;
                assert(Lnk::spec_link_size_wf(&y.left));
                assert(Lnk::spec_link_size_wf(&y.right));
                let ghost x_left_sz = Lnk::spec_size_link(&x.left);
                let ghost y_left_sz = Lnk::spec_size_link(&y.left);
                let ghost y_right_sz = Lnk::spec_size_link(&y.right);
                proof {
                    if bst_input {
                        lemma_bst_decompose(&orig_right);
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies xk.is_lt(&k) by {
                            lemma_contains_left(&y, k);
                        };
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies k.is_lt(&yk) by {};
                    }
                }
                x.right = y.left.take();
                assert(1 + x_left_sz + y_left_sz + 1 + y_right_sz <= usize::MAX as nat);
                update(&mut x);
                proof {
                    if bst_input {
                        assert(Lnk::spec_bst_link(&x.right));
                        assert(Lnk::spec_bst_link(&x.left));
                        assert(Lnk::spec_bst_link(&Some(x)));
                    }
                }
                y.left = Some(x);
                update(&mut y);
                proof {
                    lemma_wf_assemble_node(&*y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        lemma_bst_decompose(&orig_right);
                        assert(Lnk::spec_bst_link(&y.right));
                        lemma_contains_root(&y);
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
                        assert(Lnk::spec_bst_link(&Some(y)));
                    }
                }
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    fn rotate_right<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>)
        requires
            Lnk::spec_link_size_wf(old(link)),
            Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) == Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(link, k) <==> spec_contains_link(old(link), k),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
    {
        if let Some(mut x) = link.take() {
            let ghost bst_input = Lnk::spec_bst_link(&Some(x));
            let ghost xk = x.key;
            let ghost orig_left = x.left;
            assert(Lnk::spec_link_size_wf(&x.left));
            assert(Lnk::spec_link_size_wf(&x.right));
            if let Some(mut y) = x.left.take() {
                let ghost yk = y.key;
                let ghost b  = y.right;
                let ghost a  = y.left;
                assert(Lnk::spec_link_size_wf(&y.left));
                assert(Lnk::spec_link_size_wf(&y.right));
                let ghost x_right_sz = Lnk::spec_size_link(&x.right);
                let ghost y_left_sz = Lnk::spec_size_link(&y.left);
                let ghost y_right_sz = Lnk::spec_size_link(&y.right);
                proof {
                    if bst_input {
                        lemma_bst_decompose(&orig_left);
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies k.is_lt(&xk) by {
                            lemma_contains_right(&y, k);
                        };
                        assert forall |k: T| #[trigger] spec_contains_link(&b, k) implies yk.is_lt(&k) by {};
                    }
                }
                x.left = y.right.take();
                assert(1 + y_left_sz + x_right_sz + 1 + y_right_sz <= usize::MAX as nat);
                update(&mut x);
                proof {
                    if bst_input {
                        assert(Lnk::spec_bst_link(&x.right));
                        assert(Lnk::spec_bst_link(&x.left));
                        assert(Lnk::spec_bst_link(&Some(x)));
                    }
                }
                y.right = Some(x);
                update(&mut y);
                proof {
                    lemma_wf_assemble_node(&*y);
                    reveal_with_fuel(spec_contains_link, 3);
                    if bst_input {
                        lemma_bst_decompose(&orig_left);
                        assert(Lnk::spec_bst_link(&y.left));
                        lemma_contains_root(&y);
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
                        assert(Lnk::spec_bst_link(&Some(y)));
                    }
                }
                *link = Some(y);
            } else {
                *link = Some(x);
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn insert_link<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>, value: T, priority: u64)
        requires
            Lnk::spec_size_link(old(link)) + 1 <= usize::MAX as nat,
            Lnk::spec_link_size_wf(old(link)),
            T::obeys_partial_cmp_spec(),
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) <= Lnk::spec_size_link(old(link)) + 1,
            Lnk::spec_size_link(link) >= Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(old(link), k) ==> spec_contains_link(link, k),
            forall|k: T| spec_contains_link(link, k) ==> (spec_contains_link(old(link), k) || k == value),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
        decreases old(link),
    {
        proof { reveal_with_fuel(spec_contains_link, 3); }
        if let Some(mut node) = link.take() {
            let ghost orig_key = node.key;
            let ghost orig_left = node.left;
            let ghost orig_right = node.right;
            proof {
                assert forall |k: T|
                    #[trigger] spec_contains_link(old(link), k) <==>
                    (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                    by {};
            }
            assert(Lnk::spec_link_size_wf(&node.left));
            assert(Lnk::spec_link_size_wf(&node.right));
            if value < node.key {
                insert_link(&mut node.left, value, priority);
                update(&mut node);
                proof {
                    lemma_wf_assemble_node(&*node);
                    assert forall |k: T| #[trigger] spec_contains_link(old(link), k)
                        implies spec_contains_link(&Some(node), k) by {
                        if spec_contains_link(&node.left, k) { lemma_contains_left(&node, k); }
                        if spec_contains_link(&node.right, k) { lemma_contains_right(&node, k); }
                    };
                    assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                        implies (spec_contains_link(old(link), k) || k == value) by {};
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                        assert(node.key == orig_key);
                        assert forall |k: T| #[trigger] spec_contains_link(&node.left, k)
                            implies k.is_lt(&node.key) by {
                            if spec_contains_link(&orig_left, k) {
                            } else {
                                assert(value.is_lt(&node.key));
                            }
                        };
                        assert(Lnk::spec_bst_link(&Some(node)));
                    }
                }
                *link = Some(node);
                let need_rotate_right = match link.as_ref().unwrap().left.as_ref() {
                    Some(left) => left.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate_right {
                    rotate_right(link);
                }
            } else if node.key < value {
                insert_link(&mut node.right, value, priority);
                update(&mut node);
                proof {
                    lemma_wf_assemble_node(&*node);
                    assert forall |k: T| #[trigger] spec_contains_link(old(link), k)
                        implies spec_contains_link(&Some(node), k) by {
                        if spec_contains_link(&node.left, k) { lemma_contains_left(&node, k); }
                        if spec_contains_link(&node.right, k) { lemma_contains_right(&node, k); }
                    };
                    assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                        implies (spec_contains_link(old(link), k) || k == value) by {};
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                        assert(node.key == orig_key);
                        assert forall |k: T| #[trigger] spec_contains_link(&node.right, k)
                            implies node.key.is_lt(&k) by {
                            if spec_contains_link(&orig_right, k) {
                            } else {
                                assert(node.key.is_lt(&value));
                            }
                        };
                        assert(Lnk::spec_bst_link(&Some(node)));
                    }
                }
                *link = Some(node);
                let need_rotate_left = match link.as_ref().unwrap().right.as_ref() {
                    Some(right) => right.priority < link.as_ref().unwrap().priority,
                    None => false,
                };
                if need_rotate_left {
                    rotate_left(link);
                }
            } else {
                *link = Some(node);
            }
        } else {
            let n = Box::new(Node { key: value, priority, size: 1, left: None, right: None });
            proof { lemma_wf_assemble_node(&*n); }
            *link = Some(n);
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn delete_link<T: StTInMtT + Ord + IsLtTransitive>(link: &mut Link<T>, target: &T)
        requires
            Lnk::spec_link_size_wf(old(link)),
            Lnk::spec_size_link(old(link)) < usize::MAX as nat,
            T::obeys_partial_cmp_spec(),
        ensures
            Lnk::spec_link_size_wf(link),
            Lnk::spec_size_link(link) <= Lnk::spec_size_link(old(link)),
            forall|k: T| spec_contains_link(link, k) ==> spec_contains_link(old(link), k),
            Lnk::spec_bst_link(old(link)) ==> Lnk::spec_bst_link(link),
        decreases Lnk::spec_size_link(old(link)),
    {
        proof { reveal_with_fuel(spec_contains_link, 3); }
        if let Some(mut node) = link.take() {
            let ghost orig_key = node.key;
            let ghost orig_left = node.left;
            let ghost orig_right = node.right;
            proof {
                assert forall |k: T|
                    #[trigger] spec_contains_link(old(link), k) <==>
                    (node.key == k || spec_contains_link(&node.left, k) || spec_contains_link(&node.right, k))
                    by {};
            }
            assert(Lnk::spec_link_size_wf(&node.left));
            assert(Lnk::spec_link_size_wf(&node.right));
            if *target < node.key {
                delete_link(&mut node.left, target);
                update(&mut node);
                proof {
                    lemma_wf_assemble_node(&*node);
                    assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                        implies spec_contains_link(old(link), k) by {
                        if spec_contains_link(&node.left, k) {
                            assert(spec_contains_link(&orig_left, k));
                        }
                    };
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                        assert forall |k: T| #[trigger] spec_contains_link(&node.left, k)
                            implies k.is_lt(&node.key) by {
                            assert(spec_contains_link(&orig_left, k));
                        };
                    }
                }
                *link = Some(node);
            } else if node.key < *target {
                delete_link(&mut node.right, target);
                update(&mut node);
                proof {
                    lemma_wf_assemble_node(&*node);
                    assert forall |k: T| #[trigger] spec_contains_link(&Some(node), k)
                        implies spec_contains_link(old(link), k) by {
                        if spec_contains_link(&node.right, k) {
                            assert(spec_contains_link(&orig_right, k));
                        }
                    };
                    if Lnk::spec_bst_link(old(link)) {
                        lemma_bst_decompose(old(link));
                        assert forall |k: T| #[trigger] spec_contains_link(&node.right, k)
                            implies node.key.is_lt(&k) by {
                            assert(spec_contains_link(&orig_right, k));
                        };
                    }
                }
                *link = Some(node);
            } else {
                // Found the target.
                if node.left.is_none() && node.right.is_none() {
                    // Leaf: remove (link stays None from take).
                } else if node.right.is_none() {
                    proof {
                        if Lnk::spec_bst_link(old(link)) {
                            lemma_bst_decompose(old(link));
                        }
                    }
                    *link = node.left.take();
                } else if node.left.is_none() {
                    proof {
                        if Lnk::spec_bst_link(old(link)) {
                            lemma_bst_decompose(old(link));
                        }
                    }
                    *link = node.right.take();
                } else {
                    // Two children: rotate smaller-priority child up, then recurse.
                    let left_pri = node.left.as_ref().unwrap().priority;
                    let right_pri = node.right.as_ref().unwrap().priority;
                    *link = Some(node);
                    if left_pri <= right_pri {
                        rotate_right(link);
                        if let Some(mut rotated) = link.take() {
                            let ghost rot_key = rotated.key;
                            let ghost rot_left = rotated.left;
                            let ghost rot_right = rotated.right;
                            proof {
                                assert forall |k: T|
                                    #[trigger] spec_contains_link(old(link), k) <==>
                                    (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                    by {};
                                if Lnk::spec_bst_link(old(link)) {
                                    lemma_bst_decompose(&Some(rotated));
                                    assert(Lnk::spec_bst_link(&rot_left));
                                    assert(Lnk::spec_bst_link(&rot_right));
                                    assert forall |k: T| #[trigger] spec_contains_link(&rot_left, k)
                                        implies k.is_lt(&rot_key) by {};
                                    assert forall |k: T| #[trigger] spec_contains_link(&rot_right, k)
                                        implies rot_key.is_lt(&k) by {};
                                }
                            }
                            delete_link(&mut rotated.right, target);
                            update(&mut rotated);
                            proof {
                                lemma_wf_assemble_node(&*rotated);
                                assert forall |k: T| #[trigger] spec_contains_link(&Some(rotated), k)
                                    implies spec_contains_link(old(link), k) by {
                                    if spec_contains_link(&rotated.right, k) {
                                        assert(spec_contains_link(&rot_right, k));
                                    }
                                    if spec_contains_link(&rotated.left, k) {
                                        assert(spec_contains_link(&rot_left, k));
                                    }
                                };
                                if Lnk::spec_bst_link(old(link)) {
                                    assert forall |k: T| #[trigger] spec_contains_link(&rotated.right, k)
                                        implies rotated.key.is_lt(&k) by {
                                        assert(spec_contains_link(&rot_right, k));
                                    };
                                    assert(Lnk::spec_bst_link(&Some(rotated)));
                                }
                            }
                            *link = Some(rotated);
                        }
                    } else {
                        rotate_left(link);
                        if let Some(mut rotated) = link.take() {
                            let ghost rot_key = rotated.key;
                            let ghost rot_left = rotated.left;
                            let ghost rot_right = rotated.right;
                            proof {
                                assert forall |k: T|
                                    #[trigger] spec_contains_link(old(link), k) <==>
                                    (rot_key == k || spec_contains_link(&rot_left, k) || spec_contains_link(&rot_right, k))
                                    by {};
                                if Lnk::spec_bst_link(old(link)) {
                                    lemma_bst_decompose(&Some(rotated));
                                    assert(Lnk::spec_bst_link(&rot_left));
                                    assert(Lnk::spec_bst_link(&rot_right));
                                    assert forall |k: T| #[trigger] spec_contains_link(&rot_left, k)
                                        implies k.is_lt(&rot_key) by {};
                                    assert forall |k: T| #[trigger] spec_contains_link(&rot_right, k)
                                        implies rot_key.is_lt(&k) by {};
                                }
                            }
                            delete_link(&mut rotated.left, target);
                            update(&mut rotated);
                            proof {
                                lemma_wf_assemble_node(&*rotated);
                                assert forall |k: T| #[trigger] spec_contains_link(&Some(rotated), k)
                                    implies spec_contains_link(old(link), k) by {
                                    if spec_contains_link(&rotated.left, k) {
                                        assert(spec_contains_link(&rot_left, k));
                                    }
                                    if spec_contains_link(&rotated.right, k) {
                                        assert(spec_contains_link(&rot_right, k));
                                    }
                                };
                                if Lnk::spec_bst_link(old(link)) {
                                    assert forall |k: T| #[trigger] spec_contains_link(&rotated.left, k)
                                        implies k.is_lt(&rotated.key) by {
                                        assert(spec_contains_link(&rot_left, k));
                                    };
                                    assert(Lnk::spec_bst_link(&Some(rotated)));
                                }
                            }
                            *link = Some(rotated);
                        }
                    }
                }
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn find_link<'a, T: StTInMtT + Ord>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
        requires Lnk::spec_bst_link(link),
        ensures found.is_some() ==> spec_contains_link(link, *found.unwrap()),
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                if (*target) == node.key {
                    proof { lemma_contains_root(node); }
                    Some(&node.key)
                } else if (*target) < node.key {
                    let r = find_link(&node.left, target);
                    proof {
                        if r.is_some() { lemma_contains_left(node, *r.unwrap()); }
                    }
                    r
                } else {
                    let r = find_link(&node.right, target);
                    proof {
                        if r.is_some() { lemma_contains_right(node, *r.unwrap()); }
                    }
                    r
                }
            }
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> (min_val: Option<&T>)
        requires Lnk::spec_bst_link(link),
        ensures min_val.is_some() ==> spec_contains_link(link, *min_val.unwrap()),
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => {
                    proof { lemma_contains_root(node); }
                    Some(&node.key)
                },
                | Some(_) => {
                    let r = min_link(&node.left);
                    proof {
                        if r.is_some() { lemma_contains_left(node, *r.unwrap()); }
                    }
                    r
                },
            },
        }
    }

    /// - APAS: Work O(log n) expected, Span O(log n) expected
    /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst case; Span Θ(log n) expected
    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> (max_val: Option<&T>)
        requires Lnk::spec_bst_link(link),
        ensures max_val.is_some() ==> spec_contains_link(link, *max_val.unwrap()),
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => {
                    proof { lemma_contains_root(node); }
                    Some(&node.key)
                },
                | Some(_) => {
                    let r = max_link(&node.right);
                    proof {
                        if r.is_some() { lemma_contains_right(node, *r.unwrap()); }
                    }
                    r
                },
            },
        }
    }

    fn height_link<T: StTInMtT + Ord>(link: &Link<T>) -> (h: usize)
        requires
            Lnk::spec_size_link(link) < usize::MAX as nat,
            Lnk::spec_link_size_wf(link),
        ensures h as nat == Lnk::spec_height_link(link),
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => {
                proof { lemma_size_wf_child_bounded(link); }
                let lh = height_link(&node.left);
                let rh = height_link(&node.right);
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

    /// - APAS: Work Θ(n), Span Θ(n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn in_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        requires Lnk::spec_link_size_wf(link),
        ensures out@.len() == old(out)@.len() + Lnk::spec_size_link(link) as int,
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    /// - APAS: Work Θ(n), Span Θ(n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        requires Lnk::spec_link_size_wf(link),
        ensures out@.len() == old(out)@.len() + Lnk::spec_size_link(link) as int,
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {
        open spec fn spec_bsttreapmteph_wf(&self) -> bool {
            self@.finite()
        }

        fn new() -> (empty_tree: Self)
            ensures empty_tree@ == Set::<<T as View>::V>::empty(), empty_tree.spec_bsttreapmteph_wf()
        {
            BSTTreapMtEph {
                locked_root: RwLock::new(None, Ghost(BSTTreapMtEphInv)),
                ghost_locked_root: Ghost(Set::<<T as View>::V>::empty()),
            }
        }

        fn insert(&mut self, value: T, priority: u64)
            ensures self@.contains(value@), self.spec_bsttreapmteph_wf()
        {
            proof { use_type_invariant(&*self); }
            let ghost value_view = value@;
            let ghost old_set = self.ghost_locked_root@;
            let (mut current, write_handle) = self.locked_root.acquire_write();
            let sz = size_link(&current);
            if sz + 1 < usize::MAX {
                insert_link(&mut current, value, priority);
            }
            write_handle.release_write(current);
            self.ghost_locked_root = Ghost(old_set.insert(value_view));
        }

        fn delete(&mut self, target: &T)
            ensures !self@.contains(target@), self.spec_bsttreapmteph_wf()
        {
            proof { use_type_invariant(&*self); }
            let ghost target_view = target@;
            let ghost old_set = self.ghost_locked_root@;
            let (mut current, write_handle) = self.locked_root.acquire_write();
            delete_link(&mut current, target);
            write_handle.release_write(current);
            self.ghost_locked_root = Ghost(old_set.remove(target_view));
        }

        fn find(&self, target: &T) -> (found: Option<T>)
            ensures found.is_some() <==> self@.contains(target@)
        {
            let handle = self.locked_root.acquire_read();
            let result = find_link(handle.borrow(), target).cloned();
            handle.release_read();
            proof { assume(result.is_some() <==> self@.contains(target@)); }
            result
        }

        fn contains(&self, target: &T) -> (found: bool)
            ensures found <==> self@.contains(target@)
        {
            self.find(target).is_some()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite()
        {
            proof { use_type_invariant(&*self); }
            let handle = self.locked_root.acquire_read();
            let result = size_link(handle.borrow());
            handle.release_read();
            proof { assume(result as nat == self@.len()); }
            result
        }

        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self@.len() == 0), self@.finite()
        {
            self.size() == 0
        }

        fn height(&self) -> (h: usize) {
            let handle = self.locked_root.acquire_read();
            let link: &Link<T> = handle.borrow();
            let result = height_link(link);
            handle.release_read();
            result
        }

        fn minimum(&self) -> (min_val: Option<T>)
            ensures min_val.is_some() ==> self@.contains(min_val.unwrap()@)
        {
            let handle = self.locked_root.acquire_read();
            let result = min_link(handle.borrow()).cloned();
            handle.release_read();
            proof { assume(result.is_some() ==> self@.contains(result.unwrap()@)); }
            result
        }

        fn maximum(&self) -> (max_val: Option<T>)
            ensures max_val.is_some() ==> self@.contains(max_val.unwrap()@)
        {
            let handle = self.locked_root.acquire_read();
            let result = max_link(handle.borrow()).cloned();
            handle.release_read();
            proof { assume(result.is_some() ==> self@.contains(result.unwrap()@)); }
            result
        }

        fn in_order(&self) -> (ordered: ArraySeqStPerS<T>)
            ensures ordered@.len() == self@.len()
        {
            let handle = self.locked_root.acquire_read();
            let mut out = Vec::with_capacity(size_link(handle.borrow()));
            in_order_collect(handle.borrow(), &mut out);
            handle.release_read();
            let ordered = ArraySeqStPerS::from_vec(out);
            proof { assume(ordered@.len() == self@.len()); }
            ordered
        }

        fn pre_order(&self) -> (preordered: ArraySeqStPerS<T>)
            ensures preordered@.len() == self@.len()
        {
            let handle = self.locked_root.acquire_read();
            let mut out = Vec::with_capacity(size_link(handle.borrow()));
            pre_order_collect(handle.borrow(), &mut out);
            handle.release_read();
            let preordered = ArraySeqStPerS::from_vec(out);
            proof { assume(preordered@.len() == self@.len()); }
            preordered
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> Default for BSTTreapMtEph<T> {
        fn default() -> (d: Self) { Self::new() }
    }


    //		11. derive impls in verus!

    impl<T: StTInMtT + Ord + Clone> Clone for Node<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = Node {
                key: self.key.clone(),
                priority: self.priority,
                size: self.size,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            };
            proof { accept(cloned == *self); }
            cloned
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> Clone for BSTTreapMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let handle = self.locked_root.acquire_read();
            let inner_clone = clone_link(handle.borrow());
            handle.release_read();
            proof {
                accept(spec_bsttreapmteph_link_wf(&inner_clone));
                accept(self.ghost_locked_root@.finite());
            }
            let cloned = BSTTreapMtEph {
                locked_root: RwLock::new(inner_clone, Ghost(BSTTreapMtEphInv)),
                ghost_locked_root: Ghost(self.ghost_locked_root@),
            };
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    } // verus!


    //		12. macros

    #[macro_export]
    macro_rules! BSTTreapMtEphLit {
        () => {
            < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new();
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

    impl<T: StTInMtT + Ord + IsLtTransitive> std::fmt::Debug for BSTTreapMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTTreapMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord + IsLtTransitive> std::fmt::Display for BSTTreapMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTTreapMtEph")
        }
    }

    impl<T: StTInMtT + Ord + std::fmt::Debug> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("priority", &self.priority)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T: StTInMtT + Ord + std::fmt::Display> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Node(key={}, priority={}, size={})", self.key, self.priority, self.size)
        }
    }

    impl std::fmt::Debug for Lnk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Lnk").finish()
        }
    }

    impl std::fmt::Display for Lnk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Lnk")
        }
    }

    impl std::fmt::Debug for BSTTreapMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTTreapMtEphInv").finish()
        }
    }

    impl std::fmt::Display for BSTTreapMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTTreapMtEphInv")
        }
    }
}
