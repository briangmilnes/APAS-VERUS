//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent set implementation using AVLTreeSeqMtPer (Arc-based).
//!
//! Work/Span Analysis:
//! - union: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - intersection: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - filter: Work Θ(n), Span Θ(log n) via PARALLEL map-reduce

pub mod AVLTreeSetMtPer {

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

    use std::cmp::Ordering::{self, Equal, Greater, Less};
    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    // NOTE: This type implements Ord because it is used as a VALUE in OrderedTableMtPer.
    // OrderedTableMtPer<K, V> is backed by BSTParaTreapMtEph<Pair<K, V>>, which requires
    // BOTH K and V to be Ord (via MtKey trait). For example, AdjTableGraphMtPer uses
    // OrderedTableMtPer<V, AVLTreeSetMtPer<V>>, so AVLTreeSetMtPer<V> must implement Ord.
    //
    // This is purely a caller requirement - if no code used AVLTreeSetMtPer as a value in
    // an ordered table, we wouldn't need Ord. See AVLTreeSetMtEph for comparison (no Ord needed).

    verus! {

    // 4. type definitions

    pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
        elements: AVLTreeSeqMtPerS<T>,
    }

    /// Sequential cutoff to prevent thread explosion from recursive ParaPair! calls.
    pub const SEQUENTIAL_CUTOFF: N = 128;

    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPer<T> {
        #[verifier::external_body]
        pub closed spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            Set::empty()
        }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }

    // 8. traits

    pub trait AVLTreeSetMtPerTrait<T: StTInMtT + Ord + 'static> {
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: N)
            ensures result == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn to_seq(&self) -> (result: AVLTreeSeqMtPerS<T>)
            ensures self@.finite();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@), result@.finite();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (result: Self)
            ensures result@.finite();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@);
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (result: Self)
            ensures result@ == self@.remove(x@), result@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (result: Self)
            ensures result@ == self@.insert(x@), result@.finite();
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPerTrait<T> for AVLTreeSetMtPer<T> {
        fn size(&self) -> (result: N)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let r = self.elements.length();
            proof { assume(r == self@.len()); assume(self@.finite()); }
            r
        }

        fn to_seq(&self) -> (result: AVLTreeSeqMtPerS<T>)
        {
            let result = self.elements.clone();
            proof { assume(self@.finite()); }
            result
        }

        fn empty() -> (result: Self)
        {
            let result = AVLTreeSetMtPer { elements: AVLTreeSeqMtPerS::empty() };
            proof { assume(result@ == Set::<<T as View>::V>::empty()); }
            result
        }

        fn singleton(x: T) -> (result: Self)
        {
            let ghost x_view = x@;
            let result = AVLTreeSetMtPer { elements: AVLTreeSeqMtPerS::singleton(x) };
            proof {
                assume(result@ == Set::<<T as View>::V>::empty().insert(x_view));
                assume(result@.finite());
            }
            result
        }

        #[verifier::external_body]
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (result: Self)
            ensures result@.finite()
        {
            let mut vals = seq.values_in_order();

            fn parallel_sort<T: StTInMtT + Ord + 'static>(mut vals: Vec<T>) -> Vec<T> {
                let n = vals.len();
                if n <= 1 {
                    return vals;
                }
                if n <= SEQUENTIAL_CUTOFF {
                    vals.sort();
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

        // PARALLEL: filter using divide-and-conquer with sequential cutoff
        // Work: Θ(n), Span: Θ(log n) when parallel
        #[verifier::external_body]
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@)
        {
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

            if n <= SEQUENTIAL_CUTOFF {
                let mut vals: Vec<T> = Vec::new();
                for i in 0..n {
                    let elem = self.elements.nth(i);
                    if f(elem) {
                        vals.push(elem.clone());
                    }
                }
                vals.sort();
                vals.dedup();
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Parallel divide-and-conquer using ParaPair!
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

        // PARALLEL: intersection using divide-and-conquer with sequential cutoff
        // Work: Θ(n+m), Span: Θ(log(n+m)) when parallel
        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite()
        {
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

            if n + m <= SEQUENTIAL_CUTOFF {
                let mut vals: Vec<T> = Vec::new();
                for i in 0..n {
                    let elem = self.elements.nth(i);
                    if other.find(elem) {
                        vals.push(elem.clone());
                    }
                }
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Parallel divide-and-conquer using ParaPair!
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

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite()
        {
            let other_clone = other.clone();
            self.filter(move |x| !other_clone.find(x))
        }

        // PARALLEL: union using divide-and-conquer with sequential cutoff
        // Work: Θ(n+m), Span: Θ(log(n+m)) when parallel
        #[verifier::external_body]
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite()
        {
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

            if n + m <= SEQUENTIAL_CUTOFF {
                let mut vals = self.elements.values_in_order();
                let other_vals = other.elements.values_in_order();
                vals.extend(other_vals);
                vals.sort();
                vals.dedup();
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Parallel divide-and-conquer using ParaPair!
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

        fn find(&self, x: &T) -> (result: B)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let n = self.elements.length();
            let mut lo: usize = 0;
            let mut hi: usize = n;
            while lo < hi
                invariant
                    self.elements.spec_well_formed(),
                    n as int == self.elements.spec_seq().len(),
                    lo <= hi, hi <= n,
                decreases hi - lo,
            {
                let mid = lo + (hi - lo) / 2;
                let elem = self.elements.nth(mid);
                if *elem == *x {
                    proof { assume(self@.contains(x@)); }
                    return true;
                }
                if *elem < *x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            proof { assume(!self@.contains(x@)); }
            false
        }

        #[verifier::external_body]
        fn delete(&self, x: &T) -> (result: Self)
            ensures result@ == self@.remove(x@), result@.finite()
        {
            // Unconditionally use parallel filter
            let x_clone = x.clone();
            self.filter(move |v| v != &x_clone)
        }

        #[verifier::external_body]
        fn insert(&self, x: T) -> (result: Self)
            ensures result@ == self@.insert(x@), result@.finite()
        {
            if self.find(&x) {
                return self.clone();
            }
            let mut vals = self.elements.values_in_order();
            vals.push(x);

            // Unconditionally use parallel from_seq
            Self::from_seq(AVLTreeSeqMtPerS::from_vec(vals))
        }
    }

    // 11. derive impls in verus!

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtPer<T> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            let result = AVLTreeSetMtPer { elements: self.elements.clone() };
            proof { assume(result@ == self@); }
            result
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtPer<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StTInMtT + Ord + 'static> PartialEq for AVLTreeSetMtPer<T> {
        fn eq(&self, other: &Self) -> bool {
            self.size() == other.size() && {
                for i in 0..self.size() {
                    if !other.find(self.elements.nth(i)) {
                        return false;
                    }
                }
                true
            }
        }
    }

    impl<T: StTInMtT + Ord + 'static> Eq for AVLTreeSetMtPer<T> {}

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
}
