//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! StEphemeral (mutable) implicit-order AVL tree sequence.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 5. view impls
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls
// 11. derive impls in verus!
// 13. derive impls outside verus!

// 1. module

pub mod AVLTreeSeqStEph {

    use std::fmt::Debug;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    // 3. broadcast use

    broadcast use {
        vstd::seq::group_seq_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    // 4. type definitions

    pub type Link<T> = Option<Box<AVLTreeNode<T>>>;

    pub struct AVLTreeNode<T: StT> {
        pub value: T,
        pub height: N,
        pub left_size: N,
        pub right_size: N,
        pub left: Link<T>,
        pub right: Link<T>,
        pub index: N,
    }

    pub struct AVLTreeSeqStEphS<T: StT> {
        pub root: Link<T>,
        pub next_key: N,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqIterStEph<'a, T: StT> {
        pub stack: Vec<&'a AVLTreeNode<T>>,
        pub current: Option<&'a AVLTreeNode<T>>,
    }

    // 5. view impls

    impl<T: StT> View for AVLTreeSeqStEphS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }
    }

    // 6. spec fns

    /// In-order traversal of the tree as a sequence of element views.
    pub open spec fn spec_inorder<T: StT>(link: Link<T>) -> Seq<T::V>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => spec_inorder(node.left) + seq![node.value@] + spec_inorder(node.right),
        }
    }

    pub open spec fn spec_cached_size<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => 1 + node.left_size as nat + node.right_size as nat,
        }
    }

    pub open spec fn spec_cached_height<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.height as nat,
        }
    }

    pub open spec fn spec_nat_max(a: nat, b: nat) -> nat {
        if a >= b { a } else { b }
    }

    /// Well-formedness: cached height and sizes match the actual tree structure.
    pub open spec fn spec_avltreeseqsteph_wf<T: StT>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_avltreeseqsteph_wf(node.left)
                && spec_avltreeseqsteph_wf(node.right)
                && node.left_size as nat == spec_cached_size(&node.left)
                && node.right_size as nat == spec_cached_size(&node.right)
                && node.height as nat == 1 + spec_nat_max(
                    spec_cached_height(&node.left),
                    spec_cached_height(&node.right),
                )
            }
        }
    }

    // 7. proof fns

    /// Under well-formedness, cached size equals in-order sequence length.
    proof fn lemma_size_eq_inorder_len<T: StT>(link: &Link<T>)
        requires spec_avltreeseqsteph_wf(*link),
        ensures spec_cached_size(link) == spec_inorder(*link).len(),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_size_eq_inorder_len::<T>(&node.left);
                lemma_size_eq_inorder_len::<T>(&node.right);
            }
        }
    }

    // 8. traits

    pub trait AVLTreeSeqStEphTrait<T: StT>: Sized {
        spec fn spec_seq(&self) -> Seq<T::V>;
        spec fn spec_well_formed(&self) -> bool;

        fn empty() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_well_formed();

        fn new() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_well_formed();

        fn length(&self) -> (len: N)
            requires self.spec_well_formed(),
            ensures len as nat == self.spec_seq().len();

        fn nth(&self, index: N) -> (elem: &T)
            requires self.spec_well_formed(), (index as int) < self.spec_seq().len(),
            ensures elem@ == self.spec_seq()[index as int];

        fn set(&mut self, index: N, item: T) -> (outcome: Result<(), &'static str>)
            requires old(self).spec_well_formed(), (index as int) < old(self).spec_seq().len();

        fn singleton(item: T) -> (tree: Self)
            ensures tree.spec_seq().len() == 1, tree.spec_well_formed();

        fn isEmpty(&self) -> (empty: B)
            requires self.spec_well_formed(),
            ensures empty == (self.spec_seq().len() == 0);

        fn isSingleton(&self) -> (single: B)
            requires self.spec_well_formed(),
            ensures single == (self.spec_seq().len() == 1);

        fn subseq_copy(&self, start: N, length: N) -> (sub: Self)
            requires self.spec_well_formed();

        fn new_root() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_well_formed();

        fn update(&mut self, index: N, item: T)
            requires
                old(self).spec_well_formed(),
                (index as int) < old(self).spec_seq().len();

        fn from_vec(values: Vec<T>) -> (tree: AVLTreeSeqStEphS<T>);

        fn to_arrayseq(&self) -> (seq: ArraySeqStEphS<T>)
            requires self.spec_well_formed();

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqIterStEph<'a, T>);

        fn push_back(&mut self, value: T)
            requires old(self).spec_well_formed();

        fn contains_value(&self, target: &T) -> (found: B)
            requires self.spec_well_formed();

        fn insert_value(&mut self, value: T)
            requires old(self).spec_well_formed();

        fn delete_value(&mut self, target: &T) -> (deleted: bool)
            requires old(self).spec_well_formed();
    }

    // 9. impls

    fn h_fn<T: StT>(n: &Link<T>) -> (height: N)
        ensures height as nat == spec_cached_height(n),
    {
        match n {
            None => 0,
            Some(b) => b.height,
        }
    }

    fn size_link_fn<T: StT>(n: &Link<T>) -> (size: N)
        ensures size as nat == spec_cached_size(n),
    {
        match n {
            None => 0,
            Some(b) => {
                proof { assume(1 + b.left_size + b.right_size < usize::MAX); }
                1 + b.left_size + b.right_size
            }
        }
    }

    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseqsteph_wf(old(n).left),
            spec_avltreeseqsteph_wf(old(n).right),
        ensures
            n.left_size as nat == spec_cached_size(&n.left),
            n.right_size as nat == spec_cached_size(&n.right),
            n.height as nat == 1 + spec_nat_max(
                spec_cached_height(&n.left), spec_cached_height(&n.right)),
            n.value == old(n).value,
            n.left == old(n).left,
            n.right == old(n).right,
            n.index == old(n).index,
            spec_avltreeseqsteph_wf(Some(*n)),
    {
        n.left_size = size_link_fn(&n.left);
        n.right_size = size_link_fn(&n.right);
        let hl = h_fn(&n.left);
        let hr = h_fn(&n.right);
        assume(1 + (if hl >= hr { hl } else { hr }) as int <= usize::MAX as int);
        n.height = 1 + if hl >= hr { hl } else { hr };
    }

    fn rotate_right_fn<T: StT>(mut y: Box<AVLTreeNode<T>>) -> (rotated: Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseqsteph_wf(Some(y)),
            y.left is Some,
        ensures
            spec_inorder(Some(rotated)) =~= spec_inorder(Some(y)),
            spec_avltreeseqsteph_wf(Some(rotated)),
            spec_cached_size(&Some(rotated)) == spec_cached_size(&Some(y)),
    {
        let ghost old_y = *y;
        proof {
            assert(spec_avltreeseqsteph_wf(old_y.left));
            assert(spec_avltreeseqsteph_wf(old_y.right));
        }
        let mut x = y.left.take().unwrap();
        let ghost old_x = *x;
        proof {
            assert(spec_avltreeseqsteph_wf(old_x.left));
            assert(spec_avltreeseqsteph_wf(old_x.right));
        }
        let b = x.right.take();
        proof { assert(b == old_x.right); }
        y.left = b;
        proof {
            assert(spec_avltreeseqsteph_wf(y.left));
            assert(spec_avltreeseqsteph_wf(y.right));
        }
        update_meta(&mut y);
        x.right = Some(y);
        proof {
            assert(spec_avltreeseqsteph_wf(x.left));
            assert(spec_avltreeseqsteph_wf(x.right));
        }
        update_meta(&mut x);
        proof { reveal_with_fuel(spec_inorder, 3); }
        x
    }

    fn rotate_left_fn<T: StT>(mut x: Box<AVLTreeNode<T>>) -> (rotated: Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseqsteph_wf(Some(x)),
            x.right is Some,
        ensures
            spec_inorder(Some(rotated)) =~= spec_inorder(Some(x)),
            spec_avltreeseqsteph_wf(Some(rotated)),
            spec_cached_size(&Some(rotated)) == spec_cached_size(&Some(x)),
    {
        let ghost old_x = *x;
        proof {
            assert(spec_avltreeseqsteph_wf(old_x.left));
            assert(spec_avltreeseqsteph_wf(old_x.right));
        }
        let mut y = x.right.take().unwrap();
        let ghost old_y = *y;
        proof {
            assert(spec_avltreeseqsteph_wf(old_y.left));
            assert(spec_avltreeseqsteph_wf(old_y.right));
        }
        let b = y.left.take();
        proof { assert(b == old_y.left); }
        x.right = b;
        proof {
            assert(spec_avltreeseqsteph_wf(x.left));
            assert(spec_avltreeseqsteph_wf(x.right));
        }
        update_meta(&mut x);
        y.left = Some(x);
        proof {
            assert(spec_avltreeseqsteph_wf(y.left));
            assert(spec_avltreeseqsteph_wf(y.right));
        }
        update_meta(&mut y);
        proof { reveal_with_fuel(spec_inorder, 3); }
        y
    }

    fn rebalance_fn<T: StT>(mut n: Box<AVLTreeNode<T>>) -> (balanced: Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseqsteph_wf(n.left),
            spec_avltreeseqsteph_wf(n.right),
        ensures
            spec_inorder(Some(balanced)) =~= spec_inorder(Some(n)),
            spec_avltreeseqsteph_wf(Some(balanced)),
            spec_cached_size(&Some(balanced))
                == 1 + spec_cached_size(&n.left) + spec_cached_size(&n.right),
    {
        update_meta(&mut n);
        let hl = h_fn(&n.left);
        let hr = h_fn(&n.right);
        if hl > hr.saturating_add(1) {
            // Left-heavy: n.left must be Some since hl > 1.
            proof {
                if n.left is None { assert(spec_cached_height(&n.left) == 0); }
                assert(n.left is Some);
            }
            if h_fn(&n.left.as_ref().unwrap().right) > h_fn(&n.left.as_ref().unwrap().left) {
                // Left-right case: rotate left child left, then rotate right.
                let left = n.left.take().unwrap();
                proof {
                    // left.right is Some because its height > 0.
                    let lrh = spec_cached_height(&left.right);
                    if left.right is None { assert(lrh == 0); }
                    assert(left.right is Some);
                }
                n.left = Some(rotate_left_fn(left));
                update_meta(&mut n);
            }
            proof { reveal_with_fuel(spec_inorder, 2); }
            return rotate_right_fn(n);
        }
        if hr > hl.saturating_add(1) {
            // Right-heavy: n.right must be Some since hr > 1.
            proof {
                if n.right is None { assert(spec_cached_height(&n.right) == 0); }
                assert(n.right is Some);
            }
            if h_fn(&n.right.as_ref().unwrap().left) > h_fn(&n.right.as_ref().unwrap().right) {
                // Right-left case: rotate right child right, then rotate left.
                let right = n.right.take().unwrap();
                proof {
                    let rlh = spec_cached_height(&right.left);
                    if right.left is None { assert(rlh == 0); }
                    assert(right.left is Some);
                }
                n.right = Some(rotate_right_fn(right));
                update_meta(&mut n);
            }
            proof { reveal_with_fuel(spec_inorder, 2); }
            return rotate_left_fn(n);
        }
        n
    }

    pub fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) -> (inserted: Link<T>)
        requires
            spec_avltreeseqsteph_wf(node),
            0 <= index as int <= spec_inorder(node).len(),
            *old(next_key) < usize::MAX,
            spec_cached_size(&node) + 1 < usize::MAX,
        ensures
            spec_avltreeseqsteph_wf(inserted),
            spec_cached_size(&inserted) == spec_cached_size(&node) + 1,
            *next_key == *old(next_key) + 1,
        decreases node,
    {
        match node {
            None => {
                let key = *next_key;
                *next_key += 1;
                Some(Box::new(AVLTreeNode {
                    value,
                    height: 1,
                    left_size: 0,
                    right_size: 0,
                    left: None,
                    right: None,
                    index: key,
                }))
            }
            Some(mut n) => {
                proof {
                    lemma_size_eq_inorder_len::<T>(&n.left);
                    lemma_size_eq_inorder_len::<T>(&n.right);
                }
                let left_size = n.left_size;
                if index <= left_size {
                    n.left = insert_at_link(n.left.take(), index, value, next_key);
                    proof {
                        assert(spec_avltreeseqsteph_wf(n.left));
                        assert(spec_avltreeseqsteph_wf(n.right));
                    }
                } else {
                    n.right = insert_at_link(
                        n.right.take(), index - left_size - 1, value, next_key,
                    );
                    proof {
                        assert(spec_avltreeseqsteph_wf(n.left));
                        assert(spec_avltreeseqsteph_wf(n.right));
                    }
                }
                Some(rebalance_fn(n))
            }
        }
    }

    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> (elem: &'a T)
        requires spec_avltreeseqsteph_wf(*node), (index as int) < spec_inorder(*node).len(),
        ensures elem@ == spec_inorder(*node)[index as int],
        decreases *node,
    {
        let n = node.as_ref().expect("index out of bounds");
        proof { lemma_size_eq_inorder_len::<T>(&n.left); }
        proof { lemma_size_eq_inorder_len::<T>(&n.right); }
        let left_size = n.left_size;
        if index < left_size {
            nth_link(&n.left, index)
        } else if index == left_size {
            &n.value
        } else {
            nth_link(&n.right, index - left_size - 1)
        }
    }

    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> (outcome: Result<(), &'static str>)
        requires
            spec_avltreeseqsteph_wf(*old(node)),
            (index as int) < spec_inorder(*old(node)).len(),
        ensures
            spec_avltreeseqsteph_wf(*node),
            spec_cached_size(node) == spec_cached_size(old(node)),
            spec_cached_height(node) == spec_cached_height(old(node)),
            outcome is Ok,
        decreases *old(node),
    {
        let cur = node.take();
        match cur {
            None => {
                *node = None;
                Err("Index out of bounds")
            }
            Some(mut n) => {
                proof { lemma_size_eq_inorder_len::<T>(&n.left); }
                proof { lemma_size_eq_inorder_len::<T>(&n.right); }
                let left_size = n.left_size;
                let result = if index < left_size {
                    set_link(&mut n.left, index, value)
                } else if index == left_size {
                    n.value = value;
                    Ok(())
                } else {
                    set_link(&mut n.right, index - left_size - 1, value)
                };
                *node = Some(n);
                result
            }
        }
    }

    #[verifier::external_body]
    fn compare_trees<T: StT>(a: &Link<T>, b: &Link<T>) -> (equal: bool) {
        let sa = size_link_fn(a);
        let sb = size_link_fn(b);
        if sa != sb { return false; }
        for i in 0..sa {
            if nth_link(a, i) != nth_link(b, i) {
                return false;
            }
        }
        true
    }

    // 9. trait impl

    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {
        open spec fn spec_seq(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }

        open spec fn spec_well_formed(&self) -> bool {
            spec_avltreeseqsteph_wf(self.root)
        }

        fn empty() -> (tree: Self) {
            AVLTreeSeqStEphS { root: None, next_key: 0 }
        }

        fn new() -> (tree: Self) {
            Self::empty()
        }

        fn length(&self) -> (len: N) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            size_link_fn(&self.root)
        }

        fn nth(&self, index: N) -> (elem: &T) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            nth_link(&self.root, index)
        }

        fn set(&mut self, index: N, item: T) -> (outcome: Result<(), &'static str>) {
            set_link(&mut self.root, index, item)
        }

        fn singleton(item: T) -> (tree: Self) {
            let mut t = AVLTreeSeqStEphS { root: None, next_key: 0 };
            t.root = insert_at_link(t.root.take(), 0, item, &mut t.next_key);
            proof { lemma_size_eq_inorder_len::<T>(&t.root); }
            t
        }

        fn isEmpty(&self) -> (empty: B) {
            self.length() == 0
        }

        fn isSingleton(&self) -> (single: B) {
            self.length() == 1
        }

        fn subseq_copy(&self, start: N, length: N) -> (sub: Self) {
            assert(self.spec_well_formed());
            let n = self.length();
            let s = if start < n { start } else { n };
            let sum = start.wrapping_add(length);
            let sat = if sum >= start { sum } else { usize::MAX };
            let e = if sat < n { sat } else { n };
            if e <= s {
                return Self::empty();
            }
            let mut vals: Vec<T> = Vec::new();
            let mut i: usize = s;
            while i < e
                invariant
                    self.spec_well_formed(),
                    n as int == self.spec_seq().len(),
                    s <= i, i <= e, e <= n,
                decreases e - i,
            {
                vals.push(self.nth(i).clone());
                i += 1;
            }
            AVLTreeSeqStEphS::from_vec(vals)
        }

        fn new_root() -> (tree: Self) {
            Self::empty()
        }

        fn update(&mut self, index: N, item: T) {
            assert(self.spec_well_formed());
            assert((index as int) < self.spec_seq().len());
            let _ = self.set(index, item);
        }

        fn from_vec(values: Vec<T>) -> (tree: AVLTreeSeqStEphS<T>) {
            let length = values.len();
            let mut t = AVLTreeSeqStEphS { root: None, next_key: 0 };
            let mut i: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < length
                invariant
                    i <= length,
                    length == values@.len(),
                    spec_avltreeseqsteph_wf(t.root),
                    spec_cached_size(&t.root) == i as nat,
                    t.next_key == i,
                decreases length - i,
            {
                proof { lemma_size_eq_inorder_len::<T>(&t.root); }
                assume(i + 1 < usize::MAX);
                t.root = insert_at_link(t.root.take(), i, values[i].clone(), &mut t.next_key);
                i += 1;
            }
            t
        }

        fn to_arrayseq(&self) -> (seq: ArraySeqStEphS<T>) {
            assert(self.spec_well_formed());
            let n = self.length();
            let mut vals: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.spec_well_formed(),
                    n as int == self.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                vals.push(self.nth(i).clone());
                i += 1;
            }
            ArraySeqStEphS::from_vec(vals)
        }

        #[verifier::external_body]
        fn iter<'a>(&'a self) -> (it: AVLTreeSeqIterStEph<'a, T>) {
            let mut it = AVLTreeSeqIterStEph {
                stack: Vec::new(),
                current: None,
            };
            push_left_iter(&mut it, &self.root);
            it
        }

        fn push_back(&mut self, value: T) {
            assert(self.spec_well_formed());
            let len = self.length();
            assume(self.next_key < usize::MAX);
            assume(spec_cached_size(&self.root) + 1 < usize::MAX);
            let node = insert_at_link(self.root.take(), len, value, &mut self.next_key);
            self.root = node;
        }

        fn contains_value(&self, target: &T) -> (found: B) {
            assert(self.spec_well_formed());
            let n = self.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.spec_well_formed(),
                    n as int == self.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                if *self.nth(i) == *target {
                    return true;
                }
                i += 1;
            }
            false
        }

        fn insert_value(&mut self, value: T) {
            assert(self.spec_well_formed());
            self.push_back(value);
        }

        fn delete_value(&mut self, target: &T) -> (deleted: bool) {
            assert(self.spec_well_formed());
            let len = self.length();
            let mut found_index: Option<N> = None;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_well_formed(),
                    len as int == self.spec_seq().len(),
                    i <= len,
                    forall|k: N| found_index == Some(k) ==> (k as int) < len as int,
                decreases len - i,
            {
                if *self.nth(i) == *target {
                    found_index = Some(i);
                    assert((i as int) < len as int);
                    break;
                }
                i += 1;
            }
            if let Some(idx) = found_index {
                assert(idx < len);
                let mut out_vec: Vec<T> = Vec::new();
                let mut j: usize = 0;
                while j < idx
                    invariant
                        self.spec_well_formed(),
                        len as int == self.spec_seq().len(),
                        j <= idx, idx < len,
                    decreases idx - j,
                {
                    out_vec.push(self.nth(j).clone());
                    j += 1;
                }
                let mut k: usize = idx + 1;
                while k < len
                    invariant
                        self.spec_well_formed(),
                        len as int == self.spec_seq().len(),
                        k <= len, idx < len,
                    decreases len - k,
                {
                    out_vec.push(self.nth(k).clone());
                    k += 1;
                }
                *self = AVLTreeSeqStEphS::from_vec(out_vec);
                true
            } else {
                false
            }
        }
    }

    // 10. iterators (structs inside verus!, impl outside)

    // 11. derive impls in verus!

    impl<T: StT> Clone for AVLTreeNode<T> {
        #[verifier::external_body]
        fn clone(&self) -> (copy: Self) {
            AVLTreeNode {
                value: self.value.clone(),
                height: self.height,
                left_size: self.left_size,
                right_size: self.right_size,
                left: self.left.clone(),
                right: self.right.clone(),
                index: self.index,
            }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT> PartialEqSpecImpl for AVLTreeSeqStEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}

    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = compare_trees(&self.root, &other.root);
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: StT> Clone for AVLTreeSeqStEphS<T> {
        #[verifier::external_body]
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@,
        {
            AVLTreeSeqStEphS {
                root: self.root.clone(),
                next_key: self.next_key,
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> Default for AVLTreeSeqStEphS<T> {
        fn default() -> Self { Self::new() }
    }

    // Iterator (outside verus! — stack-based traversal not verified)

    fn push_left_iter<'a, T: StT>(it: &mut AVLTreeSeqIterStEph<'a, T>, link: &'a Link<T>) {
        let mut cursor = link;
        while let Some(node) = cursor.as_ref() {
            it.stack.push(node);
            cursor = &node.left;
        }
    }

    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            let node = self.stack.pop()?;
            let value_ref: &T = &node.value;
            push_left_iter(self, &node.right);
            Some(value_ref)
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSeqStEphLit {
        () => { $crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => {{
            let mut t = $crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS::from_vec(Vec::new());
            for _ in 0..$n { t.push_back($x); }
            t
        }};
        ($($x:expr),* $(,)?) => {{
            let mut t = $crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS::from_vec(Vec::new());
            $( { t.push_back($x); } )*
            t
        }};
    }
}
