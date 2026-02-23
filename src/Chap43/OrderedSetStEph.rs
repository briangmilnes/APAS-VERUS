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
        // Base set operations (ADT 41.1) - ephemeral semantics
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
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@), self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&mut self, f: F)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&mut self, other: &Self)
            ensures self@ == old(self)@.intersect(other@), self@.finite();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&mut self, other: &Self)
            ensures self@ == old(self)@.union(other@), self@.finite();
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&mut self, other: &Self)
            ensures self@ == old(self)@.difference(other@), self@.finite();
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
        fn split(&mut self, k: &T) -> (result: (Self, B, Self))
            where Self: Sized
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log(|self| + |other|)), Span Θ(log(|self| + |other|)), Parallelism Θ(1)
        fn join(&mut self, other: Self)
            ensures self@.finite();
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
        fn split_rank(&mut self, i: usize) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.finite();
    }

    // 9. impls

    impl<T: StT + Ord> OrderedSetStEphTrait<T> for OrderedSetStEph<T> {
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite()
        { self.base_set.size() }

        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty()
        {
            OrderedSetStEph {
                base_set: AVLTreeSetStEph::empty(),
            }
        }

        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@), result@.finite()
        {
            OrderedSetStEph {
                base_set: AVLTreeSetStEph::singleton(x),
            }
        }

        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(x@)
        { self.base_set.find(x) }

        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@), self@.finite()
        { self.base_set.insert(x); }

        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@), self@.finite()
        { self.base_set.delete(x); }

        fn filter<F: PredSt<T>>(&mut self, f: F)
            ensures self@.finite()
        {
            let result = self.base_set.filter(f);
            self.base_set = result;
        }

        fn intersection(&mut self, other: &Self)
            ensures self@ == old(self)@.intersect(other@), self@.finite()
        {
            let result = self.base_set.intersection(&other.base_set);
            self.base_set = result;
        }

        fn union(&mut self, other: &Self)
            ensures self@ == old(self)@.union(other@), self@.finite()
        {
            let result = self.base_set.union(&other.base_set);
            self.base_set = result;
        }

        fn difference(&mut self, other: &Self)
            ensures self@ == old(self)@.difference(other@), self@.finite()
        {
            let result = self.base_set.difference(&other.base_set);
            self.base_set = result;
        }

        #[verifier::external_body]
        fn to_seq(&self) -> (result: AVLTreeSeqStPerS<T>)
            ensures self@.finite()
        {
            let eph_seq = self.base_set.to_seq();
            let len = eph_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(eph_seq.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(elements)
        }

        #[verifier::external_body]
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (result: Self)
            ensures result@.finite()
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
        fn split(&mut self, k: &T) -> (result: (Self, B, Self))
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

            *self = Self::empty();

            (Self::from_seq(left_seq), found, Self::from_seq(right_seq))
        }

        fn join(&mut self, other: Self)
            ensures self@.finite()
        { self.union(&other); }

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
        fn split_rank(&mut self, i: usize) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.finite()
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

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for OrderedSetStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            OrderedSetStEph {
                base_set: self.base_set.clone(),
            }
        }
    }

    #[verifier::external_body]
    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> (result: OrderedSetStEph<T>)
        ensures result@.finite()
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
}
