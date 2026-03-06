//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Single-threaded ephemeral set implementation using ArraySeqStEph as backing store.
//!
//! View: elements@.to_set()
//! Invariant: elements@.no_duplicates()
//!
//! All membership tests use linear scan. This keeps proofs clean: to_set()
//! correctness follows directly from vstd seq/set lemmas without needing
//! spec-level ordering (TotalOrder). The backing ArraySeq is unordered.

pub mod ArraySetStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

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
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_clone, lemma_cloned_view_eq, lemma_seq_map_cloned_view_eq};

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_lib_default,
        vstd::seq_lib::group_seq_properties,
        vstd::prelude::Seq::group_seq_extra,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::set_lib::group_set_lib_default,
    };


    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySetStEph<T: StT + Ord> {
        pub elements: ArraySeqStEphS<T>,
    }

    pub type ArraySetS<T> = ArraySetStEph<T>;


    // 5. view impls

    impl<T: StT + Ord> View for ArraySetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> {
            self.elements@.to_set()
        }
    }


    // 7. proof fns

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
            assert(tail.no_duplicates()) by {
                assert forall|i: int, j: int| 0 <= i < j < tail.len()
                    implies tail[i] != tail[j] by {
                    assert(s[i + 1] != s[j + 1]);
                }
            }
            lemma_filter_remove::<V>(tail, v);
            let pred = |e: V| e != v;
            assert(s =~= seq![head] + tail);
            Seq::filter_distributes_over_add(seq![head], tail, pred);
            assert(s.filter(pred) =~= seq![head].filter(pred) + tail.filter(pred));
            if head == v {
                assert(seq![head].filter(pred) =~= Seq::empty());
                assert(s.filter(pred) =~= tail.filter(pred));
                assert(s.to_set().remove(v) =~= tail.to_set());
            } else {
                assert(seq![head].filter(pred) =~= seq![head]);
                assert(s.filter(pred) =~= seq![head] + tail.filter(pred));
                assert(!tail.contains(head));
                assert(!tail.filter(pred).contains(head));
                vstd::seq_lib::seq_to_set_distributes_over_add(seq![head], tail.filter(pred));
                lemma_push_not_contains_to_set(tail.filter(pred), head);
                assert((seq![head] + tail.filter(pred)).to_set() =~= (tail.filter(pred) + seq![head]).to_set());
                assert(s.filter(pred).to_set() =~= tail.filter(pred).to_set().insert(head));
                assert(s.to_set().remove(v) =~= tail.to_set().remove(v).insert(head));
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
        assert forall|i: int, j: int| 0 <= i < j < s2.len()
            implies s2[i] != s2[j] by {
            if j < s.len() as int {
                // Both in original seq
                assert(s[i] != s[j]);
            } else {
                // j is the new element position
                assert(j == s.len() as int);
                assert(s2[j] == x);
                assert(s2[i] == s[i]);
                assert(!s.contains(x));
                // s[i] != x because x not in s
            }
        }
    }

    /// Elements of a filtered seq are elements of the original.
    proof fn lemma_filter_in_original<V>(s: Seq<V>, pred: spec_fn(V) -> bool)
        ensures forall|v: V| s.filter(pred).contains(v) ==> s.contains(v)
        decreases s.len()
    {
        reveal(Seq::filter);
        if s.len() > 0 {
            lemma_filter_in_original(s.drop_last(), pred);
            let sub = s.drop_last().filter(pred);
            if pred(s.last()) {
                assert forall|v: V| s.filter(pred).contains(v) implies s.contains(v) by {
                    if s.filter(pred).contains(v) {
                        let j = choose|j: int| 0 <= j < s.filter(pred).len() && s.filter(pred)[j] == v;
                        if j < sub.len() {
                            assert(sub.contains(v));
                            assert(s.drop_last().contains(v));
                            let k = choose|k: int| 0 <= k < s.drop_last().len() && s.drop_last()[k] == v;
                            assert(s[k] == v);
                        } else {
                            assert(v == s.last());
                            assert(s[s.len() as int - 1] == v);
                        }
                    }
                }
            } else {
                assert forall|v: V| s.filter(pred).contains(v) implies s.contains(v) by {
                    if sub.contains(v) {
                        assert(s.drop_last().contains(v));
                        let k = choose|k: int| 0 <= k < s.drop_last().len() && s.drop_last()[k] == v;
                        assert(s[k] == v);
                    }
                }
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
            assert(sdl.no_duplicates()) by {
                assert forall|i: int, j: int| 0 <= i < j < sdl.len()
                    implies sdl[i] != sdl[j] by {
                    assert(s[i] != s[j]);
                }
            }
            lemma_filter_preserves_no_dups(sdl, pred);
            let sub = sdl.filter(pred);
            if pred(s.last()) {
                // s.filter(pred) == sub.push(s.last())
                assert(!sub.contains(s.last())) by {
                    lemma_filter_in_original(sdl, pred);
                    if sub.contains(s.last()) {
                        assert(sdl.contains(s.last()));
                        let k = choose|k: int| 0 <= k < sdl.len() && sdl[k] == s.last();
                        assert(s[k] == s[s.len() as int - 1]);
                        assert(k != s.len() as int - 1);
                        assert(false);
                    }
                }
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
            assert(tail.no_duplicates()) by {
                assert forall|i: int, j: int| 0 <= i < j < tail.len()
                    implies tail[i] != tail[j] by { assert(s[i + 1] != s[j + 1]); }
            }
            lemma_filter_to_set_intersect(tail, set);
            assert(s =~= seq![head] + tail);
            Seq::filter_distributes_over_add(seq![head], tail, pred);
            reveal(Seq::filter);
            if set.contains(head) {
                assert(seq![head].filter(pred) =~= seq![head]);
                assert(s.filter(pred) =~= seq![head] + tail.filter(pred));
                assert(!tail.contains(head));
                assert(!tail.filter(pred).contains(head)) by {
                    lemma_filter_in_original(tail, pred);
                }
                vstd::seq_lib::seq_to_set_distributes_over_add(seq![head], tail.filter(pred));
                lemma_push_not_contains_to_set(tail.filter(pred), head);
            } else {
                assert(seq![head].filter(pred) =~= Seq::empty());
                assert(s.filter(pred) =~= tail.filter(pred));
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
            assert(tail.no_duplicates()) by {
                assert forall|i: int, j: int| 0 <= i < j < tail.len()
                    implies tail[i] != tail[j] by { assert(s[i + 1] != s[j + 1]); }
            }
            lemma_filter_to_set_difference(tail, set);
            assert(s =~= seq![head] + tail);
            Seq::filter_distributes_over_add(seq![head], tail, pred);
            reveal(Seq::filter);
            if !set.contains(head) {
                assert(seq![head].filter(pred) =~= seq![head]);
                assert(s.filter(pred) =~= seq![head] + tail.filter(pred));
                assert(!tail.contains(head));
                assert(!tail.filter(pred).contains(head)) by {
                    lemma_filter_in_original(tail, pred);
                }
                vstd::seq_lib::seq_to_set_distributes_over_add(seq![head], tail.filter(pred));
                lemma_push_not_contains_to_set(tail.filter(pred), head);
            } else {
                assert(seq![head].filter(pred) =~= Seq::empty());
                assert(s.filter(pred) =~= tail.filter(pred));
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
        assert forall|v: V| sub.to_set().contains(v)
            implies orig.to_set().contains(v) by {
            if sub.to_set().contains(v) {
                assert(sub.contains(v));
                let idx = choose|i: int| 0 <= i < sub.len() && sub[i] == v;
                assert(orig.contains(sub[idx]));
            }
        }
    }


    // 8. traits

    pub trait ArraySetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_wf(&self) -> bool;

        fn size(&self) -> (count: usize)
            requires self.spec_wf()
            ensures count == self@.len(), self@.finite();

        fn to_seq(&self) -> (seq: ArraySeqStEphS<T>)
            requires self.spec_wf(),
            ensures
                self@.finite(),
                seq@.no_duplicates(),
                seq@.to_set() =~= self@;

        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_wf();

        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree@.finite(), tree.spec_wf();

        fn from_seq(seq: ArraySeqStEphS<T>) -> (constructed: Self)
            ensures constructed@.finite(), constructed.spec_wf();

        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
            requires
                self.spec_wf(),
                self@.finite(),
            ensures
                filtered@.finite(),
                filtered@.subset_of(self@),
                filtered.spec_wf();

        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_wf(),
                other.spec_wf(),
                self@.finite(),
                other@.finite(),
            ensures
                common@ == self@.intersect(other@),
                common@.finite(),
                common.spec_wf();

        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_wf(),
                other.spec_wf(),
                self@.finite(),
                other@.finite(),
            ensures
                remaining@ == self@.difference(other@),
                remaining@.finite(),
                remaining.spec_wf();

        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_wf(),
                other.spec_wf(),
                self@.finite(),
                other@.finite(),
            ensures
                combined@ == self@.union(other@),
                combined@.finite(),
                combined.spec_wf();

        fn find(&self, x: &T) -> (found: B)
            requires self@.finite(),
            ensures found == self@.contains(x@);

        fn delete(&mut self, x: &T)
            requires
                old(self).spec_wf(),
                old(self)@.finite(),
            ensures
                self@ == old(self)@.remove(x@),
                self@.finite(),
                self.spec_wf();

        fn insert(&mut self, x: T)
            requires
                old(self).spec_wf(),
                old(self)@.finite(),
            ensures
                self@ == old(self)@.insert(x@),
                self@.finite(),
                self.spec_wf();
    }


    // 9. impls

    impl<T: StT + Ord> ArraySetStEph<T> {
        pub open spec fn spec_wf(&self) -> bool {
            self.elements@.no_duplicates()
        }
    }

    // 9. impls

    impl<T: StT + Ord> ArraySetStEphTrait<T> for ArraySetStEph<T> {
        open spec fn spec_wf(&self) -> bool {
            self.elements@.no_duplicates()
        }

        fn size(&self) -> (count: usize)
        {
            proof {
                self.elements@.unique_seq_to_set();
            }
            self.elements.length()
        }

        fn to_seq(&self) -> (seq: ArraySeqStEphS<T>)
        {
            let seq = self.elements.clone();
            proof {
                assume(obeys_feq_clone::<T>());
                lemma_seq_map_cloned_view_eq(
                    self.elements.seq@,
                    seq.seq@,
                );
            }
            seq
        }

        fn empty() -> (empty: Self)
        {
            ArraySetStEph {
                elements: ArraySeqStEphS::empty(),
            }
        }

        fn singleton(x: T) -> (tree: Self)
        {
            let ghost x_view = x@;
            let mut v: Vec<T> = Vec::new();
            v.push(x);
            let ghost v_snapshot = v@;
            assert(v_snapshot.len() == 1);
            assert(v_snapshot[0]@ == x_view);
            let elements = ArraySeqStEphS::from_vec(v);
            proof {
                assert(elements.spec_index(0) == v_snapshot[0]);
                assert(elements.spec_index(0)@ == x_view);
                assert(elements@.len() == 1) by {
                    assert(elements.spec_len() == 1);
                };
                assert(elements@[0] == elements.spec_index(0)@);
                assert(elements@ =~= seq![x_view]);
                Seq::<<T as View>::V>::empty().lemma_push_to_set_commute(x_view);
                assert(seq![x_view] =~= Seq::<<T as View>::V>::empty().push(x_view));
            }
            ArraySetStEph { elements }
        }

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
                    constructed.spec_wf(),
                    i <= seq.spec_len(),
                decreases seq.spec_len() - i,
            {
                let elem = seq.nth(i).clone();
                constructed.insert(elem);
                i += 1;
            }
            constructed
        }

        fn find(&self, x: &T) -> (found: B)
        {
            proof { assume(obeys_feq_full::<T>()); }
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                    obeys_feq_full::<T>(),
                    forall|j: int| 0 <= j < i ==> self.elements@[j] != x@,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if feq(elem, x) {
                    proof {
                        let ii = i as int;
                        lemma_seq_index_in_map_to_set(self.elements.seq@, ii);
                    }
                    return true;
                }
                i += 1;
            }
            proof {
                assert(forall|j: int| 0 <= j < self.elements@.len() ==> self.elements@[j] != x@);
                assert(!self.elements@.contains(x@));
                assert(!self.elements@.to_set().contains(x@));
            }
            false
        }

        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
        {
            proof { assume(obeys_feq_full::<T>()); }
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
                    obeys_feq_full::<T>(),
                    rv_views.len() == result_vec@.len(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> rv_views[j] == result_vec@[j]@,
                    rv_views.no_duplicates(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        ==> old_view.subrange(0, i as int).to_set().contains(rv_views[j]),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                proof { assume(f.requires((&*elem,))); }
                if f(elem) {
                    let cloned_elem = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                        assert(elem@ == old_view[i as int]);
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    proof {
                        // cv not in rv_views because cv = old_view[i] and old_view has no dups
                        // and rv_views elements are all in subrange(0, i) which excludes index i.
                        assert(!rv_views.contains(cv)) by {
                            if rv_views.contains(cv) {
                                let k = choose|k: int| 0 <= k < rv_views.len() && rv_views[k] == cv;
                                assert(old_view.subrange(0, i as int).to_set().contains(cv));
                                assert(old_view.subrange(0, i as int).contains(cv));
                                let m = choose|m: int| 0 <= m < i && old_view.subrange(0, i as int)[m] == cv;
                                assert(old_view[m] == old_view[i as int]);
                                assert(m != i as int);
                                assert(false);
                            }
                        };
                        rv_views = rv_views.push(cv);
                        lemma_push_preserves_no_dups(rv_views.drop_last(), cv);
                        // Update containment: all previous elements in subrange(0, i) ⊆ subrange(0, i+1)
                        let ghost next_sub = old_view.subrange(0, (i + 1) as int);
                        assert forall|j: int| #![trigger rv_views[j]]
                            0 <= j < rv_views.len()
                            implies next_sub.to_set().contains(rv_views[j]) by {
                            if j < rv_views.len() - 1 {
                                assert(old_view.subrange(0, i as int).to_set().contains(rv_views[j]));
                                let m = choose|m: int| 0 <= m < i && old_view.subrange(0, i as int)[m] == rv_views[j];
                                assert(next_sub[m] == rv_views[j]);
                            } else {
                                assert(rv_views[j] == cv);
                                assert(next_sub[i as int] == cv);
                            }
                        };
                    }
                } else {
                    proof {
                        let ghost next_sub = old_view.subrange(0, (i + 1) as int);
                        assert forall|j: int| #![trigger rv_views[j]]
                            0 <= j < rv_views.len()
                            implies next_sub.to_set().contains(rv_views[j]) by {
                            assert(old_view.subrange(0, i as int).to_set().contains(rv_views[j]));
                            let m = choose|m: int| 0 <= m < i && old_view.subrange(0, i as int)[m] == rv_views[j];
                            assert(next_sub[m] == rv_views[j]);
                        };
                    }
                }
                i += 1;
            }
            let filtered = ArraySetStEph { elements: ArraySeqStEphS::from_vec(result_vec) };
            proof {
                assert(old_view.subrange(0, n as int) =~= old_view);
                assert(filtered.elements@ =~= rv_views) by {
                    assert(filtered.elements.seq@.len() == result_vec@.len());
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies filtered.elements@[j] == rv_views[j] by {
                        assert(filtered.elements.spec_index(j) == result_vec@[j]);
                    };
                };
                // subset_of: rv_views.to_set() ⊆ old_view.to_set() = self@
                assert(filtered@.subset_of(self@)) by {
                    assert forall|e: <T as View>::V| filtered@.contains(e)
                        implies self@.contains(e) by {
                        if filtered@.contains(e) {
                            assert(rv_views.to_set().contains(e));
                            assert(rv_views.contains(e));
                            let k = choose|k: int| 0 <= k < rv_views.len() && rv_views[k] == e;
                            assert(old_view.to_set().contains(e));
                        }
                    };
                };
                vstd::seq_lib::seq_to_set_is_finite(filtered.elements@);
            }
            filtered
        }

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            proof { assume(obeys_feq_full::<T>()); }
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
                    other.spec_wf(),
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
                proof {
                    assert(next_prefix =~= prefix.push(old_view[i as int]));
                    Seq::filter_distributes_over_add(prefix, seq![old_view[i as int]], filt);
                }
                if other.find(elem) {
                    let cloned_elem = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                        assert(elem@ == old_view[i as int]);
                        assert(other_set.contains(old_view[i as int]));
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt).push(old_view[i as int]));
                    }
                } else {
                    proof {
                        assert(elem@ == old_view[i as int]);
                        assert(!other_set.contains(old_view[i as int]));
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt));
                    }
                }
                i += 1;
            }
            let common = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            proof {
                assert(old_view.subrange(0, n as int) =~= old_view);
                assert(common.elements@ =~= rv_views) by {
                    assert(common.elements.seq@.len() == result_vec@.len());
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies common.elements@[j] == rv_views[j] by {
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

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            proof { assume(obeys_feq_full::<T>()); }
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
                    other.spec_wf(),
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
                proof {
                    assert(next_prefix =~= prefix.push(old_view[i as int]));
                    Seq::filter_distributes_over_add(prefix, seq![old_view[i as int]], filt);
                }
                if !other.find(elem) {
                    let cloned_elem = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                        assert(elem@ == old_view[i as int]);
                        assert(!other_set.contains(old_view[i as int]));
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt).push(old_view[i as int]));
                    }
                } else {
                    proof {
                        assert(elem@ == old_view[i as int]);
                        assert(other_set.contains(old_view[i as int]));
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt));
                    }
                }
                i += 1;
            }
            let remaining = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            proof {
                assert(old_view.subrange(0, n as int) =~= old_view);
                assert(remaining.elements@ =~= rv_views) by {
                    assert(remaining.elements.seq@.len() == result_vec@.len());
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies remaining.elements@[j] == rv_views[j] by {
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

        fn union(&self, other: &Self) -> (combined: Self)
        {
            proof { assume(obeys_feq_full::<T>()); }
            let ghost self_view = self.elements@;
            let ghost other_view = other.elements@;
            let ghost self_set = self@;
            let self_len = self.elements.length();
            let other_len = other.elements.length();
            let mut result_vec: Vec<T> = Vec::new();
            let ghost mut rv_views: Seq<<T as View>::V> = Seq::empty();

            // Phase 1: copy all of self's elements.
            let mut i: usize = 0;
            while i < self_len
                invariant
                    i <= self_len,
                    self_len as int == self.elements.spec_len(),
                    self.elements@ == self_view,
                    obeys_feq_full::<T>(),
                    rv_views =~= self_view.subrange(0, i as int),
                    rv_views.len() == result_vec@.len(),
                    forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len() ==> rv_views[j] == result_vec@[j]@,
                decreases self_len - i,
            {
                let elem = self.elements.nth(i);
                let cloned_elem = elem.clone();
                proof {
                    lemma_cloned_view_eq(*elem, cloned_elem);
                }
                let ghost cv = cloned_elem@;
                result_vec.push(cloned_elem);
                proof {
                    rv_views = rv_views.push(cv);
                    assert(self_view.subrange(0, (i + 1) as int) =~=
                        self_view.subrange(0, i as int).push(self_view[i as int]));
                }
                i += 1;
            }
            proof {
                assert(self_view.subrange(0, self_len as int) =~= self_view);
            }

            // Phase 2: add other's elements not in self.
            let mut j: usize = 0;
            while j < other_len
                invariant
                    j <= other_len,
                    other_len as int == other.elements.spec_len(),
                    other.elements@ == other_view,
                    self.spec_wf(),
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
                proof {
                    assert(next_prefix =~= prefix.push(other_view[j as int]));
                    Seq::filter_distributes_over_add(prefix, seq![other_view[j as int]], filt);
                }
                if !self.find(elem) {
                    let cloned_elem = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                        assert(elem@ == other_view[j as int]);
                        assert(!self_set.contains(other_view[j as int]));
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt).push(other_view[j as int]));
                        assert(self_view + next_prefix.filter(filt) =~=
                            (self_view + prefix.filter(filt)).push(other_view[j as int]));
                    }
                } else {
                    proof {
                        assert(elem@ == other_view[j as int]);
                        assert(self_set.contains(other_view[j as int]));
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt));
                    }
                }
                j += 1;
            }
            let combined = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            proof {
                assert(other_view.subrange(0, other_len as int) =~= other_view);
                assert(combined.elements@ =~= rv_views) by {
                    assert(combined.elements.seq@.len() == result_vec@.len());
                    assert forall|k: int| #![trigger rv_views[k]]
                        0 <= k < rv_views.len()
                        implies combined.elements@[k] == rv_views[k] by {
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
                assert(combined.elements@.no_duplicates()) by {
                    let sv = self_view;
                    let fv = other_view.filter(filt);
                    let concat = sv + fv;
                    assert(combined.elements@ =~= concat);
                    assert(sv.no_duplicates());
                    assert(fv.no_duplicates());
                    assert forall|i2: int, j2: int| 0 <= i2 < j2 < concat.len()
                        implies concat[i2] != concat[j2] by {
                        if i2 < sv.len() as int && j2 < sv.len() as int {
                            assert(sv[i2] != sv[j2]);
                        } else if i2 >= sv.len() as int && j2 >= sv.len() as int {
                            let fi = i2 - sv.len() as int;
                            let fj = j2 - sv.len() as int;
                            assert(concat[i2] == fv[fi]);
                            assert(concat[j2] == fv[fj]);
                            assert(fv[fi] != fv[fj]);
                        } else {
                            // i2 in self_view, j2 in filtered other
                            let fj = j2 - sv.len() as int;
                            assert(concat[i2] == sv[i2]);
                            assert(concat[j2] == fv[fj]);
                            lemma_filter_in_original(other_view, filt);
                            assert(!self_set.contains(fv[fj]));
                            assert(sv.to_set().contains(sv[i2]));
                            assert(self_set.contains(sv[i2]));
                        }
                    };
                };
            }
            combined
        }

        fn delete(&mut self, x: &T)
        {
            proof { assume(obeys_feq_full::<T>()); }
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
                    obeys_feq_full::<T>(),
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
                proof {
                    assert(next_prefix =~= prefix.push(old_view[i as int]));
                    Seq::filter_distributes_over_add(prefix, seq![old_view[i as int]], filt);
                }
                if !feq(elem, x) {
                    let cloned_elem = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                        assert(elem@ == old_view[i as int]);
                        assert(old_view[i as int] != x_view);
                    }
                    let ghost cv = cloned_elem@;
                    result_vec.push(cloned_elem);
                    proof {
                        rv_views = rv_views.push(cv);
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt).push(old_view[i as int]));
                    }
                } else {
                    proof {
                        assert(elem@ == old_view[i as int]);
                        assert(old_view[i as int] == x_view);
                        reveal(Seq::filter);
                        assert(next_prefix.filter(filt) =~= prefix.filter(filt));
                    }
                }
                i += 1;
            }
            self.elements = ArraySeqStEphS::from_vec(result_vec);
            proof {
                let ghost filt = |e: <T as View>::V| e != x_view;
                assert(old_view.subrange(0, n as int) =~= old_view);
                assert(self.elements@ =~= rv_views) by {
                    assert(self.elements.seq@.len() == result_vec@.len());
                    assert forall|j: int| #![trigger rv_views[j]]
                        0 <= j < rv_views.len()
                        implies self.elements@[j] == rv_views[j] by {
                        assert(self.elements.spec_index(j) == result_vec@[j]);
                    };
                };
                assert(self.elements@ =~= old(self).elements@.filter(filt));
                lemma_filter_remove(old(self).elements@, x@);
                assert(self@ == old(self)@.remove(x@));
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                lemma_filter_preserves_no_dups(old(self).elements@, filt);
            }
        }

        fn insert(&mut self, x: T)
        {
            if !self.find(&x) {
                proof { assume(obeys_feq_full::<T>()); }
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
                        obeys_feq_full::<T>(),
                        rv_views =~= old_view.subrange(0, i as int),
                        rv_views.len() == new_vec@.len(),
                        forall|j: int| #![trigger rv_views[j]]
                            0 <= j < rv_views.len() ==> rv_views[j] == new_vec@[j]@,
                    decreases n - i,
                {
                    let elem = self.elements.nth(i);
                    let cloned_elem = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, cloned_elem);
                    }
                    let ghost cv = cloned_elem@;
                    new_vec.push(cloned_elem);
                    proof {
                        rv_views = rv_views.push(cv);
                        assert(old_view.subrange(0, (i + 1) as int) =~=
                            old_view.subrange(0, i as int).push(old_view[i as int]));
                    }
                    i += 1;
                }
                new_vec.push(x);
                proof {
                    rv_views = rv_views.push(x_view);
                    assert(old_view.subrange(0, n as int) =~= old_view);
                }
                self.elements = ArraySeqStEphS::from_vec(new_vec);
                proof {
                    assert(self.elements@ =~= rv_views) by {
                        assert(self.elements.seq@.len() == new_vec@.len());
                        assert forall|j: int| #![trigger rv_views[j]]
                            0 <= j < rv_views.len()
                            implies self.elements@[j] == rv_views[j] by {
                            assert(self.elements.spec_index(j) == new_vec@[j]);
                        };
                    };
                    assert(self.elements@ =~= old(self).elements@.push(x@));
                    lemma_push_not_contains_to_set(old(self).elements@, x@);
                    lemma_push_preserves_no_dups(old(self).elements@, x@);
                    assert(self.elements@.to_set() =~= old(self).elements@.to_set().insert(x@));
                    assert(self@ == old(self)@.insert(x@));
                    vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                    assert(self.elements@.no_duplicates());
                }
            }
            proof {
                if old(self)@.contains(x@) {
                    assert(self@ == old(self)@);
                }
                assert(self@.finite());
                assert(self.spec_wf());
            }
        }
    }


    // 11. derive impls in verus!

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
            proof {
                assume(self.spec_wf());
                assume(other.spec_wf());
            }
            let equal = self.size() == other.size() && {
                let n = self.elements.length();
                let mut i: usize = 0;
                let mut all_found = true;
                while i < n
                    invariant
                        self.spec_wf(),
                        other.spec_wf(),
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

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

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
