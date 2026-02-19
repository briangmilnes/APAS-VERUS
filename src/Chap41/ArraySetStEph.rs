//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral set implementation using ArraySeqStEph as backing store.

pub mod ArraySetStEph {

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
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    pub struct ArraySetStEph<T: StT + Ord> {
        elements: ArraySeqStEphS<T>,
    }

    pub type ArraySetS<T> = ArraySetStEph<T>;

    // 5. view impls

    impl<T: StT + Ord> View for ArraySetStEph<T> {
        type V = Set<T>;
        #[verifier::external_body]
        open spec fn view(&self) -> Set<T> { Set::empty() }
    }

    // 8. traits

    pub trait ArraySetStEphTrait<T: StT + Ord>: Sized + View<V = Set<T>> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: N)
            ensures result == self@.len(), self@.finite();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn to_seq(&self) -> (result: ArraySeqStEphS<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Set::<T>::empty();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<T>::empty().insert(x), result@.finite();
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: ArraySeqStEphS<T>) -> (result: Self)
            ensures result@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@);
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(*x);
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(*x), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x), self@.finite();
    }

    // 9. impls

    impl<T: StT + Ord> ArraySetStEphTrait<T> for ArraySetStEph<T> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.len(), self@.finite()
        { self.elements.length() }

        #[verifier::external_body]
        fn to_seq(&self) -> (result: ArraySeqStEphS<T>)
            ensures self@.finite()
        { self.elements.clone() }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Set::<T>::empty()
        {
            ArraySetStEph {
                elements: ArraySeqStEphS::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<T>::empty().insert(x), result@.finite()
        {
            ArraySetStEph {
                elements: ArraySeqStEphS::singleton(x),
            }
        }

        #[verifier::external_body]
        fn from_seq(seq: ArraySeqStEphS<T>) -> (result: Self)
            ensures result@.finite()
        {
            if seq.length() == 0 {
                return Self::empty();
            }
            let mut result = Self::empty();
            for i in 0..seq.length() {
                let elem = seq.nth(i).clone();
                let singleton_set = Self::singleton(elem);
                result = result.union(&singleton_set);
            }
            result
        }

        #[verifier::external_body]
        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@)
        {
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

        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite()
        {
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

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite()
        {
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

        #[verifier::external_body]
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite()
        {
            let self_len = self.elements.length();
            let other_len = other.elements.length();
            let mut result = Vec::with_capacity(self_len + other_len);

            for i in 0..self_len {
                result.push(self.elements.nth(i).clone());
            }

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

        #[verifier::external_body]
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(*x)
        {
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

        #[verifier::external_body]
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(*x), self@.finite()
        {
            let mut result = Vec::new();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if elem != x {
                    result.push(elem.clone());
                }
            }
            self.elements = ArraySeqStEphS::from_vec(result);
        }

        #[verifier::external_body]
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x), self@.finite()
        {
            if !self.find(&x) {
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

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for ArraySetStEph<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            ArraySetStEph {
                elements: self.elements.clone(),
            }
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StT + Ord> Default for ArraySetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> PartialEq for ArraySetStEph<T> {
        fn eq(&self, other: &Self) -> bool {
            self.size() == other.size() && {
                for i in 0..self.elements.length() {
                    if !other.find(self.elements.nth(i)) {
                        return false;
                    }
                }
                true
            }
        }
    }

    impl<T: StT + Ord> fmt::Debug for ArraySetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
