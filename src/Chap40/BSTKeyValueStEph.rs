//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Key-Value BST (dictionary/table) with ephemeral treap structure.

//  Table of Contents
//  1. module
//  3. broadcast use
//  4. type definitions
//  5. view impls
//  6. spec fns
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!

// 1. module

pub mod BSTKeyValueStEph {

    use std::fmt;

    use core::cmp::Ordering;

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus! {

    // 3. broadcast use
    broadcast use { vstd::map::group_map_axioms, vstd::map_lib::group_map_union, vstd::set::group_set_axioms };



    // 4. type definitions

    pub type Link<K, V> = Option<Box<Node<K, V>>>;

    pub struct Node<K: StT + Ord, V: StT> {
        pub key: K,
        pub value: V,
        pub priority: u64,
        pub left: Link<K, V>,
        pub right: Link<K, V>,
    }

    pub struct BSTKeyValueStEph<K: StT + Ord, V: StT> {
        pub root: Link<K, V>,
        pub size: usize,
    }

    pub type BSTreeKeyValue<K, V> = BSTKeyValueStEph<K, V>;

    pub struct Lnk;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for BSTKeyValueStEph<K, V> {
        type V = Map<K, V>;
        open spec fn view(&self) -> Map<K, V> {
            spec_content_link(&self.root)
        }
    }

    // 6. spec fns

    // Free spec fns: proofs require body unfolding, trait methods are uninterpreted.
    pub open spec fn spec_content_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> Map<K, V>
        decreases *link,
    {
        match link {
            None => Map::empty(),
            Some(node) =>
                spec_content_link(&node.left)
                    .union_prefer_right(spec_content_link(&node.right))
                    .insert(node.key, node.value),
        }
    }

    pub open spec fn spec_node_count_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => 1 + spec_node_count_link(&node.left) + spec_node_count_link(&node.right),
        }
    }

    pub open spec fn spec_ordered_link<K: StT + Ord + TotalOrder, V: StT>(link: &Link<K, V>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_ordered_link(&node.left)
                && spec_ordered_link(&node.right)
                && (forall |k: K| #[trigger] spec_content_link(&node.left).contains_key(k)
                    ==> (TotalOrder::le(k, node.key) && k != node.key))
                && (forall |k: K| #[trigger] spec_content_link(&node.right).contains_key(k)
                    ==> (TotalOrder::le(node.key, k) && k != node.key))
            }
        }
    }

    pub open spec fn spec_root_key_link<K: StT + Ord + TotalOrder, V: StT>(link: &Link<K, V>) -> K {
        match link {
            Some(node) => node.key,
            None => arbitrary(),
        }
    }

    pub open spec fn spec_has_left_child_link<K: StT + Ord + TotalOrder, V: StT>(link: &Link<K, V>) -> bool {
        match link {
            Some(node) => node.left.is_some(),
            None => false,
        }
    }

    pub open spec fn spec_has_right_child_link<K: StT + Ord + TotalOrder, V: StT>(link: &Link<K, V>) -> bool {
        match link {
            Some(node) => node.right.is_some(),
            None => false,
        }
    }

    // 7. proof fns

    /// Left-rotation content equality: chain of algebraic identities on Map with union_prefer_right.
    /// Requires xk != yk (distinct keys) and !c.contains_key(xk) (BST ordering disjointness).
    proof fn lemma_rotate_left_content_eq<K: StT + Ord, V: StT>(
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
        // Chain: LHS = a.upr((b.upr(c)).insert(yk,yv)).insert(xk,xv)
        // Step 1: m1.upr(m2.insert(k,v)) =~= m1.upr(m2).insert(k,v) [always true]
        assert(a.union_prefer_right(b.union_prefer_right(c).insert(yk, yv))
            =~= a.union_prefer_right(b.union_prefer_right(c)).insert(yk, yv));
        // Step 2: upr associativity
        assert(a.union_prefer_right(b.union_prefer_right(c))
            =~= a.union_prefer_right(b).union_prefer_right(c));
        // Step 3: insert commute when xk != yk
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(yk, yv).insert(xk, xv)
            =~= a.union_prefer_right(b).union_prefer_right(c).insert(xk, xv).insert(yk, yv));
        // Step 4: m.insert(k,v).upr(c) =~= m.upr(c).insert(k,v) when k not in c
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(xk, xv)
            =~= a.union_prefer_right(b).insert(xk, xv).union_prefer_right(c));
    }

    /// Right-rotation content equality: chain of algebraic identities on Map with union_prefer_right.
    /// Requires xk != yk (distinct keys) and !c.contains_key(yk) (BST ordering disjointness).
    proof fn lemma_rotate_right_content_eq<K: StT + Ord, V: StT>(
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
        // Chain: RHS = a.upr((b.upr(c)).insert(xk,xv)).insert(yk,yv)
        // Step 1: m1.upr(m2.insert(k,v)) =~= m1.upr(m2).insert(k,v) [always true]
        assert(a.union_prefer_right(b.union_prefer_right(c).insert(xk, xv))
            =~= a.union_prefer_right(b.union_prefer_right(c)).insert(xk, xv));
        // Step 2: upr associativity
        assert(a.union_prefer_right(b.union_prefer_right(c))
            =~= a.union_prefer_right(b).union_prefer_right(c));
        // Step 3: insert commute when xk != yk
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(xk, xv).insert(yk, yv)
            =~= a.union_prefer_right(b).union_prefer_right(c).insert(yk, yv).insert(xk, xv));
        // Step 4: m.insert(k,v).upr(c) =~= m.upr(c).insert(k,v) when k not in c
        assert(a.union_prefer_right(b).union_prefer_right(c).insert(yk, yv)
            =~= a.union_prefer_right(b).insert(yk, yv).union_prefer_right(c));
    }

    /// Insert on left commutes with union_prefer_right + insert on top when keys differ
    /// and the inserted key is not in the right map.
    proof fn lemma_insert_left_commutes<K: StT + Ord, V: StT>(
        left: Map<K, V>, right: Map<K, V>,
        nk: K, nv: V, ik: K, iv: V,
    )
        requires ik != nk, !right.contains_key(ik),
        ensures
            left.insert(ik, iv).union_prefer_right(right).insert(nk, nv)
            =~=
            left.union_prefer_right(right).insert(nk, nv).insert(ik, iv),
    {}

    /// Insert on right commutes with union_prefer_right + insert on top when keys differ
    /// and the inserted key is not in the left map.
    proof fn lemma_insert_right_commutes<K: StT + Ord, V: StT>(
        left: Map<K, V>, right: Map<K, V>,
        nk: K, nv: V, ik: K, iv: V,
    )
        requires ik != nk,
        ensures
            left.union_prefer_right(right.insert(ik, iv)).insert(nk, nv)
            =~=
            left.union_prefer_right(right).insert(nk, nv).insert(ik, iv),
    {}

    proof fn lemma_content_left_contains_key<K: StT + Ord, V: StT>(
        node: &Box<Node<K, V>>, k: K,
    )
        requires spec_content_link(&node.left).contains_key(k),
        ensures spec_content_link(&Some(*node)).contains_key(k),
    {
    }

    proof fn lemma_content_right_contains_key<K: StT + Ord, V: StT>(
        node: &Box<Node<K, V>>, k: K,
    )
        requires spec_content_link(&node.right).contains_key(k),
        ensures spec_content_link(&Some(*node)).contains_key(k),
    {
    }

    /// Left-rotation rearranges subtrees but preserves key membership.
    proof fn lemma_rotate_left_preserves_keys<K: StT + Ord, V: StT>(
        a: Map<K, V>, b: Map<K, V>, c: Map<K, V>,
        xk: K, xv: V, yk: K, yv: V,
    )
        ensures
            forall|k: K|
                #[trigger] a.union_prefer_right(
                    b.union_prefer_right(c).insert(yk, yv)
                ).insert(xk, xv).contains_key(k)
                ==>
                a.union_prefer_right(b).insert(xk, xv)
                    .union_prefer_right(c).insert(yk, yv).contains_key(k),
    {
    }

    /// Right-rotation rearranges subtrees but preserves key membership.
    proof fn lemma_rotate_right_preserves_keys<K: StT + Ord, V: StT>(
        a: Map<K, V>, b: Map<K, V>, c: Map<K, V>,
        xk: K, xv: V, yk: K, yv: V,
    )
        ensures
            forall|k: K|
                #[trigger] a.union_prefer_right(b).insert(xk, xv)
                    .union_prefer_right(c).insert(yk, yv).contains_key(k)
                ==>
                a.union_prefer_right(
                    b.union_prefer_right(c).insert(yk, yv)
                ).insert(xk, xv).contains_key(k),
    {
    }

    /// Lift left-child membership to the containing link.
    proof fn lemma_left_key_in_link<K: StT + Ord, V: StT>(
        link: &Link<K, V>, k: K,
    )
        requires
            link is Some,
            match *link {
                Some(node) => spec_content_link(&node.left).contains_key(k),
                None => false,
            },
        ensures spec_content_link(link).contains_key(k),
    {
    }

    /// Lift right-child membership to the containing link.
    proof fn lemma_right_key_in_link<K: StT + Ord, V: StT>(
        link: &Link<K, V>, k: K,
    )
        requires
            link is Some,
            match *link {
                Some(node) => spec_content_link(&node.right).contains_key(k),
                None => false,
            },
        ensures spec_content_link(link).contains_key(k),
    {
    }

    /// The root node's own key is always in the link's content.
    proof fn lemma_node_key_in_link<K: StT + Ord, V: StT>(
        link: &Link<K, V>,
    )
        requires link is Some,
        ensures
            match *link {
                Some(node) => spec_content_link(link).contains_key(node.key),
                None => true,
            },
    {
    }

    proof fn lemma_ordered_assemble_kv<K: StT + Ord + TotalOrder, V: StT>(link: &Link<K, V>)
        requires
            match link {
                None => true,
                Some(node) => {
                    spec_ordered_link(&node.left)
                    && spec_ordered_link(&node.right)
                    && (forall |k: K| #[trigger] spec_content_link(&node.left).contains_key(k)
                        ==> (TotalOrder::le(k, node.key) && k != node.key))
                    && (forall |k: K| #[trigger] spec_content_link(&node.right).contains_key(k)
                        ==> (TotalOrder::le(node.key, k) && k != node.key))
                }
            }
        ensures spec_ordered_link(link),
    {}


    /// Strict less-than transitivity: (le(a,b) && a!=b) && (le(b,c) && b!=c) ==> (le(a,c) && a!=c).
    proof fn lemma_strict_lt_transitive<K: StT + Ord + TotalOrder>(a: K, b: K, c: K)
        requires
            TotalOrder::le(a, b), a != b,
            TotalOrder::le(b, c), b != c,
        ensures
            TotalOrder::le(a, c), a != c,
    {
        K::transitive(a, b, c);
        if a == c {
            K::antisymmetric(a, b);
        }
    }

    /// Strict greater-than transitivity: (le(b,a) && a!=b) && (le(c,b) && b!=c) ==> (le(c,a) && a!=c).
    proof fn lemma_strict_gt_transitive<K: StT + Ord + TotalOrder>(a: K, b: K, c: K)
        requires
            TotalOrder::le(b, a), a != b,
            TotalOrder::le(c, b), b != c,
        ensures
            TotalOrder::le(c, a), a != c,
    {
        K::transitive(c, b, a);
        if a == c {
            K::antisymmetric(b, a);
        }
    }

    // 8. traits

    pub trait LinkTrait<K: StT + Ord, V: StT>: Sized {
        spec fn spec_height_link(link: &Link<K, V>) -> nat;
        spec fn spec_min_key_link(link: &Link<K, V>) -> Option<K>;
        spec fn spec_max_key_link(link: &Link<K, V>) -> Option<K>;
    }

    pub trait NodeTrait<K: StT + Ord, V: StT>: Sized {
        spec fn spec_height(&self) -> nat;
        spec fn spec_node_count(&self) -> nat;
        spec fn spec_content(&self) -> Map<K, V>;
        spec fn spec_min_key(&self) -> Option<K>;
        spec fn spec_max_key(&self) -> Option<K>;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new(key: K, value: V, priority: u64) -> (node: Self);
    }

    pub trait BSTKeyValueStEphTrait<K: StT + Ord + TotalOrder, V: StT>: Sized + View<V = Map<K, V>> {
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;
        spec fn spec_bstkeyvaluesteph_wf(&self) -> bool;
        spec fn spec_min_key(&self) -> Option<K>;
        spec fn spec_max_key(&self) -> Option<K>;

        /// - Alg Analysis: APAS (Ch40 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn new() -> (empty: Self)
            ensures
                empty.spec_size() == 0,
                empty@ == Map::<K, V>::empty(),
                empty.spec_bstkeyvaluesteph_wf();
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
                old(self).spec_size() < usize::MAX,
                old(self).spec_bstkeyvaluesteph_wf(),
            ensures
                self@ == old(self)@.insert(key, value),
                self@.contains_key(key),
                self.spec_size() >= old(self).spec_size(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_bstkeyvaluesteph_wf();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS; filter + rebuild
        fn delete(&mut self, key: &K)
            requires old(self).spec_bstkeyvaluesteph_wf(),
            ensures
                self@ == old(self)@.remove(*key),
                self.spec_size() <= old(self).spec_size(),
                self.spec_bstkeyvaluesteph_wf();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, O(n) worst
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst — matches APAS
        fn find(&self, key: &K) -> (found: Option<&V>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures
                found is Some <==> self@.contains_key(*key),
                found is Some ==> *found.unwrap() == self@[*key];
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, O(n) worst
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst — matches APAS
        fn contains(&self, key: &K) -> (contains: bool)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures contains == self@.contains_key(*key);
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, O(n) worst
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst — matches APAS
        fn get(&self, key: &K) -> (value: Option<&V>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures
                value is Some <==> self@.contains_key(*key),
                value is Some ==> *value.unwrap() == self@[*key];
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn keys(&self) -> (keys: ArraySeqStPerS<K>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures keys.spec_len() == self.spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn values(&self) -> (values: ArraySeqStPerS<V>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures values.spec_len() == self.spec_size();
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected — matches APAS
        fn minimum_key(&self) -> (minimum: Option<&K>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures
                self.spec_size() == 0 ==> minimum is None,
                self.spec_size() > 0 ==> minimum is Some,
                match (minimum, self.spec_min_key()) {
                    (Some(rv), Some(sv)) => *rv == sv,
                    (None, None) => true,
                    _ => false,
                };
        /// - Alg Analysis: APAS (Ch40 ref): Work O(log n) expected, Span O(log n) expected
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, Span O(log n) expected — matches APAS
        fn maximum_key(&self) -> (maximum: Option<&K>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures
                self.spec_size() == 0 ==> maximum is None,
                self.spec_size() > 0 ==> maximum is Some,
                match (maximum, self.spec_max_key()) {
                    (Some(rv), Some(sv)) => *rv == sv,
                    (None, None) => true,
                    _ => false,
                };

        // Internal associated functions.

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_link(link: &Link<K, V>) -> (height: usize)
            requires Lnk::spec_height_link(link) < usize::MAX as nat,
            ensures height == Lnk::spec_height_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(link: &mut Link<K, V>)
            requires spec_ordered_link(old(link)),
            ensures
                link.is_some() == old(link).is_some(),
                spec_content_link(link) == spec_content_link(old(link)),
                spec_ordered_link(link),
                spec_node_count_link(link) == spec_node_count_link(old(link)),
                // After non-trivial rotation, root key changes and comes from right subtree.
                spec_has_right_child_link(old(link)) ==> (
                    spec_root_key_link(link) != spec_root_key_link(old(link))
                    && TotalOrder::le(spec_root_key_link(old(link)), spec_root_key_link(link))
                );
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(link: &mut Link<K, V>)
            requires spec_ordered_link(old(link)),
            ensures
                link.is_some() == old(link).is_some(),
                spec_content_link(link) == spec_content_link(old(link)),
                spec_ordered_link(link),
                spec_node_count_link(link) == spec_node_count_link(old(link)),
                // After non-trivial rotation, root key changes and comes from left subtree.
                spec_has_left_child_link(old(link)) ==> (
                    spec_root_key_link(link) != spec_root_key_link(old(link))
                    && TotalOrder::le(spec_root_key_link(link), spec_root_key_link(old(link)))
                );
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn insert_link(link: &mut Link<K, V>, key: K, value: V, priority: u64) -> (inserted: bool)
            requires spec_ordered_link(old(link)),
            ensures
                link.is_some(),
                spec_content_link(link) == spec_content_link(old(link)).insert(key, value),
                spec_ordered_link(link),
                spec_node_count_link(link) == spec_node_count_link(old(link)) + if inserted { 1nat } else { 0nat },
            decreases old(link);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn delete_link(link: &mut Link<K, V>, key: &K) -> (deleted: bool)
            requires spec_ordered_link(old(link)),
            ensures
                spec_content_link(link) == spec_content_link(old(link)).remove(*key),
                spec_ordered_link(link),
                spec_node_count_link(link) + if deleted { 1nat } else { 0nat } == spec_node_count_link(old(link)),
            decreases spec_node_count_link(old(link));
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn find_link<'a>(link: &'a Link<K, V>, key: &K) -> (found: Option<&'a V>)
            requires
                spec_ordered_link(link),
            ensures
                link.is_none() ==> found.is_none(),
                found.is_some() ==> spec_content_link(link).contains_key(*key),
                found is Some ==> *found.unwrap() == spec_content_link(link)[*key],
                spec_content_link(link).contains_key(*key) ==> found is Some,
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn min_key_link(link: &Link<K, V>) -> (minimum: Option<&K>)
            ensures
                link.is_none() ==> minimum.is_none(),
                link.is_some() ==> minimum.is_some(),
                match (minimum, Lnk::spec_min_key_link(link)) {
                    (Some(rv), Some(sv)) => *rv == sv,
                    (None, None) => true,
                    _ => false,
                },
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn max_key_link(link: &Link<K, V>) -> (maximum: Option<&K>)
            ensures
                link.is_none() ==> maximum.is_none(),
                link.is_some() ==> maximum.is_some(),
                match (maximum, Lnk::spec_max_key_link(link)) {
                    (Some(rv), Some(sv)) => *rv == sv,
                    (None, None) => true,
                    _ => false,
                },
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect_keys(link: &Link<K, V>, out: &mut Vec<K>)
            ensures out.len() == old(out).len() + spec_node_count_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect_values(link: &Link<K, V>, out: &mut Vec<V>)
            ensures out.len() == old(out).len() + spec_node_count_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect_in_order_kvp(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>)
            ensures out.len() == old(out).len() + spec_node_count_link(link),
            decreases *link;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn find_min_priority_idx_kvp(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (maximum: usize)
            requires start < end, end <= items.len(),
            ensures start <= maximum && maximum < end;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, O(n^2) worst
        fn build_treap_from_vec(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (maximum: Link<K, V>)
            requires start <= end, end <= items.len(),
            ensures maximum.is_none() == (start == end),
            decreases end - start;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_by_key_kvp(
            items: &Vec<(K, V, u64)>, key: &K,
        ) -> (maximum: Vec<(K, V, u64)>)
            ensures maximum.len() <= items.len();
    }



        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn clone_link<K: StT + Ord + TotalOrder, V: StT>(link: &Link<K, V>) -> (cloned: Link<K, V>)
        requires spec_ordered_link(link),
        ensures
            spec_content_link(&cloned) == spec_content_link(link),
            spec_node_count_link(&cloned) == spec_node_count_link(link),
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => {
                proof { reveal_with_fuel(spec_ordered_link, 2); }
                let k = node.key.clone();
                let v = node.value.clone();
                proof { assume(k == node.key && v == node.value); } // accept hole: Clone bridge
                Some(Box::new(Node {
                    key: k,
                    value: v,
                    priority: node.priority,
                    left: clone_link(&node.left),
                    right: clone_link(&node.right),
                }))
            }
        }
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> LinkTrait<K, V> for Lnk {
        open spec fn spec_height_link(link: &Link<K, V>) -> nat
            decreases *link,
        {
            match link {
                None => 0,
                Some(node) => {
                    let l = Self::spec_height_link(&node.left);
                    let r = Self::spec_height_link(&node.right);
                    1 + if l >= r { l } else { r }
                }
            }
        }


        open spec fn spec_min_key_link(link: &Link<K, V>) -> Option<K>
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => match node.left {
                    None => Some(node.key),
                    Some(_) => Self::spec_min_key_link(&node.left),
                },
            }
        }

        open spec fn spec_max_key_link(link: &Link<K, V>) -> Option<K>
            decreases *link,
        {
            match link {
                None => None,
                Some(node) => match node.right {
                    None => Some(node.key),
                    Some(_) => Self::spec_max_key_link(&node.right),
                },
            }
        }
    }

    impl<K: StT + Ord, V: StT> NodeTrait<K, V> for Node<K, V> {
        open spec fn spec_height(&self) -> nat
            decreases *self,
        {
            let l = match self.left { None => 0nat, Some(n) => NodeTrait::spec_height(&*n) };
            let r = match self.right { None => 0nat, Some(n) => NodeTrait::spec_height(&*n) };
            1 + if l >= r { l } else { r }
        }

        open spec fn spec_node_count(&self) -> nat
            decreases *self,
        {
            let l = match self.left { None => 0nat, Some(n) => NodeTrait::spec_node_count(&*n) };
            let r = match self.right { None => 0nat, Some(n) => NodeTrait::spec_node_count(&*n) };
            1 + l + r
        }

        open spec fn spec_content(&self) -> Map<K, V>
            decreases *self,
        {
            let l = match self.left { None => Map::empty(), Some(n) => NodeTrait::spec_content(&*n) };
            let r = match self.right { None => Map::empty(), Some(n) => NodeTrait::spec_content(&*n) };
            l.union_prefer_right(r).insert(self.key, self.value)
        }

        open spec fn spec_min_key(&self) -> Option<K>
            decreases *self,
        {
            match self.left {
                None => Some(self.key),
                Some(n) => NodeTrait::spec_min_key(&*n),
            }
        }

        open spec fn spec_max_key(&self) -> Option<K>
            decreases *self,
        {
            match self.right {
                None => Some(self.key),
                Some(n) => NodeTrait::spec_max_key(&*n),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new(key: K, value: V, priority: u64) -> (node: Self)
            ensures
                node.key == key,
                node.value == value,
                node.priority == priority,
                node.left is None,
                node.right is None,
        {
            Node { key, value, priority, left: None, right: None }
        }
    }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn compare_kv_links<K: StT + Ord + TotalOrder, V: StT>(a: &Link<K, V>, b: &Link<K, V>) -> (equal: bool)
        requires spec_ordered_link(a), spec_ordered_link(b),
        ensures
            (a is None && b is None) ==> equal,
            (a is Some && b is None) ==> !equal,
            (a is None && b is Some) ==> !equal,
        decreases *a,
    {
        match (a, b) {
            (None, None) => true,
            (Some(an), Some(bn)) => {
                proof { reveal_with_fuel(spec_ordered_link, 2); }
                an.key == bn.key && an.value == bn.value
                    && compare_kv_links(&an.left, &bn.left)
                    && compare_kv_links(&an.right, &bn.right)
            }
            _ => false,
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT> BSTKeyValueStEphTrait<K, V> for BSTKeyValueStEph<K, V> {
        open spec fn spec_size(&self) -> nat { self.size as nat }
        open spec fn spec_height(&self) -> nat { Lnk::spec_height_link(&self.root) }
        open spec fn spec_bstkeyvaluesteph_wf(&self) -> bool {
            self.size as nat == spec_node_count_link(&self.root)
            && spec_ordered_link(&self.root)
        }
        open spec fn spec_min_key(&self) -> Option<K> { Lnk::spec_min_key_link(&self.root) }
        open spec fn spec_max_key(&self) -> Option<K> { Lnk::spec_max_key_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self) { BSTKeyValueStEph { root: None, size: 0 } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize) { self.size }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool) { self.size == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (height: usize) { Self::height_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn insert(&mut self, key: K, value: V, priority: u64) {
            let ghost old_count = spec_node_count_link(&self.root);
            let inserted = Self::insert_link(&mut self.root, key, value, priority);
            if inserted {
                self.size = self.size + 1;
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn delete(&mut self, key: &K) {
            let deleted = Self::delete_link(&mut self.root, key);
            if deleted {
                proof { assert(self.size as nat >= 1nat); }
                self.size = self.size - 1;
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn find(&self, key: &K) -> Option<&V> { Self::find_link(&self.root, key) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn contains(&self, key: &K) -> bool { self.find(key).is_some() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn get(&self, key: &K) -> Option<&V> { self.find(key) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn keys(&self) -> ArraySeqStPerS<K> {
            let mut out = Vec::new();
            Self::collect_keys(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn values(&self) -> ArraySeqStPerS<V> {
            let mut out = Vec::new();
            Self::collect_values(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn minimum_key(&self) -> Option<&K> { Self::min_key_link(&self.root) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn maximum_key(&self) -> Option<&K> { Self::max_key_link(&self.root) }

        // Internal associated functions.

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_link(link: &Link<K, V>) -> (height: usize)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(link: &mut Link<K, V>) {
            proof { reveal_with_fuel(spec_ordered_link, 2); }
            let ghost old_content = spec_content_link(link);
            let ghost old_count = spec_node_count_link(link);
            if let Some(mut x) = link.take() {
                let ghost a_content = spec_content_link(&x.left);
                let ghost xk = x.key;
                let ghost xv = x.value;
                let ghost x_right_content = spec_content_link(&x.right);

                if let Some(mut y) = x.right.take() {
                    let ghost b_content = spec_content_link(&y.left);
                    let ghost c_content = spec_content_link(&y.right);
                    let ghost yk = y.key;
                    let ghost yv = y.value;
                    proof {
                        reveal_with_fuel(spec_content_link, 2);
                        assert(x_right_content =~=
                            b_content.union_prefer_right(c_content).insert(yk, yv));
                        assert(x_right_content.contains_key(yk));
                        assert(TotalOrder::le(xk, yk));
                        assert(xk != yk);
                        // Capture all ordering facts before mutations.
                        assert(forall |k: K| #[trigger] a_content.contains_key(k)
                            ==> (TotalOrder::le(k, xk) && k != xk));
                        assert(forall |k: K| #[trigger] x_right_content.contains_key(k)
                            ==> (TotalOrder::le(xk, k) && k != xk));
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> (TotalOrder::le(k, yk) && k != yk));
                        assert(forall |k: K| #[trigger] c_content.contains_key(k)
                            ==> (TotalOrder::le(yk, k) && k != yk));
                        assert(spec_ordered_link(&x.left));
                        assert(spec_ordered_link(&y.left));
                        assert(spec_ordered_link(&y.right));
                    }

                    x.right = y.left.take();

                    // Prove new x (left=A, right=B, key=xk) is ordered.
                    proof {
                        // B was in x.right content, so B > xk.
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> x_right_content.contains_key(k));
                        assert(forall |k: K| #[trigger] spec_content_link(&x.right).contains_key(k)
                            ==> (TotalOrder::le(xk, k) && k != xk));
                    }

                    y.left = Some(x);

                    // Prove new x (y.left) is ordered.
                    proof {
                        lemma_ordered_assemble_kv(&y.left);
                    }

                    // Ordering of new y: left=new_x, right=C, key=yk.
                    proof {
                        reveal_with_fuel(spec_content_link, 2);
                        let ghost new_x_content = spec_content_link(&y.left);
                        assert(new_x_content =~=
                            a_content.union_prefer_right(b_content).insert(xk, xv));
                        // A keys < yk by transitivity: A < xk < yk.
                        assert forall |k: K| #[trigger] a_content.contains_key(k)
                            implies (TotalOrder::le(k, yk) && k != yk) by {
                            if a_content.contains_key(k) {
                                // Inline transitivity: le(k,xk) && le(xk,yk) => le(k,yk).
                                K::transitive(k, xk, yk);
                                // k != yk: if k == yk, then le(yk,xk) from le(k,xk),
                                // combined with le(xk,yk) and antisymmetric => xk == yk, contradiction.
                                if k == yk {
                                    K::antisymmetric(k, xk);
                                }
                            }
                        };
                        // B keys < yk (from original y ordering, captured above).
                        // xk < yk (established above).
                        // Combine: all new_x_content keys < yk.
                        assert(forall |k: K| #[trigger] new_x_content.contains_key(k)
                            ==> (TotalOrder::le(k, yk) && k != yk));
                        // C keys > yk (from original y ordering, captured above).
                        assert(forall |k: K| #[trigger] c_content.contains_key(k)
                            ==> (TotalOrder::le(yk, k) && k != yk));
                    }

                    // Re-assert y's ordering components before move.
                    proof {
                        assert(spec_ordered_link(&y.left));
                        assert(spec_ordered_link(&y.right));
                        assert(forall |k: K| #[trigger] spec_content_link(&y.left).contains_key(k)
                            ==> (TotalOrder::le(k, yk) && k != yk));
                        assert(forall |k: K| #[trigger] spec_content_link(&y.right).contains_key(k)
                            ==> (TotalOrder::le(yk, k) && k != yk));
                    }

                    *link = Some(y);
                    proof {
                        reveal_with_fuel(spec_ordered_link, 2);
                        reveal_with_fuel(spec_content_link, 3);
                        reveal_with_fuel(spec_node_count_link, 3);
                        // Prove xk not in C (BST ordering: all C keys > yk > xk).
                        if c_content.contains_key(xk) {
                            K::antisymmetric(xk, yk);
                        }
                        lemma_rotate_left_content_eq(a_content, b_content, c_content, xk, xv, yk, yv);
                        assert(spec_content_link(link) =~= old_content);
                        assert(spec_node_count_link(link) == old_count);
                    }
                } else {
                    *link = Some(x);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(link: &mut Link<K, V>) {
            proof { reveal_with_fuel(spec_ordered_link, 2); }
            let ghost old_content = spec_content_link(link);
            let ghost old_count = spec_node_count_link(link);
            if let Some(mut x) = link.take() {
                let ghost c_content = spec_content_link(&x.right);
                let ghost xk = x.key;
                let ghost xv = x.value;
                let ghost x_left_content = spec_content_link(&x.left);

                if let Some(mut y) = x.left.take() {
                    let ghost a_content = spec_content_link(&y.left);
                    let ghost b_content = spec_content_link(&y.right);
                    let ghost yk = y.key;
                    let ghost yv = y.value;
                    proof {
                        reveal_with_fuel(spec_content_link, 2);
                        assert(x_left_content =~=
                            a_content.union_prefer_right(b_content).insert(yk, yv));
                        assert(x_left_content.contains_key(yk));
                        assert(TotalOrder::le(yk, xk));
                        assert(yk != xk);
                        // Capture all ordering facts before mutations.
                        assert(forall |k: K| #[trigger] a_content.contains_key(k)
                            ==> (TotalOrder::le(k, yk) && k != yk));
                        assert(forall |k: K| #[trigger] x_left_content.contains_key(k)
                            ==> (TotalOrder::le(k, xk) && k != xk));
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> (TotalOrder::le(yk, k) && k != yk));
                        assert(forall |k: K| #[trigger] c_content.contains_key(k)
                            ==> (TotalOrder::le(xk, k) && k != xk));
                        assert(spec_ordered_link(&x.right));
                        assert(spec_ordered_link(&y.left));
                        assert(spec_ordered_link(&y.right));
                    }

                    x.left = y.right.take();

                    // Prove new x (left=B, right=C, key=xk) is ordered.
                    proof {
                        // B was in x.left content, so B < xk.
                        assert(forall |k: K| #[trigger] b_content.contains_key(k)
                            ==> x_left_content.contains_key(k));
                        assert(forall |k: K| #[trigger] spec_content_link(&x.left).contains_key(k)
                            ==> (TotalOrder::le(k, xk) && k != xk));
                    }

                    y.right = Some(x);

                    // Prove new x (y.right) is ordered.
                    proof {
                        lemma_ordered_assemble_kv(&y.right);
                    }

                    // Ordering of new y: left=A, right=new_x, key=yk.
                    proof {
                        reveal_with_fuel(spec_content_link, 2);
                        let ghost new_x_content = spec_content_link(&y.right);
                        assert(new_x_content =~=
                            b_content.union_prefer_right(c_content).insert(xk, xv));
                        // C keys > yk by transitivity: C > xk > yk.
                        assert forall |k: K| #[trigger] c_content.contains_key(k)
                            implies (TotalOrder::le(yk, k) && k != yk) by {
                            if c_content.contains_key(k) {
                                lemma_strict_gt_transitive(k, xk, yk);
                            }
                        };
                        // B keys > yk (from original y ordering, captured above).
                        // xk > yk (established above).
                        // Combine: all new_x_content keys > yk.
                        assert(forall |k: K| #[trigger] new_x_content.contains_key(k)
                            ==> (TotalOrder::le(yk, k) && k != yk));
                        // A keys < yk (from original y ordering, captured above).
                        assert(forall |k: K| #[trigger] spec_content_link(&y.left).contains_key(k)
                            ==> (TotalOrder::le(k, yk) && k != yk));
                    }

                    *link = Some(y);
                    proof {
                        reveal_with_fuel(spec_ordered_link, 2);
                        reveal_with_fuel(spec_content_link, 3);
                        reveal_with_fuel(spec_node_count_link, 3);
                        // Prove yk not in C (BST ordering: all C keys > xk > yk).
                        if c_content.contains_key(yk) {
                            K::antisymmetric(yk, xk);
                        }
                        lemma_rotate_right_content_eq(a_content, b_content, c_content, xk, xv, yk, yv);
                        assert(spec_content_link(link) =~= old_content);
                        assert(spec_node_count_link(link) == old_count);
                    }
                } else {
                    *link = Some(x);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn insert_link(link: &mut Link<K, V>, key: K, value: V, priority: u64) -> (inserted: bool)
            decreases old(link),
        {
            proof { reveal_with_fuel(spec_ordered_link, 2); }
            let ghost old_content = spec_content_link(link);
            let ghost old_count = spec_node_count_link(link);
            if let Some(mut node) = link.take() {
                let ghost old_left_content = spec_content_link(&node.left);
                let ghost old_right_content = spec_content_link(&node.right);
                let ghost node_key = node.key;
                let ghost node_value = node.value;
                let ghost old_left_count = spec_node_count_link(&node.left);
                let ghost old_right_count = spec_node_count_link(&node.right);
                proof {
                    reveal_with_fuel(spec_content_link, 2);
                    reveal_with_fuel(spec_node_count_link, 2);
                    assert(old_content =~=
                        old_left_content.union_prefer_right(old_right_content).insert(node_key, node_value));
                    assert(old_count == 1 + old_left_count + old_right_count);
                    // Capture all ordering facts before mutations.
                    assert(spec_ordered_link(&node.left));
                    assert(spec_ordered_link(&node.right));
                    assert(forall |k: K| #[trigger] old_left_content.contains_key(k)
                        ==> (TotalOrder::le(k, node_key) && k != node_key));
                    assert(forall |k: K| #[trigger] old_right_content.contains_key(k)
                        ==> (TotalOrder::le(node_key, k) && k != node_key));
                }

                let c = TotalOrder::cmp(&key, &node.key);
                match c {
                    Ordering::Less => {
                        // Key not in right subtree (all right keys > node_key > key).
                        proof {
                            assert(TotalOrder::le(key, node_key));
                            assert(key != node_key);
                            if old_right_content.contains_key(key) {
                                // Right ordering: le(node_key, key) && key != node_key.
                                // Combined with le(key, node_key): antisymmetric => key == node_key.
                                K::antisymmetric(key, node_key);
                            }
                        }
                        let inserted = Self::insert_link(&mut node.left, key, value, priority);
                        // Ordering: new left = old_left.insert(key,value). All < node_key.
                        proof {
                            assert(forall |k: K| #[trigger]
                                spec_content_link(&node.left).contains_key(k)
                                ==> (TotalOrder::le(k, node_key) && k != node_key));
                            // Right subtree unchanged — reassert ordering.
                            assert(forall |k: K| #[trigger]
                                spec_content_link(&node.right).contains_key(k)
                                ==> (TotalOrder::le(node_key, k) && k != node_key));
                        }
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_content_link, 2);
                            reveal_with_fuel(spec_node_count_link, 2);
                            lemma_insert_left_commutes(old_left_content, old_right_content,
                                node_key, node_value, key, value);
                            assert(spec_content_link(link) =~= old_content.insert(key, value));
                            lemma_ordered_assemble_kv(link);
                        }
                        let need_rotate = match link.as_ref().unwrap().left.as_ref() {
                            Some(left) => left.priority < link.as_ref().unwrap().priority,
                            None => false,
                        };
                        if need_rotate {
                            Self::rotate_right(link);
                        }
                        inserted
                    }
                    Ordering::Greater => {
                        proof {
                            assert(TotalOrder::le(node_key, key));
                            assert(key != node_key);
                        }
                        let inserted = Self::insert_link(&mut node.right, key, value, priority);
                        proof {
                            assert(forall |k: K| #[trigger]
                                spec_content_link(&node.right).contains_key(k)
                                ==> (TotalOrder::le(node_key, k) && k != node_key));
                            // Left subtree unchanged — reassert ordering.
                            assert(forall |k: K| #[trigger]
                                spec_content_link(&node.left).contains_key(k)
                                ==> (TotalOrder::le(k, node_key) && k != node_key));
                        }
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_content_link, 2);
                            reveal_with_fuel(spec_node_count_link, 2);
                            lemma_insert_right_commutes(old_left_content, old_right_content,
                                node_key, node_value, key, value);
                            assert(spec_content_link(link) =~= old_content.insert(key, value));
                            lemma_ordered_assemble_kv(link);
                        }
                        let need_rotate = match link.as_ref().unwrap().right.as_ref() {
                            Some(right) => right.priority < link.as_ref().unwrap().priority,
                            None => false,
                        };
                        if need_rotate {
                            Self::rotate_left(link);
                        }
                        inserted
                    }
                    Ordering::Equal => {
                        assert(key == node_key);
                        node.value = value;
                        // Both children unchanged — reassert ordering.
                        proof {
                            assert(forall |k: K| #[trigger]
                                spec_content_link(&node.left).contains_key(k)
                                ==> (TotalOrder::le(k, node_key) && k != node_key));
                            assert(forall |k: K| #[trigger]
                                spec_content_link(&node.right).contains_key(k)
                                ==> (TotalOrder::le(node_key, k) && k != node_key));
                        }
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_content_link, 2);
                            reveal_with_fuel(spec_node_count_link, 2);
                            assert(spec_content_link(link) =~= old_content.insert(key, value));
                            lemma_ordered_assemble_kv(link);
                        }
                        false
                    }
                }
            } else {
                *link = Some(Box::new(Node::new(key, value, priority)));
                proof {
                    reveal_with_fuel(spec_content_link, 2);
                    reveal_with_fuel(spec_node_count_link, 2);
                    assert(spec_content_link(link) =~= old_content.insert(key, value));
                    lemma_ordered_assemble_kv(link);
                }
                true
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn delete_link(link: &mut Link<K, V>, key: &K) -> (deleted: bool)
            decreases spec_node_count_link(old(link)),
        {
            let ghost old_content = spec_content_link(link);
            let ghost old_count = spec_node_count_link(link);

            if let Some(mut node) = link.take() {
                let ghost old_left_content = spec_content_link(&node.left);
                let ghost old_right_content = spec_content_link(&node.right);
                let ghost node_key = node.key;
                let ghost node_value = node.value;
                let ghost old_left_count = spec_node_count_link(&node.left);
                let ghost old_right_count = spec_node_count_link(&node.right);
                proof {
                    reveal_with_fuel(spec_ordered_link, 2);
                    reveal_with_fuel(spec_content_link, 2);
                    reveal_with_fuel(spec_node_count_link, 2);
                    assert(old_content =~=
                        old_left_content.union_prefer_right(old_right_content).insert(node_key, node_value));
                    assert(old_count == 1 + old_left_count + old_right_count);
                    assert(spec_ordered_link(&node.left));
                    assert(spec_ordered_link(&node.right));
                    assert(forall |k: K| #[trigger] spec_content_link(&node.left).contains_key(k)
                        ==> (TotalOrder::le(k, node.key) && k != node.key));
                    assert(forall |k: K| #[trigger] spec_content_link(&node.right).contains_key(k)
                        ==> (TotalOrder::le(node.key, k) && k != node.key));
                }

                let c = TotalOrder::cmp(key, &node.key);
                match c {
                    Ordering::Less => {
                        proof {
                            assert(TotalOrder::le(*key, node_key));
                            assert(*key != node_key);
                            if old_right_content.contains_key(*key) {
                                K::antisymmetric(*key, node_key);
                            }
                        }
                        let deleted = Self::delete_link(&mut node.left, key);
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_content_link, 2);
                            reveal_with_fuel(spec_node_count_link, 2);
                            assert(spec_content_link(link) =~= old_content.remove(*key));
                            assert forall |k: K| #[trigger] spec_content_link(&node.left).contains_key(k)
                                implies (TotalOrder::le(k, node.key) && k != node.key) by {
                                assert(old_left_content.contains_key(k));
                            };
                            lemma_ordered_assemble_kv(link);
                        }
                        deleted
                    }
                    Ordering::Greater => {
                        proof {
                            assert(TotalOrder::le(node_key, *key));
                            assert(*key != node_key);
                            if old_left_content.contains_key(*key) {
                                K::antisymmetric(node_key, *key);
                            }
                        }
                        let deleted = Self::delete_link(&mut node.right, key);
                        *link = Some(node);
                        proof {
                            reveal_with_fuel(spec_content_link, 2);
                            reveal_with_fuel(spec_node_count_link, 2);
                            assert(spec_content_link(link) =~= old_content.remove(*key));
                            assert forall |k: K| #[trigger] spec_content_link(&node.right).contains_key(k)
                                implies (TotalOrder::le(node.key, k) && k != node.key) by {
                                assert(old_right_content.contains_key(k));
                            };
                            lemma_ordered_assemble_kv(link);
                        }
                        deleted
                    }
                    Ordering::Equal => {
                        assert(*key == node_key);
                        if node.left.is_none() && node.right.is_none() {
                            // Leaf: remove entirely. link is already None from take().
                            proof {
                                reveal_with_fuel(spec_node_count_link, 2);
                                assert(old_content.remove(*key) =~= Map::<K,V>::empty());
                            }
                            true
                        } else {
                            // Has child(ren): rotate target down, then recurse into subtree.
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
                                // After rotate_right, key is in the right subtree.
                                let mut rot = link.take().unwrap();
                                let ghost rot_left_content = spec_content_link(&rot.left);
                                let ghost rot_right_content = spec_content_link(&rot.right);
                                proof {
                                    reveal_with_fuel(spec_ordered_link, 2);
                                    reveal_with_fuel(spec_content_link, 2);
                                    reveal_with_fuel(spec_node_count_link, 2);
                                    assert(old_content =~=
                                        rot_left_content.union_prefer_right(rot_right_content).insert(rot.key, rot.value));
                                }
                                let deleted = Self::delete_link(&mut rot.right, key);
                                *link = Some(rot);
                                proof {
                                    reveal_with_fuel(spec_content_link, 2);
                                    reveal_with_fuel(spec_node_count_link, 2);
                                    // Key not in left subtree (ordering + antisymmetric).
                                    if rot_left_content.contains_key(*key) {
                                        K::antisymmetric(*key, rot.key);
                                    }
                                    assert(spec_content_link(link) =~= old_content.remove(*key));
                                    assert forall |k: K| #[trigger] spec_content_link(&rot.right).contains_key(k)
                                        implies (TotalOrder::le(rot.key, k) && k != rot.key) by {
                                        assert(rot_right_content.contains_key(k));
                                    };
                                    lemma_ordered_assemble_kv(link);
                                }
                                deleted
                            } else {
                                Self::rotate_left(link);
                                // Unfold ordering before take — link still Some(rotated).
                                proof {
                                    reveal_with_fuel(spec_ordered_link, 2);
                                    assert(spec_ordered_link(link));
                                }
                                let mut rot = link.take().unwrap();
                                let ghost rot_left_content = spec_content_link(&rot.left);
                                let ghost rot_right_content = spec_content_link(&rot.right);
                                proof {
                                    reveal_with_fuel(spec_content_link, 2);
                                    reveal_with_fuel(spec_node_count_link, 2);
                                    assert(old_content =~=
                                        rot_left_content.union_prefer_right(rot_right_content).insert(rot.key, rot.value));
                                }
                                let deleted = Self::delete_link(&mut rot.left, key);
                                *link = Some(rot);
                                // Content proof.
                                proof {
                                    reveal_with_fuel(spec_content_link, 2);
                                    reveal_with_fuel(spec_node_count_link, 2);
                                    if rot_right_content.contains_key(*key) {
                                        K::antisymmetric(rot.key, *key);
                                    }
                                    assert(spec_content_link(link) =~= old_content.remove(*key));
                                }
                                // Left subtree ordering after deletion.
                                proof {
                                    assert forall |k: K| #[trigger] spec_content_link(&rot.left).contains_key(k)
                                        implies (TotalOrder::le(k, rot.key) && k != rot.key) by {
                                        assert(rot_left_content.contains_key(k));
                                    };
                                }
                                // Incrementally build conjunction to work around Z3 flakiness.
                                proof {
                                    reveal_with_fuel(spec_ordered_link, 2);
                                    // Connect rot fields to link.unwrap() fields.
                                    let node = link.unwrap();
                                    assert(node.left == rot.left);
                                    assert(node.right == rot.right);
                                    assert(node.key == rot.key);
                                    let ghost c1 = spec_ordered_link(&node.left);
                                    let ghost c2 = spec_ordered_link(&node.right);
                                    assert(c1);
                                    assert(c2);
                                    assert(c1 && c2);
                                    let ghost c3 = forall |k: K| #[trigger] spec_content_link(&node.left).contains_key(k)
                                        ==> (TotalOrder::le(k, node.key) && k != node.key);
                                    assert(c3);
                                    assert(c1 && c2 && c3);
                                    let ghost c4 = forall |k: K| #[trigger] spec_content_link(&node.right).contains_key(k)
                                        ==> (TotalOrder::le(node.key, k) && k != node.key);
                                    assert(c4);
                                    assert(c1 && c2 && c3 && c4);
                                    assert(spec_ordered_link(link) == (c1 && c2 && c3 && c4));
                                    assert(spec_ordered_link(link));
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
        fn find_link<'a>(link: &'a Link<K, V>, key: &K) -> (found: Option<&'a V>)
            decreases *link,
        {
            proof { reveal_with_fuel(spec_ordered_link, 2); }
            match link {
                | None => None,
                | Some(node) => {
                    let c = TotalOrder::cmp(key, &node.key);
                    match c {
                        Ordering::Equal => {
                            assert(*key == node.key);
                            proof {
                                assert(spec_content_link(link) =~=
                                    spec_content_link(&node.left)
                                        .union_prefer_right(spec_content_link(&node.right))
                                        .insert(node.key, node.value));
                            }
                            Some(&node.value)
                        }
                        Ordering::Less => {
                            let r = Self::find_link(&node.left, key);
                            proof {
                                assert(spec_content_link(link) =~=
                                    spec_content_link(&node.left)
                                        .union_prefer_right(spec_content_link(&node.right))
                                        .insert(node.key, node.value));
                                if r.is_some() {
                                    assert(spec_content_link(&node.left).contains_key(*key));
                                }
                                // key < node.key, key != node.key (from TotalOrder::cmp).
                                // If key were in right: right ordering gives le(node.key, key) && key != node.key.
                                // Combined with le(key, node.key): antisymmetric → key == node.key. Contradiction.
                                if spec_content_link(&node.right).contains_key(*key) {
                                    K::antisymmetric(*key, node.key);
                                }
                                if spec_content_link(link).contains_key(*key) {
                                    assert(spec_content_link(&node.left).contains_key(*key));
                                }
                            }
                            r
                        }
                        Ordering::Greater => {
                            let r = Self::find_link(&node.right, key);
                            proof {
                                assert(spec_content_link(link) =~=
                                    spec_content_link(&node.left)
                                        .union_prefer_right(spec_content_link(&node.right))
                                        .insert(node.key, node.value));
                                if r.is_some() {
                                    assert(spec_content_link(&node.right).contains_key(*key));
                                }
                                // key > node.key, key != node.key.
                                // If key were in left: left ordering gives le(key, node.key) && key != node.key.
                                // Combined with le(node.key, key): antisymmetric → key == node.key. Contradiction.
                                if spec_content_link(&node.left).contains_key(*key) {
                                    K::antisymmetric(node.key, *key);
                                }
                                if spec_content_link(link).contains_key(*key) {
                                    assert(spec_content_link(&node.right).contains_key(*key));
                                }
                            }
                            r
                        }
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn min_key_link(link: &Link<K, V>) -> (minimum: Option<&K>)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n) expected, O(n) worst
        fn max_key_link(link: &Link<K, V>) -> (maximum: Option<&K>)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect_keys(link: &Link<K, V>, out: &mut Vec<K>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_keys(&node.left, out);
                out.push(node.key.clone());
                Self::collect_keys(&node.right, out);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect_values(link: &Link<K, V>, out: &mut Vec<V>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_values(&node.left, out);
                out.push(node.value.clone());
                Self::collect_values(&node.right, out);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect_in_order_kvp(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_in_order_kvp(&node.left, out);
                out.push((node.key.clone(), node.value.clone(), node.priority));
                Self::collect_in_order_kvp(&node.right, out);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn find_min_priority_idx_kvp(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (maximum: usize) {
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n) expected, O(n^2) worst
        fn build_treap_from_vec(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (maximum: Link<K, V>)
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
            let mut node = Node::new(key, value, priority);
            node.left = left;
            node.right = right;
            Some(Box::new(node))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_by_key_kvp(
            items: &Vec<(K, V, u64)>, key: &K,
        ) -> (maximum: Vec<(K, V, u64)>) {
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
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord + TotalOrder, V: StT> Default for BSTreeKeyValue<K, V> {
        fn default() -> (default_val: Self)
            ensures default_val.spec_size() == 0, default_val@ == Map::<K, V>::empty(),
        { Self::new() }
    }



    impl<K: StT + Ord + TotalOrder, V: StT> Clone for Node<K, V> {
        fn clone(&self) -> Self {
            proof { assume(spec_ordered_link(&self.left)); assume(spec_ordered_link(&self.right)); } // Clone body: ordering bridge
            Node {
                key: self.key.clone(),
                value: self.value.clone(),
                priority: self.priority,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            }
        }
    }

    impl<K: StT + Ord + TotalOrder, V: StT> Clone for BSTKeyValueStEph<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned@ == self@,
                cloned.size == self.size,
        {
            proof { assume(spec_ordered_link(&self.root)); } // Clone body: ordering bridge
            BSTKeyValueStEph {
                root: clone_link(&self.root),
                size: self.size,
            }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord + TotalOrder, V: StT> PartialEqSpecImpl for BSTKeyValueStEph<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord + TotalOrder, V: StT> Eq for BSTKeyValueStEph<K, V> {}

    impl<K: StT + Ord + TotalOrder, V: StT> PartialEq for BSTKeyValueStEph<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof { assume(spec_ordered_link(&self.root)); assume(spec_ordered_link(&other.root)); } // PartialEq body: ordering bridge
            let equal = compare_kv_links(&self.root, &other.root) && self.size == other.size;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    }

    // 13. derive impls outside verus!



    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for Node<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("value", &self.value)
                .field("priority", &self.priority)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for BSTKeyValueStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTKeyValueStEph")
                .field("root", &self.root)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<K: StT + Ord + fmt::Display, V: StT + fmt::Display> fmt::Display for Node<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}: {})", self.key, self.value)
        }
    }

    impl<K: StT + Ord + fmt::Display, V: StT + fmt::Display> fmt::Display for BSTKeyValueStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTKeyValueStEph(size={})", self.size)
        }
    }

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

    // 12. macros

    #[macro_export]
    macro_rules! BSTKeyValueStEphLit {
        () => {
            < $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEph<_, _> as $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEphTrait<_, _> >::new()
        };
        ( $( ($k:expr, $v:expr) ),* $(,)? ) => {{
            use std::hash::{Hash, Hasher};
            let mut __tree = < $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEph<_, _> as $crate::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::BSTKeyValueStEphTrait<_, _> >::new();
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
