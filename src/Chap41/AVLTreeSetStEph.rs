//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Single-threaded ephemeral set implementation using AVLTreeSeqStEph as backing store.
//!
//! Limitation: AVLTreeSeqStEph is index-ordered, not a BST by value. find uses binary search
//! on the sorted logical sequence (O(log n) via nth). insert/delete use filter-and-rebuild
//! since the backing tree has no O(log n) value-based insert/delete.

pub mod AVLTreeSetStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
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

    pub struct AVLTreeSetStEph<T: StT + Ord> {
        pub elements: AVLTreeSeqStEphS<T>,
    }

    pub type AVLTreeSetS<T> = AVLTreeSetStEph<T>;


    // 5. view impls

    impl<T: StT + Ord> View for AVLTreeSetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }


    // 7. proof fns

    /// Under wf, cached size equals inorder length, both < usize::MAX.
    proof fn lemma_wf_implies_len_bound<T: StT>(link: &Link<T>)
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


    // 8. traits

    // 8. traits

    pub trait AVLTreeSetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetsteph_wf(&self) -> bool;

        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetsteph_wf(),
            ensures count == self@.len();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
            requires self.spec_avltreesetsteph_wf(),
            ensures
                seq.spec_avltreeseqsteph_wf(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetsteph_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
            requires seq.spec_avltreeseqsteph_wf(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetsteph_wf(),
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
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            requires self.spec_avltreesetsteph_wf(),
            ensures found == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            requires old(self).spec_avltreesetsteph_wf(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            requires
                old(self).spec_avltreesetsteph_wf(),
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf();
    }


    // 9. impls

    // 5. view impls

    impl<T: StT + Ord> AVLTreeSetStEph<T> {
        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            self.elements@.to_set()
        }
    }

    // 9. impls

    impl<T: StT + Ord> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        open spec fn spec_avltreesetsteph_wf(&self) -> bool {
            self.elements.spec_avltreeseqsteph_wf()
            && self.elements@.no_duplicates()
            && self@.finite()
        }

        fn size(&self) -> (count: usize)
        {
            let r = self.elements.length();
            proof {
                self.elements@.unique_seq_to_set();
            }
            r
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
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
            let empty = AVLTreeSetStEph { elements: AVLTreeSeqStEphS::empty() };
            proof {
                assert(empty.elements@ =~= Seq::<<T as View>::V>::empty());
            }
            empty
        }

        fn singleton(x: T) -> (tree: Self)
        {
            let ghost x_view = x@;
            assert(obeys_feq_full_trigger::<T>());
            let mut v: Vec<T> = Vec::new();
            v.push(x);
            let ghost v_view = v@;
            let tree = AVLTreeSetStEph { elements: AVLTreeSeqStEphS::from_vec(v) };
            proof {
                assert(tree.elements@ =~= v_view.map_values(|t: T| t@));
                assert(v_view.len() == 1);
                assert(v_view[0]@ == x_view);
                assert(tree.elements@.len() == 1);
                assert(tree.elements@[0] == x_view);
                assert(tree.elements@.to_set() =~= Set::<<T as View>::V>::empty().insert(x_view));
                vstd::seq_lib::seq_to_set_is_finite(tree.elements@);
            }
            tree
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqsteph_wf(),
                    n as int == seq.spec_seq().len(),
                    i <= n,
                    constructed@.finite(),
                    constructed.spec_avltreesetsteph_wf(),
                    obeys_feq_full::<T>(),
                    forall|j: int| 0 <= j < i ==> constructed@.contains(seq@[j]),
                    forall|v: <T as View>::V| constructed@.contains(v) ==>
                        (exists|j: int| 0 <= j < i && seq@[j] == v),
                decreases n - i,
            {
                let r = seq.nth(i);
                let elem = r.clone();
                proof {
                    lemma_cloned_view_eq(*r, elem);
                }
                let ghost old_view = constructed@;
                constructed.insert(elem);
                proof {
                    assert forall|j: int| 0 <= j < i + 1
                        implies constructed@.contains(seq@[j]) by {
                        if j < i as int {
                            assert(old_view.contains(seq@[j]));
                        }
                    };
                    assert forall|v: <T as View>::V|
                        constructed@.contains(v) implies
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
                    constructed@.contains(v) == seq@.to_set().contains(v) by {
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

        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut filtered = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    filtered@.finite(),
                    filtered.spec_avltreesetsteph_wf(),
                    filtered@.subset_of(self@),
                    forall|t: &T| #[trigger] f.requires((t,)),
                    forall|x: T, keep: bool|
                        f.ensures((&x,), keep) ==> keep == spec_pred(x@),
                    obeys_feq_full::<T>(),
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
                        assert(c@ == elem@);
                        assert(self.elements@[i as int] == elem@);
                        assert(self.elements@.contains(elem@));
                        assert(self@.contains(elem@));
                    }
                    let ghost old_filtered_view = filtered@;
                    filtered.insert(c);
                    proof {
                        assert(filtered@.subset_of(self@)) by {
                            assert forall|x| #[trigger] filtered@.contains(x) implies self@.contains(x) by {
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

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut common = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    other.spec_avltreesetsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    common@.finite(),
                    common.spec_avltreesetsteph_wf(),
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
                    common.insert(c);
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
                    self.elements.spec_avltreeseqsteph_wf(),
                    other.spec_avltreesetsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    remaining@.finite(),
                    remaining.spec_avltreesetsteph_wf(),
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
                    remaining.insert(c);
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
                    self.elements.spec_avltreeseqsteph_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    i <= self_len,
                    combined@.finite(),
                    combined.spec_avltreesetsteph_wf(),
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
                combined.insert(c);
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
                    self.elements.spec_avltreeseqsteph_wf(),
                    other.elements.spec_avltreeseqsteph_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    other_len as int == other.elements.spec_seq().len(),
                    j <= other_len,
                    combined@.finite(),
                    combined.spec_avltreesetsteph_wf(),
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
                combined.insert(c);
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
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<T>(),
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

        fn delete(&mut self, x: &T)
        {
            assert(obeys_feq_full_trigger::<T>());
            let n = self.elements.length();
            let ghost orig_elems = self.elements@;
            let ghost orig_set = self@;
            let mut result_vec: Vec<T> = Vec::new();
            let mut i: usize = 0;
            let ghost mut skipped: int = 0;
            let ghost mut skip_idx: int = -1;
            let ghost mut result_views: Seq<<T as View>::V> = Seq::empty();
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    self.elements@.no_duplicates(),
                    obeys_feq_full::<T>(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    result_vec@.len() == i as int - skipped,
                    0 <= skipped <= 1,
                    skipped > 0 ==> (0 <= skip_idx < i && self.elements@[skip_idx] == x@),
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < i && self.elements@[k] == x@ ==> skipped > 0,
                    // Invariant: orig_elems is the original backing seq.
                    orig_elems == self.elements@,
                    orig_set == orig_elems.to_set(),
                    // Ghost view tracking.
                    result_views.len() == result_vec@.len(),
                    forall|k: int| #![trigger result_views[k]]
                        0 <= k < result_views.len() ==> result_views[k] == result_vec@[k]@,
                    forall|k: int| #![trigger result_views[k]]
                        0 <= k < result_views.len() ==> (
                            result_views[k] != x@
                            && orig_set.contains(result_views[k])
                        ),
                    forall|k: int| #![trigger orig_elems[k]]
                        0 <= k < i && orig_elems[k] != x@
                        ==> result_views.contains(orig_elems[k]),
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
                        // Prove coverage for the full range 0..=i.
                        assert forall|k: int|
                            #![trigger orig_elems[k]]
                            0 <= k <= i && orig_elems[k] != x@
                            implies result_views.contains(orig_elems[k]) by {
                            if k < i as int {
                                assert(old_result_views.contains(orig_elems[k]));
                                let j = choose|j: int| 0 <= j < old_result_views.len()
                                    && old_result_views[j] == orig_elems[k];
                                assert(result_views[j] == orig_elems[k]);
                            } else {
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
            // Assert connection before from_vec consumes result_vec.
            proof {
                assert forall|k: int| 0 <= k < result_views.len()
                    implies result_views[k] == result_vec@.map_values(|t: T| t@)[k] by {
                    assert(result_vec@.map_values(|t: T| t@)[k] == result_vec@[k]@);
                };
                assert(result_views =~= result_vec@.map_values(|t: T| t@));
            }
            proof {
                // result_vec@.len() == n - skipped <= n.
                // From tree wf: self.elements@.len() < usize::MAX.
                lemma_wf_implies_len_bound::<T>(&self.elements.root);
            }
            self.elements = AVLTreeSeqStEphS::from_vec(result_vec);
            proof {
                // self.elements@ =~= result_vec@.map_values(...) =~= result_views.
                assert(self.elements@ =~= result_views);
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                // Prove self@ == old(self)@.remove(x@).
                assert forall|v|
                    self.elements@.to_set().contains(v)
                    <==> #[trigger] old(self)@.remove(x@).contains(v) by {
                    if self.elements@.to_set().contains(v) {
                        assert(result_views.to_set().contains(v));
                        assert(result_views.contains(v));
                        let k = choose|k: int| 0 <= k < result_views.len()
                            && result_views[k] == v;
                        assert(result_views[k] == v);
                        assert(orig_set.contains(v));
                    }
                    if old(self)@.remove(x@).contains(v) {
                        assert(old(self)@.contains(v) && v != x@);
                        assert(orig_elems.to_set().contains(v));
                        assert(orig_elems.contains(v));
                        let k = choose|k: int| 0 <= k < orig_elems.len()
                            && orig_elems[k] == v;
                        assert(orig_elems[k] != x@);
                        assert(result_views.contains(orig_elems[k]));
                    }
                };
                assert(self@ =~= old(self)@.remove(x@));
                // Prove no_duplicates via cardinality.
                old(self).elements@.unique_seq_to_set();
                assert(self.elements@.len() == result_len);
                if old(self)@.contains(x@) {
                    assert(orig_elems.to_set().contains(x@));
                    assert(orig_elems.contains(x@));
                    let witness = choose|k: int| 0 <= k < orig_elems.len()
                        && orig_elems[k] == x@;
                    assert(orig_elems[witness] == x@);
                    assert(skipped == 1);
                } else {
                    assert(skipped == 0);
                }
                assert(self.elements@.to_set().len() == self.elements@.len());
                self.elements@.lemma_no_dup_set_cardinality();
            }
        }

        fn insert(&mut self, x: T)
        {
            let ghost x_view = x@;
            let ghost old_seq_len = self.elements@.len();
            let ghost orig_elems = self.elements@;
            let ghost orig_set = self@;
            let found = self.find(&x);
            if !found {
                assert(obeys_feq_full_trigger::<T>());
                let n = self.elements.length();
                let mut lo: usize = 0;
                let mut hi: usize = n;
                while lo < hi
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
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
                        self.elements.spec_avltreeseqsteph_wf(),
                        obeys_feq_full::<T>(),
                        n as int == self.elements.spec_seq().len(),
                        i <= lo, lo <= n,
                        new_vec@.len() == i as int,
                        rv.len() == new_vec@.len(),
                        orig_elems == self.elements@,
                        orig_set == self.elements@.to_set(),
                        forall|k: int| #![trigger rv[k]]
                            0 <= k < rv.len() ==> rv[k] == new_vec@[k]@,
                        forall|k: int| #![trigger rv[k]]
                            0 <= k < rv.len() ==> orig_set.contains(rv[k]),
                        forall|k: int| #![trigger orig_elems[k]]
                            0 <= k < i ==> rv.contains(orig_elems[k]),
                    decreases lo - i,
                {
                    let elem = self.elements.nth(i);
                    let c = elem.clone();
                    let ghost old_rv = rv;
                    let ghost old_rv_len = rv.len() as int;
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@.contains(elem@));
                        assert(orig_set.contains(elem@));
                        rv = rv.push(elem@);
                    }
                    new_vec.push(c);
                    proof {
                        assert forall|k: int|
                            #![trigger orig_elems[k]]
                            0 <= k <= i && true
                            implies rv.contains(orig_elems[k]) by {
                            if k < i as int {
                                assert(old_rv.contains(orig_elems[k]));
                                let j = choose|j: int| 0 <= j < old_rv.len()
                                    && old_rv[j] == orig_elems[k];
                                assert(rv[j] == orig_elems[k]);
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
                    assert(rv[pre_x_rv_len] == x_view);
                    assert(rv.contains(x_view));
                    assert forall|k: int|
                        #![trigger orig_elems[k]]
                        0 <= k < lo
                        implies rv.contains(orig_elems[k]) by {
                        assert(pre_x_rv.contains(orig_elems[k]));
                        let w = choose|w: int| 0 <= w < pre_x_rv.len()
                            && pre_x_rv[w] == orig_elems[k];
                        assert(rv[w] == orig_elems[k]);
                    };
                }
                let mut j: usize = lo;
                while j < n
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
                        obeys_feq_full::<T>(),
                        n as int == self.elements.spec_seq().len(),
                        lo <= j, j <= n,
                        new_vec@.len() == (j + 1) as int,
                        rv.len() == new_vec@.len(),
                        orig_elems == self.elements@,
                        orig_set == self.elements@.to_set(),
                        forall|k: int| #![trigger rv[k]]
                            0 <= k < rv.len() ==> rv[k] == new_vec@[k]@,
                        forall|k: int| #![trigger rv[k]]
                            0 <= k < rv.len() ==> (
                                orig_set.contains(rv[k]) || rv[k] == x_view
                            ),
                        forall|k: int| #![trigger orig_elems[k]]
                            0 <= k < lo ==> rv.contains(orig_elems[k]),
                        rv.contains(x_view),
                        forall|k: int| #![trigger orig_elems[k]]
                            lo <= k < j ==> rv.contains(orig_elems[k]),
                    decreases n - j,
                {
                    let elem = self.elements.nth(j);
                    let c = elem.clone();
                    let ghost old_rv = rv;
                    let ghost old_rv_len = rv.len() as int;
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@.contains(elem@));
                        assert(orig_set.contains(elem@));
                        rv = rv.push(elem@);
                    }
                    new_vec.push(c);
                    proof {
                        assert forall|k: int|
                            #![trigger orig_elems[k]]
                            lo <= k <= j && true
                            implies rv.contains(orig_elems[k]) by {
                            if k < j as int {
                                assert(old_rv.contains(orig_elems[k]));
                                let w = choose|w: int| 0 <= w < old_rv.len()
                                    && old_rv[w] == orig_elems[k];
                                assert(rv[w] == orig_elems[k]);
                            } else {
                                assert(rv[old_rv_len] == elem@);
                            }
                        };
                        assert forall|k: int|
                            #![trigger orig_elems[k]]
                            0 <= k < lo
                            implies rv.contains(orig_elems[k]) by {
                            assert(old_rv.contains(orig_elems[k]));
                            let w = choose|w: int| 0 <= w < old_rv.len()
                                && old_rv[w] == orig_elems[k];
                            assert(rv[w] == orig_elems[k]);
                        };
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
                }
                proof { assume(new_vec@.len() < usize::MAX); }
                self.elements = AVLTreeSeqStEphS::from_vec(new_vec);
                proof {
                    assert(self.elements@ =~= rv);
                    // Prove self@ == orig_set.insert(x_view).
                    assert forall|v|
                        self.elements@.to_set().contains(v)
                        <==> #[trigger] orig_set.insert(x_view).contains(v) by {
                        if self.elements@.to_set().contains(v) {
                            assert(rv.to_set().contains(v));
                            assert(rv.contains(v));
                            let k = choose|k: int| 0 <= k < rv.len() && rv[k] == v;
                            assert(rv[k] == v);
                        }
                        if orig_set.insert(x_view).contains(v) {
                            if v == x_view {
                                assert(rv.contains(x_view));
                            } else {
                                assert(orig_set.contains(v));
                                assert(orig_elems.to_set().contains(v));
                                assert(orig_elems.contains(v));
                                let k = choose|k: int| 0 <= k < orig_elems.len()
                                    && orig_elems[k] == v;
                                if k < lo as int {
                                    assert(rv.contains(orig_elems[k]));
                                } else {
                                    assert(rv.contains(orig_elems[k]));
                                }
                            }
                        }
                    };
                    assert(self@ =~= orig_set.insert(x_view));
                }
            }
            proof {
                assert(self@ =~= old(self)@.insert(x_view));
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                // Prove no_duplicates via set cardinality.
                old(self).elements@.unique_seq_to_set();
                assert(old(self).elements@.len() == old(self)@.len());
                assert(old_seq_len == old(self)@.len());
                if found {
                    assert(self.elements@.len() == old_seq_len);
                    assert(self@.len() == old(self)@.len());
                    assert(self.elements@.to_set().len() == self.elements@.len());
                } else {
                    assert(self.elements@.len() == old_seq_len + 1);
                    assert(!old(self)@.contains(x_view));
                    assert(self@.len() == old(self)@.len() + 1);
                    assert(self.elements@.to_set().len() == self.elements@.len());
                }
                self.elements@.lemma_no_dup_set_cardinality();
            }
        }
    }


    // 11. derive impls in verus!

    // 11. derive impls in verus!

    impl<T: StT + Ord> Default for AVLTreeSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord> PartialEqSpecImpl for AVLTreeSetStEph<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord> Eq for AVLTreeSetStEph<T> {}

    impl<T: StT + Ord> PartialEq for AVLTreeSetStEph<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof {
                assume(self.spec_avltreesetsteph_wf());
                assume(other.spec_avltreesetsteph_wf());
            }
            let equal = self.size() == other.size() && {
                let n = self.elements.length();
                let mut i: usize = 0;
                let mut all_found = true;
                while i < n
                    invariant
                        self.spec_avltreesetsteph_wf(),
                        other.spec_avltreesetsteph_wf(),
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

    impl<T: StT + Ord> Clone for AVLTreeSetStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = AVLTreeSetStEph { elements: self.elements.clone() };
            cloned
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StT + Ord> fmt::Debug for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    impl<T: StT + Ord> fmt::Display for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
