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
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		4. spec functions

    /// Wrapping addition for usize — matches vstd wrapping_add spec with in-range casts.
    pub open spec fn spec_wrapping_add(x: N, y: N) -> N {
        if x + y > usize::MAX as int {
            ((x + y) - (usize::MAX as int + 1)) as N
        } else {
            (x + y) as N
        }
    }

    /// Wrapping multiplication for usize — matches vstd wrapping_mul spec with in-range casts.
    pub open spec fn spec_wrapping_mul(x: N, y: N) -> N {
        ((x as nat * y as nat) % (usize::MAX as nat + 1)) as N
    }

    pub open spec fn spec_sum_fn() -> spec_fn(N, N) -> N { |x: N, y: N| spec_wrapping_add(x, y) }
    pub open spec fn spec_product_fn() -> spec_fn(N, N) -> N { |x: N, y: N| spec_wrapping_mul(x, y) }
    pub open spec fn spec_or_fn() -> spec_fn(B, B) -> B { |x: B, y: B| x || y }
    pub open spec fn spec_and_fn() -> spec_fn(B, B) -> B { |x: B, y: B| x && y }
    pub open spec fn spec_max_fn() -> spec_fn(N, N) -> N { |x: N, y: N| if x >= y { x } else { y } }

    //		8. traits

    pub trait DivConReduceStTrait {
        /// Find maximum element via reduce.
        /// Pattern: reduce max identity
        /// - APAS: Work Θ(n), Span Θ(lg n) — Example 26.2, D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential while loop, Span = Work.
        fn max_element(a: &ArraySeqStPerS<N>) -> (max: Option<N>)
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
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential reduce, Span = Work.
        fn sum(a: &ArraySeqStPerS<N>) -> (total: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_sum_fn(), 0),
            ensures
                total == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_sum_fn(), 0);

        /// Product of all elements via reduce.
        /// Pattern: reduce (*) 1 identity
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential reduce, Span = Work.
        fn product(a: &ArraySeqStPerS<N>) -> (total: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_product_fn(), 1),
            ensures
                total == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_product_fn(), 1);

        /// Logical OR of all elements via reduce.
        /// Pattern: reduce (||) false identity
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential reduce, Span = Work.
        fn any(a: &ArraySeqStPerS<B>) -> (found: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_or_fn(), false),
            ensures
                found == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_or_fn(), false);

        /// Logical AND of all elements via reduce.
        /// Pattern: reduce (&&) true identity
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential reduce, Span = Work.
        fn all(a: &ArraySeqStPerS<B>) -> (all_true: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_and_fn(), true),
            ensures
                all_true == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_and_fn(), true);
    }

    //		9. impls

    impl DivConReduceStTrait for ArraySeqStPerS<N> {
        fn max_element(a: &ArraySeqStPerS<N>) -> (max: Option<N>) {
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

        fn sum(a: &ArraySeqStPerS<N>) -> (total: N) {
            ArraySeqStPerS::reduce(a,
                &(|x: &N, y: &N| -> (ret: N)
                    ensures ret == spec_wrapping_add(*x, *y)
                { (*x).wrapping_add(*y) }),
                Ghost(spec_sum_fn()), 0)
        }

        fn product(a: &ArraySeqStPerS<N>) -> (total: N) {
            ArraySeqStPerS::reduce(a,
                &(|x: &N, y: &N| -> (ret: N)
                    ensures ret == spec_wrapping_mul(*x, *y)
                { (*x).wrapping_mul(*y) }),
                Ghost(spec_product_fn()), 1)
        }

        fn any(a: &ArraySeqStPerS<B>) -> (found: B) {
            ArraySeqStPerS::reduce(a,
                &(|x: &B, y: &B| -> (ret: B)
                    ensures ret == spec_or_fn()(*x, *y)
                { *x || *y }),
                Ghost(spec_or_fn()), false)
        }

        fn all(a: &ArraySeqStPerS<B>) -> (all_true: B) {
            ArraySeqStPerS::reduce(a,
                &(|x: &B, y: &B| -> (ret: B)
                    ensures ret == spec_and_fn()(*x, *y)
                { *x && *y }),
                Ghost(spec_and_fn()), true)
        }
    }

    } // verus!
} // mod
