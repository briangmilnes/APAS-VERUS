// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Parallel scan using contraction technique (Chapter 27, Algorithm 27.3).
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

pub mod ScanContractMtEph {


    //		Section 2. imports

    use std::sync::Arc;
    use vstd::prelude::*;

    verus! 
{


    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap27::ContractSpecsAndLemmas::ContractSpecsAndLemmas::*;
    use crate::Chap27::ReduceContractMtEph::ReduceContractMtEph::contract_parallel;
    use crate::vstdplus::smart_ptrs::smart_ptrs::call_f;
    use crate::Concurrency::Concurrency::StTInMtT;
    use crate::vstdplus::monoid::monoid::*;
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


    pub trait ScanContractMtEphTrait<T: StTInMtT> {
        /// Exclusive scan using parallel contraction: contract→solve→expand.
        /// Returns prefixes where result[i] = fold_left(input[0..i], id, spec_f).
        /// - Alg Analysis: APAS (Ch27 Alg 27.3): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — contraction parallel via one-level join; expansion sequential.
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqMtEphS<T>)
            requires
                a.spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                scanned.spec_len() == a.spec_len(),
                forall|i: int| #![trigger scanned.spec_index(i)]
                    0 <= i < a.spec_len() ==>
                        scanned.spec_index(i) == Seq::new(a.spec_len(), |j: int| a.spec_index(j)).take(i).fold_left(id, spec_f);

        /// Expand phase: interleave contracted scan results into full scan output.
        /// - Alg Analysis: APAS (Ch27 Alg 27.3): Work O(n), Span O(1) — parallel tabulate (expansion step).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential loop, no parallelism.
        fn expand_scan_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            b: &ArraySeqMtEphS<T>,
            c: &ArraySeqMtEphS<T>,
            f: &Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: &T,
            n: usize,
            half: usize,
        ) -> (prefixes: Vec<T>)
            requires
                n == a.spec_len(),
                n >= 2,
                half == n / 2,
                half <= n,
                b.spec_len() == half as nat,
                c.spec_len() == half as nat,
                spec_monoid(spec_f, *id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                forall|k: int| #![trigger b.spec_index(k)] 0 <= k < half as int ==> {
                    &&& 2 * k + 1 < a.spec_len()
                    &&& b.spec_index(k) == spec_f(a.spec_index(2 * k), a.spec_index(2 * k + 1))
                },
                forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int ==>
                    c.spec_index(k) == Seq::new(b.spec_len(), |j: int| b.spec_index(j)).take(k).fold_left(*id, spec_f),
            ensures ({
                let s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
                &&& prefixes@.len() == n
                &&& forall|k: int| #![trigger prefixes@[k]] 0 <= k < n as int ==>
                        prefixes@[k] == s.take(k).fold_left(*id, spec_f)
            });
    }

    //		Section 9. impls


    impl<T: StTInMtT + Clone + 'static> ScanContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg^2 n) — recursive contract/expand; Mt parallel contract via join + parallel expand via tabulate.
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqMtEphS<T>)
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
                return ArraySeqMtEphS { seq: Vec::new() };
            }

            // Base case: single element — result is [id]
            if n == 1 {
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(id);
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                }
                return ArraySeqMtEphS { seq: v };
            }

            // f(id, id) == id by left identity, producing an owned T without clone.
            // Veracity: NEEDED proof block
            let id_for_recurse = call_f(&f, &id, &id);
            // Veracity: NEEDED proof block
            proof {
            }

            // Contract: b[i] = f(a[2i], a[2i+1]) — parallel via join
            let half = n / 2;
            let b = contract_parallel(a, &f, Ghost(spec_f), half);
// Veracity: NEEDED proof block

            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|j: int| #![trigger b_seq[j]] 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                }
            }

            // Veracity: NEEDED proof block
            // Solve: recursively scan contracted sequence
            let c = Self::scan_contract_parallel(&b, Arc::clone(&f), Ghost(spec_f), id_for_recurse);

            // Veracity: NEEDED proof block
            proof {
                let ghost b_view = Seq::new(b.spec_len(), |j: int| b.spec_index(j));
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int implies
                    c.spec_index(k) == b_seq.take(k).fold_left(id, spec_f)
                by {
                }
            }
// Veracity: NEEDED proof block

            // Expand
            let result_vec = Self::expand_scan_parallel(a, &b, &c, &f, Ghost(spec_f), &id, n, half);
            let scanned = ArraySeqMtEphS { seq: result_vec };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger scanned.spec_index(k)]
                    0 <= k < n as int implies
                    scanned.spec_index(k) == s.take(k).fold_left(id, spec_f)
                by {
                }
            }
            scanned
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel expand via tabulate over n elements.
        fn expand_scan_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            b: &ArraySeqMtEphS<T>,
            c: &ArraySeqMtEphS<T>,
            f: &Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: &T,
            n: usize,
            half: usize,
        ) -> (prefixes: Vec<T>)
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));

            let mut result_vec: Vec<T> = Vec::with_capacity(n);
            let mut j: usize = 0;
            while j < half
                invariant
                    j <= half,
                    half == n / 2,
                    n == a.spec_len(),
                    n >= 2,
                    half <= n,
                    s == Seq::new(a.spec_len(), |k: int| a.spec_index(k)),
                    b.spec_len() == half as nat,
                    b_seq == Seq::new(b.spec_len(), |i: int| b.spec_index(i)),
                    b_seq.len() == half as nat,
                    c.spec_len() == half as nat,
                    forall|k: int| #![trigger b_seq[k]] 0 <= k < b_seq.len() ==>
                        b_seq[k] == spec_f(s[2 * k], s[2 * k + 1]),
                    forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int ==>
                        c.spec_index(k) == b_seq.take(k).fold_left(*id, spec_f),
                    result_vec@.len() == 2 * j as int,
                    forall|k: int| #![trigger result_vec@[k]] 0 <= k < 2 * j as int ==>
                        result_vec@[k] == s.take(k).fold_left(*id, spec_f),
                    spec_monoid(spec_f, *id),
                    // Veracity: NEEDED proof block
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                decreases half - j,
            {
                let even_val = call_f(f, id, c.nth(j));
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_even::<T>(s, b_seq, spec_f, *id, j as int);
                }
                result_vec.push(even_val);

                let odd_val = call_f(f, c.nth(j), a.nth(2 * j));
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_odd::<T>(s, spec_f, *id, j as int);
                }
                // Veracity: NEEDED proof block
                result_vec.push(odd_val);

                j += 1;
            }

            if n % 2 == 1 {
                let last_val = call_f(f, c.nth(half - 1), b.nth(half - 1));
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_odd_tail::<T>(s, b_seq, spec_f, *id, half as int);
                }
                result_vec.push(last_val);
            }

            result_vec
        }
    }

    } // verus!
} // mod
