//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Implicit-order AVL tree providing O(lg(n)) nth and set by maintaining subtree sizes.
//!
//! Abstract:
//! - `AVLTreeS<T>` stores a balanced binary tree; in-order traversal defines the sequence order.
//! - `AVLTreeNode<T>` stores `value`, `height`, `left_size`, `right_size`, and children.

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

pub mod AVLTreeSeq {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    // 3. broadcast use

    broadcast use vstd::seq::group_seq_axioms;

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

    pub struct AVLTreeS<T: StT> {
        pub root: Link<T>,
        pub next_key: N,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqIter<'a, T: StT> {
        pub stack: Vec<&'a AVLTreeNode<T>>,
        pub current: Option<&'a AVLTreeNode<T>>,
    }

    // 5. view impls

    impl<T: StT> View for AVLTreeS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }
    }

    // 6. spec fns

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

    pub open spec fn spec_wf<T: StT>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_wf(node.left)
                && spec_wf(node.right)
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

    proof fn lemma_size_eq_inorder_len<T: StT>(link: &Link<T>)
        requires spec_wf(*link),
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

    pub trait AVLTreeSeq<T: StT>: Sized {
        spec fn spec_seq(&self) -> Seq<T::V>;
        spec fn spec_well_formed(&self) -> bool;

        fn empty() -> (result: Self)
            ensures result.spec_seq() =~= Seq::<T::V>::empty(), result.spec_well_formed();

        fn new() -> (result: Self)
            ensures result.spec_seq() =~= Seq::<T::V>::empty(), result.spec_well_formed();

        fn length(&self) -> (result: N)
            requires self.spec_well_formed(),
            ensures result as nat == self.spec_seq().len();

        fn nth(&self, index: N) -> (result: &T)
            requires self.spec_well_formed(), (index as int) < self.spec_seq().len(),
            ensures result@ == self.spec_seq()[index as int];

        fn set(&mut self, index: N, item: T) -> (result: Result<(), &'static str>)
            requires old(self).spec_well_formed(), (index as int) < old(self).spec_seq().len();

        fn singleton(item: T) -> (result: Self)
            ensures result.spec_seq().len() == 1, result.spec_well_formed();

        fn isEmpty(&self) -> (result: B)
            requires self.spec_well_formed(),
            ensures result == (self.spec_seq().len() == 0);

        fn isSingleton(&self) -> (result: B)
            requires self.spec_well_formed(),
            ensures result == (self.spec_seq().len() == 1);

        fn subseq_copy(&self, start: N, length: N) -> (result: Self)
            requires self.spec_well_formed();

        fn new_root() -> (result: Self)
            ensures result.spec_seq() =~= Seq::<T::V>::empty(), result.spec_well_formed();

        fn update(&mut self, index: N, item: T);

        fn from_vec(values: Vec<T>) -> (result: AVLTreeS<T>);

        fn to_arrayseq(&self) -> (result: ArraySeqStEphS<T>);

        fn iter<'a>(&'a self) -> (result: AVLTreeSeqIter<'a, T>);

        fn push_back(&mut self, value: T);

        fn contains_value(&self, target: &T) -> (result: B);

        fn insert_value(&mut self, value: T);

        fn delete_value(&mut self, target: &T) -> (result: bool);

        fn is_tree_empty(&self) -> (result: bool);

        fn values_in_order(&self) -> (result: Vec<T>);
    }

    // 9. impls

    fn h_fn<T: StT>(n: &Link<T>) -> (result: N)
        ensures result as nat == spec_cached_height(n),
    {
        match n {
            None => 0,
            Some(b) => b.height,
        }
    }

    #[verifier::external_body]
    fn size_link_fn<T: StT>(n: &Link<T>) -> (result: N)
        ensures result as nat == spec_cached_size(n),
    {
        match n {
            None => 0,
            Some(b) => 1 + b.left_size + b.right_size,
        }
    }

    #[verifier::external_body]
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {
        n.left_size = size_link_fn(&n.left);
        n.right_size = size_link_fn(&n.right);
        let hl = h_fn(&n.left);
        let hr = h_fn(&n.right);
        n.height = 1 + if hl >= hr { hl } else { hr };
    }

    #[verifier::external_body]
    fn rotate_right_fn<T: StT>(mut y: Box<AVLTreeNode<T>>) -> (result: Box<AVLTreeNode<T>>)
        requires spec_wf(Some(y)),
        ensures
            spec_inorder(Some(result)) =~= spec_inorder(Some(y)),
            spec_wf(Some(result)),
    {
        let mut x = y.left.take().expect("rotate_right requires left child");
        let t2 = x.right.take();
        y.left = t2;
        update_meta(&mut y);
        x.right = Some(y);
        update_meta(x.right.as_mut().unwrap());
        update_meta(&mut x);
        x
    }

    #[verifier::external_body]
    fn rotate_left_fn<T: StT>(mut x: Box<AVLTreeNode<T>>) -> (result: Box<AVLTreeNode<T>>)
        requires spec_wf(Some(x)),
        ensures
            spec_inorder(Some(result)) =~= spec_inorder(Some(x)),
            spec_wf(Some(result)),
    {
        let mut y = x.right.take().expect("rotate_left requires right child");
        let t2 = y.left.take();
        x.right = t2;
        update_meta(&mut x);
        y.left = Some(x);
        update_meta(y.left.as_mut().unwrap());
        update_meta(&mut y);
        y
    }

    #[verifier::external_body]
    fn rebalance_fn<T: StT>(mut n: Box<AVLTreeNode<T>>) -> (result: Box<AVLTreeNode<T>>)
        requires spec_wf(Some(n)),
        ensures
            spec_inorder(Some(result)) =~= spec_inorder(Some(n)),
            spec_wf(Some(result)),
    {
        update_meta(&mut n);
        let hl = h_fn(&n.left);
        let hr = h_fn(&n.right);
        if hl > hr.saturating_add(1) {
            if h_fn(&n.left.as_ref().unwrap().right) > h_fn(&n.left.as_ref().unwrap().left) {
                let left = n.left.take().unwrap();
                n.left = Some(rotate_left_fn(left));
            }
            return rotate_right_fn(n);
        }
        if hr > hl.saturating_add(1) {
            if h_fn(&n.right.as_ref().unwrap().left) > h_fn(&n.right.as_ref().unwrap().right) {
                let right = n.right.take().unwrap();
                n.right = Some(rotate_right_fn(right));
            }
            return rotate_left_fn(n);
        }
        n
    }

    #[verifier::external_body]
    pub(crate) fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) -> (result: Link<T>) {
        match node {
            None => {
                debug_assert!(index == 0, "insert_at_link reached None with index > 0");
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
                let left_size = n.left_size;
                if index <= left_size {
                    n.left = insert_at_link(n.left.take(), index, value, next_key);
                } else {
                    n.right = insert_at_link(n.right.take(), index - left_size - 1, value, next_key);
                }
                Some(rebalance_fn(n))
            }
        }
    }

    #[verifier::external_body]
    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> (result: &'a T)
        requires spec_wf(*node), (index as int) < spec_inorder(*node).len(),
        ensures result@ == spec_inorder(*node)[index as int],
    {
        let n = node.as_ref().expect("index out of bounds");
        let left_size = n.left_size;
        if index < left_size {
            return nth_link(&n.left, index);
        }
        if index == left_size {
            return &n.value;
        }
        nth_link(&n.right, index - left_size - 1)
    }

    #[verifier::external_body]
    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> (result: Result<(), &'static str>) {
        match node {
            None => Err("Index out of bounds"),
            Some(n) => {
                let left_size = n.left_size;
                if index < left_size {
                    set_link(&mut n.left, index, value)
                } else if index == left_size {
                    n.value = value;
                    Ok(())
                } else {
                    set_link(&mut n.right, index - left_size - 1, value)
                }
            }
        }
    }

    #[verifier::external_body]
    fn push_inorder<T: StT>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(n) = link {
            push_inorder(&n.left, out);
            out.push(n.value.clone());
            push_inorder(&n.right, out);
        }
    }

    #[verifier::external_body]
    fn compare_trees<T: StT>(a: &Link<T>, b: &Link<T>) -> (result: bool) {
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

    impl<T: StT> AVLTreeSeq<T> for AVLTreeS<T> {
        open spec fn spec_seq(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }

        open spec fn spec_well_formed(&self) -> bool {
            spec_wf(self.root)
        }

        fn empty() -> (result: Self) {
            AVLTreeS { root: None, next_key: 0 }
        }

        fn new() -> (result: Self) {
            Self::empty()
        }

        fn length(&self) -> (result: N) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            size_link_fn(&self.root)
        }

        #[verifier::external_body]
        fn nth(&self, index: N) -> (result: &T) {
            assert!(index < self.length(), "index out of bounds");
            nth_link(&self.root, index)
        }

        #[verifier::external_body]
        fn set(&mut self, index: N, item: T) -> (result: Result<(), &'static str>) {
            set_link(&mut self.root, index, item)
        }

        #[verifier::external_body]
        fn singleton(item: T) -> (result: Self) {
            let mut t = Self::empty();
            t.root = insert_at_link(t.root.take(), 0, item, &mut t.next_key);
            t
        }

        fn isEmpty(&self) -> (result: B) {
            self.length() == 0
        }

        fn isSingleton(&self) -> (result: B) {
            self.length() == 1
        }

        #[verifier::external_body]
        fn subseq_copy(&self, start: N, length: N) -> (result: Self) {
            let n = self.length();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return Self::empty();
            }
            let mut vals = Vec::<T>::with_capacity(e - s);
            for i in s..e {
                vals.push(self.nth(i).clone());
            }
            AVLTreeS::from_vec(vals)
        }

        fn new_root() -> (result: Self) {
            Self::empty()
        }

        #[verifier::external_body]
        fn update(&mut self, index: N, item: T) {
            let _ = self.set(index, item);
        }

        #[verifier::external_body]
        fn from_vec(values: Vec<T>) -> (result: AVLTreeS<T>) {
            let length = values.len();
            let mut t = AVLTreeS::empty();
            for (i, v) in values.into_iter().enumerate() {
                t.root = insert_at_link(t.root.take(), i, v, &mut t.next_key);
            }
            debug_assert!(t.length() == length);
            t
        }

        #[verifier::external_body]
        fn to_arrayseq(&self) -> (result: ArraySeqStEphS<T>) {
            let len = self.length();
            if len == 0 {
                return ArraySeqStEphS::empty();
            }
            let mut it = self.iter();
            let first = it.next().expect("length > 0 but iter was empty").clone();
            let mut out = ArraySeqStEphS::new(len, first.clone());
            let _ = out.set(0, first);
            let mut index: N = 1;
            for v in it {
                let _ = out.set(index, v.clone());
                index += 1;
            }
            out
        }

        #[verifier::external_body]
        fn iter<'a>(&'a self) -> (result: AVLTreeSeqIter<'a, T>) {
            let mut it = AVLTreeSeqIter {
                stack: Vec::new(),
                current: None,
            };
            push_left_iter(&mut it, &self.root);
            it
        }

        #[verifier::external_body]
        fn push_back(&mut self, value: T) {
            let len = self.length();
            let node = insert_at_link(self.root.take(), len, value, &mut self.next_key);
            self.root = node;
        }

        #[verifier::external_body]
        fn contains_value(&self, target: &T) -> (result: B) {
            for v in self.iter() {
                if v == target {
                    return true;
                }
            }
            false
        }

        #[verifier::external_body]
        fn insert_value(&mut self, value: T) {
            self.push_back(value);
        }

        #[verifier::external_body]
        fn delete_value(&mut self, target: &T) -> (result: bool) {
            let len = self.length();
            let mut found_index: Option<N> = None;
            for i in 0..len {
                if self.nth(i) == target {
                    found_index = Some(i);
                    break;
                }
            }
            if let Some(idx) = found_index {
                let mut out_vec = Vec::<T>::with_capacity(len - 1);
                for i in 0..idx {
                    out_vec.push(self.nth(i).clone());
                }
                for i in (idx + 1)..len {
                    out_vec.push(self.nth(i).clone());
                }
                *self = AVLTreeS::from_vec(out_vec);
                true
            } else {
                false
            }
        }

        #[verifier::external_body]
        fn is_tree_empty(&self) -> (result: bool) {
            self.length() == 0
        }

        #[verifier::external_body]
        fn values_in_order(&self) -> (result: Vec<T>) {
            let mut out = Vec::with_capacity(self.length());
            push_inorder(&self.root, &mut out);
            out
        }
    }

    // 11. derive impls in verus!

    impl<T: StT> Clone for AVLTreeNode<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self) {
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
    impl<T: StT> PartialEqSpecImpl for AVLTreeS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT> Eq for AVLTreeS<T> {}

    impl<T: StT> PartialEq for AVLTreeS<T> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let r = compare_trees(&self.root, &other.root);
            proof { assume(r == (self@ == other@)); }
            r
        }
    }

    impl<T: StT> Clone for AVLTreeS<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@,
        {
            AVLTreeS {
                root: self.root.clone(),
                next_key: self.next_key,
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> Default for AVLTreeS<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StT> Debug for AVLTreeS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let elts = (0..self.length()).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: StT> Display for AVLTreeS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            for v in self.iter() {
                if !first { write!(f, ", ")?; }
                first = false;
                write!(f, "{v}")?;
            }
            write!(f, "]")
        }
    }

    // Iterator (outside verus!)

    fn push_left_iter<'a, T: StT>(it: &mut AVLTreeSeqIter<'a, T>, link: &'a Link<T>) {
        let mut cursor = link;
        while let Some(node) = cursor.as_ref() {
            it.stack.push(node);
            cursor = &node.left;
        }
    }

    impl<'a, T: StT> Iterator for AVLTreeSeqIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            let node = self.stack.pop()?;
            let value_ref: &T = &node.value;
            push_left_iter(self, &node.right);
            Some(value_ref)
        }
    }
}
