//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Single-threaded ephemeral set implementation using ArraySeqStEph as backing store.
//!
//! View: elements@.to_set()
//! Invariant: elements@.no_duplicates()
//!
//! All membership tests use linear scan. This keeps proofs clean: to_set()
//! correctness follows directly from vstd seq/set lemmas without needing
//! spec-level ordering (TotalOrder). The backing ArraySeq is unordered.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod ArraySetStEph {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::seq_set::{lemma_push_not_contains_to_set, lemma_seq_index_in_map_to_set};
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, obeys_feq_clone, lemma_cloned_view_eq, lemma_seq_map_cloned_view_eq};

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::set::group_set_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_lib_default,
        vstd::seq_lib::group_seq_properties,
        vstd::prelude::Seq::group_seq_extra,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::set_lib::group_set_lib_default,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct ArraySetStEph<T: StT + Ord> {
        pub elements: ArraySeqStEphS<T>,
    }

    pub type ArraySetS<T> = ArraySetStEph<T>;

    //		Section 5. view impls


    impl<T: StT + Ord> View for ArraySetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> {
            self.elements@.to_set()
        }
    }

    //		Section 7. proof fns/broadcast groups


    /// Filtering out a value from a no-dup seq produces to_set().remove(v).
    proof fn lemma_filter_remove<V>(s: Seq<V>, v: V)
        requires s.no_duplicates()
        ensures s.filter(|e: V| e != v).to_set() =~= s.to_set().remove(v)
        decreases s.len()
    {
        if s.len() == 0 {
        } else {
            let head = s[0];
            let tail = s.subrange(1, s.len() as int);
            lemma_filter_remove::<V>(tail, v);
            let pred = |e: V| e != v;
            // Veracity: NEEDED assert
            assert(s =~= seq![head] + tail);
            Seq::filter_distributes_over_add(seq![head], tail, pred);
            if head == v {
            } else {
                vstd::seq_lib::seq_to_set_distributes_over_add(seq![head], tail.filter(pred));
                lemma_push_not_contains_to_set(tail.filter(pred), head);
            }
        }
    }

    /// Appending a fresh element to a no-dup seq preserves no_duplicates.
    proof fn lemma_push_preserves_no_dups<V>(s: Seq<V>, x: V)
        requires
            s.no_duplicates(),
            !s.contains(x),
        ensures
            s.push(x).no_duplicates()
    {
        let s2 = s.push(x);
    }

    /// Elements of a filtered seq are elements of the original.
    proof fn lemma_filter_in_original<V>(s: Seq<V>, pred: spec_fn(V) -> bool)
        ensures forall|v: V| s.filter(pred).contains(v) ==> #[trigger] s.contains(v)
        decreases s.len()
    {
        reveal(Seq::filter);
        if s.len() > 0 {
            lemma_filter_in_original(s.drop_last(), pred);
            let sub = s.drop_last().filter(pred);
            if pred(s.last()) {
            } else {
            }
        }
    }

    /// Filtering preserves no_duplicates.
    proof fn lemma_filter_preserves_no_dups<V>(s: Seq<V>, pred: spec_fn(V) -> bool)
        requires s.no_duplicates()
        ensures s.filter(pred).no_duplicates()
        decreases s.len()
    {
        reveal(Seq::filter);
        if s.len() > 0 {
            let sdl = s.drop_last();
            lemma_filter_preserves_no_dups(sdl, pred);
            let sub = sdl.filter(pred);
            if pred(s.last()) {
                // s.filter(pred) == sub.push(s.last())
                lemma_push_preserves_no_dups(sub, s.last());
            }
        }
    }

    /// Filtering a no-dup seq by set membership gives the intersection.
    proof fn lemma_filter_to_set_intersect<V>(s: Seq<V>, set: Set<V>)
        requires s.no_duplicates()
        ensures s.filter(|e: V| set.contains(e)).to_set() =~= s.to_set().intersect(set)
        decreases s.len()
    {
        let pred = |e: V| set.contains(e);
        if s.len() == 0 {
        } else {
            let head = s[0];
            let tail = s.subrange(1, s.len() as int);
            lemma_filter_to_set_intersect(tail, set);
            // Veracity: NEEDED assert
            assert(s =~= seq![head] + tail);
            Seq::filter_distributes_over_add(seq![head], tail, pred);
            reveal(Seq::filter);
            if set.contains(head) {
                vstd::seq_lib::seq_to_set_distributes_over_add(seq![head], tail.filter(pred));
                lemma_push_not_contains_to_set(tail.filter(pred), head);
            } else {
            }
        }
    }

    /// Filtering a no-dup seq by set non-membership gives the difference.
    proof fn lemma_filter_to_set_difference<V>(s: Seq<V>, set: Set<V>)
        requires s.no_duplicates()
        ensures s.filter(|e: V| !set.contains(e)).to_set() =~= s.to_set().difference(set)
        decreases s.len()
    {
        let pred = |e: V| !set.contains(e);
        if s.len() == 0 {
        } else {
            let head = s[0];
            let tail = s.subrange(1, s.len() as int);
            lemma_filter_to_set_difference(tail, set);
            // Veracity: NEEDED assert
            assert(s =~= seq![head] + tail);
            Seq::filter_distributes_over_add(seq![head], tail, pred);
            reveal(Seq::filter);
            if !set.contains(head) {
                vstd::seq_lib::seq_to_set_distributes_over_add(seq![head], tail.filter(pred));
                lemma_push_not_contains_to_set(tail.filter(pred), head);
            } else {
            }
        }
    }

    /// A subsequence of a no_duplicates seq also has no duplicates, and its
    /// to_set() is a subset.
    proof fn lemma_subseq_no_dups_subset<V>(orig: Seq<V>, sub: Seq<V>)
        requires
            orig.no_duplicates(),
            forall|i: int| #![trigger sub[i]] 0 <= i < sub.len() ==> orig.contains(sub[i]),
            sub.no_duplicates(),
        ensures
            sub.to_set().subset_of(orig.to_set())
    {
    }

    //		Section 8. traits


    pub trait ArraySetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_arraysetsteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — DIFFERS: stored count, not array scan
        fn size(&self) -> (count: usize)
            requires self.spec_arraysetsteph_wf()
            ensures count == self@.len(), self@.finite();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential clone of element array
        fn to_seq(&self) -> (seq: ArraySeqStEphS<T>)
            requires self.spec_arraysetsteph_wf(),
            ensures
                self@.finite(),
                seq@.no_duplicates(),
                seq@.to_set() =~= self@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — DIFFERS: single-element array, not boolean array
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n²), Span O(n²) — DIFFERS: sequential insert loop, each O(n)
        fn from_seq(seq: ArraySeqStEphS<T>) -> (constructed: Self)
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u + Σ W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(Σ W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + Σ W(f(x))), Span O(n + Σ W(f(x))) — DIFFERS: sequential filter
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_arraysetsteph_wf(),
                self@.finite(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_arraysetsteph_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_arraysetsteph_wf(),
                other.spec_arraysetsteph_wf(),
                self@.finite(),
                other@.finite(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_arraysetsteph_wf(),
                other.spec_arraysetsteph_wf(),
                self@.finite(),
                other@.finite(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_arraysetsteph_wf(),
                other.spec_arraysetsteph_wf(),
                self@.finite(),
                other@.finite(),
            ensures
                combined@ == self@.union(other@),
                combined.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: linear scan, not indexed
        fn find(&self, x: &T) -> (found: bool)
            requires self@.finite(),
            ensures found == self@.contains(x@);

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: copy with filter
        fn delete(&mut self, x: &T)
            requires
                old(self).spec_arraysetsteph_wf(),
                old(self)@.finite(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: find + copy
        fn insert(&mut self, x: T)
            requires
                old(self).spec_arraysetsteph_wf(),
                old(self)@.finite(),
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_arraysetsteph_wf();
    }

    //		Section 9. impls


    impl<T: StT + Ord> ArraySetStEph<T> {
        pub open spec fn spec_arraysetsteph_wf(&self) -> bool {
            self.elements@.no_duplicates()
            && self@.finite()
            && obeys_feq_full::<T>()
        }
    }


    impl<T: StT + Ord> ArraySetStEphTrait<T> for ArraySetStEph<T> {
        open spec fn spec_arraysetsteph_wf(&self) -> bool {
            self.elements@.no_duplicates()
            && self@.finite()
            && obeys_feq_full::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            // Veracity: NEEDED proof block
            proof {
                self.elements@.unique_seq_to_set();
            }
            self.elements.length()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_seq(&self) -> (seq: ArraySeqStEphS<T>)
        {
            let seq = self.elements.clone();
            // Veracity: NEEDED proof block
            proof {
                // obeys_feq_clone follows from obeys_feq_full in wf.
                lemma_seq_map_cloned_view_eq(
                    self.elements.seq@,
                    seq.seq@,
                );
            }
            seq
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
            let empty = ArraySetStEph {
                elements: ArraySeqStEphS::empty(),
            };
            // Veracity: NEEDED assert
            assert(obeys_feq_full_trigger::<T>());
            empty
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
        {
            let ghost x_view = x@;
            let mut v: Vec<T> = Vec::new();
            v.push(x);
            let ghost v_snapshot = v@;
            let elements = ArraySeqStEphS::from_vec(v);
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(elements@[0] == elements.spec_index(0)@);
                Seq::<<T as View>::V>::empty().lemma_push_to_set_commute(x_view);
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<T>());
            }
            ArraySetStEph { elements }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_seq(seq: ArraySeqStEphS<T>) -> (constructed: Self)
        {
            if seq.length() == 0 {
                return Self::empty();
            }
            let mut constructed = Self::empty();
            let mut i: usize = 0;
            while i < seq.length()
                invariant
                    constructed@.finite(),
                    constructed.spec_arraysetsteph_wf(),
                    i <= seq.spec_len(),
                    forall|j: int| 0 <= j < i ==> #[trigger] constructed@.contains(seq@[j]),
                    forall|v: <T as View>::V| #[trigger] constructed@.contains(v) ==>
                        (exists|j: int| 0 <= j < i && seq@[j] == v),
                decreases seq.spec_len() - i,
            {
                let r = seq.nth(i);
                let elem = r.clone();
                // Veracity: NEEDED proof block
                proof {
                    lemma_cloned_view_eq(*r, elem);
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

        #[verifier::loop_isolation(false)]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn find(&self, x: &T) -> (found: bool)
        {
            // Veracity: NEEDED assert
            assert(obeys_feq_full_trigger::<T>());
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                    forall|j: int| 0 <= j < i ==> self.elements@[j] != x@,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if feq(elem, x) {
                    // Veracity: NEEDED proof block
                    proof {
                        let ii = i as int;
                        lemma_seq_index_in_map_to_set(self.elements.seq@, ii);
                    }
                    return true;
                }
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            false
        }

        #[verifier::loop_isolation(false)]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let ghost old_view = self.elements@;
            let mut result_vec: Vec<T> = Vec::new();
            let ghost mut rv_views: Seq<<T as View>::V> = Seq::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    n == self.elements.spec_len(),
                    i <= n,
                    self.elements@ == old_view,
                    old_view.no_duplicates(),
                    forall|t: &T| #[trigger] f.requires((t,)),
                    forall|x: T, keep: bool|
                        f.ensures((&x,), keep) ==> keep == spec_pred(x@),
                    rv_views.len() == result_vec@.len(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> rv_views[j] == result_vec@[j]@,
                    rv_views.no_duplicates(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        ==> old_view.subrange(0, i as int).to_set().contains(rv_views[j]),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> spec_pred(rv_views[j]),
                    forall|j: int| #![trigger old_view[j]]
                        0 <= j < i && spec_pred(old_view[j])
                        ==> rv_views.to_set().contains(old_view[j]),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if f(elem) {
                    let cloned_elem = elem.clone();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(!rv_views.contains(cv)) by {
                            if rv_views.contains(cv) {
                                let k = choose|k: int| 0 <= k < rv_views.len() && rv_views[k] == cv;
                                let m = choose|m: int| 0 <= m < i && old_view.subrange(0, i as int)[m] == cv;
                                // Veracity: NEEDED assert
                                assert(old_view[m] == old_view[i as int]);
                            }
                        };
                        let ghost old_rv = rv_views;
                        rv_views = rv_views.push(cv);
                        lemma_push_preserves_no_dups(rv_views.drop_last(), cv);
                        let ghost next_sub = old_view.subrange(0, (i + 1) as int);
                        // Veracity: NEEDED assert
                        assert forall|j: int| #![trigger rv_views[j]]
                            0 <= j < rv_views.len()
                            implies next_sub.to_set().contains(rv_views[j]) by {
                            if j < rv_views.len() - 1 {
                                let m = choose|m: int| 0 <= m < i && old_view.subrange(0, i as int)[m] == rv_views[j];
                            } else {
                                // Veracity: NEEDED assert
                                assert(next_sub[i as int] == cv);
                            }
                        };
                        // spec_pred for new rv_views
                        // completeness: elements at indices < i+1 satisfying spec_pred are in rv_views
                    }
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        let ghost next_sub = old_view.subrange(0, (i + 1) as int);
                        // completeness: spec_pred(old_view[i]) is false, so invariant extends
                    }
                }
                i += 1;
            }
            let filtered = ArraySetStEph { elements: ArraySeqStEphS::from_vec(result_vec) };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(filtered.elements@ =~= rv_views) by {
                    // Veracity: NEEDED assert
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies filtered.elements@[j] == rv_views[j] by {
                        // Veracity: NEEDED assert
                        assert(filtered.elements.spec_index(j) == result_vec@[j]);
                    };
                };
                // subset_of: rv_views.to_set() ⊆ old_view.to_set() = self@
                vstd::seq_lib::seq_to_set_is_finite(filtered.elements@);
                // spec_arraysetsteph_wf
                // spec_pred direction
                // completeness direction
            }
            filtered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let ghost old_view = self.elements@;
            let ghost other_set = other@;
            let mut result_vec: Vec<T> = Vec::new();
            let ghost mut rv_views: Seq<<T as View>::V> = Seq::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                    self.elements@ == old_view,
                    old_view.no_duplicates(),
                    other.spec_arraysetsteph_wf(),
                    other@.finite(),
                    other@ == other_set,
                    obeys_feq_full::<T>(),
                    rv_views =~=
                        old_view.subrange(0, i as int).filter(
                            |e: <T as View>::V| other_set.contains(e)),
                    rv_views.len() == result_vec@.len(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> rv_views[j] == result_vec@[j]@,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                let ghost prefix = old_view.subrange(0, i as int);
                let ghost next_prefix = old_view.subrange(0, (i + 1) as int);
                let ghost filt = |e: <T as View>::V| other_set.contains(e);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(next_prefix =~= prefix.push(old_view[i as int]));
                    Seq::filter_distributes_over_add(prefix, seq![old_view[i as int]], filt);
                }
                if other.find(elem) {
                    let cloned_elem = elem.clone();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    // Veracity: NEEDED proof block
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                    }
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        reveal(Seq::filter);
                    }
                }
                i += 1;
            }
            let common = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(old_view.subrange(0, n as int) =~= old_view);
                // Veracity: NEEDED assert
                assert(common.elements@ =~= rv_views) by {
                    // Veracity: NEEDED assert
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies common.elements@[j] == rv_views[j] by {
                        // Veracity: NEEDED assert
                        assert(common.elements.spec_index(j) == result_vec@[j]);
                    };
                };
                lemma_filter_to_set_intersect(self.elements@, other@);
                lemma_filter_preserves_no_dups(self.elements@,
                    |e: <T as View>::V| other_set.contains(e));
                vstd::seq_lib::seq_to_set_is_finite(common.elements@);
            }
            common
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let ghost old_view = self.elements@;
            let ghost other_set = other@;
            let mut result_vec: Vec<T> = Vec::new();
            let ghost mut rv_views: Seq<<T as View>::V> = Seq::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                    self.elements@ == old_view,
                    old_view.no_duplicates(),
                    other.spec_arraysetsteph_wf(),
                    other@.finite(),
                    other@ == other_set,
                    obeys_feq_full::<T>(),
                    rv_views =~=
                        old_view.subrange(0, i as int).filter(
                            |e: <T as View>::V| !other_set.contains(e)),
                    rv_views.len() == result_vec@.len(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> rv_views[j] == result_vec@[j]@,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                let ghost prefix = old_view.subrange(0, i as int);
                let ghost next_prefix = old_view.subrange(0, (i + 1) as int);
                let ghost filt = |e: <T as View>::V| !other_set.contains(e);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(next_prefix =~= prefix.push(old_view[i as int]));
                    Seq::filter_distributes_over_add(prefix, seq![old_view[i as int]], filt);
                }
                if !other.find(elem) {
                    let cloned_elem = elem.clone();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    // Veracity: NEEDED proof block
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                    }
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        reveal(Seq::filter);
                    }
                }
                i += 1;
            }
            let remaining = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(old_view.subrange(0, n as int) =~= old_view);
                // Veracity: NEEDED assert
                assert(remaining.elements@ =~= rv_views) by {
                    // Veracity: NEEDED assert
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies remaining.elements@[j] == rv_views[j] by {
                        // Veracity: NEEDED assert
                        assert(remaining.elements.spec_index(j) == result_vec@[j]);
                    };
                };
                lemma_filter_to_set_difference(self.elements@, other@);
                lemma_filter_preserves_no_dups(self.elements@,
                    |e: <T as View>::V| !other_set.contains(e));
                vstd::seq_lib::seq_to_set_is_finite(remaining.elements@);
            }
            remaining
        }

        #[verifier::loop_isolation(false)]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let ghost self_view = self.elements@;
            let ghost other_view = other.elements@;
            let ghost self_set = self@;
            let self_len = self.elements.length();
            let other_len = other.elements.length();
            let mut result_vec: Vec<T> = Vec::new();
            let ghost mut rv_views: Seq<<T as View>::V> = Seq::empty();

            let mut i: usize = 0;
            while i < self_len
                invariant
                    i <= self_len,
                    self_len as int == self.elements.spec_len(),
                    self.elements@ == self_view,
                    rv_views =~= self_view.subrange(0, i as int),
                    rv_views.len() == result_vec@.len(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> rv_views[j] == result_vec@[j]@,
                decreases self_len - i,
            {
                let elem = self.elements.nth(i);
                let cloned_elem = elem.clone();
                // Veracity: NEEDED proof block
                proof {
                    lemma_cloned_view_eq(*elem, cloned_elem);
                }
                let ghost cv = cloned_elem@;
                result_vec.push(cloned_elem);
                // Veracity: NEEDED proof block
                proof {
                    rv_views = rv_views.push(cv);
                }
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }

            let mut j: usize = 0;
            while j < other_len
                invariant
                    j <= other_len,
                    other_len as int == other.elements.spec_len(),
                    other.elements@ == other_view,
                    self.spec_arraysetsteph_wf(),
                    self@.finite(),
                    self@ == self_set,
                    obeys_feq_full::<T>(),
                    rv_views =~= self_view +
                        other_view.subrange(0, j as int).filter(
                            |e: <T as View>::V| !self_set.contains(e)),
                    rv_views.len() == result_vec@.len(),
                    forall|k: int| #![trigger rv_views[k]]
                        0 <= k < rv_views.len() ==> rv_views[k] == result_vec@[k]@,
                decreases other_len - j,
            {
                let elem = other.elements.nth(j);
                let ghost prefix = other_view.subrange(0, j as int);
                let ghost next_prefix = other_view.subrange(0, (j + 1) as int);
                let ghost filt = |e: <T as View>::V| !self_set.contains(e);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(next_prefix =~= prefix.push(other_view[j as int]));
                    Seq::filter_distributes_over_add(prefix, seq![other_view[j as int]], filt);
                }
                if !self.find(elem) {
                    let cloned_elem = elem.clone();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    // Veracity: NEEDED proof block
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                    }
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        reveal(Seq::filter);
                    }
                }
                j += 1;
            }
            let combined = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(other_view.subrange(0, other_len as int) =~= other_view);
                // Veracity: NEEDED assert
                assert(combined.elements@ =~= rv_views) by {
                    // Veracity: NEEDED assert
                    assert forall|k: int| #![trigger rv_views[k]]
                        0 <= k < rv_views.len()
                        implies combined.elements@[k] == rv_views[k] by {
                        // Veracity: NEEDED assert
                        assert(combined.elements.spec_index(k) == result_vec@[k]);
                    };
                };
                // rv_views =~= self_view + other_view.filter(|e| !self_set.contains(e))
                let ghost filt = |e: <T as View>::V| !self_set.contains(e);
                lemma_filter_to_set_difference(other_view, self_set);
                // other_view.filter(filt).to_set() =~= other@ \ self@
                vstd::seq_lib::seq_to_set_distributes_over_add(self_view, other_view.filter(filt));
                // rv_views.to_set() = self_view.to_set() ∪ other_view.filter(filt).to_set()
                //   = self@ ∪ (other@ \ self@)
                //   = self@ ∪ other@

                // no_duplicates: self_view has no dups, filtered other has no dups,
                // and they're disjoint (filtered other excludes self@).
                lemma_filter_preserves_no_dups(other_view, filt);
                vstd::seq_lib::seq_to_set_is_finite(combined.elements@);
                // Prove no_duplicates for the concatenation.
            }
            combined
        }

        #[verifier::loop_isolation(false)]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn delete(&mut self, x: &T)
        {
            let ghost old_view = self.elements@;
            let ghost x_view = x@;
            let mut result_vec: Vec<T> = Vec::new();
            let ghost mut rv_views: Seq<<T as View>::V> = Seq::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                    self.elements@ == old_view,
                    old_view.no_duplicates(),
                    x@ == x_view,
                    rv_views =~=
                        old_view.subrange(0, i as int).filter(|e: <T as View>::V| e != x_view),
                    rv_views.len() == result_vec@.len(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> rv_views[j] == result_vec@[j]@,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                let ghost prefix = old_view.subrange(0, i as int);
                let ghost next_prefix = old_view.subrange(0, (i + 1) as int);
                let ghost filt = |e: <T as View>::V| e != x_view;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(next_prefix =~= prefix.push(old_view[i as int]));
                    Seq::filter_distributes_over_add(prefix, seq![old_view[i as int]], filt);
                }
                if !feq(elem, x) {
                    let cloned_elem = elem.clone();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    // Veracity: NEEDED proof block
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                    }
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        reveal(Seq::filter);
                    }
                }
                i += 1;
            }
            self.elements = ArraySeqStEphS::from_vec(result_vec);
            // Veracity: NEEDED proof block
            proof {
                let ghost filt = |e: <T as View>::V| e != x_view;
                // Veracity: NEEDED assert
                assert(old_view.subrange(0, n as int) =~= old_view);
                // Veracity: NEEDED assert
                assert(self.elements@ =~= rv_views) by {
                    // Veracity: NEEDED assert
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies self.elements@[j] == rv_views[j] by {
                        // Veracity: NEEDED assert
                        assert(self.elements.spec_index(j) == result_vec@[j]);
                    };
                };
                lemma_filter_remove(old(self).elements@, x@);
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                lemma_filter_preserves_no_dups(old(self).elements@, filt);
            }
        }

        #[verifier::loop_isolation(false)]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, x: T)
        {
            if !self.find(&x) {
                let ghost old_view = self.elements@;
                let ghost x_view = x@;
                let n = self.elements.length();
                let mut new_vec: Vec<T> = Vec::new();
                let ghost mut rv_views: Seq<<T as View>::V> = Seq::empty();
                let mut i: usize = 0;
                while i < n
                    invariant
                        i <= n,
                        n as int == self.elements.spec_len(),
                        self.elements@ == old_view,
                        rv_views =~= old_view.subrange(0, i as int),
                        rv_views.len() == new_vec@.len(),
                        forall|j: int| #![trigger rv_views[j]]
                            0 <= j < rv_views.len() ==> rv_views[j] == new_vec@[j]@,
                    decreases n - i,
                {
                    let elem = self.elements.nth(i);
                    let cloned_elem = elem.clone();
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                    }
                    let ghost cv = cloned_elem@;
                    new_vec.push(cloned_elem);
                    // Veracity: NEEDED proof block
                    proof {
                        rv_views = rv_views.push(cv);
                    }
                    i += 1;
                }
                new_vec.push(x);
                // Veracity: NEEDED proof block
                proof {
                    rv_views = rv_views.push(x_view);
                }
                self.elements = ArraySeqStEphS::from_vec(new_vec);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(self.elements@ =~= rv_views) by {
                        // Veracity: NEEDED assert
                        assert forall|j: int| #![trigger rv_views[j]]
                            0 <= j < rv_views.len()
                            implies self.elements@[j] == rv_views[j] by {
                            // Veracity: NEEDED assert
                            assert(self.elements.spec_index(j) == new_vec@[j]);
                        };
                    };
                    lemma_push_not_contains_to_set(old(self).elements@, x@);
                    lemma_push_preserves_no_dups(old(self).elements@, x@);
                    vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                }
            }
            // Veracity: NEEDED proof block
            proof {
                if old(self)@.contains(x@) {
                }
                // Veracity: NEEDED assert
                assert(self.spec_arraysetsteph_wf());
            }
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: StT + Ord> Default for ArraySetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord> PartialEqSpecImpl for ArraySetStEph<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord> Eq for ArraySetStEph<T> {}

    impl<T: StT + Ord> PartialEq for ArraySetStEph<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            // Veracity: NEEDED proof block
            proof {
                assume(self.spec_arraysetsteph_wf());
                assume(other.spec_arraysetsteph_wf());
            }
            let equal = self.size() == other.size() && {
                let n = self.elements.length();
                let mut i: usize = 0;
                let mut all_found = true;
                while i < n
                    invariant
                        self.spec_arraysetsteph_wf(),
                        other.spec_arraysetsteph_wf(),
                        n == self.elements@.len(),
                        i <= n,
                    decreases n - i,
                {
                    if !other.find(self.elements.nth(i)) {
                        all_found = false;
                        break;
                    }
                    i += 1;
                }
                all_found
            };
            // Veracity: NEEDED proof block
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: StT + Ord> Clone for ArraySetStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = ArraySetStEph {
                elements: self.elements.clone(),
            };
            // Veracity: NEEDED proof block
            proof {
                assume(obeys_feq_clone::<T>());
                lemma_seq_map_cloned_view_eq(
                    self.elements.seq@,
                    cloned.elements.seq@,
                );
            }
            cloned
        }
    }

    } 

    //		Section 13. macros


    #[macro_export]
    macro_rules! ArraySetStEphLit {
        () => {
            < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord> fmt::Debug for ArraySetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord> fmt::Display for ArraySetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
