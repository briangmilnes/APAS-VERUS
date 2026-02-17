//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! MtPer (immutable, thread-safe, structurally shared) AVL tree sequence using Arc path-copying.

pub mod AVLTreeSeqMtPer {

    use std::fmt::{Debug, Formatter};
    use std::sync::Arc;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Arc<Node<T>>>;

    struct Node<T: StTInMtT> {
        value: T,
        height: N,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn height<T: StTInMtT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.height) }
    fn size<T: StTInMtT>(n: &Link<T>) -> N { n.as_ref().map_or(0, |r| r.size) }

    fn mk<T: StTInMtT>(value: T, left: Link<T>, right: Link<T>) -> Arc<Node<T>> {
        let hl = height(&left);
        let hr = height(&right);
        let sz = 1 + size(&left) + size(&right);
        Arc::new(Node {
            value,
            height: 1 + hl.max(hr),
            size: sz,
            left,
            right,
        })
    }

    fn rotate_right<T: StTInMtT>(y: Arc<Node<T>>) -> Arc<Node<T>> {
        let x = y.left.as_ref().expect("rotate_right requires left").clone();
        let t2 = x.right.clone();
        let new_y = mk(y.value.clone(), t2.clone(), y.right.clone());
        mk(x.value.clone(), x.left.clone(), Some(new_y))
    }

    fn rotate_left<T: StTInMtT>(x: Arc<Node<T>>) -> Arc<Node<T>> {
        let y = x.right.as_ref().expect("rotate_left requires right").clone();
        let t2 = y.left.clone();
        let new_x = mk(x.value.clone(), x.left.clone(), t2.clone());
        mk(y.value.clone(), Some(new_x), y.right.clone())
    }

    fn rebalance<T: StTInMtT>(n: Arc<Node<T>>) -> Arc<Node<T>> {
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

    fn nth_ref<T: StTInMtT>(mut cur: &Link<T>, mut index: N) -> &T {
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

    fn set_rec<T: StTInMtT>(cur: &Link<T>, index: N, value: T) -> Result<Link<T>, &'static str> {
        match cur {
            | None => {
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

    fn inorder_collect<T: StTInMtT>(cur: &Link<T>, out: &mut Vec<T>) {
        if let Some(n) = cur {
            inorder_collect(&n.left, out);
            out.push(n.value.clone());
            inorder_collect(&n.right, out);
        }
    }

    fn build_balanced_from_slice<T: StTInMtT>(a: &[T]) -> Link<T> {
        fn rec<T: StTInMtT>(a: &[T]) -> Link<T> {
            if a.is_empty() {
                return None;
            }
            let mid = a.len() / 2;
            
            // Parallel construction of left and right subtrees
            let Pair(left, right) = crate::ParaPair!(
                move || rec(&a[..mid]),
                move || rec(&a[mid + 1..])
            );
            
            Some(mk(a[mid].clone(), left, right))
        }
        rec(a)
    }

    pub struct AVLTreeSeqMtPerS<T: StTInMtT> {
        root: Link<T>,
    }

    pub trait AVLTreeSeqMtPerTrait<T: StTInMtT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                 -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn new()                                   -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn length(&self)                           -> N;
        /// APAS: Work Θ(lg(n)), Span Θ(lg(n))
        fn nth(&self, index: N)                    -> &T;
        /// APAS (ephemeral set Θ(lg n)); MtPer path-copy Θ(lg n) allocations. Work Θ(lg n), Span Θ(lg n)
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
    }

    impl<T: StTInMtT> AVLTreeSeqMtPerTrait<T> for AVLTreeSeqMtPerS<T> {
        fn empty() -> Self { AVLTreeSeqMtPerS { root: None } }
        fn new() -> Self { Self::empty() }
        fn length(&self) -> N { size(&self.root) }
        fn nth(&self, index: N) -> &T { nth_ref(&self.root, index) }
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {
            Ok(AVLTreeSeqMtPerS {
                root: set_rec(&self.root, index, item)?,
            })
        }
        fn singleton(item: T) -> Self {
            AVLTreeSeqMtPerS {
                root: Some(mk(item, None, None)),
            }
        }
        fn isEmpty(&self) -> B { self.length() == 0 }
        fn isSingleton(&self) -> B { self.length() == 1 }
        fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.length();
            let s = start.min(n);
            let e = (start + length).min(n);
            if s >= e {
                return Self::empty();
            }
            
            // Parallel extraction using thread spawning with Arc for sharing
            use std::sync::Arc;
            
            let result_len = e - s;
            
            // Create array of Mutexes for safe concurrent writes
            let slots: Vec<_> = (0..result_len)
                .map(|_| Arc::new(std::sync::Mutex::new(None)))
                .collect();
            
            std::thread::scope(|scope| {
                for i in s..e {
                    let self_ref = self;
                    let idx = i - s;
                    let slot = Arc::clone(&slots[idx]);
                    
                    scope.spawn(move || {
                        let value = self_ref.nth(i).clone();
                        *slot.lock().unwrap() = Some(value);
                    });
                }
            });
            
            // All threads joined, extract values
            let vals: Vec<T> = slots
                .into_iter()
                .map(|slot| Arc::try_unwrap(slot).unwrap().into_inner().unwrap().unwrap())
                .collect();
            
            Self::from_vec(vals)
        }
        fn from_vec(values: Vec<T>) -> Self {
            AVLTreeSeqMtPerS {
                root: build_balanced_from_slice(&values),
            }
        }
        fn values_in_order(&self) -> Vec<T> {
            let mut out = Vec::new();
            inorder_collect(&self.root, &mut out);
            out
        }
    }

    impl<T: StTInMtT> Clone for AVLTreeSeqMtPerS<T> {
        fn clone(&self) -> Self {
            AVLTreeSeqMtPerS {
                root: self.root.clone(),
            }
        }
    }

    impl<T: StTInMtT> Default for AVLTreeSeqMtPerS<T> {
        fn default() -> Self { Self::empty() }
    }

    // Iterator implementation
    pub struct AVLTreeSeqMtPerIter<T: StTInMtT> {
        values: Vec<T>,
        index: usize,
    }

    impl<T: StTInMtT> Iterator for AVLTreeSeqMtPerIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let val = self.values[self.index].clone();
                self.index += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    impl<T: StTInMtT> IntoIterator for AVLTreeSeqMtPerS<T> {
        type Item = T;
        type IntoIter = AVLTreeSeqMtPerIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            AVLTreeSeqMtPerIter {
                values: self.values_in_order(),
                index: 0,
            }
        }
    }

    impl<T: StTInMtT> PartialEq for AVLTreeSeqMtPerS<T> {
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

    impl<T: StTInMtT> Eq for AVLTreeSeqMtPerS<T> {}

    impl<T: StTInMtT> Debug for AVLTreeSeqMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            for i in 0..self.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.nth(i))?;
            }
            write!(f, "]")
        }
    }
}
