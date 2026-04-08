//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! StPer (immutable, structurally shared) AVL tree sequence using Arc path-copying.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 10b. iterators
//	Section 12b. derive impls in verus!
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module

pub mod AVLTreeSeqStPer {


    //		Section 2. imports

    use std::sync::Arc;
    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    use vstd::slice::slice_subrange;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{lemma_cloned_view_eq, obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger};

    verus! 
{


    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    //		Section 3. broadcast use


    broadcast use {
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4. type definitions


    pub type Link<T> = Option<Arc<Node<T>>>;

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StT> {
        pub value: T,
        pub height: usize,
        pub size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqStPerS<T: StT> {
        pub root: Link<T>,
    }

    //		Section 5b. view impls


    impl<T: StT> View for AVLTreeSeqStPerS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }
    }

    //		Section 6b. spec fns


    pub open spec fn avltreeseqstper_iter_invariant<'a, T: StT>(it: &AVLTreeSeqStPerIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

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
    /// Includes a capacity bound mirroring the StEph pattern: the subtree size fits in usize
    /// with room to spare, enabling safe insertion without overflow.
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
                && (spec_cached_size(&node.left) + spec_cached_size(&node.right) + 1 < usize::MAX)
            }
        }
    }

    //		Section 7b. proof fns/broadcast groups


    /// Under well-formedness, cached size equals in-order sequence length.
    pub proof fn lemma_size_eq_inorder_len<T: StT>(link: &Link<T>)
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

    /// Under well-formedness, the cached size is strictly less than usize::MAX.
    /// This follows directly from the capacity bound in spec_avltreeseqstper_wf.
    pub proof fn lemma_size_lt_usize_max<T: StT>(link: &Link<T>)
        requires spec_avltreeseqstper_wf(*link),
        ensures spec_cached_size(link) < usize::MAX,
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_size_lt_usize_max::<T>(&node.left);
                lemma_size_lt_usize_max::<T>(&node.right);
                // wf: size = 1 + left_size + right_size AND left_size + right_size + 1 < usize::MAX.
                // So size = 1 + left_size + right_size < usize::MAX.
            }
        }
    }

    /// Under struct-level well-formedness, the sequence length is bounded by usize::MAX.
    /// This broadcast lemma fires automatically when `s.spec_avltreeseqstper_wf()` is in context.
    pub broadcast proof fn lemma_wf_implies_len_bound_stper<T: StT>(s: AVLTreeSeqStPerS<T>)
        requires #[trigger] s.spec_avltreeseqstper_wf(),
        ensures s@.len() < usize::MAX,
    {
        lemma_size_lt_usize_max::<T>(&s.root);
        lemma_size_eq_inorder_len::<T>(&s.root);
    }

    pub broadcast group group_avltreeseqstper_len_bound {
        lemma_wf_implies_len_bound_stper,
    }

    //		Section 8b. traits


    /// Spec accessors for AVL tree nodes (Arc<Node>), enabling trait-based contracts.
    pub trait AVLTreeSeqStPerNodeSpec<T: StT>: Sized {
        spec fn node_wf(self) -> bool;
        spec fn node_inorder(self) -> Seq<T::V>;
        spec fn node_cached_size(self) -> nat;
        spec fn node_left(&self) -> Link<T>;
        spec fn node_right(&self) -> Link<T>;
    }

    /// Exec operations on non-empty AVL tree nodes (Arc<Node>).
    pub trait AVLTreeSeqStPerNodeFns<T: StT>: Sized + AVLTreeSeqStPerNodeSpec<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(self) -> (rotated: Self)
            requires self.node_wf(), self.node_left().is_some(), obeys_feq_clone::<T>(),
            ensures
                rotated.node_inorder() =~= self.node_inorder(),
                rotated.node_wf(),
                rotated.node_cached_size() == self.node_cached_size(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(self) -> (rotated: Self)
            requires self.node_wf(), self.node_right().is_some(), obeys_feq_clone::<T>(),
            ensures
                rotated.node_inorder() =~= self.node_inorder(),
                rotated.node_wf(),
                rotated.node_cached_size() == self.node_cached_size(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rebalance(self) -> (balanced: Self)
            requires self.node_wf(), obeys_feq_clone::<T>(),
            ensures
                balanced.node_inorder() =~= self.node_inorder(),
                balanced.node_wf(),
                balanced.node_cached_size() == self.node_cached_size(),
            ;
    }

    /// Spec accessors for AVL tree links, enabling trait-based contracts.
    pub trait AVLTreeSeqStPerLinkSpec<T: StT>: Sized {
        spec fn link_wf(self) -> bool;
        spec fn link_inorder(self) -> Seq<T::V>;
        spec fn link_cached_size(self) -> nat;
        spec fn link_cached_height(self) -> nat;
    }

    /// Exec operations on AVL tree links (Option<Arc<Node>>).
    pub trait AVLTreeSeqStPerLinkFns<T: StT>: Sized + AVLTreeSeqStPerLinkSpec<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn height_fn(&self) -> (h: usize)
            requires (*self).link_cached_height() <= usize::MAX as nat,
            ensures h as nat == (*self).link_cached_height(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_fn(&self) -> (sz: usize)
            requires (*self).link_cached_size() <= usize::MAX as nat,
            ensures sz as nat == (*self).link_cached_size(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn nth_ref(&self, index: usize) -> (elem: &T)
            requires (*self).link_wf(), (index as int) < (*self).link_inorder().len(),
            ensures elem@ == (*self).link_inorder()[index as int],
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set_rec(&self, index: usize, value: T) -> (outcome: Result<Self, &'static str>)
            requires
                (*self).link_wf(),
                (index as int) < (*self).link_inorder().len(),
                obeys_feq_clone::<T>(),
            ensures
                outcome is Ok,
                outcome.unwrap().link_wf(),
                outcome.unwrap().link_cached_size() == (*self).link_cached_size(),
                outcome.unwrap().link_inorder() =~= (*self).link_inorder().update(index as int, value@),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn inorder_collect(&self, out: &mut Vec<T>)
            requires (*self).link_wf(),
            ensures true,
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn compare_trees(&self, other: &Self) -> (equal: bool)
            requires
                (*self).link_wf(),
                (*other).link_wf(),
                obeys_feq_full::<T>(),
            ensures equal == ((*self).link_inorder() =~= (*other).link_inorder()),
            ;
    }

    pub trait AVLTreeSeqStPerTrait<T: StT>: Sized {
        spec fn spec_seq(&self) -> Seq<T::V>;
        spec fn spec_avltreeseqstper_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseqstper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseqstper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (tree: Self)
            ensures tree.spec_seq() =~= seq![item@], tree.spec_avltreeseqstper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize)
            requires self.spec_avltreeseqstper_wf(),
            ensures len as nat == self.spec_seq().len();

        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — DIFFERS: tree traversal to indexed node
        fn nth(&self, index: usize) -> (elem: &T)
            requires self.spec_avltreeseqstper_wf(), (index as int) < self.spec_seq().len(),
            ensures elem@ == self.spec_seq()[index as int];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isEmpty(&self) -> (empty: bool)
            requires self.spec_avltreeseqstper_wf(),
            ensures empty == (self.spec_seq().len() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isSingleton(&self) -> (single: bool)
            requires self.spec_avltreeseqstper_wf(),
            ensures single == (self.spec_seq().len() == 1);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set(&self, index: usize, item: T) -> (outcome: Result<Self, &'static str>)
            requires
                self.spec_avltreeseqstper_wf(),
                (index as int) < self.spec_seq().len(),
                obeys_feq_clone::<T>(),
            ensures
                outcome is Ok,
                outcome.unwrap().spec_avltreeseqstper_wf(),
                outcome.unwrap().spec_seq() =~= self.spec_seq().update(index as int, item@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn subseq_copy(&self, start: usize, length: usize) -> (sub: Self)
            requires self.spec_avltreeseqstper_wf(),
            ensures sub.spec_avltreeseqstper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_vec(values: Vec<T>) -> (tree: Self)
            requires values@.len() < usize::MAX,
            ensures
                tree.spec_avltreeseqstper_wf(),
                tree.spec_seq() =~= values@.map_values(|t: T| t@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn values_in_order(&self) -> (values: Vec<T>)
            requires self.spec_avltreeseqstper_wf(),
            ensures values@.map_values(|t: T| t@) =~= self.spec_seq();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_arrayseq(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_avltreeseqstper_wf(),
            ensures
                seq.spec_len() == self.spec_seq().len();

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqStPerIter<'a, T>)
            requires self.spec_avltreeseqstper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.spec_seq(),
                avltreeseqstper_iter_invariant(&it);
    }

    //		Section 9b. impls


    impl<T: StT> AVLTreeSeqStPerNodeSpec<T> for Arc<Node<T>> {
        open spec fn node_wf(self) -> bool { spec_avltreeseqstper_wf(Some(self)) }
        open spec fn node_inorder(self) -> Seq<T::V> { spec_inorder(Some(self)) }
        open spec fn node_cached_size(self) -> nat { spec_cached_size(&Some(self)) }
        open spec fn node_left(&self) -> Link<T> { self.left }
        open spec fn node_right(&self) -> Link<T> { self.right }
    }

    impl<T: StT> AVLTreeSeqStPerLinkSpec<T> for Link<T> {
        open spec fn link_wf(self) -> bool { spec_avltreeseqstper_wf(self) }
        open spec fn link_inorder(self) -> Seq<T::V> { spec_inorder(self) }
        open spec fn link_cached_size(self) -> nat { spec_cached_size(&self) }
        open spec fn link_cached_height(self) -> nat { spec_cached_height(&self) }
    }


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn mk<T: StT>(value: T, left: Link<T>, right: Link<T>) -> (node: Arc<Node<T>>)
        requires
            1 + spec_cached_size(&left) + spec_cached_size(&right) < usize::MAX as nat,
            1 + spec_nat_max(spec_cached_height(&left), spec_cached_height(&right)) <= usize::MAX as nat,
        ensures
            spec_inorder(Some(node)) =~= spec_inorder(left) + seq![value@] + spec_inorder(right),
            node.size as nat == 1 + spec_cached_size(&left) + spec_cached_size(&right),
            node.height as nat == 1 + spec_nat_max(
                spec_cached_height(&left), spec_cached_height(&right)),
            spec_avltreeseqstper_wf(left) && spec_avltreeseqstper_wf(right) ==> spec_avltreeseqstper_wf(Some(node)),
            node.left == left,
            node.right == right,
    {
        let hl = left.height_fn();
        let hr = right.height_fn();
        let sz = 1 + left.size_fn() + right.size_fn();
        let h = 1 + if hl >= hr { hl } else { hr };
        Arc::new(Node { value, height: h, size: sz, left, right })
    }

    impl<T: StT> AVLTreeSeqStPerNodeFns<T> for Arc<Node<T>> {

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    #[verifier::rlimit(15)]
    fn rotate_right(self) -> (rotated: Self)
    {
        let ghost node = self;
        let y = self;
        let ghost old_y = y;
        let x = y.left.as_ref().unwrap().clone();
        // Veracity: NEEDED proof block
        proof {
            // Unfold wf: x == y.left.unwrap(), wf(y.left) holds.
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(y.left));
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(y.right));
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(x.left));
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(x.right));
            // Size bound (strict <):
            // y.size < usize::MAX (wf capacity bound), y.size = 1 + x.size + size(C).
            // x.size = 1 + size(A) + size(B), so 2 + size(A) + size(B) + size(C) < usize::MAX.
            // For first mk (new_y = mk(y_val, B, C)):
            //   1 + size(B) + size(C) <= 2 + size(A) + size(B) + size(C) - 1 < usize::MAX.
            lemma_size_lt_usize_max::<T>(&y.left);  // x.size < usize::MAX
            // Height: h(B) <= x.height-1, h(C) <= y.height-1.
            // max(h(B), h(C)) < y.height. So 1 + max(h(B), h(C)) <= y.height <= N::MAX.
        }
        let t2 = x.right.clone();
        let y_val = y.value.clone_plus();
        let new_y = mk(y_val, t2, y.right.clone());
        // Veracity: NEEDED proof block
        proof {
            // new_y.size = 1 + size(B) + size(C) < usize::MAX (from above).
            // 1 + size(A) + new_y.size = 2 + size(A) + size(B) + size(C) < usize::MAX.
            lemma_height_le_size::<T>(&x.left);
            lemma_height_le_size::<T>(&Some(new_y));
        }
        let x_val = x.value.clone_plus();
        let result = mk(x_val, x.left.clone(), Some(new_y));
        // Veracity: NEEDED proof block
        proof { reveal_with_fuel(spec_inorder, 3); }
        result
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_left(self) -> (rotated: Self)
    {
        let ghost node = self;
        let x = self;
        let ghost old_x = x;
        let y = x.right.as_ref().unwrap().clone();
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(x.left));
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(x.right));
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(y.left));
            // Veracity: NEEDED assert
            assert(spec_avltreeseqstper_wf(y.right));
            // Size bound (strict <):
            // x.size < usize::MAX (wf capacity bound), x.size = 1 + size(A) + y.size.
            // y.size = 1 + size(B) + size(C), so 2 + size(A) + size(B) + size(C) < usize::MAX.
            // For first mk (new_x = mk(x_val, A, B)):
            //   1 + size(A) + size(B) <= 2 + size(A) + size(B) + size(C) - 1 < usize::MAX.
            lemma_size_lt_usize_max::<T>(&x.right);  // y.size < usize::MAX
        }
        let t2 = y.left.clone();
        let x_val = x.value.clone_plus();
        let new_x = mk(x_val, x.left.clone(), t2);
        // Veracity: NEEDED proof block
        proof {
            // new_x.size = 1 + size(A) + size(B) < usize::MAX (from above).
            // 1 + new_x.size + size(C) = 2 + size(A) + size(B) + size(C) < usize::MAX.
            lemma_height_le_size::<T>(&Some(new_x));
            lemma_height_le_size::<T>(&y.right);
        }
        let y_val = y.value.clone_plus();
        let result = mk(y_val, Some(new_x), y.right.clone());
        // Veracity: NEEDED proof block
        proof { reveal_with_fuel(spec_inorder, 3); }
        result
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rebalance(self) -> (balanced: Self)
    {
        let ghost node = self;
        let n = self;
        let hl = n.left.height_fn();
        let hr = n.right.height_fn();
        if hl > hr.saturating_add(1) {
            // Veracity: NEEDED proof block
            proof {
            }
            let left = n.left.as_ref().unwrap().clone();
            let ghost left_size = spec_cached_size(&Some(left));
            // Veracity: NEEDED proof block
            proof {
                // left == n.left.unwrap(), so size(Some(left)) == size(n.left).
            }
            if left.right.height_fn() > left.left.height_fn() {
                // Left-right case: inner rotate_left, then rebuild with mk, then rotate_right.
                let rotated = left.rotate_left();
                let n_val = n.value.clone_plus();
                // Veracity: NEEDED proof block
                proof {
                    // n.size < usize::MAX (wf), n.size = 1 + left_size + right_size.
                    // 1 + left_size + right_size = 1 + rotated_size + right_size < usize::MAX.
                    lemma_height_le_size::<T>(&Some(rotated));
                    lemma_height_le_size::<T>(&n.right);
                }
                let rebuilt = mk(n_val, Some(rotated), n.right.clone());
                // Veracity: NEEDED proof block
                proof { reveal_with_fuel(spec_inorder, 2); }
                return rebuilt.rotate_right();
            }
            // Veracity: NEEDED proof block
            proof { reveal_with_fuel(spec_inorder, 2); }
            return n.rotate_right();
        }
        if hr > hl.saturating_add(1) {
            // Veracity: NEEDED proof block
            proof {
            }
            let right = n.right.as_ref().unwrap().clone();
            let ghost right_size = spec_cached_size(&Some(right));
            // Veracity: NEEDED proof block
            proof {
            }
            if right.left.height_fn() > right.right.height_fn() {
                let rotated = right.rotate_right();
                let n_val = n.value.clone_plus();
                // Veracity: NEEDED proof block
                proof {
                    // n.size < usize::MAX (wf), n.size = 1 + left_size + right_size.
                    lemma_height_le_size::<T>(&n.left);
                    lemma_height_le_size::<T>(&Some(rotated));
                }
                let rebuilt = mk(n_val, n.left.clone(), Some(rotated));
                // Veracity: NEEDED proof block
                proof { reveal_with_fuel(spec_inorder, 2); }
                return rebuilt.rotate_left();
            }
            // Veracity: NEEDED proof block
            proof { reveal_with_fuel(spec_inorder, 2); }
            return n.rotate_left();
        }
        n
    }

    } // impl AVLTreeSeqStPerNodeFns

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
    fn build_balanced_from_slice<T: StT>(a: &[T]) -> (link: Link<T>)
        requires a.len() < usize::MAX,
        ensures
            spec_avltreeseqstper_wf(link),
            spec_inorder(link) =~= a@.map_values(|t: T| t@),
        decreases a.len(),
    {
        if a.is_empty() {
            return None;
        }
        let mid = a.len() / 2;
        let left_slice = slice_subrange(a, 0, mid);
        let right_slice = slice_subrange(a, mid + 1, a.len());
        let left = build_balanced_from_slice(left_slice);
        let right = build_balanced_from_slice(right_slice);
        let val = a[mid].clone();
        // Veracity: NEEDED proof block
        proof {
            lemma_size_eq_inorder_len::<T>(&left);
            lemma_size_eq_inorder_len::<T>(&right);
            lemma_height_le_size::<T>(&left);
            lemma_height_le_size::<T>(&right);
            // Veracity: NEEDED assert
            assert(obeys_feq_full_trigger::<T>());
            // Veracity: NEEDED assert
            assert(cloned(a@[mid as int], val));
            // mid + (a.len() - mid - 1) + 1 = a.len() < usize::MAX, so mk's strict bound holds.
        }
        let node = mk(val, left, right);
        // Veracity: NEEDED proof block
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
        }
        Some(node)
    }

    impl<T: StT> AVLTreeSeqStPerLinkFns<T> for Link<T> {

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn height_fn(&self) -> (h: usize)
    {
        match self {
            None => 0,
            Some(node) => node.height,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn size_fn(&self) -> (sz: usize)
    {
        match self {
            None => 0,
            Some(node) => node.size,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn nth_ref(&self, index: usize) -> (elem: &T)
        decreases *self,
    {
        let node = self.as_ref().unwrap();
        // Veracity: NEEDED proof block
        proof { lemma_size_eq_inorder_len::<T>(&node.left); }
        let ls = node.left.size_fn();
        if index < ls {
            node.left.nth_ref(index)
        } else if index == ls {
            &node.value
        } else {
            // Veracity: NEEDED proof block
            proof { lemma_size_eq_inorder_len::<T>(&node.right); }
            node.right.nth_ref(index - ls - 1)
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn set_rec(&self, index: usize, value: T) -> (outcome: Result<Self, &'static str>)
        decreases *self,
    {
        match self {
            None => {
                if index == 0 {
                    Ok(Some(mk(value, None, None)))
                } else {
                    Err("Index out of bounds")
                }
            }
            Some(n) => {
                // Veracity: NEEDED proof block
                proof { lemma_size_eq_inorder_len::<T>(&n.left); }
                let ls = n.left.size_fn();
                if index < ls {
                    let new_left = n.left.set_rec(index, value)?;
                    let n_val = n.value.clone_plus();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_height_le_size::<T>(&new_left);
                        lemma_height_le_size::<T>(&n.right);
                    }
                    Ok(Some(mk(n_val, new_left, n.right.clone()).rebalance()))
                } else if index == ls {
                    Ok(Some(mk(value, n.left.clone(), n.right.clone())))
                } else {
                    // Veracity: NEEDED proof block
                    proof { lemma_size_eq_inorder_len::<T>(&n.right); }
                    let new_right = n.right.set_rec(index - ls - 1, value)?;
                    let n_val = n.value.clone_plus();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_height_le_size::<T>(&n.left);
                        lemma_height_le_size::<T>(&new_right);
                    }
                    Ok(Some(mk(n_val, n.left.clone(), new_right).rebalance()))
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn inorder_collect(&self, out: &mut Vec<T>)
        decreases *self,
    {
        if let Some(n) = self {
            n.left.inorder_collect(out);
            out.push(n.value.clone());
            n.right.inorder_collect(out);
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
    fn compare_trees(&self, other: &Self) -> (equal: bool)
    {
        // Veracity: NEEDED proof block
        proof { lemma_size_eq_inorder_len::<T>(self); }
        // Veracity: NEEDED proof block
        proof { lemma_size_eq_inorder_len::<T>(other); }
        let sa = self.size_fn();
        let sb = other.size_fn();
        if sa != sb {
            return false;
        }
        let ghost seq_a = spec_inorder(*self);
        let ghost seq_b = spec_inorder(*other);
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < sa
            invariant
                sa == sb,
                sa as nat == seq_a.len(),
                sb as nat == seq_b.len(),
                seq_a == spec_inorder(*self),
                seq_b == spec_inorder(*other),
                spec_avltreeseqstper_wf(*self),
                spec_avltreeseqstper_wf(*other),
                obeys_feq_full::<T>(),
                0 <= i <= sa,
                forall|j: int| 0 <= j < i as int ==> seq_a[j] == seq_b[j],
            decreases sa - i,
        {
            let ai = self.nth_ref(i);
            let bi = other.nth_ref(i);
            let eq = feq(ai, bi);
            if !eq {
                return false;
            }
            i += 1;
        }
        true
    }

    } // impl AVLTreeSeqStPerLinkFns

    // 9. trait impl

    impl<T: StT> AVLTreeSeqStPerTrait<T> for AVLTreeSeqStPerS<T> {
        open spec fn spec_seq(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }

        open spec fn spec_avltreeseqstper_wf(&self) -> bool {
            spec_avltreeseqstper_wf(self.root)
            && obeys_feq_full::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (tree: Self) {
                      // Veracity: NEEDED assert
                      assert(obeys_feq_full_trigger::<T>());
            AVLTreeSeqStPerS { root: None }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self) {
            Self::empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (tree: Self) {
                      // Veracity: NEEDED assert
                      assert(obeys_feq_full_trigger::<T>());
            AVLTreeSeqStPerS {
                root: Some(mk(item, None, None)),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize) {
            // Veracity: NEEDED proof block
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            self.root.size_fn()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn nth(&self, index: usize) -> (elem: &T) {
            // Veracity: NEEDED proof block
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            self.root.nth_ref(index)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isEmpty(&self) -> (empty: bool) {
            self.length() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isSingleton(&self) -> (single: bool) {
            self.length() == 1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set(&self, index: usize, item: T) -> (outcome: Result<Self, &'static str>) {
            // Veracity: NEEDED proof block
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            Ok(AVLTreeSeqStPerS {
                root: self.root.set_rec(index, item)?,
            })
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn subseq_copy(&self, start: usize, length: usize) -> (sub: Self) {
            let n = self.length();
            // Veracity: NEEDED proof block
            proof {
                lemma_size_eq_inorder_len::<T>(&self.root);
                lemma_size_lt_usize_max::<T>(&self.root);
                // n as nat == spec_cached_size(&self.root) < usize::MAX.
            }
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
                    n < usize::MAX,
                    s <= i, i <= e, e <= n,
                    vals@.len() == (i - s) as nat,
                decreases e - i,
            {
                vals.push(self.nth(i).clone());
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            Self::from_vec(vals)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_vec(values: Vec<T>) -> (tree: Self) {
                      // Veracity: NEEDED assert
                      assert(obeys_feq_full_trigger::<T>());
            let tree = AVLTreeSeqStPerS {
                root: build_balanced_from_slice(values.as_slice()),
            };
            tree
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
                // Veracity: NEEDED proof block
                proof {
                    lemma_cloned_view_eq::<T>(*elem, val);
                }
                vals.push(val);
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            vals
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
                elements: Ghost(self.spec_seq()),
                pos: Ghost(0int),
            }
        }
    }


    // veracity: no_requires
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn push_left_iter_stper<'a, T: StT>(it: &mut AVLTreeSeqStPerIter<'a, T>, cur: Option<&'a Node<T>>)
        ensures
            it@.0 == old(it)@.0,
            it@.1 == old(it)@.1,
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

    //		Section 10b. iterators


    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqStPerIter<'a, T: StT> {
        pub stack: Vec<&'a Node<T>>,
        pub current: Option<&'a Node<T>>,
        pub elements: Ghost<Seq<T::V>>,
        pub pos: Ghost<int>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqStPerGhostIterator<'a, T: StT> {
        pub pos: int,
        pub elements: Seq<T::V>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T: StT> View for AVLTreeSeqStPerIter<'a, T> {
        type V = (int, Seq<T::V>);
        open spec fn view(&self) -> (int, Seq<T::V>) {
            (self.pos@, self.elements@)
        }
    }

    impl<'a, T: StT> View for AVLTreeSeqStPerGhostIterator<'a, T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, T: StT> Iterator for AVLTreeSeqStPerIter<'a, T> {
        type Item = &'a T;

        #[verifier::external_body]
        fn next(&mut self) -> (next: Option<Self::Item>)
            ensures
                ({
                    let (old_index, old_seq) = old(self)@;
                    match next {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index >= old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& element@ == old_seq[old_index]
                        },
                    }
                }),
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

    impl<'a, T: StT> vstd::pervasive::ForLoopGhostIteratorNew for AVLTreeSeqStPerIter<'a, T> {
        type GhostIter = AVLTreeSeqStPerGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> AVLTreeSeqStPerGhostIterator<'a, T> {
            AVLTreeSeqStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T: StT> vstd::pervasive::ForLoopGhostIterator for AVLTreeSeqStPerGhostIterator<'a, T> {
        type ExecIter = AVLTreeSeqStPerIter<'a, T>;
        type Item = T::V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &AVLTreeSeqStPerIter<'a, T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T::V> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &AVLTreeSeqStPerIter<'a, T>) -> AVLTreeSeqStPerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT> IntoIterator for &'a AVLTreeSeqStPerS<T> {
        type Item = &'a T;
        type IntoIter = AVLTreeSeqStPerIter<'a, T>;

        fn into_iter(self) -> (it: AVLTreeSeqStPerIter<'a, T>)
            requires self.spec_avltreeseqstper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == spec_inorder(self.root),
                avltreeseqstper_iter_invariant(&it),
        {
            self.iter()
        }
    }

    //		Section 12b. derive impls in verus!


    impl<T: StT> Default for AVLTreeSeqStPerS<T> {
        fn default() -> Self { Self::empty() }
    }


    #[cfg(verus_keep_ghost)]
    impl<T: StT> PartialEqSpecImpl for AVLTreeSeqStPerS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT> Eq for AVLTreeSeqStPerS<T> {}

    impl<T: StT> PartialEq for AVLTreeSeqStPerS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            // Veracity: NEEDED proof block
            proof {
                assume(spec_avltreeseqstper_wf(self.root));
                assume(spec_avltreeseqstper_wf(other.root));
                assume(obeys_feq_full::<T>());
            }
            self.root.compare_trees(&other.root)
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
            // Veracity: NEEDED proof block
            proof { assume(copy@ == self@ && (self.spec_avltreeseqstper_wf() ==> copy.spec_avltreeseqstper_wf())); }  // accept hole: Arc::clone external_body
            copy
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


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

    impl<'a, T: StT> Debug for AVLTreeSeqStPerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AVLTreeSeqStPerGhostIterator").finish()
        }
    }

    impl<'a, T: StT> Display for AVLTreeSeqStPerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AVLTreeSeqStPerGhostIterator")
        }
    }

    //		Section 14a. derive impls outside verus!

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

    //		Section 14b. derive impls outside verus!

    impl<T: StT> Debug for AVLTreeSeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let n = self.root.size_fn();
            write!(f, "[")?;
            for i in 0..n {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", self.root.nth_ref(i))?;
            }
            write!(f, "]")
        }
    }

    impl<T: StT> Display for AVLTreeSeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let n = self.root.size_fn();
            write!(f, "[")?;
            for i in 0..n {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", self.root.nth_ref(i))?;
            }
            write!(f, "]")
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
