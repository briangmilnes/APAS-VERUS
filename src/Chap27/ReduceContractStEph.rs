//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Sequential reduce using contraction technique (Chapter 27, Algorithm 27.2).
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod ReduceContractStEph {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap27::ContractSpecsAndLemmas::ContractSpecsAndLemmas::*;
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

    //		Section 8. traits


    pub trait ReduceContractStEphTrait<T: StT> {
        /// Reduce a sequence using contraction: contract→solve→expand.
        /// Subsumes Example 27.1 (Maximal Element): call with max and 0 identity.
        /// - Alg Analysis: APAS (Ch27 Alg 27.2): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: sequential contraction, no parallel tabulate
        fn reduce_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                reduced == Seq::new(a.spec_len(), |i: int| a.spec_index(i)).fold_left(id, spec_f);
    }

    //		Section 9. impls


    impl<T: StT + Clone> ReduceContractStEphTrait<T> for ArraySeqStEphS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive contraction halving n each step; St sequential.
        fn reduce_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            // Base case: empty
            if n == 0 {
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block (speed hint)
                proof {
                }
                return id;
            }

            // Base case: single element — use f(id, a[0]) to avoid unspecified clone
            if n == 1 {
                let reduced = f(&id, a.nth(0));
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    reveal_with_fuel(Seq::fold_left, 2);
                }
                return reduced;
            }

            // Contract: b[i] = f(a[2i], a[2i+1])
            let half = n / 2;
            let mut b_vec: Vec<T> = Vec::with_capacity(half);
            let mut i: usize = 0;
            while i < half
                invariant
                    i <= half,
                    half == n / 2,
                    n == a.spec_len(),
                    n >= 2,
                    half <= n,
                    s == Seq::new(a.spec_len(), |k: int| a.spec_index(k)),
                    b_vec@.len() == i as int,
                    forall|j: int| #![trigger b_vec@[j]] 0 <= j < i as int ==> {
                        &&& 2 * j + 1 < s.len()
                        &&& b_vec@[j] == spec_f(s[2 * j], s[2 * j + 1])
                    },
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                decreases half - i,
            {
                let left = a.nth(2 * i);
                let right = a.nth(2 * i + 1);
                let combined = f(left, right);
                b_vec.push(combined);
                i += 1;
            }
            let b = ArraySeqStEphS { seq: b_vec };

            // Veracity: NEEDED proof block
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
            let contracted_result = Self::reduce_contract(&b, f, Ghost(spec_f), id);

            // Expand: handle odd-length sequences
            if n % 2 == 1 {
                // Veracity: NEEDED proof block
                let last = a.nth(n - 1);
                let reduced = f(&contracted_result, last);
                // Veracity: NEEDED proof block
                proof {
                    let s_even = s.subrange(0, (n - 1) as int);
                    let s_last_part = s.subrange((n - 1) as int, n as int);

                    // s.fold_left(id, f) == s_last_part.fold_left(s_even.fold_left(id, f), f)
                    s.lemma_fold_left_split(id, spec_f, (n - 1) as int);

                    // s_last_part has one element, fold equals f(acc, s[n-1])
                    reveal(Seq::fold_left);

                    // s_even.fold_left(id, f) == b_seq.fold_left(id, f) by contraction
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(b_seq =~= Seq::new(
                        (s_even.len() / 2) as nat,
                        |i: int| spec_f(s_even[2 * i], s_even[2 * i + 1]),
                    ));
                    lemma_contraction_even::<T>(s_even, spec_f, id);
                // Veracity: NEEDED proof block
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
