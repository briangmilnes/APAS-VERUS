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

    /// Under well-formedness, cached size equals in-order sequence length.
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

    pub trait AVLTreeSeqStEphTrait<T: StT>: Sized {
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

        fn from_vec(values: Vec<T>) -> (result: AVLTreeSeqStEphS<T>);

        fn to_arrayseq(&self) -> (result: ArraySeqStEphS<T>);

        fn iter<'a>(&'a self) -> (result: AVLTreeSeqIterStEph<'a, T>);

        fn push_back(&mut self, value: T);

        fn contains_value(&self, target: &T) -> (result: B);

        fn insert_value(&mut self, value: T);

        fn delete_value(&mut self, target: &T) -> (result: bool);
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

    fn size_link_fn<T: StT>(n: &Link<T>) -> (result: N)
        ensures result as nat == spec_cached_size(n),
    {
        match n {
            None => 0,
            Some(b) => {
                proof { assume(1 + b.left_size + b.right_size < usize::MAX); }
                1 + b.left_size + b.right_size
            }
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

    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {
        open spec fn spec_seq(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }

        open spec fn spec_well_formed(&self) -> bool {
            spec_wf(self.root)
        }

        fn empty() -> (result: Self) {
            AVLTreeSeqStEphS { root: None, next_key: 0 }
        }

        fn new() -> (result: Self) {
            Self::empty()
        }

        fn length(&self) -> (result: N) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            size_link_fn(&self.root)
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
                assume(t.spec_seq().len() == 1);
                assume(t.spec_well_formed());
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
            proof { assume(self.spec_well_formed()); }
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

        fn new_root() -> (result: Self) {
            Self::empty()
        }

        fn update(&mut self, index: N, item: T) {
            proof {
                assume(self.spec_well_formed());
                assume((index as int) < self.spec_seq().len());
            }
            let _ = self.set(index, item);
        }

        fn from_vec(values: Vec<T>) -> (result: AVLTreeSeqStEphS<T>) {
            let length = values.len();
            let mut t = AVLTreeSeqStEphS::empty();
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
            proof { assume(t.spec_well_formed()); }
            t
        }

        fn to_arrayseq(&self) -> (result: ArraySeqStEphS<T>) {
            proof { assume(self.spec_well_formed()); }
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
        fn iter<'a>(&'a self) -> (result: AVLTreeSeqIterStEph<'a, T>) {
            let mut it = AVLTreeSeqIterStEph {
                stack: Vec::new(),
                current: None,
            };
            push_left_iter(&mut it, &self.root);
            it
        }

        fn push_back(&mut self, value: T) {
            proof { assume(self.spec_well_formed()); }
            let len = self.length();
            let node = insert_at_link(self.root.take(), len, value, &mut self.next_key);
            self.root = node;
        }

        fn contains_value(&self, target: &T) -> (result: B) {
            proof { assume(self.spec_well_formed()); }
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
            proof { assume(self.spec_well_formed()); }
            self.push_back(value);
        }

        fn delete_value(&mut self, target: &T) -> (result: bool) {
            proof { assume(self.spec_well_formed()); }
            let len = self.length();
            let mut found_index: Option<N> = None;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_well_formed(),
                    len as int == self.spec_seq().len(),
                    i <= len,
                decreases len - i,
            {
                if *self.nth(i) == *target {
                    found_index = Some(i);
                    break;
                }
                i += 1;
            }
            if let Some(idx) = found_index {
                proof { assume(idx < len); }
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
    impl<T: StT> PartialEqSpecImpl for AVLTreeSeqStEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}

    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let r = compare_trees(&self.root, &other.root);
            proof { assume(r == (self@ == other@)); }
            r
        }
    }

    impl<T: StT> Clone for AVLTreeSeqStEphS<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@,
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
