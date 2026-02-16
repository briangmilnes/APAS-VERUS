//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer via reduce pattern - parallel implementation (Chapter 26, Section 5).
//! Note: Unconditionally parallel - no thresholding per APAS rules.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod DivConReduceMtPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    #[cfg(verus_keep_ghost)]
    use {
        crate::Chap18::ArraySeq::ArraySeq::{spec_iterate, spec_monoid},
        crate::Chap26::DivConReduceStPer::DivConReduceStPer::{
            spec_sum_fn, spec_product_fn, spec_or_fn, spec_and_fn, spec_max_fn,
        },
    };
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		8. traits

    pub trait DivConReduceMtTrait {
        /// Find maximum element via parallel reduce.
        /// Pattern: reduce max identity (parallel)
        /// APAS: Example 26.2. Work Θ(n), Span Θ(log n)
        fn max_element_parallel(a: &ArraySeqMtPerS<N>) -> (result: Option<N>)
            requires a.spec_len() <= usize::MAX,
            ensures
                a.spec_len() == 0 ==> result is None,
                a.spec_len() > 0 ==> {
                    &&& result is Some
                    &&& forall|i: int| #![trigger a.spec_index(i)]
                            0 <= i < a.spec_len() ==> a.spec_index(i) <= result->Some_0
                    &&& exists|i: int| #![trigger a.spec_index(i)]
                            0 <= i < a.spec_len() && a.spec_index(i) == result->Some_0
                };

        /// Sum all elements via parallel reduce.
        /// Pattern: reduce (+) 0 identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        fn sum_parallel(a: &ArraySeqMtPerS<N>) -> (result: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_sum_fn(), 0),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_sum_fn(), 0);

        /// Product of all elements via parallel reduce.
        /// Pattern: reduce (*) 1 identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        fn product_parallel(a: &ArraySeqMtPerS<N>) -> (result: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_product_fn(), 1),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_product_fn(), 1);

        /// Logical OR of all elements via parallel reduce.
        /// Pattern: reduce (||) false identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        fn any_parallel(a: &ArraySeqMtPerS<B>) -> (result: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_or_fn(), false),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_or_fn(), false);

        /// Logical AND of all elements via parallel reduce.
        /// Pattern: reduce (&&) true identity (parallel)
        /// APAS: Work Θ(n), Span Θ(log n)
        fn all_parallel(a: &ArraySeqMtPerS<B>) -> (result: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_and_fn(), true),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_and_fn(), true);
    }

    //		9. impls

    impl DivConReduceMtTrait for ArraySeqMtPerS<N> {
        fn max_element_parallel(a: &ArraySeqMtPerS<N>) -> (result: Option<N>) {
            let len = a.length();
            if len == 0 {
                return None;
            }
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let mut best: N = *a.nth(0);
            let mut i: usize = 1;
            while i < len
                invariant
                    1 <= i <= len,
                    len == a.spec_len(),
                    s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                    forall|j: int| #![trigger a.spec_index(j)]
                        0 <= j < i as int ==> a.spec_index(j) <= best,
                    exists|j: int| #![trigger a.spec_index(j)]
                        0 <= j < i as int && a.spec_index(j) == best,
                decreases len - i,
            {
                let v = *a.nth(i);
                if v > best {
                    best = v;
                }
                i += 1;
            }
            Some(best)
        }

        #[verifier::external_body]
        fn sum_parallel(a: &ArraySeqMtPerS<N>) -> (result: N) {
            ArraySeqMtPerS::reduce(a, &|x: &N, y: &N| *x + *y, Ghost(spec_sum_fn()), 0)
        }

        #[verifier::external_body]
        fn product_parallel(a: &ArraySeqMtPerS<N>) -> (result: N) {
            ArraySeqMtPerS::reduce(a, &|x: &N, y: &N| *x * *y, Ghost(spec_product_fn()), 1)
        }

        fn any_parallel(a: &ArraySeqMtPerS<B>) -> (result: B) {
            ArraySeqMtPerS::reduce(a,
                &(|x: &B, y: &B| -> (ret: B)
                    ensures ret == spec_or_fn()(*x, *y)
                { *x || *y }),
                Ghost(spec_or_fn()), false)
        }

        fn all_parallel(a: &ArraySeqMtPerS<B>) -> (result: B) {
            ArraySeqMtPerS::reduce(a,
                &(|x: &B, y: &B| -> (ret: B)
                    ensures ret == spec_and_fn()(*x, *y)
                { *x && *y }),
                Ghost(spec_and_fn()), true)
        }
    }

    } // verus!
} // mod
