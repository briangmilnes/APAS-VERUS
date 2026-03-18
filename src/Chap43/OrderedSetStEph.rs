//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered set implementation extending AVLTreeSetStEph.

pub mod OrderedSetStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{feq, obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::feq;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStEph<T: StT + Ord> {
        pub base_set: AVLTreeSetStEph<T>,
    }

    pub type OrderedSetEph<T> = OrderedSetStEph<T>;

    // 5. view impls

    impl<T: StT + Ord> View for OrderedSetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.base_set@ }
    }

    // 8. traits

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with ephemeral semantics
    pub trait OrderedSetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_orderedsetsteph_wf(&self) -> bool;

        // Base set operations (ADT 41.1) - ephemeral semantics
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- delegates to AVLTreeSetStEph.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedsetsteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty AVLTreeSetStEph
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps AVLTreeSetStEph.singleton
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree@.finite(),
                tree.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- delegates to AVLTreeSetStEph.find (BST search)
        fn find(&self, x: &T) -> (found: B)
            requires self.spec_orderedsetsteph_wf(),
            ensures found == self@.contains(x@);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- delegates to AVLTreeSetStEph.insert (BST insert)
        fn insert(&mut self, x: T)
            requires old(self).spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.insert(x@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- delegates to AVLTreeSetStEph.delete (BST delete)
        fn delete(&mut self, x: &T)
            requires old(self).spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.remove(x@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to AVLTreeSetStEph.filter
        fn filter<F: PredSt<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        )
            requires
                old(self).spec_orderedsetsteph_wf(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to AVLTreeSetStEph.intersection (sequential)
        fn intersection(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.intersect(other@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to AVLTreeSetStEph.union (sequential)
        fn union(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.union(other@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to AVLTreeSetStEph.difference (sequential)
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.difference(other@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to AVLTreeSetStEph.to_seq (in-order traversal)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- delegates to AVLTreeSetStEph.from_seq (n inserts)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            requires seq.spec_avltreeseqstper_wf(),
            ensures
                constructed@.finite(),
                constructed.spec_orderedsetsteph_wf();

        // Ordering operations (ADT 43.1)
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then returns first element
        fn first(&self) -> (first: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| self@.contains(t@) ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then returns last element
        fn last(&self) -> (last: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| self@.contains(t@) ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then scans for predecessor
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: T| self@.contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then scans for successor
        fn next(&self, k: &T) -> (successor: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: T| self@.contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        fn split(&mut self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
            requires old(self).spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.1 == old(self)@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(old(self)@),
                split.2@.subset_of(old(self)@),
                split.0@.disjoint(split.2@),
                !split.0@.contains(k@),
                !split.2@.contains(k@),
                forall|x| old(self)@.contains(x) ==> split.0@.contains(x) || split.2@.contains(x) || x == k@;
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to union (sequential)
        fn join(&mut self, other: Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures self@ == old(self)@.union(other@), self@.finite(), self.spec_orderedsetsteph_wf();
        /// - APAS: Work Θ(log n + m), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then filters by range
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then counts elements < k
        fn rank(&self, k: &T) -> (rank: usize)
            where T: TotalOrder
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                rank <= self@.len(),
                rank as int == self@.filter(|x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then indexes
        fn select(&self, i: usize) -> (selected: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@),
                selected matches Some(v) ==> self@.filter(|x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then partitions by rank
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires old(self).spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(old(self)@),
                split.1@.subset_of(old(self)@),
                split.0@.disjoint(split.1@),
                forall|x| old(self)@.contains(x) ==> split.0@.contains(x) || split.1@.contains(x);
    }

    // 9. impls

    impl<T: StT + Ord> OrderedSetStEphTrait<T> for OrderedSetStEph<T> {
        open spec fn spec_orderedsetsteph_wf(&self) -> bool {
            self.base_set.spec_avltreesetsteph_wf()
        }

        fn size(&self) -> (count: usize)
        { self.base_set.size() }

        fn empty() -> (empty: Self)
        {
            OrderedSetStEph {
                base_set: AVLTreeSetStEph::empty(),
            }
        }

        fn singleton(x: T) -> (tree: Self)
        {
            OrderedSetStEph {
                base_set: AVLTreeSetStEph::singleton(x),
            }
        }

        fn find(&self, x: &T) -> (found: B)
        { self.base_set.find(x) }

        fn insert(&mut self, x: T)
        { self.base_set.insert(x); }

        fn delete(&mut self, x: &T)
        { self.base_set.delete(x); }

        fn filter<F: PredSt<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        )
        {
            let found = self.base_set.filter(f, Ghost(spec_pred));
            self.base_set = found;
        }

        fn intersection(&mut self, other: &Self)
        {
            let found = self.base_set.intersection(&other.base_set);
            self.base_set = found;
        }

        fn union(&mut self, other: &Self)
        {
            let found = self.base_set.union(&other.base_set);
            self.base_set = found;
        }

        fn difference(&mut self, other: &Self)
        {
            let found = self.base_set.difference(&other.base_set);
            self.base_set = found;
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
        {
            let eph_seq = self.base_set.to_seq();
            let len = eph_seq.length();
            let mut elements: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    eph_seq.spec_avltreeseqsteph_wf(),
                    len as nat == eph_seq@.len(),
                    0 <= i <= len,
                    elements@.len() == i as int,
                decreases len - i,
            {
                elements.push(eph_seq.nth(i).clone());
                i = i + 1;
            }
            let result = AVLTreeSeqStPerS::from_vec(elements);
            proof {
                // T::clone preserves View (StT bound); from_vec maps values through View.
                assume(result@ =~= eph_seq@);  // Clone/view bridging.
            }
            result
        }

        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
        {
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqstper_wf(),
                    n as nat == seq.spec_seq().len(),
                    0 <= i <= n,
                    constructed.spec_orderedsetsteph_wf(),
                    constructed@.finite(),
                decreases n - i,
            {
                let elem = seq.nth(i).clone();
                constructed.insert(elem);
                i = i + 1;
            }
            constructed
        }

        fn first(&self) -> (first: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| self@.contains(t@) ==> TotalOrder::le(v, t),
        {
            assert(obeys_feq_full_trigger::<T>());
            let len = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            if len == 0 {
                None
            } else {
                let ghost vals = spec_inorder_values::<T>(self.base_set.elements.root);
                proof { lemma_inorder_values_maps_to_views::<T>(self.base_set.elements.root); }
                let first_ref = self.base_set.elements.nth(0);
                let mut min_val = first_ref.clone();
                proof {
                    lemma_cloned_view_eq(*first_ref, min_val);
                    assert(min_val@ == vals[0int]@);
                    assert(min_val == vals[0int]);
                    T::reflexive(min_val);
                }
                let ghost mut min_idx: int = 0;
                let mut i: usize = 1;
                while i < len
                    invariant
                        self.base_set.elements.spec_avltreeseqsteph_wf(),
                        self.base_set.elements@.no_duplicates(),
                        len as nat == self.base_set.elements@.len(),
                        obeys_feq_full::<T>(),
                        1 <= i, i <= len,
                        0 <= min_idx, min_idx < i,
                        vals == spec_inorder_values::<T>(self.base_set.elements.root),
                        vals.len() == self.base_set.elements@.len(),
                        vals.map_values(|t: T| t@) =~= self.base_set.elements@,
                        min_val@ == self.base_set.elements@[min_idx],
                        vals[min_idx] == min_val,
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i ==> TotalOrder::le(min_val, vals[j]),
                    decreases len - i,
                {
                    let elem_ref = self.base_set.elements.nth(i);
                    let c = TotalOrder::cmp(elem_ref, &min_val);
                    proof {
                        assert(elem_ref@ == vals[i as int]@);
                        assert(*elem_ref == vals[i as int]);
                    }
                    match c {
                        core::cmp::Ordering::Less => {
                            let ghost old_min = min_val;
                            min_val = elem_ref.clone();
                            proof {
                                lemma_cloned_view_eq(*elem_ref, min_val);
                                min_idx = i as int;
                                assert(min_val@ == vals[i as int]@);
                                assert(min_val == vals[i as int]);
                                assert forall|j: int| 0 <= j < i + 1
                                    implies #[trigger] TotalOrder::le(min_val, vals[j]) by {
                                    if j == i as int {
                                        T::reflexive(min_val);
                                    } else {
                                        T::transitive(min_val, old_min, vals[j]);
                                    }
                                };
                            }
                        },
                        core::cmp::Ordering::Equal => {
                            proof { T::reflexive(min_val); }
                        },
                        core::cmp::Ordering::Greater => {
                        },
                    }
                    i = i + 1;
                }
                proof {
                    assert forall|t: T| self@.contains(t@)
                        implies TotalOrder::le(min_val, t) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(vals[j]@ == t@);
                        assert(vals[j] == t);
                    };
                }
                Some(min_val)
            }
        }

        fn last(&self) -> (last: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| self@.contains(t@) ==> TotalOrder::le(t, v),
        {
            assert(obeys_feq_full_trigger::<T>());
            let len = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            if len == 0 {
                None
            } else {
                let ghost vals = spec_inorder_values::<T>(self.base_set.elements.root);
                proof { lemma_inorder_values_maps_to_views::<T>(self.base_set.elements.root); }
                let first_ref = self.base_set.elements.nth(0);
                let mut max_val = first_ref.clone();
                proof {
                    lemma_cloned_view_eq(*first_ref, max_val);
                    assert(max_val@ == vals[0int]@);
                    assert(max_val == vals[0int]);
                    T::reflexive(max_val);
                }
                let ghost mut max_idx: int = 0;
                let mut i: usize = 1;
                while i < len
                    invariant
                        self.base_set.elements.spec_avltreeseqsteph_wf(),
                        self.base_set.elements@.no_duplicates(),
                        len as nat == self.base_set.elements@.len(),
                        obeys_feq_full::<T>(),
                        1 <= i, i <= len,
                        0 <= max_idx, max_idx < i,
                        vals == spec_inorder_values::<T>(self.base_set.elements.root),
                        vals.len() == self.base_set.elements@.len(),
                        vals.map_values(|t: T| t@) =~= self.base_set.elements@,
                        max_val@ == self.base_set.elements@[max_idx],
                        vals[max_idx] == max_val,
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i ==> TotalOrder::le(vals[j], max_val),
                    decreases len - i,
                {
                    let elem_ref = self.base_set.elements.nth(i);
                    let c = TotalOrder::cmp(elem_ref, &max_val);
                    proof {
                        assert(elem_ref@ == vals[i as int]@);
                        assert(*elem_ref == vals[i as int]);
                    }
                    match c {
                        core::cmp::Ordering::Greater => {
                            let ghost old_max = max_val;
                            max_val = elem_ref.clone();
                            proof {
                                lemma_cloned_view_eq(*elem_ref, max_val);
                                max_idx = i as int;
                                assert(max_val@ == vals[i as int]@);
                                assert(max_val == vals[i as int]);
                                assert forall|j: int| 0 <= j < i + 1
                                    implies #[trigger] TotalOrder::le(vals[j], max_val) by {
                                    if j == i as int {
                                        T::reflexive(max_val);
                                    } else {
                                        T::transitive(vals[j], old_max, max_val);
                                    }
                                };
                            }
                        },
                        core::cmp::Ordering::Equal => {
                            proof { T::reflexive(max_val); }
                        },
                        core::cmp::Ordering::Less => {
                        },
                    }
                    i = i + 1;
                }
                proof {
                    assert forall|t: T| self@.contains(t@)
                        implies TotalOrder::le(t, max_val) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(vals[j]@ == t@);
                        assert(vals[j] == t);
                    };
                }
                Some(max_val)
            }
        }

        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            where T: TotalOrder
        {
            assert(obeys_feq_full_trigger::<T>());
            let len = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let ghost vals = spec_inorder_values::<T>(self.base_set.elements.root);
            proof { lemma_inorder_values_maps_to_views::<T>(self.base_set.elements.root); }
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqsteph_wf(),
                    self.base_set.elements@.no_duplicates(),
                    len as nat == self.base_set.elements@.len(),
                    obeys_feq_full::<T>(),
                    0 <= i, i <= len,
                    vals == spec_inorder_values::<T>(self.base_set.elements.root),
                    vals.len() == self.base_set.elements@.len(),
                    vals.map_values(|t: T| t@) =~= self.base_set.elements@,
                    !found ==> forall|j: int| #![trigger vals[j]]
                        0 <= j < i ==> !(TotalOrder::le(vals[j], *k) && vals[j]@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(vals[best_idx], *k) && vals[best_idx]@ != k@ &&
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i && TotalOrder::le(vals[j], *k) && vals[j]@ != k@
                            ==> TotalOrder::le(vals[j], vals[best_idx])
                    ),
                decreases len - i,
            {
                let elem_ref = self.base_set.elements.nth(i);
                proof {
                    assert(elem_ref@ == vals[i as int]@);
                    assert(*elem_ref == vals[i as int]);
                }
                let c = TotalOrder::cmp(elem_ref, k);
                match c {
                    core::cmp::Ordering::Less => {
                        // *elem_ref < *k: a predecessor candidate.
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                T::reflexive(vals[i as int]);
                            }
                        } else {
                            let best_ref = self.base_set.elements.nth(best_pos);
                            proof {
                                assert(best_ref@ == vals[best_idx]@);
                                assert(*best_ref == vals[best_idx]);
                            }
                            let c2 = TotalOrder::cmp(elem_ref, best_ref);
                            match c2 {
                                core::cmp::Ordering::Greater => {
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(vals[j], *k) && vals[j]@ != k@
                                            implies #[trigger] TotalOrder::le(vals[j], vals[best_idx]) by {
                                            if j == i as int {
                                                T::reflexive(vals[i as int]);
                                            } else {
                                                T::transitive(vals[j], vals[old_best], vals[i as int]);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        T::total(vals[i as int], vals[best_idx]);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                        // *elem_ref == *k: not strictly less.
                    },
                    core::cmp::Ordering::Greater => {
                        // *k < *elem_ref: not <= k.
                        proof {
                            // le(*k, *elem_ref) && *elem_ref != *k
                            // Suppose le(vals[i], *k) were true.
                            // Then le(*k, vals[i]) && le(vals[i], *k) → vals[i] == *k.
                            // But *elem_ref != *k and vals[i] == *elem_ref → contradiction.
                            if TotalOrder::le(vals[i as int], *k) {
                                T::antisymmetric(vals[i as int], *k);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            if !found {
                None
            } else {
                let result_ref = self.base_set.elements.nth(best_pos);
                let result = result_ref.clone();
                proof {
                    lemma_cloned_view_eq(*result_ref, result);
                    assert(result@ == vals[best_idx]@);
                    assert(result == vals[best_idx]);
                    assert forall|t: T| self@.contains(t@) && TotalOrder::le(t, *k) && t@ != k@
                        implies TotalOrder::le(t, result) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(vals[j]@ == t@);
                        assert(vals[j] == t);
                    };
                }
                Some(result)
            }
        }

        fn next(&self, k: &T) -> (successor: Option<T>)
            where T: TotalOrder
        {
            assert(obeys_feq_full_trigger::<T>());
            let len = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let ghost vals = spec_inorder_values::<T>(self.base_set.elements.root);
            proof { lemma_inorder_values_maps_to_views::<T>(self.base_set.elements.root); }
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqsteph_wf(),
                    self.base_set.elements@.no_duplicates(),
                    len as nat == self.base_set.elements@.len(),
                    obeys_feq_full::<T>(),
                    0 <= i, i <= len,
                    vals == spec_inorder_values::<T>(self.base_set.elements.root),
                    vals.len() == self.base_set.elements@.len(),
                    vals.map_values(|t: T| t@) =~= self.base_set.elements@,
                    !found ==> forall|j: int| #![trigger vals[j]]
                        0 <= j < i ==> !(TotalOrder::le(*k, vals[j]) && vals[j]@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(*k, vals[best_idx]) && vals[best_idx]@ != k@ &&
                        forall|j: int| #![trigger vals[j]]
                            0 <= j < i && TotalOrder::le(*k, vals[j]) && vals[j]@ != k@
                            ==> TotalOrder::le(vals[best_idx], vals[j])
                    ),
                decreases len - i,
            {
                let elem_ref = self.base_set.elements.nth(i);
                proof {
                    assert(elem_ref@ == vals[i as int]@);
                    assert(*elem_ref == vals[i as int]);
                }
                let c = TotalOrder::cmp(elem_ref, k);
                match c {
                    core::cmp::Ordering::Greater => {
                        // *elem_ref > *k: a successor candidate.
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                T::reflexive(vals[i as int]);
                            }
                        } else {
                            let best_ref = self.base_set.elements.nth(best_pos);
                            proof {
                                assert(best_ref@ == vals[best_idx]@);
                                assert(*best_ref == vals[best_idx]);
                            }
                            let c2 = TotalOrder::cmp(elem_ref, best_ref);
                            match c2 {
                                core::cmp::Ordering::Less => {
                                    // New element is smaller (closer to k), update best.
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(*k, vals[j]) && vals[j]@ != k@
                                            implies #[trigger] TotalOrder::le(vals[best_idx], vals[j]) by {
                                            if j == i as int {
                                                T::reflexive(vals[i as int]);
                                            } else {
                                                T::transitive(vals[i as int], vals[old_best], vals[j]);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        T::total(vals[best_idx], vals[i as int]);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                        // *elem_ref == *k: not strictly greater.
                    },
                    core::cmp::Ordering::Less => {
                        // *elem_ref < *k: le(*k, vals[i]) would mean le(*k, *elem_ref).
                        // Combined with le(*elem_ref, *k), we'd get *elem_ref == *k,
                        // but *elem_ref != *k, contradiction.
                        proof {
                            if TotalOrder::le(*k, vals[i as int]) {
                                T::antisymmetric(*k, vals[i as int]);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            if !found {
                None
            } else {
                let result_ref = self.base_set.elements.nth(best_pos);
                let result = result_ref.clone();
                proof {
                    lemma_cloned_view_eq(*result_ref, result);
                    assert(result@ == vals[best_idx]@);
                    assert(result == vals[best_idx]);
                    assert forall|t: T| self@.contains(t@) && TotalOrder::le(*k, t) && t@ != k@
                        implies TotalOrder::le(result, t) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(vals[j]@ == t@);
                        assert(vals[j] == t);
                    };
                }
                Some(result)
            }
        }

        fn split(&mut self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
        {
            assert(obeys_feq_full_trigger::<T>());
            let n = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let ghost old_view = self@;
            let ghost old_elems = self.base_set.elements@;
            let mut left = Self::empty();
            let mut right = Self::empty();
            let mut found = false;
            let mut j: usize = 0;
            while j < n
                invariant
                    self.base_set.elements.spec_avltreeseqsteph_wf(),
                    self.base_set.elements@.no_duplicates(),
                    n as nat == self.base_set.elements@.len(),
                    obeys_feq_full::<T>(),
                    old_view == self.base_set.elements@.to_set(),
                    old_elems == self.base_set.elements@,
                    0 <= j <= n,
                    left.spec_orderedsetsteph_wf(),
                    left@.finite(),
                    right.spec_orderedsetsteph_wf(),
                    right@.finite(),
                    left@.subset_of(old_view),
                    right@.subset_of(old_view),
                    left@.disjoint(right@),
                    !left@.contains(k@),
                    !right@.contains(k@),
                    found ==> old_view.contains(k@),
                    !found ==> forall|idx: int| #![trigger old_elems[idx]]
                        0 <= idx < j ==> old_elems[idx] != k@,
                    // Provenance: left/right elements come from visited indices.
                    forall|x| left@.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && #[trigger] old_elems[idx] == x,
                    forall|x| right@.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && #[trigger] old_elems[idx] == x,
                    // Coverage: every visited element is accounted for.
                    forall|idx: int| #![trigger old_elems[idx]]
                        0 <= idx < j ==> left@.contains(old_elems[idx]) || right@.contains(old_elems[idx]) || old_elems[idx] == k@,
                decreases n - j,
            {
                let elem_ref = self.base_set.elements.nth(j);
                if feq(elem_ref, k) {
                    found = true;
                    proof {
                        assert(elem_ref@ == old_elems[j as int]);
                        assert(old_elems[j as int] == k@);
                        assert(old_view.contains(k@));
                    }
                } else {
                    let cloned = elem_ref.clone();
                    proof {
                        lemma_cloned_view_eq(*elem_ref, cloned);
                        assert(cloned@ == old_elems[j as int]);
                        assert(old_view.contains(cloned@));
                        assert(cloned@ != k@);
                    }
                    if cloned < *k {
                        let ghost old_left_view = left@;
                        left.insert(cloned);
                        proof {
                            assert(left@.subset_of(old_view)) by {
                                assert forall|x| #[trigger] left@.contains(x) implies old_view.contains(x) by {
                                    if !old_left_view.contains(x) {
                                        assert(x == cloned@);
                                    }
                                };
                            };
                            assert(!left@.contains(k@)) by {
                                assert forall|x| #[trigger] left@.contains(x) implies x != k@ by {
                                    if !old_left_view.contains(x) {
                                        assert(x == cloned@);
                                    }
                                };
                            };
                            assert(left@.disjoint(right@)) by {
                                assert forall|x| !(#[trigger] left@.contains(x) && right@.contains(x)) by {
                                    if left@.contains(x) && right@.contains(x) {
                                        if !old_left_view.contains(x) {
                                            assert(x == cloned@);
                                            assert(x == old_elems[j as int]);
                                            let idx = choose|idx: int| 0 <= idx < j && old_elems[idx] == x;
                                            assert(old_elems[idx] == old_elems[j as int]);
                                        }
                                    }
                                };
                            };
                            assert forall|x| left@.contains(x) implies
                                exists|idx: int| 0 <= idx < j + 1 && #[trigger] old_elems[idx] == x by {
                                if old_left_view.contains(x) {
                                } else {
                                    assert(x == cloned@);
                                    assert(old_elems[j as int] == x);
                                }
                            };
                        }
                    } else {
                        let ghost old_right_view = right@;
                        right.insert(cloned);
                        proof {
                            assert(right@.subset_of(old_view)) by {
                                assert forall|x| #[trigger] right@.contains(x) implies old_view.contains(x) by {
                                    if !old_right_view.contains(x) {
                                        assert(x == cloned@);
                                    }
                                };
                            };
                            assert(!right@.contains(k@)) by {
                                assert forall|x| #[trigger] right@.contains(x) implies x != k@ by {
                                    if !old_right_view.contains(x) {
                                        assert(x == cloned@);
                                    }
                                };
                            };
                            assert(left@.disjoint(right@)) by {
                                assert forall|x| !(#[trigger] left@.contains(x) && right@.contains(x)) by {
                                    if left@.contains(x) && right@.contains(x) {
                                        if !old_right_view.contains(x) {
                                            assert(x == cloned@);
                                            assert(x == old_elems[j as int]);
                                            let idx = choose|idx: int| 0 <= idx < j && old_elems[idx] == x;
                                            assert(old_elems[idx] == old_elems[j as int]);
                                        }
                                    }
                                };
                            };
                            assert forall|x| right@.contains(x) implies
                                exists|idx: int| 0 <= idx < j + 1 && #[trigger] old_elems[idx] == x by {
                                if old_right_view.contains(x) {
                                } else {
                                    assert(x == cloned@);
                                    assert(old_elems[j as int] == x);
                                }
                            };
                        }
                    }
                }
                j = j + 1;
            }
            proof {
                // found == old_view.contains(k@)
                if !found {
                    if old_view.contains(k@) {
                        assert(old_elems.to_set().contains(k@));
                        assert(old_elems.contains(k@));
                        let idx = choose|idx: int| 0 <= idx < old_elems.len() && old_elems[idx] == k@;
                        assert(old_elems[idx] != k@);
                    }
                }
                // Coverage: old_view -> left | right | {k@}
                assert forall|x| old_view.contains(x)
                    implies left@.contains(x) || right@.contains(x) || x == k@ by {
                    assert(old_elems.to_set().contains(x));
                    assert(old_elems.contains(x));
                    let idx = choose|idx: int| 0 <= idx < old_elems.len() && old_elems[idx] == x;
                    assert(left@.contains(old_elems[idx]) || right@.contains(old_elems[idx]) || old_elems[idx] == k@);
                };
            }
            *self = Self::empty();
            (left, found, right)
        }

        fn join(&mut self, other: Self)
            ensures self@ == old(self)@.union(other@), self@.finite(), self.spec_orderedsetsteph_wf()
        { self.union(&other); }

        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut range = Self::empty();
            let n = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let ghost vals = spec_inorder_values::<T>(self.base_set.elements.root);
            proof { lemma_inorder_values_maps_to_views::<T>(self.base_set.elements.root); }
            let mut i: usize = 0;
            while i < n
                invariant
                    self.base_set.elements.spec_avltreeseqsteph_wf(),
                    self.base_set.elements@.no_duplicates(),
                    n as nat == self.base_set.elements@.len(),
                    obeys_feq_full::<T>(),
                    vals == spec_inorder_values::<T>(self.base_set.elements.root),
                    vals.len() == self.base_set.elements@.len(),
                    vals.map_values(|t: T| t@) =~= self.base_set.elements@,
                    0 <= i <= n,
                    range.spec_orderedsetsteph_wf(),
                    range@.finite(),
                    range@.subset_of(self@),
                decreases n - i,
            {
                let elem_ref = self.base_set.elements.nth(i);
                if *elem_ref >= *k1 && *elem_ref <= *k2 {
                    let cloned = elem_ref.clone();
                    proof {
                        lemma_cloned_view_eq(*elem_ref, cloned);
                        assert(cloned@ == self.base_set.elements@[i as int]);
                        assert(self.base_set.elements@.to_set().contains(cloned@));
                        assert(self@.contains(cloned@));
                        // After insert, range@ == old(range)@.insert(cloned@).
                        // subset_of: old(range)@.subset_of(self@) && self@.contains(cloned@)
                        // ==> old(range)@.insert(cloned@).subset_of(self@).
                    }
                    range.insert(cloned);
                }
                i = i + 1;
            }
            range
        }

        fn rank(&self, k: &T) -> (rank: usize)
            where T: TotalOrder
        {
            assert(obeys_feq_full_trigger::<T>());
            let n = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let ghost vals = spec_inorder_values::<T>(self.base_set.elements.root);
            proof { lemma_inorder_values_maps_to_views::<T>(self.base_set.elements.root); }
            let ghost elems = self.base_set.elements@;
            let mut count: usize = 0;
            let ghost mut counted: Set<T::V> = Set::empty();
            let mut j: usize = 0;
            while j < n
                invariant
                    self.base_set.elements.spec_avltreeseqsteph_wf(),
                    elems.no_duplicates(),
                    n as nat == elems.len(),
                    elems =~= self.base_set.elements@,
                    obeys_feq_full::<T>(),
                    0 <= j <= n,
                    vals == spec_inorder_values::<T>(self.base_set.elements.root),
                    vals.len() == elems.len(),
                    vals.map_values(|t: T| t@) =~= elems,
                    counted.finite(),
                    count as nat == counted.len(),
                    0 <= count <= j,
                    counted.subset_of(elems.to_set()),
                    forall|x: T::V| #[trigger] counted.contains(x) ==>
                        exists|t: T| t@ == x && TotalOrder::le(t, *k) && t@ != k@,
                    forall|idx: int| #![trigger vals[idx]]
                        0 <= idx < j && TotalOrder::le(vals[idx], *k) && vals[idx] != *k
                        ==> counted.contains(vals[idx]@),
                    forall|x: T::V| #[trigger] counted.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && #[trigger] elems[idx] == x,
                decreases n - j,
            {
                let elem_ref = self.base_set.elements.nth(j);
                proof {
                    assert(elem_ref@ == vals[j as int]@);
                    assert(*elem_ref == vals[j as int]);
                }
                let c = TotalOrder::cmp(elem_ref, k);
                match c {
                    core::cmp::Ordering::Less => {
                        proof {
                            let ghost x = vals[j as int]@;
                            let ghost old_counted = counted;
                            assert(!old_counted.contains(x)) by {
                                if old_counted.contains(x) {
                                    let idx = choose|idx: int| 0 <= idx < j && elems[idx] == x;
                                    assert(elems[idx] == elems[j as int]);
                                }
                            };
                            counted = old_counted.insert(x);
                            assert(elems[j as int] == x);
                            assert(elems.to_set().contains(x));
                        }
                        count = count + 1;
                    },
                    core::cmp::Ordering::Equal => {},
                    core::cmp::Ordering::Greater => {
                        proof {
                            if TotalOrder::le(vals[j as int], *k) {
                                T::antisymmetric(vals[j as int], *k);
                            }
                        }
                    },
                }
                j = j + 1;
            }
            proof {
                assert forall|x: T::V|
                    elems.to_set().contains(x)
                    && (exists|t: T| t@ == x && TotalOrder::le(t, *k) && t@ != k@)
                    implies #[trigger] counted.contains(x) by {
                    assert(elems.contains(x));
                    let idx = choose|idx: int| 0 <= idx < elems.len() && elems[idx] == x;
                    assert(vals[idx]@ == x);
                    let t: T = choose|t: T| t@ == x && TotalOrder::le(t, *k) && t@ != k@;
                    assert(t == vals[idx]);
                };
                assert(counted =~= self@.filter(
                    |x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, *k) && t@ != k@));
            }
            count
        }

        fn select(&self, i: usize) -> (selected: Option<T>)
            where T: TotalOrder
        {
            assert(obeys_feq_full_trigger::<T>());
            let sz = self.size();
            if i >= sz {
                None
            } else {
                proof { self.base_set.elements@.unique_seq_to_set(); }
                let elem = self.base_set.elements.nth(i);
                let result = elem.clone();
                proof {
                    lemma_cloned_view_eq(*elem, result);
                    assert(self.base_set.elements@.to_set().contains(result@));
                    // Filter cardinality requires sortedness of the backing sequence,
                    // which is true for AVL trees but not captured in the wf spec.
                    assume(self@.filter(|x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, result) && t@ != result@).len() == i as int);
                }
                Some(result)
            }
        }

        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            assert(obeys_feq_full_trigger::<T>());
            let n = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let ghost old_view = self@;
            let ghost old_elems = self.base_set.elements@;
            let split_at: usize = if i >= n { n } else { i };
            let mut left = Self::empty();
            let mut right = Self::empty();
            let mut j: usize = 0;
            while j < n
                invariant
                    self.base_set.elements.spec_avltreeseqsteph_wf(),
                    self.base_set.elements@.no_duplicates(),
                    n as nat == self.base_set.elements@.len(),
                    obeys_feq_full::<T>(),
                    old_view == self.base_set.elements@.to_set(),
                    old_elems == self.base_set.elements@,
                    0 <= j <= n,
                    left.spec_orderedsetsteph_wf(),
                    left@.finite(),
                    right.spec_orderedsetsteph_wf(),
                    right@.finite(),
                    left@.subset_of(old_view),
                    right@.subset_of(old_view),
                    split_at <= n,
                    // Provenance: left has elements from indices < split_at only.
                    forall|x| left@.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && idx < split_at as int && #[trigger] old_elems[idx] == x,
                    // Provenance: right has elements from indices >= split_at only.
                    forall|x| right@.contains(x) ==>
                        exists|idx: int| split_at as int <= idx && idx < j && #[trigger] old_elems[idx] == x,
                    // Coverage: visited elements are in left or right.
                    forall|idx: int| #![trigger old_elems[idx]]
                        0 <= idx < j ==> left@.contains(old_elems[idx]) || right@.contains(old_elems[idx]),
                decreases n - j,
            {
                let elem_ref = self.base_set.elements.nth(j);
                let cloned = elem_ref.clone();
                proof {
                    lemma_cloned_view_eq(*elem_ref, cloned);
                    assert(cloned@ == old_elems[j as int]);
                    assert(old_view.contains(cloned@));
                }
                if j < split_at {
                    let ghost old_left_view = left@;
                    left.insert(cloned);
                    proof {
                        assert(left@.subset_of(old_view)) by {
                            assert forall|x| #[trigger] left@.contains(x) implies old_view.contains(x) by {
                                if !old_left_view.contains(x) {
                                    assert(x == elem_ref@);
                                }
                            };
                        };
                        assert forall|x| left@.contains(x) implies
                            exists|idx: int| 0 <= idx < j + 1 && idx < split_at as int && #[trigger] old_elems[idx] == x by {
                            if old_left_view.contains(x) {
                            } else {
                                assert(x == elem_ref@);
                                assert(old_elems[j as int] == x);
                            }
                        };
                    }
                } else {
                    let ghost old_right_view = right@;
                    right.insert(cloned);
                    proof {
                        assert(right@.subset_of(old_view)) by {
                            assert forall|x| #[trigger] right@.contains(x) implies old_view.contains(x) by {
                                if !old_right_view.contains(x) {
                                    assert(x == elem_ref@);
                                }
                            };
                        };
                        assert forall|x| right@.contains(x) implies
                            exists|idx: int| split_at as int <= idx && idx < j + 1 && #[trigger] old_elems[idx] == x by {
                            if old_right_view.contains(x) {
                            } else {
                                assert(x == elem_ref@);
                                assert(old_elems[j as int] == x);
                            }
                        };
                    }
                }
                j = j + 1;
            }
            proof {
                // Disjointness from provenance + no_duplicates.
                assert(left@.disjoint(right@)) by {
                    assert forall|x| !(#[trigger] left@.contains(x) && right@.contains(x)) by {
                        if left@.contains(x) && right@.contains(x) {
                            let idx1 = choose|idx: int| 0 <= idx < n && idx < split_at as int && old_elems[idx] == x;
                            let idx2 = choose|idx: int| split_at as int <= idx && idx < n as int && old_elems[idx] == x;
                            assert(idx1 != idx2);
                        }
                    };
                };
                // Coverage: every element in old_view was visited.
                assert forall|x| old_view.contains(x)
                    implies left@.contains(x) || right@.contains(x) by {
                    assert(old_elems.to_set().contains(x));
                    assert(old_elems.contains(x));
                    let idx = choose|idx: int| 0 <= idx < old_elems.len() && old_elems[idx] == x;
                    assert(left@.contains(old_elems[idx]) || right@.contains(old_elems[idx]));
                };
            }
            *self = Self::empty();
            (left, right)
        }
    }

    

    impl<T: StT + Ord> OrderedSetStEph<T> {
        /// Returns an iterator over the set elements in sorted order.
        pub fn iter(&self) -> (it: OrderedSetStEphIter<'_, T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedSetStEphIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStEphIter<'a, T: StT + Ord> {
        pub seq: &'a AVLTreeSeqStEphS<T>,
        pub pos: usize,
        pub len: usize,
    }

    impl<'a, T: StT + Ord> View for OrderedSetStEphIter<'a, T> {
        type V = (int, Seq<T::V>);
        open spec fn view(&self) -> (int, Seq<T::V>) { (self.pos as int, self.seq@) }
    }

    pub open spec fn iter_invariant<'a, T: StT + Ord>(it: &OrderedSetStEphIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T: StT + Ord> std::iter::Iterator for OrderedSetStEphIter<'a, T> {
        type Item = &'a T;

        #[verifier::external_body]
        fn next(&mut self) -> (next: Option<&'a T>)
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
            if self.pos < self.len {
                let elem = self.seq.nth(self.pos);
                self.pos = self.pos + 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStEphGhostIterator<'a, T: StT + Ord> {
        pub pos: int,
        pub elements: Seq<T::V>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T: StT + Ord> View for OrderedSetStEphGhostIterator<'a, T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, T: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedSetStEphIter<'a, T> {
        type GhostIter = OrderedSetStEphGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> OrderedSetStEphGhostIterator<'a, T> {
            OrderedSetStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedSetStEphGhostIterator<'a, T> {
        type ExecIter = OrderedSetStEphIter<'a, T>;
        type Item = T::V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedSetStEphIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedSetStEphIter<'a, T>) -> OrderedSetStEphGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT + Ord> std::iter::IntoIterator for &'a OrderedSetStEph<T> {
        type Item = &'a T;
        type IntoIter = OrderedSetStEphIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedSetStEphIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for OrderedSetStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            OrderedSetStEph {
                base_set: self.base_set.clone(),
            }
        }
    }

    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> (constructed: OrderedSetStEph<T>)
        ensures constructed@.finite(), constructed.spec_orderedsetsteph_wf()
    {
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        OrderedSetStEph::from_seq(seq)
    }

    } // verus!

    // 12. macros

    /// Macro for creating ephemeral ordered sets from sorted element lists
    #[macro_export]
    macro_rules! OrderedSetStEphLit {
        () => {
            $crate::Chap43::OrderedSetStEph::OrderedSetStEph::OrderedSetStEph::empty()
        };
        ($($elem:expr),+ $(,)?) => {
            $crate::Chap43::OrderedSetStEph::OrderedSetStEph::from_sorted_elements(vec![$($elem),+])
        };
    }

    // 13. derive impls outside verus!

    impl<T: StT + Ord> Default for OrderedSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> PartialEq for OrderedSetStEph<T> {
        fn eq(&self, other: &Self) -> bool {
            self.size() == other.size() && {
                let seq = self.to_seq();
                for i in 0..seq.length() {
                    if !other.find(seq.nth(i)) {
                        return false;
                    }
                }
                true
            }
        }
    }

    impl<T: StT + Ord> fmt::Debug for OrderedSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord> fmt::Display for OrderedSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
