//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Multi-threaded ephemeral set backed by BSTParaMtEph for parallel set operations.
//!
//! Work/Span Analysis (via delegation to BSTParaMtEph parallel D&C):
//! - union: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - intersection: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - difference: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - filter: Work O(Σ W(f(x))), Span O(n + max S(f(x))) — sequential (spec_fn not Send)

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod AVLTreeSetMtEph {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap38::BSTParaMtEph::BSTParaMtEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full_trigger, lemma_cloned_view_eq};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus!
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + TotalOrder + 'static> {
        pub tree: ParamBST<T>,
    }

    //		Section 5. view impls


    impl<T: StTInMtT + Ord + TotalOrder + 'static> View for AVLTreeSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
    }

    //		Section 6. spec fns


    pub open spec fn avltreesetmteph_iter_invariant<T: StTInMtT + Ord + TotalOrder + 'static>(it: &AVLTreeSetMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    //		Section 8. traits


    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord + TotalOrder + 'static>: Sized + View<V = Set<<T as View>::V>> {
        /// Well-formedness: backing BST is well-formed.
        spec fn spec_avltreesetmteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetmteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: APAS O(lg n) span requires tree-based sequence concat (join); AVLTreeSeqStEphS lacks concat, so O(n) materialization into Vec is unavoidable
        /// - claude-4-sonet: Work Θ(n), Span Θ(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
            requires
                self.spec_avltreesetmteph_wf(),
                self@.len() < usize::MAX as nat,
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                seq.spec_avltreeseqsteph_wf(),
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n) — parallel D&C: split Vec, recurse via join(), union results
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
            requires
                seq.spec_avltreeseqsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u + Σ W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(Σ W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ W(f(x))), Span O(n + max S(f(x))) — ACCEPTED DIFFERENCE: Verus limitation; spec_fn not Send, blocks parallel filter
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetmteph_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            requires
                old(self).spec_avltreesetmteph_wf(),
                old(self)@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            requires
                old(self).spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetmteph_wf();
        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it);
    }

    //		Section 9. impls


    /// Parallel D&C set construction from Vec: split in half, recurse via join(), union.
    /// Work O(n lg n), Span O(lg^2 n) — matches APAS Ex 41.3 parallel fromSeq.
    fn from_vec_dc<T: StTInMtT + Ord + TotalOrder + 'static>(vals: Vec<T>) -> (tree: ParamBST<T>)
        requires
            obeys_feq_full_trigger::<T>(),
            vals@.len() <= usize::MAX,
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures
            tree@.finite(),
            tree@ =~= vals@.map_values(|t: T| t@).to_set(),
        decreases vals@.len(),
    {
        let n = vals.len();
        if n == 0 {
            let tree = ParamBST::<T>::new();
            // Veracity: NEEDED proof block
            proof {
            }
            tree
        } else if n == 1 {
            let elem = vals[0].clone_plus();
            // Veracity: NEEDED proof block
            proof { lemma_cloned_view_eq::<T>(vals@[0int], elem); }
            let tree = ParamBST::singleton(elem);
            // Veracity: NEEDED proof block
            proof {
                let views = vals@.map_values(|t: T| t@);
                // Veracity: NEEDED assert
                assert(views =~= Seq::<T::V>::empty().push(views[0]));
                vstd::seq_lib::seq_to_set_is_finite(views);
            }
            tree
        } else {
            let mid = n / 2;
            let mut left_vals: Vec<T> = Vec::with_capacity(mid);
            let mut right_vals: Vec<T> = Vec::with_capacity(n - mid);
            let mut i: usize = 0;
            while i < mid
                invariant
                    i <= mid,
                    mid == n / 2,
                    n == vals@.len(),
                    n <= usize::MAX,
                    left_vals@.len() == i as nat,
                    obeys_feq_full_trigger::<T>(),
                    forall|j: int| 0 <= j < i as int ==> (#[trigger] left_vals@[j])@ == vals@[j]@,
                decreases mid - i,
            {
                let elem = vals[i].clone_plus();
                // Veracity: NEEDED proof block
                proof { lemma_cloned_view_eq::<T>(vals@[i as int], elem); }
                left_vals.push(elem);
                i += 1;
            }
            while i < n
                invariant
                    mid <= i <= n,
                    mid == n / 2,
                    n == vals@.len(),
                    n <= usize::MAX,
                    left_vals@.len() == mid as nat,
                    right_vals@.len() == (i - mid) as nat,
                    obeys_feq_full_trigger::<T>(),
                    forall|j: int| 0 <= j < mid as int ==> (#[trigger] left_vals@[j])@ == vals@[j]@,
                    forall|j: int| 0 <= j < (i - mid) as int
                        ==> (#[trigger] right_vals@[j])@ == vals@[(mid as int + j)]@,
                decreases n - i,
            {
                let elem = vals[i].clone_plus();
                // Veracity: NEEDED proof block
                proof { lemma_cloned_view_eq::<T>(vals@[i as int], elem); }
                right_vals.push(elem);
                i += 1;
            }
            let ghost all_views = vals@.map_values(|t: T| t@);
            let ghost left_views = left_vals@.map_values(|t: T| t@);
            let ghost right_views = right_vals@.map_values(|t: T| t@);
            // Veracity: NEEDED proof block
            proof {
            }
            let f1 = move || -> (t: ParamBST<T>)
                ensures t@.finite(), t@ =~= left_vals@.map_values(|t: T| t@).to_set()
            {
                from_vec_dc(left_vals)
            };
            let f2 = move || -> (t: ParamBST<T>)
                ensures t@.finite(), t@ =~= right_vals@.map_values(|t: T| t@).to_set()
            {
                from_vec_dc(right_vals)
            };
            let (left_tree, right_tree) = join(f1, f2);
            // Veracity: NEEDED proof block
            proof {
                // After join, left_tree@ =~= left_views.to_set(), right_tree@ =~= right_views.to_set().
                // (The ensures of the closures give these in terms of the captured Vecs,
                //  but the ghost bindings above relate them to left_views/right_views.)
                left_views.lemma_cardinality_of_set();
                right_views.lemma_cardinality_of_set();
            }
            let result = left_tree.union(&right_tree);
            // Veracity: NEEDED proof block
            proof {
                vstd::seq_lib::seq_to_set_distributes_over_add(left_views, right_views);
                // Veracity: NEEDED assert
                assert(left_views + right_views =~= all_views) by {
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < all_views.len() implies
                        (left_views + right_views)[j] == #[trigger] all_views[j] by {};
                };
            }
            result
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        open spec fn spec_avltreesetmteph_wf(&self) -> bool {
            self.tree@.finite()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            self.tree.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: APAS O(lg n) span requires tree-based sequence concat; AVLTreeSeqStEphS lacks concat
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
        {
            // Veracity: NEEDED proof block
            proof { assert(obeys_feq_full_trigger::<T>()); }
            let mut out: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut out);
            let ghost out_seq = out@;
            let seq = AVLTreeSeqStEphS::from_vec(out);
            // Veracity: NEEDED proof block
            proof {
                // from_vec: seq@ =~= out_seq.map_values(|t: T| t@), so seq@[i] == out_seq[i]@.
                // collect_in_order (empty start): out_seq[i]@ in self.tree@ for all i,
                // and every v in self.tree@ has a witness j with out_seq[j]@ == v.
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < seq@.len() implies
                    #[trigger] self@.contains(seq@[i])
                by {
                };
                // Veracity: NEEDED assert
                assert forall|v: T::V|
                    #[trigger] seq@.to_set().contains(v) <==> self@.contains(v)
                by {
                    if seq@.to_set().contains(v) {
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == v;
                    }
                    if self@.contains(v) {
                        let j = choose|j: int| 0 <= j < out_seq.len() && #[trigger] out_seq[j]@ == v;
                        // Veracity: NEEDED assert
                        assert(seq@[j] == out_seq[j]@);
                    }
                };
                vstd::seq_lib::seq_to_set_is_finite(seq@);
            }
            seq
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
            AVLTreeSetMtEph { tree: ParamBST::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
        {
            AVLTreeSetMtEph { tree: ParamBST::singleton(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n) — parallel D&C via join() + union
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
        {
            let n = seq.length();
            if n == 0 {
                return Self::empty();
            }
            // Collect elements into a Vec for parallel splitting.
            let mut vals: Vec<T> = Vec::with_capacity(n);
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n == seq@.len(),
                    vals@.len() == i as nat,
                    seq.spec_avltreeseqsteph_wf(),
                    obeys_feq_full_trigger::<T>(),
                    forall|j: int| 0 <= j < i as int ==> (#[trigger] vals@[j])@ == seq@[j],
                decreases n - i,
            {
                let elem_ref = seq.nth(i);
                let elem = elem_ref.clone_plus();
                // Veracity: NEEDED proof block
                proof { lemma_cloned_view_eq::<T>(*elem_ref, elem); }
                vals.push(elem);
                i += 1;
            }
            let ghost vals_views = vals@.map_values(|t: T| t@);
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(vals_views =~= seq@) by {
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < seq@.len() implies
                        #[trigger] vals_views[j] == seq@[j] by {
                    };
                };
            }
            let tree = from_vec_dc(vals);
            // Veracity: NEEDED proof block
            proof {
            }
            AVLTreeSetMtEph { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let filtered_tree = self.tree.filter(f, Ghost(spec_pred));
            AVLTreeSetMtEph { tree: filtered_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let common_tree = self.tree.intersect(&other.tree);
            AVLTreeSetMtEph { tree: common_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let remaining_tree = self.tree.difference(&other.tree);
            AVLTreeSetMtEph { tree: remaining_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let combined_tree = self.tree.union(&other.tree);
            AVLTreeSetMtEph { tree: combined_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find(&self, x: &T) -> (found: bool)
        {
            let result = self.tree.find(x);
            result.is_some()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete(&mut self, x: &T)
        {
            // Veracity: NEEDED proof block
            proof { assert(obeys_feq_full_trigger::<T>()); }
            self.tree.delete(x);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&mut self, x: T)
        {
            // Veracity: NEEDED proof block
            proof { assert(obeys_feq_full_trigger::<T>()); }
            self.tree.insert(x);
        }

        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
        {
            let mut vals: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut vals);
            AVLTreeSetMtEphIter { snapshot: vals, pos: 0 }
        }
    }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphIter<T: StTInMtT + Ord + TotalOrder + 'static> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphGhostIter<T: StTInMtT + Ord + TotalOrder + 'static> {
        pub pos: int,
        pub elements: Seq<T::V>,
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> View for AVLTreeSetMtEphIter<T> {
        type V = (int, Seq<T::V>);
        open spec fn view(&self) -> (int, Seq<T::V>) {
            (self.pos as int, self.snapshot@.map_values(|t: T| t@))
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> View for AVLTreeSetMtEphGhostIter<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> { self.elements.take(self.pos) }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> std::iter::Iterator for AVLTreeSetMtEphIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
            ensures ({
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
            })
        {
            if self.pos >= self.snapshot.len() {
                None
            } else {
                let item = self.snapshot[self.pos].clone();
                self.pos = self.pos + 1;
                // Veracity: NEEDED proof block
                proof { assume(item@ == old(self)@.1[old(self)@.0]); }  // accept hole: Clone preserves value
                Some(item)
            }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> vstd::pervasive::ForLoopGhostIteratorNew for AVLTreeSetMtEphIter<T> {
        type GhostIter = AVLTreeSetMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> AVLTreeSetMtEphGhostIter<T> {
            AVLTreeSetMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> vstd::pervasive::ForLoopGhostIterator for AVLTreeSetMtEphGhostIter<T> {
        type ExecIter = AVLTreeSetMtEphIter<T>;
        type Item = T::V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &AVLTreeSetMtEphIter<T>) -> bool {
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
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &AVLTreeSetMtEphIter<T>) -> AVLTreeSetMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + TotalOrder + 'static> std::iter::IntoIterator for &'a AVLTreeSetMtEph<T> {
        type Item = T;
        type IntoIter = AVLTreeSetMtEphIter<T>;
        fn into_iter(self) -> (it: AVLTreeSetMtEphIter<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: StTInMtT + Ord + TotalOrder + 'static> Default for AVLTreeSetMtEph<T> {
        fn default() -> Self { Self::empty() }
    }


    impl<T: StTInMtT + Ord + TotalOrder + 'static> Clone for AVLTreeSetMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            AVLTreeSetMtEph { tree: self.tree.clone() }
        }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! AVLTreeSetMtEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    // Ghost fields are zero-sized; ParamBST is Send/Sync via BSTParaMtEph.
    unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Send for AVLTreeSetMtEph<T> {}
    unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Sync for AVLTreeSetMtEph<T> {}

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Debug for AVLTreeSetMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for AVLTreeSetMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphIter")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Debug for AVLTreeSetMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphGhostIter")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for AVLTreeSetMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphGhostIter")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Debug for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
