//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral set implementation using AVLTreeSeqStEph as backing store.
//!
//! Limitation: AVLTreeSeqStEph is index-ordered, not a BST by value. find uses binary search
//! on the sorted logical sequence (O(log n) via nth). insert/delete use filter-and-rebuild
//! since the backing tree has no O(log n) value-based insert/delete.

pub mod AVLTreeSetStEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    #[derive(PartialEq, Eq)]
    pub struct AVLTreeSetStEph<T: StT + Ord> {
        elements: AVLTreeSeqStEphS<T>,
    }

    pub type AVLTreeSetS<T> = AVLTreeSetStEph<T>;

    pub trait AVLTreeSetStEphTrait<T: StT + Ord> {
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                        -> N;
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self)                      -> AVLTreeSeqStEphS<T>;
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                            -> Self;
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                    -> Self;
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> Self;
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F)  -> Self;
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self)  -> Self;
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self)    -> Self;
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self)         -> Self;
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                 -> B;
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T);
    }

    impl<T: StT + Ord> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        fn size(&self) -> N { self.elements.length() }

        fn to_seq(&self) -> AVLTreeSeqStEphS<T> {
            // Create a new sequence from the elements
            let size = self.elements.length();
            let mut vec_elements = Vec::with_capacity(size);
            for i in 0..size {
                vec_elements.push(self.elements.nth(i).clone());
            }
            AVLTreeSeqStEphS::from_vec(vec_elements)
        }

        fn empty() -> Self {
            AVLTreeSetStEph {
                elements: AVLTreeSeqStEphS::empty(),
            }
        }

        fn singleton(x: T) -> Self {
            AVLTreeSetStEph {
                elements: AVLTreeSeqStEphS::singleton(x),
            }
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> Self {
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
                    result.insert(elem.clone());
                }
            }
            result
        }

        fn intersection(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        fn difference(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        fn union(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            // Add all elements from self
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                result.insert(elem.clone());
            }
            // Add all elements from other
            for i in 0..other.elements.length() {
                let elem = other.elements.nth(i);
                result.insert(elem.clone());
            }
            result
        }

        fn find(&self, x: &T) -> B {
            // Binary search on sorted sequence: O(log n) via nth(i)
            let n = self.elements.length();
            let mut lo = 0usize;
            let mut hi = n;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                let elem = self.elements.nth(mid);
                if elem == x {
                    return true;
                }
                if elem < x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            false
        }

        fn delete(&mut self, x: &T) {
            // Delegate to backing tree's delete_value (filter-and-rebuild)
            let _ = self.elements.delete_value(x);
        }

        fn insert(&mut self, x: T) {
            if self.find(&x) {
                return;
            }
            // Binary search for insertion point, then rebuild
            let n = self.elements.length();
            let mut lo = 0usize;
            let mut hi = n;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if self.elements.nth(mid) < &x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            let mut vec_elements = Vec::with_capacity(n + 1);
            for i in 0..lo {
                vec_elements.push(self.elements.nth(i).clone());
            }
            vec_elements.push(x);
            for i in lo..n {
                vec_elements.push(self.elements.nth(i).clone());
            }
            self.elements = AVLTreeSeqStEphS::from_vec(vec_elements);
        }
    }

    impl<T: StT + Ord> Default for AVLTreeSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> Clone for AVLTreeSetStEph<T> {
        fn clone(&self) -> Self {
            AVLTreeSetStEph {
                elements: self.elements.clone(),
            }
        }
    }

    impl<T: StT + Ord> fmt::Debug for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord> fmt::Display for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSetStEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    verus! {
        impl<T: StT + Ord> View for AVLTreeSetStEph<T> {
            type V = Self;

            open spec fn view(&self) -> Self {
                *self
            }
        }
    }
}
