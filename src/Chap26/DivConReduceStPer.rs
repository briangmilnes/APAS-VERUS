// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Divide-and-conquer via reduce pattern - sequential implementation (Chapter 26, Section 5).
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod DivConReduceStPer {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		Section 6. spec fns


    /// Wrapping addition for usize — matches vstd wrapping_add spec with in-range casts.
    pub open spec fn spec_wrapping_add(x: usize, y: usize) -> usize {
        if x + y > usize::MAX as int {
            ((x + y) - (usize::MAX as int + 1)) as usize
        } else {
            (x + y) as usize
        }
    }

    /// Wrapping multiplication for usize — matches vstd wrapping_mul spec with in-range casts.
    pub open spec fn spec_wrapping_mul(x: usize, y: usize) -> usize {
        ((x as nat * y as nat) % (usize::MAX as nat + 1)) as usize
    }

    pub open spec fn spec_sum_fn() -> spec_fn(usize, usize) -> usize { |x: usize, y: usize| spec_wrapping_add(x, y) }
    pub open spec fn spec_product_fn() -> spec_fn(usize, usize) -> usize { |x: usize, y: usize| spec_wrapping_mul(x, y) }
    pub open spec fn spec_or_fn() -> spec_fn(bool, bool) -> bool { |x: bool, y: bool| x || y }
    pub open spec fn spec_and_fn() -> spec_fn(bool, bool) -> bool { |x: bool, y: bool| x && y }
    pub open spec fn spec_max_fn() -> spec_fn(usize, usize) -> usize { |x: usize, y: usize| if x >= y { x } else { y } }

    //		Section 8. traits


    pub trait DivConReduceStTrait {
        /// Find maximum element via reduce.
        /// Pattern: reduce max identity
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: sequential while loop, not D&C
        fn max_element(a: &ArraySeqStPerS<usize>) -> (max: Option<usize>)
            requires a.spec_len() <= usize::MAX,
            ensures
                a.spec_len() == 0 ==> max is None,
                a.spec_len() > 0 ==> {
                    &&& max is Some
                    &&& forall|i: int| #![trigger a.spec_index(i)]
                            0 <= i < a.spec_len() ==> a.spec_index(i) <= max->Some_0
                    &&& exists|i: int| #![trigger a.spec_index(i)]
                            0 <= i < a.spec_len() && a.spec_index(i) == max->Some_0
                };

        /// Sum all elements via reduce.
        /// Pattern: reduce (+) 0 identity
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: delegates to sequential reduce
        fn sum(a: &ArraySeqStPerS<usize>) -> (total: usize)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_sum_fn(), 0),
            ensures
                total == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_sum_fn(), 0);

        /// Product of all elements via reduce.
        /// Pattern: reduce (*) 1 identity
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: delegates to sequential reduce
        fn product(a: &ArraySeqStPerS<usize>) -> (total: usize)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_product_fn(), 1),
            ensures
                total == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_product_fn(), 1);

        /// Logical OR of all elements via reduce.
        /// Pattern: reduce (||) false identity
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: delegates to sequential reduce
        fn any(a: &ArraySeqStPerS<bool>) -> (found: bool)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_or_fn(), false),
            ensures
                found == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_or_fn(), false);

        /// Logical AND of all elements via reduce.
        /// Pattern: reduce (&&) true identity
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: delegates to sequential reduce
        fn all(a: &ArraySeqStPerS<bool>) -> (all_true: bool)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_and_fn(), true),
            ensures
                all_true == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_and_fn(), true);
    }

    //		Section 9. impls


    impl DivConReduceStTrait for ArraySeqStPerS<usize> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — single-pass max scan; St sequential.
        fn max_element(a: &ArraySeqStPerS<usize>) -> (max: Option<usize>) {
            let len = a.length();
            if len == 0 {
                return None;
            }
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let mut best: usize = *a.nth(0);
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to reduce; St sequential.
        fn sum(a: &ArraySeqStPerS<usize>) -> (total: usize) {
            ArraySeqStPerS::reduce(a,
                &(|x: &usize, y: &usize| -> (ret: usize)
                    ensures ret == spec_wrapping_add(*x, *y)
                { (*x).wrapping_add(*y) }),
                Ghost(spec_sum_fn()), 0)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to reduce; St sequential.
        fn product(a: &ArraySeqStPerS<usize>) -> (total: usize) {
            ArraySeqStPerS::reduce(a,
                &(|x: &usize, y: &usize| -> (ret: usize)
                    ensures ret == spec_wrapping_mul(*x, *y)
                { (*x).wrapping_mul(*y) }),
                Ghost(spec_product_fn()), 1)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to reduce; St sequential.
        fn any(a: &ArraySeqStPerS<bool>) -> (found: bool) {
            ArraySeqStPerS::reduce(a,
                &(|x: &bool, y: &bool| -> (ret: bool)
                    ensures ret == spec_or_fn()(*x, *y)
                { *x || *y }),
                Ghost(spec_or_fn()), false)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to reduce; St sequential.
        fn all(a: &ArraySeqStPerS<bool>) -> (all_true: bool) {
            ArraySeqStPerS::reduce(a,
                &(|x: &bool, y: &bool| -> (ret: bool)
                    ensures ret == spec_and_fn()(*x, *y)
                { *x && *y }),
                Ghost(spec_and_fn()), true)
        }
    }

    } // verus!
} // mod
