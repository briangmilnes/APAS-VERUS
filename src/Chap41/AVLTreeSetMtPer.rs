//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Multi-threaded persistent set backed by BSTParaMtEph for parallel set operations.
//!
//! Work/Span Analysis (via delegation to BSTParaMtEph parallel D&C):
//! - union: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - intersection: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - difference: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - filter: Work O(Σ W(f(x))), Span O(n + max S(f(x))) — sequential (spec_fn not Send)

pub mod AVLTreeSetMtPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 12. derive impls in verus!
    // 13. macros
    // 14. derive impls outside verus!

    use std::cmp::Ordering::{self, Equal, Greater, Less};
    use std::fmt;

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
    use crate::Types::Types::*;

    verus! {

// 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
        pub tree: ParamBST<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
    }

    // 6. spec fns

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPer<T> {
        pub open spec fn spec_avltreesetmtper_wf(&self) -> bool {
            self.tree@.finite()
        }
    }

    // 7. proof fns

    /// ParamBST type_invariant guarantees ghost_locked_root@.finite(),
    /// which means AVLTreeSetMtPer::spec_avltreesetmtper_wf() always holds.
    /// This wraps the Chap38 helper that has visibility to the type_invariant.
    pub fn assert_avltreesetmtper_always_wf<T: StTInMtT + Ord + 'static>(s: &AVLTreeSetMtPer<T>)
        ensures s.spec_avltreesetmtper_wf()
    {
        crate::Chap38::BSTParaMtEph::BSTParaMtEph::assert_parambst_view_finite(&s.tree);
    }

    /// ParamBST size is stored as usize, so @.len() <= usize::MAX.
    /// Returns the size so callers can use it in capacity proofs.
    pub fn assert_avltreesetmtper_bounded_size<T: StTInMtT + Ord + 'static>(
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

    // 8. traits

    pub trait AVLTreeSetMtPerTrait<T: StTInMtT + Ord + 'static>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetmtper_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetmtper_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential in-order traversal
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree.spec_avltreesetmtper_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — DIFFERS: sequential loop of inserts
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ W(f(x))), Span O(n + max S(f(x))) — DIFFERS: sequential filter (spec_fn not Send)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (updated: Self)
            requires
                self.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures updated@ == self@.remove(x@), updated.spec_avltreesetmtper_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (updated: Self)
            requires
                self.spec_avltreesetmtper_wf(),
                self@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures updated@ == self@.insert(x@), updated.spec_avltreesetmtper_wf();
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPerTrait<T> for AVLTreeSetMtPer<T> {
        open spec fn spec_avltreesetmtper_wf(&self) -> bool {
            self.tree@.finite()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            self.tree.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqMtPerS<T>)
        {
            let mut vals: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut vals);
            let ghost vals_seq = vals@;
            let seq = AVLTreeSeqMtPerS::from_vec(vals);
            proof {
                // from_vec: seq@ =~= vals_seq.map_values(|t: T| t@), so seq@[i] == vals_seq[i]@.
                // collect_in_order (empty start): vals_seq[i]@ in self.tree@ for all i,
                // and every v in self.tree@ has a witness j with vals_seq[j]@ == v.
                assert forall|i: int| 0 <= i < seq@.len() implies
                    #[trigger] self@.contains(seq@[i])
                by {
                    assert(seq@[i] == vals_seq[i]@);
                };
                assert forall|v: T::V|
                    #[trigger] seq@.to_set().contains(v) <==> self@.contains(v)
                by {
                    if seq@.to_set().contains(v) {
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == v;
                        assert(seq@[j] == vals_seq[j]@);
                    }
                    if self@.contains(v) {
                        let j = choose|j: int| 0 <= j < vals_seq.len() && #[trigger] vals_seq[j]@ == v;
                        assert(seq@[j] == vals_seq[j]@);
                    }
                };
                assert(seq@.to_set() =~= self@);
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (constructed: Self)
        {
            proof { assert(obeys_feq_full_trigger::<T>()); }
            let vals = seq.values_in_order();
            let n = vals.len();
            let mut tree = ParamBST::new();
            proof {
                assert(vals@.len() == seq@.len());
            }
            if n > usize::MAX - 2 {
                proof { assert(false); }
                return AVLTreeSetMtPer { tree };
            }
            let ghost seq_view = seq@;
            let mut i: usize = 0;
            while i < n
                invariant
                    tree@.finite(),
                    i <= n,
                    n == vals@.len(),
                    n <= usize::MAX - 2,
                    tree@.len() <= i as nat,
                    tree@.len() < usize::MAX as nat,
                    vstd::laws_cmp::obeys_cmp_spec::<T>(),
                    view_ord_consistent::<T>(),
                    vals@.map_values(|t: T| t@) =~= seq_view,
                    forall|j: int| 0 <= j < i as int ==> #[trigger] tree@.contains(vals@[j]@),
                    forall|v: T::V| tree@.contains(v) ==>
                        exists|j: int| 0 <= j < i as int && #[trigger] vals@[j]@ == v,
                decreases n - i,
            {
                let ghost old_tree = tree@;
                let elem = &vals[i];
                let cloned = elem.clone_plus();
                proof {
                    assert(obeys_feq_full_trigger::<T>());
                    lemma_cloned_view_eq::<T>(*elem, cloned);
                    assert(cloned@ == vals@[i as int]@);
                }
                tree.insert(cloned);
                proof {
                    assert forall|j: int| 0 <= j < i as int implies
                        #[trigger] tree@.contains(vals@[j]@)
                    by {
                        assert(old_tree.contains(vals@[j]@));
                    };
                    assert(tree@.contains(vals@[i as int]@));
                    assert forall|v: T::V| tree@.contains(v) implies
                        exists|j: int| 0 <= j < i + 1 && #[trigger] vals@[j]@ == v
                    by {
                        if v == cloned@ {
                            assert(vals@[i as int]@ == v);
                        } else {
                            assert(old_tree.contains(v));
                        }
                    };
                }
                i += 1;
            }
            proof {
                assert forall|v: T::V| tree@.contains(v) <==> seq_view.to_set().contains(v)
                by {
                    if tree@.contains(v) {
                        let j = choose|j: int| 0 <= j < n as int && #[trigger] vals@[j]@ == v;
                        assert(seq_view[j] == vals@[j]@);
                        assert(seq_view.to_set().contains(v));
                    }
                    if seq_view.to_set().contains(v) {
                        let j = choose|j: int| 0 <= j < seq_view.len() && seq_view[j] == v;
                        assert(vals@[j]@ == seq_view[j]);
                        assert(tree@.contains(vals@[j]@));
                    }
                };
                assert(tree@ =~= seq_view.to_set());
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
            proof { assert(obeys_feq_full_trigger::<T>()); }
            tree.delete(x);
            AVLTreeSetMtPer { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&self, x: T) -> (updated: Self)
        {
            let mut tree = self.tree.clone();
            proof { assert(obeys_feq_full_trigger::<T>()); }
            tree.insert(x);
            AVLTreeSetMtPer { tree }
        }
    }

    // 12. derive impls in verus!

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtPer<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StTInMtT + Ord + 'static> PartialEqSpecImpl for AVLTreeSetMtPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StTInMtT + Ord + 'static> Eq for AVLTreeSetMtPer<T> {}

    impl<T: StTInMtT + Ord + 'static> PartialEq for AVLTreeSetMtPer<T> {
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
                proof {
                    // self_vals.len() == self.tree@.len(), other_vals.len() == other.tree@.len().
                    // If self@ == other@, then self@.len() == other@.len(), contradiction.
                    if self@ == other@ {
                        assert(self@.len() == other@.len());
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
            proof { assume(all_eq == (self@ == other@)); }
            all_eq
        }
    }

    impl<T: StTInMtT + Ord + 'static> PartialOrd for AVLTreeSetMtPer<T> {
        #[verifier::external_body]
        fn partial_cmp(&self, other: &Self) -> (ord: Option<Ordering>) {
            Some(self.cmp(other))
        }
    }

    impl<T: StTInMtT + Ord + 'static> Ord for AVLTreeSetMtPer<T> {
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
                let c = self_seq[i].cmp(&other_seq[i]);
                if c != Equal {
                    return c;
                }
                i += 1;
            }
            n_self.cmp(&n_other)
        }
    }

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@, cloned.spec_avltreesetmtper_wf() == self.spec_avltreesetmtper_wf(),
        {
            AVLTreeSetMtPer { tree: self.tree.clone() }
        }
    }

    } // verus!

    // Ghost fields are zero-sized; ParamBST is Send/Sync via BSTParaMtEph.
    unsafe impl<T: StTInMtT + Ord + 'static> Send for AVLTreeSetMtPer<T> {}
    unsafe impl<T: StTInMtT + Ord + 'static> Sync for AVLTreeSetMtPer<T> {}

    // 13. macros

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

    // 14. derive impls outside verus!

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtPer<T> {
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

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtPer<T> {
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
