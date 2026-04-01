//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! BST with general reduced values augmentation using associative functions.

//  Table of Contents
//  1. module
//  4. type definitions
//  5. view impls
//  7. proof fns
//  8. traits
//  9. impls
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!

// 1. module

pub mod BSTReducedStEph {

    use std::fmt;
    use std::marker::PhantomData;
    use std::ops::Add;

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    pub type Link<K, V, R> = Option<Box<Node<K, V, R>>>;

    pub struct Node<K: StT + Ord, V: StT, R: StT> {
        pub key: K,
        pub value: V,
        pub priority: u64,
        pub size: usize,
        pub reduced_value: R,
        pub left: Link<K, V, R>,
        pub right: Link<K, V, R>,
    }

    /// Example: Sum reduction for numeric values
    pub struct SumOp<T>(PhantomData<T>);

    /// Example: Count reduction (counts number of elements)
    pub struct CountOp<T>(PhantomData<T>);

    pub struct BSTReducedStEph<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> {
        pub root: Link<K, V, R>,
        pub _op: PhantomData<Op>,
    }

    pub type BSTreeReduced<K, V, R, Op> = BSTReducedStEph<K, V, R, Op>;

    // Type aliases for common reductions
    pub type BSTSumStEph<K, V> = BSTReducedStEph<K, V, V, SumOp<V>>;

    pub type BSTCountStEph<K, V> = BSTReducedStEph<K, V, usize, CountOp<V>>;

    pub struct Lnk;

    // 5. view impls

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> View for BSTReducedStEph<K, V, R, Op> {
        type V = Map<K, V>;
        open spec fn view(&self) -> Map<K, V> {
            Lnk::spec_content_link(&self.root)
        }
    }


    // 7. proof fns

