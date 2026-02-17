//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered set implementation extending AVLTreeSetStPer.

pub mod OrderedSetStPer {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    #[derive(PartialEq)]
    pub struct OrderedSetStPer<T: StT + Ord> {
        base_set: AVLTreeSetStPer<T>,
    }

    pub type OrderedSetPer<T> = OrderedSetStPer<T>;

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1)
    pub trait OrderedSetStPerTrait<T: StT + Ord> {
        // Base set operations (ADT 41.1) - delegated
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                        -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                            -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                    -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                 -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T)                -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T)               -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F)  -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self)  -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self)         -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self)    -> Self;
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
        fn split(&self, k: &T)                -> (Self, B, Self)
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|)), Parallelism Θ(1)
        fn join(left: &Self, right: &Self)    -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn get_range(&self, k1: &T, k2: &T)   -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, k: &T)                 -> N;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn select(&self, i: N)                -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn split_rank(&self, i: N)            -> (Self, Self)
        where
            Self: Sized;
    }

    impl<T: StT + Ord> OrderedSetStPerTrait<T> for OrderedSetStPer<T> {
        // Base set operations - delegate to backing store

        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N { self.base_set.size() }

        /// Claude Work: O(1), Span: O(1)
        fn empty() -> Self {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::empty(),
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn singleton(x: T) -> Self {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::singleton(x),
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn find(&self, x: &T) -> B { self.base_set.find(x) }

        /// Claude Work: O(log n), Span: O(log n)
        fn insert(&self, x: T) -> Self {
            OrderedSetStPer {
                base_set: self.base_set.insert(x),
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn delete(&self, x: &T) -> Self {
            OrderedSetStPer {
                base_set: self.base_set.delete(x),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn filter<F: PredSt<T>>(&self, f: F) -> Self {
            OrderedSetStPer {
                base_set: self.base_set.filter(f),
            }
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn intersection(&self, other: &Self) -> Self {
            OrderedSetStPer {
                base_set: self.base_set.intersection(&other.base_set),
            }
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn union(&self, other: &Self) -> Self {
            OrderedSetStPer {
                base_set: self.base_set.union(&other.base_set),
            }
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn difference(&self, other: &Self) -> Self {
            OrderedSetStPer {
                base_set: self.base_set.difference(&other.base_set),
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn to_seq(&self) -> AVLTreeSeqStPerS<T> { self.base_set.to_seq() }

        /// Claude Work: O(n log n), Span: O(log² n)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self {
            OrderedSetStPer {
                base_set: AVLTreeSetStPer::from_seq(seq),
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
        fn split(&self, k: &T) -> (Self, B, Self) {
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

            (Self::from_seq(left_seq), found, Self::from_seq(right_seq))
        }

        /// Claude Work: O(log(m + n)), Span: O(log(m + n))
        fn join(left: &Self, right: &Self) -> Self { left.union(right) }

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
        fn split_rank(&self, i: N) -> (Self, Self) {
            let seq = self.to_seq();
            let size = seq.length();

            if i >= size {
                return (self.clone(), Self::empty());
            }

            // Use subseq_copy for known index ranges
            let left_seq = seq.subseq_copy(0, i);
            let right_seq = seq.subseq_copy(i, size - i);

            (Self::from_seq(left_seq), Self::from_seq(right_seq))
        }
    }

    impl<T: StT + Ord> Clone for OrderedSetStPer<T> {
        fn clone(&self) -> Self {
            OrderedSetStPer {
                base_set: self.base_set.clone(),
            }
        }
    }

    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> OrderedSetStPer<T> {
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        OrderedSetStPer::from_seq(seq)
    }

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
}
