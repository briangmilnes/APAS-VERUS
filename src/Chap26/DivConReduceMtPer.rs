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
    use crate::Chap26::DivConReduceStPer::DivConReduceStPer::*;
    use crate::vstdplus::monoid::monoid::*;
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
        /// - APAS: Work Θ(n), Span Θ(lg n) — Example 26.2, D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
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
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn sum_parallel(a: &ArraySeqMtPerS<N>) -> (result: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_sum_fn(), 0),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_sum_fn(), 0);

        /// Product of all elements via parallel reduce.
        /// Pattern: reduce (*) 1 identity (parallel)
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn product_parallel(a: &ArraySeqMtPerS<N>) -> (result: N)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_product_fn(), 1),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_product_fn(), 1);

        /// Logical OR of all elements via parallel reduce.
        /// Pattern: reduce (||) false identity (parallel)
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn any_parallel(a: &ArraySeqMtPerS<B>) -> (result: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_or_fn(), false),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_or_fn(), false);

        /// Logical AND of all elements via parallel reduce.
        /// Pattern: reduce (&&) true identity (parallel)
        /// - APAS: Work Θ(n), Span Θ(lg n) — D&C reduce with constant-time op.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn all_parallel(a: &ArraySeqMtPerS<B>) -> (result: B)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_and_fn(), true),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_and_fn(), true);
    }

    //		9. impls

    /// Helper: establish fold_left one-step decomposition via lemma_fold_left_split.
    proof fn lemma_fold_left_step(s: Seq<N>, acc: N)
        requires s.len() > 0,
        ensures s.fold_left(acc, spec_max_fn())
            == s.subrange(1, s.len() as int).fold_left(
                spec_max_fn()(acc, s[0]), spec_max_fn()),
    {
        reveal_with_fuel(Seq::<_>::fold_left::<_>, 2);
        s.lemma_fold_left_split(acc, spec_max_fn(), 1);
        let first = s.subrange(0, 1);
        assert(first =~= Seq::new(1, |i: int| s[0]));
    }

    /// fold_left(s, acc, max) >= acc and every s[i] <= fold_left result.
    proof fn lemma_max_fold_left_bound(s: Seq<N>, acc: N)
        ensures
            s.fold_left(acc, spec_max_fn()) >= acc,
            forall|i: int| #![trigger s[i]] 0 <= i < s.len()
                ==> s[i] <= s.fold_left(acc, spec_max_fn()),
        decreases s.len(),
    {
        if s.len() == 0 {
        } else {
            let rest = s.subrange(1, s.len() as int);
            let new_acc = spec_max_fn()(acc, s[0]);
            lemma_fold_left_step(s, acc);
            lemma_max_fold_left_bound(rest, new_acc);
            assert forall|i: int| #![trigger s[i]] 0 <= i < s.len()
                implies s[i] <= s.fold_left(acc, spec_max_fn())
            by {
                if i == 0 {
                } else {
                    assert(rest[i - 1] == s[i]);
                }
            }
        }
    }

    /// fold_left(s, acc, max) is either acc itself or some element of s.
    proof fn lemma_max_fold_left_achievable(s: Seq<N>, acc: N)
        ensures
            s.fold_left(acc, spec_max_fn()) == acc
            || exists|i: int| #![trigger s[i]] 0 <= i < s.len()
                && s[i] == s.fold_left(acc, spec_max_fn()),
        decreases s.len(),
    {
        if s.len() == 0 {
        } else {
            let rest = s.subrange(1, s.len() as int);
            let new_acc = spec_max_fn()(acc, s[0]);
            lemma_fold_left_step(s, acc);
            lemma_max_fold_left_achievable(rest, new_acc);
            let result = rest.fold_left(new_acc, spec_max_fn());
            if result == new_acc {
                if new_acc == acc {
                } else {
                    assert(s[0] == result);
                }
            } else {
                let j = choose|j: int| 0 <= j < rest.len() && rest[j] == result;
                assert(rest[j] == s[j + 1]);
                assert(s[j + 1] == result);
            }
        }
    }

    impl DivConReduceMtTrait for ArraySeqMtPerS<N> {
        fn max_element_parallel(a: &ArraySeqMtPerS<N>) -> (result: Option<N>) {
            let len = a.length();
            if len == 0 {
                return None;
            }

            proof {
                assert forall|x: N| #[trigger] spec_max_fn()(0 as N, x) == x by {}
                assert forall|x: N| #[trigger] spec_max_fn()(x, 0 as N) == x by {}
                assert forall|x: N, y: N, z: N|
                    #[trigger] spec_max_fn()(spec_max_fn()(x, y), z)
                    == spec_max_fn()(x, spec_max_fn()(y, z)) by {}
            }

            let max_val = ArraySeqMtPerS::reduce(a,
                &(|x: &N, y: &N| -> (ret: N)
                    ensures ret == spec_max_fn()(*x, *y)
                { if *x >= *y { *x } else { *y } }),
                Ghost(spec_max_fn()), 0);

            proof {
                let s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
                lemma_max_fold_left_bound(s, 0);
                lemma_max_fold_left_achievable(s, 0);
                assert(max_val == s.fold_left(0 as N, spec_max_fn()));
                assert forall|i: int| #![trigger a.spec_index(i)]
                    0 <= i < a.spec_len() implies a.spec_index(i) <= max_val
                by {
                    assert(s[i] == a.spec_index(i));
                }
            }

            Some(max_val)
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
