// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Parallel reduce using contraction technique (Chapter 27, Algorithm 27.2).
//! Uses the help-first scheduler for fork-join parallelism.
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod ReduceContractMtEph {


    //		Section 2. imports

    use std::sync::Arc;
    use vstd::prelude::*;

    verus! 
{


    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap27::ContractSpecsAndLemmas::ContractSpecsAndLemmas::*;
    use crate::Concurrency::Concurrency::StTInMtT;
    use crate::vstdplus::monoid::monoid::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::call_f;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 8. traits


    pub trait ReduceContractMtEphTrait<T: StTInMtT> {
        /// Reduce a sequence using parallel contraction: contract→solve→expand.
        /// Subsumes Example 27.1 (Maximal Element): call with max and 0 identity.
        /// - Alg Analysis: APAS (Ch27 Alg 27.2): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — contraction via one-level join, each half sequential.
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            requires
                a.spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                reduced == Seq::new(a.spec_len(), |i: int| a.spec_index(i)).fold_left(id, spec_f);
    }

    //		Section 9. impls


    /// Parallel contraction: build b[j] = f(a[2j], a[2j+1]) using fork-join.
    /// Parallelism via the help-first scheduler's join.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n/2) — two parallel halves via join, each O(n/4).
    pub fn contract_parallel<T: StTInMtT, F: Fn(&T, &T) -> T + Send + Sync + 'static>(
        a: &ArraySeqMtEphS<T>,
        f: &Arc<F>,
        Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
        half: usize,
    ) -> (b: ArraySeqMtEphS<T>)
        requires
            half == a.spec_len() / 2,
            a.spec_len() >= 2,
            half <= usize::MAX,
            obeys_feq_clone::<T>(),
            forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
        ensures
            b.spec_len() == half as nat,
            forall|j: int| #![trigger b.spec_index(j)] 0 <= j < half as int ==> {
                &&& 2 * j + 1 < a.spec_len()
                &&& b.spec_index(j) == spec_f(a.spec_index(2 * j), a.spec_index(2 * j + 1))
            },
    {
        use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;

        let n = a.length();
        let mid_half = half / 2;

        // Clone array into Arc for sharing between closures
        let a_cloned = a.clone();
        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block (speed hint)
        proof {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|i: int| 0 <= i < a.spec_len() implies
                a_cloned.spec_index(i) == a.spec_index(i)
            by {
                axiom_cloned_implies_eq_owned(a.seq@[i], a_cloned.seq@[i]);
            }
        }
        let a_arc = Arc::new(a_cloned);
        let f_left = Arc::clone(f);
        let f_right = Arc::clone(f);
        let a_left = Arc::clone(&a_arc);
        let a_right = Arc::clone(&a_arc);

        let ghost a_spec_len = a.spec_len();

        let fa = move || -> (r: Vec<T>)
            requires
                mid_half <= a_left.spec_len() / 2,
                a_left.spec_len() >= 2,
                a_left.spec_len() <= usize::MAX,
                a_left.spec_len() == a_spec_len,
                forall|x: &T, y: &T| #[trigger] f_left.requires((x, y)),
                forall|x: T, y: T, ret: T| f_left.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                r@.len() == mid_half as int,
                forall|j: int| #![trigger r@[j]] 0 <= j < mid_half as int ==> {
                    &&& 2 * j + 1 < a_left.spec_len()
                    &&& r@[j] == spec_f(a_left.spec_index(2 * j), a_left.spec_index(2 * j + 1))
                },
        {
            let mut v: Vec<T> = Vec::with_capacity(mid_half);
            let mut i: usize = 0;
            while i < mid_half
                invariant
                    i <= mid_half,
                    mid_half <= a_left.spec_len() / 2,
                    a_left.spec_len() >= 2,
                    a_left.spec_len() <= usize::MAX,
                    v@.len() == i as int,
                    forall|x: &T, y: &T| #[trigger] f_left.requires((x, y)),
                    forall|x: T, y: T, ret: T| f_left.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int ==> {
                        &&& 2 * j + 1 < a_left.spec_len()
                        &&& v@[j] == spec_f(a_left.spec_index(2 * j), a_left.spec_index(2 * j + 1))
                    },
                decreases mid_half - i,
            {
                let combined = call_f(&f_left, a_left.nth(2 * i), a_left.nth(2 * i + 1));
                v.push(combined);
                i += 1;
            }
            v
        };

        let fb = move || -> (r: Vec<T>)
            requires
                mid_half <= half,
                half == a_right.spec_len() / 2,
                a_right.spec_len() >= 2,
                a_right.spec_len() <= usize::MAX,
                a_right.spec_len() == a_spec_len,
                forall|x: &T, y: &T| #[trigger] f_right.requires((x, y)),
                forall|x: T, y: T, ret: T| f_right.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                r@.len() == (half - mid_half) as int,
                forall|j: int| #![trigger r@[j]] 0 <= j < (half - mid_half) as int ==> {
                    &&& 2 * (mid_half as int + j) + 1 < a_right.spec_len()
                    &&& r@[j] == spec_f(a_right.spec_index(2 * (mid_half as int + j)), a_right.spec_index(2 * (mid_half as int + j) + 1))
                },
        {
            let mut v: Vec<T> = Vec::with_capacity(half - mid_half);
            let mut i: usize = 0;
            let count = half - mid_half;
            while i < count
                invariant
                    i <= count,
                    count == half - mid_half,
                    mid_half <= half,
                    half == a_right.spec_len() / 2,
                    a_right.spec_len() >= 2,
                    a_right.spec_len() <= usize::MAX,
                    v@.len() == i as int,
                    forall|x: &T, y: &T| #[trigger] f_right.requires((x, y)),
                    forall|x: T, y: T, ret: T| f_right.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int ==> {
                        &&& 2 * (mid_half as int + j) + 1 < a_right.spec_len()
                        &&& v@[j] == spec_f(a_right.spec_index(2 * (mid_half as int + j)), a_right.spec_index(2 * (mid_half as int + j) + 1))
                    },
                decreases count - i,
            {
                let idx = mid_half + i;
                let combined = call_f(&f_right, a_right.nth(2 * idx), a_right.nth(2 * idx + 1));
                v.push(combined);
                i += 1;
            }
            v
        };

        let (b_left, b_right) = join(fa, fb);

        // Concatenate left and right halves (vstd spec: vec@ == old(vec)@ + old(other)@)
        let mut b_vec = b_left;
        let ghost left_snap = b_vec@;
        let ghost right_snap = b_right@;
        let mut b_right_mut = b_right;
        b_vec.append(&mut b_right_mut);

        let b = ArraySeqMtEphS { seq: b_vec };

        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block
        proof {
            // b_vec@ == left_snap + right_snap (from vstd Vec::append spec)
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|j: int| #![trigger b.spec_index(j)] 0 <= j < half as int implies {
                &&& 2 * j + 1 < a.spec_len()
                &&& b.spec_index(j) == spec_f(a.spec_index(2 * j), a.spec_index(2 * j + 1))
            } by {
                if j < mid_half as int {
                } else {
                    let j2 = j - mid_half as int;
                }
            }
        }

        b
    }

    impl<T: StTInMtT + Clone + 'static> ReduceContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — recursive contraction halving n each step; Mt parallel contract via join.
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            // Base case: empty
            // Veracity: NEEDED proof block
            if n == 0 {
                // Veracity: NEEDED proof block
                proof {
                }
                return id;
            }

            // Base case: single element — use f(id, a[0]) to avoid unspecified clone
            // Veracity: NEEDED proof block
            if n == 1 {
                let reduced = call_f(&f, &id, a.nth(0));
                // Veracity: NEEDED proof block
                proof {
                    reveal_with_fuel(Seq::fold_left, 2);
                }
                return reduced;
            }

            // Contract: b[i] = f(a[2i], a[2i+1]) — parallel via join
            let half = n / 2;
            // Veracity: NEEDED proof block
            let b = contract_parallel(a, &f, Ghost(spec_f), half);

            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                }
            }

            // Recurse on contracted sequence
            let id_copy = call_f(&f, &id, &id);
            let contracted_result = Self::reduce_contract_parallel(&b, Arc::clone(&f), Ghost(spec_f), id_copy);

            // Veracity: NEEDED proof block
            // Expand: handle odd-length sequences
            if n % 2 == 1 {
                let last = a.nth(n - 1);
                let reduced = call_f(&f, &contracted_result, last);
                // Veracity: NEEDED proof block
                proof {
                    let s_even = s.subrange(0, (n - 1) as int);
                    let s_last_part = s.subrange((n - 1) as int, n as int);

                    s.lemma_fold_left_split(id, spec_f, (n - 1) as int);
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(b_seq =~= Seq::new(
                        (s_even.len() / 2) as nat,
                        |i: int| spec_f(s_even[2 * i], s_even[2 * i + 1]),
                    // Veracity: NEEDED proof block
                    ));
                    lemma_contraction_even::<T>(s_even, spec_f, id);
                }
                reduced
            } else {
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(b_seq =~= Seq::new(
                        (s.len() / 2) as nat,
                        |i: int| spec_f(s[2 * i], s[2 * i + 1]),
                    ));
                    lemma_contraction_even::<T>(s, spec_f, id);
                }
                contracted_result
            }
        }
    }

    } // verus!
} // mod
