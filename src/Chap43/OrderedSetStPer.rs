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
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@), result@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(x@);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (result: Self)
            ensures result@ == self@.insert(x@), result@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (result: Self)
            ensures result@ == self@.remove(x@), result@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (result: AVLTreeSeqStPerS<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (result: Self)
            ensures result@.finite();

        // Ordering operations (ADT 43.1)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn first(&self) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn last(&self) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn previous(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn next(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split(&self, k: &T) -> (result: (Self, B, Self))
            where Self: Sized
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|)), Parallelism Θ(1)
        fn join(left: &Self, right: &Self) -> (result: Self)
            ensures result@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn get_range(&self, k1: &T, k2: &T) -> (result: Self)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, k: &T) -> (result: usize)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn select(&self, i: usize) -> (result: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split_rank(&self, i: usize) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.finite();
    }

    // 9. impls

    impl<T: StT + Ord> OrderedSetStPerTrait<T> for OrderedSetStPer<T> {
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite()
        { self.base_set.size() }

        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::empty(),
            }
        }

        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@), result@.finite()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::singleton(x),
            }
        }

        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(x@)
        { self.base_set.find(x) }

        fn insert(&self, x: T) -> (result: Self)
            ensures result@ == self@.insert(x@), result@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.insert(x),
            }
        }

        fn delete(&self, x: &T) -> (result: Self)
            ensures result@ == self@.remove(x@), result@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.delete(x),
            }
        }

        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@)
        {
            OrderedSetStPer {
                base_set: self.base_set.filter(f),
            }
        }

        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.intersection(&other.base_set),
            }
        }

        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.union(&other.base_set),
            }
        }

        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite()
        {
            OrderedSetStPer {
                base_set: self.base_set.difference(&other.base_set),
            }
        }

        fn to_seq(&self) -> (result: AVLTreeSeqStPerS<T>)
            ensures self@.finite()
        { self.base_set.to_seq() }

        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (result: Self)
            ensures result@.finite()
        {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::from_seq(seq),
            }
        }

        #[verifier::external_body]
        fn first(&self) -> (result: Option<T>)
            ensures self@.finite()
        {
            if self.size() == 0 {
                None
            } else {
                let seq = self.to_seq();
                Some(seq.nth(0).clone())
            }
        }

        #[verifier::external_body]
        fn last(&self) -> (result: Option<T>)
            ensures self@.finite()
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
        fn previous(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite()
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
        fn next(&self, k: &T) -> (result: Option<T>)
            ensures self@.finite()
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
        fn split(&self, k: &T) -> (result: (Self, B, Self))
            where Self: Sized
            ensures self@.finite()
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

        fn join(left: &Self, right: &Self) -> (result: Self)
            ensures result@.finite()
        { left.union(right) }

        #[verifier::external_body]
        fn get_range(&self, k1: &T, k2: &T) -> (result: Self)
            ensures self@.finite()
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
        fn rank(&self, k: &T) -> (result: usize)
            ensures self@.finite()
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
        fn select(&self, i: usize) -> (result: Option<T>)
            ensures self@.finite()
        {
            let seq = self.to_seq();
            if i >= seq.length() {
                None
            } else {
                Some(seq.nth(i).clone())
            }
        }

        #[verifier::external_body]
        fn split_rank(&self, i: usize) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.finite()
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
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            OrderedSetStPer {
                base_set: self.base_set.clone(),
            }
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> (result: OrderedSetStPer<T>)
        ensures result@.finite()
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
