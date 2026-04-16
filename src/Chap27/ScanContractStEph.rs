// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
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

    /// Opaque wrapper around `s.take(k).fold_left(b, f)` that prevents
    /// `lemma_fold_left_split` (auto-broadcast via `group_vstd_default`) from
    /// matching its trigger inside this module's loop invariants, which otherwise
    /// causes a quantifier matching loop after Verus 0.2026.04.10.
    #[verifier::opaque]
    pub open spec fn scan_prefix<T>(s: Seq<T>, b: T, f: spec_fn(T, T) -> T, k: int) -> T {
        s.take(k).fold_left(b, f)
    }

    /// Bridge lemma: scan_prefix is equal to s.take(k).fold_left(b, f) by definition.
    pub proof fn lemma_scan_prefix_unfold<T>(s: Seq<T>, b: T, f: spec_fn(T, T) -> T, k: int)
        ensures scan_prefix(s, b, f, k) == s.take(k).fold_left(b, f),
    {
        reveal(scan_prefix);
    }

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
// Veracity: UNNEEDED proof block                 proof {
// Veracity: UNNEEDED proof block                 }
                return ArraySeqStEphS { seq: Vec::new() };
            }

            // Base case: single element — result is [id]
            if n == 1 {
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(id);
                // Veracity: NEEDED proof block
// Veracity: UNNEEDED proof block                 proof {
// Veracity: UNNEEDED proof block                 }
                return ArraySeqStEphS { seq: v };
            }

            // f(id, id) == id by left identity, producing an owned T without clone.
            let id_for_recurse = f(&id, &id);
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block (speed hint)
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

            // Solve: recursively scan contracted sequence
            let c = Self::scan_contract(&b, f, Ghost(spec_f), id_for_recurse);
// Veracity: NEEDED proof block

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

            // Expand
            // Veracity: NEEDED proof block
            let result_vec = Self::expand_scan(a, &b, &c, f, Ghost(spec_f), &id, n, half);
            let scanned = ArraySeqStEphS { seq: result_vec };
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

            // Convert the caller's fold_left fact to the opaque scan_prefix form.
            // This shields the loop invariant below from the auto-broadcast
            // lemma_fold_left_split that otherwise matching-loops on the fold_left
            // terms (Verus 0.2026.04.10 regression).
            proof {
                assert forall|k: int| #![trigger scan_prefix(b_seq, *id, spec_f, k)]
                    0 <= k < half as int implies
                    c.spec_index(k) == scan_prefix(b_seq, *id, spec_f, k)
                by {
                    lemma_scan_prefix_unfold(b_seq, *id, spec_f, k);
                    assert(b_seq == Seq::new(b.spec_len(), |j: int| b.spec_index(j)));
                }
            }

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
                    forall|k: int| #![trigger scan_prefix(b_seq, *id, spec_f, k)]
                        0 <= k < half as int ==>
                        c.spec_index(k) == scan_prefix(b_seq, *id, spec_f, k),
                    result_vec@.len() == 2 * j as int,
                    forall|k: int| #![trigger scan_prefix(s, *id, spec_f, k)]
                        0 <= k < 2 * j as int ==>
                        result_vec@[k] == scan_prefix(s, *id, spec_f, k),
                    spec_monoid(spec_f, *id),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                // Veracity: NEEDED proof block
                decreases half - j,
            {
                let even_val = f(id, c.nth(j));
                proof {
                    lemma_expand_even::<T>(s, b_seq, spec_f, *id, j as int);
                    lemma_scan_prefix_unfold(b_seq, *id, spec_f, j as int);
                    lemma_scan_prefix_unfold(s, *id, spec_f, 2 * j as int);
                }
                result_vec.push(even_val);

                let odd_val = f(c.nth(j), a.nth(2 * j));
                proof {
                    lemma_expand_odd::<T>(s, spec_f, *id, j as int);
                    lemma_scan_prefix_unfold(s, *id, spec_f, 2 * j as int);
                    lemma_scan_prefix_unfold(s, *id, spec_f, 2 * j as int + 1);
                }
                result_vec.push(odd_val);

                j += 1;
            }

            if n % 2 == 1 {
                let last_val = f(c.nth(half - 1), b.nth(half - 1));
                proof {
                    lemma_expand_odd_tail::<T>(s, b_seq, spec_f, *id, half as int);
                    lemma_scan_prefix_unfold(b_seq, *id, spec_f, half as int - 1);
                    lemma_scan_prefix_unfold(s, *id, spec_f, 2 * half as int);
                }
                result_vec.push(last_val);
            }

            // Convert invariant back to fold_left for the ensures clause.
            proof {
                assert forall|k: int| #![trigger s.take(k).fold_left(*id, spec_f)]
                    0 <= k < 2 * half as int implies
                    result_vec@[k] == s.take(k).fold_left(*id, spec_f)
                by {
                    lemma_scan_prefix_unfold(s, *id, spec_f, k);
                }
                if n % 2 == 1 {
                    lemma_scan_prefix_unfold(s, *id, spec_f, 2 * half as int);
                }
            }

            result_vec
        }
    }

    } // verus!
} // mod
