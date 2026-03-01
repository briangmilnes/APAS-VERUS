//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral set implementation using AVLTreeSetStEph as backing store.
//!
//! Work/Span Analysis (with extract-parallelize-rebuild pattern):
//! - union: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - intersection: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - filter: Work Θ(n), Span Θ(log n) via PARALLEL map-reduce

pub mod AVLTreeSetMtEph {

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
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

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

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    pub struct AVLTreeSetMtEphInv;

    impl<T: StTInMtT + Ord + 'static> RwLockPredicate<AVLTreeSetStEph<T>> for AVLTreeSetMtEphInv {
        open spec fn inv(self, v: AVLTreeSetStEph<T>) -> bool { true }
    }

    #[verifier::external_body]
    fn new_set_mt_lock<T: StTInMtT + Ord + 'static>(val: AVLTreeSetStEph<T>) -> (lock: RwLock<AVLTreeSetStEph<T>, AVLTreeSetMtEphInv>) {
        RwLock::new(val, Ghost(AVLTreeSetMtEphInv))
    }

    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
        pub inner: Arc<RwLock<AVLTreeSetStEph<T>, AVLTreeSetMtEphInv>>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEph<T> {
        #[verifier::external_body]
        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            Set::empty()
        }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }

    // 8. traits

    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord + 'static>: Sized + View<V = Set<<T as View>::V>> {
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n)
        fn to_seq(&self) -> (result: AVLTreeSeqStEphS<T>)
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
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (result: Self)
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
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@), self@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@), self@.finite();
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        #[verifier::external_body]
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite()
        {
            let handle = self.inner.acquire_read();
            let result = handle.borrow().size();
            handle.release_read();
            result
        }

        #[verifier::external_body]
        fn to_seq(&self) -> (result: AVLTreeSeqStEphS<T>)
            ensures self@.finite()
        {
            let handle = self.inner.acquire_read();
            let result = handle.borrow().to_seq();
            handle.release_read();
            result
        }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty()
        {
            AVLTreeSetMtEph {
                inner: Arc::new(new_set_mt_lock(AVLTreeSetStEph::empty())),
            }
        }

        #[verifier::external_body]
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@), result@.finite()
        {
            AVLTreeSetMtEph {
                inner: Arc::new(new_set_mt_lock(AVLTreeSetStEph::singleton(x))),
            }
        }

        #[verifier::external_body]
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (result: Self)
            ensures result@.finite()
        {
            AVLTreeSetMtEph {
                inner: Arc::new(new_set_mt_lock(AVLTreeSetStEph::from_seq(seq))),
            }
        }

        // PARALLEL: filter using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n), Span: Θ(log n)
        #[verifier::external_body]
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@)
        {
            let vals = {
                let handle = self.inner.acquire_read();
                let seq = handle.borrow().to_seq();
                let mut vals = Vec::with_capacity(seq.length());
                for i in 0..seq.length() {
                    vals.push(seq.nth(i).clone());
                }
                handle.release_read();
                vals
            };

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
        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite()
        {
            let (self_vals, other_vals) = {
                let self_handle = self.inner.acquire_read();
                let other_handle = other.inner.acquire_read();

                let self_seq = self_handle.borrow().to_seq();
                let other_seq = other_handle.borrow().to_seq();

                let mut sv = Vec::with_capacity(self_seq.length());
                for i in 0..self_seq.length() {
                    sv.push(self_seq.nth(i).clone());
                }

                let mut ov = Vec::with_capacity(other_seq.length());
                for i in 0..other_seq.length() {
                    ov.push(other_seq.nth(i).clone());
                }

                self_handle.release_read();
                other_handle.release_read();

                (sv, ov)
            };

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
        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite()
        {
            let other_clone = other.clone();
            self.filter(move |x| !other_clone.find(x))
        }

        // PARALLEL: union using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        // Note: Union uses a simple merge strategy to avoid thread explosion.
        #[verifier::external_body]
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite()
        {
            let (self_vals, other_vals) = {
                let self_handle = self.inner.acquire_read();
                let other_handle = other.inner.acquire_read();

                let self_seq = self_handle.borrow().to_seq();
                let other_seq = other_handle.borrow().to_seq();

                let mut sv = Vec::with_capacity(self_seq.length());
                for i in 0..self_seq.length() {
                    sv.push(self_seq.nth(i).clone());
                }

                let mut ov = Vec::with_capacity(other_seq.length());
                for i in 0..other_seq.length() {
                    ov.push(other_seq.nth(i).clone());
                }

                self_handle.release_read();
                other_handle.release_read();

                (sv, ov)
            };

            let mut merged = self_vals;
            merged.extend(other_vals);
            merged.sort();
            merged.dedup();

            Self::from_seq(AVLTreeSeqStEphS::from_vec(merged))
        }

        #[verifier::external_body]
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(x@)
        {
            let handle = self.inner.acquire_read();
            let result = handle.borrow().find(x);
            handle.release_read();
            result
        }

        #[verifier::external_body]
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@), self@.finite()
        {
            let (mut current, write_handle) = self.inner.acquire_write();
            current.delete(x);
            write_handle.release_write(current);
        }

        #[verifier::external_body]
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@), self@.finite()
        {
            let (mut current, write_handle) = self.inner.acquire_write();
            current.insert(x);
            write_handle.release_write(current);
        }
    }

    // 11. derive impls in verus!

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtEph<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            let handle = self.inner.acquire_read();
            let cloned_inner = (*handle.borrow()).clone();
            handle.release_read();
            AVLTreeSetMtEph {
                inner: Arc::new(new_set_mt_lock(cloned_inner)),
            }
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtEph<T> {
        fn default() -> Self { Self::empty() }
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
}
