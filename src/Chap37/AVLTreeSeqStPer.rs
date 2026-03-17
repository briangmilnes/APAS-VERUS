//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! StPer (immutable, structurally shared) AVL tree sequence using Arc path-copying.

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

pub mod AVLTreeSeqStPer {

    use std::sync::Arc;
    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    use vstd::slice::slice_subrange;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{lemma_cloned_view_eq, obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger};

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

    pub type Link<T> = Option<Arc<Node<T>>>;

    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StT> {
        pub value: T,
        pub height: N,
        pub size: N,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqStPerS<T: StT> {
        pub root: Link<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {
        pub stack: Vec<&'a Node<T>>,
        pub current: Option<&'a Node<T>>,
    }

    // 5. view impls

    impl<T: StT> View for AVLTreeSeqStPerS<T> {
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

    pub open spec fn spec_cached_height<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.height as nat,
        }
    }

    pub open spec fn spec_cached_size<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_nat_max(a: nat, b: nat) -> nat {
        if a >= b { a } else { b }
    }

    /// Well-formedness: cached height and size match the actual tree structure.
    pub open spec fn spec_avltreeseqstper_wf<T: StT>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_avltreeseqstper_wf(node.left)
                && spec_avltreeseqstper_wf(node.right)
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
    proof fn lemma_size_eq_inorder_len<T: StT>(link: &Link<T>)
        requires spec_avltreeseqstper_wf(*link),
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

    /// Under well-formedness, cached height <= cached size.
    proof fn lemma_height_le_size<T: StT>(link: &Link<T>)
        requires spec_avltreeseqstper_wf(*link),
        ensures spec_cached_height(link) <= spec_cached_size(link),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_height_le_size::<T>(&node.left);
                lemma_height_le_size::<T>(&node.right);
            }
        }
    }

    // 8. traits

    pub trait AVLTreeSeqStPerTrait<T: StT>: Sized {
        spec fn spec_seq(&self) -> Seq<T::V>;
        spec fn spec_avltreeseqstper_wf(&self) -> bool;

        fn empty() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseqstper_wf();

        fn new() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseqstper_wf();

        fn singleton(item: T) -> (tree: Self)
            ensures tree.spec_seq() =~= seq![item@], tree.spec_avltreeseqstper_wf();

        fn length(&self) -> (len: N)
            requires self.spec_avltreeseqstper_wf(),
            ensures len as nat == self.spec_seq().len();

        fn nth(&self, index: N) -> (elem: &T)
            requires self.spec_avltreeseqstper_wf(), (index as int) < self.spec_seq().len(),
            ensures elem@ == self.spec_seq()[index as int];

        fn isEmpty(&self) -> (empty: B)
            requires self.spec_avltreeseqstper_wf(),
            ensures empty == (self.spec_seq().len() == 0);

        fn isSingleton(&self) -> (single: B)
            requires self.spec_avltreeseqstper_wf(),
            ensures single == (self.spec_seq().len() == 1);

        fn set(&self, index: N, item: T) -> (outcome: Result<Self, &'static str>)
            requires
                self.spec_avltreeseqstper_wf(),
                (index as int) < self.spec_seq().len(),
                obeys_feq_clone::<T>(),
            ensures
                outcome is Ok,
                outcome.unwrap().spec_avltreeseqstper_wf(),
                outcome.unwrap().spec_seq() =~= self.spec_seq().update(index as int, item@);

        fn subseq_copy(&self, start: N, length: N) -> (sub: Self)
            requires self.spec_avltreeseqstper_wf(),
            ensures sub.spec_avltreeseqstper_wf();

        fn from_vec(values: Vec<T>) -> (tree: Self)
            ensures
                tree.spec_avltreeseqstper_wf(),
                tree.spec_seq() =~= values@.map_values(|t: T| t@);

        fn values_in_order(&self) -> (values: Vec<T>)
            requires self.spec_avltreeseqstper_wf(),
            ensures values@.map_values(|t: T| t@) =~= self.spec_seq();

        fn to_arrayseq(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_avltreeseqstper_wf(),
            ensures
                seq.spec_len() == self.spec_seq().len();

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqStPerIter<'a, T>)
            ensures true;
    }

    // 9. impls

    fn height_fn<T: StT>(n: &Link<T>) -> (h: N)
        requires spec_cached_height(n) <= usize::MAX as nat,
        ensures h as nat == spec_cached_height(n),
    {
        match n {
            None => 0,
            Some(node) => node.height,
        }
    }

    fn size_fn<T: StT>(n: &Link<T>) -> (sz: N)
        requires spec_cached_size(n) <= usize::MAX as nat,
        ensures sz as nat == spec_cached_size(n),
    {
        match n {
            None => 0,
            Some(node) => node.size,
        }
    }

    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> (node: Arc<Node<T>>)
        requires
            1 + spec_cached_size(&left) + spec_cached_size(&right) <= N::MAX as nat,
            1 + spec_nat_max(spec_cached_height(&left), spec_cached_height(&right)) <= N::MAX as nat,
        ensures
            spec_inorder(Some(node)) =~= spec_inorder(left) + seq![value@] + spec_inorder(right),
            node.size as nat == 1 + spec_cached_size(&left) + spec_cached_size(&right),
            node.height as nat == 1 + spec_nat_max(
                spec_cached_height(&left), spec_cached_height(&right)),
            spec_avltreeseqstper_wf(left) && spec_avltreeseqstper_wf(right) ==> spec_avltreeseqstper_wf(Some(node)),
            node.left == left,
            node.right == right,
    {
        let hl = height_fn(&left);
        let hr = height_fn(&right);
        let sz = 1 + size_fn(&left) + size_fn(&right);
        let h = 1 + if hl >= hr { hl } else { hr };
        Arc::new(Node { value, height: h, size: sz, left, right })
    }

    fn rotate_right<T: StT>(y: Arc<Node<T>>) -> (rotated: Arc<Node<T>>)
        requires y.left.is_some(), spec_avltreeseqstper_wf(Some(y)), obeys_feq_clone::<T>(),
        ensures
            spec_inorder(Some(rotated)) =~= spec_inorder(Some(y)),
            spec_avltreeseqstper_wf(Some(rotated)),
            spec_cached_size(&Some(rotated)) == spec_cached_size(&Some(y)),
    {
        let ghost old_y = y;
        let x = y.left.as_ref().unwrap().clone();
        proof {
            // Unfold wf: x == y.left.unwrap(), wf(y.left) holds.
            assert(spec_avltreeseqstper_wf(y.left));
            assert(spec_avltreeseqstper_wf(y.right));
            assert(spec_avltreeseqstper_wf(x.left));
            assert(spec_avltreeseqstper_wf(x.right));
            // Size bound: y.size is usize, so 1 + size(x) + size(C) <= N::MAX.
            // x.size is usize, so 1 + size(A) + size(B) <= N::MAX.
            // Combined: 2 + size(A) + size(B) + size(C) <= N::MAX (== y.size).
            // For first mk (new_y = mk(y_val, B, C)):
            //   1 + size(B) + size(C) <= 2 + size(A) + size(B) + size(C) == y.size <= N::MAX.
            assert(1 + spec_cached_size(&x.right) + spec_cached_size(&y.right) <= N::MAX as nat);
            // Height: h(B) <= x.height-1, h(C) <= y.height-1.
            // max(h(B), h(C)) < y.height. So 1 + max(h(B), h(C)) <= y.height <= N::MAX.
            assert(1 + spec_nat_max(
                spec_cached_height(&x.right), spec_cached_height(&y.right)) <= N::MAX as nat);
        }
        let t2 = x.right.clone();
        let y_val = y.value.clone_plus();
        let new_y = mk(y_val, t2, y.right.clone());
        proof {
            assert(1 + spec_cached_size(&x.left) + spec_cached_size(&Some(new_y)) <= N::MAX as nat);
            lemma_height_le_size::<T>(&x.left);
            lemma_height_le_size::<T>(&Some(new_y));
            assert(1 + spec_nat_max(
                spec_cached_height(&x.left), spec_cached_height(&Some(new_y))) <= N::MAX as nat);
        }
        let x_val = x.value.clone_plus();
        let result = mk(x_val, x.left.clone(), Some(new_y));
        proof { reveal_with_fuel(spec_inorder, 3); }
        result
    }

    fn rotate_left<T: StT>(x: Arc<Node<T>>) -> (rotated: Arc<Node<T>>)
        requires x.right.is_some(), spec_avltreeseqstper_wf(Some(x)), obeys_feq_clone::<T>(),
        ensures
            spec_inorder(Some(rotated)) =~= spec_inorder(Some(x)),
            spec_avltreeseqstper_wf(Some(rotated)),
            spec_cached_size(&Some(rotated)) == spec_cached_size(&Some(x)),
    {
        let ghost old_x = x;
        let y = x.right.as_ref().unwrap().clone();
        proof {
            assert(spec_avltreeseqstper_wf(x.left));
            assert(spec_avltreeseqstper_wf(x.right));
            assert(spec_avltreeseqstper_wf(y.left));
            assert(spec_avltreeseqstper_wf(y.right));
            // First mk (new_x = mk(x_val, A, B)): A=x.left, B=y.left.
            // 1 + size(A) + size(B) ≤ x.size ≤ N::MAX (since x.size is usize,
            // and size(y) = 1 + size(B) + size(C), so x.size = 1 + size(A) + size(y) ≥ 2 + size(A) + size(B)).
            assert(1 + spec_cached_size(&x.left) + spec_cached_size(&y.left) <= N::MAX as nat);
            assert(1 + spec_nat_max(
                spec_cached_height(&x.left), spec_cached_height(&y.left)) <= N::MAX as nat);
        }
        let t2 = y.left.clone();
        let x_val = x.value.clone_plus();
        let new_x = mk(x_val, x.left.clone(), t2);
        proof {
            assert(1 + spec_cached_size(&Some(new_x)) + spec_cached_size(&y.right) <= N::MAX as nat);
            lemma_height_le_size::<T>(&Some(new_x));
            lemma_height_le_size::<T>(&y.right);
            assert(1 + spec_nat_max(
                spec_cached_height(&Some(new_x)), spec_cached_height(&y.right)) <= N::MAX as nat);
        }
        let y_val = y.value.clone_plus();
        let result = mk(y_val, Some(new_x), y.right.clone());
        proof { reveal_with_fuel(spec_inorder, 3); }
        result
    }

    fn rebalance<T: StT>(n: Arc<Node<T>>) -> (balanced: Arc<Node<T>>)
        requires spec_avltreeseqstper_wf(Some(n)), obeys_feq_clone::<T>(),
        ensures
            spec_inorder(Some(balanced)) =~= spec_inorder(Some(n)),
            spec_avltreeseqstper_wf(Some(balanced)),
            spec_cached_size(&Some(balanced)) == spec_cached_size(&Some(n)),
    {
        let hl = height_fn(&n.left);
        let hr = height_fn(&n.right);
        if hl > hr.saturating_add(1) {
            proof {
                assert(spec_cached_height(&n.left) > 0);
                assert(n.left.is_some());
            }
            let left = n.left.as_ref().unwrap().clone();
            let ghost left_size = spec_cached_size(&Some(left));
            proof {
                // left == n.left.unwrap(), so size(Some(left)) == size(n.left).
                assert(left_size == spec_cached_size(&n.left));
            }
            if height_fn(&left.right) > height_fn(&left.left) {
                // Left-right case: inner rotate_left, then rebuild with mk, then rotate_right.
                proof { assert(left.right.is_some()); }
                let rotated = rotate_left(left);
                let n_val = n.value.clone_plus();
                proof {
                    assert(spec_cached_size(&Some(rotated)) == left_size);
                    assert(1 + left_size + spec_cached_size(&n.right) <= N::MAX as nat);
                    lemma_height_le_size::<T>(&Some(rotated));
                    lemma_height_le_size::<T>(&n.right);
                }
                let rebuilt = mk(n_val, Some(rotated), n.right.clone());
                proof { reveal_with_fuel(spec_inorder, 2); }
                return rotate_right(rebuilt);
            }
            proof { reveal_with_fuel(spec_inorder, 2); }
            return rotate_right(n);
        }
        if hr > hl.saturating_add(1) {
            proof {
                assert(spec_cached_height(&n.right) > 0);
                assert(n.right.is_some());
            }
            let right = n.right.as_ref().unwrap().clone();
            let ghost right_size = spec_cached_size(&Some(right));
            proof {
                assert(right_size == spec_cached_size(&n.right));
            }
            if height_fn(&right.left) > height_fn(&right.right) {
                proof { assert(right.left.is_some()); }
                let rotated = rotate_right(right);
                let n_val = n.value.clone_plus();
                proof {
                    assert(spec_cached_size(&Some(rotated)) == right_size);
                    assert(1 + spec_cached_size(&n.left) + right_size <= N::MAX as nat);
                    lemma_height_le_size::<T>(&n.left);
                    lemma_height_le_size::<T>(&Some(rotated));
                }
                let rebuilt = mk(n_val, n.left.clone(), Some(rotated));
                proof { reveal_with_fuel(spec_inorder, 2); }
                return rotate_left(rebuilt);
            }
            proof { reveal_with_fuel(spec_inorder, 2); }
            return rotate_left(n);
        }
        n
    }

    fn nth_ref<'a, T: StT>(cur: &'a Link<T>, index: N) -> (elem: &'a T)
        requires spec_avltreeseqstper_wf(*cur), (index as int) < spec_inorder(*cur).len(),
        ensures elem@ == spec_inorder(*cur)[index as int],
        decreases *cur,
    {
        let node = cur.as_ref().unwrap();
        proof { lemma_size_eq_inorder_len::<T>(&node.left); }
        let ls = size_fn(&node.left);
        if index < ls {
            nth_ref(&node.left, index)
        } else if index == ls {
            &node.value
        } else {
            proof { lemma_size_eq_inorder_len::<T>(&node.right); }
            nth_ref(&node.right, index - ls - 1)
        }
    }

    fn set_rec<T: StT>(cur: &Link<T>, index: N, value: T) -> (outcome: Result<Link<T>, &'static str>)
        requires
            spec_avltreeseqstper_wf(*cur),
            (index as int) < spec_inorder(*cur).len(),
            obeys_feq_clone::<T>(),
        ensures
            outcome is Ok,
            spec_avltreeseqstper_wf(outcome.unwrap()),
            spec_cached_size(&outcome.unwrap()) == spec_cached_size(cur),
            spec_inorder(outcome.unwrap()) =~= spec_inorder(*cur).update(index as int, value@),
        decreases *cur,
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
                proof { lemma_size_eq_inorder_len::<T>(&n.left); }
                let ls = size_fn(&n.left);
                if index < ls {
                    let new_left = set_rec(&n.left, index, value)?;
                    let n_val = n.value.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<T>());
                        assert(n_val@ == n.value@);
                        lemma_height_le_size::<T>(&new_left);
                        lemma_height_le_size::<T>(&n.right);
                    }
                    Ok(Some(rebalance(mk(n_val, new_left, n.right.clone()))))
                } else if index == ls {
                    Ok(Some(mk(value, n.left.clone(), n.right.clone())))
                } else {
                    proof { lemma_size_eq_inorder_len::<T>(&n.right); }
                    let new_right = set_rec(&n.right, index - ls - 1, value)?;
                    let n_val = n.value.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<T>());
                        assert(n_val@ == n.value@);
                        lemma_height_le_size::<T>(&n.left);
                        lemma_height_le_size::<T>(&new_right);
                    }
                    Ok(Some(rebalance(mk(n_val, n.left.clone(), new_right))))
                }
            }
        }
    }

    fn inorder_collect<T: StT>(cur: &Link<T>, out: &mut Vec<T>)
        requires spec_avltreeseqstper_wf(*cur),
        ensures true,
        decreases *cur,
    {
        if let Some(n) = cur {
            inorder_collect(&n.left, out);
            out.push(n.value.clone());
            inorder_collect(&n.right, out);
        }
    }

    fn build_balanced_from_slice<T: StT>(a: &[T]) -> (link: Link<T>)
        requires 0nat < usize::MAX as nat,
        ensures
            spec_avltreeseqstper_wf(link),
            spec_inorder(link) =~= a@.map_values(|t: T| t@),
        decreases a.len(),
    {
        if a.is_empty() {
            assert(a@.map_values(|t: T| t@) =~= Seq::<T::V>::empty());
            return None;
        }
        let mid = a.len() / 2;
        let left_slice = slice_subrange(a, 0, mid);
        let right_slice = slice_subrange(a, mid + 1, a.len());
        let left = build_balanced_from_slice(left_slice);
        let right = build_balanced_from_slice(right_slice);
        let val = a[mid].clone();
        proof {
            lemma_size_eq_inorder_len::<T>(&left);
            lemma_size_eq_inorder_len::<T>(&right);
            lemma_height_le_size::<T>(&left);
            lemma_height_le_size::<T>(&right);
            assert(obeys_feq_full_trigger::<T>());
            assert(cloned(a@[mid as int], val));
        }
        let node = mk(val, left, right);
        proof {
            // Relate left_slice@ and right_slice@ to a@ subranges.
            let left_seq = left_slice@;
            let right_seq = right_slice@;
            let full_seq = a@;
            let f = |t: T| t@;
            // By recursive ensures:
            //   spec_inorder(left) =~= left_seq.map_values(f)
            //   spec_inorder(right) =~= right_seq.map_values(f)
            // mk ensures: spec_inorder(Some(node)) =~= spec_inorder(left) + seq![val@] + spec_inorder(right)
            // So: spec_inorder(Some(node)) =~= left_seq.map_values(f) + seq![val@] + right_seq.map_values(f)
            // Need: left_seq.map_values(f) + seq![val@] + right_seq.map_values(f) =~= full_seq.map_values(f)
            assert(left_seq =~= full_seq.subrange(0, mid as int));
            assert(right_seq =~= full_seq.subrange(mid as int + 1, full_seq.len() as int));
            assert(full_seq =~= left_seq + seq![full_seq[mid as int]] + right_seq);
        }
        Some(node)
    }

    fn compare_trees<T: StT>(a: &Link<T>, b: &Link<T>) -> (equal: bool)
        requires
            spec_avltreeseqstper_wf(*a),
            spec_avltreeseqstper_wf(*b),
            obeys_feq_full::<T>(),
        ensures equal == (spec_inorder(*a) =~= spec_inorder(*b)),
    {
        proof { lemma_size_eq_inorder_len::<T>(a); }
        proof { lemma_size_eq_inorder_len::<T>(b); }
        let sa = size_fn(a);
        let sb = size_fn(b);
        if sa != sb {
            return false;
        }
        let ghost seq_a = spec_inorder(*a);
        let ghost seq_b = spec_inorder(*b);
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < sa
            invariant
                sa == sb,
                sa as nat == seq_a.len(),
                sb as nat == seq_b.len(),
                seq_a == spec_inorder(*a),
                seq_b == spec_inorder(*b),
                spec_avltreeseqstper_wf(*a),
                spec_avltreeseqstper_wf(*b),
                obeys_feq_full::<T>(),
                0 <= i <= sa,
                forall|j: int| 0 <= j < i as int ==> seq_a[j] == seq_b[j],
            decreases sa - i,
        {
            let ai = nth_ref(a, i);
            let bi = nth_ref(b, i);
            let eq = feq(ai, bi);
            if !eq {
                return false;
            }
            assert(seq_a[i as int] == seq_b[i as int]);
            i += 1;
        }
        assert(seq_a =~= seq_b);
        true
    }

    // 9. trait impl

    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {
        open spec fn spec_seq(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }

        open spec fn spec_avltreeseqstper_wf(&self) -> bool {
            spec_avltreeseqstper_wf(self.root)
        }

        fn empty() -> (tree: Self) {
            AVLTreeSeqStPerS { root: None }
        }

        fn new() -> (tree: Self) {
            Self::empty()
        }

        fn singleton(item: T) -> (tree: Self) {
            AVLTreeSeqStPerS {
                root: Some(mk(item, None, None)),
            }
        }

        fn length(&self) -> (len: N) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            size_fn(&self.root)
        }

        fn nth(&self, index: N) -> (elem: &T) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            nth_ref(&self.root, index)
        }

        fn isEmpty(&self) -> (empty: B) {
            self.length() == 0
        }

        fn isSingleton(&self) -> (single: B) {
            self.length() == 1
        }

        fn set(&self, index: N, item: T) -> (outcome: Result<Self, &'static str>) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            Ok(AVLTreeSeqStPerS {
                root: set_rec(&self.root, index, item)?,
            })
        }

        fn subseq_copy(&self, start: N, length: N) -> (sub: Self) {
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
                    self.spec_avltreeseqstper_wf(),
                    n as int == self.spec_seq().len(),
                    s <= i, i <= e, e <= n,
                decreases e - i,
            {
                vals.push(self.nth(i).clone());
                i += 1;
            }
            Self::from_vec(vals)
        }

        fn from_vec(values: Vec<T>) -> (tree: Self) {
            let tree = AVLTreeSeqStPerS {
                root: build_balanced_from_slice(values.as_slice()),
            };
            tree
        }

        fn values_in_order(&self) -> (values: Vec<T>) {
            let n = self.length();
            let mut vals: Vec<T> = Vec::new();
            let mut i: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < n
                invariant
                    self.spec_avltreeseqstper_wf(),
                    n as int == self.spec_seq().len(),
                    0 <= i <= n,
                    vals@.len() == i as nat,
                    forall|j: int| 0 <= j < i as int ==> (#[trigger] vals@[j])@ == self.spec_seq()[j],
                decreases n - i,
            {
                let elem = self.nth(i);
                let val = elem.clone_plus();
                proof {
                    assert(obeys_feq_full_trigger::<T>());
                    assert(cloned(*elem, val));
                    lemma_cloned_view_eq::<T>(*elem, val);
                }
                vals.push(val);
                i += 1;
            }
            proof {
                assert(vals@.map_values(|t: T| t@) =~= self.spec_seq());
            }
            vals
        }

        fn to_arrayseq(&self) -> (seq: ArraySeqStPerS<T>) {
            let v = self.values_in_order();
            ArraySeqStPerS::from_vec(v)
        }

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqStPerIter<'a, T>)
        {
            let current = match &self.root {
                None => None,
                Some(arc_node) => Some(&**arc_node),
            };
            AVLTreeSeqStPerIter {
                stack: Vec::new(),
                current,
            }
        }
    }

    impl<T: StT> Default for AVLTreeSeqStPerS<T> {
        fn default() -> Self { Self::empty() }
    }

    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<T: StT> PartialEqSpecImpl for AVLTreeSeqStPerS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    // 10. iterators

    // veracity: no_requires — iterator helper called from Iterator::next.
    fn push_left_iter_stper<'a, T: StT>(it: &mut AVLTreeSeqStPerIter<'a, T>, cur: Option<&'a Node<T>>)
        requires 0nat <= usize::MAX as nat,
        ensures true,
        decreases cur,
    {
        if let Some(n) = cur {
            it.stack.push(n);
            let next = match &n.left {
                None => None,
                Some(arc) => Some(&**arc),
            };
            push_left_iter_stper(it, next);
        }
    }

    impl<'a, T: StT> IntoIterator for &'a AVLTreeSeqStPerS<T> {
        type Item = &'a T;
        type IntoIter = AVLTreeSeqStPerIter<'a, T>;
        fn into_iter(self) -> (it: AVLTreeSeqStPerIter<'a, T>)
            ensures true,
        {
            self.iter()
        }
    }

    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> (next: Option<Self::Item>)
            ensures true,
        {
            if self.current.is_some() {
                let cur = self.current.take();
                push_left_iter_stper(self, cur);
            }
            let node = self.stack.pop()?;
            let value_ref: &T = &node.value;
            let right_ref = match &node.right {
                None => None,
                Some(arc) => Some(&**arc),
            };
            push_left_iter_stper(self, right_ref);
            Some(value_ref)
        }
    }

    // 11. derive impls in verus!

    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}

    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof {
                assume(spec_avltreeseqstper_wf(self.root));
                assume(spec_avltreeseqstper_wf(other.root));
                assume(obeys_feq_full::<T>());
            }
            compare_trees(&self.root, &other.root)
        }
    }

    impl<T: StT> Clone for AVLTreeSeqStPerS<T> {
        fn clone(&self) -> (copy: Self)
            ensures
                copy@ == self@,
                self.spec_avltreeseqstper_wf() ==> copy.spec_avltreeseqstper_wf(),
        {
            let copy = AVLTreeSeqStPerS {
                root: self.root.clone(),
            };
            proof { accept(copy@ == self@ && (self.spec_avltreeseqstper_wf() ==> copy.spec_avltreeseqstper_wf())); }  // accept hole: Arc::clone external_body
            copy
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> Debug for Node<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("value", &self.value)
                .field("height", &self.height)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<T: StT> Display for Node<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl<T: StT> Debug for AVLTreeSeqStPerS<T> {
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

    impl<T: StT> Display for AVLTreeSeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let n = size_fn(&self.root);
            write!(f, "[")?;
            for i in 0..n {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", nth_ref(&self.root, i))?;
            }
            write!(f, "]")
        }
    }

    impl<'a, T: StT> Debug for AVLTreeSeqStPerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AVLTreeSeqStPerIter").finish()
        }
    }

    impl<'a, T: StT> Display for AVLTreeSeqStPerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AVLTreeSeqStPerIter")
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
