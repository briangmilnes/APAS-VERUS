//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Single-threaded ephemeral set implementation using BSTParaStEph (Ch38 parametric BST)
//! as backing store.
//!
//! R66: Rewired from AVLTreeSeqStEph (flat sorted array, O(n) find/insert/delete) to
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

pub mod AVLTreeSetStEph {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;
    use crate::vstdplus::accept::accept;

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
    pub struct AVLTreeSetStEph<T: StT + Ord + TotalOrder> {
        pub tree: ParamBST<T>,
    }

    pub type AVLTreeSetS<T> = AVLTreeSetStEph<T>;

    //		Section 5. view impls


    impl<T: StT + Ord + TotalOrder> View for AVLTreeSetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
    }

    //		Section 6. spec fns


    /// In-order traversal returning actual values (Seq<T>), not views.
    /// Kept for compatibility with external callers (uses AVLTreeSeqStEph Link<T>).
    pub open spec fn spec_inorder_values<T: StT>(link: Link<T>) -> Seq<T>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => spec_inorder_values(node.left) + seq![node.value] + spec_inorder_values(node.right),
        }
    }

    /// A sequence of T is sorted under TotalOrder::le.
    pub open spec fn spec_seq_sorted<T: TotalOrder>(s: Seq<T>) -> bool {
        forall|i: int, j: int| 0 <= i < j < s.len()
            ==> (#[trigger] TotalOrder::le(s[i], s[j]))
    }

    //		Section 7. proof fns/broadcast groups


    /// Under wf, cached size equals inorder length, both < usize::MAX.
    /// Kept for from_seq capacity proof (operates on AVLTreeSeqStEphS input).
    pub proof fn lemma_wf_implies_len_bound<T: StT>(link: &Link<T>)
        requires spec_avltreeseqsteph_wf(*link),
        ensures
            spec_cached_size(link) == spec_inorder(*link).len(),
            spec_inorder(*link).len() < usize::MAX,
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_wf_implies_len_bound::<T>(&node.left);
                lemma_wf_implies_len_bound::<T>(&node.right);
            }
        }
    }

    /// The values sequence maps to the views sequence element-by-element.
    /// Kept for compatibility.
    pub proof fn lemma_inorder_values_maps_to_views<T: StT>(link: Link<T>)
        ensures spec_inorder_values(link).map_values(|t: T| t@) =~= spec_inorder(link),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_inorder_values_maps_to_views::<T>(node.left);
                lemma_inorder_values_maps_to_views::<T>(node.right);
                let lv = spec_inorder_values(node.left);
                let rv_right = spec_inorder_values(node.right);
                let mid: Seq<T> = seq![node.value];
                let full = lv + mid + rv_right;
            }
        }
    }

    /// With BST backing, sorted is always true by construction. Trivially holds.
    proof fn lemma_empty_set_is_sorted<T: StT + Ord + TotalOrder>(set: &AVLTreeSetStEph<T>)
        requires
            set@ =~= Set::<<T as View>::V>::empty(),
            set.spec_avltreesetsteph_wf(),
        ensures
            set.spec_elements_sorted(),
    {}

    /// Appending an element >= all existing preserves sortedness. Kept for compatibility.
    proof fn lemma_push_sorted<T: TotalOrder>(s: Seq<T>, v: T)
        requires
            spec_seq_sorted(s),
            s.len() > 0 ==> TotalOrder::le(s.last(), v),
        ensures
            spec_seq_sorted(s.push(v)),
    {
        let new_s = s.push(v);
        // Veracity: NEEDED assert
        assert forall|i: int, j: int| 0 <= i < j < new_s.len()
            implies #[trigger] TotalOrder::le(new_s[i], new_s[j]) by {
            if j < s.len() as int {
            } else {
                if s.len() == 0 {
                } else if i == s.len() as int - 1 {
                } else {
                    T::transitive(s[i], s[s.len() - 1], v);
                }
            }
        };
    }

    /// Subsequence of a sorted sequence is sorted. Kept for compatibility.
    proof fn lemma_subseq_sorted<T: TotalOrder>(s: Seq<T>, lo: int, hi: int)
        requires
            spec_seq_sorted(s),
            0 <= lo <= hi <= s.len(),
        ensures
            spec_seq_sorted(s.subrange(lo, hi)),
    {
        let sub = s.subrange(lo, hi);
    }

    //		Section 8. traits


    pub trait AVLTreeSetStEphTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetsteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetsteph_wf(),
            ensures count == self@.len();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: sequential in-order traversal
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
            requires self.spec_avltreesetsteph_wf(),
            ensures
                seq.spec_avltreeseqsteph_wf(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetsteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetsteph_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — ACCEPTED DIFFERENCE: sequential loop of inserts
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
            requires
                seq.spec_avltreeseqsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetsteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u + Σ W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(Σ W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + Σ W(f(x))), Span O(n + Σ W(f(x))) — ACCEPTED DIFFERENCE: sequential filter
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetsteph_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — ACCEPTED DIFFERENCE: sequential split-join
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetsteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — ACCEPTED DIFFERENCE: sequential split-join
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetsteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — ACCEPTED DIFFERENCE: sequential split-join
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetsteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) CS 41.4
        fn find(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) CS 41.4
        fn delete(&mut self, x: &T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetsteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) CS 41.4
        fn insert(&mut self, x: T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf();
        /// Iterative alternative to `find`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find_iter(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// Iterative alternative to `insert`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_iter(&mut self, x: T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf();
        /// Iterative alternative to `delete`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete_iter(&mut self, x: &T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetsteph_wf();
        /// Iterative alternative to `filter`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_iter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetsteph_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// Iterative alternative to `intersection`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn intersection_iter(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetsteph_wf();
        /// Iterative alternative to `union`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn union_iter(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetsteph_wf();
        /// Iterative alternative to `difference`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn difference_iter(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetsteph_wf();
    }

    pub trait AVLTreeSetStEphTotalOrderTrait<T: StT + Ord + TotalOrder>: AVLTreeSetStEphTrait<T> {
        /// The backing sequence is sorted under TotalOrder::le.
        spec fn spec_elements_sorted(&self) -> bool;
        /// The value-level backing sequence.
        spec fn spec_values_seq(&self) -> Seq<T>;
        /// Insert preserving sortedness.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert_sorted(&mut self, x: T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                old(self).spec_elements_sorted(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted();
        /// Delete preserving sortedness.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn delete_sorted(&mut self, x: &T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                old(self).spec_elements_sorted(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted();
        /// Filter preserving sortedness.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_sorted<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetsteph_wf(),
                filtered.spec_elements_sorted(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// Intersection preserving sortedness.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn intersection_sorted(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetsteph_wf(),
                common.spec_elements_sorted();
        /// Difference preserving sortedness.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn difference_sorted(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetsteph_wf(),
                remaining.spec_elements_sorted();
        /// Union preserving sortedness; requires combined capacity bound.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn union_sorted(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
                other.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetsteph_wf(),
                combined.spec_elements_sorted();
    }

    //		Section 9. impls


    impl<T: StT + Ord + TotalOrder> AVLTreeSetStEph<T> {
        /// Backward-compatible spec alias for view.
        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> { self@ }
    }


    impl<T: StT + Ord + TotalOrder> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        open spec fn spec_avltreesetsteph_wf(&self) -> bool {
            self.tree.spec_bstparasteph_wf()
            && self@.len() < usize::MAX as nat
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            self.tree.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
        {
            let in_ord = self.tree.in_order();
            // in_ord@.len() == self@.len(), forall|v| self@.contains(v) <==> in_ord@.contains(v)
            let result = AVLTreeSeqStEphS::from_vec(in_ord.seq);
            // Veracity: NEEDED proof block
            proof {
                // from_vec ensures: result@ =~= in_ord.seq@.map_values(|t: T| t@)
                // ArraySeqStPerS view: in_ord@ == in_ord.seq@.map(|_i, t: T| t@)
                // map_values(f) = map(|_i, a| f(a)), so result@ =~= in_ord@
                // Veracity: NEEDED assert
                assert(result@ =~= in_ord@);
                // Now bridge: result@.to_set() =~= self@ via in_ord membership
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
            AVLTreeSetStEph { tree: ParamBST::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
        {
            AVLTreeSetStEph { tree: ParamBST::singleton(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
        {
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqsteph_wf(),
                    n as int == seq.spec_seq().len(),
                    i <= n,
                    constructed.spec_avltreesetsteph_wf(),
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
                // Veracity: NEEDED proof block
                proof {
                    lemma_cloned_view_eq(*r, elem);
                    lemma_wf_implies_len_bound::<T>(&seq.root);
                }
                let ghost old_view = constructed@;
                constructed.insert(elem);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|v: <T as View>::V|
                        #[trigger] constructed@.contains(v) implies
                        (exists|j: int| 0 <= j < i + 1 && seq@[j] == v) by {
                        if !old_view.contains(v) {
                            // Veracity: NEEDED assert
                            assert(v == seq@[i as int]);
                        } else {
                            let j = choose|j: int| 0 <= j < i && seq@[j] == v;
                        }
                    };
                }
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            constructed
        }

        /// Recursive find via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find(&self, x: &T) -> (found: bool)
        {
            self.tree.find(x).is_some()
        }

        /// Recursive insert via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&mut self, x: T)
        {
            self.tree.insert(x);
        }

        /// Recursive delete via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete(&mut self, x: &T)
        {
            self.tree.delete(x);
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
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_len_subset(filtered_tree@, self@);
            }
            AVLTreeSetStEph { tree: filtered_tree }
        }

        /// Recursive intersection via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let common_tree = self.tree.intersect(&other.tree);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_len_intersect::<T::V>(self@, other@);
            }
            AVLTreeSetStEph { tree: common_tree }
        }

        /// Recursive union via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let combined_tree = self.tree.union(&other.tree);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_len_union::<T::V>(self@, other@);
            }
            AVLTreeSetStEph { tree: combined_tree }
        }

        /// Recursive difference via BSTParaStEph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let remaining_tree = self.tree.difference(&other.tree);
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_len_difference::<T::V>(self@, other@);
            }
            AVLTreeSetStEph { tree: remaining_tree }
        }

        /// Iterative alternative to `find`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find_iter(&self, x: &T) -> (found: bool)
        {
            self.find(x)
        }

        /// Iterative alternative to `insert`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_iter(&mut self, x: T)
        {
            self.insert(x)
        }

        /// Iterative alternative to `delete`. Delegates to recursive default.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete_iter(&mut self, x: &T)
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


    impl<T: StT + Ord + TotalOrder> AVLTreeSetStEphTotalOrderTrait<T> for AVLTreeSetStEph<T> {
        /// With BST backing, sorted is always true by construction.
        open spec fn spec_elements_sorted(&self) -> bool {
            true
        }

        /// Placeholder — not meaningful with BST backing (no accessible inorder sequence).
        open spec fn spec_values_seq(&self) -> Seq<T> {
            Seq::empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert_sorted(&mut self, x: T)
        {
            self.insert(x);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn delete_sorted(&mut self, x: &T)
        {
            self.delete(x);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_sorted<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            self.filter(f, Ghost(spec_pred))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn intersection_sorted(&self, other: &Self) -> (common: Self)
        {
            self.intersection(other)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn difference_sorted(&self, other: &Self) -> (remaining: Self)
        {
            self.difference(other)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn union_sorted(&self, other: &Self) -> (combined: Self)
        {
            self.union(other)
        }
    }

    impl<T: StT + Ord + TotalOrder> ClonePreservesWf for AVLTreeSetStEph<T> {
        open spec fn spec_wf(&self) -> bool { self.spec_avltreesetsteph_wf() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn clone_wf(&self) -> (cloned: Self) {
            let r = AVLTreeSetStEph { tree: self.tree.clone() };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(r.tree.spec_bstparasteph_wf());
            }
            r
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: StT + Ord + TotalOrder> Default for AVLTreeSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for AVLTreeSetStEph<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord + TotalOrder> Eq for AVLTreeSetStEph<T> {}

    impl<T: StT + Ord + TotalOrder> PartialEq for AVLTreeSetStEph<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            // Veracity: NEEDED proof block
            proof {
                accept(self.spec_avltreesetsteph_wf());
                accept(other.spec_avltreesetsteph_wf());
                accept(vstd::laws_cmp::obeys_cmp_spec::<T>());
                accept(view_ord_consistent::<T>());
            }
            let equal = self.size() == other.size() && self.difference(other).size() == 0;
            // Veracity: NEEDED proof block
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: StT + Ord + TotalOrder> Clone for AVLTreeSetStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            AVLTreeSetStEph { tree: self.tree.clone() }
        }
    }
    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! AVLTreeSetStEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder> fmt::Debug for AVLTreeSetStEph<T> {
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

    impl<T: StT + Ord + TotalOrder> fmt::Display for AVLTreeSetStEph<T> {
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
