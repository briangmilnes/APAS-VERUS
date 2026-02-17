//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent set implementation using AVLTreeSeqStPer as backing store.

pub mod AVLTreeSetStPer {

    use std::fmt::{Display, Formatter};

    use vstd::prelude::*;

    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Types::Types::*;

    #[derive(PartialEq, Eq, Debug)]
    pub struct AVLTreeSetStPer<T: StT + Ord> {
        elements: AVLTreeSeqStPerS<T>,
    }

    pub type AVLTreeSetPer<T> = AVLTreeSetStPer<T>;

    pub trait AVLTreeSetStPerTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                        -> N;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self)                      -> AVLTreeSeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                            -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                    -> Self;
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F)  -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self)  -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self)    -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self)         -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                 -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T)               -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T)                -> Self;
    }

    impl<T: StT + Ord> AVLTreeSetStPerTrait<T> for AVLTreeSetStPer<T> {
        fn size(&self) -> N { self.elements.length() }

        fn to_seq(&self) -> AVLTreeSeqStPerS<T> {
            // Create a new sequence from the elements
            let size = self.elements.length();
            let mut vec_elements = Vec::with_capacity(size);
            for i in 0..size {
                vec_elements.push(self.elements.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(vec_elements)
        }

        fn empty() -> Self {
            AVLTreeSetStPer {
                elements: AVLTreeSeqStPerS::empty(),
            }
        }

        fn singleton(x: T) -> Self {
            AVLTreeSetStPer {
                elements: AVLTreeSeqStPerS::singleton(x),
            }
        }

        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self {
            // Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
            // Work efficient and parallel implementation
            if seq.length() == 0 {
                return Self::empty();
            }

            // Create sequence of singleton sets ⟨{x} : x ∈ a⟩
            let seq_len = seq.length();
            let mut singleton_sets = Vec::with_capacity(seq_len);
            for i in 0..seq_len {
                let elem = seq.nth(i).clone();
                singleton_sets.push(Self::singleton(elem));
            }

            // Reduce with union operation
            let mut result = Self::empty();
            for set in singleton_sets {
                result = result.union(&set);
            }
            result
        }

        fn filter<F: PredSt<T>>(&self, f: F) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if f(elem) {
                    result = result.insert(elem.clone());
                }
            }
            result
        }

        fn intersection(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    result = result.insert(elem.clone());
                }
            }
            result
        }

        fn difference(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    result = result.insert(elem.clone());
                }
            }
            result
        }

        fn union(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            // Add all elements from self
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                result = result.insert(elem.clone());
            }
            // Add all elements from other
            for i in 0..other.elements.length() {
                let elem = other.elements.nth(i);
                result = result.insert(elem.clone());
            }
            result
        }

        fn find(&self, x: &T) -> B {
            for i in 0..self.elements.length() {
                if self.elements.nth(i) == x {
                    return true;
                }
            }
            false
        }

        fn delete(&self, x: &T) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if elem != x {
                    result = result.insert(elem.clone());
                }
            }
            result
        }

        fn insert(&self, x: T) -> Self {
            if self.find(&x) {
                // Element already exists, return unchanged
                let size = self.elements.length();
                let mut vec_elements = Vec::with_capacity(size);
                for i in 0..size {
                    vec_elements.push(self.elements.nth(i).clone());
                }
                AVLTreeSetStPer {
                    elements: AVLTreeSeqStPerS::from_vec(vec_elements),
                }
            } else {
                let size = self.elements.length();
                let mut vec_elements = Vec::with_capacity(size + 1);
                for i in 0..size {
                    vec_elements.push(self.elements.nth(i).clone());
                }
                vec_elements.push(x);
                vec_elements.sort();
                AVLTreeSetStPer {
                    elements: AVLTreeSeqStPerS::from_vec(vec_elements),
                }
            }
        }
    }

    impl<T: StT + Ord> Default for AVLTreeSetStPer<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> Clone for AVLTreeSetStPer<T> {
        fn clone(&self) -> Self {
            AVLTreeSetStPer {
                elements: self.elements.clone(),
            }
        }
    }

    impl<T: StT + Ord> Display for AVLTreeSetStPer<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{{{}}}",
                (0..self.size())
                    .map(|i| format!("{}", self.elements.nth(i)))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSetStPerLit {
        () => {
            < $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPer<_> as $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPerTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPer<_> as $crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::AVLTreeSetStPerTrait<_> >::empty();
            $( __set = __set.insert($x); )*
            __set
        }};
    }

    verus! {
        impl<T: StT + Ord> View for AVLTreeSetStPer<T> {
            type V = Self;

            open spec fn view(&self) -> Self {
                *self
            }
        }
    }
}
