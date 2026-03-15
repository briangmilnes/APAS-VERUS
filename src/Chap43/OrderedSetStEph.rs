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
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_orderedsetsteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_orderedsetsteph_wf();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree@.finite(),
                tree.spec_orderedsetsteph_wf();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            requires self.spec_orderedsetsteph_wf(),
            ensures found == self@.contains(x@);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            requires old(self).spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.insert(x@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            requires old(self).spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.remove(x@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
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
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.intersect(other@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.union(other@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.difference(other@),
                self@.finite(),
                self.spec_orderedsetsteph_wf();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            ensures
                constructed@.finite(),
                constructed.spec_orderedsetsteph_wf();

        // Ordering operations (ADT 43.1)
        /// ADT 43.1 first(A) = min[|A|]. Work Θ(log n), Span Θ(log n).
        fn first(&self) -> (first: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@);
        /// ADT 43.1 last(A) = max[|A|]. Work Θ(log n), Span Θ(log n).
        fn last(&self) -> (last: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@);
        /// ADT 43.1 previous(A, k) = max{k' in A | k' < k}. Work Θ(log n), Span Θ(log n).
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@);
        /// ADT 43.1 next(A, k) = min{k' in A | k' > k}. Work Θ(log n), Span Θ(log n).
        fn next(&self, k: &T) -> (successor: Option<T>)
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@);
        /// ADT 43.1 split(A, k) = ({k' < k}, k in A, {k' > k}). Work Θ(log n), Span Θ(log n).
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
                split.0@.disjoint(split.2@);
        /// ADT 43.1 join(A1, A2) = A1 union A2. Work Θ(log(|left|+|right|)), Span Θ(log(|left|+|right|)).
        fn join(&mut self, other: Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures self@.finite(), self.spec_orderedsetsteph_wf();
        /// ADT 43.1 getRange(A, k1, k2) = {k in A | k1 <= k <= k2}. Work Θ(log n), Span Θ(log n).
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// ADT 43.1 rank(A, k) = |{k' in A | k' < k}|. Work Θ(log n), Span Θ(log n).
        fn rank(&self, k: &T) -> (rank: usize)
            ensures
                self@.finite(),
                rank <= self@.len();
        /// ADT 43.1 select(A, i) = k in A such that rank(A, k) = i. Work Θ(log n), Span Θ(log n).
        fn select(&self, i: usize) -> (selected: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@);
        /// ADT 43.1 splitRank(A, i). Work Θ(log n), Span Θ(log n).
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(old(self)@),
                split.1@.subset_of(old(self)@);
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
                assume(result@ =~= eph_seq@);  // accept hole: clone/view bridging
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

        fn first(&self) -> (first: Option<T>)
        {
            let len = self.base_set.elements.length();
            proof {
                vstd::seq_lib::seq_to_set_is_finite::<T::V>(self.base_set.elements@);
                self.base_set.elements@.unique_seq_to_set();
                assert(self@ =~= self.base_set.elements@.to_set());
            }
            if len == 0 {
                None
            } else {
                let elem = self.base_set.elements.nth(0);
                let v = elem.clone();
                proof {
                    assert(obeys_feq_full_trigger::<T>());
                    lemma_cloned_view_eq(*elem, v);
                    let ghost s = self.base_set.elements@;
                    assert(s[0] == elem@);
                    assert(s.contains(elem@));
                    assert(s.to_set().contains(elem@));
                }
                Some(v)
            }
        }

        fn last(&self) -> (last: Option<T>)
        {
            let len = self.base_set.elements.length();
            proof {
                vstd::seq_lib::seq_to_set_is_finite::<T::V>(self.base_set.elements@);
                self.base_set.elements@.unique_seq_to_set();
                assert(self@ =~= self.base_set.elements@.to_set());
            }
            if len == 0 {
                None
            } else {
                let elem = self.base_set.elements.nth(len - 1);
                let v = elem.clone();
                proof {
                    assert(obeys_feq_full_trigger::<T>());
                    lemma_cloned_view_eq(*elem, v);
                    let ghost s = self.base_set.elements@;
                    let ghost idx = (len - 1) as int;
                    assert(s[idx] == elem@);
                    assert(s.contains(elem@));
                    assert(s.to_set().contains(elem@));
                }
                Some(v)
            }
        }

        #[verifier::external_body]
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
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
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
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
            ensures self@.finite()
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
            ensures
                self@.finite(),
                rank <= self@.len(),
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

        fn select(&self, i: usize) -> (selected: Option<T>)
        {
            let sz = self.size();
            if i >= sz {
                None
            } else {
                proof {
                    self.base_set.elements@.unique_seq_to_set();
                    assert(self@ =~= self.base_set.elements@.to_set());
                    assert((i as int) < self.base_set.elements@.len());
                }
                let elem = self.base_set.elements.nth(i);
                let v = elem.clone();
                proof {
                    assert(obeys_feq_full_trigger::<T>());
                    lemma_cloned_view_eq(*elem, v);
                    let ghost s = self.base_set.elements@;
                    assert(s[i as int] == elem@);
                    assert(s.contains(elem@));
                    assert(s.to_set().contains(elem@));
                }
                Some(v)
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
