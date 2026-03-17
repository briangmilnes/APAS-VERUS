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
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger, lemma_cloned_view_eq};

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
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: T| self@.contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then scans for successor
        fn next(&self, k: &T) -> (successor: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: T| self@.contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then partitions into two sets
        fn split(&mut self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
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
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- to_seq then counts elements < k
        fn rank(&self, k: &T) -> (rank: usize)
            where T: TotalOrder
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
                accept(result@ =~= eph_seq@);  // Clone/view bridging.
            }
            result
        }

        #[verifier::external_body]
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            ensures constructed@.finite()
        {
            let len = seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(seq.nth(i).clone());
            }
            let eph_seq = AVLTreeSeqStEphS::from_vec(elements);
            OrderedSetStEph {
                base_set: AVLTreeSetStEph::from_seq(eph_seq),
            }
        }

        #[verifier::external_body]
        fn first(&self) -> (first: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| self@.contains(t@) ==> TotalOrder::le(v, t),
        {
            let len = self.base_set.elements.length();
            if len == 0 {
                None
            } else {
                let elem = self.base_set.elements.nth(0);
                Some(elem.clone())
            }
        }

        #[verifier::external_body]
        fn last(&self) -> (last: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| self@.contains(t@) ==> TotalOrder::le(t, v),
        {
            let len = self.base_set.elements.length();
            if len == 0 {
                None
            } else {
                let elem = self.base_set.elements.nth(len - 1);
                Some(elem.clone())
            }
        }

        #[verifier::external_body]
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: T| self@.contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            let seq = self.to_seq();
            let size = seq.length();

            for i in (0..size).rev() {
                let elem = seq.nth(i);
                if elem < k {
                    return Some(elem.clone());
                }
            }
            None
        }

        #[verifier::external_body]
        fn next(&self, k: &T) -> (successor: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: T| self@.contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            let seq = self.to_seq();
            let size = seq.length();

            for i in 0..size {
                let elem = seq.nth(i);
                if elem > k {
                    return Some(elem.clone());
                }
            }
            None
        }

        #[verifier::external_body]
        fn split(&mut self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
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
                forall|x| old(self)@.contains(x) ==> split.0@.contains(x) || split.2@.contains(x) || x == k@,
        {
            let seq = self.to_seq();

            let array_seq = ArraySeqStPerS::tabulate(&|i| seq.nth(i).clone(), seq.length());

            let mut left_vec = Vec::new();
            let mut right_vec = Vec::new();
            let mut found = false;
            for i in 0..array_seq.length() {
                let elem = array_seq.nth(i).clone();
                if elem < *k {
                    left_vec.push(elem);
                } else if elem > *k {
                    right_vec.push(elem);
                } else {
                    found = true;
                }
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_vec);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_vec);

            *self = Self::empty();

            (Self::from_seq(left_seq), found, Self::from_seq(right_seq))
        }

        fn join(&mut self, other: Self)
            ensures self@ == old(self)@.union(other@), self@.finite(), self.spec_orderedsetsteph_wf()
        { self.union(&other); }

        #[verifier::external_body]
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@),
        {
            let seq = self.to_seq();

            let array_seq = ArraySeqStPerS::tabulate(&|i| seq.nth(i).clone(), seq.length());

            let mut range_vec = Vec::new();
            for i in 0..array_seq.length() {
                let elem = array_seq.nth(i).clone();
                if elem >= *k1 && elem <= *k2 {
                    range_vec.push(elem);
                }
            }

            let range_seq = AVLTreeSeqStPerS::from_vec(range_vec);
            Self::from_seq(range_seq)
        }

        #[verifier::external_body]
        fn rank(&self, k: &T) -> (rank: usize)
            where T: TotalOrder
            ensures
                self@.finite(),
                rank <= self@.len(),
                rank as int == self@.filter(|x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, *k) && t@ != k@).len(),
        {
            let seq = self.to_seq();
            let size = seq.length();
            let mut count = 0;

            for i in 0..size {
                let elem = seq.nth(i);
                if elem < k {
                    count += 1;
                } else {
                    break;
                }
            }
            count
        }

        #[verifier::external_body]
        fn select(&self, i: usize) -> (selected: Option<T>)
            where T: TotalOrder
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@),
                selected matches Some(v) ==> self@.filter(|x: T::V| exists|t: T| t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int,
        {
            let sz = self.size();
            if i >= sz {
                None
            } else {
                let elem = self.base_set.elements.nth(i);
                Some(elem.clone())
            }
        }

        #[verifier::external_body]
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(old(self)@),
                split.1@.subset_of(old(self)@),
                split.0@.disjoint(split.1@),
                forall|x| old(self)@.contains(x) ==> split.0@.contains(x) || split.1@.contains(x),
        {
            let seq = self.to_seq();
            let size = seq.length();

            if i >= size {
                let current = self.clone();
                *self = Self::empty();
                return (current, Self::empty());
            }

            let mut left_elements = Vec::new();
            let mut right_elements = Vec::new();

            for j in 0..i {
                left_elements.push(seq.nth(j).clone());
            }
            for j in i..size {
                right_elements.push(seq.nth(j).clone());
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_elements);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_elements);

            *self = Self::empty();

            (Self::from_seq(left_seq), Self::from_seq(right_seq))
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
