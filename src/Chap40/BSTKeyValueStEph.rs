//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

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
    use crate::vstdplus::accept::accept;
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

    // 7. proof fns

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

        fn new(key: K, value: V, priority: u64) -> (node: Self);
    }

    pub trait BSTKeyValueStEphTrait<K: StT + Ord + TotalOrder, V: StT>: Sized + View<V = Map<K, V>> {
        spec fn spec_size(&self) -> nat;
        spec fn spec_height(&self) -> nat;
        spec fn spec_bstkeyvaluesteph_wf(&self) -> bool;
        spec fn spec_min_key(&self) -> Option<K>;
        spec fn spec_max_key(&self) -> Option<K>;

        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new() -> (empty: Self)
            ensures
                empty.spec_size() == 0,
                empty@ == Map::<K, V>::empty();
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures count as nat == self.spec_size();
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == (self.spec_size() == 0);
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn height(&self) -> (height: usize)
            requires self.spec_height() < usize::MAX as nat,
            ensures height as nat == self.spec_height();
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn insert(&mut self, key: K, value: V, priority: u64)
            requires old(self).spec_size() < usize::MAX,
            ensures
                self@.contains_key(key),
                self.spec_size() >= old(self).spec_size(),
                self.spec_size() <= old(self).spec_size() + 1;
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — filter + rebuild
        fn delete(&mut self, key: &K)
            requires old(self).spec_bstkeyvaluesteph_wf(),
            ensures self.spec_size() <= old(self).spec_size();
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn find(&self, key: &K) -> (found: Option<&V>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures
                self.spec_size() == 0 ==> found is None,
                found.is_some() ==> self@.contains_key(*key);
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn contains(&self, key: &K) -> (contains: bool)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures
                self.spec_size() == 0 ==> !contains,
                contains ==> self@.contains_key(*key);
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn get(&self, key: &K) -> (value: Option<&V>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures
                self.spec_size() == 0 ==> value is None,
                value.is_some() ==> self@.contains_key(*key);
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn keys(&self) -> (keys: ArraySeqStPerS<K>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures keys.spec_len() == self.spec_size();
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn values(&self) -> (values: ArraySeqStPerS<V>)
            requires self.spec_bstkeyvaluesteph_wf(),
            ensures values.spec_len() == self.spec_size();
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
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
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
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

        fn height_link(link: &Link<K, V>) -> (height: usize)
            requires Lnk::spec_height_link(link) < usize::MAX as nat,
            ensures height == Lnk::spec_height_link(link),
            decreases *link;
        fn rotate_left(link: &mut Link<K, V>)
            ensures
                link.is_some() == old(link).is_some(),
                forall|k: K| #[trigger] spec_content_link(old(link)).contains_key(k) ==> spec_content_link(link).contains_key(k);
        fn rotate_right(link: &mut Link<K, V>)
            ensures
                link.is_some() == old(link).is_some(),
                forall|k: K| #[trigger] spec_content_link(old(link)).contains_key(k) ==> spec_content_link(link).contains_key(k);
        fn insert_link(link: &mut Link<K, V>, key: K, value: V, priority: u64) -> (inserted: bool)
            ensures
                link.is_some(),
                spec_content_link(link).contains_key(key),
            decreases old(link);
        fn find_link<'a>(link: &'a Link<K, V>, key: &K) -> (found: Option<&'a V>)
            ensures
                link.is_none() ==> found.is_none(),
                found.is_some() ==> spec_content_link(link).contains_key(*key),
            decreases *link;
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
        fn collect_keys(link: &Link<K, V>, out: &mut Vec<K>)
            ensures out.len() == old(out).len() + spec_node_count_link(link),
            decreases *link;
        fn collect_values(link: &Link<K, V>, out: &mut Vec<V>)
            ensures out.len() == old(out).len() + spec_node_count_link(link),
            decreases *link;
        fn collect_in_order_kvp(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>)
            ensures out.len() == old(out).len() + spec_node_count_link(link),
            decreases *link;
        fn find_min_priority_idx_kvp(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (maximum: usize)
            requires start < end, end <= items.len(),
            ensures start <= maximum && maximum < end;
        fn build_treap_from_vec(
            items: &Vec<(K, V, u64)>, start: usize, end: usize,
        ) -> (maximum: Link<K, V>)
            requires start <= end, end <= items.len(),
            ensures maximum.is_none() == (start == end),
            decreases end - start;
        fn filter_by_key_kvp(
            items: &Vec<(K, V, u64)>, key: &K,
        ) -> (maximum: Vec<(K, V, u64)>)
            ensures maximum.len() <= items.len();
    }



    fn clone_link<K: StT + Ord, V: StT>(link: &Link<K, V>) -> (cloned: Link<K, V>)
        requires true,
        ensures
            spec_content_link(&cloned) == spec_content_link(link),
            spec_node_count_link(&cloned) == spec_node_count_link(link),
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => {
                let k = node.key.clone();
                let v = node.value.clone();
                proof { accept(k == node.key && v == node.value); } // accept hole: Clone bridge
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

    fn compare_kv_links<K: StT + Ord, V: StT>(a: &Link<K, V>, b: &Link<K, V>) -> (equal: bool)
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
        }
        open spec fn spec_min_key(&self) -> Option<K> { Lnk::spec_min_key_link(&self.root) }
        open spec fn spec_max_key(&self) -> Option<K> { Lnk::spec_max_key_link(&self.root) }

        fn new() -> (empty: Self) { BSTKeyValueStEph { root: None, size: 0 } }

        fn size(&self) -> (count: usize) { self.size }

        fn is_empty(&self) -> (is_empty: bool) { self.size == 0 }

        fn height(&self) -> (height: usize) { Self::height_link(&self.root) }

        fn insert(&mut self, key: K, value: V, priority: u64) {
            let inserted = Self::insert_link(&mut self.root, key, value, priority);
            if inserted {
                self.size = self.size + 1;
            }
        }

        fn delete(&mut self, key: &K) {
            let mut in_order: Vec<(K, V, u64)> = Vec::new();
            Self::collect_in_order_kvp(&self.root, &mut in_order);
            let filtered = Self::filter_by_key_kvp(&in_order, key);
            self.root = Self::build_treap_from_vec(&filtered, 0, filtered.len());
            self.size = filtered.len();
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

        // Internal associated functions.

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

        fn rotate_left(link: &mut Link<K, V>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.right.take() {
                    let ghost a = spec_content_link(&x.left);
                    let ghost b = spec_content_link(&y.left);
                    let ghost c = spec_content_link(&y.right);
                    let ghost xk = x.key;
                    let ghost xv = x.value;
                    let ghost yk = y.key;
                    let ghost yv = y.value;
                    x.right = y.left.take();
                    y.left = Some(x);
                    *link = Some(y);
                    proof {
                        reveal_with_fuel(spec_content_link, 3);
                        lemma_rotate_left_preserves_keys(a, b, c, xk, xv, yk, yv);
                    }
                } else {
                    *link = Some(x);
                }
            }
        }

        fn rotate_right(link: &mut Link<K, V>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.left.take() {
                    let ghost a = spec_content_link(&y.left);
                    let ghost b = spec_content_link(&y.right);
                    let ghost c = spec_content_link(&x.right);
                    let ghost xk = x.key;
                    let ghost xv = x.value;
                    let ghost yk = y.key;
                    let ghost yv = y.value;
                    x.left = y.right.take();
                    y.right = Some(x);
                    *link = Some(y);
                    proof {
                        reveal_with_fuel(spec_content_link, 3);
                        lemma_rotate_right_preserves_keys(a, b, c, xk, xv, yk, yv);
                    }
                } else {
                    *link = Some(x);
                }
            }
        }

        fn insert_link(link: &mut Link<K, V>, key: K, value: V, priority: u64) -> (inserted: bool)
            decreases old(link),
        {
            proof { reveal_with_fuel(spec_content_link, 2); }
            if let Some(mut node) = link.take() {
                let c = TotalOrder::cmp(&key, &node.key);
                match c {
                    Ordering::Less => {
                        let inserted = Self::insert_link(&mut node.left, key, value, priority);
                        *link = Some(node);
                        proof { lemma_left_key_in_link(link, key); }
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
                        let inserted = Self::insert_link(&mut node.right, key, value, priority);
                        *link = Some(node);
                        proof { lemma_right_key_in_link(link, key); }
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
                        // TotalOrder::cmp ensures: key == node.key (spec equality).
                        node.value = value;
                        *link = Some(node);
                        proof { lemma_node_key_in_link(link); }
                        false
                    }
                }
            } else {
                *link = Some(Box::new(Node::new(key, value, priority)));
                true
            }
        }

        fn find_link<'a>(link: &'a Link<K, V>, key: &K) -> (found: Option<&'a V>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => {
                    let c = TotalOrder::cmp(key, &node.key);
                    match c {
                        Ordering::Equal => {
                            // TotalOrder::cmp ensures: *key == node.key (spec equality).
                            proof { lemma_node_key_in_link(link); }
                            Some(&node.value)
                        }
                        Ordering::Less => {
                            let r = Self::find_link(&node.left, key);
                            proof {
                                if r.is_some() {
                                    lemma_left_key_in_link(link, *key);
                                }
                            }
                            r
                        }
                        Ordering::Greater => {
                            let r = Self::find_link(&node.right, key);
                            proof {
                                if r.is_some() {
                                    lemma_right_key_in_link(link, *key);
                                }
                            }
                            r
                        }
                    }
                }
            }
        }

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

        fn collect_keys(link: &Link<K, V>, out: &mut Vec<K>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_keys(&node.left, out);
                out.push(node.key.clone());
                Self::collect_keys(&node.right, out);
            }
        }

        fn collect_values(link: &Link<K, V>, out: &mut Vec<V>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_values(&node.left, out);
                out.push(node.value.clone());
                Self::collect_values(&node.right, out);
            }
        }

        fn collect_in_order_kvp(link: &Link<K, V>, out: &mut Vec<(K, V, u64)>)
            decreases *link,
        {
            if let Some(node) = link {
                Self::collect_in_order_kvp(&node.left, out);
                out.push((node.key.clone(), node.value.clone(), node.priority));
                Self::collect_in_order_kvp(&node.right, out);
            }
        }

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



    impl<K: StT + Ord, V: StT> Clone for Node<K, V> {
        fn clone(&self) -> Self {
            Node {
                key: self.key.clone(),
                value: self.value.clone(),
                priority: self.priority,
                left: clone_link(&self.left),
                right: clone_link(&self.right),
            }
        }
    }

    impl<K: StT + Ord, V: StT> Clone for BSTKeyValueStEph<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned@ == self@,
                cloned.size == self.size,
        {
            BSTKeyValueStEph {
                root: clone_link(&self.root),
                size: self.size,
            }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT> PartialEqSpecImpl for BSTKeyValueStEph<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT> Eq for BSTKeyValueStEph<K, V> {}

    impl<K: StT + Ord, V: StT> PartialEq for BSTKeyValueStEph<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = compare_kv_links(&self.root, &other.root) && self.size == other.size;
            proof { accept(equal == (self@ == other@)); }
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
