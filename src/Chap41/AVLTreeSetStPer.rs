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
                constructed.spec_avltreesetstper_wf();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
            requires self.spec_avltreesetstper_wf(),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetstper_wf();
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
            requires self.spec_avltreesetstper_wf(),
            ensures
                updated@ == self@.insert(x@),
                updated.spec_avltreesetstper_wf();
    }

    // 9. impls

    impl<T: StT + Ord> AVLTreeSetStPerTrait<T> for AVLTreeSetStPer<T> {
        open spec fn spec_avltreesetstper_wf(&self) -> bool {
            self.elements.spec_avltreeseqstper_wf()
            && self@.finite()
        }

        fn size(&self) -> (count: usize)
        {
            let r = self.elements.length();
            proof {
                assume(r == self@.len());
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
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
            let empty = AVLTreeSetStPer { elements: AVLTreeSeqStPerS::empty() };
            assert(empty.elements@ =~= Seq::<<T as View>::V>::empty());
            assert(empty@ =~= Set::<<T as View>::V>::empty());
            empty
        }

        fn singleton(x: T) -> (tree: Self)
        {
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
                vstd::seq_lib::seq_to_set_is_finite(tree.elements@);
            }
            tree
        }

        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
        {
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqstper_wf(),
                    n as int == seq.spec_seq().len(),
                    i <= n,
                    constructed@.finite(),
                    constructed.spec_avltreesetstper_wf(),
                decreases n - i,
            {
                let elem = seq.nth(i).clone();
                constructed = constructed.insert(elem);
                i += 1;
            }
            constructed
        }

        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut filtered = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    filtered@.finite(),
                    filtered.spec_avltreesetstper_wf(),
                    filtered@.subset_of(self@),
                    obeys_feq_full::<T>(),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                proof { assume(f.requires((&*elem,))); }
                if f(elem) {
                    let c = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@[i as int] == elem@);
                        assert(self.elements@.contains(elem@));
                        assert(self@.contains(elem@));
                    }
                    let ghost old_filtered_view = filtered@;
                    filtered = filtered.insert(c);
                    proof {
                        assert(filtered@.subset_of(self@)) by {
                            assert forall|x| #[trigger] filtered@.contains(x)
                                implies self@.contains(x) by {
                                if !old_filtered_view.contains(x) {
                                    assert(x == elem@);
                                }
                            };
                        };
                    }
                }
                i += 1;
            }
            filtered
        }

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut common = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    other.spec_avltreesetstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    common@.finite(),
                    common.spec_avltreesetstper_wf(),
                    obeys_feq_full::<T>(),
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
                    }
                    let ghost old_common = common@;
                    common = common.insert(c);
                    proof {
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

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut remaining = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    other.spec_avltreesetstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    remaining@.finite(),
                    remaining.spec_avltreesetstper_wf(),
                    obeys_feq_full::<T>(),
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
                    }
                    let ghost old_remaining = remaining@;
                    remaining = remaining.insert(c);
                    proof {
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

        fn union(&self, other: &Self) -> (combined: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut combined = Self::empty();
            let self_len = self.elements.length();
            let mut i: usize = 0;
            while i < self_len
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    i <= self_len,
                    combined@.finite(),
                    combined.spec_avltreesetstper_wf(),
                    obeys_feq_full::<T>(),
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
                i += 1;
            }
            let other_len = other.elements.length();
            let mut j: usize = 0;
            while j < other_len
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    other.elements.spec_avltreeseqstper_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    other_len as int == other.elements.spec_seq().len(),
                    j <= other_len,
                    combined@.finite(),
                    combined.spec_avltreesetstper_wf(),
                    obeys_feq_full::<T>(),
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

        fn find(&self, x: &T) -> (found: B)
        {
            assert(obeys_feq_full_trigger::<T>());
            let n = self.elements.length();
            let mut lo: usize = 0;
            let mut hi: usize = n;
            while lo < hi
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    obeys_feq_full::<T>(),
                    n as int == self.elements.spec_seq().len(),
                    lo <= hi, hi <= n,
                decreases hi - lo,
            {
                let mid = lo + (hi - lo) / 2;
                let elem = self.elements.nth(mid);
                if feq(elem, x) {
                    assert(self.elements@[mid as int] == x@);
                    assert(self.elements@.contains(x@));
                    return true;
                }
                if *elem < *x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            proof { assume(!self@.contains(x@)); }
            false
        }

        fn delete(&self, x: &T) -> (updated: Self)
        {
            let n = self.elements.length();
            let mut result_vec: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if *elem != *x {
                    result_vec.push(elem.clone());
                }
                i += 1;
            }
            let updated = AVLTreeSetStPer { elements: AVLTreeSeqStPerS::from_vec(result_vec) };
            proof {
                assume(updated@ == self@.remove(x@));
                vstd::seq_lib::seq_to_set_is_finite(updated.elements@);
            }
            updated
        }

        fn insert(&self, x: T) -> (updated: Self)
        {
            let ghost x_view = x@;
            if self.find(&x) {
                let updated = Self { elements: self.elements.clone() };
                proof {
                    // find(&x) == true implies self@.contains(x@).
                    // clone ensures updated.elements@ == self.elements@, so updated@ == self@.
                    // Since x@ is already in self@, self@.insert(x_view) == self@ (set idempotence).
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
            let mut i: usize = 0;
            while i < lo
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= lo, lo <= n,
                decreases lo - i,
            {
                new_vec.push(self.elements.nth(i).clone());
                i += 1;
            }
            new_vec.push(x);
            let mut j: usize = lo;
            while j < n
                invariant
                    self.elements.spec_avltreeseqstper_wf(),
                    n as int == self.elements.spec_seq().len(),
                    lo <= j, j <= n,
                decreases n - j,
            {
                new_vec.push(self.elements.nth(j).clone());
                j += 1;
            }
            let updated = AVLTreeSetStPer { elements: AVLTreeSeqStPerS::from_vec(new_vec) };
            proof {
                assume(updated@ == self@.insert(x_view));
                vstd::seq_lib::seq_to_set_is_finite(updated.elements@);
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
                        self.elements.spec_avltreeseqstper_wf(),
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
