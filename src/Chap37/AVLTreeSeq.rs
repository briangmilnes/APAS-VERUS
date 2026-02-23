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
    use crate::vstdplus::accept::accept;

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
            spec_avltreeseq_inorder(self.root)
        }
    }

    // 6. spec fns

    pub open spec fn spec_avltreeseq_inorder<T: StT>(link: Link<T>) -> Seq<T::V>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => spec_avltreeseq_inorder(node.left) + seq![node.value@] + spec_avltreeseq_inorder(node.right),
        }
    }

    pub open spec fn spec_avltreeseq_cached_size<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => 1 + node.left_size as nat + node.right_size as nat,
        }
    }

    pub open spec fn spec_avltreeseq_cached_height<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.height as nat,
        }
    }

    pub open spec fn spec_avltreeseq_nat_max(a: nat, b: nat) -> nat {
        if a >= b { a } else { b }
    }

    pub open spec fn spec_avltreeseq_wf<T: StT>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_avltreeseq_wf(node.left)
                && spec_avltreeseq_wf(node.right)
                && node.left_size as nat == spec_avltreeseq_cached_size(&node.left)
                && node.right_size as nat == spec_avltreeseq_cached_size(&node.right)
                && node.height as nat == 1 + spec_avltreeseq_nat_max(
                    spec_avltreeseq_cached_height(&node.left),
                    spec_avltreeseq_cached_height(&node.right),
                )
            }
        }
    }

    // 7. proof fns

    proof fn lemma_size_eq_inorder_len<T: StT>(link: &Link<T>)
        requires spec_avltreeseq_wf(*link),
        ensures spec_avltreeseq_cached_size(link) == spec_avltreeseq_inorder(*link).len(),
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
        spec fn spec_avltreeseq_seq(&self) -> Seq<T::V>;
        spec fn spec_avltreeseq_wf(&self) -> bool;

        fn empty() -> (result: Self)
            ensures result.spec_avltreeseq_seq() =~= Seq::<T::V>::empty(), result.spec_avltreeseq_wf();

        fn new() -> (result: Self)
            ensures result.spec_avltreeseq_seq() =~= Seq::<T::V>::empty(), result.spec_avltreeseq_wf();

        fn length(&self) -> (result: N)
            requires self.spec_avltreeseq_wf(),
            ensures result as nat == self.spec_avltreeseq_seq().len();

        fn nth(&self, index: N) -> (result: &T)
            requires self.spec_avltreeseq_wf(), (index as int) < self.spec_avltreeseq_seq().len(),
            ensures result@ == self.spec_avltreeseq_seq()[index as int];

        fn set(&mut self, index: N, item: T) -> (result: Result<(), &'static str>)
            requires old(self).spec_avltreeseq_wf(), (index as int) < old(self).spec_avltreeseq_seq().len();

        fn singleton(item: T) -> (result: Self)
            ensures result.spec_avltreeseq_seq().len() == 1, result.spec_avltreeseq_wf();

        fn isEmpty(&self) -> (result: B)
            requires self.spec_avltreeseq_wf(),
            ensures result == (self.spec_avltreeseq_seq().len() == 0);

        fn isSingleton(&self) -> (result: B)
            requires self.spec_avltreeseq_wf(),
            ensures result == (self.spec_avltreeseq_seq().len() == 1);

        fn subseq_copy(&self, start: N, length: N) -> (result: Self)
            requires self.spec_avltreeseq_wf();

        fn new_root() -> (result: Self)
            ensures result.spec_avltreeseq_seq() =~= Seq::<T::V>::empty(), result.spec_avltreeseq_wf();

        fn update(&mut self, index: N, item: T)
            requires
                old(self).spec_avltreeseq_wf(),
                (index as int) < old(self).spec_avltreeseq_seq().len();

        fn from_vec(values: Vec<T>) -> (result: AVLTreeS<T>);

        fn to_arrayseq(&self) -> (result: ArraySeqStEphS<T>)
            requires self.spec_avltreeseq_wf();

        fn iter<'a>(&'a self) -> (result: AVLTreeSeqIter<'a, T>);

        fn push_back(&mut self, value: T)
            requires old(self).spec_avltreeseq_wf();

        fn contains_value(&self, target: &T) -> (result: B)
            requires self.spec_avltreeseq_wf();

        fn insert_value(&mut self, value: T)
            requires old(self).spec_avltreeseq_wf();

        fn delete_value(&mut self, target: &T) -> (result: bool)
            requires old(self).spec_avltreeseq_wf();

        fn is_tree_empty(&self) -> (result: bool)
            requires self.spec_avltreeseq_wf();

        fn values_in_order(&self) -> (result: Vec<T>)
            requires self.spec_avltreeseq_wf();
    }

    // 9. impls

    fn cached_height<T: StT>(n: &Link<T>) -> (result: N)
        ensures result as nat == spec_avltreeseq_cached_height(n),
    {
        match n {
            None => 0,
            Some(b) => b.height,
        }
    }

    fn cached_size<T: StT>(n: &Link<T>) -> (result: N)
        ensures result as nat == spec_avltreeseq_cached_size(n),
    {
        match n {
            None => 0,
            Some(b) => {
                proof { assume(1 + b.left_size + b.right_size < usize::MAX); }
                1 + b.left_size + b.right_size
            }
        }
    }

    fn update_size_height<T: StT>(n: &mut Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseq_wf(old(n).left),
            spec_avltreeseq_wf(old(n).right),
            1 + spec_avltreeseq_cached_height(&old(n).left)
              + spec_avltreeseq_cached_height(&old(n).right) < usize::MAX,
        ensures
            n.left_size as nat == spec_avltreeseq_cached_size(&n.left),
            n.right_size as nat == spec_avltreeseq_cached_size(&n.right),
            n.height as nat == 1 + spec_avltreeseq_nat_max(
                spec_avltreeseq_cached_height(&n.left),
                spec_avltreeseq_cached_height(&n.right),
            ),
            n.value == old(n).value,
            n.left == old(n).left,
            n.right == old(n).right,
            n.index == old(n).index,
            spec_avltreeseq_inorder(Some(*n)) =~= spec_avltreeseq_inorder(Some(*old(n))),
    {
        n.left_size = cached_size(&n.left);
        n.right_size = cached_size(&n.right);
        let hl = cached_height(&n.left);
        let hr = cached_height(&n.right);
        n.height = 1 + if hl >= hr { hl } else { hr };
    }

    #[verifier::external_body]
    fn rotate_right_fn<T: StT>(mut y: Box<AVLTreeNode<T>>) -> (result: Box<AVLTreeNode<T>>)
        requires spec_avltreeseq_wf(Some(y)),
        ensures
            spec_avltreeseq_inorder(Some(result)) =~= spec_avltreeseq_inorder(Some(y)),
            spec_avltreeseq_wf(Some(result)),
    {
        let mut x = y.left.take().expect("rotate_right requires left child");
        let t2 = x.right.take();
        y.left = t2;
        update_size_height(&mut y);
        x.right = Some(y);
        update_size_height(x.right.as_mut().unwrap());
        update_size_height(&mut x);
        x
    }

    #[verifier::external_body]
    fn rotate_left_fn<T: StT>(mut x: Box<AVLTreeNode<T>>) -> (result: Box<AVLTreeNode<T>>)
        requires spec_avltreeseq_wf(Some(x)),
        ensures
            spec_avltreeseq_inorder(Some(result)) =~= spec_avltreeseq_inorder(Some(x)),
            spec_avltreeseq_wf(Some(result)),
    {
        let mut y = x.right.take().expect("rotate_left requires right child");
        let t2 = y.left.take();
        x.right = t2;
        update_size_height(&mut x);
        y.left = Some(x);
        update_size_height(y.left.as_mut().unwrap());
        update_size_height(&mut y);
        y
    }

    #[verifier::external_body]
    fn rebalance_fn<T: StT>(mut n: Box<AVLTreeNode<T>>) -> (result: Box<AVLTreeNode<T>>)
        requires spec_avltreeseq_wf(Some(n)),
        ensures
            spec_avltreeseq_inorder(Some(result)) =~= spec_avltreeseq_inorder(Some(n)),
            spec_avltreeseq_wf(Some(result)),
    {
        update_size_height(&mut n);
        let hl = cached_height(&n.left);
        let hr = cached_height(&n.right);
        if hl > hr.saturating_add(1) {
            if cached_height(&n.left.as_ref().unwrap().right) > cached_height(&n.left.as_ref().unwrap().left) {
                let left = n.left.take().unwrap();
                n.left = Some(rotate_left_fn(left));
            }
            return rotate_right_fn(n);
        }
        if hr > hl.saturating_add(1) {
            if cached_height(&n.right.as_ref().unwrap().left) > cached_height(&n.right.as_ref().unwrap().right) {
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

    fn nth_link<'a, T: StT>(node: &'a Link<T>, index: N) -> (result: &'a T)
        requires spec_avltreeseq_wf(*node), (index as int) < spec_avltreeseq_inorder(*node).len(),
        ensures result@ == spec_avltreeseq_inorder(*node)[index as int],
        decreases *node,
    {
        let n = node.as_ref().unwrap();
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

    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> (result: Result<(), &'static str>)
        requires
            spec_avltreeseq_wf(*old(node)),
            (index as int) < spec_avltreeseq_inorder(*old(node)).len(),
        ensures
            spec_avltreeseq_wf(*node),
            spec_avltreeseq_inorder(*node) =~= spec_avltreeseq_inorder(*old(node)).update(index as int, value@),
            spec_avltreeseq_cached_size(node) == spec_avltreeseq_cached_size(old(node)),
            spec_avltreeseq_cached_height(node) == spec_avltreeseq_cached_height(old(node)),
            result is Ok,
        decreases *old(node),
    {
        let mut n = node.take().unwrap();
        proof { lemma_size_eq_inorder_len::<T>(&n.left); }
        proof { lemma_size_eq_inorder_len::<T>(&n.right); }
        let left_size = n.left_size;
        if index < left_size {
            set_link(&mut n.left, index, value);
        } else if index == left_size {
            n.value = value;
        } else {
            set_link(&mut n.right, index - left_size - 1, value);
        }
        *node = Some(n);
        Ok(())
    }

    fn push_inorder<T: StT>(link: &Link<T>, out: &mut Vec<T>)
        requires spec_avltreeseq_wf(*link),
        ensures
            out@.map_values(|t: T| t@) =~=
                old(out)@.map_values(|t: T| t@) + spec_avltreeseq_inorder(*link),
        decreases *link,
    {
        broadcast use Seq::<_>::lemma_push_map_commute;

        match link {
            None => {},
            Some(n) => {
                let ghost pre = out@;
                let ghost view_fn = |t: T| t@;

                push_inorder(&n.left, out);
                // post: out@.map_values(view_fn) =~= pre.map_values(view_fn) + inorder(n.left)

                let ghost after_left = out@;
                let cloned = n.value.clone();
                proof { assume(cloned@ == n.value@); }
                out.push(cloned);
                // Vec::push: out@ == after_left.push(cloned)
                // lemma_push_map_commute: after_left.push(cloned).map_values(f) =~= after_left.map_values(f).push(f(cloned))
                assert(out@.map_values(view_fn) =~=
                    pre.map_values(view_fn) + spec_avltreeseq_inorder(n.left) + seq![n.value@]);

                push_inorder(&n.right, out);
                // post: out@.map_values(view_fn) =~= after_val.map_values(view_fn) + inorder(n.right)
                // = pre.map_values(view_fn) + inorder(n.left) + seq![n.value@] + inorder(n.right)
                // = pre.map_values(view_fn) + inorder(Some(n))
            }
        }
    }

    fn compare_trees<T: StT>(a: &Link<T>, b: &Link<T>) -> (result: bool)
        ensures result == (spec_avltreeseq_inorder(*a) =~= spec_avltreeseq_inorder(*b)),
    {
        let sa = cached_size(a);
        let sb = cached_size(b);
        if sa != sb {
            proof { assume(false == (spec_avltreeseq_inorder(*a) =~= spec_avltreeseq_inorder(*b))); }
            return false;
        }
        let mut i: usize = 0;
        while i < sa
            decreases sa - i,
        {
            proof {
                assume(spec_avltreeseq_wf(*a));
                assume(spec_avltreeseq_wf(*b));
                assume((i as int) < spec_avltreeseq_inorder(*a).len());
                assume((i as int) < spec_avltreeseq_inorder(*b).len());
            }
            let ai = nth_link(a, i);
            let bi = nth_link(b, i);
            if !(*ai == *bi) {
                proof { assume(false == (spec_avltreeseq_inorder(*a) =~= spec_avltreeseq_inorder(*b))); }
                return false;
            }
            i += 1;
        }
        proof { assume(true == (spec_avltreeseq_inorder(*a) =~= spec_avltreeseq_inorder(*b))); }
        true
    }

    // 9. trait impl

    impl<T: StT> AVLTreeSeq<T> for AVLTreeS<T> {
        open spec fn spec_avltreeseq_seq(&self) -> Seq<T::V> {
            spec_avltreeseq_inorder(self.root)
        }

        open spec fn spec_avltreeseq_wf(&self) -> bool {
            spec_avltreeseq_wf(self.root)
        }

        fn empty() -> (result: Self) {
            AVLTreeS { root: None, next_key: 0 }
        }

        fn new() -> (result: Self) {
            Self::empty()
        }

        fn length(&self) -> (result: N) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            cached_size(&self.root)
        }

        fn nth(&self, index: N) -> (result: &T) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            nth_link(&self.root, index)
        }

        fn set(&mut self, index: N, item: T) -> (result: Result<(), &'static str>) {
            set_link(&mut self.root, index, item)
        }

        fn singleton(item: T) -> (result: Self) {
            let mut t = Self::empty();
            t.root = insert_at_link(t.root.take(), 0, item, &mut t.next_key);
            proof {
                assume(t.spec_avltreeseq_seq().len() == 1);
                assume(t.spec_avltreeseq_wf());
            }
            t
        }

        fn isEmpty(&self) -> (result: B) {
            self.length() == 0
        }

        fn isSingleton(&self) -> (result: B) {
            self.length() == 1
        }

        fn subseq_copy(&self, start: N, length: N) -> (result: Self) {
            assert(self.spec_avltreeseq_wf());
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
                    self.spec_avltreeseq_wf(),
                    n as int == self.spec_avltreeseq_seq().len(),
                    s <= i, i <= e, e <= n,
                decreases e - i,
            {
                vals.push(self.nth(i).clone());
                i += 1;
            }
            AVLTreeS::from_vec(vals)
        }

        fn new_root() -> (result: Self) {
            Self::empty()
        }

        fn update(&mut self, index: N, item: T) {
            assert(self.spec_avltreeseq_wf());
            assert((index as int) < self.spec_avltreeseq_seq().len());
            let _ = self.set(index, item);
        }

        fn from_vec(values: Vec<T>) -> (result: AVLTreeS<T>) {
            let length = values.len();
            let mut t = AVLTreeS::empty();
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    length == values@.len(),
                decreases length - i,
            {
                t.root = insert_at_link(t.root.take(), i, values[i].clone(), &mut t.next_key);
                i += 1;
            }
            proof {
                assume(t.spec_avltreeseq_wf());
            }
            t
        }

        fn to_arrayseq(&self) -> (result: ArraySeqStEphS<T>) {
            assert(self.spec_avltreeseq_wf());
            let vals = self.values_in_order();
            ArraySeqStEphS::from_vec(vals)
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

        fn push_back(&mut self, value: T) {
            assert(self.spec_avltreeseq_wf());
            let len = self.length();
            let node = insert_at_link(self.root.take(), len, value, &mut self.next_key);
            self.root = node;
        }

        fn contains_value(&self, target: &T) -> (result: B) {
            assert(self.spec_avltreeseq_wf());
            let n = self.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.spec_avltreeseq_wf(),
                    n as int == self.spec_avltreeseq_seq().len(),
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
            assert(self.spec_avltreeseq_wf());
            self.push_back(value);
        }

        fn delete_value(&mut self, target: &T) -> (result: bool) {
            assert(self.spec_avltreeseq_wf());
            let len = self.length();
            let mut found_index: Option<N> = None;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_avltreeseq_wf(),
                    len as int == self.spec_avltreeseq_seq().len(),
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
                        self.spec_avltreeseq_wf(),
                        len as int == self.spec_avltreeseq_seq().len(),
                        j <= idx, idx < len,
                    decreases idx - j,
                {
                    out_vec.push(self.nth(j).clone());
                    j += 1;
                }
                let mut k: usize = idx + 1;
                while k < len
                    invariant
                        self.spec_avltreeseq_wf(),
                        len as int == self.spec_avltreeseq_seq().len(),
                        k <= len, idx < len,
                    decreases len - k,
                {
                    out_vec.push(self.nth(k).clone());
                    k += 1;
                }
                *self = AVLTreeS::from_vec(out_vec);
                true
            } else {
                false
            }
        }

        fn is_tree_empty(&self) -> (result: bool) {
            assert(self.spec_avltreeseq_wf());
            self.length() == 0
        }

        fn values_in_order(&self) -> (result: Vec<T>) {
            assert(self.spec_avltreeseq_wf());
            let mut out: Vec<T> = Vec::new();
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
            proof { accept(r == (self@ == other@)); }
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
