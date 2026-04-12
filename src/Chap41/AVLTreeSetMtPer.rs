//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Multi-threaded persistent set backed by BSTParaMtEph for parallel set operations.
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
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators — AVLTreeSetMtPer
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod AVLTreeSetMtPer {


    //		Section 2. imports

    use std::cmp::Ordering::{self, Equal, Greater, Less};
    use std::fmt;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full_trigger, lemma_cloned_view_eq};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::strictly_cloned;

    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
    use crate::Chap38::BSTParaMtEph::BSTParaMtEph::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::accept::accept;

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
    pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + TotalOrder + 'static> {
        pub tree: ParamBST<T>,
    }

    //		Section 5. view impls


    impl<T: StTInMtT + Ord + TotalOrder + 'static> View for AVLTreeSetMtPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
    }

    //		Section 8. traits


    pub trait AVLTreeSetMtPerTrait<T: StTInMtT + Ord + TotalOrder + 'static>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetmtper_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetmtper_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: APAS O(lg n) span requires tree-based sequence concat (join); AVLTreeSeqMtPerS lacks concat, so O(n) materialization into Vec is unavoidable
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqMtPerS<T>)
            requires
                self.spec_avltreesetmtper_wf(),
                self@.len() < usize::MAX as nat,
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                seq.spec_avltreeseqmtper_wf(),
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_avltreesetmtper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree.spec_avltreesetmtper_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n) — parallel D&C: split Vec, recurse via join(), union results
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (constructed: Self)
            requires
                seq@.len() <= usize::MAX - 2,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetmtper_wf();
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
                self.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetmtper_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n); parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetmtper_wf(),
                other.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures common@ == self@.intersect(other@), common.spec_avltreesetmtper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n); parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetmtper_wf(),
                other.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures remaining@ == self@.difference(other@), remaining.spec_avltreesetmtper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n); parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetmtper_wf(),
                other.spec_avltreesetmtper_wf(),
                self@.len() + other@.len() <= usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures combined@ == self@.union(other@), combined.spec_avltreesetmtper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (updated: Self)
            requires
                self.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures updated@ == self@.remove(x@), updated.spec_avltreesetmtper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (updated: Self)
            requires
                self.spec_avltreesetmtper_wf(),
                self@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures updated@ == self@.insert(x@), updated.spec_avltreesetmtper_wf();
    }

    //		Section 9. impls


    impl<T: StTInMtT + Ord + TotalOrder + 'static> AVLTreeSetMtPer<T> {
        pub open spec fn spec_avltreesetmtper_wf(&self) -> bool {
            self.tree@.finite()
        }
    }


    /// ParamBST type_invariant guarantees ghost_locked_root@.finite(),
    /// which means AVLTreeSetMtPer::spec_avltreesetmtper_wf() always holds.
    /// This wraps the Chap38 helper that has visibility to the type_invariant.
    // veracity: no_requires
    pub fn assert_avltreesetmtper_always_wf<T: StTInMtT + Ord + TotalOrder + 'static>(s: &AVLTreeSetMtPer<T>)
        ensures s.spec_avltreesetmtper_wf()
    {
        crate::Chap38::BSTParaMtEph::BSTParaMtEph::assert_parambst_view_finite(&s.tree);
    }

    /// ParamBST size is stored as usize, so @.len() <= usize::MAX.
    /// Returns the size so callers can use it in capacity proofs.
    // veracity: no_requires
    pub fn assert_avltreesetmtper_bounded_size<T: StTInMtT + Ord + TotalOrder + 'static>(
        s: &AVLTreeSetMtPer<T>,
    ) -> (sz: usize)
        ensures
            s.spec_avltreesetmtper_wf(),
            sz as nat == s@.len(),
            s@.len() <= usize::MAX as nat,
    {
        crate::Chap38::BSTParaMtEph::BSTParaMtEph::assert_parambst_view_finite(&s.tree);
        s.size()
    }


    /// Parallel D&C set construction from Vec: split in half, recurse via join(), union.
    /// Work O(n lg n), Span O(lg^2 n) Ex 41.3 parallel fromSeq.
    fn from_vec_dc_per<T: StTInMtT + Ord + TotalOrder + 'static>(vals: Vec<T>) -> (tree: ParamBST<T>)
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
                from_vec_dc_per(left_vals)
            };
            let f2 = move || -> (t: ParamBST<T>)
                ensures t@.finite(), t@ =~= right_vals@.map_values(|t: T| t@).to_set()
            {
                from_vec_dc_per(right_vals)
            };
            let (left_tree, right_tree) = join(f1, f2);
            // Veracity: NEEDED proof block
            proof {
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

    impl<T: StTInMtT + Ord + TotalOrder + 'static> AVLTreeSetMtPerTrait<T> for AVLTreeSetMtPer<T> {
        open spec fn spec_avltreesetmtper_wf(&self) -> bool {
            self.tree@.finite()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            self.tree.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: APAS O(lg n) span requires tree-based sequence concat; AVLTreeSeqMtPerS lacks concat
        fn to_seq(&self) -> (seq: AVLTreeSeqMtPerS<T>)
        {
            let mut vals: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut vals);
            let ghost vals_seq = vals@;
            let seq = AVLTreeSeqMtPerS::from_vec(vals);
            // Veracity: NEEDED proof block
            proof {
                // from_vec: seq@ =~= vals_seq.map_values(|t: T| t@), so seq@[i] == vals_seq[i]@.
                // collect_in_order (empty start): vals_seq[i]@ in self.tree@ for all i,
                // and every v in self.tree@ has a witness j with vals_seq[j]@ == v.
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
                        let j = choose|j: int| 0 <= j < vals_seq.len() && #[trigger] vals_seq[j]@ == v;
                        // Veracity: NEEDED assert
                        assert(seq@[j] == vals_seq[j]@);
                    }
                };
            }
            seq
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
            AVLTreeSetMtPer { tree: ParamBST::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
        {
            AVLTreeSetMtPer { tree: ParamBST::singleton(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n) — parallel D&C via join() + union
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (constructed: Self)
        {
            let vals = seq.values_in_order();
            let n = vals.len();
            if n == 0 {
                return Self::empty();
            }
            if n > usize::MAX - 2 {
                return Self::empty();
            }
            let ghost vals_views = vals@.map_values(|t: T| t@);
            // Veracity: NEEDED proof block
            proof {
            }
            let tree = from_vec_dc_per(vals);
            // Veracity: NEEDED proof block
            proof {
            }
            AVLTreeSetMtPer { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let filtered_tree = self.tree.filter(f, Ghost(spec_pred));
            AVLTreeSetMtPer { tree: filtered_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let common_tree = self.tree.intersect(&other.tree);
            AVLTreeSetMtPer { tree: common_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let remaining_tree = self.tree.difference(&other.tree);
            AVLTreeSetMtPer { tree: remaining_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let combined_tree = self.tree.union(&other.tree);
            AVLTreeSetMtPer { tree: combined_tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find(&self, x: &T) -> (found: bool)
        {
            let result = self.tree.find(x);
            result.is_some()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete(&self, x: &T) -> (updated: Self)
        {
            let mut tree = self.tree.clone();
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert
            proof { assert(obeys_feq_full_trigger::<T>()); }
            tree.delete(x);
            AVLTreeSetMtPer { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&self, x: T) -> (updated: Self)
        {
            let mut tree = self.tree.clone();
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert
            proof { assert(obeys_feq_full_trigger::<T>()); }
            tree.insert(x);
            AVLTreeSetMtPer { tree }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> PartialOrd for AVLTreeSetMtPer<T> {
        #[verifier::external_body]
        fn partial_cmp(&self, other: &Self) -> (ord: Option<Ordering>) {
            Some(self.cmp(other))
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> Ord for AVLTreeSetMtPer<T> {
        #[verifier::external_body]
        fn cmp(&self, other: &Self) -> (ord: Ordering)
        {
            let mut self_seq: Vec<T> = Vec::new();
            let mut other_seq: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut self_seq);
            other.tree.collect_in_order(&mut other_seq);
            let n_self = self_seq.len();
            let n_other = other_seq.len();
            let min_n = if n_self < n_other { n_self } else { n_other };
            let mut i: usize = 0;
            while i < min_n {
                let c = <T as std::cmp::Ord>::cmp(&self_seq[i], &other_seq[i]);
                if c != Equal {
                    return c;
                }
                i += 1;
            }
            <usize as std::cmp::Ord>::cmp(&n_self, &n_other)
        }
    }

    //		Section 10. iterators — AVLTreeSetMtPer

    /// Snapshot iterator over AVLTreeSetMtPer — collects elements via in_order traversal,
    /// then yields owned T values from the captured Vec.
    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtPerIter<T: StTInMtT + Ord + TotalOrder + 'static> {
        pub inner: IntoIter<T>,
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> View for AVLTreeSetMtPerIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant_avltreesetmtper<T: StTInMtT + Ord + TotalOrder + 'static>(it: &AVLTreeSetMtPerIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> std::iter::Iterator for AVLTreeSetMtPerIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
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
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for for-loop support over AVLTreeSetMtPerIter.
    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtPerGhostIterator<T: StTInMtT + Ord + TotalOrder + 'static> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> View for AVLTreeSetMtPerGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> vstd::pervasive::ForLoopGhostIteratorNew for AVLTreeSetMtPerIter<T> {
        type GhostIter = AVLTreeSetMtPerGhostIterator<T>;
        open spec fn ghost_iter(&self) -> AVLTreeSetMtPerGhostIterator<T> {
            AVLTreeSetMtPerGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> vstd::pervasive::ForLoopGhostIterator for AVLTreeSetMtPerGhostIterator<T> {
        type ExecIter = AVLTreeSetMtPerIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &AVLTreeSetMtPerIter<T>) -> bool {
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

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &AVLTreeSetMtPerIter<T>) -> AVLTreeSetMtPerGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + TotalOrder + 'static> std::iter::IntoIterator for &'a AVLTreeSetMtPer<T> {
        type Item = T;
        type IntoIter = AVLTreeSetMtPerIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1.len() == self@.len(),
                iter_invariant_avltreesetmtper(&it),
        {
            let in_ord = self.tree.in_order();
            AVLTreeSetMtPerIter { inner: in_ord.seq.into_iter() }
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: StTInMtT + Ord + TotalOrder + 'static> Default for AVLTreeSetMtPer<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StTInMtT + Ord + TotalOrder + 'static> PartialEqSpecImpl for AVLTreeSetMtPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> Eq for AVLTreeSetMtPer<T> {}

    impl<T: StTInMtT + Ord + TotalOrder + 'static> PartialEq for AVLTreeSetMtPer<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            // size() ensures finiteness — satisfies collect_in_order's requires.
            let _sz_self = self.tree.size();
            let _sz_other = other.tree.size();
            let mut self_vals: Vec<T> = Vec::new();
            let mut other_vals: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut self_vals);
            other.tree.collect_in_order(&mut other_vals);
            let n = self_vals.len();
            if n != other_vals.len() {
                // Veracity: NEEDED proof block
                proof {
                    // self_vals.len() == self.tree@.len(), other_vals.len() == other.tree@.len().
                    // If self@ == other@, then self@.len() == other@.len(), contradiction.
                    if self@ == other@ {
                    }
                }
                return false;
            }
            let mut i: usize = 0;
            let mut all_eq = true;
            while i < n
                invariant
                    i <= n,
                    n == self_vals@.len(),
                    n == other_vals@.len(),
                decreases n - i,
            {
                if self_vals[i] != other_vals[i] {
                    all_eq = false;
                }
                i = i + 1;
            }
            // Veracity: NEEDED proof block
            proof { accept(all_eq == (self@ == other@)); }
            all_eq
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> Clone for AVLTreeSetMtPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@, cloned.spec_avltreesetmtper_wf() == self.spec_avltreesetmtper_wf(),
        {
            AVLTreeSetMtPer { tree: self.tree.clone() }
        }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! AVLTreeSetMtPerLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty();
            $( __set = __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    // Ghost fields are zero-sized; ParamBST is Send/Sync via BSTParaMtEph.
    unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Send for AVLTreeSetMtPer<T> {}
    unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Sync for AVLTreeSetMtPer<T> {}

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Debug for AVLTreeSetMtPerIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtPerIter")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for AVLTreeSetMtPerIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtPerIter")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Debug for AVLTreeSetMtPerGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtPerGhostIterator")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for AVLTreeSetMtPerGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtPerGhostIterator")
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Debug for AVLTreeSetMtPer<T> {
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

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for AVLTreeSetMtPer<T> {
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
