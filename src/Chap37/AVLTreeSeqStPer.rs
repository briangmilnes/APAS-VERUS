//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! StPer (immutable, structurally shared) AVL tree sequence using Rc path-copying.

pub mod AVLTreeSeqStPer {

    use std::fmt::{Debug, Formatter};
    use std::rc::Rc;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Rc<Node<T>>>;

    struct Node<T: StT> {
        value: T,
        height: N,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn height<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }
    fn size<T: StT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }

    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> Rc<Node<T>> {
        let hl = height(&left);
        let hr = height(&right);
        let sz = 1 + size(&left) + size(&right);
        Rc::new(Node {
            value,
            height: 1 + hl.max(hr),
            size: sz,
            left,
            right,
        })
    }

    fn rotate_right<T: StT>(y: Rc<Node<T>>) -> Rc<Node<T>> {
        let x = y.left.as_ref().expect("rotate_right requires left").clone();
        let t2 = x.right.clone();
        let new_y = mk(y.value.clone(), t2.clone(), y.right.clone());
        mk(x.value.clone(), x.left.clone(), Some(new_y))
    }

    fn rotate_left<T: StT>(x: Rc<Node<T>>) -> Rc<Node<T>> {
        let y = x.right.as_ref().expect("rotate_left requires right").clone();
        let t2 = y.left.clone();
        let new_x = mk(x.value.clone(), x.left.clone(), t2.clone());
        mk(y.value.clone(), Some(new_x), y.right.clone())
    }

    fn rebalance<T: StT>(n: Rc<Node<T>>) -> Rc<Node<T>> {
        let hl = height(&n.left);
        let hr = height(&n.right);
        if hl > hr.saturating_add(1) {
            let left = n.left.as_ref().unwrap().clone();
            if height(&left.right) > height(&left.left) {
                let rotated = rotate_left(left);
                return rotate_right(mk(n.value.clone(), Some(rotated), n.right.clone()));
            }
            return rotate_right(n);
        }
        if hr > hl.saturating_add(1) {
            let right = n.right.as_ref().unwrap().clone();
            if height(&right.left) > height(&right.right) {
                let rotated = rotate_right(right);
                return rotate_left(mk(n.value.clone(), n.left.clone(), Some(rotated)));
            }
            return rotate_left(n);
        }
        n
    }

    fn nth_ref<T: StT>(mut cur: &Link<T>, mut index: N) -> &T {
        loop {
            let node = cur.as_ref().expect("index out of bounds");
            let ls = size(&node.left);
            if index < ls {
                cur = &node.left;
            } else if index == ls {
                return &node.value;
            } else {
                index -= ls + 1;
                cur = &node.right;
            }
        }
    }

    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {
        match cur {
            | None => {
                // Allow setting at index 0 on empty tree (append to empty)
                if index == 0 {
                    Ok(Some(mk(value, None, None)))
                } else {
                    Err("Index out of bounds")
                }
            }
            | Some(n) => {
                let ls = size(&n.left);
                if index < ls {
                    let new_left = set_rec(&n.left, index, value)?;
                    Ok(Some(rebalance(mk(n.value.clone(), new_left, n.right.clone()))))
                } else if index == ls {
                    Ok(Some(mk(value, n.left.clone(), n.right.clone())))
                } else {
                    let new_right = set_rec(&n.right, index - ls - 1, value)?;
                    Ok(Some(rebalance(mk(n.value.clone(), n.left.clone(), new_right))))
                }
            }
        }
    }

    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>) {
        if let Some(n) = cur {
            inorder_collect(&n.left, out);
            out.push(n.value.clone());
            inorder_collect(&n.right, out);
        }
    }

    fn build_balanced_from_slice<T: StT>(a: &[T]) -> Link<T> {
        fn rec<T: StT>(a: &[T]) -> Link<T> {
            if a.is_empty() {
                return None;
            }
            let mid = a.len() / 2;
            let left = rec(&a[..mid]);
            let right = rec(&a[mid + 1..]);
            Some(mk(a[mid].clone(), left, right))
        }
        rec(a)
    }

    pub struct AVLTreeSeqStPerS<T: StT> {
        root: Link<T>,
    }

    pub trait AVLTreeSeqStPerTrait<T: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                 -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn new()                                   -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn length(&self)                           -> N;
        /// APAS: Work Θ(lg(n)), Span Θ(lg(n))
        fn nth(&self, index: N)                    -> &T;
        /// APAS (ephemeral set Θ(lg n)); StPer path-copy Θ(lg n) allocations. Work Θ(lg n), Span Θ(lg n)
        fn set(&self, index: N, item: T)           -> Result<Self, &'static str>
        where
            Self: Sized;
        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(item: T)                      -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn isEmpty(&self)                          -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        fn isSingleton(&self)                      -> B;
        /// APAS: Work Θ(1 + lg|a|), Span Θ(1 + lg|a|)
        fn subseq_copy(&self, start: N, length: N) -> Self;
        /// Build balanced tree from values in in-order order.
        fn from_vec(values: Vec<T>)                -> Self;
        /// Collect in-order values to Vec.
        fn values_in_order(&self)                  -> Vec<T>;
        fn to_arrayseq(&self)                      -> ArraySeqStPerS<T>;
        fn iter<'a>(&'a self)                      -> AVLTreeSeqStPerIter<'a, T>;
    }

    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {
        fn empty() -> Self { AVLTreeSeqStPerS { root: None } }
        fn new() -> Self { Self::empty() }
        fn length(&self) -> N { size(&self.root) }
        fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {
            Ok(AVLTreeSeqStPerS {
                root: set_rec(&self.root, index, item)?,
            })
        }
        fn singleton(item: T) -> Self {
            AVLTreeSeqStPerS {
                root: Some(mk(item, None, None)),
            }
        }
        fn isEmpty(&self) -> B { self.length() == 0 }
        fn isSingleton(&self) -> B { self.length() == 1 }
        fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.length();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return Self::empty();
            }
            let mut vals = Vec::<T>::with_capacity(e - s);
            let all = self.values_in_order();
            vals.extend(all[s..e].iter().cloned());
            Self::from_vec(vals)
        }
        fn from_vec(values: Vec<T>) -> Self {
            AVLTreeSeqStPerS {
                root: build_balanced_from_slice(&values[..]),
            }
        }
        fn values_in_order(&self) -> Vec<T> {
            let mut out = Vec::with_capacity(self.length());
            inorder_collect(&self.root, &mut out);
            out
        }

        fn to_arrayseq(&self) -> ArraySeqStPerS<T> {
            let v = self.values_in_order();
            ArraySeqStPerS::from_vec(v)
        }

        fn iter<'a>(&'a self) -> AVLTreeSeqStPerIter<'a, T> {
            AVLTreeSeqStPerIter {
                stack: Vec::new(),
                current: self.root.as_deref(),
            }
        }
    }

    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {
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
    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}

    impl<T: StT> Clone for AVLTreeSeqStPerS<T> {
        fn clone(&self) -> Self {
            AVLTreeSeqStPerS {
                root: self.root.clone(),
            }
        }
    }

    impl<T: StT> Debug for AVLTreeSeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let v = self.values_in_order();
            f.debug_list().entries(v.iter()).finish()
        }
    }

    pub struct AVLTreeSeqStPerIter<'a, T: StT> {
        stack: Vec<&'a Node<T>>,
        current: Option<&'a Node<T>>,
    }

    impl<'a, T: StT> AVLTreeSeqStPerIter<'a, T> {
        fn push_left(&mut self, mut cur: Option<&'a Node<T>>) {
            while let Some(n) = cur {
                self.stack.push(n);
                cur = n.left.as_deref();
            }
        }
    }

    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.current.is_some() {
                let cur = self.current.take();
                self.push_left(cur);
            }
            let node = self.stack.pop()?;
            let value_ref: &T = &node.value;
            self.push_left(node.right.as_deref());
            Some(value_ref)
        }
    }
}

#[macro_export]
macro_rules! AVLTreeSeqStPerLit {
    () => { < $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS<_> as
              $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait<_> >::empty() };
    ($x:expr; $n:expr) => {{
        let __vals = vec![$x; $n];
        < $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS<_> as
          $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait<_> >::from_vec(__vals)
    }};
    ($($x:expr),* $(,)?) => {{
        let __vals = vec![$($x),*];
        < $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS<_> as
          $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait<_> >::from_vec(__vals)
    }};
}
