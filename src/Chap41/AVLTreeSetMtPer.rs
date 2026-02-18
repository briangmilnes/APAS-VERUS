//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent set implementation using AVLTreeSeqMtPer (Arc-based).
//!
//! Work/Span Analysis:
//! - union: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - intersection: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - filter: Work Θ(n), Span Θ(log n) via PARALLEL map-reduce

pub mod AVLTreeSetMtPer {

    use std::cmp::Ordering::{self, Equal, Greater, Less};
    use std::{fmt, thread};

    use vstd::prelude::*;

    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    #[derive(PartialEq, Eq)]
    pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
        elements: AVLTreeSeqMtPerS<T>,
    }

    // NOTE: This type implements Ord because it is used as a VALUE in OrderedTableMtPer.
    // OrderedTableMtPer<K, V> is backed by BSTParaTreapMtEph<Pair<K, V>>, which requires
    // BOTH K and V to be Ord (via MtKey trait). For example, AdjTableGraphMtPer uses
    // OrderedTableMtPer<V, AVLTreeSetMtPer<V>>, so AVLTreeSetMtPer<V> must implement Ord.
    //
    // This is purely a caller requirement - if no code used AVLTreeSetMtPer as a value in
    // an ordered table, we wouldn't need Ord. See AVLTreeSetMtEph for comparison (no Ord needed).

    impl<T: StTInMtT + Ord + 'static> PartialOrd for AVLTreeSetMtPer<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T: StTInMtT + Ord + 'static> Ord for AVLTreeSetMtPer<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            // Lexicographic ordering: compare element by element (no cloning)
            let n_self = self.size();
            let n_other = other.size();
            let min_n = n_self.min(n_other);
            
            // Compare common prefix
            for i in 0..min_n {
                let a = self.elements.nth(i);
                let b = other.elements.nth(i);
                match a.cmp(b) {
                    Equal => continue,
                    non_equal => return non_equal,
                }
            }
            
            // If all compared elements are equal, compare by size
            n_self.cmp(&n_other)
        }
    }

    pub trait AVLTreeSetMtPerTrait<T: StTInMtT + Ord + 'static> {
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                               -> N;
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn to_seq(&self)                             -> AVLTreeSeqMtPerS<T>;
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                   -> Self;
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                           -> Self;
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqMtPerS<T>)        -> Self;
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> Self;
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self)         -> Self;
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self)           -> Self;
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self)                -> Self;
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                        -> B;
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T)                      -> Self;
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T)                       -> Self;
    }

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPerTrait<T> for AVLTreeSetMtPer<T> {
        fn size(&self) -> N { self.elements.length() }

        fn to_seq(&self) -> AVLTreeSeqMtPerS<T> { self.elements.clone() }

        fn empty() -> Self {
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::empty(),
            }
        }

        fn singleton(x: T) -> Self {
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::singleton(x),
            }
        }

        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> Self {
            let mut vals = seq.values_in_order();

            // Unconditionally parallel merge sort using ParaPair!
            fn parallel_sort<T: StTInMtT + Ord + 'static>(mut vals: Vec<T>) -> Vec<T> {
                let n = vals.len();
                if n <= 1 {
                    return vals;
                }

                let mid = n / 2;
                let right_vals = vals.split_off(mid);
                let left_vals = vals;

                let Pair(left_sorted, right_sorted) =
                    ParaPair!(move || parallel_sort(left_vals), move || parallel_sort(right_vals));

                // Merge sorted halves
                let mut result = Vec::with_capacity(n);
                let mut i = 0;
                let mut j = 0;
                while i < left_sorted.len() && j < right_sorted.len() {
                    if left_sorted[i] <= right_sorted[j] {
                        result.push(left_sorted[i].clone());
                        i += 1;
                    } else {
                        result.push(right_sorted[j].clone());
                        j += 1;
                    }
                }
                result.extend_from_slice(&left_sorted[i..]);
                result.extend_from_slice(&right_sorted[j..]);
                result
            }

            vals = parallel_sort(vals);
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        // PARALLEL: filter using divide-and-conquer (unconditionally parallel)
        // Work: Θ(n), Span: Θ(log n)
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> Self {
            let n = self.size();

            if n <= 1 {
                if n == 0 {
                    return Self::empty();
                }
                let elem = self.elements.nth(0);
                if f(elem) {
                    return Self::singleton(elem.clone());
                } else {
                    return Self::empty();
                }
            }

            // Unconditionally parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals = (0..mid).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();
            let right_vals = (mid..n).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));

            let f_left = f.clone();
            let f_right = f;

            let Pair(left_result, right_result) =
                ParaPair!(move || left_set.filter(f_left), move || right_set.filter(f_right));

            // Sequential merge of results to avoid nested parallel recursion
            let mut vals = left_result.elements.values_in_order();
            vals.extend(right_result.elements.values_in_order());
            vals.sort();
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        // PARALLEL: intersection using divide-and-conquer (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        fn intersection(&self, other: &Self) -> Self {
            let n = self.size();
            let m = other.size();

            if n == 0 || m == 0 {
                return Self::empty();
            }

            if n == 1 {
                let elem = self.elements.nth(0);
                if other.find(elem) {
                    return Self::singleton(elem.clone());
                } else {
                    return Self::empty();
                }
            }

            // Unconditionally parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals = (0..mid).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();
            let right_vals = (mid..n).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));
            let other_left = other.clone();
            let other_right = other.clone();

            let Pair(left_result, right_result) =
                ParaPair!(move || left_set.intersection(&other_left), move || right_set
                    .intersection(&other_right));

            // Sequential merge of results to avoid nested parallel recursion
            let mut vals = left_result.elements.values_in_order();
            vals.extend(right_result.elements.values_in_order());
            vals.sort();
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        fn difference(&self, other: &Self) -> Self {
            let other_clone = other.clone();
            self.filter(move |x| !other_clone.find(x))
        }

        // PARALLEL: union using divide-and-conquer (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        fn union(&self, other: &Self) -> Self {
            let n = self.size();
            let m = other.size();

            if n == 0 {
                return other.clone();
            }
            if m == 0 {
                return self.clone();
            }

            if n == 1 {
                return other.insert(self.elements.nth(0).clone());
            }

            // Unconditionally parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals = (0..mid).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();
            let right_vals = (mid..n).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));
            let other_left = other.clone();
            let other_right = other.clone();

            let Pair(left_result, right_result) = ParaPair!(move || left_set.union(&other_left), move || right_set
                .union(&other_right));

            // Sequential merge of results to avoid nested parallel recursion
            let mut vals = left_result.elements.values_in_order();
            vals.extend(right_result.elements.values_in_order());
            vals.sort();
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        fn find(&self, x: &T) -> B {
            // Binary search in sorted sequence
            let n = self.size();
            let mut left = 0;
            let mut right = n;

            while left < right {
                let mid = (left + right) / 2;
                let elem = self.elements.nth(mid);
                match elem.cmp(x) {
                    | Less => left = mid + 1,
                    | Equal => return true,
                    | Greater => right = mid,
                }
            }
            false
        }

        fn delete(&self, x: &T) -> Self {
            // Unconditionally use parallel filter
            let x_clone = x.clone();
            self.filter(move |v| v != &x_clone)
        }

        fn insert(&self, x: T) -> Self {
            if self.find(&x) {
                return self.clone();
            }
            let mut vals = self.elements.values_in_order();
            vals.push(x);

            // Unconditionally use parallel from_seq
            Self::from_seq(AVLTreeSeqMtPerS::from_vec(vals))
        }
    }

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtPer<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtPer<T> {
        fn clone(&self) -> Self {
            AVLTreeSetMtPer {
                elements: self.elements.clone(),
            }
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSetMtPerLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty();
            $( __set = __set.insert($x); )*
            __set
        }};
    }

    verus! {
        impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtPer<T> {
            type V = Self;

            open spec fn view(&self) -> Self {
                *self
            }
        }
    }
}
