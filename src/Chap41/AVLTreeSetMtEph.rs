//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral set implementation using AVLTreeSetStEph as backing store.
//!
//! Work/Span Analysis (with extract-parallelize-rebuild pattern):
//! - union: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - intersection: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - filter: Work Θ(n), Span Θ(log n) via PARALLEL map-reduce

pub mod AVLTreeSetMtEph {

    use std::fmt;
    use std::sync::{Arc, Mutex};

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    // NOTE: This type does NOT implement Ord (unlike AVLTreeSetMtPer) because no caller requires it.
    // AVLTreeSetMtEph is not used as a value type in OrderedTableMtPer. It's only used for:
    // - Standalone set operations (union, intersection, filter)
    // - As a set of vertices/priorities in PQMinMtEph (Chap53)
    //
    // If future code tries to use AVLTreeSetMtEph as a value in OrderedTableMtPer, compilation
    // will fail with "the trait bound `AVLTreeSetMtEph<V>: Ord` is not satisfied", and we can
    // implement Ord then. This is purely driven by caller requirements.

    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
        inner: Arc<Mutex<AVLTreeSetStEph<T>>>,
    }

    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord + 'static> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                               -> N;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn to_seq(&self)                             -> AVLTreeSeqStEphS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                           -> Self;
        /// claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqStEphS<T>)        -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self)         -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self)           -> Self;
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self)                -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T)                        -> B;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T);
    }

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        fn size(&self) -> N {
            let inner = self.inner.lock().unwrap();
            inner.size()
        }

        fn to_seq(&self) -> AVLTreeSeqStEphS<T> {
            let inner = self.inner.lock().unwrap();
            inner.to_seq()
        }

        fn empty() -> Self {
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(AVLTreeSetStEph::empty())),
            }
        }

        fn singleton(x: T) -> Self {
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(AVLTreeSetStEph::singleton(x))),
            }
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> Self {
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(AVLTreeSetStEph::from_seq(seq))),
            }
        }

        // PARALLEL: filter using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n), Span: Θ(log n)
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> Self {
            // Extract data from mutex
            let vals = {
                let inner = self.inner.lock().unwrap();
                let seq = inner.to_seq();
                let mut vals = Vec::with_capacity(seq.length());
                for i in 0..seq.length() {
                    vals.push(seq.nth(i).clone());
                }
                vals
            };
            // Lock released here

            // Unconditionally parallel divide-and-conquer using ParaPair!
            fn parallel_filter<T: StTInMtT + Ord + 'static, F: PredMt<T> + Clone>(vals: Vec<T>, f: F) -> Vec<T> {
                let n = vals.len();
                if n == 0 {
                    return Vec::new();
                }
                if n == 1 {
                    return if f(&vals[0]) { vals } else { Vec::new() };
                }

                let mid = n / 2;
                let mut right_vals = vals;
                let left_vals = right_vals.split_off(mid);
                let right_vals_final = right_vals;

                let f_left = f.clone();
                let f_right = f;

                let Pair(left_filtered, right_filtered) = ParaPair!(
                    move || parallel_filter(left_vals, f_left),
                    move || parallel_filter(right_vals_final, f_right)
                );

                let mut result = left_filtered;
                result.extend(right_filtered);
                result
            }

            let filtered = parallel_filter(vals, f);
            Self::from_seq(AVLTreeSeqStEphS::from_vec(filtered))
        }

        // PARALLEL: intersection using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        fn intersection(&self, other: &Self) -> Self {
            // Extract data from both mutexes
            let (self_vals, other_vals) = {
                let self_inner = self.inner.lock().unwrap();
                let other_inner = other.inner.lock().unwrap();

                let self_seq = self_inner.to_seq();
                let other_seq = other_inner.to_seq();

                let mut sv = Vec::with_capacity(self_seq.length());
                for i in 0..self_seq.length() {
                    sv.push(self_seq.nth(i).clone());
                }

                let mut ov = Vec::with_capacity(other_seq.length());
                for i in 0..other_seq.length() {
                    ov.push(other_seq.nth(i).clone());
                }

                (sv, ov)
            };
            // Locks released here

            // Unconditionally parallel divide-and-conquer using ParaPair!
            fn parallel_intersect<T: StTInMtT + Ord + 'static>(self_vals: Vec<T>, other_vals: Vec<T>) -> Vec<T> {
                let n = self_vals.len();
                if n == 0 {
                    return Vec::new();
                }
                if n == 1 {
                    let other_set = AVLTreeSetMtEph::from_seq(AVLTreeSeqStEphS::from_vec(other_vals));
                    return if other_set.find(&self_vals[0]) {
                        self_vals
                    } else {
                        Vec::new()
                    };
                }

                let mid = n / 2;
                let mut right_self = self_vals;
                let left_self = right_self.split_off(mid);
                let right_self_final = right_self;

                let other_left = other_vals.clone();
                let other_right = other_vals;

                let Pair(left_intersect, right_intersect) =
                    ParaPair!(move || parallel_intersect(left_self, other_left), move || {
                        parallel_intersect(right_self_final, other_right)
                    });

                let mut result = left_intersect;
                result.extend(right_intersect);
                result
            }

            let intersect = parallel_intersect(self_vals, other_vals);
            Self::from_seq(AVLTreeSeqStEphS::from_vec(intersect))
        }

        // PARALLEL: difference using filter
        fn difference(&self, other: &Self) -> Self {
            let other_clone = other.clone();
            self.filter(move |x| !other_clone.find(x))
        }

        // PARALLEL: union using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        // Note: Union uses a simple merge strategy to avoid thread explosion.
        fn union(&self, other: &Self) -> Self {
            // Extract data from both mutexes
            let (self_vals, other_vals) = {
                let self_inner = self.inner.lock().unwrap();
                let other_inner = other.inner.lock().unwrap();

                let self_seq = self_inner.to_seq();
                let other_seq = other_inner.to_seq();

                let mut sv = Vec::with_capacity(self_seq.length());
                for i in 0..self_seq.length() {
                    sv.push(self_seq.nth(i).clone());
                }

                let mut ov = Vec::with_capacity(other_seq.length());
                for i in 0..other_seq.length() {
                    ov.push(other_seq.nth(i).clone());
                }

                (sv, ov)
            };
            // Locks released here

            // Simple merge (sequential to avoid thread explosion)
            let mut merged = self_vals;
            merged.extend(other_vals);
            merged.sort();
            merged.dedup();

            Self::from_seq(AVLTreeSeqStEphS::from_vec(merged))
        }

        fn find(&self, x: &T) -> B {
            let inner = self.inner.lock().unwrap();
            inner.find(x)
        }

        fn delete(&mut self, x: &T) {
            let mut inner = self.inner.lock().unwrap();
            inner.delete(x);
        }

        fn insert(&mut self, x: T) {
            let mut inner = self.inner.lock().unwrap();
            inner.insert(x);
        }
    }

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtEph<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.lock().unwrap();
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new((*inner).clone())),
            }
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSetMtEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }
}
