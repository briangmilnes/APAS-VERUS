//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent set implementation using AVLTreeSeqStPer as backing store.
//!
//! Limitation: AVLTreeSeqStPer has no value-based insert/delete. find uses binary search
//! on the sorted logical sequence (O(log n) via nth). insert/delete use filter-and-rebuild.

pub mod AVLTreeSetStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt::{Display, Formatter};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetStPer<T: StT + Ord> {
        pub elements: AVLTreeSeqStPerS<T>,
    }

    pub type AVLTreeSetPer<T> = AVLTreeSetStPer<T>;

    // 5. view impls

    impl<T: StT + Ord> View for AVLTreeSetStPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.elements@.to_set() }
    }

    // 8. traits

    pub trait AVLTreeSetStPerTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetstper_wf(&self) -> bool;

        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetstper_wf(),
            ensures count == self@.len();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            ensures
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetstper_wf();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetstper_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            requires seq.spec_avltreeseqstper_wf(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetstper_wf();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetstper_wf(),
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
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetstper_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetstper_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetstper_wf(),
                other.spec_avltreesetstper_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetstper_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            requires self.spec_avltreesetstper_wf(),
            ensures found == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (updated: Self)
            requires self.spec_avltreesetstper_wf(),
            ensures
                updated@ == self@.remove(x@),
                updated.spec_avltreesetstper_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (updated: Self)
            requires
                self.spec_avltreesetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                updated@ == self@.insert(x@),
                updated.spec_avltreesetstper_wf();
    }

    // 9. impls

    impl<T: StT + Ord> AVLTreeSetStPerTrait<T> for AVLTreeSetStPer<T> {
        open spec fn spec_avltreesetstper_wf(&self) -> bool {
            self.elements.spec_avltreeseqstper_wf()
            && self.elements@.no_duplicates()
            && self@.finite()
            && obeys_feq_full::<T>()
        }

        fn size(&self) -> (count: usize)
        {
            let r = self.elements.length();
            proof {
                self.elements@.unique_seq_to_set();
            }
            r
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
        {
            let seq = self.elements.clone();
            proof {
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                assert(seq@ =~= self.elements@);
            }
            seq
        }

        fn empty() -> (empty: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let empty = AVLTreeSetStPer { elements: AVLTreeSeqStPerS::empty() };
            assert(empty.elements@ =~= Seq::<<T as View>::V>::empty());
            assert(empty@ =~= Set::<<T as View>::V>::empty());
            empty
        }

        fn singleton(x: T) -> (tree: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let ghost x_view = x@;
            let tree = AVLTreeSetStPer { elements: AVLTreeSeqStPerS::singleton(x) };
            proof {
                assert(tree.elements@ =~= seq![x_view]);
                let s = tree.elements@;
                assert forall |v| #[trigger] s.to_set().contains(v) <==> Set::<<T as View>::V>::empty().insert(x_view).contains(v) by {
                    if Set::<<T as View>::V>::empty().insert(x_view).contains(v) {
                        assert(v == x_view);
                        assert(s[0] == x_view);
                        assert(s.contains(v));
                    }
                    if s.to_set().contains(v) {
                        assert(s.contains(v));
                        let i = choose |i: int| 0 <= i < s.len() && s[i] == v;
                        assert(i == 0);
                        assert(v == x_view);
                    }
                }
                assert(tree@ =~= Set::<<T as View>::V>::empty().insert(x_view));
                // Length-1 seq trivially has no_duplicates.
                assert(tree.elements@.no_duplicates());
                vstd::seq_lib::seq_to_set_is_finite(tree.elements@);
            }
            tree
        }

        #[verifier::loop_isolation(false)]
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
        {
            let mut constructed = Self::empty();
            let n = seq.length();
            proof {
                lemma_size_lt_usize_max::<T>(&seq.root);
                lemma_size_eq_inorder_len::<T>(&seq.root);
                assert(spec_cached_size(&seq.root) == seq.spec_seq().len());
                assert(spec_cached_size(&seq.root) < usize::MAX);
                // n as nat == spec_cached_size(&seq.root) == seq.spec_seq().len() < usize::MAX.
                assert(n < usize::MAX);
            }
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqstper_wf(),
                    n as int == seq.spec_seq().len(),
                    n < usize::MAX,
                    i <= n,
                    constructed@.finite(),
                    constructed.spec_avltreesetstper_wf(),
                    constructed@.len() <= i as nat,

                    forall|j: int| 0 <= j < i ==> #[trigger] constructed@.contains(seq@[j]),
                    forall|v: <T as View>::V| #[trigger] constructed@.contains(v) ==>
                        (exists|j: int| 0 <= j < i && seq@[j] == v),
                decreases n - i,
            {
                let r = seq.nth(i);
                let elem = r.clone();
                proof {
                    lemma_cloned_view_eq(*r, elem);
                    // Capacity: constructed@.len() <= i < n < usize::MAX (from wf lemma).
                    assert(constructed@.len() + 1 <= n as nat);
                    assert(constructed@.len() + 1 < usize::MAX as nat);
                }
                let ghost old_view = constructed@;
                constructed = constructed.insert(elem);
                proof {
                    // constructed@.len() <= i + 1 (at most one new element added).
                    assert(constructed@.len() <= (i + 1) as nat);
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

        #[verifier::loop_isolation(false)]
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let mut filtered = Self::empty();
            let n = self.elements.length();
            proof {
                lemma_size_lt_usize_max::<T>(&self.elements.root);
                lemma_size_eq_inorder_len::<T>(&self.elements.root);
                assert(spec_cached_size(&self.elements.root) == self.elements.spec_seq().len());
                assert(spec_cached_size(&self.elements.root) < usize::MAX);
                assert(n < usize::MAX);
            }
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    n < usize::MAX,
                    i <= n,
                    filtered@.finite(),
                    filtered.spec_avltreesetstper_wf(),
                    filtered@.len() <= i as nat,
                    filtered@.subset_of(self@),

                    forall|t: &T| #[trigger] f.requires((t,)),
                    forall|x: T, keep: bool|
                        f.ensures((&x,), keep) ==> keep == spec_pred(x@),
                    forall|v: T::V| #[trigger] filtered@.contains(v) ==> spec_pred(v),
                    forall|j: int| #![trigger self.elements@[j]]
                        0 <= j < i && spec_pred(self.elements@[j])
                        ==> filtered@.contains(self.elements@[j]),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                let keep = f(elem);
                proof {
                    assert(keep == spec_pred(self.elements@[i as int]));
                }
                if keep {
                    let c = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@[i as int] == elem@);
                        assert(self.elements@.contains(elem@));
                        assert(self@.contains(elem@));
                        // Capacity: filtered@.len() <= i < n < usize::MAX.
                        assert(filtered@.len() + 1 <= n as nat);
                        assert(filtered@.len() + 1 < usize::MAX as nat);
                    }
                    let ghost old_filtered_view = filtered@;
                    filtered = filtered.insert(c);
                    proof {
                        // filtered@.len() <= i + 1 after insert.
                        assert(filtered@.len() <= (i + 1) as nat);
                        assert(filtered@.subset_of(self@)) by {
                            assert forall|x| #[trigger] filtered@.contains(x)
                                implies self@.contains(x) by {
                                if !old_filtered_view.contains(x) {
                                    assert(x == elem@);
                                }
                            };
                        };
                        assert forall|v: T::V| #[trigger] filtered@.contains(v)
                            implies spec_pred(v) by {
                            if !old_filtered_view.contains(v) {
                                assert(v == c@);
                            }
                        };
                        assert forall|j: int| #![trigger self.elements@[j]]
                            0 <= j < (i + 1) as int && spec_pred(self.elements@[j])
                            implies filtered@.contains(self.elements@[j]) by {
                            if j < i as int {
                                assert(old_filtered_view.contains(self.elements@[j]));
                            } else {
                                assert(filtered@.contains(c@));
                            }
                        };
                    }
                }
                i += 1;
            }
            proof {
                assert forall|v: T::V| self@.contains(v) && spec_pred(v)
                    implies #[trigger] filtered@.contains(v) by {
                    assert(self.elements@.to_set().contains(v));
                    assert(self.elements@.contains(v));
                    let j = choose|j: int| 0 <= j < self.elements@.len()
                        && self.elements@[j] == v;
                    assert(spec_pred(self.elements@[j]));
                };
            }
            filtered
        }

        #[verifier::loop_isolation(false)]
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let mut common = Self::empty();
            let n = self.elements.length();
            proof {
                lemma_size_lt_usize_max::<T>(&self.elements.root);
                lemma_size_eq_inorder_len::<T>(&self.elements.root);
                assert(spec_cached_size(&self.elements.root) == self.elements.spec_seq().len());
                assert(spec_cached_size(&self.elements.root) < usize::MAX);
                assert(n < usize::MAX);
            }
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    other.spec_avltreesetstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    n < usize::MAX,
                    i <= n,
                    common@.finite(),
                    common.spec_avltreesetstper_wf(),
                    common@.len() <= i as nat,

                    common@.subset_of(self@.intersect(other@)),
                    forall|j: int| #![trigger self.elements@[j]]
                        0 <= j < i && other@.contains(self.elements@[j])
                        ==> common@.contains(self.elements@[j]),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    let c = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@.contains(elem@));
                        assert(self@.contains(elem@));
                        // Capacity: common@.len() <= i < n < usize::MAX.
                        assert(common@.len() + 1 <= n as nat);
                        assert(common@.len() + 1 < usize::MAX as nat);
                    }
                    let ghost old_common = common@;
                    common = common.insert(c);
                    proof {
                        // common@.len() <= i + 1 after insert.
                        assert(common@.len() <= (i + 1) as nat);
                        assert(common@.subset_of(self@.intersect(other@))) by {
                            assert forall|x| #[trigger] common@.contains(x)
                                implies self@.intersect(other@).contains(x) by {
                                if !old_common.contains(x) {
                                    assert(x == elem@);
                                }
                            };
                        };
                    }
                }
                i += 1;
            }
            proof {
                assert forall|x| self@.intersect(other@).contains(x)
                    implies #[trigger] common@.contains(x) by {
                    assert(self.elements@.to_set().contains(x));
                    assert(self.elements@.contains(x));
                    let j = choose|j: int| 0 <= j < self.elements@.len()
                        && self.elements@[j] == x;
                    assert(self.elements@[j] == x);
                    assert(other@.contains(self.elements@[j]));
                };
                assert(common@ =~= self@.intersect(other@));
            }
            common
        }

        #[verifier::loop_isolation(false)]
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let mut remaining = Self::empty();
            let n = self.elements.length();
            proof {
                lemma_size_lt_usize_max::<T>(&self.elements.root);
                lemma_size_eq_inorder_len::<T>(&self.elements.root);
                assert(spec_cached_size(&self.elements.root) == self.elements.spec_seq().len());
                assert(spec_cached_size(&self.elements.root) < usize::MAX);
                assert(n < usize::MAX);
            }
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    other.spec_avltreesetstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    n < usize::MAX,
                    i <= n,
                    remaining@.finite(),
                    remaining.spec_avltreesetstper_wf(),
                    remaining@.len() <= i as nat,

                    remaining@.subset_of(self@.difference(other@)),
                    forall|j: int| #![trigger self.elements@[j]]
                        0 <= j < i && !other@.contains(self.elements@[j])
                        ==> remaining@.contains(self.elements@[j]),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    let c = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@.contains(elem@));
                        assert(self@.contains(elem@));
                        // Capacity: remaining@.len() <= i < n < usize::MAX.
                        assert(remaining@.len() + 1 <= n as nat);
                        assert(remaining@.len() + 1 < usize::MAX as nat);
                    }
                    let ghost old_remaining = remaining@;
                    remaining = remaining.insert(c);
                    proof {
                        // remaining@.len() <= i + 1 after insert.
                        assert(remaining@.len() <= (i + 1) as nat);
                        assert(remaining@.subset_of(self@.difference(other@))) by {
                            assert forall|x| #[trigger] remaining@.contains(x)
                                implies self@.difference(other@).contains(x) by {
                                if !old_remaining.contains(x) {
                                    assert(x == elem@);
                                }
                            };
                        };
                    }
                }
                i += 1;
            }
            proof {
                assert forall|x| self@.difference(other@).contains(x)
                    implies #[trigger] remaining@.contains(x) by {
                    assert(self.elements@.to_set().contains(x));
                    assert(self.elements@.contains(x));
                    let j = choose|j: int| 0 <= j < self.elements@.len()
                        && self.elements@[j] == x;
                    assert(self.elements@[j] == x);
                    assert(!other@.contains(self.elements@[j]));
                };
                assert(remaining@ =~= self@.difference(other@));
            }
            remaining
        }

        #[verifier::loop_isolation(false)]
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let mut combined = Self::empty();
            let self_len = self.elements.length();
            proof {
                lemma_size_lt_usize_max::<T>(&self.elements.root);
                lemma_size_eq_inorder_len::<T>(&self.elements.root);
                assert(spec_cached_size(&self.elements.root) == self.elements.spec_seq().len());
                assert(spec_cached_size(&self.elements.root) < usize::MAX);
                assert(self_len < usize::MAX);
            }
            let mut i: usize = 0;
            while i < self_len
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    self_len < usize::MAX,
                    i <= self_len,
                    combined@.finite(),
                    combined.spec_avltreesetstper_wf(),
                    combined@.len() <= i as nat,

                    combined@.subset_of(self@.union(other@)),
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < i ==> combined@.contains(self.elements@[k]),
                decreases self_len - i,
            {
                let elem = self.elements.nth(i);
                let c = elem.clone();
                proof {
                    lemma_cloned_view_eq(*elem, c);
                    assert(self.elements@.contains(elem@));
                    assert(self@.contains(elem@));
                    // Capacity: combined@.len() <= i < self_len < usize::MAX.
                    assert(combined@.len() + 1 <= self_len as nat);
                    assert(combined@.len() + 1 < usize::MAX as nat);
                }
                let ghost old_combined = combined@;
                combined = combined.insert(c);
                proof {
                    // combined@.len() <= i + 1 after insert.
                    assert(combined@.len() <= (i + 1) as nat);
                    assert(combined@.subset_of(self@.union(other@))) by {
                        assert forall|x| #[trigger] combined@.contains(x)
                            implies self@.union(other@).contains(x) by {
                            if !old_combined.contains(x) {
                                assert(x == elem@);
                            }
                        };
                    };
                }
                i += 1;
            }
            let other_len = other.elements.length();
            let mut j: usize = 0;
            proof {
                self.elements@.unique_seq_to_set();
                other.elements@.unique_seq_to_set();
                lemma_size_lt_usize_max::<T>(&other.elements.root);
                lemma_size_eq_inorder_len::<T>(&other.elements.root);
            }
            while j < other_len
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    other.elements.spec_avltreeseqstper_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    other_len as int == other.elements.spec_seq().len(),
                    self_len as nat == self@.len(),
                    other_len as nat == other@.len(),
                    self@.len() + other@.len() < usize::MAX as nat,
                    j <= other_len,
                    combined@.finite(),
                    combined.spec_avltreesetstper_wf(),
                    combined@.len() <= self_len as nat + j as nat,
                    combined@.subset_of(self@.union(other@)),
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < self_len ==> combined@.contains(self.elements@[k]),
                    forall|k: int| #![trigger other.elements@[k]]
                        0 <= k < j ==> combined@.contains(other.elements@[k]),
                decreases other_len - j,
            {
                let elem = other.elements.nth(j);
                let c = elem.clone();
                proof {
                    lemma_cloned_view_eq(*elem, c);
                    assert(other.elements@.contains(elem@));
                    assert(other@.contains(elem@));
                    // combined@.len() <= self_len + j < self_len + other_len < usize::MAX.
                    assert(combined@.len() + 1 < usize::MAX as nat);
                }
                let ghost old_combined = combined@;
                combined = combined.insert(c);
                proof {
                    assert(combined@.subset_of(self@.union(other@))) by {
                        assert forall|x| #[trigger] combined@.contains(x)
                            implies self@.union(other@).contains(x) by {
                            if !old_combined.contains(x) {
                                assert(x == elem@);
                            }
                        };
                    };
                }
                j += 1;
            }
            proof {
                assert forall|x| self@.union(other@).contains(x)
                    implies #[trigger] combined@.contains(x) by {
                    if self@.contains(x) {
                        assert(self.elements@.to_set().contains(x));
                        assert(self.elements@.contains(x));
                        let k = choose|k: int| 0 <= k < self.elements@.len()
                            && self.elements@[k] == x;
                        assert(self.elements@[k] == x);
                    } else {
                        assert(other@.contains(x));
                        assert(other.elements@.to_set().contains(x));
                        assert(other.elements@.contains(x));
                        let k = choose|k: int| 0 <= k < other.elements@.len()
                            && other.elements@[k] == x;
                        assert(other.elements@[k] == x);
                    }
                };
                assert(combined@ =~= self@.union(other@));
            }
            combined
        }

        #[verifier::loop_isolation(false)]
        fn find(&self, x: &T) -> (found: B)
        {
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),

                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < i ==> self.elements@[k] != x@,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if feq(elem, x) {
                    assert(self.elements@[i as int] == x@);
                    assert(self.elements@.contains(x@));
                    return true;
                }
                i += 1;
            }
            proof {
                if self@.contains(x@) {
                    assert(self.elements@.to_set().contains(x@));
                    assert(self.elements@.contains(x@));
                    let k = choose|k: int| 0 <= k < self.elements@.len()
                        && self.elements@[k] == x@;
                    assert(false);
                }
            }
            false
        }

        #[verifier::loop_isolation(false)]
        fn delete(&self, x: &T) -> (updated: Self)
        {
            let n = self.elements.length();
            let mut result_vec: Vec<T> = Vec::new();
            let mut i: usize = 0;
            let ghost mut skipped: int = 0;
            let ghost mut skip_idx: int = -1;
            let ghost mut result_views: Seq<<T as View>::V> = Seq::empty();
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    self.elements@.no_duplicates(),

                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    result_vec@.len() == i as int - skipped,
                    0 <= skipped <= 1,
                    skipped > 0 ==> (0 <= skip_idx < i && self.elements@[skip_idx] == x@),
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < i && self.elements@[k] == x@ ==> skipped > 0,
                    // Ghost view tracking.
                    result_views.len() == result_vec@.len(),
                    forall|k: int| #![trigger result_views[k]]
                        0 <= k < result_views.len() ==> result_views[k] == result_vec@[k]@,
                    forall|k: int| #![trigger result_views[k]]
                        0 <= k < result_views.len() ==> (
                            result_views[k] != x@
                            && self.elements@.to_set().contains(result_views[k])
                        ),
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < i && self.elements@[k] != x@
                        ==> result_views.contains(self.elements@[k]),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if !feq(elem, x) {
                    let c = elem.clone();
                    let ghost old_result_views = result_views;
                    let ghost old_rv_len = result_views.len() as int;
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@.contains(elem@));
                        result_views = result_views.push(elem@);
                    }
                    result_vec.push(c);
                    proof {
                        // Prove coverage for the full range 0..=i (becomes 0..new_i after i += 1).
                        assert forall|k: int|
                            #![trigger self.elements@[k]]
                            0 <= k <= i && self.elements@[k] != x@
                            implies result_views.contains(self.elements@[k]) by {
                            if k < i as int {
                                // Old coverage: self.elements@[k] was in old_result_views.
                                assert(old_result_views.contains(self.elements@[k]));
                                let j = choose|j: int| 0 <= j < old_result_views.len()
                                    && old_result_views[j] == self.elements@[k];
                                // After push, old indices are preserved.
                                assert(result_views[j] == self.elements@[k]);
                            } else {
                                // k == i: just pushed elem@ which equals self.elements@[i].
                                assert(result_views[old_rv_len] == elem@);
                            }
                        };
                    }
                } else {
                    proof {
                        if skipped > 0 {
                            assert(self.elements@[skip_idx] != self.elements@[i as int]);
                        }
                        skip_idx = i as int;
                        skipped = skipped + 1;
                    }
                }
                i += 1;
            }
            let ghost result_len = result_vec@.len();
            proof {
                // result_vec@.len() = n - skipped <= n. From wf: n < usize::MAX.
                lemma_size_lt_usize_max::<T>(&self.elements.root);
                lemma_size_eq_inorder_len::<T>(&self.elements.root);
                // spec_cached_size(&root) < usize::MAX and n = spec_cached_size(&root).
                assert(spec_cached_size(&self.elements.root) < usize::MAX);
                assert(spec_cached_size(&self.elements.root) == n as nat);
                assert(result_len <= n as nat);
                assert(result_len < usize::MAX);
            }
            let updated = AVLTreeSetStPer { elements: AVLTreeSeqStPerS::from_vec(result_vec) };
            proof {
                vstd::seq_lib::seq_to_set_is_finite(updated.elements@);
                // Connect result_views to updated.elements@ via from_vec.
                assert(updated.elements@.len() == result_views.len());
                assert forall|k: int| 0 <= k < result_views.len()
                    implies result_views[k] == #[trigger] updated.elements@[k] by {
                    assert(result_views[k] == result_vec@[k]@);
                };
                assert(result_views =~= updated.elements@);
                // Prove updated@ == self@.remove(x@).
                assert forall|v|
                    updated.elements@.to_set().contains(v)
                    <==> #[trigger] self@.remove(x@).contains(v) by {
                    if updated.elements@.to_set().contains(v) {
                        assert(result_views.to_set().contains(v));
                        assert(result_views.contains(v));
                        let k = choose|k: int| 0 <= k < result_views.len()
                            && result_views[k] == v;
                        assert(result_views[k] == v);
                    }
                    if self@.remove(x@).contains(v) {
                        assert(self@.contains(v) && v != x@);
                        assert(self.elements@.to_set().contains(v));
                        assert(self.elements@.contains(v));
                        let k = choose|k: int| 0 <= k < self.elements@.len()
                            && self.elements@[k] == v;
                        assert(self.elements@[k] != x@);
                        assert(result_views.contains(self.elements@[k]));
                    }
                };
                assert(updated@ =~= self@.remove(x@));
                // Prove no_duplicates via cardinality.
                self.elements@.unique_seq_to_set();
                assert(updated.elements@.len() == result_len);
                if self@.contains(x@) {
                    assert(self.elements@.to_set().contains(x@));
                    assert(self.elements@.contains(x@));
                    let witness = choose|k: int| 0 <= k < self.elements@.len()
                        && self.elements@[k] == x@;
                    assert(self.elements@[witness] == x@);
                    assert(skipped == 1);
                } else {
                    assert(skipped == 0);
                }
                assert(updated.elements@.to_set().len() == updated.elements@.len());
                updated.elements@.lemma_no_dup_set_cardinality();
            }
            updated
        }

        #[verifier::loop_isolation(false)]
        fn insert(&self, x: T) -> (updated: Self)
        {
            let ghost x_view = x@;
            let ghost old_seq_len = self.elements@.len();
            let found = self.find(&x);
            if found {
                let updated = Self { elements: self.elements.clone() };
                proof {
                    assert(self@.contains(x_view));
                    assert(updated.elements@ =~= self.elements@);
                    assert(updated@ =~= self@);
                    assert(self@.insert(x_view) =~= self@);
                    vstd::seq_lib::seq_to_set_is_finite(updated.elements@);
                }
                return updated;
            }
            let n = self.elements.length();
            let mut lo: usize = 0;
            let mut hi: usize = n;
            while lo < hi
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    lo <= hi, hi <= n,
                decreases hi - lo,
            {
                let mid = lo + (hi - lo) / 2;
                if *self.elements.nth(mid) < x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            let mut new_vec: Vec<T> = Vec::new();
            let ghost mut rv: Seq<<T as View>::V> = Seq::empty();
            let mut i: usize = 0;
            while i < lo
                invariant
                    self.elements.spec_avltreeseqstper_wf(),

                    n as int == self.elements.spec_seq().len(),
                    i <= lo, lo <= n,
                    new_vec@.len() == i as int,
                    rv.len() == new_vec@.len(),
                    forall|k: int| #![trigger rv[k]]
                        0 <= k < rv.len() ==> rv[k] == new_vec@[k]@,
                    forall|k: int| #![trigger rv[k]]
                        0 <= k < rv.len() ==> self@.contains(rv[k]),
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < i ==> rv.contains(self.elements@[k]),
                decreases lo - i,
            {
                let elem = self.elements.nth(i);
                let c = elem.clone();
                let ghost old_rv = rv;
                let ghost old_rv_len = rv.len() as int;
                proof {
                    lemma_cloned_view_eq(*elem, c);
                    assert(self.elements@.contains(elem@));
                    rv = rv.push(elem@);
                }
                new_vec.push(c);
                proof {
                    assert forall|k: int|
                        #![trigger self.elements@[k]]
                        0 <= k <= i && true
                        implies rv.contains(self.elements@[k]) by {
                        if k < i as int {
                            assert(old_rv.contains(self.elements@[k]));
                            let j = choose|j: int| 0 <= j < old_rv.len()
                                && old_rv[j] == self.elements@[k];
                            assert(rv[j] == self.elements@[k]);
                        } else {
                            assert(rv[old_rv_len] == elem@);
                        }
                    };
                }
                i += 1;
            }
            // Push x into the sequence.
            let ghost pre_x_rv = rv;
            let ghost pre_x_rv_len = rv.len() as int;
            proof { rv = rv.push(x_view); }
            new_vec.push(x);
            proof {
                // x_view is now in rv (just pushed).
                assert(rv[pre_x_rv_len] == x_view);
                assert(rv.contains(x_view));
                // Coverage of [0..lo] preserved after push.
                assert forall|k: int|
                    #![trigger self.elements@[k]]
                    0 <= k < lo
                    implies rv.contains(self.elements@[k]) by {
                    assert(pre_x_rv.contains(self.elements@[k]));
                    let w = choose|w: int| 0 <= w < pre_x_rv.len()
                        && pre_x_rv[w] == self.elements@[k];
                    assert(rv[w] == self.elements@[k]);
                };
            }
            let mut j: usize = lo;
            while j < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),

                    n as int == self.elements.spec_seq().len(),
                    lo <= j, j <= n,
                    new_vec@.len() == (j + 1) as int,
                    rv.len() == new_vec@.len(),
                    forall|k: int| #![trigger rv[k]]
                        0 <= k < rv.len() ==> rv[k] == new_vec@[k]@,
                    forall|k: int| #![trigger rv[k]]
                        0 <= k < rv.len() ==> (
                            self@.contains(rv[k]) || rv[k] == x_view
                        ),
                    // All elements from [0..lo] and x_view are covered.
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < lo ==> rv.contains(self.elements@[k]),
                    rv.contains(x_view),
                    // Elements from [lo..j] are also covered.
                    forall|k: int| #![trigger self.elements@[k]]
                        lo <= k < j ==> rv.contains(self.elements@[k]),
                decreases n - j,
            {
                let elem = self.elements.nth(j);
                let c = elem.clone();
                let ghost old_rv = rv;
                let ghost old_rv_len = rv.len() as int;
                proof {
                    lemma_cloned_view_eq(*elem, c);
                    assert(self.elements@.contains(elem@));
                    rv = rv.push(elem@);
                }
                new_vec.push(c);
                proof {
                    // Coverage for [lo..j+1].
                    assert forall|k: int|
                        #![trigger self.elements@[k]]
                        lo <= k <= j && true
                        implies rv.contains(self.elements@[k]) by {
                        if k < j as int {
                            assert(old_rv.contains(self.elements@[k]));
                            let w = choose|w: int| 0 <= w < old_rv.len()
                                && old_rv[w] == self.elements@[k];
                            assert(rv[w] == self.elements@[k]);
                        } else {
                            assert(rv[old_rv_len] == elem@);
                        }
                    };
                    // Coverage for [0..lo] preserved after push.
                    assert forall|k: int|
                        #![trigger self.elements@[k]]
                        0 <= k < lo
                        implies rv.contains(self.elements@[k]) by {
                        assert(old_rv.contains(self.elements@[k]));
                        let w = choose|w: int| 0 <= w < old_rv.len()
                            && old_rv[w] == self.elements@[k];
                        assert(rv[w] == self.elements@[k]);
                    };
                    // x_view preserved.
                    assert(old_rv.contains(x_view));
                    let w = choose|w: int| 0 <= w < old_rv.len() && old_rv[w] == x_view;
                    assert(rv[w] == x_view);
                }
                j += 1;
            }
            // Assert connection before from_vec consumes new_vec.
            proof {
                assert forall|k: int| 0 <= k < rv.len()
                    implies rv[k] == new_vec@.map_values(|t: T| t@)[k] by {
                    assert(new_vec@.map_values(|t: T| t@)[k] == new_vec@[k]@);
                };
                assert(rv =~= new_vec@.map_values(|t: T| t@));
                // new_vec@.len() == n + 1 (from second loop exit with j == n).
                // n as nat == self@.len() (from unique_seq_to_set + wf).
                // self@.len() + 1 < usize::MAX (from requires).
                self.elements@.unique_seq_to_set();
                assert(n as nat == self@.len());
                assert(new_vec@.len() == (n + 1) as nat);
                assert(new_vec@.len() < usize::MAX);
            }
            let updated = AVLTreeSetStPer { elements: AVLTreeSeqStPerS::from_vec(new_vec) };
            proof {
                assert(updated.elements@ =~= rv);
                vstd::seq_lib::seq_to_set_is_finite(updated.elements@);
                // Prove updated@ == self@.insert(x_view).
                assert forall|v|
                    updated.elements@.to_set().contains(v)
                    <==> #[trigger] self@.insert(x_view).contains(v) by {
                    if updated.elements@.to_set().contains(v) {
                        assert(rv.to_set().contains(v));
                        assert(rv.contains(v));
                        let k = choose|k: int| 0 <= k < rv.len() && rv[k] == v;
                        assert(rv[k] == v);
                    }
                    if self@.insert(x_view).contains(v) {
                        if v == x_view {
                            assert(rv.contains(x_view));
                        } else {
                            assert(self@.contains(v));
                            assert(self.elements@.to_set().contains(v));
                            assert(self.elements@.contains(v));
                            let k = choose|k: int| 0 <= k < self.elements@.len()
                                && self.elements@[k] == v;
                            if k < lo as int {
                                assert(rv.contains(self.elements@[k]));
                            } else {
                                assert(rv.contains(self.elements@[k]));
                            }
                        }
                    }
                };
                assert(updated@ =~= self@.insert(x_view));
                // Prove no_duplicates via set cardinality.
                self.elements@.unique_seq_to_set();
                assert(old_seq_len == self@.len());
                assert(!self@.contains(x_view));
                assert(updated.elements@.len() == old_seq_len + 1);
                assert(updated@.len() == self@.len() + 1);
                assert(updated.elements@.to_set().len() == updated.elements@.len());
                updated.elements@.lemma_no_dup_set_cardinality();
            }
            updated
        }
    }

    

    impl<T: StT + Ord> Default for AVLTreeSetStPer<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord> PartialEqSpecImpl for AVLTreeSetStPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord> PartialEq for AVLTreeSetStPer<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof {
                assume(self.spec_avltreesetstper_wf());
                assume(other.spec_avltreesetstper_wf());
            }
            let equal = self.size() == other.size() && {
                let n = self.elements.length();
                let mut i: usize = 0;
                let mut all_found = true;
                while i < n
                    invariant
                        self.spec_avltreesetstper_wf(),
                        other.spec_avltreesetstper_wf(),
                        n == self.elements.spec_seq().len(),
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

    impl<T: StT + Ord> Eq for AVLTreeSetStPer<T> {}

    impl<T: StT + Ord> Clone for AVLTreeSetStPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = AVLTreeSetStPer { elements: self.elements.clone() };
            cloned
        }
    }

    } 

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StT + Ord> std::fmt::Debug for AVLTreeSetStPer<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord> Display for AVLTreeSetStPer<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{{{}}}",
                (0..self.size())
                    .map(|i| format!("{}", self.elements.nth(i)))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}
