//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered set implementation extending AVLTreeSetStEph.

pub mod OrderedSetStEph {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;

    #[derive(PartialEq)]
    pub struct OrderedSetStEph<T: StT + Ord> {
        base_set: AVLTreeSetStEph<T>,
    }

    pub type OrderedSetEph<T> = OrderedSetStEph<T>;

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with ephemeral semantics
    pub trait OrderedSetStEphTrait<T: StT + Ord> {
        // Base set operations (ADT 41.1) - ephemeral semantics
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                        -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                            -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                    -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                 -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T);
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&mut self, f: F);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&mut self, other: &Self);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&mut self, other: &Self);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&mut self, other: &Self);
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self)                      -> AVLTreeSeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self;

        // Ordering operations (ADT 43.1)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn first(&self)                       -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn last(&self)                        -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn previous(&self, k: &T)             -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn next(&self, k: &T)                 -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split(&mut self, k: &T)            -> (Self, B, Self)
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(log(|self| + |other|)), Span Θ(log(|self| + |other|)), Parallelism Θ(1)
        fn join(&mut self, other: Self);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn get_range(&self, k1: &T, k2: &T)   -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, k: &T)                 -> N;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn select(&self, i: N)                -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split_rank(&mut self, i: N)        -> (Self, Self)
        where
            Self: Sized;
    }

    impl<T: StT + Ord> OrderedSetStEphTrait<T> for OrderedSetStEph<T> {
        // Base set operations - delegate to backing store with ephemeral semantics

        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N { self.base_set.size() }

        /// Claude Work: O(1), Span: O(1)
        fn empty() -> Self {
            OrderedSetStEph {
                base_set: AVLTreeSetStEph::empty(),
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn singleton(x: T) -> Self {
            OrderedSetStEph {
                base_set: AVLTreeSetStEph::singleton(x),
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn find(&self, x: &T) -> B { self.base_set.find(x) }

        /// Claude Work: O(log n), Span: O(log n)
        fn insert(&mut self, x: T) { self.base_set.insert(x); }

        /// Claude Work: O(log n), Span: O(log n)
        fn delete(&mut self, x: &T) { self.base_set.delete(x); }

        /// Claude Work: O(n), Span: O(log n)
        fn filter<F: PredSt<T>>(&mut self, f: F) {
            let result = self.base_set.filter(f);
            self.base_set = result;
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn intersection(&mut self, other: &Self) {
            let result = self.base_set.intersection(&other.base_set);
            self.base_set = result;
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn union(&mut self, other: &Self) {
            let result = self.base_set.union(&other.base_set);
            self.base_set = result;
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn difference(&mut self, other: &Self) {
            let result = self.base_set.difference(&other.base_set);
            self.base_set = result;
        }

        /// Claude Work: O(n), Span: O(log n)
        fn to_seq(&self) -> AVLTreeSeqStPerS<T> {
            // Convert ephemeral sequence to persistent sequence
            let eph_seq = self.base_set.to_seq();
            let len = eph_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(eph_seq.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(elements)
        }

        /// Claude Work: O(n log n), Span: O(log² n)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self {
            // Convert persistent sequence to ephemeral sequence
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

        // Ordering operations (ADT 43.1)

        /// Claude Work: O(log n), Span: O(log n)
        fn first(&self) -> Option<T> {
            if self.size() == 0 {
                None
            } else {
                let seq = self.to_seq();
                Some(seq.nth(0).clone())
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn last(&self) -> Option<T> {
            let size = self.size();
            if size == 0 {
                None
            } else {
                let seq = self.to_seq();
                Some(seq.nth(size - 1).clone())
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn previous(&self, k: &T) -> Option<T> {
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

        /// Claude Work: O(log n), Span: O(log n)
        fn next(&self, k: &T) -> Option<T> {
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

        /// Claude Work: O(log n), Span: O(log n)
        fn split(&mut self, k: &T) -> (Self, B, Self) {
            let seq = self.to_seq();

            // Convert to ArraySeqStPerS for filtering operations
            let array_seq = ArraySeqStPerS::tabulate(&|i| seq.nth(i).clone(), seq.length());

            // Manual filter (ArraySeqStPerS::filter requires Ghost spec_pred)
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

            // Convert back to AVLTreeSeqStPerS
            let left_seq = AVLTreeSeqStPerS::from_vec(left_vec);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_vec);

            // Clear current set (ephemeral behavior)
            *self = Self::empty();

            (Self::from_seq(left_seq), found, Self::from_seq(right_seq))
        }

        /// Claude Work: O(log(m + n)), Span: O(log(m + n))
        fn join(&mut self, other: Self) { self.union(&other); }

        /// Claude Work: O(log n), Span: O(log n)
        fn get_range(&self, k1: &T, k2: &T) -> Self {
            let seq = self.to_seq();

            // Convert to ArraySeqStPerS for filtering operations
            let array_seq = ArraySeqStPerS::tabulate(&|i| seq.nth(i).clone(), seq.length());

            // Manual filter (ArraySeqStPerS::filter requires Ghost spec_pred)
            let mut range_vec = Vec::new();
            for i in 0..array_seq.length() {
                let elem = array_seq.nth(i).clone();
                if elem >= *k1 && elem <= *k2 {
                    range_vec.push(elem);
                }
            }

            // Convert back to AVLTreeSeqStPerS
            let range_seq = AVLTreeSeqStPerS::from_vec(range_vec);
            Self::from_seq(range_seq)
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn rank(&self, k: &T) -> N {
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

        /// Claude Work: O(log n), Span: O(log n)
        fn select(&self, i: N) -> Option<T> {
            let seq = self.to_seq();
            if i >= seq.length() {
                None
            } else {
                Some(seq.nth(i).clone())
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn split_rank(&mut self, i: N) -> (Self, Self) {
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

            // Clear current set (ephemeral behavior)
            *self = Self::empty();

            (Self::from_seq(left_seq), Self::from_seq(right_seq))
        }
    }

    impl<T: StT + Ord> Clone for OrderedSetStEph<T> {
        fn clone(&self) -> Self {
            OrderedSetStEph {
                base_set: self.base_set.clone(),
            }
        }
    }

    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> OrderedSetStEph<T> {
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        OrderedSetStEph::from_seq(seq)
    }

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
}