    proof fn lemma_ordered_assemble_reduced<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>)
        requires
            match link {
                None => true,
                Some(node) => {
                    Lnk::spec_ordered_link(&node.left)
                    && Lnk::spec_ordered_link(&node.right)
                    && (forall |k: K| #[trigger] Lnk::spec_content_link(&node.left).contains_key(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Less)
                    && (forall |k: K| #[trigger] Lnk::spec_content_link(&node.right).contains_key(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Greater)
                }
            }
        ensures Lnk::spec_ordered_link(link),
    {}

    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    proof fn lemma_cmp_antisymmetry_reduced<K: StT + Ord>(a: K, b: K)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Greater,
        ensures
            b.cmp_spec(&a) == std::cmp::Ordering::Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec antisymmetry: Less(a,b) implies Greater(b,a).
    proof fn lemma_cmp_antisymmetry_lt_reduced<K: StT + Ord>(a: K, b: K)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Less,
        ensures
            b.cmp_spec(&a) == std::cmp::Ordering::Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    proof fn lemma_wf_assemble<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>)
        requires match link {
            None => true,
            Some(n) => {
                n.size as nat == 1 + Lnk::spec_size_link(&n.left) + Lnk::spec_size_link(&n.right)
                && Lnk::spec_link_size_wf(&n.left)
                && Lnk::spec_link_size_wf(&n.right)
            }
        },
        ensures Lnk::spec_link_size_wf(link),
    {}

    /// cmp_spec transitivity: Less(a,b) && Less(b,c) ==> Less(a,c).
    proof fn lemma_cmp_transitivity_lt_reduced<K: StT + Ord>(a: K, b: K, c: K)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Less,
            b.cmp_spec(&c) == std::cmp::Ordering::Less,
        ensures
            a.cmp_spec(&c) == std::cmp::Ordering::Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Greater(a,b) && Greater(b,c) ==> Greater(a,c).
    proof fn lemma_cmp_transitivity_gt_reduced<K: StT + Ord>(a: K, b: K, c: K)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            a.cmp_spec(&b) == std::cmp::Ordering::Greater,
            b.cmp_spec(&c) == std::cmp::Ordering::Greater,
        ensures
            a.cmp_spec(&c) == std::cmp::Ordering::Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Left-rotation content equality for Map with union_prefer_right.
    proof fn lemma_rotate_left_content_eq_reduced<K: StT + Ord, V: StT>(
        a: Map<K, V>, b: Map<K, V>, c: Map<K, V>,
        xk: K, xv: V, yk: K, yv: V,
    )
        requires xk != yk, !c.contains_key(xk),
        ensures
            a.union_prefer_right(
                b.union_prefer_right(c).insert(yk, yv)
            ).insert(xk, xv)
            =~=
            a.union_prefer_right(b).insert(xk, xv)
                .union_prefer_right(c).insert(yk, yv),
    {
        assert(a.union_prefer_right(b.union_prefer_right(c).insert(yk, yv))
            =~= a.union_prefer_right(b.union_prefer_right(c)).insert(yk, yv));
        assert(a.union_prefer_right(b.union_prefer_right(c))
            =~= a.union_prefer_right(b).union_prefer_right(c));
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(yk, yv).insert(xk, xv)
            =~= a.union_prefer_right(b).union_prefer_right(c).insert(xk, xv).insert(yk, yv));
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(xk, xv)
            =~= a.union_prefer_right(b).insert(xk, xv).union_prefer_right(c));
    }

    /// Right-rotation content equality for Map with union_prefer_right.
    proof fn lemma_rotate_right_content_eq_reduced<K: StT + Ord, V: StT>(
        a: Map<K, V>, b: Map<K, V>, c: Map<K, V>,
        xk: K, xv: V, yk: K, yv: V,
    )
        requires xk != yk, !c.contains_key(yk),
        ensures
            a.union_prefer_right(b).insert(yk, yv)
                .union_prefer_right(c).insert(xk, xv)
            =~=
            a.union_prefer_right(
                b.union_prefer_right(c).insert(xk, xv)
            ).insert(yk, yv),
    {
        assert(a.union_prefer_right(b.union_prefer_right(c).insert(xk, xv))
            =~= a.union_prefer_right(b.union_prefer_right(c)).insert(xk, xv));
        assert(a.union_prefer_right(b.union_prefer_right(c))
            =~= a.union_prefer_right(b).union_prefer_right(c));
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(xk, xv).insert(yk, yv)
            =~= a.union_prefer_right(b).union_prefer_right(c).insert(yk, yv).insert(xk, xv));
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(yk, yv)
            =~= a.union_prefer_right(b).insert(yk, yv).union_prefer_right(c));
    }


    // Root key of a link (arbitrary if None).
    pub open spec fn spec_root_key_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> K {
        match link {
            Some(node) => node.key,
            None => arbitrary(),
        }
    }

    // Whether a link's root has a left child.
    pub open spec fn spec_has_left_child_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> bool {
        match link {
            Some(node) => node.left.is_some(),
            None => false,
        }
    }

    // Whether a link's root has a right child.
    pub open spec fn spec_has_right_child_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> bool {
        match link {
            Some(node) => node.right.is_some(),
            None => false,
        }
    }

    // 8. traits

    pub trait LinkTrait<K: StT + Ord, V: StT, R: StT>: Sized {
        spec fn spec_size_link(link: &Link<K, V, R>) -> nat;
        spec fn spec_link_size_wf(link: &Link<K, V, R>) -> bool;
        spec fn spec_height_link(link: &Link<K, V, R>) -> nat;
        spec fn spec_content_link(link: &Link<K, V, R>) -> Map<K, V>;
        spec fn spec_ordered_link(link: &Link<K, V, R>) -> bool;
    }

    pub trait NodeTrait<K: StT + Ord, V: StT, R: StT>: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_bstreducedsteph_size_wf(&self) -> bool;

        spec fn spec_height(&self) -> nat;

        spec fn spec_content(&self) -> Map<K, V>;

        fn new(key: K, value: V, priority: u64, reduced_value: R) -> (node: Self);
    }

    /// Trait for associative reduction operations
    pub trait ReduceOp<V: StT, R: StT> {
        spec fn spec_identity() -> R;
        spec fn spec_combine(a: R, b: R) -> R;
        spec fn spec_lift(value: V) -> R;

        fn identity() -> (id_val: R)
            ensures id_val == Self::spec_identity();
        fn combine(a: R, b: R) -> (combined: R)
            ensures combined == Self::spec_combine(a, b);
        fn lift(value: &V) -> (lifted: R)
            ensures lifted == Self::spec_lift(*value);
    }

    pub trait BSTReducedStEphTrait<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>>: Sized + View<V = Map<K, V>> {
        spec fn spec_size(&self) -> nat;
        spec fn spec_bstreducedsteph_wf(&self) -> bool;
        spec fn spec_height(&self) -> nat;

        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn new() -> (empty: Self)
            ensures
                empty.spec_size() == 0,
                empty.spec_bstreducedsteph_wf(),
                empty@ == Map::<K, V>::empty();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn size(&self) -> (count: usize)
            ensures count as nat == self.spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == (self.spec_size() == 0);
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn height(&self) -> (height: usize)
            requires self.spec_height() < usize::MAX as nat,
            ensures height as nat == self.spec_height();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, O(n) worst
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst — matches APAS
        fn insert(&mut self, key: K, value: V, priority: u64)
            requires
                old(self).spec_size() + 1 <= usize::MAX as nat,
                old(self).spec_bstreducedsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                self@ == old(self)@.insert(key, value),
                self.spec_bstreducedsteph_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected — rotation-based
        fn delete(&mut self, key: &K)
            requires
                old(self).spec_bstreducedsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                self@ == old(self)@.remove(*key),
                self.spec_bstreducedsteph_wf(),
                self.spec_size() <= old(self).spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, O(n) worst
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst — matches APAS
        fn find(&self, key: &K) -> (found: Option<&V>)
            requires
                self.spec_bstreducedsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                found is Some <==> self@.contains_key(*key),
                found is Some ==> *found.unwrap() == self@[*key];
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, O(n) worst
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst — matches APAS
        fn contains(&self, key: &K) -> (contains: bool)
            requires
                self.spec_bstreducedsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures contains == self@.contains_key(*key);
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, O(n) worst
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst — matches APAS
        fn get(&self, key: &K) -> (value: Option<&V>)
            requires
                self.spec_bstreducedsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                value is Some <==> self@.contains_key(*key),
                value is Some ==> *value.unwrap() == self@[*key];
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn keys(&self) -> (keys: ArraySeqStPerS<K>)
            requires self.spec_bstreducedsteph_wf(),
            ensures keys.spec_len() == self.spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn values(&self) -> (values: ArraySeqStPerS<V>)
            requires self.spec_bstreducedsteph_wf(),
            ensures values.spec_len() == self.spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected — matches APAS
        fn minimum_key(&self) -> (minimum: Option<&K>)
            requires self.spec_bstreducedsteph_wf(),
            ensures
                self.spec_size() == 0 ==> minimum is None,
                self.spec_size() > 0 ==> minimum is Some;
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected — matches APAS
        fn maximum_key(&self) -> (maximum: Option<&K>)
            requires self.spec_bstreducedsteph_wf(),
            ensures
                self.spec_size() == 0 ==> maximum is None,
                self.spec_size() > 0 ==> maximum is Some;
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1) — reads augmented field at root
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn reduced_value(&self) -> (reduced: R)
            requires self.spec_bstreducedsteph_wf(),
            ensures self.spec_size() == 0 ==> reduced == Op::spec_identity();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n), Span O(log n) — range query on augmented BST
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn range_reduce(&self, low: &K, high: &K) -> (reduced: R)
            requires self.spec_bstreducedsteph_wf(),
            ensures self.spec_size() == 0 ==> reduced == Op::spec_identity();

        // Internal associated functions.

        fn size_link(link: &Link<K, V, R>) -> (count: usize)
            ensures count as nat == Lnk::spec_size_link(link);
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1) — reads augmented reduced value
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn reduced_value_link(link: &Link<K, V, R>) -> (reduced: R)
            ensures link.is_none() ==> reduced == Op::spec_identity();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1) — recomputes size and reduced value from children
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn update_node(node: &mut Node<K, V, R>)
            requires
                1 + Lnk::spec_size_link(&old(node).left) + Lnk::spec_size_link(&old(node).right) <= usize::MAX as nat,
                Lnk::spec_link_size_wf(&old(node).left),
                Lnk::spec_link_size_wf(&old(node).right),
            ensures
                node.size as nat == 1 + Lnk::spec_size_link(&node.left) + Lnk::spec_size_link(&node.right),
                Lnk::spec_link_size_wf(&node.left),
                Lnk::spec_link_size_wf(&node.right),
                node.key == old(node).key,
                node.value == old(node).value,
                node.left == old(node).left,
                node.right == old(node).right;
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1) — corresponds to APAS makeNode with reduced values
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn make_node(
            key: K, value: V, priority: u64,
            left: Link<K, V, R>, right: Link<K, V, R>,
        ) -> (reduced: Link<K, V, R>)
            requires
                1 + Lnk::spec_size_link(&left) + Lnk::spec_size_link(&right) <= usize::MAX as nat,
                Lnk::spec_link_size_wf(&left),
                Lnk::spec_link_size_wf(&right),
            ensures
                Lnk::spec_size_link(&reduced) == 1 + Lnk::spec_size_link(&left) + Lnk::spec_size_link(&right),
                Lnk::spec_link_size_wf(&reduced);
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn rotate_left(link: &mut Link<K, V, R>)
            requires
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_ordered_link(old(link)),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
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
        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn rotate_right(link: &mut Link<K, V, R>)
            requires
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_ordered_link(old(link)),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
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
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected — matches APAS
        fn insert_link(link: &mut Link<K, V, R>, key: K, value: V, priority: u64)
            requires
                Lnk::spec_size_link(old(link)) + 1 <= usize::MAX as nat,
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_ordered_link(old(link)),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                Lnk::spec_link_size_wf(link),
                Lnk::spec_size_link(link) <= Lnk::spec_size_link(old(link)) + 1,
                Lnk::spec_size_link(link) >= Lnk::spec_size_link(old(link)),
                Lnk::spec_content_link(link) == Lnk::spec_content_link(old(link)).insert(key, value),
                Lnk::spec_ordered_link(link),
            decreases old(link);
        fn delete_link(link: &mut Link<K, V, R>, key: &K) -> (deleted: bool)
            requires
                Lnk::spec_ordered_link(old(link)),
                Lnk::spec_link_size_wf(old(link)),
                Lnk::spec_size_link(old(link)) <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
            ensures
                Lnk::spec_content_link(link) == Lnk::spec_content_link(old(link)).remove(*key),
                Lnk::spec_ordered_link(link),
                Lnk::spec_link_size_wf(link),
                Lnk::spec_size_link(link) + if deleted { 1nat } else { 0nat } == Lnk::spec_size_link(old(link)),
            decreases Lnk::spec_size_link(old(link));
        fn find_link<'a>(link: &'a Link<K, V, R>, key: &K) -> (found: Option<&'a V>)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall |a: K, b: K| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b),
                Lnk::spec_ordered_link(link),
            ensures
                link.is_none() ==> found.is_none(),
                found.is_some() ==> Lnk::spec_content_link(link).contains_key(*key),
                found is Some ==> *found.unwrap() == Lnk::spec_content_link(link)[*key],
                Lnk::spec_content_link(link).contains_key(*key) ==> found is Some,
            decreases *link;
        fn min_key_link(link: &Link<K, V, R>) -> (minimum: Option<&K>)
            ensures
                link.is_none() ==> minimum.is_none(),
                link.is_some() ==> minimum.is_some(),
            decreases *link;
        fn max_key_link(link: &Link<K, V, R>) -> (maximum: Option<&K>)
            ensures
                link.is_none() ==> maximum.is_none(),
                link.is_some() ==> maximum.is_some(),
            decreases *link;
        fn collect_keys(link: &Link<K, V, R>, out: &mut Vec<K>)
            requires Lnk::spec_link_size_wf(link),
            ensures out.len() == old(out).len() + Lnk::spec_size_link(link),
            decreases *link;
        fn collect_values(link: &Link<K, V, R>, out: &mut Vec<V>)
            requires Lnk::spec_link_size_wf(link),
            ensures out.len() == old(out).len() + Lnk::spec_size_link(link),
            decreases *link;
        fn collect_in_order_kvp(link: &Link<K, V, R>, out: &mut Vec<(K, V, u64)>)
            requires Lnk::spec_link_size_wf(link),
            ensures out.len() == old(out).len() + Lnk::spec_size_link(link),
            decreases *link;
        fn height_link(link: &Link<K, V, R>) -> (height: usize)
            requires Lnk::spec_height_link(link) < usize::MAX as nat,
            ensures height == Lnk::spec_height_link(link),
            decreases *link;
        fn filter_by_key_kvp(
            items: &Vec<(K, V, u64)>, key: &K,
        ) -> (height: Vec<(K, V, u64)>)
            ensures height.len() <= items.len();
        fn find_min_priority_idx_kvp(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (height: usize)
            requires start < end, end <= items.len(),
            ensures start <= height && height < end;
        fn build_treap_from_vec(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (height: Link<K, V, R>)
            requires start <= end, end <= items.len(),
            ensures
                Lnk::spec_link_size_wf(&height),
                Lnk::spec_size_link(&height) == (end - start) as nat,
            decreases end - start;
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n), Span O(log n) — range query on augmented BST
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn range_reduce_link(link: &Link<K, V, R>, low: &K, high: &K) -> (reduced: R)
            ensures link.is_none() ==> reduced == Op::spec_identity(),
            decreases *link;
    }


    // 9. impls

    impl<K: StT + Ord, V: StT, R: StT> LinkTrait<K, V, R> for Lnk {
        open spec fn spec_size_link(link: &Link<K, V, R>) -> nat {
            match link {
                None => 0,
                Some(n) => n.size as nat,
            }
        }

        open spec fn spec_link_size_wf(link: &Link<K, V, R>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(n) => {
                    n.size as nat == 1 + Self::spec_size_link(&n.left) + Self::spec_size_link(&n.right)
                    && Self::spec_link_size_wf(&n.left)
                    && Self::spec_link_size_wf(&n.right)
                }
            }
        }

        open spec fn spec_height_link(link: &Link<K, V, R>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(n) => {
                    let l = Self::spec_height_link(&n.left);
                    let r = Self::spec_height_link(&n.right);
                    1 + if l >= r { l } else { r }
                }
            }
        }

        open spec fn spec_content_link(link: &Link<K, V, R>) -> Map<K, V>
            decreases *link,
        {
            match link {
                None => Map::empty(),
                Some(n) =>
                    Self::spec_content_link(&n.left)
                        .union_prefer_right(Self::spec_content_link(&n.right))
                        .insert(n.key, n.value),
            }
        }

        open spec fn spec_ordered_link(link: &Link<K, V, R>) -> bool
            decreases *link,
        {
            match link {
                None => true,
                Some(node) => {
                    Self::spec_ordered_link(&node.left)
                    && Self::spec_ordered_link(&node.right)
                    && (forall |k: K| #[trigger] Self::spec_content_link(&node.left).contains_key(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Less)
                    && (forall |k: K| #[trigger] Self::spec_content_link(&node.right).contains_key(k)
                        ==> k.cmp_spec(&node.key) == std::cmp::Ordering::Greater)
                }
            }
        }
    }

    fn clone_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> (cloned: Link<K, V, R>)
        requires Lnk::spec_ordered_link(link),
        ensures
            Lnk::spec_content_link(&cloned) == Lnk::spec_content_link(link),
            Lnk::spec_size_link(&cloned) == Lnk::spec_size_link(link),
            Lnk::spec_link_size_wf(link) ==> Lnk::spec_link_size_wf(&cloned),
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => {
                let k = node.key.clone();
                let v = node.value.clone();
                proof { assume(k == node.key && v == node.value); } // accept hole: Clone bridge
                Some(Box::new(Node {
                    key: k,
                    value: v,
                    priority: node.priority,
                    size: node.size,
                    reduced_value: node.reduced_value.clone(),
                    left: clone_link(&node.left),
                    right: clone_link(&node.right),
                }))
            }
        }
    }

    impl<K: StT + Ord, V: StT, R: StT> NodeTrait<K, V, R> for Node<K, V, R> {
        open spec fn spec_size(&self) -> nat {
            self.size as nat
        }

        open spec fn spec_bstreducedsteph_size_wf(&self) -> bool
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

        open spec fn spec_content(&self) -> Map<K, V>
            decreases *self,
        {
            let l = Lnk::spec_content_link(&self.left);
            let r = Lnk::spec_content_link(&self.right);
            l.union_prefer_right(r).insert(self.key, self.value)
        }

        fn new(key: K, value: V, priority: u64, reduced_value: R) -> (node: Self)
            ensures
                node.key == key,
                node.value == value,
                node.priority == priority,
                node.size == 1,
                node.reduced_value == reduced_value,
                node.left is None,
                node.right is None,
        {
            Node {
                key,
                value,
                priority,
                size: 1,
                reduced_value,
                left: None,
                right: None,
            }
        }
    }

    impl<T: StT + Add<Output = T> + Default + Copy> ReduceOp<T, T> for SumOp<T> {
        uninterp spec fn spec_identity() -> T;
        uninterp spec fn spec_combine(a: T, b: T) -> T;
        open spec fn spec_lift(value: T) -> T { value }

        #[verifier::external_body] // accept hole: T::default() not expressible in spec
        fn identity() -> (id_val: T) { T::default() }
        #[verifier::external_body] // accept hole
        fn combine(a: T, b: T) -> (combined: T) { a + b }
        fn lift(value: &T) -> (lifted: T) { *value }
    }

    impl<T: StT> ReduceOp<T, usize> for CountOp<T> {
        open spec fn spec_identity() -> usize { 0 }
        open spec fn spec_combine(a: usize, b: usize) -> usize { (a + b) as usize }
        open spec fn spec_lift(value: T) -> usize { 1 }

        fn identity() -> (id_val: usize) { 0 }
        #[verifier::external_body] // accept hole
        fn combine(a: usize, b: usize) -> (combined: usize) { a + b }
        fn lift(_value: &T) -> (lifted: usize) { 1 }
    }

    fn compare_reduced_links<K: StT + Ord, V: StT, R: StT>(a: &Link<K, V, R>, b: &Link<K, V, R>) -> (equal: bool)
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
                an.key == bn.key && an.value == bn.value
                    && compare_reduced_links(&an.left, &bn.left)
                    && compare_reduced_links(&an.right, &bn.right)
            }
            _ => false,
        }
    }

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> BSTReducedStEphTrait<K, V, R, Op>
        for BSTReducedStEph<K, V, R, Op>
    {
        open spec fn spec_size(&self) -> nat { Lnk::spec_size_link(&self.root) }
        open spec fn spec_bstreducedsteph_wf(&self) -> bool {
            Lnk::spec_link_size_wf(&self.root) && Lnk::spec_ordered_link(&self.root)
        }
        open spec fn spec_height(&self) -> nat { Lnk::spec_height_link(&self.root) }

        fn new() -> (empty: Self) {
            BSTReducedStEph {
                root: None,
                _op: PhantomData,
            }
        }

        fn size(&self) -> (count: usize) { Self::size_link(&self.root) }

        fn is_empty(&self) -> (is_empty: bool) { self.size() == 0 }

        fn height(&self) -> (height: usize) { Self::height_link(&self.root) }

        fn insert(&mut self, key: K, value: V, priority: u64) {
            Self::insert_link(&mut self.root, key, value, priority);
        }

        fn delete(&mut self, key: &K) {
            Self::delete_link(&mut self.root, key);
        }

        fn find(&self, key: &K) -> Option<&V> { Self::find_link(&self.root, key) }

        fn contains(&self, key: &K) -> bool { self.find(key).is_some() }

        fn get(&self, key: &K) -> Option<&V> { self.find(key) }

        fn keys(&self) -> ArraySeqStPerS<K> {
            let mut out = Vec::new();
            Self::collect_keys(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn values(&self) -> ArraySeqStPerS<V> {
            let mut out = Vec::new();
            Self::collect_values(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn minimum_key(&self) -> Option<&K> { Self::min_key_link(&self.root) }

        fn maximum_key(&self) -> Option<&K> { Self::max_key_link(&self.root) }

        fn reduced_value(&self) -> R { Self::reduced_value_link(&self.root) }

        fn range_reduce(&self, low: &K, high: &K) -> R {
            Self::range_reduce_link(&self.root, low, high)
        }

        // Internal associated functions.

        fn size_link(link: &Link<K, V, R>) -> (count: usize) {
            match link.as_ref() {
                None => 0,
                Some(n) => n.size,
            }
        }

        fn reduced_value_link(link: &Link<K, V, R>) -> R {
            match link.as_ref() {
                None => Op::identity(),
                Some(n) => n.reduced_value.clone(),
            }
        }

        fn update_node(node: &mut Node<K, V, R>) {
            node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right);
            let left_reduced = Self::reduced_value_link(&node.left);
            let right_reduced = Self::reduced_value_link(&node.right);
            let node_reduced = Op::lift(&node.value);
            node.reduced_value = Op::combine(left_reduced, Op::combine(node_reduced, right_reduced));
        }

        fn make_node(
            key: K, value: V, priority: u64,
            left: Link<K, V, R>, right: Link<K, V, R>,
        ) -> (count: Link<K, V, R>) {
            let ghost left_sz = Lnk::spec_size_link(&left);
            let ghost right_sz = Lnk::spec_size_link(&right);
            let node_reduced = Op::lift(&value);
            let mut node = Node::new(key, value, priority, node_reduced);
            node.left = left;
            node.right = right;
            Self::update_node(&mut node);
            let count = Some(Box::new(node));
            proof { lemma_wf_assemble(&count); }
            count
        }

        fn rotate_left(link: &mut Link<K, V, R>) {

            let ghost old_content = Lnk::spec_content_link(link);
            if let Some(mut x) = link.take() {
                assert(Lnk::spec_link_size_wf(&x.left));
                assert(Lnk::spec_link_size_wf(&x.right));
                let ghost a_content = Lnk::spec_content_link(&x.left);
                let ghost x_right_content = Lnk::spec_content_link(&x.right);
                let ghost xk = x.key;
                let ghost xv = x.value;
                proof {

                    assert(old_content =~=
                        a_content.union_prefer_right(x_right_content).insert(xk, xv));
                    // Capture ordering facts before mutations.
                    assert(forall |k: K| #[trigger] a_content.contains_key(k)
                        ==> k.cmp_spec(&xk) == std::cmp::Ordering::Less);
                    assert(forall |k: K| #[trigger] x_right_content.contains_key(k)
                        ==> k.cmp_spec(&xk) == std::cmp::Ordering::Greater);
                    assert(Lnk::spec_ordered_link(&x.left));
                    assert(Lnk::spec_ordered_link(&x.right));
                }
                if let Some(mut y) = x.right.take() {
                    assert(Lnk::spec_link_size_wf(&y.left));
                    assert(Lnk::spec_link_size_wf(&y.right));
                    let ghost b_content = Lnk::spec_content_link(&y.left);
                    let ghost c_content = Lnk::spec_content_link(&y.right);
                    let ghost yk = y.key;
                    let ghost yv = y.value;
                    let ghost x_left_sz = Lnk::spec_size_link(&x.left);
                    let ghost y_left_sz = Lnk::spec_size_link(&y.left);
                    let ghost y_right_sz = Lnk::spec_size_link(&y.right);
                    proof {
    
                        assert(x_right_content =~=
                            b_content.union_prefer_right(c_content).insert(yk, yv));
                        assert(x_right_content.contains_key(yk));
                        assert(yk.cmp_spec(&xk) == std::cmp::Ordering::Greater);
                        lemma_cmp_antisymmetry_reduced(yk, xk);
                        assert(xk.cmp_spec(&yk) == std::cmp::Ordering::Less);
                        // Capture y's ordering.
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Less);
                        assert(forall |k: K| #[trigger] c_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Greater);
                        assert(Lnk::spec_ordered_link(&y.left));
                        assert(Lnk::spec_ordered_link(&y.right));
                    }

                    x.right = y.left.take();

                    // B was in x.right content, so B > xk.
                    proof {
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> x_right_content.contains_key(k));
                        assert(forall |k: K| #[trigger] Lnk::spec_content_link(&x.right).contains_key(k)
                            ==> k.cmp_spec(&xk) == std::cmp::Ordering::Greater);
                    }

                    assert(Lnk::spec_link_size_wf(&x.left));
                    assert(Lnk::spec_link_size_wf(&x.right));
                    assert(1 + x_left_sz + y_left_sz + 1 + y_right_sz <= usize::MAX as nat);
                    Self::update_node(&mut x);

                    // Prove new x ordered before moving into y.left.
                    proof {
                        assert(Lnk::spec_ordered_link(&x.left));
                        assert(Lnk::spec_ordered_link(&x.right));
                        assert(forall |k: K| #[trigger] Lnk::spec_content_link(&x.left).contains_key(k)
                            ==> k.cmp_spec(&x.key) == std::cmp::Ordering::Less);
                        assert(forall |k: K| #[trigger] Lnk::spec_content_link(&x.right).contains_key(k)
                            ==> k.cmp_spec(&x.key) == std::cmp::Ordering::Greater);
                    }

                    y.left = Some(x);
                    Self::update_node(&mut y);

                    // Prove y.left (new x) is ordered.
                    proof {
                        lemma_ordered_assemble_reduced(&y.left);
                    }

                    // Ordering of new y: left=new_x, right=C, key=yk.
                    proof {
    
                        let ghost new_x_content = Lnk::spec_content_link(&y.left);
                        assert(new_x_content =~=
                            a_content.union_prefer_right(b_content).insert(xk, xv));
                        // A keys < yk by transitivity: A < xk < yk.
                        assert forall |k: K| #[trigger] a_content.contains_key(k)
                            implies k.cmp_spec(&yk) == std::cmp::Ordering::Less by {
                            if a_content.contains_key(k) {
                                lemma_cmp_transitivity_lt_reduced(k, xk, yk);
                            }
                        };
                        // B keys < yk (from original y ordering).
                        // xk < yk.
                        // Combine: all new_x_content keys < yk.
                        assert(forall |k: K| #[trigger] new_x_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Less);
                        // C keys > yk (from original y ordering).
                        assert(forall |k: K| #[trigger] c_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Greater);
                        assert(Lnk::spec_ordered_link(&y.left));
                        assert(Lnk::spec_ordered_link(&y.right));
                    }

                    *link = Some(y);
                    proof {



                        lemma_wf_assemble(link);
                        // Content equality through rotation.
                        if c_content.contains_key(xk) {
                            lemma_cmp_antisymmetry_reduced(xk, yk);
                        }
                        lemma_rotate_left_content_eq_reduced(
                            a_content, b_content, c_content, xk, xv, yk, yv);
                        assert(Lnk::spec_content_link(link) =~= old_content);
                        // Root key postcondition: new root is yk, old was xk, yk > xk.
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        assert(yk != xk);
                    }
                } else {
                    *link = Some(x);
                    proof { lemma_wf_assemble(link); }
                }
            }
        }

        fn rotate_right(link: &mut Link<K, V, R>) {

            let ghost old_content = Lnk::spec_content_link(link);
            if let Some(mut x) = link.take() {
                assert(Lnk::spec_link_size_wf(&x.left));
                assert(Lnk::spec_link_size_wf(&x.right));
                let ghost c_content = Lnk::spec_content_link(&x.right);
                let ghost x_left_content = Lnk::spec_content_link(&x.left);
                let ghost xk = x.key;
                let ghost xv = x.value;
                proof {

                    assert(old_content =~=
                        x_left_content.union_prefer_right(c_content).insert(xk, xv));
                    assert(forall |k: K| #[trigger] x_left_content.contains_key(k)
                        ==> k.cmp_spec(&xk) == std::cmp::Ordering::Less);
                    assert(forall |k: K| #[trigger] c_content.contains_key(k)
                        ==> k.cmp_spec(&xk) == std::cmp::Ordering::Greater);
                    assert(Lnk::spec_ordered_link(&x.left));
                    assert(Lnk::spec_ordered_link(&x.right));
                }
                if let Some(mut y) = x.left.take() {
                    assert(Lnk::spec_link_size_wf(&y.left));
                    assert(Lnk::spec_link_size_wf(&y.right));
                    let ghost a_content = Lnk::spec_content_link(&y.left);
                    let ghost b_content = Lnk::spec_content_link(&y.right);
                    let ghost yk = y.key;
                    let ghost yv = y.value;
                    let ghost x_right_sz = Lnk::spec_size_link(&x.right);
                    let ghost y_left_sz = Lnk::spec_size_link(&y.left);
                    let ghost y_right_sz = Lnk::spec_size_link(&y.right);
                    proof {
    
                        assert(x_left_content =~=
                            a_content.union_prefer_right(b_content).insert(yk, yv));
                        assert(x_left_content.contains_key(yk));
                        assert(yk.cmp_spec(&xk) == std::cmp::Ordering::Less);
                        lemma_cmp_antisymmetry_lt_reduced(yk, xk);
                        assert(xk.cmp_spec(&yk) == std::cmp::Ordering::Greater);
                        assert(forall |k: K| #[trigger] a_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Less);
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Greater);
                        assert(Lnk::spec_ordered_link(&y.left));
                        assert(Lnk::spec_ordered_link(&y.right));
                    }

                    x.left = y.right.take();

                    // B was in x.left content, so B < xk.
                    proof {
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> x_left_content.contains_key(k));
                        assert(forall |k: K| #[trigger] Lnk::spec_content_link(&x.left).contains_key(k)
                            ==> k.cmp_spec(&xk) == std::cmp::Ordering::Less);
                    }

                    assert(Lnk::spec_link_size_wf(&x.left));
                    assert(Lnk::spec_link_size_wf(&x.right));
                    assert(1 + y_right_sz + x_right_sz + 1 + y_left_sz <= usize::MAX as nat);
                    Self::update_node(&mut x);

                    // Prove new x ordered before moving into y.right.
                    proof {
                        assert(Lnk::spec_ordered_link(&x.left));
                        assert(Lnk::spec_ordered_link(&x.right));
                        assert(forall |k: K| #[trigger] Lnk::spec_content_link(&x.left).contains_key(k)
                            ==> k.cmp_spec(&x.key) == std::cmp::Ordering::Less);
                        assert(forall |k: K| #[trigger] Lnk::spec_content_link(&x.right).contains_key(k)
                            ==> k.cmp_spec(&x.key) == std::cmp::Ordering::Greater);
                    }

                    y.right = Some(x);
                    Self::update_node(&mut y);

                    // Prove y.right (new x) is ordered.
                    proof {
                        lemma_ordered_assemble_reduced(&y.right);
                    }

                    // Ordering of new y: left=A, right=new_x, key=yk.
                    proof {
    
                        let ghost new_x_content = Lnk::spec_content_link(&y.right);
                        assert(new_x_content =~=
                            b_content.union_prefer_right(c_content).insert(xk, xv));
                        // C keys > yk by transitivity: C > xk > yk.
                        assert forall |k: K| #[trigger] c_content.contains_key(k)
                            implies k.cmp_spec(&yk) == std::cmp::Ordering::Greater by {
                            if c_content.contains_key(k) {
                                lemma_cmp_transitivity_gt_reduced(k, xk, yk);
                            }
                        };
                        // B keys > yk (from original y ordering).
                        // xk > yk.
                        // Combine: all new_x_content keys > yk.
                        assert(forall |k: K| #[trigger] new_x_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Greater);
                        // A keys < yk (from original y ordering).
                        assert(forall |k: K| #[trigger] a_content.contains_key(k)
                            ==> k.cmp_spec(&yk) == std::cmp::Ordering::Less);
                        assert(Lnk::spec_ordered_link(&y.left));
                        assert(Lnk::spec_ordered_link(&y.right));
                    }

                    *link = Some(y);
                    proof {



                        lemma_wf_assemble(link);
                        // Content equality through rotation.
                        if c_content.contains_key(yk) {
                            lemma_cmp_antisymmetry_lt_reduced(yk, xk);
                        }
                        lemma_rotate_right_content_eq_reduced(
                            a_content, b_content, c_content, xk, xv, yk, yv);
                        assert(Lnk::spec_content_link(link) =~= old_content);
                        // Root key postcondition: new root is yk, old was xk, yk < xk.
                        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                        assert(yk != xk);
                    }
                } else {
                    *link = Some(x);
                    proof { lemma_wf_assemble(link); }
                }
            }
        }

        fn insert_link(link: &mut Link<K, V, R>, key: K, value: V, priority: u64)
            decreases old(link),
        {
            proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
            let ghost old_content = Lnk::spec_content_link(link);
            if let Some(mut node) = link.take() {
                assert(Lnk::spec_link_size_wf(&node.left));
                assert(Lnk::spec_link_size_wf(&node.right));
                let ghost old_left_content = Lnk::spec_content_link(&node.left);
                let ghost old_right_content = Lnk::spec_content_link(&node.right);
                let ghost node_key = node.key;
                let ghost node_value = node.value;
                proof {
                    assert(Lnk::spec_ordered_link(&node.left));
                    assert(Lnk::spec_ordered_link(&node.right));
                    assert(forall |k: K| #[trigger] old_left_content.contains_key(k)
                        ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Less);
                    assert(forall |k: K| #[trigger] old_right_content.contains_key(k)
                        ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                }
                match key.cmp(&node.key) {
                    std::cmp::Ordering::Less => {
                        assert(key.cmp_spec(&node_key) == std::cmp::Ordering::Less);
                        Self::insert_link(&mut node.left, key, value, priority);
                        proof {
                            assert(Lnk::spec_content_link(&node.left)
                                == old_left_content.insert(key, value));
                            assert(Lnk::spec_ordered_link(&node.left));
                            assert(forall |k: K| #[trigger]
                                old_left_content.insert(key, value).contains_key(k)
                                ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Less);
                            assert(Lnk::spec_ordered_link(&node.right));
                            assert(forall |k: K| #[trigger] old_right_content.contains_key(k)
                                ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                        }
                        Self::update_node(&mut *node);
                        *link = Some(node);
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble_reduced(link);
                            // Content: key not in right (key < node_key, right keys > node_key).
                            if old_right_content.contains_key(key) {
                                assert(key.cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                            }
                            assert(!old_right_content.contains_key(key));
                            assert(old_left_content.insert(key, value)
                                .union_prefer_right(old_right_content)
                                =~= old_left_content.union_prefer_right(old_right_content)
                                    .insert(key, value));
                            // key != node_key: cmp_spec == Less, not Equal.
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            assert(key != node_key);
                            assert(old_left_content.union_prefer_right(old_right_content)
                                .insert(key, value).insert(node_key, node_value)
                                =~= old_left_content.union_prefer_right(old_right_content)
                                    .insert(node_key, node_value).insert(key, value));
                            assert(Lnk::spec_content_link(link) =~= old_content.insert(key, value));
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
                        assert(key.cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                        Self::insert_link(&mut node.right, key, value, priority);
                        proof {
                            assert(Lnk::spec_content_link(&node.right)
                                == old_right_content.insert(key, value));
                            assert(Lnk::spec_ordered_link(&node.right));
                            assert(forall |k: K| #[trigger]
                                old_right_content.insert(key, value).contains_key(k)
                                ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                            assert(Lnk::spec_ordered_link(&node.left));
                            assert(forall |k: K| #[trigger] old_left_content.contains_key(k)
                                ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Less);
                        }
                        Self::update_node(&mut *node);
                        *link = Some(node);
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble_reduced(link);
                            // m1.upr(m2.insert(k,v)) =~= m1.upr(m2).insert(k,v) (always true).
                            assert(old_left_content.union_prefer_right(
                                old_right_content.insert(key, value))
                                =~= old_left_content.union_prefer_right(old_right_content)
                                    .insert(key, value));
                            // key != node_key: cmp_spec == Greater, not Equal.
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            assert(key != node_key);
                            assert(old_left_content.union_prefer_right(old_right_content)
                                .insert(key, value).insert(node_key, node_value)
                                =~= old_left_content.union_prefer_right(old_right_content)
                                    .insert(node_key, node_value).insert(key, value));
                            assert(Lnk::spec_content_link(link) =~= old_content.insert(key, value));
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
                        assert(key == node_key);
                        assert(old_content.contains_key(key));
                        node.value = value;
                        Self::update_node(&mut *node);
                        *link = Some(node);
                        proof {
                            lemma_wf_assemble(link);
                            lemma_ordered_assemble_reduced(link);
                            assert(old_content =~=
                                old_left_content.union_prefer_right(old_right_content)
                                    .insert(node_key, node_value));
                            assert(old_content.insert(key, value) =~=
                                old_left_content.union_prefer_right(old_right_content)
                                    .insert(key, value));
                            assert(Lnk::spec_content_link(link) =~= old_content.insert(key, value));
                        }
                    },
                }
            } else {
                let node_reduced = Op::lift(&value);
                *link = Some(Box::new(Node {
                    key,
                    value,
                    priority,
                    size: 1,
                    reduced_value: node_reduced,
                    left: None,
                    right: None,
                }));
                proof {
                    lemma_wf_assemble(link);
                    lemma_ordered_assemble_reduced(link);

                    assert(Lnk::spec_content_link(link) =~=
                        Map::<K,V>::empty().insert(key, value));
                    assert(old_content =~= Map::<K,V>::empty());
                    assert(Lnk::spec_content_link(link) =~= old_content.insert(key, value));
                }
            }
        }

        fn delete_link(link: &mut Link<K, V, R>, key: &K) -> (deleted: bool)
            decreases Lnk::spec_size_link(old(link)),
        {
            proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
            let ghost old_content = Lnk::spec_content_link(link);
            let ghost old_size = Lnk::spec_size_link(link);

            if let Some(mut node) = link.take() {
                let ghost old_left_content = Lnk::spec_content_link(&node.left);
                let ghost old_right_content = Lnk::spec_content_link(&node.right);
                let ghost node_key = node.key;
                let ghost node_value = node.value;
                let ghost old_left_size = Lnk::spec_size_link(&node.left);
                let ghost old_right_size = Lnk::spec_size_link(&node.right);
                proof {
                    assert(Lnk::spec_ordered_link(&node.left));
                    assert(Lnk::spec_ordered_link(&node.right));
                    assert(Lnk::spec_link_size_wf(&node.left));
                    assert(Lnk::spec_link_size_wf(&node.right));
                    assert(forall |k: K| #[trigger] old_left_content.contains_key(k)
                        ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Less);
                    assert(forall |k: K| #[trigger] old_right_content.contains_key(k)
                        ==> k.cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                    assert(old_content =~=
                        old_left_content.union_prefer_right(old_right_content).insert(node_key, node_value));
                    assert(old_size == 1 + old_left_size + old_right_size);
                }

                match key.cmp(&node.key) {
                    std::cmp::Ordering::Less => {
                        assert((*key).cmp_spec(&node_key) == std::cmp::Ordering::Less);
                        proof {
                            if old_right_content.contains_key(*key) {
                                assert((*key).cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                            }
                        }
                        let deleted = Self::delete_link(&mut node.left, key);
                        Self::update_node(&mut *node);
                        *link = Some(node);
                        proof {
                            lemma_wf_assemble(link);
                            assert forall |k: K| #[trigger] Lnk::spec_content_link(&node.left).contains_key(k)
                                implies k.cmp_spec(&node.key) == std::cmp::Ordering::Less by {
                                assert(old_left_content.contains_key(k));
                            };
                            lemma_ordered_assemble_reduced(link);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            assert(*key != node_key);
                            assert(Lnk::spec_content_link(link) =~= old_content.remove(*key));
                        }
                        deleted
                    }
                    std::cmp::Ordering::Greater => {
                        assert((*key).cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                        proof {
                            if old_left_content.contains_key(*key) {
                                assert((*key).cmp_spec(&node_key) == std::cmp::Ordering::Less);
                            }
                        }
                        let deleted = Self::delete_link(&mut node.right, key);
                        Self::update_node(&mut *node);
                        *link = Some(node);
                        proof {
                            lemma_wf_assemble(link);
                            assert forall |k: K| #[trigger] Lnk::spec_content_link(&node.right).contains_key(k)
                                implies k.cmp_spec(&node.key) == std::cmp::Ordering::Greater by {
                                assert(old_right_content.contains_key(k));
                            };
                            lemma_ordered_assemble_reduced(link);
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            assert(*key != node_key);
                            assert(Lnk::spec_content_link(link) =~= old_content.remove(*key));
                        }
                        deleted
                    }
                    std::cmp::Ordering::Equal => {
                        assert(*key == node_key);
                        if node.left.is_none() && node.right.is_none() {
                            proof {
                                assert(old_content.remove(*key) =~= Map::<K,V>::empty());
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
                                proof {
                                    // From rotate_right ensures: new root < old root.
                                    assert(new_root_key.cmp_spec(&node_key) == std::cmp::Ordering::Less);
                                    assert(new_root_key != node_key);
                                }
                                let mut rot = link.take().unwrap();
                                let ghost rot_left_content = Lnk::spec_content_link(&rot.left);
                                let ghost rot_right_content = Lnk::spec_content_link(&rot.right);
                                proof {
                                    assert(old_content =~=
                                        rot_left_content.union_prefer_right(rot_right_content).insert(rot.key, rot.value));
                                }
                                let deleted = Self::delete_link(&mut rot.right, key);
                                Self::update_node(&mut *rot);
                                *link = Some(rot);
                                proof {
                                    // rot.key == new_root_key != *key.
                                    assert(rot.key != *key);
                                    // rot.key < *key by antisymmetry of rot.key.cmp_spec(key) == Less.
                                    lemma_cmp_antisymmetry_lt_reduced(rot.key, *key);
                                    // If *key in left: key.cmp_spec(&rot.key) == Less, contradicts Greater.
                                    if rot_left_content.contains_key(*key) {
                                        assert((*key).cmp_spec(&rot.key) == std::cmp::Ordering::Less);
                                    }
                                    assert(!rot_left_content.contains_key(*key));
                                    assert(Lnk::spec_content_link(link) =~= old_content.remove(*key));
                                    assert forall |k: K| #[trigger] Lnk::spec_content_link(&rot.right).contains_key(k)
                                        implies k.cmp_spec(&rot.key) == std::cmp::Ordering::Greater by {
                                        assert(rot_right_content.contains_key(k));
                                    };
                                    lemma_wf_assemble(link);
                                    lemma_ordered_assemble_reduced(link);
                                }
                                deleted
                            } else {
                                Self::rotate_left(link);
                                let ghost new_root_key = spec_root_key_link(link);
                                proof {
                                    // From rotate_left ensures: new root > old root.
                                    assert(new_root_key.cmp_spec(&node_key) == std::cmp::Ordering::Greater);
                                    assert(new_root_key != node_key);
                                }
                                let mut rot = link.take().unwrap();
                                let ghost rot_left_content = Lnk::spec_content_link(&rot.left);
                                let ghost rot_right_content = Lnk::spec_content_link(&rot.right);
                                proof {
                                    assert(old_content =~=
                                        rot_left_content.union_prefer_right(rot_right_content).insert(rot.key, rot.value));
                                }
                                let deleted = Self::delete_link(&mut rot.left, key);
                                Self::update_node(&mut *rot);
                                *link = Some(rot);
                                proof {
                                    // rot.key == new_root_key != *key.
                                    assert(rot.key != *key);
                                    // rot.key > *key by antisymmetry of rot.key.cmp_spec(key) == Greater.
                                    lemma_cmp_antisymmetry_reduced(rot.key, *key);
                                    // If *key in right: key.cmp_spec(&rot.key) == Greater, contradicts Less.
                                    if rot_right_content.contains_key(*key) {
                                        assert((*key).cmp_spec(&rot.key) == std::cmp::Ordering::Greater);
                                    }
                                    assert(!rot_right_content.contains_key(*key));
                                    assert(Lnk::spec_content_link(link) =~= old_content.remove(*key));
                                    assert forall |k: K| #[trigger] Lnk::spec_content_link(&rot.left).contains_key(k)
                                        implies k.cmp_spec(&rot.key) == std::cmp::Ordering::Less by {
                                        assert(rot_left_content.contains_key(k));
                                    };
                                    lemma_wf_assemble(link);
                                    lemma_ordered_assemble_reduced(link);
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

        fn find_link<'a>(link: &'a Link<K, V, R>, key: &K) -> (found: Option<&'a V>)
            decreases *link,
        {
            proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
            match link {
                | None => None,
                | Some(node) => {
                    match key.cmp(&node.key) {
                        std::cmp::Ordering::Equal => {
                            assert(*key == node.key);
                            proof {
                                assert(Lnk::spec_content_link(link) =~=
                                    Lnk::spec_content_link(&node.left)
                                        .union_prefer_right(Lnk::spec_content_link(&node.right))
                                        .insert(node.key, node.value));
                            }
                            Some(&node.value)
                        }
                        std::cmp::Ordering::Less => {
                            let r = Self::find_link(&node.left, key);
                            proof {
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                assert(Lnk::spec_content_link(link) =~=
                                    Lnk::spec_content_link(&node.left)
                                        .union_prefer_right(Lnk::spec_content_link(&node.right))
                                        .insert(node.key, node.value));
                                if Lnk::spec_content_link(link).contains_key(*key) {
                                    assert(!Lnk::spec_content_link(&node.right).contains_key(*key));
                                    assert(Lnk::spec_content_link(&node.left).contains_key(*key));
                                }
                            }
                            r
                        }
                        std::cmp::Ordering::Greater => {
                            let r = Self::find_link(&node.right, key);
                            proof {
                                reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                assert(Lnk::spec_content_link(link) =~=
                                    Lnk::spec_content_link(&node.left)
                                        .union_prefer_right(Lnk::spec_content_link(&node.right))
                                        .insert(node.key, node.value));
                                if Lnk::spec_content_link(link).contains_key(*key) {
                                    assert(!Lnk::spec_content_link(&node.left).contains_key(*key));
                                    assert(Lnk::spec_content_link(&node.right).contains_key(*key));
                                }
                            }
                            r
                        }
                    }
                }
            }
        }

        fn min_key_link(link: &Link<K, V, R>) -> (minimum: Option<&K>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_key_link(&node.left),
                },
            }
        }

        fn max_key_link(link: &Link<K, V, R>) -> (maximum: Option<&K>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_key_link(&node.right),
                },
            }
        }

        fn collect_keys(link: &Link<K, V, R>, out: &mut Vec<K>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_keys(&node.left, out);
                out.push(node.key.clone());
                Self::collect_keys(&node.right, out);
            }
        }

        fn collect_values(link: &Link<K, V, R>, out: &mut Vec<V>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_values(&node.left, out);
                out.push(node.value.clone());
                Self::collect_values(&node.right, out);
            }
        }

        fn collect_in_order_kvp(link: &Link<K, V, R>, out: &mut Vec<(K, V, u64)>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_in_order_kvp(&node.left, out);
                out.push((node.key.clone(), node.value.clone(), node.priority));
                Self::collect_in_order_kvp(&node.right, out);
            }
        }

        fn height_link(link: &Link<K, V, R>) -> (height: usize)
            decreases *link,
        {
            match link {
                | None => 0,
                | Some(node) => {
                    let l = Self::height_link(&node.left);
                    let r = Self::height_link(&node.right);
                    1 + if l >= r { l } else { r }
                }
            }
        }

        fn filter_by_key_kvp(
            items: &Vec<(K, V, u64)>, key: &K,
        ) -> (height: Vec<(K, V, u64)>) {
            let mut filtered: Vec<(K, V, u64)> = Vec::new();
            let mut i: usize = 0;
            while i < items.len()
                invariant
                    i <= items.len(),
                    filtered.len() <= i,
                decreases items.len() - i,
            {
                if items[i].0 != *key {
                    filtered.push((items[i].0.clone(), items[i].1.clone(), items[i].2));
                }
                i = i + 1;
            }
            filtered
        }

        fn find_min_priority_idx_kvp(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (height: usize) {
            let mut min_idx = start;
            let mut i = start + 1;
            while i < end
                invariant
                    start <= min_idx, min_idx < end, min_idx < i,
                    i <= end, end <= items.len(),
                decreases end - i,
            {
                if items[i].2 < items[min_idx].2 {
                    min_idx = i;
                }
                i = i + 1;
            }
            min_idx
        }

        fn build_treap_from_vec(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (height: Link<K, V, R>)
            decreases end - start,
        {
            if start >= end {
                return None;
            }
            let min_idx = Self::find_min_priority_idx_kvp(items, start, end);
            let key = items[min_idx].0.clone();
            let value = items[min_idx].1.clone();
            let priority = items[min_idx].2;
            let left = Self::build_treap_from_vec(items, start, min_idx);
            let right = Self::build_treap_from_vec(items, min_idx + 1, end);
            Self::make_node(key, value, priority, left, right)
        }

        fn range_reduce_link(link: &Link<K, V, R>, low: &K, high: &K) -> R
            decreases *link,
        {
            match link {
                | None => Op::identity(),
                | Some(node) => {
                    let mut height = Op::identity();
                    if node.key > *low {
                        height = Op::combine(height, Self::range_reduce_link(&node.left, low, high));
                    }
                    if !(node.key < *low) && !(node.key > *high) {
                        height = Op::combine(height, Op::lift(&node.value));
                    }
                    if node.key < *high {
                        height = Op::combine(height, Self::range_reduce_link(&node.right, low, high));
                    }
                    height
                }
            }
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> Default for BSTreeReduced<K, V, R, Op> {
        fn default() -> (default_val: Self)
            ensures default_val.spec_size() == 0, default_val.spec_bstreducedsteph_wf(), default_val@ == Map::<K, V>::empty(),
        { Self::new() }
    }


    impl<K: StT + Ord, V: StT, R: StT> Clone for Node<K, V, R> {
        fn clone(&self) -> Self {
            proof { assume(Lnk::spec_ordered_link(&self.left)); assume(Lnk::spec_ordered_link(&self.right)); } // Clone body: ordering bridge
            Node {
                key: self.key.clone(),
                value: self.value.clone(),
                priority: self.priority,
                size: self.size,
                reduced_value: self.reduced_value.clone(),
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            }
        }
    }

    impl<T> Clone for SumOp<T> {
        fn clone(&self) -> Self { SumOp(PhantomData) }
    }

    impl<T> Clone for CountOp<T> {
        fn clone(&self) -> Self { CountOp(PhantomData) }
    }

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> Clone for BSTReducedStEph<K, V, R, Op> {
        fn clone(&self) -> Self {
            BSTReducedStEph {
                root: self.root.clone(),
                _op: PhantomData,
            }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> PartialEqSpecImpl for BSTReducedStEph<K, V, R, Op> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> Eq for BSTReducedStEph<K, V, R, Op> {}

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> PartialEq for BSTReducedStEph<K, V, R, Op> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof { assume(Lnk::spec_ordered_link(&self.root)); assume(Lnk::spec_ordered_link(&other.root)); } // PartialEq body: ordering bridge
            let equal = compare_reduced_links(&self.root, &other.root);
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    }

    // 13. derive impls outside verus!

    impl<T: fmt::Debug> fmt::Debug for SumOp<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("SumOp").field(&self.0).finish()
        }
    }

    impl<T: fmt::Debug> fmt::Debug for CountOp<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("CountOp").field(&self.0).finish()
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug, R: StT + fmt::Debug, Op: ReduceOp<V, R> + fmt::Debug> fmt::Debug
        for BSTReducedStEph<K, V, R, Op>
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTReducedStEph")
                .field("root", &self.root)
                .field("_op", &self._op)
                .finish()
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug, R: StT + fmt::Debug> fmt::Debug for Node<K, V, R> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("value", &self.value)
                .field("priority", &self.priority)
                .field("size", &self.size)
                .field("reduced_value", &self.reduced_value)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T> fmt::Display for SumOp<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "SumOp")
        }
    }

    impl<T> fmt::Display for CountOp<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "CountOp")
        }
    }

    impl<K: StT + Ord + fmt::Display, V: StT + fmt::Display, R: StT + fmt::Display> fmt::Display for Node<K, V, R> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}: {} r={})", self.key, self.value, self.reduced_value)
        }
    }

    impl<K: StT + Ord + fmt::Display, V: StT + fmt::Display, R: StT + fmt::Display, Op: ReduceOp<V, R>> fmt::Display
        for BSTReducedStEph<K, V, R, Op>
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.root {
                None => write!(f, "BSTReducedStEph(empty)"),
                Some(_) => write!(f, "BSTReducedStEph(non-empty)"),
            }
        }
    }

    // 12. macros

    #[macro_export]
    macro_rules! BSTReducedStEphLit {
        () => {
            < $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEph<_, _, _, _> as $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEphTrait<_, _, _, _> >::new()
        };
        ( $( ($k:expr, $v:expr) ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEph<_, _, _, _> as $crate::Chap40::BSTReducedStEph::BSTReducedStEph::BSTReducedStEphTrait<_, _, _, _> >::new();
            $( {
                let __key = $k;
                let mut __h = ::std::collections::hash_map::DefaultHasher::new();
                __key.hash(&mut __h);
                __tree.insert(__key, $v, __h.finish());
            } )*
            __tree
        }};
    }
}
