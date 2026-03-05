//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! BST with general reduced values augmentation using associative functions.

//  Table of Contents
//  1. module
//  4. type definitions
//  5. view impls
//  6. spec fns
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

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

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


    // 5. view impls

    impl<K: StT + Ord, V: StT, R: StT, Op: ReduceOp<V, R>> View for BSTReducedStEph<K, V, R, Op> {
        type V = Map<K, V>;
        open spec fn view(&self) -> Map<K, V> {
            spec_content_link(&self.root)
        }
    }


    // 6. spec fns

    pub open spec fn spec_size_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> nat {
        match link {
            None => 0,
            Some(n) => n.size as nat,
        }
    }

    pub open spec fn spec_size_wf_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> bool
        decreases *link,
    {
        match link {
            None => true,
            Some(n) => {
                n.size as nat == 1 + spec_size_link(&n.left) + spec_size_link(&n.right)
                && spec_size_wf_link(&n.left)
                && spec_size_wf_link(&n.right)
            }
        }
    }

    pub open spec fn spec_height_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(n) => {
                let l = spec_height_link(&n.left);
                let r = spec_height_link(&n.right);
                1 + if l >= r { l } else { r }
            }
        }
    }

    pub open spec fn spec_content_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> Map<K, V>
        decreases *link,
    {
        match link {
            None => Map::empty(),
            Some(n) =>
                spec_content_link(&n.left)
                    .union_prefer_right(spec_content_link(&n.right))
                    .insert(n.key, n.value),
        }
    }


    // 7. proof fns

    proof fn lemma_wf_assemble<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>)
        requires match link {
            None => true,
            Some(n) => {
                n.size as nat == 1 + spec_size_link(&n.left) + spec_size_link(&n.right)
                && spec_size_wf_link(&n.left)
                && spec_size_wf_link(&n.right)
            }
        },
        ensures spec_size_wf_link(link),
    {}


    // 8. traits

    pub trait NodeTrait<K: StT + Ord, V: StT, R: StT>: Sized {
        spec fn spec_size(&self) -> nat;

        spec fn spec_size_wf(&self) -> bool;

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
        spec fn spec_wf(&self) -> bool;
        spec fn spec_height(&self) -> nat;

        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new() -> (empty: Self)
            ensures
                empty.spec_size() == 0,
                empty.spec_wf(),
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
            requires
                old(self).spec_size() + 1 <= usize::MAX as nat,
                old(self).spec_wf(),
            ensures
                self.spec_wf(),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size() >= old(self).spec_size();
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — filter + rebuild
        fn delete(&mut self, key: &K)
            requires old(self).spec_wf(),
            ensures
                self.spec_wf(),
                self.spec_size() <= old(self).spec_size();
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn find(&self, key: &K) -> (found: Option<&V>)
            requires self.spec_wf(),
            ensures self.spec_size() == 0 ==> found is None;
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn contains(&self, key: &K) -> (contains: bool)
            requires self.spec_wf(),
            ensures self.spec_size() == 0 ==> !contains;
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Θ(n) worst
        fn get(&self, key: &K) -> (value: Option<&V>)
            requires self.spec_wf(),
            ensures self.spec_size() == 0 ==> value is None;
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn keys(&self) -> (keys: ArraySeqStPerS<K>)
            requires self.spec_wf(),
            ensures keys.spec_len() == self.spec_size();
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn values(&self) -> (values: ArraySeqStPerS<V>)
            requires self.spec_wf(),
            ensures values.spec_len() == self.spec_size();
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn minimum_key(&self) -> (minimum: Option<&K>)
            requires self.spec_wf(),
            ensures
                self.spec_size() == 0 ==> minimum is None,
                self.spec_size() > 0 ==> minimum is Some;
        /// - APAS: Work Θ(log n) expected, Span Θ(log n) expected
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn maximum_key(&self) -> (maximum: Option<&K>)
            requires self.spec_wf(),
            ensures
                self.spec_size() == 0 ==> maximum is None,
                self.spec_size() > 0 ==> maximum is Some;
        /// - APAS: Work Θ(1), Span Θ(1) — reads augmented field at root.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn reduced_value(&self) -> (reduced: R)
            requires self.spec_wf(),
            ensures self.spec_size() == 0 ==> reduced == Op::spec_identity();
        /// - APAS: Work Θ(log n), Span Θ(log n) — range query on augmented BST.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn range_reduce(&self, low: &K, high: &K) -> (reduced: R)
            requires self.spec_wf(),
            ensures self.spec_size() == 0 ==> reduced == Op::spec_identity();

        // Internal associated functions.

        fn size_link(link: &Link<K, V, R>) -> (count: usize)
            ensures count as nat == spec_size_link(link);
        /// - APAS: Work Θ(1), Span Θ(1) — reads augmented reduced value.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn reduced_value_link(link: &Link<K, V, R>) -> (reduced: R)
            ensures link.is_none() ==> reduced == Op::spec_identity();
        /// - APAS: Work Θ(1), Span Θ(1) — recomputes size and reduced value from children.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn update_node(node: &mut Node<K, V, R>)
            requires
                1 + spec_size_link(&old(node).left) + spec_size_link(&old(node).right) <= usize::MAX as nat,
                spec_size_wf_link(&old(node).left),
                spec_size_wf_link(&old(node).right),
            ensures
                node.size as nat == 1 + spec_size_link(&node.left) + spec_size_link(&node.right),
                spec_size_wf_link(&node.left),
                spec_size_wf_link(&node.right),
                node.key == old(node).key,
                node.left == old(node).left,
                node.right == old(node).right;
        /// - APAS: Work Θ(1), Span Θ(1) — corresponds to APAS makeNode with reduced values.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn make_node(
            key: K, value: V, priority: u64,
            left: Link<K, V, R>, right: Link<K, V, R>,
        ) -> (reduced: Link<K, V, R>)
            requires
                1 + spec_size_link(&left) + spec_size_link(&right) <= usize::MAX as nat,
                spec_size_wf_link(&left),
                spec_size_wf_link(&right),
            ensures
                spec_size_link(&reduced) == 1 + spec_size_link(&left) + spec_size_link(&right),
                spec_size_wf_link(&reduced);
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn rotate_left(link: &mut Link<K, V, R>)
            requires
                spec_size_link(old(link)) <= usize::MAX as nat,
                spec_size_wf_link(old(link)),
            ensures
                spec_size_link(link) == spec_size_link(old(link)),
                spec_size_wf_link(link);
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn rotate_right(link: &mut Link<K, V, R>)
            requires
                spec_size_link(old(link)) <= usize::MAX as nat,
                spec_size_wf_link(old(link)),
            ensures
                spec_size_link(link) == spec_size_link(old(link)),
                spec_size_wf_link(link);
        /// - Claude-Opus-4.6: Work Θ(log n) expected, Span Θ(log n) expected
        fn insert_link(link: &mut Link<K, V, R>, key: K, value: V, priority: u64)
            requires
                spec_size_link(old(link)) + 1 <= usize::MAX as nat,
                spec_size_wf_link(old(link)),
            ensures
                spec_size_wf_link(link),
                spec_size_link(link) <= spec_size_link(old(link)) + 1,
                spec_size_link(link) >= spec_size_link(old(link)),
            decreases old(link);
        fn find_link<'a>(link: &'a Link<K, V, R>, key: &K) -> (found: Option<&'a V>)
            ensures link.is_none() ==> found.is_none(),
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
            requires spec_size_wf_link(link),
            ensures out.len() == old(out).len() + spec_size_link(link),
            decreases *link;
        fn collect_values(link: &Link<K, V, R>, out: &mut Vec<V>)
            requires spec_size_wf_link(link),
            ensures out.len() == old(out).len() + spec_size_link(link),
            decreases *link;
        fn collect_in_order_kvp(link: &Link<K, V, R>, out: &mut Vec<(K, V, u64)>)
            requires spec_size_wf_link(link),
            ensures out.len() == old(out).len() + spec_size_link(link),
            decreases *link;
        fn height_link(link: &Link<K, V, R>) -> (height: usize)
            requires spec_height_link(link) < usize::MAX as nat,
            ensures height == spec_height_link(link),
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
                spec_size_wf_link(&height),
                spec_size_link(&height) == (end - start) as nat,
            decreases end - start;
        /// - APAS: Work Θ(log n), Span Θ(log n) — range query on augmented BST.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn range_reduce_link(link: &Link<K, V, R>, low: &K, high: &K) -> (reduced: R)
            ensures link.is_none() ==> reduced == Op::spec_identity(),
            decreases *link;
    }


    // 9. impls

    fn clone_link<K: StT + Ord, V: StT, R: StT>(link: &Link<K, V, R>) -> (cloned: Link<K, V, R>)
        ensures
            spec_content_link(&cloned) == spec_content_link(link),
            spec_size_link(&cloned) == spec_size_link(link),
            spec_size_wf_link(link) ==> spec_size_wf_link(&cloned),
        decreases *link,
    {
        match link {
            None => None,
            Some(node) => {
                let k = node.key.clone();
                let v = node.value.clone();
                proof { assume(k == node.key && v == node.value); } // clone bridge, cf. PartialEq pattern
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

        open spec fn spec_size_wf(&self) -> bool
            decreases *self,
        {
            self.size as nat == 1 + spec_size_link(&self.left) + spec_size_link(&self.right)
            && spec_size_wf_link(&self.left)
            && spec_size_wf_link(&self.right)
        }

        open spec fn spec_height(&self) -> nat
            decreases *self,
        {
            let l = spec_height_link(&self.left);
            let r = spec_height_link(&self.right);
            1 + if l >= r { l } else { r }
        }

        open spec fn spec_content(&self) -> Map<K, V>
            decreases *self,
        {
            let l = spec_content_link(&self.left);
            let r = spec_content_link(&self.right);
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

    impl<T: ArithmeticT> ReduceOp<T, T> for SumOp<T> {
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
        open spec fn spec_size(&self) -> nat { spec_size_link(&self.root) }
        open spec fn spec_wf(&self) -> bool { spec_size_wf_link(&self.root) }
        open spec fn spec_height(&self) -> nat { spec_height_link(&self.root) }

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
            let mut in_order: Vec<(K, V, u64)> = Vec::new();
            Self::collect_in_order_kvp(&self.root, &mut in_order);
            let filtered = Self::filter_by_key_kvp(&in_order, key);
            self.root = Self::build_treap_from_vec(&filtered, 0, filtered.len());
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
            let ghost left_sz = spec_size_link(&left);
            let ghost right_sz = spec_size_link(&right);
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
            if let Some(mut x) = link.take() {
                assert(spec_size_wf_link(&x.left));
                assert(spec_size_wf_link(&x.right));
                if let Some(mut y) = x.right.take() {
                    assert(spec_size_wf_link(&y.left));
                    assert(spec_size_wf_link(&y.right));
                    let ghost x_left_sz = spec_size_link(&x.left);
                    let ghost y_left_sz = spec_size_link(&y.left);
                    let ghost y_right_sz = spec_size_link(&y.right);
                    x.right = y.left.take();
                    assert(spec_size_wf_link(&x.left));
                    assert(spec_size_wf_link(&x.right));
                    assert(1 + x_left_sz + y_left_sz + 1 + y_right_sz <= usize::MAX as nat);
                    Self::update_node(&mut x);
                    y.left = Some(x);
                    Self::update_node(&mut y);
                    *link = Some(y);
                    proof { lemma_wf_assemble(link); }
                } else {
                    *link = Some(x);
                    proof { lemma_wf_assemble(link); }
                }
            }
        }

        fn rotate_right(link: &mut Link<K, V, R>) {
            if let Some(mut x) = link.take() {
                assert(spec_size_wf_link(&x.left));
                assert(spec_size_wf_link(&x.right));
                if let Some(mut y) = x.left.take() {
                    assert(spec_size_wf_link(&y.left));
                    assert(spec_size_wf_link(&y.right));
                    let ghost x_right_sz = spec_size_link(&x.right);
                    let ghost y_left_sz = spec_size_link(&y.left);
                    let ghost y_right_sz = spec_size_link(&y.right);
                    x.left = y.right.take();
                    assert(spec_size_wf_link(&x.left));
                    assert(spec_size_wf_link(&x.right));
                    assert(1 + y_right_sz + x_right_sz + 1 + y_left_sz <= usize::MAX as nat);
                    Self::update_node(&mut x);
                    y.right = Some(x);
                    Self::update_node(&mut y);
                    *link = Some(y);
                    proof { lemma_wf_assemble(link); }
                } else {
                    *link = Some(x);
                    proof { lemma_wf_assemble(link); }
                }
            }
        }

        fn insert_link(link: &mut Link<K, V, R>, key: K, value: V, priority: u64)
            decreases old(link),
        {
            if let Some(mut node) = link.take() {
                assert(spec_size_wf_link(&node.left));
                assert(spec_size_wf_link(&node.right));
                if key < node.key {
                    Self::insert_link(&mut node.left, key, value, priority);
                    Self::update_node(&mut *node);
                    *link = Some(node);
                    proof { lemma_wf_assemble(link); }
                    let need_rotate = match link.as_ref().unwrap().left.as_ref() {
                        Some(left) => left.priority < link.as_ref().unwrap().priority,
                        None => false,
                    };
                    if need_rotate {
                        Self::rotate_right(link);
                    }
                } else if key > node.key {
                    Self::insert_link(&mut node.right, key, value, priority);
                    Self::update_node(&mut *node);
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
                    node.value = value;
                    Self::update_node(&mut *node);
                    *link = Some(node);
                    proof { lemma_wf_assemble(link); }
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
                proof { lemma_wf_assemble(link); }
            }
        }

        fn find_link<'a>(link: &'a Link<K, V, R>, key: &K) -> (found: Option<&'a V>)
            decreases *link,
        {
            match link {
                | None => None,
                | Some(node) => {
                    if *key == node.key {
                        Some(&node.value)
                    } else if *key < node.key {
                        Self::find_link(&node.left, key)
                    } else {
                        Self::find_link(&node.right, key)
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
            ensures default_val.spec_size() == 0, default_val.spec_wf(), default_val@ == Map::<K, V>::empty(),
        { Self::new() }
    }


    impl<K: StT + Ord, V: StT, R: StT> Clone for Node<K, V, R> {
        fn clone(&self) -> Self {
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
