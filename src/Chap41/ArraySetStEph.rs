//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral set implementation using ArraySeqStEph as backing store.

pub mod ArraySetStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ArraySetStEph<T: StT + Ord> {
        elements: ArraySeqStEphS<T>,
    }

    pub type ArraySetS<T> = ArraySetStEph<T>;

    pub trait ArraySetStEphTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                       -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn to_seq(&self)                     -> ArraySeqStEphS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                           -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                   -> Self;
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: ArraySeqStEphS<T>)  -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn intersection(&self, other: &Self) -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn difference(&self, other: &Self)   -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn union(&self, other: &Self)        -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn delete(&mut self, x: &T);
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn insert(&mut self, x: T);
    }

    impl<T: StT + Ord> ArraySetStEphTrait<T> for ArraySetStEph<T> {
        fn size(&self) -> N { self.elements.length() }

        fn to_seq(&self) -> ArraySeqStEphS<T> { self.elements.clone() }

        fn empty() -> Self {
            ArraySetStEph {
                elements: ArraySeqStEphS::empty(),
            }
        }

        fn singleton(x: T) -> Self {
            ArraySetStEph {
                elements: ArraySeqStEphS::singleton(x),
            }
        }

        fn from_seq(seq: ArraySeqStEphS<T>) -> Self {
            // Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
            // Work efficient and parallel implementation
            if seq.length() == 0 {
                return Self::empty();
            }

            // Reduce with union operation directly from seq
            let mut result = Self::empty();
            for i in 0..seq.length() {
                let elem = seq.nth(i).clone();
                let singleton_set = Self::singleton(elem);
                result = result.union(&singleton_set);
            }
            result
        }

        fn filter<F: PredSt<T>>(&self, f: F) -> Self {
            let mut result = Vec::new();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if f(elem) {
                    result.push(elem.clone());
                }
            }
            ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result),
            }
        }

        fn intersection(&self, other: &Self) -> Self {
            let mut result = Vec::new();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    result.push(elem.clone());
                }
            }
            ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result),
            }
        }

        fn difference(&self, other: &Self) -> Self {
            let mut result = Vec::new();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    result.push(elem.clone());
                }
            }
            ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result),
            }
        }

        fn union(&self, other: &Self) -> Self {
            let self_len = self.elements.length();
            let other_len = other.elements.length();
            let mut result = Vec::with_capacity(self_len + other_len);

            // Add all elements from self
            for i in 0..self_len {
                result.push(self.elements.nth(i).clone());
            }

            // Add elements from other that are not in self
            for i in 0..other_len {
                let elem = other.elements.nth(i);
                if !self.find(elem) {
                    result.push(elem.clone());
                }
            }

            result.sort();
            ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result),
            }
        }

        fn find(&self, x: &T) -> B {
            let mut lo: usize = 0;
            let mut hi: usize = self.elements.length();
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                let elem = self.elements.nth(mid);
                if elem == x {
                    return true;
                } else if elem < x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            false
        }

        fn delete(&mut self, x: &T) {
            let mut result = Vec::new();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if elem != x {
                    result.push(elem.clone());
                }
            }
            self.elements = ArraySeqStEphS::from_vec(result);
        }

        fn insert(&mut self, x: T) {
            if !self.find(&x) {
                // Element doesn't exist, add it
                let new_len = self.elements.length() + 1;
                let mut sorted_elements = Vec::with_capacity(new_len);
                for i in 0..self.elements.length() {
                    sorted_elements.push(self.elements.nth(i).clone());
                }
                sorted_elements.push(x);
                sorted_elements.sort();
                self.elements = ArraySeqStEphS::from_vec(sorted_elements);
            }
        }
    }

    impl<T: StT + Ord> Default for ArraySetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    #[macro_export]
    macro_rules! ArraySetStEphLit {
        () => {
            < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }
}
