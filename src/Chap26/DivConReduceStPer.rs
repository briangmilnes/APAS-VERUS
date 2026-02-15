//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer via reduce pattern - sequential implementation (Chapter 26, Section 5).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod DivConReduceStPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeq::ArraySeq::{spec_iterate, spec_monoid};
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		4. spec functions

    pub open spec fn spec_sum_fn() -> spec_fn(N, N) -> N { |x: N, y: N| (x + y) as N }
    pub open spec fn spec_product_fn() -> spec_fn(N, N) -> N { |x: N, y: N| (x * y) as N }
    pub open spec fn spec_or_fn() -> spec_fn(B, B) -> B { |x: B, y: B| x || y }
    pub open spec fn spec_and_fn() -> spec_fn(B, B) -> B { |x: B, y: B| x && y }
    pub open spec fn spec_max_fn() -> spec_fn(N, N) -> N { |x: N, y: N| if x >= y { x } else { y } }

    //		8. traits

    pub trait DivConReduceStTrait {
        /// Find maximum element via reduce.
        /// Pattern: reduce max identity
        /// APAS: Example 26.2. Work Θ(n), Span Θ(n)
        fn max_element(a: &ArraySeqStPerS<N>) -> (result: Option<N>)
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

        /// Sum all elements via reduce.
        /// Pattern: reduce (+) 0 identity
        /// APAS: Work Θ(n), Span Θ(n)
        fn sum(a: &ArraySeqStPerS<N>) -> (result: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_sum_fn(), 0),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_sum_fn(), 0);

        /// Product of all elements via reduce.
        /// Pattern: reduce (*) 1 identity
        /// APAS: Work Θ(n), Span Θ(n)
        fn product(a: &ArraySeqStPerS<N>) -> (result: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_product_fn(), 1),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_product_fn(), 1);

        /// Logical OR of all elements via reduce.
        /// Pattern: reduce (||) false identity
        /// APAS: Work Θ(n), Span Θ(n)
        fn any(a: &ArraySeqStPerS<B>) -> (result: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_or_fn(), false),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_or_fn(), false);

        /// Logical AND of all elements via reduce.
        /// Pattern: reduce (&&) true identity
        /// APAS: Work Θ(n), Span Θ(n)
        fn all(a: &ArraySeqStPerS<B>) -> (result: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_and_fn(), true),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_and_fn(), true);
    }

    //		9. impls

    impl DivConReduceStTrait for ArraySeqStPerS<N> {
        #[verifier::external_body]
        fn max_element(a: &ArraySeqStPerS<N>) -> (result: Option<N>) {
            if a.length() == 0 {
                return None;
            }
            Some(ArraySeqStPerS::reduce(a, &|x, y| (*x).max(*y), Ghost(spec_max_fn()), *a.nth(0)))
        }

        #[verifier::external_body]
        fn sum(a: &ArraySeqStPerS<N>) -> (result: N) {
            ArraySeqStPerS::reduce(a, &|x, y| x + y, Ghost(spec_sum_fn()), 0)
        }

        #[verifier::external_body]
        fn product(a: &ArraySeqStPerS<N>) -> (result: N) {
            ArraySeqStPerS::reduce(a, &|x, y| x * y, Ghost(spec_product_fn()), 1)
        }

        #[verifier::external_body]
        fn any(a: &ArraySeqStPerS<B>) -> (result: B) {
            ArraySeqStPerS::reduce(a, &|x, y| *x || *y, Ghost(spec_or_fn()), false)
        }

        #[verifier::external_body]
        fn all(a: &ArraySeqStPerS<B>) -> (result: B) {
            ArraySeqStPerS::reduce(a, &|x, y| *x && *y, Ghost(spec_and_fn()), true)
        }
    }

    } // verus!
} // mod
