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
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

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
        // Base set operations (ADT 41.1) - delegated
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            ensures found == self@.contains(x@);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (updated: Self)
            ensures updated@ == self@.insert(x@), updated@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (updated: Self)
            ensures updated@ == self@.remove(x@), updated@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
            ensures filtered@.finite(), filtered@.subset_of(self@);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            ensures constructed@.finite();

        // Ordering operations (ADT 43.1)
        /// ADT 43.1 first(A) = min[|A|]. Work Θ(log n), Span Θ(log n).
        fn first(&self) -> (first: Option<T>)
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@);
        /// ADT 43.1 last(A) = max[|A|]. Work Θ(log n), Span Θ(log n).
        fn last(&self) -> (last: Option<T>)
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
        fn split(&self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
            ensures
                self@.finite(),
                split.1 == self@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(self@),
                split.2@.subset_of(self@),
                split.0@.disjoint(split.2@);
        /// ADT 43.1 join(A1, A2) = A1 union A2. Work Θ(log(|left|+|right|)), Span Θ(log(|left|+|right|)).
        fn join(left: &Self, right: &Self) -> (joined: Self)
            ensures joined@ == left@.union(right@), joined@.finite();
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
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@);
        /// ADT 43.1 splitRank(A, i). Work Θ(log n), Span Θ(log n).
        fn split_rank(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures
                self@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(self@),
                split.1@.subset_of(self@);
    }

    // 9. impls

    impl<T: StT + Ord> OrderedSetStPerTrait<T> for OrderedSetStPer<T> {
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite()
        { self.base_set.size() }

        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::empty(),
            }
        }

        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree@.finite()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::singleton(x),
            }
        }

        fn find(&self, x: &T) -> (found: B)
            ensures found == self@.contains(x@)
        { self.base_set.find(x) }

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

        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
            ensures filtered@.finite(), filtered@.subset_of(self@)
        {
            OrderedSetStPer {
                base_set: self.base_set.filter(f),
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
            ensures self@.finite()
        { self.base_set.to_seq() }

        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            ensures constructed@.finite()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::from_seq(seq),
            }
        }

        #[verifier::external_body]
        fn first(&self) -> (first: Option<T>)
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
        {
            if self.size() == 0 {
                None
            } else {
                let seq = self.to_seq();
                Some(seq.nth(0).clone())
            }
        }

        #[verifier::external_body]
        fn last(&self) -> (last: Option<T>)
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
        {
            let size = self.size();
            if size == 0 {
                None
            } else {
                let seq = self.to_seq();
                Some(seq.nth(size - 1).clone())
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
        fn split(&self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
            ensures
                self@.finite(),
                split.1 == self@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(self@),
                split.2@.subset_of(self@),
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

            (Self::from_seq(left_seq), found, Self::from_seq(right_seq))
        }

        fn join(left: &Self, right: &Self) -> (joined: Self)
            ensures joined@ == left@.union(right@), joined@.finite()
        { left.union(right) }

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

        #[verifier::external_body]
        fn select(&self, i: usize) -> (selected: Option<T>)
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@),
        {
            let seq = self.to_seq();
            if i >= seq.length() {
                None
            } else {
                Some(seq.nth(i).clone())
            }
        }

        #[verifier::external_body]
        fn split_rank(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures
                self@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(self@),
                split.1@.subset_of(self@),
        {
            let seq = self.to_seq();
            let size = seq.length();

            if i >= size {
                return (self.clone(), Self::empty());
            }

            let left_seq = seq.subseq_copy(0, i);
            let right_seq = seq.subseq_copy(i, size - i);

            (Self::from_seq(left_seq), Self::from_seq(right_seq))
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

    #[verifier::external_body]
    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> (constructed: OrderedSetStPer<T>)
        ensures constructed@.finite()
    {
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        OrderedSetStPer::from_seq(seq)
    }

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

    impl<T: StT + Ord> Default for OrderedSetStPer<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> PartialEq for OrderedSetStPer<T> {
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
}
