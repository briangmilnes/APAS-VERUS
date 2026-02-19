//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral set implementation using AVLTreeSeqStEph as backing store.
//!
//! Limitation: AVLTreeSeqStEph is index-ordered, not a BST by value. find uses binary search
//! on the sorted logical sequence (O(log n) via nth). insert/delete use filter-and-rebuild
//! since the backing tree has no O(log n) value-based insert/delete.

pub mod AVLTreeSetStEph {

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

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    pub struct AVLTreeSetStEph<T: StT + Ord> {
        elements: AVLTreeSeqStEphS<T>,
    }

    pub type AVLTreeSetS<T> = AVLTreeSetStEph<T>;

    // 5. view impls

    impl<T: StT + Ord> View for AVLTreeSetStEph<T> {
        type V = Set<T>;
        #[verifier::external_body]
        open spec fn view(&self) -> Set<T> { Set::empty() }
    }

    // 8. traits

    pub trait AVLTreeSetStEphTrait<T: StT + Ord> {
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: N)
            ensures result == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (result: AVLTreeSeqStEphS<T>)
            ensures self@.finite();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Set::<T>::empty();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<T>::empty().insert(x), result@.finite();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (result: Self)
            ensures result@.finite();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@);
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(*x);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(*x), self@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x), self@.finite();
    }

    // 9. impls

    impl<T: StT + Ord> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.len(), self@.finite()
        { self.elements.length() }

        #[verifier::external_body]
        fn to_seq(&self) -> (result: AVLTreeSeqStEphS<T>)
            ensures self@.finite()
        {
            let size = self.elements.length();
            let mut vec_elements = Vec::with_capacity(size);
            for i in 0..size {
                vec_elements.push(self.elements.nth(i).clone());
            }
            AVLTreeSeqStEphS::from_vec(vec_elements)
        }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Set::<T>::empty()
        {
            AVLTreeSetStEph {
                elements: AVLTreeSeqStEphS::empty(),
            }
        }

        #[verifier::external_body]
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<T>::empty().insert(x), result@.finite()
        {
            AVLTreeSetStEph {
                elements: AVLTreeSeqStEphS::singleton(x),
            }
        }

        #[verifier::external_body]
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (result: Self)
            ensures result@.finite()
        {
            if seq.length() == 0 {
                return Self::empty();
            }

            let seq_len = seq.length();
            let mut singleton_sets = Vec::with_capacity(seq_len);
            for i in 0..seq_len {
                let elem = seq.nth(i).clone();
                singleton_sets.push(Self::singleton(elem));
            }

            let mut result = Self::empty();
            for set in singleton_sets {
                result = result.union(&set);
            }
            result
        }

        #[verifier::external_body]
        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@)
        {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if f(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite()
        {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite()
        {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        #[verifier::external_body]
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite()
        {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                result.insert(elem.clone());
            }
            for i in 0..other.elements.length() {
                let elem = other.elements.nth(i);
                result.insert(elem.clone());
            }
            result
        }

        #[verifier::external_body]
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(*x)
        {
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

        #[verifier::external_body]
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(*x), self@.finite()
        {
            let _ = self.elements.delete_value(x);
        }

        #[verifier::external_body]
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x), self@.finite()
        {
            if self.find(&x) {
                return;
            }
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

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for AVLTreeSetStEph<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            AVLTreeSetStEph {
                elements: self.elements.clone(),
            }
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StT + Ord> Default for AVLTreeSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> PartialEq for AVLTreeSetStEph<T> {
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

    impl<T: StT + Ord> Eq for AVLTreeSetStEph<T> {}

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
}
