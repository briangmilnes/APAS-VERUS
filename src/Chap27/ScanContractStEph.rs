//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Sequential scan using contraction technique (Chapter 27, Algorithm 27.3).
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod ScanContractStEph {


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


    pub trait ScanContractStEphTrait<T: StT> {
        /// Exclusive scan using contraction: contract→solve→expand.
        /// Returns prefixes where result[i] = fold_left(input[0..i], id, spec_f).
        /// - Alg Analysis: APAS (Ch27 Alg 27.3): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: sequential contraction/expansion loops
        fn scan_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqStEphS<T>)
            requires
                a.spec_len() <= usize::MAX,
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
        fn expand_scan<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            b: &ArraySeqStEphS<T>,
            c: &ArraySeqStEphS<T>,
            f: &F,
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


    impl<T: StT + Clone> ScanContractStEphTrait<T> for ArraySeqStEphS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive contract/expand; St sequential.
        fn scan_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqStEphS<T>)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            // Base case: empty
            if n == 0 {
                // Veracity: NEEDED proof block
                proof {
                }
                return ArraySeqStEphS { seq: Vec::new() };
            }

            // Base case: single element — result is [id]
            if n == 1 {
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(id);
                // Veracity: NEEDED proof block
                proof {
                }
                return ArraySeqStEphS { seq: v };
            }

            // f(id, id) == id by left identity, producing an owned T without clone.
            let id_for_recurse = f(&id, &id);
            // Veracity: NEEDED proof block
            proof {
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

            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|j: int| #![trigger b_seq[j]] 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                }
            }

            // Solve: recursively scan contracted sequence
            let c = Self::scan_contract(&b, f, Ghost(spec_f), id_for_recurse);

            // Veracity: NEEDED proof block
            proof {
                let ghost b_view = Seq::new(b.spec_len(), |j: int| b.spec_index(j));
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int implies
                    c.spec_index(k) == b_seq.take(k).fold_left(id, spec_f)
                by {
                }
            }

            // Expand
            let result_vec = Self::expand_scan(a, &b, &c, f, Ghost(spec_f), &id, n, half);
            let scanned = ArraySeqStEphS { seq: result_vec };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger scanned.spec_index(k)]
                    0 <= k < n as int implies
                    scanned.spec_index(k) == s.take(k).fold_left(id, spec_f)
                by {
                }
            }
            scanned
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — single loop over n/2 pairs; St sequential.
        fn expand_scan<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            b: &ArraySeqStEphS<T>,
            c: &ArraySeqStEphS<T>,
            f: &F,
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
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                decreases half - j,
            {
                let even_val = f(id, c.nth(j));
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_even::<T>(s, b_seq, spec_f, *id, j as int);
                }
                result_vec.push(even_val);

                let odd_val = f(c.nth(j), a.nth(2 * j));
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_odd::<T>(s, spec_f, *id, j as int);
                }
                result_vec.push(odd_val);

                j += 1;
            }

            if n % 2 == 1 {
                let last_val = f(c.nth(half - 1), b.nth(half - 1));
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
