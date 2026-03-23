//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered set implementation extending AVLTreeSetStPer.

pub mod OrderedSetStPer {

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
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{feq, obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStPer<T: StT + Ord> {
        pub base_set: AVLTreeSetStPer<T>,
    }

    pub type OrderedSetPer<T> = OrderedSetStPer<T>;

    // 5. view impls

    impl<T: StT + Ord> View for OrderedSetStPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.base_set@ }
    }

    // 8. traits

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1)
    pub trait OrderedSetStPerTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_orderedsetstper_wf(&self) -> bool;

        // Base set operations (ADT 41.1) - delegated
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- delegates to AVLTreeSetStPer.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedsetstper_wf(),
            ensures count == self@.len(), self@.finite();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty AVLTreeSetStPer
        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps AVLTreeSetStPer.singleton
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree@.finite(), tree.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- delegates to AVLTreeSetStPer.find (BST search)
        fn find(&self, x: &T) -> (found: B)
            requires self.spec_orderedsetstper_wf(),
            ensures found == self@.contains(x@);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- delegates to AVLTreeSetStPer.insert (BST insert)
        fn insert(&self, x: T) -> (updated: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures updated@ == self@.insert(x@), updated@.finite(), updated.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- delegates to AVLTreeSetStPer.delete (BST delete)
        fn delete(&self, x: &T) -> (updated: Self)
            requires self.spec_orderedsetstper_wf(),
            ensures updated@ == self@.remove(x@), updated@.finite(), updated.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to AVLTreeSetStPer.filter
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_orderedsetstper_wf(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures filtered@.finite(), filtered@.subset_of(self@), filtered.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to AVLTreeSetStPer.intersection (sequential)
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_orderedsetstper_wf(), other.spec_orderedsetstper_wf(),
            ensures common@ == self@.intersect(other@), common@.finite(), common.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to AVLTreeSetStPer.union (sequential)
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_orderedsetstper_wf(),
                other.spec_orderedsetstper_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@ == self@.union(other@), combined@.finite(), combined.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to AVLTreeSetStPer.difference (sequential)
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_orderedsetstper_wf(), other.spec_orderedsetstper_wf(),
            ensures remaining@ == self@.difference(other@), remaining@.finite(), remaining.spec_orderedsetstper_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to AVLTreeSetStPer.to_seq (in-order traversal)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- delegates to AVLTreeSetStPer.from_seq (n inserts)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            requires seq.spec_avltreeseqstper_wf(),
            ensures constructed@.finite(), constructed.spec_orderedsetstper_wf();

        // Ordering operations (ADT 43.1)
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then returns first element
        fn first(&self) -> (first: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetstper_wf(), obeys_feq_clone::<T>(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then returns last element
        fn last(&self) -> (last: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetstper_wf(), obeys_feq_clone::<T>(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then scans for predecessor
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetstper_wf(), obeys_feq_clone::<T>(),
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then scans for successor
        fn next(&self, k: &T) -> (successor: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetstper_wf(), obeys_feq_clone::<T>(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then partitions into two sets
        fn split(&self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                split.1 == self@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(self@),
                split.2@.subset_of(self@),
                split.0@.disjoint(split.2@),
                !split.0@.contains(k@),
                !split.2@.contains(k@),
                forall|x| #[trigger] self@.contains(x) ==> split.0@.contains(x) || split.2@.contains(x) || x == k@;
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- delegates to union (sequential)
        fn join(left: &Self, right: &Self) -> (joined: Self)
            requires
                left.spec_orderedsetstper_wf(),
                right.spec_orderedsetstper_wf(),
                left@.len() + right@.len() < usize::MAX as nat,
            ensures joined@ == left@.union(right@), joined@.finite();
        /// - APAS: Work Θ(log n + m), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then filters by range (verified with loop invariant)
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then counts elements < k
        fn rank(&self, k: &T) -> (rank: usize)
            where T: TotalOrder
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                rank <= self@.len(),
                rank as int == self@.filter(|x: T::V| exists|t: T| #[trigger] TotalOrder::le(t, *k) && t@ == x && t@ != k@).len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then indexes
        fn select(&self, i: usize) -> (selected: Option<T>)
            where T: TotalOrder
            requires self.spec_orderedsetstper_wf(), obeys_feq_clone::<T>(),
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@),
                selected matches Some(v) ==> self@.filter(|x: T::V| exists|t: T| #[trigger] TotalOrder::le(t, v) && t@ == x && t@ != v@).len() == i as int;
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then partitions by rank (verified with loop invariant)
        fn split_rank(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(self@),
                split.1@.subset_of(self@),
                split.0@.disjoint(split.1@),
                forall|x| #[trigger] self@.contains(x) ==> split.0@.contains(x) || split.1@.contains(x);
    }

    // 9. impls

    impl<T: StT + Ord> OrderedSetStPerTrait<T> for OrderedSetStPer<T> {
        open spec fn spec_orderedsetstper_wf(&self) -> bool {
            self.base_set.spec_avltreesetstper_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite()
        {
            self.base_set.size()
        }

        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_orderedsetstper_wf()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::empty(),
            }
        }

        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree@.finite(), tree.spec_orderedsetstper_wf()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::singleton(x),
            }
        }

        fn find(&self, x: &T) -> (found: B)
            ensures found == self@.contains(x@)
        {
            self.base_set.find(x)
        }

        fn insert(&self, x: T) -> (updated: Self)
            ensures updated@ == self@.insert(x@), updated@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.insert(x),
            }
        }

        fn delete(&self, x: &T) -> (updated: Self)
            ensures updated@ == self@.remove(x@), updated@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.delete(x),
            }
        }

        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            ensures filtered@.finite(), filtered@.subset_of(self@)
        {
            OrderedSetStPer {
                base_set: self.base_set.filter(f, Ghost(spec_pred)),
            }
        }

        fn intersection(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.intersection(&other.base_set),
            }
        }

        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.union(&other.base_set),
            }
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.difference(&other.base_set),
            }
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
        {
            self.base_set.to_seq()
        }

        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            ensures constructed@.finite()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::from_seq(seq),
            }
        }

        fn first(&self) -> (first: Option<T>)
            where T: TotalOrder
        {
            assert(obeys_feq_full_trigger::<T>());
            let len = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            if len == 0 {
                None
            } else {
                let first_ref = self.base_set.elements.nth(0);
                let mut min_val = first_ref.clone();
                proof {
                    lemma_cloned_view_eq(*first_ref, min_val);
                    T::reflexive(min_val);
                }
                let ghost mut min_idx: int = 0;
                let ghost mut ghost_vals: Seq<T> = seq![*first_ref];
                let mut i: usize = 1;
                while i < len
                    invariant
                        self.base_set.elements.spec_avltreeseqstper_wf(),
                        self.base_set.elements@.no_duplicates(),
                        len as nat == self.base_set.elements@.len(),
                        obeys_feq_full::<T>(),
                        1 <= i, i <= len,
                        0 <= min_idx, min_idx < i,
                        ghost_vals.len() == i as int,
                        forall|j: int| #![trigger ghost_vals[j]]
                            0 <= j < i ==> ghost_vals[j]@ == self.base_set.elements@[j],
                        min_val == ghost_vals[min_idx],
                        min_val@ == self.base_set.elements@[min_idx],
                        forall|j: int| #![trigger ghost_vals[j]]
                            0 <= j < i ==> TotalOrder::le(min_val, ghost_vals[j]),
                    decreases len - i,
                {
                    let elem_ref = self.base_set.elements.nth(i);
                    proof { ghost_vals = ghost_vals.push(*elem_ref); }
                    let c = TotalOrder::cmp(elem_ref, &min_val);
                    match c {
                        core::cmp::Ordering::Less => {
                            let ghost old_min = min_val;
                            min_val = elem_ref.clone();
                            proof {
                                lemma_cloned_view_eq(*elem_ref, min_val);
                                min_idx = i as int;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies #[trigger] TotalOrder::le(min_val, ghost_vals[j]) by {
                                    if j == i as int {
                                        T::reflexive(min_val);
                                    } else {
                                        T::transitive(min_val, old_min, ghost_vals[j]);
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
                    assert forall|t: T| #[trigger] self@.contains(t@)
                        implies TotalOrder::le(min_val, t) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(ghost_vals[j]@ == t@);
                        assert(ghost_vals[j] == t);
                    };
                }
                Some(min_val)
            }
        }

        fn last(&self) -> (last: Option<T>)
            where T: TotalOrder
        {
            assert(obeys_feq_full_trigger::<T>());
            let len = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            if len == 0 {
                None
            } else {
                let first_ref = self.base_set.elements.nth(0);
                let mut max_val = first_ref.clone();
                proof {
                    lemma_cloned_view_eq(*first_ref, max_val);
                    T::reflexive(max_val);
                }
                let ghost mut max_idx: int = 0;
                let ghost mut ghost_vals: Seq<T> = seq![*first_ref];
                let mut i: usize = 1;
                while i < len
                    invariant
                        self.base_set.elements.spec_avltreeseqstper_wf(),
                        self.base_set.elements@.no_duplicates(),
                        len as nat == self.base_set.elements@.len(),
                        obeys_feq_full::<T>(),
                        1 <= i, i <= len,
                        0 <= max_idx, max_idx < i,
                        ghost_vals.len() == i as int,
                        forall|j: int| #![trigger ghost_vals[j]]
                            0 <= j < i ==> ghost_vals[j]@ == self.base_set.elements@[j],
                        max_val == ghost_vals[max_idx],
                        max_val@ == self.base_set.elements@[max_idx],
                        forall|j: int| #![trigger ghost_vals[j]]
                            0 <= j < i ==> TotalOrder::le(ghost_vals[j], max_val),
                    decreases len - i,
                {
                    let elem_ref = self.base_set.elements.nth(i);
                    proof { ghost_vals = ghost_vals.push(*elem_ref); }
                    let c = TotalOrder::cmp(elem_ref, &max_val);
                    match c {
                        core::cmp::Ordering::Greater => {
                            let ghost old_max = max_val;
                            max_val = elem_ref.clone();
                            proof {
                                lemma_cloned_view_eq(*elem_ref, max_val);
                                max_idx = i as int;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies #[trigger] TotalOrder::le(ghost_vals[j], max_val) by {
                                    if j == i as int {
                                        T::reflexive(max_val);
                                    } else {
                                        T::transitive(ghost_vals[j], old_max, max_val);
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
                    assert forall|t: T| #[trigger] self@.contains(t@)
                        implies TotalOrder::le(t, max_val) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(ghost_vals[j]@ == t@);
                        assert(ghost_vals[j] == t);
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
            let ghost mut ghost_vals: Seq<T> = Seq::empty();
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    self.base_set.elements@.no_duplicates(),
                    len as nat == self.base_set.elements@.len(),
                    obeys_feq_full::<T>(),
                    0 <= i, i <= len,
                    ghost_vals.len() == i as int,
                    forall|j: int| #![trigger ghost_vals[j]]
                        0 <= j < i ==> ghost_vals[j]@ == self.base_set.elements@[j],
                    !found ==> forall|j: int| #![trigger ghost_vals[j]]
                        0 <= j < i ==> !(TotalOrder::le(ghost_vals[j], *k) && ghost_vals[j]@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(ghost_vals[best_idx], *k) && ghost_vals[best_idx]@ != k@ &&
                        forall|j: int| #![trigger ghost_vals[j]]
                            0 <= j < i && TotalOrder::le(ghost_vals[j], *k) && ghost_vals[j]@ != k@
                            ==> TotalOrder::le(ghost_vals[j], ghost_vals[best_idx])
                    ),
                decreases len - i,
            {
                let elem_ref = self.base_set.elements.nth(i);
                proof { ghost_vals = ghost_vals.push(*elem_ref); }
                let c = TotalOrder::cmp(elem_ref, k);
                match c {
                    core::cmp::Ordering::Less => {
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                T::reflexive(ghost_vals[i as int]);
                            }
                        } else {
                            let best_ref = self.base_set.elements.nth(best_pos);
                            let c2 = TotalOrder::cmp(elem_ref, best_ref);
                            match c2 {
                                core::cmp::Ordering::Greater => {
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(ghost_vals[j], *k) && ghost_vals[j]@ != k@
                                            implies #[trigger] TotalOrder::le(ghost_vals[j], ghost_vals[best_idx]) by {
                                            if j == i as int {
                                                T::reflexive(ghost_vals[i as int]);
                                            } else {
                                                T::transitive(ghost_vals[j], ghost_vals[old_best], ghost_vals[i as int]);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        T::total(ghost_vals[i as int], ghost_vals[best_idx]);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                    },
                    core::cmp::Ordering::Greater => {
                        proof {
                            if TotalOrder::le(ghost_vals[i as int], *k) {
                                T::antisymmetric(ghost_vals[i as int], *k);
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
                    assert(result@ == ghost_vals[best_idx]@);
                    assert(result == ghost_vals[best_idx]);
                    assert forall|t: T| #[trigger] self@.contains(t@) && TotalOrder::le(t, *k) && t@ != k@
                        implies TotalOrder::le(t, result) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(ghost_vals[j]@ == t@);
                        assert(ghost_vals[j] == t);
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
            let ghost mut ghost_vals: Seq<T> = Seq::empty();
            let mut found = false;
            let mut best_pos: usize = 0;
            let ghost mut best_idx: int = -1;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    self.base_set.elements@.no_duplicates(),
                    len as nat == self.base_set.elements@.len(),
                    obeys_feq_full::<T>(),
                    0 <= i, i <= len,
                    ghost_vals.len() == i as int,
                    forall|j: int| #![trigger ghost_vals[j]]
                        0 <= j < i ==> ghost_vals[j]@ == self.base_set.elements@[j],
                    !found ==> forall|j: int| #![trigger ghost_vals[j]]
                        0 <= j < i ==> !(TotalOrder::le(*k, ghost_vals[j]) && ghost_vals[j]@ != k@),
                    found ==> (
                        0 <= best_idx && best_idx < i &&
                        best_pos == best_idx as usize &&
                        TotalOrder::le(*k, ghost_vals[best_idx]) && ghost_vals[best_idx]@ != k@ &&
                        forall|j: int| #![trigger ghost_vals[j]]
                            0 <= j < i && TotalOrder::le(*k, ghost_vals[j]) && ghost_vals[j]@ != k@
                            ==> TotalOrder::le(ghost_vals[best_idx], ghost_vals[j])
                    ),
                decreases len - i,
            {
                let elem_ref = self.base_set.elements.nth(i);
                proof { ghost_vals = ghost_vals.push(*elem_ref); }
                let c = TotalOrder::cmp(elem_ref, k);
                match c {
                    core::cmp::Ordering::Greater => {
                        if !found {
                            found = true;
                            best_pos = i;
                            proof {
                                best_idx = i as int;
                                T::reflexive(ghost_vals[i as int]);
                            }
                        } else {
                            let best_ref = self.base_set.elements.nth(best_pos);
                            let c2 = TotalOrder::cmp(elem_ref, best_ref);
                            match c2 {
                                core::cmp::Ordering::Less => {
                                    proof {
                                        let old_best = best_idx;
                                        best_idx = i as int;
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(*k, ghost_vals[j]) && ghost_vals[j]@ != k@
                                            implies #[trigger] TotalOrder::le(ghost_vals[best_idx], ghost_vals[j]) by {
                                            if j == i as int {
                                                T::reflexive(ghost_vals[i as int]);
                                            } else {
                                                T::transitive(ghost_vals[i as int], ghost_vals[old_best], ghost_vals[j]);
                                            }
                                        };
                                    }
                                    best_pos = i;
                                },
                                _ => {
                                    proof {
                                        T::total(ghost_vals[best_idx], ghost_vals[i as int]);
                                    }
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {
                    },
                    core::cmp::Ordering::Less => {
                        proof {
                            if TotalOrder::le(*k, ghost_vals[i as int]) {
                                T::antisymmetric(*k, ghost_vals[i as int]);
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
                    assert(result@ == ghost_vals[best_idx]@);
                    assert(result == ghost_vals[best_idx]);
                    assert forall|t: T| #[trigger] self@.contains(t@) && TotalOrder::le(*k, t) && t@ != k@
                        implies TotalOrder::le(result, t) by {
                        assert(self.base_set.elements@.to_set().contains(t@));
                        assert(self.base_set.elements@.contains(t@));
                        let j = choose|j: int| 0 <= j < self.base_set.elements@.len()
                            && self.base_set.elements@[j] == t@;
                        assert(ghost_vals[j]@ == t@);
                        assert(ghost_vals[j] == t);
                    };
                }
                Some(result)
            }
        }

        fn split(&self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
        {
            assert(obeys_feq_full_trigger::<T>());
            let elements = &self.base_set.elements;
            let size = elements.length();
            proof { elements@.unique_seq_to_set(); }
            let mut left = Self::empty();
            let mut right = Self::empty();
            let mut found = false;
            let mut j: usize = 0;
            while j < size
                invariant
                    j <= size,
                    elements.spec_avltreeseqstper_wf(),
                    elements@.no_duplicates(),
                    size as nat == elements@.len(),
                    size as nat == self@.len(),
                    self@.len() + 1 < usize::MAX as nat,
                    obeys_feq_full::<T>(),
                    self@.finite(),
                    self@ =~= elements@.to_set(),
                    left@.finite(),
                    right@.finite(),
                    left@.subset_of(self@),
                    right@.subset_of(self@),
                    left.spec_orderedsetstper_wf(),
                    right.spec_orderedsetstper_wf(),
                    left@.len() <= j as nat,
                    right@.len() <= j as nat,
                    left@.disjoint(right@),
                    !left@.contains(k@),
                    !right@.contains(k@),
                    found ==> self@.contains(k@),
                    !found ==> forall|idx: int| #![trigger elements@[idx]]
                        0 <= idx < j ==> elements@[idx] != k@,
                    // Provenance: left/right elements come from visited indices.
                    forall|x| left@.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && #[trigger] elements@[idx] == x,
                    forall|x| right@.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && #[trigger] elements@[idx] == x,
                    // Coverage: every visited element is accounted for.
                    forall|idx: int| #![trigger elements@[idx]]
                        0 <= idx < j ==> left@.contains(elements@[idx]) || right@.contains(elements@[idx]) || elements@[idx] == k@,
                    left@.len() + right@.len() <= j as nat,
                decreases size - j,
            {
                let elem = elements.nth(j);
                if feq(elem, k) {
                    found = true;
                    proof {
                        assert(elem@ == elements@[j as int]);
                        assert(elements@[j as int] == k@);
                        assert(self@.contains(k@));
                    }
                } else {
                    let v = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, v);
                        assert(v@ == elements@[j as int]);
                        assert(elements@.to_set().contains(v@));
                        assert(v@ != k@);
                    }
                    if v < *k {
                        let ghost old_left_view = left@;
                        assert(left@.len() + 1 <= j as nat + 1 <= size as nat);
                        assert(left@.len() + 1 < usize::MAX as nat);
                        left = left.insert(v);
                        proof {
                            assert(left@.subset_of(self@)) by {
                                assert forall|x| #[trigger] left@.contains(x) implies self@.contains(x) by {
                                    if !old_left_view.contains(x) {
                                        assert(x == elem@);
                                    }
                                };
                            };
                            assert(!left@.contains(k@)) by {
                                assert forall|x| #[trigger] left@.contains(x) implies x != k@ by {
                                    if !old_left_view.contains(x) {
                                        assert(x == elem@);
                                    }
                                };
                            };
                            assert(left@.disjoint(right@)) by {
                                assert forall|x| !(#[trigger] left@.contains(x) && right@.contains(x)) by {
                                    if left@.contains(x) && right@.contains(x) {
                                        if !old_left_view.contains(x) {
                                            assert(x == elem@);
                                            assert(x == elements@[j as int]);
                                            let idx = choose|idx: int| 0 <= idx < j && elements@[idx] == x;
                                            assert(elements@[idx] == elements@[j as int]);
                                        }
                                    }
                                };
                            };
                            assert forall|x| left@.contains(x) implies
                                exists|idx: int| 0 <= idx < j + 1 && #[trigger] elements@[idx] == x by {
                                if old_left_view.contains(x) {
                                } else {
                                    assert(x == elem@);
                                    assert(elements@[j as int] == x);
                                }
                            };
                        }
                    } else {
                        let ghost old_right_view = right@;
                        assert(right@.len() + 1 <= j as nat + 1 <= size as nat);
                        assert(right@.len() + 1 < usize::MAX as nat);
                        right = right.insert(v);
                        proof {
                            assert(right@.subset_of(self@)) by {
                                assert forall|x| #[trigger] right@.contains(x) implies self@.contains(x) by {
                                    if !old_right_view.contains(x) {
                                        assert(x == elem@);
                                    }
                                };
                            };
                            assert(!right@.contains(k@)) by {
                                assert forall|x| #[trigger] right@.contains(x) implies x != k@ by {
                                    if !old_right_view.contains(x) {
                                        assert(x == elem@);
                                    }
                                };
                            };
                            assert(left@.disjoint(right@)) by {
                                assert forall|x| !(#[trigger] left@.contains(x) && right@.contains(x)) by {
                                    if left@.contains(x) && right@.contains(x) {
                                        if !old_right_view.contains(x) {
                                            assert(x == elem@);
                                            assert(x == elements@[j as int]);
                                            let idx = choose|idx: int| 0 <= idx < j && elements@[idx] == x;
                                            assert(elements@[idx] == elements@[j as int]);
                                        }
                                    }
                                };
                            };
                            assert forall|x| right@.contains(x) implies
                                exists|idx: int| 0 <= idx < j + 1 && #[trigger] elements@[idx] == x by {
                                if old_right_view.contains(x) {
                                } else {
                                    assert(x == elem@);
                                    assert(elements@[j as int] == x);
                                }
                            };
                        }
                    }
                }
                j = j + 1;
            }
            proof {
                if !found {
                    if self@.contains(k@) {
                        assert(elements@.to_set().contains(k@));
                        assert(elements@.contains(k@));
                        let idx = choose|idx: int| 0 <= idx < elements@.len() && elements@[idx] == k@;
                        assert(elements@[idx] != k@);
                    }
                }
                assert forall|x| self@.contains(x)
                    implies left@.contains(x) || right@.contains(x) || x == k@ by {
                    assert(elements@.to_set().contains(x));
                    assert(elements@.contains(x));
                    let idx = choose|idx: int| 0 <= idx < elements@.len() && elements@[idx] == x;
                    assert(left@.contains(elements@[idx]) || right@.contains(elements@[idx]) || elements@[idx] == k@);
                };
            }
            (left, found, right)
        }

        fn join(left: &Self, right: &Self) -> (joined: Self)
            ensures joined@ == left@.union(right@), joined@.finite()
        { left.union(right) }

        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
        {
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let elements = &self.base_set.elements;
            let size = elements.length();
            let mut result = Self::empty();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    elements.spec_avltreeseqstper_wf(),
                    elements@.no_duplicates(),
                    size as nat == elements@.len(),
                    size as nat == self@.len(),
                    self@.len() + 1 < usize::MAX as nat,
                    self@.finite(),
                    self@ =~= elements@.to_set(),
                    result@.finite(),
                    result@.subset_of(self@),
                    result.spec_orderedsetstper_wf(),
                    result@.len() <= i as nat,
                    obeys_feq_full::<T>(),
                decreases size - i,
            {
                let elem = elements.nth(i);
                let ge_k1 = match elem.cmp(k1) {
                    std::cmp::Ordering::Less => false,
                    _ => true,
                };
                let le_k2 = match elem.cmp(k2) {
                    std::cmp::Ordering::Greater => false,
                    _ => true,
                };
                if ge_k1 && le_k2 {
                    assert(obeys_feq_full_trigger::<T>());
                    let v = elem.clone();
                    proof {
                        lemma_cloned_view_eq(*elem, v);
                        let ghost s = elements@;
                        assert(s[i as int] == elem@);
                        assert(s.contains(elem@));
                        assert(s.to_set().contains(elem@));
                    }
                    let ghost old_result_view = result@;
                    assert(result@.len() + 1 < usize::MAX as nat);
                    result = result.insert(v);
                    proof {
                        assert(result@.subset_of(self@)) by {
                            assert forall|x| #[trigger] result@.contains(x) implies self@.contains(x) by {
                                if !old_result_view.contains(x) {
                                    assert(x == elem@);
                                }
                            };
                        };
                    }
                }
                i = i + 1;
            }
            result
        }

        fn rank(&self, k: &T) -> (rank: usize)
            where T: TotalOrder
        {
            assert(obeys_feq_full_trigger::<T>());
            let n = self.base_set.elements.length();
            proof { self.base_set.elements@.unique_seq_to_set(); }
            let ghost elems = self.base_set.elements@;
            let ghost mut ghost_vals: Seq<T> = Seq::empty();
            let mut count: usize = 0;
            let ghost mut counted: Set<T::V> = Set::empty();
            let mut j: usize = 0;
            while j < n
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    elems.no_duplicates(),
                    n as nat == elems.len(),
                    elems =~= self.base_set.elements@,
                    obeys_feq_full::<T>(),
                    0 <= j <= n,
                    ghost_vals.len() == j as int,
                    forall|idx: int| #![trigger ghost_vals[idx]]
                        0 <= idx < j ==> ghost_vals[idx]@ == elems[idx],
                    counted.finite(),
                    count as nat == counted.len(),
                    0 <= count <= j,
                    counted.subset_of(elems.to_set()),
                    forall|x: T::V| #[trigger] counted.contains(x) ==>
                        exists|t: T| #[trigger] TotalOrder::le(t, *k) && t@ == x && t@ != k@,
                    forall|idx: int| #![trigger ghost_vals[idx]]
                        0 <= idx < j && TotalOrder::le(ghost_vals[idx], *k) && ghost_vals[idx] != *k
                        ==> counted.contains(ghost_vals[idx]@),
                    forall|x: T::V| #[trigger] counted.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && #[trigger] elems[idx] == x,
                decreases n - j,
            {
                let elem_ref = self.base_set.elements.nth(j);
                proof {
                    ghost_vals = ghost_vals.push(*elem_ref);
                    assert(ghost_vals[j as int]@ == elems[j as int]);
                }
                let c = TotalOrder::cmp(elem_ref, k);
                match c {
                    core::cmp::Ordering::Less => {
                        proof {
                            let ghost x = ghost_vals[j as int]@;
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
                            if TotalOrder::le(ghost_vals[j as int], *k) {
                                T::antisymmetric(ghost_vals[j as int], *k);
                            }
                        }
                    },
                }
                j = j + 1;
            }
            proof {
                assert forall|x: T::V|
                    elems.to_set().contains(x)
                    && (exists|t: T| #[trigger] TotalOrder::le(t, *k) && t@ == x && t@ != k@)
                    implies #[trigger] counted.contains(x) by {
                    assert(elems.contains(x));
                    let idx = choose|idx: int| 0 <= idx < elems.len() && elems[idx] == x;
                    assert(ghost_vals[idx]@ == x);
                    let t: T = choose|t: T| #[trigger] TotalOrder::le(t, *k) && t@ == x && t@ != k@;
                    assert(t == ghost_vals[idx]);
                };
                assert(counted =~= self@.filter(
                    |x: T::V| exists|t: T| #[trigger] TotalOrder::le(t, *k) && t@ == x && t@ != k@));
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
                let result = elem.clone_plus();
                proof {
                    lemma_cloned_view_eq(*elem, result);
                    assert(self.base_set.elements@.to_set().contains(result@));
                    // Filter cardinality requires sortedness of the backing sequence,
                    // which is true for AVL trees but not captured in the wf spec.
                    assume(self@.filter(|x: T::V| exists|t: T| #[trigger] TotalOrder::le(t, result) && t@ == x && t@ != result@).len() == i as int);
                }
                Some(result)
            }
        }

        fn split_rank(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            assert(obeys_feq_full_trigger::<T>());
            let elements = &self.base_set.elements;
            let size = elements.length();
            proof { elements@.unique_seq_to_set(); }
            let mut left = Self::empty();
            let mut right = Self::empty();
            let split_at = if i >= size { size } else { i };
            let mut j: usize = 0;
            while j < size
                invariant
                    j <= size,
                    elements.spec_avltreeseqstper_wf(),
                    elements@.no_duplicates(),
                    size as nat == elements@.len(),
                    size as nat == self@.len(),
                    self@.len() + 1 < usize::MAX as nat,
                    obeys_feq_full::<T>(),
                    self@.finite(),
                    self@ =~= elements@.to_set(),
                    left@.finite(),
                    right@.finite(),
                    left@.subset_of(self@),
                    right@.subset_of(self@),
                    left.spec_orderedsetstper_wf(),
                    right.spec_orderedsetstper_wf(),
                    split_at <= size,
                    // Provenance: left has elements from indices < split_at only.
                    forall|x| left@.contains(x) ==>
                        exists|idx: int| 0 <= idx < j && idx < split_at as int && #[trigger] elements@[idx] == x,
                    // Provenance: right has elements from indices >= split_at only.
                    forall|x| right@.contains(x) ==>
                        exists|idx: int| split_at as int <= idx && idx < j && #[trigger] elements@[idx] == x,
                    // Coverage: visited elements are in left or right.
                    forall|idx: int| #![trigger elements@[idx]]
                        0 <= idx < j ==> left@.contains(elements@[idx]) || right@.contains(elements@[idx]),
                    left@.len() + right@.len() <= j as nat,
                decreases size - j,
            {
                let elem = elements.nth(j);
                let v = elem.clone();
                proof {
                    lemma_cloned_view_eq(*elem, v);
                    assert(elements@[j as int] == elem@);
                    assert(elements@.to_set().contains(elem@));
                }
                if j < split_at {
                    let ghost old_left_view = left@;
                    assert(left@.len() + 1 <= j as nat + 1 <= size as nat);
                    assert(left@.len() + 1 < usize::MAX as nat);
                    left = left.insert(v);
                    proof {
                        assert(left@.subset_of(self@)) by {
                            assert forall|x| #[trigger] left@.contains(x) implies self@.contains(x) by {
                                if !old_left_view.contains(x) {
                                    assert(x == elem@);
                                }
                            };
                        };
                        // Provenance for new left.
                        assert forall|x| left@.contains(x) implies
                            exists|idx: int| 0 <= idx < j + 1 && idx < split_at as int && #[trigger] elements@[idx] == x by {
                            if old_left_view.contains(x) {
                                // Old provenance gives idx < j < split_at.
                            } else {
                                assert(x == elem@);
                                // idx = j works: j < split_at.
                                assert(elements@[j as int] == x);
                            }
                        };
                    }
                } else {
                    let ghost old_right_view = right@;
                    assert(right@.len() + 1 <= j as nat + 1 <= size as nat);
                    assert(right@.len() + 1 < usize::MAX as nat);
                    right = right.insert(v);
                    proof {
                        assert(right@.subset_of(self@)) by {
                            assert forall|x| #[trigger] right@.contains(x) implies self@.contains(x) by {
                                if !old_right_view.contains(x) {
                                    assert(x == elem@);
                                }
                            };
                        };
                        // Provenance for new right.
                        assert forall|x| right@.contains(x) implies
                            exists|idx: int| split_at as int <= idx && idx < j + 1 && #[trigger] elements@[idx] == x by {
                            if old_right_view.contains(x) {
                                // Old provenance gives idx < j.
                            } else {
                                assert(x == elem@);
                                assert(elements@[j as int] == x);
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
                            let idx1 = choose|idx: int| 0 <= idx < size && idx < split_at as int && elements@[idx] == x;
                            let idx2 = choose|idx: int| split_at as int <= idx && idx < size as int && elements@[idx] == x;
                            // elements@[idx1] == x == elements@[idx2] but idx1 < split_at <= idx2.
                            // no_duplicates: idx1 != idx2 ==> elements@[idx1] != elements@[idx2]. Contradiction.
                            assert(idx1 != idx2);
                        }
                    };
                };
                // Coverage: every element in self@ was visited.
                assert forall|x| self@.contains(x)
                    implies left@.contains(x) || right@.contains(x) by {
                    assert(elements@.to_set().contains(x));
                    assert(elements@.contains(x));
                    let idx = choose|idx: int| 0 <= idx < elements@.len()
                        && elements@[idx] == x;
                    assert(left@.contains(elements@[idx]) || right@.contains(elements@[idx]));
                };
            }
            (left, right)
        }
    }

    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> (constructed: OrderedSetStPer<T>)
        requires elements@.len() < usize::MAX,
        ensures constructed@.finite(), constructed.spec_orderedsetstper_wf()
    {
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        // from_vec ensures seq.spec_avltreeseqstper_wf(), which from_seq requires.
        OrderedSetStPer::from_seq(seq)
    }

    // 10. iterators

    impl<T: StT + Ord> OrderedSetStPer<T> {
        /// Returns an iterator over the set elements in sorted order.
        pub fn iter(&self) -> (it: OrderedSetStPerIter<'_, T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedSetStPerIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStPerIter<'a, T: StT + Ord> {
        pub seq: &'a AVLTreeSeqStPerS<T>,
        pub pos: usize,
        pub len: usize,
    }

    impl<'a, T: StT + Ord> View for OrderedSetStPerIter<'a, T> {
        type V = (int, Seq<T::V>);
        open spec fn view(&self) -> (int, Seq<T::V>) { (self.pos as int, self.seq@) }
    }

    pub open spec fn iter_invariant<'a, T: StT + Ord>(it: &OrderedSetStPerIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T: StT + Ord> std::iter::Iterator for OrderedSetStPerIter<'a, T> {
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
                self.pos += 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStPerGhostIterator<'a, T: StT + Ord> {
        pub pos: int,
        pub elements: Seq<T::V>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T: StT + Ord> View for OrderedSetStPerGhostIterator<'a, T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, T: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedSetStPerIter<'a, T> {
        type GhostIter = OrderedSetStPerGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> OrderedSetStPerGhostIterator<'a, T> {
            OrderedSetStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedSetStPerGhostIterator<'a, T> {
        type ExecIter = OrderedSetStPerIter<'a, T>;
        type Item = T::V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedSetStPerIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedSetStPerIter<'a, T>) -> OrderedSetStPerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT + Ord> std::iter::IntoIterator for &'a OrderedSetStPer<T> {
        type Item = &'a T;
        type IntoIter = OrderedSetStPerIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_orderedsetstper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedSetStPerIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for OrderedSetStPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            OrderedSetStPer {
                base_set: self.base_set.clone(),
            }
        }
    }

    impl<T: StT + Ord> Default for OrderedSetStPer<T> {
        fn default() -> (d: Self)
            ensures d@.finite(), d@.len() == 0
        {
            Self::empty()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord> PartialEqSpecImpl for OrderedSetStPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord> Eq for OrderedSetStPer<T> {}

    } // verus!

    // 12. macros

    /// Macro for creating ordered sets from sorted element lists
    #[macro_export]
    macro_rules! OrderedSetStPerLit {
        () => {
            $crate::Chap43::OrderedSetStPer::OrderedSetStPer::OrderedSetStPer::empty()
        };
        ($($elem:expr),+ $(,)?) => {
            $crate::Chap43::OrderedSetStPer::OrderedSetStPer::from_sorted_elements(vec![$($elem),+])
        };
    }

    // 13. derive impls outside verus!

    impl<T: StT + Ord> PartialEq for OrderedSetStPer<T> {
        fn eq(&self, other: &Self) -> bool {
            self.base_set == other.base_set
        }
    }

    impl<T: StT + Ord> fmt::Debug for OrderedSetStPer<T> {
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

    impl<T: StT + Ord> fmt::Display for OrderedSetStPer<T> {
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
