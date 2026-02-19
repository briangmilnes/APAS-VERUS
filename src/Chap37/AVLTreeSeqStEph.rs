//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! StEphemeral (mutable) implicit-order AVL tree sequence.

pub mod AVLTreeSeqStEph {

    use std::fmt::Debug;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // Type declarations inside verus! so downstream modules can use them as struct fields.

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

    } // verus!

    impl<T: StT> Clone for AVLTreeNode<T> {
        fn clone(&self) -> Self {
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

    impl<T: StT> AVLTreeNode<T> {
        fn new(value: T, index: N) -> Self {
            AVLTreeNode {
                value,
                height: 1,
                left_size: 0,
                right_size: 0,
                left: None,
                right: None,
                index,
            }
        }
    }

    pub trait AVLTreeSeqStEphTrait<T: StT> {
        /// APAS: Work Θ(1), Span Θ(1).
        /// claude-4-sonet: Work Θ(1), Span Θ(1).
        fn empty()                                 -> Self;
        /// APAS: Work Θ(1), Span Θ(1).
        fn new()                                   -> Self;
        /// APAS: Work Θ(1), Span Θ(1).
        fn length(&self)                           -> N;
        /// APAS: Work Θ(lg(n)), Span Θ(lg(n)).
        fn nth(&self, index: N)                    -> &T;
        /// APAS: Work Θ(lg(n)), Span Θ(lg(n)).
        fn set(&mut self, index: N, item: T)       -> Result<&mut Self, &'static str>;
        /// APAS: Work Θ(1), Span Θ(1).
        fn singleton(item: T)                      -> Self;
        /// APAS: Work Θ(1), Span Θ(1).
        fn isEmpty(&self)                          -> B;
        /// APAS: Work Θ(1), Span Θ(1).
        fn isSingleton(&self)                      -> B;
        /// APAS: Work Θ(1 + lg(|a|)), Span Θ(1 + lg(|a|)).
        fn subseq_copy(&self, start: N, length: N) -> Self;
        fn new_root()                              -> Self;
        fn update(&mut self, item_at: (N, T))      -> &mut AVLTreeSeqStEphS<T>;
        fn from_vec(values: Vec<T>)                -> AVLTreeSeqStEphS<T>;
        fn to_arrayseq(&self)                      -> ArraySeqStEphS<T>;
        fn iter<'a>(&'a self)                      -> AVLTreeSeqIterStEph<'a, T>;
        fn push_back(&mut self, value: T);
        fn contains_value(&self, target: &T)       -> B;
        fn insert_value(&mut self, value: T);
        fn delete_value(&mut self, target: &T)     -> bool;
    }

    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {
        fn empty() -> Self { AVLTreeSeqStEphS::new_root() }

        fn new() -> Self { AVLTreeSeqStEphS::new_root() }

        fn length(&self) -> N { size_link(&self.root) }

        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            set_link(&mut self.root, index, item)?;
            Ok(self)
        }

        fn singleton(item: T) -> Self {
            let mut t = AVLTreeSeqStEphS::new_root();
            t.root = insert_at_link(t.root.take(), 0, item, &mut t.next_key);
            t
        }

        fn isEmpty(&self) -> B { self.length() == 0 }

        fn isSingleton(&self) -> B { self.length() == 1 }

        fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.length();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return AVLTreeSeqStEphS::empty();
            }
            let mut vals = Vec::<T>::with_capacity(e - s);
            for i in s..e {
                vals.push(self.nth(i).clone());
            }
            AVLTreeSeqStEphS::from_vec(vals)
        }

        fn new_root() -> Self {
            AVLTreeSeqStEphS {
                root: None,
                next_key: 0,
            }
        }

        fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeSeqStEphS<T> {
            let _ = self.set(index, item);
            self
        }

        fn from_vec(values: Vec<T>) -> AVLTreeSeqStEphS<T> {
            let length = values.len();
            let mut t = AVLTreeSeqStEphS::new_root();
            for (i, v) in values.into_iter().enumerate() {
                t.root = insert_at_link(t.root.take(), i, v, &mut t.next_key);
            }
            debug_assert!(t.length() == length);
            t
        }

        fn to_arrayseq(&self) -> ArraySeqStEphS<T> {
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

        fn iter<'a>(&'a self) -> AVLTreeSeqIterStEph<'a, T> { AVLTreeSeqIterStEph::new(&self.root) }

        fn push_back(&mut self, value: T) {
            let len = self.length();
            let node = insert_at_link(self.root.take(), len, value, &mut self.next_key);
            self.root = node;
        }

        fn contains_value(&self, target: &T) -> B {
            for v in self.iter() {
                if v == target {
                    return true;
                }
            }
            false
        }

        fn insert_value(&mut self, value: T) { self.push_back(value); }

        fn delete_value(&mut self, target: &T) -> bool {
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
                *self = AVLTreeSeqStEphS::from_vec(out_vec);
                true
            } else {
                false
            }
        }
    }

    pub struct AVLTreeSeqIterStEph<'a, T: StT> {
        stack: Vec<&'a AVLTreeNode<T>>,
        current: Option<&'a AVLTreeNode<T>>,
    }

    impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {
        fn new(root: &'a Link<T>) -> Self {
            let mut it = AVLTreeSeqIterStEph {
                stack: Vec::new(),
                current: None,
            };
            it.push_left(root);
            it
        }
        fn push_left(&mut self, link: &'a Link<T>) {
            let mut cursor = link;
            while let Some(node) = cursor.as_ref() {
                self.stack.push(node);
                cursor = &node.left;
            }
        }
    }

    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            let node = self.stack.pop()?;
            let value_ref: &T = &node.value;
            self.push_left(&node.right);
            Some(value_ref)
        }
    }

    fn h<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }

    fn size_link<T: StT>(n: &Link<T>) -> N {
        if let Some(b) = n {
            1 + b.left_size + b.right_size
        } else {
            0
        }
    }

    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>) {
        n.left_size = size_link(&n.left);
        n.right_size = size_link(&n.right);
        let hl = h(&n.left);
        let hr = h(&n.right);
        n.height = 1 + hl.max(hr);
    }

    fn rotate_right<T: StT>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        let mut x = y.left.take().expect("rotate_right requires left child");
        let t2 = x.right.take();
        y.left = t2;
        update_meta(&mut y);
        x.right = Some(y);
        update_meta(x.right.as_mut().unwrap());
        update_meta(&mut x);
        x
    }

    fn rotate_left<T: StT>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        let mut y = x.right.take().expect("rotate_left requires right child");
        let t2 = y.left.take();
        x.right = t2;
        update_meta(&mut x);
        y.left = Some(x);
        update_meta(y.left.as_mut().unwrap());
        update_meta(&mut y);
        y
    }

    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        update_meta(&mut n);
        let hl = h(&n.left);
        let hr = h(&n.right);
        if hl > hr.saturating_add(1) {
            if h(&n.left.as_ref().unwrap().right) > h(&n.left.as_ref().unwrap().left) {
                let left = n.left.take().unwrap();
                n.left = Some(rotate_left(left));
            }
            return rotate_right(n);
        }
        if hr > hl.saturating_add(1) {
            if h(&n.right.as_ref().unwrap().left) > h(&n.right.as_ref().unwrap().right) {
                let right = n.right.take().unwrap();
                n.right = Some(rotate_right(right));
            }
            return rotate_left(n);
        }
        n
    }

    pub(crate) fn insert_at_link<T: StT>(node: Link<T>, index: N, value: T, next_key: &mut N) -> Link<T> {
        match node {
            | None => {
                debug_assert!(index == 0, "insert_at_link reached None with index > 0");
                let key = *next_key;
                *next_key += 1;
                Some(Box::new(AVLTreeNode::new(value, key)))
            }
            | Some(mut n) => {
                let left_size = n.left_size;
                if index <= left_size {
                    n.left = insert_at_link(n.left.take(), index, value, next_key);
                } else {
                    n.right = insert_at_link(n.right.take(), index - left_size - 1, value, next_key);
                }
                Some(rebalance(n))
            }
        }
    }

    fn nth_link<T: StT>(node: &Link<T>, index: N) -> &T {
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

    fn set_link<T: StT>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {
        match node {
            | None => Err("Index out of bounds"),
            | Some(n) => {
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

    impl<T: StT> Default for AVLTreeSeqStEphS<T> {
        fn default() -> Self { Self::new() }
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

    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.length() != other.length() {
                return false;
            }
            for i in 0..self.length() {
                if self.nth(i) != other.nth(i) {
                    return false;
                }
            }
            true
        }
    }

    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}

    impl<T: StT> Clone for AVLTreeSeqStEphS<T> {
        fn clone(&self) -> Self {
            AVLTreeSeqStEphS {
                root: self.root.clone(),
                next_key: self.next_key,
            }
        }
    }
}
