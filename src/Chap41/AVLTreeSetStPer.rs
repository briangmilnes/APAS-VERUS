//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Single-threaded persistent set implementation using BSTParaStEph (Ch38 parametric BST)
//! as backing store.
//!
//! R67: Rewired from AVLTreeSeqStPer (flat sorted array, O(n) find/insert/delete) to
//! BSTParaStEph (BST with recursive split/join, O(log n) operations).
//! Default names are now recursive (via BST); `_iter` variants delegate to defaults.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod AVLTreeSetStPer {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq, lemma_reveal_view_injective};
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
    vstd::laws_cmp::group_laws_cmp,
};

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetStPer<T: StT + Ord + TotalOrder> {
        pub tree: ParamBST<T>,
    }

    pub type AVLTreeSetPer<T> = AVLTreeSetStPer<T>;

    //		Section 5. view impls


    impl<T: StT + Ord + TotalOrder> View for AVLTreeSetStPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
    }

    //		Section 6. spec fns


    /// In-order traversal returning actual values (Seq<T>), not views, for persistent trees.
    /// Kept for compatibility with external callers (uses AVLTreeSeqStPer Link<T>).
    pub open spec fn spec_inorder_values_per<T: StT>(link: Link<T>) -> Seq<T>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => spec_inorder_values_per(node.left) + seq![node.value] + spec_inorder_values_per(node.right),
        }
    }

    /// A sequence of T is sorted under TotalOrder::le (persistent variant).
    /// Kept for compatibility.
    pub open spec fn spec_seq_sorted_per<T: TotalOrder>(s: Seq<T>) -> bool {
        forall|i: int, j: int| 0 <= i < j < s.len()
            ==> (#[trigger] TotalOrder::le(s[i], s[j]))
    }

    //		Section 7. proof fns/broadcast groups


    /// The values sequence maps to the views sequence element-by-element (persistent variant).
    /// Kept for compatibility.
    pub proof fn lemma_inorder_values_maps_to_views_per<T: StT>(link: &Link<T>)
        ensures spec_inorder_values_per(*link).map_values(|t: T| t@) =~= spec_inorder(*link),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_inorder_values_maps_to_views_per::<T>(&node.left);
                lemma_inorder_values_maps_to_views_per::<T>(&node.right);
                assert(
                    (spec_inorder_values_per(node.left)
                        + seq![node.value]
                        + spec_inorder_values_per(node.right))
                    .map_values(|t: T| t@) =~=
                        spec_inorder_values_per(node.left).map_values(|t: T| t@)
                        + seq![node.value].map_values(|t: T| t@)
                        + spec_inorder_values_per(node.right).map_values(|t: T| t@));
            }
        }
    }

    /// Appending an element >= all existing preserves sortedness (persistent variant).
    /// Kept for compatibility.
    proof fn lemma_push_sorted_per<T: TotalOrder>(s: Seq<T>, v: T)
        requires
            spec_seq_sorted_per(s),
            s.len() > 0 ==> TotalOrder::le(s.last(), v),
        ensures
            spec_seq_sorted_per(s.push(v)),
    {
        let new_s = s.push(v);
        assert forall|i: int, j: int| 0 <= i < j < new_s.len()
            implies #[trigger] TotalOrder::le(new_s[i], new_s[j]) by {
            if j < s.len() as int {
                assert(TotalOrder::le(s[i], s[j]));
            } else {
                if i == s.len() as int - 1 {
                    assert(s[i] == s[s.len() as int - 1]);
                } else {
                    let last_idx = s.len() as int - 1;
                    assert(TotalOrder::le(s[i], s[last_idx]));
                    T::transitive(s[i], s[last_idx], v);
                }
            }
        };
    }

    /// If two sequences have equal mapped views and feq holds, the sequences are equal.
    /// Kept for compatibility.
    proof fn lemma_map_view_feq_implies_ext_eq_per<T: View + Eq + Clone>(a: Seq<T>, b: Seq<T>)
        requires
            a.map_values(|t: T| t@) =~= b.map_values(|t: T| t@),
            obeys_feq_full::<T>(),
        ensures
            a =~= b,
    {
        lemma_reveal_view_injective::<T>();
        assert(a.map_values(|t: T| t@).len() == a.len());
        assert(b.map_values(|t: T| t@).len() == b.len());
        assert(a.len() == b.len());
        assert forall|k: int| 0 <= k < a.len()
            implies #[trigger] a[k] == b[k] by {
            assert(0 <= k && k < b.len());
            assert(a.map_values(|t: T| t@)[k] == a[k]@);
            assert(b.map_values(|t: T| t@)[k] == b[k]@);
            assert(a.map_values(|t: T| t@)[k] == b.map_values(|t: T| t@)[k]);
            assert(a[k]@ == b[k]@);
        };
    }

    /// Subsequence of a sorted sequence is sorted (persistent variant).
    /// Kept for compatibility.
    proof fn lemma_subseq_sorted_per<T: TotalOrder>(s: Seq<T>, lo: int, hi: int)
        requires
            spec_seq_sorted_per(s),
            0 <= lo <= hi <= s.len(),
        ensures
            spec_seq_sorted_per(s.subrange(lo, hi)),
    {
        let sub = s.subrange(lo, hi);
        assert forall|i: int, j: int| 0 <= i < j < sub.len()
            implies #[trigger] TotalOrder::le(sub[i], sub[j]) by {
            assert(sub[i] == s[lo + i]);
            assert(sub[j] == s[lo + j]);
        };
    }

    //		Section 8. traits


    pub trait AVLTreeSetStPerTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetstper_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetstper_wf(),
            ensures count == self@.len();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential in-order traversal
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            requires self.spec_avltreesetstper_wf(),
            ensures
                seq.spec_avltreeseqstper_wf(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetstper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetstper_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — DIFFERS: sequential loop of inserts
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            requires
                seq.spec_avltreeseqstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetstper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u + Σ W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(Σ W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + Σ W(f(x))), Span O(n + Σ W(f(x))) — DIFFERS: sequential filter
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetstper_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — DIFFERS: sequential split-join
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetstper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — DIFFERS: sequential split-join
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetstper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — DIFFERS: sequential split-join
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetstper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        fn find(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        fn delete(&self, x: &T) -> (updated: Self)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                updated@ == self@.remove(x@),
                updated.spec_avltreesetstper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        fn insert(&self, x: T) -> (updated: Self)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                updated@ == self@.insert(x@),
                updated.spec_avltreesetstper_wf();
        /// Iterative alternative to `find`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find_iter(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// Iterative alternative to `insert`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_iter(&self, x: T) -> (updated: Self)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                updated@ == self@.insert(x@),
                updated.spec_avltreesetstper_wf();
        /// Iterative alternative to `delete`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete_iter(&self, x: &T) -> (updated: Self)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                updated@ == self@.remove(x@),
                updated.spec_avltreesetstper_wf();
        /// Iterative alternative to `filter`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_iter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetstper_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// Iterative alternative to `intersection`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn intersection_iter(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetstper_wf();
        /// Iterative alternative to `union`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn union_iter(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetstper_wf();
        /// Iterative alternative to `difference`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn difference_iter(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetstper_wf();
    }

    pub trait AVLTreeSetStPerTotalOrderTrait<T: StT + Ord + TotalOrder>: AVLTreeSetStPerTrait<T> {
        /// The backing sequence is sorted under TotalOrder::le.
        spec fn spec_elements_sorted_per(&self) -> bool;
        /// The value-level backing sequence.
        spec fn spec_values_seq_per(&self) -> Seq<T>;
        /// Insert preserving sortedness.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert_sorted_per(&self, x: T) -> (updated: Self)
            requires
                self.spec_avltreesetstper_wf(),
                self.spec_elements_sorted_per(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                updated@ == self@.insert(x@),
                updated.spec_avltreesetstper_wf(),
                updated.spec_elements_sorted_per();
    }

    //		Section 9. impls


    impl<T: StT + Ord + TotalOrder> AVLTreeSetStPerTrait<T> for AVLTreeSetStPer<T> {
        open spec fn spec_avltreesetstper_wf(&self) -> bool {
            self.tree.spec_bstparasteph_wf()
            && self@.len() < usize::MAX as nat
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            self.tree.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
        {
            let in_ord = self.tree.in_order();
            let result = AVLTreeSeqStPerS::from_vec(in_ord.seq);
            proof {
                assert(result@.len() == in_ord@.len());
                assert forall|i: int| 0 <= i < result@.len()
                    implies #[trigger] result@[i] == in_ord@[i] by {};
                assert(result@ =~= in_ord@);
                assert forall|v: <T as View>::V| #[trigger] result@.to_set().contains(v)
                    implies self@.contains(v) by {
                    assert(result@.contains(v));
                };
                assert forall|v: <T as View>::V| self@.contains(v)
                    implies #[trigger] result@.to_set().contains(v) by {
                    let j = choose|j: int| 0 <= j < in_ord@.len() && in_ord@[j] == v;
                    assert(result@[j] == in_ord@[j]);
                };
                assert(result@.to_set() =~= self@);
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            AVLTreeSetStPer { tree: ParamBST::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            AVLTreeSetStPer { tree: ParamBST::singleton(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqstper_wf(),
                    n as int == seq.spec_seq().len(),
                    i <= n,
                    constructed.spec_avltreesetstper_wf(),
                    constructed@.len() <= i as nat,
                    vstd::laws_cmp::obeys_cmp_spec::<T>(),
                    view_ord_consistent::<T>(),
                    forall|j: int| 0 <= j < i ==> #[trigger] constructed@.contains(seq@[j]),
                    forall|v: <T as View>::V| #[trigger] constructed@.contains(v) ==>
                        (exists|j: int| 0 <= j < i && seq@[j] == v),
                decreases n - i,
            {
                let r = seq.nth(i);
                let elem = r.clone();
                proof {
                    lemma_cloned_view_eq(*r, elem);
                    lemma_size_lt_usize_max::<T>(&seq.root);
                    lemma_size_eq_inorder_len::<T>(&seq.root);
                }
                let ghost old_view = constructed@;
                constructed = constructed.insert(elem);
                proof {
                    assert forall|j: int| 0 <= j < i + 1
                        implies #[trigger] constructed@.contains(seq@[j]) by {
                        if j < i as int {
                            assert(old_view.contains(seq@[j]));
                        }
                    };
                    assert forall|v: <T as View>::V|
                        #[trigger] constructed@.contains(v) implies
                        (exists|j: int| 0 <= j < i + 1 && seq@[j] == v) by {
                        if !old_view.contains(v) {
                            assert(v == seq@[i as int]);
                        } else {
                            let j = choose|j: int| 0 <= j < i && seq@[j] == v;
                            assert(j < i + 1);
                        }
                    };
                }
                i += 1;
            }
            proof {
                assert forall|v: <T as View>::V|
                    #[trigger] constructed@.contains(v) == seq@.to_set().contains(v) by {
                    if constructed@.contains(v) {
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == v;
                        assert(seq@.contains(v));
                    }
                    if seq@.to_set().contains(v) {
                        assert(seq@.contains(v));
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == v;
                        assert(constructed@.contains(seq@[j]));
                    }
                };
            }
            constructed
        }

        /// Recursive find via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find(&self, x: &T) -> (found: bool)
        {
            self.tree.find(x).is_some()
        }

        /// Persistent insert via BSTParaStEph: clone, mutate, wrap.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&self, x: T) -> (updated: Self)
        {
            let mut tree = self.tree.clone();
            tree.insert(x);
            AVLTreeSetStPer { tree }
        }

        /// Persistent delete via BSTParaStEph: clone, mutate, wrap.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete(&self, x: &T) -> (updated: Self)
        {
            let mut tree = self.tree.clone();
            tree.delete(x);
            AVLTreeSetStPer { tree }
        }

        /// Recursive filter via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let filtered_tree = self.tree.filter(f, Ghost(spec_pred));
            proof {
                vstd::set_lib::lemma_len_subset(filtered_tree@, self@);
            }
            AVLTreeSetStPer { tree: filtered_tree }
        }

        /// Recursive intersection via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let common_tree = self.tree.intersect(&other.tree);
            proof {
                assert(common_tree@ =~= self@.intersect(other@));
                vstd::set_lib::lemma_len_intersect::<T::V>(self@, other@);
            }
            AVLTreeSetStPer { tree: common_tree }
        }

        /// Recursive union via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let combined_tree = self.tree.union(&other.tree);
            proof {
                assert(combined_tree@ =~= self@.union(other@));
                vstd::set_lib::lemma_len_union::<T::V>(self@, other@);
            }
            AVLTreeSetStPer { tree: combined_tree }
        }

        /// Recursive difference via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let remaining_tree = self.tree.difference(&other.tree);
            proof {
                assert(remaining_tree@ =~= self@.difference(other@));
                vstd::set_lib::lemma_len_difference::<T::V>(self@, other@);
            }
            AVLTreeSetStPer { tree: remaining_tree }
        }

        /// Iterative alternative to `find`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find_iter(&self, x: &T) -> (found: bool)
        {
            self.find(x)
        }

        /// Iterative alternative to `insert`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_iter(&self, x: T) -> (updated: Self)
        {
            self.insert(x)
        }

        /// Iterative alternative to `delete`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete_iter(&self, x: &T) -> (updated: Self)
        {
            self.delete(x)
        }

        /// Iterative alternative to `filter`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_iter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            self.filter(f, Ghost(spec_pred))
        }

        /// Iterative alternative to `intersection`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn intersection_iter(&self, other: &Self) -> (common: Self)
        {
            self.intersection(other)
        }

        /// Iterative alternative to `union`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn union_iter(&self, other: &Self) -> (combined: Self)
        {
            self.union(other)
        }

        /// Iterative alternative to `difference`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn difference_iter(&self, other: &Self) -> (remaining: Self)
        {
            self.difference(other)
        }
    }


    impl<T: StT + Ord + TotalOrder> AVLTreeSetStPerTotalOrderTrait<T> for AVLTreeSetStPer<T> {
        /// With BST backing, sorted is always true by construction.
        open spec fn spec_elements_sorted_per(&self) -> bool {
            true
        }

        /// Placeholder — not meaningful with BST backing (no accessible inorder sequence).
        open spec fn spec_values_seq_per(&self) -> Seq<T> {
            Seq::empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert_sorted_per(&self, x: T) -> (updated: Self)
        {
            self.insert(x)
        }
    }

    impl<T: StT + Ord + TotalOrder> ClonePreservesWf for AVLTreeSetStPer<T> {
        open spec fn spec_wf(&self) -> bool { self.spec_avltreesetstper_wf() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn clone_wf(&self) -> (cloned: Self) {
            let r = AVLTreeSetStPer { tree: self.tree.clone() };
            proof {
                assert(r.tree@ == self.tree@);
                assert(obeys_feq_full_trigger::<T>());
                assert(r.tree@.finite());
                assert(r.tree.spec_bstparasteph_wf());
                assert(r@.len() < usize::MAX as nat);
            }
            r
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: StT + Ord + TotalOrder> Default for AVLTreeSetStPer<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for AVLTreeSetStPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord + TotalOrder> Eq for AVLTreeSetStPer<T> {}

    impl<T: StT + Ord + TotalOrder> PartialEq for AVLTreeSetStPer<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof {
                assume(self.spec_avltreesetstper_wf());
                assume(other.spec_avltreesetstper_wf());
                assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
                assume(view_ord_consistent::<T>());
            }
            let equal = self.size() == other.size() && self.difference(other).size() == 0;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: StT + Ord + TotalOrder> Clone for AVLTreeSetStPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            AVLTreeSetStPer { tree: self.tree.clone() }
        }
    }
    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! AVLTreeSetStPerLit {
        () => {
            < $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPer<_> as $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPerTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPer<_> as $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPerTrait<_> >::empty();
            $( __set = __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder> fmt::Debug for AVLTreeSetStPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut v: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut v);
            write!(f, "{{")?;
            for i in 0..v.len() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", v[i])?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Display for AVLTreeSetStPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut v: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut v);
            write!(f, "{{")?;
            for i in 0..v.len() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v[i])?;
            }
            write!(f, "}}")
        }
    }
}
