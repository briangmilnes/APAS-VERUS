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
    // 6. spec fns
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

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

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

    // 6. spec fns

    /// In-order traversal returning actual values (Seq<T>), not views.
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

    // 7. proof fns

    /// Under wf, cached size equals inorder length, both < usize::MAX.
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
    pub proof fn lemma_inorder_values_maps_to_views<T: StT>(link: Link<T>)
        ensures spec_inorder_values(link).map_values(|t: T| t@) =~= spec_inorder(link),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_inorder_values_maps_to_views::<T>(node.left);
                lemma_inorder_values_maps_to_views::<T>(node.right);
                // Each part maps correctly; extensional equality closes it.
                let lv = spec_inorder_values(node.left);
                let rv_right = spec_inorder_values(node.right);
                let mid: Seq<T> = seq![node.value];
                let full = lv + mid + rv_right;
                // Verus needs help seeing the map distributes over concat.
                assert(full.map_values(|t: T| t@) =~=
                    lv.map_values(|t: T| t@) + mid.map_values(|t: T| t@) + rv_right.map_values(|t: T| t@));
            }
        }
    }

    /// An empty AVL tree set has its elements trivially sorted.
    proof fn lemma_empty_set_is_sorted<T: StT + Ord + TotalOrder>(set: &AVLTreeSetStEph<T>)
        requires
            set@ =~= Set::<<T as View>::V>::empty(),
            set.spec_avltreesetsteph_wf(),
        ensures
            set.spec_elements_sorted(),
    {
        // From wf: set.elements@.no_duplicates() holds.
        // unique_seq_to_set() gives: set.elements@.len() == set.elements@.to_set().len()
        // set.elements@.to_set() == set@ =~= Set::empty() ==> len == 0
        // Hence set.elements@.len() == 0, so set.elements@ =~= Seq::empty().
        set.elements@.unique_seq_to_set();
        assert(set.elements@.len() == 0);
        lemma_inorder_values_maps_to_views::<T>(set.elements.root);
        let vals = spec_inorder_values::<T>(set.elements.root);
        // vals.map_values(|t: T| t@) =~= set.elements@ which has len 0.
        // map_values preserves length, so vals.len() == 0.
        // spec_seq_sorted(Seq::empty()) holds vacuously (no (i,j) pair to violate it).
        assert(vals.len() == 0);
        assert(set.spec_elements_sorted());
    }

    /// Appending an element >= all existing preserves sortedness.
    proof fn lemma_push_sorted<T: TotalOrder>(s: Seq<T>, v: T)
        requires
            spec_seq_sorted(s),
            s.len() > 0 ==> TotalOrder::le(s.last(), v),
        ensures
            spec_seq_sorted(s.push(v)),
    {
        let new_s = s.push(v);
        assert forall|i: int, j: int| 0 <= i < j < new_s.len()
            implies #[trigger] TotalOrder::le(new_s[i], new_s[j]) by {
            if j < s.len() as int {
                // Both in original sequence.
                assert(new_s[i] == s[i]);
                assert(new_s[j] == s[j]);
            } else {
                // j is the new element.
                assert(new_s[j] == v);
                assert(new_s[i] == s[i]);
                if s.len() == 0 {
                    // Impossible since i < j and j < new_s.len() == 1.
                } else if i == s.len() as int - 1 {
                    // s[i] is the last element, directly <= v.
                } else {
                    // s[i] <= s.last() by sorted, s.last() <= v by precondition.
                    assert(TotalOrder::le(s[i], s[s.len() - 1]));
                    T::transitive(s[i], s[s.len() - 1], v);
                }
            }
        };
    }

    /// Subsequence of a sorted sequence is sorted.
    proof fn lemma_subseq_sorted<T: TotalOrder>(s: Seq<T>, lo: int, hi: int)
        requires
            spec_seq_sorted(s),
            0 <= lo <= hi <= s.len(),
        ensures
            spec_seq_sorted(s.subrange(lo, hi)),
    {
        let sub = s.subrange(lo, hi);
        assert forall|i: int, j: int| 0 <= i < j < sub.len()
            implies #[trigger] TotalOrder::le(sub[i], sub[j]) by {
            assert(sub[i] == s[lo + i]);
            assert(sub[j] == s[lo + j]);
        };
    }

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
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
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
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf();
        /// Iterative alternative to `find`.
        fn find_iter(&self, x: &T) -> (found: B)
            requires self.spec_avltreesetsteph_wf(),
            ensures found == self@.contains(x@);
        /// Iterative alternative to `insert`.
        fn insert_iter(&mut self, x: T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf();
        /// Iterative alternative to `delete`.
        fn delete_iter(&mut self, x: &T)
            requires old(self).spec_avltreesetsteph_wf(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetsteph_wf();
        /// Iterative alternative to `filter`.
        fn filter_iter<F: PredSt<T>>(
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
        /// Iterative alternative to `intersection`.
        fn intersection_iter(&self, other: &Self) -> (common: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetsteph_wf();
        /// Iterative alternative to `union`.
        fn union_iter(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                other.spec_avltreesetsteph_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetsteph_wf();
        /// Iterative alternative to `difference`.
        fn difference_iter(&self, other: &Self) -> (remaining: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
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
        fn insert_sorted(&mut self, x: T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                old(self).spec_elements_sorted(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted();
        /// Delete preserving sortedness.
        fn delete_sorted(&mut self, x: &T)
            requires
                old(self).spec_avltreesetsteph_wf(),
                old(self).spec_elements_sorted(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted();
        /// Filter preserving sortedness.
        fn filter_sorted<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
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
        fn intersection_sorted(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
                other.spec_avltreesetsteph_wf(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetsteph_wf(),
                common.spec_elements_sorted();
        /// Difference preserving sortedness.
        fn difference_sorted(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
                other.spec_avltreesetsteph_wf(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetsteph_wf(),
                remaining.spec_elements_sorted();
        /// Union preserving sortedness; requires combined capacity bound.
        fn union_sorted(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                self.spec_elements_sorted(),
                other.spec_avltreesetsteph_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetsteph_wf(),
                combined.spec_elements_sorted();
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
                      assert(obeys_feq_full_trigger::<T>());
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
                    // Capacity: constructed@.len() <= i < n < usize::MAX (from tree wf).
                    lemma_wf_implies_len_bound::<T>(&seq.root);
                }
                let ghost old_view = constructed@;
                constructed.insert(elem);
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

        /// Iterative alternative to `filter`.
        fn filter_iter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
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
                    filtered@.len() <= i as nat,
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
                        // Capacity: filtered@.len() <= i < n < usize::MAX (from tree wf).
                        lemma_wf_implies_len_bound::<T>(&self.elements.root);
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

        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            self.filter_iter(f, Ghost(spec_pred))
        }

        /// Iterative alternative to `intersection`.
        fn intersection_iter(&self, other: &Self) -> (common: Self)
        {
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
                    common@.len() <= i as nat,
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
                        // Capacity: common@.len() <= i < n < usize::MAX (from tree wf).
                        lemma_wf_implies_len_bound::<T>(&self.elements.root);
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

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            self.intersection_iter(other)
        }

        /// Iterative alternative to `difference`.
        fn difference_iter(&self, other: &Self) -> (remaining: Self)
        {
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
                    remaining@.len() <= i as nat,
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
                        // Capacity: remaining@.len() <= i < n < usize::MAX (from tree wf).
                        lemma_wf_implies_len_bound::<T>(&self.elements.root);
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

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            self.difference_iter(other)
        }

        /// Iterative alternative to `union`.
        fn union_iter(&self, other: &Self) -> (combined: Self)
        {
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
                    combined@.len() <= i as nat,
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
                    // Capacity: combined@.len() <= i < self_len < usize::MAX (from tree wf).
                    lemma_wf_implies_len_bound::<T>(&self.elements.root);
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
            proof {
                self.elements@.unique_seq_to_set();
                other.elements@.unique_seq_to_set();
            }
            while j < other_len
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    other.elements.spec_avltreeseqsteph_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    other_len as int == other.elements.spec_seq().len(),
                    self_len as nat == self@.len(),
                    other_len as nat == other@.len(),
                    self@.len() + other@.len() < usize::MAX as nat,
                    j <= other_len,
                    combined@.finite(),
                    combined.spec_avltreesetsteph_wf(),
                    combined@.len() <= self_len as nat + j as nat,
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
                    // combined@.len() <= self_len + j < self_len + other_len < usize::MAX.
                    assert(combined@.len() + 1 < usize::MAX as nat);
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

        fn union(&self, other: &Self) -> (combined: Self)
        {
            self.union_iter(other)
        }

        /// Iterative alternative to `find`.
        #[verifier::loop_isolation(false)]
        fn find_iter(&self, x: &T) -> (found: B)
        {
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
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

        fn find(&self, x: &T) -> (found: B)
        {
            self.find_iter(x)
        }

        /// Iterative alternative to `delete`.
        #[verifier::loop_isolation(false)]
        fn delete_iter(&mut self, x: &T)
        {
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

        fn delete(&mut self, x: &T)
        {
            self.delete_iter(x)
        }

        /// Iterative alternative to `insert`.
        #[verifier::loop_isolation(false)]
        fn insert_iter(&mut self, x: T)
        {
            let ghost x_view = x@;
            let ghost old_seq_len = self.elements@.len();
            let ghost orig_elems = self.elements@;
            let ghost orig_set = self@;
            let found = self.find(&x);
            if !found {
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
                proof {
                    // Capacity bound from requires: old(self)@.len() + 1 < usize::MAX.
                    // self@.len() == self.elements@.len() (via unique_seq_to_set + no_duplicates).
                    // new_vec has n+1 elements. n+1 == self.elements@.len() + 1 < usize::MAX.
                    old(self).elements@.unique_seq_to_set();
                    assert(new_vec@.len() as nat == self.elements@.len() + 1);
                    assert(new_vec@.len() < usize::MAX);
                }
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
        fn insert(&mut self, x: T)
        {
            self.insert_iter(x)
        }
    }

    // 9. impls (TotalOrder-gated trait impl)

    impl<T: StT + Ord + TotalOrder> AVLTreeSetStEphTotalOrderTrait<T> for AVLTreeSetStEph<T> {
        /// The backing sequence is sorted under TotalOrder::le.
        open spec fn spec_elements_sorted(&self) -> bool {
            spec_seq_sorted(spec_inorder_values(self.elements.root))
        }

        /// The value-level backing sequence.
        open spec fn spec_values_seq(&self) -> Seq<T> {
            spec_inorder_values(self.elements.root)
        }

        /// Insert preserving sortedness.
        #[verifier::loop_isolation(false)]
        fn insert_sorted(&mut self, x: T)
        {
            let ghost x_view = x@;
            let ghost old_seq_len = self.elements@.len();
            let ghost orig_elems = self.elements@;
            let ghost orig_set = self@;
            let ghost orig_vals = spec_inorder_values::<T>(self.elements.root);
            let found = self.find(&x);
            if !found {
                let n = self.elements.length();
                proof {
                    lemma_inorder_values_maps_to_views::<T>(self.elements.root);
                }
                // Binary search using TotalOrder::cmp.
                let mut lo: usize = 0;
                let mut hi: usize = n;
                while lo < hi
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
                        n as int == self.elements.spec_seq().len(),
                        lo <= hi, hi <= n,
                        orig_elems == self.elements@,
                        orig_vals == spec_inorder_values::<T>(self.elements.root),
                        spec_seq_sorted(orig_vals),
                        orig_vals.len() == orig_elems.len(),
                        forall|k: int| #![trigger orig_vals[k]]
                            0 <= k < lo ==> (TotalOrder::le(orig_vals[k], x) && orig_vals[k] != x),
                        forall|k: int| #![trigger orig_vals[k]]
                            hi <= k < n ==> TotalOrder::le(x, orig_vals[k]),
                    decreases hi - lo,
                {
                    let mid = lo + (hi - lo) / 2;
                    let elem = self.elements.nth(mid);
                    let c = TotalOrder::cmp(elem, &x);
                    proof {
                        lemma_inorder_values_maps_to_views::<T>(self.elements.root);
                        assert(orig_vals[mid as int]@ == elem@);
                    }
                    match c {
                        core::cmp::Ordering::Less => {
                            proof {
                                assert forall|k: int| 0 <= k < mid + 1
                                    implies (#[trigger] TotalOrder::le(orig_vals[k], x) && orig_vals[k] != x) by {
                                    if k < lo as int {
                                    } else if k == mid as int {
                                    } else {
                                        assert(TotalOrder::le(orig_vals[k], orig_vals[mid as int]));
                                        T::transitive(orig_vals[k], orig_vals[mid as int], x);
                                        if orig_vals[k] == x {
                                            T::reflexive(x);
                                            T::antisymmetric(orig_vals[mid as int], x);
                                        }
                                    }
                                };
                            }
                            lo = mid + 1;
                        },
                        _ => {
                            proof {
                                let elem_val = orig_vals[mid as int];
                                match c {
                                    core::cmp::Ordering::Equal => {
                                        T::reflexive(x);
                                    },
                                    core::cmp::Ordering::Greater => {},
                                    _ => {},
                                }
                                assert(TotalOrder::le(x, elem_val));
                                assert forall|k: int| mid as int <= k < n as int
                                    implies #[trigger] TotalOrder::le(x, orig_vals[k]) by {
                                    if k == mid as int {
                                    } else if k >= hi as int {
                                    } else {
                                        assert(TotalOrder::le(orig_vals[mid as int], orig_vals[k]));
                                        T::transitive(x, orig_vals[mid as int], orig_vals[k]);
                                    }
                                };
                            }
                            hi = mid;
                        },
                    }
                }
                // lo == hi == insertion point.
                // Build new_vec: orig[0..lo] ++ [x] ++ orig[lo..n].
                let mut new_vec: Vec<T> = Vec::new();
                let ghost mut rv: Seq<<T as View>::V> = Seq::empty();
                let mut i: usize = 0;
                while i < lo
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
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
                        // Positional: rv[k] == orig_elems[k] for k < i.
                        forall|k: int| #![trigger rv[k]]
                            0 <= k < rv.len() ==> rv[k] == orig_elems[k],
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
                    i += 1;
                }
                // Push x.
                let ghost pre_x_rv = rv;
                let ghost pre_x_rv_len = rv.len() as int;
                proof { rv = rv.push(x_view); }
                new_vec.push(x);
                proof {
                    assert(rv[pre_x_rv_len] == x_view);
                }
                // Copy elements from insertion point onward.
                let mut j: usize = lo;
                while j < n
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
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
                        // Positional tracking.
                        forall|k: int| #![trigger rv[k]]
                            0 <= k < lo ==> rv[k] == orig_elems[k],
                        rv[lo as int] == x_view,
                        forall|k: int| #![trigger rv[lo as int + 1 + k]]
                            0 <= k < j - lo ==> rv[lo as int + 1 + k] == orig_elems[lo as int + k],
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
                        // Preserve positional invariants for prior entries.
                        assert forall|k: int| 0 <= k < lo as int
                            implies #[trigger] rv[k] == orig_elems[k] by {
                            assert(old_rv[k] == orig_elems[k]);
                        };
                        assert forall|k: int| 0 <= k < (j + 1) - lo
                            implies #[trigger] rv[lo as int + 1 + k] == orig_elems[lo as int + k] by {
                            if k < (j as int - lo as int) {
                                assert(old_rv[lo as int + 1 + k] == orig_elems[lo as int + k]);
                            } else {
                                assert(rv[old_rv_len] == elem@);
                                assert(elem@ == self.elements@[j as int]);
                            }
                        };
                    }
                    j += 1;
                }
                // Build tree from new_vec.
                proof {
                    assert forall|k: int| 0 <= k < rv.len()
                        implies rv[k] == new_vec@.map_values(|t: T| t@)[k] by {
                        assert(new_vec@.map_values(|t: T| t@)[k] == new_vec@[k]@);
                    };
                    assert(rv =~= new_vec@.map_values(|t: T| t@));
                }
                proof {
                    // Capacity bound from requires: old(self)@.len() + 1 < usize::MAX.
                    old(self).elements@.unique_seq_to_set();
                    assert(new_vec@.len() as nat == self.elements@.len() + 1);
                    assert(new_vec@.len() < usize::MAX);
                }
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
                        }
                        if orig_set.insert(x_view).contains(v) {
                            if v == x_view {
                                assert(rv[lo as int] == x_view);
                            } else {
                                assert(orig_set.contains(v));
                                assert(orig_elems.to_set().contains(v));
                                assert(orig_elems.contains(v));
                                let k = choose|k: int| 0 <= k < orig_elems.len()
                                    && orig_elems[k] == v;
                                if k < lo as int {
                                    assert(rv[k] == orig_elems[k]);
                                } else {
                                    assert(rv[lo as int + 1 + (k - lo as int)] == orig_elems[k]);
                                }
                            }
                        }
                    };
                    assert(self@ =~= orig_set.insert(x_view));
                    // Prove sorted via positional + feq bridge.
                    lemma_inorder_values_maps_to_views::<T>(self.elements.root);
                    let new_vals = spec_inorder_values::<T>(self.elements.root);
                    // Key bridge: orig_vals.map_values(|t| t@) =~= orig_elems.
                    assert(orig_vals.map_values(|t: T| t@) =~= orig_elems);
                    // Establish: orig_vals.map_values(|t| t@) =~= orig_elems.
                    // Already known from the earlier call before the binary search loop.
                    // Positional rv facts give us rv[k] in terms of orig_elems[...].
                    // Chain: new_vals[k]@ == rv[k] == orig_elems[m] == orig_vals[m]@.
                    // Under feq: new_vals[k] == orig_vals[m].
                    assert forall|ii: int, jj: int| 0 <= ii < jj < new_vals.len()
                        implies #[trigger] TotalOrder::le(new_vals[ii], new_vals[jj]) by {
                        // Establish new_vals[k]@ == rv[k].
                        assert(new_vals[ii]@ == rv[ii]);
                        assert(new_vals[jj]@ == rv[jj]);
                        if ii < lo as int && jj < lo as int {
                            // Both before insertion point.
                            assert(rv[ii] == orig_elems[ii]);
                            assert(rv[jj] == orig_elems[jj]);
                            assert(orig_vals[ii]@ == orig_elems[ii]);
                            assert(orig_vals[jj]@ == orig_elems[jj]);
                            assert(new_vals[ii] == orig_vals[ii]);
                            assert(new_vals[jj] == orig_vals[jj]);
                            assert(TotalOrder::le(orig_vals[ii], orig_vals[jj]));
                        } else if ii < lo as int && jj == lo as int {
                            // ii before, jj is x.
                            assert(rv[ii] == orig_elems[ii]);
                            assert(rv[jj] == x_view);
                            assert(orig_vals[ii]@ == orig_elems[ii]);
                            assert(new_vals[ii] == orig_vals[ii]);
                            assert(new_vals[jj] == x);
                            assert(TotalOrder::le(orig_vals[ii], x));
                        } else if ii < lo as int && jj > lo as int {
                            // ii before, jj after.
                            let mj = jj - 1;
                            let kj = mj - lo as int;
                            assert(rv[ii] == orig_elems[ii]);
                            // Trigger the forall with k = kj.
                            assert(rv[lo as int + 1 + kj] == orig_elems[lo as int + kj]);
                            assert(rv[jj] == orig_elems[mj]);
                            assert(orig_vals[ii]@ == orig_elems[ii]);
                            assert(orig_vals[mj]@ == orig_elems[mj]);
                            assert(new_vals[ii] == orig_vals[ii]);
                            assert(new_vals[jj] == orig_vals[mj]);
                            assert(ii < mj);
                            assert(TotalOrder::le(orig_vals[ii], orig_vals[mj]));
                        } else if ii == lo as int && jj > lo as int {
                            // ii is x, jj after.
                            let mj = jj - 1;
                            let kj = mj - lo as int;
                            assert(rv[ii] == x_view);
                            assert(rv[lo as int + 1 + kj] == orig_elems[lo as int + kj]);
                            assert(rv[jj] == orig_elems[mj]);
                            assert(orig_vals[mj]@ == orig_elems[mj]);
                            assert(new_vals[ii] == x);
                            assert(new_vals[jj] == orig_vals[mj]);
                            assert(mj >= lo as int);
                            assert(TotalOrder::le(x, orig_vals[mj]));
                        } else {
                            // Both after insertion point: ii > lo, jj > ii > lo.
                            let mi = ii - 1;
                            let mj = jj - 1;
                            let ki = mi - lo as int;
                            let kj = mj - lo as int;
                            assert(rv[lo as int + 1 + ki] == orig_elems[lo as int + ki]);
                            assert(rv[lo as int + 1 + kj] == orig_elems[lo as int + kj]);
                            assert(rv[ii] == orig_elems[mi]);
                            assert(rv[jj] == orig_elems[mj]);
                            assert(orig_vals[mi]@ == orig_elems[mi]);
                            assert(orig_vals[mj]@ == orig_elems[mj]);
                            assert(new_vals[ii] == orig_vals[mi]);
                            assert(new_vals[jj] == orig_vals[mj]);
                            assert(mi < mj);
                            assert(TotalOrder::le(orig_vals[mi], orig_vals[mj]));
                        }
                    };
                }
            }
            proof {
                assert(self@ =~= old(self)@.insert(x_view));
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
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

        /// Delete preserving sortedness.
        #[verifier::loop_isolation(false)]
        fn delete_sorted(&mut self, x: &T)
        {
            let n = self.elements.length();
            let ghost orig_elems = self.elements@;
            let ghost orig_set = self@;
            let ghost orig_vals = spec_inorder_values::<T>(self.elements.root);
            proof { lemma_inorder_values_maps_to_views::<T>(self.elements.root); }
            let mut result_vec: Vec<T> = Vec::new();
            let mut i: usize = 0;
            let ghost mut skipped: int = 0;
            let ghost mut skip_idx: int = -1;
            let ghost mut result_views: Seq<<T as View>::V> = Seq::empty();
            // Ghost: tracks which orig_vals index each result entry came from.
            let ghost mut orig_idx_map: Seq<int> = Seq::empty();
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    self.elements@.no_duplicates(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    result_vec@.len() == i as int - skipped,
                    0 <= skipped <= 1,
                    skipped > 0 ==> (0 <= skip_idx < i && self.elements@[skip_idx] == x@),
                    forall|k: int| #![trigger self.elements@[k]]
                        0 <= k < i && self.elements@[k] == x@ ==> skipped > 0,
                    orig_elems == self.elements@,
                    orig_set == orig_elems.to_set(),
                    orig_vals == spec_inorder_values::<T>(self.elements.root),
                    spec_seq_sorted(orig_vals),
                    orig_vals.len() == orig_elems.len(),
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
                    // Index map: maps each result position to its orig_vals index.
                    orig_idx_map.len() == result_views.len(),
                    forall|k: int| #![trigger orig_idx_map[k]]
                        0 <= k < orig_idx_map.len() ==> (
                            0 <= orig_idx_map[k] < n
                            && result_views[k] == orig_elems[orig_idx_map[k]]
                        ),
                    // Strictly increasing index map.
                    forall|a: int, b: int| #![trigger orig_idx_map[a], orig_idx_map[b]]
                        0 <= a < b < orig_idx_map.len() ==> orig_idx_map[a] < orig_idx_map[b],
                    // Last entry tracks the latest orig index pushed.
                    orig_idx_map.len() > 0 ==> orig_idx_map.last() < i,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if !feq(elem, x) {
                    let c = elem.clone();
                    let ghost old_result_views = result_views;
                    let ghost old_rv_len = result_views.len() as int;
                    let ghost old_idx_map = orig_idx_map;
                    proof {
                        lemma_cloned_view_eq(*elem, c);
                        assert(self.elements@.contains(elem@));
                        result_views = result_views.push(elem@);
                        orig_idx_map = orig_idx_map.push(i as int);
                    }
                    result_vec.push(c);
                    proof {
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
                        // Strictly increasing: old entries all < old last < i == new entry.
                        assert forall|a: int, b: int|
                            #![trigger orig_idx_map[a], orig_idx_map[b]]
                            0 <= a < b < orig_idx_map.len()
                            implies orig_idx_map[a] < orig_idx_map[b] by {
                            if b < old_idx_map.len() as int {
                                assert(old_idx_map[a] < old_idx_map[b]);
                            } else {
                                // b is the new entry at index old_rv_len.
                                assert(orig_idx_map[b] == i as int);
                                if a < old_idx_map.len() as int {
                                    // old_idx_map[a] <= old_idx_map.last() < i.
                                    if old_idx_map.len() > 1 {
                                        assert(old_idx_map[a] <= old_idx_map[old_idx_map.len() - 1]);
                                    }
                                }
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
                assert forall|k: int| 0 <= k < result_views.len()
                    implies result_views[k] == result_vec@.map_values(|t: T| t@)[k] by {
                    assert(result_vec@.map_values(|t: T| t@)[k] == result_vec@[k]@);
                };
                assert(result_views =~= result_vec@.map_values(|t: T| t@));
            }
            proof {
                lemma_wf_implies_len_bound::<T>(&self.elements.root);
            }
            self.elements = AVLTreeSeqStEphS::from_vec(result_vec);
            proof {
                assert(self.elements@ =~= result_views);
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                assert forall|v|
                    self.elements@.to_set().contains(v)
                    <==> #[trigger] old(self)@.remove(x@).contains(v) by {
                    if self.elements@.to_set().contains(v) {
                        assert(result_views.to_set().contains(v));
                        assert(result_views.contains(v));
                        let k = choose|k: int| 0 <= k < result_views.len()
                            && result_views[k] == v;
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
                // Prove sorted: result is a subsequence of orig_vals (via orig_idx_map).
                lemma_inorder_values_maps_to_views::<T>(self.elements.root);
                let new_vals = spec_inorder_values::<T>(self.elements.root);
                assert(orig_vals.map_values(|t: T| t@) =~= orig_elems);
                // For each k: new_vals[k]@ == result_views[k] == orig_elems[orig_idx_map[k]] == orig_vals[orig_idx_map[k]]@.
                // Under feq: new_vals[k] == orig_vals[orig_idx_map[k]].
                // For i < j: orig_idx_map[i] < orig_idx_map[j], so sorted gives le.
                assert forall|ii: int, jj: int| 0 <= ii < jj < new_vals.len()
                    implies #[trigger] TotalOrder::le(new_vals[ii], new_vals[jj]) by {
                    assert(new_vals[ii]@ == result_views[ii]);
                    assert(new_vals[jj]@ == result_views[jj]);
                    let mi = orig_idx_map[ii];
                    let mj = orig_idx_map[jj];
                    assert(result_views[ii] == orig_elems[mi]);
                    assert(result_views[jj] == orig_elems[mj]);
                    assert(orig_vals[mi]@ == orig_elems[mi]);
                    assert(orig_vals[mj]@ == orig_elems[mj]);
                    assert(new_vals[ii] == orig_vals[mi]);
                    assert(new_vals[jj] == orig_vals[mj]);
                    assert(mi < mj);
                    assert(TotalOrder::le(orig_vals[mi], orig_vals[mj]));
                };
            }
        }

        fn filter_sorted<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let mut filtered = Self::empty();
            proof { lemma_empty_set_is_sorted(&filtered); }
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    filtered@.finite(),
                    filtered.spec_avltreesetsteph_wf(),
                    filtered.spec_elements_sorted(),
                    filtered@.subset_of(self@),
                    filtered@.len() <= i as nat,
                    obeys_feq_full::<T>(),
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
                        assert(c@ == elem@);
                        assert(self.elements@[i as int] == elem@);
                        assert(self.elements@.contains(elem@));
                        assert(self@.contains(elem@));
                        lemma_wf_implies_len_bound::<T>(&self.elements.root);
                        assert(filtered@.len() + 1 < usize::MAX as nat);
                    }
                    let ghost old_filtered = filtered@;
                    filtered.insert_sorted(c);
                    proof {
                        assert(filtered@.subset_of(self@)) by {
                            assert forall|x| #[trigger] filtered@.contains(x)
                                implies self@.contains(x) by {
                                if !old_filtered.contains(x) { assert(x == elem@); }
                            };
                        };
                        assert forall|v: T::V| #[trigger] filtered@.contains(v)
                            implies spec_pred(v) by {
                            if !old_filtered.contains(v) { assert(v == c@); }
                        };
                        assert forall|j: int| #![trigger self.elements@[j]]
                            0 <= j < (i + 1) as int && spec_pred(self.elements@[j])
                            implies filtered@.contains(self.elements@[j]) by {
                            if j < i as int {
                                assert(old_filtered.contains(self.elements@[j]));
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

        fn intersection_sorted(&self, other: &Self) -> (common: Self)
        {
            let mut common = Self::empty();
            proof { lemma_empty_set_is_sorted(&common); }
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
                    common.spec_elements_sorted(),
                    common@.len() <= i as nat,
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
                        lemma_wf_implies_len_bound::<T>(&self.elements.root);
                        assert(common@.len() + 1 < usize::MAX as nat);
                    }
                    let ghost old_common = common@;
                    common.insert_sorted(c);
                    proof {
                        assert(common@.subset_of(self@.intersect(other@))) by {
                            assert forall|x| #[trigger] common@.contains(x)
                                implies self@.intersect(other@).contains(x) by {
                                if !old_common.contains(x) { assert(x == elem@); }
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

        fn difference_sorted(&self, other: &Self) -> (remaining: Self)
        {
            let mut remaining = Self::empty();
            proof { lemma_empty_set_is_sorted(&remaining); }
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
                    remaining.spec_elements_sorted(),
                    remaining@.len() <= i as nat,
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
                        lemma_wf_implies_len_bound::<T>(&self.elements.root);
                        assert(remaining@.len() + 1 < usize::MAX as nat);
                    }
                    let ghost old_remaining = remaining@;
                    remaining.insert_sorted(c);
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

        fn union_sorted(&self, other: &Self) -> (combined: Self)
        {
            let mut combined = Self::empty();
            proof { lemma_empty_set_is_sorted(&combined); }
            let self_len = self.elements.length();
            let mut i: usize = 0;
            // Establish: self_len == self@.len() and other.elements.length() == other@.len().
            // Needed for the capacity bound in the second loop.
            proof {
                self.elements@.unique_seq_to_set();
                other.elements@.unique_seq_to_set();
            }
            while i < self_len
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    self_len as nat == self@.len(),
                    i <= self_len,
                    combined@.finite(),
                    combined.spec_avltreesetsteph_wf(),
                    combined.spec_elements_sorted(),
                    combined@.len() <= i as nat,
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
                    lemma_wf_implies_len_bound::<T>(&self.elements.root);
                    assert(combined@.len() + 1 < usize::MAX as nat);
                }
                let ghost old_combined = combined@;
                combined.insert_sorted(c);
                proof {
                    assert(combined@.subset_of(self@.union(other@))) by {
                        assert forall|x| #[trigger] combined@.contains(x)
                            implies self@.union(other@).contains(x) by {
                            if !old_combined.contains(x) { assert(x == elem@); }
                        };
                    };
                }
                i += 1;
            }
            let other_len = other.elements.length();
            let mut j: usize = 0;
            proof {
                other.elements@.unique_seq_to_set();
            }
            while j < other_len
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    other.elements.spec_avltreeseqsteph_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    other_len as int == other.elements.spec_seq().len(),
                    self_len as nat == self@.len(),
                    other_len as nat == other@.len(),
                    j <= other_len,
                    combined@.finite(),
                    combined.spec_avltreesetsteph_wf(),
                    combined.spec_elements_sorted(),
                    combined@.len() <= self_len as nat + j as nat,
                    obeys_feq_full::<T>(),
                    self@.len() + other@.len() < usize::MAX as nat,
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
                    // combined@.len() <= self_len + j < self_len + other_len
                    //   == self@.len() + other@.len() < usize::MAX (from invariant + requires)
                    assert(combined@.len() + 1 < usize::MAX as nat);
                }
                let ghost old_combined = combined@;
                combined.insert_sorted(c);
                proof {
                    assert(combined@.subset_of(self@.union(other@))) by {
                        assert forall|x| #[trigger] combined@.contains(x)
                            implies self@.union(other@).contains(x) by {
                            if !old_combined.contains(x) { assert(x == elem@); }
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
