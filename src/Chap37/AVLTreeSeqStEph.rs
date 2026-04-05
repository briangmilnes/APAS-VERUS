//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! StEphemeral (mutable) implicit-order AVL tree sequence.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module

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

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{lemma_cloned_view_eq, obeys_feq_full};
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    verus! {

    //		2. imports

    // 2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    //		3. broadcast use

    // 3. broadcast use

    broadcast use {
        vstd::seq::group_seq_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		4. type definitions

    // 4. type definitions

    pub type Link<T> = Option<Box<AVLTreeNode<T>>>;

    pub struct AVLTreeNode<T: StT> {
        pub value: T,
        pub height: usize,
        pub left_size: usize,
        pub right_size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
        pub index: usize,
    }

    pub struct AVLTreeSeqStEphS<T: StT> {
        pub root: Link<T>,
        pub next_key: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqIterStEph<'a, T: StT> {
        pub stack: Vec<&'a AVLTreeNode<T>>,
        pub current: Option<&'a AVLTreeNode<T>>,
        pub elements: Ghost<Seq<T::V>>,
        pub pos: Ghost<int>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqStEphGhostIterator<'a, T: StT> {
        pub pos: int,
        pub elements: Seq<T::V>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    //		5. view impls

    // 5. view impls

    impl<T: StT> View for AVLTreeSeqStEphS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }
    }

    impl<'a, T: StT> View for AVLTreeSeqIterStEph<'a, T> {
        type V = (int, Seq<T::V>);
        open spec fn view(&self) -> (int, Seq<T::V>) {
            (self.pos@, self.elements@)
        }
    }

    impl<'a, T: StT> View for AVLTreeSeqStEphGhostIterator<'a, T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            self.elements.take(self.pos)
        }
    }

    //		6. spec fns

    // 6. spec fns

    pub open spec fn avltreeseqsteph_iter_invariant<'a, T: StT>(it: &AVLTreeSeqIterStEph<'a, T>) -> bool {
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

    pub open spec fn spec_subseq<V>(seq: Seq<V>, start: nat, length: nat) -> Seq<V> {
        let n = seq.len();
        let s = if start < n { start } else { n };
        let e_raw = start + length;
        let e = if e_raw < n { e_raw } else { n };
        if e <= s { Seq::<V>::empty() } else { seq.subrange(s as int, e as int) }
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
                && (node.left_size + node.right_size + 1 < usize::MAX)
            }
        }
    }

    //		7. proof fns/broadcast groups

    // 7. proof fns

    /// Under well-formedness, cached size equals in-order sequence length.
    pub proof fn lemma_size_eq_inorder_len<T: StT>(link: &Link<T>)
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

    /// Under well-formedness, cached height is at most cached size.
    proof fn lemma_height_le_size<T: StT>(link: &Link<T>)
        requires spec_avltreeseqsteph_wf(*link),
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
    pub proof fn lemma_size_lt_usize_max<T: StT>(link: &Link<T>)
        requires spec_avltreeseqsteph_wf(*link),
        ensures spec_cached_size(link) < usize::MAX,
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_size_lt_usize_max::<T>(&node.left);
                lemma_size_lt_usize_max::<T>(&node.right);
            }
        }
    }

    /// Under struct-level well-formedness, the sequence length is bounded by usize::MAX.
    pub broadcast proof fn lemma_wf_implies_len_bound_steph<T: StT>(s: AVLTreeSeqStEphS<T>)
        requires #[trigger] s.spec_avltreeseqsteph_wf(),
        ensures s@.len() < usize::MAX,
    {
        lemma_size_lt_usize_max::<T>(&s.root);
        lemma_size_eq_inorder_len::<T>(&s.root);
    }

    pub broadcast group group_avltreeseqsteph_len_bound {
        lemma_wf_implies_len_bound_steph,
    }

    //		8. traits

    // 8. traits

    /// Spec accessors for AVL tree links, enabling trait-based contracts.
    pub trait AVLTreeSeqStEphLinkSpec<T: StT>: Sized {
        spec fn link_wf(self) -> bool;
        spec fn link_inorder(self) -> Seq<T::V>;
        spec fn link_cached_size(self) -> nat;
        spec fn link_cached_height(self) -> nat;
    }

    impl<T: StT> AVLTreeSeqStEphLinkSpec<T> for Link<T> {
        open spec fn link_wf(self) -> bool { spec_avltreeseqsteph_wf(self) }
        open spec fn link_inorder(self) -> Seq<T::V> { spec_inorder(self) }
        open spec fn link_cached_size(self) -> nat { spec_cached_size(&self) }
        open spec fn link_cached_height(self) -> nat { spec_cached_height(&self) }
    }

    /// Exec operations on AVL tree links (Option<Box<AVLTreeNode>>).
    pub trait AVLTreeSeqStEphLinkFns<T: StT>: Sized + AVLTreeSeqStEphLinkSpec<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn h_fn(&self) -> (height: usize)
            requires (*self).link_cached_height() <= usize::MAX as nat,
            ensures height as nat == (*self).link_cached_height(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link_fn(&self) -> (size: usize)
            requires (*self).link_wf(),
            ensures size as nat == (*self).link_cached_size(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert_at_link(self, index: usize, value: T, next_key: &mut usize) -> (inserted: Self)
            requires
                self.link_wf(),
                0 <= index as int <= self.link_inorder().len(),
                *old(next_key) < usize::MAX,
                self.link_cached_size() + 1 < usize::MAX,
            ensures
                inserted.link_wf(),
                inserted.link_inorder() =~= self.link_inorder().insert(index as int, value@),
                inserted.link_cached_size() == self.link_cached_size() + 1,
                *next_key == *old(next_key) + 1,
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn nth_link(&self, index: usize) -> (elem: &T)
            requires (*self).link_wf(), (index as int) < (*self).link_inorder().len(),
            ensures elem@ == (*self).link_inorder()[index as int],
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set_link(&mut self, index: usize, value: T) -> (outcome: Result<(), &'static str>)
            requires
                old(self).link_wf(),
                (index as int) < old(self).link_inorder().len(),
            ensures
                (*self).link_wf(),
                (*self).link_cached_size() == old(self).link_cached_size(),
                (*self).link_cached_height() == old(self).link_cached_height(),
                outcome is Ok,
                (*self).link_inorder() =~= old(self).link_inorder().update(index as int, value@),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn compare_trees(&self, other: &Self) -> (equal: bool)
            requires
                (*self).link_wf(),
                (*other).link_wf(),
                obeys_feq_full::<T>(),
            ensures equal == ((*self).link_inorder() =~= (*other).link_inorder()),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        // veracity: no_requires
        fn clone_link(&self) -> (copy: Self)
            ensures
                copy.link_inorder() =~= (*self).link_inorder(),
                (*self).link_wf() ==> copy.link_wf(),
                copy.link_cached_size() == (*self).link_cached_size(),
                copy.link_cached_height() == (*self).link_cached_height(),
            ;
    }

    pub trait AVLTreeSeqStEphTrait<T: StT>: Sized {
        spec fn spec_seq(&self) -> Seq<T::V>;
        spec fn spec_avltreeseqsteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseqsteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseqsteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize)
            requires self.spec_avltreeseqsteph_wf(),
            ensures len as nat == self.spec_seq().len();

        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — DIFFERS: tree traversal to indexed node
        fn nth(&self, index: usize) -> (elem: &T)
            requires self.spec_avltreeseqsteph_wf(), (index as int) < self.spec_seq().len(),
            ensures elem@ == self.spec_seq()[index as int];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set(&mut self, index: usize, item: T) -> (outcome: Result<(), &'static str>)
            requires old(self).spec_avltreeseqsteph_wf(), (index as int) < old(self).spec_seq().len(),
            ensures
                outcome is Ok,
                self.spec_avltreeseqsteph_wf(),
                self.spec_seq() =~= old(self).spec_seq().update(index as int, item@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (tree: Self)
            ensures
                tree.spec_seq().len() == 1,
                tree.spec_seq()[0] == item@,
                tree.spec_avltreeseqsteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isEmpty(&self) -> (empty: bool)
            requires self.spec_avltreeseqsteph_wf(),
            ensures empty == (self.spec_seq().len() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isSingleton(&self) -> (single: bool)
            requires self.spec_avltreeseqsteph_wf(),
            ensures single == (self.spec_seq().len() == 1);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn subseq_copy(&self, start: usize, length: usize) -> (sub: Self)
            requires self.spec_avltreeseqsteph_wf(),self.spec_seq().len() < usize::MAX,
            ensures sub.spec_seq() =~= spec_subseq(self.spec_seq(), start as nat, length as nat);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new_root() -> (tree: Self)
            ensures tree.spec_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseqsteph_wf();

        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — DIFFERS: tree traversal + path reconstruction
        fn update(&mut self, index: usize, item: T)
            requires
                old(self).spec_avltreeseqsteph_wf(),
                (index as int) < old(self).spec_seq().len(),
            ensures
                self.spec_avltreeseqsteph_wf(),
                self.spec_seq() =~= old(self).spec_seq().update(index as int, item@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_vec(values: Vec<T>) -> (tree: AVLTreeSeqStEphS<T>)
            requires
                obeys_feq_full::<T>(),
                values@.len() < usize::MAX,
            ensures
                spec_avltreeseqsteph_wf(tree.root),
                tree.next_key as nat == spec_cached_size(&tree.root),
                spec_inorder(tree.root) =~= values@.map_values(|t: T| t@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_arrayseq(&self) -> (seq: ArraySeqStEphS<T>)
            requires self.spec_avltreeseqsteph_wf(),
            ensures
                seq.spec_len() == self.spec_seq().len(),
                forall|i: int| #![trigger seq.spec_index(i)]
                    0 <= i < seq.spec_len() ==> seq.spec_index(i)@ == self.spec_seq()[i];

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqIterStEph<'a, T>)
            requires self.spec_avltreeseqsteph_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.spec_seq(),
                avltreeseqsteph_iter_invariant(&it);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn push_back(&mut self, value: T)
            requires
                old(self).spec_avltreeseqsteph_wf(),
                old(self).spec_seq().len() + 1 < usize::MAX,
            ensures
                self.spec_seq() =~= old(self).spec_seq().push(value@),
                self.spec_avltreeseqsteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn contains_value(&self, target: &T) -> (found: bool)
            requires self.spec_avltreeseqsteph_wf(),
            ensures found == exists|j: int| 0 <= j < self.spec_seq().len()
                && self.spec_seq()[j] == target@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert_value(&mut self, value: T)
            requires
                old(self).spec_avltreeseqsteph_wf(),
                old(self).spec_seq().len() + 1 < usize::MAX,
            ensures
                self.spec_seq() =~= old(self).spec_seq().push(value@),
                self.spec_avltreeseqsteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn delete_value(&mut self, target: &T) -> (deleted: bool)
            requires old(self).spec_avltreeseqsteph_wf(),
            ensures
                !deleted ==> self.spec_seq() =~= old(self).spec_seq(),
                deleted ==> exists|idx: int|
                    #![trigger old(self).spec_seq()[idx]]
                    0 <= idx < old(self).spec_seq().len()
                    && old(self).spec_seq()[idx] == target@
                    && self.spec_seq() =~=
                        old(self).spec_seq().subrange(0, idx)
                        + old(self).spec_seq().subrange(idx + 1,
                            old(self).spec_seq().len() as int);
    }

    //		9. impls

    // 9. impls

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn update_meta<T: StT>(n: &mut Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseqsteph_wf(old(n).left),
            spec_avltreeseqsteph_wf(old(n).right),
            spec_cached_size(&old(n).left) + spec_cached_size(&old(n).right) + 1 < usize::MAX,
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
        n.left_size = n.left.size_link_fn();
        n.right_size = n.right.size_link_fn();
        let hl = n.left.h_fn();
        let hr = n.right.h_fn();
        proof {
            lemma_height_le_size::<T>(&n.left);
            lemma_height_le_size::<T>(&n.right);
            // h_left <= size_left, h_right <= size_right.
            // size_left + size_right + 1 < usize::MAX (precondition).
            // max(h_left, h_right) <= size_left + size_right < usize::MAX - 1.
            // 1 + max(h_left, h_right) <= 1 + size_left + size_right < usize::MAX.
        }
        n.height = 1 + if hl >= hr { hl } else { hr };
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rebalance_fn<T: StT>(mut n: Box<AVLTreeNode<T>>) -> (balanced: Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseqsteph_wf(n.left),
            spec_avltreeseqsteph_wf(n.right),
            spec_cached_size(&n.left) + spec_cached_size(&n.right) + 1 < usize::MAX,
        ensures
            spec_inorder(Some(balanced)) =~= spec_inorder(Some(n)),
            spec_avltreeseqsteph_wf(Some(balanced)),
            spec_cached_size(&Some(balanced))
                == 1 + spec_cached_size(&n.left) + spec_cached_size(&n.right),
    {
        update_meta(&mut n);
        let hl = n.left.h_fn();
        let hr = n.right.h_fn();
        if hl > hr.saturating_add(1) {
            // Left-heavy: n.left must be Some since hl > 1.
            proof {
                if n.left is None { assert(spec_cached_height(&n.left) == 0); }
                assert(n.left is Some);
            }
            if n.left.as_ref().unwrap().right.h_fn() > n.left.as_ref().unwrap().left.h_fn() {
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
            if n.right.as_ref().unwrap().left.h_fn() > n.right.as_ref().unwrap().right.h_fn() {
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

    impl<T: StT> AVLTreeSeqStEphLinkFns<T> for Link<T> {

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn h_fn(&self) -> (height: usize)
    {
        match self {
            None => 0,
            Some(b) => b.height,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn size_link_fn(&self) -> (size: usize)
    {
        match self {
            None => 0,
            Some(b) => {
                1 + b.left_size + b.right_size
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn insert_at_link(self, index: usize, value: T, next_key: &mut usize) -> (inserted: Self)
        decreases self,
    {
        let ghost node = self;
        match self {
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
                let ghost old_n = *n;
                proof {
                    lemma_size_eq_inorder_len::<T>(&n.left);
                    lemma_size_eq_inorder_len::<T>(&n.right);
                }
                let left_size = n.left_size;
                let ghost old_left_size = spec_cached_size(&old_n.left);
                if index <= left_size {
                    let ghost old_right = n.right;
                    n.left = n.left.take().insert_at_link(index, value, next_key);
                    proof {
                        assert(spec_avltreeseqsteph_wf(n.left));
                        assert(n.right == old_right);
                        assert(spec_avltreeseqsteph_wf(n.right));
                        assert(spec_inorder(n.left)
                            =~= spec_inorder(old_n.left).insert(index as int, value@));
                    }
                } else {
                    let ghost old_left = n.left;
                    n.right = n.right.take().insert_at_link(
                        index - left_size - 1, value, next_key,
                    );
                    proof {
                        assert(spec_avltreeseqsteph_wf(n.right));
                        assert(n.left == old_left);
                        assert(spec_avltreeseqsteph_wf(n.left));
                        assert(spec_inorder(n.right)
                            =~= spec_inorder(old_n.right).insert(
                                (index - left_size - 1) as int, value@));
                    }
                }
                Some(rebalance_fn(n))
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn nth_link(&self, index: usize) -> (elem: &T)
        decreases *self,
    {
        let n = self.as_ref().expect("index out of bounds");
        proof { lemma_size_eq_inorder_len::<T>(&n.left); }
        proof { lemma_size_eq_inorder_len::<T>(&n.right); }
        let left_size = n.left_size;
        if index < left_size {
            n.left.nth_link(index)
        } else if index == left_size {
            &n.value
        } else {
            n.right.nth_link(index - left_size - 1)
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn set_link(&mut self, index: usize, value: T) -> (outcome: Result<(), &'static str>)
        decreases *old(self),
    {
        let cur = self.take();
        match cur {
            None => {
                *self = None;
                Err("Index out of bounds")
            }
            Some(mut n) => {
                let ghost old_n = *n;
                proof { lemma_size_eq_inorder_len::<T>(&n.left); }
                proof { lemma_size_eq_inorder_len::<T>(&n.right); }
                let left_size = n.left_size;
                let result = if index < left_size {
                    n.left.set_link(index, value)
                } else if index == left_size {
                    n.value = value;
                    Ok(())
                } else {
                    n.right.set_link(index - left_size - 1, value)
                };
                *self = Some(n);
                result
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
    fn compare_trees(&self, other: &Self) -> (equal: bool)
    {
        proof { lemma_size_eq_inorder_len::<T>(self); }
        proof { lemma_size_eq_inorder_len::<T>(other); }
        let sa = self.size_link_fn();
        let sb = other.size_link_fn();
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
                0 <= i <= sa,
                forall|j: int| 0 <= j < i as int ==> seq_a[j] == seq_b[j],
            decreases sa - i,
        {
            let ai = self.nth_link(i);
            let bi = other.nth_link(i);
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    // veracity: no_requires
    fn clone_link(&self) -> (copy: Self)
        decreases *self,
    {
              assert(obeys_feq_full_trigger::<T>());
        match self {
            None => None,
            Some(node) => {
                let left = node.left.clone_link();
                let right = node.right.clone_link();
                let new_value = node.value.clone_plus();
                proof { assume(new_value@ == node.value@); }
                Some(Box::new(AVLTreeNode {
                    value: new_value,
                    height: node.height,
                    left_size: node.left_size,
                    right_size: node.right_size,
                    left,
                    right,
                    index: node.index,
                }))
            }
        }
    }

    } // impl AVLTreeSeqStEphLinkFns

    // 9. trait impl

    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {
        open spec fn spec_seq(&self) -> Seq<T::V> {
            spec_inorder(self.root)
        }

        open spec fn spec_avltreeseqsteph_wf(&self) -> bool {
            spec_avltreeseqsteph_wf(self.root)
            && self.next_key as nat == spec_cached_size(&self.root)
            && obeys_feq_full::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            AVLTreeSeqStEphS { root: None, next_key: 0 }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            Self::empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            self.root.size_link_fn()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn nth(&self, index: usize) -> (elem: &T) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            self.root.nth_link(index)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set(&mut self, index: usize, item: T) -> (outcome: Result<(), &'static str>) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            self.root.set_link(index, item)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            let mut t = AVLTreeSeqStEphS { root: None, next_key: 0 };
            t.root = t.root.take().insert_at_link(0, item, &mut t.next_key);
            proof { lemma_size_eq_inorder_len::<T>(&t.root); }
            t
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isEmpty(&self) -> (empty: bool) {
            self.length() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isSingleton(&self) -> (single: bool) {
            self.length() == 1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn subseq_copy(&self, start: usize, length: usize) -> (sub: Self) {
            assert(self.spec_avltreeseqsteph_wf());
            assert(obeys_feq_full::<T>());
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
                    self.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<T>(),
                    n as int == self.spec_seq().len(),
                    n < usize::MAX,
                    s <= i, i <= e, e <= n,
                    vals@.len() == (i - s) as nat,
                    forall|j: int| 0 <= j < (i - s) as int ==> (#[trigger] vals@[j])@ == self.spec_seq()[s as int + j],
                decreases e - i,
            {
                let elem = self.nth(i);
                let val = elem.clone_plus();
                proof {
                    assert(cloned(*elem, val));
                    lemma_cloned_view_eq::<T>(*elem, val);
                }
                vals.push(val);
                i += 1;
            }
            let tree = AVLTreeSeqStEphS::from_vec(vals);
            proof {
                let expected = spec_subseq(self.spec_seq(), start as nat, length as nat);
                assert(expected =~= self.spec_seq().subrange(s as int, e as int));
                assert(tree.spec_seq().len() == (e - s) as nat);
                assert(expected.len() == (e - s) as nat);
                assert(tree.spec_seq() =~= expected);
            }
            tree
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new_root() -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            Self::empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn update(&mut self, index: usize, item: T) {
            assert(self.spec_avltreeseqsteph_wf());
            assert((index as int) < self.spec_seq().len());
            let _ = self.set(index, item);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_vec(values: Vec<T>) -> (tree: AVLTreeSeqStEphS<T>) {
                      assert(obeys_feq_full_trigger::<T>());
            broadcast use Seq::<_>::lemma_push_map_commute;
            let length = values.len();
            let mut t = AVLTreeSeqStEphS { root: None, next_key: 0 };
            let mut i: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < length
                invariant
                    obeys_feq_full::<T>(),
                    i <= length,
                    length == values@.len(),
                    length < usize::MAX,
                    spec_avltreeseqsteph_wf(t.root),
                    spec_inorder(t.root) =~= values@.take(i as int).map_values(|v: T| v@),
                    spec_cached_size(&t.root) == i as nat,
                    t.next_key == i,
                decreases length - i,
            {
                let ghost old_seq = spec_inorder(t.root);
                proof { lemma_size_eq_inorder_len::<T>(&t.root); }
                let cloned_val: T = values[i].clone_plus();
                proof {
                    assert(cloned(values@[i as int], cloned_val));
                    lemma_cloned_view_eq::<T>(values@[i as int], cloned_val);
                }
                t.root = t.root.take().insert_at_link(i, cloned_val, &mut t.next_key);
                proof {
                    assert(old_seq.len() == i as int);
                    assert(values@.take(i as int + 1) =~= values@.take(i as int).push(values@[i as int]));
                    assert(values@.take(i as int + 1).map_values(|v: T| v@) =~=
                        values@.take(i as int).map_values(|v: T| v@).push(values@[i as int]@));
                }
                i += 1;
            }
            proof {
                assert(values@.take(length as int) =~= values@);
            }
            t
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn to_arrayseq(&self) -> (seq: ArraySeqStEphS<T>) {
            assert(self.spec_avltreeseqsteph_wf());
            let n = self.length();
            let mut vals: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<T>(),
                    n as int == self.spec_seq().len(),
                    i <= n,
                    vals@.len() == i as nat,
                    forall|j: int| 0 <= j < i as int ==> (#[trigger] vals@[j])@ == self.spec_seq()[j],
                decreases n - i,
            {
                let elem = self.nth(i);
                let val = elem.clone_plus();
                proof {
                    assert(cloned(*elem, val));
                    lemma_cloned_view_eq::<T>(*elem, val);
                }
                vals.push(val);
                i += 1;
            }
            ArraySeqStEphS::from_vec(vals)
        }

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqIterStEph<'a, T>)
        {
            let mut it = AVLTreeSeqIterStEph {
                stack: Vec::new(),
                current: None,
                elements: Ghost(self.spec_seq()),
                pos: Ghost(0int),
            };
            push_left_iter(&mut it, &self.root);
            it
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn push_back(&mut self, value: T) {
            assert(self.spec_avltreeseqsteph_wf());
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            let ghost old_inorder = spec_inorder(self.root);
            let len = self.length();
            let node = self.root.take().insert_at_link(len, value, &mut self.next_key);
            self.root = node;
            proof {
                assert(spec_inorder(self.root) =~= old_inorder.insert(len as int, value@));
                assert(old_inorder.insert(old_inorder.len() as int, value@) =~= old_inorder.push(value@));
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn contains_value(&self, target: &T) -> (found: bool) {
            assert(self.spec_avltreeseqsteph_wf());
            assert(obeys_feq_full::<T>());
            let n = self.length();
            let ghost seq = self.spec_seq();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<T>(),
                    n as int == seq.len(),
                    seq == self.spec_seq(),
                    i <= n,
                    forall|j: int| 0 <= j < i as int ==> seq[j] != target@,
                decreases n - i,
            {
                let elem = self.nth(i);
                let eq = feq(elem, target);
                if eq {
                    assert(seq[i as int] == target@);
                    return true;
                }
                i += 1;
            }
            false
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert_value(&mut self, value: T) {
            assert(self.spec_avltreeseqsteph_wf());
            self.push_back(value);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn delete_value(&mut self, target: &T) -> (deleted: bool) {
            assert(self.spec_avltreeseqsteph_wf());
            assert(obeys_feq_full::<T>());
            let len = self.length();
            let ghost old_seq = self.spec_seq();
            let mut found_index: usize = len;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<T>(),
                    len as int == old_seq.len(),
                    old_seq == self.spec_seq(),
                    i <= len,
                    found_index <= len,
                    found_index < len ==> old_seq[found_index as int] == target@,
                    found_index == len ==> forall|j: int| 0 <= j < i as int ==> old_seq[j] != target@,
                decreases len - i,
            {
                if found_index == len {
                    let elem = self.nth(i);
                    let eq = feq(elem, target);
                    if eq {
                        found_index = i;
                    }
                }
                i += 1;
            }
            if found_index < len {
                let idx = found_index;
                let mut out_vec: Vec<T> = Vec::new();
                let mut j: usize = 0;
                while j < idx
                    invariant
                        self.spec_avltreeseqsteph_wf(),
                        obeys_feq_full::<T>(),
                        len as int == old_seq.len(),
                        old_seq == self.spec_seq(),
                        j <= idx, idx < len,
                        out_vec@.len() == j as nat,
                        forall|m: int| 0 <= m < j as int ==> (#[trigger] out_vec@[m])@ == old_seq[m],
                    decreases idx - j,
                {
                    let elem = self.nth(j);
                    let val = elem.clone_plus();
                    proof {
                        assert(cloned(*elem, val));
                        lemma_cloned_view_eq::<T>(*elem, val);
                    }
                    out_vec.push(val);
                    j += 1;
                }
                let mut k: usize = idx + 1;
                while k < len
                    invariant
                        self.spec_avltreeseqsteph_wf(),
                        obeys_feq_full::<T>(),
                        len as int == old_seq.len(),
                        old_seq == self.spec_seq(),
                        idx + 1 <= k, k <= len, idx < len,
                        out_vec@.len() == (k - 1) as nat,
                        forall|m: int| 0 <= m < idx as int ==> (#[trigger] out_vec@[m])@ == old_seq[m],
                        forall|m: int| idx as int <= m < (k - 1) as int ==> (#[trigger] out_vec@[m])@ == old_seq[m + 1],
                    decreases len - k,
                {
                    let elem = self.nth(k);
                    let val = elem.clone_plus();
                    proof {
                        assert(cloned(*elem, val));
                        lemma_cloned_view_eq::<T>(*elem, val);
                    }
                    out_vec.push(val);
                    k += 1;
                }
                *self = AVLTreeSeqStEphS::from_vec(out_vec);
                proof {
                    assert(self.spec_seq().len() == (len - 1) as nat);
                    let expected = old_seq.subrange(0, idx as int) + old_seq.subrange(idx as int + 1, len as int);
                    assert(expected.len() == (len - 1) as nat);
                    assert forall|m: int| 0 <= m < expected.len() implies (#[trigger] self.spec_seq()[m]) == expected[m] by {
                        if m < idx as int {
                            assert(expected[m] == old_seq[m]);
                            assert(out_vec@[m]@ == old_seq[m]);
                        } else {
                            assert(expected[m] == old_seq[m + 1]);
                            assert(out_vec@[m]@ == old_seq[m + 1]);
                        }
                    }
                    assert(self.spec_seq() =~= expected);
                }
                true
            } else {
                false
            }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT> PartialEqSpecImpl for AVLTreeSeqStEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT> Default for AVLTreeSeqStEphS<T> {
        fn default() -> Self { Self::new() }
    }

    // 10. iterators

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    // veracity: no_requires
    fn push_left_iter<'a, T: StT>(it: &mut AVLTreeSeqIterStEph<'a, T>, link: &'a Link<T>)
        ensures
            it@.0 == old(it)@.0,
            it@.1 == old(it)@.1,
        decreases *link,
    {
        if let Some(node) = link {
            it.stack.push(node);
            push_left_iter(it, &node.left);
        }
    }

    impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {
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
            let node = self.stack.pop()?;
            let value_ref: &T = &node.value;
            push_left_iter(self, &node.right);
            Some(value_ref)
        }
    }

    impl<'a, T: StT> vstd::pervasive::ForLoopGhostIteratorNew for AVLTreeSeqIterStEph<'a, T> {
        type GhostIter = AVLTreeSeqStEphGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> AVLTreeSeqStEphGhostIterator<'a, T> {
            AVLTreeSeqStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T: StT> vstd::pervasive::ForLoopGhostIterator for AVLTreeSeqStEphGhostIterator<'a, T> {
        type ExecIter = AVLTreeSeqIterStEph<'a, T>;
        type Item = T::V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &AVLTreeSeqIterStEph<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &AVLTreeSeqIterStEph<'a, T>) -> AVLTreeSeqStEphGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT> IntoIterator for &'a AVLTreeSeqStEphS<T> {
        type Item = &'a T;
        type IntoIter = AVLTreeSeqIterStEph<'a, T>;

        fn into_iter(self) -> (it: AVLTreeSeqIterStEph<'a, T>)
            requires self.spec_avltreeseqsteph_wf(),
            ensures
                it@.0 == 0,
                it@.1 == spec_inorder(self.root),
                avltreeseqsteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    // 11. derive impls in verus!

    impl<T: StT> Clone for AVLTreeNode<T> {
        fn clone(&self) -> (copy: Self)
            ensures true,
            decreases *self,
        {
            let left = match &self.left {
                None => None,
                Some(boxed) => Some(Box::new((&**boxed).clone())),
            };
            let right = match &self.right {
                None => None,
                Some(boxed) => Some(Box::new((&**boxed).clone())),
            };
            AVLTreeNode {
                value: self.value.clone(),
                height: self.height,
                left_size: self.left_size,
                right_size: self.right_size,
                left,
                right,
                index: self.index,
            }
        }
    }

    impl<T: StT> Eq for AVLTreeSeqStEphS<T> {}

    impl<T: StT> PartialEq for AVLTreeSeqStEphS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof {
                assume(spec_avltreeseqsteph_wf(self.root));
                assume(spec_avltreeseqsteph_wf(other.root));
                assume(obeys_feq_full::<T>());
            }
            self.root.compare_trees(&other.root)
        }
    }

    impl<T: StT> Clone for AVLTreeSeqStEphS<T> {
        fn clone(&self) -> (copy: Self)
            ensures
                copy@ == self@,
                self.spec_avltreeseqsteph_wf() ==> copy.spec_avltreeseqsteph_wf(),
        {
                      assert(obeys_feq_full_trigger::<T>());
            AVLTreeSeqStEphS {
                root: self.root.clone_link(),
                next_key: self.next_key,
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> Debug for AVLTreeNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AVLTreeNode")
                .field("value", &self.value)
                .field("height", &self.height)
                .field("left_size", &self.left_size)
                .field("right_size", &self.right_size)
                .finish()
        }
    }

    impl<T: StT> Display for AVLTreeNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl<T: StT> Debug for AVLTreeSeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let n = self.length();
            let elts = (0..n).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: StT> Display for AVLTreeSeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let n = self.length();
            for i in 0..n {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", self.nth(i))?;
            }
            write!(f, "]")
        }
    }

    impl<'a, T: StT> Debug for AVLTreeSeqIterStEph<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AVLTreeSeqIterStEph").finish()
        }
    }

    impl<'a, T: StT> Display for AVLTreeSeqIterStEph<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AVLTreeSeqIterStEph")
        }
    }

    impl<'a, T: StT> Debug for AVLTreeSeqStEphGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AVLTreeSeqStEphGhostIterator")
        }
    }

    impl<'a, T: StT> Display for AVLTreeSeqStEphGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AVLTreeSeqStEphGhostIterator")
        }
    }

    //		12. macros

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
