// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Divide-and-conquer via reduce pattern - parallel implementation (Chapter 26, Section 5).
//! Note: Unconditionally parallel - no thresholding per APAS rules.
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module


pub mod DivConReduceMtPer {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
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

    //		Section 7. proof fns/broadcast groups


    /// Helper: establish fold_left one-step decomposition via lemma_fold_left_split.
    proof fn lemma_fold_left_step(s: Seq<usize>, acc: usize)
        requires s.len() > 0,
        ensures s.fold_left(acc, spec_max_fn())
            == s.subrange(1, s.len() as int).fold_left(
                spec_max_fn()(acc, s[0]), spec_max_fn()),
    {
        reveal_with_fuel(Seq::<_>::fold_left::<_>, 2);
        s.lemma_fold_left_split(acc, spec_max_fn(), 1);
        let first = s.subrange(0, 1);
        // Veracity: NEEDED assert (speed hint)
        assert(first =~= Seq::new(1, |i: int| s[0]));
    }

    /// fold_left(s, acc, max) >= acc and every s[i] <= fold_left result.
    proof fn lemma_max_fold_left_bound(s: Seq<usize>, acc: usize)
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
            // Veracity: NEEDED assert
            assert forall|i: int| #![trigger s[i]] 0 <= i < s.len()
                implies s[i] <= s.fold_left(acc, spec_max_fn())
            by {
                if i == 0 {
                } else {
                    // Veracity: NEEDED assert
                    assert(rest[i - 1] == s[i]);
                }
            }
        }
    }

    /// fold_left(s, acc, max) is either acc itself or some element of s.
    proof fn lemma_max_fold_left_achievable(s: Seq<usize>, acc: usize)
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
            let max_val = rest.fold_left(new_acc, spec_max_fn());
            if max_val == new_acc {
                if new_acc == acc {
                } else {
                    // Veracity: NEEDED assert (speed hint)
                    assert(s[0] == max_val);
                }
            } else {
                let j = choose|j: int| 0 <= j < rest.len() && rest[j] == max_val;
                // Veracity: NEEDED assert (speed hint)
                assert(rest[j] == s[j + 1]);
                // Veracity: NEEDED assert (speed hint)
                assert(s[j + 1] == max_val);
            }
        }
    }

    //		Section 8. traits


    pub trait DivConReduceMtTrait {
        /// Find maximum element via parallel reduce.
        /// Pattern: reduce max identity (parallel)
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n) — D&C reduce with constant-time op.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn max_element_parallel(a: &ArraySeqMtPerS<usize>) -> (max: Option<usize>)
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

        /// Sum all elements via parallel reduce.
        /// Pattern: reduce (+) 0 identity (parallel)
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n) — D&C reduce with constant-time op.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn sum_parallel(a: &ArraySeqMtPerS<usize>) -> (total: usize)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_sum_fn(), 0),
            ensures
                total == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_sum_fn(), 0);

        /// Product of all elements via parallel reduce.
        /// Pattern: reduce (*) 1 identity (parallel)
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n) — D&C reduce with constant-time op.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn product_parallel(a: &ArraySeqMtPerS<usize>) -> (total: usize)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_product_fn(), 1),
            ensures
                total == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_product_fn(), 1);

        /// Logical OR of all elements via parallel reduce.
        /// Pattern: reduce (||) false identity (parallel)
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n) — D&C reduce with constant-time op.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn any_parallel(a: &ArraySeqMtPerS<bool>) -> (found: bool)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_or_fn(), false),
            ensures
                found == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_or_fn(), false);

        /// Logical AND of all elements via parallel reduce.
        /// Pattern: reduce (&&) true identity (parallel)
        /// - Alg Analysis: APAS (Ch26 Alg 26.2): Work O(n), Span O(lg n) — D&C reduce with constant-time op.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to ArraySeqMtPerS::reduce (parallel). Agrees with APAS.
        fn all_parallel(a: &ArraySeqMtPerS<bool>) -> (all_true: bool)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_and_fn(), true),
            ensures
                all_true == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_and_fn(), true);
    }

    //		Section 9. impls


    //		9. bridge fns (named closures with ensures per standard 8)

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to parallel reduce; Mt parallel.
    fn call_reduce_max(a: &ArraySeqMtPerS<usize>) -> (reduced: usize)
        requires spec_monoid(spec_max_fn(), 0usize)
        ensures reduced == spec_iterate(
            Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_max_fn(), 0usize)
    {
        let f = |x: &usize, y: &usize| -> (r: usize)
            ensures r == spec_max_fn()(*x, *y)
        { if *x >= *y { *x } else { *y } };
        ArraySeqMtPerS::reduce(a, &f, Ghost(spec_max_fn()), 0)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to parallel reduce; Mt parallel.
    fn call_reduce_sum(a: &ArraySeqMtPerS<usize>) -> (reduced: usize)
        requires spec_monoid(spec_sum_fn(), 0usize)
        ensures reduced == spec_iterate(
            Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_sum_fn(), 0usize)
    {
        let f = |x: &usize, y: &usize| -> (r: usize)
            ensures r == spec_wrapping_add(*x, *y)
        { (*x).wrapping_add(*y) };
        ArraySeqMtPerS::reduce(a, &f, Ghost(spec_sum_fn()), 0)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to parallel reduce; Mt parallel.
    fn call_reduce_product(a: &ArraySeqMtPerS<usize>) -> (reduced: usize)
        requires spec_monoid(spec_product_fn(), 1usize)
        ensures reduced == spec_iterate(
            Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_product_fn(), 1usize)
    {
        let f = |x: &usize, y: &usize| -> (r: usize)
            ensures r == spec_wrapping_mul(*x, *y)
        { (*x).wrapping_mul(*y) };
        ArraySeqMtPerS::reduce(a, &f, Ghost(spec_product_fn()), 1)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to parallel reduce; Mt parallel.
    fn call_reduce_or(a: &ArraySeqMtPerS<bool>) -> (reduced: bool)
        requires spec_monoid(spec_or_fn(), false)
        ensures reduced == spec_iterate(
            Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_or_fn(), false)
    {
        let f = |x: &bool, y: &bool| -> (r: bool)
            ensures r == spec_or_fn()(*x, *y)
        { *x || *y };
        ArraySeqMtPerS::reduce(a, &f, Ghost(spec_or_fn()), false)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to parallel reduce; Mt parallel.
    fn call_reduce_and(a: &ArraySeqMtPerS<bool>) -> (reduced: bool)
        requires spec_monoid(spec_and_fn(), true)
        ensures reduced == spec_iterate(
            Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_and_fn(), true)
    {
        let f = |x: &bool, y: &bool| -> (r: bool)
            ensures r == spec_and_fn()(*x, *y)
        { *x && *y };
        ArraySeqMtPerS::reduce(a, &f, Ghost(spec_and_fn()), true)
    }


    impl DivConReduceMtTrait for ArraySeqMtPerS<usize> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel D&C reduce via call_reduce_max; Mt parallel.
        fn max_element_parallel(a: &ArraySeqMtPerS<usize>) -> (max: Option<usize>) {
            let len = a.length();
            if len == 0 {
                return None;
            }

            // Veracity: NEEDED proof block (speed hint)
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert forall|x: usize| #[trigger] spec_max_fn()(0 as usize, x) == x by {}
                // Veracity: NEEDED assert (speed hint)
                assert forall|x: usize| #[trigger] spec_max_fn()(x, 0 as usize) == x by {}
// Veracity: UNNEEDED assert                 assert forall|x: usize, y: usize, z: usize|
// Veracity: UNNEEDED assert                     #[trigger] spec_max_fn()(spec_max_fn()(x, y), z)
// Veracity: UNNEEDED assert                     == spec_max_fn()(x, spec_max_fn()(y, z)) by {}
            }

            let max_val = call_reduce_max(a);
// Veracity: NEEDED proof block

            proof {
                let s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
                lemma_max_fold_left_bound(s, 0);
                lemma_max_fold_left_achievable(s, 0);
                // Veracity: NEEDED assert (speed hint)
                assert(max_val == s.fold_left(0 as usize, spec_max_fn()));
                // Veracity: NEEDED assert
                assert forall|i: int| #![trigger a.spec_index(i)]
                    0 <= i < a.spec_len() implies a.spec_index(i) <= max_val
                by {
                    // Veracity: NEEDED assert
                    assert(s[i] == a.spec_index(i));
                }
            }

            Some(max_val)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to call_reduce_sum; Mt parallel.
        fn sum_parallel(a: &ArraySeqMtPerS<usize>) -> (total: usize) {
            call_reduce_sum(a)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to call_reduce_product; Mt parallel.
        fn product_parallel(a: &ArraySeqMtPerS<usize>) -> (total: usize) {
            call_reduce_product(a)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to call_reduce_or; Mt parallel.
        fn any_parallel(a: &ArraySeqMtPerS<bool>) -> (found: bool) {
            call_reduce_or(a)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — delegates to call_reduce_and; Mt parallel.
        fn all_parallel(a: &ArraySeqMtPerS<bool>) -> (all_true: bool) {
            call_reduce_and(a)
        }
    }

    } // verus!
} // mod
