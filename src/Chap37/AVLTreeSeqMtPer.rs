//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! MtPer (immutable, thread-safe, structurally shared) AVL tree sequence using Arc path-copying.

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

pub mod AVLTreeSeqMtPer {

    use std::sync::Arc;
    use std::fmt::{Debug, Formatter};

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // 2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::vstdplus::accept::accept;

    // 3. broadcast use

    broadcast use vstd::seq::group_seq_axioms;

    // 4. type definitions

    pub type Link<T> = Option<Arc<Node<T>>>;

    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StTInMtT> {
        pub value: T,
        pub height: N,
        pub size: N,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqMtPerS<T: StTInMtT> {
        pub root: Link<T>,
    }

    // 5. view impls

    impl<T: StTInMtT> View for AVLTreeSeqMtPerS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }
    }

    // 6. spec fns

    /// In-order traversal of the tree as a sequence of element views.
    pub open spec fn spec_inorder<T: StTInMtT>(link: Link<T>) -> Seq<T::V>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => spec_inorder(node.left) + seq![node.value@] + spec_inorder(node.right),
        }
    }

    pub open spec fn spec_cached_height<T: StTInMtT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.height as nat,
        }
    }

    pub open spec fn spec_cached_size<T: StTInMtT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_nat_max(a: nat, b: nat) -> nat {
        if a >= b { a } else { b }
    }

    /// Well-formedness: cached height and size match the actual tree structure.
    pub open spec fn spec_wf<T: StTInMtT>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_wf(node.left)
                && spec_wf(node.right)
                && node.height as nat == 1 + spec_nat_max(
                    spec_cached_height(&node.left),
                    spec_cached_height(&node.right),
                )
                && node.size as nat == 1 + spec_cached_size(&node.left)
                    + spec_cached_size(&node.right)
            }
        }
    }

    // 7. proof fns

    /// Under well-formedness, cached size equals in-order sequence length.
    proof fn lemma_size_eq_inorder_len<T: StTInMtT>(link: &Link<T>)
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

    pub trait AVLTreeSeqMtPerTrait<T: StTInMtT>: Sized {
        spec fn spec_seq(&self) -> Seq<T::V>;
        spec fn spec_well_formed(&self) -> bool;

        fn empty() -> (result: Self)
            ensures result.spec_seq() =~= Seq::<T::V>::empty(), result.spec_well_formed();

        fn new() -> (result: Self)
            ensures result.spec_seq() =~= Seq::<T::V>::empty(), result.spec_well_formed();

        fn singleton(item: T) -> (result: Self)
            ensures result.spec_seq() =~= seq![item@], result.spec_well_formed();

        fn length(&self) -> (result: N)
            requires self.spec_well_formed(),
            ensures result as nat == self.spec_seq().len();

        fn nth(&self, index: N) -> (result: &T)
            requires self.spec_well_formed(), (index as int) < self.spec_seq().len(),
            ensures result@ == self.spec_seq()[index as int];

        fn isEmpty(&self) -> (result: B)
            requires self.spec_well_formed(),
            ensures result == (self.spec_seq().len() == 0);

        fn isSingleton(&self) -> (result: B)
            requires self.spec_well_formed(),
            ensures result == (self.spec_seq().len() == 1);

        fn set(&self, index: N, item: T) -> (result: Result<Self, &'static str>)
            requires self.spec_well_formed(), (index as int) < self.spec_seq().len();

        fn subseq_copy(&self, start: N, length: N) -> (result: Self)
            requires self.spec_well_formed();

        fn from_vec(values: Vec<T>) -> (result: Self);

        fn values_in_order(&self) -> (result: Vec<T>);
    }

    // 9. impls

    fn height_fn<T: StTInMtT>(n: &Link<T>) -> (result: N)
        ensures result as nat == spec_cached_height(n),
    {
        match n {
            None => 0,
            Some(node) => node.height,
        }
    }

    fn size_fn<T: StTInMtT>(n: &Link<T>) -> (result: N)
        ensures result as nat == spec_cached_size(n),
    {
        match n {
            None => 0,
            Some(node) => node.size,
        }
    }

    #[verifier::external_body]
    fn mk<T: StTInMtT>(value: T, left: Link<T>, right: Link<T>) -> (result: Arc<Node<T>>)
        requires
            1 + spec_cached_size(&left) + spec_cached_size(&right) <= N::MAX as nat,
            1 + spec_nat_max(spec_cached_height(&left), spec_cached_height(&right)) <= N::MAX as nat,
        ensures
            spec_inorder(Some(result)) =~= spec_inorder(left) + seq![value@] + spec_inorder(right),
            result.size as nat == 1 + spec_cached_size(&left) + spec_cached_size(&right),
            result.height as nat == 1 + spec_nat_max(
                spec_cached_height(&left), spec_cached_height(&right)),
            spec_wf(left) && spec_wf(right) ==> spec_wf(Some(result)),
    {
        let hl = height_fn(&left);
        let hr = height_fn(&right);
        let sz = 1 + size_fn(&left) + size_fn(&right);
        let h = 1 + if hl >= hr { hl } else { hr };
        Arc::new(Node { value, height: h, size: sz, left, right })
    }

    #[verifier::external_body]
    fn rotate_right<T: StTInMtT>(y: Arc<Node<T>>) -> (result: Arc<Node<T>>)
        requires y.left.is_some(), spec_wf(Some(y)),
        ensures
            spec_inorder(Some(result)) =~= spec_inorder(Some(y)),
            spec_wf(Some(result)),
    {
        let x = y.left.as_ref().unwrap().clone();
        let t2 = x.right.clone();
        let new_y = mk(y.value.clone(), t2, y.right.clone());
        mk(x.value.clone(), x.left.clone(), Some(new_y))
    }

    #[verifier::external_body]
    fn rotate_left<T: StTInMtT>(x: Arc<Node<T>>) -> (result: Arc<Node<T>>)
        requires x.right.is_some(), spec_wf(Some(x)),
        ensures
            spec_inorder(Some(result)) =~= spec_inorder(Some(x)),
            spec_wf(Some(result)),
    {
        let y = x.right.as_ref().unwrap().clone();
        let t2 = y.left.clone();
        let new_x = mk(x.value.clone(), x.left.clone(), t2);
        mk(y.value.clone(), Some(new_x), y.right.clone())
    }

    #[verifier::external_body]
    fn rebalance<T: StTInMtT>(n: Arc<Node<T>>) -> (result: Arc<Node<T>>)
        requires spec_wf(Some(n)),
        ensures
            spec_inorder(Some(result)) =~= spec_inorder(Some(n)),
            spec_wf(Some(result)),
    {
        let hl = height_fn(&n.left);
        let hr = height_fn(&n.right);
        if hl > hr.saturating_add(1) {
            let left = n.left.as_ref().unwrap().clone();
            if height_fn(&left.right) > height_fn(&left.left) {
                let rotated = rotate_left(left);
                return rotate_right(mk(n.value.clone(), Some(rotated), n.right.clone()));
            }
            return rotate_right(n);
        }
        if hr > hl.saturating_add(1) {
            let right = n.right.as_ref().unwrap().clone();
            if height_fn(&right.left) > height_fn(&right.right) {
                let rotated = rotate_right(right);
                return rotate_left(mk(n.value.clone(), n.left.clone(), Some(rotated)));
            }
            return rotate_left(n);
        }
        n
    }

    #[verifier::external_body]
    fn nth_ref<'a, T: StTInMtT>(cur: &'a Link<T>, index: N) -> (result: &'a T)
        requires spec_wf(*cur), (index as int) < spec_inorder(*cur).len(),
        ensures result@ == spec_inorder(*cur)[index as int],
    {
        let mut cur = cur;
        let mut index = index;
        loop {
            let node = cur.as_ref().unwrap();
            let ls = size_fn(&node.left);
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

    #[verifier::external_body]
    fn set_rec<T: StTInMtT>(cur: &Link<T>, index: N, value: T) -> (result: Result<Link<T>, &'static str>)
        requires spec_wf(*cur), (index as int) < spec_inorder(*cur).len(),
        ensures result.is_ok(),
    {
        match cur {
            None => {
                if index == 0 {
                    Ok(Some(mk(value, None, None)))
                } else {
                    Err("Index out of bounds")
                }
            }
            Some(n) => {
                let ls = size_fn(&n.left);
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

    #[verifier::external_body]
    fn inorder_collect<T: StTInMtT>(cur: &Link<T>, out: &mut Vec<T>) {
        if let Some(n) = cur {
            inorder_collect(&n.left, out);
            out.push(n.value.clone());
            inorder_collect(&n.right, out);
        }
    }

    #[verifier::external_body]
    fn build_balanced_from_slice<T: StTInMtT>(a: &[T]) -> (result: Link<T>)
        ensures spec_wf(result),
    {
        fn rec<T: StTInMtT>(a: &[T]) -> Link<T> {
            if a.is_empty() {
                return None;
            }
            let mid = a.len() / 2;
            let crate::Types::Types::Pair(left, right) = crate::ParaPair!(
                move || rec(&a[..mid]),
                move || rec(&a[mid + 1..])
            );
            Some(mk(a[mid].clone(), left, right))
        }
        rec(a)
    }

    #[verifier::external_body]
    fn compare_trees<T: StTInMtT>(a: &Link<T>, b: &Link<T>) -> (result: bool) {
        let sa = size_fn(a);
        let sb = size_fn(b);
        if sa != sb { return false; }
        for i in 0..sa {
            if nth_ref(a, i) != nth_ref(b, i) {
                return false;
            }
        }
        true
    }

    // 9. trait impl

    impl<T: StTInMtT> AVLTreeSeqMtPerTrait<T> for AVLTreeSeqMtPerS<T> {
        open spec fn spec_seq(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }

        open spec fn spec_well_formed(&self) -> bool {
            spec_wf(self.root)
        }

        fn empty() -> (result: Self) {
            AVLTreeSeqMtPerS { root: None }
        }

        fn new() -> (result: Self) {
            Self::empty()
        }

        fn singleton(item: T) -> (result: Self) {
            AVLTreeSeqMtPerS {
                root: Some(mk(item, None, None)),
            }
        }

        fn length(&self) -> (result: N) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            size_fn(&self.root)
        }

        fn nth(&self, index: N) -> (result: &T) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            nth_ref(&self.root, index)
        }

        fn isEmpty(&self) -> (result: B) {
            self.length() == 0
        }

        fn isSingleton(&self) -> (result: B) {
            self.length() == 1
        }

        fn set(&self, index: N, item: T) -> (result: Result<Self, &'static str>) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            Ok(AVLTreeSeqMtPerS {
                root: set_rec(&self.root, index, item)?,
            })
        }

        #[verifier::external_body]
        fn subseq_copy(&self, start: N, length: N) -> (result: Self) {
            let n = self.length();
            let s = start.min(n);
            let e = (start + length).min(n);
            if s >= e {
                return Self::empty();
            }
            use std::sync::Arc;
            let result_len = e - s;
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
            let vals: Vec<T> = slots
                .into_iter()
                .map(|slot| Arc::try_unwrap(slot).unwrap().into_inner().unwrap().unwrap())
                .collect();
            Self::from_vec(vals)
        }

        fn from_vec(values: Vec<T>) -> (result: Self) {
            AVLTreeSeqMtPerS {
                root: build_balanced_from_slice(&values),
            }
        }

        fn values_in_order(&self) -> (result: Vec<T>) {
            proof { assume(self.spec_well_formed()); }
            let mut out: Vec<T> = Vec::new();
            inorder_collect(&self.root, &mut out);
            out
        }
    }

    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<T: StTInMtT> PartialEqSpecImpl for AVLTreeSeqMtPerS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StTInMtT> Eq for AVLTreeSeqMtPerS<T> {}

    impl<T: StTInMtT> PartialEq for AVLTreeSeqMtPerS<T> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let r = compare_trees(&self.root, &other.root);
            proof { accept(r == (self@ == other@)); }
            r
        }
    }

    impl<T: StTInMtT> Clone for AVLTreeSeqMtPerS<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@,
        {
            AVLTreeSeqMtPerS {
                root: self.root.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StTInMtT> Default for AVLTreeSeqMtPerS<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StTInMtT> Debug for AVLTreeSeqMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let n = size_fn(&self.root);
            write!(f, "[")?;
            for i in 0..n {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", nth_ref(&self.root, i))?;
            }
            write!(f, "]")
        }
    }

    // Iterator (outside verus! — consuming iterator over collected values)

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
}

#[macro_export]
macro_rules! AVLTreeSeqMtPerLit {
    () => { < $crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerS<_> as
              $crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait<_> >::empty() };
    ($x:expr; $n:expr) => {{
        let __vals = vec![$x; $n];
        < $crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerS<_> as
          $crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait<_> >::from_vec(__vals)
    }};
    ($($x:expr),* $(,)?) => {{
        let __vals = vec![$($x),*];
        < $crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerS<_> as
          $crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait<_> >::from_vec(__vals)
    }};
}
